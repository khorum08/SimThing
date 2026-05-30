# C-0 — First §11-Gate M-4 Atlas Slice: Full-Tile Protocol-Oracle Parity + VRAM Report Results

## Base HEAD

`6c05bb653baea22d0aa01a25adccac0230789474` (post V7.8-MET-SCENARIO-ACCEPT-0 / CLAUSE-SPEC-0 acceptance, pre-C-0)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-gpu/src/atlas_mask.rs` | **New** — protocol CPU oracle + atlas GPU dispatch |
| `crates/simthing-gpu/src/shaders/structured_field_stencil_atlas_mask.wgsl` | **New** — semantic-free atlas tile-local mask WGSL |
| `crates/simthing-gpu/src/lib.rs` | Export atlas_mask module |
| `crates/simthing-driver/tests/support/c0_atlas_protocol_oracle.rs` | **New** — C-0 fixture + VRAM budget report support |
| `crates/simthing-driver/tests/phase_m_c0_m4_atlas_protocol_oracle.rs` | **New** — 13 C-0 tests |
| `docs/design_v7_8_production_track.md` | C-0 row Done/Pending Opus Review; cleanup section corrected |
| `docs/design_v7_8.md` | C-0 implementation evidence note (Line C) |
| `docs/workshop/mapping_current_guidance.md` | C-0 status row |
| `docs/workshop/sead_self_ai_track.md` | C-0 compact note |
| `docs/worklog.md` | Append-only C-0 line |
| `docs/tests/phase_m_c0_m4_atlas_protocol_oracle_results.md` | **New** — this report |

**No production mapping runtime, no default SimSession wiring, no semantic WGSL, no simthing-sim map awareness, no A-0/B-0/L3/FrontierV2-5/ACT-EVENT-OBS-PIPE implementation, no invariant changes.**

## Opus/product ruling summary

V7.8-MET-SCENARIO-ACCEPT-0 accepted all three M/E/T named scenarios. Product priority opened **C-0
only** (Line C / M atlas). A-0 and B-0 remain queued. VRAM budget set: **1.5 GiB default,
configurable, no architectural hard cap, multiplier reporting mandatory** (`V78AtlasVramBudget`).

## v7.8 Line C status

| Item | Status |
|---|---|
| Named scenario | `MultiTheaterAtlasMapping` — NamedScenarioAccepted |
| C-0 implementation | **Landed / Pending Opus Review** |
| `request_atlas_batching` | **Still rejected at admission** until C-0 accepted |
| `MappingExecutionProfile::default()` | **Disabled** |
| Production mapping runtime | **Not implemented** |
| M-6A active mask / M-5 source identity | **Deferred** |

## Pre-edit evaluation

| # | Question | Answer |
|---|---|---|
| 1 | What did Opus open? | C-0 only — first §11-gate M-4 atlas slice with full-tile protocol-oracle parity + VRAM report vs active budget. A-0/B-0 queued. |
| 2 | What is C-0 required to prove? | Multiple homogeneous square tiles packed into one atlas dispatch; algebraic G=0 isolation; full-tile parity vs exact per-tile-protocol CPU oracle; VRAM multiplier vs active budget. |
| 3 | What does full-tile protocol-oracle parity mean? | Every cell in every tile compared against `cpu_caller_managed_atlas_protocol` modeling seed clear, horizon hops, tile-local mask, and boundary behavior — not corridor probe alone. |
| 4 | Why corridor agreement alone is insufficient? | Remedial sandbox showed t44 can pass while full-tile L∞ diverges (~409) due to boundary/source_col semantics. |
| 5 | Preferred isolation policy? | Algebraic tile-local mask G=0 (flush-packed, 1.0× VRAM). |
| 6 | Physical gutter fallback? | G≥H when algebraic mask not admitted; 6.76× for 10×10 reference; 9.0× for C-0 8×8 fixture. |
| 7 | Active VRAM budget? | `V78AtlasVramBudget.max_bytes = 1_610_612_736` (1.5 GiB), configurable, no hard cap. |
| 8 | VRAM multiplier reporting? | Mandatory; occupancy checked against active budget via `build_c0_vram_budget_report`. |
| 9 | Rejected until C-0 passes? | `request_atlas_batching`, active mask, source identity, production runtime, default-on atlas. |
| 10 | Why not production mapping runtime? | C-0 is fixture/test-support proof only; session wiring and admission relaxation are separate gates post-Opus acceptance. |

## Atlas fixture shape

```text
tile_count = 4
tile_width = 8
tile_height = 8
horizon = 8
isolation = AlgebraicTileLocalMaskG0 (primary)
fallback = PhysicalGutterGteH (estimated)
atlas dimensions = 16 × 16
payload cells = 256
total atlas cells = 256
n_dims = 4
```

## GPU path summary

- Pack 4 homogeneous 8×8 tiles flush into 16×16 atlas buffer.
- Single `AtlasMaskGpuOp` atlas dispatch path with tile-local mask WGSL.
- Caller-managed protocol: hop-1 dispatch → per-tile seed clear → horizon hops (total ≥2 dispatches).
- **Not per-tile fake** — one packed atlas buffer, one atlas shader pipeline.

## CPU protocol oracle summary

`cpu_caller_managed_atlas_protocol` models:

- Per-tile seed placement (2×2 cluster per tile origin)
- Hop-1 stencil then per-tile seed-cell-only clear (not column-wide)
- Remaining horizon hops with `FlushTileLocalMask` + fixed denominator
- Same operator semantics as GPU (alpha=1.0, gamma=0.8, Normalized)

## Full-tile parity table

| Metric | Value | Acceptance |
|---|---|---|
| `full_tile_max_abs_error` | `0.000030517578` | PASS (≤ 0.0001 f32 tolerance) |
| `full_tile_l_inf` | `0.000030517578` | PASS |
| `cell_count_compared` | 256 | — |
| `corridor_t44` (non-authoritative) | `0.00000023841858` | diagnostic only |
| Classification | **GpuVerifiedApproximate** | not bit-exact; honest f32 tolerance |

## VRAM budget report

| Field | Value |
|---|---|
| `active_budget_bytes` | 1_610_612_736 |
| `active_budget_gib` | 1.5 |
| `budget_configurable` | true |
| `architectural_hard_cap` | false |
| `multiplier_reporting_required` | true |
| `algebraic_mask_multiplier` | 1.0 |
| `algebraic_mask_bytes` | 32_768 |
| `algebraic_mask_fits_active_budget` | true |
| `physical_gutter_multiplier` (8×8, G=H=8) | 9.0 |
| `physical_gutter_multiplier` (10×10 ref, G=H=8) | 6.76 |
| `physical_gutter_bytes` | 73_728 |
| `headroom_bytes` | 1_610_604_544 |
| `headroom_percent` | ~99.999% |

Occupancy checked against the **active** budget, not a frozen constant.

## Isolation policy report

| Policy | Status |
|---|---|
| AlgebraicTileLocalMaskG0 | **Primary path — tested** |
| PhysicalGutterGteH | **Fallback estimated/reported** |
| Active mask (M-6A) | **Not implemented** |
| Source identity (M-5) | **Not implemented** |

## Admission posture

- `request_atlas_batching: true` → compile preview **rejects** (test `c0_request_atlas_batching_still_rejected_until_gate_acceptance`).
- `MappingExecutionProfile::default()` → **Disabled**.
- C-0 proves first bounded M-4 atlas slice at fixture level; atlas remains opt-in/default-off pending Opus acceptance.

## Guardrail scans

| Scan | Expected | Result |
|---|---|---|
| `C-0\|M-4\|atlas\|V78AtlasVramBudget` in crates/docs | C-0 artifacts present | PASS |
| Active mask / source identity / M-6A / M-5 | deferred/rejected only | PASS |
| A-0 / B-0 / E-11B / D-2a | queued only in production track | PASS |
| ClauseThing / ClauseScript / L3 | parked only | PASS |
| FrontierV2-5 / ACT-5 / EVENT-3 / OBS-5 / PIPE-1 | rejection only | PASS |
| semantic WGSL / production mapping runtime / default SimSession | guardrail-only | PASS |
| simthing-sim map awareness | none added | PASS |
| scratch/tmp/log under docs/tests | none / delete | PASS — 0 found |

## Test results

```text
cargo test -p simthing-driver --test phase_m_c0_m4_atlas_protocol_oracle -- --nocapture
  test result: ok. 13 passed; 0 failed

