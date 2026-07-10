// CI fixture: COLUMN-INDEX-MINT — ungrandfathered public mint (HEURISTIC).
// Must not match residual-grandfathered paths (registry, accumulator_op, runtime_0080, …).
struct ColumnIndex;
impl ColumnIndex {
    fn new(_: usize) -> Self {
        Self
    }
}

pub fn mint_unsealed_column_index() -> ColumnIndex {
    ColumnIndex::new(0)
}
