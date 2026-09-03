# RECURSIVE-RESOURCE-FILTER-UNIFICATION-0 results

Status: **PROBATION / proof-present / DA-review-pending / UNMERGED**

Authority: Board remand `5526580726`; handoff
`handoffs/RECURSIVE-RESOURCE-FILTER-UNIFICATION-0.hd.md`; `HD-RECEIPT 4085c47cac11`.
Implementation base: `073c82c1226b97c0c694e724cabfc4639c52e115`.
Initial coding ingress: `ORIENT-RECEIPT a54d4b41439e`, rule stamp `6ecf0749606e5905`, orientation
digest `910ccc6a8ce02bd3818cbb256006634423b1d7cc6eabd3fe313fa07ced3c0412`.
After the canonical anchor mutation, fresh coding ingress is `ORIENT-RECEIPT 190e0178bcad`, rule
stamp `238baeafe1f2f0da`, with regenerated orientation digest
`2d86845dacf7d4a80f8989db71e0426c1179daa056c587bb31e13adb183521ec`. The immutable Board/PR
packet carries this fresh receipt.

## Outcome

The graduated resident RF market now has one canonical API description: a recursively composable
constrained-resource filter whose resident R emits continuous edge resource and whose unchanged
frozen Q emits canonical identity-bearing T_s. The production and generalized CPU names are literal
Rust aliases over the existing authorities. No wrapper, conversion, field, arithmetic, response
state, fallback, or second economic implementation was added.

The exact public/runtime peer-authority set identified by 15.0 decreases from five symbols to two.
One unused public compiler wrapper is deleted; its owner-view compiler and the report-to-local-
allocation helper are crate/module internal. The two still-required public legacy report/oracle
witnesses remain quarantined. Thus:

```text
N_peer_runtime_authorities(before) = 5
N_peer_runtime_authorities(after)  = 2
strict decrease                    = 3
conversion-free aliases added      = 2
public/runtime nouns removed        = 3
N_new_economic_authorities          = 0
```

## Production call graph

```text
SimSession::step_once
  -> GrowthEntitlementMarketBinding::resolve_batch_resident
     -> RecursiveResourceFilterRuntime (= ResidentClearingRuntime)
        -> dispatch(N, Some(root rows))
           -> one graduated R over born AllocatorWeight / AllocatedFlow
           -> one frozen exact Q
           -> canonical T_s enters the IntegrationSchedule live head
        -> dispatch(N+1, None)
           -> identical T_s is the child's resident recursive supply intake
           -> optional 15.2 deformation executes inside this one Current->Next mint
        -> materialize (asynchronous replay/structural consequence only)

explicit CpuVendorizedOracle posture
  -> resolve_batch_cpu_vendorized_oracle
     -> clear_constrained_claims_at_generation (quarantined CPU reference door)

proof/reference view only
  -> evaluate_recursive_resource_filter_oracle
     = run_arena_allocation_oracle (same function item, same bits)
  -> execute_resident_apportionment_cpu (unchanged frozen Q reference)
```

There is no ordinary-session fallback from resident admission/execution to a CPU door and no edge
from a structural consequence recorder back to claim, demand, R, or Q.

## Symbol-keyed archaeology and disposition

Callers below name runtime/source consumers; public re-export sites and focused proof consumers are
included where they are part of the authority posture. Definitions are not counted as callers.

