# PALMA-PATH-6 session/RegionField min-plus band results

Status: **PARTIAL / TEST-PROFILE PASS** (2026-06-11)

Default `SimSession` tick does **not** schedule this band — opt-in profile only (ledgered below). The band is `TraversalFieldBandSession` in `min_plus_traversal_field`; tests use explicit `dispatch_*` methods (no public `tick()`).

## Deliverable

- `crates/simthing-driver/src/min_plus_traversal_field.rs` — opt-in `TraversalFieldBandSession` (FieldScheduler + GPU min-plus utility)
- `crates/simthing-driver/tests/palma_path_6_session_regionfield.rs` — 8 proof tests on PATH-5 property tree

Guide: [`../design_0_0_8_1_palma_pathfinding_integration_guide.md`](../design_0_0_8_1_palma_pathfinding_integration_guide.md)

## Schedule type

**Test-profile / explicit opt-in** — `TRAVERSAL_FIELD_BAND_DEFAULT_ENABLED = false`. Callers `enable()` then explicit `dispatch_*`. Uses generic `FieldScheduler` cadence (same posture as `FirstSliceMappingSession`), **not** default production pass graph or `SimSession::tick`.

Profile id: `min_plus_traversal_field_v1` (`TRAVERSAL_FIELD_UTILITY_ID`).

## Fixture shape

Same 8×8 PATH-5 admitted tree:

- Location → 64 gridcells with `palma/grid_traversal` Named `w` / `d` columns
- Fleet convoy under start gridcell (7,7)
- Row-major gridcell id binding in `TraversalFieldGridBinding`

## W source column resolution

1. W seeded on gridcell property columns (PATH-5).
2. `project_tree_to_values` → session shadow buffer.
3. Band `gather_w_from_shadow` reads `w_global_col` per gridcell slot in row-major order.

## D output column resolution

1. Band GPU dispatch writes flat D → `scatter_d_to_shadow` at `d_global_col` per slot.
2. `sync_d_from_shadow_to_properties` copies shadow D into gridcell property columns.

## GPU invocation path

**Session band `tick(mode, shadow_writeback)`** — when scheduler dispatches:

1. Gather W from shadow
2. `MinPlusTraversalFieldOp::dispatch_traversal` with explicit execution mode
3. **Production default:** `GpuResident` — no CPU readback, no shadow D writeback
4. **Diagnostic:** `DiagnosticReadback` + `shadow_writeback=true` for PATH-5/6 property path
5. **Verification:** `OracleVerification` for CPU oracle compare (tests)

PATH-7 refactored proof scaffolding out of the production hot path — see [`palma_path_7_gpu_traversal_utility_results.md`](palma_path_7_gpu_traversal_utility_results.md).

## Proof coverage

| Test | Claim |
|---|---|
| `min_plus_band_default_off` | Default disabled; tick is no-op |
| `session_band_gathers_w_from_admitted_shadow_columns` | W shadow gather matches property columns |
| `session_band_dispatches_gpu_min_plus_not_manual_test_body` | Band tick dispatches GPU; oracle err < 1e-4 |
| `session_band_writes_d_to_shadow_and_property_columns` | D writeback matches CPU oracle |
| `after_band_movable_samples_d_and_reparents_generically` | Property sampling + Reparent after band |
| `path6_blocker_ledger_default_simsession_not_wired` | Default SimSession not wired |
| `on_event_cadence_skips_until_dirty` | OnEvent cadence skips without `event_pending` |

## CPU / GPU parity

Verification readback on band tick: **PASS** (`max_d_field_error < 1e-4`).

## Blocker ledger

| Item | Status |
|---|---|
| Opt-in band module in `simthing-driver` | **PASS** |
| FieldScheduler cadence integration | **PASS** |
| W from admitted property/shadow columns | **PASS** |
| D writeback to property/shadow columns | **PASS** |
| GPU MinPlusStencilOp reused | **PASS** |
| Default `SimSession::tick` schedules band | **NOT WIRED** |
| `RegionFieldOperatorSpec::MinPlus` in spec admission | **NOT ADDED** (band uses PALMA profile, not StructuredFieldStencilOp) |
| Production movement policy | **NOT LANDED** |

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-driver --test palma_path_min_plus_oracle
cargo test -p simthing-gpu --test min_plus_stencil
cargo test -p simthing-driver --test palma_path_6_session_regionfield
target/debug/deps/palma_path_5_install_session_property-*.exe   # regression 7/7 PASS
```

**`cargo test --workspace` not run.**

## Boundaries preserved

No pathfinding engine, graph manager, movement engine, route object, predecessor table, semantic SEAD, ClauseThing/simthing-sim changes, semantic GPU branches, sqrt/magnitude, or production Dijkstra/A*.
