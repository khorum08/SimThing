//! EVENT-GENERATION-STAMP-0 Remand 5 — live feeder production egress referee.
//!
//! Exercises the ordinary `DispatchCoordinator::tick` path with a live emission
//! session. Observes `WorldGpuState::production_event_egress` after the tick.
//! Turns RED if the production egress call is removed or wrapped in `if false`
//! (`admit_invocations` stays 0). Forced ring lag must not perturb sim values.

use simthing_core::{ClampBehavior, DimensionRegistry, SimProperty, SubFieldRole, SubFieldSpec};
use simthing_feeder::{feeder_channel, DispatchCoordinator, TickGpuError, TransformPatcher};
use simthing_gpu::{
    set_debug_readback_allowed, GpuContext, Pipelines, SlotAllocator, WorldGpuState,
};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn minimal_registry(n_dims: u32) -> DimensionRegistry {
    let mut reg = DimensionRegistry::new();
    let sub_fields: Vec<SubFieldSpec> = (0..n_dims)
        .map(|i| SubFieldSpec {
            role: SubFieldRole::Named(format!("c{i}")),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: format!("c{i}"),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        })
        .collect();
    reg.register(SimProperty {
        namespace: "egs".into(),
        name: "cols".into(),
        admission_disposition: Default::default(),
        layout: simthing_core::PropertyLayout { sub_fields },
        decay: None,
        intensity_behavior: None,
        fission_templates: vec![],
        fusion_templates: vec![],
        on_expire: None,
        description: String::new(),
        intensity_labels: vec![],
    });
    reg
}
