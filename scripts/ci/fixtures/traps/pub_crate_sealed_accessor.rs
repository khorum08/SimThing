// CI trap: pub(crate) sealed accessor — not a public sealed producer door.
pub struct ThresholdEvent {
    pub slot: u32,
}

impl ThresholdEvent {
    pub(crate) fn from_kernel_readback(slot: u32) -> Self {
        Self { slot }
    }
}
