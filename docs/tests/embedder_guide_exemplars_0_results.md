# EMBEDDER-GUIDE-EXEMPLARS-0 results

- Track: 0.0.8.7 RF arena modernization (rung 11.2)
- Status: **PROBATION / proof-present / DA-review-pending**
- Dispatch: board comment `5381072419`
- Canonical handoff: `handoffs/EMBEDDER-GUIDE-EXEMPLARS-0.hd.md`
- HD-RECEIPT: `890b845b5655`
- ORIENT-RECEIPT: `a5dc59920dd4`
- Orientation rule stamp: `61818ff7d4adda84`
- ANCHOR-ACK: `orientation-harness-core@8a365d1c0864`
- ANCHOR-ACK: `scanner-selftest-delta-gate@34fb2662baae`
- Expected route: `DA-RESERVE(gate-wiring)`
- Coding branch: `coder/embedder-guide-exemplars-0`
- Canonical base (handoff `base_sha`): `fc313a21e95d0853e93d2e3680ddc671f03bac4b`
- Live PR base (current master at dispatch): `680e2d423101ba758050fa90d5ed424a46f32d2d`
- tested_code_sha: `06c1595d787ccff23e3865cb8d23197b0d714e78`
- Pointer: 11.2 only. No merge, no pointer movement, no 11.3/12.x.

## Archaeology (first step)

Read `crates/simthing-embedder/src` and `tests/vendor_door_0.rs` before writing
the guide. The five-verb door already exists. The guide and exemplars call it.

The door did not re-export every type a cold reader needs to *call* the verbs
without an engine-crate `use`. Adding those re-exports on the verb modules is
onboarding surface, not a new evaluator, authority, opcode, or simulation path.
No engine crate was edited. No scenario pack was wired. `POW` was not minted.

## Surfaces

| Path | Role |
|---|---|
| `docs/embedders_guide.md` | DOC-BUDGET-capped cold-reader five-verb guide (103 / 120 lines) |
| `crates/simthing-embedder/tests/finance_toy_0.rs` | finance-toy exemplar: Derive → Populate → Overlay → Bind → Run |
| `crates/simthing-embedder/tests/network_saturation_triad_0.rs` | network-saturation full-Triad + authored `exp(k * ln x)` law |
| `crates/simthing-embedder/src/{populate,overlay,bind}.rs` | re-export existing types/primitives needed to call the door |
| `scripts/ci/embedder_guide_exemplars_check.sh` | CI admission gate (no cargo) |
| `scripts/ci/fixtures/embedder_guide/known_bad_staircase.rs` | planted staircase defect |
| `scripts/ci/doctrine_scan.sh` / `.github/workflows/doctrine-scan.yml` | stock-gate + GHA check/selftest pair |
| `scripts/ci/doc_budget_baseline.tsv` | new row `docs/embedders_guide.md` cap 120 |
| `scripts/ci/test_inventory.tsv` | three exemplar rows + planted-fixture row |

## Five-verb teaching order

Derive, Populate, Overlay, Bind, Run. Every rust fence in the guide is a
verbatim substring of an exemplar that runs. Cited paths:

- `crates/simthing-embedder/tests/finance_toy_0.rs`
- `crates/simthing-embedder/tests/network_saturation_triad_0.rs`

## Authored law

Volume-delay is `1 + 0.15 * (v/c)^4`, composed as
`eml_exp_pinned_f32(4.0 * eml_ln_pinned_f32(ratio))`. At ratio `2.0` the law
is `3.4`; the piecewise rival is `2.5`; the bits disagree. No `POW` opcode.

Need / corridor / front / chokepoint are ordinary Bind velocity and aggregate
thresholds over tree + overlay-born values. They are not hand-fed readouts.

## Biting falsifiers

| Planted rival | Named RED |
|---|---|
| Staircase / piecewise volume-delay without EXP/LN composition | `FAIL(authored-law-staircase)` |
| Engine-crate `use` in an exemplar | `FAIL(door-import:<crate>)` |
| Guide cites a missing exemplar path | `FAIL(guide-path:...)` |
| Guide rust fence not present in an exemplar | `FAIL(guide-drift:...)` |
| Staircase bits equal to `exp(k * ln x)` at ratio 2.0 | cargo assert in `volume_delay_power_law_reds_a_staircase_rival` |

Renaming a local symbol does not trip the gate. Replacing the law with a
staircase does.

## Evidence

| Command / proof | Result |
|---|---|
| `cargo test -p simthing-embedder --test finance_toy_0 --test network_saturation_triad_0 --test vendor_door_0 -- --test-threads=1` | PASS — 1 + 2 + 6 |
| `embedder_guide_exemplars_check.sh --check` | `EMBEDDER-GUIDE-EXEMPLARS-VERDICT: PASS` |
| `embedder_guide_exemplars_check.sh --selftest` | live_shape / staircase / door_import / guide_path PASS |
| `doc_budget_check.sh --check` | PASS |
| `test_inventory_drift_check.sh` | PASS (1307/1307) |
| `test_lifecycle_expiry_check.sh --schema` | PASS |
| Detachability | PASS `production_coupling=0 proof_coupling=0 ceiling=0` |
| `gen_orientation.sh --check` | PASS |
| `gen_digest.sh --check` | PASS |
| `lifecycle_schema_pr_gate.sh 680e2d42..06c1595d` | PASS |
| `agent_scan.sh` at `06c1595d` | PASS `delta_inspect=0` `DOCTRINE-SCAN-VERDICT: PASS failures=0 inspect=0` |
| Hosted Doctrine Scan | pending push |

## Scope disposition

Return **PROBATION / proof-present / DA-review-pending**. Coding does not invoke
`/clearance` or `/relay-lint`, merge, graduate, move the pointer, or begin 11.3.
