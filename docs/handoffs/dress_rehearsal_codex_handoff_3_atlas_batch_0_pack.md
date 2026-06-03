# Codex/Cursor Handoff 3 — `ATLAS-BATCH-0-PACK` Implementation Contract

**Recipient model: Cursor**
**Role: production implementation agent**

**From:** Opus (design authority) · **Date:** 2026-06-03 · **Gate:** authored + accepted.
**Predecessors:** `ATLAS-BATCH-0-GEN` and `ATLAS-BATCH-0-LOC` are **closed / PASS**. PACK consumes LOC.

> **DESIGN-AUTHORITY RULING — PACK is CPU-only this pass.** PACK builds the **pack-plan descriptor + the
> `G=0` algebraic-mask CPU oracle + the numeric VRAM report**. It does **NOT** run GPU dispatch. The
> batched GPU dispatch + GPU=CPU bit-exact parity is a **separate later slice (`ATLAS-BATCH-0-PACK-GPU`)**.
> EC-A2 is therefore split: **EC-A2a** (pack plan + `G=0` algebra + VRAM budget — PACK proves) and
> **EC-A2b** (batched dispatch + bit-exact parity — PACK-GPU, **deferred**). **PACK may NOT claim EC-A2b /
> "batched dispatch + CPU parity."** Saying it did is a false closure.

---

## 1. Harness

**Fixed base (cite on handoff back):**
1. `docs/design_0_0_8_0.md` §0 — transient constitution (conformance; conflict-as-resource-flow; recursive
   allocation; SEAD = GPU threshold crossings, not CPU planner; §0.5 two-layer harness).
2. `docs/invariants.md` — Scenario Proof; AccumulatorOp v2; Resource Flow Substrate; Mapping invariants
   (G=0 algebraic tile-local mask; ping-pong/CPU-oracle parity posture; "index arithmetic has one home");
   semantic-free `simthing-sim`.
3. `docs/design_0_0_8_0_consumer_pulled_production_track.md` §12–§12.5 — ATLAS-BATCH-0 ladder; GEN/LOC
   closure; PACK = this gate; OWNER routing; retirement map.
4. `docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md` — 20×20/13-system topology; §4.1 galactic heatmap.
5. `crates/simthing-core/src/accumulator_op.rs` — Accumulator vocabulary (reference only; do not edit).
6. `docs/workshop/sead_self_ai_track.md` — SEAD charter.

**Rung-local (ephemeral — this rung only):** `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_loc.rs`;
`crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_gen.rs`;
`docs/tests/scenario_0080_2_atlas_batch_0_loc_report.md`. **Do not grow the fixed base.**

## 2. Established decisions — DO NOT re-derive (Opus-locked)

- **CPU-only, fixture-only, deterministic.** No GPU, no `simthing-gpu`/`simthing-core`/`simthing-sim`
  coupling, no live SimThings, no numeric columns, no economy, no SEAD. Mirrors GEN/LOC posture + a
  packing algebra and a CPU oracle.
- **Input:** `pack.rs` includes LOC via `#[path = "dress_rehearsal_atlas_batch_0_loc.rs"] mod loc;` and
  consumes `loc::LocationMaterialization::canonical()`. **No `lib.rs` export.** GEN/LOC sources untouched.
- **Three homogeneous tile classes, kept SEPARATE** (even though two share 10×10), because role + channel
  sets differ and STORE needs the distinction: `Galactic20x20` (1 tile), `StarSystem10x10` (13),
  `PlanetSurface10x10` (13). Every packed tile preserves its **source `LocationId` + role**.
- **`G=0` mask = algebraic, tile-local, numeric-only.** No inter-tile bleed; no semantic WGSL; no
  map/faction/gameplay branching; the oracle sees only dims/indices/mask/tile-bounds. Out-of-tile samples
  resolve to **0**. PACK implements the **descriptor + a CPU oracle** proving the no-bleed property over a
  dummy generic field — **no GPU dispatch.**
