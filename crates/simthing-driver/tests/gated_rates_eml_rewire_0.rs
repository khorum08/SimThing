//! GATED-RATES-EML-REWIRE-0 proofs.
//!
//! Ordinary production path: gated_rates EvalEML + role-pathway columns, and
//! mapping field_urgency lanes as named StructuralScalarChannel identities.
//! Wrong implementations are test-side only.

use simthing_core::{
    eml_opcode, ColumnIndex, EmlExpressionRegistry, EmlNodeGpu, PropertyLayout, RoleOffset,
    SubFieldRole, MAX_EML_TREE_NODES,
};
use simthing_driver::{
    build_gated_rate_ops, field_urgency_eml_nodes, field_urgency_plan_channels,
    FirstSliceMappingError, ResolvedGatedRate, ResolvedMagnitude, EML_RESOURCE,
    EML_WEIGHT_PRESSURE, EML_WEIGHT_RESOURCE,
};
use simthing_gpu::eval_eml_cpu;
use simthing_spec::FIRST_SLICE_FIELD_URGENCY_COL;

const GATED_RATES_SRC: &str = include_str!("../src/gated_rates.rs");
const MAPPING_RUNTIME_SRC: &str = include_str!("../src/mapping_runtime.rs");

const N_DIMS: u32 = 8;
const SLOT: u32 = 0;

#[derive(Clone, Copy, Debug)]
enum Case {
    GateBelow,
    GateEqual,
    GateAbove,
    UngatedAdd,
    GatedMultBelow,
    GatedMultAbove,
    MappingSuccessor,
    MappingAuthoredAdmitMutant,
    GateAlwaysOnMutant,
    PlanChannelBound,
}

#[derive(Debug)]
enum Outcome {
    Bits {
        production: u32,
        oracle: u32,
    },
    Mapping {
        production: u32,
        pre_delete: u32,
        mutant: Option<u32>,
    },
    Bound(Result<(), FirstSliceMappingError>),
}

fn dummy_offset() -> RoleOffset {
    PropertyLayout::standard(0)
        .offset_of(&SubFieldRole::Amount)
        .expect("standard Amount")
}

fn oracle_col(raw: usize) -> ColumnIndex {
    ColumnIndex::from_raw_for_oracle_or_rehearsal(raw)
}

fn gated_term(
    trigger: Option<(ColumnIndex, f32)>,
    magnitude: f32,
    is_mult: bool,
) -> ResolvedGatedRate {
    ResolvedGatedRate {
        id: "oracle".into(),
        participant_slot: SLOT,
        base_offset: dummy_offset(),
        intrinsic_offset: dummy_offset(),
        base_col: oracle_col(0),
        intrinsic_col: oracle_col(2),
        trigger,
        magnitude: ResolvedMagnitude::Literal(magnitude),
        is_mult,
    }
}

fn pre_rewire_gate(trigger: f32, at_least: f32) -> f32 {
    if trigger >= at_least {
        1.0
    } else {
        0.0
    }
}

fn pre_rewire_effective_rate(base: f32, terms: &[(f32, Option<(f32, f32)>, bool)]) -> f32 {
    let mut add = 0.0f32;
    let mut mult = 1.0f32;
    for &(magnitude, trigger, is_mult) in terms {
        let gate = match trigger {
            None => 1.0,
            Some((value, at_least)) => pre_rewire_gate(value, at_least),
        };
        if is_mult {
            mult += magnitude * gate;
        } else {
            add += magnitude * gate;
        }
    }
    (base + add) * mult
}

fn production_effective_rate(terms: &[ResolvedGatedRate], values: &[f32]) -> f32 {
    let mut registry = EmlExpressionRegistry::new();
    let _ops = build_gated_rate_ops(terms, &mut registry);
    let tree_id = registry
        .tree_id_by_display_name("gated_effective_rate")
        .expect("production gated-rate tree registered");
    let nodes = registry.get_nodes(tree_id).expect("production nodes");
    eval_eml_cpu(nodes, SLOT, values, N_DIMS, [0.0; 4])
}

fn always_on_gate_mutant(base: f32, magnitude: f32) -> f32 {
    // Test-side wrong implementation: drop CMP_GE and always contribute.
    base + magnitude
}

fn slot_value_raw(col: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_opcode::SLOT_VALUE,
        flags: 0,
        a: col,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn op(opcode: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

/// Pre-delete first_slice mapping: bare positional SLOT_VALUE columns 1/2/3.
fn pre_delete_field_urgency_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_value_raw(0),
        slot_value_raw(2),
        op(eml_opcode::MUL),
        slot_value_raw(1),
        slot_value_raw(3),
        op(eml_opcode::MUL),
        op(eml_opcode::ADD),
        op(eml_opcode::RETURN_TOP),
    ]
}

