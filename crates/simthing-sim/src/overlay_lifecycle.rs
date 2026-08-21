//! Overlay lifecycle management — step 4 and step 7 of the day boundary.
//!
//! ## Step 4: GPU decision consume + structural writeback
//!
//! Production maps property and owning-generation conditions onto existing
//! Phase-5 registrations. The GPU owns the conjunctive state and deadline
//! decision; [`resolve_overlay_lifecycle`] retains the legacy decrementing
//! behavior only as an oracle. `OverrideReceived` is rejected at admission.
//!
//! Each `Overlay` carries an `OverlayLifecycle`. At the boundary:
//! - `UntilDissolved` overlays are removed only by explicit removal (no automatic
//!   condition on the unit form).
//! - `UntilDissolvedWith { dissolution_conditions }` and
//!   `Transient { dissolution_conditions }` overlays are removed when *all*
//!   conditions are met. Conditions are AND-ed (all must be satisfied).
//!
//! `DissolveCondition` variants:
//! - `PropertyReaches { property, sub_field, value }` — true when the
//!   SimThing's property sub-field is ≥ the target value (for Rising direction).
//! - `PropertyBelow { property, sub_field, value }` — true when < value.
//! - `AfterTicks { remaining }` — true when remaining reaches 0. The
//!   lifecycle manager decrements `remaining` by 1 each boundary.
//! - `OverrideReceived` — true when a new instruction overlay replaces this
//!   one. Checked by the `AttachOverlay` handler in step 7.
//! - `Never` — always false; the overlay persists until explicitly removed.
//!
//! When an overlay dissolves, its `on_expire` `ExpireEffect`s are applied
//! to the CPU shadow. These are small velocity bumps or intensity sets that
//! model "what happens when this policy ends."
//!
//! ## Step 7: attach new overlays
//!
//! `BoundaryRequest::AttachOverlay` items are applied here. Each carries a
//! new `Overlay` and a target `SimThingId`. Attachment is the only within-day
//! structural change that doesn't require a slot mutation.
//!
//! After step 7 the overlay list for every SimThing is authoritative for day
//! N+1. `gpu_sync` then refreshes the derived overlay span projection to reflect those lists.

use simthing_core::{
    admit_overlay_lifecycle, establish_overlay_deadline, DimensionRegistry, DissolveCondition,
    GenerationStamp, OverlayId, OverlayLifecycle, OverlayLifecycleAdmitError, SimThing, SimThingId,
    SubFieldRole,
};
use simthing_gpu::{
    OverlayLifecycleProjectionBinding, OverlayLifecycleProjectionPlan,
    OverlayLifecycleProjectionSeed, OverlayLifecycleStateGpu, SlotAllocator, ThresholdRegistration,
    DIR_DOWNWARD, DIR_UPWARD, THRESH_BUF_OWNING_GENERATION, THRESH_BUF_VALUES,
};
use std::collections::HashMap;

use crate::threshold_registry::{ThresholdRegistry, ThresholdSemantic};
use crate::tree_index::{node_at_path_mut, paths_preorder};

/// Counts from one boundary's lifecycle pass.
#[derive(Clone, Debug, Default)]
pub struct LifecycleOutcome {
    pub dissolved: u32,
    pub dissolved_overlays: Vec<(SimThingId, OverlayId)>,
    pub after_ticks_decremented: u32,
    pub overlays_attached: u32,
}

/// Logical identity sidecar for compact GPU rows. Physical row numbers are
/// rebuilt at each admitted sync and never become durable identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OverlayLifecycleTarget {
    pub sim_thing_id: SimThingId,
    pub overlay_id: OverlayId,
}

#[derive(Clone, Debug, Default)]
pub struct OverlayLifecycleAdmissionState {
    activation_generations: HashMap<(SimThingId, OverlayId), GenerationStamp>,
    routed_provenance: HashMap<(SimThingId, OverlayId), GenerationStamp>,
    satisfied_masks: HashMap<(SimThingId, OverlayId), u32>,
    after_ticks_remaining: HashMap<(SimThingId, OverlayId, u32), u32>,
}