- **Single transform home.** One pair of functions `pack_coord(plan, location_id, x, y) -> (ax, ay)` and
  `unpack_coord(plan, ax, ay) -> (location_id, x, y)` — all coordinate mapping routes through them
  (mirrors the single-indexing-home rule). No ad-hoc atlas arithmetic elsewhere.
- **VRAM report is numeric, derived from channel count** — not a fixed hand-wave. `bytes_per_cell` per
  class = `channel_count(class) * 4` (f32/channel, from LOC channel descriptors). Report padding overhead
  (atlas-rectangle cells − used tile cells) → `vram_multiplier`; budget `V78AtlasVramBudget`; `budget_pass`
  numeric.
- **PACK preserves STORE metadata, implements no STORE behavior:** keep source `LocationId`, role, tile
  origin, tile-local↔atlas transforms, per-class channels, owner-indexed channel descriptors. Do **not**
  implement owner masked reduction, per-channel accumulation, child-result storage, or occupant reduction.
- **No R1/R4:** no diffusion, gradient, exact sqrt, SEAD, threshold, or `BoundaryRequest`. PACK produces
  the packed spatial substrate those rungs consume.

## 3. Deliverables

`crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_pack.rs`:
- `TileClassDescriptor { class_id, role, tile_width, tile_height, channels: ChannelSet, source_location_ids: Vec<LocationId> }`
- `PackedTile { source_location_id, source_role, class_id, atlas_origin: (u32,u32), tile_dims: (u32,u32) }`
- `GZeroMaskDescriptor` — tile bounds + the algebraic out-of-tile→0 rule.
- `VramReport { unpacked_cell_count, packed_cell_count, mask_or_gutter_overhead_cells,
  bytes_per_cell_assumption, unpacked_bytes_estimate, packed_bytes_estimate, vram_multiplier,
  budget_name: "V78AtlasVramBudget", budget_pass: bool }`
- `AtlasBatchPlan { classes: Vec<TileClassDescriptor>, tiles: Vec<PackedTile>, mask: GZeroMaskDescriptor,
  vram: VramReport, total_packed_cells }` with `from_materialization(&LocationMaterialization)` + `canonical()`.
- `pack_coord` / `unpack_coord` (the sole transform home) + a CPU `g_zero_sample(plan, ax, ay, neighbor, field) -> f32`
  oracle (in-tile → field value; crossing the tile boundary → 0).
- `STATUS_PASS` const guarded by `docs_status_matches_gate`. The const must state **EC-A2a only; EC-A2b deferred.**

A deterministic packing strategy of your choice (e.g., row-major tiles into the smallest enclosing
rectangle per class); document it; the multiplier follows from it.

## 4. Files Cursor MAY create / edit

- create `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_pack.rs`
- create `crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_pack.rs`
- create `docs/tests/scenario_0080_2_atlas_batch_0_pack_report.md`, `…_status_row.md`,
  `…_cargo_test_2026_06_03.txt`, and (optional, only if it adds visibility) `…_vram_report_2026_06_03.txt`
- update **after green** `docs/design_0_0_8_0_consumer_pulled_production_track.md`, `docs/worklog.md`

## 5. Files Cursor MUST NOT edit

`docs/design_0_0_8_0.md`; `docs/invariants.md`; `…/dress_rehearsal_atlas_batch_0_gen.rs`;
`…/dress_rehearsal_atlas_batch_0_loc.rs` (and the accepted GEN/LOC test/report/log artifacts);
`crates/simthing-driver/src/lib.rs`; **`crates/simthing-sim/**`, `crates/simthing-gpu/**`,
`crates/simthing-core/**`** (GPU is **not** in scope this pass — touching gpu/core is a stop condition).

## 6. Tests Cursor MUST add (target `dress_rehearsal_atlas_batch_0_pack`)

1. `pack_plan_is_deterministic` — same LOC materialization → identical `AtlasBatchPlan`.
2. `locations_group_into_expected_tile_classes` — exactly 3 classes: 1×`Galactic20x20`, 13×`StarSystem10x10`,
   13×`PlanetSurface10x10`, kept separate; each tile retains source `LocationId` + role.
