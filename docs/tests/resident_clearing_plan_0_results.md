# RESIDENT-CLEARING-PLAN-0 results

> **Status: PROBATION / archaeology-first / proof-present /
> DA-review-pending / UNMERGED.** Coding lane
> only; no merge, graduation, pointer movement, closeout apply, score/band
> implementation, apportionment, dispatch, cutover, or 14.3+ work.

**Date:** 2026-08-31  
**Owner dispatch:** Board comment `5478901209`  
**Narrow remand:** Board comment `5480270212` under DA ruling `5480224895`
**Continuation remand:** Board comment `5481459059` under DA ruling `5481435541`
**Target C remand:** Board comment `5482524832` under DA ruling `5482369734`
**Target D remand:** Board comment `5483265699` under DA ruling `5483217338`
**Rev-6 archaeology handoff:** Board comment `5487846392` under Owner/DA
edict `5485986010`
**Gate-wiring re-dispatch:** Board comment `5488171717`
**Row-11 ruling re-dispatch:** Board comment `5488433166` under DA ruling
`5488315659`
**Latest authority master:** `fd8ca8faf084694da86e14749dcd81bd55911a6d`
**Exact dispatch master:** `d78334d0fda287c829d4dd63c30b834c88b1f3ed`  
**Authored handoff base:** `2ba6e28cd4ba83a52a5b0fa99a4a299ca31ae9d6`  
**pre-remand head:** `545eb3dc09f51b89f72ab7b391784124ad119e37`
**remand source commit:** `e9042e40960aace583fb79670080530870cd05c0`
**continuation old head:** `929713cb33cab979b851f8af2aa3eddce8fb3988`
**continuation A/B source commit:** `6649589388a8e440949638a8ce778999ddc97c31`
**target C old head:** `3043a89e7ad78889a435604ab5ca0e36e0c88ccf`
**target C source commit:** `c2e63c3d65a5aa0dac6ba55c89da6828cb05034f`
**target D old head:** `6b42b2f2f5a957ecab6f3ffc9d07f5ff90902e20`
**target D source commit:** `77c10b1be5afb869afbb296e321025bd4c439eb3`
**HD-RECEIPT:** `0417d2e81c96`  
**ORIENT-RECEIPT:** `608708890df0`
**orientation_rule_stamp:** `2d0b3d2759bbd480`
**orientation_digest_sha:**
`2128c53f66ba6bd8814817baa30d8f177fc8590af89d2baf9dcc1b7777574311`
**semantic expected route:** `DA-RESERVE(binding)`; prior mechanical route
`DA-RESERVE(gate-wiring)` due to governed kernel-surface self-application

The final evidence-tail SHA and hosted workflow/job identifiers are carried by
the exact-head PR relay so this document does not create a self-referential
commit hash.

## Required anchor acknowledgements

- `accumulator-exact-vs-soft-semantics@0efceafc77cf`, `accumulator-op-v2-invariants@32fb4fc36080`, `actionband-8x-sequencing@067ef8ace1e0`, `actionband-axis-budget@52275c538689`, `actionband-binding-laws@d6a8b1b2d673`, `actionband-constitutional-placement@d56d9a04a620`
- `actionband-crossing-surface@623db585f145`, `actionband-determinism-lifecycle@6306c484732c`, `actionband-eml-payload-purity@2a1d981f3958`, `actionband-executive@9c7e004e213b`, `actionband-fenced-questions@c40674d92d18`, `actionband-field-triad-authority@56cf5cdf2d2c`
- `actionband-gpu-physical-model@3252c1b3c3b5`, `actionband-native-authority-table@541a03cb00a1`, `actionband-performance-model@8d93f06d4bae`, `actionband-target-forms@c3b7bce99f1f`, `actionband-vendorization-direction@20336db0d366`, `admission-ladder-necessity-test@4bedf826f6f7`
- `candidate-f-exhaustive-proof-method@7c5ce0b93dab`, `core-gpu-residency@f9b19479262a`, `core-overlays@94a8955e46f2`, `core-property-value-model@1be54f2e4803`, `core-rf-arenas@5dd14f66897b`, `eml-admission-shapes@bdcc0b9512f7`
- `eml-extension-ladder@7755bc72ffbe`, `eml-integration-plan@8eba54b02320`, `eml-triad-integration@dada7d680557`, `evaluation-identity-invariants@64ad30392930`, `exact-numeric-candidate-f@6938a2efadb5`, `field-policy-time-decisions@ac0f34f33cb2`
- `field-sweep-preservation@acc521a5a361`, `intrinsic-constrained-clearing@957b7c81b756`, `movement-front-adjudications@5af6a29acb75`, `orientation-harness-core@8a365d1c0864`, `overlay-closure-thesis@241cc54c5706`, `overlay-designer-closure@4a047b29243d`
- `overlay-germ@f0c8d2ebade9`, `overlay-promoted-laws@248c7893b462`, `overlay-scale-laws@c2ffb2826df7`, `rf-arena-allocation-invariants@82864469489b`, `rf-arena-substrate@17b5f1e5c2ba`, `rf-market-candidate-laws@ef8d37c169b7`
- `rf-market-falsifiers@30c747c48e9a`, `rf-market-mirror-cycle@13231438befe`, `rf-market-port-census@6dc8eb45dd49`, `rf-market-receive-not-recompute@2356c9e3f477`, `rf-market-settled-code-census@959d022781da`, `scanner-selftest-delta-gate@34fb2662baae`
- `seal-residue-cross-crate@c61c33d90efc`, `simthing-0087-binding-laws@567370293add`, `simthing-0087-pillars@61487cba1f9e`, `stead-events-are-rf@56c2f9cf3bff`, `stead-rejected-shapes@7f75f8b55271`, `stead-shared-surface-ledger@75e60cb1ef7`
- `stead-spatial-contract-core@8585db4ac631`, `stemthing-binding-laws@6787a118c3ca`, `stemthing-lane-not-leg@a7b0edf05665`, `stemthing-slot-identity-ruling@02c87b9126e1`, `structural-execution-convergence@6b4cedec482b`, `workshop-candidate-homing@3e584f0ad175`

