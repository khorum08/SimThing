# NEUTRAL-STREAM-TERMINATION-0 — candidate and zero-carry runtime STOP

Status: **STOP / PROBATION / partial proof-present / DA-review-pending / OPEN / UNMERGED**.
Zero-capable Draw admission is implemented and proved. The admitted-zero matrix now reproduces two runtime failures once both authored demand and carry are zero. The implicated CPU oracle and resident shader remain unmodified and outside this HD.

## Authority and preserved history

- Branch: codex/neutral-stream-termination-0.
- Current amended-master base: 7c06c74948372a7464af7a52d101056570703fc5.
- HD-declared base: 2235d342192dd94c78ea35b5c6bb28dc233597a0.
- Original production base: 164c80158e224d8b9daa52176b232d435a3f640f.
- Owner law5561338557; current resume5561922424; DA zero-Draw ruling5561871558.
- Preserved before-production RED: ac824e2a7e046fd540b6e3c3519bd69d27a1d526.
- First STOP: d02a352cfb1abd436e2407e43ea30487db42fe82 / Board5561504810.
- First DA companion5561543289; prior master merge b5cad3d9.
- Second STOP/candidate: 08b9e9668547ea6d77bf034867eca9d7a5be7a86 / Board5561780100.
- Current normal master merge: 35c0c4fe. No RED/STOP/merge history rewritten.
- HD-RECEIPT: f77fee8211b7 (supersedes fe3d0bdcd784).
- ORIENT-RECEIPT: 8e5e945a589b
- orientation_rule_stamp: bf0f2eeec6f51a1f
- orientation_digest_sha: b63c8c4b90c8b83f3dcae5673f37ec2bbfd5cef7807fd1a06a1733ccf0cc1887

Rendered/read the new coding projection and DA ruling. All63 required anchors were queried and ACKed; the six newly triggered RF-market anchors were read. Their returned bodies overlap within the complete rf-market-mirror-cycle section, verified mechanically; the carried57 hashes did not change. No reorientation.

## Admitted Draw-bound change completed

flow_market.rs changes exactly two lines: the admission guard now rejects only min_quantity > max_quantity, and its existing error text states 0 <= min_quantity <= max_quantity. authorize_draw and all price/weight/lifecycle/identity/settlement/grant semantics remain unchanged.

The workshop fixture uses the real admit_specialization_flow_market door. No fabricated private fields, alternate constructor, Deserialize bypass or new Draw type.

~~~text
Draw [0,0] admitted; inclusive authorization exact
Draw [0,100] admitted; inclusive authorization exact
Draw [1,100] admitted; inclusive authorization exact
Draw [1,0] and [101,100]: InvalidDrawBounds
~~~

For each admitted interval, the referee checks requested0, both endpoints, and max+1 through the existing authorize_draw; out-of-range inputs return the exact QuantityOutsideEnvelope fields. Established-stream refused-zero controls under [1,100] remain GREEN in both postures, with no termination/history append.

## Positive carry GREEN; zero-with-zero-carry runtime RED

One real SimSession per posture, cloned authored fixtures with the same persistent IDs, supply4, admitted Draw[0,100], no persistence deformation:

| Generation | Authored demand | Required/current result |
| --- | --- | --- |
| N1 | 10 | G4/U6 in both postures |
| N2 | 0, still member | effective6 -> G4/U2, no termination |
| N3 | 0, still member | effective2 -> G2/U0, no termination |
| N4 | 0, still member | effective0; resident fails; CPU completes but loses final product observation |
| N5 in CPU case | demand property removed | one termination row with empty final_products, violating claimant/G/U/generation provenance |

Separate retirement cases after N2 and N3 prove the already-born final products G4/U2 and G2/U0 identically in both postures. No termination occurs while authored0 remains a member. The N4/N5 cases use the same ordinary execution and continuation path; they are not helper-only clears.

Actual .git/1511-zero-watchpoint.log excerpt:

