//! Feeder work-queue types.
//!
//! The feeder thread receives semantic work items from the rest of the
//! simulation — gameplay code, AI, scripted events — and routes them to the
//! Transform Patcher (continuous within day) or queues them for the Tree
//! Maintainer (boundary-only).
//!
//! Per design_v4.md §11:
//!
//! - `PatchTransform` is the unit of within-day continuous mutation. It
//!   carries a `PropertyTransformDelta` (sub-field roles, not column indices)
//!   targeted at one SimThing. The Patcher resolves roles → columns via the
//!   registry and applies the op to that thing's row in the dense values
//!   buffer.
//! - Structural mutations (slot allocation, reparenting, AddDimension) only
//!   fire at the day boundary and are routed to the Tree Maintainer via
//!   `BoundaryRequest`.
//!
//! The queue is a standard `mpsc::channel`. The producer side is `Clone` so
//! multiple gameplay threads can submit work without locking. The consumer
//! side lives on the feeder thread and is consumed by `TransformPatcher::drain`.

use simthing_core::{
    Overlay, OverlayId, PropertyTransformDelta, SimPropertyId, SimThing, SimThingId,
};
use std::sync::mpsc::{channel, Receiver, Sender};

// ── Player intent ─────────────────────────────────────────────────────────────

/// A player-issued overlay to be attached at the next day boundary.
///
/// Semantically distinct from a structural `BoundaryRequest::AttachOverlay`
/// so the patcher can route, count, and apply mid-day shadow effects
/// independently of other boundary work.
#[derive(Clone, Debug)]
pub struct PlayerIntentOverlay {
    pub target: SimThingId,
    pub overlay: Overlay,
}

// ── AI intent ─────────────────────────────────────────────────────────────────

/// An AI-issued overlay plus an urgency signal.
///
/// Flows through a dedicated `AiSender`/`AiReceiver` channel — separate from
/// the player feeder channel so AI submissions don't contend with player and
/// boundary work. The patcher drains both channels in a single `drain()` call.
///
/// `urgency` is a [0.0, 1.0] hint the AI layer attaches to each overlay. It
/// does not change how the overlay is applied — the transform delta hits the
/// CPU shadow mid-day and `attach_overlay` fires at the boundary exactly like
/// a player intent. `urgency` is surfaced in `BoundaryOutcome::ai_intents` so
/// downstream systems (observability, UI) can prioritise which AI interventions
/// to surface.
#[derive(Clone, Debug)]
pub struct AiIntentOverlay {
    pub target: SimThingId,
    pub overlay: Overlay,
    /// Urgency hint in [0.0, 1.0]. Higher = AI considers this more time-sensitive.
    pub urgency: f32,
}

/// Producer handle for AI intent overlays. `Clone` so multiple AI subsystems
/// can submit work independently.
#[derive(Clone, Debug)]
pub struct AiSender {
    inner: Sender<AiIntentOverlay>,
}

impl AiSender {
    pub fn submit(&self, intent: AiIntentOverlay) -> Result<(), FeederError> {
        self.inner
            .send(intent)
            .map_err(|_| FeederError::Disconnected)
    }

    pub fn submit_ai_intent(
        &self,
        target: SimThingId,
        overlay: Overlay,
        urgency: f32,
    ) -> Result<(), FeederError> {
        self.submit(AiIntentOverlay {
            target,
            overlay,
            urgency,
        })
    }
}

/// Consumer handle for AI intent overlays. Owned by `TransformPatcher`; not
/// `Clone`.
#[derive(Debug)]
pub struct AiReceiver {
    inner: Receiver<AiIntentOverlay>,
}

impl AiReceiver {
    /// Drain all currently queued AI intents without blocking.
    pub fn drain_now(&self) -> Vec<AiIntentOverlay> {
        let mut out = Vec::new();
        while let Ok(item) = self.inner.try_recv() {
            out.push(item);
        }
        out
    }
}

/// Build a connected AI sender/receiver pair.
pub fn ai_channel() -> (AiSender, AiReceiver) {
    let (tx, rx) = channel();
    (AiSender { inner: tx }, AiReceiver { inner: rx })
}

// ── Work items ────────────────────────────────────────────────────────────────

/// Continuous within-day mutation. Targets exactly one SimThing's row and
/// applies the carried `PropertyTransformDelta` to it. The Patcher resolves
/// every `SubFieldRole` to a global column index via the registry before
/// touching values.
#[derive(Clone, Debug)]
pub struct PatchTransform {
    /// Which SimThing's row this patch hits. Must already be allocated in the
    /// `SlotAllocator` — patches against an unknown id are no-ops, the
    /// Patcher reports the count via `PatcherStats::missing_targets`.
    pub target: SimThingId,
    /// Semantic mutation. Roles, not columns; properties referenced here must
    /// be active in the registry or the patch is silently skipped (mirrors
    /// `PropertyTransformDelta::apply_to_data` behavior).
    pub delta: PropertyTransformDelta,
}

