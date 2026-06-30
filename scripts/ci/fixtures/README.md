# Doctrine scan fixture corpus (CI-A-FIXTURES-0)

Inert test inputs for **`CI-A-SELF-TEST-0`**. Fixtures are **not** scanned by normal `doctrine_scan.sh` on production paths — the runner targets `crates/**`, root `deny.toml`, and `allow/*` only.

Do **not** point `doctrine_scan.sh` at this directory as production input.

## Purpose

- **known_bad/** — one fixture per scan (or high-risk variant) that must **FAIL** when targeted by the self-test runner.
- **traps/** — false-positive shapes that must **not** hard FAIL (or must stay excluded for HEURISTIC scans).

## Known-bad fixtures

| Fixture | Intended scan | Expected verdict |
|---|---|---|
| `known_bad/b3_buffer_escape.rs` | `B3-BUFFER-ESCAPE` | FAIL — public `&Buffer` return |
| `known_bad/forge_minter.rs` | `FORGE-MINTERS` | FAIL — `for_kernel_readback` minter name |
| `known_bad/unsafe_fn.rs` | `UNSAFE-FN` | FAIL — `unsafe fn` |
| `known_bad/unsafe_allow_attr.rs` | `UNSAFE-ALLOW-ATTR` | FAIL — `#![allow(unsafe_code)]` on lib.rs |
| `known_bad/unsafe_forbid_missing.rs` | `UNSAFE-FORBID-ATTR` | FAIL — lib.rs missing `#![forbid(unsafe_code)]` |
| `known_bad/as5_column_alias.rs` | `AS5-COLUMN-ALIAS` | FAIL — `type ColumnIndex = usize` |
| `known_bad/deny_toml_stub.txt` | `DENY-TOML-STUB` | FAIL — stub deny.toml content (self-test copies to temp `deny.toml`; not named `deny.toml` here to avoid tripping production whole-tree scan) |
| `known_bad/raw_data_index.rs` | `RAW-DATA-INDEX` | INSPECT — production `.data[0]` |
| `known_bad/sim_kind_read.rs` | `SIM-KIND-READ` | INSPECT — production `.kind` read |
| `known_bad/semantic_words_production.rs` | `SEMANTIC-WORDS` | INSPECT — production semantic word |
| `known_bad/spec_string_channel.rs` | `SPEC-STRING-CHANNEL` | INSPECT — stringly channel identity |
| `known_bad/allow_sealed_producer.rs` | `ALLOW-SEALED-PRODUCERS` | FAIL — explicit sealed return |
| `known_bad/allow_sealed_producer_split.rs` | `ALLOW-SEALED-PRODUCERS` | FAIL — split-declaration sealed return |
| `known_bad/allow_sealed_producer_self.rs` | `ALLOW-SEALED-PRODUCERS` | FAIL — `-> Self` in sealed impl |
| `known_bad/allow_sealed_constructor_new.rs` | `ALLOW-SEALED-PRODUCERS` | FAIL — sealed `new() -> Self` |
| `known_bad/allow_sealed_producer_doc_hidden.rs` | `ALLOW-SEALED-PRODUCERS` | FAIL — doc-hidden public minter |
| `known_bad/allow_buffer_handle.rs` | `ALLOW-BUFFER-HANDLES` | FAIL — public buffer handle escape |
| `known_bad/allow_kernel_surface_lib.rs` | `ALLOW-KERNEL-SURFACE` | FAIL — export not on allowlist |
| `known_bad/malformed_allowlist_wrong_door.txt` | allowlist validation | scanner error — wrong door-class grammar |
| `known_bad/malformed_allowlist_missing_rationale.txt` | allowlist validation | scanner error — missing rationale field |

## Trap fixtures

| Fixture | False-positive class | Expected non-failing behavior |
|---|---|---|
| `traps/jomini_write.rs` | jomini-style `write_*` | Must not trip sealed-producer door-class grammar |
| `traps/studio_antialiasing.rs` | module name substring | Must not trip `SEMANTIC-WORDS` |
| `traps/pub_crate_sealed_accessor.rs` | `pub(crate)` sealed accessor | Must not count as public sealed producer |
| `traps/comment_semantic_words.rs` | semantic words in `//` comment | Excluded by comment filter |
| `traps/cfg_test_semantic_words.rs` | semantic words in `#[cfg(test)]` | Excluded by cfg(test) heuristic filter |
| `traps/cfg_test_kind_read.rs` | `.kind` in `#[cfg(test)]` | Excluded by cfg(test) heuristic filter |

## Self-test (next rung)

`CI-A-SELF-TEST-0` will wire `doctrine_selftest.sh` to run each RELIABLE known-bad against its scan (must FAIL) and trap corpus + clean master (must NOT FAIL).
