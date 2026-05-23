//! CPU-side threshold semantic registry.
//!
//! GPU Pass 7 emits `ThresholdEvent { slot, col, value, event_kind }`.
//! `event_kind` is an opaque `u32` assigned at threshold-registration time.
//! This module owns the mapping from that u32 back to a meaningful CPU action.
//!
//! ## Design
//!
//! `ThresholdRegistry` is a `Vec<ThresholdSemantic>` indexed directly by
//! `event_kind`. Registration is append-only within a day; the index grows
//! monotonically. At each day boundary the registry is rebuilt from scratch
//! so that tombstoned slots and newly-spawned SimThings are reflected.
//!
//! `ThresholdBuilder` walks the SimThing tree and derives both:
//! - `Vec<ThresholdRegistration>` (GPU upload via `state.upload_thresholds`)
//! - `Vec<ThresholdSemantic>` (CPU lookup via `ThresholdRegistry`)
//!
//! In parallel, keyed by the same sequential `event_kind` index.
//!
//! ## Sources of thresholds
//!
//! Per design_v4.md §6 and §7:
//!
//! 1. **`FissionThreshold`** on a `SimProperty` — when Amount/Intensity
//!    crosses the threshold on any live SimThing that has that property.
//!    One GPU `ThresholdRegistration` per (live sim_thing, fission_template).
//!
//! 2. **Fusion thresholds** are registered from `FissionLineageRecord`s held
//!    by the boundary protocol. Each lineage record produces one
//!    `FusionTrigger` watching the child's activating-property Intensity
//!    column, threshold = template.fusion_intensity_threshold, direction =
//!    Upward (intensity climbs back up as the schism dissolves). When the
//!    threshold fires, fusion executes: the parent absorbs a scar
//!    multiplier on its activating-property Amount and the child is
//!    tombstoned.
//!
//! 3. **`DecayBehavior::OnThreshold`** — property self-removes when its own
//!    Amount crosses a threshold. Emits `PropertyExpiry`.
//!
//! 4. **`DecayBehavior::IntensityGated`** — property self-removes when
//!    intensity drops below the floor. Emits `PropertyExpiry`.
//!
//! 5. **`DecayBehavior::WhenProperty`** — property self-removes when another
//!    property on the same SimThing crosses a threshold. Emits `PropertyExpiry`.
//!
//! 6. **Velocity alerts** — registered by the AI layer against a specific
//!    SimThing/property/sub-field trajectory.

use serde::{Deserialize, Serialize};
use simthing_core::{
    DecayBehavior, DimensionRegistry, Direction, SimPropertyId, SimThing, SimThingId, SubFieldRole,
};
use simthing_feeder::{
    CapabilityUnlockEvent, CapabilityUnlockRegistration, ScriptedEventTriggerEvent,
    ScriptedEventTriggerRegistration,
};
use simthing_gpu::{
    SlotAllocator, ThresholdEvent, ThresholdRegistration, DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD,
    THRESH_BUF_OUTPUT, THRESH_BUF_VALUES,
};

use crate::fission::FissionLineageRecord;

// ── Semantic action ───────────────────────────────────────────────────────────

/// What a fired `ThresholdEvent` means to the CPU boundary protocol.
/// Indexed by `ThresholdEvent::event_kind` in the `ThresholdRegistry`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ThresholdSemantic {
    /// A `FissionThreshold` fired. The boundary must check the secondary
    /// condition (if any) and, if satisfied, spawn a new SimThing child.
    FissionTrigger {
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
        /// Index into `SimProperty::fission_templates`.
        template_idx: usize,
    },

    /// A `FusionThreshold` fired on a previously-fissioned child. The
    /// boundary merges the child back into the parent, applies the scar
    /// coefficient, and removes the child's slot.
    FusionTrigger {
        /// The child SimThing to dissolve.
        child_id: SimThingId,
        /// The parent that receives the scar.
        parent_id: SimThingId,
        property_id: SimPropertyId,
        /// Index into `SimProperty::fusion_templates`.
        template_idx: usize,
    },

    /// A `DecayBehavior` threshold fired. The boundary removes this property
    /// from the SimThing's `properties` map and tombstones the registry
    /// column if no other live SimThing carries it.
    PropertyExpiry {
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
    },

    /// Velocity alert (AI-registered). Scans per-slot `values`.
    VelocityAlert {
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
        sub_field: SubFieldRole,
    },

    /// Aggregate alert (AI-registered). Scans post-reduction `output_vectors`
    /// on inner nodes (e.g. location instability from cohort children).
    AggregateAlert {
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
        sub_field: SubFieldRole,
    },

    /// A capability tree progress threshold fired. Emitted from a
    /// `CapabilityUnlockRegistration` (one per `ActivationMode::Threshold`
    /// entry compiled by `simthing-spec::CapabilityTreeBuilder`). PR 5's
    /// `CapabilityTreeBoundaryHandler` reads the `sim_thing_id` to find
    /// the faction's `CapabilityTreeInstance`, then looks up the entry via
    /// `definition.by_threshold[(property_id, sub_field)]`.
    CapabilityUnlock {
        sim_thing_id: SimThingId,
        property_id:  SimPropertyId,
        sub_field:    SubFieldRole,
    },

    /// A scripted event's threshold trigger fired. Emitted from a
    /// `ScriptedEventTriggerRegistration` (one per `CompiledTrigger::Threshold`
    /// scripted event registered with the session). The spec layer's
    /// `ScriptedEventBoundaryHandler::handle_tick` matches `event_id` against
    /// its `ScriptedEventDefinition` slice and fires the corresponding effects
    /// under standard cooldown/priority gating.
    ScriptedEventTrigger {
        /// Matches `simthing_spec::EventKey.0`.
        event_id: String,
    },
}

