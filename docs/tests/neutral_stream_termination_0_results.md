# NEUTRAL-STREAM-TERMINATION-0 — admitted referee corrections and recursive-root STOP

Status: **STOP / PROBATION / partial proof-present / DA-review-pending / OPEN / UNMERGED**.
Both referee corrections admitted by DA 5563027436 are GREEN. The terminal parity referee now reaches one later recursive-root assertion that still omits a lawful zero companion. That later block is explicitly outside the remand's semantic-edit allowance and remains unchanged. Production and both E8 pins are unchanged. The return requests admission to correct this remaining test contract.

## Binding state and preserved history

- Branch: codex/neutral-stream-termination-0.
- Amended-master comparison base: 369358733df35c1f556fe4c8844df20223dd776d (PR #1989).
- HD-declared base: c5d7d1de28d183087862f00cfacf7bb9341b9e84.
- Original production base: 164c80158e224d8b9daa52176b232d435a3f640f.
- Owner neutral-departure law5561338557; current dispatch5563145290; DA referee ruling5563027436.
- Preserved before-production RED: ac824e2a7e046fd540b6e3c3519bd69d27a1d526.
- STOP1: d02a352cfb1abd436e2407e43ea30487db42fe82 / Board5561504810; DA replay companion5561543289; normal master merge b5cad3d9.
- STOP2: 08b9e9668547ea6d77bf034867eca9d7a5be7a86 / Board5561780100; DA Draw companion5561871558; normal master merge35c0c4fe.
- STOP3: a553f4419d1f688d20bb976e44becd8a50172919 / Board5562050572; DA canonical-zero companions5562198662; normal master merge51ff290e.
- STOP4: d634130fec8f226cecf13596e38a546f72ca5e1c / Board5562461195; DA planner companion5562607392; normal master mergea1357473.
- STOP5: 7d4b3b38a2f4309a8fa376b429ffd103b04d7a2a / Board5562855342.
- Current normal master merge: 5e50997f. No RED/STOP/merge rewritten, replanted, rebased or squashed.
- HD-RECEIPT: 252edc08cfb2 (supersedes accde9f346a7).
- ORIENT-RECEIPT: 8e5e945a589b
- orientation_rule_stamp: bf0f2eeec6f51a1f
- orientation_digest_sha: b63c8c4b90c8b83f3dcae5673f37ec2bbfd5cef7807fd1a06a1733ccf0cc1887

Rendered/read the current coding projection and DA ruling. All 63 previously queried anchor ACKs carry forward unchanged, with no new reach row or anchor delta. Rule-source files are unchanged; no reorientation. Exact tested/committed head and fresh Agent Scan are bound in the Board return.

## Admitted dual-digest contract GREEN

`no_collision_products_remain_bit_identical_to_dispatched_master` retains the identical 338 fixtures, case count, planner/executor path and per-case CPU/GPU equality assertions. Every product remains in the full canonical digest. A second digest covers products whose admitted claim has a nonzero request, including G0/U-positive results.

Both assertions are live and permanent:

~~~text
cases=338
historical-positive-request-digest=05cb01d96dc69dbe
retained-canonical-digest=f967436fc86a9690
retained-zero-rows=1
~~~

The sole zero-request row is explicitly constrained to case 325, source 1000, semantic row 0, requested 0/G0/U0, generation 4, STATUS_OK and integration band 3. That case must have exactly two products; positive source 1001 remains G3/U2. The historical positive byte digest binds all positive-request outputs, and the retained-stream digest additionally binds the zero row's bytes and position. The original historical digest was not replaced or weakened. Exact-cap target: 3/3, including 12 active-set cases across 3104 GPU physical runs and the Owner cap-collision CPU/GPU/workgroup/partition witness.

The prior STOP's independent scratch diagnostic reproduced the old digest exactly with the old plan filter and proved all positive product bytes identical across 338 cases. The admitted test now carries both contracts permanently without any alternate production solver or fixture mutation.

## Admitted parity zero block GREEN

Only the [0,1,1]/supply 2 assertion block changes semantically. The plan and GPU result each contain 3 identities. The plan retains source 0 with requested 0; CPU/GPU canonical vectors compare equal. The complete result map is source 0 G0/U0, source 1 G1/U0, source 2 G1/U0. The zero product retains its admitted semantic row; all products retain generation 0, integration band 1 and STATUS_OK. Total grant is 2, consumed entirely by the two positive members.

~~~text
15.11 parity retained-zero block:
 source1000 G0/U0; source1001 G1/U0; source1002 G1/U0
 generation0 / integration_band1 / STATUS_OK / total grant2
~~~

All following invalid-value, failed-dispatch recovery, signed-zero and overflow assertions in the negative matrix execute unchanged and pass before the later recursive-root failure. The independent pin remains0x64c8_2fb4_de76_90ac. Source comparison verifies that everything before and after the admitted zero block is byte-for-byte unchanged from STOP5, including the entire recursive and final qualification functions.

## Remaining recursive-root assertion — STOP before editing

Exact locus at this candidate: `crates/simthing-workshop/tests/resident_clearing_parity_0.rs:1488-1491`, inside `three_recursive_edges_self_consume_exact_ts_and_u_recurs_once_at_n_plus_one` (the equivalent assertion was at line 1451 before the admitted block grew).

The unchanged root fixture deliberately uses requests [10,0], supply 8 and generation 10. Its comment says the zero-request companion preserves the same two-row resident shape without competing for the eight conserved units. The old result-map assertion nevertheless requires only source 1000 G8/U2:

~~~text
assertion left == right failed at1488
 actual: {1000: (8,2), 1001: (0,0)}
expected: {1000: (8,2)}
resident_clearing_parity_terminal_referee:0 passed;1 failed;exit101
~~~

The result now retains the lawful zero companion. Positive source1000 remains G8/U2. The assertion must be reconciled with the canonical zero-member law, but HD252edc08cfb2 admits semantic changes only inside the earlier [0,1,1] block and explicitly requires all later recursive logic to remain unchanged. It also requires STOP while a non-superseded witness remains red after the correction. **Return this root-result map assertion for bounded DA test-contract admission.** No production change, E8 roll, fixture mutation, source suppression, supply/carry change or official later-assertion edit was made.

The official terminal referee stops here. Its later recursive edges and final qualification phase cannot be claimed as a passing whole-referee certificate.

## Separate diagnostic of the unexecuted tail

To make the scope return concrete and check for another hidden failure, an untracked .git diagnostic reuses the current fixture source and already-built candidate libraries. It does not write the official referee or production source.

- One diagnostic invokes the final `scale_multitree_physical_invariance_and_exact_qualification_hold` function unchanged. All its scale/physical-shape/multitree checks and qualification tuple mutants pass; its independent record reports64c82fb4de7690ac.
- A scratch copy of the recursive function changes only the root map expectation to include source1001 G0/U0 and adds stronger root assertions for requested0, semantic-row identity, STATUS_OK, generation10, integration band10 and total grant8. Every subsequent recursive/temporal assertion remains unchanged. The literal chain G8/U2 -> G6/U2 -> G4/U2 passes; N10->N11 produces authored2+U2 and the once-mint refusal passes.

~~~text
DIAGNOSTIC ROOT:
 source1000 G8/U2; source1001 G0/U0; generation10; band10; STATUS_OK; total grant8
DIAGNOSTIC recursive tail PASS:
 unchanged literal chain G8/U2 -> G6/U2 -> G4/U2;
 N10->N11 authored2+U2; once-mint refusal
RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT:64c82fb4de7690ac
DIAGNOSTIC unchanged final scale/multitree/qualification function PASS
2 passed;0 failed;1 official test filtered;3.74s;exit0
~~~

This is bounded diagnostic evidence for the requested correction, not an assertion that the official terminal referee passed. No new permanent test or alternate production arithmetic was introduced. The Board packet includes the reproduction helper and labels the scratch-only expected-map correction explicitly.

## Preserved session, zero-member and frozen proofs

The unchanged initial-zero matrix uses real Draw[0,0] and Draw[0,100] in both ResidentRequired and CpuVendorizedOracle. N1 authored/effective0 is a live member with canonical claimant-bearing G0/U0@generation1; there is no termination or positive grant-lifecycle relation. Actual property removal atN2 emits exactly one termination with full claimant/scope/granter identity and G0/U0@N1 provenance. Resident history reports[(1,0,0)]; CPU already-born provenance agrees.

The unchanged established-zero matrix uses real Draw[0,100], one persistent member and supply4 in both postures:

| Boundary | Demand/membership | Result |
| --- | --- | --- |
| N1 | authored10, live | G4/U6 |
| N2 | authored0 + priorU6, live | G4/U2; no termination |
| N3 | authored0 + priorU2, live | G2/U0; no termination |
| N4 | authored0 + priorU0, live | canonical G0/U0; no termination |
| N5 | actual property removal | one termination carrying G0/U0@generation4 |

Separate retirement after N2/N3 retains exact G4/U2@generation2 and G2/U0@generation3. Real Draw[1,100] requested0 still returns QuantityOutsideEnvelope before history append in both postures. Draw admission/authorization also proves [0,0], [0,100], [1,100], inclusive endpoints, max+1 refusal, and typed InvalidDrawBounds for [1,0]/[101,100].

All18 established departure sessions pass: owner change/property removal/node removal x step/run/record x both postures, plus two never-started controls. Each establishes G4/U6 atN1, appends one neutral fact and no other history atN2, leaves history unchanged atN3/N4, re-enters the same ID with pure authored2 -> G2/U0 atN5, then observes that fresh product on departureN6. Recorded cases reverse two real RF arenas. Canonical rows/keys match across postures and history roundtrips to identical JSON bytes.

Both-posture partial-set controls preserve TemporalSourceMismatch with U-positive and U0. Subsequent total departure after the touched failure returns GenerationFaulted at2 before another hot tick; day, identity, GPU values and history remain unchanged. Real RF reduce-up state (count1/surplus3/deficit7) and a published standing view replay identically with/without actual termination. Authored membership mutation uses the existing admitted-tree swap, without session/lease/buffer/history reconstruction.

`resident_clearing_apportionment_0.rs` remains byte-for-byte unchanged and passes7/7. Its [0,5]/supply3 case passes Rust mirror and GPU assertions with source0 G0/U0 and source1 G3/U2. All initial/established/termination/partial/fault session matrices are unchanged from the preserved production candidate; session integration10/10.

## Production and E8 remain unchanged

No production or sealed source changed in this resume. The existing implementation separates membership presence from Draw quantity, terminates an empty established stream before temporal remint, and observes only already-born resident-history spans or CPU results in the existing continuation. One IntegrationSchedule records deterministic typed final claimant/G/U/generation provenance; observer U never feeds economics. Zero clearing results use the ordinary scorer/seal, consume no supply and do not participate in positive allocation; positive lifecycle publication and zero-grant rejection remain unchanged. The resident shader retains its one classification correction; the planner retains admitted zero identities; all positive allocation and15.10 permit arithmetic/lifecycle remain frozen.

Authority census remains **one Draw / one resident clearer / one IntegrationSchedule and history / one continuation lane / one TreeExecutionLease and private seal**.

The final fourth E8 roll remains at **0x64c8_2fb4_de76_90ac** in both fixed literals. Preserved actual SimSession::open refusal before economics:

~~~text
old required:13527848702859969630 =0xbbbc_92b8_bd32_845e
observed:7262106853007855788 =0x64c8_2fb4_de76_90ac
UnqualifiedAdapter;0 passed/1 failed/9 filtered;exit101
~~~

The two literals were rolled together with the final planner edit at STOP5; no sealed-source edit followed. No fifth roll occurred for this test-only remand. Qualification ABI/child-share/planner/temporal mutation matrix4/4 passes again. The separate unchanged final-phase diagnostic confirms the independent record; the official whole parity referee remains blocked as described above. No build.rs/component-list/comparator/record/ABI/dynamic-pin change.

## Validation and limits

- Six touched packages/tests check PASS: core, kernel, GPU, Spec, driver, workshop.
- Selected9-target corpus: **37 passed /1 failed /0 ignored**,88.14s, exit101 only for the later recursive-root map assertion. Exact-cap3/3, session integration10/10, frozen apportionment7/7, generation-abort3/3, recursion-axis5/5, recursive RF1/1, substrate-binding4/4, execution-lifetime4/4; terminal parity0/1. Every admitted correction passes.
- Qualification4/4; core execution-authority3/3; actual private IntegrationSchedule compile-fail1/1 (E0451). Separate scratch-tail diagnostics2/2 are not counted in the official corpus or inventory.
- Structural15/15 exit0: inventory1442/1442 missing0/extra0, drift prove, constitutional check/selftest, lifecycle schema/prove, digest, detachability/selftest, anchor check/selftest, plan/observation/slot/overlay censuses. Expected planted failures were caught by passing selftests. Existing anchor-coverage advisory remains INSPECT unanchored40/59 with final PASS.
- Full workspace/all-targets: **145 groups /550 passed /1 failed /14 ignored**. Command: cargo test --workspace --all-targets --no-fail-fast -j 1 --quiet. Exit/elapsed: 101 (219.09s). The sole failure is the unchanged recursive-root map in the terminal parity referee.
- Source audit:15 changed paths relative to amended master, all in HD. This resume changes only the dual-digest function and allowed parity zero block plus this results packet. Fixtures/executors/case count, all later parity source, frozen apportionment and session matrices, production and both pins remain unchanged. All63 anchor ACKs and RED/five-STOP ancestry are preserved. Generated baseline output restored after the workspace run.

No green full-workspace or hosted implementation certificate is claimed at this explicit later-assertion STOP. No implementation PR, triage mutation, clearance, relay-lint, graduation, merge, pointer/canon/closeout/gate edit. Fresh committed-head Agent Scan and every scan-id-bearing INSPECT are bound in the Board return; final implementation-head triage remains orchestration-owned.

Logs: .git/1511-referee-handoff.txt, referee-check.log, referee-frozen.log, referee-qualification.log, referee-permit.log, referee-compilefail.log, referee-workspace.log, referee-final-*.log, referee-audit.log, referee-tail-diagnostic.log, referee-agent-scan.log (all referee-prefixed names expand under .git/1511-). Prior refusal: .git/1511-plan-e8-refusal.log.

## Changed-file ledger relative to amended master

| Path | Owned purpose |
| --- | --- |
| crates/simthing-core/src/generation_stamp.rs | Typed neutral observation and one canonical recorder; entry defaults and existing doctest literal. |
| crates/simthing-core/src/lib.rs | Existing-history observation re-exports. |
| crates/simthing-driver/src/growth_entitlement.rs | Membership/Draw separation, pre-remint termination, existing provenance and complete-set CPU fail-close. |
| crates/simthing-gpu/src/resident_clearing_runtime.rs | Final observed production pin literal only; unchanged this resume. |
| crates/simthing-spec/src/spec/owner_channel_rf.rs | One history-only replay alternative. |
| crates/simthing-spec/src/spec/flow_market.rs | Admitted zero-capable bound guard/error text, two lines. |
| crates/simthing-spec/src/spec/constrained_clearing.rs | Canonical zero results from ordinary scorer/seal, excluded from allocation arithmetic. |
| crates/simthing-kernel/src/shaders/resident_clearing_apportionment.wgsl | One all-zero-band classification condition. |
| crates/simthing-kernel/src/resident_clearing_apportionment.rs | Remove post-admission zero omission; retain admitted identities. |
| crates/simthing-workshop/tests/resident_session_integration_conformance_0.rs | Preserved departure/zero/bounds/partial/fault/replay matrices. |
| crates/simthing-workshop/tests/exact_cap_projection_0.rs | Admitted dual-digest function only; unchanged338 fixtures and executor path. |
| crates/simthing-workshop/tests/resident_clearing_parity_0.rs | Final independent pin plus admitted [0,1,1] zero block only; recursive-root assertion untouched. |
| docs/tests/neutral_stream_termination_0_results.md | Consolidated admitted proofs, remaining root-map STOP,63 ACKs. |
| scripts/ci/test_inventory.tsv | Preserved departure ownership/rename, four proof rows, doctest hash; no new row this resume. |
| scripts/ci/anchor_reach_log.tsv | Three preserved ingress reach rows; no new row this resume. |

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
