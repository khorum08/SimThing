// CI fixture: ALLOW-SEALED-PRODUCERS — explicit sealed return.
pub struct ThresholdEvent {
    pub slot: u32,
}

pub fn forge_probe(_slot: u32) -> ThresholdEvent {
    ThresholdEvent { slot: _slot }
}
