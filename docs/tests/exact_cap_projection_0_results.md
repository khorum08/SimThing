# EXACT-CAP-PROJECTION-0 — ingress and falsification evidence

Status: **PROBATION / proof-present / DA-review-pending / STOP / OPEN / UNMERGED**. The Owner falsifier is planted and RED. No production remedy has been applied; this is a scope-adjudication packet, not a 15.9 exit certificate.

- Dispatch: board comment 5552404418.
- Branch: `codex/exact-cap-projection-0`.
- Base: `be7c0f688051d7e31ec21c4e7f069dd6bc77de8d`.
- HD-RECEIPT: 4b4c679f6cae
- ORIENT-RECEIPT: 468c464f975d
- orientation_rule_stamp: 53a4ada59778a8b5
- orientation_digest_sha: 783eb156251784906c84cc866fc7a38c6e3eecaefca571a365143276412b83c9

The handoff was rendered first, fresh coding orientation obtained as dispatched, and all 49 REQUIRED-ANCHORS retrieved through `anchor_query.sh`. Their hashes match the already-read session anchors; the new orientation receipt replaces the 15.8 receipt.

## Binding STOP: qualification companion lies outside the admitted surfaces

Dispatch [5552404418](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5552404418) and the rendered HD explicitly require:

> implementation requires another production surface or gate-code edit; STOP with exact source/consumer reason before broadening

Both permitted apportionment files are already named components of the graduated 15.7 complete semantic kernel bundle. `simthing-gpu/build.rs` hashes their bytes into `SEMANTIC_KERNEL_BUNDLE_HASH`; `ResidentClearingQualification::capture` includes that hash in the qualification record. `ensure_production_qualified` compares the record's fingerprint against the fixed `QUALIFIED_RESIDENT_CLEARING_FINGERPRINT`. Ordinary `ResidentClearingRuntime::admit_sealed_market_with_persistence_deformations` calls this admission before creating the exact executor. Consequently, changing the permitted CPU/WGSL semantics invalidates the existing production qualification, as E8 requires.

The exact-source census found two necessary companion locations outside every entry in the HD `surfaces` list:

| Location | Existing binding | Required companion scope |
|---|---|---|
| `crates/simthing-gpu/src/resident_clearing_runtime.rs:22` | `QUALIFIED_RESIDENT_CLEARING_FINGERPRINT = 0x1c26_d4ee_5861_ec68` | After the new algorithm passes qualification on the real adapter, update only this pin to the observed qualified tuple |
| `crates/simthing-workshop/tests/resident_clearing_parity_0.rs:52` | Independent `QUALIFIED_RECORD_FINGERPRINT` asserted by the frozen parity terminal referee | Update only its matching qualification pin, preserving every numerical golden, assertion, and refusal witness |

**Requested orchestration/DA scope decision:** admit those two exact pin locations as mandatory E8 companions, contingent on actual amended-source qualification, or identify an already-admitted way to requalify that preserves the fixed seal. No build-script expansion is needed: both target files are already in the component list. No gate edit is needed. The HD and both pins remain unchanged. No new fingerprint is guessed, no qualified tuple is bypassed, and no weakened assertion is proposed.

The Owner RED ran before any production edit. Coding stopped before implementing or broadening the remedy once this source/consumer dependency was established. Existing source remains byte-identical to the dispatched base. The new test deliberately continues to fail; it is neither ignored nor inverted to bless the old refusal.

## Current caller archaeology and error propagation

| Surface | Current caller / consumer | Cap/error behavior |
|---|---|---|
| Kernel `execute_resident_apportionment_cpu` | Existing workshop `resident_clearing_apportionment_0`, `resident_clearing_parity_0`, `recursive_resource_filter_formalization_0`; the new Owner falsifier adds one test caller | Reads admitted live bases, converts to exact common Q149 via `exact_capped_basis`, calls the one private `settle_resident_apportionment_over_share_vector` |
| Private CPU settlement | Called only by that public kernel reference entry | Scope grouping, earlier-band executable grant ceilings, exact band quotient/remainder, Hamilton and tie rotation; post-share `grant > claim.requested` returns `ResidentApportionmentError::ArithmeticOverflow` |
| Driver `ResidentClearingRuntime::dispatch_market` (immediate input) | Ordinary qualified session binding calls `WorldGpuState::encode_resident_apportionment_with_dispatch_into` | Dispatches the existing `ResidentApportionmentSession`; no host projection |
| GPU live head `encode_spatial_apportionment` / `encode_temporal_apportionment` | Same driver dispatch switch, through the existing live head | Calls the existing spatial-product / temporal-demand variants on `WorldGpuState`; both reach the same exact WGSL executor |
| `WorldGpuState` exact encode methods | Immediate, spatial and temporal wrappers | Pass the real resident values and canonical buffers to the corresponding existing session encode methods |
| WGSL `settle_partition`, W32/W64 entry points | Existing `ResidentApportionmentSession` pipelines and partition dispatch | Mirrors Q149, precedence, exact quotients/remainders and tie ranking; `granted > current.requested` writes `STATUS_ARITHMETIC_OVERFLOW` |
| Kernel `ResidentApportionmentSession::readback_products` | Direct exact GPU referees | Any error status returns the corresponding typed error; it does not return a partial success vector |
| Driver materialization / GPU live head | Production schedule observation and subsequent consumers | Failed canonical products are rejected by existing product-success checks; this rung adds no alternate status, recovery, or retry semantics |
| Production admission | Driver admission -> `ResidentClearingQualification::admit` -> fixed fingerprint comparison | A changed semantic bundle requires requalification at the out-of-scope pin before this production consumer can admit it |

