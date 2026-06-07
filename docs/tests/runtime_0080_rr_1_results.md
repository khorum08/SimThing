# RUNTIME-0080-RR-1 Results

Status: IMPLEMENTED / PASS — nested sparse residency for galaxy→system→planet-surface
Verdict: PASS
Primitive: `NESTED-SPARSE-RESIDENCY-0`
Rung: `RUNTIME-0080-RR-1`
Scope: RR-0 recursive world consumed under nested sparse residency (galaxy always resident; system/surface materialize on descend; ascend deactivates child tiers; child visibility follows resident parent)
Stable report checksum: `e6153526c1541764`
Deterministic replay checksum: residency trace FNV over RR-0 structural checksum + 8-step canonical access pattern

## Scope Ledger

| Spec element | Required by spec | Implemented in RR-1 | Status | Evidence | Deviation |
|---|---|---|---|---|---|
| RR-0 recursive world consumed | yes | yes | implemented | `build_recursive_world`; `rr_0_world_consumed=true`; `is_flattened=false` | |
| Galaxy 20×20 always resident | yes | yes | implemented | `galaxy_materialized_rows==400` on every trace step | |
| 13 system handles/addressable child nodes | yes | yes | implemented | `system_handles.len()==13` | |
| System 10×10 materializes on descend | yes | yes | implemented | `system_materialized_rows==100` on `DescendToSystem` | |
| System 10×10 deactivates/inactivates on ascend | yes | yes | implemented | `AscendToGalaxy` zeros system rows | |
| Planet handle addressable through parent system | yes | yes | implemented | `planet.parent_system_id` matches `system.id` per mapping parity | |
| Planet surface 10×10 materializes on descend | yes | yes | implemented | `surface_materialized_rows==100` on `DescendToSurface` | |
| Planet surface 10×10 deactivates/inactivates on ascend | yes | yes | implemented | `AscendToSystem` zeros surface rows | |
| Starport child visible through resident system | yes | yes | implemented | `child_visibility.starport_visible` when system tier active | |
| Pop cohort child visible through resident surface | yes | yes | implemented | `child_visibility.pop_cohort_visible` when surface tier active | |
| Factory child visible through resident surface | yes | yes | implemented | `child_visibility.factory_visible` when surface tier active | |
| Terran system residency path proven | yes | yes | implemented | Terran `DescendToSystem` + `DescendToSurface` trace | |
| Pirate system residency path proven | yes | yes | implemented | Pirate `DescendToSystem` + `DescendToSurface` trace | |
| No galaxy→wrong-system leakage | yes | yes | implemented | `try_access_system_at_galaxy_cell` rejects cell mismatch | |
| No system→wrong-planet leakage | yes | yes | implemented | `try_access_surface_for_system` rejects active-system mismatch | |
| No inactive-surface child leakage | yes | yes | implemented | galaxy-only snapshot `visible_child_count==0` | |
| Sparse accounting proves inactive systems/surfaces are not fully materialized | yes | yes | implemented | `inert_cell_count>0`; `resident_cell_count<3000` until deepest step | |
| Mapping parity vs RR-0 | yes | yes | implemented | 13 `mapping_parity_rows` all match owners, parents, dims, child placements | |
| GPU economy deferred to RR-2 | no | n/a | deferred | AccumulatorOp GPU path | |
| Recursive GPU reduce/disburse deferred to RR-3 | no | n/a | deferred | §0.2 GPU reduce-up/disburse-down | |
| Integrated recursive 100-tick rehearsal deferred to RR-4 | no | n/a | deferred | recursive GPU horizon | |

No Deviation Record required — rows 1–18 are `implemented`.

## RR-0 consumption summary

RR-1 consumes the RR-0 recursive world via `build_recursive_world(0x0080_2000)`. The world is not flattened (`rr_0_is_flattened=false`). RR-0 structural checksum is preserved and referenced in the residency execution checksum. Mapping parity rows confirm all 13 systems retain RR-0 owners, parent galaxy cells, 10×10 system subgrids, 10×10 planet surfaces, and pop/factory child placements.

## Residency model summary

```text
Galaxy (20×20) — always resident (400 cells)
└── System {id} (10×10) — materializes on DescendToSystem; deactivates on AscendToGalaxy
    ├── Starport child — visible only when system tier is resident
    └── Surface {id} (10×10) — materializes on DescendToSurface; deactivates on AscendToSystem
        ├── Pop cohort child — visible only when surface tier is resident
        └── Factory district child — visible only when surface tier is resident
```

Total logical cell budget: 3,000 (`400 + 13×(100+100)`). Only active tiers contribute to `resident_cell_count`; inactive system/surface tiers remain in `inert_cell_count`.

