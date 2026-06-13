# BH3 Closeout PR7 Sample Import Results

> **Artifact lifecycle: PROBATION** (fresh PR7 canonical sample import proof; review for promotion,
> archive, or deletion by PR 9).

## Verdict

**PASS** — canonical ClauseScript sample `ct_bh3_closeout_sample` parses and lowers through all
accepted PR2–PR6 scenario-container surfaces into one coherent `HydratedScenarioPack`, with
default-off posture preserved and no runtime/movement/pathfinding semantics.

## Files changed

| Area | Path |
|---|---|
| Canonical example | `docs/clausething/examples/ct_bh3_closeout_sample.clause` |
| Examples index | `docs/clausething/examples/README.md` |
| Test fixture mirror | `crates/simthing-clausething/tests/fixtures/ct_bh3_closeout_sample.clause` |
| Sample import tests | `crates/simthing-clausething/tests/ct_scenario_container.rs` |
| ClauseThing spec | `docs/clausething/ClauseThing_Spec.md` |
| Closeout ladder | `docs/design_0_0_8_2_clausething_closeout_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/bh3_closeout_pr2..pr6_*` | PROBATION | Retained; not superseded by PR7 |
| Fable/BH2 review packets | CURRENT_EVIDENCE | Retained |
| `ct_scenario_container.rs` canonical sample tests | LIVE_GUARDRAIL | Extended (45 tests) |
| Scratch logs / duplicate reports / `target/` / worktrees | DELETE | None found |

## Deleted/superseded artifacts

None. PR2–PR6 per-PR reports remain PROBATION evidence for PR9 promotion review.

## Canonical sample path

- Authoring: `docs/clausething/examples/ct_bh3_closeout_sample.clause`
- Test mirror: `crates/simthing-clausething/tests/fixtures/ct_bh3_closeout_sample.clause`

## Sample contents

- Scenario metadata
- Three locations (`alpha`, `beta`, `gamma`) with properties, overlays on alpha/beta, cohort child on alpha
- Two bounded N4 links (`alpha→beta`, `alpha→gamma`) on row-major 2×2 grid placement
- One SaturatingFlux `field_operator` (`alpha_choke_flux`)
- One PALMA `palma_feedstock` (`alpha_wd`, default-off)
- One FIELD_POLICY `commitment` (`stabilize_alpha` with `attach_overlay` effect)

## Lowered generic shape

Single `HydratedScenarioPack` containing:

- `GameModeSpec` with properties, overlays, and patched `RegionFieldSpec`
- Root `World` + three `Location` children + cohort child
- `HydratedScenarioGridMetadata` with two canonical links
- `HydratedScenarioPalmaFeedstock`
- `HydratedScenarioCommitment` / `FirstSliceCommitmentSpec` on region field
- `MappingExecutionProfile::Disabled`

## Tests run

- `cargo test -p simthing-clausething --test ct_scenario_container` — 45 passed
- `cargo fmt --all -- --check` — pass
- `git diff --check` — pass

## Docs updated

- `docs/clausething/ClauseThing_Spec.md` — §3.5 canonical sample note
- `docs/clausething/examples/README.md` — sample pointer
- `docs/design_0_0_8_2_clausething_closeout_ladder.md` — PR7 PASS status + gap update
- `docs/design_0_0_8_1_clausething_production_track.md` — PR7 addendum

## Remaining risks

- Driver admit/install + GPU exercise for the sample is PR8
- Sample uses alpha-hub N4 links because row-major 2×2 placement cannot admit `beta→gamma` as N4
- No live threshold crossing or PALMA traversal proof in this PR

## Lifecycle classification for new artifacts

| Artifact | Classification |
|---|---|
| `docs/tests/bh3_closeout_pr7_sample_import_results.md` | PROBATION |
| `ct_scenario_container.rs` canonical sample tests | LIVE_GUARDRAIL |
| `docs/clausething/examples/ct_bh3_closeout_sample.clause` | CURRENT_EVIDENCE (canonical authoring reference) |