The driver plans exact inputs via `plan_resident_exact_apportionment`; `clearing_weight_projection.rs` is not a caller or alternate settlement authority. `ResidentApportionmentSession::new` loads `shaders/resident_clearing_apportionment.wgsl`; immediate, spatial and temporal input variants share it. The CPU reference has no ordinary production numerical caller in this census. Its three existing workshop caller files remain unchanged.

## Falsifier-first transcript

Command on stamped production source at `be7c0f688051d7e31ec21c4e7f069dd6bc77de8d`, with the new unignored test:

```text
cargo test -p simthing-workshop --test exact_cap_projection_0 -- --nocapture --test-threads=1
requests=[1,100], admitted live bases=[1,1], same scope/equality band, S=101
CPU=Err(ArithmeticOverflow)
GPU W32 rows_per_dispatch=1: Err(ArithmeticOverflow)
GPU W32 rows_per_dispatch=4294967295: Err(ArithmeticOverflow)
GPU W64 rows_per_dispatch=1: Err(ArithmeticOverflow)
GPU W64 rows_per_dispatch=4294967295: Err(ArithmeticOverflow)
FAILED: feasible cap collision must saturate and redistribute to (1,100): ArithmeticOverflow
0 passed; 1 failed; 0 ignored; shell nonzero / cargo test failure
```

The fixture uses existing `TreeExecutionAuthority`, semantic-plan admission, canonical rows/buffers, `ResidentApportionmentPlan::build`, CPU reference, real resident values, and the production WGSL exact executor. It adds no solver or alternative economic input type. Both bases are exactly 1 after the existing cap-to-request basis conversion. The old share is 101/2 for each row before the frozen Hamilton step, so the row capped at request 1 fails the post-share guard. All four GPU dispatch shapes exercise that same failure. The assertion still requires exact successful products `(G1,U0)` and `(G100,U0)` and byte-for-byte CPU/GPU equality.

## Checks and proof limits

- `cargo check -p simthing-kernel`: PASS.
- Frozen exact corpus and cross-rung referees: **28 passed / 0 failed**: apportionment (7), parity (1), formalization (1), 15.5 (5), 15.6 (4), 15.7 (4), 15.8 (6). Existing Q149/neutral-request, deterministic tie, hard precedence, E6, canonical ordering, row/workgroup/partition, recursion, E5/E7, permit, actual-session and provenance assertions are unchanged.
- GPU qualification unit matrix with `--features eml-resource-profiling`: **4 passed**. Existing ABI, child-share, planner and temporal component mutation witnesses remain load-bearing. Production pin remains `1c26d4ee5861ec68`.
- Authority census remains unchanged: one resident production authority, five CPU oracle doors, two CPU call-site families, two pre-existing peer residues; duplicate settlement, economic adapter, global coupling and private solver counts all zero.
- Exact structural, inventory, committed Agent Scan and hosted Scan/Exec results are recorded with the immutable packet head in the PR/board return. Hosted smoke/structural success does not turn this intentional Owner RED into an implementation PASS.
- **Not claimed:** GREEN cap products, three-row/multiple-freeze proof, amended CPU/WGSL algorithm correspondence, termination proof, amended no-collision bit-identity certificate, full zero-red implementation certificate, or graduation readiness. These require implementing and qualifying the remedy after the scope STOP is resolved.

Changed-file ledger: only this results doc, `crates/simthing-workshop/tests/exact_cap_projection_0.rs`, its `scripts/ci/test_inventory.tsv` row, and the append-only `scripts/ci/anchor_reach_log.tsv` receipt. Zero production, gate, HD, canon, pointer, golden, or qualification-pin edits. 15.10 and departing-stream disposal remain fenced.

## Routing

Return this exact scope dependency to orchestration/DA. Preserve the current HD receipt and fresh coding receipt until an authoritative HD amendment changes the former. Coding does not amend the HD, merge, graduate, run final clearance/relay on orchestration's behalf, or treat the existing qualification seal as optional.

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
