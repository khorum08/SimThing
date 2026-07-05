# TP-DIPLOMACY-FLOW-0 Results

## Status

**PROBATION / DA-OWNER REVIEW — not self-mergeable.** Phase 6 blocked until DA clearance.

## Identity

| Field | Value |
|---|---|
| PR | (pending open) |
| Branch | `tp-diplomacy-flow-0` |
| Base | `origin/master` @ `b8ed0500c4` |
| Head | `c251e8ae` |
| Rung | Phase 5 `TP-DIPLOMACY-FLOW-0` |
| Mechanism | **B — consumer-side application** from `simthing-workshop` |

## Mechanism B — consumer-side application

| Stage | Location | Notes |
|---|---|---|
| Base ClauseThing hydration | `hydrate_scenario` on generic TP planet-surface clause (no diplomacy blocks) | No `simthing-clausething/src` diplomacy edits |
| Workshop post-hydration | `simthing-workshop/src/diplomacy_post_hydration.rs` | Scenario candidate code |
| Test driver | `simthing-workshop/tests/tp_diplomacy_flow_0.rs` | Applies hydrator after base pack |

**Base ClauseThing pipeline took no scenario-specific diplomacy edits.**

## Homing Boundary Classification

Classifier: *Would this code exist if this scenario didn't?*

| Symbol / path | Would this exist without TP? | Classification | Action |
|---|---:|---|---|
| `apply_diplomacy_post_hydration` | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `DiplomacyHydrationError` | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `TP_DISTRUST_RESOURCE_KEY` / `BASELINE_BORDER_DISTRUST_SURPLUS` | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `HOSTILITY_DISTRUST_THRESHOLD` / `HOSTILITY_COMMITMENT_EVENT_KIND` | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `diplomacy_post_hydration.rs` tree walks (`game_session_child_mut`, coord match) | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `tp_diplomacy_flow_0.rs` test driver | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `simthing-clausething/src/**` | — | untouched | no engine diplomacy semantics |
| `simthing-spec/src/**` | — | untouched | no engine diplomacy semantics |
| `simthing-kernel/src/**` | — | untouched | no substrate widening |

No symbol in this delta is classified as generic future-utility engine surface. Zero engine-crate source edits.

## Substrate widening

| Item | Status |
|---|---|
| Engine crates touched | **none** |
| Generic future-utility justification | n/a — workshop-only delta |
| Gameplay semantics in engine crates | **no** |

## Diplomacy RF proof

| Proof | Test | Result |
|---|---|---|
| Distrust threshold → hostility commitment | `distrust_threshold_emits_hostility_commitment` | PASS — reduce-up/writeback crosses threshold; GPU `emit_on_threshold` emits `HOST` (`0x484F5354`) |
| Trust/distrust GPU==CPU | `trust_distrust_gpu_matches_cpu_oracle` | PASS — bit-exact reduce-up surplus/deficit aggregate (owner-local RTX 4080) |
| Influence round-trip to owner | `influence_round_trip_reduces_to_owner` | PASS — `evaluate_runtime_rf_tick` owner silo writeback with `applied_surplus > 0`; no disburse hand-copy |
| Workshop homing required | `workshop_post_hydration_application_is_required` | PASS — surplus delta zero without `apply_diplomacy_post_hydration` |
| No CPU planner | — | yes — threshold + RF oracle paths only |

## Scope ledger

| Item | Touched? | Notes |
|---|---|---|
| simthing-workshop | yes | diplomacy hydrator + tests |
| simthing-clausething/src | no | generic hydration only via existing clause |
| simthing-spec/src | no | consumer of existing RF/threshold APIs |
| simthing-kernel/src | no | — |
| simthing-sim / gpu / driver | no | dev-deps / test-only imports |
| test_inventory.tsv | yes | 4 new rows |
| test_lifecycle_boundary_rows.tsv | yes | 4 new rows |
| test_lifecycle_tracks.tsv | no | no per-rung lifecycle track |

## Rustification / lifecycle

| Test | birth_track | class | verdict |
|---|---|---|---|
| `distrust_threshold_emits_hostility_commitment` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `trust_distrust_gpu_matches_cpu_oracle` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `influence_round_trip_reduces_to_owner` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `workshop_post_hydration_application_is_required` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |

## Citable GPU proof

```
DOCTRINE-TESTS-VERDICT: PASS
head_sha: c251e8ae
profile: owner-local GPU / tp_diplomacy_flow_0
owner_local: true
proof: trust_distrust_gpu_matches_cpu_oracle
result: PASS
```

Owner-local execution: 4/4 `tp_diplomacy_flow_0` tests PASS on real adapter (no silent GPU skip).

## Graduation routing

- CI verdict: pending PR head proofs
- Risk class: workshop-homed scenario semantics only; no engine widening
- Phase 6 status: **blocked** pending DA clearance