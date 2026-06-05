# SCENARIO-0080-2-R4-IMPL-0 — FIELD_POLICY Field-Consumption + Exact Sqrt EC2 Report

**Verdict:** PASS
**Date:** 2026-06-04
**Gate:** R4 — FIELD_POLICY field-consumption + exact sqrt (EC2)
**Implementation:** SCENARIO-0080-2-R4-FIELD_POLICY-FIELD-CONSUMPTION

## Files Touched

- crates/simthing-driver/src/dress_rehearsal_r4_field_policy_consumption.rs
- crates/simthing-driver/tests/dress_rehearsal_r4_field_policy_consumption.rs
- crates/simthing-driver/src/lib.rs
- docs/tests/scenario_0080_2_r4_field_policy_consumption_report.md
- docs/design_0_0_8_0_consumer_pulled_production_track.md
- docs/worklog.md
- docs/workshop/mapping_current_guidance.md

## Scope Confirmation

R4 only. The implementation is an opt-in/default-off fixture over the single galactic tier. It
consumes the accepted R1 disruption/location-status heatmap, the implemented/pass R2 economy/blockade
signals, and the implemented/pass R3 owner-masked disposition overlays. It builds faction-specific
composite opportunity fields, runs `StructuredFieldStencilOperator::GradientXY` via the `simthing-gpu`
CPU oracle, derives exact Q16 pre-sqrt `mag2`, applies Candidate-F `sqrt_cr_f_bits` for commitment
magnitude, and threshold-gates `SitStill` vs `StepOpportunity` without relocating movers. It adds no
REENROLL, `BoundaryRequest`, R5 movement, R6 combat, new semantic WGSL, default SimSession wiring, CPU
planner, hard currency/markets/trade/`ai_budget`, ClauseThing, UI, or invariant edit.

## Upstream Contracts Consumed

- R1 report: checksum `17de0080304b3da7`, CPU oracle parity true, 400-cell `final_disruption` + `location_status`.
- R2 report: checksum `4fe0590589ddd975`, CPU oracle parity true, production/blockade rows consumed.
- R3 report: checksum `28afb4a204d101d2`, CPU oracle parity true, modifier overlays consumed.
- ATLAS-BATCH-0 owner/channel layout referenced via the shared `atlas_store` module include.

## Composite Field (per faction)

Each galactic cell builds a bounded composite opportunity from:

- R1 `final_disruption` and `location_status`;
- R1 cell `patrol_count` / `pirate_count`;
- R2 per-cell production/divert economy signal;
- R3 owner-masked modifier bps (`apply_modifier_bps`).

Pirate field weights low patrol penalty and high opportunity (emission + raiding logistics).
Patrol field weights high disruption seek (decay multiplier) plus defensive logistics on economy reads.
A deterministic spatial bias (`cell_index * 0.01 + x * 0.001`) keeps `GradientXY` non-degenerate at fleet cells.

## GradientXY + Exact Magnitude Authority

- Operator: `StructuredFieldStencilOperator::GradientXY { target_col_y: 2 }` on 20×20, horizon 1, zero boundary.
- CPU oracle: `simthing_gpu::cpu_horizon`.
- Fixed-point: Q16 via `MAG2_Q16_SCALE` (`simthing-spec`).
- Exact pre-sqrt `mag2`: integer `dx²+dy²` → pinned f32 mag2 bits.
- Candidate-F commitment: `sqrt_cr_f_bits` (Rust port of `crates/simthing-driver/tests/wgsl/sqrt_cr_f_candidate.wgsl`, hash `e2e9e27601ee2e13`).
- Diagnostic only: raw f32 `sqrt(dx²+dy²)` bits; never gates the threshold decision.

## Mover Field-Read Summary

Canonical movers are the fleet occupants with the strongest composite-field gradient magnitude per faction
(Pirate fleet + Terran patrol fleet). Each mover reads the parent grid at its own cell; positions are unchanged
after R4 (`occupant_positions_before == occupant_positions_after`).

## Threshold + Decisions

- Movement threshold: `0.01f32` (`MOVEMENT_THRESHOLD_MAG_BITS = 0x3c23d70a`).
- Commitment uses Candidate-F exact magnitude bits only.
- `StepOpportunity` emits a candidate target cell/direction when threshold passes and a higher-opportunity neighbor exists.
- `movement_applied = false` for all movers; no `BoundaryRequest`.

## Artifact Summary

| field | value |
|---|---:|
| mover_count | 2 |
| sit_still_count | varies with threshold |
| step_opportunity_count | ≥1 at default threshold |
| stable checksum | f0acbe2ccb98badb |
| CPU oracle parity | true |
| gradientxy_consumed | true |
| gpu_diagnostic_run | false (CPU oracle primary) |

## Identity Evidence

- No occupant relocation.
- No REENROLL.
- No `BoundaryRequest`.
- No combat resolution.
- No new semantic WGSL in R4 (Candidate-F artifact referenced, not re-authored).
- No default `SimSession` pass-graph change.

## Test Commands And Results

`cargo test -p simthing-driver --test dress_rehearsal_r4_field_policy_consumption`

Result: 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.

`cargo test -p simthing-driver --test dress_rehearsal_r3_capability_mask_down`

Result: 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.

`cargo test -p simthing-driver --test dress_rehearsal_r2_recursive_allocation`

Result: 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.

`cargo test -p simthing-driver --test dress_rehearsal_r1_disruption_heatmap`

Result: 34 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.

`cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store`

Result: 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.

`cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu`

Result: 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out (RTX discrete adapter gate present).

`cargo check --workspace`

Result: PASS (existing warning noise only).

### Nearest exact-sqrt / GradientXY harness tests

`cargo test -p simthing-driver --test phase_m_jit_sqrt_mag2_0_fixed_exact`

Result: 7 passed; 0 failed.

`cargo test -p simthing-driver --test phase_m_jit_sqrt_mag0_f_exact_magnitude`

Result: 12 passed; 0 failed.

`cargo test -p simthing-gpu gradient_xy_cpu_oracle`

Result: 2 passed; 0 failed.

## Artifact Excerpt

See generated markdown in `DressRehearsalR4Report::artifact` via `render_dress_rehearsal_r4_artifact` for per-mover
`GradientXY`, exact `mag2_bits`, Candidate-F magnitude bits, threshold, and decision rows.
