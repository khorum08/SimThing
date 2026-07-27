// CI fixture: CELL-STORAGE-POLYMORPHISM — tagged/heterogeneous matrix-cell storage reach.
// Must fire HEURISTIC INSPECT under CONSTITUTION-TRIPWIRES-0.
pub enum HeterogeneousMatrixCell {
    F32(f32),
    TaggedUnion(Box<dyn std::any::Any>),
}
