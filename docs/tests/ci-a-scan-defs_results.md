# CI-A-SCAN-DEFS-0R + CI-A-RUNNER-0R Results

## Status

**PROBATION** — Gate 1 remediation landed (kernel-surface completeness, door-class legitimacy, HEURISTIC baseline shrink). Self-reported positive control: zero hard FAIL, zero whole-tree HEURISTIC INSPECT. Gate 1 remains **HELD** pending Opus re-audit. This doc does **not** claim DA acceptance.

## PR / branch / merge

- Branch: `ci-a-scan-defs-0r`
- PR: [#1025](https://github.com/khorum08/SimThing/pull/1025)
- Merge: `1b4ed6e4a4` (master)

## Recipient Agent

Cursor

## Orchestrator / §1A Triage Agent

Codex

## Executive DA

Opus / Owner

## What changed

### 0R-a — kernel surface completeness

- Fixed export enumeration to capture **grouped** (`pub use m::{A,B}`) and **single-line** (`pub use m::Symbol;`) forms.
- Re-derived `scripts/ci/allow/kernel_surface.txt` (195 symbols, +3 vs prior incomplete list).
- Added `scripts/ci/audit_kernel_surface.py` (regenerate) and `scripts/ci/verify_kernel_surface.py` (diff audit).

### 0R-b — door-class legitimacy

- Reclassified all `kernel_surface.txt` rows: **zero** `inert-util`; uses `surface-inert`, `authority-export`, `sealed-export`.
- File-aware allowlist validation in `doctrine_scan.sh` (per-file door-class rules; rejects `inert-util` in kernel_surface).

### 0R-c — HEURISTIC baseline shrink

- HEURISTIC-only runner filters: exclude `tests/` paths, `#[cfg(test)] mod tests {` regions (generic, not scan-specific).
- Tightened `scans.tsv` excludes (`assert_`, `#[test]`, `SimThingKind::` for semantic-word test fixtures).
- README records CI-A-WORKFLOW-0 rule: RELIABLE whole-tree, HEURISTIC PR-diff scoped.

## 0R-a completeness proof

Audit commands (from repo root):

```bash
python scripts/ci/audit_kernel_surface.py
python scripts/ci/verify_kernel_surface.py
```

`verify_kernel_surface.py` output (this branch):

```text
lib.rs exports: 195
kernel_surface.txt: 195
missing: []
extra: []
build_overlay_deltas: present
project_tree_to_values: present
ResolvedGpuBuffers: present
forms: grouped=20 single-line=3
```

Grouped + single-line capture: 20 grouped `pub use` blocks + 3 single-line exports in `lib.rs` (`build_overlay_deltas`, `project_tree_to_values`, `ResolvedGpuBuffers`).

## 0R-b legitimacy proof

```bash
grep -n ' | inert-util | ' scripts/ci/allow/kernel_surface.txt && echo KERNEL_SURFACE_INERT_LAUNDERING || true
```

Expected: no output (confirmed).

File-aware validation proofs:

- Append `forge_x | read | bad | retire` to `sealed_producers.txt` → rejected (grammar).
- Append `Bad | inert-util | bad | retire` to `kernel_surface.txt` → rejected (`inert-util` forbidden).
- Append `Bad | sealed-export | bad | retire` to `sealed_producers.txt` → rejected (wrong file door-class set).

Sample legitimate rows:

```text
ThresholdEvent | sealed-export | Sealed event record export; produced only through sanctioned read/cpu-oracle doors | ...
WorldGpuState | authority-export | Kernel-owned world GPU state surface; authority-bearing runtime handle | ...
cpu_oracle_threshold_events | authority-export | CPU-oracle authority surface; xref sealed_producers:cpu_oracle_threshold_events | ...
WORKGROUP_SIZE | surface-inert | Inert public kernel constant | ...
```

## 0R-c HEURISTIC baseline proof

| Metric | Prior (#1022) | After 0R |
|---|---|---|
| Whole-tree INSPECT total | 81 (73 SEMANTIC-WORDS + 8 SIM-KIND-READ) | **0** |
| RELIABLE hard FAIL | 0 | 0 |

Remaining baseline explanation: prior 81 flags were entirely `#[cfg(test)] mod tests` bodies in `fission.rs` / `boundary.rs` plus doc-comment fixtures already excluded by comment rules. Production `threshold_registry.rs` doc mention of "faction" is excluded by `///` comment filter. No production semantic leak masked.

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `bash scripts/ci/doctrine_scan.sh` | Positive control; report + footer |
| `python scripts/ci/verify_kernel_surface.py` | Incomplete kernel_surface vs lib.rs |
| `grep inert-util kernel_surface.txt` (expect empty) | Authority laundering as inert-util |
| Malformed allowlist / scans.tsv | Scanner/data-format rot |
| RELIABLE scans still PASS 0 | Blocklist regression |

## Scope Ledger

| Path | Why touched |
|---|---|
| `scripts/ci/audit_kernel_surface.py` | Re-derive kernel_surface (wired in README) |
| `scripts/ci/verify_kernel_surface.py` | Completeness audit proof |
| `scripts/ci/allow/kernel_surface.txt` | Re-derived + reclassified exports |
| `scripts/ci/doctrine_scan.sh` | File-aware validation + HEURISTIC filters |
| `scripts/ci/scans.tsv` | HEURISTIC exclude tightening |
| `scripts/ci/README.md` | Door-class rules + workflow delta-scoping note |
| `docs/tests/ci-a-scan-defs_results.md` | This evidence doc |
| `docs/tests/current_evidence_index.md` | PR/merge fields update |

**Not touched:** `crates/**`, workflows, fixtures, self-test, triage log, spam check, Track B/C.

## Known gaps / next

- Gate 1 re-audit by Opus (allowlist legitimacy, classification honesty).
- `readback_threshold_*` still absent from `sealed_producers.txt` (`readback_*` vs `read_*` grammar) — unchanged from #1022.
- `CI-A-ALLOWLIST-SCANS-0` blocked until Gate 1 clears.
- `CI-A-WORKFLOW-0` must implement HEURISTIC PR-diff scoping in CI.

## DOCTRINE SCAN REPORT (positive control)

```
DOCTRINE SCAN REPORT  (commit <sha>, 2026-06-30T21:09:46Z)
  scanner self-test: SKIPPED
  --- results ---
  B3-BUFFER-ESCAPE  PASS  0  design §5 B3 buffer escape
  FORGE-MINTERS  PASS  0  design §5 forge minters
  UNSAFE-FN  PASS  0  design §5 unsafe fn
  UNSAFE-ALLOW-ATTR  PASS  0  design §5 allow unsafe attr
  UNSAFE-FORBID-ATTR  PASS  0  design §5 forbid unsafe attr
  AS5-COLUMN-ALIAS  PASS  0  design §5 AS-5 ColumnIndex alias
  DENY-TOML-STUB  PASS  0  design §0.6.6 deny.toml stub
  RAW-DATA-INDEX  PASS  0  design §5 raw data[N] index
  SIM-KIND-READ  PASS  0  design §5 sim .kind read
  SEMANTIC-WORDS  PASS  0  design §5 semantic words below spec
  SPEC-STRING-CHANNEL  PASS  0  design §5 stringly channel identity
  --- summary ---
  hard failures: 0   inspect flags: 0   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
```
