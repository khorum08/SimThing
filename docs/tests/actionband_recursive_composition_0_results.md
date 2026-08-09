# ACTIONBAND-RECURSIVE-COMPOSITION-0 results

- Track: 0.0.8.7 RF arena modernization (rung 7.3)
- Status: **PROBATION — DA review pending**
- Branch: `codex/actionband-recursive-composition-0`
- Canonical base: `f1e4a53f1918cd85a011858be53ba967f0f81312`
- HD-RECEIPT: `f7dea5471359`
- ORIENT-RECEIPT: `d676e8577e1b`
- orientation_rule_stamp: `3422182133007944`

## Product

The driver projects the already-frozen 7.1 dependency spans into a canonical,
flat table of pre-admitted child-instance rows at session build. A child row
must be present, initially inactive, uniquely owned by one parent lifecycle,
within the frozen concurrent cap, and acyclic. Missing rows, shared child
lifecycle, cap overflow, and runtime-lifecycle cycles fail lowering. Reversing
authored sibling order or instance append order produces the same numeric
fingerprint because physical dependency rows are canonicalized and never carry
semantic order.

The common recursive shape reuses the graduated depth-1 crossing entry, target
descriptors, EML program ids, sealed `BandCrossingDelta` join, and whole-buffer
`StateCurrent -> StateNext` swap. There is no recursive executor or CPU child
scheduler. A parent crossing at generation `t` writes only inactive child-next
activation bits. Child invocations in the same dispatch read inactive
`StateCurrent` and remain inert; they can execute only after the swap at `t+1`.
Satisfied subordinate rows write a terminal bit and collapse. A later parent
crossing observes all terminal child-current bits and may then emit. Recursive
templates outside the shared direct single-channel, one-band, no-velocity fast
shape fail closed.

Unresolved parents and inactive children leave the existing 16-byte sealed
threshold packet unwritten. Its prefilled impossible-column sentinel is
discarded before token minting, so a CPU join cannot authorize a structural
consequence. Authorized rows retain the exact graduated packet shape and
identity check.

Trivial requirements remain inline: an existing EML program reads ordinary
state, native RF-claim, and native scalar-CostBand columns and emits through the
existing fixed structural binding without a child row or ActionBand-local RF,
CostBand, conservation, clearing, holding, or transaction authority. No
`crates/simthing-core/src` file changed.

## TEST-BUDGET INSPECT

All five tests share one focused integration target. Each admits a distinct
exit-proof obligation:

| Test | Admission reason |
|---|---|
| `parent_activates_children_next_then_resolves_after_later_collapse` | Proves the three-generation parent -> child-next -> concurrent child execution/collapse -> later parent-resolution trace on the production GPU path. |
| `sibling_and_instance_append_perturbations_compile_bit_identically` | Proves sibling declaration and instance append order are non-semantic and compile to one canonical numeric plan. |
| `runtime_child_construction_shared_lifecycle_and_nonfast_recursion_are_red` | Plants three forbidden rivals: missing pre-admitted child construction, two-parent child lifecycle races, and recursion outside the shared depth-1/2 entry. |
| `trivial_state_rf_and_scalar_costband_gate_stays_inline` | Proves one existing EML program composes ordinary state, native RF, and scalar CostBand columns with zero dependency rows. |
| `state_carry_curve_reports_increasing_active_cardinality` | Records the required rows/bytes/timestamp curve needed to adjudicate, rather than assume, a section 15.6 compact-list remedy. |

## State carry cardinality curve

Adapter: NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan. Each point used five
warmups and 31 GPU timestamp samples; the reported statistic is the median.
The workload uses the same depth-1 entry, one sealed crossing, and the existing
full contiguous `StateCurrent -> StateNext` buffer copy. CPU join, maps,
readback, and structural application are excluded.

| Materialized rows | State bytes | Carry median |
|---:|---:|---:|
| 1 | 32 | 4,096 ns |
| 64 | 2,048 | 4,096 ns |
| 1,024 | 32,768 | 5,120 ns |
| 4,096 | 131,072 | 8,192 ns |

Assessment against section 15: the fixed floor remains flat through 64 rows,
then rises with contiguous bytes. This is consistent with the current/next
carry crossing from command overhead toward bandwidth cost. It does not show
section 15.6 sparse-gather amplification, because the measured operation is a
contiguous state copy and the recursive fast entry performs no target-world
re-gather. A compact active-list remedy is therefore not justified by this
curve and was not applied.

## Boundaries retained

- The 7.1 admission door, seven target forms, dependency/cap budgets, and
  pre-8.x atomic/persistent scarce-lane rejection are unchanged.
- The 7.2 crossing identity, read-only `StateCurrent`, read-only world-values
  binding, `StateNext` write authority, sealed packet shape, and structural
  boundary are unchanged.
- No movement vocabulary, 7.4/7.5 behavior, 8.x clearing/holding, StemThing-B,
  or Vector CostBand work was introduced.
- No CI script or workflow file was edited. Only the required reach and test
  inventory data ledgers changed under `scripts/ci/`.

## Load-bearing anchor acknowledgements

- `actionband-gpu-physical-model@f324b18cd960`
- `actionband-binding-laws@030bb13655df`
- `actionband-performance-model@adeea5dbaee7`
- `actionband-determinism-lifecycle@283367293fc1`
- `actionband-executive@9a8e0c500c49`
- `actionband-8x-sequencing@52a1faeb85b5`
- `actionband-fenced-questions@54276c7829fa`

The complete path-projected anchor ACK set, exact-head batteries, doctrine
verdict, hosted checks, and routing block belong in the PR/Board relay.
