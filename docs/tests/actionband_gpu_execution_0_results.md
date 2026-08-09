# ACTIONBAND-GPU-EXECUTION-0 results

- Track: 0.0.8.7 RF arena modernization (rung 7.2)
- Status: **REMAND-RESPONSE / DA-review-pending**
- Branch: `codex/actionband-gpu-execution-0`
- base_sha: `afcedfcfc3d0a04c15d11d0e55ac2551a9d2ed0a`
- HD-RECEIPT: `52bfd6f2c34d`
- ORIENT-RECEIPT: `3b3b8c42b4e7`
- orientation_rule_stamp: `8acaf97ae0e6037b`
- expected_route: `DA-RESERVE(gate-wiring)`

## Product

The frozen 7.1 product now lowers once into flat, domain-free template, target,
band, EML, binding, and sparse active-instance tables. Numerical state is held
in distinct private GPU `StateCurrent` and `StateNext` buffers. One dispatch
reads current and writes next; a post-submit buffer-owner swap advances the
generation without same-generation recursion.

Target evaluation covers the seven already-admitted forms with total
`satisfied` and projection/distance output. Point and AABB projections are
componentwise, scalar/interval projections are closed form, PALMA remains a
distance-field threshold consumer, and projected EML sets execute the admitted
membership/projection programs on GPU. The shader reuses the canonical existing
EML evaluator source and therefore preserves its pinned exact semantics.
The production `dispatch` path leaves numerical state and projection resident
on GPU and advances buffer ownership without copying either surface to CPU.
Numerical readback exists only on the separately proof-gated
`dispatch_and_readback` path. The only production readback is the already-sealed
threshold consequence packet needed at an existing CPU boundary.

Crossing work can be obtained only by joining the immutable plan to existing
sealed `BandCrossingDelta` rows. ActionBand has no comparator, crossing record,
listener, CPU evaluator, planner, scheduler, or callback. EML produces a scalar
payload only; destination kind/index are copied from an immutable closed binding
table. Property-next, RF-claim, and CostBand bindings write the existing
GPU-authoritative world-value surface directly; overlay-event,
structural-request, and telemetry bindings use the existing sealed
`ThresholdEmissionGpu` consequence surface. There is no ActionBand-local generic
emission record. GPU-authorized structural rows select a pre-admitted
`BoundaryRequest`, which the CPU submits to the existing feeder boundary without
inspecting or re-evaluating numerical state.

Inactive plans return the `Inactive` execution variant before any ActionBand GPU
buffer is created. Their hot rows and hot bytes are exactly zero. Active rows are
fixed-width and sorted by opaque `(template_index, slot)` identity; pointer
traversal and runtime allocation are absent. Program/binding buckets are formed
deterministically from numeric program and destination shape only, and each
bucket's sealed crossing range drives a distinct GPU dispatch. The admitted
current/previous velocity-column pair is frozen into the template table and
executed on GPU; its result is carried in GPU state and supplied to EML payloads.

## TEST-BUDGET INSPECT

All five tests live in one integration target and exercise production APIs and
the real GPU shader. They are admitted because each covers a distinct exit-proof
failure class:

| Test | Admission reason / defect caught |
|---|---|
| `sparse_gpu_state_ping_pongs_and_matches_exact_eml_oracle` | Runs production dispatch with proof readback disabled, then two proof generations; checks current/next progression, admitted velocity execution, exact GPU/CPU-referee EML parity, all six destination families on their existing surfaces, fixed structural output, boundary submission without CPU re-evaluation, and the repeated timestamp measurement. |
| `sealed_crossings_are_the_only_emission_ingress_and_destinations_stay_frozen` | Empty sealed evidence emits nothing; the existing Phase-5 delta joins exactly once. It catches a rival crossing input/record or caller-authored destination. |
| `inactive_rows_allocate_zero_hot_storage_and_dense_mutant_is_red` | Proves zero active rows means zero hot bytes and plants a dense two-row allocation against a one-row frozen cap, which fails in production lowering. |
| `bucketing_is_numeric_deterministic_and_labels_are_semantic_shadow_only` | Same numeric product under different labels has the same fingerprint/buckets; equal program/binding shapes share a bucket, a distinct binding shape separates, and the resulting two numeric ranges execute as two production GPU dispatches. |
| `inherited_admission_and_cpu_authority_fences_remain_closed` | Rechecks the one-shot 7.1 door, consumed-marker absence, forbidden CPU authority vocabulary, and WGSL read-current/write-next qualifiers. |

## Repeated depth-1 measurement

Method: one active scalar-bound template, one admitted three-node payload program
(`PARAM(0) * 2`), one sealed crossing, and one fixed structural binding. After
warm-up, 15 ActionBand samples and 15 existing threshold-crossing samples were
recorded with adapter GPU timestamp queries. ActionBand time is the sum of its
target-evaluation and fixed-emission compute passes; CPU mapping/copy time is not
included. The existing baseline is the unchanged `AccumulatorOpSession`
threshold-scan pass on the same adapter and one-row/one-dimension shape.

Observed local median:

- ActionBand depth-1 GPU compute: **10,944 ns**
- Existing sealed crossing GPU compute: **5,000 ns**
- Ratio: **2.189x**

These are raw local engineering measurements, not a pass/fail threshold or a
broad hardware performance claim. Their disposition is reserved to DA. The
exact-head run and hosted artifacts are carried in the relay.

## Scope and inherited boundaries

- 7.1 remains frozen: one admission door, seven target forms, private existing
  threshold indices, sole `BandCrossingDelta`, and both prior horizon markers absent.
- 7.3 subordinate activation/composition, 7.4 vendorization, 7.5 semantic-shadow
  readback, 8.x scarce holding/clearing, and ActionBand-local RF machinery remain absent.
- No production movement vocabulary was introduced.
- No file under `.github/workflows/` was touched. No CI gate code was edited.
  `scripts/ci/anchor_reach_log.tsv` and `scripts/ci/test_inventory.tsv` are only
  append-only data-ledger updates earned by required anchor reads and five tests.

## Load-bearing anchor acknowledgements

- `actionband-gpu-physical-model@f324b18cd960`
- `actionband-eml-payload-purity@fe43cb1c07cf`
- `actionband-crossing-surface@79a5366b0247`
- `actionband-target-forms@92de7a7eec5b`
- `actionband-binding-laws@030bb13655df`
- `actionband-8x-sequencing@52a1faeb85b5`
- `structural-execution-convergence@6b4cedec482b`
- `movement-front-adjudications@5af6a29acb75`
- `one-tree-owners-never-spatial@a8689d4344f9`

The complete path-projected anchor ACK set, exact-head batteries, doctrine
verdict, hosted checks, and graduation-routing block are carried in the PR and
board relay.
