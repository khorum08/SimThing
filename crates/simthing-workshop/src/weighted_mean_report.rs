//! Rich report formatting for WeightedMean AccumulatorOp parity spike.

use crate::weighted_mean::WeightedMeanReport;

pub fn format_weighted_mean_report(report: &WeightedMeanReport) -> String {
    format!(
        "WeightedMean AccumulatorOp parity report\n\
         scenario: {}\n\
         n_parents: {}\n\
         n_children: {}\n\
         warm_runs: {}\n\
         \n\
         Correctness:\n\
           max_abs_error: {}\n\
           mean_abs_error: {}\n\
           max_ulp_diff: {}\n\
           bit_exact: {}\n\
           within_tolerance: {}\n\
           repeated_runs_identical: {}\n\
         \n\
         Coverage:\n\
           zero_weight_cases: {}\n\
           single_child_cases: {}\n\
           mixed_magnitude_cases: {}\n\
           negative_value_cases: {}\n\
         \n\
         Timing:\n\
           cpu_oracle_us: {}\n\
           gpu_cold_total_us: {}\n\
           gpu_warm_mean_us: {}\n\
           gpu_warm_min_us: {}\n\
           gpu_warm_max_us: {}\n\
           dispatch_only_unavailable_reason: wgpu timestamp queries not implemented in this spike\n\
         \n\
         Interpretation:\n\
           correctness_gate: {}\n\
           determinism_gate: {}\n\
           parity_classification: {}\n\
           accumulatorop_weightedmean_gate: {}\n\
           note: {}",
        report.scenario_name,
        report.n_parents,
        report.n_children,
        report.warm_runs,
        report.max_abs_error,
        report.mean_abs_error,
        report.max_ulp_diff,
        report.bit_exact,
        report.within_tolerance,
        report.repeated_runs_identical,
        report.zero_weight_cases,
        report.single_child_cases,
        report.mixed_magnitude_cases,
        report.negative_value_cases,
        report.cpu_oracle_us,
        report.gpu_cold_total_us,
        report.gpu_warm_mean_us,
        report.gpu_warm_min_us,
        report.gpu_warm_max_us,
        report.correctness_gate,
        report.determinism_gate,
        report.parity_classification,
        report.accumulatorop_weightedmean_gate,
        report.timing_note,
    )
}
