//! Rich report formatting for EML Phase 5 spike benchmarks.

use crate::eml_phase5::EmlGpuRichReport;

pub fn format_rich_report(report: &EmlGpuRichReport) -> String {
    let correctness_gate = if report.eml_vs_cpu_max_abs_error <= 1e-4
        && report.eml_vs_cpu_mean_abs_error <= 1e-5
        && report.hardcoded_vs_cpu_max_abs_error <= 1e-4
        && report.hardcoded_vs_cpu_mean_abs_error <= 1e-5
        && report.eml_vs_hardcoded_max_abs_error <= 1e-4
    {
        "PASS"
    } else {
        "FAIL"
    };

    let determinism_gate = if report.eml_repeated_runs_identical && report.hardcoded_repeated_runs_identical {
        "PASS"
    } else {
        "FAIL"
    };

    format!(
        "EML Phase 5 intensity spike rich report\n\
         n_slots: {}\n\
         warm_runs: {}\n\
         \n\
         Correctness:\n\
           EML vs CPU max_abs_error: {}\n\
           EML vs CPU mean_abs_error: {}\n\
           hardcoded vs CPU max_abs_error: {}\n\
           hardcoded vs CPU mean_abs_error: {}\n\
           EML vs hardcoded max_abs_error: {}\n\
           EML vs hardcoded mean_abs_error: {}\n\
           EML repeated runs identical: {}\n\
           hardcoded repeated runs identical: {}\n\
         \n\
         Timing:\n\
           cpu_node_eval_us: {}\n\
           cpu_direct_eval_us: {}\n\
           gpu_eml_cold_total_us: {}\n\
           gpu_eml_warm_mean_us: {}\n\
           gpu_eml_warm_min_us: {}\n\
           gpu_eml_warm_max_us: {}\n\
           gpu_hardcoded_warm_mean_us: {}\n\
           gpu_hardcoded_warm_min_us: {}\n\
           gpu_hardcoded_warm_max_us: {}\n\
           dispatch_only_unavailable_reason: wgpu timestamp queries not implemented in this spike\n\
         \n\
         Interpretation:\n\
           correctness_gate: {}\n\
           determinism_gate: {}\n\
           shader_performance_gate: INFORMATIVE_ONLY\n\
           note: {}",
        report.n_slots,
        report.warm_runs,
        report.eml_vs_cpu_max_abs_error,
        report.eml_vs_cpu_mean_abs_error,
        report.hardcoded_vs_cpu_max_abs_error,
        report.hardcoded_vs_cpu_mean_abs_error,
        report.eml_vs_hardcoded_max_abs_error,
        report.eml_vs_hardcoded_mean_abs_error,
        report.eml_repeated_runs_identical,
        report.hardcoded_repeated_runs_identical,
        report.cpu_node_eval_us,
        report.cpu_direct_eval_us,
        report.gpu_eml_cold_total_us,
        report.gpu_eml_warm_mean_us,
        report.gpu_eml_warm_min_us,
        report.gpu_eml_warm_max_us,
        report.gpu_hardcoded_warm_mean_us,
        report.gpu_hardcoded_warm_min_us,
        report.gpu_hardcoded_warm_max_us,
        correctness_gate,
        determinism_gate,
        report.timing_note,
    )
}
