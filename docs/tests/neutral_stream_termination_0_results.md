# NEUTRAL-STREAM-TERMINATION-0 — candidate and second scope-gap evidence

Status: **STOP / PROBATION / partial proof-present / DA-review-pending / OPEN / UNMERGED**.
The admitted candidate is preserved on the same branch. The required admitted-zero witness is RED at an out-of-HD market admission rule. This is not a completed implementation certificate.

## Authority and preserved history

- Original base: 164c80158e224d8b9daa52176b232d435a3f640f.
- Owner law5561338557; original dispatch5561439851.
- Preserved before-production RED: ac824e2a7e046fd540b6e3c3519bd69d27a1d526.
- First STOP evidence: d02a352cfb1abd436e2407e43ea30487db42fe82; Board5561504810.
- DA companion admission5561543289; resume dispatch5561575387.
- Amended-HD master: 2235d342192dd94c78ea35b5c6bb28dc233597a0.
- Normal merge on the same branch: b5cad3d9; neither RED nor STOP was rewritten.
- Branch: codex/neutral-stream-termination-0.
- HD-RECEIPT: fe3d0bdcd784 (supersedes dd5a7548635d).
- ORIENT-RECEIPT: 8e5e945a589b
- orientation_rule_stamp: bf0f2eeec6f51a1f
- orientation_digest_sha: b63c8c4b90c8b83f3dcae5673f37ec2bbfd5cef7807fd1a06a1733ccf0cc1887

The amended projection was rendered/read before resuming. The 55 carried anchor hashes are unchanged; the two newly triggered anchors were queried, their relevant binding passages reviewed, and the amended57 ACKs are below. The clausescript-compatibility anchor resolves to the full committed ladder; this packet does not claim a fresh line-by-line reread of every historical rung.

## Blocking scope gap: zero-capable Draw admission

The HD explicitly requires an admitted zero-capable Draw fixture, with authored zero remaining the same stream and prior U participating once. That fixture cannot pass the existing public admission door:

~~~rust
// crates/simthing-spec/src/spec/flow_market.rs:185
if draw.min_quantity == 0 || draw.min_quantity > draw.max_quantity {
    return Err(FlowMarketAdmissionError::InvalidDrawBounds { draw: draw.id });
}
~~~

The error at94 requires 0 < min_quantity <= max_quantity. The new fixture authors min_quantity=0, max_quantity=100 through admit_specialization_flow_market. The actual call returns InvalidDrawBounds before zero-demand temporal execution. AdmittedSpecializationFlowMarket has private fields, no Deserialize implementation and no mutable Draw accessor; its sole struct construction is inside that rejecting admission function. A fabricated admitted market or bypass around authorize_draw would not satisfy the proof.

Ordinary runtime refusal is separately GREEN in both postures: an established G4/U6 stream receives authored0 under its existing [1,100] Draw; the boundary returns QuantityOutsideEnvelope through the Draw error and appends no termination or other history.

~~~text
ResidentRequired: claim quantity 0 is outside Draw envelope draw [1, 100]
CpuVendorizedOracle: claim quantity 0 is outside Draw envelope draw [1, 100]
authored_zero_continues_the_stream_or_refuses_at_draw:
  Err(InvalidDrawBounds { draw: "draw" }) at zero-capable fixture admission
focused file: 7 passed; 1 failed
selected nine-target regression: 35 passed; 1 failed; cargo exit101
~~~

**Requested orchestration disposition:** seek DA admission for crates/simthing-spec/src/spec/flow_market.rs to permit the explicitly authored zero lower bound, define the exact valid bounds (including max=0), align the error text, and retain zero refusal under positive-minimum Draw policies; or amend the HD's proof obligation. No change was made to that file. The amended HD's fence says: "a second out-of-HD production surface remains a STOP."

A bounds amendment alone is not an execution certificate. After admission is resolved, run the blocked resident/CPU zero-carry matrix. Static follow-up: the existing CPU oracle skips requested==0 claims at constrained_clearing.rs:310, so zero-with-zero-carry final-product provenance also needs examination before claiming full lifecycle equivalence. This is not a reproduced runtime failure or authority to edit that additional file.

## Candidate, provenance and unchanged authority census

authorize_current_flow (growth_entitlement.rs:144) walks admitted semantic scope and resolved owner, then tests demand-property presence. Every present exact nonnegative quantity is passed to the one Draw authority and pushed on success. It does not filter requested==0. An error returns before settlement; a refused zero cannot become termination. Claims are canonically sorted by persistent SimThingId.

