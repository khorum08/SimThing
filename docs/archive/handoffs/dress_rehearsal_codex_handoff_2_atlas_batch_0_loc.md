# Codex/Cursor Handoff 2 — `ATLAS-BATCH-0-LOC` Implementation Contract

**Recipient model: Cursor**
**Role: production implementation agent**

**From:** Opus (design authority) · **Date:** 2026-06-03 · **Gate:** authored + accepted.
**Predecessor:** `ATLAS-BATCH-0-GEN` is **closed / PASS** (6/6). LOC consumes its descriptor.

---

## 1. Fixed base harness (cite on handoff back)

1. `docs/design_0_0_8_0.md` §0 — transient constitution (conformance; conflict-as-resource-flow;
   recursive allocation; FIELD_POLICY = GPU threshold crossings, no CPU planner; §0.5 harness incl. **two-layer**).
2. `docs/invariants.md` — Scenario Proof; AccumulatorOp v2; Resource Flow Substrate; Mapping invariants
   (esp. **"Local index arithmetic has one home"**); semantic-free `simthing-sim`.
3. `docs/design_0_0_8_0_consumer_pulled_production_track.md` §12–§12.5 — ATLAS-BATCH-0 ladder, OWNER
   routing, GEN closure, LOC = this gate.
4. `docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md` — concrete 20×20 / 13-system topology.
5. `crates/simthing-core/src/accumulator_op.rs` — the GPU-resident Accumulator vocabulary later rungs
   lower to (LOC produces structure that PACK/STORE/R1/R4 consume).
6. `docs/workshop/field_policy_track.md` — FIELD_POLICY charter.

**Rung-local (ephemeral, this rung only — §0.5 two-layer):** `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_gen.rs`
(the descriptor LOC consumes); `docs/tests/scenario_0080_2_atlas_batch_0_gen_report.md`.

---

## 2. Established decisions — DO NOT re-derive (Opus-locked)

- **LOC produces a descriptor layer, NOT live `SimThing` instances.** It computes the gridcell-primitive
  *layout* + occupant placement + typed channel metadata. SimThing instantiation, the GPU `SlotAllocator`,
  and numeric property columns are **later** (PACK/STORE/runtime). LOC is fixture-only, deterministic, no
  `simthing-core`/`simthing-sim`/`simthing-gpu` coupling, no GPU, no economy.
- **`Location` is a descriptor *role*, never a runtime `match kind`.** Use a `LocationRole` enum on the
  descriptor (`Galactic`, `StarSystem`, `PlanetSurface`). No new `SimThingKind` variant; no runtime
  behavior branches on role.
- **Three gridded `Location` tiers materialize:** the **galactic starmap** (1, 20×20), **each star
  system** (13, 10×10), **each planet surface** (13, 10×10) = **27 Locations.** Planets do **not** become
  their own gridded Location.
- **Occupants (not Locations):** planet, starport, factory district, pop cohort (fixed); patrol/pirate
  fleets (movers). A **planet** is an occupant of its star-system Location carrying a `surface_location`
  link to its planet-surface Location (the bridge to the next tier).
- **Dense cells + sparse occupants.** Cells get the dense slot range; occupants are a **separate
  placement list** keyed to `(location, x, y)` — occupants get **no** field slots in LOC. Co-located
  occupants are **distinct records** and must never be collapsed.
- **Single indexing home.** `cell(x,y) = map_base + y·width + x` lives in **exactly one function**; all
  code routes through it (mirrors `invariants.md` "Local index arithmetic has one home"). No ad-hoc
  indexing anywhere else.
- **`map_base` allocated by a narrow fixture-scoped helper** (sequential, deterministic), **not** the
  `simthing-gpu` `SlotAllocator`. Order: galactic (base 0), then systems in index order, then surfaces in
  system-index order; each Location gets a contiguous `[base, base + width·height)` range; ranges are
  non-overlapping.
- **Channels are typed descriptors only, NOT numeric columns.** Per-Location `ChannelSet` declares the
  tier's channels (surface → `{labor, production}`; system → `{disruption}` + production pass-through;
  galactic → `{disruption, fleet_strength{owner}, patrol_presence, pirate_presence}`). Owner-indexed
  channels are typed `(channel, owner)` entries. STORE/PACK materialize the actual columns later.
- **Owner identity is copied from GEN into occupant/cell metadata.** Owner-*columns*, masked reduction,
  capture, and overlays are **deferred** (STORE / R3). No capture, no reparenting, no overlay runtime.
- **Module wiring:** `loc.rs` includes GEN as a private submodule
  (`#[path = "dress_rehearsal_atlas_batch_0_gen.rs"] mod gen;`) and consumes its types. The LOC **test**
  `#[path]`-includes `loc.rs`. **Do NOT export to `lib.rs`** (fixture-only; no production wiring). GEN's
  own test binary is separate — no duplicate-symbol conflict.

---

## 3. Exact LOC deliverables

A fixture-scoped module `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_loc.rs` that consumes
`DressRehearsalMap::canonical()` and produces a deterministic materialization:

- `LocationRole { Galactic, StarSystem, PlanetSurface }`
- `LocationGridDescriptor { id, role, parent: Option<LocationId>, map_base: u32, width: u32, height: u32,
  channels: ChannelSet }`
- `OccupantPlacement { kind, owner, location_id, cell: GridCell, mobility: Fixed | Mover,
  channels: Vec<ChannelDescriptor>, surface_location: Option<LocationId> }`
- `ChannelDescriptor` (typed: `Labor`, `Production`, `Disruption`, `PatrolPresence`, `PiratePresence`,
  `FleetStrength(Owner)`, …) + a `ChannelSet`.
