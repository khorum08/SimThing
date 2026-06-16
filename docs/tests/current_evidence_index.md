# Current Evidence Index — MapThing / MapGeneratorCLI vertical (live guardrails + amendment provenance)

> **Status: LIVE LEDGER (2026-06-15, executive design authority).** This is the durable, compact index of
> **current evidence** for the closed ClauseThing / MapThing / MapGeneratorCLI vertical. It exists because
> the per-rung PROBATION/process reports under `docs/archive/superseded_tests/` were **expunged from the
> working tree** (to stop agents re-deriving superseded small-scale assumptions) — but the *provenance* of
> live guardrails must not be erased. **This index is current evidence; it is NOT a superseded-proof
> archive.** Where a full report was removed, the live `.rs` test + the cited PR / merge SHA are the proof.
>
> Restoring detail: `git show <SHA>` / `git log` recover any expunged report; the durable findings are
> digested in [`../adr/ClauseThingADR.md`](../adr/ClauseThingADR.md) and [`../clausething/ClauseThingDoc.md`](../clausething/ClauseThingDoc.md).

## Live guardrails (run today; the proof is the test, not a report)

| Guardrail | Live test (in `crates/`) | What it proves |
|---|---|---|
| **GPU compact evidence** | `simthing-clausething/tests/mapgenerator_cli_pr10_gpu_compact_evidence.rs` | A generated pack admits/installs via `install_atomic` + `SimSession::open_from_spec` and produces **real-adapter** GPU-resident compact evidence (`field_values`/`reduction_parent_value`/`eml_output` `is_none()`). |
| **Constitution guards** | `simthing-clausething/tests/mapgen_constitution_guards.rs` | No Euclidean/forbidden-vocab/new-kind tokens in the closed generators; horizon bounded; positions structural-spatial; default-off. |
| **Structural admission** | `simthing-clausething/tests/mapgen_structural_admission.rs` | No fixed edge cap; budget-based admission; checked-`u128` capacity; **real 1500-star/300×300 producer spiral lowers structurally**. |
| **Vast-scale layout / atlas deferral** | `simthing-clausething/tests/mapgen_vast_scale_layout.rs` | A ~1000-edge lattice lays out with honored positions; Movement-Front over a vast lattice **typed-defers** to the atlas without invalidating the layout. |
| **STEAD spatial contract** | `simthing-clausething/tests/stead_spatial_contract_guards.rs` | Active source/docs (incl. producer `mapgenerator_emitter`/`topology`) never reassert withdrawn drift doctrine; budget admission exported; no fixed edge cap; positions honored; MF typed atlas deferral; PALMA structural/no-routes; **constitution §0 carries the §0.8 STEAD clause + `stead_spatial_contract.md` pointer forward**; index has no stale pending rows. |
| **RF ⇄ STEAD binding** | `simthing-clausething/tests/mapgen_rf_stead_binding.rs` | RF/Accumulator arenas over gridcell Locations require structural placements + record the `StructuralGridFrame`; spatially-neutral arenas need no grid. |
| **Galaxy connectivity** | `simthing-mapgenerator/tests/connectivity.rs` | `connect_components` (authored-coord union-find + Kruskal + nearest-pair fallback) guarantees ONE interconnected galaxy (no island clusters); merges islands, no-op when connected, deterministic, no self/dup; proven on the real 1500-star disc. |
| **Producer topology STEAD** | `simthing-mapgenerator/tests/topology_stead.rs` | All producer adjacency (base hyperlanes + special routes + partition/cluster bridges) selects from **authored structural gridcell coordinates** (`PlacedSystemSeed.coord`), not emission order; authored-near connects, emission-near-but-authored-far does not; 1500-star spiral base hyperlanes are local with no self/duplicate/unknown-endpoint links; **the 1500-star placement is a dispersed sparse spiral, never a row-major brick**. |
| **Candidate F exact sqrt** | `simthing-driver/tests/phase_m_jit_sqrt_exact5f_exhaustive_sweep.rs` (+ artifact) | The **permanently enshrined** exact-magnitude chain for decision gates (constitution §0.7) — bit-exact Euclidean/sqrt ops route through Candidate F, never avoided. |

## Amendment / closeout provenance (PR # · merge SHA · status)

