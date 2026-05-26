//! Replay v3 — spec session state serialization.
//!
//! Adds a `SpecSnapshot` at recording start and per-frame `SpecDelta`s on top
//! of the structural replay v2 stream. The format is additive: old replays
//! without spec records open cleanly, and sim-only consumers skip the
//! `spec_snapshot` line and ignore `spec_entries` on frames.
//!
//! ## Logical-key invariant
//!
//! Every cross-reference uses an authored string id (`tree_id`, `event_id`)
//! or a logical compound key (`CapabilityEntryKey`, `CategoryKey`). Raw
//! `OverlayId`s are never serialized for spec state — they are process-local
//! atomic ids that change every install. See
//! `docs/adr/spec_session_state_replay.md` §`OverlayId stability`.
//!
//! ## LDJSON stream extension
//!
//! ```text
//! { "kind": "snapshot", "snapshot": { … } }              // existing v2
//! { "kind": "spec_snapshot", "spec_snapshot": { … } }    // new v3, optional
//! { "kind": "frame", "day": 1, "entries": [ … ], "spec_entries": [ … ] }
//! ```
//!
//! The driver writes the spec snapshot via `ReplayWriter::write_extra` and
//! attaches `spec_entries` to each frame as opaque JSON values (so
//! `simthing-sim` stays spec-free).

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use serde::{Deserialize, Serialize};
use simthing_core::SimThingId;
use simthing_sim::{ReplayDriver, ReplayFrame, ReplaySnapshot};
use simthing_spec::{
    ActivationMode, CapabilityEntryKey, CapabilityTreeNotification, CategoryKey, EventKey,
    GameModeSpec, ScriptedEventInstanceKey,
};
use thiserror::Error;

use crate::scenario::Scenario;
use crate::session::{SessionError, SimSession};
use crate::spec_session::{CapabilityInstanceKey, PreBoundarySnapshot, SpecSessionState};

// ── Snapshot types ────────────────────────────────────────────────────────────

/// Serialized form of the mutable spec-layer state captured at recording start.
///
/// Reconstructable fields (`capability_definitions`, `capability_instances`,
/// `scripted_event_definitions`) are rebuilt by re-running install on replay
/// open and are deliberately absent here.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpecSnapshot {
    pub day: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capability_states: Vec<CapabilityStateSnapshot>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scripted_cooldowns: Vec<ScriptedCooldownSnapshot>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub queued_selections: Vec<QueuedSelectionSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapabilityStateSnapshot {
    pub owner_id: SimThingId,
    pub tree_thing_id: SimThingId,
    /// `CapabilityTreeDefinition.tree_id` — authored string, stable across
    /// process boundaries.
    pub definition_logical_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activation_modes: Vec<(CapabilityEntryKey, ActivationMode)>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub active_by_category: Vec<(CategoryKey, Vec<CapabilityEntryKey>)>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptedCooldownSnapshot {
    pub owner_id: SimThingId,
    pub event_id: EventKey,
    pub cooldown_remaining: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueuedSelectionSnapshot {
    pub owner_id: SimThingId,
    pub definition_logical_id: String,
    pub entry: CapabilityEntryKey,
}

// ── Per-frame deltas ──────────────────────────────────────────────────────────

/// Per-boundary spec-state mutation. Emitted by diffing
/// `PreBoundarySnapshot` vs. the post-boundary `SpecSessionState`. Replayed
/// in order against a freshly-installed session to reproduce the same state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SpecDelta {
    CapabilityActivationModeChanged {
        owner_id: SimThingId,
        definition_logical_id: String,
        entry: CapabilityEntryKey,
        /// `None` clears the runtime override (entry reverts to authored
        /// default). `Some(mode)` installs the override.
        mode: Option<ActivationMode>,
    },
    CapabilityActiveSetChanged {
        owner_id: SimThingId,
        definition_logical_id: String,
        category: CategoryKey,
        /// Full new active list for the category (replace, not append).
        /// Empty vec means the category has no active entries.
        active: Vec<CapabilityEntryKey>,
    },
    CapabilityNotification(CapabilityTreeNotification),
    ScriptedCooldownChanged {
        owner_id: SimThingId,
        event_id: EventKey,
        cooldown_remaining: u32,
    },
    ScriptedInstanceSlotChanged {
        owner_id: SimThingId,
        event_id: EventKey,
        current_slot: u32,
    },
    ScriptedInstanceRemoved {
        owner_id: SimThingId,
        event_id: EventKey,
    },
    PlayerSelectionQueued {
        owner_id: SimThingId,
        definition_logical_id: String,
        entry: CapabilityEntryKey,
    },
}

// ── Errors ────────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum ReplayOpenError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("replay: {0}")]
    Replay(#[from] simthing_sim::ReplayError),
    #[error("session: {0}")]
    Session(#[from] SessionError),
    #[error("replay missing structural snapshot")]
    MissingStructuralSnapshot,
    #[error(
        "capability tree `{definition_logical_id}` for owner `{owner_id:?}` not present in installed spec"
    )]
    UnknownCapabilityTree {
        owner_id: SimThingId,
        definition_logical_id: String,
    },
    #[error("capability state missing for {owner_id:?}/{definition_logical_id}")]
    MissingCapabilityState {
        owner_id: SimThingId,
        definition_logical_id: String,
    },
    #[error("could not resolve queued player selection: {0}")]
    SelectionResolution(String),
}

