# RESIDENT-CLEARING-PLAN-0 results

> **Status: PROBATION / proof-present / DA-review-pending.** Coding lane
> only; no merge, graduation, pointer movement, closeout apply, score/band
> implementation, apportionment, dispatch, cutover, or 14.3+ work.

**Date:** 2026-08-31  
**Owner dispatch:** Board comment `5478901209`  
**Exact dispatch master:** `d78334d0fda287c829d4dd63c30b834c88b1f3ed`  
**Authored handoff base:** `2ba6e28cd4ba83a52a5b0fa99a4a299ca31ae9d6`  
**tested_code_sha:** `b4a19ea00a4240e1ff62706d4b840d4258743b0e`  
**HD-RECEIPT:** `0417d2e81c96`  
**ORIENT-RECEIPT:** `abd383d5c8c6`  
**orientation_rule_stamp:** `6550f3a270f552cc`  
**expected route:** `DA-RESERVE(binding)`

The final evidence-tail SHA and hosted workflow/job identifiers are carried by
the exact-head PR relay so this document does not create a self-referential
commit hash.

## Required anchor acknowledgements

- `accumulator-exact-vs-soft-semantics@0efceafc77cf`, `accumulator-op-v2-invariants@32fb4fc36080`, `actionband-8x-sequencing@067ef8ace1e0`, `actionband-axis-budget@52275c538689`, `actionband-binding-laws@d6a8b1b2d673`, `actionband-constitutional-placement@d56d9a04a620`
- `actionband-crossing-surface@623db585f145`, `actionband-determinism-lifecycle@6306c484732c`, `actionband-eml-payload-purity@2a1d981f3958`, `actionband-executive@9c7e004e213b`, `actionband-fenced-questions@c40674d92d18`, `actionband-field-triad-authority@56cf5cdf2d2c`
- `actionband-gpu-physical-model@3252c1b3c3b5`, `actionband-native-authority-table@541a03cb00a1`, `actionband-performance-model@8d93f06d4bae`, `actionband-target-forms@c3b7bce99f1f`, `actionband-vendorization-direction@20336db0d366`, `admission-ladder-necessity-test@4bedf826f6f7`
- `candidate-f-exhaustive-proof-method@7c5ce0b93dab`, `core-gpu-residency@f9b19479262a`, `core-overlays@94a8955e46f2`, `core-property-value-model@1be54f2e4803`, `core-rf-arenas@5dd14f66897b`, `eml-admission-shapes@bdcc0b9512f7`
- `eml-extension-ladder@7755bc72ffbe`, `eml-integration-plan@8eba54b02320`, `eml-triad-integration@dada7d680557`, `evaluation-identity-invariants@64ad30392930`, `exact-numeric-candidate-f@6938a2efadb5`, `field-policy-time-decisions@ac0f34f33cb2`
- `field-sweep-preservation@dada7d680557`, `intrinsic-constrained-clearing@957b7c81b756`, `movement-front@ade58203ecee`, `movement-front-adjudications@5af6a29acb75`, `orientation-harness-core@8a365d1c0864`, `overlay-closure-thesis@241cc54c5706`, `overlay-designer-closure@4a047b29243d`
- `overlay-germ@90d1adf9b357`, `overlay-promoted-laws@248c7893b462`, `overlay-scale-laws@c2ffb2826df7`, `rf-arena-allocation-invariants@82864469489b`, `rf-arena-substrate@17b5f1e5c2ba`, `scanner-selftest-delta-gate@34fb2662baae`
- `seal-residue-cross-crate@c61c33d90efc`, `simthing-0087-binding-laws@567370293add`, `simthing-0087-pillars@61487cba1f9e`, `stead-events-are-rf@b261a7ff0630`, `stead-rejected-shapes@7f75f8b55271`, `stead-shared-surface-ledger@54dca5feaebe`
- `stead-spatial-contract-core@8585db4ac631`, `stemthing-binding-laws@6787a118c3ca`, `stemthing-lane-not-leg@01d8ee2408b6`, `stemthing-slot-identity-ruling@02c87b9126e1`, `structural-execution-convergence@6b4cedec482b`, `workshop-candidate-homing@3e584f0ad175`

## First-step final-home and coupling census

This census was published from the exact dispatch master before production
source edits.

| Concern | Existing authority at dispatch | Coupling fence | Final owner |
|---|---|---|---|
| local semantic identity | `simthing-core::SimThingId(u32)`; legacy process-global mint exists but persisted trees carry local raw values | no widening and no process-global uniqueness assumption | core realm qualification wraps, never alters, the local id |
| physical row identity | core `SlotIndex(u32)` minted by kernel `SlotAllocator` | physical rows and source ordinals never cross as destination identity | destination kernel plan remaps canonical realm-qualified identity |
| generation | core `GenerationStamp`; caller supplies clearing authority per call | no second clock or generation authority | core context binds the supplied generation reference |
| schedule | caller-owned core `IntegrationSchedule` | no second log, scheduler, barrier, or host lock | transient core binding borrows the existing schedule |
| registry / residency | core `DimensionRegistry`; kernel `SlotAllocator` and placement books | no global semantic registry or shared mutable residency authority | transient core binding borrows the existing attachments |
| GPU buffers | instance-owned `wgpu::Buffer` sets | no WGSL or dispatch in 14.2; adapter sharing cannot share semantic state | GPU owns one private buffer set per tree plan |
| crate direction | core <- kernel <- GPU; workshop consumes all through normal dependencies | no manifest edit, dependency inversion, or workshop algorithm | unchanged one-way dependency direction |

