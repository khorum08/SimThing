//! RF-5: install + EvalEML transport for need / weight_profile bindings.
//!
//! - Inputs and weights are live Amount columns of **existing** properties.
//! - Need is written to the existing Arena participant **AllocatorWeight** cell.
//! - Thresholds inject into `resource_economy.emit_on_threshold` and ride the
//!   same sealed AccumulatorOp event path as field-economy disruption.
//! - No synthetic need host, no default weight seeds, no Studio feeder values.

use simthing_core::{
    eml_nodes, rebuild_emit_on_threshold_ops, AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode,
    DimensionRegistry, EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta,
    EmlNodeGpu, EmlTreeId, EmitOnThresholdBuffer, EmitOnThresholdRegistration, GateSpec, ScaleSpec,
    SimThing, SimThingId, SlotIndex, SourceSpec, SubFieldRole, ThresholdDirection,
};
use simthing_spec::{
    EmlGadgetInstanceSpec, NeedWeightProfileBindingSpec, NeedWeightProfileThresholdSpec,
    PropertyKey, ResourceFlowSpec, SpecError,
};

use crate::arena_hierarchy::resolve_node_columns;
use crate::arena_participant::ArenaParticipantScaffold;
use crate::install::{find_simthing_mut, resolve_install_target, InstallError};
use crate::resource_economy_compile::ResourceEconomyRegistry;
use crate::scenario::Scenario;
use simthing_gpu::SlotAllocator;

const NEED_WEIGHT_TREE_BASE: u32 = 7_200_000;

/// Fully resolved binding ready for EvalEML op build + threshold injection.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedNeedWeightProfile {
    pub id: String,
    pub profile: String,
    pub participant_slot: u32,
    pub hosted_id: SimThingId,
    /// ScenarioListed target_id used at resolve (for overlay host matching).
    pub install_target_id: String,
    pub arena: String,
    pub arena_idx: u32,
    /// Global column of the existing AllocatorWeight cell (live need).
    pub need_col: ColumnIndex,
    /// Global columns for WeightedAccumulator inputs (Amount roles).
    pub input_cols: Vec<ColumnIndex>,
    /// Global columns for WeightedAccumulator weights (Amount roles).
    pub weight_cols: Vec<ColumnIndex>,
    /// Compiled EvalEML nodes.
    pub nodes: Vec<EmlNodeGpu>,
    pub threshold: Option<NeedWeightProfileThresholdSpec>,
    /// Authored weight property keys (telemetry / proof identity).
    pub weight_property_keys: Vec<PropertyKey>,
    pub input_property_keys: Vec<PropertyKey>,
}

/// Extract the single WeightedAccumulator from an authored stack (fail-closed).
pub fn extract_weighted_accumulator(
    binding: &NeedWeightProfileBindingSpec,
) -> Result<(usize, usize), InstallError> {
    if binding.stack.gadgets.len() != 1 {
        return Err(InstallError::NeedWeightProfileInvalid {
            binding: binding.id.clone(),
            reason: "stack must contain exactly one WeightedAccumulator gadget".into(),
        });
    }
    match &binding.stack.gadgets[0] {
        EmlGadgetInstanceSpec::WeightedAccumulator {
            input_cols,
            weight_cols,
            ..
        } => {
            if input_cols.is_empty() || input_cols.len() != weight_cols.len() {
                return Err(InstallError::NeedWeightProfileInvalid {
                    binding: binding.id.clone(),
                    reason: "WeightedAccumulator input_cols/weight_cols length mismatch".into(),
                });
            }
            Ok((input_cols.len(), weight_cols.len()))
        }
        other => Err(InstallError::NeedWeightProfileInvalid {
            binding: binding.id.clone(),
            reason: format!("expected WeightedAccumulator, got `{}`", other.kind_name()),
        }),
    }
}

