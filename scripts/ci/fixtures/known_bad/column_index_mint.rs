// CI fixture: COLUMN-INDEX-MINT — unfenced use of a named admission door (HEURISTIC).
// Must not match the constructor definitions or the registry-owned layout pathway.
struct ColumnIndex;
impl ColumnIndex {
    fn from_raw_for_oracle_or_rehearsal(_: usize) -> Self {
        Self
    }
}

pub fn mint_unsealed_column_index() -> ColumnIndex {
    ColumnIndex::from_raw_for_oracle_or_rehearsal(0)
}