impl OverlayLifecycleAdmissionState {
    pub(crate) fn admit_routed_overlay(
        &mut self,
        target: SimThingId,
        overlay_id: OverlayId,
        lifecycle: &OverlayLifecycle,
        source_generation: GenerationStamp,
        destination_generation: GenerationStamp,
    ) -> Result<(), OverlayLifecycleAdmitError> {
        admit_overlay_lifecycle(lifecycle)?;
        let (active_lifecycle, suspended) = match lifecycle {
            OverlayLifecycle::Suspended { when_activated } => (when_activated.as_ref(), true),
            active => (active, false),
        };
        if !suspended {
            validate_authored_deadlines(active_lifecycle, destination_generation)?;
        }

        let key = (target, overlay_id);
        self.activation_generations.remove(&key);
        self.satisfied_masks.remove(&key);
        self.after_ticks_remaining
            .retain(|(sim_thing_id, resident_overlay_id, _), _| {
                (*sim_thing_id, *resident_overlay_id) != key
            });
        self.seed_after_ticks_remaining(key, active_lifecycle);
        if !suspended {
            self.activation_generations
                .insert(key, destination_generation);
        }
        self.routed_provenance.insert(key, source_generation);
        Ok(())
    }

    pub(crate) fn activate_overlay(
        &mut self,
        target: SimThingId,
        overlay_id: OverlayId,
        lifecycle: &OverlayLifecycle,
        destination_generation: GenerationStamp,
    ) -> Result<(), OverlayLifecycleAdmitError> {
        admit_overlay_lifecycle(lifecycle)?;
        self.establish_activation((target, overlay_id), lifecycle, destination_generation)
    }

    pub(crate) fn suspend_overlay(
        &mut self,
        target: SimThingId,
        overlay_id: OverlayId,
        lifecycle: &OverlayLifecycle,
        destination_generation: GenerationStamp,
    ) {
        let key = (target, overlay_id);
        let Some(activation) = self.activation_generations.remove(&key) else {
            return;
        };
        let elapsed = destination_generation
            .get()
            .saturating_sub(activation.get());
        if let Some(conditions) = lifecycle_conditions(lifecycle) {
            for (condition_index, condition) in conditions.iter().enumerate() {
                if let DissolveCondition::AfterTicks { remaining } = condition {
                    let remaining_at_activation = self
                        .after_ticks_remaining
                        .get(&(target, overlay_id, condition_index as u32))
                        .copied()
                        .unwrap_or(*remaining);
                    self.after_ticks_remaining.insert(
                        (target, overlay_id, condition_index as u32),
                        remaining_at_activation.saturating_sub(elapsed),
                    );
                }
            }
        }
    }

    fn seed_after_ticks_remaining(
        &mut self,
        key: (SimThingId, OverlayId),
        lifecycle: &OverlayLifecycle,
    ) {
        if let Some(conditions) = lifecycle_conditions(lifecycle) {
            for (condition_index, condition) in conditions.iter().enumerate() {
                if let DissolveCondition::AfterTicks { remaining } = condition {
                    self.after_ticks_remaining
                        .insert((key.0, key.1, condition_index as u32), *remaining);
                }
            }
        }
    }

    fn establish_activation(
        &mut self,
        key: (SimThingId, OverlayId),
        lifecycle: &OverlayLifecycle,
        destination_generation: GenerationStamp,
    ) -> Result<(), OverlayLifecycleAdmitError> {
        if let Some(conditions) = lifecycle_conditions(lifecycle) {
            for (condition_index, condition) in conditions.iter().enumerate() {
                if let DissolveCondition::AfterTicks { remaining } = condition {
                    let remaining = self
                        .after_ticks_remaining
                        .get(&(key.0, key.1, condition_index as u32))
                        .copied()
                        .unwrap_or(*remaining);
                    establish_overlay_deadline(destination_generation, remaining)?;
                }
            }
        }
        self.activation_generations
            .insert(key, destination_generation);
        Ok(())
    }

    fn after_ticks_remaining(
        &self,
        key: (SimThingId, OverlayId),
        condition_index: u32,
        authored_remaining: u32,
    ) -> u32 {
        self.after_ticks_remaining
            .get(&(key.0, key.1, condition_index))
            .copied()
            .unwrap_or(authored_remaining)
    }

    pub fn routed_provenance(
        &self,
        target: SimThingId,
        overlay_id: OverlayId,
    ) -> Option<GenerationStamp> {
        self.routed_provenance.get(&(target, overlay_id)).copied()
    }

    pub fn activation_generation(
        &self,
        target: SimThingId,
        overlay_id: OverlayId,
    ) -> Option<GenerationStamp> {
        self.activation_generations
            .get(&(target, overlay_id))
            .copied()
    }

