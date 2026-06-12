//! CT-RF-EML-RATE-0: per-tick `EvalEML` effective-rate band.
//!
//! Trigger-gated rate contributions compile to one `EvalEML` tree per
//! (participant, flow pair):
//!
//! ```text
//! intrinsic = (base + Σ add_rate × gate) × (1 + Σ mult_rate × gate)
//! gate      = trigger_column ≥ at_least  (1.0 / 0.0)
//! ```
//!
//! The ops run on a dedicated OrderBand **before** every arena reduce band
//! (the sync shifts arena bands up by one), recomputing the intrinsic column
//! from the immutable base column each tick — rising *and* falling trigger
//! edges are exact by construction, and per-tick transforms directly on rate
//! columns (which compound) are impossible here.

use simthing_core::{
    eml_nodes, AccumulatorOp, CombineFn, ConsumeMode, DimensionRegistry, EmlConsumerMask,
    EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec,
    ScaleSpec, SimThing, SourceSpec, SubFieldRole,
};
use simthing_spec::{GatedRateOpSpec, GatedRateSpec, ResourceFlowSpec, SpecError};
use std::collections::BTreeMap;

use crate::arena_hierarchy::resolve_node_columns;
use crate::arena_participant::ArenaParticipantScaffold;
use crate::install::{find_simthing_mut, resolve_install_target, InstallError};
use crate::scenario::Scenario;

/// Sub-field carrying the install-folded static rate the EML band reads.
pub const RATE_BASE_SUB_FIELD: &str = "rate_base";

/// Dedicated tree-id range for gated-rate formulas (clear of the child-share
/// formulas the arena sync registers).
const GATED_RATE_TREE_BASE: u32 = 7_100_000;

/// One fully resolved gated rate term, ready for tree building and seeding.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedGatedRate {
    pub id: String,
    pub participant_slot: u32,
    /// Local data offsets within the flow property (node seeding).
    pub base_offset: usize,
    pub intrinsic_offset: usize,
    /// Global columns (EML SLOT_VALUE / op targets).
    pub base_col: u32,
    pub intrinsic_col: u32,
    pub trigger_col: u32,
    pub at_least: f32,
    /// Add terms carry direction sign; mult terms carry the raw fraction.
    pub rate: f32,
    pub is_mult: bool,
}

