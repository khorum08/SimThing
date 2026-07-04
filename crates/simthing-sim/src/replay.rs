//! Replay serialization + playback.
//!
//! ## Format
//!
//! Line-delimited JSON. The first line is a `ReplaySnapshot` (initial state
//! at the moment recording started). Every subsequent line is a `ReplayFrame`
//! produced from one boundary's delta log:
//!
//! ```text
//! { "kind": "snapshot", "snapshot": { day, root, registry } }
//! { "kind": "frame", "day": 1, "entries": [ ... ] }
//! { "kind": "frame", "day": 2, "entries": [ ... ] }
//! ...
//! ```
//!
//! The format trades raw write throughput for debuggability — replays are
//! grep-able and diff-able as text. A binary frame format can replace the
//! `Write`/`Read` impls later behind the same trait surface.
//!
//! ## Scope: structural reproduction + optional numeric checkpoints
//!
//! `ReplayDriver` reconstructs the **tree structure, dimension registry, and
//! slot allocator** from a recorded session. It does **not** re-run GPU
//! passes — float values from velocity integration and overlay application are
//! not recomputed during playback.
//!
//! When recorded, each `ReplayFrame` may optionally carry a post-boundary
//! `shadow_values` checkpoint (the CPU shadow after GPU readback). That enables
//! numeric audit/diff of integrated state at day boundaries without bit-exact
//! re-simulation.
//!
//! What this gets you:
//! - Verify a session's structural history (which SimThings spawned when, what
//!   overlays were attached, when properties expired).
//! - Reconstruct the tree as it stood at any recorded day boundary.
//! - Diff two replay files to find divergence points.
//!
//! What it does not get you:
//! - Bit-exact value reproduction across hardware. For that, capture GPU
//!   readbacks alongside the delta log — a separate feature.
//!
//! Replay snapshots do not expose raw root `.kind`
//! (`replay_snapshot_hides_raw_root_kind_compile_fail`):
//!
//! ```compile_fail
//! use simthing_sim::ReplaySnapshot;
//!
//! fn peek_replay_root_kind(snap: ReplaySnapshot) {
//!     let _ = snap.root.access(|root| root.kind.clone());
//! }
//! ```

use std::io::{BufRead, Write};

use serde::{Deserialize, Serialize};
use simthing_core::{DimensionRegistry, OverlayLifecycle, SimThing, SimThingId};
use simthing_gpu::SlotAllocator;

use crate::delta_log::BoundaryDeltaEntry;
use crate::fission::FissionLineageRecord;
use crate::sim_runtime_tree::SimRuntimeTree;

// ── Snapshot ──────────────────────────────────────────────────────────────────

/// Initial state captured at the start of a recording. Subsequent frames are
/// applied on top of this to reconstruct later state.
///
/// `fission_lineage` records the persistent lineage vec from
/// `BoundaryProtocol` at snapshot time so that `ReplayDriver` can re-register
/// `FusionTrigger` thresholds for any fissions that occurred before recording
/// started. Old snapshots without this field deserialize cleanly via the
/// `#[serde(default)]` attribute.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplaySnapshot {
    pub day: u32,
    pub root: SimRuntimeTree,
    pub registry: DimensionRegistry,
    #[serde(default)]
    pub fission_lineage: Vec<FissionLineageRecord>,
}

// ── Frame ─────────────────────────────────────────────────────────────────────

/// One day's worth of structural changes.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReplayFrame {
    pub day: u32,
    pub entries: Vec<BoundaryDeltaEntry>,
    /// Post-boundary CPU shadow (`n_slots × n_dims`), when recording enabled it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shadow_values: Option<Vec<f32>>,
    /// Per-frame spec-layer deltas (capability states, scripted cooldowns,
    /// notifications, player selections). Written by `simthing-driver`'s
    /// recording path; ignored by sim-only consumers. Stored as opaque JSON
    /// values so `simthing-sim` stays spec-free — the driver deserializes them
    /// as `SpecDelta` via `spec_replay::json_to_spec_deltas`.
    ///
    /// Old replays without this field parse cleanly via the `#[serde(default)]`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spec_entries: Vec<serde_json::Value>,
}

/// Discriminated record written one-per-line to the replay stream.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ReplayRecord {
    Snapshot { snapshot: ReplaySnapshot },
    Frame { frame: ReplayFrame },
}

