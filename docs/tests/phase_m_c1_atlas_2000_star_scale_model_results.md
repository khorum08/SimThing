# C-1 — 2000-Star Atlas Scale Model and Budget Gate Results

## Base HEAD
`6c05bb653baea22d0aa01a25adccac0230789474` (post C-0 landing, pre-C-1)

## Files changed
| File | Change |
|------|--------|
| `crates/simthing-driver/tests/support/c1_atlas_scale_model.rs` | **New** — pure allocation-safe 2000-star scale model + VRAM estimates |
| `crates/simthing-driver/tests/phase_m_c1_atlas_scale_model.rs` | **New** — 10 C-1 compliance + posture tests |
| `docs/tests/phase_m_c1_atlas_2000_star_scale_model_results.md` | **New** — this report |
| `docs/design_v7_8_production_track.md` | Added C-1 row (Done / Pending Opus Review) |
| `docs/design_v7_8.md` | Compact Line C note for C-1 |
| `docs/workshop/mapping_current_guidance.md` | C-1 status row |
| `docs/workshop/sead_self_ai_track.md` | Minimal cross-reference note |
| `docs/worklog.md` | Append-only C-1 entry |

**No production mapping runtime, no `request_atlas_batching` admission relaxation, no default-on atlas, no active-mask/source-identity (M-6A/M-5), no A-0/B-0 implementation, no ClauseThing/L3, no FrontierV2-5, no SEAD ladder reopen, no semantic WGSL, no simthing-sim map awareness, no invariant changes.**

## C-0 summary (context for C-1)
C-0 landed the first §11-gate M-4 atlas slice: homogeneous-square tile atlas dispatch, algebraic tile-local mask G=0 (preferred, 1.0×), full-tile protocol-oracle parity, and VRAM-multiplier report against the active configurable `V78AtlasVramBudget` (1.5 GiB default = 1_610_612_736 bytes, no architectural hard cap).

C-0 effective accounting (mandated for C-1):
- 256 payload cells → 32,768 algebraic-mask bytes → **128 bytes per payload cell** effective under G=0.
- Physical gutter fallback (10×10 H=8 reference) ≈ 6.76×.

C-0 is **Done / Pending Opus Review** (fingerprint `a974fe44e20620f3`). It did **not** implement production mapping runtime.

## User target scale model (exact handoff envelope)
```text
starmap_width = 200
starmap_height = 150
star_count = 2000
star_local_grid = 10 × 10
avg_planet_systems_per_star = 5
planet_system_grid = 10 × 10
avg_satellites_per_planet_system = 5
planet_surface_grid = 10 × 10
satellite_surface_grid = 10 × 10
```

C-1 model parameters (self-contained, matches handoff):
- n_dims = 4
- effective_algebraic_bytes_per_payload_cell = 128 (C-0 measured)

## Derived counts (exact)
| Level | Formula | Count |
|-------|---------|-------|
| starmap_cells | 200 × 150 | 30,000 |
| star_local_cells | 2000 × 100 | 200,000 |
| planet_system_count | 2000 × 5 | 10,000 |
| planet_system_orbital_cells | 10,000 × 100 | 1,000,000 |
| satellite_count | 10,000 × 5 | 50,000 |
| surface_body_count | 10,000 planets + 50,000 satellites | 60,000 |
| surface_cells | 60,000 × 100 | 6,000,000 |
| **total_dense_cells_if_all_resident** | — | **7,230,000** |

## C-0 accounting basis (used by C-1)
- Raw payload: n_dims × 4 bytes.
- Effective algebraic G=0 (C-0 measured): **128 bytes per payload cell**.
- This effective figure already incorporates the tile-local mask overhead for flush-packed algebraic isolation.

## Algebraic mask VRAM estimate (G=0)
- Total dense cells = 7,230,000
- Algebraic bytes = 7,230,000 × 128 = **925,440,000**
- Algebraic GiB ≈ **0.862**

## Physical gutter fallback estimate (10×10 H=8 reference)
- Multiplier = **6.76** (ratified C-0 / atlas isolation policy)
- Gutter bytes ≈ 6,255,974,400
- Gutter GiB ≈ **5.826**

## Active budget comparison (1.5 GiB default)
- Default budget = 1,610,612,736 bytes (1.5 GiB)
- Algebraic fits default = **true** (headroom ≈ 685 MiB / 0.638 GiB)
- Gutter fits default = **false**
- Minimum budget for gutter fallback ≈ **5.826 GiB**

