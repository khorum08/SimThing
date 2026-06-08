# MOBILITY-GPU-KERNEL-8 Results

Date: 2026-06-02

Verdict: **PASS / deterministic varied-input projection-batch replay soak over the KERNEL-6 chain**

## Scope

MOBILITY-GPU-KERNEL-8 adds a driver test/support fixture that exercises four deterministic
generic-column projection variants through the existing KERNEL-6 semantic-free chain
(KERNEL-0 → KERNEL-5). Each variant runs at least two explicit dispatch repetitions with
per-variant CPU oracle, GPU checksum, projection checksum, and parity classification.

Variants: baseline 34k projection, sparse-delta move-mask toggles, dense-bulk move-mask cluster,
and parent-key offset. No new shader text; registration-only and disabled paths remain zero-cost.

## Files Touched

- `crates/simthing-driver/tests/support/mobility_gpu_kernel8_variant_batch_fixture.rs`
- `crates/simthing-driver/tests/mobility_gpu_kernel8_variant_batch_fixture.rs`
- `crates/simthing-driver/tests/support/mobility_gpu_kernel6_chain_fixture.rs` (optional `columns_override`)
- `crates/simthing-driver/tests/support/mobility_gpu_kernel5_second_kernel_fixture.rs` (optional `columns_override`)
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_gpu_kernel8_results.md`

## Variant Batch

| Variant | Pattern | Row count |
| --- | --- | --- |
| `baseline_34k_projection` | Unmodified KERNEL-4/KERNEL-6 34k projection | 34,000 |
| `sparse_delta_move_mask` | Toggle move_mask every 1000th row | 34,000 |
| `dense_bulk_move_mask` | Set move_mask on rows 10,000–10,049 | 34,000 |
| `parent_key_offset` | Wrapping +3/+7 on src/dst parent columns | 34,000 |

Replay repetitions per variant: **2**. Distinct projection and chain checksums verified across variants.

## GPU Execution

Local GPU execution ran and classified as **ExactParity** for all variants and replays.
Hosts without a wgpu adapter classify **GpuUnavailable** honestly.

## Tests

```bash
cargo test -p simthing-driver --test mobility_gpu_kernel8_variant_batch_fixture  # 27 passed
cargo test -p simthing-driver --test mobility_gpu_kernel7_replay_fixture
cargo test -p simthing-driver --test mobility_gpu_kernel6_chain_fixture
cargo test -p simthing-driver --test mobility_gpu_kernel5_second_kernel_fixture
cargo test -p simthing-driver --test mobility_gpu_kernel4_34k_projection_fixture
cargo test -p simthing-driver --test mobility_gpu_kernel3_projection_fixture
cargo test -p simthing-driver --test mobility_gpu_kernel2_34k_fixture
cargo test -p simthing-driver --test mobility_gpu_kernel1_dispatch_fixture
cargo test -p simthing-driver --test mobility_gpu_kernel0_fixture
cargo test -p simthing-driver --test mobility_runtime1b_dispatch_fixture
cargo test -p simthing-driver --test gpu_exec0_readiness_fixture
cargo test -p simthing-driver --test mobility_runtime1b_gpu_passgraph_fixture
cargo test -p simthing-driver --test mobility_runtime1a_runtime_fixture
cargo test -p simthing-spec --test mobility_runtime1_production_fixture
cargo test -p simthing-spec --test mobility_runtime0_composition
cargo check --workspace
```

## Posture Attestation

- No default schedule, gameplay path, or default `SimSession` path.
- No designer-authored WGSL or semantic/raw WGSL intake.
- No new shader text; KERNEL-8 reuses KERNEL-6 chain shaders.
- No live-slot compaction, GPU allocator, nondeterministic atomics, CPU planner/urgency/commitment.
- No Hybrid-Strata/faction-index scaling and no v7.8 closed-ladder reopen.
- Source baseline projection is not mutated by variant construction or dispatch.
- Registration-only mode executes no variant dispatches.
- Fast-lane under "semantic-free + default-off + parity-backed = ship it."
