# CI-A-ALLOWLIST-SCANS-0 Results

## Status

**PROBATION** — closed-set RELIABLE allowlist scans implemented; 0R/0R2 patch sealed-producer `Self` return and constructor gaps. Self-reported implementer proof only; not DA acceptance.

## PR / branch / merge

- Branch: `ci-a-allowlist-scans-0r2` (0R2)
- PR: [#1027](https://github.com/khorum08/SimThing/pull/1027) (0), [#1028](https://github.com/khorum08/SimThing/pull/1028) (0R), [#1029](https://github.com/khorum08/SimThing/pull/1029) (0R2)
- Merge: `78ad4631a3` (0), `74f0c810c1` (0R), `a1fc28babf` (0R2, master)

## Recipient Agent

Cursor

## Orchestrator / §1A Triage Agent

Codex

## Executive DA

Opus / Owner

## What changed

### CI-A-ALLOWLIST-SCANS-0

- Added `scripts/ci/scan_allowlists.py` — stdlib closed-set engine for sealed producers, buffer handles, kernel surface.
- Extended `doctrine_scan.sh` with generic `@ALLOWLIST:` dispatch (no per-symbol hard-coding).
- Added three RELIABLE scans to `scans.tsv`: `ALLOW-SEALED-PRODUCERS`, `ALLOW-BUFFER-HANDLES`, `ALLOW-KERNEL-SURFACE`.
- Completed `sealed_producers.txt` with sanctioned session readback + CPU oracle batch doors discovered during enumeration.
- Extended producer grammar: `readback_*` under `read`; `execute_ops_cpu_with_emissions` / `execute_threshold_ops_cpu` under `cpu_oracle`.
- Updated `scripts/ci/README.md` and design lifecycle row → PROBATION.

### CI-A-ALLOWLIST-SCANS-0R — sealed-producer `Self` return gap

- `scan_allowlists.py`: track inherent `impl SealedType { … }` blocks via brace-aware line map.
- `return_type_is_sealed`: treat `-> Self` as sealed when enclosing impl target is in `SEALED_IMPL_TARGETS`.
- Constructor exclusion (`new`/`default`) applies only outside sealed impl blocks (0R2).

### CI-A-ALLOWLIST-SCANS-0R2 — sealed `new/default -> Self` constructor gap

- `scan_sealed_producers`: compute `impl_sealed` before constructor skip; skip `new`/`default` only when `impl_sealed is None`.
- Public sealed constructors hard FAIL (no allowlist door shape); exposes 3 pre-existing `gpu_readback.rs` readback `new` (crate remediation out of scope).

## Load-bearing proofs

| Proof | Result |
|---|---|
| `bash scripts/ci/doctrine_scan.sh` | FAIL 3 — pre-existing `gpu_readback.rs` sealed `new` (doctrine-correct per 0R2); buffer/kernel scans PASS |
| `python scripts/ci/verify_kernel_surface.py` | PASS — 195/195 exact match |

## Negative controls (local, reverted before commit)

### 0 baseline

| Control | Command / mutation | Expected | Observed |
|---|---|---|---|
| Unsanctioned sealed producer | Temp probe `pub fn forge_probe() -> crate::ThresholdEvent` | FAIL | exit 1 |
| Split-declaration producer | `pub fn forge_split(...)\n-> ThresholdEvent` | FAIL | exit 1 |
| Public buffer escape | Temp probe `pub fn leak_buffer() -> &wgpu::Buffer` | FAIL | `buffer-handles` exit 1 |
| New lib.rs export | Append `pub mod _doctrine_probe_mod;` | FAIL | missing from kernel_surface.txt |
| Malformed producer row | `forge_x \| read \| bad \| retire` | scanner error | grammar rejection, exit 1 |
| inert-util in kernel_surface | `Bad \| inert-util \| ...` | scanner error | forbidden class, exit 1 |

### 0R — Self return in sealed impl

| Control | Mutation | Expected | Observed |
|---|---|---|---|
| Same-line Self producer | `impl ThresholdEvent { pub fn forge_probe(...) -> Self { ... } }` | FAIL | exit 1 — `forge_probe -> Self (ThresholdEvent)` |
| Split-declaration Self producer | `#[doc(hidden)] pub fn forge_split(...)\n-> Self` | FAIL | exit 1 — `forge_split -> Self (ThresholdEvent)` |
| Non-sealed constructor | `impl PlainHelper { pub fn new() -> Self { ... } }` | no FAIL | not flagged (with probe) |

### 0R2 — sealed `new/default -> Self`

| Control | Mutation | Expected | Observed |
|---|---|---|---|
| Sealed `new` | `impl ThresholdEvent { pub fn new(...) -> Self }` | FAIL | exit 1 — `new -> Self (ThresholdEvent)` |
| Sealed `default` | `impl ThresholdEvent { pub fn default() -> Self }` | FAIL | exit 1 — `default -> Self (ThresholdEvent)` |
| Split/doc-hidden sealed `new` | `#[doc(hidden)] pub fn new_split(...)\n-> Self` | FAIL | exit 1 — `new_split -> Self (ThresholdEvent)` |
| Non-sealed constructor | `impl PlainHelper { pub fn new() -> Self }` | no FAIL | not flagged |
| Pre-existing (master) | `gpu_readback.rs` readback `pub fn new -> Self` ×3 | FAIL | exit 1 — EmissionRecordReadback, ThresholdEmissionReadback, ThresholdEventCandidatesReadback |

## Scope Ledger

| Path | Touched |
|---|---|
| `scripts/ci/scan_allowlists.py` | yes (0 + 0R + 0R2) |
| `scripts/ci/doctrine_scan.sh` | yes (0 only) |
| `scripts/ci/scans.tsv` | yes (0 only) |
| `scripts/ci/allow/sealed_producers.txt` | yes (0 only) |
| `scripts/ci/README.md` | yes (0 only) |
| `docs/tests/ci-a-allowlist-scans_results.md` | yes |
| `docs/tests/current_evidence_index.md` | yes |
| `docs/design_0_0_8_4_6_ci_scaffolding.md` | yes (0 PROBATION flip) |
| `crates/**`, workflows, fixtures, self-test, triage | **no** (temp probe only, reverted) |

## Conformance

- Closed-set scans are data-driven via `@ALLOWLIST:` + allowlists; runner remains thin.
- No crate edits committed, no new dependencies, no fixtures/workflow/triage artifacts.
- Legitimate remediation: one allowlist row with rationale + promotion-blocker.

## Known gaps / next

- `CI-A-FIXTURES-0` — blocked until 0R2 lands; committed negative-control corpus.
- Pre-existing FAIL: `gpu_readback.rs` public `new -> Self` on three readback types (crate seal rung, not 0R2 scope).
- `CI-A-SELF-TEST-0`, `CI-A-WORKFLOW-0`, `CI-A-INSPECT-TRIAGE-0`.
- `validate_and_mint_placed_participants_by_location_id` remains a core re-export, not a kernel-local `pub fn` — out of kernel sealed-producer enumeration scope.

## DOCTRINE SCAN REPORT

```
DOCTRINE SCAN REPORT  (commit a1fc28babf, 2026-06-30T22:37:35Z)
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
  ALLOW-SEALED-PRODUCERS  FAIL  3  design §5 sealed producer allowlist crates/simthing-kernel/src/gpu_readback.rs:77: unsanctioned sealed producer `new` -> Self (EmissionRecordReadback)
 crates/simthing-kernel/src/gpu_readback.rs:178: unsanctioned sealed producer `new` -> Self (ThresholdEmissionReadback)
 crates/simthing-kernel/src/gpu_readback.rs:291: unsanctioned sealed producer `new` -> Self (ThresholdEventCandidatesReadback)
  ALLOW-BUFFER-HANDLES  PASS  0  design §5 buffer handle allowlist
  ALLOW-KERNEL-SURFACE  PASS  0  design §5 kernel surface allowlist
  --- summary ---
  hard failures: 3   inspect flags: 0   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only
DOCTRINE-SCAN-VERDICT: FAIL  failures=3 inspect=0 selftest=SKIPPED
```
