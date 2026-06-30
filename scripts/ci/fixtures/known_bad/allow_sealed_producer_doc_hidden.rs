// CI fixture: ALLOW-SEALED-PRODUCERS — doc-hidden public minter -> Self.
pub struct ThresholdEvent {
    pub slot: u32,
}

impl ThresholdEvent {
    #[doc(hidden)]
    pub fn forge_hidden(_slot: u32) -> Self {
        Self { slot: _slot }
    }
}
