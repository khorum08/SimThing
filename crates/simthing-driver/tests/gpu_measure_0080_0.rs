use std::sync::OnceLock;

use simthing_driver::{
    run_dress_rehearsal_r6c_integrated_run, run_gpu_measure_0080_0, DressRehearsalR6cInput,
    GpuMeasure0080Input, GpuMeasure0080Report, GpuMeasure0080ShapeReport, GPU_MEASURE_R4_F32_BOUND,
    GPU_MEASURE_VERDICT_INTEGER_BIT_EXACT, GPU_MEASURE_VERDICT_UNMEASURED,
    GPU_MEASURE_VERDICT_VERIFIED_APPROXIMATE, R6C_GPU_POSTURE,
};

static REPORT: OnceLock<GpuMeasure0080Report> = OnceLock::new();

fn report() -> &'static GpuMeasure0080Report {
    REPORT.get_or_init(|| run_gpu_measure_0080_0(&GpuMeasure0080Input::explicit_opt_in()))
}

fn shape(name: &'static str) -> &'static GpuMeasure0080ShapeReport {
    report()
        .shape_reports
        .iter()
        .find(|shape| shape.shape_name == name)
        .unwrap_or_else(|| panic!("missing GPU-MEASURE-0080-0 shape {name}"))
}

#[test]
fn gpu_measure_0080_0_requires_discrete_gpu_or_reports_unavailable() {
    let default_report = run_gpu_measure_0080_0(&GpuMeasure0080Input::default_simsession());
    assert!(default_report.disabled_no_op);
    assert!(!default_report.admitted);
    assert!(default_report.adapter.is_none());

    let measured = report();
    assert!(
        measured.admitted,
        "GPU-MEASURE-0080-0 must fail honestly when no discrete GPU is available: {:?}",
        measured.diagnostics
    );
    let adapter = measured.adapter.as_ref().expect("adapter report");
    assert!(adapter.selected_discrete_gpu);
    assert!(!adapter.adapter_name.is_empty());
}

#[test]
fn gpu_measure_0080_0_r1_disruption_integer_shape_bit_exact_if_measured() {
    let r1 = shape("R1 disruption input + bounded recurrence");
    assert!(r1.measured_on_gpu);
    assert_eq!(r1.verdict, GPU_MEASURE_VERDICT_INTEGER_BIT_EXACT);
    assert!(r1.bit_exact, "{r1:?}");
    assert_eq!(r1.cpu_oracle_checksum, r1.gpu_checksum);
}

#[test]
fn gpu_measure_0080_0_r2_owner_reduce_shape_bit_exact_if_measured() {
    let r2 = shape("R2 owner reduce-up + disburse-down");
    assert!(r2.measured_on_gpu);
    assert_eq!(r2.verdict, GPU_MEASURE_VERDICT_INTEGER_BIT_EXACT);
    assert!(r2.bit_exact, "{r2:?}");
    assert_eq!(r2.cpu_oracle_checksum, r2.gpu_checksum);
}

#[test]
fn gpu_measure_0080_0_r6_combat_attrition_shape_bit_exact_if_measured() {
    let r6 = shape("R6 combat damage reduce + attrition emission");
    assert!(r6.measured_on_gpu);
    assert_eq!(r6.verdict, GPU_MEASURE_VERDICT_INTEGER_BIT_EXACT);
    assert!(r6.bit_exact, "{r6:?}");
    assert_eq!(r6.cpu_oracle_checksum, r6.gpu_checksum);
}

#[test]
fn gpu_measure_0080_0_r6b_construction_and_fusion_shape_bit_exact_if_measured() {
    let r6b = shape("R6B construction threshold + fusion sum");
    assert!(r6b.measured_on_gpu);
    assert_eq!(r6b.verdict, GPU_MEASURE_VERDICT_INTEGER_BIT_EXACT);
    assert!(r6b.bit_exact, "{r6b:?}");
    assert_eq!(r6b.cpu_oracle_checksum, r6b.gpu_checksum);
}

#[test]
fn gpu_measure_0080_0_r4_gradient_magnitude_verified_approximate_if_measured() {
    let r4 = shape("R4 GradientXY + Candidate-F magnitude");
    assert!(r4.measured_on_gpu);
    assert_eq!(r4.verdict, GPU_MEASURE_VERDICT_VERIFIED_APPROXIMATE);
    assert_eq!(r4.f32_bound, Some(GPU_MEASURE_R4_F32_BOUND));
    assert!(
        r4.max_abs_delta.expect("R4 max_abs_delta") <= GPU_MEASURE_R4_F32_BOUND,
        "{r4:?}"
    );
    assert!(r4.notes.contains("Candidate-F bits match"));
}

#[test]
fn gpu_measure_0080_0_reports_explicit_verdict_per_shape() {
    let measured = report();
    assert_eq!(measured.shape_reports.len(), 6);
    for expected in [
        "R1 disruption input + bounded recurrence",
        "R2 owner reduce-up + disburse-down",
        "R4 GradientXY + Candidate-F magnitude",
        "R6 combat damage reduce + attrition emission",
        "R6B construction threshold + fusion sum",
        "R6C integrated 100-tick whole-run execution",
    ] {
        let shape = shape(expected);
        assert!(!shape.verdict.is_empty());
    }
}

#[test]
fn gpu_measure_0080_0_unmeasured_shapes_keep_conformant_wording() {
    let r6c = shape("R6C integrated 100-tick whole-run execution");
    assert!(!r6c.measured_on_gpu);
    assert_eq!(r6c.verdict, GPU_MEASURE_VERDICT_UNMEASURED);
    assert_eq!(r6c.gpu_value_summary, GPU_MEASURE_VERDICT_UNMEASURED);
    assert_eq!(
        report().r6c_integrated_run_posture,
        GPU_MEASURE_VERDICT_UNMEASURED
    );
}

#[test]
fn gpu_measure_0080_0_no_semantic_wgsl_or_new_op() {
    let measured = report();
    assert!(measured.no_semantic_wgsl);
    assert!(measured.no_new_accumulator_op);
    assert!(measured.no_invariant_edit);
    assert!(measured.no_pinned_number_change);
}

#[test]
fn gpu_measure_0080_0_does_not_reopen_or_change_0080_2_behavior() {
    let measured = report();
    assert!(!measured.scenario_0080_2_reopened);
    let r6c = shape("R6C integrated 100-tick whole-run execution");
    let baseline =
        run_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::explicit_opt_in());
    assert_eq!(baseline.gpu_posture, R6C_GPU_POSTURE);
    assert_eq!(r6c.cpu_oracle_checksum, baseline.summary.stable_checksum);
}

#[test]
fn gpu_measure_0080_0_report_checksum_stable() {
    let (first, second) = simthing_driver::replay_gpu_measure_0080_0();
    assert_eq!(first.stable_report_checksum, second.stable_report_checksum);
    assert_eq!(first.shape_reports, second.shape_reports);
    println!(
        "gpu_measure_0080_0 stable_report_checksum=0x{:016x}",
        first.stable_report_checksum
    );
    for shape in &first.shape_reports {
        println!(
            "gpu_measure_0080_0 shape={} verdict={} cpu=0x{:016x} gpu=0x{:016x} max_abs_delta={:?}",
            shape.shape_name,
            shape.verdict,
            shape.cpu_oracle_checksum,
            shape.gpu_checksum,
            shape.max_abs_delta
        );
    }
}
