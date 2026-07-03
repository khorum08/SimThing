use simthing_workshop::transfer_contention::{
    compare_transfer_contention_rich_with_harness, conservation_check_contention_records,
    format_transfer_contention_report, make_manual_priority_edge_scenario,
    make_transfer_contention_scenario, priority_allocation_check, replay_transfer_records_n_ticks,
    resolve_cpu_contention_n_ticks, write_transfer_contention_reports,
    write_transfer_contention_reports_bundle, TransferContentionHarness, RESIDENT_TICKS,
};

fn assert_report_ok(report: &simthing_workshop::transfer_contention::TransferContentionReport) {
    eprintln!("{}", format_transfer_contention_report(report));
    assert_eq!(report.conservation_gate, "PASS", "{:?}", report);
    assert_eq!(report.priority_gate, "PASS", "{:?}", report);
    assert_eq!(report.replay_gate, "PASS", "{:?}", report);
    assert_eq!(report.determinism_gate, "PASS", "{:?}", report);
    assert!(!report.queue_cross_pool_contention);
    assert!(report.final_state_matches_cpu_summary_mode, "{:?}", report);
    assert!(report.final_state_matches_cpu_records_mode, "{:?}", report);
    assert!(report.summaries_match_cpu, "{:?}", report);
    assert!(report.replay_from_records_matches_gpu, "{:?}", report);
}

#[test]
fn transfer_contention_replay_reconstructs_cpu() {
    let scenario =
        make_transfer_contention_scenario("transfer_replay_small", 64, 256, 0.7, false, false);
    let ticks = 8;
    let (cpu_pools, cpu_queues, _, records) = resolve_cpu_contention_n_ticks(&scenario, ticks);
    let replayed = replay_transfer_records_n_ticks(
        &scenario.pools,
        &scenario.queues,
        &scenario.requests,
        &records,
        ticks,
        scenario.requests.len(),
    )
    .unwrap();
    assert_eq!(replayed.0, cpu_pools);
    assert_eq!(replayed.1, cpu_queues);
}

#[test]
fn transfer_contention_gpu_resident_small() {
    let scenario = make_transfer_contention_scenario(
        "transfer_contention_small",
        64,
        1_024,
        0.7,
        false,
        false,
    );
    let harness = TransferContentionHarness::new().unwrap();
    let report = compare_transfer_contention_rich_with_harness(&harness, &scenario, 16).unwrap();
    assert_report_ok(&report);
}

#[test]
fn transfer_contention_gpu_resident_sparse_100k() {
    let scenario = make_transfer_contention_scenario(
        "transfer_contention_sparse_100k",
        1_024,
        100_000,
        0.05,
        false,
        false,
    );
    let harness = TransferContentionHarness::new().unwrap();
    let report =
        compare_transfer_contention_rich_with_harness(&harness, &scenario, RESIDENT_TICKS).unwrap();
    assert_report_ok(&report);
}

#[test]
fn transfer_contention_gpu_resident_bursty_100k() {
    let scenario = make_transfer_contention_scenario(
        "transfer_contention_bursty_100k",
        1_024,
        100_000,
        1.0,
        false,
        true,
    );
    let harness = TransferContentionHarness::new().unwrap();
    let report =
        compare_transfer_contention_rich_with_harness(&harness, &scenario, RESIDENT_TICKS).unwrap();
    assert_report_ok(&report);
}

#[test]
fn transfer_contention_gpu_resident_hotspot_100k() {
    let scenario = make_transfer_contention_scenario(
        "transfer_contention_hotspot_100k",
        16,
        100_000,
        1.0,
        true,
        true,
    );
    let harness = TransferContentionHarness::new().unwrap();
    let report =
        compare_transfer_contention_rich_with_harness(&harness, &scenario, RESIDENT_TICKS).unwrap();
    assert_report_ok(&report);
    write_transfer_contention_reports(&report).expect("write transfer contention reports");
}
#[test]
#[ignore = "large transfer contention GPU resident benchmark"]
fn transfer_contention_gpu_resident_hotspot_1m() {
    let scenario = make_transfer_contention_scenario(
        "transfer_contention_hotspot_1m",
        4_096,
        1_000_000,
        1.0,
        true,
        true,
    );
    let harness = TransferContentionHarness::new().unwrap();
    let report =
        compare_transfer_contention_rich_with_harness(&harness, &scenario, RESIDENT_TICKS).unwrap();
    assert_report_ok(&report);
}

#[test]
fn transfer_contention_report_bundle() {
    let harness = TransferContentionHarness::new().unwrap();
    let scenarios = [
        make_transfer_contention_scenario(
            "transfer_contention_sparse_100k",
            1_024,
            100_000,
            0.05,
            false,
            false,
        ),
        make_transfer_contention_scenario(
            "transfer_contention_bursty_100k",
            1_024,
            100_000,
            1.0,
            false,
            true,
        ),
        make_transfer_contention_scenario(
            "transfer_contention_hotspot_100k",
            16,
            100_000,
            1.0,
            true,
            true,
        ),
    ];
    let mut reports = Vec::with_capacity(scenarios.len());
    for scenario in scenarios {
        let report =
            compare_transfer_contention_rich_with_harness(&harness, &scenario, RESIDENT_TICKS)
                .unwrap();
        assert_report_ok(&report);
        reports.push(report);
    }
    write_transfer_contention_reports_bundle(&reports)
        .expect("write transfer contention reports bundle");
}
