# MOBILITY-GPU-KERNEL-1 — registered-node dispatch of column-transform kernel results

Date: 2026-06-02

## Verdict

**PASS**

Implemented opt-in/default-off test/support dispatch of the MOBILITY-GPU-KERNEL-0 semantic-free
column-transform kernel through the green RUNTIME-1B registered pass-graph node. Registration
remains non-executing until dispatch is explicitly invoked. No default schedule, gameplay path,
or designer WGSL.

## Files Touched

- `crates/simthing-driver/tests/support/mobility_gpu_kernel1_dispatch_fixture.rs`
- `crates/simthing-driver/tests/mobility_gpu_kernel1_dispatch_fixture.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_gpu_kernel1_results.md`

## Implemented Dispatch Scope

- Named gate `mobility_gpu_kernel1_explicit_opt_in_gate`; default-off deterministic no-op when disabled.
- Requires RUNTIME-1B node registration before dispatch; routes MOBILITY-GPU-KERNEL-0 through `mobility_runtime1b_non_scheduled_composition_node`.
- Reuses existing KERNEL-0 built-in WGSL (no new shader text); delegates to `run_mobility_gpu_kernel0_fixture`.
- CPU oracle preserved from KERNEL-0; GPU checksum with `ExactParity` or honest `GpuUnavailable`.

## GPU Execution Availability

Real GPU execution was **available** on the test host; dispatched kernel achieved **ExactParity**.
Hosts without wgpu adapter classify `GpuUnavailable` honestly.

## Tests Run

| Command | Result |
| --- | --- |
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

- No designer-authored shader input, no semantic/raw WGSL intake, no new shader text.
- No default schedule, no gameplay integration, no default `SimSession` lib path.
- Default production scheduling, Hybrid-Strata/faction-index scaling, and mobility scheduled dispatch (RUNTIME-1B-DISPATCH) remain closed.
- Hybrid-Strata/faction-index scaling, closed v7.8 ladders, and invariant posture unchanged.
