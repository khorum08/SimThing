# GPU-STRUCTURAL-UPLOAD-PACKET-0 — Structural upload packet for GPU-resident horizon

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added GPU-STRUCTURAL-UPLOAD-PACKET-0 PROBATION row |
| `docs/tests/gpu_structural_upload_packet_0_results.md` | PROBATION | This report |
| `docs/tests/studio_panel_gap_and_scenario_link_canon_0_results.md` | PROBATION | Link canonicalization prerequisite |
| `docs/tests/scenario_native_session_0_results.md` | PROBATION | Session/projection baseline |
| `docs/0.8.3 Simthing Studio Production.md` | PROBATION | Standing Studio production synthesis updated |

## Why this is not hygiene

This PR defines the first formal GPU-resident bridge artifact: a deterministic POD-style structural upload packet derived from `SimThingScenarioSpec` through `StudioStructuralProjection`. It table-sets the data contract for future runtime vertical-test GPU upload without adding kernels or simulation execution.

## Pre-edit orientation answers

| Question | Answer |
|---|---|
| Dense location row shape | `dense_index`, `simthing_id_raw`, `system_id`, `row`, `col` (structural authority fields) |
| Dense link row shape | `from_dense_index`, `to_dense_index` (canonical min/max dense pair) |
| Projection rejects invalid links? | Yes — self, direct duplicate, reversed duplicate, unknown endpoints |
| Structural authority | `SimThingScenarioSpec.structural_grid`, placements, canonical links |
| Projection-only | Upload packet rows, GPU readiness, render anchors, Bevy entities |
| Future GPU buffer rows | `StudioGpuStructuralFrameRow` (32 B), `StudioGpuLocationRow` (32 B), `StudioGpuLinkRow` (16 B) |

## Upload packet type summary

`StudioGpuStructuralUploadPacket { frame, locations, links }` built by `build_gpu_structural_upload_packet_from_scenario()`.

## Row layout summary

| Row | Size | Alignment | Fields |
|---|---|---|---|
| `StudioGpuStructuralFrameRow` | 32 B | 4 | width, height, occupied_cells, location_count, link_count + 3 reserved u32 |
| `StudioGpuLocationRow` | 32 B | 4 | dense_index, simthing_id_raw, system_id, row, col + 3 reserved u32 |
| `StudioGpuLinkRow` | 16 B | 4 | from_dense_index, to_dense_index + 2 reserved u32 |

POD proof via `repr(C)`, `size_of`, `align_of`, and deterministic `pod_row_bytes` slice tests (no bytemuck dependency added to mapeditor).

## Scenario authority derivation proof

Packet builds from `build_structural_projection(scenario)` only — never from `StudioGalaxyViewModel` or render anchors.

## Canonical link dependency

Invalid self/duplicate/reversed/unknown links fail packet build through projection validation.

## GPU-resident horizon compatibility

Future path: scenario authority → structural projection → upload packet → GPU buffers → Studio presentation. No kernels or device upload in this PR.

## Tests added

20 new tests in `scenario_projection.rs` covering derivation, projection use, frame/location/link preservation, deterministic ordering, rejection cases, no render/Bevy metadata, count overflow, readiness integration, atlas-required distinction.

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-spec
cargo test -p simthing-spec
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
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

- `crates/simthing-mapeditor/src/scenario_projection.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/gpu_structural_upload_packet_0_results.md`

## Deleted/archived artifacts

None.

## Deferred work

GPU device upload, WGSL kernels, runtime vertical-test loading, RF/Accumulator execution, heatmap rendering, pathfinding, route/predecessor semantics.

## DA status

**PROBATION** — pending owner design-authority approval.