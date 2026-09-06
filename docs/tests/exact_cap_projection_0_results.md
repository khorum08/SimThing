# EXACT-CAP-PROJECTION-0 — implementation evidence

Status: **PROBATION / proof-present / DA-review-pending / OPEN / UNMERGED**. CAP COLLISION = SATURATE AND REDISTRIBUTE is implemented in the existing CPU reference and production WGSL authority. The immutable PR/board return binds this document to the tested implementation head and hosted run IDs; orchestration owns final triage and graduation routing.

- Dispatch: [5552404418](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5552404418).
- Scope supplement: DA [5553053933](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5553053933); resume [5555445244](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5555445244).
- Branch / draft PR: `codex/exact-cap-projection-0` / #1978.
- Dispatched base: `be7c0f688051d7e31ec21c4e7f069dd6bc77de8d`.
- Preserved RED packet head: `a204573387da0a16924b7b1a30f37037df2a4c69`.
- HD-RECEIPT: 4b4c679f6cae
- ORIENT-RECEIPT: 468c464f975d
- orientation_rule_stamp: 53a4ada59778a8b5
- orientation_digest_sha: 783eb156251784906c84cc866fc7a38c6e3eecaefca571a365143276412b83c9

The handoff was rendered first, fresh coding orientation obtained as dispatched, and all 49 REQUIRED-ANCHORS retrieved through `anchor_query.sh`. The comment-only DA supplement expressly retains the HD and orientation receipts. The conforming scope STOP returned in 5552619019 is resolved by admitting exactly the two fixed qualification pin literals. Neither the HD nor the bundle component list changes.

## Archaeology before implementation

| Existing surface | Caller / consumer and disposition |
|---|---|
| `crates/simthing-kernel/src/resident_clearing_apportionment.rs` public `execute_resident_apportionment_cpu` | Existing workshop callers: `resident_clearing_apportionment_0.rs`, `resident_clearing_parity_0.rs`, `recursive_resource_filter_formalization_0.rs`. The new `exact_cap_projection_0.rs` adds a test caller only. No ordinary production CPU numerical caller was found. |
| Same file, private `settle_resident_apportionment_over_share_vector` | Sole private CPU exact clearer, called only by that public reference. Existing scope / hard-precedence grouping, Q149 inputs, canonical products, Hamilton and tie rotation remain here. Old post-share cap guard returned `ArithmeticOverflow`; the new active set precedes the same final rounding. |
| `crates/simthing-driver/src/resident_clearing_runtime.rs`, `dispatch_market` | Ordinary qualified session: immediate inputs reach `WorldGpuState::encode_resident_apportionment_with_dispatch_into`; spatial and temporal inputs reach the existing live-head exact encode methods. No host projection or new caller. |
| `crates/simthing-gpu/src/resident_clearing_runtime.rs` live head | `encode_spatial_apportionment` and `encode_temporal_apportionment` use existing WorldGpuState wrappers. All immediate / spatial / temporal variants reach the same kernel executor. Only the separately admitted qualification literal changes in this file. |
| `crates/simthing-kernel/src/shaders/resident_clearing_apportionment.wgsl`, `settle_partition` | Existing session pipelines invoke W32/W64 entry points. The same exact active set runs from immutable admitted input rows and live values for every physical dispatch shape. The output ABI and status codes are unchanged. |
| Kernel `ResidentApportionmentSession::readback_products` | Existing typed status-to-error conversion rejects failed vectors. No partial-success readback or new error/retry semantics. |
| Driver materialization / GPU live head | Existing canonical product success checks remain the only downstream error consumers. 15.10 failure/retry/rollback remains fenced. |
| Driver admission -> GPU `ResidentClearingQualification::admit` | The fixed fingerprint comparison occurs before creating the exact executor. The changed semantic bundle must pass this existing production gate. |

`plan_resident_exact_apportionment` still plans the admitted exact inputs. `clearing_weight_projection.rs` is not the cap-settlement authority and is unchanged. The product constructor, semantic sorting, scratch layout, bindings, entry points and production dispatch wrappers are unchanged.

