# STUDIO-FACTION-IDENTITY-FIELDS-0 Results

## Status
**DA-GRADUATED / COMPLETE** — merged [#1302](https://github.com/khorum08/SimThing/pull/1302) @ `f18efd1b`.

## PR / branch / merge
| Field | Value |
|---|---|
| PR | [#1302](https://github.com/khorum08/SimThing/pull/1302) |
| branch | `studio-faction-identity-fields-0` |
| base | `master` |
| head_sha | `9da40fc5240646da9692788485692dd72d3e385a` |
| merge | `f18efd1b` (squash) |

## What changed
- Spec: `OWNER_COLOR_RGB` / `OWNER_FACTION_NAME` / `OWNER_FACTION_ALLIANCE` + reserved slots; helpers `owner_faction_*` / `parse_color_rgb_text` / `format_color_rgb`
- Clausething: owner grammar `color_rgb` / `faction_name` / `faction_alliance`; required color when identity engaged; hydrate onto Owner SimThings
- Canonical TP: Terran `64,160,255` / Pirate `220,64,48` + names + alliance `none`
- 8 headless proofs + TEST-BUDGET triage

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
| | |
|---|---|
| Specified | Faction identity authority + clause hydrate + canonical colors |
| Implemented | Spec props/helpers + clausething grammar/hydrate + canonical TP values + proofs |
| Proxied | none |
| Deferred | 11.3 star names; 11.4 source_base Studio wire; UI rungs |
| Out of scope | mapeditor UI, mapgenerator, driver/kernel, gameplay/diplomacy |

## Conformance
color_rgb authority YES · faction_name YES · alliance placeholder-only YES · distinct TP colors YES · fail-loud YES · empty-resolver retained YES · no UI/gameplay YES · triage_log TEST-BUDGET YES

## Known residuals
- 11.4 still owns production ingest `source_base` wire
- Next: `STUDIO-STAR-NAMING-PASS-0` (11.3, Frontier)

## Graduation routing
**DA PASS** — sticky `class-envelope-violation` is expected Tier-A (spec+clausething outside live-ops UI class). Pointer → `STUDIO-STAR-NAMING-PASS-0`.

## DA ACK
```text
ANCHOR-ACK: clausething-closed-vertical@beb30ffaba50
ANCHOR-ACK: one-tree-owners-never-spatial@c88002b72898
ANCHOR-ACK: stead-spatial-contract-core@b4a112cd02e8
ANCHOR-ACK: orientation-harness-core@8a365d1c0864
```
