# Codex/Cursor Handoff 5 — `ATLAS-BATCH-0-STORE` Implementation Contract

**Recipient model: Cursor**
**Role: production implementation agent**

**From:** Opus (design authority) · **Date:** 2026-06-03 · **Gate:** authored + accepted.
**Predecessors:** GEN / LOC / PACK (EC-A2a) / PACK-GPU (EC-A2b GpuVerified) all closed / PASS.

> ## DESIGN-AUTHORITY RULINGS (locked before this contract)
> **(1) PACK-GPU review note → Option A (accept as fixture-local caller glue).** The `..._pack_origins`
> caller-managed helper is **accepted** — it uses only the existing `AtlasMaskGpuOp` + existing WGSL,
> stays inside `simthing-driver` fixture code, has real GPU evidence, and edits no `simthing-gpu`. **No
> remedial.** If a *production* `simthing-gpu` path ever needs row-major atlas origins, that is a
> separate GPU-primitive gate — not now, not STORE. STORE inherits a clean substrate.
>
> **(2) STORE scope → CPU descriptor + CPU oracle only.** STORE proves the **storage shape** for
> EC-A3: child/occupant flow results land in the correct `(location, cell, channel, owner)` slots,
> co-located occupants preserved, never blind-summed by position. The **GPU masked-reduction parity**
> (live `EvalEML` `CMP_EQ` + `Sum` over owner-indexed columns) is split out as a **separate deferred
> slice `ATLAS-BATCH-0-STORE-GPU`** — it pulls the AccumulatorOp GPU runtime and is its own gate.
> **STORE must NOT run a live OWNER masked reduction; it must NOT claim GPU closure.** The OWNER
> masked-reduction **runtime stays parked** until STORE-GPU / R3. (CPU oracle now = the reference the
> later GPU slice checks against — same pattern as PACK → PACK-GPU.)

---

## 1. Harness

**Fixed base (cite on handoff back):** 1) `docs/design_0_0_8_0.md` §0 · 2) `docs/invariants.md`
(Scenario Proof; AccumulatorOp v2; **`GpuVerified` vs `ExactDeterministic`**; "index arithmetic has one
home"; semantic-free `simthing-sim`) · 3) `docs/design_0_0_8_0_consumer_pulled_production_track.md`
§12–§12.5 (ladder; GEN/LOC/PACK/PACK-GPU closure; STORE = this gate; OWNER routing §12.4) · 4)
`docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md` · 5) `crates/simthing-core/src/accumulator_op.rs`
(reference only — STORE does **not** call it) · 6) `docs/workshop/field_policy_track.md`.

**Rung-local (ephemeral):** `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_loc.rs`
(`LocationMaterialization`, `OccupantPlacement`, `ChannelSet`, `Mobility`, `cell_index`);
`crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_pack.rs` (`AtlasBatchPlan`, `pack_coord`,
`unpack_coord`). **Do not grow the fixed base.**

## 2. Established decisions — DO NOT re-derive (Opus-locked)

- **CPU-only, fixture-only, deterministic.** No GPU, no `simthing-gpu`/`-core`/`-sim`. No live masked
  reduction, no AccumulatorOp/EvalEML calls. STORE builds a CPU oracle table + correctness proofs.
- **Input:** `store.rs` includes PACK via `#[path = "dress_rehearsal_atlas_batch_0_pack.rs"] mod pack;`
  (chains LOC→GEN). Consume `LocationMaterialization::canonical()` (occupants) + `AtlasBatchPlan::canonical()`
  (atlas coords). **No `lib.rs` export.** Do **not** edit GEN/LOC/PACK/PACK-GPU sources.
- **`ChildContribution` is generic storage data, not gameplay:**
  ```
  ChildContribution { source_occupant_id, location_id, cell: GridCell, owner: Owner,
                      channel: <a channel from the occupant's LOC ChannelSet>, value: f32 }
  ```
  `value` is a **deterministic generic fixture seed** (e.g., a stable per-occupant number) — **not** a
  gameplay-computed quantity. STORE applies **no recipes/rates** (labor=10/tick etc. is R2). It proves
  storage/aggregation *shape*, not value computation.
