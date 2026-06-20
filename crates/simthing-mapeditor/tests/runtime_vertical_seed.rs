//! VERTICAL-TEST-SCENARIO-SEED-0 — load/projection/GPU proof for runtime vertical seed fixture.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use simthing_core::{SimThing, SimThingKind};
use simthing_mapeditor::{
    build_gpu_structural_upload_packet_from_scenario, build_structural_projection,
    load_scenario_authority_from_path, load_studio_session_from_scenario_path,
    prove_gpu_buffer_residency_blocking, prove_gpu_structural_validation_blocking,
    runtime_vertical_seed_scenario_spec, StudioSessionSource,
    RUNTIME_VERTICAL_SEED_PROVENANCE_SOURCE, RUNTIME_VERTICAL_SEED_SCENARIO_ID,
};
use simthing_spec::{
    deserialize_scenario_authority, serialize_scenario_authority, validate_scenario_links,
    validate_stead_mapping_consistency, SimThingScenarioSpec, SCENARIO_RENDER_WORLD_X_PROPERTY_ID,
    SCENARIO_RENDER_WORLD_Y_PROPERTY_ID, SCENARIO_RENDER_WORLD_Z_PROPERTY_ID,
};
use tempfile::TempDir;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/runtime_vertical_seed.simthing-scenario.json")
}

fn assert_no_render_authority(scenario: &SimThingScenarioSpec) {
    fn walk(node: &SimThing) {
        for property_id in node.properties.keys() {
            assert!(
                !matches!(
                    property_id.0,
                    id if id == SCENARIO_RENDER_WORLD_X_PROPERTY_ID.0
                        || id == SCENARIO_RENDER_WORLD_Y_PROPERTY_ID.0
                        || id == SCENARIO_RENDER_WORLD_Z_PROPERTY_ID.0
                ),
                "render authority property present on SimThing {}",
                node.id.raw()
            );
        }
        for child in &node.children {
            walk(child);
        }
    }
    walk(&scenario.root);
}

fn semantic_seed_equivalent(left: &SimThingScenarioSpec, right: &SimThingScenarioSpec) -> bool {
    if left.scenario_id != right.scenario_id
        || left.structural_grid.frame != right.structural_grid.frame
        || left.links != right.links
        || left.provenance != right.provenance
    {
        return false;
    }
    if left.structural_grid.placements.len() != right.structural_grid.placements.len() {
        return false;
    }
    for (a, b) in left
        .structural_grid
        .placements
        .iter()
        .zip(right.structural_grid.placements.iter())
    {
        if a.location_id != b.location_id
            || a.target_id != b.target_id
            || a.system_id != b.system_id
            || a.row != b.row
            || a.col != b.col
        {
            return false;
        }
    }
    left.root.subtree_size() == right.root.subtree_size()
        && left.gridcell_locations().count() == right.gridcell_locations().count()
        && left
            .gridcell_locations()
            .zip(right.gridcell_locations())
            .all(|(a, b)| a.kind == b.kind && !a.children.is_empty() && !b.children.is_empty())
}

fn loaded_fixture() -> SimThingScenarioSpec {
    load_scenario_authority_from_path(&fixture_path()).expect("fixture loads")
}

fn builder_seed() -> SimThingScenarioSpec {
    runtime_vertical_seed_scenario_spec()
}

// --- Part G: scenario/spec tests ---

#[test]
fn runtime_vertical_seed_scenario_is_valid_simthing_scenario_spec() {
    let scenario = builder_seed();
    validate_stead_mapping_consistency(&scenario).expect("STEAD valid");
    validate_scenario_links(&scenario).expect("links valid");
    assert_eq!(scenario.scenario_id, RUNTIME_VERTICAL_SEED_SCENARIO_ID);
    assert_eq!(
        scenario.provenance.source,
        RUNTIME_VERTICAL_SEED_PROVENANCE_SOURCE
    );
}

#[test]
fn runtime_vertical_seed_scenario_serializes_and_deserializes() {
    let scenario = builder_seed();
    let json = serialize_scenario_authority(&scenario).expect("serialize");
    let round = deserialize_scenario_authority(&json).expect("deserialize");
    assert_eq!(round.scenario_id, scenario.scenario_id);
    assert_eq!(round.structural_grid.frame, scenario.structural_grid.frame);
    assert_eq!(round.links, scenario.links);
    assert_eq!(round.provenance, scenario.provenance);
}

#[test]
fn runtime_vertical_seed_scenario_preserves_root_tree() {
    let scenario = builder_seed();
    let json = serialize_scenario_authority(&scenario).expect("serialize");
    let round = deserialize_scenario_authority(&json).expect("deserialize");
    assert_eq!(round.root.subtree_size(), scenario.root.subtree_size());
    assert_eq!(round.root.kind, SimThingKind::World);
}

#[test]
fn runtime_vertical_seed_scenario_preserves_structural_grid() {
    let scenario = builder_seed();
    let json = serialize_scenario_authority(&scenario).expect("serialize");
    let round = deserialize_scenario_authority(&json).expect("deserialize");
    assert_eq!(round.structural_grid.frame, scenario.structural_grid.frame);
    assert_eq!(
        round.structural_grid.placements.len(),
        scenario.structural_grid.placements.len()
    );
}