/// AI-facing threshold registration for a rate/trajectory column on `values`.
#[derive(Clone, Debug)]
pub struct VelocityAlertRegistration {
    pub sim_thing_id: SimThingId,
    pub property_id: SimPropertyId,
    pub sub_field: SubFieldRole,
    pub threshold: f32,
    pub direction: Direction,
}

/// AI-facing threshold on post-reduction `output_vectors` (parent aggregates).
#[derive(Clone, Debug)]
pub struct AggregateAlertRegistration {
    pub sim_thing_id: SimThingId,
    pub property_id: SimPropertyId,
    pub sub_field: SubFieldRole,
    pub threshold: f32,
    pub direction: Direction,
}

/// Fired aggregate alert surfaced by the boundary protocol.
#[derive(Clone, Debug, PartialEq)]
pub struct AggregateAlertEvent {
    pub sim_thing_id: SimThingId,
    pub property_id: SimPropertyId,
    pub sub_field: SubFieldRole,
    pub value: f32,
}

/// Fired velocity alert surfaced by the boundary protocol.
#[derive(Clone, Debug, PartialEq)]
pub struct VelocityAlertEvent {
    pub sim_thing_id: SimThingId,
    pub property_id: SimPropertyId,
    pub sub_field: SubFieldRole,
    pub value: f32,
}

// ── Registry ──────────────────────────────────────────────────────────────────

/// Parallel-indexed companion to the GPU threshold_registry buffer.
/// `registry[event_kind]` → `ThresholdSemantic`.
/// Rebuilt from scratch at each boundary by `ThresholdBuilder`.
#[derive(Clone, Debug, Default)]
pub struct ThresholdRegistry {
    entries: Vec<ThresholdSemantic>,
}

impl ThresholdRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Look up the semantic action for a fired event.
    pub fn get(&self, event_kind: u32) -> Option<&ThresholdSemantic> {
        self.entries.get(event_kind as usize)
    }

    /// Push a new entry and return the event_kind assigned to it.
    pub fn push(&mut self, sem: ThresholdSemantic) -> u32 {
        let idx = self.entries.len() as u32;
        self.entries.push(sem);
        idx
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Filter a slice of GPU `ThresholdEvent`s down to just the
    /// `ThresholdSemantic::CapabilityUnlock` arms, resolved into
    /// `CapabilityUnlockEvent`s for `simthing-spec`'s boundary handler.
    ///
    /// This is the conversion bridge that lets the spec layer stay
    /// independent of `simthing-gpu` and `simthing-sim`: callers that hold
    /// raw threshold events call this, then hand the result to
    /// `CapabilityTreeBoundaryHandler::handle_capability_unlock_events`.
    /// Non-`CapabilityUnlock` arms and events with out-of-range `event_kind`
    /// are silently filtered out.
    pub fn extract_capability_unlocks(
        &self,
        events: &[ThresholdEvent],
    ) -> Vec<CapabilityUnlockEvent> {
        events
            .iter()
            .filter_map(|event| match self.get(event.event_kind)? {
                ThresholdSemantic::CapabilityUnlock {
                    sim_thing_id,
                    property_id,
                    sub_field,
                } => Some(CapabilityUnlockEvent {
                    sim_thing_id: *sim_thing_id,
                    property_id:  *property_id,
                    sub_field:    sub_field.clone(),
                }),
                _ => None,
            })
            .collect()
    }

    /// Filter a slice of GPU `ThresholdEvent`s down to just the
    /// `ThresholdSemantic::ScriptedEventTrigger` arms, resolved into
    /// `ScriptedEventTriggerEvent`s for `simthing-spec`'s scripted-event
    /// boundary handler. Sibling to `extract_capability_unlocks` — same
    /// pattern, different semantic arm. Non-`ScriptedEventTrigger` arms and
    /// out-of-range `event_kind`s are silently filtered out.
    pub fn extract_scripted_event_triggers(
        &self,
        events: &[ThresholdEvent],
    ) -> Vec<ScriptedEventTriggerEvent> {
        events
            .iter()
            .filter_map(|event| match self.get(event.event_kind)? {
                ThresholdSemantic::ScriptedEventTrigger { event_id } => {
                    Some(ScriptedEventTriggerEvent {
                        event_id: event_id.clone(),
                    })
                }
                _ => None,
            })
            .collect()
    }
}

// ── Builder ───────────────────────────────────────────────────────────────────

/// Walks the SimThing tree and derives GPU + CPU threshold registrations.
/// Call `build()` at each day boundary after structural mutations complete.
/// Output: `(gpu_regs, cpu_registry)` ready for upload + lookup.
pub struct ThresholdBuilder;

impl ThresholdBuilder {
    /// Walk the tree and build both the GPU registration vec and the parallel
    /// CPU registry. The two vecs are index-aligned: `gpu_regs[i]` fires with
    /// `event_kind = i`, which the boundary resolves via `cpu_registry.get(i)`.
    pub fn build(
        root: &SimThing,
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
    ) -> (Vec<ThresholdRegistration>, ThresholdRegistry) {
        Self::build_with_velocity_alerts(root, dim_reg, allocator, &[])
    }

    pub fn build_with_velocity_alerts(
        root: &SimThing,
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        velocity_alerts: &[VelocityAlertRegistration],
    ) -> (Vec<ThresholdRegistration>, ThresholdRegistry) {
        Self::build_with_alerts(root, dim_reg, allocator, velocity_alerts, &[])
    }

    pub fn build_with_alerts(
        root: &SimThing,
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        velocity_alerts: &[VelocityAlertRegistration],
        aggregate_alerts: &[AggregateAlertRegistration],
    ) -> (Vec<ThresholdRegistration>, ThresholdRegistry) {
        Self::build_with_lineage(
            root, dim_reg, allocator,
            velocity_alerts, aggregate_alerts, &[],
        )
    }