/// Day-boundary-only request. The feeder accumulates these during the day
/// and the Tree Maintainer drains them at boundary time. Within-day handling
/// of any of these would violate Invariant I7 (structural mutations only at
/// the boundary).
#[derive(Clone, Debug)]
pub enum BoundaryRequest {
    /// Insert a brand-new SimThing as a child of the given parent. Slot
    /// allocation happens at the boundary; the new id is reported back via
    /// the maintainer's outcome record.
    AddChild { parent: SimThingId, child: SimThing },
    /// Tombstone a SimThing's slot. Its row stays indexed in the GPU buffer
    /// (slot-reuse is LIFO) but is no longer reachable from the tree.
    Remove { target: SimThingId },
    /// Move a subtree under a new parent. The child keeps its slot — only
    /// the parent pointer changes.
    Reparent {
        child: SimThingId,
        new_parent: SimThingId,
    },
    /// Attach a new permanent or transient overlay to an existing SimThing.
    /// New instruction overlays from player/AI for day N+1 flow through here
    /// (per design_v4.md §10 step 7).
    AttachOverlay {
        target: SimThingId,
        overlay: Overlay,
    },
    /// Activate a suspended overlay at the boundary. No-op if the overlay is
    /// missing or already active.
    ActivateOverlay {
        target: SimThingId,
        overlay_id: OverlayId,
    },
    /// Suspend an active overlay at the boundary. No-op if the overlay is
    /// missing or already suspended.
    SuspendOverlay {
        target: SimThingId,
        overlay_id: OverlayId,
    },
    /// Register a brand-new `SubFieldSpec` mid-session. Triggers the
    /// `AddDimension` path: registry grows, every existing row gets the
    /// default value for the new column, GPU buffers reallocate. Mods/DLC
    /// path; not used by base content.
    AddDimension { property: SimPropertyId },
}

/// Outer enum the channel actually carries. The Patcher splits these on
/// drain: `Patch` items go to the value buffer immediately, `Boundary`
/// items get parked for the Tree Maintainer, and `PlayerIntent` items get
/// parked separately for attachment at the next day boundary.
#[derive(Clone, Debug)]
pub enum FeederWork {
    Patch(PatchTransform),
    Boundary(BoundaryRequest),
    PlayerIntent(PlayerIntentOverlay),
}

// ── Channel wrapper ───────────────────────────────────────────────────────────

/// Producer handle. `Clone` so multiple gameplay threads can submit work
/// without coordination. Send failures (receiver dropped) are surfaced as
/// `FeederError::Disconnected`.
#[derive(Clone, Debug)]
pub struct FeederSender {
    inner: Sender<FeederWork>,
}

impl FeederSender {
    pub fn send(&self, work: FeederWork) -> Result<(), FeederError> {
        self.inner.send(work).map_err(|_| FeederError::Disconnected)
    }

    /// Convenience: build and submit a `PatchTransform` in one call.
    pub fn submit_patch(
        &self,
        target: SimThingId,
        delta: PropertyTransformDelta,
    ) -> Result<(), FeederError> {
        self.send(FeederWork::Patch(PatchTransform { target, delta }))
    }

    pub fn submit_boundary(&self, req: BoundaryRequest) -> Result<(), FeederError> {
        self.send(FeederWork::Boundary(req))
    }

    /// Submit a player-authored overlay for attachment at the next day boundary.
    pub fn submit_player_intent(
        &self,
        target: SimThingId,
        overlay: Overlay,
    ) -> Result<(), FeederError> {
        self.send(FeederWork::PlayerIntent(PlayerIntentOverlay {
            target,
            overlay,
        }))
    }
}

/// Consumer handle. Owned by the feeder thread; not `Clone`. The Patcher
/// pulls items off this in `drain`.
#[derive(Debug)]
pub struct FeederReceiver {
    inner: Receiver<FeederWork>,
}

impl FeederReceiver {
    /// Drain everything currently waiting on the channel without blocking.
    /// Returns items in send order. Stops at the first `Empty` or
    /// `Disconnected` result from `try_recv`.
    pub fn drain_now(&self) -> Vec<FeederWork> {
        let mut out = Vec::new();
        while let Ok(item) = self.inner.try_recv() {
            out.push(item);
        }
        out
    }
}

/// Build a connected sender/receiver pair. Standard `mpsc::channel` under
/// the hood; nothing exotic.
pub fn feeder_channel() -> (FeederSender, FeederReceiver) {
    let (tx, rx) = channel();
    (FeederSender { inner: tx }, FeederReceiver { inner: rx })
}

// ── Errors ────────────────────────────────────────────────────────────────────

#[derive(thiserror::Error, Debug)]
pub enum FeederError {
    #[error("feeder receiver has been dropped; sender can no longer deliver work")]
    Disconnected,
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        PropertyTransformDelta, SimPropertyId, SimThing, SimThingKind, SubFieldRole, TransformOp,
    };

}
