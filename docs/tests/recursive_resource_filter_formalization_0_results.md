# RECURSIVE-RESOURCE-FILTER-FORMALIZATION-0 results

Status: **PROBATION / proof-present / DA-review-pending / UNMERGED / no
15.1**. Coding has not merged, graduated Phase 15, moved the pointer, started
the held harness-fix session, begun response compression, or closed the track.

## Provenance and receipts

- Board handoff: `5519518567`
- exact owner-mandated branch base / `origin/master` at dispatch:
  `fad9141508f58f2d2b31fb1bc694a2e72bb7a0ca`
- final integration base: `7bce757549c44cd28b20ff5bcf23ab1d57a4f4c2`
  (concurrent Owner-approved 15.0 persistence-census addendum, PR #1938;
  audited and incorporated before push)
- HD authored base: `2266ee531556b3ed7420365faeb1f25bc4d3f630`
  (ancestor of the dispatch base)
- branch: `codex/recursive-resource-filter-formalization-0`
- `HD-RECEIPT: 73859b617dc4`
- `ORIENT-RECEIPT: 6146c31f6c64`
- orientation rule stamp: `a7ee98a6a51642a7`
- orientation digest:
  `8948f15d7cb48f9f6ad9f1ca78a7b8bf2e63b2486b5af30920d005e73140cd4a`
- exact reviewed head and hosted workflow ids: recorded in the immutable final
  PR/Board packet after this report is committed

All 63 rendered required anchors plus the four Phase-15 temporary anchors and
`resident-clearing-cutover-canon` were acknowledged through
`scripts/ci/anchor_query.sh`; the append-only reach ledger carries the receipts.

## Archaeology-first disposition

| Surface | Graduated authority found | 15.0 disposition |
|---|---|---|
| recursive continuous allocation | `plan_arena_allocation_with_pressure` and `run_arena_allocation_oracle` perform direct-child pressure sums upward and one child-share EML downward | **recognized as R**; no new operator |
| generalized share-vector oracle | `run_arena_allocation_oracle` consumes caller-owned born values and executes the same child-share law at every depth | **executable reference R**; documentation names the already-running role |
| pressure production | raw entitlement `P` enters the direct-child sum; sealed born Gu-Yang `F` enters the same `AllocatorWeight` through the neutral identity binding | **already represented**; no response program or second field solve |
| exact settlement | `execute_resident_apportionment_cpu` and the resident exact kernel consume finite `AllocatedFlow`, request caps, supply, hard precedence, identity, and generation; Q149 plus quotient/remainder/ties/U are frozen | **Q remains separate and unchanged** |
| canonical product | `ResidentConstrainedProduct`, `ResidentSettlementOutput`, and `ResidentRecursiveSupplyIntake` are one exact type | **T_s remains literal**, without adapter/newtype |
| spatial production recursion | non-root child share reads the parent's live `AllocatedFlow`; `dispatch(None)` reads prior canonical `T_s` directly from the resident live head | **identity already executable** at continuous and exact strata |
| temporal recursion | `produce_runtime_rf_next_generation_demands` performs the one authority-minted `d' + U` carry to N+1 and rejects a second mint | **identity already executable**; never same-generation |
| hard precedence | ordered classes are admitted before continuous allocation and exact residue order | **orthogonal authority**; never response curvature |
| root/interior/leaf | one plan builder emits the same child-share tree id for every edge; a leaf contributes born state and has no outgoing edge | **degenerate bindings of one R** |
| production cutover | `ResidentClearingRuntime::dispatch(Some(rows))` binds the root; `dispatch(None)` binds resident recursive intake; both use the same allocator/exact kernel | **binding distinction only**, not two economic operators |
| CPU clearing vocabulary | five compatibility doors remain quarantined behind `CpuVendorizedOracle`; ordinary resident continuation has no CPU fallback | **15.1 census input only**; no deletion in 15.0 |
| physical invariance | frozen 14.5 covers row/order/workgroup/partition/realm matrices; frozen 14.6 covers tree-local schedules and causal live-head intake | **re-run unchanged** |

