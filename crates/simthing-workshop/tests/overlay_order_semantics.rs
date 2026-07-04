use simthing_workshop::overlay_order::{
    apply_compiled_overlays_cpu, apply_overlays_cpu_current,
    compare_overlay_order_rich_with_harness, compile_overlay_order_bands,
    format_overlay_order_report, make_manual_adversarial_scenario, make_overlay_order_scenario,
    make_unsafe_grouping_trap_scenario, write_overlay_order_semantics_reports_bundle,
    OverlayOrderHarness,
};

fn assert_report_ok(report: &simthing_workshop::overlay_order::OverlayOrderReport) {
    eprintln!("{}", format_overlay_order_report(report));
    assert_eq!(report.semantic_gate, "PASS", "{:?}", report);
    assert_eq!(report.determinism_gate, "PASS", "{:?}", report);
    assert!(report.within_loose_tolerance, "{:?}", report);
}

#[test]
fn overlay_order_gpu_matches_cpu_small() {
    let scenario = make_overlay_order_scenario("overlay_order_small", 128, 8, 4, true);
    let harness = OverlayOrderHarness::new().unwrap();
    let report = compare_overlay_order_rich_with_harness(&harness, &scenario).unwrap();
    assert_report_ok(&report);
}
