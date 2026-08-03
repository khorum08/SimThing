use crate::keys::CapabilityEntryKey;
use crate::runtime::{
    CapabilityTreeDefinition, CapabilityTreeDefinitionId, CapabilityTreeDiagnostic,
    CapabilityTreeInstance, CapabilityTreeNotification, CapabilityTreeState,
};
use crate::spec::capability::{ActivationMode, MaxActivePolicy, ReplacementPolicy};
use simthing_core::{DimensionRegistry, OverlayId, SimThingId};
use simthing_feeder::{BoundaryRequest, CapabilityUnlockEvent};
use std::collections::{HashMap, HashSet};

const PREREQ_RESET_EPSILON: f32 = 0.01;

pub struct CapabilityTreeBoundaryHandler<'a> {
    pub registry: &'a DimensionRegistry,
    pub definitions: &'a HashMap<CapabilityTreeDefinitionId, CapabilityTreeDefinition>,
}

pub struct CapabilityBoundaryContext<'a> {
    pub n_dims: usize,
    pub shadow: &'a mut [f32],
    pub instances: &'a HashMap<SimThingId, CapabilityTreeInstance>,
    pub states: &'a mut HashMap<SimThingId, CapabilityTreeState>,
    pub requests: &'a mut Vec<BoundaryRequest>,
    pub notifications: &'a mut Vec<CapabilityTreeNotification>,
    pub diagnostics: &'a mut Vec<CapabilityTreeDiagnostic>,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum CapabilityTreeError {
    #[error("entry `{0}` has authored ActivationMode != PlayerSelection - cannot drive via handle_player_selection")]
    PlayerSelectionRequiresPlayerSelectionMode(String),
    #[error("category `{0}` declared max_active > 1 but only Unlimited / Limited(1) are supported in v0")]
    UnsupportedMaxActive(String),
    #[error("entry `{0}` is not present in its capability tree definition")]
    EntryNotInTree(String),
    #[error("missing capability tree state for owner `{0:?}`")]
    MissingState(SimThingId),
    #[error("missing capability tree instance for owner `{0:?}`")]
    MissingInstance(SimThingId),
}

impl<'a> CapabilityTreeBoundaryHandler<'a> {
    /// Activate capability entries whose progress thresholds fired this tick.
    ///
    /// Caller is responsible for resolving raw GPU `ThresholdEvent`s into
    /// `CapabilityUnlockEvent`s (via `ThresholdRegistry::extract_capability_unlocks`
    /// in `simthing-sim`, or by direct construction in tests). This keeps
    /// `simthing-spec` independent of the GPU and threshold-registry crates.
    pub fn handle_capability_unlock_events(
        &self,
        events: &[CapabilityUnlockEvent],
        ctx: &mut CapabilityBoundaryContext<'_>,
    ) -> Result<(), CapabilityTreeError> {
        let mut touched = HashSet::new();

        for event in events {
            let Some(instance) = self.instance_by_tree_thing(event.sim_thing_id, ctx) else {
                ctx.diagnostics
                    .push(CapabilityTreeDiagnostic::UnknownThresholdSimThing {
                        sim_thing_id: event.sim_thing_id,
                    });
                continue;
            };
            let instance = instance.clone();

            let Some(definition) = self.definitions.get(&instance.definition_id) else {
                ctx.diagnostics
                    .push(CapabilityTreeDiagnostic::UnknownDefinition {
                        definition_id: instance.definition_id,
                    });
                continue;
            };

            let Some(entry_key) = definition
                .by_threshold
                .get(&(event.property_id, event.sub_field.clone()))
                .cloned()
            else {
                continue;
            };

            let Some(entry) = definition.entries.get(&entry_key) else {
                ctx.diagnostics
                    .push(CapabilityTreeDiagnostic::EntryNotInTree {
                        definition_id: instance.definition_id,
                        entry: entry_key,
                    });
                continue;
            };

            if self.prereqs_met(definition, &entry_key, &instance, ctx)? {
                self.emit_activation(definition, &instance, entry_key, ctx, false)?;
            } else {
                let idx = shadow_index(instance.tree_slot, ctx.n_dims, entry.progress_col);
                if let Some(cell) = ctx.shadow.get_mut(idx) {
                    *cell = (entry.research_cost - PREREQ_RESET_EPSILON).max(0.0);
                }
                let state = ctx
                    .states
                    .get_mut(&instance.owner_id)
                    .ok_or(CapabilityTreeError::MissingState(instance.owner_id))?;
                state
                    .activation_mode_by_entry
                    .insert(entry_key, ActivationMode::OnPrereqMet);
            }
            touched.insert(instance.owner_id);
        }

        for owner_id in touched {
            self.sweep_on_prereq_met(owner_id, ctx)?;
        }

        Ok(())
    }

