# Codex/Cursor Handoff 4 — `ATLAS-BATCH-0-PACK-GPU` Contract (with EC-A2b design correction)

**Recipient model: Cursor**
**Role: production implementation agent**

**From:** Opus (design authority) · **Date:** 2026-06-03 · **Gate:** authored + accepted **with one
EC-A2b redefinition (below).**

> ## DESIGN-AUTHORITY RULING — EC-A2b parity standard corrected
> The literal EC-A2b ("bit-exact `f32::to_bits()`") is the **wrong standard** and is **not** how this
> substrate's parity works. Findings (verified in code):
> - **The GPU primitive already exists** — `crates/simthing-gpu/src/atlas_mask.rs`: `AtlasMaskGpuOp`
>   (single-atlas `dispatch_once`) with **`AtlasIsolationMode::TileLocalMaskG0`** (algebraic tile-local
>   `G=0`) + the matching CPU oracle `cpu_caller_managed_atlas_protocol` / `cpu_atlas_horizon`.
>   Semantic-free, fixture-support, not production-wired. **Obstacle 1 (no batched G=0 dispatch) is
>   resolved — no new GPU code is needed.**
> - **Parity is `GpuVerified` (L∞ tolerance), not bit-exact.** Every atlas/stencil parity in the repo
>   uses L∞ max-error (`max_full_tile_error`, `< 1e-4`). `invariants.md` reserves `to_bits()` bit-exact
>   for **`ExactDeterministic`** (pinned/algebraic) ops; an f32 diffusion stencil is **`GpuVerified`**
>   (GPU/CPU differ in last bits via FMA/reorder). Demanding `to_bits()` here would assert a false
>   `ExactDeterministic` classification.
>
> **Ruling:** **EC-A2b = one batched GPU dispatch per homogeneous tile class (`AtlasMaskGpuOp`,
> `TileLocalMaskG0`) matches the CPU oracle within the established `GpuVerified` tolerance (full-tile
> L∞ ≤ 1e-4), with `G=0` tile-local no-bleed proven.** True bit-exact `to_bits()` parity is split out as
> **`EC-A2b-exact` (DEFERRED)** — it requires a pinned fixed-point stencil (a separate exact-arithmetic
> track), not the f32 path. PACK-GPU proves EC-A2b (GpuVerified); it may **not** claim bit-exact / EC-A2b-exact.

---

## 1. Harness

**Fixed base (cite on handoff back):** 1) `docs/design_0_0_8_0.md` §0 · 2) `docs/invariants.md`
(Scenario Proof; **`GpuVerified` vs `ExactDeterministic` parity classes**; Mapping invariants incl. the
C-2 algebraic tile-local `G=0` mask + mandatory VRAM-multiplier; semantic-free `simthing-sim`) ·
3) `docs/design_0_0_8_0_consumer_pulled_production_track.md` §12–§12.5 (GEN/LOC/PACK closure; EC-A2
split; this gate) · 4) `docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md` · 5)
`crates/simthing-core/src/accumulator_op.rs` (reference only) · 6) `docs/workshop/sead_self_ai_track.md`.

**Rung-local (ephemeral):** `crates/simthing-gpu/src/atlas_mask.rs` (the GPU primitive + CPU oracle —
read/call only, **do not edit**); `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_pack.rs`
(the accepted plan); `docs/tests/scenario_0080_2_atlas_batch_0_pack_vram_report_2026_06_03.txt`;
`crates/simthing-driver/tests/phase_m_c0_m4_atlas_protocol_oracle.rs` (existing usage example of the
atlas-mask API). **Do not grow the fixed base.**

## 2. Established decisions — DO NOT re-derive (Opus-locked)

