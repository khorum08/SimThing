# TP-DIPLOMACY-FLOW-0 / 0R Results

## Status

**PROBATION / DA-OWNER REVIEW — not self-mergeable.** Phase 6 blocked until DA clearance.

## Identity

| Field | Value |
|---|---|
| PR | [#1150](https://github.com/khorum08/SimThing/pull/1150) |
| Branch | `tp-diplomacy-flow-0` |
| Base | `origin/master` @ `b8ed0500c4` |
| Tested code SHA | `a2c6e942db531ff8a233d82a48d055a73cc864fe` |
| Current PR head | `058d4060ff30387d21fb3a579b4fe309358e0364` |
| Rung | Phase 5 `TP-DIPLOMACY-FLOW-0` + remedial `TP-DIPLOMACY-FLOW-0R` |
| Mechanism | **B — consumer-side application** from `simthing-workshop` |

## TP-DIPLOMACY-FLOW-0R — stale GPU proof SHA repair

| Item | Value |
|---|---|
| Stale GPU proof SHA repaired | **yes** |
| Old proof SHA (PR body / results) | `9b56c987` (short; inconsistent with prior PR heads) |
| Tested code SHA | `a2c6e942db531ff8a233d82a48d055a73cc864fe` |
| Current PR head | `058d4060ff30387d21fb3a579b4fe309358e0364` |
| Owner-local GPU test re-run at tested code SHA | **yes** — 2026-07-05 |

## Coverage basis (post-`tested_code_sha` commits)

```bash
git diff --name-only a2c6e942db531ff8a233d82a48d055a73cc864fe..HEAD
```

```
docs/tests/tp_diplomacy_flow_0_results.md
```

**coverage_basis:** PASS — commits after `tested_code_sha` are docs/evidence-only and do not affect the tested binary.

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
| `influence_round_trip_reduces_to_owner` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `trust_distrust_gpu_matches_cpu_oracle` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `workshop_post_hydration_application_is_required` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |

## Citable GPU proof (0R)

```
DOCTRINE-TESTS-VERDICT: PASS
tested_code_sha: a2c6e942db531ff8a233d82a48d055a73cc864fe

coverage_basis: PASS — commits after tested_code_sha are docs/evidence-only and do not affect the tested binary
profile: owner-local GPU / tp_diplomacy_flow_0
owner_local: true
proof: trust_distrust_gpu_matches_cpu_oracle
result: PASS
```

| Field | Value |
|---|---|
| GPU adapter initialized | **yes** — `GpuContext::new_blocking()` succeeds (`require_gpu()` hard-fails without adapter) |
| GPU path actually executed | **yes** — `trust_distrust_gpu_matches_cpu_oracle` + `distrust_threshold_emits_hostility_commitment` GPU threshold scan |
| Any skip/ignore behavior | **no** — `0 ignored`; no `#[ignore]` on GPU proof tests |

### Raw tail (`cargo test -p simthing-workshop --test tp_diplomacy_flow_0 -- --nocapture` @ `a2c6e942`)

```
    Finished `test` profile [optimized + debuginfo] target(s) in 0.48s
     Running tests\tp_diplomacy_flow_0.rs (target\debug\deps\tp_diplomacy_flow_0-4bd9e01c3139ff84.exe)

running 4 tests
test influence_round_trip_reduces_to_owner ... ok
test workshop_post_hydration_application_is_required ... ok
test distrust_threshold_emits_hostility_commitment ... ok
test trust_distrust_gpu_matches_cpu_oracle ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s
```

## Load-bearing proofs (@ `tested_code_sha` / owner-local 2026-07-05)

| Proof | Verdict |
|---|---|
| `cargo check -p simthing-workshop` | PASS |
| `cargo check -p simthing-clausething` | PASS |
| `cargo test -p simthing-workshop --test tp_diplomacy_flow_0` | PASS — 4/4 |
| `test_inventory_check.sh` | INSPECT exit 0 (`failures=0`; 2 pre-existing extra fixture rows) |
| `test_inventory_drift_check.sh` | PASS |
| `test_lifecycle_boundary_check.sh` | PASS |
| `test_lifecycle_expiry_check.sh --schema` | PASS |
| `test_lifecycle_expiry_check.sh --prove` | PASS |
| `gen_digest.sh --check` | PASS |
| `doctrine_scan.sh` | INSPECT `failures=0 inspect=415` |
| `git diff --check origin/master...HEAD` | PASS |

## Graduation routing

- Ready for DA relay: **no** — awaiting owner/orchestrator decision after 0R evidence repair (not self-mergeable)
- CI verdict: owner-local proofs PASS at `tested_code_sha`; coverage_basis PASS; doctrine scan hard failures 0
- Risk class: workshop-homed scenario semantics only; no engine widening
- Phase 6 status: **blocked** pending DA clearance