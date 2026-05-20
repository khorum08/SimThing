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
use crate::fission::FissionLineageRecord;

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
    pub day:             u32,
    pub root:            SimThing,
    pub registry:        DimensionRegistry,
    #[serde(default)]
    pub fission_lineage: Vec<FissionLineageRecord>,
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
///
/// `fission_lineage` mirrors `BoundaryProtocol::fission_lineage` — it grows
/// and shrinks as `FissionLineageAdded`/`Removed` entries are applied so that
/// callers can re-register `FusionTrigger` thresholds after replay if needed.
#[derive(Debug)]
pub struct ReplayDriver {
    pub day:             u32,
    pub root:            SimThing,
    pub registry:        DimensionRegistry,
    pub allocator:       SlotAllocator,
    pub fission_lineage: Vec<FissionLineageRecord>,
}

impl ReplayDriver {
    /// Initialize a driver from a snapshot. Allocates slots for every node in
    /// the recorded tree and seeds the fission lineage from the snapshot.
    pub fn from_snapshot(snapshot: ReplaySnapshot) -> Self {
        let mut allocator = SlotAllocator::new();
        allocator.populate_from_tree(&snapshot.root);
        Self {
            day:             snapshot.day,
            root:            snapshot.root,
            registry:        snapshot.registry,
            fission_lineage: snapshot.fission_lineage,
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
    }

    fn apply_entry(&mut self, entry: BoundaryDeltaEntry) {
        match entry {
            BoundaryDeltaEntry::SimThingAdded { parent, node } => {
                // Allocate slots for every node in the spawned subtree before
                // attaching it, so allocator state stays consistent.
                self.allocator.populate_from_tree(&node);
                if let Some(p) = find_node_mut(&mut self.root, parent) {
                    p.children.push(node);
                }
                // If parent not found: slots were allocated but tree is not
                // mutated — mirrors live behavior that rejects unknown targets.
            }
            BoundaryDeltaEntry::FissionOccurred { parent, node } => {
                // Same shape as SimThingAdded: locate parent, attach subtree.
                self.allocator.populate_from_tree(&node);
                if let Some(p) = find_node_mut(&mut self.root, parent) {
                    p.children.push(node);
                }
            }
            BoundaryDeltaEntry::FissionLineageAdded { record } => {
                self.fission_lineage.push(record);
            }
            BoundaryDeltaEntry::FissionLineageRemoved { record } => {
                self.fission_lineage.retain(|r| r != &record);
            }
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
                // rather than panic.
                if property_id.index() < self.registry.properties.len() {
                    self.registry.restore(property_id);
                }
            }
            BoundaryDeltaEntry::SimThingRemoved { id } => {
                if let Some(detached) = detach_subtree(&mut self.root, id) {
                    tombstone_subtree(&detached, &mut self.allocator);
                }
            }
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
        ReplaySnapshot { day: 0, root, registry, fission_lineage: Vec::new() }
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

    #[test]
    fn driver_replays_sim_thing_added() {
        let snap = fixture();
        let root_id = snap.root.id;
        let mut driver = ReplayDriver::from_snapshot(snap);

        // Spawn a new fleet node and deliver it as SimThingAdded under root.
        let fleet = SimThing::new(SimThingKind::Fleet, 0);
        let fleet_id = fleet.id;

        let frame = ReplayFrame {
            day:     1,
            entries: vec![BoundaryDeltaEntry::SimThingAdded {
                parent: root_id,
                node:   fleet,
            }],
        };
        driver.apply_frame(frame);

        // Node must appear in the tree under root.
        assert!(driver.root.children.iter().any(|c| c.id == fleet_id),
            "fleet must be a direct child of root after SimThingAdded");
        // Slot must be allocated.
        assert!(driver.allocator.slot_of(fleet_id).is_some(),
            "fleet slot must be allocated");
    }

    #[test]
    fn driver_replays_fission_occurred_with_node() {
        let snap = fixture();
        let cohort_id = snap.root.children[0].id;
        let mut driver = ReplayDriver::from_snapshot(snap);

        // Simulate a fission child spawned under the cohort.
        let rebel = SimThing::new(SimThingKind::Cohort, 0);
        let rebel_id = rebel.id;

        let frame = ReplayFrame {
            day:     1,
            entries: vec![BoundaryDeltaEntry::FissionOccurred {
                parent: cohort_id,
                node:   rebel,
            }],
        };
        driver.apply_frame(frame);

        // Fission child must appear under the cohort.
        let cohort = driver.root.children.iter().find(|c| c.id == cohort_id).unwrap();
        assert_eq!(cohort.children.len(), 1, "one fission child under cohort");
        assert_eq!(cohort.children[0].id, rebel_id);
        assert!(driver.allocator.slot_of(rebel_id).is_some(), "rebel slot allocated");
    }

    #[test]
    fn driver_replays_fission_lineage_round_trip() {
        use crate::fission::FissionLineageRecord;
        use simthing_core::SimPropertyId;

        let snap = fixture();
        let cohort_id = snap.root.children[0].id;
        let rebel_id  = SimThing::new(SimThingKind::Cohort, 0).id;
        let mut driver = ReplayDriver::from_snapshot(snap);

        let record = FissionLineageRecord {
            parent_id:    cohort_id,
            child_id:     rebel_id,
            property_id:  SimPropertyId(0),
            template_idx: 0,
        };

        // Add then remove.
        let frame = ReplayFrame {
            day: 1,
            entries: vec![
                BoundaryDeltaEntry::FissionLineageAdded { record },
            ],
        };
        driver.apply_frame(frame);
        assert_eq!(driver.fission_lineage.len(), 1);
        assert_eq!(driver.fission_lineage[0].child_id, rebel_id);

        let frame2 = ReplayFrame {
            day: 2,
            entries: vec![
                BoundaryDeltaEntry::FissionLineageRemoved { record },
            ],
        };
        driver.apply_frame(frame2);
        assert!(driver.fission_lineage.is_empty(), "lineage removed");
    }

    #[test]
    fn snapshot_carries_fission_lineage_through_serde() {
        use crate::fission::FissionLineageRecord;
        use simthing_core::SimPropertyId;
        use std::io::Cursor;

        let mut snap = fixture();
        let cohort_id = snap.root.children[0].id;
        let rebel_id  = SimThing::new(SimThingKind::Cohort, 0).id;
        snap.fission_lineage.push(FissionLineageRecord {
            parent_id:    cohort_id,
            child_id:     rebel_id,
            property_id:  SimPropertyId(0),
            template_idx: 0,
        });

        let mut buf: Vec<u8> = Vec::new();
        let mut writer = ReplayWriter::new(&mut buf);
        writer.write_snapshot(&snap).unwrap();
        drop(writer);

        let mut reader = ReplayReader::new(Cursor::new(buf));
        let restored = reader.read_snapshot().unwrap();
        assert_eq!(restored.fission_lineage.len(), 1);
        assert_eq!(restored.fission_lineage[0].child_id, rebel_id);
    }
}
