# 0088 native RF policy-consumption reserve

STOP / proof-present / RED / OPEN / UNMERGED. This packet is a retained first
consumer for an engine-boundary gap, not a GREEN leaf or rung-closeout claim.
Dispatch: Board 5619313604; HD-RECEIPT 70df16e1babd; ORIENT-RECEIPT 28f56884d309.
Base: 5ae17dfb9ee360044e491627fe8f461c81555cb0 (source/cache PR #2033).
Exact retained head and checks are recorded in the PR and Board return.

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