# MOBILITY-GPU-KERNEL-6 Results

Date: 2026-06-02

Verdict: **PASS / ordered semantic-free KERNEL-0 -> KERNEL-5 chain over the 34k projection**

## Scope

MOBILITY-GPU-KERNEL-6 adds a driver test/support fixture for an ordered two-kernel chain over the
MOBILITY-GPU-KERNEL-4 34k composition-derived generic column projection:

1. MOBILITY-GPU-KERNEL-0 parent-select column transform.
2. MOBILITY-GPU-KERNEL-5 row-digest / move-weight transform.

The fixture stays explicitly opt-in/default-off, reuses the existing registered-node path and
built-in semantic-free shaders, and adds no new shader text. The whole-chain CPU oracle covers
KERNEL-0 outputs (`out_parent`, `out_changed`) and KERNEL-5 outputs (`out_digest`, `out_weight`).
GPU results are classified as `ExactParity`, honest `GpuUnavailable`, or `GpuExecutionFailed`.

## Files Touched

- `crates/simthing-driver/tests/support/mobility_gpu_kernel6_chain_fixture.rs`
- `crates/simthing-driver/tests/mobility_gpu_kernel6_chain_fixture.rs`
- `crates/simthing-driver/tests/support/mobility_gpu_kernel1_dispatch_fixture.rs`
- `crates/simthing-driver/tests/support/mobility_gpu_kernel3_projection_fixture.rs`
- `crates/simthing-driver/tests/support/mobility_gpu_kernel4_34k_projection_fixture.rs`
- `crates/simthing-driver/tests/support/mobility_gpu_kernel5_second_kernel_fixture.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_gpu_kernel6_results.md`

## Posture

- No default schedule, gameplay path, or default `SimSession` path.
- No designer-authored WGSL or semantic/raw WGSL intake.
- No live-slot compaction, GPU allocator, nondeterministic atomics, CPU planner, urgency, or commitment.
- No Hybrid-Strata/faction-index scaling and no v7.8 closed-ladder reopen.
- Disabled path is a deterministic no-op with zero projection or dispatch cost.
- Local GPU execution ran and classified as `ExactParity`; fallback behavior honestly reports
  `GpuUnavailable` if no adapter is present. The required local battery completed without
  `GpuExecutionFailed`.

## Tests

```bash
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
