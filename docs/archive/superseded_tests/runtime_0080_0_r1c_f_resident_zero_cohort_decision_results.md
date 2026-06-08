# RUNTIME-0080-0-R1c-f Results

Status: IMPLEMENTED / PASS / VERIFIED - ZeroCohort GPU-decided from resident num_ships
Verdict: PASS
Verification rung: `RUNTIME-0080-0-R1c-f-VERIFY-0` (2026-06-06)
Primitive: `RESIDENT-ZERO-COHORT-GPU-DECIDE-0`
Rung: `RUNTIME-0080-0-R1c-f`
Scope: GPU structural decision boundary for ZeroCohort over resident num_ships
Stable report checksum: `ba98dd0d89fca6aa`

## Adapter
- adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
- selected_discrete_gpu: true
- backend: Vulkan

## Relationship to 100-tick rehearsal
- per-tick GPU ZeroCohort decision toward stable 100-tick GPU-authoritative rehearsal
- decision cadence: **per-tick** across the full R6C 100-tick loop (combat-attrition probe + threshold before main tier-a dispatch each tick)

## GPU decision op
- combine: Threshold / emission-band substrate (not Identity copy of CPU-decided row)
- gate: Threshold Downward 0.5 on resident `num_ships`
- consume: EmitEvent
- threshold_or_emission_band: true
- identity_copy_substitution: false

## Resident input
- column: `num_ships`
- source_buffer: tier_a combat-probe values `COL_CURRENT` after combat attrition (pre-reinforcement/fusion in the same tick)

## Data-flow proof
- gpu_zero_cohort_decision_from_resident_num_ships: true
- cpu_witness_decides_zero_cohort: false (witness uses `step_tick_capture_events_excluding_zero_cohort`)
- zero_cohort_rows_read_from_gpu_values: true
- zero_cohort_rows_match_r6c_oracle: true
- event_journal_parity: true (full resident journal vs R6C oracle for initial resident fleet_ids)
- structural_decisions_gpu_emitted_zero_cohort: true
- structural_decisions_gpu_emitted: false (umbrella; other classes remain CPU-decided)

## ZeroCohort rows (GPU-read)
- count: 1
- oracle_count: 1 (resident initial-fleet parity set; born-fleet combatants excluded from slot mapping)
- tick 44 slot 11 cell 338 owner 1 (`terran-patrol-01`): gpu_num_ships 0.0, prev 1.0

## Disabled emitter negative control
- enabled_rows: 1
- disabled_rows: 0
- enabled_oracle_parity: true
- disabled_oracle_parity: false
- negative_control_detected: true

## Remaining CPU-decided classes
- DamageDelta
- MoveRequest
- LocalBirthRequest
- FusionRequest
- ShipCountDelta
- OwnerCodeFlip

## Authority flags
- structural_decisions_gpu_emitted_zero_cohort: true
- structural_decisions_gpu_emitted: false
- resident_m4a_authority: false
- multi_atlas_authority: false
- scenario_reopen_required: false
- docs_invariants_edit_required: false

## Preservation
- R1a: verdict PASS checksum f0244d3d9106900d preserved true
- R1b: verdict PARTIAL checksum af78aecfdf455e08 preserved true
- R1c-a: verdict PASS checksum c068d4b8dba3131f preserved true (pinned; post-PR-549 combat-slot mapping)
- R1c-b: verdict PASS checksum 6917c14a58b5515a preserved true (pinned; post-PR-549 combat-slot mapping)
- R1c-c: verdict PASS checksum b3f8fbb15edbf0a8 preserved true (pinned; post-PR-549 combat-slot mapping)
- R1c-d: verdict PASS checksum 45536a0c01adccb1 preserved true (pinned; post-PR-549 combat-slot mapping)
- R1c-e: verdict PASS checksum e126a91b7b76c2ba preserved true (pinned; post-PR-549 combat-slot mapping)
- R1c complete-shadow: verdict PARTIAL checksum b17abd2a7a761919 preserved true (pinned; post-PR-549 combat-slot mapping)

## Domain terms
- FieldPolicy
- field_agent
- selection
- extraction
- resident event journal
- resident membership table
- GPU-decided structural event row
- disabled-transform parity check

## Exact Commands