| Symbol | Current callers | Role | Disposition / after-state |
|---|---|---|---|
| `ResidentClearingRuntime::dispatch` | `GrowthEntitlementMarketBinding::resolve_batch_resident`; frozen 14.6 and 15.2 resident witnesses | sole ordinary resident R->Q dispatch | **KEEP-CANONICAL** as the implementation behind the type alias |
| `GrowthEntitlementMarketBinding::resolve_batch_resident` | `SimSession::step_once` resident posture | sole ordinary production entry | **KEEP-CANONICAL**; parameter now spells the canonical filter alias |
| `GrowthEntitlementMarketBinding::resolve_batch_cpu_vendorized_oracle` | `SimSession::step_once` only under explicit CPU posture | diagnostic/reference selector | **KEEP-ORACLE-QUARANTINED** |
| `run_arena_allocation_oracle` | RF burn-in and ClauseThing reference tests; frozen 15.0/14.5 proofs | generalized executable CPU R reference | **REHOME/ALIAS**; implementation retained, canonical item alias added |
| `evaluate_recursive_resource_filter_oracle` | 15.1 terminal identity/three-edge referee | conversion-free canonical CPU R name | **KEEP-ORACLE-QUARANTINED**; item alias, not a new authority |
| `execute_resident_apportionment_cpu` | frozen 15.0, 14.4, and 14.5 proof suites | exact Q CPU reference | **KEEP-ORACLE-QUARANTINED** unchanged |
| `ResidentConstrainedProduct` | resident runtime, flow-market structural consequence, frozen proofs | canonical identity-bearing T_s | **KEEP-CANONICAL** |
| `ResidentSettlementOutput` | frozen exact/spatial recursion proofs | settlement role view of T_s | **REHOME/ALIAS** already conversion-free |
| `ResidentRecursiveSupplyIntake` | resident recursive intake and frozen proofs | child supply role view of the same T_s | **REHOME/ALIAS** already conversion-free |
| `ResidentClearingBatchBinding` | `resolve_batch_resident`; resident admission tests | root physical/claim binding into the same operator | **KEEP-CANONICAL**; not an economic translator |
| `ResidentClearingReplayEnvelope` | resident live-head transport and 14.6 seam proof | replay/transport envelope | **KEEP-CANONICAL** non-economic transport |
| `clear_constrained_claims_at_generation` | driver `growth_entitlement`; spec `runtime_rf_tick`; explicit embedder `run::cpu_filter_oracle`; proof corpus | primary generalized CPU compatibility door | **KEEP-ORACLE-QUARANTINED** |
| `clear_reduced_owner_channels` | 11.2f proof and frozen 14.1 comparator | generationless compatibility door | **KEEP-ORACLE-QUARANTINED** |
| `clear_reduced_owner_channels_at_generation` | frozen 14.1/14.5 proofs | generation-aware compatibility door | **KEEP-ORACLE-QUARANTINED** |
| `clear_stamped_owner_channels` | driver germ proof and frozen comparator; no embedder re-export after 15.4 | stamped compatibility door | **KEEP-ORACLE-QUARANTINED** |
| `produce_runtime_rf_next_generation_demands` | driver `runtime_rf_tick_compile`; frozen comparator | CPU Current->Next reference door | **KEEP-ORACLE-QUARANTINED**; 15.2 semantics frozen |
| `produce_runtime_rf_next_generation_demands_for_tick` | frozen 15.0, 15.2, 14.5 and 14.3 witnesses | driver wrapper over the one temporal reference door | **KEEP-ORACLE-QUARANTINED** |
| `apply_owner_silo_runtime_disburse_down_cpu` | automaton reception, driver compile plan, legacy spec/report/scenario paths, proof tests | pre-cutover CPU disburser/reference | **KEEP-ORACLE-QUARANTINED**; still-required compatibility witness, counted after |
| `compile_owner_silo_disburse_down_plan` | none before deletion | unused scenario-to-legacy-plan public wrapper | **DELETE** |
| `compile_owner_silo_disburse_down_plan_from_owner_view` | driver runtime-local-allocation and runtime-RF-tick compilers | retained internal legacy proof compiler | **REHOME/ALIAS** to `pub(crate)`; no public/runtime authority |
| `evaluate_owner_silo_disburse_down_with_rf_source` | driver recursive-source compiler; spec loaded-report, local-allocation, scenario-ingestion paths | legacy selectable report/oracle witness | **KEEP-ORACLE-QUARANTINED**; counted after |
| `runtime_local_allocation_from_owner_silo_disburse_report` | only its own local-allocation evaluation module | report-to-local-allocation helper | **REHOME/ALIAS** to private module helper; no public/runtime authority |
| `allocator_from_disbursements` | no production caller; conservation oracle exports only | proof-only conservation projection | **KEEP-ORACLE-QUARANTINED**; excluded from runtime peer metric |
| `record_resident_structural_grant` | `resolve_batch_resident` after exact grant | sparse structural consequence recorder | **KEEP-CANONICAL** one-way; excluded from peer-market metric |
| `record_cleared_market_grant` | CPU oracle resolution and proof callers after exact grant | sparse structural consequence recorder | **KEEP-ORACLE-QUARANTINED** one-way; excluded from peer-market metric |
| economic product `From`/`Into`, seam payload translator, duplicate feedback adapter | no caller; absent | forbidden peer/adaptor family | **DELETE/REMAIN-ABSENT** |

Archaeology found no semantic MISSING and no contradiction with the accepted
`exact market = Q compose R` theorem.

## Strict deletion census

The metric universe is the exact five-symbol peer set admitted by the 15.0 preparation census:

| Symbol | Before public/runtime | After public/runtime | Mechanical evidence |
|---|---:|---:|---|
| `apply_owner_silo_runtime_disburse_down_cpu` | 1 | 1 | still-required public compatibility oracle |
| `compile_owner_silo_disburse_down_plan` | 1 | 0 | definition and root re-export deleted |
| `compile_owner_silo_disburse_down_plan_from_owner_view` | 1 | 0 | `pub fn` -> `pub(crate) fn` |
| `evaluate_owner_silo_disburse_down_with_rf_source` | 1 | 1 | still-required public report/oracle witness |
| `runtime_local_allocation_from_owner_silo_disburse_report` | 1 | 0 | `pub fn` -> private `fn`; both re-export layers deleted |
| **Total** | **5** | **2** | **2 < 5; decrease 3** |

