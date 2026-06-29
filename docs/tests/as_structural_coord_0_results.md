# AS-STRUCTURAL-COORD-0 Results

## Status

PASS — structural grid coordinates promoted to `StructuralCoord` with private fields; Movement-Front / N4 theater compile path migrated. Parent **AS-STRUCTURAL-COORD-0** moved to **PROBATION**.

## PR / branch / merge

- Branch: `codex/as-structural-coord-0`
- PR: (pending push)
- Merge: (pending)

## What changed

- Added `simthing_core::StructuralCoord` (private `col`/`row`, integer `new`, accessors, explicit `from_render_floor`).
- Added `simthing_core::RenderCoord` with `to_structural_cell()` as the named render→structural boundary.
- Removed driver-local `StructuralGridCoordinate { pub col, row }`; N4 theater compile + atlas partition + mapping plan compile now use `StructuralCoord`.
- `CompiledStructuralN4Theater::cell_slot`, `coord_for_system`, `has_n4_edge`, and `MappingPlanCompileSpec::min_plus_dest` accept only `StructuralCoord`.

## Structural/render coordinate audit

| Surface | Before 0 | After 0 |
|---|---|---|
| Structural N4 theater coords | `StructuralGridCoordinate { pub col, row }` in driver | `StructuralCoord` from `simthing-core` (private fields) |
| Float → structural construction | implicit via public struct literals / bare u32 fields | uncompilable; only `StructuralCoord::new(u32,u32)` or `RenderCoord::to_structural_cell()` |
| Render floats into `cell_slot` | would type-check if mis-wired | uncompilable (`compile_fail` on `RenderCoord` → `cell_slot`) |
| Scenario placement integers | unchanged (`u32` on `SimThingStructuralGridPlacement`) | admitted into theater via `StructuralCoord::new(placement.col, placement.row)` |
| Render/UI float positions | unchanged in mapeditor/mapgenerator | not migrated (out of scope) |

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `structural_coord_rejects_render_floats_compile_fail` (`structural_coord` module doc) | `StructuralCoord::new(1.0_f32, 2.0_f32)` |
| `structural_coord_fields_are_private_compile_fail` (`structural_coord` module doc) | `StructuralCoord { col: 1, row: 2 }` |
| `structural_path_rejects_render_coord_compile_fail` (`structural_n4_theater_compile` module doc) | `theater.cell_slot(RenderCoord::new(...))` |
| `structural_coord_integer_roundtrip_preserved` | integer `new` / accessors / tuples stable |
| `migrated_structural_path_behavior_preserved` | N4 theater compile parity after newtyping |

## Scope Ledger

| File / area | Why touched |
|---|---|
| `crates/simthing-core/src/structural_coord.rs` | Rung type + boundary + proofs |
| `crates/simthing-core/src/lib.rs` | Export `StructuralCoord`, `RenderCoord` |
| `crates/simthing-driver/src/structural_n4_theater_compile.rs` | Primary migrated structural path |
| `crates/simthing-driver/src/structural_n4_atlas_partition.rs` | Atlas partition uses `StructuralCoord` |
| `crates/simthing-driver/src/mapping_plan_compile.rs` | `min_plus_dest: StructuralCoord` |
| `crates/simthing-driver/src/lib.rs` | Re-export `StructuralCoord` |
| `crates/simthing-driver/tests/*mapping*`, `*structural_n4*` | Compiler-forced test updates |
| `docs/design_0_0_8_4_admission_substrate.md` | AS-STRUCTURAL-COORD-0 → PROBATION |
| `docs/tests/current_evidence_index.md` | AS-STRUCTURAL-COORD-0 row |

**Guard retired:** driver `StructuralGridCoordinate` public field literals (implicit “any u32 pair is a structural coord” seam). Replaced by private-field `StructuralCoord` + `compile_fail` proofs — no new grep battery.

**Not touched:** AS-5 index newtypes, AS-7 SimulationFabric, AS-8 PackedUpload, mapgenerator `LatticeCoord`, mapeditor render projection, 0.0.8.5 Terran-Pirate.

## Known gaps / next

- `SimThingStructuralGridPlacement.col/row` remain plain `u32` at spec layer — future rung may wrap admission from scenario authority.
- Mapgenerator `LatticeCoord`, mapeditor render anchors, and clausething lowerer paths still use local integer/float types — migrate when those surfaces are load-bearing for STEAD compile.
- `RenderCoord` / `from_render_floor` exist but are not yet wired into mapeditor UI projection (explicit boundary documented, not yet exercised in production UI path).
