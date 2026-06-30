# CI-A-FIXTURES-0 Results

## Status

**PROBATION** — committed doctrine scan fixture corpus (known-bad + traps). Inert until `CI-A-SELF-TEST-0`. Not DA acceptance.

## PR / branch / merge

- Branch: `ci-a-fixtures-0`
- PR: [#1031](https://github.com/khorum08/SimThing/pull/1031)
- Merge: `a2878eb63b` (master)

## Recipient Agent

Cursor

## Orchestrator / §1A Triage Agent

Codex

## Executive DA

Opus / Owner

## What changed

- Created `scripts/ci/fixtures/` with `known_bad/` (20 fixtures) and `traps/` (6 fixtures).
- Added `scripts/ci/fixtures/README.md` mapping fixture → scan → expected verdict.
- Updated `scripts/ci/README.md` with fixture layout and self-test next-rung note.
- Design row `CI-A-FIXTURES-0` → **PROBATION**.

## Fixture inventory

```
scripts/ci/fixtures/README.md
scripts/ci/fixtures/known_bad/allow_buffer_handle.rs
scripts/ci/fixtures/known_bad/allow_kernel_surface_lib.rs
scripts/ci/fixtures/known_bad/allow_sealed_constructor_new.rs
scripts/ci/fixtures/known_bad/allow_sealed_producer.rs
scripts/ci/fixtures/known_bad/allow_sealed_producer_doc_hidden.rs
scripts/ci/fixtures/known_bad/allow_sealed_producer_self.rs
scripts/ci/fixtures/known_bad/allow_sealed_producer_split.rs
scripts/ci/fixtures/known_bad/as5_column_alias.rs
scripts/ci/fixtures/known_bad/b3_buffer_escape.rs
scripts/ci/fixtures/known_bad/deny_toml_stub.txt
scripts/ci/fixtures/known_bad/forge_minter.rs
scripts/ci/fixtures/known_bad/malformed_allowlist_missing_rationale.txt
scripts/ci/fixtures/known_bad/malformed_allowlist_wrong_door.txt
scripts/ci/fixtures/known_bad/raw_data_index.rs
scripts/ci/fixtures/known_bad/semantic_words_production.rs
scripts/ci/fixtures/known_bad/sim_kind_read.rs
scripts/ci/fixtures/known_bad/spec_string_channel.rs
scripts/ci/fixtures/known_bad/unsafe_allow_attr.rs
scripts/ci/fixtures/known_bad/unsafe_fn.rs
scripts/ci/fixtures/known_bad/unsafe_forbid_missing.rs
scripts/ci/fixtures/traps/cfg_test_kind_read.rs
scripts/ci/fixtures/traps/cfg_test_semantic_words.rs
scripts/ci/fixtures/traps/comment_semantic_words.rs
scripts/ci/fixtures/traps/jomini_write.rs
scripts/ci/fixtures/traps/pub_crate_sealed_accessor.rs
scripts/ci/fixtures/traps/studio_antialiasing.rs
```

**Note:** `deny_toml_stub.txt` holds stub content; not named `deny.toml` in-tree because `DENY-TOML-STUB` glob matches any `deny.toml` and would trip production scan. Self-test copies to temp path.

## Known-bad coverage

| Scan | Fixture(s) | Expected |
|---|---|---|
| `B3-BUFFER-ESCAPE` | `b3_buffer_escape.rs` | FAIL |
| `FORGE-MINTERS` | `forge_minter.rs` | FAIL |
| `UNSAFE-FN` | `unsafe_fn.rs` | FAIL |
| `UNSAFE-ALLOW-ATTR` | `unsafe_allow_attr.rs` | FAIL |
| `UNSAFE-FORBID-ATTR` | `unsafe_forbid_missing.rs` | FAIL |
| `AS5-COLUMN-ALIAS` | `as5_column_alias.rs` | FAIL |
| `DENY-TOML-STUB` | `deny_toml_stub.txt` | FAIL (via self-test temp copy) |
| `ALLOW-SEALED-PRODUCERS` | `allow_sealed_producer.rs`, `_split.rs`, `_self.rs`, `_constructor_new.rs`, `_doc_hidden.rs` | FAIL |
| `ALLOW-BUFFER-HANDLES` | `allow_buffer_handle.rs` | FAIL |
| `ALLOW-KERNEL-SURFACE` | `allow_kernel_surface_lib.rs` | FAIL |
| allowlist validation | `malformed_allowlist_*.txt` | scanner error |
| `RAW-DATA-INDEX` | `raw_data_index.rs` | INSPECT (production `.data[0]`, not in comment/test) |
| `SIM-KIND-READ` | `sim_kind_read.rs` | INSPECT (production `match thing.kind`) |
| `SEMANTIC-WORDS` | `semantic_words_production.rs` | INSPECT (identifier contains `faction`, not excluded) |
| `SPEC-STRING-CHANNEL` | `spec_string_channel.rs` | INSPECT (`owner_ref: Option<String>`) |

## Trap coverage

| Trap | Expected non-failing |
|---|---|
| `jomini_write.rs` | `write_*` does not match sealed-producer grammar |
| `studio_antialiasing.rs` | module name does not contain semantic-word pattern |
| `pub_crate_sealed_accessor.rs` | `pub(crate)` excluded from public producer scan |
| `comment_semantic_words.rs` | `//` comment excluded |
| `cfg_test_semantic_words.rs` | `#[cfg(test)] mod tests` region excluded |
| `cfg_test_kind_read.rs` | `.kind` inside cfg(test) excluded |

## Load-bearing proofs

| Proof | Result |
|---|---|
| `bash scripts/ci/doctrine_scan.sh` | PASS — fixtures do not affect production globs |
| `python scripts/ci/verify_kernel_surface.py` | PASS — 195/195 |
| `find scripts/ci/fixtures -type f \| sort` | 27 files |

## Scope Ledger

| Path | Touched |
|---|---|
| `scripts/ci/fixtures/**` | yes |
| `scripts/ci/README.md` | yes |
| `docs/tests/ci-a-fixtures_results.md` | yes |
| `docs/tests/current_evidence_index.md` | yes |
| `docs/design_0_0_8_4_6_ci_scaffolding.md` | yes (PROBATION) |
| `crates/**`, `scans.tsv`, `doctrine_scan.sh`, `scan_allowlists.py`, `allow/**` | **no** |

## Known gaps / next

- `CI-A-SELF-TEST-0` — wire `doctrine_selftest.sh` to exercise corpus.
- `CI-A-WORKFLOW-0`, `CI-A-INSPECT-TRIAGE-0`.

## DOCTRINE SCAN REPORT

```
DOCTRINE SCAN REPORT  (commit a2878eb63b, 2026-06-30T23:14:27Z)
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
