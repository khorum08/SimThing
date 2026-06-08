# RUNTIME-0080-0-R1c-b Results

Status: IMPLEMENTED / PASS - resident allocation into marked free slots; no compaction
Verdict: PASS
Primitive: `RESIDENT-FREESLOT-ALLOC-0`
Rung: `RUNTIME-0080-0-R1c-b`
Scope: resident free-slot allocation from R1c-a marks and R1b LocalBirthRequest rows
Stable report checksum: `a64c50e921431a68`

## Adapter

- adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
- selected_discrete_gpu: true
- backend: Vulkan

## Resident Free-Slot Allocation

- relationship_to_r1c_a: consumes R1c-a resident mark table
- free_slot_mark_count_before_allocation: 11
- local_birth_request_count: 4
- allocation_rows_written: 4
- allocation_failures: 0
- mark_table_before_allocation: `[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11]`
- allocated_slots: `[0, 1, 2, 3]`
- expected_allocated_slots: `[0, 1, 2, 3]`
- mark_table_after_allocation: `[4, 5, 6, 7, 8, 9, 11]`
- allocation_parity_measured_from_gpu_values: true
- gpu_select_dispatch_count: 4
- allocation_row_copy_dispatch_count: 4
- allocation_readback_count: 8
- allocation_ops_uploaded: 36

## GPU-Side Allocation Rows

- tick 49 request 0 owner 1 source_cell 338 allocated_slot 0 success true failure none
- tick 49 request 1 owner 2 source_cell 284 allocated_slot 1 success true failure none
- tick 99 request 2 owner 1 source_cell 338 allocated_slot 2 success true failure none
- tick 99 request 3 owner 2 source_cell 284 allocated_slot 3 success true failure none

## Boundary Pass

- cpu_boundary_pass_consumes_allocation_row: true
- cpu_boundary_pass_does_not_select_slot: true
- cpu_selected_any_slot: false
- rows_consumed: 4
- enrolled_slots: `[0, 1, 2, 3]`

## Disabled-Transform Parity Check

- writers_enabled_rows: 4
- writers_disabled_rows: 0
- writers_enabled_allocation_parity: true
- writers_disabled_allocation_parity: false
- negative_control_detected: true
- disabled_report_checksum: `e3e2b4640b8a0b0b`

## Authority Flags

- resident_free_list_allocation_authority: true
- allocation_rows_written_from_gpu_values: true
- allocated_slot_read_from_gpu_value: true
- resident_compaction_authority: false
- resident_reenroll_scatter_authority: false
- resident_arena_membership_rewrite_authority: false
- resident_fusion_compaction_authority: false
- resident_lineage_rewrite_authority: false
- m4a_or_multi_atlas_authority: false
- docs_invariants_edit_required: false
- scenario_reopen_required: false

## Preservation

- R1a: verdict PASS checksum `f0244d3d9106900d` preserved true
- R1b: verdict PARTIAL checksum `af78aecfdf455e08` preserved true
- R1c-a: verdict PASS checksum `2f4cd7b82b07ca7d` preserved true
- R1c: verdict PARTIAL checksum `8fdd8977a84b4699` preserved true

## Domain Terms

- FieldPolicy
- field_agent
- selection
- extraction
- resident event journal
- resident mark table
- resident free-slot allocation
- GPU-side allocation rows
- disabled-transform parity check

## Non-Claims

R1c-b does not implement resident compaction, resident REENROLL scatter, resident arena membership rewrite, fusion compaction, lineage rewrite, M-4A / multi-atlas, system-to-planet recursion, multi-faction economy expansion, invariant edits, or scenario reopen.

## Command Results

- `cargo test -p simthing-driver --test runtime_0080_0_r1c_b` -> 23 passed; 0 failed; 0 ignored; finished in 219.73s.
- `cargo test -p simthing-driver --test runtime_0080_0_r1c_a` -> 9 passed; 0 failed; 0 ignored; finished in 74.89s.
- `cargo test -p simthing-driver --test runtime_0080_0_r1c` -> 11 passed; 0 failed; 0 ignored; finished in 19.26s.
- `cargo test -p simthing-driver --test runtime_0080_0_r1b` -> 26 passed; 0 failed; 0 ignored; finished in 16.80s.
- `cargo test -p simthing-driver --test runtime_0080_0_r1a` -> 35 passed; 0 failed; 0 ignored; finished in 12.98s.
- `cargo test -p simthing-driver --test runtime_0080_0_r0` -> 16 passed; 0 failed; 0 ignored; finished in 10.69s.
- `cargo test -p simthing-driver --test gpu_measure_0080_0` -> 11 passed; 0 failed; 0 ignored; finished in 2.80s.
- `cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run` -> 22 passed; 0 failed; 0 ignored; finished in 0.20s.
- `cargo test -p simthing-driver --test dress_rehearsal_r6b_ship_cohort_reinforcement` -> 24 passed; 0 failed; 0 ignored; finished in 0.02s.
- `cargo test -p simthing-driver --test dress_rehearsal_r6_combat_hp_damage` -> 25 passed; 0 failed; 0 ignored; finished in 0.01s.
- `cargo test -p simthing-driver --test dress_rehearsal_r5_movement_reenroll` -> 17 passed; 0 failed; 0 ignored; finished in 0.01s.
- `cargo test -p simthing-driver --test dress_rehearsal_r4_field_policy_consumption` -> 16 passed; 0 failed; 0 ignored; finished in 0.01s.
- `cargo test -p simthing-driver --test dress_rehearsal_r3_capability_mask_down` -> 13 passed; 0 failed; 0 ignored; finished in 0.00s.
- `cargo test -p simthing-driver --test dress_rehearsal_r2_recursive_allocation` -> 13 passed; 0 failed; 0 ignored; finished in 0.00s.
- `cargo test -p simthing-driver --test dress_rehearsal_r1_disruption_heatmap` -> 34 passed; 0 failed; 0 ignored; finished in 0.00s.
- `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu` -> 10 passed; 0 failed; 0 ignored; finished in 0.00s.
- `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store` -> 11 passed; 0 failed; 0 ignored; finished in 0.00s.
- `cargo test -p simthing-gpu` -> passed: 170 passed / 1 ignored in lib tests, 3 passed in GPU bridge tests, 30 passed in structured field stencil tests, 0 doctests.
- `cargo build --workspace` -> passed.
- `cargo fmt --all -- --check` -> passed.
- `cargo check --workspace` -> passed.

During the sweep, `runtime_0080_0_r1c_a` and `runtime_0080_0_r1c` initially failed only their pinned stable-checksum assertions after R1b's report surface added the GPU-read LocalBirthRequest projection. Their behavioral tests passed; the expected checksum constants and corresponding report docs were updated to `2f4cd7b82b07ca7d` for R1c-a and `8fdd8977a84b4699` for R1c, then both focused tests passed.
