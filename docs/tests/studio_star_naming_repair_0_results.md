# STUDIO-STAR-NAMING-REPAIR-0 Results

## Status
**PROBATION / DA-RESERVE** - [PR #1316](https://github.com/khorum08/SimThing/pull/1316); merge: **NOT MERGED**.

## Identity
| Field | Value |
|---|---|
| branch | `codex/studio-star-naming-repair-0` |
| stacked base | `codex/studio-frosted-glass-exit-stamp` ([#1315](https://github.com/khorum08/SimThing/pull/1315)) |
| tested code SHA | `a130cacab88e9501c9902c5a9b3279d818a957df` |
| route | DA-reserve; no self-merge |

## Corrected Diagnosis / Repair
- The canonical base-disc already carried 1,500 deterministic display-name entries and a golden rewrite produced no diff.
- Embedded ClauseScript hydration read those names, then dropped the metadata while rebuilding namespaced authority gridcells.
- `HydratedEmbeddedStaticGalaxyScenario` now retains names keyed by namespaced system ID; `attach_embedded_gridcells` reapplies them through `apply_star_system_display_name_metadata`.
- No grammar, resolver, generator, Spec, picker, mapeditor render, selection, or UI source changed.

## Proof Matrix
| proof | result |
|---|---|
| production canonical clause load resolves all systems | PASS: 1,500/1,500 |
| names non-empty and not hex fallback | PASS |
| two deterministic regenerations | PASS |
| canonical JSON byte-current | PASS; no data diff |
| placements, links, owners preserved | PASS |
| mapeditor render sources unchanged | PASS |
| ClauseScript loader regression | PASS |
| 11.2 faction identity regression | PASS |

## Validation
- `cargo check`: spec, mapgenerator, clausething, mapeditor PASS.
- Full `simthing-spec` and `simthing-mapgenerator` suites PASS.
- Repair target 8/8; naming pass 4/4; faction identity 8/8; faction nameplates 10/10 PASS.
- Canonical scenario target 4/4 PASS in an isolated clean worktree, independent of the shared workspace's operator-owned untracked sibling output; that artifact remains untracked and is excluded from this PR.
- Windows debug `simthing-studio.exe` rebuilt; native production picker loaded the canonical clause and reported 1,500 systems / 2,714 links / StructuralRebindReady hydrate PASS; rendered labels were semantic names, not hex IDs.
- TEST-BUDGET green triage and inspect justification recorded.

## Seal
`scenarios/terran_pirate_galaxy.base_disc.json` carries 1,500 per-system display names. The file was already correct on merged master; this rung repairs retention through production hydration and does not fabricate a canonical data diff.

## Graduation Routing
DA review required. Self-merge is not permitted.