## First-step final-home and coupling census

This census was published from the exact dispatch master before production
source edits.

| Concern | Existing authority at dispatch | Coupling fence | Final owner |
|---|---|---|---|
| local semantic identity | `simthing-core::SimThingId(u32)`; legacy process-global mint exists but persisted trees carry local raw values | no widening and no process-global uniqueness assumption | core realm qualification wraps, never alters, the local id |
| physical row identity | core `SlotIndex(u32)` minted by kernel `SlotAllocator` | physical rows and source ordinals never cross as destination identity | destination kernel plan remaps canonical realm-qualified identity |
| generation | core `GenerationStamp`; one caller-owned live per-tree authority record | no second clock, serialized authority, or semantic-plan generation field | sealed core capsule borrows `TreeGenerationAuthority`; GPU header/owner carry its transient value |
| schedule | caller-owned core `IntegrationSchedule` | no second log, scheduler, barrier, or host lock | transient core binding borrows the existing schedule |
| registry / residency | core `DimensionRegistry`; kernel `SlotAllocator` and placement books | no global semantic registry or shared mutable residency authority | transient core binding borrows the existing attachments |
| GPU buffers | instance-owned `wgpu::Buffer` sets | no WGSL or dispatch in 14.2; adapter sharing cannot share semantic state | GPU owns one private buffer set per tree plan |
| crate direction | core <- kernel <- GPU; workshop consumes all through normal dependencies | no manifest edit, dependency inversion, or workshop algorithm | unchanged one-way dependency direction |

## Final-home map

| Crate | New owned surface | Role |
|---|---|---|
| `simthing-core` | `TreeRealmId`, `ExecutionIncarnation`, `TreeGenerationAuthority`, `TreeExecutionAuthority`, opaque `TreeExecutionContext`, `TreeExecutionBinding`, `RealmQualified<T>`, `SeamEmissionOrdinal`, `SeamFactId`, `SeamFact<T>`, typed errors | one private live authority capsule borrowing the real attachments; durable realm/local identity; non-convertible deferred emission ordinal; live-incarnation-checked destination remap |
| `simthing-kernel` | typed owner/resource/scope/draw ids and ordinals, `DenseOrdinalRange`, admitted budgets, consumer-owned non-serde replay envelope, typed transport/domain replay error, canonical dictionaries/rows, `ResidentPlanContext`, `SemanticPlanDigest`, `ResidentClearingPlan` and checked binding/errors | concrete reusable deterministic resident economic-resolution plan germ |
| `simthing-gpu` | stable POD header/owner/id/row types, seven checked descriptors, `ResidentClearingAbi`, tree-bound owner, private `ResidentClearingBuffers`, `ResidentGenerationAdvance`, typed errors | final physical representation, instance-owned storage, and no-allocation transient header advance; no pipeline, shader, encoder, or dispatch |
| `simthing-workshop` | `ResidentClearingPlanObservation` and one observation function | consumer only; no plan construction, sorting, layout, allocation, or algorithm |

`TreeExecutionContext` is an O(1), non-`Clone`, non-`Copy`, non-serde opaque
handle containing a private allocation seal plus captured incarnation. It has no
public constructor and is not a durable byte product. Its semantic-plan binding
is only `realm[16] + root[4]`; generation, incarnation, and attachment witness
identities are runtime-only. `SeamFactId` canonical bytes remain 32 bytes
(`source realm[16] + seam[8] + generation[4] + ordinal[4]`). These forms
are O(1), contain no host/device/process/address coordinate, and use no global
mint. `SimThingId` remains 4 bytes. `git diff` from dispatch is empty for
`simthing.rs` and `ids.rs`, proving `SimThing` and `SimThingId` layouts were not
edited.

`TreeGenerationAuthority` owns the unrepeatable execution-capsule mint in the
same caller-owned record as live generation. `TreeExecutionAuthority::seal`
atomically consumes that token before capturing the exact borrowed root,
schedule, registry, and residency behind one private `Arc` seal. A second seal
over the same generation authority fails `GenerationAuthorityAlreadySealed`,
even when every supplied value/reference is otherwise lawful; a separate tree
with its own generation authority seals normally. `seal_context` then succeeds
once inside the admitted wrapper. Every binding, plan, GPU, and seam
door compares the seal by allocation identity and checks the live incarnation;
equal raw root id and equal generation values cannot cross-bind capsules.
Migration atomically advances that live record, invalidating retained old
contexts without caller cooperation. Fork key `77` deterministically produces
a distinct realm.

## Deterministic plan and seam proof

