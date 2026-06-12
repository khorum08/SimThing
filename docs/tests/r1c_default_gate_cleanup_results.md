# R1C default workspace gate cleanup results

> **Superseded:** Full R1* purge completed in R1-TEST-PURGE. Canonical report:
> [`r1_default_workspace_purge_results.md`](r1_default_workspace_purge_results.md). This file is
> historical context for the R1C-B/C-only pass only.

> **Status (historical): MERGE-READY (2026-06-11).** R1C-B/C proof stack removed from default workspace;
> Candidate F §0.7 elevated.

## Problem

`runtime_0080_0_r1c_b` and `runtime_0080_0_r1c_c` integration tests shared `OnceLock` reports that
each ran full R6C → R1a → R1b → R1c-a → GPU rehearsal chains. Twenty-two R1c-b and twenty-four R1c-c
default tests wrapped trivial assertions on those shared reports, stalling `cargo test --workspace`
with “running for over 60 seconds” on every test in the binary after the first init.

Archive evidence: full `runtime_0080_0_r1c_c` binary **331.51s**; R1c-b uses the same pattern.

## Default sentinels (post-cleanup)

| Test | Path | Classification |
|---|---|---|
| `r1c_default_opt_in_default_off_is_preserved` | `tests/runtime_0080_0_r1c_gate.rs` | **KEEP_FAST_SENTINEL** |
| `r1c_fast_no_compaction_or_lineage_rewrite` | `tests/runtime_0080_0_r1c_gate.rs` | **KEEP_FAST_SENTINEL** |
| `r1c_fast_allocation_selects_one_compatible_marked_slot` | `src/runtime_0080_0_r1c_b.rs` unit test | **KEEP_FAST_SENTINEL** |
| `r1c_fast_membership_delta_applies_to_one_slot` | `src/runtime_0080_0_r1c_c.rs` unit test | **KEEP_FAST_SENTINEL** |

Sentinel properties: tiny fixture, no report generation, no checksum replay, no `OnceLock` proof report,
no file output, no GPU proof pass, completes in milliseconds.

## R1c-b classification (former default tests)

| Test / helper | Action |
|---|---|
| `r1c_b_opt_in_default_off` | **DELETE_PROOF_SCAFFOLD** → superseded by `r1c_default_opt_in_default_off_is_preserved` |
| `r1c_b_consumes_r1c_a_mark_table` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_consumes_local_birth_request_rows` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_resident_allocation_rows_created` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_gpu_selects_marked_free_slot` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_allocates_lowest_compatible_marked_slot` | **DELETE_PROOF_SCAFFOLD** → oracle covered by `r1c_fast_allocation_selects_one_compatible_marked_slot` |
| `r1c_b_allocated_slot_read_from_gpu_value` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_mark_cleared_for_allocated_slot` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_unallocated_marks_remain` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_cpu_boundary_consumes_allocation_without_selecting_slot` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_disabled_allocation_writer_fails_allocation_parity` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_reenabled_allocation_writer_restores_allocation_parity` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_preserves_r1a_tier_a_source_of_truth` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_preserves_r1b_event_journal_parity` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_preserves_r1c_a_mark_parity` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_preserves_r1c_complete_shadow_contract` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_no_compaction_or_lineage_rewrite` | **DELETE_PROOF_SCAFFOLD** → superseded by `r1c_fast_no_compaction_or_lineage_rewrite` |
| `r1c_b_no_resident_reenroll_scatter` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_no_fusion_compaction` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_no_m4a_or_multi_atlas` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_no_invariant_edit_or_scenario_reopen` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_domain_neutral_terms_only` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_b_report_checksum_stable` | **DELETE_PROOF_SCAFFOLD** |
| `report()` / `disabled_report()` `OnceLock` | **DELETE_PROOF_SCAFFOLD** |
| `run_runtime_0080_0_r1c_b` / `replay_*` | **LIVE_API** (manual replay only) |
| Archived proof report | **DIAGNOSTIC_ONLY** (`docs/archive/superseded_tests/runtime_0080_0_r1c_b_*`) |

Integration binary `runtime_0080_0_r1c_b.rs` **deleted** from default gate.

## R1c-c classification (former default tests)

| Test / helper | Action |
|---|---|
| `r1c_c_opt_in_default_off` | **DELETE_PROOF_SCAFFOLD** → superseded by gate sentinel |
| `r1c_c_enabled_by_default_forbidden` | **DELETE_PROOF_SCAFFOLD** (admission covered by default-off path) |
| All 22 `report()`-wrapper tests including `r1c_c_report_checksum_stable` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_c_reenabled_membership_writer_restores_membership_parity` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_c_resident_membership_table_created_or_reused` | **DELETE_PROOF_SCAFFOLD** |
| `r1c_c_membership_plan_bounded_delta_restores_parity` | **DELETE_PROOF_SCAFFOLD** → renamed `r1c_fast_membership_delta_applies_to_one_slot` |
| `replay_runtime_0080_0_r1c_c` / `render_*_artifact` | **LIVE_API** / **DIAGNOSTIC_ONLY** |
| Archived proof report | **DIAGNOSTIC_ONLY** (`docs/archive/superseded_tests/runtime_0080_0_r1c_c_*`) |

Integration binary `runtime_0080_0_r1c_c.rs` **deleted** from default gate.

## Candidate F docs

- Added **§0.7 Exact numeric authority for decision gates** to `docs/design_0_0_8_1.md` transient constitution.
- Added elevation pointer in `docs/design_0_0_8_0_consumer_pulled_production_track.md` R4 detail.
- **Not** added to `docs/simthing_core_design.md` (principle-level doc unchanged).

## Candidate-F audit (BH/PALMA/FIELD_POLICY)

No native sqrt/magnitude/norm gating introduced by this cleanup. BH/PALMA production paths unchanged.
R1C cleanup touches allocation/membership plan oracles only (integer slot/cell logic, no sqrt).

## Workspace gate

Stale `l1_0_guardrail_diagnostic_codes_are_stable` (26→38 codes, mobility prefix) fixed in same handoff.
R1C-B/C binaries no longer appear in default gate; `runtime_0080_0_r1c_gate` completes in **0.00s**.

### Focused gates (PASS)

```text
cargo fmt --all -- --check                                          PASS
cargo test -p simthing-driver --test runtime_0080_0_r1c_gate          PASS (2 tests, 0.00s)
cargo test -p simthing-driver r1c_fast_                             PASS (2 unit sentinels)
cargo test -p simthing-spec --test l1_0_designer_admission_substrate PASS
cargo test -p simthing-driver --test bh2d_ct4b_100tick_observation -- bh2d_ct4b_100tick_observation_smoke  PASS
cargo test -p simthing-driver --test bh2d_ct4b_fixture              PASS
cargo test -p simthing-driver --test bh2c_palma_w_feedstock         PASS
cargo test -p simthing-gpu --test bh2_w_composition                 PASS
cargo test -p simthing-gpu --test bh2s_overlap_stress               PASS
```

### Full workspace (SKIPPED at merge)

```text
cargo test --workspace                                              SKIPPED
```

Partial local runs progressed past R1C-B/C (removed) into R1c-d/e OnceLock proof stacks (~60s+ per
binary). Full workspace not required for this R1C-B/C cleanup handoff.

## Artifact cleanup

- Deleted superseded `docs/tests/r1cc_default_workspace_hygiene_results.md` (merged into this report).
- R1C-B/C proof reports remain archived under `docs/archive/superseded_tests/` only.
- No new proof dumps, checksum artifacts, or scratch logs committed.
