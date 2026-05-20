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
//! ## Scope: structural reproduction
//!
//! `ReplayDriver` reconstructs the **tree structure, dimension registry, and
//! slot allocator** from a recorded session. It does **not** re-run GPU
//! passes — float values from velocity integration and overlay application are
//! not part of the replay surface (they're recomputed each session and would
//! make the log too large). Threshold-driven events (fission, expiry) survive
//! as `BoundaryDeltaEntry`s and are replayed at the structural level.
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

use std::io::{BufRead, Write};

use serde::{Deserialize, Serialize};
use simthing_core::{DimensionRegistry, SimThing, SimThingId};
use simthing_gpu::SlotAllocator;

use crate::delta_log::BoundaryDeltaEntry;

// ── Snapshot ──────────────────────────────────────────────────────────────────

/// Initial state captured at the start of a recording. Subsequent frames are
/// applied on top of this to reconstruct later state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplaySnapshot {
    pub day:      u32,
    pub root:     SimThing,
    pub registry: DimensionRegistry,
}

// ── Frame ─────────────────────────────────────────────────────────────────────

/// One day's worth of structural changes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplayFrame {
    pub day:     u32,
    pub entries: Vec<BoundaryDeltaEntry>,
}

/// Discriminated record written one-per-line to the replay stream.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ReplayRecord {
    Snapshot { snapshot: ReplaySnapshot },
    Frame    { frame:    ReplayFrame },
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
    inner:           W,
    snapshot_written: bool,
}

impl<W: Write> ReplayWriter<W> {
    pub fn new(inner: W) -> Self {
        Self { inner, snapshot_written: false }
    }

    /// Write the initial snapshot. Must be called before any frames.
    pub fn write_snapshot(&mut self, snapshot: &ReplaySnapshot) -> Result<(), ReplayError> {
        let rec = ReplayRecord::Snapshot { snapshot: snapshot.clone() };
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
        let rec = ReplayRecord::Frame { frame: frame.clone() };
        serde_json::to_writer(&mut self.inner, &rec)?;
        self.inner.write_all(b"\n")?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), ReplayError> {
        self.inner.flush().map_err(Into::into)
    }

    pub fn into_inner(self) -> W { self.inner }
}

// ── Reader ────────────────────────────────────────────────────────────────────

/// LDJSON replay reader. Streams records one line at a time.
pub struct ReplayReader<R: BufRead> {
    inner:         R,
    snapshot_read: bool,
    buf:           String,
}

impl<R: BufRead> ReplayReader<R> {
    pub fn new(inner: R) -> Self {
        Self { inner, snapshot_read: false, buf: String::new() }
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
    pub fn next_frame(&mut self) -> Result<Option<ReplayFrame>, ReplayError> {
        loop {
            self.buf.clear();
            let n = self.inner.read_line(&mut self.buf)?;
            if n == 0 {
                return Ok(None);
            }
            let line = self.buf.trim_end();
            if line.is_empty() { continue; }
            match serde_json::from_str(line)? {
                ReplayRecord::Frame { frame } => return Ok(Some(frame)),
                ReplayRecord::Snapshot { .. } => return Err(ReplayError::UnexpectedSnapshot),
            }
        }
    }
}

// ── Driver ────────────────────────────────────────────────────────────────────

/// In-memory replay state. Maintains the reconstructed tree + registry +
/// allocator as frames are applied. Equivalent in structural content to what
/// `BoundaryProtocol` carries at the same point in a live session, but does
/// not run GPU passes or maintain a shadow.
#[derive(Debug)]
pub struct ReplayDriver {
    pub day:       u32,
    pub root:      SimThing,
    pub registry:  DimensionRegistry,
    pub allocator: SlotAllocator,
}

impl ReplayDriver {
    /// Initialize a driver from a snapshot. Allocates slots for every node in
    /// the recorded tree.
    pub fn from_snapshot(snapshot: ReplaySnapshot) -> Self {
        let mut allocator = SlotAllocator::new();
        allocator.populate_from_tree(&snapshot.root);
        Self {
            day:       snapshot.day,
            root:      snapshot.root,
            registry:  snapshot.registry,
            allocator,
        }
    }