    /// Build with fusion lineage. Each `FissionLineageRecord` produces one
    /// `FusionTrigger` registration watching the child's activating-property
    /// Intensity column, threshold = template.fusion_intensity_threshold,
    /// direction = Upward (intensity climbs back up as the schism dissolves).
    ///
    /// Lineage entries whose property has been tombstoned, whose template
    /// index is out of range, whose child slot can't be resolved, or whose
    /// property has no Intensity sub-field are silently skipped.
    pub fn build_with_lineage(
        root: &SimThing,
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        velocity_alerts: &[VelocityAlertRegistration],
        aggregate_alerts: &[AggregateAlertRegistration],
        lineage: &[FissionLineageRecord],
    ) -> (Vec<ThresholdRegistration>, ThresholdRegistry) {
        let mut gpu_regs = Vec::new();
        let mut cpu_reg = ThresholdRegistry::new();
        Self::walk(root, dim_reg, allocator, &mut gpu_regs, &mut cpu_reg);
        Self::push_fusion_lineage(
            dim_reg,
            allocator,
            lineage,
            &mut gpu_regs,
            &mut cpu_reg,
        );
        Self::push_velocity_alerts(
            dim_reg,
            allocator,
            velocity_alerts,
            &mut gpu_regs,
            &mut cpu_reg,
        );
        Self::push_aggregate_alerts(
            dim_reg,
            allocator,
            aggregate_alerts,
            &mut gpu_regs,
            &mut cpu_reg,
        );
        (gpu_regs, cpu_reg)
    }

    /// Build with capability unlocks. Each `CapabilityUnlockRegistration`
    /// produces one upward-direction Pass 7 watch on its `(slot, col)` pair
    /// with the matching `ThresholdSemantic::CapabilityUnlock` entry on the
    /// CPU registry. Capability unlocks watch the `values` buffer (per-slot
    /// progress columns), not `output_vectors`.
    ///
    /// Registrations whose property has been tombstoned, whose `sim_thing_id`
    /// can't be resolved to a slot, or whose `sub_field` is not present in
    /// the property layout are silently skipped — mirrors the velocity-alert
    /// and lineage skipping behavior.
    ///
    /// Full-rebuild path only. B2 append-only integration is a future PR;
    /// the first fission boundary after a capability tree is initialized
    /// takes the full-rebuild path regardless.
    pub fn build_with_capability_unlocks(
        root:                  &SimThing,
        dim_reg:               &DimensionRegistry,
        allocator:             &SlotAllocator,
        velocity_alerts:       &[VelocityAlertRegistration],
        capability_unlocks:    &[CapabilityUnlockRegistration],
    ) -> (Vec<ThresholdRegistration>, ThresholdRegistry) {
        let mut gpu_regs = Vec::new();
        let mut cpu_reg = ThresholdRegistry::new();
        Self::walk(root, dim_reg, allocator, &mut gpu_regs, &mut cpu_reg);
        Self::push_velocity_alerts(
            dim_reg, allocator, velocity_alerts,
            &mut gpu_regs, &mut cpu_reg,
        );
        Self::push_capability_unlocks(
            dim_reg, allocator, capability_unlocks,
            &mut gpu_regs, &mut cpu_reg,
        );
        (gpu_regs, cpu_reg)
    }

    /// Build with scripted event triggers. Each `ScriptedEventTriggerRegistration`
    /// produces one Pass 7 watch on its `(slot, col)` pair with the matching
    /// `ThresholdSemantic::ScriptedEventTrigger` entry on the CPU registry.
    /// Scripted event triggers watch the `values` buffer; targeting
    /// `output_vectors` aggregates is a future extension.
    ///
    /// Registrations whose `slot` exceeds the allocator's current capacity are
    /// kept verbatim — the caller is responsible for resolving `ScopeRef`
    /// targets to live slots before registration. (We don't have a property_id
    /// to validate against on this path, unlike capability unlocks.)
    ///
    /// Full-rebuild path only; B2 append-only integration is a future PR.
    pub fn build_with_scripted_event_triggers(
        root:                    &SimThing,
        dim_reg:                 &DimensionRegistry,
        allocator:               &SlotAllocator,
        velocity_alerts:         &[VelocityAlertRegistration],
        scripted_event_triggers: &[ScriptedEventTriggerRegistration],
    ) -> (Vec<ThresholdRegistration>, ThresholdRegistry) {
        let mut gpu_regs = Vec::new();
        let mut cpu_reg = ThresholdRegistry::new();
        Self::walk(root, dim_reg, allocator, &mut gpu_regs, &mut cpu_reg);
        Self::push_velocity_alerts(
            dim_reg, allocator, velocity_alerts,
            &mut gpu_regs, &mut cpu_reg,
        );
        Self::push_scripted_event_triggers(
            scripted_event_triggers,
            &mut gpu_regs, &mut cpu_reg,
        );
        (gpu_regs, cpu_reg)
    }

    fn push_scripted_event_triggers(
        scripted_event_triggers: &[ScriptedEventTriggerRegistration],
        gpu_regs:                &mut Vec<ThresholdRegistration>,
        cpu_reg:                 &mut ThresholdRegistry,
    ) {
        for trigger in scripted_event_triggers {
            let event_kind = cpu_reg.push(ThresholdSemantic::ScriptedEventTrigger {
                event_id: trigger.event_id.clone(),
            });
            gpu_regs.push(ThresholdRegistration {
                slot:      trigger.slot,
                col:       trigger.col,
                threshold: trigger.threshold,
                direction: direction_to_u32(&trigger.direction),
                event_kind,
                buffer:    THRESH_BUF_VALUES,
            });
        }
    }