## Commodity/default profile verdict
**Algebraic mask (G=0) is viable** under the 1.5 GiB default active budget for the all-resident 2000-star estimate.

**Physical gutter fallback is not viable** under the 1.5 GiB default for the all-resident estimate. It requires a raised active budget/profile (dedicated/headless servers or explicit larger `V78AtlasVramBudget`).

## Larger profile / headless-server note
The budget is **active and configurable** (no architectural hard cap). Raising `max_bytes` for high-VRAM environments makes the gutter fallback viable while preserving the same algebraic-first preference and mandatory multiplier reporting.

## Cadence / sparse-residency implications
Even the algebraic path at 7.23 M cells leaves limited headroom once ping-pong buffers, command buffers, other fields, and command encoding are considered. Real 2000-star games will require:
- Sparse resident subsets (only "interesting" regions loaded into atlas at any tick).
- Cadenced / scheduled updates (not every surface every tick).
- Horizon and LOD discipline.
- Active budget accounting + multiplier reporting on every atlas-using scenario.

## Admission posture (enforced by existing L1/L2 substrate)
- `request_atlas_batching` remains **rejected** at designer admission until C-0 (and subsequent C-line gates) are accepted.
- `MappingExecutionProfile` default remains **Disabled**.
- No production mapping runtime, no default-on atlas, no gutter fallback under commodity profile without explicit raised budget.

## Test results
All 10 required C-1 tests pass:
- `c1_scale_model_counts_2000_star_game`
- `c1_algebraic_mask_budget_fits_1p5_gib_default`
- `c1_physical_gutter_fallback_exceeds_1p5_gib_default`
- `c1_vram_budget_is_active_configurable_not_architectural_cap`
- `c1_uses_c0_effective_bytes_per_payload_cell`
- `c1_does_not_authorize_production_runtime`
- `c1_does_not_open_active_mask_or_source_identity`
- `c1_does_not_open_a0_b0_l3_frontierv2_5`
- `c1_reports_per_level_breakdown`
- `c1_algebraic_mask_first_recommended_for_this_scale`

C-0 regression suite continues to pass.

## Scans run (PowerShell-native + prior rg where available)
- C-1 / 2000-star / algebraic / gutter terms: present only in model, tests, and required doc updates.
- `request_atlas_batching` / default-on / production runtime: only in rejection/diagnostic contexts; no relaxation.
- Active mask / source identity / M-6A / M-5: explicitly deferred/rejected in designer admission and docs.
- A-0 / B-0 / E-11B / D-2: correctly shown as queued / NamedScenarioAccepted only.
- ClauseThing / L3 / FrontierV2-5 / ACT-5 etc.: parked / rejected.
- simthing-sim map awareness (RegionCell / atlas / source_mask): none introduced.
- Scratch / tmp / log files in `docs/tests/`: none relevant to C-1 (cleaned any transient outputs from this run).

## Transient cleanup result
No authoritative evidence deleted. Only transient build/test artifacts (if any) from this run were left for normal cargo cleanup.

## Final verdict

**PASS** — C-1 modeled the 2000-star target envelope: 200×150 starmap, 2000 10×10 star grids, 10,000 10×10 planet-system grids, and 60,000 10×10 planet/satellite surfaces. The all-resident estimate is 7,230,000 dense cells.

- Algebraic G=0 atlas accounting **fits** under the 1.5 GiB default active budget (≈0.862 GiB, ~0.638 GiB headroom).
- Physical gutter fallback **exceeds** the 1.5 GiB default (≈5.826 GiB) and requires a raised active budget/profile.
- C-1 added **no** production mapping runtime, no default-on atlas, no active-mask/source-identity (M-6A/M-5), no A-0/B-0, no ClauseThing/L3, no FrontierV2-5, no ACT/EVENT/OBS/PIPE reopen, no semantic WGSL, and no simthing-sim map awareness.
- C-line must continue **algebraic-mask-first**, with mandatory active budget accounting, multiplier reporting, cadence/scheduling, and sparse resident subsets where appropriate. Gutter fallback is only viable under explicitly raised budgets.

C-1 is complete and ready for Opus / design-authority review. It provides the required budget-gate evidence for Line C / M without violating any v7.8 constitutional constraints.