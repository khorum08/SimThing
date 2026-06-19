# SCENARIO-SESSION-OWNER-ROOT-REVISED-0 — restore owner doctrine and demote proof-slice ladder

> **Lifecycle: PROBATION** — doctrine conflict corrected; terminology corrected; lower-layer evidence reclassified; targeted guards pass. Pending owner DA approval. No broad DA closure of scenario-tree implementation.

**Date:** 2026-06-19  
**PR:** SCENARIO-SESSION-OWNER-ROOT-REVISED-0  
**Base:** `master` after PR #775 / DRIVER-STRUCTURAL-ATLAS-HALO-0

## Exact conflict found

`crates/simthing-core/src/simthing.rs` previously stated that political structures, factions, and non-physical groupings are **overlays, not nodes** in the SimThing tree. That contradicted the permanent core design and active constitution: Owner entities are **sibling children of the GameSession root**, not overlays and not spatial parents. Asset ownership is by reference/property/column, never spatial reparenting.

Separately, the Terran Pirate / mapping / atlas PR ladder (#764–#775) had been treated as main-track scenario-ontology progress. It is lower-layer compile/scheduler/GPU golden-fixture evidence only.

## Code correction summary

| File | Change |
|---|---|
| `crates/simthing-core/src/simthing.rs` | Added `SimThingKind::Owner`; deprecated `Faction` for legacy serde/install; replaced overlay-only doctrine with GameSession/Owner/spatial-containment doctrine; `kind_matches` accepts `"Owner"` and legacy `"Faction"` for both variants |
| `crates/simthing-core/src/property.rs` | Added `SimThingKindTag::Owner`; retained `Faction` as legacy alias |
| `crates/simthing-sim/src/fission.rs` | Minimal `kind_tag_to_kind` arm for `Owner` (compile compatibility only) |

**Not introduced:** runtime owner/faction engine; spatial-reparenting ownership model; `SimThingKind::GameSession` / `Scenario` (deferred to PR ladder).

## Constitution terminology correction summary

`docs/design_0_0_8_3.md`:

- §0 ambition language: faction drives → **owner-entity drives**; factions → **owners** / **owner entities**.
- Added **§0 “Terminology correction — owner, not faction”** before §0.1.
- Preserved recursive reduce-up / disburse-down, anti-flattening, anti-hygiene, anti-special-engine rules.

## Compatibility decision for `SimThingKind::Faction`

**Preferred path applied (PASS, not PARTIAL):**

```rust
Owner,
#[deprecated(note = "Use Owner. Retained only for legacy serialized data compatibility.")]
Faction,
```

- New authoring uses `Owner` / `"Owner"`.
- Legacy serialized `Faction` and `AllOfKind { kind: "Faction" }` remain compatible via deprecated variant + `kind_matches` aliasing.
- No serde migration required.

## Owner variant status

| Item | Status |
|---|---|
| `SimThingKind::Owner` added | **Yes** |
| Legacy `Faction` retained as deprecated variant | **Yes** |
| `SimThingKindTag::Owner` added | **Yes** |

## Lower-layer assets folded forward (PRs #764–#775)

Reclassified as **reusable lower-layer golden-fixture evidence** in `docs/tests/current_evidence_index.md`:

- structural N4 theater compile/admission (#769)
- mapping plan compile (#771)
- sim resident mapping tick (#770)
- mapping readback policy (#772)
- sim atlas scheduler (#773)
- structural atlas partition (#774)
- structural one-cell halo admission (#775)
- e10 seam guards (#766)
- Studio load/projection machinery for map display (#764–#765)
- Terran Pirate skeleton as golden fixture (#764–#768), not canonical Scenario save-game shape

## Hygiene-kabuki artifacts demoted/reclassified

| Superseded habit | Action |
|---|---|
| Terran-Pirate-only PRs as primary development direction | Demoted — golden fixture only |
| One-more-proof-slice handoffs without arbitrary scenario ingestion | Guardrail added to evidence index |
| Evidence/doc churn treated as product progress | Reclassified rows #764–#775 as `LOWER_LAYER_GOLDEN_FIXTURE` |

**Doctrine note (evidence index):** Terran Pirate is a golden fixture for lower-layer compile/scheduler/GPU proofs. It is not the canonical shape of a full Scenario save-game tree.

**Guardrail:** Future main-track scenario PRs must introduce or generalize scenario/session/owner/map ingestion capability. A PR centered only on Terran Pirate is remedial unless explicitly scoped as a lower-layer regression fixture.

## Specified-vs-implemented ledger

| Specified | Implemented | Status |
|---|---|---|
| Demote Terran Pirate ladder to lower-layer evidence | Evidence index rows #764–#775 tagged `LOWER_LAYER_GOLDEN_FIXTURE` | PASS |
| Correct simthing.rs owner doctrine | Owner as Session child; not overlay; not spatial parent | PASS |
| Constitution Owner terminology | §0 correction + owner-entity language | PASS |
| `SimThingKind::Owner` + legacy `Faction` | Owner added; Faction deprecated | PASS |
| Targeted e10 guards | `e10_owner_doctrine_and_evidence_reclassification_guards` | PASS |
| No runtime owner engine | No owner engine added | PASS |
| No spatial reparenting ownership | Doctrine + guards only | PASS |
| Scenario → GameSession → Owners → GalaxyMap tree | Documented forward PR ladder only | DEFERRED |
| `SimThingKind::Scenario` / `GameSession` | Not in this PR | DEFERRED |
| General scenario ingestion API | PR 5 in forward ladder | DEFERRED |
| Studio Scenario-root load/save/display | PR 6 in forward ladder | DEFERRED |

## Forward PR ladder (post this PR)

1. **SCENARIO-SERIALIZABLE-SIMTHING-ROOT-0** — Scenario as serializable file root  
2. **SCENARIO-GAMESESSION-CHILD-0** — Scenario contains one GameSession child  
3. **SESSION-OWNER-ENTITIES-0** — Owner SimThings as GameSession siblings  
4. **SESSION-GALAXYMAP-WORLDSTATE-0** — GalaxyMap as GameSession child  
5. **GENERAL-SCENARIO-INGESTION-ADMISSION-0** — arbitrary scenario ingestion  
6. **STUDIO-SCENARIO-LOAD-SAVE-DISPLAY-0** — Studio Scenario tree IO/display  
7. **SESSION-RESOURCE-FLOW-SILOS-0** — generic owner stockpile reduce-up/disburse-down  
8. **SCENARIO-CORPUS-FUZZ-0** — corpus runner; anti proof-theater relapse  

## Guard coverage

| Guard | Location |
|---|---|
| simthing.rs must not say owners/factions are overlays-only | `e10_owner_doctrine_and_evidence_reclassification_guards` |
| simthing.rs states owners are Session children | same |
| Constitution “owner, not faction” correction | same |
| §0 bare “factions” ontology forbidden | same |
| Evidence index lower-layer reclassification | same |
| Hygiene relapse guardrail text | same |
| Existing e10 seam guards (sim upward imports) | `e10_does_not_import_arena_registry_into_simthing_sim` (unchanged) |

## Validation commands

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-core` | PASS |
| `cargo test -p simthing-core` | PASS (72/72) |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec -j 1` | PASS (all integration + unit tests) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission e10_does_not_import_arena_registry_into_simthing_sim` | PASS |
| `cargo test -p simthing-spec --test e10_resource_flow_admission e10_owner_doctrine_and_evidence_reclassification_guards` | PASS |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18/18) |
| `git diff --check` | PASS |
| `git diff --name-only master...HEAD` | see committed paths below |

**Changed paths (vs `master`):**

- `crates/simthing-core/src/property.rs`
- `crates/simthing-core/src/simthing.rs`
- `crates/simthing-sim/src/fission.rs`
- `crates/simthing-spec/tests/e10_resource_flow_admission.rs`
- `docs/design_0_0_8_3.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/scenario_session_owner_root_revised_0_results.md`

## DA status

User design-authority correction applied for Owner/GameSession ontology in core doctrine and constitution terminology. **No broad DA closure** of Scenario → GameSession → Owners → GalaxyMap tree implementation — that remains owed across PRs 1–8 above.