# STUDIO-DISRUPTION-SELECT-SCREEN-0 Results

## Status
**PROBATION / DRAFT** — PR [#1420](https://github.com/khorum08/SimThing/pull/1420); dispatch `5022654370`; Remand 1 `5023235060`; Remand 2 `5024193617`; DA live-STEAD remedial `5025282291` (ruling `5025156207`). Owner OVL remains OPEN until re-freeze + nonzero selected-system capture. Prior freeze `715fdde4…fe58` is STOP provenance only.

## Identity SHAs
| role | full SHA |
|---|---|
| base / STOP tip | `8ffdaf610c4c9b48e76c673c155041fd9c5cde5d` |
| handoff_head | `9f1949d9cd54c997414ea7a87004741e3c423500` |
| implementation_code_sha | `f509863a636b7bf2362ae5ded166532c25fd9a42` |
| tested_code_sha | `063a4f6743650fd3156ef25c8046be711c396a8d` |
| evidence_head_sha / final_head_sha | PR tip after this evidence commit (bound in PR body) |

## DA remedial `5025282291` (live STEAD readback)
- Canonical CPU observation seam: `simthing_driver::hosted_property_observation` keyed by hosted `SimThingId` + `PropertyKey` + `SubFieldRole` (no public `ColumnIndex`).
- ResourceEconomy materialize retains typed observation loci from compiled field-economy `emit_on_threshold` (before need-binding inject).
- Bridge refreshes `StudioDisruptionReadoutMap` after field-bearing open and each successful tick from **one** `GpuValuesSnapshot` via `disruption_readout_snapshot_with_readback` + `LiveDisruptionAuthorityReadback`.
- Structural join: Spec placement raw id / exact location_id/target_id / pack `(row,col)` → system_id. Clause locations may enroll via authored `ownership_volume` onto volume anchors (TP `pirate_outpost` → `pirate_border`, `terran_shipyard` → `terran_core`).
- Fail-soft `0.0` only for structural-shell / genuinely absent / no structural join; partial mapping and readback errors fail loud.

## Remand 2 correction (retained)
- No production `DisruptionReadoutRecord::new`; falsifiers use test-local `DisruptionAuthorityReadback`.

## Remand 1 corrections (retained)
1. Owned/hostile/neutral selection; eligibility `star_id == selected_id`.
2. Live selected disruption invalidates dirty keys.
3. Duplicate TEST-BUDGET rows removed.

## Proof matrix
| test | catches |
|---|---|
| breakpoints_and_above_100_clamp | wrong piecewise / missing clamp |
| deselect_restores_identity | sticky screen after clear |
| applies_to_owned_neutral_and_hostile_selection | wrong system / ownership gate |
| coexists_with_11_6_owned_brighten | co-owned set inherits disruption |
| live_disruption_invalidates_visual_dirty_gate | live accretion does not refresh visuals |
| no_wgsl_or_scenario_mutation_surface | kernel/Spec creep |
| canonical_host_system_moves_zero_to_nonzero_unrelated_stays_zero | Absent wall / global copy |
| authored_host_placement_swap_moves_system_id_with_zero_code_change | hard-coded system id |
| two_loci_in_one_system_report_exact_max | sum/first instead of max |
| live_map_refreshes_when_runtime_disruption_changes | open-only map |
| structural_shell_absent_field_stays_typed_zero | shell invents nonzero |
| selected_star_telemetry_matches_live_map_and_piecewise | 12.3/live divergence |
| structural_mapping_all_miss_is_empty_partial_fails_loud | wrong fail-soft/fail-loud |

## Scope Ledger
| | |
|---|---|
| Specified | Selected-star disruption screen + live per-system STEAD readback |
| Implemented | mapeditor presentation + driver observation seam + bridge tick refresh + STEAD location enrollment |
| Proxied | none for live values on field-bearing path |
| Deferred | Owner OVL; re-freeze after orch accept |
| Out of scope | Spec mutation; WGSL; CPU planner; 12.5; `DisruptionReadoutRecord::new` |

## Conformance
piecewise YES · clamp YES · deselect YES · live STEAD map YES · placement swap YES · exact max YES · tick refresh YES · shell 0.0 YES · 12.3 match YES · OVL OPEN

## Sticky disposition
Keep draft/open/unmerged. Owner alone closes OVL after corrected freeze shows nonzero selected-system raw matching blur/red.
