# MOBILITY-GPU-KERNEL-4 - 34k composition-derived GPU column projection results

Date: 2026-06-02

## Verdict

**PASS**

Implemented a deterministic 34,000-row RUNTIME composition-derived projection soak in
`simthing-driver` test/support. The fixture generates accepted RUNTIME-0 composition input,
projects the composition output into generic GPU columns, and dispatches through the green
MOBILITY-GPU-KERNEL-3 -> KERNEL-1 -> KERNEL-0 registered-node path.

## Files Touched

- `crates/simthing-driver/tests/support/mobility_gpu_kernel4_34k_projection_fixture.rs`
- `crates/simthing-driver/tests/mobility_gpu_kernel4_34k_projection_fixture.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_gpu_kernel4_results.md`

## 34k Projection Scope

- Named gate `mobility_gpu_kernel4_34k_projection_explicit_opt_in_gate`; default-off no-op when disabled.
- 34,000 final composition rows, including moved and unmoved entities.
- Flat-star/spatial-movement first slice only; no nested/capture semantics.
- Sparse move masks, dense move clusters, first/last edge rows, and repeated destination parents.
- Generic columns only: `entity_id`, `src_parent`, `dst_parent`, `move_mask`, `out_parent`, `out_changed`.
- Owner/econ records are present as composition context, but no owner/econ/gameplay terms become shader semantics.
- Deterministic row order preserved from accepted composition output; stable under input permutation.

## GPU Execution Availability

Local execution classified as **ExactParity**. Hosts without a wgpu adapter classify
`GpuUnavailable` honestly; `GpuExecutionFailed` is reserved for real execution failure with
diagnostics.

## Tests Run

| Command | Result |
| --- | --- |
| `cargo test -p simthing-driver --test mobility_gpu_kernel4_34k_projection_fixture` | 27 passed |
| `cargo test -p simthing-driver --test mobility_gpu_kernel3_projection_fixture` | 23 passed |
| `cargo test -p simthing-driver --test mobility_gpu_kernel2_34k_fixture` | 21 passed |
| `cargo test -p simthing-driver --test mobility_gpu_kernel1_dispatch_fixture` | 18 passed |
| `cargo test -p simthing-driver --test mobility_gpu_kernel0_fixture` | 16 passed |
| `cargo test -p simthing-driver --test mobility_runtime1b_dispatch_fixture` | 15 passed |
| `cargo test -p simthing-driver --test gpu_exec0_readiness_fixture` | 13 passed |
| `cargo test -p simthing-driver --test mobility_runtime1b_gpu_passgraph_fixture` | 23 passed |
| `cargo test -p simthing-driver --test mobility_runtime1a_runtime_fixture` | 21 passed |
| `cargo test -p simthing-spec --test mobility_runtime1_production_fixture` | 28 passed |
| `cargo test -p simthing-spec --test mobility_runtime0_composition` | 23 passed |
| `cargo check --workspace` | 0 errors |

## Posture Attestation

- No default schedule, gameplay path, or default `SimSession` path added.
- No designer-authored WGSL, semantic/raw WGSL intake, or new shader text added.
- No live-slot compaction, GPU allocator, nondeterministic atomics, or CPU planner/urgency/commitment added.
- Registration remains non-executing until explicit dispatch.
- Disabled path remains deterministic no-op with zero generated projection rows.
- Hybrid-Strata/faction-index scaling and closed v7.8 ladders remain closed/parked.
- `docs/invariants.md` was not edited.