    /// Apply one frame's entries to the in-memory tree. Each entry is replayed
    /// as the equivalent structural mutation:
    ///
    /// - `SimThingAdded` / `SimThingRemoved`: skipped — we cannot re-derive
    ///   the spawned subtree from id alone. Use `OverlayAttached`-style full
    ///   payload variants once they exist for these.
    /// - `OverlayAttached`: located by `target`, overlay pushed onto its
    ///   `overlays` vec.
    /// - `SimThingReparented`: subtree detached and re-attached.
    /// - `PropertyExpired`: property removed from the target's properties map.
    /// - `DimensionAdded`: `registry.restore(property_id)`.
    /// - `FissionOccurred` / `FusionOccurred`: structurally observed but not
    ///   recreatable from id alone; see structural reproduction caveat in the
    ///   module doc.
    ///
    /// The lossy variants (`SimThingAdded`, `FissionOccurred`) are a known
    /// limitation — the current delta log records ids without the spawned
    /// `SimThing` payload. A `SimThingSpawned { node: SimThing }` variant
    /// would close this gap; see worklog "Next session pickup."
    pub fn apply_frame(&mut self, frame: ReplayFrame) {
        for entry in frame.entries {
            self.apply_entry(entry);
        }
        self.day = frame.day;
    }

    fn apply_entry(&mut self, entry: BoundaryDeltaEntry) {
        match entry {
            BoundaryDeltaEntry::OverlayAttached { target, overlay } => {
                if let Some(node) = find_node_mut(&mut self.root, target) {
                    node.overlays.push(overlay);
                }
            }
            BoundaryDeltaEntry::SimThingReparented { child, new_parent } => {
                if let Some(subtree) = detach_subtree(&mut self.root, child) {
                    if let Some(parent) = find_node_mut(&mut self.root, new_parent) {
                        parent.children.push(subtree);
                    }
                    // If new_parent vanished, the subtree is dropped — replay
                    // mirrors live behavior, which rejects unknown targets.
                }
            }
            BoundaryDeltaEntry::PropertyExpired { sim_thing_id, property_id } => {
                if let Some(node) = find_node_mut(&mut self.root, sim_thing_id) {
                    node.properties.remove(&property_id);
                }
            }
            BoundaryDeltaEntry::DimensionAdded { property_id } => {
                // The recorded property must exist in the snapshot's registry
                // for restore to succeed. If it was registered live after the
                // snapshot was taken, the replay can't see it — skip silently
                // rather than panic. Matches the live `AddDimension` handler's
                // bounds check.
                if property_id.index() < self.registry.properties.len() {
                    self.registry.restore(property_id);
                }
            }
            BoundaryDeltaEntry::SimThingRemoved { id } => {
                if let Some(detached) = detach_subtree(&mut self.root, id) {
                    tombstone_subtree(&detached, &mut self.allocator);
                }
            }
            // Lossy variants — structural intent observed, payload not reconstructable.
            BoundaryDeltaEntry::SimThingAdded   { .. } => { /* lossy */ }
            BoundaryDeltaEntry::FissionOccurred { .. } => { /* lossy */ }
            BoundaryDeltaEntry::FusionOccurred { parent: _, child } => {
                // Best-effort: if the child still exists structurally, remove it.
                if let Some(detached) = detach_subtree(&mut self.root, child) {
                    tombstone_subtree(&detached, &mut self.allocator);
                }
            }
            BoundaryDeltaEntry::VelocityAlert { .. } => { /* observation only */ }
        }
    }
}

// ── Internal tree helpers ─────────────────────────────────────────────────────

fn find_node_mut(root: &mut SimThing, id: SimThingId) -> Option<&mut SimThing> {
    if root.id == id { return Some(root); }
    for child in &mut root.children {
        if let Some(n) = find_node_mut(child, id) { return Some(n); }
    }
    None
}

fn detach_subtree(root: &mut SimThing, id: SimThingId) -> Option<SimThing> {
    if root.id == id {
        // Cannot detach the root itself.
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
    use std::io::Cursor;
    use simthing_core::{
        DimensionRegistry, SimProperty, SimPropertyId, SimThing, SimThingKind,
    };

    fn fixture() -> ReplaySnapshot {
        let mut registry = DimensionRegistry::new();
        registry.register(SimProperty::simple("core", "loyalty", 0));
        let mut root = SimThing::new(SimThingKind::World, 0);
        let cohort = SimThing::new(SimThingKind::Cohort, 0);
        root.add_child(cohort);
        ReplaySnapshot { day: 0, root, registry }
    }

    #[test]
    fn snapshot_round_trips_through_ldjson() {
        let snap = fixture();
        let snap_id = snap.root.id;
        let snap_child_id = snap.root.children[0].id;

        let mut buf: Vec<u8> = Vec::new();
        let mut writer = ReplayWriter::new(&mut buf);
        writer.write_snapshot(&snap).unwrap();
        drop(writer);

        let mut reader = ReplayReader::new(Cursor::new(buf));
        let restored = reader.read_snapshot().unwrap();
        assert_eq!(restored.day, 0);
        assert_eq!(restored.root.id, snap_id);
        assert_eq!(restored.root.children[0].id, snap_child_id);
        assert_eq!(restored.registry.properties.len(), 1);
    }

    #[test]
    fn writer_rejects_frame_before_snapshot() {
        let mut buf: Vec<u8> = Vec::new();
        let mut writer = ReplayWriter::new(&mut buf);
        let frame = ReplayFrame { day: 1, entries: Vec::new() };
        let err = writer.write_frame(&frame).unwrap_err();
        assert!(matches!(err, ReplayError::MissingSnapshot));
    }

    #[test]
    fn reader_returns_none_after_last_frame() {
        let snap = fixture();
        let mut buf: Vec<u8> = Vec::new();
        let mut writer = ReplayWriter::new(&mut buf);
        writer.write_snapshot(&snap).unwrap();
        writer.write_frame(&ReplayFrame { day: 1, entries: Vec::new() }).unwrap();
        drop(writer);

        let mut reader = ReplayReader::new(Cursor::new(buf));
        let _ = reader.read_snapshot().unwrap();
        assert!(reader.next_frame().unwrap().is_some());
        assert!(reader.next_frame().unwrap().is_none());
    }

    #[test]
    fn driver_replays_overlay_attached() {
        use simthing_core::{
            OverlayId, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta,
            SubFieldRole, TransformOp,
        };

        let snap = fixture();
        let cohort_id = snap.root.children[0].id;
        let mut driver = ReplayDriver::from_snapshot(snap);

        let overlay = simthing_core::Overlay {
            id:        OverlayId::new(),
            kind:      OverlayKind::Policy,
            source:    OverlaySource::Player,
            affects:   Vec::new(),
            transform: PropertyTransformDelta {
                property_id:      SimPropertyId(0),
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Set(0.42))],
            },
            lifecycle: OverlayLifecycle::Permanent,
        };
        let oid = overlay.id;
        let frame = ReplayFrame {
            day:     1,
            entries: vec![BoundaryDeltaEntry::OverlayAttached {
                target: cohort_id, overlay,
            }],
        };
        driver.apply_frame(frame);

        assert_eq!(driver.day, 1);
        assert_eq!(driver.root.children[0].overlays.len(), 1);
        assert_eq!(driver.root.children[0].overlays[0].id, oid);
    }