~~~text
ResidentRequired N4 authored0:
 GpuSync(GrowthEntitlement("ordinary growth resident clearing failed:
 resident exact output reported a typed GPU product failure"))
 already-born resident facts=[(1,4,6),(2,4,2),(3,2,0)]
 no termination

CpuVendorizedOracle termination N5:
 NeutralStreamTerminationFact {
  granter: SimThingId(21), owner_ref: OwnerRef("owner/15.8"),
  resource_key: "simthing::residency-row-capacity", scope_id: "stead/21",
  termination_generation: GenerationStamp(5), final_products: []
 }
 expected: claimant SimThingId(22), G0/U0, generation4

admitted-zero lifecycle failures: [resident N4 product failure, CPU N5 missing final product]
0 passed; 1 failed; 8 filtered out; cargo exit101
~~~

The referee collects both posture failures before asserting, so one cannot hide the other. It retains boundary-success, no-premature-termination and full provenance expectations.

## Exact out-of-HD loci and requested disposition

**CPU:** crates/simthing-spec/src/spec/constrained_clearing.rs:310 skips every requested==0 claim before building grants. The ordinary CPU continuation records final observations from those already-born grants (growth_entitlement.rs:469); after N4 the result has no claimant product, and neutral departure atN5 emits final_products=[]. This is now the concrete runtime blocker anticipated by DA5561871558, not a static suspicion.

**Resident:** crates/simthing-kernel/src/shaders/resident_clearing_apportionment.wgsl:493 explicitly treats an all-zero requested band as arithmetic overflow:

~~~wgsl
if (overflow || wide_is_zero(band_requested_total)) {
    write_product(current, 0u, 0u, STATUS_ARITHMETIC_OVERFLOW);
    return;
}
~~~

The actual session error is observed at driver resident_clearing_runtime.rs:1444-1445 when materialization sees an unsuccessful resident product. The shader trace explains the all-zero-band failure; the runtime error itself exposes generic ResidentProductFailure, not a separately decoded GPU status. The Rust apportionment mirror already handles zero basis with granted0 at resident_clearing_apportionment.rs:719; this was read only, not changed.

**Return both loci to orchestration/DA for bounded scope admission and disposition of canonical zero-member results.** No change to either out-of-HD surface, no fabricated driver G/U, no empty-set/zero tombstone workaround, no weakening of positive-demand Q149/precedence/cap/identity laws. The HD explicitly requires STOP when the CPU requested-zero skip reproduces an Owner-law failure. Full completion cannot be claimed while the resident zero boundary fails and CPU termination loses claimant provenance.

## Preserved implementation and earlier falsifiers

RED ac824e2a remains the stamped-production falsifier: a never-started departure succeeds without a fact; an established actual resident G4/U6 stream departs and returns DepartingFlowDispositionRequired rather than the Owner-required success (0 passed/1 failed/5 filtered, exit101).

The first candidate exposed an exhaustive replay E0004. Owner-directed STOPd02a352c restored production and returned the exact gap. DA5561543289 admitted the one history-only alternative in owner_channel_rf.rs; only that line changed. The second STOP08b9e966 exposed the now-resolved min_quantity==0 admission refusal. All evidence remains in branch history.

Current candidate: authorize_current_flow visits semantic scope/owner and demand-property presence, passes quantities including0 through the one Draw authority, and sorts claims by persistent ID. An error returns before settlement. Empty membership with a prior continuation terminates before temporal mint/new flow dispatch; an empty never-started stream appends nothing.

The one IntegrationSchedule receives a typed observation with semantic granter/owner/resource/scope, termination generation and final claimant/G/U/source-generation data. Resident provenance references the exact already-materialized history span; CPU provenance retains already-born oracle observations in the existing continuation. Neither feeds observer U into economics. The recorder sorts by claimant ID and derives a deterministic key. The superseded refusal variant/branch is deleted; historical results remain untouched.

Both unchanged ordinary loops use the existing permit before the hot cycle and finish_generation as sole N->N+1 commit. CPU complete-set comparison preserves resident TemporalSourceMismatch, including U0 partial departures. One Draw authority, one resident clearing authority, one IntegrationSchedule/history, one continuation lane, one TreeExecutionLease/private seal; no census growth.

## Other completed live-session proofs

The full departure matrix remains GREEN: owner change/property removal/node removal x step/run/record x resident/CPU =18 established sessions, plus two never-started controls. Each establishes G4/U6 atN1, appends exactly one neutral termination and no other history atN2, leaves all history unchanged atN3/N4, re-enters the same ID with pure authored2 -> G2/U0 atN5, and observes that fresh product on the second terminationN6. Recording also reverses two real RF arenas.

Canonical termination rows/keys compare equal across postures, and actual history roundtrips to identical JSON bytes. Real nonzero RF reduce-up state (count1/surplus3/deficit7, buckets/fold) and one published standing view replay identically with/without termination. The Spec companion is exactly the admitted history-only alternative.

Four partial controls (both postures, supply4/U-positive and supply40/U0) preserve TemporalSourceMismatch. Total departure after that touched failure returns GenerationFaulted at2 before another hot tick; day, identity, GPU values and history stay unchanged. Authored removal uses the existing admitted-tree swap only; no session/coordinator/lease/allocator/buffer/history reconstruction.

## E8 at this resumed source state

Prior pin0x7627_07c7_6e23_ee41 is preserved as intermediate history. After the final two-line flow_market.rs edit, actual SimSession::open refuses before economics:

~~~text
UnqualifiedAdapter {
 required: 8513782173694946881,
 observed: 18356698552482888684
}
cargo exit101
~~~

Observed18356698552482888684 = **0xfec0_1813_55ab_7bec**. Only the production QUALIFIED_RESIDENT_CLEARING_FINGERPRINT and independent QUALIFIED_RECORD_FINGERPRINT literals were rolled to this value. No later bundled-source edits. ABI/child-share/planner/temporal mutation matrix4/4 PASS. No build.rs/comparator/component-list/record/ABI/golden changes. Any future admitted shader/oracle edit requires another final-source-state refusal/observed two-pin cycle; this STOP is not a final graduation pin certificate.

## Validation and limits

- Touched spec/driver/workshop package/tests check PASS; actual tests compile the final witness.
- Focused file8/9; sole failure is admitted-zero lifecycle at N4/N5, not Draw admission.
- Selected9-target regression: **36 passed /1 failed /0 ignored**, cargo exit101 solely for the zero-runtime witness carrying both failures. The other eight targets pass28/28. Committed-head Agent Scan is carried by the Board return. No full workspace or hosted implementation certificate is claimed at STOP.
- Core execution-authority3/3 and qualification mutations4/4 PASS.
- Structural15/15 exit0; inventory1441/1441 missing0/extra0; drift prove, constitutional/selftest, lifecycle schema/prove, digest, detachability/selftest, anchor/selftest, plan/observation/slot/overlay censuses PASS. Existing anchor-coverage advisory remains INSPECT unanchored40/59; final anchor verdict PASS.
- No implementation PR, final clearance, relay-lint, triage mutation, graduation, merge, pointer/canon/closeout/gate edit.

Raw logs: .git/1511-zero-check.log, zero-e8-refusal.log, zero-runtime.log, zero-watchpoint.log, zero-frozen-and-red.log, zero-qualification.log, zero-permit.log, zero-final-*.log. Prior .git/1511-* RED/refusal/STOP logs remain. The Board return binds the committed source head and carries every scan-id-bearing INSPECT.

Exact runtime falsifier command:

~~~text
cargo test -p simthing-workshop --test resident_session_integration_conformance_0 authored_zero_continues_the_stream_or_refuses_at_draw -- --exact --nocapture --test-threads=1
~~~

## Exact changed-file ledger against amended master7c06c749

| Surface | Purpose |
| --- | --- |
| crates/simthing-core/src/generation_stamp.rs | Typed history row/payload/recorder; serde defaults; existing sealed-schedule doctest literal. |
| crates/simthing-core/src/lib.rs | Re-export two observation types. |
| crates/simthing-driver/src/growth_entitlement.rs | Neutral final departure, provenance in existing continuation, complete-set CPU fail-close, delete superseded error. |
| crates/simthing-gpu/src/resident_clearing_runtime.rs | Observed production pin literal only. |
| crates/simthing-spec/src/spec/owner_channel_rf.rs | Exact history-only alternative, one line. |
| crates/simthing-spec/src/spec/flow_market.rs | DA-admitted bound guard and matching error text, two lines. |
| crates/simthing-workshop/tests/resident_session_integration_conformance_0.rs | Departure/zero/bounds/partial/fault/replay proofs; preserves both zero-runtime failures. |
| crates/simthing-workshop/tests/resident_clearing_parity_0.rs | Independent observed pin literal only. |
| docs/tests/neutral_stream_termination_0_results.md | Consolidated evidence/STOP packet and63 ACKs. |
| scripts/ci/test_inventory.tsv | Departure ownership/rename, three added proof rows, existing doctest hash update. |
| scripts/ci/anchor_reach_log.tsv | Three append-only ingress reach rows. |

No edit to constrained_clearing.rs, resident_clearing_apportionment.wgsl or its Rust mirror. Admitted session.rs and driver resident_clearing_runtime.rs remain unchanged. Both normal merges bring only landed HD governance.

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
