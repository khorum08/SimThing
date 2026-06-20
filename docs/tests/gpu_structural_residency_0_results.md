# GPU-STRUCTURAL-RESIDENCY-0 — Structural packet GPU buffer residency

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added GPU-STRUCTURAL-RESIDENCY-0 PROBATION row |
| `docs/tests/gpu_structural_residency_0_results.md` | PROBATION | This report |
| `docs/tests/gpu_structural_upload_packet_0_results.md` | PROBATION | CPU packet prerequisite |
| `docs/design_0_0_8_3_studio_production.md` | PROBATION | Standing Studio production synthesis updated |

## Why this is not hygiene

This PR proves the first true GPU-resident structural bridge: scenario-derived upload packet rows become `COPY_DST | COPY_SRC` buffers with byte-stable readback. It table-sets residency for future RF/Movement-Front/runtime vertical-test surfaces without introducing simulation kernels.

## Pre-edit orientation answers

| Question | Answer |
|---|---|
| Where should structural packet GPU residency live? | `simthing-gpu/src/structural_upload.rs` for buffers/upload/readback; mapeditor owns CPU packet/projection |
| Reusable buffer helpers? | Yes — existing `GpuContext`, staging-buffer readback pattern from BH tests |
| Crate boundary | mapeditor: CPU packet; simthing-gpu: GPU buffers (no mapeditor dependency) |
| Byte layout uploaded | `StructuralFrameGpuRow` (32 B), `StructuralLocationGpuRow` (32 B), `StructuralLinkGpuRow` (16 B) |
| Proof without semantic kernels | Buffer upload + staging readback; `readback_matches_source` compares exact bytes |

## GPU buffer residency boundary

```text
SimThingScenarioSpec
  -> StudioStructuralProjection
  -> StudioGpuStructuralUploadPacket (mapeditor)
  -> StructuralUploadRows (mapeditor bridge)
  -> StructuralUploadGpuBuffers (simthing-gpu)
  -> readback proof
```

GPU buffers are projection/cache. Save/load remains `SimThingScenarioSpec`. Studio config remains presentation-only.

## Crate-boundary decision

- `simthing-mapeditor`: `to_structural_gpu_rows()`, `prove_gpu_buffer_residency_blocking()`, readiness fields
- `simthing-gpu`: `StructuralFrameGpuRow`, upload, readback — generic names, no Studio coupling

## Row layout proof

| Row | Size | Alignment |
|---|---|---|
| `StructuralFrameGpuRow` | 32 B | 4 |
| `StructuralLocationGpuRow` | 32 B | 4 |
| `StructuralLinkGpuRow` | 16 B | 4 |

`bytemuck::Pod` + `Zeroable` on GPU rows; `size_of`/`align_of` unit tests.

## Upload function summary

`upload_structural_rows_to_gpu(device, queue, frame, locations, links)` allocates three buffers, uploads row bytes, reports sizes/counts, rejects empty locations and count/byte overflow.

## Readback proof

`readback_structural_upload_blocking()` copies GPU buffers to staging, maps, and returns exact bytes. `readback_matches_source()` verifies frame/location/link bytes match source rows.

## WGSL usage

**Not used.** Buffer upload + readback proves byte-stable residency without semantic kernels.

## Tests added

**simthing-gpu** (11): layout stability, upload allocation, byte sizes, readback frame/location/link, empty-location rejection, count overflow.

**simthing-mapeditor** (7): packet→GPU row conversion, canonical link order, no render/route metadata, readiness defers without device, GPU proof upload+readback, empty packet rejection.

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-gpu
cargo test -p simthing-gpu
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo check -p simthing-spec
cargo test -p simthing-spec
cargo check -p simthing-core
cargo test -p simthing-core
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_rf_stead_binding
cargo test -p simthing-clausething --test mapgen_movement_front
git diff --check
```

## Files changed

- `crates/simthing-gpu/src/structural_upload.rs`
- `crates/simthing-gpu/src/lib.rs`
- `crates/simthing-mapeditor/src/scenario_projection.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/Cargo.toml`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/gpu_structural_residency_0_results.md`

## Deleted/archived artifacts

None.

## Deferred work

Runtime vertical-test loading, RF/Accumulator execution, Movement-Front execution, heatmap rendering, pathfinding, route/predecessor semantics, live sim loop integration, WGSL structural validation shaders, Studio runtime GPU context wiring.

## DA status

**PROBATION** — pending owner design-authority approval.