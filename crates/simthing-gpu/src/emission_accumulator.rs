//! C-8d emission substrate planner → AccumulatorOp.

use simthing_core::{
    AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, EmlConsumerKind, EmlExpressionRegistry,
    EmlTreeId, GateSpec, ScaleSpec, SlotIndex, SourceSpec,
};

use crate::{AccumulatorOpGpu, EncodeError};

/// Formula kind tags for [`EmissionOpPlanSignature::formula_kinds`].
pub const FORMULA_KIND_IDENTITY_FLOOR: u32 = 0;
pub const FORMULA_KIND_CONSTANT: u32 = 1;
pub const FORMULA_KIND_EVAL_EML: u32 = 2;

/// Sentinel values for [`EmissionOpPlanSignature`] optional fields.
pub const NO_TREE_ID: u32 = u32::MAX;
pub const NO_MAX_EMIT: u32 = u32::MAX;
pub const NO_CONSTANT: u32 = 0;

#[derive(Clone, Debug, PartialEq)]
pub enum EmissionFormula {
    IdentityFloor,
    EvalEml { tree_id: EmlTreeId },
    Constant { value: f32 },
}

#[derive(Clone, Debug, PartialEq)]
pub struct EmissionRegistration {
    pub source_slot: u32,
    pub source_col: u32,
    pub tree_id: Option<EmlTreeId>,
    pub formula: EmissionFormula,
    pub max_emit: Option<u32>,
    pub reg_idx: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EmissionPlan {
    pub ops: Vec<AccumulatorOp>,
    pub reg_indices: Vec<u32>,
    pub n_bands: u32,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum EmissionPlanError {
    #[error("EvalEML emission requires an EML registry")]
    MissingEmlRegistry,
    #[error(transparent)]
    EmlRegistry(#[from] simthing_core::EmlRegistryError),
    #[error("non-finite constant emission value")]
    NonFiniteConstant,
    #[error("max_emit is not supported until shader clamp is implemented")]
    MaxEmitUnsupported,
    #[error(
        "registration tree_id {registration_tree_id:?} does not match EvalEML formula tree_id {formula_tree_id}"
    )]
    MismatchedTreeIdField {
        registration_tree_id: Option<u32>,
        formula_tree_id: u32,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum EmissionSyncError {
    #[error(transparent)]
    Plan(#[from] EmissionPlanError),
    #[error(transparent)]
    EmlUpload(#[from] crate::accumulator_op::EmlUploadError),
    #[error(transparent)]
    Encode(#[from] EncodeError),
    #[error(transparent)]
    Session(#[from] crate::AccumulatorOpSessionError),
}

fn formula_kind(formula: &EmissionFormula) -> u32 {
    match formula {
        EmissionFormula::IdentityFloor => FORMULA_KIND_IDENTITY_FLOOR,
        EmissionFormula::Constant { .. } => FORMULA_KIND_CONSTANT,
        EmissionFormula::EvalEml { .. } => FORMULA_KIND_EVAL_EML,
    }
}

fn formula_tree_id(formula: &EmissionFormula) -> Option<EmlTreeId> {
    match formula {
        EmissionFormula::EvalEml { tree_id } => Some(*tree_id),
        _ => None,
    }
}

fn validate_registration(reg: &EmissionRegistration) -> Result<(), EmissionPlanError> {
    if reg.max_emit.is_some() {
        return Err(EmissionPlanError::MaxEmitUnsupported);
    }
    if let EmissionFormula::EvalEml {
        tree_id: formula_id,
    } = &reg.formula
    {
        if let Some(reg_tree) = reg.tree_id {
            if reg_tree != *formula_id {
                return Err(EmissionPlanError::MismatchedTreeIdField {
                    registration_tree_id: Some(reg_tree.0),
                    formula_tree_id: formula_id.0,
                });
            }
        }
    }
    Ok(())
}

/// Build logical emission ops (non-conservation, `ConsumeMode::EmitEvent`).
pub fn plan_emission_ops(
    registrations: &[EmissionRegistration],
    registry: Option<&EmlExpressionRegistry>,
) -> Result<EmissionPlan, EmissionPlanError> {
    let mut ops = Vec::with_capacity(registrations.len());
    let mut reg_indices = Vec::with_capacity(registrations.len());

    for reg in registrations {
        validate_registration(reg)?;
        let op = match &reg.formula {
            EmissionFormula::IdentityFloor => AccumulatorOp {
                source: SourceSpec::SlotValue {
                    slot: SlotIndex::new(reg.source_slot),
                    col: ColumnIndex::new(reg.source_col as usize),
                },
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::EmitEvent,
                targets: vec![],
            },
            EmissionFormula::Constant { value } => {
                if !value.is_finite() {
                    return Err(EmissionPlanError::NonFiniteConstant);
                }
                AccumulatorOp {
                    source: SourceSpec::Constant(*value),
                    combine: CombineFn::Identity,
                    gate: GateSpec::OrderBand(0),
                    scale: ScaleSpec::Identity,
                    consume: ConsumeMode::EmitEvent,
                    targets: vec![],
                }
            }
            EmissionFormula::EvalEml { tree_id } => {
                let Some(registry) = registry else {
                    return Err(EmissionPlanError::MissingEmlRegistry);
                };
                registry.assert_consumer_admissible(*tree_id, EmlConsumerKind::Emission)?;
                AccumulatorOp {
                    source: SourceSpec::SlotValue {
                        slot: SlotIndex::new(reg.source_slot),
                        col: ColumnIndex::new(reg.source_col as usize),
                    },
                    combine: CombineFn::EvalEML { tree_id: tree_id.0 },
                    gate: GateSpec::OrderBand(0),
                    scale: ScaleSpec::Identity,
                    consume: ConsumeMode::EmitEvent,
                    targets: vec![],
                }
            }
        };
        ops.push(op);
        reg_indices.push(reg.reg_idx);
    }

    Ok(EmissionPlan {
        n_bands: if ops.is_empty() { 0 } else { 1 },
        ops,
        reg_indices,
    })
}

/// Encode emission ops, storing stable registration ids in `combine_b`.
pub fn encode_emission_plan(
    plan: &EmissionPlan,
    registry: Option<&EmlExpressionRegistry>,
) -> Result<Vec<AccumulatorOpGpu>, EncodeError> {
    let mut gpu_ops = Vec::with_capacity(plan.ops.len());
    for (op, reg_idx) in plan.ops.iter().zip(plan.reg_indices.iter()) {
        let mut gpu = AccumulatorOpGpu::from_op_with_eml(op, registry)?;
        gpu.combine_b = *reg_idx;
        gpu_ops.push(gpu);
    }
    Ok(gpu_ops)
}

pub fn emission_plan_signature_fields(
    registrations: &[EmissionRegistration],
) -> (
    Vec<u32>,
    Vec<u32>,
    Vec<u32>,
    Vec<u32>,
    Vec<u32>,
    Vec<u32>,
    Vec<u32>,
) {
    let mut source_slots = Vec::with_capacity(registrations.len());
    let mut source_cols = Vec::with_capacity(registrations.len());
    let mut tree_ids = Vec::with_capacity(registrations.len());
    let mut formula_kinds = Vec::with_capacity(registrations.len());
    let mut reg_indices = Vec::with_capacity(registrations.len());
    let mut constant_value_bits = Vec::with_capacity(registrations.len());
    let mut max_emit_values = Vec::with_capacity(registrations.len());
    for reg in registrations {
        source_slots.push(reg.source_slot);
        source_cols.push(reg.source_col);
        tree_ids.push(
            formula_tree_id(&reg.formula)
                .map(|t| t.0)
                .unwrap_or(NO_TREE_ID),
        );
        formula_kinds.push(formula_kind(&reg.formula));
        reg_indices.push(reg.reg_idx);
        constant_value_bits.push(match reg.formula {
            EmissionFormula::Constant { value } => value.to_bits(),
            _ => NO_CONSTANT,
        });
        max_emit_values.push(reg.max_emit.unwrap_or(NO_MAX_EMIT));
    }
    (
        source_slots,
        source_cols,
        tree_ids,
        formula_kinds,
        reg_indices,
        constant_value_bits,
        max_emit_values,
    )
}

fn emission_cell_index(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

/// CPU-oracle twin of kernel EmitEvent readback for driver parity burn-in.
pub fn cpu_oracle_emission_records(
    flat: &[f32],
    n_dims: u32,
    emissions: &[EmissionRegistration],
) -> Result<Vec<crate::EmissionRecord>, EmissionPlanError> {
    emissions
        .iter()
        .map(|emission| {
            let idx = emission_cell_index(emission.source_slot, emission.source_col, n_dims);
            let source = flat[idx];
            let emit_count = match &emission.formula {
                EmissionFormula::IdentityFloor => source.floor().max(0.0) as u32,
                EmissionFormula::Constant { value } => u32::from(source >= *value),
                EmissionFormula::EvalEml { .. } => {
                    return Err(EmissionPlanError::MissingEmlRegistry);
                }
            };
            Ok(simthing_kernel::readback::emission_record_from_cpu_oracle(
                emission.reg_idx,
                emit_count,
                simthing_kernel::ReadbackAuthority::for_kernel_readback(),
            ))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        eml_opcode, EmlConsumerMask, EmlExecutionClass, EmlFormulaMeta, EmlNodeGpu,
    };

    fn exact_meta(id: u32) -> EmlFormulaMeta {
        EmlFormulaMeta {
            tree_id: EmlTreeId(id),
            execution_class: EmlExecutionClass::ExactDeterministic,
            allowed_consumers: EmlConsumerMask(
                EmlConsumerMask::ALL_PRODUCTION | EmlConsumerMask::DEBUG_ORACLE,
            ),
            max_abs_error: None,
            deterministic_gpu: true,
            requires_guard_for_hard_threshold: false,
            node_count: 0,
            max_stack_depth: 0,
            has_loops: false,
            has_recursion: false,
            display_name: "exact".into(),
        }
    }

    #[test]
    fn plan_identity_floor_emission() {
        let regs = vec![EmissionRegistration {
            source_slot: 0,
            source_col: 0,
            tree_id: None,
            formula: EmissionFormula::IdentityFloor,
            max_emit: None,
            reg_idx: 7,
        }];
        let plan = plan_emission_ops(&regs, None).unwrap();
        assert_eq!(plan.ops.len(), 1);
        assert!(matches!(plan.ops[0].consume, ConsumeMode::EmitEvent));
        assert_eq!(plan.reg_indices, vec![7]);
    }

    #[test]
    fn c8d_emission_accepts_exact_deterministic_formula() {
        let mut registry = EmlExpressionRegistry::new();
        let id = EmlTreeId(1);
        registry
            .register_formula(
                id,
                exact_meta(1),
                vec![EmlNodeGpu {
                    opcode: eml_opcode::LITERAL_F32,
                    flags: 0,
                    a: 1.0f32.to_bits(),
                    b: 0,
                    c: 0,
                    d: 0,
                }],
            )
            .unwrap();
        let regs = vec![EmissionRegistration {
            source_slot: 0,
            source_col: 0,
            tree_id: Some(id),
            formula: EmissionFormula::EvalEml { tree_id: id },
            max_emit: None,
            reg_idx: 0,
        }];
        assert!(plan_emission_ops(&regs, Some(&registry)).is_ok());
    }

    #[test]
    fn c8d_emission_rejects_cpu_oracle_only_formula() {
        let mut registry = EmlExpressionRegistry::new();
        let id = EmlTreeId(2);
        let mut meta = exact_meta(2);
        meta.execution_class = EmlExecutionClass::CpuOracleOnly;
        meta.allowed_consumers = EmlConsumerMask(EmlConsumerMask::DEBUG_ORACLE);
        assert!(registry
            .register_formula(
                id,
                meta,
                vec![EmlNodeGpu {
                    opcode: eml_opcode::LITERAL_F32,
                    flags: 0,
                    a: 1.0f32.to_bits(),
                    b: 0,
                    c: 0,
                    d: 0,
                }]
            )
            .is_err());
    }

    #[test]
    fn c8d_emission_rejects_fast_approx_without_tolerance_gate() {
        let mut registry = EmlExpressionRegistry::new();
        let id = EmlTreeId(3);
        let mut meta = exact_meta(3);
        meta.execution_class = EmlExecutionClass::FastApproximate;
        meta.allowed_consumers = EmlConsumerMask(EmlConsumerMask::DEBUG_ORACLE);
        registry
            .register_formula(
                id,
                meta,
                vec![EmlNodeGpu {
                    opcode: eml_opcode::LITERAL_F32,
                    flags: 0,
                    a: 1.0f32.to_bits(),
                    b: 0,
                    c: 0,
                    d: 0,
                }],
            )
            .unwrap();
        let regs = vec![EmissionRegistration {
            source_slot: 0,
            source_col: 0,
            tree_id: Some(id),
            formula: EmissionFormula::EvalEml { tree_id: id },
            max_emit: None,
            reg_idx: 0,
        }];
        assert!(plan_emission_ops(&regs, Some(&registry)).is_err());
    }

    #[test]
    fn encode_sets_reg_idx_in_combine_b() {
        let regs = vec![EmissionRegistration {
            source_slot: 0,
            source_col: 0,
            tree_id: None,
            formula: EmissionFormula::Constant { value: 2.0 },
            max_emit: None,
            reg_idx: 42,
        }];
        let plan = plan_emission_ops(&regs, None).unwrap();
        let gpu = encode_emission_plan(&plan, None).unwrap();
        assert_eq!(gpu[0].combine_b, 42);
    }

    #[test]
    fn c8d_max_emit_rejected_until_supported() {
        let regs = vec![EmissionRegistration {
            source_slot: 0,
            source_col: 0,
            tree_id: None,
            formula: EmissionFormula::IdentityFloor,
            max_emit: Some(5),
            reg_idx: 0,
        }];
        assert_eq!(
            plan_emission_ops(&regs, None),
            Err(EmissionPlanError::MaxEmitUnsupported)
        );
    }

    #[test]
    fn c8d_mismatched_registration_tree_id_rejected() {
        let regs = vec![EmissionRegistration {
            source_slot: 0,
            source_col: 0,
            tree_id: Some(EmlTreeId(1)),
            formula: EmissionFormula::EvalEml {
                tree_id: EmlTreeId(2),
            },
            max_emit: None,
            reg_idx: 0,
        }];
        assert_eq!(
            plan_emission_ops(&regs, None),
            Err(EmissionPlanError::MismatchedTreeIdField {
                registration_tree_id: Some(1),
                formula_tree_id: 2,
            })
        );
    }

    #[test]
    fn signature_uses_formula_tree_id_not_parallel_field() {
        let regs = vec![EmissionRegistration {
            source_slot: 0,
            source_col: 0,
            tree_id: None,
            formula: EmissionFormula::EvalEml {
                tree_id: EmlTreeId(7),
            },
            max_emit: None,
            reg_idx: 3,
        }];
        let (_, _, tree_ids, _, reg_indices, constant_bits, max_emit) =
            emission_plan_signature_fields(&regs);
        assert_eq!(tree_ids, vec![7]);
        assert_eq!(reg_indices, vec![3]);
        assert_eq!(constant_bits, vec![NO_CONSTANT]);
        assert_eq!(max_emit, vec![NO_MAX_EMIT]);
    }
}
