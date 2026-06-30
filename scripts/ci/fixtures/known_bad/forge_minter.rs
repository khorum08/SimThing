// CI fixture: FORGE-MINTERS — forbidden forge minter name.
use crate::ThresholdEvent;

pub fn for_kernel_readback(_gpu: &ThresholdEventGpu) -> ThresholdEvent {
    unimplemented!("fixture only")
}

pub struct ThresholdEventGpu {
    pub slot: u32,
}

pub struct ThresholdEvent {
    pub slot: u32,
}
