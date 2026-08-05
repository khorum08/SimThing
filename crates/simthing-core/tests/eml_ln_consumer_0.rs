//! EML-LN-PRIMITIVE-0 consumer: library gadgets remain as authored shapes, but
//! closed EvalEML registration rejects LN (remand 5186492955). Oracle-only
//! referees stay active without production admission.

use simthing_kernel::{
    EmlOperatorGadget, EntropyTermGadget, LogAccumulateMapGadget, OpcodeGateError, PowerLawGadget,
};

#[test]
fn eml_ln_primitive_0_power_law_gadget_admits_and_matches_its_oracle() {
    let gadget = PowerLawGadget { x_col: 0, a: 0.5 };
    assert_eq!(
        gadget.compile_nodes(),
        Err(OpcodeGateError::UnwhitelistedOpcode {
            opcode: simthing_core::eml_opcode::LN,
        }),
        "power law must not admit into closed EvalEML after remand 5186492955"
    );
    // Oracle remains for candidate/reference use without EvalEML dispatch.
    for x in [1.0f32, 2.0, 4.0, 16.0, 0.125] {
        assert!(gadget.oracle(x).is_finite(), "power law oracle at x={x}");
    }
}

#[test]
fn eml_ln_primitive_0_eml_operator_gadget_admits_and_matches_its_oracle() {
    let gadget = EmlOperatorGadget {
        x_col: 0,
        y_col: 1,
    };
    assert_eq!(
        gadget.compile_nodes(),
        Err(OpcodeGateError::UnwhitelistedOpcode {
            opcode: simthing_core::eml_opcode::LN,
        }),
        "eml operator must not admit into closed EvalEML after remand 5186492955"
    );
    for (x, y) in [(0.0f32, 1.0), (1.0, 2.0), (-1.0, 4.0), (0.5, 8.0)] {
        assert!(
            gadget.oracle(x, y).is_finite(),
            "eml operator oracle at x={x} y={y}"
        );
    }
}

#[test]
fn eml_ln_primitive_0_entropy_term_gadget_admits_and_matches_its_oracle() {
    let gadget = EntropyTermGadget { p_col: 0 };
    assert_eq!(
        gadget.compile_nodes(),
        Err(OpcodeGateError::UnwhitelistedOpcode {
            opcode: simthing_core::eml_opcode::LN,
        }),
        "entropy term must not admit into closed EvalEML after remand 5186492955"
    );
    for p in [0.0f32, 0.01, 0.25, 0.5, 1.0] {
        let out = gadget.oracle(p);
        if p == 0.0 {
            assert_eq!(out.to_bits(), 0.0f32.to_bits(), "p=0 is authored away");
        } else {
            assert!(out.is_finite(), "entropy term oracle at p={p}");
        }
    }
}

#[test]
fn eml_ln_primitive_0_log_accumulate_map_gadget_admits_and_matches_its_oracle() {
    let gadget = LogAccumulateMapGadget { x_col: 0 };
    assert_eq!(
        gadget.compile_nodes(),
        Err(OpcodeGateError::UnwhitelistedOpcode {
            opcode: simthing_core::eml_opcode::LN,
        }),
        "log accumulate map must not admit into closed EvalEML after remand 5186492955"
    );
    for x in [1.0f32, 2.0, 10.0, 100.0] {
        assert!(gadget.oracle(x).is_finite(), "log accumulate map oracle at x={x}");
    }
}

#[test]
fn eml_ln_primitive_0_log_accumulate_is_not_bit_equivalent_to_product() {
    let log_map = LogAccumulateMapGadget { x_col: 0 };
    let corpus: [f32; 6] = [1.25, 2.0, 3.5, 0.5, 4.0, 8.0];

    let sequential_product = corpus.iter().copied().product::<f32>();
    let misuse_product_of_log_maps = corpus.iter().map(|x| log_map.oracle(*x)).product::<f32>();

    assert_ne!(
        sequential_product.to_bits(),
        misuse_product_of_log_maps.to_bits(),
        "LogAccumulate map outputs must not be treated as Product-path amounts"
    );

    let sum_ln: f32 = corpus.iter().map(|x| log_map.oracle(*x)).sum();
    let log_sum_exp = simthing_core::eml_exp_pinned_f32(
        sum_ln.clamp(
            f32::from_bits(simthing_core::EML_EXP_DOMAIN_MIN_BITS),
            f32::from_bits(simthing_core::EML_EXP_DOMAIN_MAX_BITS),
        ),
    );
    assert_ne!(
        sequential_product.to_bits(),
        log_sum_exp.to_bits(),
        "log-sum-exp reconstruction and sequential Product are distinct laws on this corpus"
    );
}