No semantic MISSING item was found. The successful result is recognition of the
graduated representation, not runtime expansion.

## Accepted operator

For every node `v`, the accepted operator is:

```text
R_v(S_v, Phi_v, {eligible_c, U_c})
  upward   -> P_up = direct_child_sum(eligible_c)
  downward -> x_(v->c) = child_share(S_v, live AllocatedFlow_v,
                                     eligible_c, P_up)

T_s = Q(request, supply, hard_band, identity, generation,
        R_v.downward)
```

This is the charter's candidate
`R_v:(S_v,Phi_v,{P_c,F_c,U_c,...})->(P_up,{x_(v->c)},U_v)` with
the existing typed eligibility projection made explicit. `P_c` and `F_c` are
not summed as two independent economic influences: the admitted commitment
class selects raw `P` or sealed serviceable `F`, and the neutral identity binds
that value once into `AllocatorWeight`. `U` is emitted by frozen Q and forwarded
only across the Current-to-Next boundary. There is one R, two projections, one
Q, one canonical T_s, and one temporal recurrence.

## Sufficient-statistic verdict

The presumption is **proved sufficient and shown non-minimal as simultaneous
runtime storage**.

1. Before R, `(P,F)` is a sufficient typed description of the two lawful
   eligibility sources within one already-authoritative precedence class.
   Exactly one leg is selected for a participant, so R needs only the existing
   eligible scalar `AllocatorWeight`; retaining both as a response tuple would
   duplicate upstream provenance.
2. R's upward sufficient statistic is the existing direct-child sum
   `weight_sum/P_up`. Its downward state is the existing incoming
   `AllocatedFlow` plus that sum and each child's eligible weight. Multiplying
   all child weights by one positive scalar leaves emitted `AllocatedFlow` bits
   unchanged (`17:33` and `34:66` both emit exact `17.0,33.0` under supply 50).
   Therefore lambda is entirely implicit in normalization.
3. For exact Q within fixed supply, hard band, identity, and generation, the
   existing tuple `(requested, AllocatedFlow)` is necessary and sufficient.
   The focused falsifier holds `AllocatedFlow=(17.0,33.0)` fixed while changing
   requests `(17,33)->(5,45)`; grants/U change because request caps are real.
   Continuous flow alone is insufficient, but the missing statistic is already
   present in `ResidentApportionmentClaim`; it is not `A_v(lambda)`.
4. A second falsifier holds requests and flow fixed and changes only hard
   precedence. Exact products change from `(6,11),(13,20)` to
   `(17,0),(2,31)`. This proves hard precedence is contextual ordering, not
   continuous response curvature.

Verdict: **no ShadowPrice, DemandCurve, ResponseProgram, LambdaPlane,
ScarcityColumn, A(lambda) store, or richer resident tuple is necessary.**
`A_v(lambda)` remains proof notation only.

## Runtime-real versus proof-only quantities

| Quantity | Classification | Existing representation |
|---|---|---|
| root supply / incoming continuous resource `S_v` | runtime-real | root intrinsic flow or parent live `AllocatedFlow` |
| incoming exact resource | runtime-real | canonical `ResidentRecursiveSupplyIntake` T_s in the resident live head |
| raw entitlement pressure `P` | runtime-real before eligibility projection | direct-child `AllocatorWeight` source |
| serviceable born Gu-Yang flow `F` | runtime-real before eligibility projection | sealed ActionBand bound source, neutral identity to the same weight cell |
| eligible child scalar | runtime-real and minimal for R | `AllocatorWeight` |
| `P_up` | runtime-real transient | direct-child `weight_sum` and parent `weight` |
| `x_(v->c)` | runtime-real | child `AllocatedFlow` |
| request / cap | runtime-real | `ResidentApportionmentClaim::requested` |
| hard precedence | runtime-real admitted context | `ResidentApportionmentClaim::precedence`; ordered separately |
| exact grant and `U` | runtime-real | canonical `ResidentConstrainedProduct` |
| identity, realm, generation, integration band | runtime-real context | resident plan/buffer owner and canonical product fields |
| `Phi_v` | proof grouping, not a new object | existing admitted layout/program/identity/generation context |
| `R_v`, `Q compose R`, response tuple | proof names over existing functions/cells | no production struct or allocation |
| lambda / `A_v(lambda)` | proof-only | implicit normalized share; no stored column/plane/program |
| root/interior/leaf operator labels | proof-only roles | one plan/oracle and one child-share tree id |

