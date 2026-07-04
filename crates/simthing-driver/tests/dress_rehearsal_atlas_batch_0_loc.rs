#[path = "../src/dress_rehearsal_atlas_batch_0_loc.rs"]
mod dress_rehearsal_atlas_batch_0_loc;

use dress_rehearsal_atlas_batch_0_loc::{
    cell_index, channel_set_has_kind, ChannelDescriptor, ChannelKind, DressRehearsalMap, FleetKind,
    GridCell, LocationId, LocationMaterialization, LocationRole, Mobility, OccupantKind,
    OccupantPlacement, Owner, DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_ID,
    DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_STATUS_PASS, EXPECTED_LOCATION_COUNT,
    EXPECTED_OCCUPANT_COUNT, EXPECTED_TOTAL_CELL_SLOTS,
};

#[test]
fn loc_materialization_is_deterministic() {
    let map = DressRehearsalMap::canonical();
    let left = LocationMaterialization::from_map(&map);
    let right = LocationMaterialization::from_map(&map);
    assert_eq!(left, right);
}
