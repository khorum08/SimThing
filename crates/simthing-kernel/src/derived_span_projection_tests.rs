use super::*;

fn logical(raw: u32) -> SimThingId {
    SimThingId::from_session_raw(raw)
}

fn compact_directory(
    total: u64,
    rows: Vec<(SimThingId, LogicalRowRange)>,
) -> LogicalSubtreeDirectory {
    LogicalSubtreeDirectory::admit(total, rows).expect("compact logical directory admits")
}
