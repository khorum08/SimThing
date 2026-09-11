# 0088 native RF ingress and policy-consumption evidence

PROBATION / proof-present / clearance-pending / OPEN / UNMERGED. The native RF
first consumer is GREEN after the #2036 engine coverage completion. This is an
ordinary ingress implementation leaf, not a rung-closeout or census-ready claim.
Latest dispatch: Board 5627253305; HD-RECEIPT 70df16e1babd;
ORIENT-RECEIPT 28f56884d309 (mechanically CURRENT on resume).
Current base: e89aeade3299a60ffb20fdab8e0deb1575700f89 (#2036).
Original dispatch: Board 5619313604; original base:
5ae17dfb9ee360044e491627fe8f461c81555cb0 (source/cache PR #2033).
Exact retained head and checks are recorded in the PR and Board return.

## Current result after #2036: unchanged first consumer is GREEN

Rebased onto e89aeade3299a60ffb20fdab8e0deb1575700f89 without conflicts. The
owning test, original beta-policy assertion, native specimen and front-end code
have no Git diff against RED head 2fb660c2645196ae46581ad1033db79edefcb8db.
That pre-rebase RED is preserved on `codex/0088-ingress-overlay-red`; the earlier
RED b98f3e1c9584fdbd84ae20aa9d03f1f09d3e2f62 remains on
`codex/0088-ingress-policy-red`. Original semantic RED 66c36cf9 and original F1
e59dbef317689bf6c23a8a9cf208de35b0fa1946 remain available.

The unchanged first consumer passed from a clean committed checkout at rebased
head df8295feab9d715a3f20cc3c5e74f2f5a3bb78ce. Native and reproducible-cache
routes agree on every canonical hosted allocation; direct programmatic admission
agrees on beta's allocation. All 19 participants, unequal RF/spatial parentage,
inherited ownership, zero-valued PRESENT children, empty Owner spatial seats,
both installed overlays and actual parsed policy multipliers remain checked.

| Authored variant | beta_3 born allocation | beta born allocation |
| --- | ---: | ---: |
| Baseline: child flow 8, beta policy 3 | 7.8761916 | 36.257145 |
| Independent child flow 8 -> 20 | 19.876192 | 36.257145 |
| Independent beta policy 3 -> 7 | 9.48813 | 45.928776 |

Both independent discriminators are now GREEN after three ticks on each route.
No expected value is injected into the session and no observation was replaced.
The existing producer diagnostics show alpha/beta weights 8/27 at baseline and
8/343 with beta policy 7; these are diagnostic only, not the judge.

Causality is the landed #2036 coverage completion: the ordinary session RF sync
uses `collect_weight_overlay_targets` over the sealed runtime tree's active
installed-overlay authority. That per-flow-property target set participates in
the existing protected-interior classification, so this source's actual
AllocatorWeight overlays are covered even though it has zero need bindings.
The same source and assertion stayed RED after #2035 and become GREEN after
#2036 with no caller replay, invented binding, copied policy state or post-RF
patch. Coding has not modified engine code or minted a new policy law.

Cargo check passes. Inventory drift and lifecycle schema pass: 967 active rows,
1536 discovered identities, 569 parked, zero unledgered/stale. Local scan is
INSPECT solely for the existing Orchestration-triaged SPEC-LOWERER-KIND-READ at
rehearsal_ingress_fields.rs:173. Exact final-head package checks and hosted
Doctrine Scan / Clearance results are in the PR and Board return. The stale
historical 0.0.8.7 Board-pointer referee remains unchanged and Orchestration-owned.
Routing is now `novelty_claim: NO`, ordinary rehearsal-ingress-mapping to
Orchestration for O* review/merge. No direct DA relay or self-merge.

The sections below preserve the earlier RED findings and their historical
validation/state; their reserve posture is superseded by the current GREEN above.

## Historical resume after #2035: retained application witness stayed RED

Dispatch 5626475812 expected the unchanged first consumer to become GREEN on the
landed interior-policy composition repair. Rebase completed onto #2035 at
639ef0fcc3e6b28d208bfa8109827722648ab0b9. Original RED code commit 66c36cf9 and
pre-rebase Orchestrator-triaged head b98f3e1c9584fdbd84ae20aa9d03f1f09d3e2f62
remain available; the latter is preserved as `codex/0088-ingress-policy-red`.
The only rebase conflict was an append collision in `triage_log.tsv`; both
Orchestration-authored rows were retained verbatim, with no disposition change.

The original test and native specimen are byte-identical in Git to the retained
RED. A clean-checkout run at rebased head
03385cbfad76d3c3cfa02488a3954f8bc5459674 still fails at the original line 205:
beta's canonical born allocation remains **21.692308 -> 21.692308** under policy
3 -> 7. The independent child change still moves beta_3 from
**5.448718 -> 17.448719**. Native/cache equality, direct programmatic beta control,
all 19 participants, parentage, ownership, installed overlays and parsed policy
multipliers pass before the RED. Three ticks were executed on every route.

The remaining implementation gap is narrower than the original double-write
finding: `arena_allocation_sync.rs:152` derives `authored_weight_slots` only from
`resolved_need_bindings`. The retained source expresses its policy with ordinary
`OverlaySpec` modifiers targeting the AllocatorWeight subfield; it does not author
need bindings. A temporary read-only diagnostic on the same owning test reported
`explicit_resource_flow=false, resolved_need_bindings=0` on all six native/cache
opens, while the existing installed-overlay assertions passed. That diagnostic
was removed and the owning test restored exactly before final verification.

`install.rs:359` resolves need bindings only from the derived ResourceFlowSpec's
explicit need-binding list; `session.rs::sync_resource_flow` passes that resolved
list into the planner. Consequently the new protected-slot set is empty for this
admitted overlay-policy consumer. The existing raw diagnostics still show parent
weights 7/6, and the unchanged canonical allocation discriminator remains RED.
Plan-level tests that supply a nonempty protected set do not establish this live
overlay-to-policy classification path.

This return does not request a new semantic interpretation of the admitted law.
It reports that the landed implementation has not covered its retained
application consumer. Applying the ruled law to installed AllocatorWeight
overlays needs driver-side authority outside this HD. No overlay is translated
into invented need bindings, no policy state is copied, and no caller replay,
post-RF patch or alternate observation is used. The leaf therefore remains a
reserve; its posture cannot truthfully change to ordinary GREEN implementation.

Cargo check passes. The current inventory drift check passes with 966 active rows,
1535 discovered identities and 569 parked, zero unledgered/stale; lifecycle schema
passes. Local scan remains INSPECT solely for the already Orchestrator-triaged
SPEC-LOWERER-KIND-READ at rehearsal_ingress_fields.rs:173. Current package and
hosted results are recorded with the final head in the PR and Board return.
The separate stale historical Board-pointer referee remains Orchestration-owned
and unchanged. Parent leaves #2032 and #2033 are now merged; the historical
sections below describe their state at the original reserve.

## Reproducer and completed front-end work

`scenarios/rehearsal_ingress_rf.clause` declares one ordinary energy property with
IntrinsicFlow, AllocatedFlow (Amount), AllocatorWeight, and governed Balance roles.
There are 19 admitted participants: GameSession, two Owners, two spatial sites,
and seven children per site. Each owner has four positive children, two claimants
and a zero-valued PRESENT child. Beta_3's RF parent is beta_2 while its spatial
parent remains beta_site; alpha_3's RF parent is alpha_site. Owners are empty
spatial seats; owner bindings on the sites propagate to their children through
existing intrinsic-owner admission. Each Owner carries its own weight multiplier
overlay (alpha 2, beta 3; the second policy variant uses beta 7).

The private front-end helper `rehearsal_ingress_fields.rs`, called from the existing
scenario hydrator, maps native sub_field, property_value, resource_parent,
owner_ref and owner overlay declarations directly into the existing PropertySpec,
SimThing, typed resource-parent edge, intrinsic owner binding and OverlaySpec
representations. It adds no hydrated type, selector-per-participant, registry
authority, engine dependency arrow or runtime producer. RF membership is derived
by ordinary admission from populated properties and parent edges. Documents that
do not use the new cell/edge/binding declarations retain their existing hydrate
stage; no historical source or referee was rewritten.

The source references the existing rehearsal_ingress_cache.base.json dependency
by relative path and pins its bytes in rehearsal_ingress_rf.dependencies.json.
Both native and reproducible-cache routes use the source/cache leaf unchanged.

Owning test:
`crates/simthing-mapeditor/tests/rehearsal_ingress_native_rf.rs::rehearsal_ingress_native_rf_preserves_graph_and_attributable_born_changes`.
One behavior-regression AUDIT / ledger-only inventory row, current-track birth,
DSU 0. No penned referee was consumed or renewed.

## Observed RED

The test verifies all 19 derived participants, unequal RF parentage, both installed
owner overlays, inherited ownership, empty owner spatial seats, and both zero
intrinsic-flow cells. It checks the parsed beta multiplier really changes to 7.
It opens ordinary Studio sessions, runs three ticks, and reads canonical hosted
AllocatedFlow observations. A direct `SimSession::open_from_spec` control with
the same compiled property registry and unchanged program also runs three ticks.
The native, cache and direct programmatic results agree before the RED assertion.

| Variant | beta_3 born allocation | beta born allocation |
| --- | ---: | ---: |
| Baseline (beta flow 8, beta policy 3) | 5.448718 | 21.692308 |
| Unselected child flow 8 -> 20 | 17.448719 | 21.692308 |
| Second owner policy 3 -> 7 | 5.448718 | 21.692308 |

The child contribution discriminator is GREEN on both source routes. The policy
discriminator is RED: `assert_ne!` receives 21.692308 on both sides. Earlier
one-tick probes produced the same values; three ticks do not cure the missing
policy influence. No expected value is injected into the running session.

Raw producer-buffer reads are diagnostics only; they show the post-RF parent
weights are alpha 7 and beta 6 under both policy variants. The canonical born
allocation remains the actual judge. Installed overlay counts are one per Owner;
the parsed policy programs retain multipliers 2/3 or 2/7.

## Boundary and required disposition

Read-only localization: `crates/simthing-driver/src/arena_allocation_plan.rs`,
`plan_arena_allocation_with_pressure`, reduces direct-child weight into BOTH
the parent's weight and weight_sum cells. `sum_reduction_to_targets_ops` uses
CombineFn::Sum, ScaleSpec::Identity and ConsumeMode::ResetTarget. Subsequently
`child_share_eml.rs::compile_child_share_formula_nodes` consumes the child's
weight cell for disbursement. This is consistent with the measured 7/6 child
pressure sums and the erased internal-node policy influence.

The gap is the admitted composition of an authored allocator policy with the
recursive branch-pressure consumer. The existing neutral direct-child pressure
law must remain intact; this packet does not prescribe replacing that sum or
mutating a policy coefficient into it. No caller-side replay, per-leaf policy
copy, post-RF patch, alternate observation or substituted economy is introduced
to make the witness GREEN. Determining the lawful consumer composition or an
explicit admission refusal requires engine/spec authority outside this coding HD.
Return to Orchestration under its genuine-novelty STOP rule; if a generic
capability is missing, its admitted D increment must land with this first consumer.

## Validation and separate baseline issue

- `cargo check -p simthing-mapeditor`: PASS.
- Full mapeditor package: 9 PASS / 1 expected RED, precisely the policy assertion.
  The four bridge/UI tests, three cache tests and two World-root records pass.
- Full ClauseThing package: 63 PASS / 1 FAIL / 2 ignored. The failure is
  `anchor_disposition_admission_0::board_and_orientation_render_property_admission_inventory`.
  Its line 454 compares the current Board pointer `0088-INGRESS-FIDELITY-0` with
  `none` from the closed, hard-coded 0.0.8.7 design document. The test, dispatch
  script, active-track data, design document, orientation and historical HD are
  unchanged from this leaf's base. The failure precedes its live-preview call.
  This is a separate historical pointer-check issue, not a hydrate failure; it
  remains unmodified and is returned for Orchestration's disposition.
- Inventory drift: 964 rows / 1533 discovered / 569 parked; no missing or stale
  rows. Lifecycle schema and diff check pass. Exact-head scan findings and hosted
  reports are in the return; no self-triage.

Reproduce the semantic RED from the committed checkout with:

```text
cargo test -p simthing-mapeditor --test rehearsal_ingress_native_rf -- --test-threads=1 --nocapture
```

The completed bridge/UI and source/cache leaves remain independent reviewable
heads (#2032 and #2033; Board returns 5620231592 and 5620569093). No engine,
protected/census, gate, Cargo, historical test, merge or graduation edit occurs
in this reserve. Remaining hydration/generation, admitted JSON-loader, historical
successor and census work stays open under the same rung.

## Orchestration dispositions before DA relay

`SPEC-LOWERER-KIND-READ` is **GREEN** at Orchestrator tier. The cited `GameSession`
kind read only locates the canonical session container so authored aliases resolve
to existing `SimThingId` identities; no gameplay behavior, RF participation,
allocation law, role semantics, or execution path is selected by kind. The
corresponding triage row is landed on this branch.

The separate ClauseThing package failure is classified **pre-existing harness
error**, not semantic reserve: the historical test reads the closed 0.0.8.7 design
document for its expected pointer while the live Board correctly follows the open
0.0.8.8 track. Astra reproduced the same failure on the unchanged parent head.
It therefore does not contaminate the retained policy RED and is excluded from
the DA semantic question; Orchestration owns repair of that stale pointer referee
separately.