## Preserved falsifier-first RED

Before any production edit, including at committed packet head `a204573387da0a16924b7b1a30f37037df2a4c69`:

```text
cargo test -p simthing-workshop --test exact_cap_projection_0 -- --nocapture --test-threads=1
requests=[1,100], live bases=[1,1], one scope/equality band, S=101
CPU=Err(ArithmeticOverflow)
GPU W32 partition1 / single: Err(ArithmeticOverflow), Err(ArithmeticOverflow)
GPU W64 partition1 / single: Err(ArithmeticOverflow), Err(ArithmeticOverflow)
FAILED: feasible cap collision must saturate and redistribute to (1,100)
0 passed; 1 failed; 0 ignored
```

Local transcripts: `.git/159-owner-red.log`, `.git/159-owner-red-head.log`. The original Owner assertion is retained without ignore, inversion, new semantics, or expected-error substitution. The fixture uses admitted TreeExecutionAuthority, resident semantic plan/buffers, existing exact plan, real GPU values and the production shader.

## One exact active-set algorithm

For one hard-precedence equality band, let `S0=min(remaining supply,sum positive-basis request caps)` and `B0=sum exact Q149 bases`. The existing E6 zero-basis ceiling and prior-band accounting remain unchanged. A zero total basis still grants zero.

1. Start `S=S0`, `B=B0`, frozen count zero.
2. Scan the immutable original band. A row is frozen precisely when `b_i*S > B*r_i`, an exact integer cross-product comparison. Sum all currently frozen caps and bases.
3. If the frozen count is unchanged, the active set is final. Otherwise subtract the total frozen caps from S0 and the total frozen bases from B0, and repeat.
4. Frozen rows receive their request exactly. Only nonfrozen rows enter the pre-existing exact quotient/remainder, Hamilton largest-remainder and `(granter+generation)` tie rotation. Construct and sort the same canonical products, with `U=request-G` and status OK.

CPU and WGSL have the same `exact_quota_exceeds_cap` comparison and scan/subtract/fixed-point order. CPU retains a local final-active list; WGSL reclassifies immutable rows in its two existing final quotient/remainder scans. Neither reads another invocation's output or stores shared frozen flags. WGSL keeps the original W32/W64 entry points, input/output halves and partition machinery. There is no float lambda, alternate rounding, host solver, or cross-band redistribution.

Termination and safety: removing rows whose cap/basis ratio is strictly below S/B strictly raises the active ratio. Therefore a frozen row stays frozen; any nonfinal pass increases the frozen count by at least one. Since S0 is no larger than the executable cap sum, at least one positive-basis row remains active. At most n-1 freezes plus one final scan are possible. CPU bounds scans by the band length; WGSL uses the enclosing admitted row count, an upper bound. Progress, denominator and checked arithmetic guards remain typed failures for violated invariants.

The existing scope overflow guard requires `sum(requests)-supply <= u32::MAX` when positive. As supply is u32, the request sum is below 2^33. Capped Q149 bases therefore sum below 2^182, and multiplying that sum by any u32 cap is below 2^214, fitting the existing 224-bit machinery. The individual numerator fits too. Cap classification runs before division: a previously frozen row can have a final notional quotient above u32 without making a lawful band overflow. No error is swallowed to classify that row.

Final active quotas are at or below integer caps; a fractional quota's Hamilton increment cannot exceed its cap, and an integral capped quota has zero residue. Frozen caps plus final active grants total S0 exactly. Prior precedence bands therefore consume their executable amount without reserving zero-basis requests, and only remaining supply reaches later bands.

## GREEN and no-collision certificates

`cargo test -p simthing-workshop --test exact_cap_projection_0 -- --nocapture --test-threads=1`: **3 passed / 0 failed / 0 ignored**. Transcript `.git/159-green-unqualified.log` was obtained through the existing lower-level CPU/GPU exact referees before changing the qualification pins.

