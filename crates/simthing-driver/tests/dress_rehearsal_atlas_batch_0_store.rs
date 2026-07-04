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
fn store_is_deterministic() {
    let materialization = canonical_materialization();
    let left = store_oracle_from_materialization(&materialization);
    let right = store_oracle_from_materialization(&materialization);
    assert_eq!(left, right);
    let contributions_only = child_contributions_from_materialization(&materialization);
    let entries = aggregate_contributions(&contributions_only);
    assert_eq!(entries, left.entries);
}
