# MapGen PR10 End-to-End Compact Evidence Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (PR10 end-to-end report; DA-approved 2026-06-14).

## Verdict

**PASS / DA-APPROVED (2026-06-14, Opus / Design Authority; Cursor PR 10)** — DA performed a genuine
pre-merge audit (branch source, not the PR body) and reran the full battery green on a real GPU adapter
(`mapgen_pr10_end_to_end_compact_evidence` 3 passed; clausething 8/10/16/19/23/19/21/45; `mapgen_pr8` 6;
`ct_bh3` 2; fmt/`git diff --check` clean). Confirmed: a GPU risky-token scan found **no new
WGSL/compute-pipeline/shader-module, no new `SimThingKind`, no sqrt/distance/normalize/euclidean, no
Route/Predecessor** in added source; the generator edits are **install-correctness only** (per-system
unique property names + arena `explicit_participants` resolving **real install slots** via a DFS tree
walk instead of `enumerate()` indices, so the pack actually admits/installs — still inert render
positions, no new capability); the harness does real `install_atomic` + `SimSession::open_from_spec` +
`FirstSliceMappingSession` with **compact evidence only** (`field_values`/`reduction_parent_value`/`eml_output`
all `is_none()` — no full-field CPU decision readback), GPU-adapter required, mapping default-off
preserved. tiny MapGen canonical sample executes end-to-end
through PR2 neutral parse → PR3 lattice → PR4 RF → PR5 links → PR6 Movement-Front → PR7 PALMA →
`install_atomic` admission → GPU-resident mapping tick + scheduled W/PALMA chain + compact D probe on a
real adapter. Compact evidence only; no full-field CPU decision readback; no CPU planner; no route/path/
predecessor/movement/border/frontline semantics; no Euclidean authority; no new `SimThingKind`; no semantic
WGSL; no new GPU kernel; no `simthing-sim` changes; no FIELD-MOVIE-DATASET-0 export.

## Track scope

0.0.8.2.5 MapGen PR10: first rung proving the tiny pentad fixture end-to-end through ingest, generation,
admission/install, and GPU compact evidence. **Do not merge until DA review.**

PR10 runs the tiny MapGen sample end-to-end through ingest/generate/admit/install/GPU compact evidence.
PR10 uses existing GPU-resident surfaces. PR10 does not add new gameplay semantics. PR10 does not add
pathfinding/movement/routes/predecessors. PR10 does not add semantic WGSL. PR10 does not add a new GPU
kernel. PR10 does not implement FIELD-MOVIE-DATASET-0 export. PR10 does not reopen 0.0.8.2 closeout.

## Binding read-order recorded

1. `docs/invariants.md`
2. `docs/simthing_core_design.md` §1.1 and §7
3. `docs/design_0_0_8_1.md` §0.7
4. `docs/adr/mapping_sparse_regioncell.md`
5. `docs/adr/resource_flow_substrate.md`
6. `docs/design_0_0_8_2_5_mapgen_ladder.md` §0, §3, §6 PR10, §8, §9
7. `docs/design_0_0_8_1_border_hack_track.md`
8. `docs/design_0_0_8_1_palma_pathfinding_integration_guide.md`
9. `docs/clausething/mapgen_corpus_manifest.md`
10. `docs/clausething/MapGenThing.md`
11. `docs/tests/mapgen_pr1`–`mapgen_pr9` results
12. `docs/tests/clausething_closeout_results.md`

## Files changed