// ── Wrapper for the spec_snapshot record line ────────────────────────────────

#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum SpecRecord {
    SpecSnapshot { spec_snapshot: SpecSnapshot },
}

/// Build a `kind: "spec_snapshot"` line value for `ReplayWriter::write_extra`.
pub fn make_spec_snapshot_record(snapshot: SpecSnapshot) -> impl Serialize {
    SpecRecord::SpecSnapshot {
        spec_snapshot: snapshot,
    }
}

// ── Collection (live state → snapshot) ────────────────────────────────────────

/// Capture the mutable spec-layer state for serialization at recording start.
pub fn collect_spec_snapshot(state: &SpecSessionState, day: u32) -> SpecSnapshot {
    let def_logical = |def_id| {
        state
            .capability_definitions
            .get(&def_id)
            .map(|d| d.tree_id.clone())
            .unwrap_or_default()
    };

    let mut capability_states: Vec<CapabilityStateSnapshot> = state
        .capability_states
        .iter()
        .filter(|(_, st)| {
            !st.activation_mode_by_entry.is_empty() || !st.active_by_category.is_empty()
        })
        .map(|(key, st)| {
            let mut activation_modes: Vec<_> = st
                .activation_mode_by_entry
                .iter()
                .map(|(k, v)| (k.clone(), *v))
                .collect();
            activation_modes.sort_by(|a, b| {
                a.0.category
                    .namespace
                    .cmp(&b.0.category.namespace)
                    .then_with(|| a.0.category.name.cmp(&b.0.category.name))
                    .then_with(|| a.0.entry_id.cmp(&b.0.entry_id))
            });
            let mut active_by_category: Vec<_> = st
                .active_by_category
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            active_by_category.sort_by(|a, b| {
                a.0.namespace
                    .cmp(&b.0.namespace)
                    .then_with(|| a.0.name.cmp(&b.0.name))
            });
            CapabilityStateSnapshot {
                owner_id: key.owner_id,
                tree_thing_id: key.tree_thing_id,
                definition_logical_id: def_logical(key.definition_id),
                activation_modes,
                active_by_category,
            }
        })
        .collect();
    capability_states.sort_by(|a, b| {
        a.owner_id
            .cmp(&b.owner_id)
            .then_with(|| a.definition_logical_id.cmp(&b.definition_logical_id))
    });

    let mut scripted_cooldowns: Vec<ScriptedCooldownSnapshot> = state
        .scripted_event_instances
        .values()
        .filter(|inst| inst.cooldown_remaining > 0)
        .map(|inst| ScriptedCooldownSnapshot {
            owner_id: inst.key.owner_id,
            event_id: inst.key.event_id.clone(),
            cooldown_remaining: inst.cooldown_remaining,
        })
        .collect();
    scripted_cooldowns.sort_by(|a, b| {
        a.owner_id
            .cmp(&b.owner_id)
            .then_with(|| a.event_id.0.cmp(&b.event_id.0))
    });

    SpecSnapshot {
        day,
        capability_states,
        scripted_cooldowns,
        queued_selections: Vec::new(), // selections are surfaced via per-frame deltas
    }
}

// ── Diff (pre vs post → deltas) ───────────────────────────────────────────────

