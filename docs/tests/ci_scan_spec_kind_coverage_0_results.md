# CI-SCAN-SPEC-KIND-COVERAGE-0 Results

## Status

**PROBATION / DA-OWNER REVIEW — gate-state, DA-held, not self-mergeable.**

Separate scan-only PR. No product crates, no TP files, no workshop code, no registry, no `testthing/` scaffold.

## Parent incident

`TP-FLEETS-SHIPS-0` briefly placed Fleet/Cohort-specific RF traversal in `simthing-spec`. `TP-FLEETS-SHIPS-0R`
removed it. This rung closes the scan coverage hole that let the drift reach DA review: `SIM-KIND-READ` and
`SEMANTIC-WORDS` did not cover `crates/simthing-spec/src/**` or `crates/simthing-clausething/src/**`.

## Merged workshop doctrine

Read and applied: `design_0_0_8_5…§0A` / `§0A.1`, `ci_screening_surface.md` §12, `design_0_0_8_4_6_ci_scaffolding.md`,
`simthing_core_design.md` §1 / §3 / §5.

Scenario-born candidate engine-shaped code belongs in **`simthing-workshop`** while proofing. Elevation out of
workshop is a separate DA admission PR after the slice is proven. This rung adds no workshop code.

## Root cause

Drift was less-conformant generic `SimThingKind` branching in `simthing-spec`, not a categorical ban on all
spec-layer kind reads. Legitimate role-resolution (column admission, diagnostics) may still read kinds; the
forbidden shape is scenario-specific gameplay branching in production spec/lowerer paths.

## Scan added

| Field | Value |
|---|---|
| Scan ID | `SPEC-LOWERER-KIND-READ` |
| Severity | `HEURISTIC` |
| Targets | `crates/simthing-{spec,clausething}/src/**` |
| Pattern | `match .*\.kind` \| `\.kind\s*(==|!=)` |
| Verdict | `INSPECT` only (exit 0) |
| Data home | `scripts/ci/scans.tsv` row 18 |

## Why HEURISTIC / INSPECT, not RELIABLE / FAIL

A regex cannot distinguish legitimate role-resolution from drift-shaped gameplay branching. This scan is a
**tripwire for DA/triage judgment**, not a verdict. Spec/lowering `SimThingKind` reads are **not categorically
illegal**.

## Delta-scoping

PR CI (`doctrine_pr_scan.sh` / `doctrine_scan.sh --pr-delta`) flags only diff-introduced/touched hits.
Pre-existing baseline debt is **not** re-flagged per PR. Whole-tree mode remains for master positive control,
baseline inventory, and self-test fixture validation.

## Closed-lowerer weighting

Doctrine reference on the scan row:

> HEURISTIC tripwire: spec/lowering kind read may be legitimate role-resolution, but **closed-lowerer hits are
> higher suspicion** because lowerers are constitutionally closed unless a DA-authorized amendment names them.

Not a hard block — triage routing guidance only.

## Promotion blocker

`retire when spec-layer role resolution is role-keyed by SubFieldRole/column admission boundaries rather than SimThingKind branching`

(Long-term fix is role-keyed admission, not banning all kind reads.)

## Workshop candidate-home routing

If a scenario needs candidate engine-shaped code, place it in **`simthing-workshop`** while proofing. Do not place
it in engine crates, `simthing-spec`, or lowerers as generic corpus behavior. Elevation out of workshop is a
separate DA admission PR after the slice is proven.

## Pre-existing spec/lowering kind-read triage backlog

Recorded via:

```bash
git grep -nE "match .*\\.kind|\\.kind\\s*(==|!=)|SimThingKind::" -- crates/simthing-spec/src crates/simthing-clausething/src
```

| Metric | Value |
|---|---|
| Label | pre-existing spec/lowering kind-read triage backlog |
| Line count | **163** |
| Files | **16** |
| Classification | spec admission/role-resolution (planet/grid, spatial root, RF runtime checks, designer mobility, ingestion diagnostics); clausething lowerer construction + structural lattice validation + jomini error kind matching |
| Fixed here? | **no** — backlog only, not violations |

Narrower scan pattern (`match .*\.kind|\.kind\s*(==|!=)`) matches **91** production lines across **14** files;
recorded as the scan's whole-tree baseline surface (INSPECT on master push, not per-PR noise).

## Fixtures / self-test

| Fixture | Scan | Expected |
|---|---|---|
| `fixtures/known_bad/spec_fleet_cohort_kind_branch.rs` | `SPEC-LOWERER-KIND-READ` | INSPECT |
| `fixtures/known_bad/clausething_kind_branch.rs` | `SPEC-LOWERER-KIND-READ` | INSPECT |
| `fixtures/traps/role_resolution_kind_param_match.rs` | `SPEC-LOWERER-KIND-READ` | PASS (no `.kind` field branch) |
| Rot test | neutralized pattern → selftest FAIL | wired |
| Positive control | whole-tree hard FAIL=0; INSPECT allowed when no hard failures | wired |

## Proof commands

```bash
bash scripts/ci/doctrine_selftest.sh
# DOCTRINE-SELFTEST-VERDICT: PASS (SPEC-LOWERER-KIND-READ fixtures + rot + role-resolution trap PASS)

bash scripts/ci/doctrine_scan.sh
# SPEC-LOWERER-KIND-READ  INSPECT  90  (whole-tree baseline backlog surface)
# TEST-INVENTORY-DRIFT  PASS  0
# DOCTRINE-SCAN-VERDICT: INSPECT  failures=0 inspect=90 selftest=SKIPPED

bash scripts/ci/gen_digest.sh --check
# gen_digest --check: PASS

bash scripts/ci/doctrine_pr_scan.sh --prove-delta
# PR-delta proof: PASS (net-new spec kind branch -> INSPECT; baseline outside delta suppressed)

git diff --check origin/master...HEAD
# (recorded at commit — no conflict markers)
```

`cargo run` / workspace cargo: **not run** (forbidden).

**Stock-gate note:** three fixture ledger rows added to `scripts/ci/test_inventory.tsv` so
`TEST-INVENTORY-DRIFT` remains PASS after new `scripts/ci/fixtures/**` files (required by the stock gate;
not product/test corpus change).

## Scope ledger

| Path class | Touched? |
|---|---|
| `crates/**` | no |
| TP files (`docs/tests/tp_*`, TP scenario code) | no |
| `.github/**` | no |
| `doctrine_exec_profiles.tsv` | no |
| `test_lifecycle_tracks.tsv` | no |
| `simthing-workshop` code | no |
| docs + scans + fixtures only | yes |

Allowed edits: `scripts/ci/scans.tsv`, `scripts/ci/fixtures/**`, `scripts/ci/doctrine_selftest.sh`,
`scripts/ci/doctrine_pr_scan.sh`, `docs/design_0_0_8_4_6_ci_scaffolding.md`, `docs/ci_screening_surface.md`,
`docs/tests/current_evidence_index.md`, `docs/sanctioned_surface.md` (gen_digest output),
`docs/tests/ci_scan_spec_kind_coverage_0_results.md`.

## Graduation routing

- `CI-SCAN-SPEC-KIND-COVERAGE-0` implementation complete pending DA review
- **PROBATION / DA-OWNER REVIEW**
- Gate-state / DA-held
- Not self-mergeable
- DA/Owner clearance required