The first settle_boundary_claims branch (growth_entitlement.rs:231) handles empty membership before new temporal mint, flow dispatch or structural resolution. With a prior continuation it authorizes on the existing permit, appends one typed NeutralStreamTermination fact to the existing IntegrationSchedule, and retires the continuation. Without a prior continuation it appends nothing.

Resident continuation retains its opaque ticket and the exact range of its already-materialized ordinary-flow batch in the one schedule. Termination observes those existing G/U/generation facts. No new departure readback, re-clear, host-U economic input or physical slot/arena scope identity is introduced.

CPU continuation retains the already-born clear result's final product observations in that same continuation, used only for termination. CPU temporal preparation also checks complete canonical source-set equality, matching resident fail-close even when the departed claimant had U0. There is no subset matcher, tombstone or partial retirement.

The fact carries semantic granter/owner/resource/scope, termination generation and final claimant/G/U/source-generation observations. The recorder sorts by semantic claimant ID and derives a deterministic key from the full observation. GrowthEntitlementError::DepartingFlowDispositionRequired and its reachable branch are deleted; historical results are untouched.

Both unchanged SimSession loops acquire the same permit before the hot cycle, call authorize_current_flow and settle_boundary_claims, and commit only through finish_generation. Census remains one recurring-demand Draw authority, one resident clearing authority, one IntegrationSchedule/history, one continuation lane and one TreeExecutionLease/private seal. Constitutional check confirms RESIDENT-CLEARING-PRODUCTION-AUTHORITY=1 and duplicate-settlement/economic-adapter/global-coupling/private-field-solver counts all0. Detachability production_coupling=0, proof_coupling=0.

## Preserved falsifier and resolved first STOP

At RED commit ac824e2a, stamped production was exercised through a real SimSession:

~~~text
ResidentRequired; established=false: boundary succeeds, no facts
ResidentRequired; established=true: prior facts=[(1,4,6)]
departure: Err(GpuSync(GrowthEntitlement(
 "departing ordinary flow requires consequence-only disposition; STOP for DA adjudication")))
FAILED: Owner 15.11 neutral departure must complete the boundary
0 passed; 1 failed; 5 filtered out; cargo exit101
~~~

The first candidate exposed E0004 at owner_channel_rf.rs:724: the new row was absent from its exhaustive replay match. The Owner directed a scope return; the candidate was saved and production restored in d02a352c. DA5561543289 admitted exactly the history-only alternative. That one-line Spec change is now applied; nothing else in that file changed. The resumed package/tests check passes without another exhaustive-consumer failure.

## Live-session proofs completed

The final-claimant matrix exercises **3 causes x 3 loops x 2 postures =18 established sessions**, plus two never-started controls. Causes: owner change, demand-property removal, node removal. Loops: step_once, run(1), record_to_path(...,1). Recording cases reverse the two real RF arenas.

Each established session proves N1 authored10/supply4 -> G4/U6; N2 exactly one termination with semantic provenance and no other appended row; N3/N4 no stream and the whole history unchanged; N5 same persistent claimant re-enters with authored2 -> G2/U0; N6 a second property-removal termination observes that already-born N5 result in both postures.

Canonical termination rows, including keys, compare equal across postures for cloned fixtures with identical IDs. Actual history roundtrips to identical JSON bytes. The authorized Spec consumer proof compares genuine nonzero RF reduce-up state (product_count1, surplus3, deficit7, exact buckets/fold) and one published standing view, with and without the actual termination fact; outputs are equal.

Removal fixtures use the existing public admitted-tree swap only to edit authored membership. Session, coordinator, lease, resident buffers, allocator, continuation and history remain in place; re-entry uses the same claimant ID. No session reconstruction or host-U helper.

Partial controls exercise two claimants in both postures at supply4 (U positive) and supply40 (U0). Removing one while another survives yields the existing TemporalSourceMismatch message and no new history. Removing the final survivor after that touched failure still returns GenerationFaulted for generation2 before another hot tick: day, identity, GPU values and history remain unchanged.

## E8 conditional companion

OLD production/referee pin: 0xf12f_7455_8d8b_dab5 (17379237397841828533).
Actual refusal at SimSession::open, before economics:

