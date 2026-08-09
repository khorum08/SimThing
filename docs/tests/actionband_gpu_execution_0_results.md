# ACTIONBAND-GPU-EXECUTION-0 results

- Track: 0.0.8.7 RF arena modernization (rung 7.2)
- Status: **REMAND-RESPONSE / DA-review-pending**
- Branch: `codex/actionband-gpu-execution-0`
- base_sha: `80a3ea8c1db54d33cfd3eccc2f82cf1fd294520c`
- HD-RECEIPT: `52bfd6f2c34d`
- ORIENT-RECEIPT: `d676e8577e1b`
- orientation_rule_stamp: `3422182133007944`
- expected_route: `orchestrator review -> DA review`

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
Numerical state readback exists only on the separately proof-gated
`dispatch_and_readback` path. The sparse production egress is the unchanged
`StructuralCommitment` packet minted through the existing sealed
threshold/emission/boundary token chain.

Crossing work can be obtained only by joining the immutable plan to existing
sealed `BandCrossingDelta` rows. ActionBand has no comparator, crossing record,
listener, CPU evaluator, planner, scheduler, or callback. EML produces a scalar
payload only. Rung 7.2 admits exactly one structural binding per band;
property-next, RF-claim, CostBand, overlay-event, and telemetry bindings fail
closed as deferred to their owning rungs. The shader performs no world-value
write. Its internal `ThresholdEmissionGpu.reg_idx` remains the original threshold
registration identity and carries the original crossing slot/column, never an
ActionBand binding index. Kernel code verifies that identity before minting the
unchanged `StructuralCommitment`.

The authorized generic application seam lives beside the existing structural
mutation authority in `simthing-sim`:
`StructuralCommitment -> pre-admitted BoundaryRequest -> FeederSender::submit_boundary
-> apply_structural_mutations`. CPU selection reads sealed `event_kind` only and
submits a session-fixed request. It does not inspect commitment value, slot, or
column and does not recompute EML, displacement, crossing, or destination. There
is no ActionBand-local generic emission record or boundary queue.

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
| `sparse_gpu_state_ping_pongs_and_matches_exact_eml_oracle` | Runs production dispatch with proof readback disabled, then two proof generations; checks current/next progression, admitted velocity execution, exact GPU/CPU-referee EML parity, sealed `StructuralCommitment` minting, real `Remove` application through the generic boundary authority, a RED CPU numeric re-derivation mutant, a RED binding-index overload, deferred non-7.2 destinations, and the repeated timestamp measurement. |
| `sealed_crossings_are_the_only_emission_ingress_and_destinations_stay_frozen` | Empty sealed evidence emits nothing; the existing Phase-5 delta joins exactly once. It catches a rival crossing input/record or caller-authored destination. |
| `inactive_rows_allocate_zero_hot_storage_and_dense_mutant_is_red` | Proves zero active rows means zero hot bytes and plants a dense two-row allocation against a one-row frozen cap, which fails in production lowering. |
| `bucketing_is_numeric_deterministic_and_labels_are_semantic_shadow_only` | Same numeric product under different labels has the same fingerprint/buckets; equal program/binding shapes share a bucket, a distinct EML program shape separates, and the resulting two numeric ranges execute as two production GPU dispatches. |
| `inherited_admission_and_cpu_authority_fences_remain_closed` | Rechecks the one-shot 7.1 door, forbidden CPU evaluator/planner/scheduler/local-queue vocabulary, WGSL read-current/write-next qualifiers, absence of raw value writes, preservation of threshold registration/locus identity, and absence of numeric selection in the generic structural door. |

## Repeated depth-1 measurement

Method: one active scalar-bound template, one admitted three-node payload program
(`PARAM(0) * 2`), one sealed crossing, and one fixed structural binding. After
five warm-up dispatches, 31 ActionBand samples and 31 existing
threshold-crossing samples were recorded on the same adapter/backend/run with
GPU timestamp queries; the statistic is the median. ActionBand time is the sum
of its target-evaluation and fixed-emission compute passes. CPU sealed join,
copies, maps, readback, boundary submission, and structural apply are excluded.
The existing baseline is the unchanged `AccumulatorOpSession` threshold-scan
compute pass on the same NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan adapter and
one-row/one-dimension shape. Its public timestamp accessor has 1,000 ns resolution.

Observed local median:

- ActionBand target evaluation median: **4,960 ns**
- ActionBand EML + fixed structural emission median: **5,088 ns**
- ActionBand total median: **9,920 ns**
- Existing sealed crossing GPU compute: **5,000 ns**
- Marginal delta: **4,920 ns**
- Ratio: **1.984x**

The ratio is consistent with the §15.2 physical path at this one-row size: the
target-evaluation gather costs about one dispatch floor and the entire marginal
delta is attributable to the requested EML + fixed-emission pass. There is no
second comparison, CPU re-derivation, duplicate field projection, or
emission-pass world gather. This tiny shape does not validate §15.6's scaling
prediction that sparse gathers dominate at production cardinality; fixed
dispatch cost masks that question here. These are raw local engineering
measurements, not a pass/fail threshold or broad hardware performance claim.

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
- `actionband-performance-model@adeea5dbaee7`
- `actionband-determinism-lifecycle@283367293fc1`
- `structural-execution-convergence@6b4cedec482b`
- `movement-front-adjudications@5af6a29acb75`
- `one-tree-owners-never-spatial@a8689d4344f9`

The complete path-projected anchor ACK set, exact-head batteries, doctrine
verdict, hosted checks, and graduation-routing block are carried in the PR and
board relay.
