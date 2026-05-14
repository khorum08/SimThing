use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};

/// Stable identifier for a SimThing instance. Assigned at creation, never reused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SimThingId(u32);

impl SimThingId {
    pub fn new() -> Self {
        static NEXT: AtomicU32 = AtomicU32::new(1);
        Self(NEXT.fetch_add(1, Ordering::Relaxed))
    }

    pub fn raw(self) -> u32 {
        self.0
    }
}

impl Default for SimThingId {
    fn default() -> Self {
        Self::new()
    }
}

/// Stable index into `DimensionRegistry::properties`. Assigned at registration,
/// never reused within a session (tombstoned columns stay indexed).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SimPropertyId(pub u32);

impl SimPropertyId {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// Stable identifier for an Overlay instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OverlayId(u32);

impl OverlayId {
    pub fn new() -> Self {
        static NEXT: AtomicU32 = AtomicU32::new(1);
        Self(NEXT.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for OverlayId {
    fn default() -> Self {
        Self::new()
    }
}