    pub fn observe_gpu_rows(
        &mut self,
        targets: &[OverlayLifecycleTarget],
        rows: &[OverlayLifecycleStateGpu],
    ) {
        for (target, row) in targets.iter().zip(rows) {
            self.satisfied_masks.insert(
                (target.sim_thing_id, target.overlay_id),
                row.satisfied_mask(),
            );
        }
    }
}

/// Append lifecycle predicates to the canonical Phase-5 registration packet.
/// This builds bindings only: crossing authority remains the kernel's sole
/// `threshold_crossed` call.
pub fn append_overlay_lifecycle_registrations(
    root: &SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    generation: GenerationStamp,
    gpu_regs: &mut Vec<ThresholdRegistration>,
    cpu_reg: &mut ThresholdRegistry,
    admission: &mut OverlayLifecycleAdmissionState,
) -> (OverlayLifecycleProjectionPlan, Vec<OverlayLifecycleTarget>) {
    let mut plan = OverlayLifecycleProjectionPlan::default();
    let mut targets = Vec::new();
    append_node_lifecycle(
        root,
        registry,
        allocator,
        generation,
        gpu_regs,
        cpu_reg,
        admission,
        &mut plan,
        &mut targets,
        false,
    );
    (plan, targets)
}

/// Derive the complete session admission catalogue, including the active
/// lifecycle nested under every suspended overlay. Catalogue derivation uses a
/// cloned semantic shadow: it reserves template/capacity without activating a
/// suspended overlay or minting numerical authority on the CPU.
pub fn derive_overlay_lifecycle_admission_catalog(
    root: &SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    generation: GenerationStamp,
    admission: &OverlayLifecycleAdmissionState,
) -> (OverlayLifecycleProjectionPlan, Vec<ThresholdRegistration>) {
    let mut gpu_regs = Vec::new();
    let mut cpu_reg = ThresholdRegistry::new();
    let mut admission_shadow = admission.clone();
    let mut plan = OverlayLifecycleProjectionPlan::default();
    let mut targets = Vec::new();
    append_node_lifecycle(
        root,
        registry,
        allocator,
        generation,
        &mut gpu_regs,
        &mut cpu_reg,
        &mut admission_shadow,
        &mut plan,
        &mut targets,
        true,
    );
    (plan, gpu_regs)
}