~~~text
ResidentClearing(LiveHead(UnqualifiedAdapter {
 required: 17379237397841828533,
 observed: 8513782173694946881
}))
cargo exit101
~~~

Observed8513782173694946881 = **0x7627_07c7_6e23_ee41**. Only QUALIFIED_RESIDENT_CLEARING_FINGERPRINT and independent QUALIFIED_RECORD_FINGERPRINT were updated to that observed value in the same candidate source state. No later bundled-driver edit occurred. No build.rs/component-list/comparator/record/ABI/golden change. Existing ABI/child-share/planner/temporal mutation matrix4/4 passes. Further authorized bundled-source edits require another final-source refusal/observed-pin cycle.

## Validation and limits

- Touched core/spec/driver/workshop package and tests check: PASS.
- Selected workshop regression: **9 groups /35 passed /1 failed /0 ignored**, exit101 solely for admitted-zero fixture admission.
- Other eight targets28/28; five unchanged15.8 tests in the changed file5/5. New executable departure and partial proofs pass. Admitted-zero temporal execution remains unproved.
- Core execution-authority corpus3/3; updated schedule compile-fail1/1 with actual E0451 on private resident_live_head.
- Qualification mutation matrix4/4.
- Structural battery15/15 exit0. Inventory1440/1440, missing0/extra0; drift prove, constitutional check/selftest, lifecycle schema/prove, digest, detachability check/selftest, anchor check/selftest, plan/observation/slot/overlay censuses pass. The schedule compile-fail content-hash ledger row was updated mechanically after adding the new optional payload field.
- Anchor coverage emits its existing advisory INSPECT unanchored=40/59; final ANCHOR-CHECK-VERDICT is PASS.
- Committed-head Agent Scan and any scan-id-bearing INSPECTs are carried by the Board return. Orchestration owns triage.
- Full workspace/all-targets and hosted implementation Scan/Exec are **not claimed** at this STOP. No implementation PR, final clearance, relay-lint, graduation, pointer/canon/gate edit or merge.

Local raw evidence: .git/1511-owner-red.log, check.log, check-tests.log, e8-old-pin-refusal.log, frozen-and-scope-red.log, qualification.log, permit.log, schedule-doctest.log and final-*.log. The Board return binds this packet and sources to the committed head. Excerpts here make the blocker reviewable without local log access.

Regression command:

~~~text
cargo test -p simthing-workshop --test resident_session_integration_conformance_0 --test generation_abort_safety_0 --test tree_execution_authority_lifetime_0 --test exact_cap_projection_0 --test resident_clearing_parity_0 --test resident_clearing_apportionment_0 --test recursion_axis_conformance_0 --test resident_filter_substrate_binding_0 --test recursive_resource_filter_formalization_0 --no-fail-fast -- --nocapture --test-threads=1
~~~

## Changed-file ledger against amended master2235d342

| Surface | Exact purpose |
| --- | --- |
| crates/simthing-core/src/generation_stamp.rs | Typed history payload/row/recorder, serde defaults and existing sealed-schedule doctest literal. |
| crates/simthing-core/src/lib.rs | Re-export two observation types. |
| crates/simthing-driver/src/growth_entitlement.rs | Neutral final departure, continuation provenance, whole-set CPU fail-close, delete superseded error. |
| crates/simthing-gpu/src/resident_clearing_runtime.rs | Observed qualification pin literal only. |
| crates/simthing-spec/src/spec/owner_channel_rf.rs | Exact DA-admitted history-only match alternative, one line. |
| crates/simthing-workshop/tests/resident_session_integration_conformance_0.rs | Expand flipped departure witness; zero and partial/fault controls; replay proof. |
| crates/simthing-workshop/tests/resident_clearing_parity_0.rs | Independent pin literal only; frozen assertions unchanged. |
| docs/tests/neutral_stream_termination_0_results.md | This candidate/STOP packet and ACKs. |
| scripts/ci/test_inventory.tsv | Departure witness rename/ownership; two new proof rows; existing compile-fail hash update. |
| scripts/ci/anchor_reach_log.tsv | Two append-only ingress reach rows. |

Admitted session.rs and driver resident_clearing_runtime.rs need no edits. flow_market.rs remains unchanged and outside the HD. The normal merge brings only the landed HD amendment and creates no delta against amended master.

## Required anchor acknowledgments