| Witness | Exact grants and distinguishing obligation |
|---|---|
| Owner `[1,100]`, bases `[1,1]`, S101 | `[1,100]`, U `[0,0]`, CPU and W32/W64 x single/partition1 |
| Three-row `[1,2,100]`, bases `[1,2,1]`, S103 | `[1,2,100]`; two caps freeze in the first pass |
| `[1,4,100]`, equal bases, S10 | Initial ratio 10/3 freezes cap1; ratio 9/2 freezes cap4; remaining row gets5. Two nonfinal iterations. |
| Same `[1,4,100]` with equal minimum subnormals | `[1,4,5]`; common Q149 unit survives both freezes |
| `[1,100]`, bases `[1,min-subnormal]`, S101 | `[1,100]`; frozen row's final notional quotient exceeds u32, but its exact cap classification succeeds |
| `[1,100,100]`, bases `[1,0.3125,7]`, S12 | `[1,0,11]`; strict rational cap collision is resolved before integer rounding |
| `[1,100,100,9]`, bases `[1,1,0,9]`, precedences `[0,0,0,1]`, S110 | `[1,100,0,9]`; zero-basis request receives U100 and does not strand later-band supply |
| `[1,100,100,100]`, equal bases, S6, six generations | Freeze cap1; Hamilton gives the remaining five units only to the final three-row active tie, with the unchanged rotation |

The combined 12-case cap matrix runs every row permutation, both admission orders, both original/rotated physical slot layouts and all four GPU dispatch shapes: **3,104 GPU runs**, each compared with fixed expected G/U and the canonical full CPU product vector. Admission/slot changes cannot select economics.

Before production edits, the new no-collision corpus ran on the byte-unchanged dispatched solver: **338 cases**, CPU/GPU equality and FNV-1a digest **`05cb01d96dc69dbe`** over every byte of every canonical CPU product. That observed value was then fixed in the test before editing the two algorithm sources. The amended solver returns the same digest. The corpus covers zero/mixed bases, bounded integer bases/supplies, all six canonical product fields plus status/reserved bytes, neutral requests at 2^24, minimum subnormals, wide caps/extreme finite allocations, precedence/E6 and eight tie generations. Transcript `.git/159-no-collision-master.log` preserves the pre-edit observation. Existing numerical goldens are unchanged.

Mechanically, if no rational quota breaches a cap, the first scan terminates with original S/B and the complete original band. The existing quotient/remainder, source ordering and tie rotation then receive the identical inputs. Every no-collision product bit is preserved; the cap-collision fixtures above are outside that condition.

## E8 old-pin refusal and observed requalification

After final algorithm edits, with both old pins still `1c26d4ee5861ec68`, ordinary production session admission was exercised:

```text
cargo test -p simthing-workshop --test resident_session_integration_conformance_0 ordinary_session_identity_half_and_registry_permutation_cross_real_generations -- --nocapture --test-threads=1
ResidentClearing(LiveHead(UnqualifiedAdapter {
  required: 2028542802327104616,
  observed: 9297387406934488982
}))
0 passed; 1 failed; 0 ignored; 5 filtered out
```

The failure is at ordinary admission before economics, through the existing fixed comparison. Transcript `.git/159-e8-old-pin-refusal.log`. The observed integer is **`0x8106_f496_4185_3796`**. Both admitted literals are transcribed to that value in the same implementation commit; their diffs contain one literal replacement each. No dynamic pin, comparator/record change or build-script change.

The unchanged parity referee subsequently prints `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: 8106f49641853796` and passes; ordinary production sessions and all frozen cross-rung referees also pass. Provenance: Vulkan; NVIDIA GeForce RTX 4080 Laptop GPU; vendor4318/device10144; NVIDIA595.79; rustc1.95.0 `(59807616e)`, x86_64-pc-windows-msvc, LLVM22.1.2; EML_RESOURCE_PROFILING; wgpu22.1.0/naga22.1.0; dependency lock hash2005979115394712535; semantic bundle hash4510924765333878963; workgroups32/64; existing subgroup-independent assumption; ABI1. The full record is preserved in `.git/159-final-frozen.log`.

## Validation and unchanged authority

