# BH3 Closeout PR6 FIELD_POLICY Threshold Results

> **Artifact lifecycle: PROBATION** (fresh PR6 scenario commitment proof; review for promotion,
> archive, or deletion by PR 7/9).

## Verdict

**PASS** — scenario containers now admit one top-level `commitment` block that lowers into
generic `FirstSliceCommitmentSpec` / optional `CommitmentEffectSpec` feedstock on the referenced
scenario `field_operator` region field, with default-off posture preserved and no CPU planner,
movement, route, or runtime semantics.

## Files changed

| Area | Path |
|---|---|
| Commitment hydrator | `crates/simthing-clausething/src/hydrate_scenario_commitment.rs` |
| Scenario composition | `crates/simthing-clausething/src/hydrate_scenario.rs` |
| Shared effect parser export | `crates/simthing-clausething/src/hydrate_category_economy.rs` |
| Public exports | `crates/simthing-clausething/src/lib.rs` |
| Scenario tests | `crates/simthing-clausething/tests/ct_scenario_container.rs` |
| Fixture | `crates/simthing-clausething/tests/fixtures/ct_scenario_container_with_commitment.clause` |
| ClauseThing spec | `docs/clausething/ClauseThing_Spec.md` |
| Closeout ladder | `docs/design_0_0_8_2_clausething_closeout_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/bh3_closeout_pr2..pr5_*` | PROBATION | Retained |
| Fable/BH2 review packets | CURRENT_EVIDENCE | Retained |
| `ct_scenario_container.rs` | LIVE_GUARDRAIL | Extended (42 tests) |
| Scratch logs / duplicate reports / `target/` / worktrees | DELETE | None found |

## Deleted/superseded artifacts

None.

## Scenario FIELD_POLICY / commitment grammar supported

```clause
commitment = stabilize_alpha {
    threshold = 0.75
    event_kind = 7
    field_urgency = {
        source = alpha_choke_flux
        column = 2
        weight = 1.0
    }
    effect = {
        attach_overlay = alpha_pressure_bonus
        target = alpha
    }
}
```

Requires a scenario `field_operator` with matching header id. One block per scenario.

## Lowered generic shape

- `HydratedScenarioCommitment` on `HydratedScenarioPack`
- Patched scenario `RegionFieldSpec`:
  - `parent_formula` (`field_urgency` weights)
  - `reduction` (grid-derived parent slot)
  - `commitment` (`FirstSliceCommitmentSpec` with `urgency_col = 4`)
- Optional `CommitmentEffectSpec` via CT-3b+4a shape or scenario `attach_overlay` resolution

## Guardrail/rejection cases

- Missing `threshold`
- Non-finite `threshold`
- Missing `field_urgency` / `source`
- Unknown `field_urgency.source`
- Invalid/out-of-range `column` (must match `choke_output_col` when present)
- Non-finite weight
- Unknown `attach_overlay` / effect target
- Duplicate commitment block
- Forbidden route/path/movement/pathfinding/border/frontline/arbitrary_graph/non_grid_topology fields
- `enabled = true`

## Tests run

- `cargo test -p simthing-clausething --test ct_scenario_container` — 42 passed
- `cargo fmt --all -- --check` — pass
- `git diff --check` — pass

## Docs updated

- `docs/clausething/ClauseThing_Spec.md` — §3.4 commitment/threshold note
- `docs/design_0_0_8_2_clausething_closeout_ladder.md` — PR6 PASS status + gap update
- `docs/design_0_0_8_1_clausething_production_track.md` — PR6 addendum

## Remaining risks

- Canonical end-to-end sample (PR7) and driver admit/install exercise (PR8) still outstanding
- Scenario commitment assumes first-slice `urgency_col = 4` doctrine from existing admission surfaces
- No live GPU threshold crossing exercise in this PR

## Lifecycle classification for new artifacts

| Artifact | Classification |
|---|---|
| `docs/tests/bh3_closeout_pr6_field_policy_threshold_results.md` | PROBATION |
| `ct_scenario_container.rs` commitment tests | LIVE_GUARDRAIL |