ANCHOR-ACK: accumulator-exact-vs-soft-semantics@0efceafc77cf
ANCHOR-ACK: accumulator-op-v2-invariants@32fb4fc36080
ANCHOR-ACK: actionband-8x-sequencing@067ef8ace1e0
ANCHOR-ACK: actionband-axis-budget@52275c538689
ANCHOR-ACK: actionband-binding-laws@d6a8b1b2d673
ANCHOR-ACK: actionband-constitutional-placement@d56d9a04a620
ANCHOR-ACK: actionband-crossing-surface@623db585f145
ANCHOR-ACK: actionband-determinism-lifecycle@6306c484732c
ANCHOR-ACK: actionband-eml-payload-purity@2a1d981f3958
ANCHOR-ACK: actionband-executive@9c7e004e213b
ANCHOR-ACK: actionband-fenced-questions@c40674d92d18
ANCHOR-ACK: actionband-field-triad-authority@56cf5cdf2d2c
ANCHOR-ACK: actionband-gpu-physical-model@3252c1b3c3b5
ANCHOR-ACK: actionband-native-authority-table@541a03cb00a1
ANCHOR-ACK: actionband-performance-model@8d93f06d4bae
ANCHOR-ACK: actionband-target-forms@c3b7bce99f1f
ANCHOR-ACK: actionband-vendorization-direction@20336db0d366
ANCHOR-ACK: admission-ladder-necessity-test@4bedf826f6f7
ANCHOR-ACK: candidate-f-exhaustive-proof-method@7c5ce0b93dab
ANCHOR-ACK: clausescript-compatibility@65b16bee758c
ANCHOR-ACK: core-gpu-residency@f9b19479262a
ANCHOR-ACK: core-overlays@94a8955e46f2
ANCHOR-ACK: core-property-value-model@1be54f2e4803
ANCHOR-ACK: core-rf-arenas@5dd14f66897b
ANCHOR-ACK: eml-admission-shapes@bdcc0b9512f7
ANCHOR-ACK: eml-extension-ladder@7755bc72ffbe
ANCHOR-ACK: eml-integration-plan@8eba54b02320
ANCHOR-ACK: eml-triad-integration@dada7d680557
ANCHOR-ACK: evaluation-identity-invariants@64ad30392930
ANCHOR-ACK: exact-numeric-candidate-f@6938a2efadb5
ANCHOR-ACK: field-policy-time-decisions@4309cdd821fe
ANCHOR-ACK: field-sweep-preservation@acc521a5a361
ANCHOR-ACK: founding-ontology-invariants@46802793fba7
ANCHOR-ACK: intrinsic-constrained-clearing@957b7c81b756
ANCHOR-ACK: movement-front-adjudications@5af6a29acb75
ANCHOR-ACK: one-tree-owners-never-spatial@9a10c1be61ee
ANCHOR-ACK: orientation-harness-core@8a365d1c0864
ANCHOR-ACK: overlay-closure-thesis@241cc54c5706
ANCHOR-ACK: overlay-designer-closure@4a047b29243d
ANCHOR-ACK: overlay-germ@f0c8d2ebade9
ANCHOR-ACK: overlay-promoted-laws@248c7893b462
ANCHOR-ACK: overlay-scale-laws@c2ffb2826df7
ANCHOR-ACK: rf-arena-allocation-invariants@82864469489b
ANCHOR-ACK: rf-arena-substrate@17b5f1e5c2ba
ANCHOR-ACK: scanner-selftest-delta-gate@34fb2662baae
ANCHOR-ACK: seal-residue-cross-crate@c61c33d90efc
ANCHOR-ACK: simthing-0087-binding-laws@567370293add
ANCHOR-ACK: simthing-0087-pillars@61487cba1f9e
ANCHOR-ACK: stead-events-are-rf@1f3bdde23cee
ANCHOR-ACK: stead-rejected-shapes@7f75f8b55271
ANCHOR-ACK: stead-shared-surface-ledger@2d7062067214
ANCHOR-ACK: stead-spatial-contract-core@8585db4ac631
ANCHOR-ACK: stemthing-binding-laws@6787a118c3ca
ANCHOR-ACK: stemthing-lane-not-leg@9a1d443b7981
ANCHOR-ACK: stemthing-slot-identity-ruling@02c87b9126e1
ANCHOR-ACK: structural-execution-convergence@6b4cedec482b
ANCHOR-ACK: workshop-candidate-homing@3e584f0ad175
