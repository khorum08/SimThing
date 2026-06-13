# BH3-CLOSEOUT-PR2 - scenario-container grammar

> **Artifact lifecycle: PROBATION** (fresh PR2 parse/lower proof; review for promotion, archive,
> or deletion by PR 7/9).

## Verdict

**PASS** - `hydrate_scenario` imports a minimal multi-location ClauseScript scenario container and
lowers it to existing generic surfaces without adding runtime semantics.

## Lifecycle audit before implementation

| Artifact | Classification | Action |
|---|---|---|
| `docs/design_0_0_8_2_clausething_closeout_ladder.md` | LIVE LEDGER | Updated with PR2 status and this PROBATION report |
| `docs/tests/bh3_authoring_0_results.md` | PROBATION | Retained; PR4/PR9 lifecycle review unchanged |
| `docs/tests/fable_review_0_0_8_1_result.md` | CURRENT_EVIDENCE | Retained |
| `docs/tests/fable_review_bh2_track_packet.md` | CURRENT_EVIDENCE | Retained |
| `docs/tests/bh2d_ct4b_100tick_scenario_observations.md` | CURRENT_EVIDENCE | Retained |
| `docs/tests/r1_default_workspace_purge_results.md` | CURRENT_EVIDENCE | Retained |
| `crates/simthing-clausething/tests/` closed CT hydrator tests | LIVE_GUARDRAIL | Retained |
| `crates/simthing-clausething/tests/fixtures/bh3_*.clause` | PROBATION | Retained for PR4 |
| Scratch logs / duplicate reports / committed `target/` / worktrees | DELETE | None found in tracked PR2 scope |
| This report | PROBATION | Added |

No stale active proof scaffolding was found that needed deletion before PR2.

## Implementation

Files added or updated:

| Area | Path |
|---|---|
| Scenario hydrator | `crates/simthing-clausething/src/hydrate_scenario.rs` |
| Public API export | `crates/simthing-clausething/src/lib.rs` |
| Focused guardrail test | `crates/simthing-clausething/tests/ct_scenario_container.rs` |
| Original fixture | `crates/simthing-clausething/tests/fixtures/ct_scenario_container_minimal.clause` |
| ClauseThing spec | `docs/clausething/ClauseThing_Spec.md` |
| Closeout ladder | `docs/design_0_0_8_2_clausething_closeout_ladder.md` |
| 0.0.8.1 production-track addendum | `docs/design_0_0_8_1_clausething_production_track.md` |

Lowering shape:

- `GameModeSpec` receives flattened `PropertySpec` and `OverlaySpec` declarations.
- Root `SimThingKind::World` owns `SimThingKind::Location` children.
- ClauseThing retains authored node ids, display names, properties, overlays, and child
  declarations in `HydratedScenarioNode`.
- Overlays use existing `InstallTargetSpec::ScenarioListed` target ids.
- `install_targets` is emitted in the same id-to-`SimThingId` shape the driver scenario surface
  consumes, without adding a driver dependency to `simthing-clausething`.

## Explicit non-scope

PR2 does not add adjacency/link lowering, graph objects, routes, paths, predecessors, movement,
frontline/border services, PALMA W/D authoring, SaturatingFlux closure, FIELD_POLICY unification,
driver install closure, GPU kernels, Bevy/editor code, semantic WGSL, or `simthing-sim`
awareness. Candidate F is not implicated.

## Tests run

```text
cargo test -p simthing-clausething --test ct_scenario_container
cargo fmt --all -- --check
```

## Guardrails covered

- Minimal multi-location scenario parses and lowers.
- Root has multiple `Location` children.
- Location properties, overlays, and child declarations survive lowering.
- Overlay install targets preserve authored node identity through `ScenarioListed`.
- Generic `GameModeSpec` JSON stays free of link/route/path/predecessor fields.
- Duplicate location ids are rejected.
- `link`, `route`, `path`, and `predecessor` are rejected as outside PR2 grammar.
- Custom/deprecated child kinds are rejected rather than adding new scenario nouns.

## Remaining closeout work

PR3 still owns adjacency/link grammar and bounded topology lowering. PR4/5/6 still own
SaturatingFlux authoring closure, PALMA W/D feedstock, and FIELD_POLICY threshold feedstock under
the scenario container. PR7/8 still own the canonical sample and driver admit/install/GPU exercise.
