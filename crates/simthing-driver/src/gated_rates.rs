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
    eml_nodes, AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, DimensionRegistry,
    EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu,
    EmlTreeId, GateSpec, RoleOffset, ScaleSpec, SimThing, SlotIndex, SourceSpec, SubFieldRole,
    MAX_EML_TREE_NODES,
};
use simthing_spec::{
    GatedRateOpSpec, RateFormulaOp, RateFormulaOperandSpec, ResourceFlowSpec, SpecError,
};
use std::collections::BTreeMap;

use crate::arena_hierarchy::resolve_node_columns_for_property;
use crate::arena_registry::ArenaRegistry;
use crate::install::{find_simthing_mut, resolve_install_target, InstallError};
use crate::scenario::Scenario;

/// Sub-field carrying the install-folded static rate the EML band reads.
pub const RATE_BASE_SUB_FIELD: &str = "rate_base";

/// Dedicated tree-id range for gated-rate formulas (clear of the child-share
/// formulas the arena sync registers).
const GATED_RATE_TREE_BASE: u32 = 7_100_000;

/// One fully resolved dynamic rate term, ready for tree building and seeding.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedGatedRate {
    pub id: String,
    pub participant_slot: u32,
    /// Local data offsets within the flow property (node seeding).
    pub base_offset: RoleOffset,
    pub intrinsic_offset: RoleOffset,
    /// Role-pathway columns (EML SLOT_VALUE / op targets). Never reminted raw.
    pub base_col: ColumnIndex,
    pub intrinsic_col: ColumnIndex,
    /// `(trigger_col, at_least)` for gated terms; `None` = always-on dynamic.
    pub trigger: Option<(ColumnIndex, f32)>,
    pub magnitude: ResolvedMagnitude,
    pub is_mult: bool,
}

