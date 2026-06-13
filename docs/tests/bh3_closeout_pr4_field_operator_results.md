# BH3 Closeout PR4 Field Operator Results

> **Artifact lifecycle: PROBATION** (fresh PR4 scenario-contained field-operator proof; review for
> promotion, archive, or deletion by PR 7/9).

## Verdict

**PASS** — scenario containers now admit one top-level `field_operator` block that lowers through
the existing BH-3 hydrator into generic `RegionFieldSpec` / optional compose surfaces with
default-off posture preserved.

## Files changed

| Area | Path |
|---|---|
| Shared BH-3 hydrator + PR4 guardrails | `crates/simthing-clausething/src/hydrate_field_operator.rs` |
| Scenario composition | `crates/simthing-clausething/src/hydrate_scenario.rs` |
| Public exports | `crates/simthing-clausething/src/lib.rs` |
| Scenario tests | `crates/simthing-clausething/tests/ct_scenario_container.rs` |
| BH-3 parse tests | `crates/simthing-clausething/tests/bh3_authoring_parse.rs` |
| Fixture | `crates/simthing-clausething/tests/fixtures/ct_scenario_container_with_field_operator.clause` |
| ClauseThing spec | `docs/clausething/ClauseThing_Spec.md` |
| Closeout ladder | `docs/design_0_0_8_2_clausething_closeout_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |
| Border Hack track | `docs/design_0_0_8_1_border_hack_track.md` |

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/bh3_authoring_0_results.md` | PROBATION | Retained; superseded as primary proof by this report for scenario path |
| `docs/tests/bh3_closeout_pr2_scenario_container_results.md` | PROBATION | Retained |
| `docs/tests/bh3_closeout_pr3_link_topology_results.md` | PROBATION | Retained |
| Fable/BH2 review packets | CURRENT_EVIDENCE | Retained |
| `ct_scenario_container.rs`, `bh3_authoring_parse.rs` | LIVE_GUARDRAIL | Extended |
| Scratch logs / duplicate reports / `target/` / worktrees | DELETE | None found |

## Deleted/superseded artifacts

None.

## Scenario field_operator grammar supported

Top-level scenario blocks now admit one header-block field operator using the existing BH-3
`saturating_flux { u_sat chi choke_output_col }` shape plus the accepted grid/column fields.
Standalone top-level field-operator documents remain supported via `hydrate_field_operator_pack`.

## Lowered generic shape

`hydrate_scenario` calls `hydrate_field_operator_property` and merges:

- `game_mode.region_fields` from the operator pack
- `game_mode.mapping_execution_profile = Disabled`
- optional `w_impedance_compose` / `stress_compose` on `HydratedScenarioPack`

No `simthing-sim`, driver, GPU, PALMA, FIELD_POLICY-unification, route, movement, border, or
frontline surfaces changed.

## Guardrail/rejection cases

- missing `u_sat`
- `chi > 0.25` (CFL, dt=1.0)
- non-finite numeric literals
- `choke_output_col` out of range / equal to `source_col`
- second scenario `field_operator` block
- forbidden service vocabulary inside field_operator (`border`, `route`, `pathfinding`, …)
- PR3 link/location guardrails unchanged

## Tests run

```text
cargo test -p simthing-clausething --test ct_scenario_container
cargo test -p simthing-clausething --test bh3_authoring_parse
cargo fmt --all -- --check
git diff --check
```

## Docs updated

- `docs/clausething/ClauseThing_Spec.md` — PR4 scenario field_operator note
- `docs/design_0_0_8_2_clausething_closeout_ladder.md` — PR4 PASS + census row
- `docs/design_0_0_8_1_clausething_production_track.md` — PR4 addendum
- `docs/design_0_0_8_1_border_hack_track.md` — BH-3 authoring closed in scenario path

## Remaining risks

- PR5 still owns PALMA W/D feedstock grammar under scenario containers.
- PR6 still owns FIELD_POLICY threshold feedstock unification at scenario level.
- PR7/8 still own canonical sample + driver admit/install/GPU exercise.

## Lifecycle classification for new artifacts

`docs/tests/bh3_closeout_pr4_field_operator_results.md`: **PROBATION** — review at PR 7/9.
