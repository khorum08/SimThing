# RUNTIME-0080-0-R1c-d Results

Status: IMPLEMENTED / PASS - resident compaction-map and lineage staging; no M-4A
Verdict: PASS
Primitive: `RESIDENT-COMPACTION-LINEAGE-0`
Rung: `RUNTIME-0080-0-R1c-d`
Scope: resident compaction-map and lineage staging from R1b/R1c-a/R1c-b/R1c-c rows; no M-4A
Stable report checksum: `51b0066e4bd6e111`

## Adapter
- adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
- selected_discrete_gpu: true
- backend: Vulkan

## Resident Compaction And Lineage
- relationship_to_r1c_c: consumes landed R1c-c resident membership row classes/projection plus R1b/R1c-a/R1c-b structural inputs
- compaction_representation: old_slot -> new_slot_or_tombstone append-only map plus derived active view
- lineage_representation: append-only resident lineage staging rows keyed by generic lineage_event_id
- resident_compaction_map_created: true
- resident_lineage_staging_created: true
- gpu_writes_compaction_rows: true
- gpu_writes_lineage_rows: true
- compaction_rows_read_from_gpu_values: true
- lineage_rows_read_from_gpu_values: true
- compaction_rows_written: 16
- lineage_rows_written: 26
- tombstone_rows_written: 2
- fusion_absorption_rows_written: 10
- birth_lineage_rows_written: 4
- gpu_compaction_copy_dispatch_count: 16
- gpu_lineage_copy_dispatch_count: 26
- compaction_readback_count: 1
- lineage_readback_count: 1
- compaction_ops_uploaded: 176
- lineage_ops_uploaded: 260

## Inputs Consumed
- consumes_r1b_event_journal: true
- consumes_r1c_a_mark_table: true
- consumes_r1c_b_allocation_rows: true
- consumes_r1c_c_membership_rows: true
- r1b_event_rows_consumed: 247
- r1c_a_mark_rows_consumed: 11
- r1c_b_allocation_rows_consumed: 4
- r1c_c_membership_rows_consumed: 6

## CPU Shadow
- consumes_compaction_rows_without_redeciding: true
- consumes_lineage_rows_without_redeciding: true
- cpu_decided_any_compaction_row: false
- cpu_decided_any_lineage_row: false
- compaction_shadow_matches_gpu_rows: true
- lineage_shadow_matches_gpu_rows: true

## Disabled-Writer Parity Checks
- disabled_compaction_writer_negative_control_detected: true
- disabled_lineage_writer_negative_control_detected: true
- compaction writers enabled rows: 16
- compaction writers disabled rows: 0
- compaction writers enabled parity: true
- compaction writers disabled parity: false
- lineage writers enabled rows: 26
- lineage writers disabled rows: 0
- lineage writers enabled parity: true
- lineage writers disabled parity: false

## Authority Flags
- resident_compaction_authority: true (resident compaction-map staging only)
- resident_lineage_staging_authority: true
- resident_fusion_compaction_authority: false
- resident_lineage_rewrite_authority: false
- resident_m4a_authority: false
- multi_atlas_authority: false
- system_planet_recursion_authority: false
- default_session_wiring: false
- docs_invariants_edit_required: false
- scenario_reopen_required: false

## Preservation
- R1a: verdict PASS checksum f0244d3d9106900d preserved true
- R1b: verdict PARTIAL checksum af78aecfdf455e08 preserved true
- R1c-a: verdict PASS checksum 2f4cd7b82b07ca7d preserved true
- R1c-b: verdict PASS checksum a64c50e921431a68 preserved true
- R1c-c: verdict PASS checksum 9581b0838619d9c0 preserved true
- R1c: verdict PARTIAL checksum 8fdd8977a84b4699 preserved true

## Domain Terms
- FieldPolicy
- field_agent
- selection
- extraction
- resident event journal
- resident mark table
- resident allocation rows
- resident membership table
- resident compaction map
- resident lineage staging
- disabled-writer parity check

## Exact Commands

```text
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
- `simthing-gpu`: passed, including lib, bridge integration, structured field stencil, and doctests.
- `cargo build --workspace`: passed.
- `cargo fmt --all -- --check`: passed.
- `cargo check --workspace`: passed.

## Non-Claims

- no physical resident table compaction
- no fusion compaction rewrite
- no lineage rewrite beyond resident staging rows
- no M-4A / multi-atlas
- no system-to-planet recursion
- no default SimSession wiring
- no invariant edit
- no scenario reopen
- `structural_decisions_gpu_emitted` remains under the wider R1c stop-line umbrella

## Diagnostics
- resident_compaction_map_and_lineage_staging_pass
- gpu_stages_compaction_rows_from_resident_inputs
- gpu_stages_lineage_rows_from_resident_inputs
- cpu_shadow_consumes_compaction_and_lineage_without_redeciding
- disabled_compaction_and_lineage_writer_negative_controls_detected
- no_m4a_multi_atlas_recursion_or_default_session_wiring_claimed
