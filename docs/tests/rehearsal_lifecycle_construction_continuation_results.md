# Construction Leaf A — separated funded crossing continuation STOP

**PROBATION / proof-present / BLOCKED / DA-review-pending. Model 1 UNDECIDED; Model 2 CLOSED.**
This is a production/precondition STOP, not FAIL-MODEL-1 and not a graduation.

Authority: Board [5707765404](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5707765404),
DA combined ruling [5707743893](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5707743893).
Sole coding lane. Same PR #2062 / `codex/0088-construction-leaf-a` rebased ONCE from
`1cd598bd74d51ee298a0259c541f494646a62640` onto exact master
`308adb53ece34b725b1c2e164d3399d1fe2d64c4`. Backup branch
`codex/0088-construction-pre-2070` preserves the old history. The four-file local
witness commit `3bcb5ea3` was cherry-picked unchanged as `fd502250`; its historical
RED packets remain historical. No sorting or boundary splitting replaced it.

## Result and isolation

All 12 retained tests are GREEN on the repaired tree. The added required-success
matrix fails at G6 in eight sessions: placement-refusal/restart versus ample-capacity
control, each at host P/Q and original/reversed child/arena/recipe order. Every failure is
`ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))`.
The four controls have no fragmentation, Remove, refusal, or cancellation and do
complete their first birth at G3. Thus placement refusal is not necessary for the defect.
All 24 retries refuse with `ExecutionIdentity` for faulted generation 6; values,
tick/day, integration-schedule length, binding table and ActionBand generation are
unchanged. Controls additionally preserve the first product's placement.

The test does not accept the error as success: after collecting every case it
requires `failures.is_empty()`. No ignore, expected panic, pin replacement, baseline
refresh, manual dispatch advancement, false crossing, restamping, late reinstall,
stock rewrite, refund, or production repair is used.

## Executed lifecycle and material/reservation disposition

The ordinary recipe costs 1A + 1B per completion at each project. Opposing weights
are P/Q = 3/1 for A and 1/3 for B. Original source is a single 4A/4B pulse. G1 authored
zero-flow overlays stop replenishment. Numerical readback is assertion-only.

| Generation | Owned stock P(A,B,receipt), Q(A,B,receipt) | Structural/placement result |
|---|---|---|
| G1 | (3,1,0), (1,3,0) | Exact fixed removals of blockers A/C leave two separated holes; no product or placement. |
| G2 | (2,0,1), (0,2,1) | Each ordinary recipe consumes one full owned pair. Threshold .5 produces the sealed first crossing; the admitted structural request is queued. No acquired placement yet. |
| G3 | unchanged | Market grant quantity 2, then typed `NoContiguousExtent { quantity: 2 }`; attempted G3/revalue G4/unmet 2. No product, component, row, relation or placement; two free rows remain. |
| G4 | unchanged | Exact remaining blockers removed in one authored boundary; capacity 5. Fixed source policy changes FLOW to 1. No replay of the refused product. |
| G5 | (3.75,1.25,1), (1.25,3.75,1) | The policy applies to root and P/Q, so new supply is 3A/3B total. Fixed source policy changes FLOW back to 0. |
| G6 | (2.75,.25,2), (.25,2.75,2) | Recipes consume another full pair each. Sealed second crossing arrives with source stamp 5, but dispatcher expects 2. Touched generation faults; no second birth is queued/completed. |

Declared disposition is **scrap after recipe consumption on placement refusal**:
inputs already consumed at G2 are not refunded; residual A/B stays ordinary owned
WIP. The scalar receipt records that consumption and is not a product or reservation.
Before the G6 fault, A and B each satisfy residual P+Q plus consumed-pair receipts
= supplied amount (4, then 7). Refused unmet quantity is a diagnostic revalue fact,
not a retained exclusive reservation: the exact placement is absent and physical
capacity is unchanged. The refused candidate is never automatically resubmitted.
Restart pays new input through authored source FLOW, never through a stock write.
The earlier cancellation test uses the same explicit scrap/no-refund disposition.

