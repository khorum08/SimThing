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
           within_strict_tolerance: {}\n\
           within_loose_tolerance: {}\n\
           repeated_runs_identical: {}\n\
         \n\
         Max error diagnostics:\n\
           max_error_parent_index: {}\n\
           max_error_cpu_value: {}\n\
           max_error_gpu_value: {}\n\
           max_error_abs: {}\n\
           max_error_ulp: {}\n\
           max_error_range_offset: {}\n\
           max_error_range_len: {}\n\
         \n\
         Coverage (ranges):\n\
           empty_ranges: {}\n\
           non_empty_zero_weight_ranges: {}\n\
           single_child_ranges: {}\n\
           mixed_magnitude_ranges: {}\n\
           negative_value_ranges: {}\n\
         \n\
         Coverage (children):\n\
           negative_value_children: {}\n\
           mixed_magnitude_children: {}\n\
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
           bit_exact_gate: {}\n\
           strict_tolerance_gate: {}\n\
           loose_tolerance_gate: {}\n\
           determinism_gate: {}\n\
           parity_classification: {}\n\
           accumulatorop_weightedmean_gate: {}\n\
           decision: {}\n\
           note: {}",
        report.scenario_name,
        report.n_parents,
        report.n_children,
        report.warm_runs,
        report.max_abs_error,
        report.mean_abs_error,
        report.max_ulp_diff,
        report.bit_exact,
        report.within_strict_tolerance,
        report.within_loose_tolerance,
        report.repeated_runs_identical,
        report.max_error_parent_index,
        report.max_error_cpu_value,
        report.max_error_gpu_value,
        report.max_error_abs,
        report.max_error_ulp,
        report.max_error_range_offset,
        report.max_error_range_len,
        report.empty_ranges,
        report.non_empty_zero_weight_ranges,
        report.single_child_ranges,
        report.mixed_magnitude_ranges,
        report.negative_value_ranges,
        report.negative_value_children,
        report.mixed_magnitude_children,
        report.cpu_oracle_us,
        report.gpu_cold_total_us,
        report.gpu_warm_mean_us,
        report.gpu_warm_min_us,
        report.gpu_warm_max_us,
        report.bit_exact_gate,
        report.strict_tolerance_gate,
        report.loose_tolerance_gate,
        report.determinism_gate,
        report.parity_classification,
        report.accumulatorop_weightedmean_gate,
        report.decision,
        report.timing_note,
    )
}
