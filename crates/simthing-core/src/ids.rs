use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};
use thiserror::Error;

static NEXT_SIMTHING_ID: AtomicU32 = AtomicU32::new(1);

/// Stable identifier for a SimThing instance. Assigned at creation, never reused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SimThingId(u32);

impl SimThingId {
    pub fn new() -> Self {
        Self(NEXT_SIMTHING_ID.fetch_add(1, Ordering::Relaxed))
    }

    pub fn raw(self) -> u32 {
        self.0
    }

    /// Reconstruct a session-local id from a previously assigned raw value.
    /// Used when assembling explicit arena admission from compiled spec metadata.
    pub fn from_session_raw(raw: u32) -> Self {
        Self(raw)
    }
}

/// Errors raised when reserving SimThing ids from a loaded authority tree.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SimThingIdReservationError {
    #[error("loaded scenario authority has duplicate SimThing id {0}")]
    DuplicateId(u32),
    #[error("loaded scenario authority exhausted the process-local SimThing id space")]
    IdSpaceExhausted,
}

/// Advance the process-local SimThing allocator past a loaded id.
///
/// Save/load uses this after deserializing a persisted tree so new SimThings
/// cannot reuse ids already present in the loaded authority.
pub fn advance_simthing_id_allocator_past(
    max_loaded_id: SimThingId,
) -> Result<(), SimThingIdReservationError> {
    let Some(next_after_loaded) = max_loaded_id.raw().checked_add(1) else {
        return Err(SimThingIdReservationError::IdSpaceExhausted);
    };
    let mut current = NEXT_SIMTHING_ID.load(Ordering::Relaxed);
    while current < next_after_loaded {
        match NEXT_SIMTHING_ID.compare_exchange(
            current,
            next_after_loaded,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => return Ok(()),
            Err(observed) => current = observed,
        }
    }
    Ok(())
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
