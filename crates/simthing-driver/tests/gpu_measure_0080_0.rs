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