Plan admission narrows `max_rows`, reserves exactly that logical row capacity,
and never trusts `size_hint`. The successful fixture records `(len, capacity,
reallocations) = (4, 32, 0)`; the exact-fill witness records `(2, 2, 0)`; and
max+1 returns typed evidence `(stored, reserved, reallocations) = (2, 2, 0)`
before storing the third row. Before inserting each new distinct axis value it
also refuses that axis's admitted maximum. A running checked semantic-byte
projection uses the final canonical length arithmetic and refuses the first
over-budget row before dictionaries or canonical rows exist. The admitted
`BTreeSet` values are already in canonical total order; rows then map through
checked binary-search ordinals and sort as typed ordinal tuples. No hash map,
registration-order enumeration, physical row, device coordinate, or foreign
ordinal participates.

The witness constructs the same four admissions in original, reversed, and
rotated order plus replay reconstruction. Dictionaries, ranges, rows,
canonical bytes, and digest are byte-identical. Validated replay requires a
consumer-owned outer envelope, reconstructs through the same admission door,
and verifies stored canonical bytes plus recomputed digest. Rebuilding after migration and advancing
generation N to N+1 are also byte-identical because incarnation and generation
are transient and excluded from semantic plan identity.

Canonical fixture:

- bytes: `308`
- digest: `a61ebfee74156dd1e39bb8c5ec089ca4`
- dictionaries: owners `3`, resources `2`, scopes `2`, draws `4`
- rows: `4`

Tree B's realm-qualified local owner `7` resolves to different B-local and
A-local dictionary ordinals, proving the destination dictionary never consumes
a foreign layout ordinal. The actual core destination-remap door validates the
source capsule's live incarnation before exposing the realm-qualified subject;
after migration, retained old context plus old fact fails there.

The prior claim that an owner-dictionary ordinal proved retry idempotence or
lawful multiplicity is withdrawn. `SeamEmissionOrdinal` is now a distinct type
with no public raw constructor, `From<u32>`, serde path, or conversion from any
resident ordinal. Its mint plus IntegrationSchedule retry-versus-multiplicity
proof remains explicitly dated in `ASYNC-RETRY-IDEMPOTENCE-DEBT` for the future
immutable seam-emission recorder; 14.2 does not fabricate that recorder.

## Narrow-remand falsifiers and generation separation

1. **One authority wrapper:** sealing twice over tree A's exact real root,
   generation authority, schedule, registry, and residency fails the second
   call with `GenerationAuthorityAlreadySealed`. Tree B's separate generation
   authority remains a positive control and seals beside A.
2. **Cross capsule:** context A plus tree-B's complete authority capsule fails
   `AuthorityCapsuleMismatch` although both real persisted roots have raw id
   `7` and B supplies its own schedule, registry, and residency.
3. **Live migration invalidation:** after migration updates the authority's
   live incarnation, old context plus old fact fails `StaleIncarnation` through
   `TreeExecutionContext::remap_seam_fact`, the exact door used by kernel
   destination remapping.
4. **One context:** a second `seal_context` call on the admitted wrapper fails
   `ContextAlreadyMinted`; migration yields a new valid context only after the
   old one becomes stale against the live record.

Generation N->N+1 changes zero of the 308 semantic bytes and zero of the 128
digest bits. `TreeGenerationAuthority::advance` changes only the caller-owned
live generation record; `ResidentClearingBuffers::advance_generation` mutates
the in-memory transient owner/header POD. The executable witness retains exact
pointer identity for header, owner, and row `wgpu::Buffer` objects, proving no
semantic rebuild, buffer recreation, or allocation. Queue/header upload and
dispatch remain fenced to 14.3.

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

## Target C trusted replay envelope

Replay allocation authority now flows from the consumer to the packet, never
from the packet to itself. `ResidentClearingReplayEnvelope` has private fields
and no serde implementation. A session, loader, or other trusted consumer must
construct it independently through the same nine checked budget inputs before
calling the sole public replay door:

```text
ResidentClearingPlan::replay_with_budget_envelope<'de, D>(
    trusted: ResidentClearingReplayEnvelope,
    deserializer: D,
) -> Result<ResidentClearingPlan, ResidentClearingReplayError<D::Error>>
where D: serde::Deserializer<'de>
```

The private `ResidentClearingPlanWireSeed` carries that already-admitted outer
envelope into the top-level visitor. The visitor reads the fixed-size wire
budget block and compares every claim componentwise before dictionaries, rows,
canonical bytes, or any other variable sequence is accepted. Only a narrower
or equal wire budget is validated internally and handed to the existing
bounded visitors. Consequently every visitor reservation is bounded by
`wire claim <= trusted outer envelope`; later typed reconstruction allocates
only actual already-bounded sequence lengths.

Componentwise executable claims/ceilings are:

| wire field | hostile claim | trusted outer ceiling |
|---|---:|---:|
| `max_owners` | 17 | 16 |
| `max_resources` | 17 | 16 |
| `max_scopes` | 17 | 16 |
| `max_draws` | 17 | 16 |
| `max_rows` | 33 | 32 |
| `max_semantic_plan_bytes` | 16,385 | 16,384 |
| `max_resident_bytes` | 65,537 | 65,536 |
| `max_scratch_bytes` | 8,193 | 8,192 |
| `scratch_bytes_per_row` | 65 | 64 |

Each returns typed `WireBudgetExceedsTrustedEnvelope { field, claimed,
admitted }`. The all-large falsifier declares internally consistent million
count ceilings, billion-byte semantic/resident ceilings, and 64,000,000
scratch bytes, then places `BROKEN` inside the first owner sequence. At old
head `3043a89e...`, the packet's own fixed budget block became the visitor
reservation authority. At Target C source head `c2e63c3d...`, the trusted
envelope rejects `max_owners=1,000,000 > 16` while still at the fixed budget
block, before the malformed tail is parsed and before any proportional
reservation occurs.

