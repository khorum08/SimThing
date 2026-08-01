# Scenario Containment — Handover Inventory

**Authored 2026-08-01 by the DA (Opus 5) at Owner direction, after the 0.0.8.7 TP purge.**
Covers rungs 5.9b `SCENARIO-RESIDUE-PURGE-0`, 5.9d `TP-EXPORT-ERADICATION-0`, and the two
normalization-pump repairs. Read this before authoring the next scenario or rehearsal.

---

## 1. THE RULING (binding, applies to every future scenario and rehearsal)

**A scenario or rehearsal must NEVER export its transient proofing fixtures beyond its own
crate.** Not by `pub` export, not by `dev-dependency`, and not by filesystem path. A fixture
that leaves its crate stops being transient: downstream code binds to it, and it can no
longer be deleted when its rehearsal ends.

Three corollaries, each earned by a failure found on 2026-08-01:

1. **Every core capability must have a synthetic witness.** A capability whose ONLY witness is
   a shipped scenario keeps that scenario structurally alive forever, and no purge can remove
   it — deleting the asset reds a legitimate law. This, not code coupling, is what kept
   Terran-Pirate alive: `owner-seat` requires `HostsAdmittedPolicyWeightLocus`, and
   `terran_pirate_galaxy.clause` was the only file in the repository that built one.
2. **No generated artifact that feeds the orientation digest or a handoff may be derived from
   a shipped scenario.** Two did. They published `tp::hull` / `tp::upkeep` /
   `tp::weapon_damage` and "Terran + Pirate policy/weight authorities" into every agent's
   orientation, under a heading reading *"Canonical TP live inventories"*, annotated
   *"Phase 8 (8.1-8.2) successor"* — a disposable rehearsal issuing requirements to future
   rungs.
3. **A proof may never assert that a shipped asset exists.** `ANCHOR-DISPOSITION-ADMISSION-0`
   asserted `scenarios/` held *"exactly one canonical clause"*, making a game asset a
   structural requirement of the build.

**Assets are not code.** `terran_pirate_galaxy.clause` and its siblings are `.png`-equivalents.
They are never deleted for hygiene, and ClauseThing must always ingest them natively. The
defect was never the asset; it was code, proofs, and generated artifacts reaching for it.

---

## 2. WHAT THIS BROKE — capabilities now without a witness

Measured, not estimated. **28,423 deletions across 89 files; 44 files deleted, 8 added;
inventory 1033 → 1003 rows.** All libraries build clean (`cargo check --workspace`: 0 errors).

Deleting the TP proofs removed real coverage. These capabilities now have **zero witness
anywhere in the repo** and are the first candidates for the next workplan:

| capability | last witness (deleted) | harness absorption |
|---|---|---|
| GPU **resident tick** execution + CPU match | `driver/tests/terran_pirate_skeleton_resident_tick.rs` (4 tests) | 1 incidental ref — **not covered** |
| **Mapping atlas scheduler** | `driver/tests/terran_pirate_mapping_atlas_scheduler.rs` | **0 refs — gone** |
| **RF capacity amendment** | `driver/tests/tp_rf_capacity_amendment.rs` | 3 refs — partial at best |
| **Studio scenario/clause picker** | `mapeditor/tests/tp_studio_clause_picker_0.rs` | **0 refs — gone** |
| **Studio clause API** | `mapeditor/tests/tp_studio_clause_api_1.rs` | not re-witnessed |
| **Base-disc generation** shape/determinism | `mapeditor/tests/tp_base_disc_gen.rs` | `simthing-mapgenerator` has **0 test files at all** |
| Scenario **builder semantics** | `mapeditor/tests/terran_pirate_skeleton.rs` (~12 tests) | not re-witnessed |

**`simthing-mapeditor` is down to 2 test files. `simthing-mapgenerator` has none.** That is the
largest coverage hole this work opened and it should be sized honestly in the next plan.

What was NOT lost, and should not be re-proved: the two workshop laws (byte-identical
regeneration; need-binding fail-closed) are witnessed scenario-neutrally by
`cpu_gpu_parity_matrix_0` and `determinism_matrix_0` (13 refs each), and the seven deleted
`driver/tests/support/` fixtures had **zero live consumers** — they fed only each other.

---

## 3. PRE-EXISTING BREAKAGE — not caused by this work

