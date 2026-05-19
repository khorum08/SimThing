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
//! 2. **`FusionThreshold`** on a spawned child — registered at fission time.
//!    One per live spawned child SimThing.
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

use simthing_core::{
    DecayBehavior, DimensionRegistry, Direction, SimPropertyId, SimThing, SimThingId, SubFieldRole,
};
use simthing_gpu::{SlotAllocator, ThresholdRegistration, DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD};

// ── Semantic action ───────────────────────────────────────────────────────────

/// What a fired `ThresholdEvent` means to the CPU boundary protocol.
/// Indexed by `ThresholdEvent::event_kind` in the `ThresholdRegistry`.
#[derive(Clone, Debug)]
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

    /// Velocity alert (AI-registered). Surfaced by `BoundaryOutcome`.
    VelocityAlert {
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
        sub_field: SubFieldRole,
    },
}

/// AI-facing threshold registration for a rate/trajectory column.
#[derive(Clone, Debug)]
pub struct VelocityAlertRegistration {
    pub sim_thing_id: SimThingId,
    pub property_id: SimPropertyId,
    pub sub_field: SubFieldRole,
    pub threshold: f32,
    pub direction: Direction,
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
        let mut gpu_regs = Vec::new();
        let mut cpu_reg = ThresholdRegistry::new();
        Self::walk(root, dim_reg, allocator, &mut gpu_regs, &mut cpu_reg);
        Self::push_velocity_alerts(
            dim_reg,
            allocator,
            velocity_alerts,
            &mut gpu_regs,
            &mut cpu_reg,
        );
        (gpu_regs, cpu_reg)
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
                            _pad: 0,
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
                        _pad: 0,
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
                        _pad: 0,
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
                        _pad: 0,
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
                _pad: 0,
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
}
