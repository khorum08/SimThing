# NEUTRAL-STREAM-TERMINATION-0 — canonical zero results and initial-zero planner STOP

Status: **STOP / PROBATION / partial proof-present / DA-review-pending / OPEN / UNMERGED**.
Both newly admitted production companions are implemented. The complete established-stream N1-N5 zero-member matrix now passes in both postures. An initial-zero member still disappears in the resident planner, producing no result at N1 and empty termination provenance at N2. The Rust planner is outside this HD and remains unchanged. Frozen parity also exposes the conflicting zero-omission rule; no referee was weakened.

## Authority and preserved history

- Branch: codex/neutral-stream-termination-0.
- Current amended-master comparison base: 27aa7060a39f410879b82f212a21fc9989ae04a0 (PR #1987).
- HD-declared base: 7c06c74948372a7464af7a52d101056570703fc5.
- Original production base: 164c80158e224d8b9daa52176b232d435a3f640f.
- Owner law5561338557; resume5562365470; binding DA canonical-zero ruling5562198662.
- Preserved before-production RED: ac824e2a7e046fd540b6e3c3519bd69d27a1d526.
- First STOP: d02a352cfb1abd436e2407e43ea30487db42fe82 / Board5561504810; DA companion5561543289; normal master merge b5cad3d9.
- Second STOP: 08b9e9668547ea6d77bf034867eca9d7a5be7a86 / Board5561780100; DA Draw ruling5561871558; normal master merge35c0c4fe.
- Third STOP: a553f4419d1f688d20bb976e44becd8a50172919 / Board5562050572.
- Current normal master merge: 51ff290e. No falsifier/STOP/merge rewritten, replanted, rebased or squashed.
- HD-RECEIPT: 1d07d4437e91 (supersedes f77fee8211b7).
- ORIENT-RECEIPT: 8e5e945a589b
- orientation_rule_stamp: bf0f2eeec6f51a1f
- orientation_digest_sha: b63c8c4b90c8b83f3dcae5673f37ec2bbfd5cef7807fd1a06a1733ccf0cc1887

Rendered/read the new coding projection and DA ruling. The same63 required anchors and hashes carry forward from the prior ingress, with no anchor delta or new reach row. No reorientation. Exact committed return head and its fresh Agent Scan are recorded on the Board.

## Newly admitted implementation

Resident WGSL changes one condition: `if (overflow || wide_is_zero(band_requested_total))` becomes `if (overflow)`. Real overflow retains the same refusal branch. A lawful all-zero live band reaches the existing zero-basis STATUS_OK result. No Q149, cap, Hamilton, tie, precedence, supply, product/status ABI or positive-demand code changed.

CPU constrained clearing no longer drops requested0 before the ordinary authored program scores the claim. After the existing canonical sort, each zero row is sealed through the existing ConstrainedGrant::from_clearance with ordinary scope/source/priority/order-weight/score/generation and requested0/granted0/unresolved0. Those rows are removed from the allocation worklist before any denominator, cap, Hamilton or tie arithmetic. Positive rows keep the same relative order, scores and supply; the existing final source sort canonicalizes all returned results. No second scorer or authority, and no change to grant-lifecycle zero-grant rejection. Session publication continues to omit zero-valued lifecycle relations.

## Complete N1-N5 matrix GREEN

`authored_zero_continues_the_stream_or_refuses_at_draw` uses real admitted Draw[0,100], one persistent claimant, supply4 and actual SimSession execution in both ResidentRequired and CpuVendorizedOracle:

| Generation | Authored demand / membership | Observed result in both postures |
| --- | --- | --- |
| N1 | 10, live | G4/U6 |
| N2 | 0, live, priorU6 | G4/U2; no termination |
| N3 | 0, live, priorU2 | G2/U0; no termination |
| N4 | 0, live, priorU0 | canonical claimant-bearing G0/U0; no termination |
| N5 | actual demand-property removal | exactly one termination with claimant, full scope, G0/U0@generation4 |

Separate actual departures after N2/N3 retain G4/U2@generation2 and G2/U0@generation3 identically across postures. The previous resident N4 product failure and CPU N5 empty provenance are resolved. Real Draw[1,100] refused-zero controls still fail at the existing QuantityOutsideEnvelope door before termination/history append in both postures.

## New concrete initial-zero STOP

`initial_zero_members_produce_same_canonical_result_in_both_postures` starts actual admitted sessions with authored0 at the first boundary, using Draw[0,0] and Draw[0,100] in both postures. Each case keeps the ordinary admission/session/continuation path; no positive placeholder or fixture bypass is introduced. The referee requires canonical G0/U0@generation1, no termination while live, no positive grant-lifecycle relation, and exact final claimant provenance on actual property removal atN2.

Observed in .git/1511-result-runtime.log:

~~~text
initial zero [0,0] ResidentRequired:
 N1 Ok(StepOnceOutcome { ticks_run:1, boundaries_run:1, boundary_reached:true })
 resident facts=[]                  (required: [(1,0,0)])
 N2 one NeutralStreamTerminationFact, final_products=[]
initial zero [0,100] ResidentRequired: same omission
CpuVendorizedOracle: both intervals retain exact G0/U0@generation1
 and pass the subsequent full-provenance termination assertion

initial_zero_members_produce_same_canonical_result_in_both_postures FAILED
focused file: 9 passed; 1 failed; 0 ignored; cargo exit101
~~~

The CPU has no resident-history rows by design; its initial-zero result is independently observed through the already-born CPU continuation on departure. The failures collected by this matrix are resident-only. Initial zero is not rejected, but its required product identity is lost.

Exact unadmitted production locus: `crates/simthing-kernel/src/resident_clearing_apportionment.rs:387-394`, ResidentApportionmentPlan::build:

~~~rust
// The frozen CPU authority performs identity/supply admission above,
// then silently omits zero-request rows before apportionment. Preserve
// that full u32 request domain rather than rejecting or materializing
// a zero grant.
let claims = claims
    .into_iter()
    .filter(|claim| claim.requested != 0)
    .collect();
~~~

This removes live initial-zero identities before either the Rust exact mirror or resident shader can materialize them. The established temporal path keeps admitted claim rows while resident effective demand becomes0, explaining why N4 is now green but initialN1 is not. The driver planner delegates to this build function; no edit to either unadmitted surface.

The required unmodified referee `neutral_continuous_shares_match_frozen_cpu_law_across_boundary_cases` in `crates/simthing-workshop/tests/resident_clearing_apportionment_0.rs` includes requests[0,5]/supply3 as `zero-request-omission`. Its actual CPU authority now retains the zero source, while the unmodified resident plan omits it. It fails at line465 before the subsequent GPU comparison; no claim that the failing assertion itself compared GPU output:

~~~text
assertion left == right failed: CPU mirror: zero-request-omission
 left: {SimThingId(1001): (3,2)}
right: {SimThingId(1000): (0,0), SimThingId(1001): (3,2)}
apportionment target: 6 passed; 1 failed; cargo exit101
~~~

The positive claimant is unchanged in this mixed case. The five preceding boundary cases completed their CPU/GPU comparisons before this zero-source mismatch. Later statements in this one failing function were not reached; other exact-cap/no-collision and apportionment functions passed independently.

**Requested disposition:** orchestration/DA must resolve the Rust planner's initial-zero omission and the frozen zero-omission reference contract, including bounded surface admission if required. HD1d07d4437e91 explicitly keeps the Rust resident oracle unchanged and requires STOP for any new out-of-HD production surface. No edit to the planner, frozen apportionment referee, grant-lifecycle rule, ABI, or positive-demand arithmetic; no dummy positive demand or driver-fabricated G/U. This return does not claim graduation readiness.

## Preserved implementation and proofs

RED ac824e2a remains the before-production actual-session falsifier: never-started departure succeeds without a fact; established resident G4/U6 departure returned the superseded DepartingFlowDispositionRequired (0pass/1fail/5filtered, exit101). The three STOPs preserve the exhaustive replay E0004, zero-Draw admission refusal, and N4/N5 zero-runtime failures respectively. Their later admitted remedies do not rewrite the evidence.

Current authorize_current_flow uses owner/semantic scope and demand-property presence, passes quantities including0 through the sole Draw authority and sorts claims by persistent identity. Empty membership with a prior continuation terminates before temporal mint/new flow dispatch; a never-started empty stream appends nothing. The one IntegrationSchedule receives typed granter/owner/resource/scope, termination generation and final claimant/G/U/source-generation provenance. Resident observations refer to the exact already-materialized history span; CPU observations retain already-born result data in the same continuation. Observer U never feeds later economics. Canonical source sorting and the deterministic stable key remain in the existing history.

The18 established departure sessions (owner change/property removal/node removal x step/run/record x both postures) pass, plus two never-started controls. Each establishes G4/U6 atN1, appends exactly one neutral termination and no other history atN2, keeps all history unchanged atN3/N4, re-enters the same ID with pure authored2 -> G2/U0 atN5, then records that fresh product on departureN6. Recorded cases reverse two real RF arenas. Canonical termination rows/keys match across postures and history roundtrips to identical JSON bytes. Real nonzero RF reduce-up state (count1/surplus3/deficit7) and a published standing view replay identically with/without termination; owner_channel_rf.rs remains exactly the admitted history-only alternative.

Four partial controls (both postures, supply4/U-positive and supply40/U0) preserve TemporalSourceMismatch. Subsequent total departure after that touched failure returns GenerationFaulted at2 before another hot tick; day, identity, GPU values and history remain unchanged. Both ordinary loops retain the existing permit and sole finish_generation commit. Authored removal uses the existing admitted-tree swap, without replacing session/coordinator/lease/allocator/buffers/history.

The real Draw admission matrix passes [0,0], [0,100], [1,100], inclusive endpoints and max+1/positive-minimum-zero refusals, plus typed InvalidDrawBounds for [1,0]/[101,100]. flow_market.rs remains exactly the admitted two-line guard/error-text change. No private-field construction or alternative Draw admission.

Authority census remains **one Draw authority / one resident clearing authority / one IntegrationSchedule and history / one continuation lane / one TreeExecutionLease and private seal**.

## E8 at this preserved source state

After both newly admitted production edits, the prior pin0xfec0_1813_55ab_7bec refuses actual SimSession::open before economics:

~~~text
UnqualifiedAdapter {
 required: 18356698552482888684,
 observed: 13527848702859969630
}
cargo exit101
~~~

Observed13527848702859969630 = **0xbbbc_92b8_bd32_845e**. Only QUALIFIED_RESIDENT_CLEARING_FINGERPRINT and independent QUALIFIED_RECORD_FINGERPRINT were rolled to this identical value, in the same source-state commit as both admitted fixes. No bundled-source edits afterward. No build.rs, component list, comparator, qualification-record/ABI shape, golden, dynamic pin or alternate qualification mechanism changed. The current STOP preserves this observed candidate qualification, not a final graduation certificate; any later admitted bundled edit requires another real refusal/observed roll.

## Validation and limits

- Touched kernel/spec/driver/workshop package/tests check PASS. The final tests compile both observed pin changes and the new initial-zero witness.
- Focused session file: **9 passed /1 failed /0 ignored**, 43.94s, exit101 solely for initial-zero resident result/provenance omission. The complete N1-N5 matrix and all five unchanged 15.8 tests pass.
- Other eight selected workshop targets: **27 passed /1 failed /0 ignored**, exit101 solely for the frozen zero-omission assertion. Seven whole targets pass; apportionment is6/7. Across all nine targets: **36 passed /2 failed /0 ignored**.
- Exact-cap projection3/3; generation-abort3/3; recursion-axis5/5; recursive RF1/1; independent resident parity1/1 at fingerprint bbbc92b8bd32845e; substrate-binding4/4; execution-lifetime4/4. Passing apportionment functions include overflow refusal, exact ties, caps, no-collision/physical permutation and recursive intake; the one failing function's later statements are not certified.
- Qualification ABI/child-share/planner/temporal mutation matrix **4/4 PASS**. Core execution-authority **3/3 PASS**, private IntegrationSchedule compile-fail **1/1 PASS** (E0451). Two additional filtered spec/kernel commands exited0 but selected0 tests; they are compile evidence only, not runtime certificates.
- Structural **15/15 exit0**: inventory1442/1442 missing0/extra0, drift prove, constitutional check/selftest, lifecycle schema/prove, digest, detachability/selftest, anchor check/selftest, plan/observation/slot/overlay censuses. Expected planted FAILs within selftests were detected and their selftests passed. Existing anchor-coverage advisory remains INSPECT unanchored40/59; final anchor verdict PASS.
- Source diff confirms Rust resident planner/reference, frozen apportionment referee, session.rs and driver resident_clearing_runtime.rs unchanged versus amended master. No runtime source edit followed the observed E8 roll.

No full workspace or hosted implementation certificate is claimed at this explicit scope STOP. No implementation PR, clearance, relay-lint, triage mutation, graduation, PR merge, pointer/canon/closeout/gate edit. A fresh committed-head Agent Scan and every scan-id-bearing INSPECT are attached to the Board return; final implementation-head triage remains orchestration's responsibility under DA5562198662.

Raw logs: .git/1511-result-check.log, result-e8-refusal.log, result-runtime.log, result-frozen.log, result-qualification.log, result-permit.log, result-compilefail.log, result-spec-clearing.log, result-kernel-exact.log, result-final-*.log, result-agent-scan.log (all result-prefixed names expand under .git/1511-).

## Changed-file ledger relative to amended master

| Path | Owned purpose |
| --- | --- |
| crates/simthing-core/src/generation_stamp.rs | Typed neutral observation and one canonical recorder; entry default fields and existing doctest literal. |
| crates/simthing-core/src/lib.rs | Re-export existing-history observation types. |
| crates/simthing-driver/src/growth_entitlement.rs | Membership/Draw separation, pre-remint neutral termination, existing continuation observations and complete-set CPU fail-close. |
| crates/simthing-gpu/src/resident_clearing_runtime.rs | Observed production pin literal only. |
| crates/simthing-spec/src/spec/owner_channel_rf.rs | One history-only replay alternative. |
| crates/simthing-spec/src/spec/flow_market.rs | DA-admitted bound guard and matching error text, two lines. |
| crates/simthing-kernel/src/shaders/resident_clearing_apportionment.wgsl | One all-zero-band classification condition. |
| crates/simthing-spec/src/spec/constrained_clearing.rs | Canonical zero-result retention using ordinary scorer/seal, excluded from allocation arithmetic. |
| crates/simthing-workshop/tests/resident_session_integration_conformance_0.rs | Departure/zero/bounds/partial/fault/replay proofs; initial-zero STOP matrix retained. |
| crates/simthing-workshop/tests/resident_clearing_parity_0.rs | Independent observed pin literal only. |
| docs/tests/neutral_stream_termination_0_results.md | Consolidated evidence/STOP packet and63 ACKs. |
| scripts/ci/test_inventory.tsv | Departure ownership/rename, four added proof rows, existing doctest hash update. |
| scripts/ci/anchor_reach_log.tsv | Three append-only prior ingress reach rows; no new anchor delta this resume. |

Admitted session.rs and driver resident_clearing_runtime.rs remain unchanged. The Rust kernel planner/reference and frozen apportionment referee remain unchanged. Normal master merges bring landed HD governance only.

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
ANCHOR-ACK: rf-market-candidate-laws@357f6c986fff
ANCHOR-ACK: rf-market-falsifiers@d40df3102410
ANCHOR-ACK: rf-market-mirror-cycle@1a1aca57e5f6
ANCHOR-ACK: rf-market-port-census@3bc7792c27e3
ANCHOR-ACK: rf-market-receive-not-recompute@a9856f7f3bc1
ANCHOR-ACK: rf-market-settled-code-census@3bc7792c27e3
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
