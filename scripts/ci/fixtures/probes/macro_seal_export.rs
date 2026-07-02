macro_rules! forge_sealed {
    () => {
        pub fn forged_threshold_event() -> simthing_kernel::ThresholdEvent {
            simthing_kernel::ThresholdEvent::forge_probe()
        }
    };
}

forge_sealed!();