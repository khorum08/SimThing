# TEST-NECESSITY-SWEEP-0 / 0R / 0R1 / 0R2 Results

## Status

**DONE — DA-APPROVED / MERGED (#1122 @ `3ef232506f`, executive DA 2026-07-04).** 0R substance `f21fdf54`; 0R1 reconciled closure docs; **0R2** (`e3c39a5af0b4b202d2c39867f2ad648708da3043`) was the build-integrity repair (orphaned doc-comment/attribute cleanup + two non-runnable `dependency-floor` helpers restored + `ct_3b_4a_gpu_projection.rs` survivor aligned to live production API + `scenario_io` test-fixture `Default`). DA independently re-ran the binding proof **`cargo check --workspace --all-targets` → PASS** (zero errors, all targets, clean tree at head) and verified: production logic untouched (all src deletions are inline `#[cfg(test)] mod tests` removals), the `dependency-floor` residue class + drift exception are narrow and keyed to `permanent-residue:dependency-floor` (stale-only; the unledgered-runnable-test check is intact), both restored helpers contain no `#[test]` and are imported by surviving tests, and the full inventory-drift gate passes in CI on the head. Inventory 4,070 → 731. Track D closed by `TRACK-D-CLOSEOUT-0`.

## DA adjudication (2026-07-04)

Accept conditions all TRUE: (1) full-workspace all-targets compile PASS locally; (2) R2 source changes are compile-hygiene/test-surface only, not production behavior; (3) both restored dependency-floor fixtures are required by surviving files; (4) neither restored fixture contains a runnable `#[test]`; (5) inventory-script changes narrowly scoped to dependency-floor; (6) live Doctrine Scan + Doctrine Exec green on `e3c39a5a`; (7) no forbidden proof run. Zero bounce conditions triggered. Notable reconciliation: `ct_3b_4a_gpu_projection.rs` was a latently non-compiling survivor on master (stale API never caught because normal CI does not run `--all-targets`); R2 aligned it to the current production API — a legitimate build-integrity repair, not a semantic alteration.

## 0R correction

The first implementation (prior head `45262b13`) incorrectly preserved 3,990 rows under bulk `KEEP_NECESSARY` categories. 0R reclassifies every survivor under the Necessity Test with default `DELETE_UNNECESSARY`. Old ledger labels such as AUDIT, KEEP, behavior-regression, selected representative, and promotion-target are not keep reasons.

## Mission

One-rung deletion of every test failing the Necessity Test across the GHA-safe corpus. Default decision = delete; keep only explicit #1121 keep-set classes or production/canonical/dependency break proof.

## Constitutional basis

- PR #1121 (`bc8383d0`) — Necessity Test doctrine repair
- `docs/invariants.md` — Necessity Test row
- `docs/ci_screening_surface.md` — per-boundary floor zero

## Scope

In scope: `crates/**` test deletion, ledger reconciliation, manifest with `break_if_deleted`, promotion plan, this results doc.

Out of scope: `.github/**`, production logic edits, forbidden owner-deep proof execution.

## Survivor classes (592 KEEP)

| Class | Count |
|---|---:|
| `parser_format_transformation` | 33 |
| `cpu_gpu_parity` | 162 |
| `determinism_golden` | 101 |
| `doc_named_invariant` | 121 |
| `escaped_bug` | 8 |
| `ci_scanner_fixture` | 110 |
| `active_track_live_rung` | 57 |
| `genuine_dependency` | 0 |
| `canonical_function` | 0 |
| `owner_deep_pending` | 0 |

Plus 137 ledger-only `cfg_test_mod` marker rows (`AUDIT`, non-runnable).

## Deletion summary

| Metric | Count |
|---|---:|
| Tests deleted in 0R (source) | 3,255 |
| Total tests deleted (from 4070 baseline) | 3,478 |
| Files deleted | 152 |
| Inventory before 0R | 3,990 |
| Inventory after 0R | 729 (592 KEEP + 137 markers) |
| Inventory from baseline | 4,070 → 729 |

Primary 0R deletion classes: behavior-regression without escaped-bug proof, AUDIT corpus, admission-adjacent enumeration, hygiene tables, representative/promotion-target residue, builder/enum/kind checks owned by type system, callable-only tests.

## Rejected old keep labels (not keep reasons)

| Label | Approx. rows reclassified to DELETE |
|---|---:|
| behavior-regression (generic) | ~2,400 |
| AUDIT | ~320 |
| selected representative | ~180 |
| promotion-target | ~25 |
| admission-adjacent | ~450 |

## Owner-deep manifest

| Bucket | Count |
|---|---:|
| delete (source removed) | 14 |
| keep (explicit keep-set) | 0 |
| owner-local compile only | 0 |

tools/mapeditor/gpu admission rows deleted in source under Necessity Test; no owner-deep status used as keep reason.

## Promotion backlog

| | Count |
|---|---:|
| Before | 25 |
| After | 0 |

## Source-touch scope proof

This PR touches `src/**` only to delete in-source `#[cfg(test)]` test bodies, test-only audit modules, compile-fail fixture seams, or ledger references to those test surfaces. It does not delete production runtime logic, production type definitions, public API, validators, parsers, or canonical functions.

Sampled source categories:

- in-source `mod tests` pruning
- `#[cfg(test)]` audit module deletion
- ledger-only cfg_test_mod marker preservation
- test file deletion under `crates/**/tests`

Mechanical diff check (`origin/master...HEAD`, 127 `src/**` files touched):

```
SOURCE-TOUCH-SCOPE REPORT
  src files in diff: 127
  deleted src files: 2
  test-surface edits: 125
  production-touch edits: 0
SOURCE-TOUCH-SCOPE-VERDICT: PASS
```

Checked source edits for production-surface changes: **PASS**.

## Proof

| Gate | Result |
|---|---|
| Digest `--check` | PASS |
| Inventory check | PASS |
| Boundary check | PASS |
| Drift check | PASS |
| Five-crate survivor floor | PASS — core, kernel, sim, workshop, mapgenerator `--tests` compile |
| Doctrine Scan local | PASS |
| Doctrine Scan live | PASS on `f21fdf54d23aa61fffa8335dafcbc3f472e953a0` |
| Doctrine Exec live | PASS, ci-b-webchat-smoke, non-owner-deep, merge_ref_status=PASS, head `f21fdf54d23aa61fffa8335dafcbc3f472e953a0` |
| `git diff --check` | PASS (rerun post-push) |
| Targeted survivor proof | not run (compile floor only) |

Doctrine Exec smoke PASS proves the doctrine-exec surface is mechanically clean. It does not certify survivor necessity classification; DA/Owner review remains required for the 592 survivor assignments.

## Forbidden proof avoided

- `cargo test --workspace` — not run
- Bare full-crate test batteries — not run
- tools/mapeditor/gpu probes on GHA — not run
- Owner-deep doctrine-exec profiles — not run
- workflow_dispatch / Bevy / GPU / desktop proof — not run

## Graduation routing

- **DA/Owner-held:** yes
- **DA question:** Does #1122 at the reconciled 0R1 head correctly apply the #1121 Necessity Test, leaving only 592 explicit keep-set survivors plus 137 non-runnable marker rows, with no production logic deletion and no forbidden proof?

## 0R2 build-integrity repair

DA/Owner accepted the #1122 deletion set and survivor classification. 0R2 does not relitigate test necessity. It fixes compile-only hygiene defects caused by the deletion wave: orphaned doc comments and deleted helper/fixture modules still imported by surviving files.

The deletion set remains final. Restored dependency-floor helpers are not runnable test survivors; they are kept only because surviving tests depend on them.

| Gate | Result |
|---|---|
| `cargo check --workspace --all-targets` | PASS (owner machine, §10.3 authorized) |
| Runnable-test delta vs 0R1 | PASS — zero runnable-test additions/deletions; restored helpers only |
| Dependency-floor helpers restored | 2 | `crates/simthing-spec/tests/disburse_down_fixture.rs`, `crates/simthing-driver/tests/mapgen_pr8_scheduled_concurrency.rs` |
| Orphaned doc comments removed | 3 | `crates/simthing-driver/src/install.rs`, `crates/simthing-driver/src/spec_session.rs`, `crates/simthing-driver/tests/mobility_gpu_kernel10_stream_accounting_fixture.rs` |

Additional compile hygiene (not production logic): `ct_3b_4a_gpu_projection.rs` import/slot API alignment; `scenario_io.rs` test fixture `..Default::default()` for struct fields added upstream.

## Merge

Not performed. Ready for DA/Owner closure review, not orchestrator merge-clear.