## Final-home map

| Crate | New owned surface | Role |
|---|---|---|
| `simthing-core` | `TreeRealmId`, `ExecutionIncarnation`, `RealmQualified<T>`, `SeamFactId`, `SeamFact<T>`, `TreeExecutionContext`, `TreeExecutionBinding`, typed errors | fixed-width durable realm/local identity, transient incarnation, canonical seam retry identity, and a checked borrowing view over the existing root/generation/schedule/registry/residency authorities |
| `simthing-kernel` | typed owner/resource/scope/draw ids and ordinals, `DenseOrdinalRange`, admitted budgets, canonical dictionaries/rows, `ResidentPlanContext`, `SemanticPlanDigest`, `ResidentClearingPlan` and checked binding/errors | concrete reusable deterministic resident economic-resolution plan germ |
| `simthing-gpu` | stable POD header/owner/id/row types, seven checked descriptors, `ResidentClearingAbi`, tree-bound owner, private `ResidentClearingBuffers`, typed errors | final physical representation and instance-owned resident storage; no pipeline, shader, encoder, or dispatch |
| `simthing-workshop` | `ResidentClearingPlanObservation` and one observation function | consumer only; no plan construction, sorting, layout, allocation, or algorithm |

`TreeExecutionContext` canonical bytes are 32 bytes
(`realm[16] + incarnation[8] + root[4] + generation[4]`). `SeamFactId`
canonical bytes are also 32 bytes
(`source realm[16] + seam[8] + generation[4] + ordinal[4]`). These forms
are O(1), contain no host/device/process/address coordinate, and use no global
mint. `SimThingId` remains 4 bytes. `git diff` from dispatch is empty for
`simthing.rs` and `ids.rs`, proving `SimThing` and `SimThingId` layouts were not
edited.

The checked binding accepts only the context's existing root and generation
authority, then borrows the caller-owned schedule, registry, and residency.
The runtime witness rejects wrong-root and wrong-generation bindings.
Migration preserves realm/root/generation and changes incarnation; fork key
`77` deterministically produces a distinct realm; stale-incarnation seam facts
fail closed.

## Deterministic plan and seam proof

The plan derives each typed dictionary by canonical total-order sort plus
deduplication, maps rows through checked binary-search ordinals, then sorts the
typed ordinal tuples. No hash map, registration-order enumeration, physical
row, device coordinate, or foreign ordinal participates.

The witness constructs the same four admissions in original, reversed, and
rotated order plus replay reconstruction. Dictionaries, ranges, rows,
canonical bytes, and digest are byte-identical. Rebuilding after migration is
also byte-identical because incarnation is transient and excluded from the
semantic plan binding.

Canonical fixture:

- bytes: `312`
- digest: `4050c40a073a0aa83042dec5ee4409cd`
- dictionaries: owners `3`, resources `2`, scopes `2`, draws `4`
- rows: `4`

Tree B's realm-qualified local owner `7` has one B-local source ordinal. Tree
A receives its `SeamFact`, validates B's active incarnation, ignores the
source ordinal as destination position, and binary-remaps the canonical
realm-qualified subject to a different A-local ordinal. A retry retains the
same `SeamFactId`; lawful multiplicity changes `source_ordinal`, so the two ids
are distinct.

## Simultaneous A/B witness

Both trees are reconstructed through ordinary persisted `SimThing` JSON and
installed through the real `SlotAllocator::install_initial_tree` door. Each
has root local id `7` and child local id `8`; no global id mint, unsafe
constructor, or test-only identity wrapper participates.

| | Tree A | Tree B |
|---|---|---|
| realm | `1` | `2` |
| incarnation | `11` | `22` |
| generation | `10` | `20` |
| root local id | `7` | `7` |
| schedule / registry / residency | independently owned | independently owned |
| semantic dictionary / plan | independently constructed | independently constructed |
| GPU buffer owner and seven allocations | independently owned | independently owned |

The real Vulkan adapter simultaneously allocates both plans. Pointer identity,
realm, generation, plan bytes, dictionaries, and buffer owners differ; only
the physical adapter is shared.

## ABI and budget proof

All counts narrow through checked conversion. Dense end, count-times-stride,
canonical byte length, scratch bytes, alignment, total resident bytes, host
allocation representation, and device `max_buffer_size` are checked before the
first GPU allocation.

