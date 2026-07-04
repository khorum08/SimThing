# TEST-NECESSITY-SWEEP-0 / 0R Results

## Status

**HOLD / 0R COMPLETE — DA/OWNER REVIEW** — implemented on branch `grok/test-necessity-sweep-0`; merge not authorized.

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

Plus 137 `cfg_test_mod` marker rows (`AUDIT`, ledger-only).

## Deletion summary

| Metric | Count |
|---|---:|
| Tests deleted in 0R (source) | 3,255 |
| Total tests deleted (from 4070 baseline) | 3,478 |
| Files deleted | 152 |
| Inventory before 0R | 3,990 |
| Inventory after 0R | 729 (592 KEEP + 137 markers) |

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

## Proof

| Gate | Result |
|---|---|
| Digest `--check` | PASS |
| Inventory check | PASS |
| Boundary check | PASS |
| Drift check | PASS |
| Five-crate survivor floor | PASS — core, kernel, sim, workshop, mapgenerator `--tests` compile |
| Doctrine Scan local | TIMEOUT on Windows harness (>5m); `rg` present — rely on live CI |
| Doctrine Scan live | pending CI on push |
| Doctrine Exec live | not run (forbidden) |
| `git diff --check` | pending post-push |
| Targeted survivor proof | not run (compile floor only) |

## Forbidden proof avoided

- `cargo test --workspace` — not run
- Bare full-crate test batteries — not run
- tools/mapeditor/gpu probes on GHA — not run
- Owner-deep doctrine-exec profiles — not run
- workflow_dispatch / Bevy / GPU / desktop proof — not run

## Graduation routing

- **DA/Owner-held:** yes
- **Reason:** 0R mass reclassification; owner review of survivor class assignments and tools/mapeditor/gpu compile uncertainty

## Merge

Not performed. Return for orchestration triage.