// ── Errors ────────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum ReplayError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("first record must be a snapshot; got a frame")]
    MissingSnapshot,
    #[error("snapshot appears mid-stream after frames have been read")]
    UnexpectedSnapshot,
}

// ── Writer ────────────────────────────────────────────────────────────────────

/// LDJSON replay writer. Emits one record per line into the underlying writer.
/// Caller is responsible for flushing / closing.
pub struct ReplayWriter<W: Write> {
    inner: W,
    snapshot_written: bool,
}

impl<W: Write> ReplayWriter<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            snapshot_written: false,
        }
    }

    /// Write the initial snapshot. Must be called before any frames.
    pub fn write_snapshot(&mut self, snapshot: &ReplaySnapshot) -> Result<(), ReplayError> {
        let rec = ReplayRecord::Snapshot {
            snapshot: snapshot.clone(),
        };
        serde_json::to_writer(&mut self.inner, &rec)?;
        self.inner.write_all(b"\n")?;
        self.snapshot_written = true;
        Ok(())
    }

    /// Append one boundary's delta log as a frame.
    pub fn write_frame(&mut self, frame: &ReplayFrame) -> Result<(), ReplayError> {
        if !self.snapshot_written {
            return Err(ReplayError::MissingSnapshot);
        }
        let rec = ReplayRecord::Frame {
            frame: frame.clone(),
        };
        serde_json::to_writer(&mut self.inner, &rec)?;
        self.inner.write_all(b"\n")?;
        Ok(())
    }

    /// Write an arbitrary JSON record as a raw line. Used by the driver layer
    /// to emit format extensions (e.g., spec replay snapshots) between the
    /// structural snapshot and the first frame, without `simthing-sim` knowing
    /// about driver-level record types. The caller is responsible for tagging
    /// (`{ "kind": "...", ... }`) so readers can dispatch.
    pub fn write_extra<T: Serialize>(&mut self, value: &T) -> Result<(), ReplayError> {
        serde_json::to_writer(&mut self.inner, value)?;
        self.inner.write_all(b"\n")?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), ReplayError> {
        self.inner.flush().map_err(Into::into)
    }

    pub fn into_inner(self) -> W {
        self.inner
    }
}

// ── Reader ────────────────────────────────────────────────────────────────────

/// LDJSON replay reader. Streams records one line at a time.
pub struct ReplayReader<R: BufRead> {
    inner: R,
    snapshot_read: bool,
    buf: String,
}

impl<R: BufRead> ReplayReader<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            snapshot_read: false,
            buf: String::new(),
        }
    }

    /// Read the initial snapshot. Must be the first call.
    pub fn read_snapshot(&mut self) -> Result<ReplaySnapshot, ReplayError> {
        if self.snapshot_read {
            return Err(ReplayError::UnexpectedSnapshot);
        }
        self.buf.clear();
        let n = self.inner.read_line(&mut self.buf)?;
        if n == 0 {
            return Err(ReplayError::MissingSnapshot);
        }
        match serde_json::from_str(self.buf.trim_end())? {
            ReplayRecord::Snapshot { snapshot } => {
                self.snapshot_read = true;
                Ok(snapshot)
            }
            ReplayRecord::Frame { .. } => Err(ReplayError::MissingSnapshot),
        }
    }

    /// Read the next frame. Returns `Ok(None)` at end-of-stream.
    ///
    /// Unknown record kinds (`kind != "frame"` and `kind != "snapshot"`) are
    /// skipped silently — driver-layer extensions like `spec_snapshot` are
    /// invisible to sim-only consumers, which preserves forward compatibility.
    pub fn next_frame(&mut self) -> Result<Option<ReplayFrame>, ReplayError> {
        loop {
            self.buf.clear();
            let n = self.inner.read_line(&mut self.buf)?;
            if n == 0 {
                return Ok(None);
            }
            let line = self.buf.trim_end();
            if line.is_empty() {
                continue;
            }
            // Peek at the `kind` field via a generic Value so we can skip
            // unknown record kinds (e.g., `spec_snapshot` emitted by the
            // driver layer) without failing deserialization.
            let value: serde_json::Value = serde_json::from_str(line)?;
            let kind = value.get("kind").and_then(|v| v.as_str()).unwrap_or("");
            match kind {
                "frame" => {
                    let frame_val = value
                        .get("frame")
                        .cloned()
                        .unwrap_or(serde_json::Value::Null);
                    let frame: ReplayFrame = serde_json::from_value(frame_val)?;
                    return Ok(Some(frame));
                }
                "snapshot" => return Err(ReplayError::UnexpectedSnapshot),
                _ => continue, // skip driver-layer / future record types
            }
        }
    }
}

