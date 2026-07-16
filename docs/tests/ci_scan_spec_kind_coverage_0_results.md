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
| Pattern | `match .*\.kind` \| `\.kind\s*(==|!=)` \| parameterized `match kind { … SimThingKind:: … }` (multiline) |
| Verdict | `INSPECT` only (exit 0) |
| Data home | `scripts/ci/scans.tsv` row 18 |

## Why HEURISTIC / INSPECT, not RELIABLE / FAIL

A regex cannot distinguish legitimate role-resolution from drift-shaped gameplay branching. This scan is a
**tripwire for DA/triage judgment**, not a verdict. Spec/lowering `SimThingKind` reads are **not categorically
illegal**.

## Delta-scoping

PR CI (`doctrine_pr_scan.sh` / `doctrine_scan.sh --pr-delta`) flags only diff-introduced/touched hits.
Pre-existing baseline debt is **not** re-flagged per PR. Whole-tree mode is for backlog audit / positive control
only — **not** the PR-delta proof gate for this rung.

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
| Line count | **163** (handoff grep inventory; not a PR-delta proof gate) |
| Files | **16** |
| Classification | spec admission/role-resolution (planet/grid, spatial root, RF runtime checks, designer mobility, ingestion diagnostics); clausething lowerer construction + structural lattice validation + jomini error kind matching |
| Fixed here? | **no** — backlog only, not violations |

Whole-tree baseline backlog **exists and was recorded**; exact whole-tree INSPECT count is **not** a PR-delta
proof gate for this rung or 0R.

## 0R repair (CI-SCAN-SPEC-KIND-COVERAGE-0R — short-scan revision)

**Path A — branch-like `SimThingKind::` coverage implemented.**

- Extended `SPEC-LOWERER-KIND-READ` pattern with multiline
  `match\s+(?:&)?kind\s*\{[\s\S]*?SimThingKind::` (requires `SimThingKind::` inside the match block; avoids
  false positives on non-`SimThingKind` `match kind` sites).
- Added `known_bad/clausething_param_kind_branch.rs`, selftest case, and PR-delta prove case (5b).
- HC-EXCLUSION-REVIEW-GATE-0 later deleted the generic role-resolution marker from `scans.tsv`; role-resolution
  exclusions are now reviewed named symbols or accounted INSPECT rows.
- Removed conflicting narrow whole-tree count claims; authoritative 0R proof is PR-delta prove, not
  `doctrine_scan.sh` whole-tree ritual.
- Scope ledger corrected (see below).

## Fixtures / self-test

| Fixture | Scan | Expected |
|---|---|---|
| `fixtures/known_bad/spec_fleet_cohort_kind_branch.rs` | `SPEC-LOWERER-KIND-READ` | INSPECT |
| `fixtures/known_bad/clausething_kind_branch.rs` | `SPEC-LOWERER-KIND-READ` | INSPECT |
| `fixtures/known_bad/clausething_param_kind_branch.rs` | `SPEC-LOWERER-KIND-READ` | INSPECT |
| `fixtures/known_bad/role_resolution_exclude_site_kind_param_match.rs` | `SPEC-LOWERER-KIND-READ` | INSPECT |
| `fixtures/traps/role_resolution_kind_param_match.rs` | `SPEC-LOWERER-KIND-READ` | PASS (DA-authored named symbol) |
| Rot test | neutralized pattern → selftest FAIL | wired |

## Proof commands (0R short path)

**Authoritative 0R proof — PR-delta only; no whole-tree `doctrine_scan.sh`:**

```bash
bash scripts/ci/doctrine_pr_scan.sh --prove-delta
bash scripts/ci/gen_digest.sh --check
git diff --check origin/master...HEAD
```

**Selftest rerun (fixture/selftest wiring changed in 0R):**

```bash
bash scripts/ci/doctrine_selftest.sh
```

**Explicitly not run for 0R:**

```bash
bash scripts/ci/doctrine_scan.sh   # whole-tree — backlog audit only, not PR-delta proof gate
cargo run / cargo check / cargo test
```

Raw terminal output pasted in commit message / PR body (no PowerShell stdout/stderr redirection).

## Scope ledger

Docs + scan data + fixtures + scan harness proof extensions (`doctrine_pr_scan.sh`, `doctrine_selftest.sh`) +
fixture inventory ledger rows (`test_inventory.tsv`) + regenerated sanctioned-surface digest only.

| Path class | Touched? |
|---|---|
| `crates/**` | no |
| TP files | no |
| `.github/**` | no |
| `doctrine_exec_profiles.tsv` | no |
| `test_lifecycle_tracks.tsv` | no |
| `simthing-workshop` code | no |
| registry / `testthing/` scaffold | no |

## Graduation routing

- `CI-SCAN-SPEC-KIND-COVERAGE-0` + `0R` implementation complete pending DA review
- **PROBATION / DA-OWNER REVIEW**
- Gate-state / DA-held
- Not self-mergeable
- DA/Owner clearance required
