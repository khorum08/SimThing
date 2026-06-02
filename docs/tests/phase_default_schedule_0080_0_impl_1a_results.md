# DEFAULT-SCHEDULE-0080-0 Implementation 1A Results

**Date:** 2026-06-02

**Verdict:** PASS - 1A scenario-scoped schedule + patrol loop.

## Files Touched

- `crates/simthing-driver/src/default_schedule_0080_0.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/default_schedule_0080_0.rs`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/production_paths/default_schedule_0080_0_opening_spec.md`
- `docs/tests/phase_default_schedule_0080_0_impl_1a_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`

## Scope

Implemented only `DEFAULT-SCHEDULE-0080-0` 1A: deterministic opt-in, scenario-scoped schedule +
patrol loop for Local Patrol Economy. The schedule runs bounded deterministic steps, evaluates
patrol-side SEAD threshold conditions over existing local values (`supply`, `maintenance`,
`local_output`, `local_security`, `disruption`), and routes emitted `BoundaryRequest`s through the
existing `run_production_path_0080_0`.

1B pirate behavior was not implemented. No pirate target heuristic, pirate disruption, pirate
relocation, predator loop, or cat-and-mouse assertion was added.

## Confirmations

- Opt-in/default-off: PASS.
- Default path has no schedule and no production-path invocation: PASS.
- No global default schedule: PASS.
- SEAD threshold/event/boundary routing: PASS.
- Production-path routing through `run_production_path_0080_0`: PASS.
- Identity, owner overlay, source/destination membership, and bounded economy reassociation: PASS.
- Bounded local economy only: PASS.
- Deterministic replay: PASS.
- No gameplay, semantic/raw WGSL, new shader, GPU kernel, CPU planner, hard currency,
  markets/trade/`ai_budget`, nested Resource Flow, ClauseThing implementation, invariant edit,
  passive proof wrapper, closed-ladder reopen, or general scheduler/runtime: PASS.

## Tests Run

- `cargo test -p simthing-driver --test default_schedule_0080_0` - PASS, 17 tests.
- `cargo test -p simthing-driver --test production_path_0080_0` - PASS.
- `cargo test -p simthing-spec --test mobility_alloc0_substrate` - PASS.
- `cargo test -p simthing-spec --test mobility_reenroll0_substrate` - PASS.
- `cargo test -p simthing-spec --test mobility_idroute0_substrate` - PASS.
- `cargo test -p simthing-spec --test mobility_econ0_substrate` - PASS.
- `cargo test -p simthing-spec --test mobility_owner0_substrate` - PASS.
- `cargo test -p simthing-spec --test mobility_runtime0_composition` - PASS.
- `cargo test -p simthing-spec --test mobility_runtime1_production_fixture` - PASS.
- `cargo test -p simthing-driver --test mobility_runtime1a_runtime_fixture` - PASS.
- `cargo test -p simthing-driver --test phase_m_sead_obs4_threshold_event` - PASS.
- `cargo test -p simthing-driver --test phase_m_sead_event0_compaction` - PASS.
- `cargo test -p simthing-driver --test phase_m_sead_pipe0_observer_event_pipeline` - PASS.
- `cargo test -p simthing-spec --test sead_obs0_overlay_score_admission` - PASS.
- `cargo check --workspace` - PASS with existing warnings.

## Skipped / Future

The 1B pirate-loop tests remain named future tests and were not implemented:

- `default_schedule_0080_0_pirate_raises_disruption_and_consumes_supply`
- `default_schedule_0080_0_pirate_relocates_when_disruption_ge_half_supply`
- `default_schedule_0080_0_patrol_reduces_disruption_and_relocates_to_depleted_supply`
- `default_schedule_0080_0_pirate_is_second_identity_not_second_economy_owner`
- `default_schedule_0080_0_predator_patrol_loop_replay_deterministic`
- `default_schedule_0080_0_pirate_prefers_low_patrol_influence_high_supply_target`
- `default_schedule_0080_0_cat_and_mouse_pattern_emerges_deterministically`
