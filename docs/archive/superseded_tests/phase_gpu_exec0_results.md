# GPU-EXEC-0 — semantic-free GPU execution readiness results

Date: 2026-06-02

## Verdict

**PASS**

Implemented opt-in/default-off semantic-free GPU execution readiness in `simthing-driver`
test/support. Built-in identity-buffer pass (not mobility, not designer WGSL); reports CPU oracle
and GPU checksum with exact parity or honest `GpuUnavailable` classification. **RUNTIME-1B-DISPATCH**
remains closed.

## Files Touched

- `crates/simthing-driver/tests/support/gpu_exec0_fixture.rs`
- `crates/simthing-driver/tests/gpu_exec0_readiness_fixture.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_gpu_exec0_results.md`

## Implemented Readiness Scope

- Named gate `gpu_exec0_explicit_opt_in_gate`; default-off deterministic no-op when disabled.
- Generic semantic-free pass descriptor `gpu_exec0_identity_buffer_pass` with built-in identity-buffer WGSL.
- CPU oracle (identity copy) + `fnv64_hash_f32` checksum; optional wgpu compute dispatch + readback.
- Parity classification: `ExactParity` | `GpuUnavailable` | `GpuExecutionFailed`.
- Report fields attest: no mobility shader, no semantic/raw WGSL, no designer shader input, no default schedule, no gameplay/default `SimSession` path, RUNTIME-1B-DISPATCH closed.
- Reuses existing `simthing_gpu::GpuContext` and `fnv64_hash_f32`; no new GPU abstraction layer.

## GPU Execution Availability

Real GPU execution was **available** on the test host; identity pass achieved **ExactParity**
(CPU oracle checksum == GPU result checksum). Hosts without wgpu adapter classify `GpuUnavailable`
honestly with CPU oracle only.

## Tests Run

| Command | Result |
| --- | --- |
| `cargo test -p simthing-driver --test gpu_exec0_readiness_fixture` | 13 passed |
| `cargo test -p simthing-driver --test mobility_runtime1b_gpu_passgraph_fixture` | 21 passed |
| `cargo test -p simthing-driver --test mobility_runtime1a_runtime_fixture` | 21 passed |
| `cargo test -p simthing-spec --test mobility_runtime1_production_fixture` | 28 passed |
| `cargo test -p simthing-spec --test mobility_runtime0_composition` | 23 passed |
| `cargo check --workspace` | 0 errors |

## Posture Attestation

- No mobility shader, no mobility dispatch, no semantic/raw WGSL, no designer-authored shader input.
- No default schedule, no gameplay integration, no default `SimSession` lib path.
- RUNTIME-1B-DISPATCH remains closed (`mobility_runtime1b_dispatch_closed`).
- Hybrid-Strata/faction-index scaling, closed v7.8 ladders, and invariant posture unchanged.
