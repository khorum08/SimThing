# AS-INDEX-NEWTYPES-0 Results

## Status

PROBATION — `SlotIndex` adopted on the load-bearing `SlotAllocator` public boundary and production slot-resolution paths; slot vs layout-lane axes proved uncompilable; AccumulatorOp/GPU upload residue classified for **AS-INDEX-NEWTYPES-0B**.

## PR / branch / merge

- Branch: `codex/as-index-newtypes-0`
- PR: (pending)
- Merge: (pending)

## What changed

- Added `crates/simthing-core/src/slot_index.rs`: private-field `SlotIndex(u32)` with `new` / `raw` / `as_usize` / `saturating_add`; exported from `simthing-core` and re-exported from `simthing-gpu`.
- Migrated `simthing-gpu::SlotAllocator` public slot surface to `SlotIndex` (`alloc`, `tombstone`, `slot_of`, `owner_of`, `is_live`, gap/contiguous reservation, `claim_exclusive_slot`); `SlotAllocError` carries typed slots.
- Production adoption: driver `SlotId = SlotIndex`; compiler-forced `.raw()` / `.as_usize()` at GPU upload and shadow-row indexing boundaries across driver, gpu passes, feeder patcher, and sim boundary/fission/tree_mutation paths.
- `ColumnIndex` private newtype **not** introduced — migrated production path uses `SlotIndex` + AS-1 `RoleOffset` only (see axis audit).

## Slot/column/role-offset axis audit

| Category | Examples in this slice | Action |
|---|---|---|
| **SlotIndex** — GPU buffer row | `SlotAllocator` public API, `arena_registry::SlotId`, slot→owner lookups | Migrated to `SlotIndex` |
| **RoleOffset** — layout-resolved lane | `PropertyLayout::offset_of`, `RoleOffset::lane()`, threshold builder col resolution | Unchanged (AS-1); compile_fail vs `SlotIndex` |
| **Global flat column** | No distinct public CPU registration surface beyond registry `total_columns` / shadow `slot * n_dims + lane` | **ColumnIndex not introduced** — shadow helpers keep `u32` at upload boundary with explicit `.raw()` |
| **Raw numeric residue** | `ThresholdEvent.slot/col` u32, `AccumulatorOp` slot/col fields, `event_kind`, `n_dims`, WGSL packing | Left raw at final packing boundary (AS-5 residue → 0B) |

## OrderBand audit

OrderBand audit: **no change** — `GateSpec::OrderBand(u32)` is already a distinct enum variant, not transposed with slot/column registration surfaces in this slice.

## Owner-vs-spatial-parent verification

Owner-vs-spatial-parent obligation satisfied by AS-2 `OwnerRef` + `SimThingId` type split; compile_fail `owner_ref_rejects_spatial_parent_compile_fail` on `channel_key`.

## Load-bearing proofs

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

## Scope Ledger

| File / area | Why touched |
|---|---|
| `crates/simthing-core/src/slot_index.rs` | Rung type + proofs |
| `crates/simthing-core/src/lib.rs` | Export `SlotIndex` |
| `crates/simthing-gpu/src/slot.rs` | Primary `SlotAllocator` migration + compile_fail |
| `crates/simthing-gpu/src/lib.rs` | Re-export `SlotIndex` |
| `crates/simthing-gpu/src/{overlay_prep,projection,reduction,passes}.rs` | Compiler-forced slot `.raw()` at GPU paths |
| `crates/simthing-feeder/src/patcher.rs` | Compiler-forced slot indexing |
| `crates/simthing-driver/src/arena_registry.rs` | `SlotId = SlotIndex` production alias |
| `crates/simthing-driver/src/{arena_*,session,install,scenario,spec_session,resource_flow_*,gated_rates,min_plus_traversal_field,resource_economy_compile}.rs` | Compiler-forced SlotIndex adoption |
| `crates/simthing-sim/src/{boundary,fission,tree_mutation,overlay_lifecycle,property_expiry,threshold_registry,observability}.rs` | Compiler-forced slot API + test fixes |
| `crates/simthing-spec/src/spec/channel_key.rs` | Owner-vs-spatial-parent compile_fail |
| `docs/tests/as_index_newtypes_0_results.md` | Evidence ledger |
| `docs/tests/current_evidence_index.md` | Index row |
| `docs/design_0_0_8_4_admission_substrate.md` | Ladder row + graduation log |

**Not touched:** AS-8 PackedUpload, AS-F closeout, 0.0.8.5 Terran-Pirate, AccumulatorOp redesign, OrderBand behavior, WGSL semantics.

## Known AS-5 residue

- **AccumulatorOp / builder** — `slot` / `col` fields remain `u32` on the op struct and GPU encoding path; migrate in **AS-INDEX-NEWTYPES-0B** with `accumulator_builder_emits_same_op_after_index_newtypes` / `accumulator_op_gpu_encoding_preserved_after_index_newtypes`.
- **Integration tests** — `simthing-sim/tests/*` still use `.slot_of(...).unwrap() as usize` patterns; not required for this slice’s targeted gates; fix opportunistically in 0B or compiler-forced follow-on.
- **Shadow-row helpers** — internal `zero_shadow_row(..., slot: u32)` and fission copy helpers intentionally take raw upload indices after `.raw()` extraction.

## Validation (targeted)

- `cargo fmt -p simthing-core -p simthing-gpu -p simthing-driver -- --check` — PASS
- `cargo check -p simthing-core`, `-p simthing-gpu`, `-p simthing-driver`, `-p simthing-sim` — PASS
- `cargo test -p simthing-core --doc` — PASS (10 compile_fails incl. slot_index)
- `cargo test -p simthing-gpu --doc` — PASS (`slot_allocator_rejects_raw_integer_slot_compile_fail`)
- `cargo test -p simthing-core slot_index --lib` — PASS (2 behavior tests)
- `cargo test -p simthing-gpu slot::tests --lib` — PASS (10 tests)
- `cargo test -p simthing-sim as_sim_semantic_free_public_surface_audit --lib` — PASS
- `cargo test -p simthing-sim as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads --lib` — PASS
- `cargo test -p simthing-core structural_coord_integer_roundtrip_preserved --lib` — PASS
- `cargo test -p simthing-spec --test as_channel_newtypes_0` — PASS
- `cargo test -p simthing-spec as_channel_newtypes_production_adoption --lib` — PASS
- Scope grep: no `design_0_0_8_5_clausescript_terran_pirate_galaxy.md` diff — PASS

## Known gaps / next

- **AS-INDEX-NEWTYPES-0B** — AccumulatorOp/builder slot/col migration + encoding parity tests.
- Optional: private-field `ColumnIndex` if a global flat-column CPU registration surface is opened beyond registry layout math.
- DA re-review for PROBATION → DONE after 0B residue closes or is accepted as upload-boundary-only.
