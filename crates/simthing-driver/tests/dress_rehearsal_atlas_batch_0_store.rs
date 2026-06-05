#[path = "../src/dress_rehearsal_atlas_batch_0_store.rs"]
mod dress_rehearsal_atlas_batch_0_store;

use dress_rehearsal_atlas_batch_0_store::{
    aggregate_contributions, canonical_materialization, canonical_pack_plan,
    canonical_pirate_shared_galactic_cell, cell_index, child_contributions_from_materialization,
    entries_at_cell_index, fixture_contribution_value, pack_round_trip_cell,
    pirate_fleet_source_ids, register_constructed_co_location_occupants,
    store_oracle_constructed_planet_patrol_pirate, store_oracle_from_materialization, ChannelKind,
    LocationId, LocationMaterialization, Owner, DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_ID,
    DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_STATUS_PASS,
};

#[test]
fn store_status_matches_gate() {
    assert_eq!(
        DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_ID,
        "ATLAS-BATCH-0-STORE"
    );
    let status = DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_STATUS_PASS;
    assert!(status.contains("IMPLEMENTED / PASS"));
    assert!(status.contains("EC-A3"));
    assert!(status.contains("STORE-GPU") && status.contains("deferred"));
    assert!(status.contains("CPU-only"));
    assert!(!status.to_lowercase().contains("gpu verified"));
    assert!(!status.contains("to_bits"));
    assert!(!status.contains("ExactDeterministic"));
    assert!(!status.contains("R1") && !status.contains("R4"));
    assert!(!status.contains("FIELD_POLICY"));
    assert!(!status.contains("REENROLL"));
}

#[test]
fn store_consumes_accepted_loc_pack_inputs() {
    let materialization = canonical_materialization();
    let plan = canonical_pack_plan();
    let left = LocationMaterialization::canonical();
    let right = LocationMaterialization::canonical();
    assert_eq!(left, right);
    assert_eq!(plan, canonical_pack_plan());
    assert_eq!(materialization.locations.len(), 27);
    assert_eq!(materialization.occupants.len(), 56);
}

#[test]
fn cell_target_uses_single_indexing_home() {
    let materialization = canonical_materialization();
    let contributions = child_contributions_from_materialization(&materialization);
    assert!(!contributions.is_empty());
    for contribution in &contributions {
        let location = materialization
            .location(contribution.location_id)
            .expect("location");
        let expected = cell_index(
            location.map_base,
            location.width,
            contribution.cell_x,
            contribution.cell_y,
        );
        assert_eq!(
            contribution.cell_index, expected,
            "cell_index must use LOC cell_index(map_base, width, x, y)"
        );
    }
}

#[test]
fn co_located_pirate_fleets_sum_only_within_pirate_channels() {
    let materialization = canonical_materialization();
    let oracle = store_oracle_from_materialization(&materialization);
    let (location_id, _x, _y, cell_index) = canonical_pirate_shared_galactic_cell(&materialization);
    let at_cell = entries_at_cell_index(&oracle, location_id, cell_index);

    let pirate_presence: f32 = at_cell
        .iter()
        .filter(|e| e.key.channel == ChannelKind::PiratePresence)
        .map(|e| e.value)
        .sum();
    let pirate_strength: f32 = at_cell
        .iter()
        .filter(|e| e.key.channel == ChannelKind::FleetStrength(Owner::Pirate))
        .map(|e| e.value)
        .sum();
    assert!(pirate_presence > 0.0);
    assert!(pirate_strength > 0.0);

    let pirate_ids = pirate_fleet_source_ids(&materialization);
    assert_eq!(pirate_ids.len(), 10);
    let expected_presence: f32 = pirate_ids
        .iter()
        .map(|id| fixture_contribution_value(id, ChannelKind::PiratePresence))
        .sum();
    let expected_strength: f32 = pirate_ids
        .iter()
        .map(|id| fixture_contribution_value(id, ChannelKind::FleetStrength(Owner::Pirate)))
        .sum();
    assert!((pirate_presence - expected_presence).abs() < 1e-6);
    assert!((pirate_strength - expected_strength).abs() < 1e-6);

    for entry in &at_cell {
        assert_eq!(entry.key.owner, Owner::Pirate);
        assert!(matches!(
            entry.key.channel,
            ChannelKind::PiratePresence | ChannelKind::FleetStrength(Owner::Pirate)
        ));
    }
    assert!(
        at_cell
            .iter()
            .all(|e| e.key.channel != ChannelKind::PatrolPresence),
        "no Terran patrol channel leakage"
    );
    assert!(
        at_cell
            .iter()
            .all(|e| e.key.channel != ChannelKind::FleetStrength(Owner::Terran)),
        "no Terran fleet-strength leakage"
    );
    assert!(
        at_cell.iter().all(|e| e.key.channel != ChannelKind::Labor),
        "no labor channel leakage"
    );
    assert!(
        at_cell
            .iter()
            .all(|e| e.key.channel != ChannelKind::Production),
        "no production channel leakage"
    );
}