    fn push_capability_unlocks(
        dim_reg:            &DimensionRegistry,
        allocator:          &SlotAllocator,
        capability_unlocks: &[CapabilityUnlockRegistration],
        gpu_regs:           &mut Vec<ThresholdRegistration>,
        cpu_reg:            &mut ThresholdRegistry,
    ) {
        for unlock in capability_unlocks {
            if !dim_reg.is_active(unlock.property_id) {
                continue;
            }
            let Some(slot) = allocator.slot_of(unlock.sim_thing_id) else {
                continue;
            };
            let range  = dim_reg.column_range(unlock.property_id);
            let layout = &dim_reg.property(unlock.property_id).layout;
            let Some(col) = range.col_for_role(&unlock.sub_field, layout) else {
                continue;
            };

            let event_kind = cpu_reg.push(ThresholdSemantic::CapabilityUnlock {
                sim_thing_id: unlock.sim_thing_id,
                property_id:  unlock.property_id,
                sub_field:    unlock.sub_field.clone(),
            });
            gpu_regs.push(ThresholdRegistration {
                slot,
                col:       col as u32,
                threshold: unlock.threshold,
                direction: DIR_UPWARD,
                event_kind,
                buffer:    THRESH_BUF_VALUES,
            });
        }
    }

    /// Append registrations for a subtree to the caller's existing `gpu_regs`
    /// and `cpu_reg` instead of building fresh ones. Used by B2 Approach B
    /// (append-only threshold rebuild on pure-fission growth boundaries):
    /// the boundary already holds the existing threshold registry, so we only
    /// need to derive registrations for the newly-spawned SimThings. The
    /// event_kinds assigned to the new entries are `cpu_reg.len()` and onwards.
    pub fn append_subtree(
        node: &SimThing,
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        Self::walk(node, dim_reg, allocator, gpu_regs, cpu_reg);
    }

    /// Append the FusionTrigger registrations for the given lineage records
    /// to the caller's existing `gpu_regs` and `cpu_reg`. Companion to
    /// `append_subtree` for B2 Approach B.
    pub fn append_lineage(
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        lineage: &[FissionLineageRecord],
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        Self::push_fusion_lineage(dim_reg, allocator, lineage, gpu_regs, cpu_reg);
    }

    /// Append capability unlock registrations to the caller's existing
    /// `gpu_regs` and `cpu_reg`. Companion to `append_subtree` for B2
    /// append-only threshold rebuild on growth boundaries.
    pub fn append_capability_unlocks(
        dim_reg:            &DimensionRegistry,
        allocator:          &SlotAllocator,
        capability_unlocks: &[CapabilityUnlockRegistration],
        gpu_regs:           &mut Vec<ThresholdRegistration>,
        cpu_reg:            &mut ThresholdRegistry,
    ) {
        Self::push_capability_unlocks(
            dim_reg,
            allocator,
            capability_unlocks,
            gpu_regs,
            cpu_reg,
        );
    }

    /// Append scripted-event trigger registrations to the caller's existing
    /// `gpu_regs` and `cpu_reg`. Companion to `append_subtree` for B2
    /// append-only threshold rebuild on growth boundaries.
    pub fn append_scripted_event_triggers(
        scripted_event_triggers: &[ScriptedEventTriggerRegistration],
        gpu_regs:                &mut Vec<ThresholdRegistration>,
        cpu_reg:                 &mut ThresholdRegistry,
    ) {
        Self::push_scripted_event_triggers(
            scripted_event_triggers,
            gpu_regs,
            cpu_reg,
        );
    }

    fn push_fusion_lineage(
        dim_reg:   &DimensionRegistry,
        allocator: &SlotAllocator,
        lineage:   &[FissionLineageRecord],
        gpu_regs:  &mut Vec<ThresholdRegistration>,
        cpu_reg:   &mut ThresholdRegistry,
    ) {
        for record in lineage {
            if !dim_reg.is_active(record.property_id) { continue; }
            let prop = dim_reg.property(record.property_id);
            if record.template_idx >= prop.fission_templates.len() { continue; }
            let ft = &prop.fission_templates[record.template_idx];

            let Some(child_slot) = allocator.slot_of(record.child_id) else { continue };
            let range  = dim_reg.column_range(record.property_id);
            let layout = &prop.layout;
            let Some(col) = range.col_for_role(&SubFieldRole::Intensity, layout) else {
                continue;
            };

            let event_kind = cpu_reg.push(ThresholdSemantic::FusionTrigger {
                child_id:     record.child_id,
                parent_id:    record.parent_id,
                property_id:  record.property_id,
                template_idx: record.template_idx,
            });
            gpu_regs.push(ThresholdRegistration {
                slot:      child_slot,
                col:       col as u32,
                threshold: ft.template.fusion_intensity_threshold,
                direction: DIR_UPWARD,
                event_kind,
                buffer:    THRESH_BUF_VALUES,
            });
        }
    }

    fn walk(
        node: &SimThing,
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        if let Some(slot) = allocator.slot_of(node.id) {
            for (pid, _pval) in &node.properties {
                if !dim_reg.is_active(*pid) {
                    continue;
                }
                let prop = dim_reg.property(*pid);
                let range = dim_reg.column_range(*pid);
                let layout = &prop.layout;

                // 1. Fission thresholds from FissionThreshold list.
                for (idx, ft) in prop.fission_templates.iter().enumerate() {
                    if let Some(col) = range.col_for_role(&ft.sub_field, layout) {
                        let event_kind = cpu_reg.push(ThresholdSemantic::FissionTrigger {
                            sim_thing_id: node.id,
                            property_id: *pid,
                            template_idx: idx,
                        });
                        gpu_regs.push(ThresholdRegistration {
                            slot: slot,
                            col: col as u32,
                            threshold: ft.threshold,
                            direction: direction_to_u32(&ft.direction),
                            event_kind,
                            buffer: THRESH_BUF_VALUES,
                        });
                    }
                }

                // 2. Decay thresholds (OnThreshold, IntensityGated, WhenProperty).
                //    These emit PropertyExpiry.
                Self::push_decay_thresholds(
                    node.id,
                    *pid,
                    slot,
                    prop.decay.as_ref(),
                    range.start,
                    layout,
                    dim_reg,
                    gpu_regs,
                    cpu_reg,
                );
            }
        }

        for child in &node.children {
            Self::walk(child, dim_reg, allocator, gpu_regs, cpu_reg);
        }
    }