    #[test]
    fn driver_replays_property_expired() {
        let mut snap = fixture();
        let pid = SimPropertyId(0);
        // Seed the cohort with the property so we can verify removal.
        snap.root.children[0].add_property(pid, snap.registry.property(pid).default_value());
        let cohort_id = snap.root.children[0].id;

        let mut driver = ReplayDriver::from_snapshot(snap);
        assert!(driver.root.children[0].properties.contains_key(&pid));

        let frame = ReplayFrame {
            day:     1,
            entries: vec![BoundaryDeltaEntry::PropertyExpired {
                sim_thing_id: cohort_id, property_id: pid,
            }],
        };
        driver.apply_frame(frame);
        assert!(!driver.root.children[0].properties.contains_key(&pid));
    }

    #[test]
    fn driver_replays_reparent() {
        let mut snap = fixture();
        // Add a second sibling so we have somewhere to reparent under.
        let sib = SimThing::new(SimThingKind::Location, 0);
        let sib_id = sib.id;
        snap.root.add_child(sib);
        let cohort_id = snap.root.children[0].id;

        let mut driver = ReplayDriver::from_snapshot(snap);

        let frame = ReplayFrame {
            day:     1,
            entries: vec![BoundaryDeltaEntry::SimThingReparented {
                child:     cohort_id,
                new_parent: sib_id,
            }],
        };
        driver.apply_frame(frame);

        // cohort moved out of root's first slot and under sib.
        let sib = driver.root.children.iter().find(|c| c.id == sib_id).unwrap();
        assert_eq!(sib.children.len(), 1);
        assert_eq!(sib.children[0].id, cohort_id);
        assert!(!driver.root.children.iter().any(|c| c.id == cohort_id));
    }
}
