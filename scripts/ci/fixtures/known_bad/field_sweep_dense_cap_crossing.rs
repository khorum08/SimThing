use simthing_spec::REGION_FIELD_MAX_CELL_COUNT;

pub fn planted_dense_cap_crossing(slot_count: u32) -> bool {
    slot_count <= REGION_FIELD_MAX_CELL_COUNT
}
