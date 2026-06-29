# AS-PACKED-UPLOAD-BOUNDARY-0 Results

## Status

DONE -- DA-APPROVED -- structural GPU upload sealed behind PackedUpload; byte parity preserved. DA-approved per graduation log (2026-06-29, PR #973).

## PR / branch / merge

- Branch: `codex/as-packed-upload-boundary-0`
- PR: https://github.com/khorum08/SimThing/pull/973
- Merge: `35afe6e5e7` (master)

## What changed

- Added `PackedUpload` in `crates/simthing-gpu/src/structural_upload.rs` with private fields, `new()` count validation, and accessors.
- Changed `upload_structural_rows_to_gpu(device, queue, upload: &PackedUpload)` — free frame + row slice bundle removed from public API.
- `TryFrom<StructuralUploadRows>` / `TryFrom<&StructuralUploadRows>` bridge validation paths to packed upload.
- `readback_matches_source` now compares against `&PackedUpload`.
- Call sites: `structural_validation.rs`, `simthing-mapeditor/scenario_projection.rs` (`prove_gpu_buffer_residency_blocking`).
- Exported `PackedUpload` from `simthing-gpu`.

## Upload-boundary audit

| Surface | Public upload args | Classification |
|---|---|---|
| `upload_structural_rows_to_gpu` | `&PackedUpload` only | **Sealed (this rung)** |
| `validate_structural_rows_on_gpu` | `&StructuralUploadRows` (pre-pack) | Acceptable — packs via `PackedUpload::try_from` before upload |
| `AccumulatorOpSession::upload_ops` | `&[AccumulatorOp]` CPU semantic ops | **AS-8 residue** — encode to `AccumulatorOpGpu` inside session; not structural upload |
| `AccumulatorOpSession::upload_threshold_ops` | `&[ThresholdRegistration]` raw GPU rows | **AS-8 residue** — byte mirror upload; future packed threshold packet optional |
| `AccumulatorOpSession::upload_intent_ops` | `&[IntentDelta]` raw GPU rows | **AS-8 residue** |
| `WorldAccumulatorRuntime::upload_*_ops` | encoded GPU op slices / session delegates | **AS-8 residue** — session-internal encode boundary |

### Raw struct residue (by design, inside pack or WGSL)

| Type | Role |
|---|---|
| `Structural*GpuRow` | Primitive POD rows inside `PackedUpload` |
| `AccumulatorOpGpu` / `AccumulatorInputGpu` | WGSL layout; encoded inside accumulator session |
| `ThresholdRegistration` / `ThresholdEvent` | GPU upload/readback byte mirror |
| `IntentDelta` | GPU affine upload |
| WGSL shader text | Final semantic-free residue Rust cannot seal |

## PackedUpload API

```rust
pub struct PackedUpload { /* private */ }

impl PackedUpload {
    pub fn new(frame, locations, links) -> Result<Self, StructuralUploadError>;
    pub fn frame(&self) -> StructuralFrameGpuRow;
    pub fn locations(&self) -> &[StructuralLocationGpuRow];
    pub fn links(&self) -> &[StructuralLinkGpuRow];
}

pub fn upload_structural_rows_to_gpu(
    device: &Device,
    queue: &Queue,
    upload: &PackedUpload,
) -> Result<(StructuralUploadGpuBuffers, StructuralUploadGpuReport), StructuralUploadError>;
```

Validation in `new`: non-empty locations, `frame.location_count == locations.len()`, `frame.link_count == links.len()`.

## Load-bearing compile_fail proofs

| Proof | Location | Catches |
|---|---|---|
| `packed_upload_fields_private_compile_fail` | `structural_upload.rs` doc-test | Field-literal `PackedUpload { .. }` |
| `upload_rejects_free_structural_rows_compile_fail` | `structural_upload.rs` doc-test | Free frame + slices at upload API |
| `upload_rejects_semantic_slot_column_arguments_compile_fail` | `structural_upload.rs` doc-test | `SlotIndex` / `ColumnIndex` instead of `PackedUpload` |

## Behavior / byte parity proofs

| Test | Result |
|---|---|
| `packed_upload_rejects_empty_location_rows` | PASS |
| `packed_upload_rejects_location_count_mismatch` | PASS |
| `packed_upload_rejects_link_count_mismatch` | PASS |
| `packed_upload_public_api_preserves_prior_bytes` | PASS |
| `structural_upload_allocates_gpu_buffers_from_packed_upload` | PASS |
| `structural_upload_reports_expected_byte_sizes_from_packed_upload` | PASS |
| `structural_upload_readback_matches_packed_upload_source` | PASS |
| `structural_validation` GPU tests (via `PackedUpload::try_from`) | PASS |

## Scope Ledger

| File | Why touched |
|---|---|
| `crates/simthing-gpu/src/structural_upload.rs` | `PackedUpload` + sealed upload API + proofs |
| `crates/simthing-gpu/src/structural_validation.rs` | Pack before upload |
| `crates/simthing-gpu/src/lib.rs` | Export `PackedUpload` |
| `crates/simthing-mapeditor/src/scenario_projection.rs` | Production residency proof path |
| `docs/tests/as_packed_upload_boundary_0_results.md` | Evidence |
| `docs/tests/current_evidence_index.md` | Index row |
| `docs/design_0_0_8_4_admission_substrate.md` | AS-8 → PROBATION |

**Not touched:** AS-F, 0.0.8.5, WGSL layouts, AccumulatorOp semantics, broad grep battery.

## Known residue / next

- Accumulator/threshold/intent session upload paths remain semantic-or-raw-row at session boundary (documented above).
- Optional future: `PackedThresholdUpload` / `PackedAccumulatorUpload` if DA requests uniform pack types across all GPU upload seams.
- **AS-F closeout** — opens after DA approves AS-8 PROBATION → DONE.

## Validation (targeted)

- `cargo fmt -p simthing-gpu -p simthing-core -p simthing-driver -- --check` — PASS
- `cargo check -p simthing-gpu`, `-p simthing-core`, `-p simthing-driver` — PASS
- `cargo test -p simthing-gpu --doc` — PASS (4 doc tests incl. 3 compile_fails)
- `cargo test -p simthing-gpu structural_upload --lib` — PASS (14 tests)
- `cargo test -p simthing-gpu packed_upload --lib` — PASS (7 tests)
- Prior AS gates — PASS (core doc, accumulator parity, slot tests, sim audits, channel newtypes)
- Scope grep: no 0.0.8.5 diff — PASS
