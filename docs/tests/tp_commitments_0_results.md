# TP-COMMITMENTS-0 Results

## Status

**PROBATION pending orchestrator §5A check.**

## Identity

| Field | Value |
|---|---|
| PR | (pending push) |
| Branch | `tp-commitments-0` |
| Base | `origin/master` @ `7d44037e1932ac225285ff8eb776f24de394b890` |
| Tested code SHA | `ca19e956571d451d97d20c37f6cf0627f40b0fa9` |
| Current PR head | (pending push) |
| Rung | Phase 7.0 `TP-COMMITMENTS-0` |
| Mechanism | **B — consumer-side application** from `simthing-workshop` |

## §5A routing classification

| Check | Result |
|---|---|
| Diff limited to workshop/evidence/inventory/boundary + design doc | PASS — design doc classified as Owner-mandated hygiene addendum |
| No Homing Boundary exception | PASS |
| No substrate widening appeal | PASS |
| No new opcode/WGSL/AccumulatorRole | PASS |
| tested_code_sha coverage basis clean | PASS |
| Lifecycle correct, drift PASS, tracks untouched | PASS |
| Doctrine Scan green at head (0 hard failures) | PASS |
| Citable owner-local GPU proof | PASS |
| Every INSPECT delta has landed /triage row | PASS — pre-existing 415 HEURISTIC corpus |
| Binding DA conditions | none |

**Routing:** ORCHESTRATOR MERGE-CLEARABLE under §0.9.7, with explicit DESIGN-DOC HYGIENE ADDENDUM for `docs/design_0_0_8_5_clausescript_terran_pirate_galaxy.md` (Owner-mandated Phase 4–6 exit-proof sync).

## Design-row exit-proof/status correction

| Item | Detail |
|---|---|
| File updated | `docs/design_0_0_8_5_clausescript_terran_pirate_galaxy.md` |
| Phase 4+ rows corrected | `TP-COMBAT-ARENA-0`, `TP-DIPLOMACY-FLOW-0`, `TP-FRONTS-AUTHORING-0`, `TP-PALMA-REACH-0`, `TP-FLEET-MOVEMENT-0`, `TP-COMMITMENTS-0` |
| PR numbers / merge SHAs reflected | #1145 `a54695ec`, #1150 `9aa66c39`, #1151 `9f56794a`, #1152 `335f55c0`, #1154 `7d44037e` |
| Citable proof docs reflected | `tp_*_0_results.md` links per row |
| Current TP-COMMITMENTS-0 state | PROBATION / proof-pending |
| Drift-prevention note added | Anti-drift rule in Phase 7 exit-proof cell |
| Routing impact | Combined in implementation PR; classified as DESIGN-DOC HYGIENE ADDENDUM |

## Mechanism

| Stage | Location | Notes |
|---|---|---|
| Fleet movement theater | accepted `apply_fleet_movement_post_hydration` | 7×7 fronts + PALMA base |
| Commitment authoring | `commitments_post_hydration.rs` | Per-faction `ai_will_do` weight profiles |
| Test driver | `tp_commitments_0.rs` | GPU threshold scan + boundary materialization |

**Pipeline:** resolved L2 pressure → `field_urgency` EML → Threshold + EmitEvent → CPU consumes `BoundaryRequest::AttachOverlay` at boundary only.

## Homing Boundary Classification

| Symbol / path | Would this exist without TP? | Classification | Action |
|---|---:|---|---|
| `apply_commitments_post_hydration` | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `TpPersonalityUrgencyProfile` / faction specs | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| `tp_commitments_0.rs` test driver | no | workshop-homed scenario candidate code | keep in `simthing-workshop` |
| design-row exit-proof/status update | yes | docs-hygiene / Owner-mandated status sync | keep — classified addendum |
| engine crate source | — | none | no edits |

Engine source edits: **none.** Generic substrate widening: **none.** Gameplay semantics in engine crates: **no.**

## Substrate widening

| Item | Status |
|---|---|
| Engine crates touched | **none** |
| Generic future-utility justification | n/a |
| Gameplay semantics in engine crates | **no** |

## Commitment definitions

| Faction | Type | Event kind | Threshold | Personality weights |
|---|---|---|---|---|
| Terran | `reinforce` | `0x52454E46` | 4.0 | pressure=0.40, resource=1.0 |
| Pirate | `raid` | `0x52414944` | 4.0 | pressure=1.0, resource=0.20 |

Taxonomy also defines `attack`, `withdraw`, `fortify` event kind constants for future crossings; this rung proves `reinforce` + `raid`.

## Terran commitment proof

