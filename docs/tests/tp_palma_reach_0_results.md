# TP-PALMA-REACH-0 Results

## Status

**PROBATION / DA-OWNER REVIEW — not self-mergeable.** Phase 6.2 (`TP-FLEET-MOVEMENT-0`) blocked until DA clearance of 6.1.

## Identity

| Field | Value |
|---|---|
| PR | (pending open) |
| Branch | `tp-palma-reach-0` |
| Base | `origin/master` @ `9f56794a4f20696d83d6098d7ea2263728a639de` |
| Tested code SHA | `905fb35a0da5570564de747347e14b40a4f477df` |
| Current PR head | (pending evidence commit) |
| Rung | Phase 6.1 `TP-PALMA-REACH-0` |
| Mechanism | **B — consumer-side application** from `simthing-workshop` |

## Mechanism

| Stage | Location | Notes |
|---|---|---|
| Base ClauseThing hydration | `hydrate_scenario` on generic TP clause + fleet payloads | No PALMA semantics in engine crates |
| Fronts post-hydration | `apply_fronts_post_hydration` (Phase 6.0) | Threat / suppression / disruption RF + Movement-Front |
| PALMA post-hydration | `simthing-workshop/src/palma_reach_post_hydration.rs` | W impedance compose + PALMA feedstock over accepted fronts |
| Test driver | `simthing-workshop/tests/tp_palma_reach_0.rs` | Applies PALMA hydrator after fronts |

**Base ClauseThing pipeline took no scenario-specific PALMA edits.**

## Homing Boundary Classification

| Symbol / path | Would this exist without TP? | Classification | Action |
|---|---:|---|---|
| `apply_palma_reach_post_hydration` | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `build_tp_palma_w_compose` | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `impedance_w_composition_oracle` | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `palma_reach_gradient_probe` | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `tp_palma_reach_0.rs` test driver | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `simthing-clausething/src/**` | — | untouched | no engine PALMA semantics |
| `simthing-spec/src/**` | — | untouched | no substrate widening |
| `simthing-kernel/src/**` | — | untouched | no substrate widening |
| `simthing-sim/src/**` | — | dev-dep CPU oracle import only | no engine PALMA semantics |
| `simthing-driver/src/**` | — | dev-dep test imports only | no engine PALMA semantics |
| `simthing-gpu/src/**` | — | dev-dep test imports only | no engine PALMA semantics |

Engine source edits: **none.** Generic substrate widening: **none.** Gameplay semantics in engine crates: **no.**

## Substrate widening

| Item | Status |
|---|---|
| Engine crates touched | **none** |
| Generic future-utility justification | n/a — workshop-only delta |
| Gameplay semantics in engine crates | **no** |

## PALMA W/D field definitions

| Field | Column | Source |
|---|---|---|
| W (impedance) | `TP_PALMA_W_OUTPUT_COL` (3) | `base_w + weight_a*choke + weight_b*suppression`; choke from SaturatingFlux col 2; suppression col 1 from terran arena |
| D (reach) | `TP_PALMA_D_OUTPUT_COL` (5) | Min-plus relaxation over composed W; dest = terran patrol anchor |
| Front pressure (L1) | col 0 | Accepted threat/suppression/disruption arena seeds + diffusion |
| Choke | col 2 | SaturatingFlux choke output from front field |

Bounded theater: same 3×3 contested-border sub-volume as Phase 6.0.

## Resident D proof

| Proof | Test | Result |
|---|---|---|
| D field resident on GPU | `palma_reach_field_resident_on_gpu` | PASS — serial W→PALMA chain + compact `MinPlusTraversalDProbeOp` |
| No route reconstruction | — | yes — probe gathers D at explicit cell indices only |

## Impedance composition proof

| Proof | Test | Result |
|---|---|---|
| W from front/choke/suppression | `impedance_w_composes_from_threat_and_choke_fields` | PASS — pirate W > terran W; suppression col positive on terran |
| CPU + GPU W compose | — | PASS — `impedance_w_composition_oracle` on CPU and GPU readback |

## Gradient-only proof

| Proof | Test | Result |
|---|---|---|
| Local gradient probe | `gradient_probe_exposes_reach_without_route_object` | PASS — `palma_reach_gradient_probe` returns lowest-D N4 step |
| No route/path/predecessor object | — | yes |

## Forbidden route/path/predecessor proof

| Proof | Test | Result |
|---|---|---|
| No forbidden identifiers in definitions | `forbidden_route_path_predecessor_tokens_absent` | PASS |
| No route surfaces on pack | — | yes — `palma_feedstock` only; no path objects |

## No CPU planner proof

| Proof | Result |
|---|---|
| No attack/defend/raid/withdraw loop | yes — field seed + W compose + min-plus + oracle/probe only |
| No fleet reparenting / movement execution | yes |
| No Phase 7 commitments | yes |

## Candidate F / exact magnitude

PALMA min-plus uses tropical addition only (no sqrt). Exact-magnitude gate not load-bearing for this rung.

## Rustification / lifecycle

| Test | birth_track | class | verdict |
|---|---|---|---|
| `palma_reach_field_resident_on_gpu` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `impedance_w_composes_from_threat_and_choke_fields` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `gradient_probe_exposes_reach_without_route_object` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `palma_reach_gpu_matches_cpu_oracle` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `forbidden_route_path_predecessor_tokens_absent` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |

## Citable GPU proof

```
DOCTRINE-TESTS-VERDICT: PASS
tested_code_sha: 905fb35a0da5570564de747347e14b40a4f477df
current_pr_head: <pending evidence commit>
coverage_basis: PASS — commits after tested_code_sha are docs/evidence-only and do not affect the tested binary
profile: owner-local GPU / tp_palma_reach_0
owner_local: true
proof: palma_reach_gpu_matches_cpu_oracle
result: PASS
```

## Load-bearing proofs (owner-local 2026-07-05)

| Command | Result |
|---|---|
| `cargo check -p simthing-workshop` | PASS |
| `cargo check -p simthing-clausething` | PASS |
| `cargo test -p simthing-workshop --test tp_palma_reach_0 -- --nocapture` | PASS (5/5) |
| `test_inventory_check` | INSPECT (2 pre-existing fixture extra rows; 0 missing) |
| `test_inventory_drift_check` | PASS |
| `test_lifecycle_boundary_check` | PASS |
| `test_lifecycle_expiry_check --schema` | PASS |
| `test_lifecycle_expiry_check --prove` | PASS |
| `gen_digest --check` | PASS |
| `doctrine_scan` | PASS (0 hard failures; 415 HEURISTIC INSPECT — pre-existing corpus) |
| `git diff --check origin/master...HEAD` | PASS |

## Known gaps / next

- Fleet movement (`TP-FLEET-MOVEMENT-0`): not started — blocked.
- Phase 6.2: blocked pending DA clearance of 6.1.

## Graduation routing

DA may graduate 6.1 when all handoff gates are green under owner review. Merge remains **held** until DA/Owner clearance.