- `LocationMaterialization { locations: Vec<LocationGridDescriptor>, occupants: Vec<OccupantPlacement>,
  total_cell_slots: u32 }` with a `from_map(&DressRehearsalMap)` (and a `canonical()`).
- **One** `fn cell_index(map_base: u32, width: u32, x: u32, y: u32) -> u32` — the sole indexing home,
  plus an inverse or bounds check as needed.
- A `STATUS_PASS` const guarded by a `docs_status_matches_gate` test (mirror GEN).

Materialize: 27 Locations (1 galactic + 13 system + 13 surface); 56 occupants (13 planets w/ surface
links, 4 starports at (5,5), 13 factories, 13 pop cohorts, 3 patrol + 10 pirate fleets on the galactic
tier at their `galactic_cell`).

## 4. Files Cursor MAY create/edit

- **create** `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_loc.rs`
- **create** `crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_loc.rs`
- **create** `docs/tests/scenario_0080_2_atlas_batch_0_loc_report.md`,
  `…_status_row.md`, `…_cargo_test_2026_06_03.txt`
- **update (only after green)** `docs/design_0_0_8_0_consumer_pulled_production_track.md` (LOC status),
  `docs/worklog.md` (one entry).

## 5. Files Cursor MUST NOT edit

- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_gen.rs` (consume read-only) and the accepted
  GEN report/status/log artifacts.
- `docs/design_0_0_8_0.md`, `docs/invariants.md` (Tier-2; design authority only).
- `crates/simthing-driver/src/lib.rs` (no export — fixture-only).
- Anything under `crates/simthing-sim/**`, `crates/simthing-gpu/**`, `crates/simthing-core/**`.

## 6. Tests Cursor MUST add (target `dress_rehearsal_atlas_batch_0_loc`)

1. `loc_materialization_is_deterministic` — same GEN map → identical `LocationMaterialization`.
2. `every_location_cell_in_bounds` — each Location's cells within its `width`×`height`.
3. `cell_index_matches_row_major_formula` — `cell_index(base,w,x,y) == base + y*w + x` for sampled cells;
   all indexing goes through `cell_index`.
4. `location_slot_ranges_contiguous_and_non_overlapping` — ranges partition `[0, total_cell_slots)` with
   no gap/overlap; count = 400 + 13·100 + 13·100.
5. `co_located_occupants_remain_distinct` — the 10 pirate-ship occupants sharing one galactic cell remain
   **10 separate records**; plus a constructed planet+patrol+pirate-in-one-cell case stays 3 records.
6. `occupants_retain_gen_owner` — every occupant's owner equals its GEN source owner.
7. `channel_descriptors_present_per_tier` — surface→{labor,production}; galactic→{disruption,
   fleet_strength(per owner),patrol_presence,pirate_presence}; system→{disruption}.
8. `planet_links_to_its_surface_location` — each planet occupant's `surface_location` resolves to a
   `PlanetSurface` Location; tier parent links are consistent.
9. `docs_status_matches_gate` — `STATUS_PASS` const matches the gate id/status.

No test may import `simthing-sim`/`simthing-gpu`; no GPU; no default-on wiring.

## 7. Raw test artifact + run command

Run: `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_loc`
Save raw output to: `docs/tests/scenario_0080_2_atlas_batch_0_loc_cargo_test_2026_06_03.txt`
**The rung is done only at green (all tests pass). "Execution pending" is not acceptance** — run it,
then write the report/status. The `STATUS_PASS` const stands only once the suite is green.

## 8. Docs to update AFTER green

In `docs/design_0_0_8_0_consumer_pulled_production_track.md`: mark `ATLAS-BATCH-0-LOC` implemented/PASS
**only** after green; link the LOC source/test/report/raw-log; state that LOC materializes the
gridcell-primitive layout + occupant placement + typed channel descriptors **only**; **PACK and STORE
remain unimplemented; sparse-residency scheduler / M-4A and REENROLL remain parked.** Add one worklog
entry. If LOC tests fail, do not mark PASS.

## 9. Stale artifacts

Delete only redundant raw logs from failed/duplicate **LOC** runs after a clean final log exists. **Do
not** delete the accepted GEN report/log/status artifacts.

## 10. Stop conditions — STOP and escalate to Opus if implementation seems to need

GPU stencil dispatch · atlas batch packing · resource-flow arena execution · economy/disruption behavior
· owner masked-reduction runtime · capability-tree mask-down · movement/REENROLL · combat · FIELD_POLICY gradient
consumption · runtime `match kind` · new semantic WGSL · new `simthing-sim` map/faction/gameplay
semantics · default session wiring · `SimThing`/`SlotAllocator`/numeric-column materialization. **All
later rungs.** LOC produces *structure*; every decision stays a later GPU-resident threshold crossing.

## 11. Handoff-back format (Cursor → Opus)

```
Recipient model: Opus  ·  Role: design authority
ATLAS-BATCH-0-LOC implemented.
Raw test log: docs/tests/scenario_0080_2_atlas_batch_0_loc_cargo_test_2026_06_03.txt  (N passed; 0 failed)
Deliverables: <files>
Deviations from contract / open questions: <...>
§0.5 self-check: holds principles 1–6 — <one line>
```

## 12. §0.5 self-check (this contract)

Holds principles 1–6: it authors structure that lowers to the Accumulator/field rungs (no CPU planner,
no map traversal); everything-is-a-SimThing is honored by making `Location` an authored descriptor role,
not a runtime kind-branch; no resource-flow behavior, no allocation outside the recursive tree, no
`simthing-sim` semantics, no default wiring. LOC is the structural substrate the GPU-resident FIELD_POLICY path
is built on, proven by deterministic descriptor tests — not a runtime subsystem.