/// Resolve every authored need/weight_profile binding against the live install.
pub fn resolve_need_weight_profiles(
    spec: &ResourceFlowSpec,
    scenario: &Scenario,
    root: &SimThing,
    registry: &DimensionRegistry,
    scaffold: &ArenaParticipantScaffold,
) -> Result<Vec<ResolvedNeedWeightProfile>, InstallError> {
    let mut out = Vec::with_capacity(spec.need_weight_profiles.len());
    for binding in &spec.need_weight_profiles {
        out.push(resolve_one(binding, spec, scenario, root, registry, scaffold)?);
    }
    Ok(out)
}

fn resolve_one(
    binding: &NeedWeightProfileBindingSpec,
    spec: &ResourceFlowSpec,
    scenario: &Scenario,
    root: &SimThing,
    registry: &DimensionRegistry,
    scaffold: &ArenaParticipantScaffold,
) -> Result<ResolvedNeedWeightProfile, InstallError> {
    let (n_in, n_w) = extract_weighted_accumulator(binding)?;
    if binding.weight_properties.is_empty() {
        return Err(InstallError::NeedWeightProfileInvalid {
            binding: binding.id.clone(),
            reason: "weight_properties empty — no default-weight fallback".into(),
        });
    }
    if binding.input_properties.is_empty() {
        return Err(InstallError::NeedWeightProfileInvalid {
            binding: binding.id.clone(),
            reason: "input_properties empty — no synthetic input fallback".into(),
        });
    }
    if binding.input_properties.len() != n_in || binding.weight_properties.len() != n_w {
        return Err(InstallError::NeedWeightProfileInvalid {
            binding: binding.id.clone(),
            reason: format!(
                "property arity mismatch: inputs {}/{} weights {}/{}",
                binding.input_properties.len(),
                n_in,
                binding.weight_properties.len(),
                n_w
            ),
        });
    }

    let (arena_idx, arena) = spec
        .arenas
        .iter()
        .enumerate()
        .find(|(_, arena)| arena.name == binding.arena)
        .ok_or_else(|| SpecError::UnknownArenaReference {
            arena: binding.arena.clone(),
            context: format!("need_weight_profiles.{}", binding.id),
        })?;

    let install_target_id = match &binding.install {
        simthing_spec::InstallTargetSpec::ScenarioListed { target_id } => target_id.clone(),
        other => {
            return Err(InstallError::NeedWeightProfileInvalid {
                binding: binding.id.clone(),
                reason: format!("need bindings require ScenarioListed install, got {other:?}"),
            });
        }
    };
    let hosted_ids = resolve_install_target(&binding.install, scenario, root)?;
    if hosted_ids.is_empty() {
        return Err(InstallError::NoMatchingOwners {
            tree_id: binding.id.clone(),
            target: binding.install.clone(),
        });
    }
    if hosted_ids.len() != 1 {
        return Err(InstallError::NeedWeightProfileInvalid {
            binding: binding.id.clone(),
            reason: "install target must resolve to exactly one participant".into(),
        });
    }
    let hosted_id = hosted_ids[0];
    let raw = hosted_id.raw();
    let admitted = arena
        .explicit_participants
        .iter()
        .any(|p| p.subtree_root_id == raw);
    if !admitted {
        return Err(InstallError::NeedWeightProfileTargetNotAdmitted {
            binding: binding.id.clone(),
            arena: arena.name.clone(),
            subtree_root_id: raw,
        });
    }
    let participant_slot = scaffold
        .index
        .participant_slot(hosted_id, arena_idx as u32)
        .ok_or_else(|| InstallError::NeedWeightProfileTargetNotAdmitted {
            binding: binding.id.clone(),
            arena: arena.name.clone(),
            subtree_root_id: raw,
        })?
        .raw();

    let input_cols = resolve_amount_cols(registry, &binding.input_properties, &binding.id)?;
    let weight_cols = resolve_amount_cols(registry, &binding.weight_properties, &binding.id)?;

    let flow_property_id = registry
        .id_of(&arena.flow_property.namespace, &arena.flow_property.name)
        .ok_or_else(|| {
            InstallError::Spec(SpecError::UnknownResourceFlowProperty {
                property: format!(
                    "{}::{}",
                    arena.flow_property.namespace, arena.flow_property.name
                ),
            })
        })?;
    let flow_layout = &registry.property(flow_property_id).layout;
    // Confirm AllocatorWeight exists via hierarchy, then mint ColumnIndex via col_for_role.
    let _ = resolve_node_columns(flow_layout, &arena.name).map_err(|_| {
        InstallError::Spec(SpecError::UnknownResourceFlowProperty {
            property: format!("{} flow columns", arena.name),
        })
    })?;
    let need_col = registry
        .column_range(flow_property_id)
        .col_for_role(&SubFieldRole::Named("weight".into()), flow_layout)
        .ok_or_else(|| InstallError::NeedWeightProfileInvalid {
            binding: binding.id.clone(),
            reason: format!(
                "arena `{}` flow property missing AllocatorWeight Named(\"weight\") cell",
                arena.name
            ),
        })?;

    let nodes = build_weighted_need_nodes(&input_cols, &weight_cols);

    Ok(ResolvedNeedWeightProfile {
        id: binding.id.clone(),
        profile: binding.profile.clone(),
        participant_slot,
        hosted_id,
        install_target_id,
        arena: arena.name.clone(),
        arena_idx: arena_idx as u32,
        need_col,
        input_cols,
        weight_cols,
        nodes,
        threshold: binding.threshold.clone(),
        weight_property_keys: binding.weight_properties.clone(),
        input_property_keys: binding.input_properties.clone(),
    })
}