`Serialize<ResidentClearingPlan>` remains the canonical replay product, but
the public context-free `Deserialize<ResidentClearingPlan>` implementation is
removed. Its wire DTO, visitor, and `DeserializeSeed` are private beneath the
admitted door. A pinned `compile_fail,E0277` proves
`serde_json::from_str::<ResidentClearingPlan>` is unavailable; because the
missing trait is the generic `Deserialize` bound, `from_value` and equivalent
context-free generic routes are absent by the same type boundary. A valid
packet admitted by a consumer envelope remains byte- and digest-identical.
The six tiny-budget malformed-tail first-excess tests remain unchanged in
meaning and now also call the trusted door explicitly.

## Target D typed replay error channel

Serde's private visitor boundary still requires its concrete deserializer
error while it is walking the packet. A private `RefCell<Option<_>>` slot sits
beside the seeded visitor and records any plan-domain refusal before returning
through that boundary. The public door immediately recovers the untouched
domain value; the internal sentinel text is never exposed as caller authority.
Failures produced after the wire DTO is complete map directly to the same
domain arm.

```text
pub enum ResidentClearingReplayError<E> {
    Transport(E),
    Plan(ResidentClearingPlanError),
}
```

All domain-producing visitor paths use the slot: trusted-envelope admission,
bounded-sequence host representation/reservation/exact-capacity checks,
first-excess count or semantic-byte refusal, and variable payloads appearing
before budgets. `ResidentClearingPlan::from_wire` errors—including version,
range, budget, malformed wire, canonical-byte, digest, context, dictionary,
and ordinary reconstruction errors—map directly to `Plan`. Genuine serde
syntax/type/field failures with no stashed domain refusal remain
`Transport(D::Error)`.

The focused witness performs only structural matches for Target D:

- the all-large forged packet returns
  `Plan(WireBudgetExceedsTrustedEnvelope { field: "max_owners", claimed:
  1_000_000, admitted: 16 })` before its `BROKEN` tail;
- all nine componentwise trusted-envelope refusals compare their public
  variant fields directly;
- a parsed canonical-byte mutation returns
  `Plan(CanonicalBytesMismatch)`;
- rows appearing before budgets return
  `Plan(MalformedWire { field: "rows_before_budgets" })`;
- the incomplete JSON payload `{` returns `Transport(serde_json::Error)`;
- the six first-excess witnesses compare exact `CountBudgetExceeded` or
  `SemanticPlanBudgetExceeded` values.

No proof recovers identity or fields by parsing `Display`, `Debug`, or English.
The admitted positive replay still equals the original plan and retains the
same 308 canonical bytes and digest
`a61ebfee74156dd1e39bb8c5ec089ca4`.

## ABI and budget proof

Host admission reserves exactly `max_rows` rows after checked host narrowing,
then stores at most that many rows and at most each admitted axis count. The
max+1 witness supplies 1,000 rows and an adversarial enormous `size_hint`, yet
the constructor pulls exactly three items for `max_rows=2` and refuses the
third with logical length 2, reserved capacity 2, and zero reallocations. The
large-count/small-byte witness admits 128 rows but only 179 semantic bytes; it
pulls one row, projects the exact 180-byte first-row requirement, and refuses
before dictionary vectors, canonical rows, or complete semantic-plan storage
exist. Only after row, axis, and running-byte admission does canonical
dictionary/row materialization occur. Dense end,
count-times-stride, canonical byte length, scratch bytes, alignment, total
resident bytes, host allocation representation, and device `max_buffer_size`
remain checked before the first WGPU allocation.

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
row max+1, semantic-plan byte excess, and resident-byte excess before the
corresponding proportional host/GPU allocation.

## Serde policy census

| Surface | Policy | Executable proof |
|---|---|---|
| `TreeRealmId`, `ExecutionIncarnation`, `TreeExecutionContext` | no `Serialize` or `Deserialize`; authority cannot be forged as data | pinned `compile_fail,E0277` checks all three |
| `TreeExecutionAuthority`, `TreeGenerationAuthority`, `TreeExecutionBinding` | borrowed/live runtime authority only; no serde | no serde implementation and no public context constructor |
| `RealmQualified`, `SeamEmissionOrdinal`, `SeamFactId`, `SeamFact` | manual canonical seam bytes where defined; no general serde or raw emission-ordinal decode | type separation plus private-field construction |
| `DenseOrdinalRange` | custom deserialize through `try_new` | overflowing `start + len` rejects |
| `ResidentClearingRanges` | custom DTO; every range revalidated and canonical zero starts required | overflowing/noncanonical ranges reject |
| `ResidentClearingBudgets` | custom DTO reconstructed only through `new` | zero and scratch-inconsistent budgets reject |
| `ResidentClearingReplayEnvelope` | public consumer-constructed outer ceilings; private fields and no serde | all nine wire claims compare against it before any variable sequence reservation |
| `ResidentClearingReplayError<E>` | public generic `Transport(E)` / `Plan(ResidentClearingPlanError)` split | structural matches preserve exact trusted-envelope fields, canonical mismatch, malformed wire, and first-excess errors while incomplete JSON remains transport-typed |
| `ResidentClearingPlan` | `Serialize` only; no context-free `Deserialize`; its public replay door requires the trusted envelope and uses a private seeded top-level visitor before the existing bounded visitors and ordinary reconstruction | pinned `compile_fail,E0277`; valid admitted roundtrip; nine structurally matched outer-budget refusals; all-large forged budget plus malformed tail refuses at the budget block; six structurally matched tiny first-excess packets and the prior malformed invariant census remain green |

