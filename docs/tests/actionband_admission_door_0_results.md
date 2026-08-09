# ACTIONBAND-ADMISSION-DOOR-0 results

- Track: 0.0.8.7 RF arena modernization (rung 7.1)
- Status: **PROBATION / proof-present / DA-review-pending**
- Branch: `codex/actionband-admission-door-0`
- base_sha: `ebff7ef15c0287960958d2997bcd1d1d14e6e9b5`
- implementation_code_sha: `34d8b06690a23503289a969773669c3788fe476c`
- tested_code_sha: `34d8b06690a23503289a969773669c3788fe476c`
- final_head_sha: PR/board-relay-bound after the evidence commit; this file does not self-hash
- HD-RECEIPT: `b730ee57b576`
- ORIENT-RECEIPT: `3b3b8c42b4e7`
- orientation_rule_stamp: `8acaf97ae0e6037b`
- orientation_digest_sha: `ca1c81792340264563b5a68bf98e4413e43c5d4189192514821c95d33a31213f`
- expected_route: `DA-RESERVE(gate-wiring)`
- coverage_basis: **PASS** — focused production admission/crossing referees at the implementation identity; affected-package and exact-final-head gate evidence is bound in the PR and board relay

## What changed

`simthing-spec` now owns one stateful session-build admission door. It validates an authored ActionBand session specification and freezes private, read-only numeric tables with stable template indices and spans for channels, bands, subordinate dependencies, existing EML programs, pre-admitted emission bindings, existing threshold registrations, crossing metadata, storage reservations, and a physically separate semantic shadow.

The target schema is a closed seven-variant vocabulary. Admission rejects predicate-only EML sets, unretained velocity, non-anchored or undeclared columns, cap overruns, a second admission through the same session door, and pre-8.x atomic/persistent scarce-lane requirements. No numerical ActionBand execution landed in this rung.

Both held `HORIZON-ENTRY(2026-08-08)` markers were removed from `crates/simthing-kernel/src/decision_ingress.rs` and `crates/simthing-sim/src/overlay_lifecycle.rs`; the admitted binding to the existing sealed crossing surface is their consumer.

## Load-bearing proofs and TEST-BUDGET INSPECT

All five tests are admitted permanent production-path referees. They share one integration target to keep compile cost bounded; none is a test-only executor, intentionally broken fixture, or `#[cfg(test)]` production twin.

| Test | Why admitted / defect caught |
|---|---|
| `depth_one_template_binds_the_existing_sealed_crossing_path` | Required depth-1 exit proof: the existing CPU oracle for the sealed GPU threshold surface emits the real `BandCrossingDelta`, whose `reg_idx` resolves immutable ActionBand metadata. It catches an unbound registration or a newly minted ActionBand crossing identity. |
| `closed_targets_admit_total_forms_and_reject_predicate_only_or_unretained_velocity` | Exercises all seven closed lowering descriptors and plants the two required production admission defects. It catches open/predicate-only target admission and hidden previous-state allocation. |
| `admission_freezes_axis_dependency_and_storage_budgets` | Exercises cached-channel charging, active-subordinate-span refusal, flattened dependency-cap refusal, and storage-cap refusal. It catches undeclared runtime widening and cached-channel budget evasion. |
| `session_door_refuses_mid_session_template_mint` | Calls the real one-shot door twice. It catches any runtime/mid-session template mint path through the session authority. |
| `pre_8x_scarce_lane_semantics_fail_closed_and_labels_stay_shadow_only` | Refuses both atomic common-depth and persistent scarce-grant holding while proving labels remain outside numeric templates. It catches premature 8.x semantics and human-readable dispatch authority. |

The rival crossing path is unconstructible at the admitted production boundary: `ExistingThresholdRegistrationIndex` and every crossing-binding field have private constructors, admission mints them only after validating an existing `EmitOnThresholdRegistration`, and the product exposes metadata lookup only. There is no ActionBand comparator, crossing record, listener, CPU decision evaluator, or effect-authorizing method.