| Kind | Count | Stride | Logical bytes | Allocated bytes (16-aligned) |
|---:|---:|---:|---:|---:|
| header `0` | 1 | 72 | 72 | 80 |
| owners `1` | 3 | 32 | 96 | 96 |
| resources `2` | 2 | 8 | 16 | 16 |
| scopes `3` | 2 | 8 | 16 | 16 |
| draws `4` | 4 | 8 | 32 | 32 |
| rows `5` | 4 | 16 | 64 | 64 |
| scratch `6` | 4 | 64 | 256 | 256 |
| **total** | | | | **560 / 65,536 admitted resident bytes** |

Scratch is `rows * scratch_bytes_per_row = 4 * 64 = 256`, within the
`8,192`-byte admitted scratch budget. Executable negative cases cover dense
range overflow, inconsistent scratch admission, owner-count excess,
semantic-plan byte excess, and resident-byte excess before allocation.

## Containment and consumer census

- Production consumers: zero. Mechanical callers of `ResidentClearingPlan::build`
  and `ResidentClearingBuffers::allocate` are only the focused workshop test;
  the workshop library only observes a borrowed final-home plan.
- New global semantic state, singleton, lock, allocator, schedule, or
  generation authority: zero. Agent Scan is clean and the implementation owns
  only values or per-instance buffers.
- Frozen clearing semantics: no source changed in `simthing-spec`,
  `simthing-driver`, `simthing-sim`, `simthing-feeder`,
  `simthing-clausething`, or `simthing-mapeditor`; no score, equality-band,
  apportionment, grant, unresolved-U, replay, or structural-consequence code
  was added.
- Dispatch surface: no Cargo manifest/lockfile, WGSL, shader include,
  pipeline, command encoder, or `dispatch_workgroups` delta.
- The governed `kernel_surface.txt` adds only the 20 deliberate 14.2 exports
  with named promotion blockers, as required by Agent Scan;
  `docs/sanctioned_surface.md` is the mechanically regenerated digest of those
  same rows.
- Closeout ledger adds leases only for the six new 14.2 artifacts. Modified
  module exports, evidence index, reach log, inventory, and governed allowlist
  are not misclassified as new artifacts.

## Changed-file census

Production/final-home code and witness (9):

- `crates/simthing-core/src/lib.rs`
- `crates/simthing-core/src/tree_execution_context.rs`
- `crates/simthing-kernel/src/lib.rs`
- `crates/simthing-kernel/src/resident_clearing_plan.rs`
- `crates/simthing-gpu/src/lib.rs`
- `crates/simthing-gpu/src/resident_clearing_plan.rs`
- `crates/simthing-workshop/src/lib.rs`
- `crates/simthing-workshop/src/resident_clearing_plan.rs`
- `crates/simthing-workshop/tests/resident_clearing_plan_0.rs`

Evidence/governance (7):

- `docs/tests/resident_clearing_plan_0_results.md`
- `docs/tests/current_evidence_index.md`
- `scripts/ci/test_inventory.tsv`
- `scripts/ci/closeout_artifacts.tsv`
- `scripts/ci/anchor_reach_log.tsv`
- `scripts/ci/allow/kernel_surface.txt`
- `docs/sanctioned_surface.md`

No other file is in the exact dispatch-to-head delta. The allowlist is the
scanner-prescribed record for the new public kernel doors; no CI `.sh` or
`.py` code changed.

## Local certificate

- `cargo check -p simthing-core -p simthing-kernel -p simthing-gpu -p simthing-workshop` — PASS
- `cargo test -p simthing-workshop --test resident_clearing_plan_0 -- --nocapture` — PASS, 3/3, real adapter
- `cargo test -p simthing-workshop --test generation_critical_path_baseline_0 --offline -- --test-threads=1 --nocapture` — PASS, 4/4; generated timing packet restored byte-identically because 14.2 must not refresh 14.1 measurements
- `cargo test -p simthing-driver --test contention_arena_executed_0` — PASS, 1/1
- `cargo test -p simthing-driver --test clearing_weight_semantic_partition_0` — PASS, 2/2
- `bash scripts/ci/test_inventory_check.sh` — PASS, 1367 discovered / 1367 inventoried
- `bash scripts/ci/test_inventory_drift_check.sh` — PASS, unledgered 0 / stale 0
- `bash scripts/ci/test_lifecycle_expiry_check.sh --schema` — PASS, expired 0 / audit 0
- `bash scripts/ci/track_closeout.sh --artifact-expiry` — PASS, expired 0 / cruft 0 / malformed 0
- `bash scripts/ci/agent_scan.sh` — PASS, hard failures 0 / inspect 0
- `bash scripts/ci/doc_budget_check.sh --check` — PASS
- `bash scripts/ci/anchor_check.sh --check` — PASS (coverage/curation informational INSPECT only)
- `bash scripts/ci/gen_digest.sh --check` — PASS
- `git diff --check` — PASS

Hosted Doctrine Scan/Exec plus fresh exact-head `/clearance` and `/relay-lint`
are the PR relay's final binding proof. Coding returns PROBATION and does not
merge, graduate, move the pointer, invoke closeout, or begin 14.3.