#[test]
fn runtime_vertical_seed_scenario_preserves_links() {
    let scenario = builder_seed();
    let json = serialize_scenario_authority(&scenario).expect("serialize");
    let round = deserialize_scenario_authority(&json).expect("deserialize");
    assert_eq!(round.links, scenario.links);
    assert_eq!(round.links.len(), 1);
}

#[test]
fn runtime_vertical_seed_scenario_has_declared_map_container() {
    let scenario = builder_seed();
    simthing_spec::resolve_map_container(&scenario).expect("map container resolves");
    assert!(!scenario.structural_grid.map_container_id.is_empty());
}

#[test]
fn runtime_vertical_seed_scenario_has_gridcell_children() {
    let scenario = builder_seed();
    assert!(scenario
        .gridcell_locations()
        .all(|gridcell| !gridcell.children.is_empty()));
    assert_eq!(scenario.gridcell_locations().count(), 2);
}

#[test]
fn runtime_vertical_seed_scenario_has_no_render_authority() {
    assert_no_render_authority(&builder_seed());
    assert_no_render_authority(&loaded_fixture());
}

#[test]
fn runtime_vertical_seed_fixture_matches_builder_semantics() {
    let built = builder_seed();
    let loaded = loaded_fixture();
    assert!(
        semantic_seed_equivalent(&built, &loaded),
        "fixture JSON must be semantically equivalent to builder"
    );
}

// --- Part G: Studio load/projection tests ---

#[test]
fn runtime_vertical_seed_loads_through_studio_scenario_io() {
    let loaded = load_scenario_authority_from_path(&fixture_path()).expect("load fixture");
    assert_eq!(loaded.scenario_id, RUNTIME_VERTICAL_SEED_SCENARIO_ID);
}

#[test]
fn runtime_vertical_seed_load_rebuilds_hydration_boundary() {
    let session =
        load_studio_session_from_scenario_path(&fixture_path(), None).expect("load session");
    assert_eq!(session.hydration.grid.occupied_cells, 2);
    assert_eq!(
        session.hydration.simthing_spec_scenario_id,
        RUNTIME_VERTICAL_SEED_SCENARIO_ID
    );
}

#[test]
fn runtime_vertical_seed_load_rebuilds_view_model() {
    let session =
        load_studio_session_from_scenario_path(&fixture_path(), None).expect("load session");
    assert_eq!(session.view_model.stars.len(), 2);
    assert_eq!(session.view_model.hyperlanes.len(), 1);
}

#[test]
fn runtime_vertical_seed_session_source_is_loaded_scenario_or_fixture() {
    let session =
        load_studio_session_from_scenario_path(&fixture_path(), None).expect("load session");
    assert!(session.is_loaded_scenario());
    assert!(matches!(
        session.source,
        StudioSessionSource::LoadedScenario { .. }
    ));
    assert!(session.generated_output.is_none());
}

#[test]
fn runtime_vertical_seed_projection_uses_structural_coordinates() {
    let session =
        load_studio_session_from_scenario_path(&fixture_path(), None).expect("load session");
    for (star, placement) in session
        .view_model
        .stars
        .iter()
        .zip(session.scenario_authority.structural_grid.placements.iter())
    {
        assert_eq!(star.structural_col, placement.col);
        assert_eq!(star.structural_row, placement.row);
        assert_eq!(star.system_id, placement.system_id);
    }
}

#[test]
fn runtime_vertical_seed_does_not_use_studio_config_as_authority() {
    let fixture_text = fs::read_to_string(fixture_path()).expect("read fixture");
    assert!(!fixture_text.contains(simthing_mapeditor::STUDIO_CONFIG_FILE_NAME));
    assert!(!fixture_text.contains("settings_dialog"));
    assert!(!fixture_text.contains("star_rendering"));
    let session =
        load_studio_session_from_scenario_path(&fixture_path(), None).expect("load session");
    assert_eq!(
        session.scenario_authority.provenance.source,
        RUNTIME_VERTICAL_SEED_PROVENANCE_SOURCE
    );
    assert!(session.is_loaded_scenario());
}

#[test]
fn runtime_vertical_seed_does_not_use_bevy_state_as_authority() {
    let json = fs::read_to_string(fixture_path()).expect("read fixture");
    assert!(!json.contains("Bevy"));
    assert!(!json.contains("world_position"));
    assert!(!json.contains("sprite_scale"));
    assert!(!json.contains("render_meta"));
    let session =
        load_studio_session_from_scenario_path(&fixture_path(), None).expect("load session");
    assert!(session.generated_output.is_none());
}

// --- Part G: GPU structural tests ---

#[test]
fn runtime_vertical_seed_builds_structural_projection() {
    let scenario = loaded_fixture();
    let projection = build_structural_projection(&scenario).expect("projection");
    assert_eq!(projection.location_indices.len(), 2);
    assert_eq!(projection.link_indices.len(), 1);
}