## Two recursion identities

Spatial continuous identity:

```text
x_(p->v) = AllocatedFlow[v] = S_v
```

The focused three-edge operator trace is root→interior `50.0`, then
interior→leaf `17.0/33.0`. Every edge's plan operation has the same child-share
EML tree id. At non-root depth the operator reads the parent's live
`AllocatedFlow` directly; there is no propagated economic copy or descendant
walk.

Spatial exact identity:

```text
Q(x_(p->v)) = T_s(p->v) = ResidentRecursiveSupplyIntake[v]
```

The output and intake role names are exact aliases. Frozen 14.5 passes literal
T_s down a three-edge `8 -> 6 -> 4` chain; frozen 14.6 `dispatch(None)` consumes
the resident live-head bytes before host materialization.

Temporal identity:

```text
d_effective(v,N+1) = d_authored(v,N+1) + U(v,N)
```

The focused witness clears request `10` against supply `4`, producing `U=6`;
authored N+1 demand `2` becomes stamped demand `8` at generation 24. The same
authority's second call returns `DemandCurrentToNextAlreadyProduced`. No
same-generation reweight or re-clear exists.

## Triad exactly once

PALMA remains the upstream route/impedance authority where topology requires
it. Gu-Yang remains the upstream serviceability/realized-flow authority where
capacity requires it. ActionBand observes the sealed born product, and
`bind_immediate_flow_pressure_to_allocator_weight` performs the sole neutral
N→N+1 identity into the already-existing eligibility cell. Entitlement-first
participants use raw `P` instead.

R accepts only layout plus born cell values and never invokes a field sweep,
PALMA query, Gu-Yang solve, private flux solver, or descendant scan. Frozen 14.5
runs the actual Gu-Yang producer, verifies its two field dispatches, feeds its
sealed products through the typed identity, and then executes the same
allocator. The focused reference wrapper contains no arithmetic and delegates
directly to `run_arena_allocation_oracle`. Thus influence occurs upstream once
and response formation consumes it once; there is no second Triad application.

## Executable equivalence transcript

The new terminal referee composes only graduated authorities:

```text
execute_reference_r -> run_arena_allocation_oracle
project_q           -> execute_resident_apportionment_cpu
exact market        = Q compose R
```

Focused exact transcript:

| Case | R edge flow | Q grants/U |
|---|---|---|
| eligible weights `17,33`, supply `50`, exact supply `19` | `50 -> (17,33)` | `(6,11),(13,20)` |
| eligible weights scaled to `34,66` | bit-identical `50 -> (17,33)` | bit-identical T_s |
| same flow, request-cap change | bit-identical `17,33` | differs, proving existing request is necessary |
| same request/flow, hard bands `0,1` | bit-identical `17,33` | `(17,0),(2,31)` |

The frozen 14.5 terminal corpus, unchanged, calls the same generalized arena
oracle and exact Q mirror and remains bit-for-bit equal to the resident path:
neutral nine-item parity, born Gu-Yang-F/raw-P pressure drift, Q149 boundaries,
scale sweeps, W32/W64, dispatch partitions, physical shuffles, recursive T_s,
and independent overlapping-id realms all pass. Qualification remains
`0x1c3ca3cf8e625e48` on Vulkan / NVIDIA RTX 4080 Laptop GPU / NVIDIA 595.79.