// ── Driver ────────────────────────────────────────────────────────────────────

/// In-memory replay state. Maintains the reconstructed tree + registry +
/// allocator as frames are applied. Equivalent in structural content to what
/// `BoundaryProtocol` carries at the same point in a live session, but does
/// not run GPU passes or maintain a shadow.
///
/// `fission_lineage` mirrors `BoundaryProtocol::fission_lineage` — it grows
/// and shrinks as `FissionLineageAdded`/`Removed` entries are applied so that
/// callers can re-register `FusionTrigger` thresholds after replay if needed.
#[derive(Debug)]
pub struct ReplayDriver {
    pub day: u32,
    pub root: SimRuntimeTree,
    pub registry: DimensionRegistry,
    pub allocator: SlotAllocator,
    pub fission_lineage: Vec<FissionLineageRecord>,
    /// Latest post-boundary shadow checkpoint from a replay frame, if any.
    pub shadow_values: Option<Vec<f32>>,
}

impl ReplayDriver {
    /// Initialize a driver from a snapshot. Allocates slots for every node in
    /// the recorded tree and seeds the fission lineage from the snapshot.
    pub fn from_snapshot(snapshot: ReplaySnapshot) -> Self {
        let mut allocator = SlotAllocator::new();
        allocator.populate_from_tree(snapshot.root.inner());
        Self {
            day: snapshot.day,
            root: snapshot.root,
            registry: snapshot.registry,
            fission_lineage: snapshot.fission_lineage,
            shadow_values: None,
            allocator,
        }
    }

    /// Apply one frame's entries to the in-memory tree. Each entry is replayed
    /// as the equivalent structural mutation:
    ///
    /// - `SimThingAdded { parent, node }`: locate parent, push node as child,
    ///   allocate slots for the entire spawned subtree.
    /// - `SimThingRemoved`: detach subtree and tombstone its slots.
    /// - `FissionOccurred { parent, node }`: same as `SimThingAdded` — locate
    ///   parent, attach node subtree, allocate slots.
    /// - `FusionOccurred`: detach child subtree and tombstone its slots.
    /// - `FissionLineageAdded` / `FissionLineageRemoved`: update
    ///   `self.fission_lineage` in-place.
    /// - `OverlayAttached`: locate target, push overlay.
    /// - `SimThingReparented`: detach + re-attach under new parent.
    /// - `PropertyExpired`: remove property from target.
    /// - `DimensionAdded`: `registry.restore(property_id)` if in range.
    /// - `VelocityAlert`: observation only, no structural effect.
    pub fn apply_frame(&mut self, frame: ReplayFrame) {
        for entry in frame.entries {
            self.apply_entry(entry);
        }
        self.day = frame.day;
        if frame.shadow_values.is_some() {
            self.shadow_values = frame.shadow_values;
        }
    }

