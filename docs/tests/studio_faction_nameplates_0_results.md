# STUDIO-FACTION-NAMEPLATES-0 Results

## Status
**ORCHESTRATOR-GRADUATED / COMPLETE** — merged [#1309](https://github.com/khorum08/SimThing/pull/1309) @ `9ee45b3f`.

## PR / branch / merge
| Field | Value |
|---|---|
| PR | [#1309](https://github.com/khorum08/SimThing/pull/1309) |
| branch | `studio-faction-nameplates-0` |
| base | `master` |
| head_sha | `c32e4c0d4948b727350c5f62c58f69d6af9e6126` |
| merge | `9ee45b3f8957ae82b2c0784441a24fb65bc2d4c1` |

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

## Scope Ledger
| | |
|---|---|
| Specified | Star nameplates: authority display names + owner color_rgb; unowned neutral |
| Implemented | mapeditor presentation helpers + galaxy_render wire + 10 proofs + docs/inventory |
| Proxied | none |
| Deferred | planet labels (no subsystem); 11.6 owned-star brighten; 11.7 frosted glass |
| Out of scope | Spec mutation; selection-model authority; WGSL; kernel/sim/driver |

## Conformance
authority names YES · color_rgb YES · unowned neutral YES · no Spec mutation YES · no 11.6/11.7 YES · mapeditor presentation only YES

## Sticky disposition
`ORCHESTRATOR-CLEARABLE` on #1309 after CI green (Clearance + Doctrine Scan + Doctrine Exec). Orchestrator merge.

## Known residuals
- Next: `STUDIO-OWNED-STAR-SELECT-BRIGHTEN-0` (11.6, Std)

## Graduation routing
**ORCHESTRATOR-GRADUATED**. Pointer → `STUDIO-OWNED-STAR-SELECT-BRIGHTEN-0`.