`RECURSIVE-FILTER-PEER-RUNTIME-AUTHORITY-RESIDUE` binds the after set to the exact two surviving
symbol identities in the constitutional census. The terminal referee independently parses the four
owning source files, reconstructs the before set, and proves the same 5->2 result. Its alias
identity checks prove that the two added canonical names are not economic implementations. Three
public/runtime nouns were removed and two aliases added: the literal noun budget decreases by one.

## Five-door quarantine

| Symbol | Runtime/source callers after | Current role | Quarantine mechanism | Architecture status after |
|---|---|---|---|---|
| `clear_constrained_claims_at_generation` | driver `growth_entitlement`; spec `runtime_rf_tick`; explicit embedder `run::cpu_filter_oracle`; proofs | primary generalized CPU clearer | `CPU-CLEARING-ORACLE-DOORS=5`; explicit CPU posture/caller/re-export census | reference Q-side compatibility; not architecture |
| `clear_reduced_owner_channels` | proof/comparator only | generationless reduced-channel bridge | same five-door row; no ordinary driver caller | compatibility proof only; not architecture |
| `clear_reduced_owner_channels_at_generation` | proof/comparator only | generation-aware reduced-channel bridge | same five-door row; no ordinary driver caller | compatibility proof only; not architecture |
| `clear_stamped_owner_channels` | proof/comparator only; no embedder re-export after 15.4 | stamped reduced-channel bridge | same five-door row; no ordinary driver caller | compatibility proof only; not architecture |
| `produce_runtime_rf_next_generation_demands` | driver `runtime_rf_tick_compile`; proof comparator | CPU temporal recurrence reference | five-door row plus `CPU-CLEARING-ORACLE-CALL-SITES=2` | reference Current->Next door; not a second filter |

The row remains exactly five; the driver caller census remains exactly the two already-authorized
source identities. No sixth door or ordinary-session fallback was added.

## Ten binding proofs

| # | Binding | Mechanical discharge |
|---:|---|---|
| 1 | Identity preservation | 15.1 compares the canonical function alias with `run_arena_allocation_oracle` by function identity and bit-for-bit cell/trace output; frozen 15.0 and 14.5 retain exact R/Q witnesses. |
| 2 | Recursive composition | 15.1 executes root->depth1->depth2->leaf, exactly three edge disbursements of the same live resource, with no intermediary type or peer API; frozen 14.5 retains literal T_s `8->6->4`. |
| 3 | No second authority | 15.2 policy changes x/g only inside the same resident mint; the alias test proves no wrapper implementation; constitutional production authority remains one and new economic authorities remain zero. |
| 4 | Bounded state | `RecursiveResourceFilterRuntime` and `ResidentClearingRuntime` have exact Rust `TypeId` and `size_of` identity; no response field/program family was added; graduated resident budgets remain unchanged. |
| 5 | Subtree queries | the three-edge trace publishes one direct-child `weight_sum=1` at every interior and answers the downward pass from it; `RESIDENT-CLEARING-NO-PRIVATE-FIELD-SOLVER=0` and no host descendant reconstruction is present. |
| 6 | Triad singularity | frozen 14.5 born Gu-Yang-F/raw-P witness remains bit-exact; R consumes only born cells, and the resident forbidden-solver census remains zero. |
| 7 | Ordering separation | frozen 15.0 holds x fixed while hard precedence changes Q; frozen band/parity tests retain RF stage order, continuous share, exact residue, and Q149 as separate authorities. |
| 8 | Generation pacing | frozen 15.0 second-mint refusal and frozen 15.2 identity/decay once-mint tests pass; no same-generation reweight/re-clear edge was added. |
| 9 | Physical invariance | frozen 14.4/14.5 workgroup, dispatch partition, physical row, epoch, and Q149 suites plus frozen 14.6 independent realms/scheduling witnesses pass unchanged. |
| 10 | Deletion | terminal source census proves `5->2`, constitutional row proves the exact two after symbols, alias identity proves `N_new_economic_authorities=0`. |

## 15.2 preservation

No 15.2 production or proof source changed. `PersistenceDeformationProgram` remains optional sealed
ordinary EML inside the sole Current-to-Next mint. Absent policy remains bit-identical identity;
resident and CPU reference still agree on `100->80->64`; duplicate scope, non-finite, cap,
no-partial, and second-mint refusals remain frozen. The valuation/CostBand/Overlay chain remains
consequence-only and has no demand reinjection.

