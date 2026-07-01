# TP-RF-CAPACITY-AMENDMENT-0 Results

## Status

PROBATION. The Tier-2 RF capacity amendment is implemented on `codex/tp-rf-capacity-amendment-0` and remains held for Owner/DA review before Phase 1 builds on it.

## What changed

- Added `ResourceFlowCapacityBudgetSpec` and a checked `ResolvedResourceFlowCapacityBudget` resolver using `u128` aggregate arithmetic and `SCENARIO_STRUCTURAL_INTEGER_MAX` surface validation.
- Applied the resolved budget to all three RF arena descriptor caps: `max_participants`, `max_coupling_fanout`, and `max_orderband_depth`.
- Carried the budget through RF admission/materialization, install, session state, GPU slot shape sync, and threshold/emission reservation at install boundaries.
- Preserved legacy behavior when no budget is authored; existing small authored caps remain effective.
- Added a galaxy-scale install proof for 250 owned systems plus 20 fleets with 30 cohorts each.

## Capacity-budget ledger

| Capacity surface | Previous behavior / bound | New budget source | Checked arithmetic / overflow behavior | Galaxy-scale value for 250-owned + fleet load | Allocation timing | Proof |
|---|---|---|---|---|---|---|
| SimThing count | Scenario authored tree only; small `n_slots` could underbudget RF scaffold. | `ResourceFlowCapacityBudgetSpec.simthing_count`. | Nonzero and `<= SCENARIO_STRUCTURAL_INTEGER_MAX`; participates in checked budget validation. | 871 authored SimThings. | Admission/install boundary. | Driver test installs the authored workload. |
| Property columns | Registry total was independent of RF budget. | `property_columns`. | Nonzero structural validation; `gpu_slots * property_columns` checked as `u128`. | 6 RF columns. | Admission/install boundary. | Spec test verifies `field_value_cells`. |
| RF arenas | Authored arenas only. | `rf_arena_count`. | Nonzero structural validation; multiplies checked per-arena registration budget. | 2 arenas. | Admission/materialization boundary. | Spec and driver tests observe two arenas. |
| Per-arena participants | Authored caps such as 8/16 could reject galaxy load. | `participants_per_arena`, applied as max(authored, budget). | Nonzero structural validation; included in checked registration budget. | 704 participants per arena. | Admission/materialization boundary. | Descriptor caps assert 704. |
| Coupling fanout | Authored cap could remain at 1. | `coupling_fanout_per_arena`, applied as max(authored, budget). | Nonzero structural validation; included in checked registration budget. | 8 fanout per arena. | Admission/materialization boundary. | Descriptor caps assert 8. |
| Orderband depth | Authored cap could remain at 1. | `orderband_depth`, applied as max(authored, budget). | Nonzero structural validation; included in checked registration budget. | 16 orderband depth. | Admission/materialization boundary. | Descriptor caps assert 16. |
| Emissions | Session threshold accumulator capacity was fixed/default at attach. | `emission_capacity` and `threshold_emission_capacity`. | Nonzero structural validation; session reserves max(emission, threshold emission, default). | 704 emission records. | Install/session attach boundary. | Driver test opens a real session under budget. |
| GPU slots | Scenario `n_slots` could stay at 64 in the proof fixture. | `gpu_slots`, resolved as at least `simthing_count`. | Nonzero structural validation; slot shape sync reserves budgeted slots. | 2048 GPU slots. | Install/session attach boundary. | Driver test asserts `session.state.n_slots >= 2048`. |
| Field buffers | Derived from existing slot/column shape. | `field_buffer_cells` plus checked `field_value_cells`. | `gpu_slots * property_columns` checked as `u128`; authored field buffer surface validated. | 12288 field buffer cells. | Boundary growth only. | Spec test verifies `field_value_cells = 2048 * 6`. |
| Atlas/theater size if relevant | Not part of this RF cap amendment; atlas deferral remains governed by existing mapping/STEAD budgets. | No new source. | No new arithmetic. | Unchanged / not relevant. | No allocation here. | Boundary section below records non-goal. |
| Readback policy | Existing scoped proof/debug readback policy. | `readback_records` documents the RF proof envelope. | Nonzero structural validation. | 704 records. | Proof/readback boundary only. | Driver proof uses install/session assertions, not per-tick debug readback. |
| Explicit no-per-tick-allocation assertion | Old small pools could fail before meaningful scale proof. | Budget is resolved and reserved before tick execution. | No per-tick capacity growth path added. | Boundary-reserved pools cover the target load. | Admission/install/session attach only. | Code path updates are in install/session, not tick loops. |

## Boundary / non-goals

- No new `AccumulatorRole`.
- No new runtime match kind.
- No semantic WGSL/runtime vocabulary.
- No route, pathfinding, combat, economy, AI, or diplomacy subsystem.
- No scanner or allowlist widening.
- No per-tick allocation; budgeted growth is at admission/install/session attach boundaries.
- Atlas/theater scale is not changed by this rung.

## Load-bearing proofs

- `cargo check -p simthing-spec` passed.
- `cargo check -p simthing-driver` passed.
- `cargo test -p simthing-spec --test e10_resource_flow_admission` passed: 19 passed.
- `cargo test -p simthing-driver --test tp_rf_capacity_amendment tp_rf_capacity_budget_installs_250_owned_systems_plus_fleet_load -- --nocapture` passed.
- `cargo check -p simthing-clausething` passed.
- `cargo check -p simthing-mapeditor` passed.
- `bash scripts/ci/gen_digest.sh --check` passed.
- `bash scripts/ci/doctrine_scan.sh` passed: `DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED`.

## INSPECT / triage

None. Fresh doctrine scan reported 0 inspect flags.

## Scope Ledger

Touched scope is limited to RF budget admission, RF materialization/enrollment cap application, session install/attach reservations, literal RF spec construction fallout, one studio OwnerRef boundary compile repair needed by the driver test dev-dependency, targeted tests, and evidence docs.

## Graduation routing

Graduation routing (for DA - why PROBATION, not COMPLETE):
  CI verdict:          PASS-RELIABLE
  Triage entries:      none
  Risk class:          seal-residue + gate-wiring
  Falsification check: Verify all three GpuArenaDescriptor caps and GPU slot/emission capacity scale from a checked budget; verify no new AccumulatorRole, no semantic WGSL/runtime words, no per-tick allocation; run the targeted RF/admission tests and inspect the one-table capacity ledger.
  Recommended posture: deep - Tier-2 closed-lowerer capacity amendment; all later 0.0.8.5 rungs inherit these caps.

## Known gaps / next

Owner/DA review is still required. Phase 1 remains held until this PROBATION capacity amendment is reviewed against the tree.