Each was baselined against master before and after. Do not attribute these to the purge.

- **3 ClauseThing test targets do not compile** — `ct_2a_intrinsic_flow`,
  `ct_2c_category_economy`, `specialization_protocol_0` (19 errors; the last is a
  `PropertySpec` initializer missing the `admission_disposition` field). **CI is green through
  all of them** — doctrine-scan does not compile these targets.
- **`test_inventory_check.sh` reports 11 errors on master**, also with CI green.
- Two **false-reds** were found and fixed in passing: a Board pointer assertion that
  hard-coded a literal rung id (so it went red at *every* graduation), and an assertion on an
  orientation heading `## Specialization citizens` that `gen_orientation.sh` has **never**
  emitted.

---

## 4. OUTSTANDING MECHANISMS (rung 5.9c, still TODO)

- **Artifact provenance gate** — hard-FAIL if any generator of an orientation- or
  handoff-feeding artifact reads `scenarios/`. 19 TSVs feed orientation. This is the gate
  `SCENARIO-RESIDUE` structurally cannot be: the coupling lives in the GENERATOR while the
  artifact sits in `scripts/ci/`.
- **`DEAD-EXPORT` blind spots** — it covers `src` modules only. Both shapes below were found
  by hand and both still exist: a `tests/` target declaring ZERO test functions
  (`simthing-spec/tests/planet_child_location_admission.rs`, 240 lines, runs `0 passed`) and a
  `tests/support/` module with no consumer outside `support/`
  (`simthing-driver/tests/support/resource_economy_session.rs`, 311 lines, reads
  `scenarios/rebellion_demo.ron`). 21 advisory dead exports remain flagged.
- **Permanence vocabulary** — `test_inventory_check.sh` still MANDATES a `permanent-residue:*`
  promotion target on every KEEP row, and `test_lifecycle_expiry_check.sh` still exempts
  `permanent-residue:escaped-bug` given a 40-character note. **A 40-character string still buys
  immortality.**
- **`0.0.8.5-terran-pirate`** remains a closed track (closed 2026-07-09) carrying authoring-layer
  rows; reap due **2026-08-11**.

---

## 5. WHAT `simthing-spec` MUST GROW TO ABSORB (next workplan)

The Owner's requirement: broaden spec so future domains never need to push semantics back,
**without killing agent creativity**. Growth is expected and permitted; what is forbidden is
growing to accommodate a scenario, a save file, or a specific domain crate — unless the
capability generalizes into the recursive SimThing principle.

The concrete gap this purge exposed:

- **Owners and user seats must be authorable as generic, recursive SimThings.** Owner channels
  are how adversarial resource contention resolves, so `owner-seat` is genuine platform law —
  but until 2026-08-01 its only witness was a disposable trial proof. `specialist_citizens_minimal.clause`
  is now the first synthetic one. Spec should express owner/seat authority, policy-weight
  loci, and recursive parentage as authored data, so an arbitrary authored asset can declare
  unorthodox owners nested arbitrarily and be admitted or rejected by EXISTING law.
- **Legacy and future authored assets will produce unorthodox owner SimThings and recursive
  parentage.** Actioning them with code is constitutionally forbidden — it is building the
  engine to satisfy a stale savefile. The engine may only ADMIT or REJECT by existing law,
  never grow a branch for one asset's shape.
- **Do not over-constrain.** The point of broadening spec is to widen what an author may
  legally express, so creative scenarios need no engine change. A narrower spec does not
  prevent contamination — it *causes* it, by forcing authors to reach into engine crates for
  what they cannot express as data.

---

## 6. CHECKLIST FOR THE NEXT REHEARSAL

Before authoring one, and before it is allowed to graduate:

1. Its fixtures live in ITS crate and are referenced nowhere else — no `pub` export, no
   dev-dependency, no `../other-crate/tests/fixtures/` path.
2. It has a wall-clock lease and inventory rows from birth. **Production `src` has no lifecycle
   clock at all**, which is why 23,437 lines of dead scaffolding — 47.6% of
   `simthing-driver/src`, with 8 asserts and 0 `#[test]` across all of it — sat unreaped for
   months. Things with clocks get cleaned; things without them grow.
3. No generated artifact derives from it.
4. Every capability it exercises has, or gains, a synthetic witness that survives its deletion.
5. Deleting the whole rehearsal reds nothing but the rehearsal.