## Four-anchor canonization

| Anchor | Previous temporary home/lifecycle | Canonical home/lifecycle | Durable law |
|---|---|---|---|
| `phase15-filter-operator` | charter section 1 / `until:RECURSIVE-RESOURCE-FILTER-UNIFICATION-0` | core design 8.4.1 / `canonical` | one R with upward/downward projections and deterministic Q, aliases only |
| `phase15-recursion-axes` | charter section 2 / rung-scoped | core design 8.4.2 / `canonical` | continuous edge identity, exact T_s identity, single temporal U recurrence |
| `phase15-sufficient-statistic` | charter section 3 / presumption | core design 8.4.3 / `canonical` | born scalar/direct-child sum sufficient; implicit normalization; O(1); Triad once |
| `phase15-deletion-census` | charter section 6 / rung-scoped | core design 8.4.4 / `canonical` | exact 5->2 peer census, five-door quarantine, zero new authority |

The charter remains historical rationale but owns no live anchor. Anchor resync/check reports four
healthy canonical pointers with zero orphaned or stale lifecycle pointers.

## Changed-file census

- `crates/simthing-driver/src/arena_allocation_oracle.rs`: one conversion-free function item alias.
- `crates/simthing-driver/src/resident_clearing_runtime.rs`: one conversion-free runtime type alias.
- `crates/simthing-driver/src/growth_entitlement.rs`: sole production entry spells the alias.
- `crates/simthing-driver/src/lib.rs`: exports the function alias; deletes the unused public wrapper export.
- `crates/simthing-driver/src/owner_silo_disburse_down_compile.rs`: deletes the unused public wrapper and internalizes the retained owner-view helper.
- `crates/simthing-spec/src/spec/local_allocation_recursive_rf_source.rs`: internalizes the report helper.
- `crates/simthing-spec/src/spec/mod.rs`, `crates/simthing-spec/src/lib.rs`: remove the internalized helper re-exports.
- `crates/simthing-workshop/tests/recursive_resource_filter_unification_0.rs`: alias/three-edge/deletion/five-door closing referee.
- `docs/simthing_core_design.md`: accepted statement and four-part durable Phase-15 closure.
- `scripts/ci/doctrine_anchors.tsv`: four lifecycle repoints and hash resync for affected canonical sections.
- `scripts/ci/constitutional_surfaces.tsv`: exact after-state peer-authority residue row.
- `docs/sanctioned_surface.md`: generated constitutional surface refresh.
- `docs/orchestrator_orientation.md`: generated doctrine-anchor source-stamp refresh.
- `scripts/ci/test_inventory.tsv`, `docs/tests/current_evidence_index.md`, this report, and append-only anchor reach log: evidence registration.

No `.github/workflows/**`, CI shell/Python, exact shader, frozen 14.1 comparator, frozen 14.4/14.5/14.6
test, frozen 15.0/15.2 proof, Q arithmetic, T_s ABI, workplan pointer, or handoff pointer is changed.

## Verification

| Command / surface | Local result |
|---|---|
| touched `simthing-spec` + `simthing-driver` all-target checks | PASS |
| touched `simthing-workshop` all-target check | PASS |
| 15.1 closing referee | PASS — 2/2 |
| frozen 15.0 theorem/minimality | PASS — 1/1 |
| frozen 15.2 persistence port | PASS — 3/3, real resident GPU |
| frozen 14.4 exact/physical invariance | PASS — 6/6 |
| frozen 14.5 parity | PASS — 1/1; qualification fingerprint `1c3ca3cf8e625e48` |
| frozen 14.6 causal/self-consumption/tree isolation | PASS — 3/3 |
| constitutional authority census | PASS — production authority 1; peer residue 2; CPU doors 5; CPU callers 2; all four forbidden resident shapes 0 |
| test inventory / drift / lifecycle schema | PASS — 1399/1399; missing 0; extra 0; unledgered 0; stale 0; expired 0 |
| doctrine anchors | PASS — four Phase-15 anchors canonical; healthy pending 0; orphaned 0; stale 0; curation 88 |
| sanctioned-surface freshness | PASS |
| orientation digest freshness | PASS after generated source-stamp refresh |
| Agent Scan | PASS — doctrine failures 0; doctrine inspect 0; three expected gate-wiring notices for the anchor/surface data ledgers |
| detachability | N/A — no ClauseThing surface or dependency changed |

Local structural certificate: **ZERO-RED**. The immutable PR/Board packet records exact candidate
head, hosted Doctrine Scan/Exec workflow IDs, fresh clearance, and relay-lint.
