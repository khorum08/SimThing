//! Rich report formatting for multi-target replay / delta logging spike.

use crate::multitarget_replay::{MultiTargetReplayReport, MultiTargetResidentReport};

pub fn format_multitarget_replay_report(report: &MultiTargetReplayReport) -> String {
    format!(
        "Multi-target replay / delta logging report (upload/readback envelope mode)\n\
         scenario: {}\n\
         n_items: {}\n\
         active_count: {}\n\
         unit_cost_min/max: {}/{}\n\
         \n\
         State totals:\n\
           total_source_before: {}\n\
           total_queue_before: {}\n\
           total_units_before: {}\n\
           total_source_after: {}\n\
           total_queue_after: {}\n\
           total_units_after: {}\n\
           total_transferred: {}\n\
           total_emitted_units: {}\n\
           total_emitted_value: {}\n\
         \n\
         Correctness:\n\
           conservation_gate: {}\n\
           replay_gate: {}\n\
           determinism_gate: {}\n\
           compact_record_gate: {}\n\
           final_state_matches_cpu: {}\n\
           replay_from_compact_matches_gpu: {}\n\
           repeated_gpu_outputs_identical: {}\n\
         \n\
         Timing:\n\
           cpu_current_us: {}\n\
           gpu_compact_warm_mean_us: {}\n\
           gpu_compact_warm_min_us: {}\n\
           gpu_compact_warm_max_us: {}\n\
           gpu_full_warm_mean_us: {}\n\
           gpu_full_warm_min_us: {}\n\
           gpu_full_warm_max_us: {}\n\
           speedup_gpu_compact_vs_cpu: {:.3}x\n\
         \n\
         Record size:\n\
           compact_record_bytes: {}\n\
           full_record_bytes: {}\n\
           compact_vs_full_record_ratio: {:.4}\n\
         \n\
         Interpretation:\n\
           {}\n\
         note: {}\n\
         \n\
         Disclaimer: workshop-local; compares CPU boundary-style settlement vs GPU multi-target resolution with compact replay records; warm timings include upload/dispatch/readback (envelope mode, not GPU-resident pivot).",
        report.scenario_name,
        report.n_items,
        report.active_count,
        report.unit_cost_min,
        report.unit_cost_max,
        report.total_source_before,
        report.total_queue_before,
        report.total_units_before,
        report.total_source_after,
        report.total_queue_after,
        report.total_units_after,
        report.total_transferred,
        report.total_emitted_units,
        report.total_emitted_value,
        report.conservation_gate,
        report.replay_gate,
        report.determinism_gate,
        report.compact_record_gate,
        report.final_state_matches_cpu,
        report.replay_from_compact_matches_gpu,
        report.repeated_gpu_outputs_identical,
        report.cpu_current_us,
        report.gpu_compact_warm_mean_us,
        report.gpu_compact_warm_min_us,
        report.gpu_compact_warm_max_us,
        report.gpu_full_warm_mean_us,
        report.gpu_full_warm_min_us,
        report.gpu_full_warm_max_us,
        report.speedup_gpu_compact_vs_cpu,
        report.compact_record_bytes,
        report.full_record_bytes,
        report.compact_vs_full_record_ratio,
        report.interpretation,
        report.timing_note,
    )
}

