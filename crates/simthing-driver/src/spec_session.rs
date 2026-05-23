//! Driver-owned runtime state for `simthing-spec` boundary handlers.
//!
//! This module is deliberately above `simthing-sim`: the driver can depend on
//! both crates, while the sim boundary protocol remains spec-agnostic.

use simthing_core::SimThingId;
use simthing_feeder::{
    BoundaryRequest, CapabilityUnlockRegistration, ScriptedEventTriggerRegistration,
};
use simthing_sim::BoundaryHookContext;
use simthing_spec::{
    CapabilityBoundaryContext, CapabilityEntryKey, CapabilityTreeBoundaryHandler,
    CapabilityTreeDefinition, CapabilityTreeDefinitionId, CapabilityTreeDiagnostic,
    CapabilityTreeError, CapabilityTreeInstance, CapabilityTreeNotification, CapabilityTreeState,
    EventKey, ScriptedEventBoundaryContext, ScriptedEventBoundaryHandler, ScriptedEventDefinition,
    ScriptedEventDiagnostic,
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
    #[error("capability tree `{tree_logical_id}` has no entry `{entry_id}` for owner `{owner_id:?}`")]
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

#[derive(Debug, Default)]
pub struct SpecSessionState {
    pub capability_definitions: HashMap<CapabilityTreeDefinitionId, CapabilityTreeDefinition>,
    pub capability_instances: HashMap<CapabilityInstanceKey, CapabilityTreeInstance>,
    pub capability_states: HashMap<CapabilityInstanceKey, CapabilityTreeState>,
    pub capability_unlock_registrations: Vec<CapabilityUnlockRegistration>,
    pub scripted_events: Vec<ScriptedEventDefinition>,
    pub scripted_cooldowns: HashMap<EventKey, u32>,
    pub scripted_current_slot: u32,
    pub capability_notifications: Vec<CapabilityTreeNotification>,
    pub capability_diagnostics: Vec<CapabilityTreeDiagnostic>,
    pub scripted_event_diagnostics: Vec<ScriptedEventDiagnostic>,
    pub handler_errors: Vec<String>,
    player_selections: Vec<(CapabilityInstanceKey, CapabilityEntryKey)>,
    /// Reverse index: capability tree `SimThingId` → installed instance key.
    capability_instance_by_tree: HashMap<SimThingId, CapabilityInstanceKey>,
}

impl SpecSessionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.capability_instances.is_empty() && self.scripted_events.is_empty()
    }

    pub fn requires_boundary_tick(&self) -> bool {
        !self.scripted_events.is_empty()
            || !self.player_selections.is_empty()
            || self.capability_states.values().any(|state| {
                state
                    .activation_mode_by_entry
                    .values()
                    .any(|mode| *mode == simthing_spec::ActivationMode::OnPrereqMet)
            })
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

    pub fn add_scripted_event(&mut self, definition: ScriptedEventDefinition) {
        self.scripted_events.push(definition);
    }

    pub fn set_scripted_current_slot(&mut self, slot: u32) {
        self.scripted_current_slot = slot;
    }

    pub fn queue_player_selection(
        &mut self,
        instance: CapabilityInstanceKey,
        entry: CapabilityEntryKey,
    ) {
        self.player_selections.push((instance, entry));
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
        self.scripted_events
            .iter()
            .filter_map(|definition| definition.to_trigger_registration(self.scripted_current_slot))
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
        if self.scripted_events.is_empty() {
            return;
        }

        let threshold_events = ctx
            .threshold_registry
            .extract_scripted_event_triggers(ctx.events);
        let handler = ScriptedEventBoundaryHandler {
            registry: ctx.registry,
            definitions: &self.scripted_events,
        };
        let mut requests = Vec::new();
        let mut diagnostics = Vec::new();
        let mut event_ctx = ScriptedEventBoundaryContext {
            n_dims: ctx.n_dims,
            shadow: &*ctx.shadow,
            current_slot: self.scripted_current_slot,
            slot_to_thing,
            cooldowns: &mut self.scripted_cooldowns,
            requests: &mut requests,
            diagnostics: &mut diagnostics,
        };

        handler.handle_tick(&threshold_events, &mut event_ctx);
        ctx.requests.extend(requests);
        self.scripted_event_diagnostics.extend(diagnostics);
    }
}

