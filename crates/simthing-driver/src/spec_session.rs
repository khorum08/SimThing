//! Driver-owned runtime state for `simthing-spec` boundary handlers.
//!
//! This module is deliberately above `simthing-sim`: the driver can depend on
//! both crates, while the sim boundary protocol remains spec-agnostic.

use crate::arena_registry::ArenaRegistry;
use simthing_core::{SimThingId, SlotIndex};
use simthing_feeder::{
    BoundaryRequest, CapabilityUnlockRegistration, ScriptedEventTriggerRegistration,
};
use simthing_gpu::ThresholdEvent;
use simthing_sim::{BoundaryHookContext, ThresholdRegistry};
use simthing_spec::ResolvedResourceFlowCapacityBudget;
use simthing_spec::{
    ActivationMode, CapabilityBoundaryContext, CapabilityEntryKey, CapabilityTreeBoundaryHandler,
    CapabilityTreeDefinition, CapabilityTreeDefinitionId, CapabilityTreeDiagnostic,
    CapabilityTreeError, CapabilityTreeInstance, CapabilityTreeNotification, CapabilityTreeState,
    CompiledTrigger, EventKey, ScriptedEventBoundaryContext, ScriptedEventBoundaryHandler,
    ScriptedEventDefinition, ScriptedEventDefinitionId, ScriptedEventDiagnostic,
    ScriptedEventInstance, ScriptedEventInstanceKey,
};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum SpecSessionError {
    #[error("no capability tree instance for owner `{0:?}`")]
    UnknownOwner(SimThingId),
    #[error("owner `{owner_id:?}` has no capability tree `{tree_logical_id}`")]
    UnknownTree {
        owner_id: SimThingId,
        tree_logical_id: String,
    },
    #[error(
        "capability tree `{tree_logical_id}` has no entry `{entry_id}` for owner `{owner_id:?}`"
    )]
    UnknownEntry {
        owner_id: SimThingId,
        tree_logical_id: String,
        entry_id: String,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CapabilityInstanceKey {
    pub owner_id: SimThingId,
    pub definition_id: CapabilityTreeDefinitionId,
    pub tree_thing_id: SimThingId,
}

impl CapabilityInstanceKey {
    pub fn from_instance(instance: &CapabilityTreeInstance) -> Self {
        Self {
            owner_id: instance.owner_id,
            definition_id: instance.definition_id,
            tree_thing_id: instance.tree_thing_id,
        }
    }
}

/// Snapshot of mutable spec-layer state taken immediately before a boundary
/// runs. The recording path captures this, runs the spec hook, and diffs
/// against the post-boundary state to produce per-frame deltas.
///
/// See `docs/adr/spec_session_state_replay.md`.
#[derive(Clone, Debug)]
pub struct PreBoundarySnapshot {
    pub capability_states: HashMap<CapabilityInstanceKey, CapabilityTreeState>,
    pub scripted_event_instances: HashMap<ScriptedEventInstanceKey, ScriptedEventInstance>,
    /// Player selections queued before the boundary — the handler drains
    /// these, so the snapshot is the only place to recover which selections
    /// fired this boundary for replay.
    pub pending_selections: Vec<(CapabilityInstanceKey, CapabilityEntryKey)>,
}

#[derive(Debug, Default)]
pub struct SpecSessionState {
    pub capability_definitions: HashMap<CapabilityTreeDefinitionId, CapabilityTreeDefinition>,
    pub capability_instances: HashMap<CapabilityInstanceKey, CapabilityTreeInstance>,
    pub capability_states: HashMap<CapabilityInstanceKey, CapabilityTreeState>,
    pub capability_unlock_registrations: Vec<CapabilityUnlockRegistration>,
    /// Per-definition compiled scripted-event payload. Shared across
    /// every per-owner instance (see `docs/adr/scripted_event_scope_model.md`).
    pub scripted_event_definitions: HashMap<ScriptedEventDefinitionId, ScriptedEventDefinition>,
    /// Per-owner instance. Cooldowns and `current_slot` live here so two
    /// instances of the same definition fire / cool down independently.
    pub scripted_event_instances: HashMap<ScriptedEventInstanceKey, ScriptedEventInstance>,
    pub capability_notifications: Vec<CapabilityTreeNotification>,
    pub capability_diagnostics: Vec<CapabilityTreeDiagnostic>,
    pub scripted_event_diagnostics: Vec<ScriptedEventDiagnostic>,
    pub handler_errors: Vec<String>,
    /// Resource Flow arena participation registry (E-9). Driver/spec artifact only.
    pub arena_registry: ArenaRegistry,
    /// Arena-participant SimThing scaffold (E-10R2). Driver-only topology artifact.
    pub arena_participant_scaffold: crate::arena_participant::ArenaParticipantScaffold,
    /// CT-RF-EML-RATE-0: install-resolved gated rate terms consumed by the
    /// resource-flow sync (effective-rate EvalEML band before reduce bands).
    pub resolved_gated_rates: Vec<crate::gated_rates::ResolvedGatedRate>,
    /// RF-5A: install-resolved need bindings (full-cell loci).
    pub resolved_need_bindings: Vec<crate::need_binding::ResolvedNeedBinding>,
    /// Checked RF capacity budget used for session slot and emission reservations.
    pub resource_flow_capacity_budget: Option<ResolvedResourceFlowCapacityBudget>,
    /// Materialized production transfer/recipe/emission/threshold registrations (Phase T-3/T-4).
    pub resource_economy_registry: Option<crate::resource_economy_compile::ResourceEconomyRegistry>,
    player_selections: Vec<(CapabilityInstanceKey, CapabilityEntryKey)>,
    /// Reverse index: capability tree `SimThingId` → installed instance key.
    capability_instance_by_tree: HashMap<SimThingId, CapabilityInstanceKey>,
    /// Slot supplied to the back-compat `add_scripted_event` shim. Default
    /// 0; install drives via `set_scripted_current_slot`.
    session_root_slot: u32,
    /// Owner id supplied to the back-compat `add_scripted_event` shim.
    /// Default `SimThingId::default()`; install drives via
    /// `set_session_root_owner`.
    session_root_owner: SimThingId,
    /// Generation last uploaded to GPU for resource economy registrations (T-4 skip gate).
    resource_economy_uploaded_generation: u64,
}

impl SpecSessionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resource_economy_uploaded_generation(&self) -> u64 {
        self.resource_economy_uploaded_generation
    }

    pub fn set_resource_economy_uploaded_generation(&mut self, generation: u64) {
        self.resource_economy_uploaded_generation = generation;
    }

    pub fn is_empty(&self) -> bool {
        self.capability_instances.is_empty()
            && self.scripted_event_instances.is_empty()
            && self.resource_economy_registry.is_none()
    }

    /// Classify whether the upcoming boundary needs a spec-layer hook.
    /// Returning `false` permits the session loop to take the empty-boundary
    /// fast path (no `take_delta_log`, no `execute_with_boundary_hook`).
    ///
    /// B3 — `docs/adr/`-pending refinement: precise classification beyond
    /// "any scripted event instance exists":
    ///
    /// - **Queued player selections** → must tick (handler dequeues and
    ///   emits `ActivateOverlay`).
    /// - **Any scripted-event instance with `cooldown_remaining > 0`** →
    ///   must tick (cooldowns are decremented inside the handler; skipping
    ///   would freeze them forever).
    /// - **Any `Predicate`-trigger scripted-event definition** → must tick
    ///   (predicates re-evaluate every boundary against `ctx.shadow`).
    /// - **Any `OnPrereqMet` capability entry** → must tick (the sweep
    ///   re-checks prereqs every boundary; conservative — refining this
    ///   would require change-tracking on prereq inputs).
    /// - **Any `ThresholdEvent` matching a capability-unlock registration**
    ///   → must tick (the handler dispatches `CapabilityUnlockEvent`s).
    /// - **Any `ThresholdEvent` matching a scripted-event-trigger
    ///   registration** → must tick (the threshold-trigger arm only fires
    ///   when the event id is in the fired-set).
    ///
    /// Everything else is genuinely no-op and can skip. The previous
    /// implementation conservatively forced a tick whenever *any* scripted
    /// instance existed; pure-Threshold scripted events with no triggering
    /// event and no cooldown now skip cleanly.
    pub fn requires_boundary_tick(
        &self,
        events: &[ThresholdEvent],
        threshold_registry: &ThresholdRegistry,
    ) -> bool {
        if !self.player_selections.is_empty() {
            return true;
        }
        // Cooldown tick: any instance with non-zero cooldown must run so the
        // counter decrements. (Storing this as a per-state derived flag would
        // skip the scan but the instance map is tiny in practice.)
        if self
            .scripted_event_instances
            .values()
            .any(|inst| inst.cooldown_remaining > 0)
        {
            return true;
        }
        // Predicate triggers re-eval every boundary regardless of input.
        if self
            .scripted_event_definitions
            .values()
            .any(|def| matches!(def.trigger, CompiledTrigger::Predicate(_)))
        {
            return true;
        }
        // OnPrereqMet sweep — conservative: any entry in this state forces a
        // tick. Refining would require tracking which prereq inputs could
        // have changed this boundary.
        if self.capability_states.values().any(|state| {
            state
                .activation_mode_by_entry
                .values()
                .any(|mode| *mode == ActivationMode::OnPrereqMet)
        }) {
            return true;
        }
        // Event-driven arms: only force a tick when the actual fired events
        // match a spec registration. Zero-alloc bool checks (B3).
        if threshold_registry.has_capability_unlock_in(events) {
            return true;
        }
        if threshold_registry.has_scripted_event_trigger_in(events) {
            return true;
        }
        false
    }

    pub fn add_capability_tree_instance(
        &mut self,
        definition: CapabilityTreeDefinition,
        instance: CapabilityTreeInstance,
        state: CapabilityTreeState,
        unlock_registrations: Vec<CapabilityUnlockRegistration>,
    ) -> CapabilityInstanceKey {
        let key = CapabilityInstanceKey::from_instance(&instance);
        self.capability_definitions
            .insert(definition.id, definition);
        self.capability_instances.insert(key, instance.clone());
        self.capability_states.insert(key, state);
        self.capability_instance_by_tree
            .insert(instance.tree_thing_id, key);
        self.capability_unlock_registrations
            .extend(unlock_registrations);
        key
    }

    /// Register a definition and return its fresh
    /// `ScriptedEventDefinitionId`. Attach instances via
    /// `attach_scripted_event_instance` — N instances per definition is
    /// the v1 model (one per `InstallTargetSpec`-resolved owner).
    pub fn register_scripted_event_definition(
        &mut self,
        definition: ScriptedEventDefinition,
    ) -> ScriptedEventDefinitionId {
        let definition_id = ScriptedEventDefinitionId::new();
        self.scripted_event_definitions
            .insert(definition_id, definition);
        definition_id
    }

    /// Attach a per-owner instance to a previously-registered definition.
    /// `event_id` should match `definition.id` — it's surfaced separately
    /// because `ScriptedEventInstanceKey` keys on (owner_id, event_id) and
    /// we want to avoid re-borrowing the definition map at the call site.
    pub fn attach_scripted_event_instance(
        &mut self,
        definition_id: ScriptedEventDefinitionId,
        event_id: EventKey,
        owner_id: SimThingId,
        slot: u32,
    ) -> ScriptedEventInstanceKey {
        let key = ScriptedEventInstanceKey { owner_id, event_id };
        self.scripted_event_instances.insert(
            key.clone(),
            ScriptedEventInstance {
                key: key.clone(),
                definition_id,
                current_slot: slot,
                cooldown_remaining: 0,
            },
        );
        key
    }

    /// Convenience: register a definition and attach a single instance in
    /// one call. Used by the back-compat shim and by tests that want one
    /// instance.
    pub fn add_scripted_event_instance(
        &mut self,
        definition: ScriptedEventDefinition,
        owner_id: SimThingId,
        slot: u32,
    ) -> ScriptedEventInstanceKey {
        let event_id = definition.id.clone();
        let definition_id = self.register_scripted_event_definition(definition);
        self.attach_scripted_event_instance(definition_id, event_id, owner_id, slot)
    }

    /// Back-compat shim: install the definition as a single
    /// `SessionRoot`-style instance using `owner_id = SimThingId::default()`
    /// and `current_slot` supplied via `set_session_root_slot`. New code
    /// should call `add_scripted_event_instance` directly.
    pub fn add_scripted_event(&mut self, definition: ScriptedEventDefinition) {
        let slot = self.session_root_slot;
        let owner_id = self.session_root_owner;
        let _ = self.add_scripted_event_instance(definition, owner_id, slot);
    }

    /// Back-compat shim for `set_scripted_current_slot` callers. Sets the
    /// slot used by `add_scripted_event` when it installs a default
    /// SessionRoot instance, and refreshes any existing instance owned by
    /// `session_root_owner`.
    pub fn set_scripted_current_slot(&mut self, slot: u32) {
        self.session_root_slot = slot;
        for inst in self.scripted_event_instances.values_mut() {
            if inst.key.owner_id == self.session_root_owner {
                inst.current_slot = slot;
            }
        }
    }

    /// Set the owner id used by the back-compat `add_scripted_event` /
    /// `set_scripted_current_slot` shims. Install drives this with
    /// `scenario.root.id`; tests can leave it at default.
    pub fn set_session_root_owner(&mut self, owner_id: SimThingId) {
        self.session_root_owner = owner_id;
    }

    /// Refresh every instance's `current_slot` against `allocator`. Drops
    /// instances whose owner no longer has a slot and emits an
    /// `OwnerRemoved` diagnostic. Returns the count of removed instances.
    pub fn refresh_scripted_event_slots(
        &mut self,
        allocator: &simthing_gpu::SlotAllocator,
    ) -> usize {
        let mut stale = Vec::new();
        for inst in self.scripted_event_instances.values_mut() {
            match allocator.slot_of(inst.key.owner_id) {
                Some(slot) => inst.current_slot = slot.raw(),
                None => stale.push(inst.key.clone()),
            }
        }
        let removed = stale.len();
        for key in stale {
            self.scripted_event_instances.remove(&key);
            self.scripted_event_diagnostics
                .push(ScriptedEventDiagnostic::owner_removed(
                    key.owner_id,
                    key.event_id,
                ));
        }
        removed
    }

    pub fn queue_player_selection(
        &mut self,
        instance: CapabilityInstanceKey,
        entry: CapabilityEntryKey,
    ) {
        self.player_selections.push((instance, entry));
    }

    /// Snapshot the mutable spec-layer fields immediately before a boundary
    /// runs. Paired with `spec_replay::diff_and_emit` after the boundary to
    /// produce per-frame `SpecDelta`s. Cloning is O(active instances) — small
    /// in practice (factions × trees + factions × scripted events).
    ///
    /// `player_selections` is snapshotted because the boundary handler drains
    /// the queue; the snapshot is the only record of selections that fired
    /// this boundary, used to emit `SpecDelta::PlayerSelectionQueued`.
    pub fn pre_boundary_snapshot(&self) -> PreBoundarySnapshot {
        PreBoundarySnapshot {
            capability_states: self.capability_states.clone(),
            scripted_event_instances: self.scripted_event_instances.clone(),
            pending_selections: self.player_selections.clone(),
        }
    }

    /// Drain pending capability notifications, returning them to the caller.
    /// The recording path calls this after each boundary to convert the
    /// transient queue into `SpecDelta::CapabilityNotification` entries.
    pub fn drain_notifications(&mut self) -> Vec<CapabilityTreeNotification> {
        std::mem::take(&mut self.capability_notifications)
    }

    /// Queue a player-selection activation using logical tree and entry ids.
    ///
    /// Resolves `owner_id` + `tree_logical_id` against installed capability
    /// instances, then matches `entry_id` within that tree's compiled entries.
    pub fn queue_player_selection_by_key(
        &mut self,
        owner_id: SimThingId,
        tree_logical_id: &str,
        entry_id: &str,
    ) -> Result<(), SpecSessionError> {
        let matching: Vec<CapabilityInstanceKey> = self
            .capability_instances
            .iter()
            .filter(|(_, instance)| instance.owner_id == owner_id)
            .filter_map(|(key, instance)| {
                self.capability_definitions
                    .get(&instance.definition_id)
                    .filter(|def| def.tree_id == tree_logical_id)
                    .map(|_| *key)
            })
            .collect();

        if matching.is_empty() {
            let owner_known = self
                .capability_instances
                .values()
                .any(|instance| instance.owner_id == owner_id);
            if owner_known {
                return Err(SpecSessionError::UnknownTree {
                    owner_id,
                    tree_logical_id: tree_logical_id.into(),
                });
            }
            return Err(SpecSessionError::UnknownOwner(owner_id));
        }

        let instance_key = matching[0];
        let definition = self
            .capability_definitions
            .get(&instance_key.definition_id)
            .expect("instance definition exists");

        let entry_matches: Vec<&CapabilityEntryKey> = definition
            .entries
            .keys()
            .filter(|key| key.entry_id == entry_id)
            .collect();

        let entry_key = match entry_matches.as_slice() {
            [key] => (*key).clone(),
            _ => {
                return Err(SpecSessionError::UnknownEntry {
                    owner_id,
                    tree_logical_id: tree_logical_id.into(),
                    entry_id: entry_id.into(),
                });
            }
        };

        self.queue_player_selection(instance_key, entry_key);
        Ok(())
    }

    pub fn scripted_event_trigger_registrations(&self) -> Vec<ScriptedEventTriggerRegistration> {
        self.scripted_event_instances
            .values()
            .filter_map(|inst| {
                let def = self.scripted_event_definitions.get(&inst.definition_id)?;
                def.to_trigger_registration(inst.current_slot)
            })
            .collect()
    }

    pub fn run_boundary_handlers(&mut self, ctx: &mut BoundaryHookContext<'_>) {
        let slot_to_thing = build_slot_to_thing(ctx.allocator);
        self.run_capability_handlers(ctx);
        self.run_scripted_event_handler(ctx, &slot_to_thing);
    }

    fn run_capability_handlers(&mut self, ctx: &mut BoundaryHookContext<'_>) {
        if self.capability_instances.is_empty() {
            return;
        }

        let unlock_events = ctx
            .threshold_registry
            .extract_capability_unlocks(ctx.events);
        for event in &unlock_events {
            if !self
                .capability_instance_by_tree
                .contains_key(&event.sim_thing_id)
            {
                self.capability_diagnostics.push(
                    CapabilityTreeDiagnostic::UnknownThresholdSimThing {
                        sim_thing_id: event.sim_thing_id,
                    },
                );
            }
        }

        let keys: Vec<CapabilityInstanceKey> =
            self.capability_instance_by_tree.values().copied().collect();
        for key in keys {
            let events_for_instance: Vec<_> = unlock_events
                .iter()
                .filter(|event| event.sim_thing_id == key.tree_thing_id)
                .cloned()
                .collect();

            if !events_for_instance.is_empty() {
                self.with_capability_context(key, ctx, |handler, cap_ctx| {
                    handler.handle_capability_unlock_events(&events_for_instance, cap_ctx)
                });
            }

            self.with_capability_context(key, ctx, |handler, cap_ctx| {
                handler.sweep_on_prereq_met(key.owner_id, cap_ctx)
            });
        }

        let selections = std::mem::take(&mut self.player_selections);
        for (key, entry) in selections {
            self.with_capability_context(key, ctx, |handler, cap_ctx| {
                handler.handle_player_selection(key.owner_id, entry.clone(), cap_ctx)
            });
        }
    }

    fn with_capability_context<F>(
        &mut self,
        key: CapabilityInstanceKey,
        hook_ctx: &mut BoundaryHookContext<'_>,
        mut f: F,
    ) where
        F: FnMut(
            &CapabilityTreeBoundaryHandler<'_>,
            &mut CapabilityBoundaryContext<'_>,
        ) -> Result<(), CapabilityTreeError>,
    {
        let Some(instance) = self.capability_instances.get(&key).cloned() else {
            self.handler_errors.push(format!(
                "missing capability instance for {:?}",
                key.tree_thing_id
            ));
            return;
        };
        let Some(state) = self.capability_states.get(&key).cloned() else {
            self.handler_errors.push(format!(
                "missing capability state for {:?}",
                key.tree_thing_id
            ));
            return;
        };

        let instances = HashMap::from([(instance.owner_id, instance.clone())]);
        let mut states = HashMap::from([(instance.owner_id, state)]);
        let mut requests = Vec::<BoundaryRequest>::new();
        let mut notifications = Vec::<CapabilityTreeNotification>::new();
        let mut diagnostics = Vec::<CapabilityTreeDiagnostic>::new();
        let handler = CapabilityTreeBoundaryHandler {
            registry: hook_ctx.registry,
            definitions: &self.capability_definitions,
        };
        let mut cap_ctx = CapabilityBoundaryContext {
            n_dims: hook_ctx.n_dims,
            shadow: &mut *hook_ctx.shadow,
            instances: &instances,
            states: &mut states,
            requests: &mut requests,
            notifications: &mut notifications,
            diagnostics: &mut diagnostics,
        };

        if let Err(err) = f(&handler, &mut cap_ctx) {
            self.handler_errors.push(err.to_string());
        }

        if let Some(updated) = states.remove(&instance.owner_id) {
            self.capability_states.insert(key, updated);
        }
        hook_ctx.requests.extend(requests);
        self.capability_notifications.extend(notifications);
        self.capability_diagnostics.extend(diagnostics);
    }

    fn run_scripted_event_handler(
        &mut self,
        ctx: &mut BoundaryHookContext<'_>,
        slot_to_thing: &HashMap<u32, SimThingId>,
    ) {
        // Refresh per-instance slots and prune stale instances before any
        // dispatch this boundary.
        let _ = self.refresh_scripted_event_slots(ctx.allocator);
        if self.scripted_event_instances.is_empty() {
            return;
        }

        let threshold_events = ctx
            .threshold_registry
            .extract_scripted_event_triggers(ctx.events);

        // Iterate instances in deterministic order (sorted by owner_id +
        // event_id) so any future cross-instance ordering invariants stay
        // stable across HashMap iteration.
        let mut keys: Vec<ScriptedEventInstanceKey> =
            self.scripted_event_instances.keys().cloned().collect();
        keys.sort_by(|a, b| {
            a.owner_id
                .cmp(&b.owner_id)
                .then_with(|| a.event_id.0.cmp(&b.event_id.0))
        });

        let mut all_requests = Vec::new();
        let mut all_diagnostics = Vec::new();

        for key in keys {
            // Take a one-element definition slice and a one-entry cooldown
            // map per instance. This isolates each instance's cooldown state
            // and lets us use the existing handler unchanged (see
            // `docs/adr/scripted_event_scope_model.md` §2).
            let inst = match self.scripted_event_instances.get(&key) {
                Some(i) => i.clone(),
                None => continue,
            };
            let Some(def) = self.scripted_event_definitions.get(&inst.definition_id) else {
                continue;
            };
            let definitions_slice = std::slice::from_ref(def);

            // Per-instance cooldown — bridge to the handler's
            // `HashMap<EventKey, u32>` shape with a single entry, then
            // copy the post-handler value back to the instance.
            let mut cooldowns: HashMap<EventKey, u32> = HashMap::new();
            if inst.cooldown_remaining > 0 {
                cooldowns.insert(inst.key.event_id.clone(), inst.cooldown_remaining);
            }

            let handler = ScriptedEventBoundaryHandler {
                registry: ctx.registry,
                definitions: definitions_slice,
            };
            let mut requests = Vec::new();
            let mut diagnostics = Vec::new();
            let mut event_ctx = ScriptedEventBoundaryContext {
                n_dims: ctx.n_dims,
                shadow: &*ctx.shadow,
                current_slot: inst.current_slot,
                slot_to_thing,
                cooldowns: &mut cooldowns,
                requests: &mut requests,
                diagnostics: &mut diagnostics,
            };
            handler.handle_tick(&threshold_events, &mut event_ctx);

            // Write back cooldown to the per-instance slot.
            let new_cooldown = cooldowns.get(&inst.key.event_id).copied().unwrap_or(0);
            if let Some(slot) = self.scripted_event_instances.get_mut(&key) {
                slot.cooldown_remaining = new_cooldown;
            }
            all_requests.extend(requests);
            all_diagnostics.extend(diagnostics);
        }

        ctx.requests.extend(all_requests);
        self.scripted_event_diagnostics.extend(all_diagnostics);
    }
}