- `cargo check -p simthing-kernel`: PASS.
- Frozen workshop apportionment7, parity1, recursive formalization1, recursion-axis5, substrate-binding4, authority-lifetime4 and ordinary-session6: **28 passed / 0 failed**. Q149/neutral-request, invalid/nonfinite and real overflow refusals, Hamilton/tie, hard precedence, E6, canonical ordering, E5/E7, recursion, permits and live provenance assertions are unchanged.
- Qualification unit matrix (`cargo test -p simthing-gpu --features eml-resource-profiling resident_clearing_runtime -- --nocapture --test-threads=1`): **4 passed / 0 failed**. Existing ABI, child-share, planner and temporal mutation witnesses remain unchanged and distinguishing.
- Structural battery: **15 checks pass**: inventory1435/discovered1435/missing0/extra0, drift prove, constitutional check/selftest, lifecycle schema/prove, sanctioned digest, detachability check/selftest, anchor check/selftest, plan typing, observation, slot and overlay censuses. Logs `.git/159-final-*.log` and `.git/159-final-gate-exits.txt`.
- Full workspace, structural battery, inventory/drift, Agent Scan and hosted Scan/Exec certificates are bound to the exact implementation head in the final immutable PR/board return. No hosted smoke result substitutes for the local real-GPU tests.
- Authority census: one resident production exact authority; five CPU oracle doors; two CPU call-site families; two pre-existing peer residues; duplicate settlement, economic adapter, global coupling and private solver all zero. Final scanner evidence is attached to the return.

## Changed-file ledger and routing

Algorithm: `crates/simthing-kernel/src/resident_clearing_apportionment.rs` and `crates/simthing-kernel/src/shaders/resident_clearing_apportionment.wgsl`.

DA-admitted pin-only companions: `crates/simthing-gpu/src/resident_clearing_runtime.rs` and `crates/simthing-workshop/tests/resident_clearing_parity_0.rs`.

Proof surfaces: `crates/simthing-workshop/tests/exact_cap_projection_0.rs`, this results document, `scripts/ci/test_inventory.tsv` (three inventoried 15.9 tests), and the existing append-only `scripts/ci/anchor_reach_log.tsv` ingress receipt. Exactly eight changed paths against dispatched base. No new kernel test file was needed.

No build-script, workflow/gate-code, HD, canon, pointer, product ABI, qualification record/comparator or numerical-golden edit. 15.10 and departing-stream disposal remain fenced. Return PROBATION / proof-present / DA-review-pending / OPEN / UNMERGED for orchestration's INSPECT triage, final exact-head clearance, relay-lint and DA review. Coding does not merge or self-graduate.

## Required anchor acknowledgments

```text
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
ANCHOR-ACK: core-gpu-residency@f9b19479262a
ANCHOR-ACK: core-property-value-model@1be54f2e4803
ANCHOR-ACK: eml-admission-shapes@bdcc0b9512f7
ANCHOR-ACK: eml-extension-ladder@7755bc72ffbe
ANCHOR-ACK: eml-integration-plan@8eba54b02320
ANCHOR-ACK: eml-triad-integration@dada7d680557
ANCHOR-ACK: evaluation-identity-invariants@64ad30392930
ANCHOR-ACK: exact-numeric-candidate-f@6938a2efadb5
ANCHOR-ACK: field-policy-time-decisions@4309cdd821fe
ANCHOR-ACK: field-sweep-preservation@acc521a5a361
ANCHOR-ACK: intrinsic-constrained-clearing@957b7c81b756
ANCHOR-ACK: movement-front-adjudications@5af6a29acb75
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
ANCHOR-ACK: stead-rejected-shapes@7f75f8b55271
ANCHOR-ACK: stead-shared-surface-ledger@2d7062067214
ANCHOR-ACK: stemthing-binding-laws@6787a118c3ca
ANCHOR-ACK: stemthing-lane-not-leg@9a1d443b7981
ANCHOR-ACK: stemthing-slot-identity-ruling@02c87b9126e1
ANCHOR-ACK: workshop-candidate-homing@3e584f0ad175
```