#[test]
fn constructed_planet_patrol_pirate_same_cell_stays_distinct() {
    let materialization = canonical_materialization();
    let extended = register_constructed_co_location_occupants(&materialization);
    let oracle = store_oracle_constructed_planet_patrol_pirate(&materialization);
    let location = LocationId(1);
    let loc = extended.location(location).expect("system location");
    let index = cell_index(loc.map_base, loc.width, 3, 3);

    let at_cell = entries_at_cell_index(&oracle, location, index);
    let keys: std::collections::HashSet<(ChannelKind, Owner)> = at_cell
        .iter()
        .map(|e| (e.key.channel, e.key.owner))
        .collect();
    assert!(
        keys.len() >= 3,
        "planet+patrol+pirate must yield >=3 distinct (channel, owner) entries"
    );
    assert!(keys.contains(&(ChannelKind::ProductionPassThrough, Owner::Terran)));
    assert!(keys.contains(&(ChannelKind::PatrolPresence, Owner::Terran)));
    assert!(keys.contains(&(ChannelKind::PiratePresence, Owner::Pirate)));
    assert!(keys.contains(&(ChannelKind::FleetStrength(Owner::Pirate), Owner::Pirate)));

    let constructed_entries: Vec<_> = at_cell
        .iter()
        .filter(|e| {
            e.source_occupant_ids
                .iter()
                .any(|id| id.starts_with("constructed-"))
        })
        .collect();
    assert!(constructed_entries.len() >= 3);
}

#[test]
fn owner_indexed_entries_do_not_blind_sum_by_position() {
    let materialization = canonical_materialization();
    let extended = register_constructed_co_location_occupants(&materialization);
    let oracle = store_oracle_from_materialization(&extended);
    let location = LocationId(1);
    let loc = extended.location(location).expect("system");
    let index = cell_index(loc.map_base, loc.width, 3, 3);
    let at_cell = entries_at_cell_index(&oracle, location, index);

    let terran_strength = at_cell
        .iter()
        .find(|e| e.key.channel == ChannelKind::FleetStrength(Owner::Terran));
    let pirate_strength = at_cell
        .iter()
        .find(|e| e.key.channel == ChannelKind::FleetStrength(Owner::Pirate));
    assert!(terran_strength.is_some());
    assert!(pirate_strength.is_some());
    assert_ne!(terran_strength.unwrap().key, pirate_strength.unwrap().key);
}

#[test]
fn channel_metadata_survives_store() {
    let materialization = canonical_materialization();
    let plan = canonical_pack_plan();
    for class in &plan.classes {
        if let Some(loc) = materialization
            .locations
            .iter()
            .find(|loc| loc.role == class.role)
        {
            assert_eq!(loc.channels.channels.len(), class.channels.channels.len());
        }
    }
    let galactic = materialization
        .locations
        .iter()
        .find(|l| l.id == LocationId(0))
        .expect("galactic");
    assert_eq!(galactic.channels.channels.len(), 5);
}

#[test]
fn pack_coordinate_round_trip_preserves_store_target() {
    let materialization = canonical_materialization();
    let plan = canonical_pack_plan();
    let oracle = store_oracle_from_materialization(&materialization);
    for contribution in &oracle.contributions {
        assert!(pack_round_trip_cell(
            &plan,
            &materialization,
            contribution.location_id,
            contribution.cell_x,
            contribution.cell_y,
        )
        .is_some());
    }
}

#[test]
fn no_r1_r2_r3_r4_behavior() {
    let status = DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_STATUS_PASS;
    let forbidden = [
        "BoundedFeedback",
        "diffusion",
        "economy",
        "stockpile",
        "FIELD_POLICY",
        "BoundaryRequest",
        "REENROLL",
        "combat",
        "capability-tree",
    ];
    for term in forbidden {
        assert!(!status.contains(term), "STORE status must not claim {term}");
    }
    let source = include_str!("../src/dress_rehearsal_atlas_batch_0_store.rs");
    for term in [
        "simthing_gpu",
        "simthing_core",
        "simthing_sim",
        "AccumulatorOp",
        "EvalEML",
    ] {
        assert!(
            !source.contains(term),
            "STORE source must not reference {term}"
        );
    }
}

#[test]
fn store_cpu_oracle_is_explicitly_non_gpu() {
    let status = DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_STATUS_PASS;
    assert!(status.contains("CPU-only"));
    assert!(status.contains("not GPU"));
    assert!(!status.contains("GpuVerified"));
    let test_source = include_str!("../src/dress_rehearsal_atlas_batch_0_store.rs");
    assert!(!test_source.contains("simthing_gpu"));
}

#[test]
fn store_is_deterministic() {
    let materialization = canonical_materialization();
    let left = store_oracle_from_materialization(&materialization);
    let right = store_oracle_from_materialization(&materialization);
    assert_eq!(left, right);
    let contributions_only = child_contributions_from_materialization(&materialization);
    let entries = aggregate_contributions(&contributions_only);
    assert_eq!(entries, left.entries);
}
