// Known-bad fixture: peer overlay/action authority beside the intrinsic StemThing germ.
pub struct OverlayManager {
    pub active: Vec<u32>,
}

impl OverlayManager {
    pub fn tick_lifecycle(&mut self) {
        self.active.retain(|v| *v > 0);
    }
}
