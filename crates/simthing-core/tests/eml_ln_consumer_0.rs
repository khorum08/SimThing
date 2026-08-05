//! EML-LN-PRIMITIVE-0 consumer: PowerLaw / eml() / entropy / LogAccumulate
//! exercises through the shared EvalEML interpreter and the planted referee
//! that LogAccumulate map output is not bit-equivalent to Product.

use simthing_kernel::{
    eval_eml_cpu, EmlOperatorGadget, EntropyTermGadget, LogAccumulateMapGadget, PowerLawGadget,
};

#[test]
fn eml_ln_primitive_0_power_law_gadget_admits_and_matches_its_oracle() {
    let gadget = PowerLawGadget { x_col: 0, a: 0.5 };
    let nodes = gadget.compile_nodes().expect("power law admits");
    for x in [1.0f32, 2.0, 4.0, 16.0, 0.125] {
        let evaluated = eval_eml_cpu(&nodes, 0, &[x], 1, [0.0; 4]);
        assert_eq!(
            evaluated.to_bits(),
            gadget.oracle(x).to_bits(),
            "power law parity at x={x}"
        );
    }
}

#[test]
fn eml_ln_primitive_0_eml_operator_gadget_admits_and_matches_its_oracle() {
    let gadget = EmlOperatorGadget {
        x_col: 0,
        y_col: 1,
    };
    let nodes = gadget.compile_nodes().expect("eml operator admits");
    for (x, y) in [(0.0f32, 1.0), (1.0, 2.0), (-1.0, 4.0), (0.5, 8.0)] {
        let evaluated = eval_eml_cpu(&nodes, 0, &[x, y], 2, [0.0; 4]);
        assert_eq!(
            evaluated.to_bits(),
            gadget.oracle(x, y).to_bits(),
            "eml operator parity at x={x} y={y}"
        );
    }
}

#[test]
fn eml_ln_primitive_0_entropy_term_gadget_admits_and_matches_its_oracle() {
    let gadget = EntropyTermGadget { p_col: 0 };
    let nodes = gadget.compile_nodes().expect("entropy term admits");
    for p in [0.0f32, 0.01, 0.25, 0.5, 1.0] {
        let evaluated = eval_eml_cpu(&nodes, 0, &[p], 1, [0.0; 4]);
        assert_eq!(
            evaluated.to_bits(),
            gadget.oracle(p).to_bits(),
            "entropy term parity at p={p}"
        );
        if p == 0.0 {
            assert_eq!(evaluated.to_bits(), 0.0f32.to_bits(), "p=0 is authored away");
        }
    }
}

#[test]
fn eml_ln_primitive_0_log_accumulate_map_gadget_admits_and_matches_its_oracle() {
    let gadget = LogAccumulateMapGadget { x_col: 0 };
    let nodes = gadget.compile_nodes().expect("log accumulate map admits");
    for x in [1.0f32, 2.0, 10.0, 100.0] {
        let evaluated = eval_eml_cpu(&nodes, 0, &[x], 1, [0.0; 4]);
        assert_eq!(
            evaluated.to_bits(),
            gadget.oracle(x).to_bits(),
            "log accumulate map parity at x={x}"
        );
    }
}

#[test]
fn eml_ln_primitive_0_log_accumulate_is_not_bit_equivalent_to_product() {
    let log_map = LogAccumulateMapGadget { x_col: 0 };
    let log_nodes = log_map.compile_nodes().expect("log map admits");
    let corpus: [f32; 6] = [1.25, 2.0, 3.5, 0.5, 4.0, 8.0];

    let sequential_product = corpus.iter().copied().product::<f32>();
    let misuse_product_of_log_maps = corpus
        .iter()
        .map(|x| eval_eml_cpu(&log_nodes, 0, &[*x], 1, [0.0; 4]))
        .product::<f32>();

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
