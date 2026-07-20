# STUDIO-DISRUPTION-SELECT-SCREEN-0 Results

## Status
**PROBATION / DRAFT** — PR [#1420](https://github.com/khorum08/SimThing/pull/1420); dispatch `5022654370`; Remand 1 `5023235060`. Owner OVL remains OPEN. No freeze.

## Identity SHAs
| role | full SHA |
|---|---|
| base / post-12.10 master | `ced8ce2d80a24213676c4f606edd2126eb2bda6f` |
| handoff_head | `9f1949d9cd54c997414ea7a87004741e3c423500` |
| implementation_code_sha | `4b2db7e8f8d450c718c8006a60bc7182598be906` |
| tested_code_sha | `2bbfbbb91da0f9f0831cb0d24de340294eee76ca` |
| evidence_head_sha | Remand-1 evidence commit (PR tip; exact SHA in PR body) |
| final_head_sha | same as PR tip / Clearance head (bound in PR body, not recursively in this file) |

## Remand 1 corrections
1. Owned/neutral/hostile selection now reads a real `StudioDisruptionReadoutMap` with distinct admitted values via `selected_disruption_select_screen`; eligibility is `star_id == selected_id`; co-owned non-selected stays identity; absent selected id fails soft to identity.
2. Named dirty-gate proof: only selected disruption milli / applied blur+red change → global sync + selected per-star write; co-owned identity key does not rewrite.
3. Identities split: implementation vs tested-code vs evidence/final tip.
4. Duplicate TEST-BUDGET rows removed from `triage_log.tsv` and `inspect_justifications.tsv` (exactly one each).
5. Local governance at tested tip: orient `--selftest` PASS; agent_scan PASS; orientation `--check` PASS; doc-budget PASS; inventory drift PASS.

## What changed (implementation retained)
- Presentation mapper `studio_disruption_select_screen.rs`: exact piecewise 0/50/100 + clamp; deselect identity
- `sync_star_visuals_system` composes selected-star blur scale + red tint over 11.6 brighten base
- Dirty-gate keys carry quantized selected disruption / applied blur+red
- Studio_ops Telemetry rows: selected system id, raw disruption, blur scale, red fraction
- Minimal admitted-record constructor `DisruptionReadoutRecord::new` for falsifier map construction

## Proof matrix
| test | catches |
|---|---|
| breakpoints_and_above_100_clamp | wrong piecewise / missing clamp |
| deselect_restores_identity | sticky screen after clear |
| applies_to_owned_neutral_and_hostile_selection | wrong system / ignore readout / ownership gate / hard-coded selected |
| coexists_with_11_6_owned_brighten | co-owned set inherits disruption |
| live_disruption_invalidates_visual_dirty_gate | live accretion does not refresh visuals |
| no_wgsl_or_scenario_mutation_surface | kernel/Spec creep |

## Scope Ledger
| | |
|---|---|
| Specified | Selected-star disruption blur/tint; deselect restore; coexist with 11.6; telemetry rows |
| Implemented | mapeditor mapper + galaxy_render compose + dirty keys + Studio_ops rows + Remand-1 falsifiers |
| Proxied | admitted 12.2 `disruption_readout` (fail-soft 0 when absent) |
| Deferred | Owner OVL screenshot; freeze executable (post orch accept) |
| Out of scope | Spec mutation; WGSL; CPU planner; 12.5 fleet icons |

## Conformance
piecewise YES · clamp YES · deselect YES · admitted-readout selection YES · 11.6 coexist YES · dirty-gate YES · no Spec mut YES · no WGSL YES · OVL OPEN

## Sticky disposition
Expected `DA-RESERVE(unclassified-scope)`. Keep draft/open/unmerged. Owner alone closes OVL.

## Known residuals
- Next queued: `STUDIO-FLEET-ICONS-0` (12.5) — untouched here
