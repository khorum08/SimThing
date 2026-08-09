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

The frozen 7.1 product now seals each band crossing binding to the actual
Phase-5 threshold column and `event_kind`, then lowers once into flat,
domain-free template, target, band, EML, binding, and sparse active-instance
tables. The driver returns one private source-bound compilation containing both
the GPU plan and the structural application provenance derived from those same
frozen rows. No API accepts a detached `(event_kind, binding)` tuple.

Numerical state is held in distinct private GPU `StateCurrent` and `StateNext`
buffers. Every ActionBand shader binds `StateCurrent` read-only, writes only
`StateNext`, and advances through the ordinary whole-buffer swap. The general
multiband/multichannel/velocity path evaluates every active row into next. The ordinary direct
single-channel, one-band, no-velocity depth-1 shape uses a crossing-triggered
fast entry instead: for a non-empty crossing batch a bounded GPU buffer copy
carries current rows to next, the fast shader overwrites only crossing rows in
next from the already-sealed `post_value`, and the same whole-buffer swap
advances the generation. This preserves non-crossing rows without evaluating
or gathering them. The entry performs total target projection, EML, and fixed
emission in one dispatch, with no target-evaluation dispatch or world-value
gather. With no sealed crossing it performs no carry, compute work, or swap.

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
-> apply_structural_mutations`. Its `event_kind -> request` map is derived only
from the source-bound compilation's frozen band -> threshold/event-kind and
band -> exact binding row -> structural destination chains. CPU application
reads sealed `event_kind` only and submits that session-fixed request. It does
not inspect commitment value, slot, or column and does not recompute EML,
displacement, crossing, or destination. A separately frozen fabricated event
kind cannot consume the real commitment, and ambiguous reused event kinds fail
the generic door. There is no ActionBand-local generic emission record or
boundary queue.

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
| `sparse_gpu_state_ping_pongs_and_matches_exact_eml_oracle` | Runs production dispatch with proof readback disabled, then proof generations; checks strict read-current/write-next/swap progression, the depth-1 crossing → empty → crossing sequence (state generations 1 → 1 → 2), admitted velocity execution, exact GPU/CPU-referee EML parity, source-bound structural provenance, real `Remove` application, RED CPU numeric re-derivation, RED fabricated event-kind provenance, deferred non-7.2 destinations, the no-gather/no-evaluator depth-1 fast path, and the paired combined-path timestamp measurement. |
| `sealed_crossings_are_the_only_emission_ingress_and_destinations_stay_frozen` | Empty sealed evidence emits nothing; the existing Phase-5 delta joins exactly once; an empty depth-1 batch performs zero ActionBand compute work. It catches a rival crossing input/record or unconditional shallow dispatch. |
| `inactive_rows_allocate_zero_hot_storage_and_dense_mutant_is_red` | Proves zero active rows means zero hot bytes and plants a dense two-row allocation against a one-row frozen cap, which fails in production lowering. |
| `bucketing_is_numeric_deterministic_and_labels_are_semantic_shadow_only` | Same numeric product under different labels has the same fingerprint/buckets; equal program/binding shapes share a bucket, a distinct EML program shape separates, and the resulting two numeric ranges execute as two production GPU dispatches. |
| `inherited_admission_and_cpu_authority_fences_remain_closed` | Rechecks the one-shot 7.1 door, forbidden CPU evaluator/planner/scheduler/local-queue vocabulary, read-only `StateCurrent`, write-only-next shader progression, the bounded GPU current→next carry plus ordinary swap, absence of per-row buffer alternation or a world gather in the depth-1 entry, absence of raw value writes or detached structural-source constructors, preservation of threshold registration/locus identity, and absence of numeric selection in the generic structural door. |

## Repeated depth-1 measurement

Method: paired samples of the same one-row/one-dimension threshold workload on
one NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan adapter and run. Arm 1 is the
unchanged bare `AccumulatorOpSession` crossing pass. Arm 2 executes that same
crossing pass and attaches the direct depth-1 ActionBand incremental work: one
scalar-bound target, one admitted three-node payload program (`PARAM(0) * 2`),
one sealed crossing, and one fixed structural binding. Five paired warmups and
31 paired samples were recorded with GPU timestamp queries; the statistic is
the median. The attached combined time is the crossing timestamp plus the
crossing-triggered EML/fixed-emission timestamp for that sample. CPU sealed
join, maps, readback, boundary submission, and structural apply are excluded.
The crossing accessor has 1,000 ns resolution.

Observed local median:

- Bare crossing median: **5,000 ns**
- Attached arm crossing component median: **5,000 ns**
- Attached EML + fixed structural emission median: **5,632 ns**
- Attached combined-path median: **10,568 ns**
- Paired combined-path delta median: **5,568 ns**
- Remaining compute-pass ActionBand overhead after EML/fixed emission: **-64 ns**
- Depth-1 target-evaluation dispatches: **0**
- Depth-1 world re-gathers: **0**
- Ratio: **2.114x**

This closes the §15.2/§15.3 attribution defect: the actual attached production
shape pays the already-owed crossing plus exactly the crossing-triggered EML
and fixed-emission dispatch. The paired median delta is within 64 ns of that
emission component (the signed -64 ns residual is timestamp/median noise);
there is no independent target-evaluation pass, second comparison,
CPU re-derivation, duplicate field projection, or shallow-path world re-gather.
The bounded GPU current→next carry is a command-encoder buffer copy outside the
unchanged compute-pass timestamp envelope; it performs no semantic evaluation.
This tiny shape still does not validate §15.6 production-cardinality
sparse-gather scaling, and the ratio is not a pass/fail threshold or broad
hardware performance claim.

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