/// Diff the pre-boundary snapshot against the post-boundary state and produce
/// `SpecDelta`s capturing every change. Notifications drained from the
/// post-boundary state are appended as `CapabilityNotification` deltas.
pub fn diff_and_emit(
    pre: &PreBoundarySnapshot,
    after: &SpecSessionState,
    drained_notifications: Vec<CapabilityTreeNotification>,
) -> Vec<SpecDelta> {
    let mut deltas = Vec::new();

    let def_logical = |def_id| {
        after
            .capability_definitions
            .get(&def_id)
            .map(|d| d.tree_id.clone())
            .unwrap_or_default()
    };

    // 1. Player selections that were pending before this boundary (drained by
    //    the handler during this tick). Replaying these re-queues the same
    //    selection, which lands as `OverlayActivated` in the structural log.
    for (instance_key, entry_key) in &pre.pending_selections {
        let logical = def_logical(instance_key.definition_id);
        deltas.push(SpecDelta::PlayerSelectionQueued {
            owner_id: instance_key.owner_id,
            definition_logical_id: logical,
            entry: entry_key.clone(),
        });
    }

    // 2. Capability state diffs (activation modes + active sets per category).
    //    Sort keys for deterministic delta order so byte-equal replays produce
    //    byte-equal deltas across runs.
    let mut cap_keys: Vec<_> = after
        .capability_states
        .keys()
        .chain(pre.capability_states.keys())
        .copied()
        .collect();
    cap_keys.sort_by(|a, b| {
        a.owner_id
            .cmp(&b.owner_id)
            .then_with(|| a.tree_thing_id.cmp(&b.tree_thing_id))
    });
    cap_keys.dedup();

    for key in cap_keys {
        let empty = empty_state(key);
        let before = pre.capability_states.get(&key).unwrap_or(&empty);
        let after_st = match after.capability_states.get(&key) {
            Some(s) => s,
            None => continue, // instance dropped — no spec delta needed (handled by structural)
        };
        let logical = def_logical(key.definition_id);

        // activation_mode_by_entry: emit Some for changed/new entries, None
        // for entries that were present before and absent after.
        let mut entries: Vec<_> = after_st
            .activation_mode_by_entry
            .keys()
            .chain(before.activation_mode_by_entry.keys())
            .cloned()
            .collect();
        entries.sort_by(|a, b| {
            a.category
                .namespace
                .cmp(&b.category.namespace)
                .then_with(|| a.category.name.cmp(&b.category.name))
                .then_with(|| a.entry_id.cmp(&b.entry_id))
        });
        entries.dedup();
        for entry in entries {
            let bm = before.activation_mode_by_entry.get(&entry);
            let am = after_st.activation_mode_by_entry.get(&entry);
            if bm != am {
                deltas.push(SpecDelta::CapabilityActivationModeChanged {
                    owner_id: key.owner_id,
                    definition_logical_id: logical.clone(),
                    entry,
                    mode: am.copied(),
                });
            }
        }

        // active_by_category: emit replace-or-clear for each category that
        // changed.
        let mut cats: Vec<_> = after_st
            .active_by_category
            .keys()
            .chain(before.active_by_category.keys())
            .cloned()
            .collect();
        cats.sort_by(|a, b| {
            a.namespace
                .cmp(&b.namespace)
                .then_with(|| a.name.cmp(&b.name))
        });
        cats.dedup();
        for category in cats {
            let bv = before.active_by_category.get(&category);
            let av = after_st.active_by_category.get(&category);
            if bv.map(|v| v.as_slice()) != av.map(|v| v.as_slice()) {
                deltas.push(SpecDelta::CapabilityActiveSetChanged {
                    owner_id: key.owner_id,
                    definition_logical_id: logical.clone(),
                    category,
                    active: av.cloned().unwrap_or_default(),
                });
            }
        }
    }

    // 3. Scripted-event instance diffs (cooldown + slot churn + removal).
    let mut se_keys: Vec<_> = after
        .scripted_event_instances
        .keys()
        .chain(pre.scripted_event_instances.keys())
        .cloned()
        .collect();
    se_keys.sort_by(|a, b| {
        a.owner_id
            .cmp(&b.owner_id)
            .then_with(|| a.event_id.0.cmp(&b.event_id.0))
    });
    se_keys.dedup();

    for key in se_keys {
        match (
            pre.scripted_event_instances.get(&key),
            after.scripted_event_instances.get(&key),
        ) {
            (Some(b), Some(a)) => {
                if a.cooldown_remaining != b.cooldown_remaining {
                    deltas.push(SpecDelta::ScriptedCooldownChanged {
                        owner_id: key.owner_id.clone(),
                        event_id: key.event_id.clone(),
                        cooldown_remaining: a.cooldown_remaining,
                    });
                }
                if a.current_slot != b.current_slot {
                    deltas.push(SpecDelta::ScriptedInstanceSlotChanged {
                        owner_id: key.owner_id.clone(),
                        event_id: key.event_id.clone(),
                        current_slot: a.current_slot,
                    });
                }
            }
            (None, Some(a)) => {
                // Newly attached mid-session (rare — install-time attach is
                // covered by re-running install on replay open). Emit
                // cooldown if non-zero.
                if a.cooldown_remaining > 0 {
                    deltas.push(SpecDelta::ScriptedCooldownChanged {
                        owner_id: key.owner_id.clone(),
                        event_id: key.event_id.clone(),
                        cooldown_remaining: a.cooldown_remaining,
                    });
                }
            }
            (Some(_), None) => {
                deltas.push(SpecDelta::ScriptedInstanceRemoved {
                    owner_id: key.owner_id,
                    event_id: key.event_id,
                });
            }
            (None, None) => {}
        }
    }

    // 4. Notifications drained from the post-boundary state. Append last so
    //    the replay order matches the live order (state mutations land first,
    //    then notifications fire).
    for n in drained_notifications {
        deltas.push(SpecDelta::CapabilityNotification(n));
    }

    deltas
}

