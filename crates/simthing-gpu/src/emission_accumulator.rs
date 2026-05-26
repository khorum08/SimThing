//! C-8d emission substrate planner → AccumulatorOp.

use simthing_core::{
    AccumulatorOp, CombineFn, ConsumeMode, EmlConsumerKind, EmlExpressionRegistry, EmlTreeId,
    GateSpec, ScaleSpec, SourceSpec,
};

use crate::{AccumulatorOpGpu, EncodeError};

/// Formula kind tags for [`EmissionOpPlanSignature::formula_kinds`].
pub const FORMULA_KIND_IDENTITY_FLOOR: u32 = 0;
pub const FORMULA_KIND_CONSTANT: u32 = 1;
pub const FORMULA_KIND_EVAL_EML: u32 = 2;

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

/// Build logical emission ops (non-conservation, `ConsumeMode::EmitEvent`).
pub fn plan_emission_ops(
    registrations: &[EmissionRegistration],
    registry: Option<&EmlExpressionRegistry>,
) -> Result<EmissionPlan, EmissionPlanError> {
    let mut ops = Vec::with_capacity(registrations.len());
    let mut reg_indices = Vec::with_capacity(registrations.len());

    for reg in registrations {
        let op = match &reg.formula {
            EmissionFormula::IdentityFloor => AccumulatorOp {
                source: SourceSpec::SlotValue {
                    slot: reg.source_slot,
                    col: reg.source_col,
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
                        slot: reg.source_slot,
                        col: reg.source_col,
                    },
                    combine: CombineFn::EvalEML {
                        tree_id: tree_id.0,
                    },
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
) -> (Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>) {
    let mut source_slots = Vec::with_capacity(registrations.len());
    let mut source_cols = Vec::with_capacity(registrations.len());
    let mut tree_ids = Vec::with_capacity(registrations.len());
    let mut formula_kinds = Vec::with_capacity(registrations.len());
    for reg in registrations {
        source_slots.push(reg.source_slot);
        source_cols.push(reg.source_col);
        tree_ids.push(reg.tree_id.map(|t| t.0).unwrap_or(0));
        formula_kinds.push(formula_kind(&reg.formula));
    }
    (source_slots, source_cols, tree_ids, formula_kinds)
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
            .register_formula(id, exact_meta(1), vec![EmlNodeGpu {
                opcode: eml_opcode::LITERAL_F32,
                flags: 0,
                a: 1.0f32.to_bits(),
                b: 0,
                c: 0,
                d: 0,
            }])
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
            .register_formula(id, meta, vec![EmlNodeGpu {
                opcode: eml_opcode::LITERAL_F32,
                flags: 0,
                a: 1.0f32.to_bits(),
                b: 0,
                c: 0,
                d: 0,
            }])
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
            .register_formula(id, meta, vec![EmlNodeGpu {
                opcode: eml_opcode::LITERAL_F32,
                flags: 0,
                a: 1.0f32.to_bits(),
                b: 0,
                c: 0,
                d: 0,
            }])
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
}
