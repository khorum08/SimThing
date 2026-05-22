use crate::diagnostics::{CapabilityEntryKeyRef, CapabilityTreeDiagnostic};
use crate::error::CapabilityTreeError;
use crate::keys::{CapabilityEntryKey, CapabilityTreeDefinitionId, CategoryKey};
use crate::runtime::{CapabilityTreeDefinition, CapabilityTreeInstance, CapabilityTreeState};
use crate::spec::capability::{ActivationMode, MaxActivePolicy};
use crate::compile::capability::progress_reset_value;
use simthing_core::DimensionRegistry;
use simthing_feeder::BoundaryRequest;
use simthing_core::SimThingId;
use std::collections::HashMap;

/// Fired Pass 7 capability-unlock threshold (after CPU semantic resolution).
#[derive(Clone, Debug, PartialEq)]
pub struct CapabilityUnlockEvent {
    pub tree_thing_id: SimThingId,
    pub tree_slot:     u32,
    pub entry:         CapabilityEntryKey,
    pub value:         f32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CapabilityBoundaryOutcome {
    pub activations:  usize,
    pub suspensions:    usize,
    pub prereq_resets:  usize,
    pub on_prereq_met:  usize,
}

pub struct CapabilityTreeBoundaryHandler<'a> {
    pub registry:    &'a DimensionRegistry,
    pub definitions: &'a HashMap<CapabilityTreeDefinitionId, CapabilityTreeDefinition>,
}

pub struct CapabilityBoundaryContext<'a> {
    pub n_dims:      usize,
    pub shadow:      &'a mut [f32],
    pub instances:   &'a HashMap<SimThingId, CapabilityTreeInstance>,
    pub states:      &'a mut HashMap<SimThingId, CapabilityTreeState>,
    pub requests:    &'a mut Vec<BoundaryRequest>,
    pub diagnostics: &'a mut Vec<CapabilityTreeDiagnostic>,
}

impl<'a> CapabilityTreeBoundaryHandler<'a> {
    pub fn handle_threshold_events(
        &self,
        events: &[CapabilityUnlockEvent],
        ctx: &mut CapabilityBoundaryContext<'_>,
    ) -> Result<CapabilityBoundaryOutcome, CapabilityTreeError> {
        let mut outcome = CapabilityBoundaryOutcome::default();
        for event in events {
            self.handle_unlock_event(event, ctx, &mut outcome)?;
        }
        self.sweep_on_prereq_met(ctx, &mut outcome)?;
        Ok(outcome)
    }

    pub fn handle_player_selection(
        &self,
        owner_id: SimThingId,
        entry: CapabilityEntryKey,
        ctx: &mut CapabilityBoundaryContext<'_>,
    ) -> Result<CapabilityBoundaryOutcome, CapabilityTreeError> {
        let mut outcome = CapabilityBoundaryOutcome::default();
        let instance = ctx
            .instances
            .get(&owner_id)
            .ok_or(CapabilityTreeError::UnknownOwner(owner_id))?
            .clone();
        let definition = self
            .definitions
            .get(&instance.definition_id)
            .ok_or(CapabilityTreeError::UnknownEntry(entry.clone()))?;
        let entry_def = definition
            .entry(&entry)
            .ok_or(CapabilityTreeError::UnknownEntry(entry.clone()))?;

        if entry_def.default_activation != ActivationMode::PlayerSelection {
            return Err(CapabilityTreeError::UnknownEntry(entry));
        }

        set_progress(
            ctx.shadow,
            instance.tree_slot,
            ctx.n_dims,
            entry_def.progress_col,
            entry_def.research_cost.max(1.0),
        );

        self.activate_entry(
            &instance,
            definition,
            entry_def,
            ctx,
            &mut outcome,
        )?;
        self.sweep_on_prereq_met(ctx, &mut outcome)?;
        Ok(outcome)
    }

    fn handle_unlock_event(
        &self,
        event: &CapabilityUnlockEvent,
        ctx: &mut CapabilityBoundaryContext<'_>,
        outcome: &mut CapabilityBoundaryOutcome,
    ) -> Result<(), CapabilityTreeError> {
        let instance = find_instance(ctx.instances, event.tree_thing_id)
            .ok_or(CapabilityTreeError::UnknownTree(event.tree_thing_id))?;
        let definition = self
            .definitions
            .get(&instance.definition_id)
            .ok_or(CapabilityTreeError::UnknownEntry(event.entry.clone()))?;
        let entry_def = definition
            .entry(&event.entry)
            .ok_or(CapabilityTreeError::UnknownEntry(event.entry.clone()))?;

        let state = ctx.states.get_mut(&instance.owner_id).unwrap();
        let mode = state.activation_mode(&event.entry, entry_def.default_activation);
        if mode != ActivationMode::Threshold {
            return Ok(());
        }

        if !prereqs_met(entry_def, instance.tree_slot, ctx.n_dims, ctx.shadow) {
            let reset = progress_reset_value(entry_def.research_cost);
            set_progress(
                ctx.shadow,
                instance.tree_slot,
                ctx.n_dims,
                entry_def.progress_col,
                reset,
            );
            state.set_activation_mode(event.entry.clone(), ActivationMode::OnPrereqMet);
            ctx.diagnostics.push(CapabilityTreeDiagnostic::PrereqsUnmet {
                entry: CapabilityEntryKeyRef::from_key(&definition.tree_id, &event.entry),
            });
            ctx.diagnostics.push(CapabilityTreeDiagnostic::ProgressReset {
                entry: CapabilityEntryKeyRef::from_key(&definition.tree_id, &event.entry),
                value: reset,
            });
            ctx.diagnostics.push(CapabilityTreeDiagnostic::ActivationModeChanged {
                entry: CapabilityEntryKeyRef::from_key(&definition.tree_id, &event.entry),
                from:  ActivationMode::Threshold,
                to:    ActivationMode::OnPrereqMet,
            });
            outcome.prereq_resets += 1;
            return Ok(());
        }

        self.activate_entry(instance, definition, entry_def, ctx, outcome)
    }

