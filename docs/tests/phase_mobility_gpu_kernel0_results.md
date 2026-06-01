# MOBILITY-GPU-KERNEL-0 — semantic-free mobility column-transform kernel results

Date: 2026-06-02

## Verdict

**PASS**

Implemented opt-in/default-off semantic-free mobility-shaped GPU column-transform kernel in
`simthing-driver` test/support. Built-in parent-select kernel over generic columns
(`src_parent`, `dst_parent`, `entity_id`, `move_mask` → `out_parent`, `out_changed`); CPU oracle
and GPU checksum with exact parity or honest `GpuUnavailable`. No default schedule, gameplay path,
or designer WGSL.

## Files Touched

- `crates/simthing-driver/tests/support/mobility_gpu_kernel0_fixture.rs`
- `crates/simthing-driver/tests/mobility_gpu_kernel0_fixture.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_gpu_kernel0_results.md`

## Implemented Kernel Scope

- Named gate `mobility_gpu_kernel0_explicit_opt_in_gate`; default-off deterministic no-op when disabled.
- Built-in kernel `mobility_gpu_kernel0_column_parent_select`: if `move_mask[i] != 0` write `dst_parent[i]` else `src_parent[i]` to `out_parent[i]`; set `out_changed[i]`.
- CPU oracle + `fnv64_hash_f32` checksum; wgpu compute dispatch + readback.
- No live-slot compaction, no GPU allocator, no nondeterministic atomics, no CPU planner/urgency/commitment.

## GPU Execution Availability

Real GPU execution was **available** on the test host; column kernel achieved **ExactParity**.
Hosts without wgpu adapter classify `GpuUnavailable` honestly with CPU oracle only.

## Tests Run

| Command | Result |
| --- | --- |
| `cargo test -p simthing-driver --test mobility_gpu_kernel0_fixture` | 16 passed |
| `cargo test -p simthing-driver --test mobility_runtime1b_dispatch_fixture` | 15 passed |
| `cargo test -p simthing-driver --test gpu_exec0_readiness_fixture` | 13 passed |
| `cargo test -p simthing-driver --test mobility_runtime1b_gpu_passgraph_fixture` | 23 passed |
| `cargo test -p simthing-driver --test mobility_runtime1a_runtime_fixture` | 21 passed |
| `cargo test -p simthing-spec --test mobility_runtime1_production_fixture` | 28 passed |
| `cargo test -p simthing-spec --test mobility_runtime0_composition` | 23 passed |
| `cargo check --workspace` | 0 errors |

## Posture Attestation

- No designer-authored shader input, no semantic/raw WGSL intake, no faction/economy/gameplay semantics in shader.
- No default schedule, no gameplay integration, no default `SimSession` lib path.
- Default production scheduling, mobility composition GPU dispatch (RUNTIME-1B-DISPATCH), and Hybrid-Strata/faction-index scaling remain closed.
- Hybrid-Strata/faction-index scaling, closed v7.8 ladders, and invariant posture unchanged.
