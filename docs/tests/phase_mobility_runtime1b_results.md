# MOBILITY-RUNTIME-1B — non-scheduled GPU pass-graph node registration results

Date: 2026-06-02

## Verdict

**PASS**

Implemented opt-in/default-off non-scheduled GPU pass-graph node registration in `simthing-driver`
test/support, delegating to the green RUNTIME-1A CPU driver fixture. Registration only — no GPU
dispatch, no WGSL/shader, no default schedule, no gameplay path. **RUNTIME-1B-DISPATCH** remains closed.

## Files Touched

- `crates/simthing-driver/tests/support/mobility_runtime1b_fixture.rs`
- `crates/simthing-driver/tests/mobility_runtime1b_gpu_passgraph_fixture.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_runtime1b_results.md`

## Implemented Scope

- Named GPU pass-graph node (`mobility_runtime1b_non_scheduled_composition_node`) registered only when explicitly opted in.
- Non-scheduled registration: node exists in fixture registry with `scheduled: false`, `gpu_dispatch_enabled: false`, `wgsl_shader_present: false`.
- Delegates to `run_mobility_runtime1a_driver_fixture`; no duplicated substrate logic.
- Default `SimSession` lib path unchanged; confined to `tests/support`.

## Tests Run

| Command | Result |
| --- | --- |
| `cargo test -p simthing-driver --test mobility_runtime1b_gpu_passgraph_fixture` | 21 passed |
| `cargo test -p simthing-driver --test mobility_runtime1a_runtime_fixture` | 21 passed |
| `cargo test -p simthing-spec --test mobility_runtime1_production_fixture` | 28 passed |
| `cargo test -p simthing-spec --test mobility_runtime0_composition` | 23 passed |
| `cargo check --workspace` | 0 errors |

## Posture Attestation

- No GPU dispatch, no WGSL, no default schedule, no gameplay integration, no default production path.
- RUNTIME-1B-DISPATCH (real scheduled GPU dispatch) remains closed.
- Hybrid-Strata/faction-index scaling, closed v7.8 ladders, and invariant posture unchanged.
