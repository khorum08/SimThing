//! RF-5A: admit semantic need_binding → full-cell EvalEML + sealed threshold.
//!
//! Row authority = **entity-name uniqueness** (install_targets). PropertyKey alone
//! is never a row. No first-DFS owner, no participant property invent, no binding
//! re-home of World-root stockpile state.

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
    pub threshold: f32,
    pub event_kind: u32,
}

fn nb_err(
    binding: &NeedBindingSpec,
    reason: impl Into<String>,
    span: Option<usize>,
) -> InstallError {
    InstallError::NeedBindingInvalid {
        binding: binding.id.clone(),
        reason: reason.into(),
        span_token: span.or(binding.source_span_token),
    }
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
        return Err(nb_err(binding, "inputs and weights required", None));
    }
    if binding.inputs.len() != binding.weights.len() {
        return Err(nb_err(binding, "input/weight arity mismatch", None));
    }
    if binding.stack.gadgets.len() != 1 {
        return Err(nb_err(
            binding,
            "stack must contain exactly one WeightedAccumulator",
            None,
        ));
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
                return Err(nb_err(
                    binding,
                    "stack arity vs input/weight loci mismatch",
                    None,
                ));
            }
        }
        other => {
            return Err(nb_err(
                binding,
                format!("expected WeightedAccumulator, got {}", other.kind_name()),
                None,
            ));
        }
    }

    // Explicit arena only — no first-arena fallback.
    let (arena_idx, arena) = spec
        .arenas
        .iter()
        .enumerate()
        .find(|(_, a)| a.name == binding.arena)
        .ok_or_else(|| {
            nb_err(
                binding,
                format!(
                    "arena `{}` not found (explicit arena required; no first-arena guess)",
                    binding.arena
                ),
                binding.arena_span_token,
            )
        })?;

    let participant_hosted =
        resolve_unique_entity(scenario, &binding.participant, binding, binding.participant_span_token)?;
    let raw = participant_hosted.raw();
    if !arena
        .explicit_participants
        .iter()
        .any(|p| p.subtree_root_id == raw)
    {
        return Err(nb_err(
            binding,
            format!(
                "participant entity `{}` is not admitted to arena `{}`",
                binding.participant, arena.name
            ),
            binding.participant_span_token,
        ));
    }
    let participant_slot = scaffold
        .index
        .participant_slot(participant_hosted, arena_idx as u32)
        .ok_or_else(|| {
            nb_err(
                binding,
                "participant has no arena wrapper slot",
                binding.participant_span_token,
            )
        })?
        .raw();
    let participant_wrapper_id = allocator
        .owner_of(SlotIndex::new(participant_slot))
        .ok_or_else(|| {
            nb_err(
                binding,
                "participant slot has no owner",
                binding.participant_span_token,
            )
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
    if !entity_has_property(root, participant_wrapper_id, flow_property_id) {
        return Err(nb_err(
            binding,
            format!(
                "arena participant wrapper for `{}` does not already own flow property {}::{} (no install invent)",
                binding.participant, arena.flow_property.namespace, arena.flow_property.name
            ),
            binding.participant_span_token,
        ));
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
        .ok_or_else(|| {
            nb_err(
                binding,
                "flow property missing AllocatorWeight Named(\"weight\")",
                binding.participant_span_token,
            )
        })?;

    let inputs = resolve_loci(
        &binding.inputs,
        scenario,
        root,
        registry,
        allocator,
        binding,
    )?;
    let weights = resolve_loci(
        &binding.weights,
        scenario,
        root,
        registry,
        allocator,
        binding,
    )?;

    // EvalEML is slot-local. Cross-entity loci need multi-slot EML (not admitted).
    let eml_source_slot = inputs[0].slot;
    for cell in inputs.iter().chain(weights.iter()) {
        if cell.slot != eml_source_slot {
            return Err(nb_err(
                binding,
                format!(
                    "STOP: input/weight sources span multiple entity rows (slot {} vs {}). \
                     DA-admitted multi-locus shape requires multi-slot EvalEML or OrderBand \
                     product without unowned mirrors — not available; all loci must share one entity for RF-5A",
                    eml_source_slot, cell.slot
                ),
                None,
            ));
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
    binding: &NeedBindingSpec,
    span: Option<usize>,
) -> Result<SimThingId, InstallError> {
    let Some(ids) = scenario.install_targets.get(entity) else {
        return Err(nb_err(
            binding,
            format!("entity `{entity}` is not defined in scenario install_targets"),
            span,
        ));
    };
    if ids.is_empty() {
        return Err(nb_err(
            binding,
            format!("entity `{entity}` has zero install hosts"),
            span,
        ));
    }
    if ids.len() != 1 {
        return Err(nb_err(
            binding,
            format!(
                "entity `{entity}` is ambiguous ({} hosts); entity-name uniqueness required",
                ids.len()
            ),
            span,
        ));
    }
    Ok(ids[0])
}

fn resolve_loci(
    loci: &[SemanticPropertyLocusSpec],
    scenario: &Scenario,
    root: &SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    binding: &NeedBindingSpec,
) -> Result<Vec<ResolvedFullCell>, InstallError> {
    let mut out = Vec::with_capacity(loci.len());
    for locus in loci {
        let simthing_id =
            resolve_unique_entity(scenario, &locus.entity, binding, locus.source_span_token)?;
        let prop_id = registry
            .id_of(&locus.property.namespace, &locus.property.name)
            .ok_or_else(|| {
                nb_err(
                    binding,
                    format!(
                        "property {}::{} not registered",
                        locus.property.namespace, locus.property.name
                    ),
                    locus.source_span_token,
                )
            })?;
        if !entity_has_property(root, simthing_id, prop_id) {
            return Err(nb_err(
                binding,
                format!(
                    "entity `{}` does not own property {}::{} (no binding invent/re-home)",
                    locus.entity, locus.property.namespace, locus.property.name
                ),
                locus.source_span_token,
            ));
        }
        let layout = &registry.property(prop_id).layout;
        let col = registry
            .column_range(prop_id)
            .col_for_role(&locus.role, layout)
            .ok_or_else(|| {
                nb_err(
                    binding,
                    format!(
                        "property {}::{} missing role {:?}",
                        locus.property.namespace, locus.property.name, locus.role
                    ),
                    locus.source_span_token,
                )
            })?;
        let slot = allocator.slot_of(simthing_id).ok_or_else(|| {
            nb_err(
                binding,
                format!("entity `{}` has no GPU slot", locus.entity),
                locus.source_span_token,
            )
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
                span_token: None,
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
                span_token: None,
            });
        };
        let Some(value) = wrapper.properties.get_mut(&flow_pid) else {
            return Err(InstallError::NeedBindingInvalid {
                binding: binding.id.clone(),
                reason: "participant flow property missing (no install-time invent)".into(),
                span_token: None,
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
        let reg = EmitOnThresholdRegistration {
            slot: SlotIndex::new(binding.participant_slot),
            col: binding.need_col,
            threshold: binding.threshold,
            direction: ThresholdDirection::Upward,
            event_kind: binding.event_kind,
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
) -> Result<(), String> {
    use simthing_gpu::{DIR_UPWARD, THRESH_BUF_VALUES, ThresholdRegistration};
    let need_regs: Vec<ThresholdRegistration> = resolved
        .iter()
        .map(|b| ThresholdRegistration {
            slot: b.participant_slot,
            col: b.need_col.raw_u32(),
            threshold: b.threshold,
            direction: DIR_UPWARD,
            event_kind: b.event_kind,
            buffer: THRESH_BUF_VALUES,
        })
        .collect();
    state.set_post_rf_need_threshold_regs(need_regs);
    Ok(())
}