- **Channels are the occupant's LOC-declared `ChannelSet`** (preserve them; invent no new gameplay
  columns). Representative occupant→channel mapping (using LOC channels): PopCohort→`Labor`;
  FactoryDistrict→`Production`; Starport→`ProductionPassThrough`; PatrolFleet→`PatrolPresence` +
  `FleetStrength(Terran)`; PirateFleet→`PiratePresence` + `FleetStrength(Pirate)`; Planet→
  `ProductionPassThrough` bridge or no numeric contribution. Owner copied from the LOC occupant.
- **Dense target keyed by LOC `(location_id, cell_index)`** — via the LOC **single indexing home**
  `cell_index(map_base, width, x, y)`. **No ad-hoc indexing.** Aggregation key = `(location_id,
  cell_index, channel, owner)`; values aggregate per key (sum within a key), **never blind-summed by
  position across channels/owners**.
- **PACK round-trip is proven, not used as the primary key:** STORE proves `pack_coord`/`unpack_coord`
  map the LOC `(location, x, y)` target to a PACK atlas coordinate and back without loss. LOC cell is
  the canonical home; PACK coord is proven equivalent.
- **Owner boundary:** STORE preserves owner-indexed keys in the CPU oracle table; it does **not** run a
  live masked reduction. State plainly: *"STORE proves storage shape only; live OWNER masked-reduction
  runtime remains parked until STORE-GPU / R3."*
- **No R1/R2/R3/R4:** no BoundedFeedback/diffusion, no economy stockpile/recipe, no capability mask-down,
  no FIELD_POLICY/gradient/exact-sqrt/threshold/`BoundaryRequest`, no movement/REENROLL, no combat.

## 3. Deliverables

- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_store.rs` — `ChildContribution`, the
  contribution generator over `LocationMaterialization` occupants (generic seeded values into LOC
  channels), the CPU aggregation table keyed by `(location_id, cell_index, channel, owner)`, and a
  `STATUS_PASS` const stating **EC-A3 (CPU storage shape); STORE-GPU deferred; CPU-only, not GPU**.
- `crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_store.rs` (tests §4).
- `docs/tests/scenario_0080_2_atlas_batch_0_store_report.md`, `…_status_row.md`,
  `…_cargo_test_2026_06_03.txt`.

## 4. Tests (target `dress_rehearsal_atlas_batch_0_store`)

1. `store_status_matches_gate` — id `ATLAS-BATCH-0-STORE`; claims EC-A3 CPU storage only; **not** GPU;
   no R1/R2/R3/R4/economy/FIELD_POLICY/REENROLL/combat.
2. `store_consumes_accepted_loc_pack_inputs` — consumes `LocationMaterialization`/`AtlasBatchPlan`;
   mutates neither.
3. `cell_target_uses_single_indexing_home` — every target goes through LOC `cell_index` (and PACK
   `pack_coord`); no ad-hoc indexing.
4. `co_located_pirate_fleets_sum_only_within_pirate_channels` — the 10 canonical Pirate fleets sharing
   one galactic cell aggregate **only** into `PiratePresence` / `FleetStrength(Pirate)`; nothing leaks
   into Terran / planet / patrol channels.
5. `constructed_planet_patrol_pirate_same_cell_stays_distinct` — a constructed `(location, x, y)` with a
   planet + a patrol + a pirate → **three distinct channel/owner entries**, same dense cell slot, **no
   blind sum-by-position**.
6. `owner_indexed_entries_do_not_blind_sum_by_position` — Terran and Pirate contributions at the same
   cell stay separated by owner/channel.
7. `channel_metadata_survives_store` — LOC `ChannelSet` descriptors intact; no invented gameplay columns.
8. `pack_coordinate_round_trip_preserves_store_target` — LOC cell ↔ PACK atlas coord round-trips for the
   stored targets.
9. `no_r1_r2_r3_r4_behavior` — guard: no BoundedFeedback/diffusion/economy/capability/FIELD_POLICY/threshold/
   movement/combat path introduced.
10. `store_cpu_oracle_is_explicitly_non_gpu` — status/report do **not** claim GPU closure; no
    `simthing-gpu` import.
11. `store_is_deterministic` — same inputs → identical aggregation table.

No test imports `simthing-gpu`/`-core`/`-sim`.

## 5. Files Cursor MAY create / edit

- create the three `store` files above; **after green only** —
  `docs/design_0_0_8_0_consumer_pulled_production_track.md`, `docs/worklog.md`.

## 6. Files Cursor MUST NOT edit

`docs/design_0_0_8_0.md`; `docs/invariants.md`; `…/dress_rehearsal_atlas_batch_0_{gen,loc,pack,pack_gpu}.rs`
and accepted GEN/LOC/PACK/PACK-GPU artifacts; `crates/simthing-driver/src/lib.rs`; **`crates/simthing-gpu/**`,
`crates/simthing-core/**`, `crates/simthing-sim/**`** (STORE is CPU-only — touching these is a stop condition).

## 7. Raw evidence

`cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store -- --nocapture *>&1 | Tee-Object docs/tests/scenario_0080_2_atlas_batch_0_store_cargo_test_2026_06_03.txt`
(no GPU env var — STORE is CPU-only). **Run to green before any PASS claim.** If it fails: keep the
diagnostic log, do **not** mark PASS, do **not** update the production doc, return to Opus.

## 8. Docs update (AFTER green only)

`docs/design_0_0_8_0_consumer_pulled_production_track.md`: mark **`ATLAS-BATCH-0-STORE` PASS for EC-A3
(CPU storage shape)**; link source/test/report/raw-log; state **CPU-only**; state which co-location cases
were proven; state **OWNER masked-reduction runtime remains parked** and the **GPU masked-reduction
parity is deferred to `ATLAS-BATCH-0-STORE-GPU`**; R1/R2/R3/R4 unimplemented; M-4A sparse-residency
scheduler parked; REENROLL parked. One `docs/worklog.md` entry.

## 9. Stale artifacts

Delete only superseded duplicate STORE raw logs after a clean final green log. Never delete accepted
GEN/LOC/PACK/PACK-GPU artifacts. If none: "Deleted obsolete artifacts: none found."

## 10. Stop conditions — STOP and escalate if it seems to need

new WGSL · any `simthing-gpu`/`-core`/`-sim` edit · `AccumulatorOp`/`EvalEML` calls or a live masked
reduction · runtime `match kind` · CPU map planner · economy stockpile / production recipe ·
disruption heatmap / BoundedFeedback / diffusion · capability-tree mask-down · FIELD_POLICY / exact sqrt /
threshold / `BoundaryRequest` · REENROLL / movement · combat · M-4A sparse-residency · default session
wiring · semantic map/faction/gameplay logic. **STORE is generic child-result storage into dense 2-D
slots — storage shape only.**

## 11. Handoff-back format (Cursor → Opus)

```
Recipient model: Opus  ·  Role: design authority
ATLAS-BATCH-0-STORE implemented (EC-A3 CPU storage shape; STORE-GPU deferred; CPU-only).
Raw log: docs/tests/scenario_0080_2_atlas_batch_0_store_cargo_test_2026_06_03.txt  (N passed; 0 failed)
Co-location cases proven: 10-pirate-shared-cell; constructed planet+patrol+pirate.
Deliverables / deviations / open questions: <...>
§0.5 self-check: holds 1–6 — <one line>
```

## 12. §0.5 self-check (this contract)

Holds 1–6: STORE is a generic CPU storage/aggregation-shape proof of child contributions into dense
2-D map slots — the substrate R1/R2/R3 consume — not a runtime subsystem. No resource-flow behavior,
no allocation outside the recursive tree, no CPU planner, no `simthing-sim` semantics, no default
wiring, no runtime `match kind`. The CPU-only / STORE-GPU split keeps the OWNER masked-reduction
*runtime* parked and prevents CPU storage evidence from posing as a GPU reduction proof; the CPU
oracle is the reference the later GPU slice checks against.