    fn apply_entry(&mut self, entry: BoundaryDeltaEntry) {
        let inner = self.root.inner_mut();
        match entry {
            BoundaryDeltaEntry::SimThingAdded { parent, node } => {
                self.allocator.populate_from_tree(node.inner());
                if let Some(p) = find_node_mut(inner, parent) {
                    p.children.push(node.into_admitted());
                }
            }
            BoundaryDeltaEntry::FissionOccurred { parent, node } => {
                self.allocator.populate_from_tree(node.inner());
                if let Some(p) = find_node_mut(inner, parent) {
                    p.children.push(node.into_admitted());
                }
            }
            BoundaryDeltaEntry::FissionLineageAdded { record } => {
                self.fission_lineage.push(record);
            }
            BoundaryDeltaEntry::FissionLineageRemoved { record } => {
                self.fission_lineage.retain(|r| r != &record);
            }
            BoundaryDeltaEntry::OverlayAttached { target, overlay } => {
                if let Some(node) = find_node_mut(inner, target) {
                    node.overlays.push(overlay);
                }
            }
            BoundaryDeltaEntry::OverlayDissolved { target, overlay_id } => {
                if let Some(node) = find_node_mut(inner, target) {
                    node.overlays.retain(|o| o.id != overlay_id);
                }
            }
            BoundaryDeltaEntry::OverlayActivated { target, overlay_id } => {
                if let Some(node) = find_node_mut(inner, target) {
                    if let Some(overlay) = node.overlays.iter_mut().find(|o| o.id == overlay_id) {
                        if let OverlayLifecycle::Suspended { when_activated } =
                            overlay.lifecycle.clone()
                        {
                            overlay.lifecycle = *when_activated;
                        }
                    }
                }
            }
            BoundaryDeltaEntry::OverlaySuspended { target, overlay_id } => {
                if let Some(node) = find_node_mut(inner, target) {
                    if let Some(overlay) = node.overlays.iter_mut().find(|o| o.id == overlay_id) {
                        if !matches!(overlay.lifecycle, OverlayLifecycle::Suspended { .. }) {
                            overlay.lifecycle = OverlayLifecycle::Suspended {
                                when_activated: Box::new(overlay.lifecycle.clone()),
                            };
                        }
                    }
                }
            }
            BoundaryDeltaEntry::SimThingReparented { child, new_parent } => {
                if let Some(subtree) = detach_subtree(inner, child) {
                    if let Some(parent) = find_node_mut(inner, new_parent) {
                        parent.children.push(subtree);
                    }
                }
            }
            BoundaryDeltaEntry::PropertyExpired {
                sim_thing_id,
                property_id,
            } => {
                if let Some(node) = find_node_mut(inner, sim_thing_id) {
                    node.properties.remove(&property_id);
                }
            }
            BoundaryDeltaEntry::DimensionAdded { property_id } => {
                if property_id.index() < self.registry.properties.len() {
                    self.registry.restore(property_id);
                }
            }
            BoundaryDeltaEntry::SimThingRemoved { id } => {
                if let Some(detached) = detach_subtree(inner, id) {
                    tombstone_subtree(&detached, &mut self.allocator);
                }
            }
            BoundaryDeltaEntry::FusionOccurred { parent: _, child } => {
                if let Some(detached) = detach_subtree(inner, child) {
                    tombstone_subtree(&detached, &mut self.allocator);
                }
            }
            BoundaryDeltaEntry::VelocityAlert { .. } => { /* observation only */ }
            BoundaryDeltaEntry::AggregateAlert { .. } => { /* observation only */ }
        }
    }
}

// ── Internal tree helpers ─────────────────────────────────────────────────────

fn find_node_mut(root: &mut SimThing, id: SimThingId) -> Option<&mut SimThing> {
    if root.id == id {
        return Some(root);
    }
    for child in &mut root.children {
        if let Some(n) = find_node_mut(child, id) {
            return Some(n);
        }
    }
    None
}

fn detach_subtree(root: &mut SimThing, id: SimThingId) -> Option<SimThing> {
    if root.id == id {
        return None;
    }
    if let Some(pos) = root.children.iter().position(|c| c.id == id) {
        return Some(root.children.remove(pos));
    }
    for child in &mut root.children {
        if let Some(s) = detach_subtree(child, id) {
            return Some(s);
        }
    }
    None
}

fn tombstone_subtree(node: &SimThing, allocator: &mut SlotAllocator) {
    allocator.tombstone(node.id);
    for c in &node.children {
        tombstone_subtree(c, allocator);
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{DimensionRegistry, SimProperty, SimPropertyId, SimThing, SimThingKind};
    use std::io::Cursor;

    fn fixture() -> ReplaySnapshot {
        let mut registry = DimensionRegistry::new();
        registry.register(SimProperty::simple("core", "loyalty", 0));
        let mut root = SimThing::new(SimThingKind::World, 0);
        let cohort = SimThing::new(SimThingKind::Cohort, 0);
        root.add_child(cohort);
        ReplaySnapshot {
            day: 0,
            root: SimRuntimeTree::admit(root),
            registry,
            fission_lineage: Vec::new(),
        }
    }

    fn cohort_id(snap: &ReplaySnapshot) -> SimThingId {
        snap.root.direct_child_id(0).expect("cohort")
    }

}
