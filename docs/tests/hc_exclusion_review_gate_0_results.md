# HC-EXCLUSION-REVIEW-GATE-0 Results

## Status

**PROBATION / proof-present / DA-review-pending.** Gate-wiring rung; do not self-graduate, undraft, merge, or advance progression.

Rung: HC-EXCLUSION-REVIEW-GATE-0
HD-RECEIPT: ad128d8775a8
ORIENT-RECEIPT: c866259aba5e
Classification: gate-wiring
ANCHOR-ACK: orientation-harness-core@8a365d1c0864

## What changed

- Deleted `role-resolution-exclude-site` from the `SPEC-LOWERER-KIND-READ` exclusion column in `scripts/ci/scans.tsv`.
- Preserved the existing trap fixture identity and changed `doctrine_selftest.sh` to synthesize a marker-bearing sample in its temporary sandbox, avoiding durable fixture-ledger growth.
- Updated fixture inventory and scan docs so the generic marker is no longer documented as a valid suppression path.
- Stamped the HC-1 ladder row with PROBATION leading the exit-proof cell and regenerated orientation.

## Exclusion Census

| Scan | Exclusion tokens | Classification | Action |
|---|---|---|---|
| `B3-BUFFER-ESCAPE` | `pub\(crate\)`, `compile_fail`, doc/comment filters | scanner hygiene, not role-resolution self-service | kept |
| `FORGE-MINTERS` | `compile_fail`, doc/comment filters | scanner hygiene, not role-resolution self-service | kept |
| `UNSAFE-FN` | `compile_fail`, doc/comment filters | scanner hygiene, not role-resolution self-service | kept |
| `COLUMN-INDEX-MINT` | `column_index\.rs`, `registry\.rs`, accumulator/oracle/runtime module names, `compile_fail`, doc/comment filters | DA-authored named symbols plus scanner hygiene | kept |
| `SIM-KIND-READ` | `compile_fail`, doc/comment filters, `delta_log`, `sim_runtime_tree`, `kind_production_audit`, assert/test filters | DA-authored named symbols plus scanner hygiene | kept |
| `SEMANTIC-WORDS` | `compile_fail`, doc/comment filters, assert/test/`SimThingKind::` filter | scanner hygiene, not role-resolution self-service | kept |
| `SPEC-STRING-CHANNEL` | `compile_fail`, doc/comment filters, `channel_key\.rs` | DA-authored named symbol plus scanner hygiene | kept |
| `SPEC-LOWERER-KIND-READ` | `role-resolution-exclude-site` | generic self-service role-resolution token | deleted |
| `SPEC-LOWERER-KIND-READ` | `planet_non_grid_child_kind_label`, `is_admitted_planet_non_grid_child`, `scenario_deferral_kind_label`, `planet_child_location_error_kind_label`, `simthing_kind_label`, `location_participant_kind_label`, `non_location_participant_kind_label` | DA-authored named symbols | kept |
| `SPEC-LOWERER-KIND-READ` | `compile_fail`, doc/comment filters, assert/test filters | scanner hygiene, not role-resolution self-service | kept |

No live crate users of `role-resolution-exclude-site` were present; only the handoff, historical docs, and the selftest-generated sample carry the text after this diff.

## Falsifier

The selftest-generated `role_resolution_exclude_site_kind_param_match` sample still bears `role-resolution-exclude-site`. Current `doctrine_selftest.sh` expects it to produce `SPEC-LOWERER-KIND-READ` INSPECT. Applied to the pre-fix `scans.tsv`, the sample is excluded and the new selftest expectation fails; applied after this diff, it passes. Green-both-ways is avoided without adding a durable fixture row.

## Validation

- PASS: `bash scripts/ci/doctrine_selftest.sh`
- PASS: `bash scripts/ci/gen_orientation.sh --check`
- PASS: `bash scripts/ci/doc_budget_check.sh --check`
- PASS: live 0.0.8.6 parked-track round-trip in a disposable worktree:
  `ORIENTATION-UNPARK-VERDICT: UNPARKED receipt=19e0e85c8d3f`,
  `restored_rows: 1`, `restored_handoffs: 1`, and
  `LIVE-0086-UNPARK-PROOF: PASS receipt=19e0e85c8d3f restored_rows=1 restored_handoffs=1 active_pointer=docs/design_0_0_8_6_studio_live_ops.md`.
- PASS: `bash scripts/ci/test_inventory_drift_check.sh`
- PASS: `bash scripts/ci/track_closeout.sh --deletion-guard origin/master HEAD`
  (`TRACK-CLOSEOUT-DELETION-GUARD-VERDICT: PASS removed=0`).
- PASS: `git diff --check`
- PASS: explicit falsifier sandbox: pre-fix `scans.tsv` reports `PREFIX-FALSIFIER scan=PASS count=0`, current `scans.tsv` reports `POSTFIX-FALSIFIER scan=INSPECT count=1`, followed by `HC-FALSIFIER-PROOF: PASS pre_fix_failed_expectation=true post_fix_scanned=true`.
- PASS: `bash scripts/ci/agent_scan.sh` (`AGENT-SCAN-VERDICT: PASS delta_inspect=0` at committed head)

## Scope Ledger

- `scripts/ci/scans.tsv`: deleted the generic exclusion token only.
- `scripts/ci/doctrine_selftest.sh` and fixtures: preserved the existing trap identity and synthesized the marker-bearing falsifier in the selftest sandbox.
- `scripts/ci/test_inventory.tsv`: retained the original trap row without adding a durable falsifier row.
- `docs/design_0_0_8_4_8_4_1_harness_corrections.md`: PROBATION stamp in HC-1 exit-proof cell.
- `docs/orchestrator_orientation.md` and `docs/sanctioned_surface.md`: generated refreshes.
- `docs/ci_screening_surface.md`, `docs/tests/ci_scan_spec_kind_coverage_0_results.md`, this results doc: documentation of the closed hole.

## Graduation routing

CI verdict: PASS local handoff battery at committed head.
Triage entries: not added; deletion surfaced no new live crate findings.
Risk class: DA-reserve / gate-wiring.
Falsification check: marker-bearing fixture fails the pre-fix scanner expectation and passes after token deletion.
Recommended posture: PROBATION / proof-present / DA-review-pending.