    fn push_decay_thresholds(
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
        slot: u32,
        decay: Option<&DecayBehavior>,
        prop_col_start: usize,
        layout: &simthing_core::PropertyLayout,
        dim_reg: &DimensionRegistry,
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        let semantic = ThresholdSemantic::PropertyExpiry {
            sim_thing_id,
            property_id,
        };
        match decay {
            Some(DecayBehavior::OnThreshold {
                threshold,
                direction,
            }) => {
                // Amount col for this property.
                if let Some(col) = layout.offset_of(&SubFieldRole::Amount) {
                    let event_kind = cpu_reg.push(semantic);
                    gpu_regs.push(ThresholdRegistration {
                        slot,
                        col: (prop_col_start + col) as u32,
                        threshold: *threshold,
                        direction: direction_to_u32(direction),
                        event_kind,
                        buffer: THRESH_BUF_VALUES,
                    });
                }
            }
            Some(DecayBehavior::IntensityGated { intensity_floor }) => {
                if let Some(col) = layout.offset_of(&SubFieldRole::Intensity) {
                    let event_kind = cpu_reg.push(semantic);
                    gpu_regs.push(ThresholdRegistration {
                        slot,
                        col: (prop_col_start + col) as u32,
                        threshold: *intensity_floor,
                        direction: DIR_DOWNWARD,
                        event_kind,
                        buffer: THRESH_BUF_VALUES,
                    });
                }
            }
            Some(DecayBehavior::WhenProperty { other, threshold }) => {
                // Threshold is on `other`'s Amount column.
                if !dim_reg.is_active(*other) {
                    return;
                }
                let other_range = dim_reg.column_range(*other);
                let other_layout = &dim_reg.property(*other).layout;
                if let Some(col) = other_range.col_for_role(&SubFieldRole::Amount, other_layout) {
                    let event_kind = cpu_reg.push(semantic);
                    gpu_regs.push(ThresholdRegistration {
                        slot,
                        col: col as u32,
                        threshold: *threshold,
                        direction: DIR_EITHER,
                        event_kind,
                        buffer: THRESH_BUF_VALUES,
                    });
                }
            }
            // TowardZero and AfterTicks are handled by the overlay lifecycle
            // manager and the property expiry step on the CPU, not via GPU
            // thresholds. No GPU registration needed for those.
            _ => {}
        }
    }

    fn push_velocity_alerts(
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        velocity_alerts: &[VelocityAlertRegistration],
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        for alert in velocity_alerts {
            if !dim_reg.is_active(alert.property_id) {
                continue;
            }
            let Some(slot) = allocator.slot_of(alert.sim_thing_id) else {
                continue;
            };
            let range = dim_reg.column_range(alert.property_id);
            let layout = &dim_reg.property(alert.property_id).layout;
            let Some(col) = range.col_for_role(&alert.sub_field, layout) else {
                continue;
            };

            let event_kind = cpu_reg.push(ThresholdSemantic::VelocityAlert {
                sim_thing_id: alert.sim_thing_id,
                property_id: alert.property_id,
                sub_field: alert.sub_field.clone(),
            });
            gpu_regs.push(ThresholdRegistration {
                slot,
                col: col as u32,
                threshold: alert.threshold,
                direction: direction_to_u32(&alert.direction),
                event_kind,
                buffer: THRESH_BUF_VALUES,
            });
        }
    }

    fn push_aggregate_alerts(
        dim_reg: &DimensionRegistry,
        allocator: &SlotAllocator,
        aggregate_alerts: &[AggregateAlertRegistration],
        gpu_regs: &mut Vec<ThresholdRegistration>,
        cpu_reg: &mut ThresholdRegistry,
    ) {
        for alert in aggregate_alerts {
            if !dim_reg.is_active(alert.property_id) {
                continue;
            }
            let Some(slot) = allocator.slot_of(alert.sim_thing_id) else {
                continue;
            };
            let range = dim_reg.column_range(alert.property_id);
            let layout = &dim_reg.property(alert.property_id).layout;
            let Some(col) = range.col_for_role(&alert.sub_field, layout) else {
                continue;
            };

            let event_kind = cpu_reg.push(ThresholdSemantic::AggregateAlert {
                sim_thing_id: alert.sim_thing_id,
                property_id: alert.property_id,
                sub_field: alert.sub_field.clone(),
            });
            gpu_regs.push(ThresholdRegistration {
                slot,
                col: col as u32,
                threshold: alert.threshold,
                direction: direction_to_u32(&alert.direction),
                event_kind,
                buffer: THRESH_BUF_OUTPUT,
            });
        }
    }
}

