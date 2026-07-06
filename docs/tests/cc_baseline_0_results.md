# CC-BASELINE-0 Results

## Status

PROBATION / proof-present / orchestrator-routing-pending.

## PR / branch / head

- Branch: `codex/cc-baseline-0`
- PR: https://github.com/khorum08/SimThing/pull/1193
- tested_code_sha: 6262a305f4c870d5a936328753f1acbc29ae80f1
- coverage_basis: PASS — inventory-classification rung only; no Rust crate touched; proofs are harness gates below.
- ORIENT-RECEIPT: 0c419b2ecc07
- role: coding
- orientation_rule_stamp: 226295d8778ed5f9
- orientation_digest_sha: aaa6b82d373a682ff357d26b1ff748eceb1a5840b4a8c186f15f3da4c3d641e2

## What changed

- Classified all 137 `class=unknown` rows in `scripts/ci/test_inventory.tsv`.
- Every former unknown row is a `cfg(test)` module-marker ledger entry (`B-T6-MODULE-MARKER-EXPANSION`), not a runnable test.
- Reclassified those 137 rows to `class=deletion-candidate` with `verdict=AUDIT`, `promotion_target=ledger-only`, and a per-crate CC-SWEEP deferral note.
- Corrected 169 harness-fixture rows that referenced non-existent birth tracks (`0.0.8.4.7-orchestration-harness`, `0.0.8.4.8-corpus-clearance`) to the valid open track `0.0.8.4.6-ci-scaffolding` so lifecycle schema checks pass (pre-existing inventory hygiene; no lifecycle schema edit).
- Removed out-of-scope `docs/tests/cc_baseline_0_unknown_resolution.tsv` artifact from the branch.
- No tests deleted.
- No inventory rows deleted.
- Baseline frozen for later CC-SWEEP deletion waves.

## Baseline summary

| Metric | Before | After |
|---|---:|---:|
| total inventory rows | 935 | 935 |
| class=unknown rows | 137 | 0 |
| rows reclassified to durable classes | 0 | 0 |
| rows reclassified to pre-lifecycle | 0 | 0 |
| rows marked deletion-candidate | 0 | 137 |
| birth_track hygiene fixes (non-unknown rows) | — | 169 |
| files deleted | 0 | 0 |
| inventory rows deleted | 0 | 0 |

## Classification basis

All 137 former `unknown` rows share one shape:

- `kind=unit`, `test_name=cfg_test_mod::*`, `superseding_boundary=B-T6-MODULE-MARKER-EXPANSION`
- Original note: module-marker ledger row; not a runnable test
- Original `promotion_target=ledger-only`, `birth_track=pre-lifecycle`, `verdict=AUDIT`

Retention decision: **deletion-candidate**. These rows exist only because the drift gate discovers `#[cfg(test)] mod` markers. They are not executable tests and carry no durable residue class (seal-proof / oracle-parity / golden-byte / etc.). They remain ledgered in CC-BASELINE-0 but are flagged for per-crate CC-SWEEP removal once marker discovery policy is revisited.

## Load-bearing proofs

```text
bash scripts/ci/test_inventory_drift_check.sh
TEST-INVENTORY-DRIFT-CHECK REPORT
  rows: 935
  discovered: 933
  unledgered: 0
  stale: 0
  promotion-target rows: 0
TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS
```

```text
bash scripts/ci/test_lifecycle_expiry_check.sh --schema
LIFECYCLE-EXPIRY SCHEMA CHECK
  inventory rows: 935
  lifecycle tracks: 3
  dsu tiers: 3
LIFECYCLE-EXPIRY-VERDICT: PASS expired=0 audit=0 max_dsu_survivals=0 mode=schema
```

```text
bash scripts/ci/test_lifecycle_expiry_check.sh --scheduled
LIFECYCLE-EXPIRY SCHEDULED CHECK
  closed tracks: pre-lifecycle
  expired candidates: 137
  survivor-set expired: 0
  justified-closed (audit): 0
LIFECYCLE-EXPIRY-VERDICT: INSPECT expired=137 audit=0 max_dsu_survivals=0 mode=scheduled
```

```text
bash scripts/ci/doctrine_scan.sh
DOCTRINE-SCAN-VERDICT: INSPECT  failures=0 inspect=415 selftest=SKIPPED
```

```text
bash scripts/ci/gen_orientation.sh --check
gen_orientation --check: PASS
```

