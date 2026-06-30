// CI fixture: ALLOW-SEALED-PRODUCERS — Self return inside sealed impl.
pub struct ThresholdEvent {
    pub slot: u32,
}

impl ThresholdEvent {
    pub fn forge_self(_slot: u32) -> Self {
        Self { slot: _slot }
    }
}
