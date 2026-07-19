//! RF-5A: admit semantic need_binding → full-cell EvalEML + sealed threshold.
//!
//! Row authority = **entity-name uniqueness** (install_targets). PropertyKey alone
//! is never a row. No first-DFS owner, no participant property invent.

use simthing_core::{
    eml_nodes, rebuild_emit_on_threshold_ops, AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode,
    DimensionRegistry, EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta,
    EmlNodeGpu, EmlTreeId, EmitOnThresholdBuffer, EmitOnThresholdRegistration, GateSpec, ScaleSpec,
    SimThing, SimThingId, SlotIndex, SourceSpec, SubFieldRole, ThresholdDirection,
};
use simthing_spec::{
    EmlGadgetInstanceSpec, NeedBindingSpec, ResourceFlowSpec, SemanticPropertyLocusSpec, SpecError,
};

use crate::arena_hierarchy::resolve_node_columns;
use crate::arena_participant::ArenaParticipantScaffold;
use crate::install::{find_simthing_mut, InstallError};
use crate::resource_economy_compile::ResourceEconomyRegistry;
use crate::scenario::Scenario;
use simthing_gpu::SlotAllocator;

const NEED_BINDING_TREE_BASE: u32 = 7_300_000;

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedFullCell {
    pub entity: String,
    pub simthing_id: SimThingId,
    pub slot: u32,
    pub col: ColumnIndex,
    pub role: SubFieldRole,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedNeedBinding {
    pub id: String,
    pub profile: String,
    pub participant_slot: u32,
    pub participant_id: SimThingId,
    pub eml_source_slot: u32,
    pub need_col: ColumnIndex,
    pub inputs: Vec<ResolvedFullCell>,
    pub weights: Vec<ResolvedFullCell>,
    pub nodes: Vec<EmlNodeGpu>,
    pub threshold: Option<f32>,
    pub event_kind: Option<u32>,
}

pub fn resolve_need_bindings(
    spec: &ResourceFlowSpec,
    scenario: &Scenario,
    root: &SimThing,
    registry: &DimensionRegistry,
    scaffold: &ArenaParticipantScaffold,
    allocator: &SlotAllocator,
) -> Result<Vec<ResolvedNeedBinding>, InstallError> {
    let mut out = Vec::with_capacity(spec.need_bindings.len());
    for binding in &spec.need_bindings {
        out.push(resolve_one(
            binding, spec, scenario, root, registry, scaffold, allocator,
        )?);
    }
    Ok(out)
}

fn resolve_one(
    binding: &NeedBindingSpec,
    spec: &ResourceFlowSpec,
    scenario: &Scenario,
    root: &SimThing,
    registry: &DimensionRegistry,
    scaffold: &ArenaParticipantScaffold,
    allocator: &SlotAllocator,
) -> Result<ResolvedNeedBinding, InstallError> {
    if binding.inputs.is_empty() || binding.weights.is_empty() {
        return Err(InstallError::NeedBindingInvalid {
            binding: binding.id.clone(),
            reason: "inputs and weights required".into(),
        });
    }
    if binding.inputs.len() != binding.weights.len() {
        return Err(InstallError::NeedBindingInvalid {
            binding: binding.id.clone(),
            reason: "input/weight arity mismatch".into(),
        });
    }
    if binding.stack.gadgets.len() != 1 {
        return Err(InstallError::NeedBindingInvalid {
            binding: binding.id.clone(),
            reason: "stack must contain exactly one WeightedAccumulator".into(),
        });
    }
    match &binding.stack.gadgets[0] {
        EmlGadgetInstanceSpec::WeightedAccumulator {
            input_cols,
            weight_cols,
            ..
        } => {
            if input_cols.len() != binding.inputs.len()
                || weight_cols.len() != binding.weights.len()
            {
                return Err(InstallError::NeedBindingInvalid {
                    binding: binding.id.clone(),
                    reason: "stack arity vs input/weight loci mismatch".into(),
                });
            }
        }
        other => {
            return Err(InstallError::NeedBindingInvalid {
                binding: binding.id.clone(),
                reason: format!("expected WeightedAccumulator, got {}", other.kind_name()),
            });
        }
    }

    let (arena_idx, arena) = if let Some((i, a)) = spec
        .arenas
        .iter()
        .enumerate()
        .find(|(_, a)| a.name == binding.arena)
    {
        (i, a)
    } else if (binding.arena == "default" || binding.arena.is_empty()) && !spec.arenas.is_empty() {
        (0, &spec.arenas[0])
    } else {
        return Err(InstallError::Spec(SpecError::UnknownArenaReference {
            arena: binding.arena.clone(),
            context: format!("need_bindings.{}", binding.id),
        }));
    };

    let participant_hosted = resolve_unique_entity(scenario, &binding.participant, &binding.id)?;
    let raw = participant_hosted.raw();
    if !arena
        .explicit_participants
        .iter()
        .any(|p| p.subtree_root_id == raw)
    {
        return Err(InstallError::NeedBindingInvalid {
            binding: binding.id.clone(),
            reason: format!(
                "participant entity `{}` is not admitted to arena `{}`",
                binding.participant, arena.name
            ),
        });
    }
    let participant_slot = scaffold
        .index
        .participant_slot(participant_hosted, arena_idx as u32)
        .ok_or_else(|| InstallError::NeedBindingInvalid {
            binding: binding.id.clone(),
            reason: "participant has no arena wrapper slot".into(),
        })?
        .raw();
    let participant_wrapper_id = allocator
        .owner_of(SlotIndex::new(participant_slot))
        .ok_or_else(|| InstallError::NeedBindingInvalid {
            binding: binding.id.clone(),
            reason: "participant slot has no owner".into(),
        })?;

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
    // Fail closed: wrapper must already own the flow property (no invent).
    if !entity_has_property(root, participant_wrapper_id, flow_property_id) {
        return Err(InstallError::NeedBindingInvalid {
            binding: binding.id.clone(),
            reason: format!(
                "arena participant wrapper for `{}` does not already own flow property {}::{}",
                binding.participant, arena.flow_property.namespace, arena.flow_property.name
            ),
        });
    }
    let flow_layout = &registry.property(flow_property_id).layout;
    let _ = resolve_node_columns(flow_layout, &arena.name).map_err(|_| {
        InstallError::Spec(SpecError::UnknownResourceFlowProperty {
            property: format!("{} flow columns", arena.name),
        })
    })?;
    let need_col = registry
        .column_range(flow_property_id)
        .col_for_role(&SubFieldRole::Named("weight".into()), flow_layout)
        .ok_or_else(|| InstallError::NeedBindingInvalid {
            binding: binding.id.clone(),
            reason: "flow property missing AllocatorWeight Named(\"weight\")".into(),
        })?;

    let inputs = resolve_loci(
        &binding.inputs,
        scenario,
        root,
        registry,
        allocator,
        &binding.id,
    )?;
    let weights = resolve_loci(
        &binding.weights,
        scenario,
        root,
        registry,
        allocator,
        &binding.id,
    )?;

    // EvalEML is slot-local: all sources must share one row.
    let eml_source_slot = inputs[0].slot;
    for cell in inputs.iter().chain(weights.iter()) {
        if cell.slot != eml_source_slot {
            return Err(InstallError::NeedBindingInvalid {
                binding: binding.id.clone(),
                reason: format!(
                    "input/weight sources span multiple entity rows (slot {} vs {})",
                    eml_source_slot, cell.slot
                ),
            });
        }
    }

    let input_cols: Vec<ColumnIndex> = inputs.iter().map(|c| c.col).collect();
    let weight_cols: Vec<ColumnIndex> = weights.iter().map(|c| c.col).collect();
    let nodes = build_weighted_need_nodes(&input_cols, &weight_cols);

    Ok(ResolvedNeedBinding {
        id: binding.id.clone(),
        profile: binding.profile.clone(),
        participant_slot,
        participant_id: participant_wrapper_id,
        eml_source_slot,
        need_col,
        inputs,
        weights,
        nodes,
        threshold: binding.threshold,
        event_kind: binding.event_kind,
    })
}

fn resolve_unique_entity(
    scenario: &Scenario,
    entity: &str,
    binding_id: &str,
) -> Result<SimThingId, InstallError> {
    let Some(ids) = scenario.install_targets.get(entity) else {
        return Err(InstallError::NeedBindingInvalid {
            binding: binding_id.into(),
            reason: format!("entity `{entity}` is not defined in scenario install_targets"),
        });
    };
    if ids.is_empty() {
        return Err(InstallError::NeedBindingInvalid {
            binding: binding_id.into(),
            reason: format!("entity `{entity}` has zero install hosts"),
        });
    }
    if ids.len() != 1 {
        return Err(InstallError::NeedBindingInvalid {
            binding: binding_id.into(),
            reason: format!(
                "entity `{entity}` is ambiguous ({} hosts); entity-name uniqueness required",
                ids.len()
            ),
        });
    }
    Ok(ids[0])
}

fn resolve_loci(
    loci: &[SemanticPropertyLocusSpec],
    scenario: &Scenario,
    root: &SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    binding_id: &str,
) -> Result<Vec<ResolvedFullCell>, InstallError> {
    let mut out = Vec::with_capacity(loci.len());
    for locus in loci {
        let simthing_id = resolve_unique_entity(scenario, &locus.entity, binding_id)?;
        let prop_id = registry
            .id_of(&locus.property.namespace, &locus.property.name)
            .ok_or_else(|| {
                InstallError::Spec(SpecError::UnknownResourceFlowProperty {
                    property: format!("{}::{}", locus.property.namespace, locus.property.name),
                })
            })?;
        if !entity_has_property(root, simthing_id, prop_id) {
            return Err(InstallError::NeedBindingInvalid {
                binding: binding_id.into(),
                reason: format!(
                    "entity `{}` does not own property {}::{}",
                    locus.entity, locus.property.namespace, locus.property.name
                ),
            });
        }
        let layout = &registry.property(prop_id).layout;
        let col = registry
            .column_range(prop_id)
            .col_for_role(&locus.role, layout)
            .ok_or_else(|| InstallError::NeedBindingInvalid {
                binding: binding_id.into(),
                reason: format!(
                    "property {}::{} missing role {:?}",
                    locus.property.namespace, locus.property.name, locus.role
                ),
            })?;
        let slot = allocator.slot_of(simthing_id).ok_or_else(|| {
            InstallError::NeedBindingInvalid {
                binding: binding_id.into(),
                reason: format!("entity `{}` has no GPU slot", locus.entity),
            }
        })?.raw();
        out.push(ResolvedFullCell {
            entity: locus.entity.clone(),
            simthing_id,
            slot,
            col,
            role: locus.role.clone(),
        });
    }
    Ok(out)
}

fn entity_has_property(root: &SimThing, id: SimThingId, prop_id: simthing_core::SimPropertyId) -> bool {
    if root.id == id {
        return root.properties.contains_key(&prop_id);
    }
    for child in &root.children {
        if entity_has_property(child, id, prop_id) {
            return true;
        }
    }
    false
}

fn build_weighted_need_nodes(
    input_cols: &[ColumnIndex],
    weight_cols: &[ColumnIndex],
) -> Vec<EmlNodeGpu> {
    let mut nodes = Vec::new();
    for (in_col, w_col) in input_cols.iter().zip(weight_cols.iter()) {
        nodes.push(EmlNodeGpu {
            opcode: eml_nodes::opcode::SLOT_VALUE,
            flags: 0,
            a: in_col.raw_u32(),
            b: 0,
            c: 0,
            d: 0,
        });
        nodes.push(EmlNodeGpu {
            opcode: eml_nodes::opcode::SLOT_VALUE,
            flags: 0,
            a: w_col.raw_u32(),
            b: 0,
            c: 0,
            d: 0,
        });
        nodes.push(EmlNodeGpu {
            opcode: eml_nodes::opcode::MUL,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        });
    }
    for _ in 1..input_cols.len() {
        nodes.push(EmlNodeGpu {
            opcode: eml_nodes::opcode::ADD,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        });
    }
    nodes.push(EmlNodeGpu {
        opcode: eml_nodes::opcode::RETURN_TOP,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    });
    nodes
}

/// Zero need cell only if already present — never invent property instances.
pub fn prepare_need_binding_cells(
    resolved: &[ResolvedNeedBinding],
    registry: &DimensionRegistry,
    root: &mut SimThing,
) -> Result<(), InstallError> {
    for binding in resolved {
        let Some(wrapper) = find_simthing_mut(root, binding.participant_id) else {
            return Err(InstallError::NeedBindingInvalid {
                binding: binding.id.clone(),
                reason: "participant wrapper missing".into(),
            });
        };
        let Some(flow_pid) = registry
            .column_owners
            .get(binding.need_col.raw())
            .map(|(p, _)| *p)
        else {
            return Err(InstallError::NeedBindingInvalid {
                binding: binding.id.clone(),
                reason: "need column has no property owner in registry".into(),
            });
        };
        let Some(value) = wrapper.properties.get_mut(&flow_pid) else {
            return Err(InstallError::NeedBindingInvalid {
                binding: binding.id.clone(),
                reason: "participant flow property missing (no install-time invent)".into(),
            });
        };
        let layout = registry.property(flow_pid).layout.clone();
        value.set_role(&SubFieldRole::Named("weight".into()), &layout, 0.0);
    }
    Ok(())
}

pub fn build_need_binding_ops(
    resolved: &[ResolvedNeedBinding],
    eml_registry: &mut EmlExpressionRegistry,
) -> Vec<AccumulatorOp> {
    let mut ops = Vec::with_capacity(resolved.len());
    for (idx, binding) in resolved.iter().enumerate() {
        let tree_id = EmlTreeId(NEED_BINDING_TREE_BASE + idx as u32);
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
                    display_name: format!("need_binding_{}", binding.id),
                },
                binding.nodes.clone(),
            )
            .expect("need_binding formula registers");
        let source_col = binding
            .weights
            .first()
            .map(|c| c.col)
            .unwrap_or(binding.need_col);
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(binding.eml_source_slot),
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

pub fn inject_need_binding_thresholds(
    resolved: &[ResolvedNeedBinding],
    economy: &mut ResourceEconomyRegistry,
) {
    let mut injected = false;
    for binding in resolved {
        let (Some(threshold), Some(event_kind)) = (binding.threshold, binding.event_kind) else {
            continue;
        };
        let reg = EmitOnThresholdRegistration {
            slot: SlotIndex::new(binding.participant_slot),
            col: binding.need_col,
            threshold,
            direction: ThresholdDirection::Upward,
            event_kind,
            buffer: EmitOnThresholdBuffer::Values,
        };
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

pub fn register_post_rf_need_threshold_rescan(
    state: &mut simthing_gpu::WorldGpuState,
    resolved: &[ResolvedNeedBinding],
) {
    use simthing_gpu::{DIR_UPWARD, THRESH_BUF_VALUES, ThresholdRegistration};
    let need_regs: Vec<ThresholdRegistration> = resolved
        .iter()
        .filter_map(|b| {
            let threshold = b.threshold?;
            let event_kind = b.event_kind?;
            Some(ThresholdRegistration {
                slot: b.participant_slot,
                col: b.need_col.raw_u32(),
                threshold,
                direction: DIR_UPWARD,
                event_kind,
                buffer: THRESH_BUF_VALUES,
            })
        })
        .collect();
    state.set_post_rf_need_threshold_regs(need_regs);
}
