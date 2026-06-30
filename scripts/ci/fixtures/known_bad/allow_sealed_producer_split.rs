// CI fixture: ALLOW-SEALED-PRODUCERS — split-declaration sealed return.
pub struct ThresholdEvent {
    pub slot: u32,
}

#[doc(hidden)]
pub fn forge_split(
    _slot: u32,
) -> ThresholdEvent {
    ThresholdEvent { slot: _slot }
}