#[test]
fn runtime_vertical_seed_builds_gpu_upload_packet() {
    let scenario = loaded_fixture();
    let packet =
        build_gpu_structural_upload_packet_from_scenario(&scenario).expect("upload packet");
    assert_eq!(packet.frame.location_count, 2);
    assert_eq!(packet.frame.link_count, 1);
    assert_eq!(packet.locations.len(), 2);
    assert_eq!(packet.links.len(), 1);
}

#[test]
fn runtime_vertical_seed_uploads_to_gpu_buffers() {
    use simthing_gpu::context::GpuContext;

    let Some(ctx) = GpuContext::new_blocking().ok() else {
        eprintln!("skipping runtime_vertical_seed_uploads_to_gpu_buffers: no GPU adapter");
        return;
    };
    let scenario = loaded_fixture();
    let packet =
        build_gpu_structural_upload_packet_from_scenario(&scenario).expect("upload packet");
    let residency = prove_gpu_buffer_residency_blocking(&ctx.device, &ctx.queue, &packet);
    assert!(residency.ready, "{:?}", residency.deferred_reason);
}

#[test]
fn runtime_vertical_seed_gpu_validation_report_is_valid() {
    use simthing_gpu::context::GpuContext;

    let Some(ctx) = GpuContext::new_blocking().ok() else {
        eprintln!("skipping runtime_vertical_seed_gpu_validation_report_is_valid: no GPU adapter");
        return;
    };
    let scenario = loaded_fixture();
    let packet =
        build_gpu_structural_upload_packet_from_scenario(&scenario).expect("upload packet");
    let proof = prove_gpu_structural_validation_blocking(&ctx.device, &ctx.queue, &packet);
    assert!(proof.ready, "{:?}", proof.deferred_reason);
    let report = proof.validation_report.expect("report");
    assert_eq!(report.location_count, 2);
    assert_eq!(report.link_count, 1);
}

#[test]
fn runtime_vertical_seed_gpu_validation_reports_zero_invalid_endpoints() {
    use simthing_gpu::context::GpuContext;

    let Some(ctx) = GpuContext::new_blocking().ok() else {
        eprintln!(
            "skipping runtime_vertical_seed_gpu_validation_reports_zero_invalid_endpoints: no GPU adapter"
        );
        return;
    };
    let scenario = loaded_fixture();
    let packet =
        build_gpu_structural_upload_packet_from_scenario(&scenario).expect("upload packet");
    let proof = prove_gpu_structural_validation_blocking(&ctx.device, &ctx.queue, &packet);
    let report = proof.validation_report.expect("report");
    assert_eq!(report.invalid_link_endpoint_count, 0);
}

#[test]
fn runtime_vertical_seed_gpu_validation_reports_zero_self_links() {
    use simthing_gpu::context::GpuContext;

    let Some(ctx) = GpuContext::new_blocking().ok() else {
        eprintln!(
            "skipping runtime_vertical_seed_gpu_validation_reports_zero_self_links: no GPU adapter"
        );
        return;
    };
    let scenario = loaded_fixture();
    let packet =
        build_gpu_structural_upload_packet_from_scenario(&scenario).expect("upload packet");
    let proof = prove_gpu_structural_validation_blocking(&ctx.device, &ctx.queue, &packet);
    let report = proof.validation_report.expect("report");
    assert_eq!(report.self_link_count, 0);
}

// --- Part G: lifecycle/doc tests ---

#[test]
fn production_doc_mentions_vertical_test_seed() {
    let text = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/design_0_0_8_3_studio_production.md"),
    )
    .expect("read production doc");
    assert!(text.contains("VERTICAL-TEST-SCENARIO-SEED-0"));
    assert!(text.contains("runtime_vertical_seed"));
}

#[test]
fn evidence_index_mentions_vertical_test_seed() {
    let text = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/tests/current_evidence_index.md"),
    )
    .expect("read evidence index");
    assert!(text.contains("VERTICAL-TEST-SCENARIO-SEED-0"));
}

#[test]
fn runtime_vertical_seed_save_load_roundtrip_preserves_projection() {
    let scenario = builder_seed();
    let projection = build_structural_projection(&scenario).expect("projection");
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("seed.simthing-scenario.json");
    simthing_mapeditor::save_scenario_authority_to_path(&path, &scenario).expect("save");
    let session = load_studio_session_from_scenario_path(&path, None).expect("load");
    assert_eq!(session.structural_projection, projection);
    let known: BTreeSet<u32> = scenario
        .gridcell_locations()
        .map(|cell| cell.id.raw())
        .collect();
    let spawned = SimThing::new(SimThingKind::Cohort, 0);
    assert!(!known.contains(&spawned.id.raw()));
}

#[test]
fn production_doc_mentions_accumulator_driver_sim_convergence_1() {
    let text = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/design_0_0_8_3_studio_production.md"),
    )
    .expect("read production doc");
    assert!(text.contains("ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1"));
}

#[test]
fn evidence_index_mentions_accumulator_driver_sim_convergence_1() {
    let text = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/tests/current_evidence_index.md"),
    )
    .expect("read evidence index");
    assert!(text.contains("ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1"));
}