fn empty_state(key: CapabilityInstanceKey) -> simthing_spec::CapabilityTreeState {
    simthing_spec::CapabilityTreeState {
        owner_id: key.owner_id,
        definition_id: key.definition_id,
        activation_mode_by_entry: HashMap::new(),
        active_by_category: HashMap::new(),
    }
}

// ── JSON ↔ SpecDelta bridges (frame.spec_entries: Vec<serde_json::Value>) ────

/// Serialize a slice of `SpecDelta`s into the opaque JSON values stored on
/// `ReplayFrame::spec_entries`.
pub fn spec_deltas_to_json(deltas: &[SpecDelta]) -> Vec<serde_json::Value> {
    deltas
        .iter()
        .filter_map(|d| serde_json::to_value(d).ok())
        .collect()
}

/// Deserialize the opaque JSON values from `ReplayFrame::spec_entries` back
/// into `SpecDelta`s.
pub fn json_to_spec_deltas(
    values: &[serde_json::Value],
) -> Result<Vec<SpecDelta>, serde_json::Error> {
    values
        .iter()
        .map(|v| serde_json::from_value(v.clone()))
        .collect()
}

// ── Apply (snapshot / delta → live state) ────────────────────────────────────

/// Apply a `SpecSnapshot` to a freshly-installed `SpecSessionState`. The
/// session must already have `compile_and_install` run (so capability
/// definitions/instances and scripted-event instances exist) — the snapshot
/// only restores mutable state (cooldowns, activation modes, active sets,
/// queued selections).
pub fn apply_spec_snapshot(
    state: &mut SpecSessionState,
    snapshot: &SpecSnapshot,
) -> Result<(), ReplayOpenError> {
    for cs in &snapshot.capability_states {
        let key = find_cap_instance_key(state, cs.owner_id, &cs.definition_logical_id).ok_or_else(
            || ReplayOpenError::UnknownCapabilityTree {
                owner_id: cs.owner_id,
                definition_logical_id: cs.definition_logical_id.clone(),
            },
        )?;
        let st = state.capability_states.get_mut(&key).ok_or_else(|| {
            ReplayOpenError::MissingCapabilityState {
                owner_id: cs.owner_id,
                definition_logical_id: cs.definition_logical_id.clone(),
            }
        })?;
        st.activation_mode_by_entry = cs.activation_modes.iter().cloned().collect();
        st.active_by_category = cs.active_by_category.iter().cloned().collect();
    }

    for cd in &snapshot.scripted_cooldowns {
        let inst_key = ScriptedEventInstanceKey {
            owner_id: cd.owner_id,
            event_id: cd.event_id.clone(),
        };
        if let Some(inst) = state.scripted_event_instances.get_mut(&inst_key) {
            inst.cooldown_remaining = cd.cooldown_remaining;
        }
        // Silently skip cooldowns for events not in the installed spec —
        // matches the "spec must be a superset" doc invariant; the diagnostic
        // path catches genuine mismatches via the per-delta apply.
    }

    for sel in &snapshot.queued_selections {
        state
            .queue_player_selection_by_key(
                sel.owner_id,
                &sel.definition_logical_id,
                &sel.entry.entry_id,
            )
            .map_err(|e| ReplayOpenError::SelectionResolution(e.to_string()))?;
    }

    Ok(())
}