fn build_slot_to_thing(allocator: &simthing_gpu::SlotAllocator) -> HashMap<u32, SimThingId> {
    let mut out = HashMap::new();
    for slot in 0..allocator.capacity() as u32 {
        if let Some(id) = allocator.owner_of(SlotIndex::new(slot)) {
            out.insert(slot, id);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        DimensionRegistry, OverlayId, SimThing, SimThingKind, SubFieldRole, TransformOp,
    };
    use simthing_sim::{BoundaryHookContext, ThresholdRegistry};
    use simthing_spec::{
        ActivationMode, CapabilityCategoryDefinition, CapabilityDefinition,
        CapabilityTreeDefinition, CapabilityTreeDefinitionId, CategoryKey, CompiledEffect,
        CompiledTrigger, EventKey, EventPriority, ScriptPredicate, ScriptedEventDefinition,
    };

    fn oracle_threshold_event(
        slot: u32,
        col: u32,
        curr_value: f32,
        event_kind: u32,
    ) -> simthing_gpu::ThresholdEvent {
        use simthing_gpu::{
            cpu_oracle_threshold_events, ThresholdRegistration, DIR_UPWARD, THRESH_BUF_VALUES,
        };
        let n_dims = col + 1;
        let len = ((slot + 1) * n_dims) as usize;
        let idx = (slot * n_dims + col) as usize;
        let mut prev = vec![0.0f32; len];
        let mut curr = vec![0.0f32; len];
        let threshold = curr_value - 0.01;
        prev[idx] = threshold;
        curr[idx] = curr_value;
        let reg = ThresholdRegistration {
            slot,
            col,
            threshold,
            direction: DIR_UPWARD,
            event_kind,
            buffer: THRESH_BUF_VALUES,
        };
        cpu_oracle_threshold_events(&prev, &curr, &[], &[], n_dims, &[reg])
            .into_iter()
            .next()
            .expect("crossing event")
    }
}
