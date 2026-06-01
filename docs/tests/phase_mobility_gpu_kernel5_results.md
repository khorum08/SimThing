# MOBILITY-GPU-KERNEL-5 Results

Date: 2026-06-02

Verdict: **PASS / second semantic-free mobility-shaped GPU kernel over the KERNEL-4 34k projection**

## Scope

MOBILITY-GPU-KERNEL-5 adds a second built-in, generic, semantic-free GPU kernel fixture in
`simthing-driver` test/support. It reuses the MOBILITY-GPU-KERNEL-4 34k composition-derived
projection as input, dispatches only through the existing registered-node path, and remains
explicitly opt-in/default-off.

The kernel consumes only generic columns (`src_parent`, `dst_parent`, `entity_id`, `move_mask`) and
produces deterministic row digest + move-weight outputs. The CPU oracle covers every row. GPU
execution reports either exact CPU/GPU parity with a checksum, or an honest unavailable
classification when no adapter is available; actual execution failures are separated from adapter
unavailability.

## Guardrails

- No designer-authored shader input.
- No semantic or raw WGSL intake.
- No owner/faction/species/economy/AI/map/resource/blockade/planner/urgency/commitment terms in WGSL.
- No default schedule, gameplay path, or default `SimSession` path.
- No live-slot compaction, GPU allocator, nondeterministic atomics, CPU planner, urgency, or commitment.
- No Hybrid-Strata/faction-index scaling and no v7.8 closed-ladder reopen.
- Disabled path is a no-op with zero projection or dispatch cost.

## Tests

```bash
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
