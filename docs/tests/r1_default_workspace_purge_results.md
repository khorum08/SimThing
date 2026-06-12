# R1 default workspace purge results

> **Status: PASS (R1 purge; 2026-06-11).** Historical R1* proof-ledger/report/checksum batteries removed
> from default `cargo test --workspace`. Fast live sentinels retained. No production runtime behavior
> changed. Workspace gate: **~95s**, zero R1* “running for over 60 seconds” warnings (was 60s+ × dozens).

## Summary

Default workspace no longer runs historical R1* proof-ledger/report/checksum batteries. Fast live
sentinels remain where needed. Historical proof batteries must not be reintroduced as default tests.

Prior R1C-B/C-only cleanup: [`r1c_default_gate_cleanup_results.md`](r1c_default_gate_cleanup_results.md).

## Classification table

| Rung | Deleted tests | Ignored-heavy tests | Fast sentinels retained | Notes |
|---|---:|---:|---|---|
| R1a | 35 (`runtime_0080_0_r1a.rs` binary) | 0 | `r1_fast_default_off_or_opt_in_contract` (gate) | OnceLock report + checksum + disabled/reenabled transform theater removed |
| R1b | 24 (`runtime_0080_0_r1b.rs` binary) | 0 | gate + `r1_fast_event_journal_marks_one_free_slot_from_structural_event` (unit) | Journal parity/checksum/preservation rows deleted |
| R1c | 11 (`runtime_0080_0_r1c.rs` binary) | 0 | gate default-off contract | Stop-line report checksum replay deleted |
| R1c-a | 9 (`runtime_0080_0_r1c_a.rs` binary) | 0 | gate + `r1_fast_mark_table_marks_one_slot_from_sources` (unit) | Mark parity/checksum/disabled-writer theater deleted |
| R1c-b | 22 (prior R1C-GATE) | 0 | gate + `r1c_fast_allocation_selects_one_compatible_marked_slot` (unit) | Already purged 2026-06-11 |
| R1c-c | 24 (prior R1C-GATE) | 0 | gate + `r1c_fast_membership_delta_applies_to_one_slot` (unit) | Already purged 2026-06-11 |
| R1c-d | 31 (`runtime_0080_0_r1c_d.rs` binary) | 0 | gate + `r1_fast_compaction_remap_preserves_one_lineage_row`, `r1_fast_cpu_shadow_observes_compaction_without_redeciding` (unit) | ~102s OnceLock binary removed |
| R1c-e | 36 (`runtime_0080_0_r1c_e.rs` binary) | 0 | gate + `r1_fast_compaction_remap_preserves_one_compacted_slot_row`, `r1_fast_cpu_shadow_observes_remap_without_redeciding` (unit) | ~60s+ OnceLock binary removed |
| R1c-f | 20 (`runtime_0080_0_r1c_f.rs` binary) | 0 | gate default-off contract | Zero-cohort checksum/preservation theater deleted |

**Integration sentinel binary:** `crates/simthing-driver/tests/runtime_0080_0_r1_gate.rs` (2 tests,
milliseconds). Supersedes `runtime_0080_0_r1c_gate.rs`.

## Deleted proof patterns (all rungs)

Removed from default workspace (deleted, not renamed):

- `*_report_checksum_stable`
- `*_disabled_*_fails_parity` / `*_reenabled_*_restores_*_parity`
- `*_preserves_*_parity` / `*_preserves_*_shadow_contract`
- `*_domain_neutral_terms_only` / `*_uses_domain_neutral_terms`
- `*_no_invariant_edit_or_scenario_reopen` / `*_no_m4a_or_multi_atlas`
- `*_no_default_session_wiring`
- Shared `OnceLock` proof reports and full-scenario replays

## Production behavior

No runtime semantics changed. Purge deletes/quarantines test scaffolding only.

## Artifact cleanup

- Deleted integration proof batteries listed above (8 binaries, ~192 tests total).
- Historical markdown proof reports remain under `docs/archive/superseded_tests/` only.
- `docs/tests/runtime_0080_0_r1a_next_tick_authority_results.md` is historical closure evidence only;
  not a default gate dependency.

## Workspace gate

```text
cargo fmt --all -- --check
cargo test -p simthing-driver --test runtime_0080_0_r1_gate
cargo test -p simthing-driver r1_fast_
cargo test --workspace -- --list | grep r1
cargo test --workspace
```

*(Workspace result recorded after gate run.)*

### Full workspace (R1* gate PASS)

```text
cargo test --workspace    ~95s; zero R1* “running for over 60 seconds” warnings
```

Local agent hit pre-existing Windows elevation error (`os error 740`) on
`palma_path_5_install_session_property` (reproduces on clean `master`; unrelated to R1 purge).