Frozen 14.6, unchanged, preserves the causal production transcript:

- neutral shares bits `(1099431936,1107558400)` produce
  `[(7,6,11),(8,13,20)]` at N and unauthored N+1;
- pressure bits `(1065353216,1111752704)` produce
  `[(7,1,16),(8,18,15)]` at N and N+1;
- changing only supply to 7 retains the neutral share bits and produces
  `[(7,2,15),(8,5,28)]` at N and N+1;
- both trees, divergent generations, live-head pacing, FIFO materialization,
  recreated realm-qualified seam, and scheduling/clearing posture matrix pass.

There is no approximate comparison and no workshop-shaped reverse fitting.

## Symbol-keyed 15.1 deletion/rehome preparation

This is a census and disposition map only. No symbol is renamed, deleted, or
aliased in 15.0.

| Symbol(s) | Current role | 15.1 preparation disposition |
|---|---|---|
| `ResidentClearingRuntime::dispatch`, `resolve_batch_resident` | sole ordinary production R→Q authority | **KEEP canonical**; filter-vocabulary expression may replace split prose, not semantics |
| `run_arena_allocation_oracle` | generalized executable CPU reference R | **KEEP oracle**; recognized as R |
| `execute_resident_apportionment_cpu` | frozen exact Q oracle | **KEEP oracle**; Q remains separate |
| `ResidentConstrainedProduct`, `ResidentSettlementOutput`, `ResidentRecursiveSupplyIntake` | canonical T_s and conversion-free port views | **KEEP exact type/aliases** |
| `clear_constrained_claims_at_generation` | primary frozen CPU generalized clearer | **QUARANTINE/REHOME as vendorized oracle vocabulary**; may gain conversion-free filter view, not premature deletion |
| `clear_reduced_owner_channels`, `clear_reduced_owner_channels_at_generation`, `clear_stamped_owner_channels` | four compatibility/oracle doors alongside the primary door | **QUARANTINE** under the existing five-door retirement mechanism; cease defining architecture |
| `resolve_batch_cpu_vendorized_oracle` | explicit selected CPU posture | **KEEP explicit oracle posture**; never fallback |
| `produce_runtime_rf_next_generation_demands`, `produce_runtime_rf_next_generation_demands_for_tick` | frozen CPU-oracle Current→Next reference and driver wrapper | **REHOME/CONSOLIDATE candidate** behind one temporal filter view after call-site proof |
| `apply_owner_silo_runtime_disburse_down_cpu`, `compile_owner_silo_disburse_down_plan`, `compile_owner_silo_disburse_down_plan_from_owner_view` | pre-cutover CPU disburse/proof pipeline | **PEER-AUTHORITY AUDIT candidate**; prove callers and delete/alias only in 15.1 |
| `evaluate_owner_silo_disburse_down_with_rf_source`, `runtime_local_allocation_from_owner_silo_disburse_report` | legacy report-chain vocabulary | **PEER-AUTHORITY AUDIT candidate**; scenario-ingestion/report consumers must be classified before change |
| `allocator_from_disbursements` | conservation test oracle utility | **REHOME as proof-only** if the name implies architecture; not a runtime authority |
| `ResidentClearingBatchBinding` | root `Some(rows)` physical/claim binding | **KEEP binding**, not an economic adapter; interior `None` is the same operator's intake mode |
| `ResidentClearingReplayEnvelope` | bounded plan/seam transport | **KEEP transport**, not an economic payload translator |
| `record_resident_structural_grant`, `record_cleared_market_grant` | sparse structural/lifecycle consequences after clearing | **EXCLUDE from peer-market deletion metric**; audit names, preserve one-way structural role |
| economic `From/Into` product adapter, seam payload translator, duplicate feedback path | absent | **MUST REMAIN ZERO** |

The 15.1 metric therefore remains symbol keyed:
`N_peer_runtime_authorities(after) < N_peer_runtime_authorities(before)` and
`N_new_economic_authorities = 0`. This report does not claim the delta early.