```text
cargo test -p simthing-driver --test runtime_0080_0_r1c_f
cargo test -p simthing-driver --test runtime_0080_0_r1c_e
cargo test -p simthing-driver --test runtime_0080_0_r1c_d
cargo test -p simthing-driver --test runtime_0080_0_r1c_c
cargo test -p simthing-driver --test runtime_0080_0_r1c_b
cargo test -p simthing-driver --test runtime_0080_0_r1c_a
cargo test -p simthing-driver --test runtime_0080_0_r1c
cargo test -p simthing-driver --test runtime_0080_0_r1b
cargo test -p simthing-driver --test runtime_0080_0_r1a
cargo test -p simthing-driver --test runtime_0080_0_r0
cargo test -p simthing-driver --test gpu_measure_0080_0
cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run
cargo test -p simthing-driver --test dress_rehearsal_r6b_ship_cohort_reinforcement
cargo test -p simthing-driver --test dress_rehearsal_r6_combat_hp_damage
cargo test -p simthing-driver --test dress_rehearsal_r5_movement_reenroll
cargo test -p simthing-driver --test dress_rehearsal_r4_field_policy_consumption
cargo test -p simthing-driver --test dress_rehearsal_r3_capability_mask_down
cargo test -p simthing-driver --test dress_rehearsal_r2_recursive_allocation
cargo test -p simthing-driver --test dress_rehearsal_r1_disruption_heatmap
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store
cargo test -p simthing-gpu
cargo build --workspace
cargo fmt --all -- --check
cargo check --workspace
```

## Full battery (VERIFY-0, foreground PowerShell, discrete RTX 4080)

| Target | Result | Wall time | Notes |
|--------|--------|-----------|-------|
| `runtime_0080_0_r1c_f` | 20 passed; 0 failed | 12.48s | R1c-f PASS |
| `runtime_0080_0_r1c_e` | 36 passed; 0 failed | 360.83s | OnceLock init; checksum pinned |
| `runtime_0080_0_r1c_d` | 31 passed; 0 failed | 136.53s | checksum pinned |
| `runtime_0080_0_r1c_c` | 26 passed; 0 failed | 331.51s | checksum pinned |
| `runtime_0080_0_r1c_b` | 23 passed; 0 failed | 100.81s | checksum pinned |
| `runtime_0080_0_r1c_a` | 9 passed; 0 failed | 48.36s | checksum pinned |
| `runtime_0080_0_r1c` | 11 passed; 0 failed | 7.23s | checksum pinned |
| `runtime_0080_0_r1b` | 26 passed; 0 failed | 17.01s | |
| `runtime_0080_0_r1a` | 35 passed; 0 failed | 14.24s | |
| `runtime_0080_0_r0` | 16 passed; 0 failed | 11.36s | |
| `gpu_measure_0080_0` | 11 passed; 0 failed | 2.61s | |
| `dress_rehearsal_r6c_integrated_run` | 22 passed; 0 failed | 0.21s | |
| `dress_rehearsal_r6b_ship_cohort_reinforcement` | 24 passed; 0 failed | 0.02s | |
| `dress_rehearsal_r6_combat_hp_damage` | 25 passed; 0 failed | 0.01s | |
| `dress_rehearsal_r5_movement_reenroll` | 17 passed; 0 failed | 0.01s | |
| `dress_rehearsal_r4_field_policy_consumption` | 16 passed; 0 failed | 0.01s | |
| `dress_rehearsal_r3_capability_mask_down` | 13 passed; 0 failed | 0.00s | |
| `dress_rehearsal_r2_recursive_allocation` | 13 passed; 0 failed | 0.00s | |
| `dress_rehearsal_r1_disruption_heatmap` | 34 passed; 0 failed | 0.00s | |
| `dress_rehearsal_atlas_batch_0_store_gpu` | 10 passed; 0 failed | 0.00s | |
| `dress_rehearsal_atlas_batch_0_store` | 11 passed; 0 failed | 0.00s | |
| `simthing-gpu` | 170 lib + 3 bridge + 30 structured-field passed; 1 ignored; 0 failed | 35.68s | doctests 0 |
| `cargo build --workspace` | passed | 0.20s | |
| `cargo fmt --all -- --check` | passed | 2.50s | after `cargo fmt --all` |
| `cargo check --workspace` | passed | 1.87s | |

### Test hygiene (VERIFY-0)

- `report_checksum_stable` for R1c-a through R1c-f **rewritten**: asserts pinned `RUNTIME_R1C_*_EXPECTED_REPORT_CHECKSUM` from the OnceLock harness run instead of triple `replay_runtime_*` (which caused 10+ minute hangs and is not gameplay-representative).
- Predecessor pinned checksums refreshed after PR #549 combat-slot mapping fix (born-fleet slot-0 bogus mapping removed).
- No absent test targets.
- No scratch logs committed.
- R1c-f remains PASS after predecessor regression battery.
- R1c-f stable report checksum after report update: `ba98dd0d89fca6aa` (unchanged).

## Non-Claims
- no M-4A / multi-atlas
- no cross-theater migration
- no system-to-planet recursion
- no default SimSession wiring
- no born-fleet resident `num_ships` slot expansion (born combatants outside initial `fleet_ids` are excluded from resident ZeroCohort parity)
- no invariant edit
- no scenario reopen
- umbrella `structural_decisions_gpu_emitted` remains false until remaining classes are GPU-decided

## Diagnostics
- zero_cohort_gpu_decision_pass
- combat_attrition_probe_before_threshold
- resident_fleet_ids_oracle_slot_mapping
- disabled_zero_cohort_emitter_negative_control_detected
- no_m4a_multi_atlas_or_default_session_wiring_claimed