3. `tile_origins_are_contiguous_and_non_overlapping` — within each class atlas, tiles fit and never overlap.
4. `tile_local_coordinates_round_trip` — `pack_coord` then `unpack_coord` is identity for sampled cells.
5. `g_zero_mask_blocks_inter_tile_bleed` — CPU oracle: a seeded field where a sample crossing a tile
   boundary contributes **0**; in-tile samples pass through.
6. `vram_multiplier_report_is_numeric_and_budgeted` — every report field numeric; `budget_pass` computed,
   not asserted by fiat.
7. `channel_metadata_survives_pack` — LOC channel descriptors preserved per class; **no numeric columns**.
8. `owner_metadata_survives_pack_without_owner_runtime` — owner-indexed descriptors survive; no masked-reduction.
9. `docs_status_matches_gate` — `STATUS_PASS` matches the gate id and claims **EC-A2a only**.

**Do NOT add a GPU/dispatch parity test** (`batched_dispatch_matches_cpu_oracle_bit_exactly` is PACK-GPU).
No test imports `simthing-gpu`/`simthing-sim`/`simthing-core`.

## 7. Raw evidence

Run + capture: `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_pack 2>&1 | tee docs/tests/scenario_0080_2_atlas_batch_0_pack_cargo_test_2026_06_03.txt`
**Run to green before any PASS claim** ("execution pending" is not acceptance). If it fails: keep the
diagnostic log, do **not** mark PASS, do **not** close the production doc, return to Opus with the failure.

## 8. Docs update (AFTER green only)

`docs/design_0_0_8_0_consumer_pulled_production_track.md`: mark `ATLAS-BATCH-0-PACK` **PASS for EC-A2a
(pack plan + G=0 CPU oracle + VRAM) only**; link source/test/report/raw-log; include the **numeric VRAM
multiplier**; **state explicitly that batched GPU dispatch + CPU parity (EC-A2b) is DEFERRED to
`ATLAS-BATCH-0-PACK-GPU` and NOT proven**; STORE unimplemented; M-4A sparse-residency scheduler parked;
REENROLL parked; no economy/disruption/SEAD implemented. One `docs/worklog.md` entry.

## 9. Stale artifacts

Delete only superseded/duplicate **PACK** raw logs after a clean final green log exists (and not if a
report references them as historical evidence). **Never** delete accepted GEN/LOC artifacts. If none:
"Deleted obsolete artifacts: none found."

## 10. Stop conditions — STOP and escalate to Opus if implementation seems to need

GPU dispatch / `simthing-gpu` or `simthing-core` edits · semantic WGSL · runtime `match kind` · CPU map
planner · economy/disruption behavior · owner masked-reduction runtime · STORE / child-result storage ·
R1 heatmap/diffusion · R4 SEAD/gradient/exact-sqrt/threshold/`BoundaryRequest` · REENROLL/movement ·
combat · capability-tree mask-down · sparse-residency scheduler / M-4A · default session wiring ·
`simthing-sim` semantics. **PACK is generic atlas batch *planning* + CPU parity, not gameplay and not GPU.**

## 11. Handoff-back format (Cursor → Opus)

```
Recipient model: Opus  ·  Role: design authority
ATLAS-BATCH-0-PACK implemented (EC-A2a; EC-A2b deferred to PACK-GPU).
Raw test log: docs/tests/scenario_0080_2_atlas_batch_0_pack_cargo_test_2026_06_03.txt  (N passed; 0 failed)
VRAM multiplier: <numeric>, budget_pass=<bool>
Deliverables / deviations / open questions: <...>
§0.5 self-check: holds 1–6 — <one line>
```

## 12. §0.5 self-check (this contract)

Holds 1–6: PACK is generic atlas batch-planning + a CPU `G=0` parity oracle — structure later GPU-resident
rungs consume, never a CPU planner or map engine; `Location`/tile classes are descriptor roles, not runtime
kind-branches; no resource-flow behavior, no allocation outside the recursive tree, no `simthing-sim`
semantics, no default wiring. The honest EC-A2a/EC-A2b split prevents a numeric-only closure from
masquerading as a proven GPU dispatch.
