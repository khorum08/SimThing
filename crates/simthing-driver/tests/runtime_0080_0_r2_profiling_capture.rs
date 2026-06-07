//! Manual capture: writes human-readable profiling narrative for RUNTIME-0080-0-R2.
//! Run: `cargo test -p simthing-driver --test runtime_0080_0_r2_profiling_capture -- --ignored`

use std::path::PathBuf;

use simthing_driver::{render_runtime_0080_r2_artifact, run_runtime_0080_0_r2, Runtime0080R2Input};

#[test]
#[ignore = "manual doc capture — writes docs/tests/runtime_0080_0_r2_profiling_capture.md"]
fn write_r2_profiling_capture_markdown() {
    let report = run_runtime_0080_0_r2(&Runtime0080R2Input::explicit_opt_in());
    assert_ne!(
        report.verdict, "BLOCKED",
        "discrete GPU required for capture"
    );
    let profiling = report.profiling.as_ref().expect("profiling");
    assert!(profiling.total_wall_ms > 0.0);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../docs/tests/runtime_0080_0_r2_profiling_capture.md");
    let narrative = render_narrative(&report);
    std::fs::write(&path, narrative).expect("write profiling capture doc");
}

fn render_narrative(report: &simthing_driver::Runtime0080R2Report) -> String {
    let mut out = String::from(
        "# RUNTIME-0080-0-R2 — GPU-forward 100-tick profiling capture\n\n\
         Human-readable wall-clock, memory, and CPU/GPU pipeline narrative from a foreground capture run.\n\n",
    );
    out.push_str(&render_runtime_0080_r2_artifact(report));
    if let Some(profiling) = &report.profiling {
        out.push_str("\n## Plain-language readout\n\n");
        out.push_str(&format!(
            "The full GPU-forward rehearsal completed in **{:.2} ms** wall time. \
             The resident per-tick loop alone took **{:.2} ms** across 100 ticks \
             (**{:.3} ms** mean per tick). The CPU R6C oracle reference setup cost **{:.2} ms** \
             (parity witness, not part of the GPU-forward loop). Post-loop structural substrates \
             added **{:.2} ms**.\n\n",
            profiling.total_wall_ms,
            profiling.gpu_loop_ms,
            profiling.mean_tick_ms,
            profiling.oracle_setup_ms,
            profiling.substrate_ms,
        ));
        out.push_str(&format!(
            "**Who waited on whom:** {:.1}% of phased tick time was the CPU blocked on GPU readbacks \
             (full-buffer map_async + poll Wait). {:.1}% was CPU witness + boundary maintenance + \
             derived-input uploads. Tier-A transform dispatches enqueue GPU work without synchronizing \
             in-call; that work is largely drained at the next tick's opening readback.\n\n",
            profiling.gpu_sync_wait_fraction * 100.0,
            profiling.cpu_active_fraction * 100.0,
        ));
        let mem = &profiling.memory;
        out.push_str(&format!(
            "**Memory:** steady-state GPU-resident buffers total **{:.2} MiB** \
             (world {:.2} MiB + Tier-A {:.2} MiB + journal {:.2} MiB + ZeroCohort probe {:.2} MiB). \
             Each full Tier-A readback allocates a **{:.2} MiB** staging copy on the CPU during the sync. ",
            bytes_to_mib(mem.gpu_persistent_total_bytes),
            bytes_to_mib(mem.gpu_world_buffer_bytes),
            bytes_to_mib(mem.gpu_tier_a_session_bytes),
            bytes_to_mib(mem.gpu_journal_session_bytes),
            bytes_to_mib(mem.gpu_zero_cohort_probe_bytes),
            bytes_to_mib(mem.cpu_readback_staging_peak_bytes),
        ));
        if let Some(rss) = mem.process_working_set_bytes {
            out.push_str(&format!(
                "Process RSS at capture end: **{:.2} MiB**.\n",
                bytes_to_mib(rss),
            ));
        } else {
            out.push_str("Process RSS was not sampled on this platform.\n");
        }
    }
    out
}

fn bytes_to_mib(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}
