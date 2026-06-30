# CI-A-SELF-TEST-0 Results

## Status

**PROBATION** — executable doctrine scan self-test over committed fixture corpus. Not DA acceptance.

## PR / branch / merge

- Branch: `ci-a-self-test-0`
- PR: (pending)
- Merge: (pending)

## Recipient Agent

Cursor

## Orchestrator / §1A Triage Agent

Codex

## Executive DA

Opus / Owner

## What changed

- Added `scripts/ci/doctrine_selftest.sh` — sandbox runner exercising the committed fixture corpus against copied `doctrine_scan.sh`.
- Positive control: production `doctrine_scan.sh` must remain PASS.
- Negative controls: every RELIABLE known-bad fixture injected into a temp tree must trip its intended scan (FAIL, count > 0).
- Heuristic controls: production-shaped HEURISTIC fixtures must yield INSPECT (count > 0) with production kernel/sim baseline to avoid allowlist noise.
- Trap controls: trap fixtures on production baseline must not hard-FAIL.
- Malformed allowlist fixtures must produce scanner/data errors (non-zero exit).
- Rot test: neutralizing `B3-BUFFER-ESCAPE` pattern in a temp `scans.tsv` copy prevents known-bad detection (self-test would fail if rot went undetected).
- Fixed `unsafe_forbid_missing.rs` comment — literal `#![forbid(unsafe_code)]` in comment matched `@REQUIRE` pattern and prevented negative control from firing.
- Updated `scripts/ci/README.md`, `scripts/ci/fixtures/README.md`, design row `CI-A-SELF-TEST-0` → **PROBATION**.

## Self-test matrix

| Category | Case | Expected | Result |
|---|---|---|---|
| positive control | production tree | PASS footer, 0 hard failures | PASS |
| RELIABLE | `B3-BUFFER-ESCAPE` / `b3_buffer_escape.rs` | FAIL, count > 0 | PASS |
| RELIABLE | `FORGE-MINTERS` / `forge_minter.rs` | FAIL | PASS |
| RELIABLE | `UNSAFE-FN` / `unsafe_fn.rs` | FAIL | PASS |
| RELIABLE | `UNSAFE-ALLOW-ATTR` / `unsafe_allow_attr.rs` | FAIL | PASS |
| RELIABLE | `UNSAFE-FORBID-ATTR` / `unsafe_forbid_missing.rs` | FAIL | PASS |
| RELIABLE | `AS5-COLUMN-ALIAS` / `as5_column_alias.rs` | FAIL | PASS |
| RELIABLE | `DENY-TOML-STUB` / `deny_toml_stub.txt` | FAIL | PASS |
| RELIABLE | `ALLOW-SEALED-PRODUCERS` / 5 fixtures | FAIL | PASS |
| RELIABLE | `ALLOW-BUFFER-HANDLES` / `allow_buffer_handle.rs` | FAIL | PASS |
| RELIABLE | `ALLOW-KERNEL-SURFACE` / `allow_kernel_surface_lib.rs` | FAIL | PASS |
| allowlist validation | `malformed_allowlist_wrong_door.txt` | scanner error | PASS |
| allowlist validation | `malformed_allowlist_missing_rationale.txt` | scanner error | PASS |
| HEURISTIC | `RAW-DATA-INDEX` / `raw_data_index.rs` | INSPECT | PASS |
| HEURISTIC | `SIM-KIND-READ` / `sim_kind_read.rs` | INSPECT | PASS |
| HEURISTIC | `SEMANTIC-WORDS` / `semantic_words_production.rs` | INSPECT | PASS |
| HEURISTIC | `SPEC-STRING-CHANNEL` / `spec_string_channel.rs` | INSPECT | PASS |
| trap | 6 trap fixtures | no hard FAIL | PASS |

## Rot-test proof