`ResidentPlanContext`, dictionaries, rows, ordinals, and digest expose no direct
`Deserialize` derive that could bypass plan reconstruction. Simple authored
resource/scope/draw ids retain transparent serde because their public raw-value
constructors carry no hidden invariant.

## Rev-6 RF-market archaeology (§11.1)

This read-only census compares live-master authority
`fd8ca8faf084694da86e14749dcd81bd55911a6d` with the accepted #1911 head.
It introduces no clearing implementation, mathematical framework, pressure
plane, lane, column, type, or bridge. Rows 2, 3, 4, and 8 are deliberately
described as bindings that complete in 14.3; row 11 prevents them from being
reported as present-tense closed.

| # | Required surface | Evidence and exact disposition |
|---:|---|---|
| 1 | upward lawful demand quantity | **REUSE AS-IS.** `crates/simthing-spec/src/spec/owner_channel_rf.rs::{OwnerChannelRfBucket, OwnerChannelRfReduceUpReport, StampedReduceUpProduct}` carries exact `deficit_total`/`net_deficit`; `crates/simthing-spec/src/spec/owner_silo_disburse_down.rs::RuntimeOwnerSiloDemandBucket` and `crates/simthing-spec/src/spec/constrained_clearing.rs::ConstrainedClaim::requested` are the existing Need/demand quantity doors. No second demand quantity is required. |
| 2 | branch-attributed subtree pressure | **REUSE WITH BINDING (14.3).** `crates/simthing-driver/src/arena_hierarchy.rs::{ArenaTreeLayout,HierarchyNode::children}` provides direct-child identity, and `crates/simthing-driver/src/arena_allocation_plan.rs::sum_reduction_ops` performs additive child reduction. The 14.3 binding must preserve one aggregate per direct child and must not add descendant scans or double counting. |
| 3 | neutral pressure→weight | **REUSE WITH BINDING (14.3).** `crates/simthing-core/src/accumulator_spec.rs::AccumulatorRole::AllocatorWeight`, `crates/simthing-driver/src/need_binding.rs`, and the existing staged EvalEML→`AllocatorWeight` door provide the operand and execution path. The binding is identity over eligible `F` for immediate flow and raw `P` for entitlement-first flow; PALMA and Gu-Yang remain born inputs and are not recomputed. |
| 4 | pressure→continuous allocation | **REUSE WITH BINDING (14.3).** `ArenaNodeColumns::weight_col`, `weight_sum_col`, and `crates/simthing-driver/src/child_share_eml.rs::child_share_formula` already provide the allocator operand and proportional share operation. Only the pressure/feedback binding remains. |
| 5 | continuous child share | **REUSE WITH BINDING (14.3).** `plan_arena_allocation` currently identity-broadcasts parent `allocated_flow_col` into child `propagated_allocated_flow_col`, then `disburse_op` writes the child's `allocated_flow_col` through `child_share_formula`. The operation and formula are reused, but the materialized propagated column means direct level-N `AllocatedFlow` self-consumption at N+1 is not yet closed; 14.3 owns that binding. |
| 6 | PALMA tree-vertical path cost | **REUSE WITH BINDING.** `crates/simthing-driver/src/field_sweep_compile.rs::{compile_palma_n4_field_sweep,PalmaN4FieldSweepSpec}` and the generic `FieldSweepSession`/min-plus field authority already own path impedance. A tree adjacency/prefix lowering may bind this authority; settlement may not create a private PALMA solver or second persistent plane. |
| 7 | Gu-Yang branch serviceability | **REUSE WITH BINDING.** `field_sweep_compile.rs::{compile_gu_yang_n4_field_sweeps,GuYangN4FieldSweepSpec}` owns admitted conductance/capacity and `simthing-spec::AdmittedActionBandConservedProgressBoundSource::{GuYangAvailable,GuYangRealized}` exposes the born serviceability products. Tree adjacency and directional-versus-symmetric edge policy remain binding questions, not permission for duplicate Gu-Yang work. |
| 8 | additive subtree pressure | **REUSE WITH BINDING (14.3).** `ArenaTreeLayout`, contiguous logical subtree spans, and the existing `CombineFn::Sum` RF reduction path preserve additive pressure. No max/tropical replacement or parallel pressure ledger is admitted. |
| 9 | dirty vertical state | **REUSE WITH BINDING.** `crates/simthing-kernel/src/derived_span_projection.rs::{ChangedLocus,DerivedDependencyIndex,DerivedInvalidation}` already owns source-blind dependency invalidation over logical spans. The market binding must reuse it; no second dirty subsystem is needed. |
| 10 | hard economic precedence | **REUSE AS-IS.** `ConstrainedClaim::{priority,order_weight}`, `AuthoredClearingProgram::score`, and the canonical score-band sort in `constrained_clearing.rs` are the existing economic authorities. RF `OrderBand` is execution sequencing only and is not cited as economic precedence. |
| 11 | urgency / first-order unresolved persistence | **Archaeology finding: `MISSING`; disposition: `REHOME CHARTERED`; consumer/rung: `RESIDENT-CLEARING-SCORE-AND-BANDS-0` (14.3).** Per DA ruling `5488315659`, unresolved ordinary recursive demand product `T_d` with `u > 0` at N must enter N+1 lawful demand exactly once through the existing bounded Current→Next state plane. There is no #1911 source delta, dated constitutional debt, authored bridge, new lane, new column, or new type. `ConstrainedGrant::unresolved` and `UnresolvedDemandObservation` remain observation; `fund_unresolved_persistence` is optional authored EML/CostBand/Overlay deformation, never first-order recurrence. |
| 12 | exact discrete settlement | **REUSE AS-IS.** `crates/simthing-spec/src/spec/constrained_clearing.rs::clear_constrained_scope` owns checked integer totals, base shares, fractional remainders, generation-rotated canonical ties, and sealed `ConstrainedGrant` provenance. This is the exact quantizer/oracle; 14.2 does not alter it. |
| 13 | exact settlement RF band slot | **REUSE WITH BINDING (14.4).** `crates/simthing-driver/src/arena_hierarchy.rs::ArenaBandLayout::integration_band` is terminal after all upsweep/broadcast/disburse bands; `plan_arena_allocation` appends governed integration there. Exact settlement binds at that terminal RF stage, not as a peer clearing path. |
| 14 | holding accounts | **REUSE AS-IS.** `crates/simthing-core/src/residency_tier.rs::ResidencyCapacityPartition` owns exact `capacity`, `free`, `in_flight`, and `occupied` partitions plus issue/deliver/cancel/release transitions and `verify_exact`. |
| 15 | recursive grant subdivision | **REUSE AS-IS.** `crates/simthing-driver/src/growth_entitlement.rs::GrowthEntitlementMarketBinding` accepts an arbitrary granter, derives capacity from `growth_capacity_available(granter)`, clears child Draws, and records grants; `crates/simthing-spec/src/spec/flow_market.rs` owns partition/fusion. Child-as-granter is already the degenerate interior germ. |
| 16 | settlement-product / supply-intake identity | **REHOME EXISTING AUTHORITY (14.4).** Exact quantity/provenance already lives in `ConstrainedGrant` and durable `flow_market.rs::MarketGrantRecord`, while current oracle intake is the distinct `ConstrainedSupply`. There is no live parent-settlement→child-intake recursive edge today. 14.4 must rehome those roles onto one canonical exact `T_s` with aliases or conversion-free views; `MarketGrantRecord::from_cleared_offering` must not become a recursive adapter. |
| 17 | detached-seam exact-product identity | **REUSE WITH BINDING (14.6).** `crates/simthing-core/src/generation_stamp.rs::GenerationStamped<T>` and #1911 `crates/simthing-core/src/tree_execution_context.rs::SeamFact<TLocalId>` are payload-parametric authority envelopes. No constrained supply/grant currently crosses a detached seam; future `T_s` must remain the identical `T` beneath those wrappers. |
| 18 | replay/history | **REUSE WITH BINDING (14.6).** `generation_stamp.rs::IntegrationSchedule` is the sole append-only history and already includes grant lifecycle kinds. The #1911 plan has only a consumer-owned replay envelope and semantic plan serialization, not a second schedule. The resident `IntegrationSchedule` head remains chartered to cutover. |

