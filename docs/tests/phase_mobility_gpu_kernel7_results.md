# MOBILITY-GPU-KERNEL-7 Results

Date: 2026-06-02

Verdict: **PASS / deterministic multi-dispatch replay soak over the KERNEL-6 chain**

## Scope

MOBILITY-GPU-KERNEL-7 adds a driver test/support fixture for repeated explicit dispatch of the
existing MOBILITY-GPU-KERNEL-6 semantic-free chain over the same MOBILITY-GPU-KERNEL-4 34k
composition-derived projection.

The fixture stays explicitly opt-in/default-off, reuses KERNEL-6 and the registered-node path, and
adds no new shader text. The replay soak runs at least 8 dispatch iterations, emits a compact
per-iteration report with CPU oracle checksum, GPU checksum, projection checksum, dispatch state,
and parity classification, and verifies stable replay under source-row permutation.

## Files Touched

- `crates/simthing-driver/tests/support/mobility_gpu_kernel7_replay_fixture.rs`
- `crates/simthing-driver/tests/mobility_gpu_kernel7_replay_fixture.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_gpu_kernel7_results.md`

## Posture

- No default schedule, gameplay path, or default `SimSession` path.
- No designer-authored WGSL or semantic/raw WGSL intake.
- No new shader text; KERNEL-7 reuses the existing KERNEL-6 chain.
- No live-slot compaction, GPU allocator, nondeterministic atomics, CPU planner, urgency, or commitment.
- No Hybrid-Strata/faction-index scaling and no v7.8 closed-ladder reopen.
- Registration-only mode executes no replay iterations.
- Disabled path is a deterministic no-op with zero projection or dispatch cost.
- Repeated dispatch does not mutate the source projection.
- Local GPU execution ran and classified as `ExactParity`: 8 iterations, 34,000 rows, CPU checksum
  `14848092345613014426`, GPU checksum `14848092345613014426`. Fallback behavior honestly reports
  `GpuUnavailable` if no adapter is present.

## Tests

```bash
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
