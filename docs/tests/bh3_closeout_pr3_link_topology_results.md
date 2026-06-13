# BH3 Closeout PR3 Link Topology Results

## Verdict

PASS / PROBATION evidence. PR3 adds scenario-contained `link = { from to }` grammar and lowers it
to bounded grid-placement metadata without adding route, pathfinding, arbitrary graph, driver,
GPU, or `simthing-sim` semantics.

Merge status: DA REVIEW REQUIRED BEFORE MERGE.

## Files changed

- `crates/simthing-clausething/src/hydrate_scenario.rs`
- `crates/simthing-clausething/src/lib.rs`
- `crates/simthing-clausething/tests/ct_scenario_container.rs`
- `crates/simthing-clausething/tests/fixtures/ct_scenario_container_with_links.clause`
- `docs/clausething/ClauseThing_Spec.md`
- `docs/design_0_0_8_1_clausething_production_track.md`
- `docs/design_0_0_8_2_clausething_closeout_ladder.md`
- `docs/tests/bh3_closeout_pr3_link_topology_results.md`

## Artifact lifecycle audit

Opened by rereading the 0.0.8.2 ladder lifecycle census and the PR2 scenario-container report.
PR2 remains PROBATION evidence. Fable/BH2 reports remain CURRENT_EVIDENCE. Active
`simthing-clausething` parser/lowering tests remain LIVE_GUARDRAIL.

## Deleted/superseded artifacts

None. No scratch, target, worktree, or duplicate proof artifacts were introduced or found during
the PR3 pass.

## Scenario link grammar supported

Top-level scenario blocks now admit:

```clause
link = {
    from = alpha
    to = beta
}
```

Links are scenario-level only. Nested links are rejected. Endpoints must be existing top-level
locations, and self-links are rejected.

## Lowered generic shape

`HydratedScenarioPack` now carries `HydratedScenarioGridMetadata`:

- `grid_size`: deterministic square grid edge length.
- `placements`: row/column cell placements keyed by scenario install-target id.
- `links`: canonical, de-duplicated location pairs.
- `max_fanout`: PR3 N4 fanout cap.

This mirrors the RegionField placement vocabulary (`target_id`, row, col) at ClauseThing
hydration time. It is not installed into `simthing-sim`, not a graph object, and not a topology
engine.

## Guardrail/rejection cases

Covered:

- valid multi-location scenario with one link parses and lowers;
- unknown link endpoint rejected;
- duplicate and reversed links canonicalize deterministically;
- fanout above the N4 cap rejected;
- non-N4/diagonal links rejected rather than becoming arbitrary topology;
- nested links rejected;
- route/path/predecessor/movement/border/frontline/pathfinding/arbitrary graph/non-grid topology
  vocabulary rejected.

## Tests run

- `cargo test -p simthing-clausething --test ct_scenario_container`

## Docs updated

- `docs/clausething/ClauseThing_Spec.md`: scenario container PR3 grammar and constraints.
- `docs/design_0_0_8_1_clausething_production_track.md`: PR3 closeout addendum.
- `docs/design_0_0_8_2_clausething_closeout_ladder.md`: PR3 status and report classification.

## DA review status

Required before merge. The PR description must carry `DA REVIEW REQUIRED BEFORE MERGE`.

## Remaining risks

Row-major placement is intentionally narrow. It admits only links representable as N4 neighbors
under deterministic placement. Non-grid or arbitrary topology remains deferred to a future
topology-spec rung if a consumer proves the need.

## Lifecycle classification for new artifacts

`docs/tests/bh3_closeout_pr3_link_topology_results.md`: PROBATION. Review at PR7/PR9 for
promotion, archive, or deletion.