There are zero new `MISSING — STOP` findings. Row 11 is the one already-ruled
`MISSING` finding and is recorded exactly as its DA-chartered 14.3 rehome.

### Leaf → branch → parent allocation trace

The current and chartered bindings form one trace: leaf `RuntimeOwnerSiloDemandBucket.requested`
enters `OwnerChannelRfBucket::{deficit_total,net_deficit}`; direct-child
aggregation uses `ArenaTreeLayout::children` plus additive `sum_reduction_ops`;
14.3 binds the resulting eligible `F`/raw `P` by identity to the existing
`AllocatorWeight`; `child_share_formula` consumes parent intrinsic/allocated
flow and child weight to write child `AllocatedFlow`; 14.4 then runs the
existing exact constrained quantizer at `ArenaBandLayout::integration_band`
and exposes its canonical `T_s` directly as child exact supply. The last two
arrows are explicitly in-flight 14.3/14.4 bindings, not claims that the full
recursive production path already exists.

Draw is authorization metadata, not a second `T_d`. In
`crates/simthing-spec/src/spec/flow_market.rs`, a sealed Draw authorizes the
runtime demand emission, while `RuntimeOwnerSiloDemandBucket.requested` is the
one quantity subsequently admitted by `ConstrainedClaim::from_runtime_demand`.

## Recursive-port census (§11.2)

