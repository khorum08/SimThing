# AS-TICK-FABRIC-BOUNDARY-0C Results

## Status

PASS — pre-tick feeder enqueue sealed in `run_simulation_fabric_hot_cycle`; all hot-path GPU dispatch behind fabric. Parent **AS-TICK-FABRIC-BOUNDARY-0** → **PROBATION**.

## PR / branch / merge

- Branch: `codex/as-tick-fabric-boundary-0c`
- PR: (pending push)
- Merge: (pending)

## What changed

- Added `run_simulation_fabric_pre_tick_enqueue` and `run_simulation_fabric_hot_cycle` (pre-tick enqueue → hot step).
- Extended `SimulationFabric` / `HotFabricParts` with resolved `FeederSender` (`tx`).
- `SimSession::run` / `record_to_path` call `run_hot_cycle()` only; removed direct `submit_tick_patches()`.
- Session resolves `scenario.tick_patches` at the loop edge into `FabricHotCycleParams::tick_patches` (resolved `PatchTransform` slice, not `Scenario`).
- `submit_tick_patches_ms` preserved from `FabricHotCycleOutcome::pre_tick_enqueue_ms`.

## Pre-tick enqueue residue audit

| Question | Resolution |
|---|---|
| Is `submit_tick_patches` hot GPU dispatch? | **No** — it enqueues resolved `PatchTransform` work on the feeder channel; GPU dispatch happens in `coord.tick` inside the fabric hot step. |
| Moved or classified? | **Moved** into `run_simulation_fabric_pre_tick_enqueue` / `run_simulation_fabric_hot_cycle`. |
| State touched | `FeederSender` + `&[PatchTransform]` (resolved at session edge from `scenario.tick_patches`). |
| Semantic/boundary/tree access in fabric? | **None** — fabric receives only resolved patch slice + feeder endpoints. |
| Direct GPU hot dispatch outside fabric? | **None** — session loop has no direct `coord.tick`, RF bands, mapping scatter/seed/tick. |

## Hot-path boundary audit

| Surface | After 0C |
|---|---|
| Pre-tick patch enqueue | `run_simulation_fabric_pre_tick_enqueue` via `run_simulation_fabric_hot_cycle` |
| Ordinary GPU tick | `run_simulation_fabric_tick` via hot cycle |
| RF band dispatch | `run_resource_flow_bands_if_active` via hot cycle |
| Mapping hot dispatch | `run_mapping_hot_dispatch` via hot cycle |
| Session hot entry | `SimSession::run_hot_cycle()` → single fabric hot-cycle call |
| Commitment-effect submission | **boundary-time residue** — unchanged |
| Boundary execution | **boundary-time residue** — unchanged |

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `simulation_fabric_hot_cycle_signature_accepts_only_fabric` | hot-cycle fn accepts only fabric + resolved params, not Scenario/BoundaryProtocol/etc. |
| `simulation_fabric_pre_tick_enqueue_behavior_preserved` | fabric pre-tick enqueue count matches direct feeder send |
| `simulation_fabric_hot_step_signature_accepts_only_fabric` | 0B hot-step boundary intact |
| `simulation_fabric_rf_band_dispatch_behavior_preserved` | 0B RF behavior intact |
| `simulation_fabric_mapping_hot_dispatch_behavior_preserved` | 0B mapping behavior intact |
| `simulation_fabric_hides_boundary_protocol_compile_fail` | `fabric.proto` inaccessible |
| `simulation_fabric_hides_scenario_compile_fail` | `fabric.scenario` inaccessible |
| `simulation_fabric_hides_root_compile_fail` | `fabric.runtime_tree` inaccessible |
| `mapping_hot_path_hides_boundary_effect_compile_fail` | hot mapping cannot reach boundary effects |
| `as_sim_semantic_free_public_surface_audit` | AS-4 intact |
| `as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads` | AS-3 intact |

## Scope Ledger

| File / area | Why touched |
|---|---|
| `crates/simthing-driver/src/simulation_fabric.rs` | hot cycle, pre-tick enqueue, `tx` in fabric, proofs |
| `crates/simthing-driver/src/session.rs` | `run_hot_cycle`, loop migration, removed `submit_tick_patches` |
| `crates/simthing-driver/src/lib.rs` | public exports |
| `docs/design_0_0_8_4_admission_substrate.md` | AS-TICK-FABRIC-BOUNDARY-0 → PROBATION |
| `docs/tests/current_evidence_index.md` | 0C evidence row |

**Guard retired:** direct `submit_tick_patches()` in session hot loop.

**Not touched:** AS-5, AS-8, boundary execution, commitment-effect submission, replay format.

## Known gaps / next

- **Hold point:** After this rung, hold ladder progression. Opus remediation queue for earlier PRs/rungs is next — do not start AS-5, AS-8, or AS-F.