/// Apply a single `SpecDelta` to live state. Used by frame-by-frame replay
/// (e.g., a viewer scrubbing through history).
pub fn apply_spec_delta(
    state: &mut SpecSessionState,
    delta: &SpecDelta,
) -> Result<(), ReplayOpenError> {
    match delta {
        SpecDelta::CapabilityActivationModeChanged {
            owner_id,
            definition_logical_id,
            entry,
            mode,
        } => {
            let key = find_cap_instance_key(state, *owner_id, definition_logical_id).ok_or_else(
                || ReplayOpenError::UnknownCapabilityTree {
                    owner_id: *owner_id,
                    definition_logical_id: definition_logical_id.clone(),
                },
            )?;
            let st = state.capability_states.get_mut(&key).ok_or_else(|| {
                ReplayOpenError::MissingCapabilityState {
                    owner_id: *owner_id,
                    definition_logical_id: definition_logical_id.clone(),
                }
            })?;
            match mode {
                Some(m) => {
                    st.activation_mode_by_entry.insert(entry.clone(), *m);
                }
                None => {
                    st.activation_mode_by_entry.remove(entry);
                }
            }
        }
        SpecDelta::CapabilityActiveSetChanged {
            owner_id,
            definition_logical_id,
            category,
            active,
        } => {
            let key = find_cap_instance_key(state, *owner_id, definition_logical_id).ok_or_else(
                || ReplayOpenError::UnknownCapabilityTree {
                    owner_id: *owner_id,
                    definition_logical_id: definition_logical_id.clone(),
                },
            )?;
            let st = state.capability_states.get_mut(&key).ok_or_else(|| {
                ReplayOpenError::MissingCapabilityState {
                    owner_id: *owner_id,
                    definition_logical_id: definition_logical_id.clone(),
                }
            })?;
            if active.is_empty() {
                st.active_by_category.remove(category);
            } else {
                st.active_by_category
                    .insert(category.clone(), active.clone());
            }
        }
        SpecDelta::CapabilityNotification(n) => {
            state.capability_notifications.push(n.clone());
        }
        SpecDelta::ScriptedCooldownChanged {
            owner_id,
            event_id,
            cooldown_remaining,
        } => {
            let k = ScriptedEventInstanceKey {
                owner_id: *owner_id,
                event_id: event_id.clone(),
            };
            if let Some(inst) = state.scripted_event_instances.get_mut(&k) {
                inst.cooldown_remaining = *cooldown_remaining;
            }
        }
        SpecDelta::ScriptedInstanceSlotChanged {
            owner_id,
            event_id,
            current_slot,
        } => {
            let k = ScriptedEventInstanceKey {
                owner_id: *owner_id,
                event_id: event_id.clone(),
            };
            if let Some(inst) = state.scripted_event_instances.get_mut(&k) {
                inst.current_slot = *current_slot;
            }
        }
        SpecDelta::ScriptedInstanceRemoved { owner_id, event_id } => {
            let k = ScriptedEventInstanceKey {
                owner_id: *owner_id,
                event_id: event_id.clone(),
            };
            state.scripted_event_instances.remove(&k);
        }
        SpecDelta::PlayerSelectionQueued {
            owner_id,
            definition_logical_id,
            entry,
        } => {
            state
                .queue_player_selection_by_key(*owner_id, definition_logical_id, &entry.entry_id)
                .map_err(|e| ReplayOpenError::SelectionResolution(e.to_string()))?;
        }
    }
    Ok(())
}

