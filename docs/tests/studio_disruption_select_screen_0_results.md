# STUDIO-DISRUPTION-SELECT-SCREEN-0 Results

## Status
**STOP / DRAFT** — PR [#1420](https://github.com/khorum08/SimThing/pull/1420); orch Remand 3 `5026225403` (chain `5022654370` → `5025008394` → `5025156207` → `5025282291`). Remand 3 code corrections landed; production host→visible-system enrollment for field locations is **missing** under existing structural authority after reverting unauthorized `location.ownership_volume`. Owner OVL remains blocked. Prior freeze `715fdde4…fe58` is STOP provenance only — do not re-freeze.

## Identity SHAs
| role | full SHA |
|---|---|
| base / STOP tip | `8ffdaf610c4c9b48e76c673c155041fd9c5cde5d` |
| handoff_head | `9f1949d9cd54c997414ea7a87004741e3c423500` |
| implementation_code_sha | `eff3773742f93220f6506635da1c93da4a920505` |
| tested_code_sha | `3b01896c1e6461fe6e6238c58ec9b0efa1881f19` |
| evidence_head_sha / final_head_sha | PR tip after this evidence commit (bound in PR body / STOP comment) |

## Remand 3 corrections (`5026225403`)
1. **Typed disruption loci only** — bridge feeds `LiveDisruptionAuthorityReadback` from `StudioDisruptionObservationLocus` derived solely from hydrated `disruption_presence` (namespace + `{location}_{resource}_presence` + Amount). Generic observation door retained; all-threshold `observation_loci` removed from the disruption path.
2. **Fail loud on unmapped live loci** — empty typed locus set / structural-shell may fail soft; any nonempty typed set with total or partial mapping miss returns typed bridge error. All-miss fail-soft path removed.
3. **Biting proofs** — exact `0 → nonzero` under `step_once`; refresh requires `after != before`; real `LiveDisruptionAuthorityReadback` `max(3,8)=8`; total/partial miss + unmapped open + unknown property/role/host fail loud.
4. **Reverted** — `HydratedScenarioNode.ownership_volume`, `enroll_locations_at_ownership_volume_anchors`, and Clause location `ownership_volume` fields. Scenario-level `ownership_volume` / fleet payload refs unchanged.

## STOP — missing enrollment contract
Field hosts (`pirate_outpost`, foundry `basin`, …) are install-target entities, not Spec star `location_id`s. After grammar revert, `(row,col)` join does not place them on visible stars. CI uses `attach_disruption_host_structural_placements` (exact Spec `location_id`/`target_id` → existing `system_id`) as a **test-only** join. Production clause load has no cleared enrollment surface.

**Minimal DA clearance requested:** host-entity → generated `system_id` enrollment using existing structural authority only (no new Clause node field unless DA authorizes). Until cleared, production field-bearing open with typed disruption loci fails loud — correct, not OVL-ready.

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
| canonical_host_system_moves_zero_to_nonzero_unrelated_stays_zero | Absent wall; open already nonzero |
| authored_host_placement_swap_moves_system_id_with_zero_code_change | hard-coded system id |
| two_loci_in_one_system_report_exact_max | sum/first instead of production max |
| live_map_refreshes_when_runtime_disruption_changes | frozen nonzero open map |
| structural_shell_absent_field_stays_typed_zero | shell invents nonzero |
| selected_star_telemetry_matches_live_map_and_piecewise | 12.3/live divergence |
| structural_mapping_total_and_partial_miss_fail_loud | all-miss fail-soft blessing |
| field_bearing_unmapped_typed_loci_fail_loud_on_open | production unmapped open soft |
| observation_door_unknown_property_role_and_host_fail_loud | silent observe misses |

## Scope Ledger
| | |
|---|---|
| Specified | Selected-star disruption screen + live per-system STEAD readback |
| Implemented | typed disruption_presence loci + fail-loud mapping + observation seam + biting proofs |
| Proxied | CI Spec placement attach for host→system join only |
| Deferred | DA enrollment contract; Owner OVL; re-freeze |
| Out of scope | Spec mutation; WGSL; `location.ownership_volume`; 12.5; `DisruptionReadoutRecord::new` |

## Conformance
piecewise YES · clamp YES · deselect YES · live STEAD map YES (CI join) · placement swap YES · exact max YES · tick refresh YES · shell 0.0 YES · 12.3 match YES · fail-loud mapping YES · production enrollment NO · OVL BLOCKED

## Sticky disposition
Keep draft/open/unmerged. **STOP for DA** on host→generated-system enrollment. No ready / merge / self-graduation / 12.5 / re-freeze until orch accepts after DA clearance + corrected package.