    pub fn handle_player_selection(
        &self,
        owner_id: SimThingId,
        entry_key: CapabilityEntryKey,
        ctx: &mut CapabilityBoundaryContext<'_>,
    ) -> Result<(), CapabilityTreeError> {
        let instance = ctx
            .instances
            .get(&owner_id)
            .cloned()
            .ok_or(CapabilityTreeError::MissingInstance(owner_id))?;
        let definition = self
            .definitions
            .get(&instance.definition_id)
            .ok_or_else(|| CapabilityTreeError::EntryNotInTree(entry_key.to_string()))?;
        let entry = definition
            .entries
            .get(&entry_key)
            .ok_or_else(|| CapabilityTreeError::EntryNotInTree(entry_key.to_string()))?;

        if entry.activation != ActivationMode::PlayerSelection {
            return Err(
                CapabilityTreeError::PlayerSelectionRequiresPlayerSelectionMode(
                    entry_key.to_string(),
                ),
            );
        }

        self.emit_activation(definition, &instance, entry_key, ctx, true)
    }

    pub fn sweep_on_prereq_met(
        &self,
        owner_id: SimThingId,
        ctx: &mut CapabilityBoundaryContext<'_>,
    ) -> Result<(), CapabilityTreeError> {
        loop {
            let instance = ctx
                .instances
                .get(&owner_id)
                .cloned()
                .ok_or(CapabilityTreeError::MissingInstance(owner_id))?;
            let definition = self
                .definitions
                .get(&instance.definition_id)
                .ok_or_else(|| CapabilityTreeError::MissingState(owner_id))?;

            let candidates: Vec<CapabilityEntryKey> = ctx
                .states
                .get(&owner_id)
                .ok_or(CapabilityTreeError::MissingState(owner_id))?
                .activation_mode_by_entry
                .iter()
                .filter_map(|(entry, mode)| {
                    (*mode == ActivationMode::OnPrereqMet).then(|| entry.clone())
                })
                .collect();

            let mut activated = Vec::new();
            for entry_key in candidates {
                if self.prereqs_met(definition, &entry_key, &instance, ctx)? {
                    self.emit_activation(definition, &instance, entry_key.clone(), ctx, false)?;
                    activated.push(entry_key);
                }
            }

            if activated.is_empty() {
                break;
            }

            let state = ctx
                .states
                .get_mut(&owner_id)
                .ok_or(CapabilityTreeError::MissingState(owner_id))?;
            for entry_key in activated {
                state.activation_mode_by_entry.remove(&entry_key);
            }
        }

        Ok(())
    }

