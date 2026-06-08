# RUNTIME-0080-0-R1c-e Results

Status: IMPLEMENTED / PASS - resident compacted-view apply; no M-4A
Verdict: PASS
Primitive: `RESIDENT-COMPACTED-VIEW-APPLY-0`
Rung: `RUNTIME-0080-0-R1c-e`
Scope: resident compacted-view apply / resident slot-table rewrite; no M-4A
Stable report checksum: `d823ece4dc0f5dab`

## Adapter
- adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
- selected_discrete_gpu: true
- backend: Vulkan

## Resident Compacted-View Apply
- relationship_to_r1c_d: consumes R1c-d compaction map and lineage staging plus R1c-c membership rows
- slot_remap_representation: resident old_slot -> new_slot_or_tombstone remap rows plus survivor link
- resident_compacted_table_representation: resident compacted slot-table rows keyed by compacted slot_id
- membership_remap_representation: resident membership row remap/link rows keyed by old membership slot
- resident_slot_remap_created: true
- resident_compacted_slot_table_created: true
- resident_membership_remap_created: true
- gpu_writes_slot_remap_rows: true
- gpu_applies_compacted_slot_table: true
- gpu_writes_membership_remap_rows: true
- remap_rows_read_from_gpu_values: true
- compacted_table_read_from_gpu_values: true
- membership_remap_rows_read_from_gpu_values: true
- slot_remap_rows_written: 16
- compacted_slot_rows_written: 17
- membership_remap_rows_written: 426
- tombstone_rows_applied: 12
- absorption_rows_applied: 10
- birth_allocation_rows_preserved: 4
- lineage_rows_preserved_after_apply: true
- membership_rows_remapped_or_linked_from_gpu_values: true
- gpu_slot_remap_dispatch_count: 16
- gpu_compacted_table_dispatch_count: 17
- gpu_membership_remap_dispatch_count: 426
- slot_remap_readback_count: 1
- compacted_table_readback_count: 1
- membership_remap_readback_count: 1
- slot_remap_ops_uploaded: 176
- compacted_table_ops_uploaded: 153
- membership_remap_ops_uploaded: 3834

## Inputs Consumed
- consumes_r1c_d_compaction_rows: true
- consumes_r1c_d_lineage_rows: true
- consumes_r1c_c_membership_rows: true
- r1c_d_compaction_rows_consumed: 16
- r1c_d_lineage_rows_consumed: 26
- r1c_c_membership_rows_consumed: 426

## CPU Shadow
- consumes_slot_remap_without_redeciding: true
- consumes_compacted_table_without_redeciding: true
- consumes_lineage_without_redeciding: true
- cpu_decided_any_slot_remap: false
- cpu_decided_any_compacted_table_row: false
- cpu_decided_any_lineage_application: false
- cpu_shadow_does_not_rewrite_slot_mapping_first: true
- remap_shadow_matches_gpu_rows: true
- compacted_table_shadow_matches_gpu_rows: true

## Disabled-Writer Parity Checks
- disabled_remap_writer_negative_control_detected: true
- disabled_compacted_table_writer_negative_control_detected: true
- disabled_membership_remap_writer_negative_control_detected: true
- remap writers enabled rows: 16
- remap writers disabled rows: 0
- remap writers enabled parity: true
- remap writers disabled parity: false
- compacted-table writers enabled rows: 17
- compacted-table writers disabled rows: 0
- compacted-table writers enabled parity: true
- compacted-table writers disabled parity: false
- membership-remap writers enabled rows: 426
- membership-remap writers disabled rows: 0
- membership-remap writers enabled parity: true
- membership-remap writers disabled parity: false

## Authority Flags
- resident_slot_table_apply_authority: true
- resident_compacted_view_apply_authority: true
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
- R1c-d: verdict PASS checksum 51b0066e4bd6e111 preserved true
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
- resident compacted view
- resident slot-table apply
- disabled-transform parity check

## Exact Commands

```text
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
- no multi-faction economy expansion
- no default SimSession wiring
- no global sparse-residency scheduler
- no ClauseThing/L3
- no UI/realtime/player control
- no invariant edit
- no scenario reopen
- `structural_decisions_gpu_emitted` remains under the wider R1c stop-line umbrella

## Diagnostics
- resident_compacted_view_apply_pass
- gpu_writes_slot_remap_rows
- gpu_applies_compacted_slot_table
- gpu_writes_membership_remap_rows
- cpu_shadow_consumes_compacted_view_without_redeciding
- disabled_remap_compacted_table_and_membership_writer_negative_controls_detected
- no_m4a_multi_atlas_recursion_or_default_session_wiring_claimed
