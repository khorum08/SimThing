# STEAD-CONTRACT-0 — results (executable spatial contract and RF binding)

> **Status: DA-APPROVED 2026-06-15 (owner sign-off) — PROBATION cleared.** The owner/design authority
> reviewed STEAD-CONTRACT-0 (#701/#702) and the STEAD-CONTRACT-0R hardening (#703/#704) and approved all
> conclusions. Both are now CURRENT_EVIDENCE in `current_evidence_index.md`. Open follow-ups (§9 items 1, 2,
> 4, 5) remain as future tickets, not blockers.
> Owner-directed handoff (2026-06-15) to make STEAD/Mapping impossible to drift out of context again after
> three catastrophic drifts (positions-inert, dense-global, edge-cap).

## 1. Intent
Convert the corrected STEAD doctrine from prose-only guidance into an **executable contract** that fails loudly
when reasserted-wrong, and bind Resource Flow / Accumulator arenas to the structural grid **when (and only when)**
their participants are gridcell `Location`s. Permanently enshrine Candidate F so bit-exact Euclidean/sqrt
decision-gate ops route *through* it rather than being avoided.

## 2. Files changed

### New
| File | Purpose |
|---|---|
| `docs/stead_spatial_contract.md` | Normative binding contract — 8 defined terms, 9 sections, §9 withdrawn-phrase list. |
| `crates/simthing-clausething/tests/stead_spatial_contract_guards.rs` | 10 executable guards (section-aware forbidden-phrase scan + structural/MF/PALMA/evidence guards). |
| `crates/simthing-clausething/tests/mapgen_rf_stead_binding.rs` | 7 RF ⇄ STEAD binding tests. |
| `docs/tests/stead_contract_0_results.md` | This report. |

### Modified — source (clausething scope)
| File | Change |
|---|---|
| `src/mapgen_resource_flow.rs` | `SpatialBindingMode`, `SpatialArenaBindingReport`, `validate_spatial_binding`; `spatial_binding` field on arena expansion; RF enrollment validates Location participants against `grid_metadata` and records the `StructuralGridFrame`. |
| `src/mapgen_lattice.rs` | `StructuralGridFrame` + `from_grid_metadata`; quarantined module/scenario doctrine strings to structural wording; DEPRECATED note on `fixture_lattice_edge`. |
| `src/lib.rs` | Exported `StructuralGridFrame`, `SpatialBindingMode`, `SpatialArenaBindingReport`, `validate_spatial_binding`. |

### Modified — docs
`docs/simthing_core_design.md` (new prominent **§0** "Spatial substrate: STEAD/Mapping is not optional";
§7 back-reference); `docs/design_0_0_8_3.md` (§0.7 Candidate-F enshrinement); `docs/invariants.md`
(exact-authority row enshrinement); `docs/agents.md` (mandatory-reading item 6 for spatial tasks);
`docs/adr/ClauseThingADR.md` (D6 STEAD-CONTRACT-0 note); `docs/clausething/ClauseThingDoc.md`
(STEAD-contract + RF-binding rows); `docs/clausething/MapGeneratorCLI.md` (§4.1 contract callout);
`docs/clausething/MapGenThing.md` (quarantined positions-inert string); `docs/tests/current_evidence_index.md`
(repair — see §6).

### Modified — producer (out of literal clausething scope; see objections)
`crates/simthing-mapgenerator/src/emitter.rs`, `src/topology.rs` — quarantined "positions are inert" comments
to structural wording; `topology.rs` carries a flagged follow-up objection (index-order adjacency heuristic).

## 3. Doctrine fixed (forbidden phrases purged from active source/docs)
All 10 withdrawn phrases now appear in active files **only** inside explicitly-named withdrawal/correction
sections; bare assertions are gone. The whitelist is section-aware (heading contains "Forbidden" / "Withdrawn"
/ "Correction"). The ADR is intentionally **excluded** from the strict scan — it legitimately documents the
withdrawal under a "Correction" heading.

## 4. Tests added (17 total, all green)
- `stead_spatial_contract_guards.rs` (10): positions-inert / shape-cosmetic / topology-is-the-lattice phrase
  scans (docs + strict source); budget-admission export; `MAPGEN_MAX_LATTICE_EDGE` not reintroduced; emitted
  integer positions honored (not row-major); Movement-Front large-layout **typed** atlas deferral; PALMA
  field indexes structural `grid_size` and emits **zero** route/predecessor surfaces; evidence index has no
  placeholder provenance on merged rows.
- `mapgen_rf_stead_binding.rs` (7): Location participants require structural placements; binding report records
  the `StructuralGridFrame`; uses `grid_metadata` not render metadata; rejects missing/duplicate placement;
  spatially-neutral arenas need no grid; accumulator spatial pressure uses structural gridcell identity.

## 5. RF integration summary (Part E)
RF/Accumulator stays **generic**. A new `SpatialBindingMode` distinguishes `SpatiallyNeutral` from
`SpatiallyBoundToGridcellLocations`. When an arena's participants are gridcell `Location`s, enrollment calls
`validate_spatial_binding`, which requires each participant id to have a `StructuralGridPlacement` in
`grid_metadata` (errors on missing or duplicate placement) and records the arena's `StructuralGridFrame` in
`SpatialArenaBindingReport`. Neutral arenas are unaffected and need no grid. RF was **not** made globally spatial.

## 6. Evidence-index repair (Part H)
`docs/tests/current_evidence_index.md` is the single **live ledger**. Repairs: filled PR10 merge SHA `75505ee2`
(#690) and PR11 `31f0ee3e` (#692); set STEAD-SCALE-1 row to #700 / `793d2633` / merge `3f0ece0a`; added the
STEAD-CONTRACT-0 PROBATION row (provenance pending DA approval); added two live-guardrail rows
(STEAD spatial contract → `stead_spatial_contract_guards.rs`; RF ⇄ STEAD binding → `mapgen_rf_stead_binding.rs`);
updated the Candidate-F row to "permanently enshrined". All cited SHAs verified present in history.

## 7. Forbidden-phrase scan summary
Scanner: `scan_for_phrase` over curated ACTIVE docs (section-aware) + ACTIVE source (strict). Result: **0
violations** across the 7 curated docs and 5 source modules. Archive/superseded/ADR-correction text is out of
scope by design.

## 8. Commands run
```
cargo fmt --check                                             # clean
cargo test -p simthing-clausething                           # all suites pass, 0 failed
cargo test -p simthing-clausething --test stead_spatial_contract_guards   # 10 passed
cargo test -p simthing-clausething --test mapgen_rf_stead_binding         # 7 passed
git diff --check                                             # no whitespace errors
```

## 9. Refinements / objections recorded for owner review
1. **`fixture_lattice_edge` not yet renamed.** I added a DEPRECATED doc note pointing to
   `MapgenStructuralGridBudget` / `StructuralGridFrame` rather than renaming to `legacy_fixture_lattice_edge`
   (the handoff's preferred form) to avoid churning 9 closed test files + the producer in this pass. Follow-up.
2. **Producer `topology.rs` adjacency heuristic is on stale index-order coords.** Since STEAD-PRIVILEGE-0 the
   closed lowerer honors authored positions; the producer's *which-pairs* heuristic still uses lowered
   index-order positions. Emitted `add_hyperlane` pairs still lower correctly; only the producer heuristic is
   on stale coords. Flagged in-source; a producer behavior change with its own tests is the proper fix.
3. **I touched `simthing-mapgenerator` (producer) comments** — outside the handoff's literal clausething scope —
   to purge "positions are inert" strings the guard would otherwise let live in a sibling crate. Scoped to
   doc-comments only; no behavior change.
4. **`MapgenStructuralGridBudget` defaults to unbounded.** Intentional (SimThing models vast domains), but it
   means structural admission is permissive by default; explicit budgets gate when a caller wants a ceiling.
5. **Metadata-bytes budget is a heuristic estimate**, not a measured allocation.

## 10. STEAD-CONTRACT-0R — probation-hardening follow-up (owner-directed, post-#701 review)
The owner's review of #701/#702 cleared the substance but withheld PROBATION-clearance until the contract is
anchored in the **transient constitution §0** (so a mechanical "copy §0 forward" promotion cannot drop it) and
producer-side source is scanned. Landed in STEAD-CONTRACT-0R:
1. **`design_0_0_8_3.md` §0.8 "STEAD/Mapping spatial substrate carry-forward"** — new §0 subsection (by
   addition) stating the clause **and** the `stead_spatial_contract.md` pointer MUST propagate to every future
   constitution version verbatim; a version that drops either is defective.
2. **Guard `transient_constitution_section_0_must_carry_stead_clause_and_contract_pointer`** — proves §0.8 +
   the contract pointer + the "propagate to every future" mandate live **inside** §0.
3. **Producer scan** — `simthing-mapgenerator/src/emitter.rs` + `topology.rs` added to the active-source
   phrase scan (the named producer modules most prone to positions-inert drift).
4. Evidence index keeps #701 **PROBATION until 0R lands** and records the 0R row.

Objection #3 above (producer-comment quarantine outside literal clausething scope) is now **resolved by
design**: those producer modules are deliberately in-scope for the central guard. Objections #1 (`fixture_lattice_edge`
rename), #2 (producer `topology.rs` index-order adjacency heuristic), #4, #5 remain open follow-ups.

## 11. DA sign-off status
**DA-APPROVED 2026-06-15 (owner sign-off).** The owner — the design authority for the Mapping/STEAD track —
reviewed #701/#702 and the #703/#704 (STEAD-CONTRACT-0R) hardening, confirmed the contract is anchored in
transient constitution §0.8 and that producer-side scanning covers MapGeneratorCLI/topology drift, and
approved promoting both from PROBATION to CURRENT_EVIDENCE. Remaining open follow-ups (§9 items 1, 2, 4, 5)
are tracked as future tickets, not blockers. Track parked pending the next window.