```text
bash scripts/ci/gen_digest.sh --check
gen_digest --check: PASS
```

```text
git diff --check HEAD
PASS
```

Explicit omissions:

- `scanner surface unchanged — doctrine_selftest.sh not required`
- `doctrine_scan.sh selftest=SKIPPED expected`
- `no Rust crate touched — cargo check not required`
- `relay_lint surface unchanged — relay_lint selftest not required`
- `clearance router unchanged — clearance_check selftest not required`

## Unknown-row resolution ledger

| Crate | Count | New class | Retention basis | Future sweep target |
|---|---:|---|---|---|
| simthing-mapeditor | 29 | deletion-candidate | cfg(test) mod marker for drift completeness; not runnable | CC-SWEEP-simthing-mapeditor |
| simthing-kernel | 27 | deletion-candidate | cfg(test) mod marker for drift completeness; not runnable | CC-SWEEP-simthing-kernel |
| simthing-driver | 23 | deletion-candidate | cfg(test) mod marker for drift completeness; not runnable | CC-SWEEP-simthing-driver |
| simthing-core | 16 | deletion-candidate | cfg(test) mod marker for drift completeness; not runnable | CC-SWEEP-simthing-core |
| simthing-sim | 15 | deletion-candidate | cfg(test) mod marker for drift completeness; not runnable | CC-SWEEP-simthing-sim |
| simthing-spec | 10 | deletion-candidate | cfg(test) mod marker for drift completeness; not runnable | CC-SWEEP-simthing-spec |
| simthing-feeder | 5 | deletion-candidate | cfg(test) mod marker for drift completeness; not runnable | CC-SWEEP-simthing-feeder |
| simthing-tools | 4 | deletion-candidate | cfg(test) mod marker for drift completeness; not runnable | CC-SWEEP-simthing-tools |
| simthing-clausething | 3 | deletion-candidate | cfg(test) mod marker for drift completeness; not runnable | CC-SWEEP-simthing-clausething |
| simthing-gpu | 3 | deletion-candidate | cfg(test) mod marker for drift completeness; not runnable | CC-SWEEP-simthing-gpu |
| simthing-mapgenerator | 1 | deletion-candidate | cfg(test) mod marker for drift completeness; not runnable | CC-SWEEP-simthing-mapgenerator |
| simthing-workshop | 1 | deletion-candidate | cfg(test) mod marker for drift completeness; not runnable | CC-SWEEP-simthing-workshop |

## Scope Ledger

| Surface | Status | Notes |
|---|---|---|
| scripts/ci/test_inventory.tsv | changed | all unknown rows resolved; birth_track hygiene for harness fixtures |
| docs/tests/cc_baseline_0_results.md | added | baseline proof |
| test files | not deleted | no deletion in this rung |
| Rust production code | not touched | |
| scanner surface | not touched | doctrine_selftest.sh not run |
| clearance routing | not touched | orchestrator owns clearance |
| relay lint | not touched | |

## Conformance

- 0 rows remain `class=unknown`.
- Every reclassification has a retention basis.
- No tests or inventory rows were deleted.
- Drift gate passes.
- Lifecycle expiry schema gate passes; scheduled scan INSPECTs the 137 deletion-candidate module-marker rows on closed `pre-lifecycle` track (expected baseline posture).
- Doctrine scan has zero hard failures.
- Scanner selftest was not run because scanner surface was unchanged.
- No SHA matching was used as a routing or proof substitute.

## Known gaps / next

- Orchestrator must run `/clearance` after PR creation.
- Later `CC-SWEEP-<crate>` rungs may delete `deletion-candidate` module-marker ledger rows, one crate/surface at a time.
- CC-BASELINE-0 itself performs no deletions.
- Adding missing lifecycle track ids for harness rungs remains a separate DA/schema decision if birth tracks should reflect rung provenance instead of `0.0.8.4.6-ci-scaffolding`.

## Graduation routing

- CLEARANCE-VERDICT: ORCHESTRATOR-TO-RUN
- CI verdict: drift PASS; lifecycle schema PASS; doctrine scan INSPECT failures=0
- Triage entries: none
- Risk class: corpus-baseline, inventory-classification, retention-doctrine
- Falsification check:
  - grep `scripts/ci/test_inventory.tsv` for `class=unknown` → expect 0
  - inspect sample reclassifications for concrete retention basis
  - verify no test files deleted
  - verify drift/lifecycle/doctrine outputs
- Recommended posture: orchestrator-routing-pending
