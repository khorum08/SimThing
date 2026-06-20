# GPU-STRUCTURAL-VALIDATION-WGSL-0 — GPU validation over resident structural packet

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## GPU adapter evidence state

**REAL_ADAPTER_OBSERVED** — structural upload readback and WGSL validation GPU tests executed on a real adapter in this environment (22 structural tests passed; no adapter skips).

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added GPU-STRUCTURAL-VALIDATION-WGSL-0 PROBATION row |
| `docs/tests/gpu_structural_validation_wgsl_0_results.md` | PROBATION | This report |
| `docs/tests/gpu_structural_residency_0_results.md` | PROBATION | Upload/readback prerequisite |
| `docs/design_0_0_8_3_studio_production.md` | PROBATION | Standing Studio production synthesis updated |

## Why this is not hygiene

This PR adds the first meaningful WGSL compute pass over resident structural packet buffers: parallel dense-link endpoint bounds and self-link validation with a compact GPU-written report. It proves a GPU invariant that upload/readback alone cannot — the device can read and validate structural consistency in parallel.

## Pre-edit orientation answers

| Question | Answer |
|---|---|
| GPU tests on real adapter? | **REAL_ADAPTER_OBSERVED** |
| WGSL invariant proved | Parallel validation: link dense endpoints `< location_count`, self-link detection |
| Buffers read | `frame_buffer`, `link_buffer` (storage) |
| Compact report written | `StructuralValidationReportGpu` (32 B): location_count, link_count, invalid_link_endpoint_count, self_link_count |
| No domain semantics | Neutral names only; no route/predecessor/pathfinding/MF/RF fields |
| Projection/cache only | Derives from scenario packet rows; not save authority |

## Readback error hardening

`readback_structural_upload_blocking` now returns `Result<StructuralUploadReadback, StructuralUploadError>` with `MapAsyncFailed` and `ReadbackFailed { buffer, reason }`. `readback_pod_blocking` shared for validation report readback.

## WGSL validation invariant

`structural_validation.wgsl` dispatches one thread per link, reads frame `location_count`, atomically increments `invalid_link_endpoint_count` when endpoints are out of range and `self_link_count` when `from_dense_index == to_dense_index`.

## WGSL forbidden-token scan

`structural_validation.wgsl` scanned for forbidden semantic terms (route, predecessor, pathfinding, movement_order, fleet, faction, owner, border, frontline, combat, economy, diplomacy).

## Validation report layout

| Field | Size |
|---|---|
| `StructuralValidationReportGpu` | 32 B, align 4 |

## Valid-packet GPU proof

Canonical two-cell scenario packet: `invalid_link_endpoint_count = 0`, `self_link_count = 0`.

## Bad-row GPU proof

Intentionally bad uploaded rows: endpoint `99` detected; self-link dense pair detected.

## Tests added

**simthing-gpu**: readback Result tests, report layout, WGSL token scan, 6 GPU validation tests (valid, zero invalid/self, bad endpoint, bad self-link, zero-link).

**simthing-mapeditor**: `prove_gpu_structural_validation_blocking` end-to-end from scenario packet.

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
- `crates/simthing-gpu/src/structural_validation.rs`
- `crates/simthing-gpu/src/shaders/structural_validation.wgsl`
- `crates/simthing-gpu/src/lib.rs`
- `crates/simthing-mapeditor/src/scenario_projection.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/gpu_structural_validation_wgsl_0_results.md`

## Deleted/archived artifacts

None.

## Deferred work

Runtime vertical-test loading, RF/Accumulator execution, Movement-Front execution, heatmap rendering, pathfinding, route/predecessor semantics, live Studio GPU context wiring, runtime UI integration.

## DA status

**PROBATION** — pending owner design-authority approval. GPU evidence: **REAL_ADAPTER_OBSERVED**.