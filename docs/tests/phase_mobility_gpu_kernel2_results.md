# MOBILITY-GPU-KERNEL-2 — 34k registered-node column dispatch soak results

Date: 2026-06-02

## Verdict

**PASS**

Implemented deterministic 34k-row column soak through the green MOBILITY-GPU-KERNEL-1 registered-node
dispatch path. Edge rows, sparse (every 1000th), and dense (rows 10000–10049) move-mask clusters;
CPU oracle complete; GPU checksum with exact parity or honest `GpuUnavailable`. No new shader text.

## Files Touched

- `crates/simthing-driver/tests/support/mobility_gpu_kernel2_34k_fixture.rs`
- `crates/simthing-driver/tests/mobility_gpu_kernel2_34k_fixture.rs`
- `crates/simthing-driver/tests/support/mobility_gpu_kernel1_dispatch_fixture.rs` (re-export `MobilityGpuKernel0Gate`)
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_gpu_kernel2_results.md`

## Implemented 34k Scope

- Named gate `mobility_gpu_kernel2_34k_explicit_opt_in_gate`; default-off no-op when disabled.
- Deterministic 34,000-row columns via `generate_34k_column_probe()`; delegates to KERNEL-1 → KERNEL-0 built-in WGSL.
- Varied move_mask: edge rows, sparse stride-1000, dense cluster 10000–10049, alternating band 20000–20099.
- Registration non-executing until explicit dispatch; no 34k allocation on disabled path.

## GPU Execution Availability

Real GPU execution was **available** on the test host; 34k soak achieved **ExactParity**.
Hosts without wgpu adapter classify `GpuUnavailable` honestly.

## Tests Run

| Command | Result |
| --- | --- |
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
- Default production scheduling, Hybrid-Strata/faction-index scaling, and mobility scheduled dispatch remain closed.