| Port | Complete producer→consumer path and transition census | Disposition |
|---|---|---|
| continuous supply | `plan_arena_allocation` reads parent `allocated_flow_col`; `broadcast_op` uses `CombineFn::Identity` to write child `propagated_allocated_flow_col`; `child_share_formula` reads that f32 with propagated intrinsic flow, child weight, and propagated weight sum; `disburse_op`/EvalEML writes child `allocated_flow_col`. There is no serialization, ABI crossing, newtype, conversion trait, or numeric retype, but there is one physical copy into a separate role column. | Existing formula/operation reused; direct self-consumption binding completes at 14.3 (matrix row 5). |
| exact constrained supply | Current oracle path is `clear_constrained_scope(ConstrainedSupply, ConstrainedClaim[]) -> ConstrainedGrant[]`; `record_cleared_grant` invokes `MarketGrantRecord::from_cleared_offering` and the host residency bridge. A later `GrowthEntitlementMarketBinding` independently constructs `ConstrainedSupply { available: growth_capacity_available(granter) }`. No function currently connects a parent grant to a child exact-supply intake, so the copies are not a live recursive adapter. | Existing exact authority is rehomed to one canonical `T_s` in 14.4; no `GrantRow`/`ChildSupplyRow` pair may be added. |
| upward demand | `reduce_owner_channel_rf` returns `StampedReduceUpProduct`, a compile-time alias for `GenerationStamped<OwnerChannelRfReduceUpReport>`; parent `integrate_stamped_reduce_up` borrows that exact alias. Serialization is only the generic generation envelope; async conservation may use `OwnerChannelRfConservedValue` as a holding account, while replay still consumes the stamped products. No projection, role newtype, conversion trait, or ABI transition changes `T_d`. | **REUSE AS-IS.** |
| unresolved demand across time | `ConstrainedGrant::unresolved` → `UnresolvedDemandObservation` exists only as observation. `fund_unresolved_persistence` is the optional authored EML/CostBand/Overlay consequence and does not emit first-order demand. There is presently no ordinary `T_d(N) -> T_d(N+1)` edge. | Row-11 **REHOME CHARTERED** to 14.3: once-only Current→Next recurrence, no same-generation re-clear. |
| detached exact supply | No current constrained grant/supply payload is serialized across `SeamFact`. The available envelopes are generic `GenerationStamped<T>` and realm/incarnation-aware `SeamFact<TLocalId>`; their APIs borrow or return the same generic payload and introduce no economic conversion. | Bind identical canonical `T_s` beneath the envelope at 14.6; no payload translator exists or is admitted. |

### Alias and detached-payload proofs

The present compile-time alias proof is the declaration
`type StampedReduceUpProduct = GenerationStamped<OwnerChannelRfReduceUpReport>`:
`reduce_owner_channel_rf` returns it and `integrate_stamped_reduce_up` accepts
`&StampedReduceUpProduct`, so no `From`, `Into`, constructor, copy, projection,
or runtime “types match” validator can occur between the recursive demand
roles. For exact supply, no paired grant/supply role newtypes exist in #1911;
the canonical `T_s` role aliases are a 14.4 obligation rather than a fictional
14.2 compile proof.

`GenerationStamped<T>::product` returns `&T` and `into_product` returns `T`;
the seam substrate is likewise generic and carries realm/generation/ordinal
authority outside its subject. Therefore a detached seam can wrap and unwrap
the same future `T_s` payload type without economic projection. This is a
structural payload-identity proof; there is intentionally no production exact
supply seam consumer in 14.2.

## Settled-code census (§11.3)

| Settled surface | Classification | Evidence |
|---|---|---|
| #1911 resident-clearing plan ABI | **CONFORMING** | `ResidentClearingAdmission` and `ResidentClearingRow` contain only owner/resource/scope/draw identities or ordinals. The GPU ABI mirrors those neutral axes. There is no claim, supply, grant, child-supply, score, band, quantity, or economic-role row. |
| resident/host grant-disbursement lanes | **NOT IN SCOPE** | `record_cleared_grant` and the residency conversion bridge are one-way structural placement boundaries, not a recursive parent-output→child-input port. Their exact-copy shape is recorded so it cannot be mistaken for future `T_s` recursion. |
| grant lifecycle and holding-account rows | **CONFORMING** | `MarketGrantRecord` partition/fusion and `ResidencyCapacityPartition` exact transitions preserve quantity and conservation; grant lifecycle kinds share the one `IntegrationSchedule`. |
| Draw grammar and runtime quantity authority | **CONFORMING** | Draw seals authorization metadata. The emitted `RuntimeOwnerSiloDemandBucket.requested` remains the only quantity admitted into `ConstrainedClaim`; there is no Draw-side demand quantity universe. |
| authored unresolved-demand persistence | **IN-FLIGHT FIX** | It remains optional secondary deformation. The missing ordinary once-only `T_d` recurrence is explicitly owned by 14.3 under DA ruling `5488315659`, with no 14.2 source delta and no dated debt. |
| seam payloads carrying constrained grants/supply | **NOT IN SCOPE** | None exist. The generic stamp/seam authority is conforming and payload-neutral; the first exact `T_s` seam binding belongs to 14.6. |

No settled mismatch requires a dated constitutional-debt row. Every open
recursive binding has a named in-flight consumer: pressure, feedback, direct
continuous self-consumption, and ordinary unresolved recurrence in 14.3;
canonical exact `T_s` identity/settlement in 14.4; detached exact supply and
resident schedule-head integration in 14.6.

### OQ23 and falsifiers 26–34

**OQ23: CONFORMING; no source fix.** The probationary kernel and GPU plan ABIs
are ontology-neutral. Repository-wide type evidence finds no `ClaimRow`,
`SupplyRow`, `GrantRow`, or `ChildSupplyRow`; the only plan row is the four-axis
`ResidentClearingRow`. Thus #1911 freezes layout identity, budgets, and storage,
not separate recursive economic products.

| Falsifier | Evidence |
|---:|---|
| 26 | No live parent-`T_s`→child-`T_s` edge exists on which a semantic adapter can hide; row 16 assigns canonical identity to 14.4 and explicitly excludes `from_cleared_offering` as a recursive adapter. |
| 27 | Row 11 assigns ordinary unresolved `T_d` directly to the bounded Current→Next plane in 14.3; `fund_unresolved_persistence` remains optional authored deformation. |
| 28 | OQ23 proves the accepted resident ABI contains no separate grant/child-supply row ontologies. |
| 29 | The existing upward recursive port is compile-time alias identity; no runtime equality validator substitutes for type identity. |
| 30 | Draw authorizes emission; `RuntimeOwnerSiloDemandBucket.requested` is the quantity authority and lowers directly into `ConstrainedClaim::requested`. |
| 31 | `T_d` and future `T_s` remain distinct unless independent semantic proof resolves design OQ21; this census does not collapse them for elegance. |
| 32 | No paired recursive role newtypes or `From`/`Into` bridges exist in #1911; 14.4 is constrained to aliases or conversion-free views over one canonical `T_s`. |
| 33 | No exact supply currently crosses a seam. `GenerationStamped<T>` and the seam authority preserve the same generic payload; 14.6 may add wrappers but cannot change economic `T_s`. |
| 34 | All exposed gaps are tracked above as named 14.3/14.4/14.6 in-flight bindings; the six-surface census has no untracked mismatch and no constitutional-debt candidate. |

