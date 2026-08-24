# ACCUMULATOR-CONVERGENCE-SEAL-0 results

- Track: 0.0.8.7 RF arena modernization (remedial 11.1d)
- Status: **COMPLETE — DA-GRADUATED / merged #1819 @ `7ee2350d`** (Fable deep review, graduation ruling on Board #1332)
- Branch: `codex/accumulator-convergence-seal-0`
- Reconciled live-master base: `1e8650286d691ced0fc55824861a2fc415802011`
- Implementation / tested_code_sha: `337383e4bba5804d0ecb9f843c9a792a56097696`
- Evidence-only final head and hosted workflow IDs: bound in the PR body and board return
- HD-RECEIPT: `28067af2cdab`
- ORIENT-RECEIPT: `a5dc59920dd4`
- Orientation rule stamp: `61818ff7d4adda84`
- Orientation digest: `8bb010feac86eaa7346b7ce75c97f415afd3c58bb3220783a301b1da57825aea`
- Expected route: `DA-RESERVE(gate-wiring)`

## Archaeology and dispositions

| Surface | Pre-edit state | Disposition |
|---|---|---|
| C-8c transfer | `PipelineFlags` selected the default-off AccumulatorOp upload/dispatch path; authored economy mode and session helper could turn it on. No alternate CPU transfer executor remained in production. | Flag, branch, authored selector, helper, and enabled-state telemetry deleted. Transfer content now uploads through the sole AccumulatorOp path. |
| C-8d emission | A second default-off flag selected the AccumulatorOp emission path and depended on the existing EML substrate. No alternate CPU emission executor remained in production. | Flag, branch, authored selector, helper, and enabled-state telemetry deleted. Emission content now uploads through the sole AccumulatorOp path without changing EML vocabulary. |
| E-11 arena Resource Flow | A default-off flag plus two authored selectors chose whether the admitted arena plan reached AccumulatorOp. Flag-source telemetry and four selector-only burn/soak wrappers described the split. | Flag, both authored selectors, session resolver, flag-source telemetry, and selector-only wrappers deleted. Generic burn-in and dynamic-enrollment proof moved to `resource_flow_convergence_burn_in`; ordinary session state alone controls dispatch. |
| Authored data | Retired selector keys occurred only in repository fixtures; the shipped-data census found no external migration requiring a STOP. Clause hydrators accepted both economy and Resource Flow selector forms. | Fixtures migrated. JSON/RON structs reject retired fields as unknown; ClauseThing rejects the retired authoring blocks/keys as unsupported. |
| Existing seals | Overlay-add and governed velocity already had deletion-shaped seals and independent flags. | Flags and behavior unchanged; the complete `simthing-sim` package battery remains green. |

The final production and authored-fixture census is mechanized by
`production_and_authored_fixtures_have_no_retired_selector_vocabulary`.
It recursively scans the production/fixture surface for all three deleted flag
names, both retired mode types, the execution-profile type/key, both selector
helpers, and E-11 opt-in vocabulary. The planted admission inputs reconstruct
their retired keys at runtime, so the referee itself cannot become an allowed
textual residue. The census reports zero residues under `crates/`.

## Load-bearing proofs

| Proof | Result / defect caught |
|---|---|
| Transfer authored RED | A transfer workload carrying the retired selector key fails `ResourceEconomySpec` admission with `unknown field`; it cannot reconstruct a bypass. |
| Emission authored RED | An emission workload carrying the retired selector key fails independently at the same typed admission boundary. |
| Resource Flow authored RED | An arena workload carrying the retired Resource Flow selector fails `ResourceFlowSpec` admission. |
| Execution-profile authored RED | A game mode carrying the retired Resource Flow execution-profile key fails `GameModeSpec` admission rather than being parse-ignored. |
| Standing absence proof | Zero retired selector/flag/helper residues across production Rust and authored RON/Clause/JSON/YAML/TOML fixtures. |
| C-8c exactness | Existing single-source conservation and conjunctive min-across-inputs GPU/oracle referees pass 2/2 before deletion and at the tested code SHA. |
| C-8d exactness | Existing exact/soft admission and overflow-contract referees pass 3/3 before deletion and at the tested code SHA. |
| E-11 exactness and enrollment | Sparse owned rows and dynamic fission pass 2/2 on the real adapter: exact allocations `[4,8]`, RF-1, bit-exact replay, one generation bump, and three stable resyncs. |
| E-11 carried burn/soak | Static-10, skewed, dynamic multi-fission, two-arena, 100-cycle resync, and 512-participant cases run 4,110 GPU ticks with zero max error; a separate two-session replay pair is report-identical. |
| Hydrated authored route | CT2A passes on the converged path with RF-1. CT2C remains the same pre-existing live-master `ValidationFailedAt { site: "simthing-driver/install" }` baseline RED, reproduced unchanged in a detached baseline worktree. |

## Exact-head verification

All passing commands below ran at tested code SHA
`337383e4bba5804d0ecb9f843c9a792a56097696`.

| Command | Result |
|---|---|
| `cargo check --workspace --all-targets` | PASS |
| `cargo test -p simthing-driver --test accumulator_convergence_seal_0 -- --nocapture` | PASS — 3/3; four table-driven authored RED cases, standing absence, and E-11 carried soak |
| `cargo test -p simthing-sim --test c8c_transfer_accumulator_parity` | PASS — 2/2 |
| `cargo test -p simthing-sim --test c8d_emission_accumulator_parity` | PASS — 3/3 |
| `cargo test -p simthing-driver --test arena_participant_elimination_0 -- --nocapture` | PASS — 2/2; `NVIDIA GeForce RTX 4080 Laptop GPU`, Vulkan, DiscreteGpu |
| `cargo test -p simthing-clausething --test ct_2a_intrinsic_flow -- --nocapture` | PASS — 1/1; real adapter |
| `cargo test -p simthing-sim` | PASS — 45 tests across all package suites |
| `cargo test -p simthing-driver --lib` | PASS — 16/16 |
| `cargo test -p simthing-spec --lib` | PASS — 13/13 |
| `cargo test -p simthing-clausething --test field_economy_grammar_0` | PASS — 4/4 |
| inventory check + drift prove + lifecycle schema | PASS — 1,313 inventory rows / 1,313 discovered; zero expired |
| execution-status taxonomy scan + diff check | PASS |

The structural certificate is intentionally owed to graduation, per the DA A1
and handoff exit proof; coding does not claim it at probation.

## Anchor and scope conformance

The sanctioned path query passed with 46 anchors and the three sticky/initial
required anchors were separately re-rendered. Exact reach receipts are in
`scripts/ci/anchor_reach_log.tsv`. Load-bearing acknowledgements include:

- `accumulator-op-v2-invariants@32fb4fc36080`
- `rf-arena-allocation-invariants@82864469489b`
- `rf-arena-substrate@17b5f1e5c2ba`
- `structural-execution-convergence@6b4cedec482b`
- `field-sweep-preservation@acc521a5a361`
- `actionband-field-triad-authority@56cf5cdf2d2c`
- `orientation-harness-core@8a365d1c0864`
- `scanner-selftest-delta-gate@34fb2662baae`
- `workshop-candidate-homing@3e584f0ad175`

No new executor, CPU resolution path, history, contention mechanism, EML or
ActionBand vocabulary, Vendor Door semantic, 11.2 artifact, pointer movement,
merge, graduation, clearance, or relay-lint action is included. Return
**PROBATION / proof-present / DA-review-pending** through
`DA-RESERVE(gate-wiring)` with deep review recommended because the change is a
production-path deletion seal.
