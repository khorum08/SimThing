# AS-TICK-FABRIC-BOUNDARY-0A Results

## Status

PASS — ordinary GPU tick migrated behind `SimulationFabric`; hot tick accepts only fabric. Parent **AS-TICK-FABRIC-BOUNDARY-0** → **IN PROGRESS** (RF band + mapping dispatch remain 0B residue).

## PR / branch / merge

- Branch: `codex/as-tick-fabric-boundary-0a`
- PR: (pending push)
- Merge: (pending)

## What changed

- Added `crates/simthing-driver/src/simulation_fabric.rs` with private-field `SimulationFabric` and `run_simulation_fabric_tick`.
- `SimSession::run` / `record_to_path` invoke `run_hot_tick()` → fabric tick instead of direct `coord.tick(...)`.
- Fabric borrows only: `DispatchCoordinator`, `TransformPatcher`, feeder `rx`, resolved `DimensionRegistry`, `SlotAllocator`, `Pipelines`, `WorldGpuState`, `dt`.

## Hot-path boundary audit

| Surface | Before 0A | After 0A |
|---|---|---|
| Ordinary GPU tick entry | `self.coord.tick(&self.rx, …, &self.proto.registry, …)` inside `SimSession::run` | `run_simulation_fabric_tick(&mut SimulationFabric)` |
| Scenario / GameModeSpec in tick fn args | session loop body co-located with tick | not in fabric tick signature |
| BoundaryProtocol in tick fn args | registry/allocator borrowed from `proto` at call site | fabric holds resolved registry/allocator refs only; no `BoundaryProtocol` handle |
| SimRuntimeTree / SimThing in tick fn | not in coord.tick (already) | still absent; fabric has no tree field |
| RF band dispatch | session loop after tick | **0B residue** — still in session loop |
| Mapping hot-path dispatch | `run_mapping_step` in session loop | **0B residue** — still in session loop |
| Boundary execution | session loop on `boundary_reached` | **boundary-time residue** — unchanged outside fabric |

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `simulation_fabric_tick_signature_accepts_only_fabric` | hot tick fn type is `fn(&mut SimulationFabric<'_>) -> FabricTickOutcome` only |
| `simulation_fabric_hides_boundary_protocol_compile_fail` (`reach_boundary` doc) | `fabric.proto` field access |
| `simulation_fabric_hides_scenario_compile_fail` (`reach_scenario` doc) | `fabric.scenario` field access |
| `simulation_fabric_hides_root_compile_fail` (`reach_root` doc) | `fabric.runtime_tree` field access |
| `simulation_fabric_tick_behavior_preserved` | fabric tick matches direct `coord.tick` outcome on minimal GPU fixture |
| `as_sim_semantic_free_public_surface_audit` | AS-4 intact |
| `as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads` | AS-3 intact |

## Scope Ledger

| File / area | Why touched |
|---|---|
| `crates/simthing-driver/src/simulation_fabric.rs` | Rung type + hot tick + proofs |
| `crates/simthing-driver/src/session.rs` | `run_hot_tick` / fabric assembly; `run` + `record_to_path` migration |
| `crates/simthing-driver/src/lib.rs` | module + public exports |
| `docs/design_0_0_8_4_admission_substrate.md` | AS-TICK-FABRIC-BOUNDARY-0 → IN PROGRESS |
| `docs/tests/current_evidence_index.md` | 0A evidence row |

**Guard retired:** direct `coord.tick(...)` call pattern in session hot loop (replaced by fabric entry; no new grep battery).

**Not touched:** AS-5, AS-8, boundary execution internals, replay format, mapping/RF hot dispatch (0B).

## Known gaps / next

- **AS-7 0B:** migrate `run_resource_flow_bands` and `run_mapping_step` behind fabric or a dedicated hot-path extension.
- **AS-7 0C+:** boundary-time planning remains outside fabric by design.
- `submit_tick_patches()` still runs in session loop before fabric tick (feeder enqueue — not inside fabric yet).
