use simthing_workshop::persistent_bench::{
    compare_persistent_bench_rich_with_harness, conservation_check, format_persistent_bench_report,
    make_persistent_bench_scenario, replay_bench_records, resolve_cpu_persistent_bench,
    write_persistent_bench_reports, write_persistent_bench_reports_bundle, PersistentBenchHarness,
    DEFAULT_TICKS,
};

fn assert_report_ok(report: &simthing_workshop::persistent_bench::PersistentBenchReport) {
    eprintln!("{}", format_persistent_bench_report(report));
    assert_eq!(report.correctness_gate, "PASS", "{:?}", report);
    assert_eq!(report.conservation_gate, "PASS", "{:?}", report);
    assert_eq!(report.determinism_gate, "PASS", "{:?}", report);
    assert!(report.final_state_matches_cpu, "{:?}", report);
    assert!(report.summaries_match_cpu, "{:?}", report);
    assert!(report.repeated_pivot_outputs_identical, "{:?}", report);
}

#[test]
fn persistent_bench_cpu_oracle_conserves_resources() {
    let scenario = make_persistent_bench_scenario("pb_cpu_conservation", 16, 256, 0.8, false, true);
    let ticks = 8;
    let (final_pools, final_queues, summaries, _) =
        resolve_cpu_persistent_bench(&scenario, ticks, false);
    assert!(conservation_check(
        &scenario.pools,
        &final_pools,
        &scenario.queues,
        &final_queues,
        ticks
    ));
    assert_eq!(summaries.len(), ticks as usize);
    assert!(summaries
        .iter()
        .all(|s| s.total_emitted_units > 0 || s.active_queues == 0));
}

#[test]
fn persistent_bench_current_gpu_envelope_matches_cpu_small() {
    let scenario = make_persistent_bench_scenario("pb_envelope_small", 64, 1_024, 0.7, false, true);
    let harness = PersistentBenchHarness::new().unwrap();
    let ticks = 8;
    let (cpu_pools, cpu_queues, cpu_summaries, _) =
        resolve_cpu_persistent_bench(&scenario, ticks, false);
    let (gpu_pools, gpu_queues, gpu_summaries, _, _) = harness
        .run_current_gpu_envelope(&scenario, ticks, false)
        .unwrap();
    assert_eq!(gpu_pools, cpu_pools);
    assert_eq!(gpu_queues, cpu_queues);
    assert_eq!(gpu_summaries, cpu_summaries);
}

#[test]
fn persistent_bench_pivot_resident_matches_cpu_small() {
    let scenario = make_persistent_bench_scenario("pb_pivot_small", 64, 1_024, 0.7, false, true);
    let harness = PersistentBenchHarness::new().unwrap();
    let report =
        compare_persistent_bench_rich_with_harness(&harness, &scenario, 16, false).unwrap();
    assert_report_ok(&report);
}

