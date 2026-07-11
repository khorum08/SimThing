# STUDIO-FACTION-NAMEPLATES-0 Results

## Status
**PROBATION** — not complete; not graduated.

## PR / branch / merge
| Field | Value |
|---|---|
| PR | [#1309](https://github.com/khorum08/SimThing/pull/1309) |
| branch | `studio-faction-nameplates-0` |
| base | `master` |
| head_sha | `aa412094ba6f63aa1a550392d0ebd2c0dc0e1533` |
| merge | NOT MERGED |

## What changed
- `studio_faction_nameplates.rs`: owner color map + star nameplate RGBA from authority `color_rgb` / `owner_flow_owner_ref`; unowned = neutral
- `galaxy_render.rs`: GalaxyStarNameplate uses `star_nameplate_presentations` (name + faction color)
- Planet labels: not invented (no existing planet nameplate subsystem; star path only)
- 10 headless proofs; TEST-BUDGET triage

## Proof matrix
| test | catches |
|---|---|
| use_authority_display_names | wrong/missing names |
| apply_owner_color_rgb | owned color not applied |
| unowned_are_neutral | unowned not neutral |
| do_not_mutate_scenario_spec | presentation mutates Spec |
| no_selection_brighten | 11.6 creep |
| no_frosted_glass_or_wgsl | 11.7/WGSL creep |
| galaxy_render_uses_helper | render path bypass |
| unknown_owner_ref_is_neutral | silent wrong color |
| no_authority_crate_imports | scope leak |

## Conformance
authority names YES · color_rgb YES · unowned neutral YES · no Spec mutation YES · no 11.6/11.7 YES · mapeditor presentation only YES

## Graduation routing
**PROBATION** — expected class `studio-live-ops-ui-clock` → ORCHESTRATOR-CLEARABLE. Do not self-merge.