The census reuses the existing RF accumulator, child-share EML, FieldSweep,
constrained-clear, residency partition, grant lifecycle, generation stamp,
seam, and schedule authorities. No new mathematical framework was introduced
merely because the promoted design contains research analogies.

## Containment and consumer census

- Production consumers: zero. Mechanical callers of `ResidentClearingPlan::build`
  and `ResidentClearingBuffers::allocate` are only the focused workshop test;
  the workshop library only observes a borrowed final-home plan.
- New global semantic state, singleton, lock, allocator, schedule, or
  generation authority: zero. The one live caller-owned generation record also
  owns its once-mint atomic; incarnation state remains per admitted capsule.
  The implementation owns only values, one private per-capsule `Arc`, atomics
  inside those caller/per-instance records, or per-instance buffers.
- Frozen clearing semantics: no source changed in `simthing-spec`,
  `simthing-driver`, `simthing-sim`, `simthing-feeder`,
  `simthing-clausething`, or `simthing-mapeditor`; no score, equality-band,
  apportionment, grant, unresolved-U, replay, or structural-consequence code
  was added.
- Dispatch surface: no Cargo manifest/lockfile, WGSL, shader include,
  pipeline, command encoder, or `dispatch_workgroups` delta.
- The governed `kernel_surface.txt` adds only the 22 deliberate 14.2 exports
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

Evidence/governance (8):

- `docs/tests/resident_clearing_plan_0_results.md`
- `docs/tests/current_evidence_index.md`
- `scripts/ci/test_inventory.tsv`
- `scripts/ci/closeout_artifacts.tsv`
- `scripts/ci/anchor_reach_log.tsv`
- `scripts/ci/allow/kernel_surface.txt`
- `docs/sanctioned_surface.md`
- `scripts/ci/constitutional_surfaces.tsv`

No other file is in the exact dispatch-to-head delta. The seventeen-file
envelope is the original sixteen plus the exact remand-authorized constitutional
debt row. The allowlist is the
scanner-prescribed record for the new public kernel doors; no CI `.sh` or
`.py` code changed.

Source-versus-evidence commit map:

- remand source and executable witness: `e9042e40960aace583fb79670080530870cd05c0`
- continuation A/B source and executable witness:
  `6649589388a8e440949638a8ce778999ddc97c31`
- target C source and executable witness:
  `c2e63c3d65a5aa0dac6ba55c89da6828cb05034f`
- target D replay-error source and executable witness:
  `77c10b1be5afb869afbb296e321025bd4c439eb3`
- evidence/governance tail and exact final head: carried by the PR relay after
  all local/hosted checks, avoiding a self-referential hash in this file

## Local certificate

- `cargo check -p simthing-core -p simthing-kernel -p simthing-gpu -p simthing-workshop` — PASS
- `cargo test -p simthing-workshop --test resident_clearing_plan_0 -- --nocapture` — PASS, 3/3, real adapter
- `cargo test -p simthing-kernel --doc resident_clearing_plan --offline` — PASS, trusted-door compile-fail 1/1
- `cargo test -p simthing-core tree_execution_context --offline -- --test-threads=1 --nocapture` — PASS, stale-remap falsifier 1/1
- `cargo test -p simthing-core --doc tree_execution_context --offline` — PASS, authority-serde compile-fail 1/1
- `cargo test -p simthing-workshop --test generation_critical_path_baseline_0 --offline -- --test-threads=1 --nocapture` — PASS, 4/4; generated timing packet restored byte-identically because 14.2 must not refresh 14.1 measurements
- `cargo test -p simthing-driver --test contention_arena_executed_0` — PASS, 1/1
- `cargo test -p simthing-driver --test clearing_weight_semantic_partition_0` — PASS, 2/2
- `bash scripts/ci/test_inventory_check.sh` — PASS, 1370 discovered / 1370 inventoried
- `bash scripts/ci/test_inventory_drift_check.sh` — PASS, unledgered 0 / stale 0
- `bash scripts/ci/test_lifecycle_expiry_check.sh --schema` — PASS, expired 0 / audit 0
- `bash scripts/ci/track_closeout.sh --artifact-expiry` — PASS, expired 0 / cruft 0 / malformed 0
- `bash scripts/ci/agent_scan.sh` — PASS, hard failures 0 / inspect 0
- `bash scripts/ci/doc_budget_check.sh --check` — PASS
- `bash scripts/ci/anchor_check.sh` — PASS (coverage/curation informational INSPECT only)
- `bash scripts/ci/gen_digest.sh --check` — PASS
- `git diff --check` — PASS

Hosted Doctrine Scan/Exec plus fresh exact-head `/clearance` and `/relay-lint`
are the PR relay's final binding proof. Coding returns PROBATION and does not
merge, graduate, move the pointer, invoke closeout, or begin 14.3.