fn build_slot_to_thing(allocator: &simthing_gpu::SlotAllocator) -> HashMap<u32, SimThingId> {
    let mut out = HashMap::new();
    for slot in 0..allocator.capacity() as u32 {
        if let Some(id) = allocator.owner_of(slot) {
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

    #[test]
    fn queue_player_selection_by_key_resolves_logical_ids() {
        let mut registry = DimensionRegistry::new();
        let prop_id = registry.register(simthing_core::SimProperty::simple("core", "focus", 0));
        let mut tree = SimThing::new(SimThingKind::Custom("ideas".into()), 0);
        tree.add_property(prop_id, registry.property(prop_id).default_value());
        let tree_id = tree.id;
        let overlay_id = OverlayId::new();
        tree.add_overlay(simthing_core::Overlay {
            id: overlay_id,
            kind: simthing_core::OverlayKind::Custom("idea".into()),
            source: simthing_core::OverlaySource::System,
            affects: vec![tree_id],
            transform: simthing_core::PropertyTransformDelta {
                property_id: prop_id,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(1.0))],
            },
            lifecycle: simthing_core::OverlayLifecycle::Suspended {
                when_activated: Box::new(simthing_core::OverlayLifecycle::Permanent),
            },
        });

        let category = CategoryKey::new("ideas", "national");
        let entry = simthing_spec::CapabilityEntryKey::new(category.clone(), "focus");
        let definition_id = CapabilityTreeDefinitionId::new();
        let definition = CapabilityTreeDefinition {
            id: definition_id,
            tree_id: "national_ideas".into(),
            categories: HashMap::from([(
                category.clone(),
                CapabilityCategoryDefinition {
                    key: category,
                    property_id: prop_id,
                    max_active: None,
                    tier: 0,
                },
            )]),
            entries: HashMap::from([(
                entry.clone(),
                CapabilityDefinition {
                    key: entry.clone(),
                    display_name: "Focus".into(),
                    description: String::new(),
                    flavor_text: None,
                    activation: ActivationMode::PlayerSelection,
                    overlay_ids: vec![overlay_id],
                    effect_keys: Vec::new(),
                    effect_transforms: Vec::new(),
                    prereqs: Vec::new(),
                    progress_col: 0,
                    research_cost: 1.0,
                },
            )]),
            by_threshold: HashMap::new(),
            by_overlay: HashMap::from([(overlay_id, entry.clone())]),
        };

        let mut allocator = simthing_gpu::SlotAllocator::new();
        allocator.populate_from_tree(&tree);
        let instance = CapabilityTreeInstance {
            owner_id: tree_id,
            definition_id,
            tree_thing_id: tree_id,
            tree_slot: allocator.slot_of(tree_id).unwrap(),
        };
        let state = CapabilityTreeState {
            owner_id: tree_id,
            definition_id,
            activation_mode_by_entry: HashMap::new(),
            active_by_category: HashMap::new(),
        };
        let mut spec_state = SpecSessionState::new();
        spec_state.add_capability_tree_instance(definition, instance, state, Vec::new());

        spec_state
            .queue_player_selection_by_key(tree_id, "national_ideas", "focus")
            .expect("logical selection resolves");

        let threshold_registry = ThresholdRegistry::new();
        let mut shadow = vec![0.0; registry.total_columns];
        let mut requests = Vec::new();
        let mut hook = BoundaryHookContext {
            events: &[],
            threshold_registry: &threshold_registry,
            registry: &registry,
            allocator: &allocator,
            shadow: &mut shadow,
            n_dims: registry.total_columns,
            requests: &mut requests,
        };

        spec_state.run_boundary_handlers(&mut hook);

        assert!(matches!(
            hook.requests.as_slice(),
            [simthing_feeder::BoundaryRequest::ActivateOverlay { overlay_id: id, .. }]
            if *id == overlay_id
        ));
        assert!(spec_state.handler_errors.is_empty());
    }

    #[test]
    fn queue_player_selection_by_key_reports_unknown_entry() {
        let mut spec_state = SpecSessionState::new();
        let owner = SimThingId::new();
        let definition_id = CapabilityTreeDefinitionId::new();
        let definition = CapabilityTreeDefinition {
            id: definition_id,
            tree_id: "national_ideas".into(),
            categories: HashMap::new(),
            entries: HashMap::new(),
            by_threshold: HashMap::new(),
            by_overlay: HashMap::new(),
        };
        let instance = CapabilityTreeInstance {
            owner_id: owner,
            definition_id,
            tree_thing_id: SimThingId::new(),
            tree_slot: 0,
        };
        spec_state.add_capability_tree_instance(
            definition,
            instance,
            CapabilityTreeState {
                owner_id: owner,
                definition_id,
                activation_mode_by_entry: HashMap::new(),
                active_by_category: HashMap::new(),
            },
            Vec::new(),
        );

        let err = spec_state
            .queue_player_selection_by_key(owner, "national_ideas", "missing")
            .unwrap_err();
        assert!(matches!(
            err,
            SpecSessionError::UnknownEntry {
                entry_id,
                tree_logical_id,
                ..
            } if entry_id == "missing" && tree_logical_id == "national_ideas"
        ));
    }

    #[test]
    fn queued_player_selection_runs_through_capability_handler() {
        let mut registry = DimensionRegistry::new();
        let prop_id = registry.register(simthing_core::SimProperty::simple("core", "focus", 0));
        let mut tree = SimThing::new(SimThingKind::Custom("ideas".into()), 0);
        tree.add_property(prop_id, registry.property(prop_id).default_value());
        let tree_id = tree.id;
        let overlay_id = OverlayId::new();
        tree.add_overlay(simthing_core::Overlay {
            id: overlay_id,
            kind: simthing_core::OverlayKind::Custom("idea".into()),
            source: simthing_core::OverlaySource::System,
            affects: vec![tree_id],
            transform: simthing_core::PropertyTransformDelta {
                property_id: prop_id,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(1.0))],
            },
            lifecycle: simthing_core::OverlayLifecycle::Suspended {
                when_activated: Box::new(simthing_core::OverlayLifecycle::Permanent),
            },
        });

        let category = CategoryKey::new("ideas", "national");
        let entry = simthing_spec::CapabilityEntryKey::new(category.clone(), "focus");
        let definition_id = CapabilityTreeDefinitionId::new();
        let definition = CapabilityTreeDefinition {
            id: definition_id,
            tree_id: "ideas".into(),
            categories: HashMap::from([(
                category.clone(),
                CapabilityCategoryDefinition {
                    key: category,
                    property_id: prop_id,
                    max_active: None,
                    tier: 0,
                },
            )]),
            entries: HashMap::from([(
                entry.clone(),
                CapabilityDefinition {
                    key: entry.clone(),
                    display_name: "Focus".into(),
                    description: String::new(),
                    flavor_text: None,
                    activation: ActivationMode::PlayerSelection,
                    overlay_ids: vec![overlay_id],
                    effect_keys: Vec::new(),
                    effect_transforms: Vec::new(),
                    prereqs: Vec::new(),
                    progress_col: 0,
                    research_cost: 1.0,
                },
            )]),
            by_threshold: HashMap::new(),
            by_overlay: HashMap::from([(overlay_id, entry.clone())]),
        };

        let mut allocator = simthing_gpu::SlotAllocator::new();
        allocator.populate_from_tree(&tree);
        let instance = CapabilityTreeInstance {
            owner_id: tree_id,
            definition_id,
            tree_thing_id: tree_id,
            tree_slot: allocator.slot_of(tree_id).unwrap(),
        };
        let state = CapabilityTreeState {
            owner_id: tree_id,
            definition_id,
            activation_mode_by_entry: HashMap::new(),
            active_by_category: HashMap::new(),
        };
        let mut spec_state = SpecSessionState::new();
        let key = spec_state.add_capability_tree_instance(definition, instance, state, Vec::new());
        spec_state.queue_player_selection(key, entry);

        let threshold_registry = ThresholdRegistry::new();
        let mut shadow = vec![0.0; registry.total_columns];
        let mut requests = Vec::new();
        let mut hook = BoundaryHookContext {
            events: &[],
            threshold_registry: &threshold_registry,
            registry: &registry,
            allocator: &allocator,
            shadow: &mut shadow,
            n_dims: registry.total_columns,
            requests: &mut requests,
        };

        spec_state.run_boundary_handlers(&mut hook);

        assert!(matches!(
            hook.requests.as_slice(),
            [simthing_feeder::BoundaryRequest::ActivateOverlay { overlay_id: id, .. }]
            if *id == overlay_id
        ));
        assert!(spec_state.handler_errors.is_empty());
    }

    #[test]
    fn scripted_event_handler_runs_from_spec_session_state() {
        let registry = DimensionRegistry::new();
        let world = SimThing::new(SimThingKind::World, 0);
        let world_id = world.id;
        let mut allocator = simthing_gpu::SlotAllocator::new();
        allocator.populate_from_tree(&world);
        let slot = allocator.slot_of(world_id).unwrap();
        let threshold_registry = ThresholdRegistry::new();
        let mut shadow = Vec::new();
        let mut requests = Vec::new();
        let event = ScriptedEventDefinition {
            id: EventKey::new("remove_world"),
            trigger: CompiledTrigger::Predicate(ScriptPredicate::True),
            effects: vec![CompiledEffect::Remove {
                target: simthing_spec::ScopeRef::Current,
            }],
            cooldown: None,
            priority: EventPriority::Normal,
        };
        let mut spec_state = SpecSessionState::new();
        spec_state.set_scripted_current_slot(slot);
        spec_state.add_scripted_event(event);
        let mut hook = BoundaryHookContext {
            events: &[],
            threshold_registry: &threshold_registry,
            registry: &registry,
            allocator: &allocator,
            shadow: &mut shadow,
            n_dims: registry.total_columns,
            requests: &mut requests,
        };

        spec_state.run_boundary_handlers(&mut hook);

        assert!(matches!(
            hook.requests.as_slice(),
            [simthing_feeder::BoundaryRequest::Remove { target }]
            if *target == world_id
        ));
        assert!(spec_state.scripted_event_diagnostics.is_empty());
    }
}
