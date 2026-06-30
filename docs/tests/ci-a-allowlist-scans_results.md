# CI-A-ALLOWLIST-SCANS-0 Results

## Status

**PROBATION** — closed-set RELIABLE allowlist scans implemented; positive control zero hard FAIL. Self-reported implementer proof only; not DA acceptance.

## PR / branch / merge

- Branch: `ci-a-allowlist-scans-0`
- PR: [#1027](https://github.com/khorum08/SimThing/pull/1027)
- Merge: `78ad4631a3` (master)

## Recipient Agent

Cursor

## Orchestrator / §1A Triage Agent

Codex

## Executive DA

Opus / Owner

## What changed

- Added `scripts/ci/scan_allowlists.py` — stdlib closed-set engine for sealed producers, buffer handles, kernel surface.
- Extended `doctrine_scan.sh` with generic `@ALLOWLIST:` dispatch (no per-symbol hard-coding).
- Added three RELIABLE scans to `scans.tsv`: `ALLOW-SEALED-PRODUCERS`, `ALLOW-BUFFER-HANDLES`, `ALLOW-KERNEL-SURFACE`.
- Completed `sealed_producers.txt` with sanctioned session readback + CPU oracle batch doors discovered during enumeration.
- Extended producer grammar: `readback_*` under `read`; `execute_ops_cpu_with_emissions` / `execute_threshold_ops_cpu` under `cpu_oracle`.
- Updated `scripts/ci/README.md` and design lifecycle row → PROBATION.

## Load-bearing proofs

| Proof | Result |
|---|---|
| `bash scripts/ci/doctrine_scan.sh` | PASS — 0 hard FAIL, footer once |
| `python scripts/ci/verify_kernel_surface.py` | PASS — 195/195 exact match |

## Negative controls (local, reverted before commit)

| Control | Command / mutation | Expected | Observed |
|---|---|---|---|
| Unsanctioned sealed producer | Temp `crates/simthing-kernel/src/_doctrine_probe.rs` with `pub fn forge_probe() -> crate::ThresholdEvent` | FAIL | `scan_allowlists.py sealed-producers` exit 1 |
| Split-declaration producer | Same file, `pub fn forge_split(...)\n-> ThresholdEvent` | FAIL | exit 1 |
| Public buffer escape | Temp probe `pub fn leak_buffer(&self) -> &wgpu::Buffer` | FAIL | `scan_allowlists.py buffer-handles` exit 1 |
| New lib.rs export | Append `pub mod _doctrine_probe_mod;` to `lib.rs` | FAIL | reports missing from kernel_surface.txt |
| Malformed producer row | Append `forge_x \| read \| bad \| retire` to sealed_producers.txt | scanner error | grammar rejection, exit 1 |
| inert-util in kernel_surface | Append `Bad \| inert-util \| ...` to kernel_surface.txt | scanner error | forbidden class, exit 1 |

## Scope Ledger

| Path | Touched |
|---|---|
| `scripts/ci/scan_allowlists.py` | yes (new) |
| `scripts/ci/doctrine_scan.sh` | yes |
| `scripts/ci/scans.tsv` | yes |
| `scripts/ci/allow/sealed_producers.txt` | yes (complete sanctioned set) |
| `scripts/ci/README.md` | yes |
| `docs/tests/ci-a-allowlist-scans_results.md` | yes |
| `docs/tests/current_evidence_index.md` | yes |
| `docs/design_0_0_8_4_6_ci_scaffolding.md` | yes (PROBATION flip) |
| `crates/**`, workflows, fixtures, self-test, triage | **no** |

## Conformance

- Closed-set scans are data-driven via `@ALLOWLIST:` + allowlists; runner remains thin.
- No crate edits, no new dependencies, no fixtures/workflow/triage artifacts.
- Legitimate remediation: one allowlist row with rationale + promotion-blocker.

## Known gaps / next

- `CI-A-FIXTURES-0` — committed negative-control corpus (forward note: production semantic/.kind INSPECT controls).
- `CI-A-SELF-TEST-0`, `CI-A-WORKFLOW-0`, `CI-A-INSPECT-TRIAGE-0`.
- `validate_and_mint_placed_participants_by_location_id` remains a core re-export, not a kernel-local `pub fn` — out of kernel sealed-producer enumeration scope.

## DOCTRINE SCAN REPORT

```
DOCTRINE SCAN REPORT  (commit 78ad4631a3, 2026-06-30T22:05:13Z)
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
  ALLOW-SEALED-PRODUCERS  PASS  0  design §5 sealed producer allowlist
  ALLOW-BUFFER-HANDLES  PASS  0  design §5 buffer handle allowlist
  ALLOW-KERNEL-SURFACE  PASS  0  design §5 kernel surface allowlist
  --- summary ---
  hard failures: 0   inspect flags: 0   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
```