## Local evidence at the implementation identity

| Command | Result |
|---|---|
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test actionband_admission_door_0` | PASS — 5 passed, 0 failed |
| `cargo test -p simthing-spec` | PASS — all 44 unit, integration, and doctest cases |
| `cargo test -p simthing-kernel` | PASS — all 79 unit, integration, and doctest cases, including the sealed crossing referees |
| `cargo test -p simthing-sim` | PASS — all 42 unit, integration, and doctest cases |
| forbidden-vocabulary grep over the three new ActionBand files | PASS — zero movement/path/planner/crossing-detector/CPU-decision vocabulary |
| exact held-marker grep over the two consumer files | PASS — both markers absent |
| `bash scripts/ci/agent_scan.sh` | INSPECT — `failures=0 inspect=1`; sole finding is the expected `TEST-BUDGET` review of the five named referees above |
| `bash scripts/ci/anchor_check.sh --check` | PASS |
| `python scripts/ci/exit_proof_coverage_check.sh` | INSPECT — one inherited unrelated TODO-row finding on rung 11.2 |
| `bash scripts/ci/test_inventory_drift_check.sh` | FAIL — the five required new referees are unledgered; the prescribed ledger is under forbidden `scripts/ci/**` scope and was not edited |

Affected-package, orientation, anchor, doctrine, agent, exit-proof, hosted Doctrine Scan, and hosted Doctrine Exec results are recorded against the exact final evidence head in the PR and board relay so the repository evidence packet does not claim a self-referential SHA.

## Scope Ledger

| Surface | Purpose |
|---|---|
| `crates/simthing-spec/src/spec/action_band.rs` | Authored closed target forms, bands, channels, requirements, and declared session caps. |
| `crates/simthing-spec/src/compile/action_band_admission.rs` | Sole session-build door and immutable numeric admission product. |
| `crates/simthing-spec/src/{lib.rs,spec/mod.rs,compile/mod.rs}` | Public module/export wiring only. |
| `crates/simthing-spec/tests/actionband_admission_door_0.rs` | Five load-bearing exit-proof referees. |
| `crates/simthing-kernel/src/decision_ingress.rs`, `crates/simthing-sim/src/overlay_lifecycle.rs` | Remove the two consumed held markers only. |
| `docs/tests/actionband_admission_door_0_results.md` | This probation evidence packet. |
| `docs/design_0_0_8_7_rf_arena_modernization.md` | Only rung 7.1 moves from TODO to PROBATION. |
| `docs/orchestrator_orientation.md` | Deterministic regeneration after the 7.1 status update. |

No file under `scripts/ci/**` or `.github/workflows/**` was edited. Existing `designer_admission/mobility_*`, Movement-Front vocabulary, and successor rungs remain untouched.

## Conformance and anchor acknowledgements

The implementation was projected before edits and rechecked over the final planned path set. `movement-front-adjudications` was applied as law: PALMA reach remains a min-plus field consumed at threshold crossings, not a route, predecessor tree, `came_from` relation, or path object.

- ANCHOR-ACK: `accumulator-exact-vs-soft-semantics@0efceafc77cf`
- ANCHOR-ACK: `accumulator-op-v2-invariants@32fb4fc36080`
- ANCHOR-ACK: `actionband-8x-sequencing@52a1faeb85b5`
- ANCHOR-ACK: `actionband-axis-budget@6736b5f1d420`
- ANCHOR-ACK: `actionband-binding-laws@030bb13655df`
- ANCHOR-ACK: `actionband-constitutional-placement@a2d82cc70716`
- ANCHOR-ACK: `actionband-crossing-surface@79a5366b0247`
- ANCHOR-ACK: `actionband-eml-payload-purity@fe43cb1c07cf`
- ANCHOR-ACK: `actionband-executive@9a8e0c500c49`
- ANCHOR-ACK: `actionband-fenced-questions@54276c7829fa`
- ANCHOR-ACK: `actionband-gpu-physical-model@f324b18cd960`
- ANCHOR-ACK: `actionband-target-forms@92de7a7eec5b`
- ANCHOR-ACK: `actionband-vendorization-direction@e401f0220cd8`
- ANCHOR-ACK: `admission-ladder-necessity-test@4bedf826f6f7`
- ANCHOR-ACK: `candidate-f-exhaustive-proof-method@7c5ce0b93dab`
- ANCHOR-ACK: `core-gpu-residency@8db4198cbc29`
- ANCHOR-ACK: `core-property-value-model@17cd41a567b7`
- ANCHOR-ACK: `eml-admission-shapes@bdcc0b9512f7`
- ANCHOR-ACK: `eml-extension-ladder@7755bc72ffbe`
- ANCHOR-ACK: `eml-integration-plan@8eba54b02320`
- ANCHOR-ACK: `eml-triad-integration@dada7d680557`
- ANCHOR-ACK: `evaluation-identity-invariants@64ad30392930`
- ANCHOR-ACK: `exact-numeric-candidate-f@6938a2efadb5`
- ANCHOR-ACK: `field-policy-time-decisions@993c7d0560e8`
- ANCHOR-ACK: `field-sweep-preservation@acc521a5a361`
- ANCHOR-ACK: `intrinsic-constrained-clearing@957b7c81b756`
- ANCHOR-ACK: `movement-front-adjudications@5af6a29acb75`
- ANCHOR-ACK: `one-tree-owners-never-spatial@a8689d4344f9`
- ANCHOR-ACK: `rf-arena-allocation-invariants@82864469489b`
- ANCHOR-ACK: `rf-arena-substrate@17b5f1e5c2ba`
- ANCHOR-ACK: `seal-residue-cross-crate@49ee7c4ba6f4`
- ANCHOR-ACK: `simthing-0087-binding-laws@8f13cba4aa7a`
- ANCHOR-ACK: `simthing-0087-pillars@61487cba1f9e`
- ANCHOR-ACK: `stead-rejected-shapes@3752549ff106`
- ANCHOR-ACK: `stead-shared-surface-ledger@87eaa1e7bb9c`
- ANCHOR-ACK: `stemthing-binding-laws@6787a118c3ca`
- ANCHOR-ACK: `stemthing-lane-not-leg@a9e9caa27a0f`
- ANCHOR-ACK: `stemthing-slot-identity-ruling@02c87b9126e1`
- ANCHOR-ACK: `workshop-candidate-homing@3e584f0ad175`

## Known gaps / next

- This rung admits descriptor data only. GPU numerical execution, recursive runtime composition, claims/clearing, and every 7.2+ or 8.x execution surface remain TODO.
- Coding does not invoke `/clearance`, merge, graduate, move the workplan pointer, or dispatch a successor.
- `TEST-INVENTORY-DRIFT-CHECK` reports the five required new integration tests as unledgered. Its prescribed remedy edits `scripts/ci/test_inventory.tsv`, but this handoff forbids every `scripts/ci/**` edit. Coding stopped at that authority boundary and reports the blocker rather than modifying CI.

## Graduation routing

- CI verdict: **FAIL** locally on test-inventory admission; semantic tests are green and doctrine scan is `INSPECT failures=0 inspect=1`
- Triage entries: `TEST-BUDGET:INSPECT` — five named permanent production-path referees justified above; `EXIT-PROOF-COVERAGE:INSPECT` — inherited unrelated rung 11.2; `TEST-INVENTORY-DRIFT-CHECK:FAIL` — five new rows require forbidden `scripts/ci/**` authority
- Risk class: `gate-wiring` — constitutionally routed to `DA-RESERVE(gate-wiring)` despite zero gate edits
- Falsification check: reproduce the five named referees, confirm the crossing is an existing sealed `BandCrossingDelta`, confirm both held markers and every rival ActionBand crossing/CPU authority are absent, and verify nonzero doctrine `inspect` with `failures=0` is PASS
- Recommended posture: **deep** — the new constitutional admission/type boundary controls later GPU execution and must be DA-reviewed before graduation