fn resolve_amount_cols(
    registry: &DimensionRegistry,
    keys: &[PropertyKey],
    binding_id: &str,
) -> Result<Vec<ColumnIndex>, InstallError> {
    let mut out = Vec::with_capacity(keys.len());
    for key in keys {
        let prop_id = registry.id_of(&key.namespace, &key.name).ok_or_else(|| {
            InstallError::Spec(SpecError::UnknownResourceFlowProperty {
                property: format!("{}::{}", key.namespace, key.name),
            })
        })?;
        let layout = &registry.property(prop_id).layout;
        let col = registry
            .column_range(prop_id)
            .col_for_role(&SubFieldRole::Amount, layout)
            .ok_or_else(|| InstallError::NeedWeightProfileInvalid {
                binding: binding_id.into(),
                reason: format!(
                    "property {}::{} missing Amount role",
                    key.namespace, key.name
                ),
            })?;
        out.push(col);
    }
    Ok(out)
}

fn build_weighted_need_nodes(
    input_cols: &[ColumnIndex],
    weight_cols: &[ColumnIndex],
) -> Vec<EmlNodeGpu> {
    let mut nodes = Vec::new();
    for (in_col, w_col) in input_cols.iter().zip(weight_cols.iter()) {
        nodes.push(slot_value(in_col.raw_u32()));
        nodes.push(slot_value(w_col.raw_u32()));
        nodes.push(op_node(eml_nodes::opcode::MUL));
    }
    for _ in 1..input_cols.len() {
        nodes.push(op_node(eml_nodes::opcode::ADD));
    }
    nodes.push(op_node(eml_nodes::opcode::RETURN_TOP));
    nodes
}