- Copied `scans.tsv` into sandbox; replaced `B3-BUFFER-ESCAPE` pattern `pub fn [a-z_]+\(&self\) *-> *&` with non-matching `pub fn __NEVER_MATCH__`.
- Injected `b3_buffer_escape.rs` known-bad.
- Verified `B3-BUFFER-ESCAPE` line reads PASS 0 (scan no longer catches violation).
- Self-test rot case passes — a neutralized pattern would cause the main negative-control matrix to fail if run against that data.

## Load-bearing proofs

| Proof | Result |
|---|---|
| `bash scripts/ci/doctrine_selftest.sh` | PASS — `DOCTRINE-SELFTEST-VERDICT: PASS` |
| `bash scripts/ci/doctrine_scan.sh` | PASS — 15 scans, 0 hard FAIL |
| `python scripts/ci/verify_kernel_surface.py` | PASS — 195/195 |

## Scope Ledger

| Path | Touched |
|---|---|
| `scripts/ci/doctrine_selftest.sh` | yes (new) |
| `scripts/ci/README.md` | yes |
| `scripts/ci/fixtures/README.md` | yes |
| `scripts/ci/fixtures/known_bad/unsafe_forbid_missing.rs` | yes (comment fix — avoid `@REQUIRE` false match) |
| `docs/tests/ci-a-self-test_results.md` | yes |
| `docs/tests/current_evidence_index.md` | yes |
| `docs/design_0_0_8_4_6_ci_scaffolding.md` | yes (PROBATION) |
| `crates/**`, `scans.tsv`, `doctrine_scan.sh`, `scan_allowlists.py`, `allow/**`, `.github/**` | **no** |

## Known gaps / next

- `CI-A-WORKFLOW-0` — GitHub Actions workflow runs self-test then PR scan on `ubuntu-latest`.
- `CI-A-INSPECT-TRIAGE-0` — triage log + spam bounds.

## DOCTRINE SELFTEST REPORT

```
DOCTRINE SELFTEST REPORT
  positive control: PASS
  known-bad:
    B3-BUFFER-ESCAPE (b3_buffer_escape)  PASS
    FORGE-MINTERS (forge_minter)  PASS
    UNSAFE-FN (unsafe_fn)  PASS
    UNSAFE-ALLOW-ATTR (unsafe_allow_attr)  PASS
    UNSAFE-FORBID-ATTR (unsafe_forbid_missing)  PASS
    AS5-COLUMN-ALIAS (as5_column_alias)  PASS
    DENY-TOML-STUB (deny_toml_stub)  PASS
    ALLOW-SEALED-PRODUCERS (allow_sealed_producer)  PASS
    ALLOW-SEALED-PRODUCERS (allow_sealed_producer_split)  PASS
    ALLOW-SEALED-PRODUCERS (allow_sealed_producer_self)  PASS
    ALLOW-SEALED-PRODUCERS (allow_sealed_constructor_new)  PASS
    ALLOW-SEALED-PRODUCERS (allow_sealed_producer_doc_hidden)  PASS
    ALLOW-BUFFER-HANDLES (allow_buffer_handle)  PASS
    ALLOW-KERNEL-SURFACE (allow_kernel_surface_lib)  PASS
    allowlist validation (malformed_wrong_door)  PASS
    allowlist validation (malformed_missing_rationale)  PASS
  heuristic controls:
    RAW-DATA-INDEX (raw_data_index)  PASS
    SIM-KIND-READ (sim_kind_read)  PASS
    SEMANTIC-WORDS (semantic_words_production)  PASS
    SPEC-STRING-CHANNEL (spec_string_channel)  PASS
  traps:
    jomini_write  PASS
    studio_antialiasing  PASS
    pub_crate_sealed_accessor  PASS
    comment_semantic_words  PASS
    cfg_test_semantic_words  PASS
    cfg_test_kind_read  PASS
  rot test: PASS
DOCTRINE-SELFTEST-VERDICT: PASS
```

## DOCTRINE SCAN REPORT

```
DOCTRINE SCAN REPORT  (commit 64a9c9a836, 2026-06-30T23:50:25Z)
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
