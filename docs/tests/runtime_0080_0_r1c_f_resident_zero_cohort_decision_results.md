# RUNTIME-0080-0-R1c-f Results

Status: IMPLEMENTED / PASS - ZeroCohort GPU-decided from resident num_ships
Verdict: PASS
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
- R1c-a: verdict PASS checksum 2f4cd7b82b07ca7d preserved true
- R1c-b: verdict PASS checksum a64c50e921431a68 preserved true
- R1c-c: verdict PASS checksum 9581b0838619d9c0 preserved true
- R1c-d: verdict PASS checksum 51b0066e4bd6e111 preserved true
- R1c-e: verdict PASS checksum d823ece4dc0f5dab preserved true
- R1c complete-shadow: verdict PARTIAL checksum 8fdd8977a84b4699 preserved true

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

- `runtime_0080_0_r1c_f`: 20 passed; 0 failed.
- `runtime_0080_0_r1c_e`: 36 passed; 0 failed.
- `runtime_0080_0_r1c_d`: 31 passed; 0 failed.
- `runtime_0080_0_r1c_c`: 26 passed; 0 failed.
- `runtime_0080_0_r1c_b`: 23 passed; 0 failed.
- `runtime_0080_0_r1c_a`: 9 passed; 0 failed.
- `runtime_0080_0_r1c`: 11 passed; 0 failed.
- `runtime_0080_0_r1b`: 26 passed; 0 failed.
- `runtime_0080_0_r1a`: 35 passed; 0 failed.
- `runtime_0080_0_r0`: 16 passed; 0 failed.
- `gpu_measure_0080_0`: 11 passed; 0 failed.
- `dress_rehearsal_r6c_integrated_run`: 22 passed; 0 failed.
- `dress_rehearsal_r6b_ship_cohort_reinforcement`: 24 passed; 0 failed.
- `dress_rehearsal_r6_combat_hp_damage`: 25 passed; 0 failed.
- `dress_rehearsal_r5_movement_reenroll`: 17 passed; 0 failed.
- `dress_rehearsal_r4_field_policy_consumption`: 16 passed; 0 failed.
- `dress_rehearsal_r3_capability_mask_down`: 13 passed; 0 failed.
- `dress_rehearsal_r2_recursive_allocation`: 13 passed; 0 failed.
- `dress_rehearsal_r1_disruption_heatmap`: 34 passed; 0 failed.
- `dress_rehearsal_atlas_batch_0_store_gpu`: 10 passed; 0 failed.
- `dress_rehearsal_atlas_batch_0_store`: 11 passed; 0 failed.
- `simthing-gpu`: 170 lib tests passed, 1 ignored; 3 bridge tests passed; 30 structured-field tests passed; doctests passed.
- `cargo build --workspace`: passed.
- `cargo fmt --all -- --check`: passed.
- `cargo check --workspace`: passed.

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
