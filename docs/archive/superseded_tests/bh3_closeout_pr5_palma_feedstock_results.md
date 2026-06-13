# BH3 Closeout PR5 PALMA Feedstock Results

> **Artifact lifecycle: PROBATION** (fresh PR5 scenario PALMA feedstock proof; review for promotion,
> archive, or deletion by PR 7/9).

## Verdict

**PASS** — scenario containers now admit one top-level `palma_feedstock` block that lowers into
generic `HydratedScenarioPalmaFeedstock` metadata bound to an existing scenario `field_operator`
id, with default-off posture preserved and no pathfinding/movement/route semantics.

## Files changed

| Area | Path |
|---|---|
| PALMA feedstock hydrator | `crates/simthing-clausething/src/hydrate_palma_feedstock.rs` |
| Scenario composition | `crates/simthing-clausething/src/hydrate_scenario.rs` |
| Public exports | `crates/simthing-clausething/src/lib.rs` |
| Scenario tests | `crates/simthing-clausething/tests/ct_scenario_container.rs` |
| Fixture | `crates/simthing-clausething/tests/fixtures/ct_scenario_container_with_palma_feedstock.clause` |
| ClauseThing spec | `docs/clausething/ClauseThing_Spec.md` |
| Closeout ladder | `docs/design_0_0_8_2_clausething_closeout_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |
| PALMA guide | `docs/design_0_0_8_1_palma_pathfinding_integration_guide.md` |

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/bh3_closeout_pr2..pr4_*` | PROBATION | Retained |
| Fable/BH2 review packets | CURRENT_EVIDENCE | Retained |
| `ct_scenario_container.rs` | LIVE_GUARDRAIL | Extended (32 tests) |
| Scratch logs / duplicate reports / `target/` / worktrees | DELETE | None found |

## Deleted/superseded artifacts

None.

## Scenario PALMA feedstock grammar supported

```clause
palma_feedstock = alpha_wd {
    w_source = alpha_choke_flux
    w_output_col = 3
    d_output_col = 4
    enabled = false
}
```

Requires a scenario `field_operator` with matching header id. One block per scenario.

## Lowered generic shape

`HydratedScenarioPalmaFeedstock` on `HydratedScenarioPack`:

- `feedstock_id`, `w_source_field_operator_id`
- `w_output_col`, `d_output_col`
- `grid_size`, `n_dims`, `source_col`, `choke_output_col` (from referenced field operator)

Inert DTO for later `composed_w_min_plus_stencil_config` / driver install (PR8). No GPU, driver,
or `simthing-sim` changes in PR5.

## Guardrail/rejection cases

- missing `w_source`, `w_output_col`, or `d_output_col`
- unknown `w_source` field_operator id
- palma without field_operator
- column out of range / aliased with `source_col` or each other
- second `palma_feedstock` block
- `enabled = true`
- route/path/predecessor/movement/border/frontline/pathfinding vocabulary inside block

## Tests run

```text
cargo test -p simthing-clausething --test ct_scenario_container
cargo fmt --all -- --check
git diff --check
```

## Docs updated

- `docs/clausething/ClauseThing_Spec.md` §3.3
- `docs/design_0_0_8_2_clausething_closeout_ladder.md` PR5 status + census row
- `docs/design_0_0_8_1_clausething_production_track.md` PR5 addendum
- `docs/design_0_0_8_1_palma_pathfinding_integration_guide.md` PR5 addendum

## Remaining risks

- PR6 still owns FIELD_POLICY threshold feedstock unification at scenario level.
- PR7/8 still own canonical sample + driver admit/install/GPU exercise consuming this DTO.

## Lifecycle classification for new artifacts

`docs/tests/bh3_closeout_pr5_palma_feedstock_results.md`: **PROBATION** — review at PR 7/9.
