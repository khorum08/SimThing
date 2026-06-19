# STRUCTURAL-N4-THEATER-COMPILE-0 — driver structural theater admission surface

**Status:** PROBATION  
**Date:** 2026-06-18  
**Base:** `master` after PR #768 / TERRAN-PIRATE-MAPPING-FIRST-SLICE-0  
**Branch:** `structural-n4-theater-compile-0`

## Orientation answers

| Question | Answer |
|---|---|
| Test-local derivation promoted? | `derive_structural_theater` / `StructuralCell` / `StructuralTheater` from `terran_pirate_mapping_first_slice.rs` |
| New driver API? | `compile_structural_n4_theater` → `StructuralTheaterAdmission` with `CompiledStructuralN4Theater`, `StructuralGridCoordinate`, `AtlasDeferralReason` |
| N4 from placements only? | **Yes** — `structural_grid.placements (col,row)`; no links/render/emission-order/row-major |
| Hyperlane separation? | Link gather remains `compile_structural_link_neighbor_sum_plan`; theater compile ignores `scenario.links` |
| Execution theater cap? | `REGION_FIELD_STANDARD_MAX_GRID` (10) and `REGION_FIELD_MAX_CELL_COUNT` (1024) |
| Oversize behavior? | `StructuralTheaterAdmission::AtlasDeferred` with `FrameExceedsStandardMaxGrid`; frame/placements not shrunk |
| Atlas deferral? | Typed result; no dense-global fallback; scenario authority unchanged |
| Gu-Yang/PALMA surfaces? | Unchanged operator path through admitted theater |
| simthing-sim seam? | **Clean** — no upward dev-dependencies |
| New primitive/shader/route? | **No** |

## Why this is not mere refactor

PR #768 proved mapping first-slice GPU parity with test-local N4 derivation. Without a driver-owned compile surface, Gu-Yang/PALMA work would copy-paste theater logic into tests or Studio paths. This PR promotes derivation into `simthing-driver` as the lawful scenario→generic-operation compile seam.

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated |
| `docs/tests/terran_pirate_mapping_first_slice_0_results.md` | PROBATION (PR #768) | Retained |
| `docs/tests/structural_n4_theater_compile_0_results.md` | PROBATION (this file) | Created |
| `docs/0.8.3 Simthing Studio Production.md` | Living synthesis | Updated |
| `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` | Canonical authority | Untouched |

## New driver compile/admission API

**Module:** `crates/simthing-driver/src/structural_n4_theater_compile.rs`

```text
compile_structural_n4_theater(scenario, profile)
  -> validate_stead_mapping_consistency
  -> read structural_grid.frame + placements
  -> derive occupied cells + N4 edges
  -> evaluate REGION_FIELD_STANDARD_MAX_GRID admission
  -> Admit(CompiledStructuralN4Theater) | AtlasDeferred { reason }
```

## Small-theater admission proof

Terran Pirate canonical scenario admits: 8×8 frame, 4 occupied cells, 3 N4 edges. Gu-Yang/PALMA GPU parity tests route through `compile_structural_n4_theater`.

## Oversize atlas-deferral proof

Synthetic 11×11 scenario (1 placement) returns `AtlasDeferred` with `FrameExceedsStandardMaxGrid { width: 11, height: 11, max_grid: 10 }`. Original `structural_grid.frame` and placements unchanged.

## N4 vs link separation

Preserved from PR #768 via refactored `terran_pirate_mapping_first_slice` tests.

## Gu-Yang / PALMA parity preservation

4/4 mapping first-slice tests PASS through driver compile surface. REAL_ADAPTER_OBSERVED when GPU available.

## Forbidden-token scan

`structural_n4_theater_compile.rs` and `w_impedance_compose_bridge.rs` scanned — no pathfinding/predecessor/route/border/cpu_planner tokens.

## Validation commands

`CARGO_BUILD_JOBS=1` on Windows.

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver terran_pirate` | PASS |
| `cargo test -p simthing-driver --test terran_pirate_mapping_first_slice` | PASS (4/4) |
| `cargo test -p simthing-driver --test structural_n4_theater_compile` | PASS (3/3) |
| `cargo test -p simthing-driver --test terran_pirate_skeleton_compile` | PASS |
| `cargo test -p simthing-driver --test terran_pirate_skeleton_resident_tick` | PASS |
| `cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver bh3_field_operator` | PASS |
| `cargo test -p simthing-driver --test palma_path_5_session_property` | PASS |
| `cargo check -p simthing-sim` | PASS |
| `cargo test -p simthing-sim --test forked_four_slot_input_list_tick` | PASS |
| `cargo test -p simthing-sim --test accumulator_plan_tick_convergence` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test e10_resource_flow_admission e10_does_not_import_arena_registry_into_simthing_sim` | PASS |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS |
| `cargo test -p simthing-mapeditor --test accumulator_convergence_1_guards` | PASS |
| `cargo test -p simthing-gpu --test debug_readback_scope` | PASS |
| `cargo test -p simthing-clausething --test stead_spatial_contract_guards` | PASS |
| `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy` | PASS |
| `cargo test -p simthing-clausething --test mapgen_rf_stead_binding` | PASS |
| `cargo test -p simthing-clausething --test mapgen_movement_front` | PASS |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-driver/src/structural_n4_theater_compile.rs` (new)
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/terran_pirate_mapping_first_slice.rs`
- `crates/simthing-driver/tests/structural_n4_theater_compile.rs` (new)
- `docs/tests/structural_n4_theater_compile_0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/0.8.3 Simthing Studio Production.md`

## Deferred work

- Studio visualization of mapping outputs
- Multi-theater atlas scheduling runtime (deferral result only in this PR)
- Big-endian/portable byte-proof hardening
- sim-owned generic compiled mapping-plan tick seam

## DA status

**PROBATION** — no DA/owner sign-off claimed.