/// Materialize authored Amount values for input/weight properties onto the
/// arena participant wrapper so slot-local EvalEML can read them.
///
/// Sources (in order, no invented defaults):
/// 1. Hosted SimThing property map (if present)
/// 2. Resource-economy Constant emissions for that property
/// 3. GameMode OverlaySpec transforms targeting that property on the install host
///
/// Missing authored values remain 0 (fail-closed need, not a silent weight of 1.0).
pub fn seed_need_weight_property_values_on_participants(
    resolved: &[ResolvedNeedWeightProfile],
    registry: &DimensionRegistry,
    root: &mut SimThing,
    allocator: &SlotAllocator,
    game_mode: &simthing_spec::GameModeSpec,
    economy: Option<&ResourceEconomyRegistry>,
) -> Result<(), InstallError> {
    for binding in resolved {
        let participant_id = allocator
            .owner_of(SlotIndex::new(binding.participant_slot))
            .ok_or_else(|| InstallError::NeedWeightProfileParticipantSlotMissing {
                binding: binding.id.clone(),
                slot: binding.participant_slot,
            })?;
        let host_id = binding.hosted_id;
        let install_key = binding.install_target_id.clone();
        let amounts: Vec<(simthing_core::SimPropertyId, f32)> = {
            let host = find_simthing_mut(root, host_id).ok_or_else(|| {
                InstallError::NeedWeightProfileInvalid {
                    binding: binding.id.clone(),
                    reason: format!("hosted SimThing {} missing for property copy", host_id.raw()),
                }
            })?;
            let mut pairs = Vec::new();
            for key in binding
                .input_property_keys
                .iter()
                .chain(binding.weight_property_keys.iter())
            {
                let pid = registry.id_of(&key.namespace, &key.name).ok_or_else(|| {
                    InstallError::Spec(SpecError::UnknownResourceFlowProperty {
                        property: format!("{}::{}", key.namespace, key.name),
                    })
                })?;
                let layout = &registry.property(pid).layout;
                let mut amount = host
                    .properties
                    .get(&pid)
                    .map(|v| v.get_role(&SubFieldRole::Amount, layout))
                    .unwrap_or(0.0);
                let amount_col = registry
                    .column_range(pid)
                    .col_for_role(&SubFieldRole::Amount, layout)
                    .map(|c| c.raw_u32());
                if let (Some(econ), Some(acol)) = (economy, amount_col) {
                    for emission in &econ.registrations.emissions {
                        if emission.source_col == acol {
                            if let simthing_gpu::EmissionFormula::Constant { value } =
                                emission.formula
                            {
                                amount = value;
                            }
                        }
                    }
                }
                // Apply authored overlays targeting this property on the same install host.
                let prop_ref = format!("{}::{}", key.namespace, key.name);
                for overlay in &game_mode.overlays {
                    let targets = match &overlay.targets_property {
                        s if !s.is_empty() => s.as_str(),
                        _ => continue,
                    };
                    if targets != prop_ref && targets != key.name {
                        continue;
                    }
                    let overlay_host_ok = match &overlay.install {
                        simthing_spec::InstallTargetSpec::ScenarioListed { target_id } => {
                            target_id == &install_key
                        }
                        _ => false,
                    };
                    if !overlay_host_ok {
                        continue;
                    }
                    for (role, op) in &overlay.sub_field_deltas {
                        if *role != SubFieldRole::Amount {
                            continue;
                        }
                        match op {
                            simthing_core::TransformOp::Add(v) => amount += *v,
                            simthing_core::TransformOp::Multiply(v) => amount *= *v,
                            _ => {}
                        }
                    }
                }
                pairs.push((pid, amount));
            }
            pairs
        };
        let participant = find_simthing_mut(root, participant_id).ok_or_else(|| {
            InstallError::NeedWeightProfileParticipantSlotMissing {
                binding: binding.id.clone(),
                slot: binding.participant_slot,
            }
        })?;
        for (pid, amount) in amounts {
            let prop = registry.property(pid);
            let layout = prop.layout.clone();
            let value = participant
                .properties
                .entry(pid)
                .or_insert_with(|| prop.default_value());
            value.set_role(&SubFieldRole::Amount, &layout, amount);
        }
        // Start the AllocatorWeight need cell at 0 so the first EvalEML write can
        // Rising-cross an authored threshold (same posture as disruption presence).
        if let Some(flow_pid) = registry.column_owners.get(binding.need_col.raw()).map(|(p, _)| *p)
        {
            let prop = registry.property(flow_pid);
            let layout = prop.layout.clone();
            let value = participant
                .properties
                .entry(flow_pid)
                .or_insert_with(|| prop.default_value());
            value.set_role(&SubFieldRole::Named("weight".into()), &layout, 0.0);
        }
    }
    Ok(())
}