/// Per-tick term magnitude: a signed literal, or a resolved `value:` formula
/// chain whose property operands are already global columns.
#[derive(Clone, Debug, PartialEq)]
pub enum ResolvedMagnitude {
    Literal(f32),
    Formula {
        base: f32,
        ops: Vec<(RateFormulaOp, ResolvedOperand)>,
        /// Upkeep add-formulas negate after evaluation (bit-exact sign flip).
        negate: bool,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResolvedOperand {
    Literal(f32),
    Column(ColumnIndex),
}

/// Resolve every authored gated rate against the live install: arena →
/// admitted participant slot, flow-property base/intrinsic columns, and the
/// trigger property's Amount column. Everything unresolvable is a hard error.
pub fn resolve_gated_rates(
    spec: &ResourceFlowSpec,
    scenario: &Scenario,
    root: &SimThing,
    registry: &DimensionRegistry,
    arena_registry: &ArenaRegistry,
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
        let cols = resolve_node_columns_for_property(registry, flow_property_id, &arena.name)
            .map_err(|_| {
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
        let range = registry.column_range(flow_property_id);
        let base_col = range.col_for_role(&base_role, layout).ok_or_else(|| {
            InstallError::GatedRateMissingBaseColumn {
                gated: gated.id.clone(),
                arena: gated.arena.clone(),
            }
        })?;
        let intrinsic_col = cols.intrinsic_flow_col;

        let amount_col =
            |key: &simthing_spec::PropertyKey, what: &str| -> Result<ColumnIndex, InstallError> {
                let property_id = registry.id_of(&key.namespace, &key.name).ok_or_else(|| {
                    InstallError::GatedRateUnknownTriggerProperty {
                        gated: gated.id.clone(),
                        property: format!("{}::{} ({what})", key.namespace, key.name),
                    }
                })?;
                let property_layout = &registry.property(property_id).layout;
                registry
                    .column_range(property_id)
                    .col_for_role(&SubFieldRole::Amount, property_layout)
                    .ok_or_else(|| InstallError::GatedRateUnknownTriggerProperty {
                        gated: gated.id.clone(),
                        property: format!("{what} Amount sub-field"),
                    })
            };

        let trigger = match &gated.trigger {
            Some(trigger) => Some((amount_col(&trigger.property, "trigger")?, trigger.at_least)),
            None => None,
        };

        let is_mult = matches!(gated.op, GatedRateOpSpec::Mult);
        let magnitude = match &gated.rate_formula {
            Some(formula) => {
                let mut ops = Vec::with_capacity(formula.ops.len());
                for op in &formula.ops {
                    let operand = match &op.operand {
                        RateFormulaOperandSpec::Literal(value) => ResolvedOperand::Literal(*value),
                        RateFormulaOperandSpec::Property(key) => {
                            ResolvedOperand::Column(amount_col(key, "formula operand")?)
                        }
                    };
                    ops.push((op.op, operand));
                }
                ResolvedMagnitude::Formula {
                    base: formula.base,
                    ops,
                    negate: !is_mult && gated.direction.sign() < 0.0,
                }
            }
            None => ResolvedMagnitude::Literal(if is_mult {
                gated.rate
            } else {
                gated.direction.sign() * gated.rate
            }),
        };

        let hosted = resolve_install_target(&gated.install, scenario, root)?;
        if hosted.is_empty() {
            return Err(InstallError::NoMatchingOwners {
                tree_id: gated.id.clone(),
                target: gated.install.clone(),
            });
        }
        for hosted_id in hosted {
            let participant_slot = arena_registry
                .participant_slot(hosted_id, arena_idx as u32)
                .ok_or_else(|| InstallError::BaseFlowObligationTargetNotAdmitted {
                    obligation: gated.id.clone(),
                    arena: gated.arena.clone(),
                    subtree_root_id: hosted_id.raw(),
                })?;
            let intrinsic_offset =
                crate::install::intrinsic_flow_offset(registry, flow_property_id).ok_or_else(
                    || InstallError::GatedRateMissingBaseColumn {
                        gated: gated.id.clone(),
                        arena: gated.arena.clone(),
                    },
                )?;
            out.push(ResolvedGatedRate {
                id: gated.id.clone(),
                participant_slot: participant_slot.raw(),
                base_offset,
                intrinsic_offset,
                base_col,
                intrinsic_col,
                trigger,
                magnitude: magnitude.clone(),
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
        let participant_id = allocator
            .owner_of(SlotIndex::new(gated.participant_slot))
            .ok_or_else(|| InstallError::BaseFlowObligationParticipantSlotMissing {
                obligation: gated.id.clone(),
                arena: String::new(),
                slot: gated.participant_slot,
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
            .get(gated.intrinsic_col.raw())
            .map(|(pid, _)| *pid)
            .ok_or(InstallError::Spec(SpecError::ValidationFailedAt {
                site: "simthing-driver/gated_rates",
            }))?;
        let Some(value) = node.properties.get_mut(&flow_property_id) else {
            return Err(InstallError::Spec(SpecError::ValidationFailedAt {
                site: "simthing-driver/gated_rates",
            }));
        };
        let intrinsic = value.lane_at_offset(gated.intrinsic_offset);
        value.set_lane_at_offset(gated.base_offset, intrinsic);
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

fn slot_value(col: ColumnIndex) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_nodes::opcode::SLOT_VALUE,
        flags: 0,
        a: col.raw_u32(),
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

fn push_operand(nodes: &mut Vec<EmlNodeGpu>, operand: &ResolvedOperand) {
    match operand {
        ResolvedOperand::Literal(value) => nodes.push(literal(*value)),
        ResolvedOperand::Column(col) => nodes.push(slot_value(*col)),
    }
}

/// Push the term's magnitude — a literal, or the `value:` formula chain
/// (`base`, then ordered Add/Mult/FloorAt/CeilAt; FloorAt = "at least" →
/// `MAX`, CeilAt = "at most" → `MIN`).
fn push_magnitude(nodes: &mut Vec<EmlNodeGpu>, magnitude: &ResolvedMagnitude) {
    match magnitude {
        ResolvedMagnitude::Literal(value) => nodes.push(literal(*value)),
        ResolvedMagnitude::Formula { base, ops, negate } => {
            nodes.push(literal(*base));
            for (op, operand) in ops {
                push_operand(nodes, operand);
                nodes.push(op_node(match op {
                    RateFormulaOp::Add => eml_nodes::opcode::ADD,
                    RateFormulaOp::Mult => eml_nodes::opcode::MUL,
                    RateFormulaOp::FloorAt => eml_nodes::opcode::MAX,
                    RateFormulaOp::CeilAt => eml_nodes::opcode::MIN,
                }));
            }
            if *negate {
                nodes.push(op_node(eml_nodes::opcode::NEG));
            }
        }
    }
}

/// Push one dynamic term and fold it into the running sum: `… magnitude
/// [× gate] ADD`. Ungated terms contribute unconditionally.
fn push_term(nodes: &mut Vec<EmlNodeGpu>, term: &ResolvedGatedRate) {
    push_magnitude(nodes, &term.magnitude);
    if let Some((trigger_col, at_least)) = term.trigger {
        nodes.push(slot_value(trigger_col));
        nodes.push(literal(at_least));
        nodes.push(op_node(eml_nodes::opcode::CMP_GE));
        nodes.push(op_node(eml_nodes::opcode::MUL));
    }
    nodes.push(op_node(eml_nodes::opcode::ADD));
}

/// Build one effective-rate `EvalEML` tree + op per (participant, intrinsic
/// column) group, registered `ExactDeterministic` and gated at OrderBand 0.
pub fn build_gated_rate_ops(
    resolved: &[ResolvedGatedRate],
    eml_registry: &mut EmlExpressionRegistry,
) -> Vec<AccumulatorOp> {
    let mut groups: BTreeMap<(u32, usize), Vec<&ResolvedGatedRate>> = BTreeMap::new();
    for gated in resolved {
        groups
            .entry((gated.participant_slot, gated.intrinsic_col.raw()))
            .or_default()
            .push(gated);
    }

    let mut ops = Vec::with_capacity(groups.len());
    for (group_idx, ((slot, _), terms)) in groups.into_iter().enumerate() {
        let base_col = terms[0].base_col;
        let intrinsic_col = terms[0].intrinsic_col;
        let mut nodes = vec![slot_value(base_col)];
        for term in terms.iter().filter(|t| !t.is_mult) {
            push_term(&mut nodes, term);
        }
        nodes.push(literal(1.0));
        for term in terms.iter().filter(|t| t.is_mult) {
            push_term(&mut nodes, term);
        }
        nodes.push(op_node(eml_nodes::opcode::MUL));
        nodes.push(op_node(eml_nodes::opcode::RETURN_TOP));

        debug_assert!(
            (nodes.len() as u32) <= MAX_EML_TREE_NODES,
            "gated-rate tree must inherit the one existing per-program cap ({MAX_EML_TREE_NODES})"
        );

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
            .expect("gated effective-rate formula registers on the ordinary EML library");

        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(slot),
                col: base_col,
            },
            combine: CombineFn::EvalEML { tree_id: tree_id.0 },
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(SlotIndex::new(slot), intrinsic_col)],
        });
    }
    ops
}