fn find_cap_instance_key(
    state: &SpecSessionState,
    owner_id: SimThingId,
    def_logical_id: &str,
) -> Option<CapabilityInstanceKey> {
    state.capability_instances.iter().find_map(|(key, inst)| {
        if inst.owner_id != owner_id {
            return None;
        }
        let def = state.capability_definitions.get(&inst.definition_id)?;
        if def.tree_id == def_logical_id {
            Some(*key)
        } else {
            None
        }
    })
}

// ── Replay open ───────────────────────────────────────────────────────────────

/// Reader output: the structural snapshot, the optional spec snapshot, and
/// every frame (each with its decoded `SpecDelta`s alongside the raw frame).
pub struct LoadedReplay {
    pub structural_snapshot: ReplaySnapshot,
    pub spec_snapshot: Option<SpecSnapshot>,
    pub frames: Vec<(ReplayFrame, Vec<SpecDelta>)>,
}

/// Read an LDJSON replay file end-to-end and decode the spec-layer
/// extensions. Used by `open_replay_with_spec`; exposed so tests and tools
/// that want the full frame list (rather than the streaming `ReplayReader`)
/// can use it directly.
pub fn read_spec_replay_file(path: &Path) -> Result<LoadedReplay, ReplayOpenError> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    // Line 1: structural snapshot (required).
    buf.clear();
    let n = reader.read_line(&mut buf)?;
    if n == 0 {
        return Err(ReplayOpenError::MissingStructuralSnapshot);
    }
    let first_value: serde_json::Value = serde_json::from_str(buf.trim_end())?;
    if first_value.get("kind").and_then(|v| v.as_str()) != Some("snapshot") {
        return Err(ReplayOpenError::MissingStructuralSnapshot);
    }
    let structural_snapshot: ReplaySnapshot = serde_json::from_value(
        first_value
            .get("snapshot")
            .cloned()
            .unwrap_or(serde_json::Value::Null),
    )?;

    let mut spec_snapshot: Option<SpecSnapshot> = None;
    let mut frames: Vec<(ReplayFrame, Vec<SpecDelta>)> = Vec::new();

    loop {
        buf.clear();
        let n = reader.read_line(&mut buf)?;
        if n == 0 {
            break;
        }
        let line = buf.trim_end();
        if line.is_empty() {
            continue;
        }
        let v: serde_json::Value = serde_json::from_str(line)?;
        match v.get("kind").and_then(|s| s.as_str()) {
            Some("spec_snapshot") => {
                let ss: SpecSnapshot = serde_json::from_value(
                    v.get("spec_snapshot")
                        .cloned()
                        .unwrap_or(serde_json::Value::Null),
                )?;
                spec_snapshot = Some(ss);
            }
            Some("frame") => {
                let frame: ReplayFrame = serde_json::from_value(
                    v.get("frame").cloned().unwrap_or(serde_json::Value::Null),
                )?;
                let deltas = json_to_spec_deltas(&frame.spec_entries)?;
                frames.push((frame, deltas));
            }
            _ => continue, // skip unknown line kinds
        }
    }

    Ok(LoadedReplay {
        structural_snapshot,
        spec_snapshot,
        frames,
    })
}

/// Open a replay file with full spec-state restoration. Re-installs the
/// `GameModeSpec` against the scenario (which rebuilds capability
/// definitions/instances and scripted-event definitions), then applies the
/// recorded `SpecSnapshot` if present. Returns the live session, a
/// `ReplayDriver` seeded from the structural snapshot, and the decoded
/// frames for the caller to apply.
///
/// `scenario` must be the same `Scenario` (or one structurally equivalent)
/// that was used to record. `game_mode` must be a superset of the recorded
/// spec — entries referenced by the snapshot but missing from the spec
/// produce `ReplayOpenError::UnknownCapabilityTree`.
pub fn open_replay_with_spec(
    replay_path: &Path,
    game_mode: &GameModeSpec,
    scenario: Scenario,
) -> Result<(SimSession, ReplayDriver, Vec<(ReplayFrame, Vec<SpecDelta>)>), ReplayOpenError> {
    let loaded = read_spec_replay_file(replay_path)?;
    let mut session = SimSession::open_from_spec(scenario, game_mode)?;
    if let Some(ss) = &loaded.spec_snapshot {
        apply_spec_snapshot(&mut session.spec_state, ss)?;
    }
    let driver = ReplayDriver::from_snapshot(loaded.structural_snapshot);
    Ok((session, driver, loaded.frames))
}
