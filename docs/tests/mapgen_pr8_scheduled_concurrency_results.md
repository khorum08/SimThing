# MapGen PR8 Scheduled-Concurrency GPU Measurement Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (PR8 scheduled-concurrency report; DA-approved 2026-06-14).

## Verdict

**PASS / DA-APPROVED (2026-06-14, Opus / Design Authority)** — DA performed a genuine pre-merge audit of
the GPU source (not the PR body) and confirmed the M8 boundary holds: **scheduling only, no new kernel**.
`scheduled_w_palma_batch.rs` is pure command-encoder glue over the existing `WImpedanceComposeOp` +
`MinPlusStencilOp`; the edits to `indexed_scatter` / `min_plus_stencil` / `w_impedance_compose` are
record-into-encoder refactors reusing the existing pipelines (min-plus `D = W + min(N4 D)` math untouched);
a GPU risky-token scan found **no new WGSL / compute-pipeline / shader-module / sqrt / distance / normalize /
hypot / euclidean**. Serial (7 submits) and scheduled (1 submit) use the **identical seed buffer** and the
test asserts their compact single-cell D probes match within 1e-3 — proving the scheduled path does the same
work (not a no-op shell, serial not penalized). No full-field CPU decision readback (`report.values.is_none()`
asserted), no CPU planner, default-off preserved, no simthing-sim/new-`SimThingKind`/Euclidean. **DA reran the
battery green on a real GPU adapter** (`mapgen_pr8_scheduled_concurrency` 6 passed in ~1.3s — the GPU path
ran; clausething 8/10/16/19/23/19/45; `ct_bh3_closeout_sample_driver` 2 passed; fmt/`git diff --check` clean).
PR8 adds a GPU-resident measurement harness comparing serial queue submits vs
single-encoder scheduled W compose + PALMA min-plus over the PR7 MapGen tiny pentad slice. Uses existing
`WImpedanceComposeOp`, `MinPlusStencilOp`, and `MinPlusTraversalDProbeOp` only — no fused kernel, no semantic
WGSL, no simthing-sim changes, no route/path/predecessor/movement semantics, no full-field CPU decision
readback. Compact D probe readback only.

## Track scope

0.0.8.2.5 MapGen PR8: Gu-Yang ∥ PALMA scheduled-concurrency GPU measurement spike (M8). **Do not merge until
DA review.**

PR8 is scheduled-concurrency / GPU measurement only. PR8 reuses existing GPU-resident ops unless DA explicitly
approves otherwise. PR8 does not add semantic WGSL. PR8 does not add pathfinding/movement/routes/predecessors.
PR8 does not widen Movement-Front horizon. PR8 does not implement FIELD-MOVIE-DATASET-0 export. PR8 does not
reopen 0.0.8.2 closeout.

## Binding read-order recorded

1. `docs/invariants.md`
2. `docs/simthing_core_design.md` §1.1 and §7
3. `docs/adr/mapping_sparse_regioncell.md`
4. `docs/adr/resource_flow_substrate.md`
5. `docs/design_0_0_8_2_5_mapgen_ladder.md` §0, §3, §6 PR8, §8, §9
6. `docs/design_0_0_8_1_border_hack_track.md`
7. `docs/design_0_0_8_1_palma_pathfinding_integration_guide.md`
8. `docs/clausething/mapgen_corpus_manifest.md`
9. `docs/clausething/MapGenThing.md`
10. `docs/tests/mapgen_pr1`–`mapgen_pr7` results
11. `docs/tests/clausething_closeout_results.md`

## Files changed

| Area | Path |
|---|---|
| GPU batching helper | `crates/simthing-gpu/src/scheduled_w_palma_batch.rs` |
| Encoder record APIs | `crates/simthing-gpu/src/w_impedance_compose.rs`, `indexed_scatter.rs`, `min_plus_stencil.rs` |
| PR8 harness tests | `crates/simthing-driver/tests/mapgen_pr8_scheduled_concurrency.rs` |
| Fixture README | `crates/simthing-clausething/tests/fixtures/mapgen/README.md` |
| MapGen ladder | `docs/design_0_0_8_2_5_mapgen_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |
| PALMA guide | `docs/design_0_0_8_1_palma_pathfinding_integration_guide.md` |
| MapGen reference | `docs/clausething/MapGenThing.md` |
| PR8 report | `docs/tests/mapgen_pr8_scheduled_concurrency_results.md` |

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `mapgen_pr1`–`mapgen_pr7` reports/guardrails | CURRENT_EVIDENCE / LIVE_GUARDRAIL / PROBATION (PR7) | Unchanged |
| `scheduled_w_palma_batch.rs` | CURRENT_EVIDENCE | Generic GPU batching helper (DA-approved) |
| `mapgen_pr8_scheduled_concurrency.rs` (tests) | LIVE_GUARDRAIL | Promoted at DA approval |
| `docs/tests/mapgen_pr8_scheduled_concurrency_results.md` | CURRENT_EVIDENCE | This report; DA-approved |
| Scratch logs / duplicate reports / worktrees | DELETE | None found |

## Measurement summary

| Field | Value |
|---|---|
| Fixture | PR7 MapGen tiny pentad (`generate_default_mapgen_palma_feedstock`) |
| Grid | 3×3 |
| Traversal iterations | 4 |
| Serial queue submits (W→PALMA chain) | 7 (= 3 + iterations) |
| Scheduled queue submits (W→PALMA chain) | 1 |
| Compact evidence | Single-cell D probe at lattice (1,1) |
| Probe tolerance | 1e-3 (serial vs scheduled) |
| Full-field D readback | None in gpu_resident mode |
| Fused kernel | None |
| Mapping profile in pack | `Disabled` (explicit opt-in in harness only) |

## GPU adapter status

| Run | Adapter | Result |
|---|---|---|
| Local dev (2026-06-13) | Available | GPU tests ran; serial/scheduled probes matched within tolerance |
| CI without adapter | N/A | GPU tests skip cleanly with explicit message |

## Validation battery

```text
cargo fmt --all -- --check          PASS
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse    8 passed
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy   10 passed
cargo test -p simthing-clausething --test mapgen_resource_flow       16 passed
cargo test -p simthing-clausething --test mapgen_links               19 passed
cargo test -p simthing-clausething --test mapgen_movement_front      23 passed
cargo test -p simthing-clausething --test mapgen_palma               19 passed
cargo test -p simthing-clausething --test ct_scenario_container      45 passed
cargo test -p simthing-driver --test mapgen_pr8_scheduled_concurrency  6 passed (GPU ran locally)
cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver   (required — shared GPU harness)
git diff --check                    PASS
```

## DA sign-off status

**DA-APPROVED (2026-06-14, Opus / Design Authority)** after a genuine pre-merge GPU-source audit + battery
rerun on a real adapter. The M8 boundary holds (scheduling-only, no fused kernel). `scheduled_w_palma_batch`
+ `min_plus_stencil`/`indexed_scatter`/`w_impedance_compose` record-refactors and the harness test promoted
per the lifecycle table.

## Governance

PR9 may proceed only after DA approval of PR8 (subject to its own gate).
