//! GPU overlay numerical lifecycle (7.7).
//!
//! Production dissolve decisions compare admitted deadline_generation and
//! property thresholds. Physical slots exist only in this upload/epoch row.
//! No AfterTicks decrement, no overlay-local EML table, no OverlayHistory.

use simthing_core::{
    authored_after_ticks_duration, deadline_reached, establish_deadline, GenerationStamp,
    OverlayId, OverlayLifecycleBinding, SimThing, SimThingId,
};
use std::collections::HashMap;

use simthing_gpu::SlotAllocator;

pub const OVERLAY_LIFECYCLE_ROW_BYTES: usize = 32;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OverlayLifecycleInstanceGpu {
    pub host_slot: u32,
    pub deadline_generation: u32,
    pub threshold_col: u32,
    pub direction: u32,
    pub threshold_value: f32,
    pub active: u32,
    pub overlay_id_raw: u32,
    pub _pad: u32,
}

impl OverlayLifecycleInstanceGpu {
    pub const DEADLINE_NONE: u32 = u32::MAX;
    pub const COL_NONE: u32 = u32::MAX;
}

#[derive(Clone, Debug, Default)]
pub struct OverlayLifecycleCarry {
    pub instance_rows: usize,
    pub bytes: usize,
}

#[derive(Clone, Debug, Default)]
pub struct OverlayLifecycleGpuDecision {
    pub dissolved: Vec<(SimThingId, OverlayId)>,
    pub carry: OverlayLifecycleCarry,
}

/// Session-frozen overlay template set. Mid-session mint is refused.
#[derive(Clone, Debug, Default)]
pub struct OverlayLifecycleSession {
    templates_frozen: bool,
    template_count: u32,
    deadlines: HashMap<OverlayId, GenerationStamp>,
    current: Vec<u32>,
    next: Vec<u32>,
}

impl OverlayLifecycleSession {
    pub fn freeze_templates(&mut self, count: u32) {
        self.templates_frozen = true;
        self.template_count = count;
    }

    pub fn mint_semantic_template(&mut self) -> Result<(), OverlayLifecycleGpuError> {
        if self.templates_frozen {
            return Err(OverlayLifecycleGpuError::MidSessionTemplateMint);
        }
        self.template_count = self.template_count.saturating_add(1);
        Ok(())
    }

    pub fn swap_current_next(&mut self) {
        std::mem::swap(&mut self.current, &mut self.next);
    }