#[allow(clippy::too_many_arguments)]
fn append_node_lifecycle(
    node: &SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    generation: GenerationStamp,
    gpu_regs: &mut Vec<ThresholdRegistration>,
    cpu_reg: &mut ThresholdRegistry,
    admission: &mut OverlayLifecycleAdmissionState,
    plan: &mut OverlayLifecycleProjectionPlan,
    targets: &mut Vec<OverlayLifecycleTarget>,
    include_suspended: bool,
) {
    for overlay in &node.overlays {
        let lifecycle = match &overlay.lifecycle {
            OverlayLifecycle::Suspended { when_activated } if include_suspended => {
                when_activated.as_ref()
            }
            OverlayLifecycle::Suspended { .. } => continue,
            lifecycle => lifecycle,
        };
        admit_overlay_lifecycle(lifecycle).unwrap_or_else(|error| {
            panic!(
                "overlay {:?} lifecycle admission failed: {error}",
                overlay.id
            )
        });
        let conditions = match lifecycle {
            OverlayLifecycle::Transient {
                dissolution_conditions,
            }
            | OverlayLifecycle::UntilDissolvedWith {
                dissolution_conditions,
            } => dissolution_conditions,
            OverlayLifecycle::UntilDissolved => continue,
            OverlayLifecycle::Suspended { .. } => unreachable!("suspended lifecycle was unwrapped"),
        };
        let key = (node.id, overlay.id);
        let activation = *admission
            .activation_generations
            .entry(key)
            .or_insert(generation);
        let row = plan.rows.len() as u32;
        let required_mask = (1u32 << conditions.len()) - 1;
        let satisfied_mask = admission.satisfied_masks.get(&key).copied().unwrap_or(0);
        let slot = allocator
            .slot_of(node.id)
            .unwrap_or_else(|| {
                panic!(
                    "lifecycle overlay owner {:?} lacks an admitted slot",
                    node.id
                )
            })
            .raw();

        for (condition_index, condition) in conditions.iter().enumerate() {
            let bit = condition_index as u32;
            let registration = match condition {
                DissolveCondition::PropertyReaches {
                    property,
                    sub_field,
                    value,
                }
                | DissolveCondition::PropertyBelow {
                    property,
                    sub_field,
                    value,
                } => {
                    let range = registry
                        .try_column_range(*property)
                        .unwrap_or_else(|| panic!("lifecycle property {property:?} is not active"));
                    let property_spec = registry
                        .try_property(*property)
                        .expect("active lifecycle property has a registry entry");
                    let col = range
                        .col_for_role(sub_field, &property_spec.layout)
                        .unwrap_or_else(|| {
                            panic!("lifecycle sub-field {sub_field:?} is not admitted")
                        });
                    Some(ThresholdRegistration {
                        slot,
                        col: col.raw() as u32,
                        threshold: *value,
                        direction: if matches!(condition, DissolveCondition::PropertyReaches { .. })
                        {
                            DIR_UPWARD
                        } else {
                            DIR_DOWNWARD
                        },
                        event_kind: 0,
                        buffer: THRESH_BUF_VALUES,
                    })
                }
                DissolveCondition::AfterTicks { remaining } => {
                    let remaining = admission.after_ticks_remaining(key, bit, *remaining);
                    let deadline = establish_overlay_deadline(activation, remaining)
                        .unwrap_or_else(|error| {
                            panic!("overlay deadline admission failed: {error}")
                        });
                    Some(ThresholdRegistration {
                        slot: 0,
                        col: 0,
                        threshold: deadline.get().saturating_sub(1) as f32,
                        direction: DIR_UPWARD,
                        event_kind: 0,
                        buffer: THRESH_BUF_OWNING_GENERATION,
                    })
                }
                DissolveCondition::AtSessionEnd => None,
                DissolveCondition::OverrideReceived => unreachable!("rejected by admission"),
            };
            if let Some(mut registration) = registration {
                registration.event_kind =
                    cpu_reg.push(ThresholdSemantic::OverlayLifecycleCondition {
                        sim_thing_id: node.id,
                        overlay_id: overlay.id,
                        condition_index: bit,
                    });
                let registration_index = gpu_regs.len() as u32;
                gpu_regs.push(registration);
                plan.bindings.push(OverlayLifecycleProjectionBinding {
                    registration_index,
                    row,
                    condition_bit: bit,
                });
            }
        }
        plan.rows
            .push(OverlayLifecycleProjectionSeed::with_satisfied_mask(
                satisfied_mask,
                required_mask,
            ));
        targets.push(OverlayLifecycleTarget {
            sim_thing_id: node.id,
            overlay_id: overlay.id,
        });
    }
    for child in &node.children {
        append_node_lifecycle(
            child,
            registry,
            allocator,
            generation,
            gpu_regs,
            cpu_reg,
            admission,
            plan,
            targets,
            include_suspended,
        );
    }
}

fn lifecycle_conditions(lifecycle: &OverlayLifecycle) -> Option<&[DissolveCondition]> {
    match lifecycle {
        OverlayLifecycle::Transient {
            dissolution_conditions,
        }
        | OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions,
        } => Some(dissolution_conditions),
        OverlayLifecycle::Suspended { when_activated } => lifecycle_conditions(when_activated),
        OverlayLifecycle::UntilDissolved => None,
    }
}

fn validate_authored_deadlines(
    lifecycle: &OverlayLifecycle,
    destination_generation: GenerationStamp,
) -> Result<(), OverlayLifecycleAdmitError> {
    if let Some(conditions) = lifecycle_conditions(lifecycle) {
        for condition in conditions {
            if let DissolveCondition::AfterTicks { remaining } = condition {
                establish_overlay_deadline(destination_generation, *remaining)?;
            }
        }
    }
    Ok(())
}

/// CPU oracle only. Walk the tree and:
/// 1. Decrement AfterTicks counters on all transient overlays.
/// 2. Remove overlays whose dissolution conditions are all met.
/// 3. Apply `on_expire` effects from dissolved overlays to the CPU shadow.
///
/// `values_shadow` is the `DispatchCoordinator::shadow` slice; it is mutated
/// directly for ExpireEffect writes. The dirty-row bitmap in `TransformPatcher`
/// is NOT updated here — callers must call `mark_slot_dirty` or use
/// `upload_full_shadow` at boundary end.
///
/// `day` is the current day counter (used for future AfterDays conditions).
pub fn resolve_overlay_lifecycle(
    root: &mut SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    values_shadow: &mut [f32],
    n_dims: usize,
    _day: u32,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
) -> LifecycleOutcome {
    let mut out = LifecycleOutcome::default();
    if let Some(paths) = node_paths {
        for path in paths_preorder(paths) {
            if let Some(node) = node_at_path_mut(root, &path) {
                process_node(node, registry, allocator, values_shadow, n_dims, &mut out);
            }
        }
    } else {
        resolve_node(root, registry, allocator, values_shadow, n_dims, &mut out);
    }
    out
}