    fn activate_entry(
        &self,
        instance: &CapabilityTreeInstance,
        definition: &CapabilityTreeDefinition,
        entry_def: &crate::runtime::CapabilityDefinition,
        ctx: &mut CapabilityBoundaryContext<'_>,
        outcome: &mut CapabilityBoundaryOutcome,
    ) -> Result<(), CapabilityTreeError> {
        let category = definition
            .categories
            .get(&entry_def.category_key)
            .expect("category exists");

        if let MaxActivePolicy::Limited { count } = category.max_active {
            enforce_max_active(
                count,
                &entry_def.category_key,
                &entry_def.key,
                instance,
                definition,
                ctx,
                outcome,
            )?;
        }

        for overlay_id in &entry_def.overlay_ids {
            ctx.requests.push(BoundaryRequest::ActivateOverlay {
                target:     instance.tree_thing_id,
                overlay_id: *overlay_id,
            });
        }

        let state = ctx.states.get_mut(&instance.owner_id).unwrap();
        state
            .active_by_category
            .entry(entry_def.category_key.clone())
            .or_default()
            .retain(|k| k != &entry_def.key);
        state
            .active_by_category
            .entry(entry_def.category_key.clone())
            .or_default()
            .push(entry_def.key.clone());

        state.set_activation_mode(entry_def.key.clone(), ActivationMode::Threshold);
        outcome.activations += 1;
        Ok(())
    }

    fn sweep_on_prereq_met(
        &self,
        ctx: &mut CapabilityBoundaryContext<'_>,
        outcome: &mut CapabilityBoundaryOutcome,
    ) -> Result<(), CapabilityTreeError> {
        let instance_snapshot: Vec<_> = ctx.instances.values().cloned().collect();
        for instance in instance_snapshot {
            let definition = match self.definitions.get(&instance.definition_id) {
                Some(d) => d,
                None => continue,
            };
            let pending: Vec<_> = definition
                .entries
                .values()
                .filter(|entry| {
                    ctx.states
                        .get(&instance.owner_id)
                        .map(|s| {
                            s.activation_mode(&entry.key, entry.default_activation)
                                == ActivationMode::OnPrereqMet
                        })
                        .unwrap_or(false)
                })
                .cloned()
                .collect();

            for entry_def in pending {
                if !prereqs_met(&entry_def, instance.tree_slot, ctx.n_dims, ctx.shadow) {
                    continue;
                }
                self.activate_entry(&instance, definition, &entry_def, ctx, outcome)?;
                outcome.on_prereq_met += 1;
            }
        }
        Ok(())
    }
}

fn find_instance(
    instances: &HashMap<SimThingId, CapabilityTreeInstance>,
    tree_thing_id: SimThingId,
) -> Option<&CapabilityTreeInstance> {
    instances.values().find(|i| i.tree_thing_id == tree_thing_id)
}

fn prereqs_met(
    entry: &crate::runtime::CapabilityDefinition,
    tree_slot: u32,
    n_dims: usize,
    shadow: &[f32],
) -> bool {
    let base = tree_slot as usize * n_dims;
    entry.prereqs.iter().all(|p| {
        shadow.get(base + p.col)
            .copied()
            .unwrap_or(0.0) >= p.min_value
    })
}

fn set_progress(shadow: &mut [f32], tree_slot: u32, n_dims: usize, col: usize, value: f32) {
    let idx = tree_slot as usize * n_dims + col;
    if let Some(slot) = shadow.get_mut(idx) {
        *slot = value;
    }
}

fn enforce_max_active(
    count: usize,
    category_key: &CategoryKey,
    incoming: &CapabilityEntryKey,
    instance: &CapabilityTreeInstance,
    definition: &CapabilityTreeDefinition,
    ctx: &mut CapabilityBoundaryContext<'_>,
    outcome: &mut CapabilityBoundaryOutcome,
) -> Result<(), CapabilityTreeError> {
    let displaced: Vec<CapabilityEntryKey> = {
        let state = ctx.states.get_mut(&instance.owner_id).unwrap();
        let active = state
            .active_by_category
            .entry(category_key.clone())
            .or_default();
        active.retain(|k| k != incoming);
        let mut removed = Vec::new();
        while active.len() >= count {
            removed.push(active.remove(0));
        }
        removed
    };

    for displaced in displaced {
        let displaced_def = definition
            .entry(&displaced)
            .ok_or(CapabilityTreeError::UnknownEntry(displaced.clone()))?;
        for overlay_id in &displaced_def.overlay_ids {
            ctx.requests.push(BoundaryRequest::SuspendOverlay {
                target:     instance.tree_thing_id,
                overlay_id: *overlay_id,
            });
            outcome.suspensions += 1;
        }
        ctx.diagnostics.push(CapabilityTreeDiagnostic::MutualExclusivitySuspended {
            entry: CapabilityEntryKeyRef::from_key(&definition.tree_id, &displaced),
        });
    }
    Ok(())
}