    fn emit_activation(
        &self,
        definition: &CapabilityTreeDefinition,
        instance: &CapabilityTreeInstance,
        entry_key: CapabilityEntryKey,
        ctx: &mut CapabilityBoundaryContext<'_>,
        sweep_after: bool,
    ) -> Result<(), CapabilityTreeError> {
        // Validate the entry exists in the definition (O1b: we no longer
        // read `entry.overlay_ids` here — those are template ids. Per-clone
        // overlay ids live on `instance.by_overlay`, re-stamped at install
        // time via `OverlayId::new()`.)
        let _entry = definition
            .entries
            .get(&entry_key)
            .ok_or_else(|| CapabilityTreeError::EntryNotInTree(entry_key.to_string()))?;

        for overlay_id in clone_overlay_ids_for_entry(instance, &entry_key) {
            ctx.requests.push(BoundaryRequest::ActivateOverlay {
                target: overlay_host(instance, overlay_id),
                overlay_id,
            });
        }

        let category_key = entry_key.category.clone();
        let category = definition
            .categories
            .get(&category_key)
            .ok_or_else(|| CapabilityTreeError::UnsupportedMaxActive(category_key.to_string()))?;
        let state = ctx
            .states
            .get_mut(&instance.owner_id)
            .ok_or(CapabilityTreeError::MissingState(instance.owner_id))?;
        let category_active = state
            .active_by_category
            .entry(category_key.clone())
            .or_default();

        match &category.max_active {
            None | Some(MaxActivePolicy::Unlimited) => {
                if !category_active.contains(&entry_key) {
                    category_active.push(entry_key);
                }
            }
            Some(MaxActivePolicy::Limited {
                count: 1,
                replacement: ReplacementPolicy::SuspendOldest,
            }) => {
                if category_active.contains(&entry_key) {
                    return Ok(());
                }
                if let Some(oldest) = category_active.first().cloned() {
                    // O1b: same per-clone resolution as the activation path.
                    // `definition.entries[oldest].overlay_ids` holds template
                    // ids; the live clone's ids live on `instance.by_overlay`.
                    for overlay_id in clone_overlay_ids_for_entry(instance, &oldest) {
                        ctx.requests.push(BoundaryRequest::SuspendOverlay {
                            target: overlay_host(instance, overlay_id),
                            overlay_id,
                        });
                    }
                    category_active.remove(0);
                    ctx.notifications
                        .push(CapabilityTreeNotification::IdeaSwitched {
                            owner_id: instance.owner_id,
                            category: category_key.clone(),
                            suspended: oldest,
                            activated: entry_key.clone(),
                        });
                }
                category_active.push(entry_key);
            }
            Some(MaxActivePolicy::Limited { count, .. }) => {
                return Err(CapabilityTreeError::UnsupportedMaxActive(format!(
                    "{} ({count})",
                    category_key
                )));
            }
        }

        if sweep_after {
            self.sweep_on_prereq_met(instance.owner_id, ctx)?;
        }

        Ok(())
    }

    fn prereqs_met(
        &self,
        definition: &CapabilityTreeDefinition,
        entry_key: &CapabilityEntryKey,
        instance: &CapabilityTreeInstance,
        ctx: &CapabilityBoundaryContext<'_>,
    ) -> Result<bool, CapabilityTreeError> {
        let entry = definition
            .entries
            .get(entry_key)
            .ok_or_else(|| CapabilityTreeError::EntryNotInTree(entry_key.to_string()))?;
        Ok(entry.prereqs.iter().all(|prereq| {
            let idx = shadow_index(instance.tree_slot, ctx.n_dims, prereq.col);
            ctx.shadow
                .get(idx)
                .is_some_and(|value| *value >= prereq.min_value)
        }))
    }

    fn instance_by_tree_thing<'ctx>(
        &self,
        tree_thing_id: SimThingId,
        ctx: &'ctx CapabilityBoundaryContext<'_>,
    ) -> Option<&'ctx CapabilityTreeInstance> {
        ctx.instances
            .values()
            .find(|instance| instance.tree_thing_id == tree_thing_id)
    }
}

fn shadow_index(slot: u32, n_dims: usize, col: usize) -> usize {
    slot as usize * n_dims + col
}

/// Collect the per-clone `OverlayId`s belonging to `entry_key` on this
/// instance. `instance.by_overlay` is the inverse map (clone-id →
/// entry-key) stamped at install time by `install_tree_for_owner`; we
/// scan it for matches and sort numerically so the activation /
/// suspension request order is deterministic across runs.
///
/// Sorting by the underlying `OverlayId` counter is equivalent to
/// install-time authoring order because `OverlayId::new()` is a
/// monotonic atomic counter and install processes template overlays in
/// their authored sequence.
fn clone_overlay_ids_for_entry(
    instance: &CapabilityTreeInstance,
    entry_key: &CapabilityEntryKey,
) -> Vec<OverlayId> {
    let mut ids: Vec<OverlayId> = instance
        .by_overlay
        .iter()
        .filter_map(|(oid, ek)| (ek == entry_key).then_some(*oid))
        .collect();
    ids.sort();
    ids
}