    pub fn deadlines_mut(&mut self) -> &mut HashMap<OverlayId, GenerationStamp> {
        &mut self.deadlines
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum OverlayLifecycleGpuError {
    #[error("mid-session overlay semantic-template mint refused")]
    MidSessionTemplateMint,
    #[error("durable binding captured a physical row")]
    DurableRowCapture,
    #[error("foreign absolute deadline transported across a receive boundary")]
    ForeignAbsoluteDeadline,
    #[error("overlay-local EML table is forbidden")]
    OverlayLocalEmlTable,
    #[error("global clock used for overlay deadline compare")]
    GlobalClock,
}

pub fn refuse_overlay_local_eml_table() -> OverlayLifecycleGpuError {
    OverlayLifecycleGpuError::OverlayLocalEmlTable
}

pub fn refuse_durable_row_capture(_physical_row: u32) -> OverlayLifecycleGpuError {
    OverlayLifecycleGpuError::DurableRowCapture
}

pub fn refuse_foreign_absolute_deadline() -> OverlayLifecycleGpuError {
    OverlayLifecycleGpuError::ForeignAbsoluteDeadline
}

pub fn refuse_global_clock() -> OverlayLifecycleGpuError {
    OverlayLifecycleGpuError::GlobalClock
}

/// Numerical twin of the overlay-lifecycle WGSL. Production GPU and this
/// oracle share the same compare: generation >= deadline, no decrement.
pub fn evaluate_instance(
    generation: GenerationStamp,
    instance: &OverlayLifecycleInstanceGpu,
    host_value: Option<f32>,
) -> bool {
    if instance.active == 0 {
        return false;
    }
    let deadline_hit = instance.deadline_generation != OverlayLifecycleInstanceGpu::DEADLINE_NONE
        && deadline_reached(
            generation,
            GenerationStamp::new(instance.deadline_generation),
        );
    let property_hit = match instance.direction {
        1 => host_value
            .map(|v| v >= instance.threshold_value)
            .unwrap_or(false),
        2 => host_value
            .map(|v| v < instance.threshold_value)
            .unwrap_or(false),
        _ => false,
    };
    deadline_hit || property_hit
}

pub fn bind_tree_overlays(
    root: &SimThing,
    allocator: &SlotAllocator,
    generation: GenerationStamp,
    deadlines: &mut HashMap<OverlayId, GenerationStamp>,
) -> (Vec<OverlayLifecycleBinding>, Vec<OverlayLifecycleInstanceGpu>) {
    let mut bindings = Vec::new();
    let mut instances = Vec::new();
    fn walk(
        node: &SimThing,
        allocator: &SlotAllocator,
        generation: GenerationStamp,
        deadlines: &mut HashMap<OverlayId, GenerationStamp>,
        bindings: &mut Vec<OverlayLifecycleBinding>,
        instances: &mut Vec<OverlayLifecycleInstanceGpu>,
    ) {
        let slot = allocator.slot_of(node.id).map(|s| s.raw()).unwrap_or(u32::MAX);
        for overlay in &node.overlays {
            if !overlay.is_active() {
                continue;
            }
            let deadline = authored_after_ticks_duration(&overlay.lifecycle).and_then(|duration| {
                Some(
                    *deadlines.entry(overlay.id).or_insert_with(|| {
                        establish_deadline(generation, duration)
                            .unwrap_or(GenerationStamp::new(u32::MAX))
                    }),
                )
            });
            bindings.push(OverlayLifecycleBinding {
                overlay_id: overlay.id,
                host: node.id,
                property_id: overlay.transform.property_id,
                deadline,
            });
            instances.push(OverlayLifecycleInstanceGpu {
                host_slot: slot,
                deadline_generation: deadline
                    .map(|d| d.get())
                    .unwrap_or(OverlayLifecycleInstanceGpu::DEADLINE_NONE),
                threshold_col: OverlayLifecycleInstanceGpu::COL_NONE,
                direction: 0,
                threshold_value: 0.0,
                active: 1,
                overlay_id_raw: overlay.id.raw(),
                _pad: 0,
            });
        }
        for child in &node.children {
            walk(
                child, allocator, generation, deadlines, bindings, instances,
            );
        }
    }
    walk(
        root,
        allocator,
        generation,
        deadlines,
        &mut bindings,
        &mut instances,
    );
    (bindings, instances)
}

pub fn decide_dissolves(
    _root: &SimThing,
    generation: GenerationStamp,
    instances: &[OverlayLifecycleInstanceGpu],
    bindings: &[OverlayLifecycleBinding],
) -> OverlayLifecycleGpuDecision {
    let mut dissolved = Vec::new();
    for (instance, binding) in instances.iter().zip(bindings.iter()) {
        if evaluate_instance(generation, instance, None) {
            dissolved.push((binding.host, binding.overlay_id));
        }
    }
    OverlayLifecycleGpuDecision {
        dissolved,
        carry: OverlayLifecycleCarry {
            instance_rows: instances.len(),
            bytes: instances.len() * OVERLAY_LIFECYCLE_ROW_BYTES,
        },
    }
}

pub fn apply_structural_dissolves(
    root: &mut SimThing,
    dissolved: &[(SimThingId, OverlayId)],
) -> u32 {
    let mut count = 0;
    fn apply(node: &mut SimThing, dissolved: &[(SimThingId, OverlayId)], count: &mut u32) {
        let before = node.overlays.len();
        node.overlays
            .retain(|o| !dissolved.iter().any(|(host, id)| *host == node.id && *id == o.id));
        *count += (before - node.overlays.len()) as u32;
        for child in &mut node.children {
            apply(child, dissolved, count);
        }
    }
    apply(root, dissolved, &mut count);
    count
}
