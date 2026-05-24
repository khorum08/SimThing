//! Rich report formatting for WeightedMean current-vs-pivot performance benchmark.

use crate::weighted_mean_perf::WeightedMeanPerfReport;

pub fn format_weighted_mean_perf_report(report: &WeightedMeanPerfReport) -> String {
    format!(
        "WeightedMean current-vs-pivot performance report\n\
         scenario: {}\n\
         pivot_mode: {}\n\
         n_parents: {}\n\
         children_per_parent: {}\n\
         n_children: {}\n\
         n_dims: {}\n\
         weighted_mean_col_count: {}\n\
         overlay_density: {}\n\
         n_overlays: {}\n\
         warm_runs: {}\n\
         \n\
         Timing (warm, includes upload/dispatch/readback):\n\
           current_warm_mean_us: {}\n\
           current_warm_min_us: {}\n\
           current_warm_max_us: {}\n\
           pivot_warm_mean_us: {}\n\
           pivot_warm_min_us: {}\n\
           pivot_warm_max_us: {}\n\
           speedup_pivot_vs_current: {:.3}x\n\
         \n\
         Correctness (WeightedMean columns only):\n\
           current_max_abs_error: {}\n\
           pivot_max_abs_error: {}\n\
           current_parity_classification: {}\n\
           pivot_parity_classification: {}\n\
           current_deterministic: {}\n\
           pivot_deterministic: {}\n\
         \n\
         Interpretation:\n\
           {}\n\
         \n\
         Disclaimer: workshop-local production-shaped comparison; not a production pipeline benchmark; not pure shader time.\n\
         note: {}",
        report.scenario_name,
        report.pivot_mode,
        report.n_parents,
        report.children_per_parent,
        report.n_children,
        report.n_dims,
        report.weighted_mean_col_count,
        report.overlay_density,
        report.n_overlays,
        report.warm_runs,
        report.current_warm_mean_us,
        report.current_warm_min_us,
        report.current_warm_max_us,
        report.pivot_warm_mean_us,
        report.pivot_warm_min_us,
        report.pivot_warm_max_us,
        report.speedup_pivot_vs_current,
        report.current_max_abs_error,
        report.pivot_max_abs_error,
        report.current_parity_classification,
        report.pivot_parity_classification,
        report.current_deterministic,
        report.pivot_deterministic,
        report.interpretation,
        report.timing_note,
    )
}

pub fn write_perf_reports(report: &WeightedMeanPerfReport) -> std::io::Result<()> {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/workshop");
    std::fs::create_dir_all(&dir)?;
    std::fs::write(
        dir.join("weighted_mean_perf_report.md"),
        format_weighted_mean_perf_report(report),
    )?;
    let json = serde_json::to_string_pretty(report).unwrap_or_else(|_| "{}".to_string());
    std::fs::write(dir.join("weighted_mean_perf_report.json"), json)?;
    Ok(())
}
