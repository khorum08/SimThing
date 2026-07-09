# TP-STUDIO-STEAD-REBIND-0 Results

## Status

**DA-GRADUATED / COMPLETE** — merged [#1226](https://github.com/khorum08/SimThing/pull/1226) @ `03afe3d15234f51ca4a221757226b3bf3902f9af` (head `1c41c2372c562a9a7a9a1d1699b5aba4ee66353a`). DA light stamp 2026-07-09.

Converts `AuthorityTreeCandidate` Spec (empty STEAD grid) into `StructuralRebindReady`
for the approved Terran-Pirate clause path. **No** production mapeditor `.clause` API,
**no** UI picker, **no** closeout.

## Identity

| Field | Value |
|---|---|
| Rung | `TP-STUDIO-STEAD-REBIND-0` |
| PR | [#1226](https://github.com/khorum08/SimThing/pull/1226) |
| Merge SHA | `03afe3d15234f51ca4a221757226b3bf3902f9af` |
| tested_code_sha | `1c41c2372c562a9a7a9a1d1699b5aba4ee66353a` |
| Home | `crates/simthing-workshop/src/tp_studio_stead_rebind.rs` |
| birth_track | `0.0.8.5-terran-pirate` |

## Implemented path

```text
terran_pirate_galaxy.clause
  → ingest_tp_clause_scenario_path          [workshop]
  → AuthorityTreeCandidate Spec
  → rebind_authority_tree_candidate /
    rebind_pack_to_structural_rebind_ready  [workshop tp_studio_stead_rebind.rs]
  → StructuralRebindReady Spec
  → validate_stead_mapping_consistency PASS
  → serialize/deserialize_scenario_authority roundtrip PASS
```

## Projection mode achieved

```text
StructuralRebindReady
```

## Rebind policy

| Field | Policy |
|---|---|
| `map_container_id` | Authority GalaxyMap raw id (`game_session_galaxy_map`) |
| `placements` | One per GalaxyMap star-system Location child; `simthing_id_raw` = authority node raw id |
| Join key | Structural `(row, col)` → embedded `source_structural_grid.placements` |
| `system_id` / location / target ids | From embedded source placement at same coord |
| Authority mutation | Stamps `SCENARIO_GENERATED_SYSTEM_ID` on gridcells (required by STEAD validate) |
| `links` | From embedded `namespaced_links` remapped namespaced target → system_id string endpoints |
| GameMode / RF / combat | **Not attached** (live-run scope) |

## Authority node id proof

Test: `tp_studio_stead_rebind_0_references_authority_node_ids`

- Every placement `simthing_id_raw` is a Location child of the authority GalaxyMap.
- `map_container_id` equals GalaxyMap raw id.

## map_container_id proof

- Non-empty after rebind.
- Resolves via existing `resolve_map_container` inside `validate_stead_mapping_consistency`.

## placements proof

- Non-empty (1500 systems for TP base disc).
- `occupied_cells == placements.len()`.
- Candidate started empty; rebind fills.

## links proof or residue

- Links populated from embedded namespaced hyperlane set remapped to system_id endpoints.
- `validate_scenario_links` PASS when non-empty.
- Residue field available if join drops endpoints (report `links_residue`).

## Existing validation proof

```text
validate_stead_mapping_consistency(&rebind.scenario) → PASS
validate_scenario_links (when links non-empty) → PASS
```

Tests: `tp_studio_stead_rebind_0_structural_rebind_ready`, `…_from_candidate_api`.

## Studio session hydrate result

```text
studio_session_hydrate: NOT_RUN_IN_WORKSHOP
```

Workshop crate does not depend on Bevy `simthing-mapeditor`. Structural STEAD validation is the
binding gate named by readiness report; full `StudioSession::from_loaded_scenario` remains a
follow-on proof that can import the StructuralRebindReady Spec via production JSON IO without a
`.clause` API.

## Spec vs pack boundary maintained

- Rebind writes only Spec STEAD fields (+ necessary gridcell system_id property stamps).
- Does not attach `game_mode`, combat arena, PALMA/commitment, install_targets, RF feedstock.

## Lowerer heuristic discipline

- No new owner-key special cases.
- No new TP production constants outside workshop.
- No combat/RF semantics.
- Uses existing clausething pack embed lattice + production STEAD validate.

## Non-goals (honored)

- production mapeditor `.clause` API
- UI picker
- GameMode/RF attach
- live-run / GPU / kernel
- closeout
- API admission

## Inventory/lifecycle

| Test | birth_track | class |
|---|---|---|
| `tp_studio_stead_rebind_0_*` | `0.0.8.5-terran-pirate` | golden-byte / oracle-parity |

Workshop-homed candidate; promotion_target notes DA/Owner admission for any production elevation.

## Commands

```bash
cargo test -p simthing-workshop --test tp_studio_stead_rebind_0 -- --nocapture
cargo check -p simthing-workshop
cargo check -p simthing-mapeditor
cargo check -p simthing-clausething
bash scripts/ci/gen_orientation.sh --check
bash scripts/ci/test_inventory_drift_check.sh
bash scripts/ci/test_lifecycle_expiry_check.sh --schema
bash scripts/ci/doc_budget_check.sh --check
bash scripts/ci/doctrine_scan.sh
git diff --check
```

## Clearance routing

Workshop-homed TP candidate + tests + report. Expect
`DA-RESERVE(unclassified-scope)` unless a workshop class matches — **not novelty**.

## Known gaps

- Full Studio session hydrate not exercised in-workshop (Bevy dep boundary).
- Production mapeditor ClauseScript API still denied.
- Lowerer owner-key / TP property-id debt still blocks API admission (admission-1 later).

## Recommended next rung

```text
TP-STUDIO-CLAUSE-API-ADMISSION-1
```

Re-open DA/Owner admission with StructuralRebindReady proven. Optionally intermediate:

```text
TP-STUDIO-SESSION-HYDRATE-0
```

if DA wants mapeditor session proof before admission language.

**Not next:** UI picker (blocked until API admission).
