# TERRAN-PIRATE-MAPPING-FIRST-SLICE-0 — structural N4 Gu-Yang/PALMA GPU proof

**Status:** PROBATION  
**Date:** 2026-06-18  
**Base:** `master` after PR #767 / DRIVER-TEST-HARNESS-GREEN-0  
**Branch:** `terran-pirate-mapping-first-slice-0`

## Orientation answers

| Question | Answer |
|---|---|
| Canonical scenario authority? | `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` via `deserialize_scenario_authority` |
| Structural grid frame / placements? | Frame `8×8`, `occupied_cells=4`; placements hub `(0,0)` sys1, corridor `(1,0)` sys2, choke `(2,0)` sys4, branch `(1,1)` sys3 |
| Bounded theater? | Full authoritative `StructuralGridFrame` (`8×8`); first-slice seeds only on occupied structural cells |
| Grid N4 adjacency? | Three undirected N4 edges among occupied cells: hub↔corridor, corridor↔choke, corridor↔branch |
| N4 vs hyperlane separation? | N4 from `(col,row)` only; hyperlane links feed separate `AccumulatorOp` Sum-over-INPUT_LIST; hub↔branch connected via links but **not** N4-adjacent |
| Gu-Yang operator surface? | `RegionFieldSpec::SaturatingFlux` → `compile_region_field_preview` → `compiled_stencil_to_gpu_config` → `StructuredFieldStencilOp` |
| PALMA operator surfaces? | `WImpedanceComposeSpec` → `compile_w_impedance_compose_preview` → `WImpedanceComposeOp` + `composed_w_min_plus_stencil_config` → `MinPlusStencilOp` |
| GPU-resident execution? | **Yes** — SaturatingFlux ping-pong + W compose dispatch + min-plus ping-pong when adapter available |
| CPU oracle? | `cpu_stencil_step` horizon + `cpu_min_plus_d_from_w` over composed W |
| Outputs projection/cache only? | **Yes** — field values and D readbacks are test projection; scenario JSON not mutated |
| Scenario authority writeback? | **No** |
| New primitive/shader/route/border? | **No** |
| Bounded-theater / atlas deferral? | First-slice uses existing `8×8` frame within standard admission; atlas deferral not required for this skeleton; oversize atlas behavior remains deferred |

## Why this is not mere feature work

