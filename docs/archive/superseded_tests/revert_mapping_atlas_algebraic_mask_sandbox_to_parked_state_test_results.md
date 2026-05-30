# Revert Verification — M-4A Algebraic Atlas Mask Sandbox

**Date:** 2026-05-19  
**HEAD at verification:** `683ae0bfda6d979cc53aca05f0baeba3a90a6678` (pre-merge; post-merge SHA recorded after PR)

## Reverted / removed (transient sandbox)

- `crates/simthing-gpu/src/structured_field_stencil_atlas_mask_sandbox.rs`
- `crates/simthing-gpu/src/shaders/structured_field_stencil_atlas_mask_sandbox.wgsl`
- `crates/simthing-driver/tests/mapping_atlas_algebraic_mask_sandbox.rs`
- `crates/simthing-driver/tests/support/mapping_atlas_algebraic_mask_sandbox.rs`
- `crates/simthing-gpu/tests/structured_field_stencil_atlas_algebraic_mask_sandbox.rs`
- Temporary driver dev-deps (`bytemuck`, `wgpu`) — not landed

## Preserved (workshop / tests)

- `docs/workshop/mapping_atlas_algebraic_mask_sandbox_code_preserve.rs`
- `docs/workshop/structured_field_stencil_atlas_mask_candidate.wgsl`
- `docs/workshop/mapping_atlas_algebraic_mask_candidate_notes.md`
- `docs/tests/mapping_atlas_algebraic_mask_sandbox_test_results.md`
- `docs/tests/mapping_atlas_algebraic_mask_sandbox_full.log`

## Regression ladder

| Command | Result |
|---|---|
| `cargo test -p simthing-spec --test region_field_spec_admission` | PASS (10) |
| `cargo test -p simthing-driver --test phase_m2_field_scheduler` | PASS (12) |
| `cargo test -p simthing-gpu --test structured_field_stencil` | PASS (16) |
| `cargo test -p simthing-driver --test structured_field_region_execution` | PASS (5) |
| `cargo test -p simthing-driver --test structured_field_stencil_parent_eml` | PASS (2) |
| `cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip` | PASS (2) |
| `cargo test -p simthing-driver --test e11b_nested_materialization_ron_session` | PASS (3) |
| `cargo test -p simthing-driver --test e11b_nested_materialization` | PASS (10) |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu` | PASS (12) |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap` | PASS (13) |
| `cargo check --workspace` | PASS |
| `cargo test --workspace` | PASS |

## Posture confirmed

- M-4 remains parked; no atlas packer landed
- No mapping runtime; no pass graph wiring
- StructuredFieldStencilOp production code unchanged
- simthing-sim map-free
- Resource Flow defaults unchanged
