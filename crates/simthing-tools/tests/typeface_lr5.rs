use simthing_tools::{run_typeface_bench, CI_BENCH_CONFIG};

#[test]
fn bench_result_report_is_deterministic_enough() {
    let first = run_typeface_bench(CI_BENCH_CONFIG).expect("first bench");
    let second = run_typeface_bench(CI_BENCH_CONFIG).expect("second bench");

    assert_eq!(first.static_labels, second.static_labels);
    assert_eq!(first.damage_labels, second.damage_labels);
    assert_eq!(first.frames, second.frames);
    assert_eq!(first.initial_shape_rebuilds, second.initial_shape_rebuilds);
    assert_eq!(first.noop_shape_rebuilds, second.noop_shape_rebuilds);
    assert_eq!(first.damage_shape_rebuilds, second.damage_shape_rebuilds);
    assert_eq!(
        first.initial_rasterize_count,
        second.initial_rasterize_count
    );
    assert_eq!(
        first.noop_rasterize_count_delta,
        second.noop_rasterize_count_delta
    );
    assert_eq!(
        first.damage_rasterize_count_delta,
        second.damage_rasterize_count_delta
    );
    assert_eq!(first.instance_count, second.instance_count);
    assert_eq!(first.icon_cache_entries, second.icon_cache_entries);
}