/// Build EvalEML AccumulatorOps writing need into the existing AllocatorWeight cell.
pub fn build_need_weight_profile_ops(
    resolved: &[ResolvedNeedWeightProfile],
    eml_registry: &mut EmlExpressionRegistry,
) -> Vec<AccumulatorOp> {
    let mut ops = Vec::with_capacity(resolved.len());
    for (idx, binding) in resolved.iter().enumerate() {
        let tree_id = EmlTreeId(NEED_WEIGHT_TREE_BASE + idx as u32);
        eml_registry
            .register_formula(
                tree_id,
                EmlFormulaMeta {
                    tree_id,
                    execution_class: EmlExecutionClass::ExactDeterministic,
                    allowed_consumers: EmlConsumerMask(
                        EmlConsumerMask::ALL_PRODUCTION | EmlConsumerMask::DEBUG_ORACLE,
                    ),
                    max_abs_error: None,
                    deterministic_gpu: true,
                    requires_guard_for_hard_threshold: false,
                    node_count: binding.nodes.len() as u32,
                    max_stack_depth: 0,
                    has_loops: false,
                    has_recursion: false,
                    display_name: format!("need_weight_profile_{}", binding.id),
                },
                binding.nodes.clone(),
            )
            .expect("need weight profile formula registers");

        let source_col = binding
            .weight_cols
            .first()
            .copied()
            .unwrap_or(binding.need_col);
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(binding.participant_slot),
                col: source_col,
            },
            combine: CombineFn::EvalEML { tree_id: tree_id.0 },
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(
                SlotIndex::new(binding.participant_slot),
                binding.need_col,
            )],
        });
    }
    ops
}

/// Inject sealed emit_on_threshold registrations for need thresholds into the
/// existing resource-economy registry (same path as disruption FIELD_POLICY).
pub fn inject_need_threshold_into_economy(
    resolved: &[ResolvedNeedWeightProfile],
    economy: &mut ResourceEconomyRegistry,
) {
    let mut injected = false;
    for binding in resolved {
        let Some(th) = &binding.threshold else {
            continue;
        };
        let reg = EmitOnThresholdRegistration {
            slot: SlotIndex::new(binding.participant_slot),
            col: binding.need_col,
            threshold: th.threshold,
            direction: ThresholdDirection::Upward,
            event_kind: th.event_kind,
            buffer: EmitOnThresholdBuffer::Values,
        };
        // Validate builder shape (same as economy materialize).
        let _ = rebuild_emit_on_threshold_ops(std::slice::from_ref(&reg));
        economy.registrations.emit_on_threshold.push(reg);
        economy
            .registrations
            .report
            .threshold_emit_ids
            .push(format!("{}_need_threshold", binding.id));
        injected = true;
    }
    if injected {
        economy.registrations.report.threshold_emit_count =
            economy.registrations.emit_on_threshold.len();
        economy.generation = economy.generation.saturating_add(1).max(2);
    }
}

fn slot_value(col: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_nodes::opcode::SLOT_VALUE,
        flags: 0,
        a: col,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn op_node(opcode: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

/// Build a binding from a hydrated stack + existing property keys (workshop/production helper).
/// Callers must supply real PropertyKeys and InstallTargetSpec from authored data — never
/// invent weight seeds or Studio constants.
pub fn binding_from_hydrated_stack(
    id: impl Into<String>,
    profile: impl Into<String>,
    stack: simthing_spec::EmlGadgetStackSpec,
    arena: impl Into<String>,
    install: simthing_spec::InstallTargetSpec,
    input_properties: Vec<PropertyKey>,
    weight_properties: Vec<PropertyKey>,
    threshold: Option<NeedWeightProfileThresholdSpec>,
) -> NeedWeightProfileBindingSpec {
    NeedWeightProfileBindingSpec {
        id: id.into(),
        profile: profile.into(),
        stack,
        arena: arena.into(),
        install,
        input_properties,
        weight_properties,
        threshold,
    }
}