- **Use the existing GPU primitive; write NO new GPU code / WGSL.** Cursor **calls** (does not edit) the
  public `simthing-gpu` `atlas_mask` API: `AtlasMaskGpuOp::{new, upload, dispatch_once, readback}` (or
  `gpu_caller_managed_atlas_protocol`), `cpu_caller_managed_atlas_protocol` / `cpu_atlas_horizon`,
  `make_atlas_mask_params(.., use_tile_local_mask = true)`, `AtlasIsolationMode::TileLocalMaskG0`,
  `atlas_config` / `atlas_dims` / `C0AtlasFixtureShape`, the L∞ helpers `max_full_tile_error` /
  `full_tile_l_inf`, and `simthing_gpu::GpuContext`. Confirm exact export paths from the crate. **If the
  job appears to need new GPU code, STOP and escalate** (that's a separate GPU-primitive gate).
- **Dispatch shape:** **one dispatch per homogeneous tile class** (`dispatch_once` per class) — confirmed
  per §3.2. Three classes, all in scope (§3.3): `Galactic20x20` (1 tile, 20×20, 5 ch), `StarSystem10x10`
  (13 tiles, 10×10, 2 ch), `PlanetSurface10x10` (13 tiles, 10×10, 2 ch).
- **Field:** a **generic scalar field** seeded per class (e.g., `seed_cluster`); prove GPU==oracle over
  it. Separately assert the PACK plan's **channel descriptors are preserved** (metadata test) — do **not**
  materialize gameplay numeric columns (that's STORE).
- **Parity standard:** `GpuVerified`, **full-tile L∞ ≤ 1e-4** via `max_full_tile_error`/`full_tile_l_inf`.
  **Not** `to_bits()`. Corridor/t44 metric is **non-authoritative** (diagnostic only). The
  `cpu_caller_managed_atlas_protocol` oracle is the reference.
- **`G=0` proof:** the tile-local mask makes each tile independent — out-of-tile / cross-tile-boundary /
  out-of-atlas neighbor samples contribute **0**. Prove via full-tile L∞ over the masked dispatch.
  No signed-offset oracle change needed (the atlas-mask oracle already handles boundaries); keep any
  helper **PACK-GPU-local** — do **not** amend PACK.
- **GPU availability — two-tier (§3.8):** the GPU parity test is gated by `SIMTHING_RUN_GPU_TESTS=1`.
  **Skipped/ignored ≠ PASS.** EC-A2b closes only when the **raw saved log shows the GPU test actually
  ran** on an adapter. CPU-oracle/metadata tests always run.
- **No performance claims** unless timestamp-query-backed; a correctness parity report suffices. Any
  timing is diagnostic.
- **Fixture-only:** `pack_gpu.rs` includes PACK via `#[path = "dress_rehearsal_atlas_batch_0_pack.rs"] mod pack;`
  (which chains LOC→GEN). **No `lib.rs` export.**

## 3. Deliverables

- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_pack_gpu.rs` — consumes
  `pack::AtlasBatchPlan::canonical()`, maps each tile class to an `AtlasMaskGpuOp` dispatch + the CPU
  oracle, exposes a `STATUS_PASS` const stating **EC-A2b (GpuVerified); EC-A2b-exact deferred**.
- `crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_pack_gpu.rs` (tests §4).
- `docs/tests/scenario_0080_2_atlas_batch_0_pack_gpu_report.md`, `…_status_row.md`,
  `…_cargo_test_2026_06_03.txt`, `…_parity_2026_06_03.txt` (adapter name; per-class tile counts; output
  element count; full-tile L∞ per class; any skipped tests; whether EC-A2b actually closed).

## 4. Tests (target `dress_rehearsal_atlas_batch_0_pack_gpu`)

CPU/metadata (always run): 1) `pack_gpu_status_matches_gate` (id = `ATLAS-BATCH-0-PACK-GPU`; claims
EC-A2b GpuVerified only; not bit-exact, not STORE/R1/R4) · 2) `gpu_fixture_uses_accepted_pack_plan`
(consumes `AtlasBatchPlan::canonical()`; PACK metadata preserved) · 3) `channel_metadata_survives`
(class channel descriptors carried; no numeric columns) · 4) `no_semantic_shader_inputs` (the atlas-mask
params are generic dims/buffers/mask only — no map/faction/gameplay inputs).

GPU-gated (`SIMTHING_RUN_GPU_TESTS=1`; skip ≠ PASS): 5) `gpu_oracle_parity_galactic_20x20`
· 6) `gpu_oracle_parity_star_system_10x10_batch` (one `dispatch_once` over all 13 tiles) ·
7) `gpu_oracle_parity_planet_surface_10x10_batch` — each asserts **full-tile L∞ ≤ 1e-4** vs
`cpu_caller_managed_atlas_protocol` · 8) `g_zero_blocks_cross_tile_and_out_of_atlas` (masked dispatch:
cross-tile and out-of-atlas neighbor contributions resolve to 0; full-tile L∞ holds). **No `to_bits()`
test.** No import edits to `simthing-gpu`/`simthing-sim`/`simthing-core`.

## 5. Files Cursor MAY edit / MUST NOT edit

**MAY create/edit:** the four `pack_gpu` files above; **after green only** —
`docs/design_0_0_8_0_consumer_pulled_production_track.md`, `docs/worklog.md`.
**MUST NOT edit:** `docs/design_0_0_8_0.md`; `docs/invariants.md`; `…/dress_rehearsal_atlas_batch_0_{gen,loc,pack}.rs`
and the accepted GEN/LOC/PACK artifacts; `crates/simthing-driver/src/lib.rs`; **`crates/simthing-gpu/**`
(call, never edit — no new WGSL/GPU code), `crates/simthing-core/**`, `crates/simthing-sim/**`.**

## 6. Raw evidence

PowerShell (record the command in the log):
`$env:SIMTHING_RUN_GPU_TESTS=1; cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_pack_gpu -- --nocapture *>&1 | Tee-Object docs/tests/scenario_0080_2_atlas_batch_0_pack_gpu_cargo_test_2026_06_03.txt`
**Run to green with the GPU tier actually executing** (the raw log must show tests 5–8 ran, not
"ignored"). If no GPU adapter: keep the diagnostic log, **do not mark PASS**, do not close the production
doc, return to Opus as **blocked**.

## 7. Docs update (AFTER green, GPU tier actually ran)

`docs/design_0_0_8_0_consumer_pulled_production_track.md`: mark **`ATLAS-BATCH-0-PACK-GPU` PASS for
EC-A2b (GpuVerified, full-tile L∞ ≤ 1e-4)**; list the three proven classes + adapter; link
source/test/report/raw-log/parity; **state EC-A2b-exact (bit-exact) remains DEFERRED (needs a pinned
fixed-point stencil)**; STORE unimplemented; M-4A sparse-residency scheduler parked; REENROLL parked; no
economy/disruption/SEAD/owner-masked-reduction implemented. One `docs/worklog.md` entry.

## 8. Stale artifacts

Delete only superseded duplicate PACK-GPU logs after a clean final GPU-enabled green log exists. Never
delete accepted GEN/LOC/PACK artifacts. If none: "Deleted obsolete artifacts: none found."

## 9. Stop conditions — STOP and escalate if it seems to need

new WGSL or any `simthing-gpu`/`-core`/`-sim` edit · STORE / child-result storage · owner masked-reduction
runtime · economy/disruption · R1 diffusion beyond the generic stencil proof · R4 SEAD/gradient/exact-sqrt/
threshold/`BoundaryRequest` · REENROLL/movement · combat · capability-tree mask-down · M-4A sparse-residency
scheduler · default session wiring · runtime `match kind` · CPU map planner · a `to_bits()` bit-exact claim.

## 10. Handoff-back format (Cursor → Opus)

```
Recipient model: Opus  ·  Role: design authority
ATLAS-BATCH-0-PACK-GPU implemented (EC-A2b GpuVerified; EC-A2b-exact deferred).
GPU tier ran: yes/no  ·  adapter: <name>
Raw log: docs/tests/scenario_0080_2_atlas_batch_0_pack_gpu_cargo_test_2026_06_03.txt  (N passed; 0 failed)
Per-class full-tile L∞: galactic=<>, star_system=<>, planet_surface=<>  (all ≤ 1e-4)
Deliverables / deviations / open questions: <...>
§0.5 self-check: holds 1–6 — <one line>
```

## 11. §0.5 self-check (this contract)

Holds 1–6: generic atlas/stencil GPU proof of the packed substrate via the existing semantic-free
`AtlasMaskGpuOp` (no new GPU code, no `match kind`, no `simthing-sim` semantics, no default wiring, no
allocation beyond the recursive tree, no CPU planner). The EC-A2b→GpuVerified correction + EC-A2b-exact
deferral keeps the parity classification honest (no f32 tolerance masquerading as bit-exact
`ExactDeterministic`), and keeps EC-A2b separate from EC-A2a so CPU-only evidence cannot pose as GPU proof.
