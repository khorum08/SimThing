//! TP-STUDIO-STEAD-REBIND-0 — workshop StructuralRebindReady candidate proofs.

use std::collections::BTreeSet;
use std::path::PathBuf;

use simthing_spec::{
    deserialize_scenario_authority, game_session_galaxy_map, serialize_scenario_authority,
    validate_stead_mapping_consistency,
};
use simthing_workshop::{
    default_tp_base_disc_json_path, ingest_tp_clause_scenario_path,
    rebind_authority_tree_candidate, rebind_pack_to_structural_rebind_ready,
    TpStudioClauseIngestOptions, PROJECTION_MODE_STRUCTURAL_REBIND_READY,
};

const TERRAN_PIRATE_SCENARIO_ID: &str = "terran_pirate_galaxy";

fn approved_clause_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause",
    )
}

fn options() -> TpStudioClauseIngestOptions {
    TpStudioClauseIngestOptions {
        embedded_source_json_path: Some(default_tp_base_disc_json_path()),
    }
}

fn ingest_and_rebind() -> (
    simthing_workshop::TpStudioClauseIngestResult,
    simthing_workshop::TpStudioSteadRebindResult,
) {
    let ingest =
        ingest_tp_clause_scenario_path(&approved_clause_path(), &options()).expect("ingest");
    let rebind =
        rebind_pack_to_structural_rebind_ready(&ingest.pack).expect("rebind StructuralRebindReady");
    (ingest, rebind)
}

#[test]
fn tp_studio_stead_rebind_0_structural_rebind_ready() {
    let (ingest, rebind) = ingest_and_rebind();
    assert_eq!(ingest.scenario.scenario_id, TERRAN_PIRATE_SCENARIO_ID);
    assert!(
        ingest.scenario.structural_grid.map_container_id.is_empty(),
        "candidate must start with empty map_container_id"
    );
    assert!(
        ingest.scenario.structural_grid.placements.is_empty(),
        "candidate must start with empty placements"
    );

    assert_eq!(
        rebind.report.projection_mode,
        PROJECTION_MODE_STRUCTURAL_REBIND_READY
    );
    assert!(
        !rebind.scenario.structural_grid.map_container_id.is_empty(),
        "StructuralRebindReady requires non-empty map_container_id"
    );
    assert!(
        !rebind.scenario.structural_grid.placements.is_empty(),
        "StructuralRebindReady requires non-empty placements"
    );
    assert_eq!(
        rebind.scenario.structural_grid.frame.occupied_cells as usize,
        rebind.scenario.structural_grid.placements.len()
    );
    assert_eq!(rebind.report.stead_validation, "PASS");
    validate_stead_mapping_consistency(&rebind.scenario).expect("stead validate");
    // Embedded base has thousands of links; expect links populated when join succeeds.
    assert!(
        rebind.report.link_count > 0,
        "expected embedded hyperlane links to rebind; residue={:?}",
        rebind.report.links_residue
    );
}

#[test]
fn tp_studio_stead_rebind_0_references_authority_node_ids() {
    let (ingest, rebind) = ingest_and_rebind();
    let galaxy = game_session_galaxy_map(&rebind.scenario).expect("galaxy map");
    let authority_raws: BTreeSet<u32> = galaxy
        .children
        .iter()
        .filter(|c| c.kind == simthing_core::SimThingKind::Location)
        .map(|c| c.id.raw())
        .collect();

    assert_eq!(
        rebind.scenario.structural_grid.map_container_id,
        galaxy.id.raw().to_string(),
        "map_container_id must be authority GalaxyMap raw id"
    );

    for placement in &rebind.scenario.structural_grid.placements {
        assert!(
            authority_raws.contains(&placement.simthing_id_raw),
            "placement simthing_id_raw {} not under authority GalaxyMap",
            placement.simthing_id_raw
        );
    }

    // Candidate authority root identity preserved (same scenario_id / root kind).
    assert_eq!(rebind.scenario.scenario_id, ingest.scenario.scenario_id);
    assert_eq!(rebind.scenario.root.kind, ingest.scenario.root.kind);
}

#[test]
fn tp_studio_stead_rebind_0_candidate_still_authority_serde_roundtrips() {
    let (_ingest, rebind) = ingest_and_rebind();
    let json = serialize_scenario_authority(&rebind.scenario).expect("serialize");
    let loaded = deserialize_scenario_authority(&json).expect("deserialize");
    assert_eq!(loaded.scenario_id, TERRAN_PIRATE_SCENARIO_ID);
    assert_eq!(
        loaded.structural_grid.map_container_id,
        rebind.scenario.structural_grid.map_container_id
    );
    assert_eq!(
        loaded.structural_grid.placements.len(),
        rebind.scenario.structural_grid.placements.len()
    );
    validate_stead_mapping_consistency(&loaded).expect("stead after serde");
}

#[test]
fn tp_studio_stead_rebind_0_from_candidate_api() {
    let ingest =
        ingest_tp_clause_scenario_path(&approved_clause_path(), &options()).expect("ingest");
    let rebind = rebind_authority_tree_candidate(&ingest.scenario, &ingest.pack).expect("rebind");
    assert_eq!(
        rebind.report.projection_mode,
        PROJECTION_MODE_STRUCTURAL_REBIND_READY
    );
    validate_stead_mapping_consistency(&rebind.scenario).expect("stead");
}