/// Production structural consumer for lifecycle decisions already made on the
/// GPU resident plane. The CPU performs no predicate or countdown evaluation;
/// it maps logical identities to overlay removals and applies authored
/// writeback effects.
pub fn apply_gpu_overlay_lifecycle(
    root: &mut SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    values_shadow: &mut [f32],
    n_dims: usize,
    node_paths: &HashMap<SimThingId, Vec<usize>>,
    targets: &[OverlayLifecycleTarget],
    rows: &[OverlayLifecycleStateGpu],
) -> LifecycleOutcome {
    let mut out = LifecycleOutcome::default();
    for (target, row) in targets.iter().zip(rows) {
        if !row.is_dissolved() {
            continue;
        }
        let Some(path) = node_paths.get(&target.sim_thing_id) else {
            continue;
        };
        let Some(node) = node_at_path_mut(root, path) else {
            continue;
        };
        let Some(index) = node
            .overlays
            .iter()
            .position(|overlay| overlay.id == target.overlay_id)
        else {
            continue;
        };
        let overlay = node.overlays.remove(index);
        out.dissolved += 1;
        out.dissolved_overlays.push((node.id, overlay.id));
        if let Some(slot) = allocator.slot_of(node.id) {
            let base = slot.as_usize() * n_dims;
            let pid = overlay.transform.property_id;
            if let Some(handler) = registry
                .try_property(pid)
                .filter(|_| registry.is_active(pid))
                .and_then(|property| property.on_expire.as_ref())
            {
                apply_expire_effects(handler, registry, values_shadow, base, n_dims);
            }
        }
    }
    out
}

fn resolve_node(
    node: &mut SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    values_shadow: &mut [f32],
    n_dims: usize,
    out: &mut LifecycleOutcome,
) {
    process_node(node, registry, allocator, values_shadow, n_dims, out);
    for child in &mut node.children {
        resolve_node(child, registry, allocator, values_shadow, n_dims, out);
    }
}

fn process_node(
    node: &mut SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    values_shadow: &mut [f32],
    n_dims: usize,
    out: &mut LifecycleOutcome,
) {
    let slot = allocator.slot_of(node.id);
    let base = slot.map(|s| s.as_usize() * n_dims);

    // First pass (immutable): check which overlays should dissolve.
    // We separate condition evaluation (needs immutable `node`) from
    // AfterTicks decrement (needs mutable overlay) to satisfy the borrow
    // checker — `evaluate_condition` borrows `node` immutably, but
    // iterating `node.overlays.iter_mut()` would hold a mutable borrow.
    let mut dissolved_indices = Vec::new();
    {
        let should_dissolve: Vec<bool> = node
            .overlays
            .iter()
            .map(|overlay| match &overlay.lifecycle {
                OverlayLifecycle::Transient {
                    dissolution_conditions,
                }
                | OverlayLifecycle::UntilDissolvedWith {
                    dissolution_conditions,
                } => dissolution_conditions
                    .iter()
                    .all(|cond| evaluate_condition(cond, node, registry, values_shadow, base)),
                _ => false,
            })
            .collect();

        // Second sub-pass (mutable): decrement AfterTicks on surviving overlays.
        for (i, overlay) in node.overlays.iter_mut().enumerate() {
            let conditions = match &mut overlay.lifecycle {
                OverlayLifecycle::Transient {
                    dissolution_conditions,
                }
                | OverlayLifecycle::UntilDissolvedWith {
                    dissolution_conditions,
                } => Some(dissolution_conditions),
                _ => None,
            };
            if let Some(dissolution_conditions) = conditions {
                for cond in dissolution_conditions.iter_mut() {
                    if let DissolveCondition::AfterTicks { remaining } = cond {
                        if *remaining > 0 {
                            *remaining -= 1;
                            out.after_ticks_decremented += 1;
                        }
                    }
                }
            }
            if should_dissolve[i] {
                dissolved_indices.push(i);
            }
        }
    }

    // Second pass (reverse): remove dissolved overlays + apply expire effects.
    for i in dissolved_indices.into_iter().rev() {
        let overlay = node.overlays.remove(i);
        out.dissolved += 1;
        out.dissolved_overlays.push((node.id, overlay.id));

        // Apply on_expire effects to the CPU shadow if we have a slot.
        if let Some(base) = base {
            let pid = overlay.transform.property_id;
            if let Some(prop) = registry
                .try_property(pid)
                .filter(|_| registry.is_active(pid))
            {
                if let Some(handler) = prop.on_expire.as_ref() {
                    apply_expire_effects(handler, registry, values_shadow, base, n_dims);
                }
            }
        }
    }
}