## Active tier trace (canonical 8-step pattern)

| Step | Request | Galaxy rows | System rows | Surface rows | Resident total | Inert | Starport | Pop | Factory |
|---:|---|---:|---:|---:|---:|---:|---|---|---|
| 0 | DescendToSystem (Terran) | 400 | 100 | 0 | 500 | 2,500 | yes | no | no |
| 1 | DescendToSurface (Terran) | 400 | 100 | 100 | 600 | 2,400 | yes | yes | yes |
| 2 | AscendToSystem (Terran) | 400 | 100 | 0 | 500 | 2,500 | yes | no | no |
| 3 | AscendToGalaxy | 400 | 0 | 0 | 400 | 2,600 | no | no | no |
| 4 | DescendToSystem (Pirate) | 400 | 100 | 0 | 500 | 2,500 | yes | no | no |
| 5 | DescendToSurface (Pirate) | 400 | 100 | 100 | 600 | 2,400 | yes | yes | yes |
| 6 | AscendToSystem (Pirate) | 400 | 100 | 0 | 500 | 2,500 | yes | no | no |
| 7 | AscendToGalaxy | 400 | 0 | 0 | 400 | 2,600 | no | no | no |

## Entity/tier counts (final galaxy-only state)

| Entity / tier | Count |
|---|---:|
| Galaxy resident rows | 400 |
| Active system rows | 0 |
| Inactive system rows | 1,300 |
| Active planet surface rows | 0 |
| Inactive planet surface rows | 1,300 |
| Active child rows | 0 |
| Addressable system handles | 13 |

## Terran path proof

Terran system is selected from RR-0 world by `Runtime0080Rr0Owner::Terran`. Canonical pattern steps 0–3 descend to Terran system 10×10, descend to Terran surface 10×10 (pop + factory visible), ascend to system (surface deactivated), ascend to galaxy (system deactivated). `terran_path_proven=true`.

## Pirate path proof

Pirate system is selected from RR-0 world by `Runtime0080Rr0Owner::Pirate`. Canonical pattern steps 4–7 repeat the same residency lifecycle for the Pirate system. `pirate_path_proven=true`.

## No-leakage proof

- **Galaxy→wrong-system:** `try_access_system_at_galaxy_cell` rejects when galaxy cell does not match system's `parent_galaxy_x/y`.
- **System→wrong-planet:** `try_access_surface_for_system` rejects when active system id does not match requested system id.
- **Inactive-surface child leakage:** galaxy-only snapshot has `visible_child_count==0`; `inactive_surface_child_count==0`.

## Sparse accounting proof

At galaxy-only residency, `resident_cell_count=400` and `inert_cell_count=2600` (12 systems + 13 surfaces not materialized). At deepest surface residency, `resident_cell_count=600` (one system + one surface materialized alongside always-resident galaxy). `sparse_only_active_tiers=true` on every trace step.

## Mapping parity vs RR-0

All 13 `mapping_parity_rows` confirm: owner matches RR-0; parent galaxy linear index matches `y*20+x`; system dims 10×10 (100 cells); surface dims 10×10 (100 cells); pop kind `PopCohort`; factory kind `FactoryDistrict`. `mapping_parity_ok=true`.

## Explicit non-claims

- **No RR-2 GPU economy** — pop/factory labor/production not on GPU.
- **No RR-3 recursive GPU reduce/disburse** — reduce-up/disburse-down not on GPU.
- **No RR-4 integrated recursive GPU 100-tick rehearsal** — no integrated GPU oracle.
- **No standalone M-4A parallel-theater track** — nested containment only; not multi-theater parallel dispatch.
- **No default session wiring** — opt-in/default-off.
- **No invariant edit** — `docs/invariants.md` untouched.
- **No RUNTIME-0080-0 reopen** — flat R2 loop unchanged.
- **No pinned R2 checksum change**.

## Foreground command results

```text
cargo test -p simthing-driver --test runtime_0080_rr_1          → 30 passed
cargo test -p simthing-driver --test runtime_0080_rr_0          → 30 passed
cargo test -p simthing-driver --test runtime_0080_0_r2          → 19 passed
cargo test -p simthing-driver --test atlas_0080_0               → 17 passed
cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run → 22 passed
cargo test -p simthing-gpu                                      → 203 passed; 1 ignored
cargo build --workspace                                         → success
cargo fmt --all -- --check                                      → success
cargo check --workspace                                         → success
```

## Scratch/log cleanup

No scratch logs, `target/`, worktree artifacts, or replay LDJSON committed. All visibility in this report only.
