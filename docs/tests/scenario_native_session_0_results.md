# SCENARIO-NATIVE-SESSION-0 — Scenario-native loaded sessions and GPU-resident projection readiness

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added SCENARIO-NATIVE-SESSION-0 PROBATION row |
| `docs/tests/scenario_native_session_0_results.md` | PROBATION | This report |
| `docs/tests/scenario_saveload_ui_0_results.md` | PROBATION | UI behavior preserved; synthetic bridge note superseded here |
| `docs/tests/scenario_saveload_io_0_results.md` | PROBATION | Backend IO unchanged |
| `docs/design_0_0_8_3_studio_production.md` | PROBATION | Standing Studio production synthesis updated |

## Why this is not hygiene

This PR removes the architectural bridge that made loaded scenarios pretend to be MapGenerator `GenerationRunOutput`. The goal is to prevent future runtime-vertical-test and GPU-resident work from depending on a fake generation output path. Loaded sessions now enter through explicit `StudioSessionSource::LoadedScenario` with `SimThingScenarioSpec` as sole authority.

## Synthetic GenerationRunOutput bridge status

**Removed for loaded scenarios.** `StudioSession::from_loaded_scenario` sets `generated_output: None`. The synthetic `synthetic_generation_output_for_loaded_scenario` / `stub_generation_result_from_scenario` helpers were deleted from `scenario_io.rs`. Generated sessions retain `generated_output: Some(output)` for MapGenerator report evidence.

## Session source typing

`StudioSessionSource` distinguishes:

- `Generated { generation_profile }` — MapGenerator-produced sessions
- `LoadedScenario { scenario_path, profile_hint }` — `.simthing-scenario.json` authority loads

`StudioSession::is_loaded_scenario()` / `is_generated()` and `profile()` derive from source, not from the presence of synthetic output.

## Scenario-native summary

`StudioScenarioSummary` derives entirely from `SimThingScenarioSpec` (system/link counts, grid frame, STEAD validity, link validity, RF readiness, heatmap readiness). Generated sessions optionally attach `map_quality_status` from `GenerationReport`. Loaded sessions use `status_message()` text like `Loaded scenario: N systems, M links, STEAD valid` — never "Generated".

## Structural projection manifest

`StudioStructuralProjection` in `scenario_projection.rs` provides CPU-side dense index tables:

- `StudioLocationIndexRow` — dense_index, simthing_id_raw, system_id, row, col
- `StudioLinkIndexRow` — from_dense_index, to_dense_index

Derived deterministically from `structural_grid.placements` and `links`. No render coordinates, Bevy state, or pathfinding semantics. Invalid links or missing placements fail projection.

## GPU-resident readiness manifest

`StudioGpuResidencyReadiness` is a projection/cache (not authority) answering whether a scenario can be projected into dense structural tables for future GPU-resident RF/MF/runtime surfaces. Fields include grid dimensions, location/link counts, dense index readiness, RF/heatmap readiness, atlas_required, and optional deferred_reason. No GPU buffers, WGSL, or render metadata.

## RF/heatmap readiness integration

RF and heatmap readiness continue to derive from scenario authority via `rf_accumulator_readiness_from_simthing_spec` and `heatmap_readiness_from_simthing_spec`. Integrated into both `StudioScenarioSummary` and `StudioGpuResidencyReadiness`. Invalid STEAD → RF not ready, heatmap invalid. Valid oversized frame → `AtlasRequired`, not layout failure.

## Tests added

**`session.rs` (11):** `generated_session_source_is_generated`, `loaded_session_source_is_loaded_scenario`, `loaded_session_does_not_require_generation_output_as_authority`, `loaded_session_summary_derives_from_simthing_scenario_spec`, `generated_session_summary_derives_from_simthing_scenario_spec`, `loaded_scenario_status_does_not_say_generated`, `scenario_save_load_roundtrip_preserves_scenario_summary`, `scenario_save_load_roundtrip_preserves_structural_projection`, `scenario_save_load_roundtrip_preserves_gpu_residency_readiness`

**`scenario_projection.rs` (11):** `structural_projection_derives_from_scenario_authority`, `structural_projection_has_deterministic_dense_indices`, `structural_projection_uses_structural_coords_not_render_coords`, `structural_projection_rejects_missing_placement`, `structural_projection_rejects_invalid_link_endpoint`, `structural_projection_link_indices_use_dense_location_indices`, `gpu_residency_readiness_derives_from_scenario_authority`, `gpu_residency_readiness_rejects_invalid_stead`, `gpu_residency_readiness_reports_rf_readiness`, `gpu_residency_readiness_reports_heatmap_readiness`, `gpu_residency_readiness_reports_atlas_required_for_oversized_valid_grid`, `gpu_residency_readiness_contains_no_render_metadata`

**`app/scenario_io.rs` (3 new):** `loaded_session_authority_is_loaded_scenario_not_synthetic_output` (updated), `save_load_ui_preserves_loaded_session_source`, `load_failure_preserves_session_source`

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

- `crates/simthing-mapeditor/src/session.rs` — session source typing, scenario summary, optional generated_output
- `crates/simthing-mapeditor/src/scenario_io.rs` — removed synthetic bridge; `from_loaded_scenario` path
- `crates/simthing-mapeditor/src/scenario_projection.rs` — structural projection + GPU readiness manifest (new)
- `crates/simthing-mapeditor/src/hydration.rs` — `studio_projection_from_scenario_authority`
- `crates/simthing-mapeditor/src/lib.rs` — exports
- `crates/simthing-mapeditor/src/app/mod.rs` — status/profile from session helpers
- `crates/simthing-mapeditor/src/app/ui.rs` — loaded vs generated right-panel reporting
- `crates/simthing-mapeditor/src/app/scenario_io.rs` — source-preservation tests
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/scenario_native_session_0_results.md`

## Deleted/archived artifacts

- Synthetic `GenerationRunOutput` bridge code in `scenario_io.rs` (not archived — removed as dead path)
- No scratch scenario JSON, temp config files, or duplicate contradictory evidence left in working tree

## Deferred work

Runtime vertical-test loading, GPU kernels, RF execution arenas, heatmap textures, pathfinding, route/predecessor semantics, movement-order logic, semantic WGSL, live simulation, ClauseThing import wiring, native file dialogs.

## DA status

**PROBATION** — pending owner design-authority approval. Not promoted to CURRENT_EVIDENCE.