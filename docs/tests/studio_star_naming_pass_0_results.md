# STUDIO-STAR-NAMING-PASS-0 Results

## Status
**DA-GRADUATED / COMPLETE** — merged [#1304](https://github.com/khorum08/SimThing/pull/1304) @ `052cc192`.

## PR / branch / merge
| Field | Value |
|---|---|
| PR | [#1304](https://github.com/khorum08/SimThing/pull/1304) |
| branch | `codex/studio-star-naming-pass-0` |
| base | `master` |
| head_sha | `3c3cbee75dea7e3bfb168acd3e5458f0672f7526` |
| merge | `052cc192` (squash) |

## What changed
- Isolated 4,096-entry syllabic catalogue shuffled by domain-separated `MapGenRng` (`seed ^ NAMING_SEED_DOMAIN`)
- Assignment sorts/dedups system IDs; uniqueness across catalogue cycles via cycle suffix
- `ScenarioEmitter` writes assigned names (no blank `name = ""`)
- Canonical TP base-disc repaired: 1,500 unique names; placements/links unchanged
- Existing Spec `star_system_display_name` sufficient; no Spec source change
- 9 inventoried proofs + TEST-BUDGET triage

## Proof matrix
| test | catches |
|---|---|
| assigns_non_empty_names | blank naming |
| is_seed_stable | seed drift / input-order sensitivity |
| preserves_structure | naming mutating placement/topology text |
| names_are_unique_within_galaxy | duplicates across catalogue cycles |
| emitter_writes_names_not_blank | ScenarioEmitter blank names |
| canonical_tp_all_systems_have_display_names | stale golden / missing names |
| spec_helper_resolves_all_canonical_systems | Spec helper gaps |
| 11_1_empty_resolver_still_loads | portable load regression |
| 11_2_faction_identity_retained | Terran/Pirate identity loss |

## Scope Ledger
| | |
|---|---|
| Specified | Seed-stable naming + emitter + canonical repair + 11.1/11.2 hold |
| Implemented | mapgenerator star_names + emitter + canonical golden + proofs |
| Proxied | none |
| Deferred | 11.4 loader + source_base wire; 11.5–11.7 |
| Out of scope | mapeditor UI, driver/kernel/GPU, gameplay, Spec source |

## Conformance
deterministic YES · isolated naming RNG YES · sorted IDs YES · catalogue uniqueness YES · emitter non-blank YES · 1500 unique canonical YES · structure preserved YES · Spec helper YES · 11.1/11.2 YES · triage YES

## Sticky disposition
Auto-posted sticky may have shown `unclassified-scope` on an earlier body; current head clears as `DA-RESERVE(admitted-scope-router-gap)` — expected Phase-11 Tier-A router debt (no mapgenerator class). Not remedial.

## Known residuals
- 11.4 owns production Studio ingest `source_base` wire + ClauseScript-only loader UI
- Next: `STUDIO-CLAUSE-LOADER-SIMPLIFY-0` (11.4, Std)

## Graduation routing
**DA PASS**. Pointer → `STUDIO-CLAUSE-LOADER-SIMPLIFY-0`.

## DA ACK
```text
ANCHOR-ACK: clausething-closed-vertical@beb30ffaba50
ANCHOR-ACK: movement-front@a0592b2f37ca
ANCHOR-ACK: orientation-harness-core@8a365d1c0864
ANCHOR-ACK: stead-spatial-contract-core@b4a112cd02e8
```
