#[path = "../src/dress_rehearsal_atlas_batch_0_pack.rs"]
mod dress_rehearsal_atlas_batch_0_pack;

use dress_rehearsal_atlas_batch_0_pack::{
    channel_set_has_kind, channel_set_has_owner_indexed, channel_set_matches, g_zero_sample,
    pack_coord, unpack_coord, AtlasBatchPlan, ChannelKind, LocationMaterialization, LocationRole,
    Owner, CLASS_GALACTIC_20X20, CLASS_PLANET_SURFACE_10X10, CLASS_STAR_SYSTEM_10X10,
    DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_ID, DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_STATUS_PASS,
    PACKING_STRATEGY, V78_ATLAS_VRAM_BUDGET_BYTES,
};

#[test]
fn pack_plan_is_deterministic() {
    let materialization = LocationMaterialization::canonical();
    let left = AtlasBatchPlan::from_materialization(&materialization);
    let right = AtlasBatchPlan::from_materialization(&materialization);
    assert_eq!(left, right);
}