| Evidence | PR | Merge SHA | Status |
|---|---|---|---|
| **Candidate F exact sqrt** (artifact hash `59ab4b2892e3c690`, LF-canonical re-pin `SQRT-REPIN-0` 2026-06-11) | — | constitution §0.7 lineage | CURRENT — the only exact-magnitude authority for decision gates |
| **PR10** — generated admit/install + real-adapter GPU compact evidence | #690 | `75505ee2` | CURRENT_EVIDENCE + LIVE GPU GUARDRAIL |
| **PR11** — 1000-star producer scale envelope (parse/lattice; heap-bounded enumeration) | #692 | `31f0ee3e` | CURRENT_EVIDENCE |
| **PR12** — MapGeneratorCLI 0.0.8.6 track closeout | #693 | `1b1d374c` | CURRENT_EVIDENCE — track CLOSED |
| **STEAD-PRIVILEGE-0** — gridcell positions are structural; lowerer honors emitted `(col,row)` | #698 | `b1dcd63b` | CURRENT_EVIDENCE (closed-lowerer amendment) |
| **STEAD-SCALE-0** — layout/execution scale decoupled; Movement-Front bounded-theater + atlas deferral | #699 | `3162ca84` / merge `4ec32995` | CURRENT_EVIDENCE |
| **STEAD-SCALE-1** — removed fixed structural edge cap; budget-based `admit_structural_grid`; typed atlas deferral; current-evidence index | #700 | `793d2633` / merge `3f0ece0a` | CURRENT_EVIDENCE |
| **STEAD-CONTRACT-0** — executable STEAD spatial contract doc + guards; RF/Accumulator spatial binding over Locations; PALMA/Gu-Yang/MF structural-frame references; evidence-index repair | #701 | `a4977b1f` / merge `3e26bf58` | CURRENT_EVIDENCE — DA-APPROVED 2026-06-15 (owner sign-off; PROBATION cleared after 0R) |
| **STEAD-CONTRACT-0R** — probation-hardening: transient-constitution **§0.8** STEAD carry-forward clause; guard proves §0 carries the clause + contract pointer; producer `emitter`/`topology` added to phrase scan | #703 | `8cb5bfe1` / merge `9d957fe8` | CURRENT_EVIDENCE — DA-APPROVED 2026-06-15 (owner sign-off) |
| **MAPGENCLI-TOPOLOGY-STEAD-0** — producer base-hyperlane adjacency selects on authored structural coords (`PlacedSystemSeed.coord`), not lowered index-order; `topology_stead.rs` regressions; spiral-1500 PNG regenerated | #706 | `fd7246e7` / merge `cb17c34c` | CURRENT_EVIDENCE — DA-APPROVED 2026-06-15 (owner sign-off) |
| **MAPGENCLI-TOPOLOGY-STEAD-1** — sibling producer couplings (special routes, partition bridges + BFS/DFS ordering, cluster bridges) migrated to authored coords; dispersion guard proves spiral placement is not a brick | #708 | `8348a5f8` / merge `b1421d68` | CURRENT_EVIDENCE — DA-APPROVED 2026-06-15 (owner sign-off) |
| **MapGeneratorCLI render: starlane target/colour + 3000px spiral artifact** — `--num-hyperlanes` target, `--hyperlane-color`, size-scaled line/star; 4-arm 1500-star blue-starlane 3000px render (real algorithmic hyperlanes, no wormholes) | #710 | `10f7ae37` / merge `d649c964` | CURRENT_EVIDENCE — DA-APPROVED 2026-06-15 (owner sign-off) |
| **Galaxy connectivity + connected disc artifact** — `connect_components` pass + `--connect-galaxy`/`--no-partitions`/`--draw-core`; 1500-star disc galaxy with bright inaccessible core void, clustered, ONE connected component (all 1500 stars laned, no islands), blue lanes, 3000px | #712, #714 | `a486cc09` / merge `06c90198`; core-glow + connected-web rev `41fb024f` / merge `0599b036`; preview lane-clip fix `81e32de7` / merge `5033b9c8` | CURRENT_EVIDENCE — DA-APPROVED 2026-06-15 (owner sign-off) |
| **Connectivity proof surfaced + connected-by-default** — `GalaxyGenerationResult.connectivity` report + CLI readout; `ensure_connected` defaults ON (`--allow-disconnected` opt-out) | #718, #719 | surface `7eeb914b` / merge `4b3e82b0`; default-on `3e2616d6` / merge `9e2ff4d9` | CURRENT_EVIDENCE — DA-APPROVED 2026-06-15 (owner sign-off) |
| **Dense 2-arm spiral artifact** — `--shape-param KEY=VALUE` flag; `spiral_2` 3000-star thick/dense arms (`arm_width`/`arm_tightness`/`jitter`), bright galactic core, ONE connected component, blue lanes, 3000px | #720 | `e5b2eff6` / merge `1ce3ce2d` | CURRENT_EVIDENCE — DA-APPROVED 2026-06-15 (owner sign-off) |

## Notes
- **No global structural lattice edge cap exists.** Structural scale is governed by `MapgenStructuralGridBudget` (default unbounded) + checked-`u128` math. `200×200` is a *small reference*; `65,535` was a temporary arithmetic ceiling and is **not doctrine** (removed in STEAD-SCALE-1).
- **Execution-profile limits are separate** (`simthing-spec` `REGION_FIELD_STANDARD/EXTENDED_MAX_GRID` ≤10/32 per edge bounded local theater). A vast layout may pass structurally while a dense execution profile **defers to atlas** — that is not "the map is too large."
- Superseded per-rung process reports are intentionally **not** in the working tree. Do not restore them as active guidance; cite this index + the live tests + git history.
