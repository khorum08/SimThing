# AS-INDEX-NEWTYPES-0 / 0B Results

## Status

**0A** — PROBATION — `SlotIndex` on `SlotAllocator` + production slot paths (#969 `e39b85ba59`).

**0B** — PROBATION — AccumulatorOp/builder CPU slot/column axes typed (`SlotIndex` / `ColumnIndex`); GPU encoding parity preserved; upload-boundary residue classified. DA re-review required before AS-5 → DONE — DA-APPROVED (do not self-promote).

## PR / branch / merge

### AS-INDEX-NEWTYPES-0A

- Branch: `codex/as-index-newtypes-0`
- PR: https://github.com/khorum08/SimThing/pull/969
- Merge: `e39b85ba59` (master)

### AS-INDEX-NEWTYPES-0B

- Branch: `codex/as-index-newtypes-0b`
- PR: https://github.com/khorum08/SimThing/pull/970
- Merge: `ffae0eb5f6` (master)

## What changed

### 0A

- Added `crates/simthing-core/src/slot_index.rs`: private-field `SlotIndex(u32)` with `new` / `raw` / `as_usize` / `saturating_add`; exported from `simthing-core` and re-exported from `simthing-gpu`.
- Migrated `simthing-gpu::SlotAllocator` public slot surface to `SlotIndex`; production adoption across driver/gpu/feeder/sim.

### 0B

- Added `crates/simthing-core/src/column_index.rs`: private-field `ColumnIndex(usize)` with `new` / `raw` / `raw_u32`; removed `pub type ColumnIndex = usize` alias from `property.rs`.
- Migrated CPU-side `AccumulatorOp` / builder surfaces to `SlotIndex` + `ColumnIndex` (see audit below).
- GPU encode bridge (`accumulator_op/encode.rs`): explicit `.raw()` / `.raw_u32()` at packing boundary; `threshold_registrations_to_ops` wraps u32 → newtypes on ingress.
- Downstream adoption: `simthing-spec` region-field admission, `simthing-driver` resource-economy/runtime harnesses, `simthing-gpu` accumulators + session/cpu_oracle/runtime tests.

## AccumulatorOp slot/column audit

| Surface | Axis | Type after 0B |
|---|---|---|
| `SourceSpec::SlotValue.slot` | buffer row | `SlotIndex` |
| `SourceSpec::SlotValue.col` | global flat column | `ColumnIndex` |
| `SourceSpec::SlotRange.start` | buffer row | `SlotIndex` |
| `SourceSpec::SlotRange.col` | global flat column | `ColumnIndex` |
| `SourceSpec::SlotRange.count` | width | raw `u32` |
| `InputSpec.slot` / `.col` | row / column | `SlotIndex` / `ColumnIndex` |
| `CombineFn::WeightedMean.weight_col` | global flat column | `ColumnIndex` |
| `ScaleSpec::ByColumn.col` | global flat column | `ColumnIndex` |
| `AccumulatorOp.targets` | `(slot, col)` | `(SlotIndex, ColumnIndex)` |
| `ConjunctiveRecipeInput.slot` | buffer row | `SlotIndex` |
| `ConjunctiveRecipeRegistration.target_slot` | buffer row | `SlotIndex` |
| `DiscreteTransferRegistration.source_slot` / `.target_slot` | buffer row | `SlotIndex` |
| `EmitOnThresholdRegistration.slot` | buffer row | `SlotIndex` |
| `ColumnAwareReductionSpec.col` | global flat column | `ColumnIndex` |
| `AccumulatorOpBuilder` slot/col params | row / column | `SlotIndex` / `ColumnIndex` |
| `CombineFn::EvalEML.tree_id` | tree id | raw `u32` |
| `GateSpec::OrderBand` | band id | raw `u32` (distinct enum variant) |
| `ThresholdRegistration` (GPU upload struct) | row / column | raw `u32` (upload boundary) |
| `ThresholdEvent` / readback payloads | row / column / event | raw `u32` |
| `IntentDelta` | row / column | raw `u32` (GPU affine upload) |
| `AccumulatorOpGpu` / `AccumulatorInputGpu` | row / column | raw `u32` (WGSL layout) |

**ColumnIndex introduced** — AccumulatorOp `col` fields index the global flat values matrix (`slot * n_dims + col`), not AS-1 layout lanes. `RoleOffset` remains for property layout resolution only.

## Upload-boundary residue

| Struct / path | Fields | Why raw |
|---|---|---|
| `AccumulatorOpGpu` | `source_slot`, `source_col`, `target*_slot/col` | WGSL `bytemuck` layout |
| `AccumulatorInputGpu` | `slot`, `col` | conjunctive input list buffer |
| `ThresholdRegistration` | `slot`, `col` | session upload before `threshold_registrations_to_ops` |
| `ThresholdEvent` / emission readback | `slot`, `col`, `event_kind` | GPU byte mirror |
| `IntentDelta` | `slot`, `col` | intent affine upload |
| `EncodeError::BootstrapContention` | `slot`, `col` | decoded from GPU op rows |
| Shadow-row helpers | `slot: u32` | post-`.raw()` matrix indexing |

Every CPU→GPU conversion is explicit and local: `slot.raw()`, `col.raw_u32()`, `role_offset.lane() as u32`.

## OrderBand audit

OrderBand audit: **no change** — `GateSpec::OrderBand(u32)` is already a distinct enum variant, not transposed with slot/column registration surfaces.

## Owner-vs-spatial-parent verification

Owner-vs-spatial-parent obligation satisfied by AS-2 `OwnerRef` + `SimThingId` type split; compile_fail `owner_ref_rejects_spatial_parent_compile_fail` on `channel_key`.

## Load-bearing proofs

### 0A

| Proof | Location | Catches |
|---|---|---|
| `slot_index_fields_private_compile_fail` | `slot_index.rs` doc-test | Bare `SlotIndex(0)` forgery |
| `slot_index_rejects_role_offset_compile_fail` | `slot_index.rs` doc-test | `SlotIndex` passed where `RoleOffset` expected |
| `role_offset_rejects_slot_index_compile_fail` | `slot_index.rs` doc-test | `RoleOffset` passed where `SlotIndex` expected |
| `slot_allocator_rejects_raw_integer_slot_compile_fail` | `slot.rs` doc-test | Raw `u32` passed to typed allocator API |
| `owner_ref_rejects_spatial_parent_compile_fail` | `channel_key.rs` doc-test | `OwnerRef` passed where spatial `SimThingId` expected |
| `slot_allocator_behavior_preserved_after_slot_index_newtype` | `slot_index.rs` lib test | Newtype round-trip / ordering invariants |
| `migrated_index_path_behavior_preserved` | `slot_index.rs` lib test | Contiguous slot-range arithmetic preserved |
| `slot_index_newtype_preserved_through_allocator_api` | `slot.rs` lib test | Allocator idempotency + tombstone LIFO through typed API |
| `slot::tests` (10 tests) | `slot.rs` | Full allocator behavior preserved |

### 0B

| Proof | Location | Catches |
|---|---|---|
| `accumulator_op_rejects_raw_integer_target_slot_compile_fail` | `accumulator_op.rs` doc-test | Raw integer in `targets` |
| `accumulator_builder_rejects_raw_integer_slot_compile_fail` | `accumulator_op_builder.rs` doc-test | Raw slot in builder API |
| `accumulator_builder_rejects_role_offset_as_slot_compile_fail` | `accumulator_op_builder.rs` doc-test | `RoleOffset` as slot |
| `accumulator_builder_rejects_slot_as_role_offset_compile_fail` | `accumulator_op_builder.rs` doc-test | `SlotIndex` as role offset |
| `accumulator_builder_rejects_raw_integer_column_compile_fail` | `accumulator_op_builder.rs` doc-test | Raw column in builder API |
| `accumulator_builder_rejects_slot_as_column_compile_fail` | `accumulator_op_builder.rs` doc-test | `SlotIndex` as column |
| `column_index_fields_private_compile_fail` | `column_index.rs` doc-test | Bare `ColumnIndex(0)` forgery |
| `slot_index_rejects_column_index_compile_fail` | `column_index.rs` doc-test | `ColumnIndex` as `SlotIndex` |
| `column_index_rejects_slot_index_compile_fail` | `column_index.rs` doc-test | `SlotIndex` as `ColumnIndex` |
| `column_index_rejects_role_offset_compile_fail` | `column_index.rs` doc-test | `RoleOffset` as global column |
| `accumulator_builder_emits_same_op_after_index_newtypes` | `accumulator_op_builder.rs` lib test | Builder parity after typing |
| `accumulator_op_gpu_encoding_preserved_after_index_newtypes` | `accumulator_op/encode.rs` lib test | Byte-identical GPU rows |

## Encoding parity

- `accumulator_builder_emits_same_op_after_index_newtypes` — PASS
- `accumulator_op_gpu_encoding_preserved_after_index_newtypes` — PASS
- `AccumulatorOp::validate` — unchanged rejection shapes
- Threshold registration GPU rows — slot/col/event payloads preserved via explicit `.raw()` at encode

## Scope Ledger

| File / area | Why touched |
|---|---|
| `crates/simthing-core/src/slot_index.rs` | 0A rung type + proofs |
| `crates/simthing-core/src/column_index.rs` | 0B rung type + proofs |
| `crates/simthing-core/src/{accumulator_op,accumulator_op_builder,property,lib}.rs` | 0B CPU surface migration |
| `crates/simthing-gpu/src/accumulator_op/{encode,cpu_oracle,session,runtime}.rs` | 0B encode bridge + test fixes |
| `crates/simthing-gpu/src/{emission,intensity,transfer}_accumulator.rs` | 0B production adoption |
| `crates/simthing-spec/src/compile/region_field_admission.rs` | 0B production adoption |
| `crates/simthing-driver/src/*` (resource economy, runtime harnesses) | 0B compiler-forced adoption |
| `docs/tests/as_index_newtypes_0_results.md` | Evidence ledger |
| `docs/tests/current_evidence_index.md` | Index row |
| `docs/design_0_0_8_4_admission_substrate.md` | Ladder row + graduation log |

**Not touched:** AS-8 PackedUpload, AS-F closeout, 0.0.8.5 Terran-Pirate, AccumulatorOp semantic redesign, OrderBand behavior, WGSL semantics.

## Known AS-5 residue

- **Integration tests** — `simthing-sim/tests/*` may still use raw slot/col literals in non-AccumulatorOp paths; opportunistic follow-on.
- **Shadow-row helpers** — internal `zero_shadow_row(..., slot: u32)` intentionally take raw upload indices after `.raw()` extraction.
- **DA promotion** — AS-5 remains PROBATION until executive DA re-review after 0B merge.

## Validation (targeted)

- `cargo fmt -p simthing-core -p simthing-gpu -p simthing-driver -p simthing-sim -- --check` — PASS
- `cargo check -p simthing-core`, `-p simthing-gpu`, `-p simthing-driver`, `-p simthing-sim`, `-p simthing-spec` — PASS
- `cargo test -p simthing-core --doc` — PASS (20 compile_fails incl. column_index + builder)
- `cargo test -p simthing-core accumulator_builder_emits_same_op_after_index_newtypes --lib` — PASS
- `cargo test -p simthing-gpu accumulator_op_gpu_encoding_preserved_after_index_newtypes --lib` — PASS
- `cargo test -p simthing-gpu slot::tests --lib` — PASS (10 tests)
- Prior gates (semantic-free, kind audit, structural coord, channel newtypes) — PASS
- Scope grep: no `design_0_0_8_5_clausescript_terran_pirate_galaxy.md` diff — PASS

## Known gaps / next

- Executive DA re-review: PROBATION → DONE — DA-APPROVED for AS-5 after 0B merge review.
- **AS-8 PackedUpload** — not started (separate rung).
- **AS-F closeout** — not started.