/// Test-side mutant: authored-admit positional identity (urgency_col=4) as resource.
fn authored_admit_resource_mutant_nodes(n_dims: u32) -> Vec<EmlNodeGpu> {
    let authored_resource =
        ColumnIndex::try_from_admitted_authored(FIRST_SLICE_FIELD_URGENCY_COL, n_dims)
            .expect("authored urgency col");
    vec![
        slot_value_raw(0),
        slot_value_raw(EML_WEIGHT_PRESSURE.raw()),
        op(eml_opcode::MUL),
        slot_value_raw(authored_resource.raw_u32()),
        slot_value_raw(EML_WEIGHT_RESOURCE.raw()),
        op(eml_opcode::MUL),
        op(eml_opcode::ADD),
        op(eml_opcode::RETURN_TOP),
    ]
}

fn mapping_values() -> Vec<f32> {
    let mut values = vec![0.0f32; N_DIMS as usize];
    values[0] = 2.0; // pressure (INPUT)
    values[EML_RESOURCE.raw() as usize] = 4.0;
    values[EML_WEIGHT_PRESSURE.raw() as usize] = 0.5;
    values[EML_WEIGHT_RESOURCE.raw() as usize] = 0.25;
    values[FIRST_SLICE_FIELD_URGENCY_COL as usize] = 16.0;
    values
}

fn gate_values(base: f32, trigger: f32) -> Vec<f32> {
    let mut values = vec![0.0f32; N_DIMS as usize];
    values[0] = base;
    values[1] = trigger;
    values
}

fn bits(value: f32) -> u32 {
    value.to_bits()
}

fn run_case(case: Case) -> Outcome {
    match case {
        Case::GateBelow => {
            let base = 2.0;
            let mag = 4.0;
            let trigger = 1.0;
            let at_least = 3.0;
            let values = gate_values(base, trigger);
            let production = production_effective_rate(
                &[gated_term(Some((oracle_col(1), at_least)), mag, false)],
                &values,
            );
            let oracle =
                pre_rewire_effective_rate(base, &[(mag, Some((trigger, at_least)), false)]);
            Outcome::Bits {
                production: bits(production),
                oracle: bits(oracle),
            }
        }
        Case::GateEqual => {
            let base = 2.0;
            let mag = 4.0;
            let trigger = 3.0;
            let at_least = 3.0;
            let values = gate_values(base, trigger);
            let production = production_effective_rate(
                &[gated_term(Some((oracle_col(1), at_least)), mag, false)],
                &values,
            );
            let oracle =
                pre_rewire_effective_rate(base, &[(mag, Some((trigger, at_least)), false)]);
            Outcome::Bits {
                production: bits(production),
                oracle: bits(oracle),
            }
        }
        Case::GateAbove => {
            let base = 2.0;
            let mag = 4.0;
            let trigger = 5.0;
            let at_least = 3.0;
            let values = gate_values(base, trigger);
            let production = production_effective_rate(
                &[gated_term(Some((oracle_col(1), at_least)), mag, false)],
                &values,
            );
            let oracle =
                pre_rewire_effective_rate(base, &[(mag, Some((trigger, at_least)), false)]);
            Outcome::Bits {
                production: bits(production),
                oracle: bits(oracle),
            }
        }
        Case::UngatedAdd => {
            let base = 2.0;
            let mag = 4.0;
            let values = gate_values(base, 0.0);
            let production = production_effective_rate(&[gated_term(None, mag, false)], &values);
            let oracle = pre_rewire_effective_rate(base, &[(mag, None, false)]);
            Outcome::Bits {
                production: bits(production),
                oracle: bits(oracle),
            }
        }
        Case::GatedMultBelow => {
            let base = 2.0;
            let mag = 0.5;
            let trigger = 1.0;
            let at_least = 3.0;
            let values = gate_values(base, trigger);
            let production = production_effective_rate(
                &[gated_term(Some((oracle_col(1), at_least)), mag, true)],
                &values,
            );
            let oracle = pre_rewire_effective_rate(base, &[(mag, Some((trigger, at_least)), true)]);
            Outcome::Bits {
                production: bits(production),
                oracle: bits(oracle),
            }
        }
        Case::GatedMultAbove => {
            let base = 2.0;
            let mag = 0.5;
            let trigger = 5.0;
            let at_least = 3.0;
            let values = gate_values(base, trigger);
            let production = production_effective_rate(
                &[gated_term(Some((oracle_col(1), at_least)), mag, true)],
                &values,
            );
            let oracle = pre_rewire_effective_rate(base, &[(mag, Some((trigger, at_least)), true)]);
            Outcome::Bits {
                production: bits(production),
                oracle: bits(oracle),
            }
        }
        Case::MappingSuccessor => {
            let values = mapping_values();
            let (resource, weight_pressure, weight_resource) =
                field_urgency_plan_channels(N_DIMS).expect("plan channels in grid");
            let production_nodes =
                field_urgency_eml_nodes(resource, weight_pressure, weight_resource);
            let production = eval_eml_cpu(&production_nodes, SLOT, &values, N_DIMS, [0.0; 4]);
            let pre_delete = eval_eml_cpu(
                &pre_delete_field_urgency_nodes(),
                SLOT,
                &values,
                N_DIMS,
                [0.0; 4],
            );
            Outcome::Mapping {
                production: bits(production),
                pre_delete: bits(pre_delete),
                mutant: None,
            }
        }
        Case::MappingAuthoredAdmitMutant => {
            let values = mapping_values();
            let (resource, weight_pressure, weight_resource) =
                field_urgency_plan_channels(N_DIMS).expect("plan channels in grid");
            let production_nodes =
                field_urgency_eml_nodes(resource, weight_pressure, weight_resource);
            let production = eval_eml_cpu(&production_nodes, SLOT, &values, N_DIMS, [0.0; 4]);
            let pre_delete = eval_eml_cpu(
                &pre_delete_field_urgency_nodes(),
                SLOT,
                &values,
                N_DIMS,
                [0.0; 4],
            );
            let mutant = eval_eml_cpu(
                &authored_admit_resource_mutant_nodes(N_DIMS),
                SLOT,
                &values,
                N_DIMS,
                [0.0; 4],
            );
            Outcome::Mapping {
                production: bits(production),
                pre_delete: bits(pre_delete),
                mutant: Some(bits(mutant)),
            }
        }
        Case::GateAlwaysOnMutant => {
            let base = 2.0;
            let mag = 4.0;
            let trigger = 1.0;
            let at_least = 3.0;
            let values = gate_values(base, trigger);
            let production = production_effective_rate(
                &[gated_term(Some((oracle_col(1), at_least)), mag, false)],
                &values,
            );
            let oracle =
                pre_rewire_effective_rate(base, &[(mag, Some((trigger, at_least)), false)]);
            let mutant = always_on_gate_mutant(base, mag);
            assert_eq!(
                bits(production),
                bits(oracle),
                "production must still match oracle"
            );
            assert_ne!(
                bits(production),
                bits(mutant),
                "always-on mutant must disagree with gated production when trigger < at_least"
            );
            Outcome::Bits {
                production: bits(production),
                oracle: bits(oracle),
            }
        }
        Case::PlanChannelBound => Outcome::Bound(field_urgency_plan_channels(2).map(|_| ())),
    }
}