pub fn format_multitarget_resident_report(report: &MultiTargetResidentReport) -> String {
    format!(
        "Multi-target GPU-resident replay report\n\
         scenario: {}\n\
         n_items: {}\n\
         ticks: {}\n\
         record_ticks: {}\n\
         records_memory_fallback: {}\n\
         active_count: {}\n\
         \n\
         Correctness:\n\
           final_state_matches_cpu_summary_mode: {}\n\
           final_state_matches_cpu_records_mode: {}\n\
           summary_matches_cpu: {}\n\
           replay_from_records_matches_gpu: {}\n\
           conservation_gate: {}\n\
           replay_gate: {}\n\
           determinism_gate: {}\n\
           resident_performance_gate: {}\n\
         \n\
         Timing:\n\
           cpu_n_ticks_us: {}\n\
           cpu_per_tick_us: {:.3}\n\
           gpu_resident_summary_mean_us: {}\n\
           gpu_resident_summary_min_us: {}\n\
           gpu_resident_summary_max_us: {}\n\
           gpu_resident_summary_per_tick_us: {:.3}\n\
           gpu_resident_records_mean_us: {}\n\
           gpu_resident_records_min_us: {}\n\
           gpu_resident_records_max_us: {}\n\
           gpu_resident_records_per_tick_us: {:.3}\n\
           resident_summary_speedup_vs_cpu: {:.3}x\n\
           resident_records_speedup_vs_cpu: {:.3}x\n\
         \n\
         Data movement:\n\
           summary_bytes_total: {}\n\
           compact_record_bytes_total: {}\n\
         \n\
         Interpretation:\n\
           {}\n\
         note: {}\n\
         \n\
         Disclaimer: workshop-local; not production AccumulatorOp; no contention; not pure shader timestamp timing.",
        report.scenario_name,
        report.n_items,
        report.ticks,
        report.record_ticks,
        report.records_memory_fallback,
        report.active_count,
        report.final_state_matches_cpu_summary_mode,
        report.final_state_matches_cpu_records_mode,
        report.summary_matches_cpu,
        report.replay_from_records_matches_gpu,
        report.conservation_gate,
        report.replay_gate,
        report.determinism_gate,
        report.resident_performance_gate,
        report.cpu_n_ticks_us,
        report.cpu_per_tick_us,
        report.gpu_resident_summary_mean_us,
        report.gpu_resident_summary_min_us,
        report.gpu_resident_summary_max_us,
        report.gpu_resident_summary_per_tick_us,
        report.gpu_resident_records_mean_us,
        report.gpu_resident_records_min_us,
        report.gpu_resident_records_max_us,
        report.gpu_resident_records_per_tick_us,
        report.resident_summary_speedup_vs_cpu,
        report.resident_records_speedup_vs_cpu,
        report.summary_bytes_total,
        report.compact_record_bytes_total,
        report.interpretation,
        report.timing_note,
    )
}

pub fn write_multitarget_replay_reports(report: &MultiTargetReplayReport) -> std::io::Result<()> {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/workshop");
    std::fs::create_dir_all(&dir)?;
    std::fs::write(
        dir.join("multitarget_replay_report.md"),
        format_multitarget_replay_report(report),
    )?;
    let json = serde_json::to_string_pretty(report).unwrap_or_else(|_| "{}".to_string());
    std::fs::write(dir.join("multitarget_replay_report.json"), json)?;
    Ok(())
}

pub fn write_multitarget_resident_reports(
    report: &MultiTargetResidentReport,
) -> std::io::Result<()> {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/workshop");
    std::fs::create_dir_all(&dir)?;
    std::fs::write(
        dir.join("multitarget_replay_resident_report.md"),
        format_multitarget_resident_report(report),
    )?;
    let json = serde_json::to_string_pretty(report).unwrap_or_else(|_| "{}".to_string());
    std::fs::write(
        dir.join("multitarget_replay_resident_report.json"),
        json,
    )?;
    Ok(())
}

pub fn write_multitarget_replay_reports_bundle(
    reports: &[MultiTargetReplayReport],
) -> std::io::Result<()> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/multitarget_replay_reports.txt");
    let mut body = String::from(
        "Multi-target replay / delta logging test reports (3 scenarios)\n\
         Generated by multitarget_replay_report_bundle\n\n",
    );
    for (idx, report) in reports.iter().enumerate() {
        body.push_str(&format!("========== SCENARIO {} ==========\n\n", idx + 1));
        body.push_str(&format_multitarget_replay_report(report));
        body.push_str("\n\n");
    }
    std::fs::write(path, body)
}

pub fn write_multitarget_resident_reports_bundle(
    reports: &[MultiTargetResidentReport],
) -> std::io::Result<()> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/multitarget_replay_resident_reports.txt");
    let mut body = String::from(
        "Multi-target GPU-resident replay test reports (3 scenarios)\n\
         Generated by multitarget_resident_report_bundle\n\n",
    );
    for (idx, report) in reports.iter().enumerate() {
        body.push_str(&format!("========== SCENARIO {} ==========\n\n", idx + 1));
        body.push_str(&format_multitarget_resident_report(report));
        body.push_str("\n\n");
    }
    std::fs::write(path, body)
}