| Proof | Test | Result |
|---|---|---|
| Reinforce fires from L3 crossing | `terran_commitment_fires_from_l3_pressure_crossing` | PASS |
| Pressure source | suppression + threat arena seeds | PASS |
| Not scripted timer | threshold only after GPU urgency computed | PASS |

## Pirate commitment proof

| Proof | Test | Result |
|---|---|---|
| Raid fires from L3 crossing | `pirate_commitment_fires_from_l3_pressure_crossing` | PASS |
| Pressure source | disruption + threat arena seeds | PASS |
| Not faction branch | personality weights only | PASS |

## ai_will_do / ai_weight urgency proof

| Proof | Test | Result |
|---|---|---|
| Urgency responds to pressure | `ai_will_do_urgency_changes_with_pressure_inputs` | PASS |
| Personality divergence | terran vs pirate profiles differ on same seeds | PASS |

## Threshold / EmitEvent / BoundaryRequest proof

| Proof | Test | Result |
|---|---|---|
| GPU EmitEvent | threshold_events from `tick_with_commitment_spec_fixture` | PASS |
| BoundaryRequest only | `commitment_event_is_boundary_request_not_cpu_planner` | PASS — `AttachOverlay`, not CPU planner |

## No CPU planner proof

| Proof | Test | Result |
|---|---|---|
| Forbidden identifiers absent | `forbidden_cpu_planner_commitment_tokens_absent` | PASS |
| No CPU urgency traversal | workshop source scan | PASS |
| No CPU commitment emission | workshop source scan | PASS |

## GPU proof

```
DOCTRINE-TESTS-VERDICT: PASS
tested_code_sha: ca19e956571d451d97d20c37f6cf0627f40b0fa9
current_pr_head: (pending evidence commit)
coverage_basis: PASS — commits after tested_code_sha are docs/evidence-only and do not affect the tested binary
profile: owner-local GPU / tp_commitments_0
owner_local: true
proof: terran_commitment_fires_from_l3_pressure_crossing + pirate_commitment_fires_from_l3_pressure_crossing
result: PASS
```

## Rustification / lifecycle

| Test | birth_track | class | verdict |
|---|---|---|---|
| `terran_commitment_fires_from_l3_pressure_crossing` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `pirate_commitment_fires_from_l3_pressure_crossing` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `ai_will_do_urgency_changes_with_pressure_inputs` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `commitment_event_is_boundary_request_not_cpu_planner` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `forbidden_cpu_planner_commitment_tokens_absent` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |

All new tests: `dsu_survivals = 0`.

## Load-bearing proofs (owner-local 2026-07-05)

| Command | Result |
|---|---|
| `cargo check -p simthing-workshop` | PASS |
| `cargo check -p simthing-clausething` | PASS |
| `cargo test -p simthing-workshop --test tp_commitments_0 -- --nocapture` | PASS (5/5) |
| `test_inventory_check` | INSPECT (2 pre-existing fixture extra rows; 0 missing) |
| `test_inventory_drift_check` | PASS |
| `test_lifecycle_boundary_check` | PASS |
| `test_lifecycle_expiry_check --schema` | PASS |
| `test_lifecycle_expiry_check --prove` | PASS |
| `gen_digest --check` | PASS |
| `doctrine_scan` | PASS (0 hard failures; 415 HEURISTIC INSPECT — pre-existing) |
| `git diff --check origin/master...HEAD` | PASS |

## Known gaps / next

- Phase 8 full transpile (`TP-FULL-TRANSPILE-0`): blocked until Phase 7 graduation.
- Live run (`TP-LIVE-RUN-0`): not started.
- Track closeout (`TP-DA-CLOSEOUT-0`): not started.

## Graduation routing

**Verdict:** ORCHESTRATOR MERGE-CLEARABLE under §0.9.7.

**Merge rationale:** Conforming workshop-homed Mechanism B rung with no engine edits, no opcode/WGSL widening, no DA reserve items, and full owner-local GPU proof on threshold crossings. Design-doc exit-proof sync is an explicit Owner-mandated hygiene addendum for Phase 4–6 graduation state — not hidden as ordinary evidence.

**DA notification:** Phase 7.0 proves per-faction STEAD commitments (`reinforce`/`raid`) from L3 `field_urgency` threshold crossings over the accepted 7×7 movement theater; CPU consumes `BoundaryRequest::AttachOverlay` at boundary only. No CPU planner. Design ladder exit-proof cells for Phases 4–6 are now synchronized with merged PR/SHA/proof-doc state; anti-drift rule embedded in Phase 7 row.