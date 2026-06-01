# MOBILITY-GPU-KERNEL-3 — runtime composition to generic GPU column projection results

Date: 2026-06-02

## Verdict

**PASS**

Projects accepted RUNTIME-0 composition outputs into generic GPU column buffers and dispatches
through the green MOBILITY-GPU-KERNEL-1 registered-node path. Four fixture rows (1 moved, 3 unmoved);
CPU oracle complete; GPU checksum with exact parity or honest `GpuUnavailable`. No new shader text.

## Files Touched

- `crates/simthing-driver/tests/support/mobility_gpu_kernel3_projection_fixture.rs`
- `crates/simthing-driver/tests/mobility_gpu_kernel3_projection_fixture.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_gpu_kernel3_results.md`

## Projection Scope

- Named gate `mobility_gpu_kernel3_projection_explicit_opt_in_gate`; default-off no-op when disabled.
- RUNTIME-0/RUNTIME-1A/RUNTIME-1B composition fixture lineage (owner overlays + econ records present in composition input).
- Generic columns only: `entity_id`, `src_parent`, `dst_parent`, `move_mask` → KERNEL-0 `out_parent` / `out_changed`.
- Projection reads reenroll `final_live_slices` + `committed_moves` only; owner/econ reports not encoded into shader columns.
- Deterministic row order preserved from composition reenroll output; stable under input permutation (canonical sort).
- Delegates to KERNEL-1 → KERNEL-0 built-in WGSL; registration non-executing until explicit dispatch.

## GPU Execution Availability

Real GPU execution was **available** on the test host; projection dispatch achieved **ExactParity**.
Hosts without wgpu adapter classify `GpuUnavailable` honestly.

## Tests Run

| Command | Result |
| --- | --- |
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

- No new shader text, no designer-authored shader input, no semantic/raw WGSL intake.
- No default schedule, no gameplay integration, no default `SimSession` lib path.
- No live-slot compaction, GPU allocator, nondeterministic atomics, or CPU planner/urgency/commitment.
- Owner/economy composition context present in fixture but not encoded as shader semantics.
- Default production scheduling, Hybrid-Strata/faction-index scaling, and mobility scheduled dispatch remain closed.
