# AS-TICK-FABRIC-BOUNDARY-0B Results

## Status

PASS — RF band dispatch and mapping hot-path dispatch migrated behind `SimulationFabric::run_simulation_fabric_hot_step`. Parent **AS-TICK-FABRIC-BOUNDARY-0** remains **IN PROGRESS** (`submit_tick_patches` pre-tick enqueue residue).

## PR / branch / merge

- Branch: `codex/as-tick-fabric-boundary-0b`
- PR: #963
- Merge: `e4f7e26902`

## What changed

- Extended `SimulationFabric` with `run_simulation_fabric_hot_step`, `run_resource_flow_bands_if_active`, and `run_mapping_hot_dispatch`.
- Introduced `MappingHotPathState` (hot GPU dispatch only) and `MappingBoundaryState` (commitment effects + journal watermark) split from `SessionMappingState`.
- `SimSession::run` / `record_to_path` call `run_hot_step()` instead of direct RF/mapping dispatch after tick.
- Summary counters and mapping commitment journaling preserved via returned `FabricHotStepOutcome` / `FabricMappingHotReport`.

## Hot-path boundary audit

| Surface | Before 0B | After 0B |
|---|---|---|
| RF band dispatch | `self.state.run_resource_flow_bands(...)` in session loop | `run_resource_flow_bands_if_active(&mut SimulationFabric, pipeline_flag)` |
| Mapping scatter/seed/tick | `run_mapping_step` on full `SessionMappingState` | `run_mapping_hot_dispatch(&WorldGpuState, &mut MappingHotPathState)` via hot step |
| Commitment effect submission | session boundary-time | **unchanged outside fabric** |
| Boundary execution | session loop | **unchanged outside fabric** |
| `submit_tick_patches` | session loop before hot step | **residue** — pre-tick feeder enqueue not in fabric |

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `simulation_fabric_hot_step_signature_accepts_only_fabric` | hot-step fn accepts only fabric + hot mapping state |
| `simulation_fabric_rf_band_dispatch_behavior_preserved` | RF dispatch boolean matches session gate (`pipeline && active`) |
| `simulation_fabric_mapping_hot_dispatch_behavior_preserved` | mapping hot dispatch via fabric matches direct path |
| `simulation_fabric_hides_boundary_protocol_compile_fail` | `fabric.proto` field access |
| `simulation_fabric_hides_scenario_compile_fail` | `fabric.scenario` field access |
| `simulation_fabric_hides_root_compile_fail` | `fabric.runtime_tree` field access |
| `mapping_hot_path_hides_boundary_effect_compile_fail` | `MappingHotPathState.effect` access |
| `as_sim_semantic_free_public_surface_audit` | AS-4 intact |
| `as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads` | AS-3 intact |

## Scope Ledger

| File / area | Why touched |
|---|---|
| `crates/simthing-driver/src/simulation_fabric.rs` | hot step, RF/mapping dispatch, `MappingHotPathState`, proofs |
| `crates/simthing-driver/src/session.rs` | mapping state split, `run_hot_step`, loop migration |
| `crates/simthing-driver/src/lib.rs` | public exports |
| `docs/design_0_0_8_4_admission_substrate.md` | AS-7 residue note update |
| `docs/tests/current_evidence_index.md` | 0B evidence row |

**Guard retired:** direct `run_resource_flow_bands` / `run_mapping_step` calls in session hot loop.

**Not touched:** AS-5, AS-8, boundary execution, commitment-effect submission internals, replay format.

## Known gaps / next

- **AS-7 residue:** `submit_tick_patches()` still runs in session loop before fabric hot step.
- Parent AS-TICK-FABRIC-BOUNDARY-0 stays IN PROGRESS until pre-tick enqueue residue is sealed or classified.