fn evaluate_condition(
    cond: &DissolveCondition,
    node: &SimThing,
    registry: &DimensionRegistry,
    values_shadow: &[f32],
    base: Option<usize>,
) -> bool {
    match cond {
        DissolveCondition::AtSessionEnd => false,
        DissolveCondition::OverrideReceived => false, // handled by attach step
        DissolveCondition::AfterTicks { remaining } => *remaining == 0,
        DissolveCondition::PropertyReaches {
            property,
            sub_field,
            value,
        } => read_sub_field(node, registry, values_shadow, base, *property, sub_field)
            .map(|v| v >= *value)
            .unwrap_or(false),
        DissolveCondition::PropertyBelow {
            property,
            sub_field,
            value,
        } => read_sub_field(node, registry, values_shadow, base, *property, sub_field)
            .map(|v| v < *value)
            .unwrap_or(false),
    }
}

/// Read a sub-field value from the CPU shadow. Prefers shadow over
/// SimThing::properties because the shadow reflects GPU integration output.
fn read_sub_field(
    node: &SimThing,
    registry: &DimensionRegistry,
    shadow: &[f32],
    base: Option<usize>,
    pid: simthing_core::SimPropertyId,
    role: &SubFieldRole,
) -> Option<f32> {
    if !node.properties.contains_key(&pid) {
        return None;
    }
    if !registry.is_active(pid) {
        return None;
    }
    let base = base?;
    let range = registry.try_column_range(pid)?;
    let layout = &registry.try_property(pid)?.layout;
    let col = range.col_for_role(role, layout)?;
    shadow.get(base + col.raw()).copied()
}

fn apply_expire_effects(
    handler: &simthing_core::ExpireHandler,
    registry: &DimensionRegistry,
    shadow: &mut [f32],
    base: usize,
    _n_dims: usize,
) {
    for effect in &handler.write_back {
        match effect {
            simthing_core::ExpireEffect::AddVelocity {
                property,
                sub_field,
                delta,
            } => {
                if !registry.is_active(*property) {
                    continue;
                }
                let Some(range) = registry.try_column_range(*property) else {
                    continue;
                };
                let Some(prop) = registry.try_property(*property) else {
                    continue;
                };
                let layout = &prop.layout;
                if let Some(col) = range.col_for_role(sub_field, layout) {
                    if let Some(v) = shadow.get_mut(base + col.raw()) {
                        *v += delta;
                    }
                }
            }
            simthing_core::ExpireEffect::SetIntensity { property, value } => {
                if !registry.is_active(*property) {
                    continue;
                }
                let Some(range) = registry.try_column_range(*property) else {
                    continue;
                };
                let Some(prop) = registry.try_property(*property) else {
                    continue;
                };
                let layout = &prop.layout;
                if let Some(col) = range.col_for_role(&SubFieldRole::Intensity, layout) {
                    if let Some(v) = shadow.get_mut(base + col.raw()) {
                        *v = *value;
                    }
                }
            }
        }
    }
}

/// Route a new overlay from its required origin to a target SimThing.
/// Returns `true` only when both endpoints belong to the supplied tree.
pub fn attach_overlay(
    root: &mut SimThing,
    target: SimThingId,
    overlay: simthing_core::Overlay,
) -> bool {
    simthing_core::deliver_routed_overlay(root, target, overlay).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        DimensionRegistry, DissolveCondition, Overlay, OverlayId, OverlayKind, OverlayLifecycle,
        OverlaySource, PropertyTransformDelta, SimProperty, SimPropertyId, SimThing, SimThingKind,
        SubFieldRole, TransformOp,
    };
    use simthing_gpu::SlotAllocator;

    fn make_overlay(lifecycle: OverlayLifecycle, pid: SimPropertyId) -> Overlay {
        Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Transient,
            source: OverlaySource::System,
            origin: SimThingId::new(),
            affects: vec![],
            transform: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(0.1))],
            },
            lifecycle,
        }
    }
}
