# MOBILITY-RUNTIME-1B-DISPATCH-0 — semantic-free GPU-EXEC probe dispatch results

Date: 2026-06-02

## Verdict

**PASS**

Implemented opt-in/default-off test/support dispatch of the GPU-EXEC-0 semantic-free identity-buffer
probe through the green RUNTIME-1B registered pass-graph node. Registration remains non-executing
until dispatch is explicitly invoked. **Mobility GPU dispatch** remains closed.

## Files Touched

- `crates/simthing-driver/tests/support/mobility_runtime1b_dispatch_fixture.rs`
- `crates/simthing-driver/tests/mobility_runtime1b_dispatch_fixture.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_runtime1b_dispatch_results.md`

## Implemented Dispatch Scope

- Named gate `mobility_runtime1b_dispatch0_explicit_opt_in_gate`; default-off deterministic no-op when disabled.
- Requires RUNTIME-1B node registration before dispatch; routes GPU-EXEC-0 identity probe through `mobility_runtime1b_non_scheduled_composition_node`.
- Reuses existing GPU-EXEC-0 built-in pass (no new shader text); no mobility/faction/economy shader.
- CPU oracle + GPU checksum with `ExactParity` or honest `GpuUnavailable` classification.
- Registration-only mode: node registered, zero GPU dispatch.

## GPU Execution Availability

Real GPU execution was **available** on the test host; dispatched probe achieved **ExactParity**.
Hosts without wgpu adapter classify `GpuUnavailable` honestly.

## Tests Run

| Command | Result |
| --- | --- |
| `cargo test -p simthing-driver --test mobility_runtime1b_dispatch_fixture` | 15 passed |
| `cargo test -p simthing-driver --test gpu_exec0_readiness_fixture` | 13 passed |
| `cargo test -p simthing-driver --test mobility_runtime1b_gpu_passgraph_fixture` | 21 passed |
| `cargo test -p simthing-driver --test mobility_runtime1a_runtime_fixture` | 21 passed |
| `cargo test -p simthing-spec --test mobility_runtime1_production_fixture` | 28 passed |
| `cargo test -p simthing-spec --test mobility_runtime0_composition` | 23 passed |
| `cargo check --workspace` | 0 errors |

## Posture Attestation

- No mobility shader, no mobility composition GPU dispatch, no semantic/raw WGSL, no designer-authored shader input.
- No default schedule, no gameplay integration, no default `SimSession` lib path.
- Mobility GPU dispatch gate remains closed (`mobility_runtime1b_dispatch_closed`).
- Hybrid-Strata/faction-index scaling, closed v7.8 ladders, and invariant posture unchanged.
