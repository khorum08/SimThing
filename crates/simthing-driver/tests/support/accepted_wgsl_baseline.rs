//! Accepted project WGSL shader baseline for E-11B / Resource Flow guard tests.
//!
//! Lists shaders present under `simthing-gpu/src/shaders/` that are accepted
//! pre-A-0 project state, including C-0 atlas mask WGSL. Any file in that
//! directory not listed here fails the guard.

/// Canonical accepted WGSL filenames under `simthing-gpu/src/shaders/`.
pub const ACCEPTED_WGSL_SHADER_BASELINE: &[&str] = &[
    "accumulator_op.wgsl",
    "snapshot.wgsl",
    "structured_field_stencil.wgsl",
    // C-0 accepted atlas mask shader (Line C map batching proof).
    "structured_field_stencil_atlas_mask.wgsl",
    "values_fill.wgsl",
    // Retained for baseline continuity if reintroduced; not required to exist.
    "world_summary.wgsl",
];

/// Fail if `simthing-gpu/src/shaders/` contains a WGSL file outside the baseline.
pub fn assert_only_accepted_project_wgsl_shaders() {
    let wgsl_root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../simthing-gpu/src/shaders");
    let entries: Vec<String> = std::fs::read_dir(&wgsl_root)
        .expect("shaders dir")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    for name in &entries {
        assert!(
            ACCEPTED_WGSL_SHADER_BASELINE.contains(&name.as_str()),
            "unexpected WGSL file {name}; only accepted project baseline shaders allowed \
             (see tests/support/accepted_wgsl_baseline.rs)"
        );
    }
}