fn direction_to_u32(dir: &Direction) -> u32 {
    match dir {
        Direction::Rising => DIR_UPWARD,
        Direction::Falling => DIR_DOWNWARD,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{DimensionRegistry, SimProperty, SimThing, SimThingKind};
    use simthing_gpu::SlotAllocator;

    #[test]
    fn empty_tree_produces_no_registrations() {
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let root = SimThing::new(SimThingKind::World, 0);
        let allocator = SlotAllocator::new();
        let (gpu, cpu) = ThresholdBuilder::build(&root, &reg, &allocator);
        assert!(gpu.is_empty());
        assert!(cpu.is_empty());
    }

    #[test]
    fn event_kind_registry_indices_are_sequential() {
        let mut r = ThresholdRegistry::new();
        let id = simthing_core::SimThing::new(simthing_core::SimThingKind::Cohort, 0).id;
        let ek0 = r.push(ThresholdSemantic::PropertyExpiry {
            sim_thing_id: id,
            property_id: SimPropertyId(0),
        });
        let ek1 = r.push(ThresholdSemantic::PropertyExpiry {
            sim_thing_id: id,
            property_id: SimPropertyId(1),
        });
        assert_eq!(ek0, 0);
        assert_eq!(ek1, 1);
        assert!(r.get(0).is_some());
        assert!(r.get(2).is_none());
    }

    #[test]
    fn fusion_lineage_emits_one_intensity_threshold_per_record() {
        use crate::fission::FissionLineageRecord;
        use simthing_core::{
            Direction as Dir, FissionTemplate, FissionThreshold, SimThingKindTag,
        };

        let mut reg = DimensionRegistry::new();
        let mut prop = SimProperty::simple("core", "loyalty", 0);
        prop.fission_templates = vec![FissionThreshold {
            sub_field: SubFieldRole::Amount,
            threshold: 0.3,
            direction: Dir::Falling,
            template: FissionTemplate {
                child_kind: SimThingKindTag::Cohort,
                fusion_intensity_threshold: 0.85,
                fusion_scar_coefficient:    0.10,
                resolution_label:           "settled".into(),
                clone_capability_children:  false,
                capability_container_kinds: Vec::new(),
            },
            secondary: None,
        }];
        let pid = reg.register(prop);

        // Parent + child both in the tree, both slot-allocated.
        let mut parent = SimThing::new(SimThingKind::Cohort, 0);
        parent.add_property(pid, reg.property(pid).default_value());
        let parent_id = parent.id;
        let mut child = SimThing::new(SimThingKind::Cohort, 1);
        child.add_property(pid, reg.property(pid).default_value());
        let child_id = child.id;
        parent.add_child(child);

        let mut alloc = SlotAllocator::new();
        let parent_slot = alloc.alloc(parent_id);
        let child_slot  = alloc.alloc(child_id);

        let lineage = vec![FissionLineageRecord {
            parent_id, child_id, property_id: pid, template_idx: 0,
        }];

        let (gpu, cpu) = ThresholdBuilder::build_with_lineage(
            &parent, &reg, &alloc, &[], &[], &lineage,
        );

        // Parent + child each contribute one FissionTrigger registration
        // (from `walk`) plus the one FusionTrigger registration we asked for.
        let fusion_regs: Vec<_> = gpu
            .iter()
            .filter(|r| matches!(
                cpu.get(r.event_kind),
                Some(ThresholdSemantic::FusionTrigger { .. })
            ))
            .collect();
        assert_eq!(fusion_regs.len(), 1);

        // The fusion registration watches the child's Intensity (col 2), rising.
        assert_eq!(fusion_regs[0].slot, child_slot);
        assert_eq!(fusion_regs[0].col, 2);
        assert_eq!(fusion_regs[0].threshold, 0.85);
        assert_eq!(fusion_regs[0].direction, DIR_UPWARD);

        // Sanity: parent slot didn't get a fusion registration.
        assert!(fusion_regs.iter().all(|r| r.slot != parent_slot));
    }

    #[test]
    fn fusion_lineage_skipped_when_child_has_no_slot() {
        use crate::fission::FissionLineageRecord;
        use simthing_core::{
            Direction as Dir, FissionTemplate, FissionThreshold, SimThingKindTag,
        };

        let mut reg = DimensionRegistry::new();
        let mut prop = SimProperty::simple("core", "loyalty", 0);
        prop.fission_templates = vec![FissionThreshold {
            sub_field: SubFieldRole::Amount,
            threshold: 0.3,
            direction: Dir::Falling,
            template: FissionTemplate {
                child_kind: SimThingKindTag::Cohort,
                fusion_intensity_threshold: 0.85,
                fusion_scar_coefficient:    0.10,
                resolution_label:           "settled".into(),
                clone_capability_children:  false,
                capability_container_kinds: Vec::new(),
            },
            secondary: None,
        }];
        let pid = reg.register(prop);

        // Allocate parent but tombstone the child (simulates Remove or fusion).
        let parent_id = SimThing::new(SimThingKind::Cohort, 0).id;
        let child_id  = SimThing::new(SimThingKind::Cohort, 1).id;
        let mut alloc = SlotAllocator::new();
        alloc.alloc(parent_id);
        alloc.alloc(child_id);
        alloc.tombstone(child_id);

        let root = SimThing::new(SimThingKind::World, 0);
        let lineage = vec![FissionLineageRecord {
            parent_id, child_id, property_id: pid, template_idx: 0,
        }];

        let (gpu, cpu) = ThresholdBuilder::build_with_lineage(
            &root, &reg, &alloc, &[], &[], &lineage,
        );

        assert!(gpu.iter().all(|r| !matches!(
            cpu.get(r.event_kind),
            Some(ThresholdSemantic::FusionTrigger { .. })
        )));
    }

    // ── PR 4: capability unlock ───────────────────────────────────────────

    fn cap_property() -> SimProperty {
        // One Named sub-field at offset 0. Mirrors what
        // `CapabilityTreeBuilder::build` emits for a one-entry category.
        use simthing_core::{ClampBehavior, PropertyLayout, ReductionRule, SubFieldSpec};
        SimProperty {
            namespace:          "tech".into(),
            name:               "propulsion".into(),
            layout:             PropertyLayout {
                sub_fields: vec![SubFieldSpec {
                    role:               SubFieldRole::Named("chemical_drive".into()),
                    width:              1,
                    clamp:              ClampBehavior::Floored { min: 0.0 },
                    velocity_max:       None,
                    default:            0.0,
                    display_name:       "Chemical Drive".into(),
                    display_range:      None,
                    governed_by:        None,
                    reduction_override: Some(ReductionRule::Max),
                }],
            },
            decay:              None,
            intensity_behavior: None,
            fission_templates:  vec![],
            fusion_templates:   vec![],
            on_expire:          None,
            description:        String::new(),
            intensity_labels:   vec![],
        }
    }

    #[test]
    fn threshold_builder_with_capability_unlocks_emits_correct_event_kind() {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(cap_property());

        let mut tree = SimThing::new(SimThingKind::Custom("tech_tree".into()), 0);
        tree.add_property(pid, reg.property(pid).default_value());
        let tree_id = tree.id;

        let mut alloc = SlotAllocator::new();
        alloc.alloc(tree_id);

        let unlocks = vec![CapabilityUnlockRegistration {
            sim_thing_id: tree_id,
            property_id:  pid,
            sub_field:    SubFieldRole::Named("chemical_drive".into()),
            threshold:    5000.0,
        }];

        let (gpu, cpu) = ThresholdBuilder::build_with_capability_unlocks(
            &tree, &reg, &alloc, &[], &unlocks,
        );

        // One registration, one parallel semantic entry at the same event_kind.
        assert_eq!(gpu.len(), 1);
        assert_eq!(cpu.len(), 1);
        let event_kind = gpu[0].event_kind;
        match cpu.get(event_kind) {
            Some(ThresholdSemantic::CapabilityUnlock { sim_thing_id, property_id, sub_field }) => {
                assert_eq!(*sim_thing_id, tree_id);
                assert_eq!(*property_id, pid);
                assert_eq!(*sub_field, SubFieldRole::Named("chemical_drive".into()));
            }
            other => panic!("expected CapabilityUnlock at event_kind {event_kind}, got {other:?}"),
        }
    }

    #[test]
    fn threshold_builder_capability_unlock_resolves_slot_and_col() {
        let mut reg = DimensionRegistry::new();
        // Seed another property first so propulsion is not at column 0 — proves
        // col resolution actually does the arithmetic.
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let pid = reg.register(cap_property());

        let mut tree = SimThing::new(SimThingKind::Custom("tech_tree".into()), 0);
        tree.add_property(pid, reg.property(pid).default_value());
        let tree_id = tree.id;

        let mut alloc = SlotAllocator::new();
        // Force tree onto slot 7 (not 0) — proves slot resolution.
        for _ in 0..7 {
            alloc.alloc(simthing_core::SimThingId::new());
        }
        let slot = alloc.alloc(tree_id);
        assert_eq!(slot, 7);

        let unlocks = vec![CapabilityUnlockRegistration {
            sim_thing_id: tree_id,
            property_id:  pid,
            sub_field:    SubFieldRole::Named("chemical_drive".into()),
            threshold:    5000.0,
        }];

        let (gpu, _cpu) = ThresholdBuilder::build_with_capability_unlocks(
            &tree, &reg, &alloc, &[], &unlocks,
        );

        // Expected col = start of propulsion (after 3-column loyalty) + 0 (first sub-field).
        let propulsion_range = reg.column_range(pid);
        assert_eq!(propulsion_range.start, 3);
        assert_eq!(gpu[0].slot, 7);
        assert_eq!(gpu[0].col, propulsion_range.start as u32);
        assert_eq!(gpu[0].threshold, 5000.0);
        assert_eq!(gpu[0].direction, DIR_UPWARD);
        assert_eq!(gpu[0].buffer, THRESH_BUF_VALUES);
    }

    #[test]
    fn threshold_builder_capability_unlock_skips_unallocated_simthing() {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(cap_property());

        let tree = SimThing::new(SimThingKind::Custom("tech_tree".into()), 0);
        // Note: tree has NO slot allocated.

        let unlocks = vec![CapabilityUnlockRegistration {
            sim_thing_id: tree.id,
            property_id:  pid,
            sub_field:    SubFieldRole::Named("chemical_drive".into()),
            threshold:    5000.0,
        }];

        let (gpu, cpu) = ThresholdBuilder::build_with_capability_unlocks(
            &tree, &reg, &SlotAllocator::new(), &[], &unlocks,
        );

        // Silently skipped — matches velocity-alert and lineage behavior.
        assert!(gpu.is_empty());
        assert!(cpu.is_empty());
    }

    #[test]
    fn threshold_semantic_capability_unlock_round_trips_serde() {
        let original = ThresholdSemantic::CapabilityUnlock {
            sim_thing_id: simthing_core::SimThingId::new(),
            property_id:  SimPropertyId(7),
            sub_field:    SubFieldRole::Named("warp_drive".into()),
        };

        let json: String = serde_json::to_string(&original).expect("serialize");
        let restored: ThresholdSemantic = serde_json::from_str(&json).expect("deserialize");

        match (&original, &restored) {
            (
                ThresholdSemantic::CapabilityUnlock { sim_thing_id: a1, property_id: a2, sub_field: a3 },
                ThresholdSemantic::CapabilityUnlock { sim_thing_id: b1, property_id: b2, sub_field: b3 },
            ) => {
                assert_eq!(a1, b1);
                assert_eq!(a2, b2);
                assert_eq!(a3, b3);
            }
            _ => panic!("variant changed across round-trip: {restored:?}"),
        }
    }

    #[test]
    fn extract_capability_unlocks_resolves_threshold_events_to_unlock_events() {
        let id1 = simthing_core::SimThingId::new();
        let id2 = simthing_core::SimThingId::new();

        let mut cpu = ThresholdRegistry::new();
        let ek_unlock = cpu.push(ThresholdSemantic::CapabilityUnlock {
            sim_thing_id: id1,
            property_id:  SimPropertyId(5),
            sub_field:    SubFieldRole::Named("warp_drive".into()),
        });
        let ek_velocity = cpu.push(ThresholdSemantic::VelocityAlert {
            sim_thing_id: id2,
            property_id:  SimPropertyId(7),
            sub_field:    SubFieldRole::Velocity,
        });

        let events = vec![
            ThresholdEvent { slot: 0, col: 0, value: 1.0, event_kind: ek_unlock },
            ThresholdEvent { slot: 1, col: 1, value: 2.0, event_kind: ek_velocity },
            // Out-of-range event_kind: should be filtered.
            ThresholdEvent { slot: 2, col: 2, value: 3.0, event_kind: 99 },
        ];

        let unlocks = cpu.extract_capability_unlocks(&events);

        // Only the CapabilityUnlock arm produces an output; velocity + out-of-range
        // are silently dropped.
        assert_eq!(unlocks.len(), 1);
        assert_eq!(unlocks[0].sim_thing_id, id1);
        assert_eq!(unlocks[0].property_id, SimPropertyId(5));
        assert_eq!(unlocks[0].sub_field, SubFieldRole::Named("warp_drive".into()));
    }

    #[test]
    fn velocity_alert_registration_targets_requested_sub_field() {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(SimProperty::simple("core", "loyalty", 0));

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(pid, reg.property(pid).default_value());
        let cohort_id = cohort.id;

        let mut alloc = SlotAllocator::new();
        alloc.alloc(cohort_id);

        let alerts = vec![VelocityAlertRegistration {
            sim_thing_id: cohort_id,
            property_id: pid,
            sub_field: SubFieldRole::Velocity,
            threshold: -0.05,
            direction: Direction::Falling,
        }];

        let (gpu, cpu) =
            ThresholdBuilder::build_with_velocity_alerts(&cohort, &reg, &alloc, &alerts);

        assert_eq!(gpu.len(), 1);
        assert_eq!(gpu[0].slot, 0);
        assert_eq!(gpu[0].col, 1);
        assert_eq!(gpu[0].threshold, -0.05);
        assert_eq!(cpu.len(), 1);
        assert!(
            matches!(cpu.get(gpu[0].event_kind), Some(ThresholdSemantic::VelocityAlert {
            sim_thing_id,
            property_id,
            sub_field: SubFieldRole::Velocity,
        }) if *sim_thing_id == cohort_id && *property_id == pid)
        );
    }

    #[test]
    fn append_capability_unlocks_preserves_existing_event_kinds() {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(cap_property());

        let mut tree = SimThing::new(SimThingKind::Custom("tech_tree".into()), 0);
        tree.add_property(pid, reg.property(pid).default_value());
        let tree_id = tree.id;

        let mut alloc = SlotAllocator::new();
        alloc.alloc(tree_id);

        let seed_id = simthing_core::SimThingId::new();
        let mut gpu_regs = Vec::new();
        let mut cpu_reg = ThresholdRegistry::new();
        let seed_kind = cpu_reg.push(ThresholdSemantic::VelocityAlert {
            sim_thing_id: seed_id,
            property_id:  SimPropertyId(1),
            sub_field:    SubFieldRole::Velocity,
        });
        gpu_regs.push(ThresholdRegistration {
            slot:      0,
            col:       0,
            threshold: 1.0,
            direction: DIR_UPWARD,
            event_kind: seed_kind,
            buffer:    THRESH_BUF_VALUES,
        });

        let unlocks = vec![CapabilityUnlockRegistration {
            sim_thing_id: tree_id,
            property_id:  pid,
            sub_field:    SubFieldRole::Named("chemical_drive".into()),
            threshold:    5000.0,
        }];

        ThresholdBuilder::append_capability_unlocks(
            &reg, &alloc, &unlocks, &mut gpu_regs, &mut cpu_reg,
        );

        assert_eq!(gpu_regs.len(), 2);
        assert_eq!(cpu_reg.len(), 2);
        assert_eq!(gpu_regs[0].event_kind, seed_kind);
        assert_eq!(gpu_regs[1].event_kind, 1);
        assert!(matches!(
            cpu_reg.get(1),
            Some(ThresholdSemantic::CapabilityUnlock { .. })
        ));
    }

    #[test]
    fn append_scripted_event_triggers_preserves_existing_event_kinds() {
        let seed_id = simthing_core::SimThingId::new();
        let mut gpu_regs = Vec::new();
        let mut cpu_reg = ThresholdRegistry::new();
        let seed_kind = cpu_reg.push(ThresholdSemantic::VelocityAlert {
            sim_thing_id: seed_id,
            property_id:  SimPropertyId(1),
            sub_field:    SubFieldRole::Velocity,
        });
        gpu_regs.push(ThresholdRegistration {
            slot:      0,
            col:       0,
            threshold: 1.0,
            direction: DIR_UPWARD,
            event_kind: seed_kind,
            buffer:    THRESH_BUF_VALUES,
        });

        let triggers = vec![
            ScriptedEventTriggerRegistration {
                event_id:  "low_loyalty".into(),
                slot:      3,
                col:       2,
                threshold: 0.25,
                direction: Direction::Falling,
            },
            ScriptedEventTriggerRegistration {
                event_id:  "faction_collapse".into(),
                slot:      4,
                col:       1,
                threshold: 0.1,
                direction: Direction::Rising,
            },
        ];

        ThresholdBuilder::append_scripted_event_triggers(
            &triggers, &mut gpu_regs, &mut cpu_reg,
        );

        assert_eq!(gpu_regs.len(), 3);
        assert_eq!(cpu_reg.len(), 3);
        assert_eq!(gpu_regs[0].event_kind, seed_kind);
        assert_eq!(gpu_regs[1].event_kind, 1);
        assert_eq!(gpu_regs[2].event_kind, 2);
        assert!(matches!(
            cpu_reg.get(1),
            Some(ThresholdSemantic::ScriptedEventTrigger { event_id }) if event_id == "low_loyalty"
        ));
        assert!(matches!(
            cpu_reg.get(2),
            Some(ThresholdSemantic::ScriptedEventTrigger { event_id }) if event_id == "faction_collapse"
        ));
    }
}
