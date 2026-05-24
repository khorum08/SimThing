use simthing_workshop::overlay_order::{
    apply_compiled_overlays_cpu, apply_overlays_cpu_current, compare_overlay_order_rich_with_harness,
    compile_overlay_order_bands, format_overlay_order_report, make_manual_adversarial_scenario,
    make_overlay_order_scenario, make_unsafe_grouping_trap_scenario,
    write_overlay_order_semantics_reports_bundle, OverlayOrderHarness,
};

fn assert_report_ok(report: &simthing_workshop::overlay_order::OverlayOrderReport) {
    eprintln!("{}", format_overlay_order_report(report));
    assert_eq!(report.semantic_gate, "PASS", "{:?}", report);
    assert_eq!(report.determinism_gate, "PASS", "{:?}", report);
    assert!(report.within_loose_tolerance, "{:?}", report);
}

#[test]
fn overlay_order_cpu_oracle_handles_adversarial_ordering() {
    let scenario = make_manual_adversarial_scenario();
    let outputs = apply_overlays_cpu_current(&scenario);

    assert!((outputs[0] - 30.0).abs() <= 1e-6);
    assert!((outputs[1] - 25.0).abs() <= 1e-6);
    assert!((outputs[2] - 3.0).abs() <= 1e-6);
    assert!((outputs[3] - 8.0).abs() <= 1e-6);
    assert!((outputs[4] - 7.0).abs() <= 1e-6);
    assert!((outputs[5] - 16.0).abs() <= 1e-6);
    assert!((outputs[6] - 31.0).abs() <= 1e-6);
}

#[test]
fn overlay_order_compiler_preserves_cpu_semantics() {
    let scenario = make_overlay_order_scenario("overlay_order_adversarial_small", 128, 8, 4, true);
    let cpu_current = apply_overlays_cpu_current(&scenario);
    let compiled = compile_overlay_order_bands(&scenario);
    let cpu_compiled = apply_compiled_overlays_cpu(&scenario, &compiled);

    assert_eq!(cpu_current, cpu_compiled);
    assert!(compiled.compile_stats.compiled_op_count <= compiled.compile_stats.raw_overlay_count);

    let report = compare_overlay_order_rich_with_harness(
        &OverlayOrderHarness::new().unwrap(),
        &scenario,
    )
    .unwrap();
    assert_eq!(report.semantic_gate, "PASS");
}

#[test]
fn overlay_order_gpu_matches_cpu_small() {
    let scenario = make_overlay_order_scenario("overlay_order_small", 128, 8, 4, true);
    let harness = OverlayOrderHarness::new().unwrap();
    let report = compare_overlay_order_rich_with_harness(&harness, &scenario).unwrap();
    assert_report_ok(&report);
}

#[test]
fn overlay_order_medium_clutter_stress() {
    let scenario = make_overlay_order_scenario("overlay_order_medium_clutter", 10_000, 16, 8, true);
    let harness = OverlayOrderHarness::new().unwrap();
    let report = compare_overlay_order_rich_with_harness(&harness, &scenario).unwrap();
    assert_report_ok(&report);
}

#[test]
fn overlay_order_semantics_report_bundle() {
    let harness = OverlayOrderHarness::new().unwrap();
    let scenarios = [
        make_overlay_order_scenario("overlay_order_small", 128, 8, 4, true),
        make_overlay_order_scenario("overlay_order_medium_clutter", 10_000, 16, 8, true),
        make_overlay_order_scenario("overlay_order_wide_sparse", 10_000, 64, 2, true),
    ];
    let mut reports = Vec::with_capacity(scenarios.len());
    for scenario in scenarios {
        let report = compare_overlay_order_rich_with_harness(&harness, &scenario).unwrap();
        assert_report_ok(&report);
        reports.push(report);
    }
    write_overlay_order_semantics_reports_bundle(&reports)
        .expect("write overlay order semantics reports bundle");
}

#[test]
fn overlay_order_wide_sparse_stress() {
    let scenario = make_overlay_order_scenario("overlay_order_wide_sparse", 10_000, 64, 2, true);
    let harness = OverlayOrderHarness::new().unwrap();
    let report = compare_overlay_order_rich_with_harness(&harness, &scenario).unwrap();
    assert_report_ok(&report);
}

#[test]
fn overlay_order_does_not_group_mixed_ops_unsafely() {
    let scenario = make_unsafe_grouping_trap_scenario();
    let cpu_current = apply_overlays_cpu_current(&scenario);
    assert!((cpu_current[6] - 31.0).abs() <= 1e-6, "expected 31 got {}", cpu_current[6]);

    let compiled = compile_overlay_order_bands(&scenario);
    let cpu_compiled = apply_compiled_overlays_cpu(&scenario, &compiled);
    assert!((cpu_compiled[6] - 31.0).abs() <= 1e-6, "expected 31 got {}", cpu_compiled[6]);
    assert!(compiled.compile_stats.unsafe_grouping_detected);
}
