# STUDIO-FACTION-IDENTITY-FIELDS-0 Results

## Status
**PROBATION** — not complete; not graduated. DA-reserve by envelope.

## PR / branch / merge
| Field | Value |
|---|---|
| PR | [#1302](https://github.com/khorum08/SimThing/pull/1302) |
| branch | `studio-faction-identity-fields-0` |
| base | `master` |
| head_sha | `cb4c5fbb0f027e7fe9cabe57e68e4a1558771cbe` |
| merge | NOT MERGED |

## What changed
- Spec: `OWNER_COLOR_RGB` / `OWNER_FACTION_NAME` / `OWNER_FACTION_ALLIANCE` + reserved slots; helpers `owner_faction_*` / `parse_color_rgb_text`
- Clausething: owner grammar `color_rgb` / `faction_name` / `faction_alliance`; required color when identity engaged; hydrate onto Owner SimThings
- Canonical TP: Terran `64,160,255` / Pirate `220,64,48` + names + alliance `none`
- Tests: `studio_faction_identity_fields_0` (8 proofs); TEST-BUDGET triage

## Proof matrix
| test | catches |
|---|---|
| fields_roundtrip_authority | missing serde/helpers |
| clause_hydrates_owner_fields | grammar not writing authority |
| canonical_tp_has_distinct_colors | same/absent colors |
| canonical_tp_names_present | missing names |
| rejects_malformed_color_rgb | silent bad parse |
| missing_required_color_fails_loud | silent default color |
| 11_1_canonical_load_still_empty_resolver | portable load regression |
| no_ui_or_gameplay_semantics | mapeditor/gameplay leakage |

## Scope Ledger
Specified: faction identity authority + clause hydrate + canonical colors
Implemented: as above
Deferred: 11.3 star names; 11.4 source_base Studio wire; UI rungs
Out of scope: mapeditor/mapgen/driver/kernel/gameplay

## Conformance
color_rgb YES · faction_name YES · alliance placeholder YES · distinct TP colors YES · fail-loud YES · empty-resolver retained YES · no UI/gameplay YES

## Graduation routing
**PROBATION** — expect DA-RESERVE. Do not self-merge.