This is the first step where the canonical Terran Pirate scenario’s structural grid feeds STEAD/Movement-Front horizon operators separately from hyperlane link gather. PR #767 restored package test hygiene; this PR opens the Gu-Yang/PALMA runway without conflating adjacency mechanisms.

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated |
| `docs/tests/driver_test_harness_green_0_results.md` | PROBATION (PR #767) | Retained |
| `docs/tests/terran_pirate_mapping_first_slice_0_results.md` | PROBATION (this file) | Created |
| `docs/0.8.3 Simthing Studio Production.md` | Living synthesis | Updated § TERRAN-PIRATE-MAPPING-FIRST-SLICE-0 |
| `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` | Canonical authority | Untouched |

## Structural theater derivation

```text
validate_stead_mapping_consistency
  -> structural_grid.frame (8×8, occupied_cells=4)
  -> placements (col,row) → occupied set
  -> N4 edges: orthogonal neighbors among occupied cells only
```

## Hyperlane vs grid N4 separation proof

- **Hyperlane:** `1↔2`, `2↔3`, `2↔4` → `compile_structural_link_neighbor_sum_plan` corridor gathers slots `[0,2,3]`.
- **Grid N4:** hub↔corridor, corridor↔choke, corridor↔branch; hub↔branch **not** N4-adjacent.
- Clearing `scenario.links` leaves N4 edges unchanged (explicit source-separation assertion).

## Gu-Yang / SaturatingFlux first-slice proof

- Deterministic seeds on hub/corridor/branch occupied cells.
- 4-hop SaturatingFlux with choke column readout.
- GPU `StructuredFieldStencilOp` matches CPU `cpu_stencil_step` oracle (`<1e-4` tolerance).
- Choke values finite and bounded in `[0,1]` at occupied cells.
- REAL_ADAPTER_OBSERVED when GPU available.

## PALMA / W-impedance / min-plus first-slice proof

- Gu-Yang choke column feeds W-impedance compose inputs on structural theater.
- `WImpedanceComposeOp` GPU dispatch + `MinPlusStencilOp` GPU relaxation.
- CPU oracle `cpu_min_plus_d_from_w`; GPU D field parity `<1e-4`.
- D is a field; destination pinned to zero; no route/predecessor/came_from/path objects.
- REAL_ADAPTER_OBSERVED when GPU available.

## Studio non-ownership

No changes to `simthing-mapeditor` app/load/projection runtime paths. Proof lives in `simthing-driver` integration test only.

## simthing-sim seam preservation

`simthing-sim/Cargo.toml` has no upward dev-dependencies. Terran Pirate link-gather resident tick proof unchanged in `terran_pirate_skeleton_resident_tick.rs`.

## Gu-Yang / PALMA contract carried forward (STEAD §10)

Horizon surfaces remain bounded-theater first-slice; dense-global rejected; atlas deferral for oversize execution profiles deferred to future runs.

## Big-endian / portable byte-proof backlog (deferred)

- Explicit little-endian byte helpers
- Cross-platform byte-order evidence
- Replacing host-endian bytemuck casts in canonical artifact byte proofs

## Forbidden-token scan

`w_impedance_compose_bridge.rs` scanned — no pathfinding/predecessor/border/frontline/cpu_planner tokens introduced.

## Tests added/changed/deleted

| Action | File |
|---|---|
| Added | `crates/simthing-driver/tests/terran_pirate_mapping_first_slice.rs` (4 tests) |

## Validation commands

Run with `CARGO_BUILD_JOBS=1` on Windows.

| Command | Status | Notes |
|---|---|---|
| `cargo fmt --all -- --check` | PASS | |
| `cargo check -p simthing-driver` | PASS | |
| `cargo test -p simthing-driver terran_pirate` | PASS | Includes new + existing Terran Pirate tests |
| `cargo test -p simthing-driver --test terran_pirate_skeleton_compile` | PASS | 5/5 |
| `cargo test -p simthing-driver --test terran_pirate_skeleton_resident_tick` | PASS | |
| `cargo test -p simthing-driver --test terran_pirate_mapping_first_slice` | PASS | 4/4 |
| `cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver bh3_field_operator` | PASS | |
| `cargo test -p simthing-driver --test palma_path_5_session_property` | PASS | |
| `cargo check -p simthing-sim` | PASS | |
| `cargo test -p simthing-sim --test forked_four_slot_input_list_tick` | PASS | |
| `cargo test -p simthing-sim --test accumulator_plan_tick_convergence` | PASS | |
| `cargo check -p simthing-spec` | PASS | |
| `cargo test -p simthing-spec` | PASS | |
| `cargo test -p simthing-spec --test e10_resource_flow_admission e10_does_not_import_arena_registry_into_simthing_sim` | PASS | |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS | |
| `cargo test -p simthing-mapeditor --test accumulator_convergence_1_guards` | PASS | |
| `cargo test -p simthing-gpu --test debug_readback_scope` | PASS | |
| `cargo test -p simthing-clausething --test stead_spatial_contract_guards` | PASS | |
| `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy` | PASS | |
| `cargo test -p simthing-clausething --test mapgen_rf_stead_binding` | PASS | |
| `cargo test -p simthing-clausething --test mapgen_movement_front` | PASS | |
| `git diff --check` | PASS | |

## Windows / resource-limit notes

`CARGO_BUILD_JOBS=1` used for driver package filtered tests. No linker PDB failures observed.

## Files changed

- `crates/simthing-driver/tests/terran_pirate_mapping_first_slice.rs`
- `docs/tests/terran_pirate_mapping_first_slice_0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/0.8.3 Simthing Studio Production.md`

## Deferred work

- Studio visualization of mapping outputs
- Atlas deferral proof on oversize structural layouts
- Big-endian/portable byte-proof hardening
- sim-owned generic compiled operator-plan interface for mapping (if needed for production tick)

## DA status

**PROBATION** — no DA/owner sign-off claimed.