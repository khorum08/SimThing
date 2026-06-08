# RUNTIME-0080-0-R1c-c Results

Status: IMPLEMENTED / PASS - resident membership apply; no compaction
Verdict: PASS
Primitive: `RESIDENT-MEMBERSHIP-APPLY-0`
Rung: `RUNTIME-0080-0-R1c-c`
Scope: resident membership apply from R1b journal and R1c-b allocation rows; no compaction
Stable report checksum: `9581b0838619d9c0`

## Adapter
- adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
- selected_discrete_gpu: true
- backend: Vulkan
## Resident Membership Apply
- relationship_to_r1c_b: consumes R1c-b resident allocation rows and R1b journal rows
- membership_representation: slot-to-cell table plus append-only membership delta rows
- resident_membership_table_created_or_reused: true
- gpu_writes_membership_delta_rows: true
- membership_apply_reads_gpu_rows: true
- movement_membership_delta_count: 376
- birth_membership_delta_count: 4
- departure_membership_delta_count: 2
- owner_code_update_count: 44
- source_removals_applied: 188
- destination_additions_applied: 188
- allocated_birth_slots_added: 4
- membership_parity_measured_from_gpu_values: true
- gpu_membership_apply_dispatch_count: 426
- membership_delta_copy_dispatch_count: 426
- membership_readback_count: 427
- membership_ops_uploaded: 5112

## CPU Shadow
- observes_after_gpu_apply: true
- does_not_apply_membership_before_gpu: true
- cpu_selected_membership_effects: false
- shadow_matches_oracle: true

## Disabled-Transform Parity Check
- writers_enabled_rows: 426
- writers_disabled_rows: 0
- writers_enabled_membership_parity: true
- writers_disabled_membership_parity: false
- negative_control_detected: true
- disabled_report_checksum: 0a7ec0b9ea162cc9
## Authority Flags
- resident_membership_apply_authority: true
- resident_reenroll_scatter_authority: true
- resident_arena_membership_rewrite_authority: true
- resident_compaction_authority: false
- resident_lineage_rewrite_authority: false
- resident_fusion_compaction_authority: false
- resident_m4a_authority: false
- docs_invariants_edit_required: false
- scenario_reopen_required: false

## Preservation
- R1a: verdict PASS checksum f0244d3d9106900d preserved true
- R1b: verdict PARTIAL checksum af78aecfdf455e08 preserved true
- R1c-a: verdict PASS checksum 2f4cd7b82b07ca7d preserved true
- R1c-b: verdict PASS checksum a64c50e921431a68 preserved true
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
- membership apply
- disabled-transform parity check

## Exact Commands

```text
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

`runtime_0080_0_r1c_c`: 26 passed; 0 failed.

## Non-Claims

- no resident compaction
- no lineage rewrite
- no fusion compaction
- no M-4A / multi-atlas
- no invariant edit
- no scenario reopen
- `structural_decisions_gpu_emitted` remains false at the R1c stop-line umbrella

## Diagnostics
- resident_membership_apply_pass
- gpu_applies_move_source_removal_and_destination_addition
- gpu_applies_birth_membership_for_allocated_slots
- cpu_shadow_observes_after_gpu_apply_without_selecting_membership
- disabled_membership_writer_negative_control_detected
- no_compaction_lineage_fusion_or_m4a_claimed
