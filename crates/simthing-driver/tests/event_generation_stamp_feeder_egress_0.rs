//! EVENT-GENERATION-STAMP-0 Remand 5 — live feeder production egress referee.
//!
//! Exercises the ordinary `DispatchCoordinator::tick` path with a live emission
//! session. Observes `WorldGpuState::production_event_egress` after the tick.
//! Turns RED if the production egress call is removed or wrapped in `if false`
//! (`admit_invocations` stays 0). Forced ring lag must not perturb sim values.

use simthing_core::{
    ClampBehavior, DimensionRegistry, SimProperty, SubFieldRole, SubFieldSpec,
};
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

#[test]
fn ordinary_feeder_tick_admits_to_production_event_egress() {
    let Some(ctx) = try_gpu() else {
        return;
    };
    let n_slots = 2u32;
    let n_dims = 2u32;
    let reg = minimal_registry(n_dims);
    let allocator = SlotAllocator::new();
    let mut state = WorldGpuState::new(ctx, &reg, n_slots);
    set_debug_readback_allowed(true);

    // Activate emission session so the ordinary tick takes/restores it and
    // invokes production egress. bands=0 skips encode emission (no ops), but
    // accumulator_emission_active alone is enough for the dispatcher branch
    // to take the session and call the production egress door.
    state.ensure_emission_accumulator();
    state.set_emission_dispatch(true, 0);
    state.bind_production_generation(3);

    let values_before = state.read_values();
    let admits_before = state.production_event_egress.admit_invocations;

    let mut coord = DispatchCoordinator::new(n_slots, n_dims, 10);
    coord.shadow = values_before.clone();
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(n_slots as usize);
    let (_tx, rx) = feeder_channel();
    let outcome = coord.tick(
        &rx,
        &mut patcher,
        &reg,
        &allocator,
        &pipelines,
        &mut state,
        1.0 / 60.0,
    );

    // Live path proof: production egress door was entered on the ordinary tick.
    // if-false / removal leaves admit_invocations unchanged → RED.
    assert!(
        state.production_event_egress.admit_invocations > admits_before,
        "ordinary feeder tick must enter push_emissions_into_production_egress \
         (admit_invocations stuck — path removed or if-false guarded)"
    );

    // Empty sealed batch is Ok; unsealed/mismatch would surface gpu_error.
    // Door entry is required regardless.
    let _ = outcome.gpu_error;

    let values_after = state.read_values();
    assert_eq!(
        values_before, values_after,
        "observer egress / lag must not perturb sim values"
    );

    // Forced lag on the production ring does not write sim state.
    let lag_before = values_after.clone();
    for i in 0..300u64 {
        let _ = state.production_event_egress.push(simthing_core::StampedEgressEntry {
            generation: simthing_core::GenerationStamp::new(9),
            key: i,
            payload_bits: i,
        });
    }
    assert!(
        state.production_event_egress.backpressure_actions >= 1,
        "capacity-bounded ring must record lag/backpressure under forced push"
    );
    assert_eq!(
        lag_before,
        state.read_values(),
        "forced ring lag must leave sim values unchanged"
    );
}

#[test]
fn production_egress_error_variant_is_distinct_from_threshold_readback() {
    // Structural: swallowed errors cannot use a silent unit path — the variant exists
    // for feeder tick to surface unsealed/generation-mismatch failures.
    let a = TickGpuError::ProductionEmissionEgress("unsealed".into());
    let b = TickGpuError::AccumulatorThresholdReadback("thr".into());
    assert_ne!(a, b);
    assert!(matches!(a, TickGpuError::ProductionEmissionEgress(_)));
}
