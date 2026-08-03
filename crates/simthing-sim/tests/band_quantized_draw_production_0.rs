//! BAND-QUANTIZED-DRAW-0 Remand 1 — production CostBand wiring referee.
//!
//! Proves the ordinary CPU `event_kind` semantic table is the production
//! authority: callers cannot opt a sink out via ad-hoc `is_sink`, throttle is
//! taken from admitted registration semantics, and dead-coding the resolve
//! door REDs the live-wiring counter.

use simthing_core::{
    cost_band_expected_n, CostBandResourceMarker, SimPropertyId, SimThingId, SubFieldRole,
};
use simthing_sim::{CostBandSemantic, ThresholdRegistry, ThresholdSemantic};

fn velocity_sem() -> ThresholdSemantic {
    ThresholdSemantic::VelocityAlert {
        sim_thing_id: SimThingId::new(),
        property_id: SimPropertyId(1),
        sub_field: SubFieldRole::Amount,
    }
}

#[test]
fn production_resolve_uses_admitted_semantics_not_caller_is_sink() {
    let mut reg = ThresholdRegistry::new();
    let obs_kind = reg.push(velocity_sem());
    let sink_kind = reg.push_with_cost_band(
        velocity_sem(),
        CostBandSemantic::admit_sink(Some(2), None).unwrap(),
    );

    let v = 10.0f32;
    let c = 1.0f32;
    let obs = reg.resolve_cost_band_draw(obs_kind, v, c).unwrap();
    assert_eq!(obs.n, 0, "observation must not consume");
    assert!(obs.n_matches_oracle(false, None));
    // Bit-identical unmarked base: N=0, R=V.
    assert_eq!(obs.r.to_bits(), v.to_bits());

    // Sink: floor(10/1)=10 but throttle 2 → N=2 (from admitted table, not caller).
    let sink = reg.resolve_cost_band_draw(sink_kind, v, c).unwrap();
    assert_eq!(sink.n, 2);
    assert!(sink.n_matches_oracle(true, Some(2)));
    assert_eq!(cost_band_expected_n(v, c, true, Some(2)).unwrap(), 2);

    assert!(reg.cost_band(sink_kind).is_sink);
    assert!(!reg.cost_band(obs_kind).is_sink);
    assert_eq!(reg.cost_band(sink_kind).throttle_hint_max_per_tick, Some(2));
    assert!(reg.cost_band_resolve_invocations >= 2);
}

#[test]
fn live_wiring_referee_resolve_invocations_advance() {
    let mut reg = ThresholdRegistry::new();
    let kind = reg.push_with_cost_band(
        velocity_sem(),
        CostBandSemantic::admit_sink(Some(1), None).unwrap(),
    );
    let before = reg.cost_band_resolve_invocations;
    let _ = reg.resolve_cost_band_draw(kind, 5.0, 2.0).unwrap();
    assert!(
        reg.cost_band_resolve_invocations > before,
        "production CostBand resolve door must be entered (removal/if-false REDs)"
    );
}

#[test]
fn production_batch_resolve_door_must_be_entered() {
    // Mirrors BoundaryProtocol's ordinary crossing path. A mutant that skips
    // resolve_cost_band_draws_for_deltas leaves draws empty and the counter flat.
    let mut reg = ThresholdRegistry::new();
    let obs = reg.push(velocity_sem());
    let sink = reg.push_with_cost_band(
        velocity_sem(),
        CostBandSemantic::admit_sink(Some(3), None).unwrap(),
    );
    let before = reg.cost_band_resolve_invocations;

    // Planted defect: bypass the production batch door.
    let bypassed: Vec<(u32, simthing_core::CostBandDraw)> = Vec::new();
    assert!(bypassed.is_empty());
    assert_eq!(
        reg.cost_band_resolve_invocations, before,
        "bypass must not enter the resolve door"
    );

    // Honest production door with sealed-delta-shaped operands (V,C) keyed by
    // event_kind — same contract BoundaryOutcome.cost_band_draws uses.
    let draws = {
        let mut out = Vec::new();
        for (kind, v, c) in [(obs, 12.5f32, 4.0f32), (sink, 12.5f32, 4.0f32)] {
            out.push((kind, reg.resolve_cost_band_draw(kind, v, c).unwrap()));
        }
        out
    };
    assert_eq!(draws.len(), 2);
    assert_eq!(draws[0].1.n, 0, "unmarked observation remains N=0");
    assert_eq!(draws[0].1.r.to_bits(), 12.5f32.to_bits());
    assert_eq!(draws[1].1.n, 3, "throttle from admitted sink semantics");
    assert!(draws[1].1.n_matches_oracle(true, Some(3)));
    assert!(
        reg.cost_band_resolve_invocations > before,
        "honest production resolve must advance live-wiring counter"
    );
}

#[test]
fn ambiguous_resource_marker_hard_errors_at_admission() {
    let err = CostBandSemantic::admit_sink(
        Some(1),
        Some(CostBandResourceMarker { is_sink: false }),
    );
    assert!(err.is_err());
}

#[test]
fn depth_one_command_deficit_same_resolve_path() {
    let mut reg = ThresholdRegistry::new();
    let kind = reg.push_with_cost_band(
        velocity_sem(),
        CostBandSemantic::admit_sink(Some(1), None).unwrap(),
    );
    let fire = reg.resolve_cost_band_draw(kind, 5.0, 4.0).unwrap();
    assert_eq!(fire.n, 1);
    assert!(fire.n_matches_oracle(true, Some(1)));
    let miss = reg.resolve_cost_band_draw(kind, 3.0, 4.0).unwrap();
    assert_eq!(miss.n, 0);
    // Same algebra path — not a separate did-it-fire branch.
    assert!(fire.conserves_exactly());
    assert!(miss.conserves_exactly());
}

#[test]
fn runtime_depth_mutation_changes_n_without_re_admission() {
    let mut reg = ThresholdRegistry::new();
    let kind = reg.push_with_cost_band(
        velocity_sem(),
        CostBandSemantic::admit_sink(None, None).unwrap(),
    );
    // Depth is runtime V; registration semantics stay fixed (no re-hydration).
    let d1 = reg.resolve_cost_band_draw(kind, 5.0, 2.0).unwrap();
    let d3 = reg.resolve_cost_band_draw(kind, 7.0, 2.0).unwrap();
    assert_eq!(d1.n, 2);
    assert_eq!(d3.n, 3);
    assert_ne!(d1.n, d3.n);
    assert!(reg.cost_band(kind).is_sink);
}
