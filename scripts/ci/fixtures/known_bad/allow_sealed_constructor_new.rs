// CI fixture: ALLOW-SEALED-PRODUCERS — sealed constructor new() -> Self.
pub struct ThresholdEvent {
    pub slot: u32,
}

impl ThresholdEvent {
    pub fn new(_slot: u32) -> Self {
        Self { slot: _slot }
    }
}