#[test]
fn gated_rates_eml_rewire_table() {
    for needle in [
        "ColumnIndex::new",
        "flow_start +",
        "fold_output_into_input",
        "EmlExpressionRegistry::new",
        "ExactBearingEvidence",
        "derive_consumer_arms",
        "ExactConsumerArm",
        "ExactConsumerDigestEvidence",
    ] {
        assert!(
            !GATED_RATES_SRC.contains(needle),
            "gated_rates production must not contain {needle}"
        );
    }
    assert!(
        GATED_RATES_SRC.contains("col_for_role"),
        "gated_rates must reach registry columns through the role pathway"
    );
    assert!(
        GATED_RATES_SRC.contains("MAX_EML_TREE_NODES"),
        "gated_rates must inherit the one existing per-program cap"
    );
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("first_slice_mapping_runtime.rs");
    assert!(
        !path.exists(),
        "first_slice_mapping_runtime.rs must be deleted, found {}",
        path.display()
    );
    assert!(
        !MAPPING_RUNTIME_SRC.contains("try_from_admitted_authored(1")
            && !MAPPING_RUNTIME_SRC.contains("try_from_admitted_authored(2")
            && !MAPPING_RUNTIME_SRC.contains("try_from_admitted_authored(3"),
        "mapping gadget lanes must not use the authored-admit door"
    );
    assert!(MAX_EML_TREE_NODES > 0);

    for case in [
        Case::GateBelow,
        Case::GateEqual,
        Case::GateAbove,
        Case::UngatedAdd,
        Case::GatedMultBelow,
        Case::GatedMultAbove,
        Case::MappingSuccessor,
        Case::MappingAuthoredAdmitMutant,
        Case::GateAlwaysOnMutant,
        Case::PlanChannelBound,
    ] {
        match run_case(case) {
            Outcome::Bits { production, oracle } => {
                assert_eq!(
                    production, oracle,
                    "{case:?}: production EvalEML must be bit-identical to pre-rewire oracle"
                );
            }
            Outcome::Mapping {
                production,
                pre_delete,
                mutant,
            } => {
                assert_eq!(
                    production, pre_delete,
                    "{case:?}: structural-plan successor must be bit-identical to pre-delete lanes 1/2/3"
                );
                if let Some(mutant) = mutant {
                    assert_ne!(
                        production, mutant,
                        "{case:?}: authored-admit urgency_col-as-resource mutant must disagree"
                    );
                }
            }
            Outcome::Bound(result) => {
                assert!(
                    matches!(
                        result,
                        Err(FirstSliceMappingError::PlanChannelOutOfGrid {
                            channel: 2,
                            n_dims: 2
                        })
                    ),
                    "{case:?}: n_dims=2 must reject weight_pressure channel 2, got {result:?}"
                );
            }
        }
    }
}
