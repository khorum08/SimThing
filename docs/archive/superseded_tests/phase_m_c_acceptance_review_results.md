# C-ACCEPT-0 — Design-Authority/Product Ruling: Accept C-0/C-1; Open C-2

**Reviewer:** Opus 4.8 (design authority — v7.8 track) + project-owner product direction
("close out map batching first"). **Date:** 2026-05-30.
**Decision:** **ACCEPT C-0 and C-1 (Option A).** Open **C-2 — atlas admission relaxation** for the
proven algebraic tile-local-mask G=0 profile. Atlas stays opt-in/default-off; no production mapping
runtime, default `SimSession` wiring, or default-on atlas is authorized. A-0/B-0 stay queued; L3
parked; FrontierV2-5 and ACT/EVENT/OBS/PIPE unauthorized.

## Reviewed — code + tests, not only reports

- `crates/simthing-gpu/src/atlas_mask.rs` + `shaders/structured_field_stencil_atlas_mask.wgsl` —
  confirmed a **real packed-atlas path**: `build_flush_atlas` packs `tile_count` tiles flush into one
  `aw×ah` atlas buffer; one `@compute atlas_mask_stencil_step` pipeline with tile-local-mask bounds
  (`neighbor_valid`/`sample_neighbor` in tile-local coords). `AlgebraicTileLocalMaskG0` is the primary
  tested isolation; `PerTileStandalone` is explicitly *not* the acceptance oracle; gutter is
  estimated/reported only. WGSL is semantic-free (flat buffers/indices/masks/tile metadata).
- `crates/simthing-driver/tests/support/c0_atlas_protocol_oracle.rs` — `cpu_caller_managed_atlas_protocol`
  models seed placement, hop-1 + per-tile seed-cell-only clear, horizon hops with tile-local mask, same
  operator semantics as GPU. Acceptance is **full-tile** parity (256 cells), not corridor-only.
- `crates/simthing-driver/tests/support/c1_atlas_scale_model.rs` — constants match the handoff envelope
  exactly; `total_dense_cells_if_all_resident() == 7_230_000`; algebraic `925_440_000` bytes; gutter
  6.76×; budget comparison against the active configurable `V78AtlasVramBudget`.
- `crates/simthing-sim/src/**` — map-awareness scan **empty** (no atlas/RegionCell/source_mask/M-4).

## Verification

| Command | Result |
|---|---|
| `cargo test -p simthing-driver --test phase_m_c0_m4_atlas_protocol_oracle` | **13/13 PASS** (real GPU atlas dispatch ran) |
| `cargo test -p simthing-driver --test phase_m_c1_atlas_scale_model` | **10/10 PASS** |
| `cargo check --workspace` | **PASS** (pre-existing `simthing-driver` unused-import warning only) |

## Review answers

| # | Question | Finding |
|---|---|---|
| 1 | C-0 satisfies the §11 M-4 first-slice gate? | **Yes** — real atlas-packed homogeneous-square dispatch, algebraic G=0 primary, full-tile protocol-oracle parity. |
| 2 | Is `GpuVerifiedApproximate` honest? | **Yes** — `full_tile_max_abs_error = 3.05e-5 (2⁻¹⁵) ≤ 1e-4` field tolerance; not bit-exact, and not claimed to be. The mapping field is `source_capped_normalized` (approximate by design); this is the correct class. |
| 3 | Full-tile protocol oracle sufficient? | **Yes** — every cell vs a CPU model of the *same* atlas protocol (seed-clear + horizon hops + tile-local mask + boundary). |
| 4 | Corridor-only correctly excluded? | **Yes** — `corridor_t44` recorded as diagnostic-only; acceptance is full-tile L∞. |
| 5 | C-1 models the 2000-star envelope? | **Yes** — 200×150 + 2000×10×10 + 10,000×10×10 + 60,000×10×10 = **7,230,000** cells; arithmetic verified in tests. |
| 6 | Scale model supports algebraic-mask-first? | **Yes** — algebraic ≈ 0.862 GiB fits the 1.5 GiB default; gutter ≈ 5.826 GiB does not. |
| 7 | Gutter fallback needs raised budget? | **Yes** — ≈ 5.826 GiB; viable only on a raised active budget/profile (headless/dedicated/big-VRAM). |
| 8 | Accept C-0 and C-1 together? | **Yes.** |
| 9 | `request_atlas_batching` stays rejected until a later gate? | **Until C-2.** C-0 acceptance satisfies the invariant's "gate-passing M-4 PR" condition; C-2 implements the bounded admission relaxation. Blanket rejection lifts only through C-2's scope. |
| 10 | Next C gate? | **C-2 = atlas admission relaxation** (Option A) — designer/spec admission accepts bounded algebraic-G=0 atlas specs. The sparse-residency/cadence **runtime** (C-1's flagged need at true scale) is a **separate later production gate**, not C-2. |
| 11 | Docs over/understating atlas? | Corrected here: Line C → C-0/C-1 ACCEPTED, C-2 OPEN; atlas still opt-in/default-off/admission-gated. |

## C-2 — opened gate (bounded scope)

| Step | Scope | Status |
|---|---|---|
| **C-2** | **Atlas admission relaxation (designer/spec layer).** `simthing-spec` admission may accept a bounded Line C atlas spec **only** when it is: homogeneous-square tiles, **algebraic tile-local mask G=0**, protocol-oracle-backed, **fits the active `V78AtlasVramBudget`** (footprint computed via the C-1 model; over-budget specs rejected at import — raise the budget/profile or declare sparse residency), and carries **mandatory VRAM-multiplier reporting**. `request_atlas_batching` is relaxed **only** through this scope. **No production runtime, no default `SimSession` wiring, no default-on atlas, no scheduler/cache.** Physical-gutter and active-mask/source-identity specs stay rejected. | **OPEN** |

C-2 is the **designer-facing closure** of map batching: proof (C-0) + scale model (C-1) + admission
(C-2) make atlas an authorable, guardrail-at-the-spec-layer capability. The atlas **production
runtime / sparse-residency scheduler** (C-1's noted requirement for a live 2000-star game) is a
**separate, later, explicitly-gated** track — *not* part of this map-batching closure and not opened
here. C-2 is opened for implementation; this review does **not** implement it.

## Guardrail confirmations (no authorization)

Production mapping runtime, default `SimSession` wiring, default-on atlas, active-mask halo (M-6A),
source identity / `source_mask` (M-5), A-0/E-11B/E-11B-5, B-0/D-2/D-2a, ClauseThing/ClauseScript/L3,
FrontierV2-5, ACT-5/EVENT-3/OBS-5/PIPE-1, semantic/map-specific WGSL, `simthing-sim` map awareness,
CPU planner/urgency/commitment — **all remain unauthorized**. No invariant change is made in this
pass; C-0 acceptance operates within the existing "atlas requires gate-passing M-4 PR" invariant, and
any invariant update accompanies C-2 under its own Tier-2 cadence.

## Ruling

**ACCEPT C-0/C-1.** C-0 is the first §11-gate M-4 atlas slice — real atlas-packed GPU path, algebraic
G=0, full-tile protocol-oracle parity (`GpuVerifiedApproximate`, 3.05e-5 ≤ 1e-4), VRAM report against
the active configurable budget. C-1 validates the 2000-star envelope: algebraic G=0 (≈0.862 GiB) fits
the 1.5 GiB default; gutter (≈5.826 GiB) needs a raised profile. **Open C-2 (atlas admission
relaxation, algebraic-G=0 only).** Atlas remains opt-in/default-off; production runtime is a separate
later gate. v7.8 constitution / production-track split intact.