/// Resolve every authored gated rate against the live install: arena →
/// admitted participant slot, flow-property base/intrinsic columns, and the
/// trigger property's Amount column. Everything unresolvable is a hard error.
pub fn resolve_gated_rates(
    spec: &ResourceFlowSpec,
    scenario: &Scenario,
    root: &SimThing,
    registry: &DimensionRegistry,
    scaffold: &ArenaParticipantScaffold,
) -> Result<Vec<ResolvedGatedRate>, InstallError> {
    let mut out = Vec::with_capacity(spec.gated_rates.len());
    for gated in &spec.gated_rates {
        let (arena_idx, arena) = spec
            .arenas
            .iter()
            .enumerate()
            .find(|(_, arena)| arena.name == gated.arena)
            .ok_or_else(|| SpecError::UnknownArenaReference {
                arena: gated.arena.clone(),
                context: format!("gated_rates.{}", gated.id),
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
        let layout = &registry.property(flow_property_id).layout;
        let cols = resolve_node_columns(layout, &arena.name).map_err(|_| {
            InstallError::Spec(SpecError::UnknownResourceFlowProperty {
                property: format!("{} flow columns", arena.name),
            })
        })?;
        let base_role = SubFieldRole::Named(RATE_BASE_SUB_FIELD.into());
        let base_offset = layout.offset_of(&base_role).ok_or_else(|| {
            InstallError::GatedRateMissingBaseColumn {
                gated: gated.id.clone(),
                arena: gated.arena.clone(),
            }
        })?;
        let flow_start = registry.column_range(flow_property_id).start as u32;

        let trigger_id = registry
            .id_of(
                &gated.trigger.property.namespace,
                &gated.trigger.property.name,
            )
            .ok_or_else(|| InstallError::GatedRateUnknownTriggerProperty {
                gated: gated.id.clone(),
                property: format!(
                    "{}::{}",
                    gated.trigger.property.namespace, gated.trigger.property.name
                ),
            })?;
        let trigger_layout = &registry.property(trigger_id).layout;
        let trigger_col = registry
            .column_range(trigger_id)
            .col_for_role(&SubFieldRole::Amount, trigger_layout)
            .ok_or_else(|| InstallError::GatedRateUnknownTriggerProperty {
                gated: gated.id.clone(),
                property: "trigger Amount sub-field".into(),
            })? as u32;

        let hosted = resolve_install_target(&gated.install, scenario, root)?;
        if hosted.is_empty() {
            return Err(InstallError::NoMatchingOwners {
                tree_id: gated.id.clone(),
                target: gated.install.clone(),
            });
        }
        let (is_mult, rate) = match gated.op {
            GatedRateOpSpec::Add => (false, gated.direction.sign() * gated.rate),
            GatedRateOpSpec::Mult => (true, gated.rate),
        };
        for hosted_id in hosted {
            let participant_slot = scaffold
                .index
                .participant_slot(hosted_id, arena_idx as u32)
                .ok_or_else(|| InstallError::BaseFlowObligationTargetNotAdmitted {
                    obligation: gated.id.clone(),
                    arena: gated.arena.clone(),
                    subtree_root_id: hosted_id.raw(),
                })?;
            out.push(ResolvedGatedRate {
                id: gated.id.clone(),
                participant_slot,
                base_offset,
                intrinsic_offset: cols.intrinsic_flow_col as usize,
                base_col: flow_start + base_offset as u32,
                intrinsic_col: flow_start + cols.intrinsic_flow_col,
                trigger_col,
                at_least: gated.trigger.at_least,
                rate,
                is_mult,
            });
        }
    }
    Ok(out)
}

/// Copy each gated participant's install-folded intrinsic rate into the base
/// column the EML band recomputes from. Runs after `seed_base_flow_obligations`.
pub fn seed_gated_rate_base_columns(
    resolved: &[ResolvedGatedRate],
    registry: &DimensionRegistry,
    root: &mut SimThing,
    allocator: &simthing_gpu::SlotAllocator,
) -> Result<(), InstallError> {
    for gated in resolved {
        let participant_id = allocator.owner_of(gated.participant_slot).ok_or_else(|| {
            InstallError::BaseFlowObligationParticipantSlotMissing {
                obligation: gated.id.clone(),
                arena: String::new(),
                slot: gated.participant_slot,
            }
        })?;
        let Some(node) = find_simthing_mut(root, participant_id) else {
            return Err(InstallError::BaseFlowObligationParticipantSlotMissing {
                obligation: gated.id.clone(),
                arena: String::new(),
                slot: gated.participant_slot,
            });
        };
        let flow_property_id = registry
            .column_owners
            .get(gated.intrinsic_col as usize)
            .map(|(pid, _)| *pid)
            .ok_or(InstallError::Spec(SpecError::ValidationFailed))?;
        let Some(value) = node.properties.get_mut(&flow_property_id) else {
            return Err(InstallError::Spec(SpecError::ValidationFailed));
        };
        value.data[gated.base_offset] = value.data[gated.intrinsic_offset];
    }
    Ok(())
}

fn literal(value: f32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_nodes::opcode::LITERAL_F32,
        flags: 0,
        a: value.to_bits(),
        b: 0,
        c: 0,
        d: 0,
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

fn gate_term(nodes: &mut Vec<EmlNodeGpu>, term: &ResolvedGatedRate) {
    nodes.push(slot_value(term.trigger_col));
    nodes.push(literal(term.at_least));
    nodes.push(op_node(eml_nodes::opcode::CMP_GE));
    nodes.push(literal(term.rate));
    nodes.push(op_node(eml_nodes::opcode::MUL));
    nodes.push(op_node(eml_nodes::opcode::ADD));
}

/// Build one effective-rate `EvalEML` tree + op per (participant, intrinsic
/// column) group, registered `ExactDeterministic` and gated at OrderBand 0.
pub fn build_gated_rate_ops(
    resolved: &[ResolvedGatedRate],
    eml_registry: &mut EmlExpressionRegistry,
) -> Vec<AccumulatorOp> {
    let mut groups: BTreeMap<(u32, u32), Vec<&ResolvedGatedRate>> = BTreeMap::new();
    for gated in resolved {
        groups
            .entry((gated.participant_slot, gated.intrinsic_col))
            .or_default()
            .push(gated);
    }

    let mut ops = Vec::with_capacity(groups.len());
    for (group_idx, ((slot, intrinsic_col), terms)) in groups.into_iter().enumerate() {
        let base_col = terms[0].base_col;
        let mut nodes = vec![slot_value(base_col)];
        for term in terms.iter().filter(|t| !t.is_mult) {
            gate_term(&mut nodes, term);
        }
        nodes.push(literal(1.0));
        for term in terms.iter().filter(|t| t.is_mult) {
            gate_term(&mut nodes, term);
        }
        nodes.push(op_node(eml_nodes::opcode::MUL));
        nodes.push(op_node(eml_nodes::opcode::RETURN_TOP));

        let tree_id = EmlTreeId(GATED_RATE_TREE_BASE + group_idx as u32);
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
                    node_count: nodes.len() as u32,
                    max_stack_depth: 0,
                    has_loops: false,
                    has_recursion: false,
                    display_name: "gated_effective_rate".into(),
                },
                nodes,
            )
            .expect("gated effective-rate formula registers (exact-class opcodes, ≤32 nodes)");

        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot,
                col: base_col,
            },
            combine: CombineFn::EvalEML { tree_id: tree_id.0 },
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(slot, intrinsic_col)],
        });
    }
    ops
}
