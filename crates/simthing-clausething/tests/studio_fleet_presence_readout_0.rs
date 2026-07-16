//! STUDIO-FLEET-PRESENCE-READOUT-0 canonical TP fleet snapshot proof.

use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario_with_source_base, parse_raw_document, rebind_pack_to_structural_rebind_ready,
};
use simthing_spec::{FleetPresenceLocation, fleet_presence_snapshot};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn canonical_clause_path() -> PathBuf {
    repo_root().join("scenarios/terran_pirate_galaxy.clause")
}

/// catches: canonical ClauseScript fleets missing owner/posture/anchor through authority hydration.
#[test]
fn canonical_tp_session_returns_owner_posture_and_anchor_for_fleets() {
    let path = canonical_clause_path();
    let text = std::fs::read_to_string(&path).expect("read canonical TP clause");
    let document = parse_raw_document(text.as_bytes()).expect("parse canonical TP clause");
    let pack = hydrate_scenario_with_source_base(&document, path.parent())
        .expect("hydrate canonical TP clause");
    let (scenario, _report) =
        rebind_pack_to_structural_rebind_ready(&pack).expect("structural rebind");

    let snapshot = fleet_presence_snapshot(&scenario).expect("fleet presence snapshot");
    assert_eq!(
        snapshot.records().len(),
        22,
        "20 authored fleets + 2 combat-contact fleets"
    );
    assert!(
        snapshot
            .records()
            .iter()
            .all(|record| record.owner_ref.is_some())
    );
    assert!(
        snapshot
            .records()
            .iter()
            .all(|record| matches!(record.location, FleetPresenceLocation::Anchored(_)))
    );
    assert_eq!(
        snapshot
            .records()
            .iter()
            .filter(|record| record.posture.is_some())
            .count(),
        20,
        "authored fleet payloads carry posture; combat-contact fleets do not"
    );

    let terran = snapshot
        .records()
        .iter()
        .filter(|record| {
            record
                .owner_ref
                .as_ref()
                .is_some_and(|owner| owner.as_str() == "terran")
        })
        .count();
    let pirate = snapshot
        .records()
        .iter()
        .filter(|record| {
            record
                .owner_ref
                .as_ref()
                .is_some_and(|owner| owner.as_str() == "pirate")
        })
        .count();
    assert_eq!(terran, 11);
    assert_eq!(pirate, 11);
    let grouped = snapshot.by_system_id();
    assert!(
        !grouped.is_empty(),
        "anchored fleet snapshot must remain keyed by generated system id"
    );
    assert_eq!(
        grouped.values().map(Vec::len).sum::<usize>(),
        snapshot.records().len(),
        "system grouping must preserve every fleet record"
    );
}