cargo test -p simthing-spec --test v7_8_met_consumer_scenarios -- --nocapture
  test result: ok. 10 passed; 0 failed

cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission -- --nocapture
  test result: ok. 25 passed; 0 failed

cargo check --workspace
  Finished dev profile (pre-existing warnings only)
```

## Replay fingerprint

| Hash | Value |
|---|---|
| `protocol_oracle_hash` | (internal FNV of oracle output) |
| `gpu_output_hash` | (internal FNV of GPU output) |
| **combined_fingerprint** | **`a974fe44e20620f3`** |

## Transient cleanup result

No scratch/tmp/log artifacts under `docs/tests/` required deletion.

## Final verdict

**PASS (Pending Opus Review)** — C-0 landed the first §11-gate M-4 atlas slice: real atlas-packed
homogeneous-square tile dispatch with algebraic tile-local mask G=0, full-tile protocol-oracle parity
(max abs error 3.05e-5, classified GpuVerifiedApproximate), VRAM-multiplier report against the
active configurable V78AtlasVramBudget, and fallback gutter estimate. It did not implement
production mapping runtime, default SimSession wiring, active-mask/source-identity, A-0, B-0,
ClauseThing/L3, FrontierV2-5, ACT/EVENT/OBS/PIPE, semantic WGSL, or simthing-sim map awareness.
C-0 is implemented and **pending Opus/design-authority review** for acceptance.