/// Pick the SimThing that hosts a given overlay. EffectTarget-driven:
/// `Owner`-targeted overlays live on the owner, `SessionRoot`-targeted on
/// the scenario root, `CapabilityTree`-targeted on the clone. Falls back
/// to `tree_thing_id` when `overlay_hosts` is empty (older hand-built
/// tests pre-dating the EffectTarget ADR).
fn overlay_host(instance: &CapabilityTreeInstance, overlay_id: OverlayId) -> SimThingId {
    instance
        .overlay_hosts
        .get(&overlay_id)
        .copied()
        .unwrap_or(instance.tree_thing_id)
}

#[cfg(test)]
mod atomicity_tests {
    //! CAPABILITY-PREREQ-DAG-ADMISSION-0 max_active same-barrier atomicity referee.
    use super::*;
    use crate::keys::CategoryKey;
    use crate::runtime::{
        CapabilityCategoryDefinition, CapabilityDefinition, CapabilityTreeDefinition,
        CapabilityTreeDefinitionId,
    };
    use crate::spec::capability::EffectTarget;
    use simthing_core::{
        DimensionRegistry, PropertyTransformDelta, SimPropertyId, SubFieldRole, TransformOp,
    };
    use simthing_feeder::BoundaryRequest;

    fn idea_entry(
        category: CategoryKey,
        entry_id: &str,
        overlay_id: OverlayId,
    ) -> (CapabilityEntryKey, CapabilityDefinition) {
        let key = CapabilityEntryKey::new(category, entry_id);
        let def = CapabilityDefinition {
            key: key.clone(),
            display_name: entry_id.into(),
            description: String::new(),
            flavor_text: None,
            activation: ActivationMode::PlayerSelection,
            overlay_ids: vec![overlay_id],
            effect_keys: vec![],
            effect_transforms: vec![PropertyTransformDelta {
                property_id: SimPropertyId(0),
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(1.0))],
            }],
            effect_targets: vec![EffectTarget::Owner],
            prereqs: vec![],
            progress_col: 0,
            research_cost: 0.0,
        };
        (key, def)
    }

    fn limited_ideas_fixture() -> (
        CapabilityTreeDefinition,
        CapabilityTreeInstance,
        CapabilityEntryKey,
        CapabilityEntryKey,
        OverlayId,
        OverlayId,
    ) {
        let category = CategoryKey::new("ideas", "tier1");
        let overlay_a = OverlayId::new();
        let overlay_b = OverlayId::new();
        let (key_a, def_a) = idea_entry(category.clone(), "idea_a", overlay_a);
        let (key_b, def_b) = idea_entry(category.clone(), "idea_b", overlay_b);

        let mut entries = HashMap::new();
        entries.insert(key_a.clone(), def_a);
        entries.insert(key_b.clone(), def_b);

        let mut categories = HashMap::new();
        categories.insert(
            category.clone(),
            CapabilityCategoryDefinition {
                key: category,
                property_id: SimPropertyId(0),
                max_active: Some(MaxActivePolicy::Limited {
                    count: 1,
                    replacement: ReplacementPolicy::SuspendOldest,
                }),
                tier: 0,
            },
        );

        let definition = CapabilityTreeDefinition {
            id: CapabilityTreeDefinitionId::new(),
            tree_id: "national_ideas".into(),
            categories,
            entries,
            by_threshold: HashMap::new(),
        };

        let owner_id = SimThingId::new();
        let tree_thing_id = SimThingId::new();
        let mut by_overlay = HashMap::new();
        by_overlay.insert(overlay_a, key_a.clone());
        by_overlay.insert(overlay_b, key_b.clone());
        let mut overlay_hosts = HashMap::new();
        overlay_hosts.insert(overlay_a, owner_id);
        overlay_hosts.insert(overlay_b, owner_id);

        let instance = CapabilityTreeInstance {
            owner_id,
            definition_id: definition.id,
            tree_thing_id,
            tree_slot: 0,
            by_overlay,
            overlay_hosts,
        };

        (definition, instance, key_a, key_b, overlay_a, overlay_b)
    }

    /// Referee: activate A then B; Suspend(A)+Activate(B) land in one barrier
    /// batch — no dual-active intermediate across a generation.
    #[test]
    fn max_active_sibling_suspend_is_atomic_at_generation_barrier() {
        let (definition, instance, key_a, key_b, overlay_a, overlay_b) = limited_ideas_fixture();
        let owner_id = instance.owner_id;
        let def_id = definition.id;
        let mut definitions = HashMap::new();
        definitions.insert(def_id, definition);

        let registry = DimensionRegistry::new();
        let handler = CapabilityTreeBoundaryHandler {
            registry: &registry,
            definitions: &definitions,
        };

        let instances = HashMap::from([(owner_id, instance)]);
        let mut states = HashMap::from([(
            owner_id,
            CapabilityTreeState {
                owner_id,
                definition_id: def_id,
                activation_mode_by_entry: HashMap::new(),
                active_by_category: HashMap::new(),
            },
        )]);

        let mut shadow = vec![0.0_f32; 8];
        let mut diagnostics = Vec::new();

        // Barrier 1: activate idea_a alone.
        let mut requests = Vec::new();
        let mut notifications = Vec::new();
        {
            let mut ctx = CapabilityBoundaryContext {
                n_dims: 1,
                shadow: &mut shadow,
                instances: &instances,
                states: &mut states,
                requests: &mut requests,
                notifications: &mut notifications,
                diagnostics: &mut diagnostics,
            };
            handler
                .handle_player_selection(owner_id, key_a.clone(), &mut ctx)
                .expect("first activation");
        }
        assert_eq!(requests.len(), 1);
        assert!(matches!(
            &requests[0],
            BoundaryRequest::ActivateOverlay { overlay_id, .. } if *overlay_id == overlay_a
        ));
        let active: Vec<_> = states[&owner_id]
            .active_by_category
            .values()
            .flatten()
            .cloned()
            .collect();
        assert_eq!(active, vec![key_a.clone()]);

        // Barrier 2: switch to idea_b — suspend+activate same batch.
        requests.clear();
        notifications.clear();
        {
            let mut ctx = CapabilityBoundaryContext {
                n_dims: 1,
                shadow: &mut shadow,
                instances: &instances,
                states: &mut states,
                requests: &mut requests,
                notifications: &mut notifications,
                diagnostics: &mut diagnostics,
            };
            handler
                .handle_player_selection(owner_id, key_b.clone(), &mut ctx)
                .expect("second activation switches ideas");
        }

        let suspends: Vec<_> = requests
            .iter()
            .filter_map(|r| match r {
                BoundaryRequest::SuspendOverlay { overlay_id, .. } => Some(*overlay_id),
                _ => None,
            })
            .collect();
        let activates: Vec<_> = requests
            .iter()
            .filter_map(|r| match r {
                BoundaryRequest::ActivateOverlay { overlay_id, .. } => Some(*overlay_id),
                _ => None,
            })
            .collect();
        assert_eq!(suspends, vec![overlay_a]);
        assert_eq!(activates, vec![overlay_b]);
        // Atomicity = both requests land in the SAME barrier batch (v1 §8 /
        // generation barrier). Unlock emission order is unchanged; the
        // maintainer drains the whole batch at one generation.
        assert_eq!(
            requests.len(),
            2,
            "switch must be exactly one suspend + one activate in one barrier"
        );

        let active_after: Vec<_> = states[&owner_id]
            .active_by_category
            .values()
            .flatten()
            .cloned()
            .collect();
        assert_eq!(active_after, vec![key_b.clone()]);
        assert!(!active_after.contains(&key_a));

        assert!(notifications.iter().any(|n| matches!(
            n,
            CapabilityTreeNotification::IdeaSwitched {
                suspended,
                activated,
                ..
            } if suspended == &key_a && activated == &key_b
        )));
    }
}