## Authored-persistence census for 15.2

The additive Owner-approved census in PR #1938 was integrated before relay.
The complete production-symbol and caller census finds one authored economic
persistence chain and no authored demand-re-injection program:

| Symbol/program | Causal output | Classification | 15.2 feed |
|---|---|---|---|
| `AuthoredPersistenceValuation::value_program` | values observed U for a later consequence | **CONSEQUENCE-ONLY** | lawful as-is; it never returns a demand |
| `cost_band_quantize` inside `fund_unresolved_persistence` | funds the later consequence | **CONSEQUENCE-ONLY** | lawful as-is; CostBand remainder is explicitly distinct from U |
| `PersistenceOverlayBinding` plus `dispatch_until_dissolved` | emits an ordinary later-state overlay | **CONSEQUENCE-ONLY** | lawful as-is; `PersistenceConsequence` has no claim/demand output |
| `fund_unresolved_persistence` | composes valuation→CostBand→Overlay no earlier than N+1 | **CONSEQUENCE-ONLY** | retain beside the future port; no migration |
| `contention_arena_executed_0` caller | executable consequence witness | **PROOF-ONLY CALLER** | no runtime migration |
| `generation_critical_path_baseline` caller | frozen comparator's structural-consequence leg; overlay dropped after mint | **PROOF-ONLY CALLER** | frozen and untouched |
| `carry_unresolved_demand_to_next_generation` | native unconditional identity carry `d' + U` | **SUBSTRATE, NOT AUTHORED PROGRAM** | 15.2 owns the optional sealed deformation inside this once-mint |

Totals: authored persistence programs `1`; consequence-only `1`;
demand-re-injection `0`; unmigrated re-injection `0`. There is therefore no
present double-counting chain to compensate or migrate. This classification
feeds 15.2 but performs none of its implementation.

## Changed-file census

- `crates/simthing-driver/src/arena_allocation_oracle.rs`: documentation binds
  the existing implementation to executable R; zero arithmetic/ABI change.
- `crates/simthing-workshop/tests/recursive_resource_filter_formalization_0.rs`:
  one proof-only terminal referee over existing R, Q, and Current→Next doors.
- `docs/tests/recursive_resource_filter_formalization_0_results.md`: this
  theorem, archaeology, transcript, and 15.1 preparation packet.
- `docs/tests/current_evidence_index.md`: current probation row.
- `scripts/ci/test_inventory.tsv`: one terminal referee entry.
- `scripts/ci/anchor_reach_log.tsv`: append-only required-anchor acknowledgments.

No exact shader, resident product, production dispatch, Phase-14 test, Phase-14
canon, constitutional surface, frozen 14.1 comparator, or workplan pointer was
changed.

## Verification

Local PASS:

- `cargo check -p simthing-driver --all-targets --offline`
- `cargo check -p simthing-workshop --all-targets --offline`
- focused 15.0 terminal theorem: 1/1
- frozen 14.5 resident parity: 1/1, bit-for-bit, exact qualification retained
- frozen 14.6 cutover/causal suite: 3/3
- test inventory/check and drift: 1391/1391, zero missing/extra/stale
- doctrine anchors: PASS; four Phase-15 pending anchors healthy, zero
  orphaned/stale
- constitutional surfaces: PASS; one resident production authority, five CPU
  oracle doors, two CPU oracle call sites, zero duplicate settlement/economic
  adapters/global coupling/private field solvers
- Agent Scan: PASS on the committed delta, zero failures/inspect findings
- formatting of both touched Rust files and `git diff --check`

Final inventory/drift, doctrine-anchor validation, Agent Scan, hosted Doctrine
Scan/Exec, exact-head clearance, relay lint, PR, and Board ids are recorded in
the final immutable handoff packet after commit/push.

Structural certificate at this report stage: **local ZERO-RED; hosted
exact-head confirmation pending final packet.**