| Area | Path |
|---|---|
| PR10 end-to-end harness | `crates/simthing-driver/tests/mapgen_pr10_end_to_end_compact_evidence.rs` |
| Install/admission fixes | `crates/simthing-clausething/src/mapgen_lattice.rs`, `mapgen_links.rs`, `mapgen_resource_flow.rs` |
| Guard test updates | `crates/simthing-clausething/tests/mapgen_lattice_hierarchy.rs`, `mapgen_constitution_guards.rs`, `mapgen_links.rs` |
| MapGen ladder | `docs/design_0_0_8_2_5_mapgen_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |
| PALMA guide | `docs/design_0_0_8_1_palma_pathfinding_integration_guide.md` |
| MapGen reference | `docs/clausething/MapGenThing.md` |
| Fixture README | `crates/simthing-clausething/tests/fixtures/mapgen/README.md` |
| PR10 report | `docs/tests/mapgen_pr10_end_to_end_results.md` |

## End-to-end stages run

| Stage | Surface | Evidence |
|---|---|---|
| 1. Load fixture | `tiny_pentad_hub_slice_raw.clause` | Raw bytes embedded in test |
| 2. PR2 neutral parse | `parse_mapgen_neutral_document` | Ok |
| 3. PR3 lattice | `generate_mapgen_lattice_hierarchy` | 5 systems, ordinary `Location` gridcells |
| 4. PR4 RF | `generate_mapgen_resource_flow_enrollment` | 2 arenas, expansion report |
| 5. PR5 links | `generate_default_mapgen_links_enrollment` | 3 N4 links + lane couplings |
| 6. PR6 Movement-Front | `generate_default_mapgen_movement_front_authoring` | L1 SaturatingFlux, L2 reduction, L3 commitment |
| 7. PR7 PALMA | `generate_default_mapgen_palma_feedstock` | PALMA + W compose feedstock |
| 8. Admit/install | `install_atomic` + `compile_*_preview` | Region field, W compose, commitment admitted |
| 9. GPU mapping | `FirstSliceMappingSession` (explicit opt-in) | Scheduled tick; `field_values.is_none()` |
| 10. GPU PALMA | scheduled W/PALMA chain + compact D probe | 1 encoder submit; 1 probe cell; finite D |

## GPU adapter status

- **Adapter present on validation machine:** yes (wgpu adapter available)
- **GPU actually ran for PASS test:** yes — `mapgen_pr10_end_to_end_compact_gpu_evidence` requires adapter (`expect`, not skip)
- **GPU skip treated as PASS:** no — harness source asserts `PR10 PASS requires GPU adapter`

## Compact evidence summary

Bounded counts from the PR10 GPU test (hub cell row=1 col=1):

- `region_fields`: 1
- `rf_arenas`: 2
- `links`: 3
- `lane_couplings`: 2
- `palma_feedstock`: 1
- `commitment`: present
- `mapping_scheduled`: true
- `threshold_events`: ≤ 4 (observed: 1)
- `d_probe_cells`: 1 (≤ cap 4)
- `d_probe_finite`: true
- `traversal_gpu_resident`: true
- `full_field_readback`: false
- `scheduled_encoder_submits`: 1
- `serial_submits`: 7 (baseline comparison only)

## Forbidden-boundary summary

- No route/path/predecessor/movement/border/frontline/cpu_planner surfaces in generated pack JSON
- P1 horizon remains bounded (`MAPGEN_MF_DEFAULT_HORIZON`, no `allow_extended_horizon`)
- Mapping profile default-off until test explicitly opts into `SparseRegionFieldV1`
- Render positions remain inert metadata (lattice `(row,col)` for links/seeds)
- PALMA W/D remains field feedstock — compact D probe only

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `mapgen_pr1`–`mapgen_pr8` reports/guardrails | CURRENT_EVIDENCE / LIVE_GUARDRAIL | Unchanged |
| `mapgen_pr9_constitution_guards_results.md` | CURRENT_EVIDENCE | Unchanged (DA-approved) |
| `mapgen_pr10_end_to_end_compact_evidence.rs` | LIVE_GUARDRAIL | Promoted at DA approval |
| `mapgen_pr10_end_to_end_results.md` | CURRENT_EVIDENCE | This report; DA-approved |
| Prior `mapgen_*` guard batteries | LIVE_GUARDRAIL | Unchanged |
| Scratch logs / duplicate reports / worktrees | DELETE | None found |

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_resource_flow
cargo test -p simthing-clausething --test mapgen_links
cargo test -p simthing-clausething --test mapgen_movement_front
cargo test -p simthing-clausething --test mapgen_palma
cargo test -p simthing-clausething --test mapgen_constitution_guards
cargo test -p simthing-clausething --test ct_scenario_container
cargo test -p simthing-driver --test mapgen_pr8_scheduled_concurrency
cargo test -p simthing-driver --test mapgen_pr10_end_to_end_compact_evidence
cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver
git diff --check
```

## Test results (2026-06-13 local validation)

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | pass |
| `cargo test -p simthing-clausething --test mapgen_neutral_ast_parse` | 8 passed |
| `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy` | 10 passed |
| `cargo test -p simthing-clausething --test mapgen_resource_flow` | 16 passed |
| `cargo test -p simthing-clausething --test mapgen_links` | 19 passed |
| `cargo test -p simthing-clausething --test mapgen_movement_front` | 23 passed |
| `cargo test -p simthing-clausething --test mapgen_palma` | 19 passed |
| `cargo test -p simthing-clausething --test mapgen_constitution_guards` | 21 passed |
| `cargo test -p simthing-clausething --test ct_scenario_container` | 45 passed |
| `cargo test -p simthing-driver --test mapgen_pr8_scheduled_concurrency` | 6 passed |
| `cargo test -p simthing-driver --test mapgen_pr10_end_to_end_compact_evidence` | 3 passed |
| `cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver` | 2 passed |
| `git diff --check` | pass |

**Total focused tests:** 172 passed, 0 failed.

## DA sign-off status

**DA-APPROVED (2026-06-14, Opus / Design Authority)** after a genuine pre-merge audit + full-battery rerun
on a real GPU adapter. The end-to-end harness `mapgen_pr10_end_to_end_compact_evidence` is durable and
confirmed **LIVE_GUARDRAIL**. Only the Design Authority writes a DA sign-off.

## PR11 closeout

**May now proceed** — under its own DA-review gate.
