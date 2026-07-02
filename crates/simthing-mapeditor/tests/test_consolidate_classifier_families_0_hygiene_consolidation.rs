// TEST-CONSOLIDATE-CLASSIFIER-FAMILIES-0: metadata table preserving retired hygiene-theater classifier inputs.

struct HygieneTheaterCase {
    original_binary: &'static str,
    original_test: &'static str,
    classifier_input: &'static str,
}

const HYGIENE_THEATER_CASES: &[HygieneTheaterCase] = &[
    HygieneTheaterCase {
        original_binary: "studio_frame_phase_gpu_telemetry",
        original_test: "studio_performance_settings_renders_diagnostic_controls",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_frame_phase_gpu_telemetry",
        original_test: "studio_performance_settings_renders_frame_phase_section",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_frame_phase_gpu_telemetry",
        original_test: "studio_performance_telemetry_does_not_mark_runtime_saveload_status_dirty",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_frame_phase_gpu_telemetry",
        original_test: "studio_performance_telemetry_formats_build_profile",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_frame_phase_gpu_telemetry",
        original_test: "studio_performance_telemetry_formats_frame_phase_ms",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_frame_phase_gpu_telemetry",
        original_test: "studio_performance_telemetry_formats_gpu_adapter_unknown_gracefully",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_frame_phase_gpu_telemetry",
        original_test: "studio_performance_telemetry_formats_present_mode_unknown_gracefully",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_frame_phase_gpu_telemetry",
        original_test: "studio_performance_telemetry_formats_vram_as_tracked_assets_only",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_render_loop_dirty_gate",
        original_test: "performance_settings_render_loop_diagnostics_formats_counts",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_render_loop_dirty_gate",
        original_test: "performance_telemetry_remains_presentation_only",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_settings_performance_telemetry",
        original_test: "studio_performance_telemetry_defaults_to_warming_up_without_diagnostics",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_settings_performance_telemetry",
        original_test: "studio_performance_telemetry_does_not_mark_runtime_saveload_status_dirty",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_settings_performance_telemetry",
        original_test: "studio_performance_telemetry_estimates_mesh_bytes",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_settings_performance_telemetry",
        original_test: "studio_performance_telemetry_estimates_texture_bytes",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_settings_performance_telemetry",
        original_test: "studio_performance_telemetry_formats_fps_with_one_decimal",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_settings_performance_telemetry",
        original_test: "studio_performance_telemetry_formats_vram_in_mb",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_settings_performance_telemetry",
        original_test: "studio_settings_window_includes_render_loop_diagnostics",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "studio_settings_performance_telemetry",
        original_test: "studio_settings_window_renders_performance_section",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
];

#[test]
fn hygiene_theater_cases_table_preserves_inputs() {
    let mut seen = std::collections::BTreeSet::new();

    for case in HYGIENE_THEATER_CASES {
        assert!(seen.insert((case.original_binary, case.original_test)),
            "duplicate classifier input case: {}::{}", case.original_binary, case.original_test);
        assert!(!case.classifier_input.is_empty(),
            "classifier input must be non-empty for {}::{}", case.original_binary, case.original_test);
    }

    assert_eq!(HYGIENE_THEATER_CASES.len(), 18,
        "table row count must match consolidated classifier-input inventory");
}