In the ample-capacity control, the first product and its child are born at G3 with
committed placement and remain resident through G6; all growth-refusal logs stay
empty. The same second-crossing error occurs. Material consumption progresses,
but this does not prove a second structural completion.

## Production cause and requested scope gap

`session.rs::dispatch_action_band_boundary` passes the canonical boundary's sealed
crossings to `CrossingConsequenceDispatch::dispatch_sealed_and_apply`.
`action_band_consequence.rs` has two incompatible assumptions in this ordinary path:

- `dispatch_sealed_and_apply` returns early on empty crossing batches. Facility
  generation only advances on nonempty dispatches.
- `GenerationBoundCrossingDedupe::admit` fixes a source-minus-facility offset on the
  first batch, then requires every later source stamp to equal offset + facility
  generation. `observe_boundary` also uses that facility generation.

Here the G2 crossing has source stamp 1 and advances facility 0 to 1. With offset 1,
the next expected source stamp is 2. Empty G3-G5 do not advance the facility, so
G6's legitimate source stamp 5 is refused. The logged facility remains `Some(1)`.
This is a generic sparse-crossing continuation gap, not an insufficiency of owned WIP.
Adjudicate source-generation versus facility-generation progression while preserving
sealed provenance, replay/duplicate refusal, generation dedupe, and touched-generation
fail-stop. No production-source change is proposed or made by this leaf.

## Proof accounting and limits

Ordered prerequisite execution on synchronized `fd502250`: ingress 1 PASS FIRST;
allocation 2 PASS; WIP 3 PASS; recipe/joint lifecycle 4 PASS; funded cancellation
1 PASS; kernel exact placement retirement/reuse 1 PASS; unchanged multi-remove
1 PASS; sim mixed-family identity 3 PASS. Current thirteenth E8 required/observed
`ec5a2a30afaee795`, semantic bundle `9c2091194366b23b`; no sealed component changed.
The cancellation test covers eight cases, exact placement release, canonical
ActionBand shrink and healthy G5/G6. Kernel reuse is a placement-book unit proof,
not a substitute for actual post-cancellation product reuse in the continuing session.

The added matrix executed four clean placement refusals, four first-birth controls,
eight G6 failures and 24 immutable retry refusals. Driver boundary-acceptance laws
also execute GREEN (9 cases), including stale/foreign row additions/remaps,
missing removal evidence, bound identity removal, registry/dimension fences.
These pure acceptance-law cases are not a claim of full continuing-session negatives.

Still **UNPROVED** because G6 blocks the required path: successful restart after
funded refusal/cancellation; actual physical extent reuse by a new funded product;
two fresh successful product subtrees in one continuing session; their complete
property/overlay/membership/observation/child/binding matrix; final full determinism
and integrated foreign/stale negative battery. G7-G14 assertions remain required
and unexecuted, not counted as coverage. No Model-1 terminal semantic verdict follows.

The new source and raw logs are linked in the companion packet. The PR body and
Board return bind fresh final-head ordered execution, hosted Doctrine/Exec artifacts,
and the post-final-body clearance verdict. Hosted smoke does not prove GPU execution.

## Scope and conformance

New append: one table-driven integration test with two helpers and an isolation
control; one ordinary inventory row; these two result documents. Every prior source
assertion, including the exact 115-line multi-remove witness, remains byte-preserved.
Full PR scope: four driver rehearsal tests, historical/current evidence, ordinary
test inventory. No production source, scenario, gate, class, exception, E8, workplan,
or orientation changes. Other worktrees and the main checkout's unrelated dirty
files remain untouched. No subagent, merge, or new coding lane.

ORIENT-RECEIPT: 28f56884d309
role: coding
orientation_rule_stamp: 73e54d6b56b7266b
orientation_digest_sha: e5b941f898d53f86a8be2e19afba51d52a41dda06f69dff4d3b52817ffed89c6
ANCHOR-ACK: rehearsal-0088-construction-contract@bdd51c7c4ae2

Receipt carried; governance and contract unchanged. Direct Board authority; no HD
receipt invented. Keep OPEN / DRAFT / UNMERGED; orchestration routes the scope gap.
