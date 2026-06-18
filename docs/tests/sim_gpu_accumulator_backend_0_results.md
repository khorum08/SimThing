# SIM-GPU-ACCUMULATOR-BACKEND-0 — Sim-owned GPU backend for AccumulatorOp plans

> **Lifecycle: PROBATION** — sim-owned GPU backend landed; CPU/GPU tick parity proven; real-adapter GPU evidence observed. Pending owner DA approval.

**Date:** 2026-06-18  
**PR:** SIM-GPU-ACCUMULATOR-BACKEND-0  
**Base:** `master` after PR #760 / ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1

## Artifact lifecycle audit

| Artifact | Regime | Action |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated with BACKEND-0 row |
| `docs/tests/accumulator_driver_sim_convergence_1_results.md` | PROBATION | Preserved — CPU/GPU AO source evidence |
| `docs/tests/sim_gpu_accumulator_backend_0_results.md` | PROBATION | Created (this file) |
| `docs/0.8.3 Simthing Studio Production.md` | Living synthesis | Updated § SIM-GPU-ACCUMULATOR-BACKEND-0 |

No scratch logs or contradictory evidence introduced.

## Why this is not hygiene

PR #760 proved canonical AccumulatorOp Sum-over-INPUT_LIST with a CPU sim seam and separate GPU AO tests. This PR closes the execution-ownership gap:

```text
driver compiles
sim owns tick/backend choice
gpu executes semantic-free AccumulatorOp
Studio only loads/projects/presents
```

## Orientation answers

| Question | Answer |
|---|---|
| What executes under simthing-sim today? | `execute_accumulator_plan_tick_cpu` (CPU oracle); now also `execute_accumulator_plan_tick_gpu` |
| What executes in simthing-gpu tests? | Direct `AccumulatorOpSession` proofs — regression preserved |
| Sim GPU API | `execute_accumulator_plan_tick_gpu(ctx, plan, inputs)` + `AccumulatorTickBackend` enum |
| Reuses AccumulatorOpSession / AO-WGSL-0? | Yes — `new`, `upload_values`, `upload_ops_resolving_input_lists`, `tick`, `readback_full` |
| Exact f32 validation | Shared `validate_accumulator_plan_inputs` via `is_exact_integer_f32` |
| GPU adapter absence | `gpu_context_blocking()` → `SimTickError::GpuUnavailable`; tests skip with `GPU_TESTS_SKIPPED_NO_ADAPTER` |
| Studio bypass | App/load sources scanned; forbidden `AccumulatorOpSession::new`, sim GPU tick calls |
| Gu-Yang/PALMA | Deferred per STEAD §10 — not implemented |

## CPU sim seam summary

`execute_accumulator_plan_tick_cpu` preserved: seeds input channel, zeros output channel, runs `execute_ops_cpu`, returns output channel scalars.

## GPU sim backend summary

`execute_accumulator_plan_tick_gpu`:

1. Validates input length and exact-integer f32 bound
2. Seeds value grid on input channel
3. Creates `AccumulatorOpSession::new(ctx, slot_count, n_dims)`
4. `upload_values` → `upload_ops_resolving_input_lists` → `tick(band=0)` → `readback_full`
5. Extracts output channel as projection/cache (not scenario authority)

`execute_accumulator_plan_tick_with_backend` dispatches CPU or GPU.

## AccumulatorOpSession reuse proof

No new WGSL shader. GPU backend calls existing AO-WGSL-0 path through `AccumulatorOpSession`.

## Driver compile proof

Existing driver tests unchanged and passing:

- `driver_compiles_vertical_seed_links_to_input_list_plan`
- `driver_compiled_plan_uses_accumulator_op_sum_input_list`
- `driver_compile_rejects_invalid_scenario_links`
- `driver_compile_does_not_use_studio_or_bevy_state`

## CPU vs GPU tick parity proof

| Test | Result |
|---|---|
| `sim_gpu_tick_vertical_seed_outputs_20_10` | PASS |
| `sim_gpu_tick_matches_cpu_tick_for_vertical_seed` | PASS |
| Vertical seed output | `[20.0, 10.0]` from `[10.0, 20.0]` |

GPU evidence: **REAL_ADAPTER_OBSERVED**

## Studio bypass guard

| Test | Result |
|---|---|
| `studio_app_sources_do_not_construct_accumulator_op_session` | PASS |
| `studio_load_path_does_not_execute_sim_gpu_tick` | PASS |
| `studio_load_path_does_not_execute_accumulator_runtime` | PASS |
| `studio_remains_projection_and_proof_harness` | PASS |

## Gu-Yang / PALMA contract carried forward

Deferred:

- **Gu-Yang:** grid N4 → `saturating_flux_choke_threshold` + `structured_field_stencil`
- **PALMA:** grid N4 → `min_plus_stencil` + `w_impedance_compose`

## Big-endian / portable byte-proof backlog

Deferred: explicit little-endian helpers, cross-platform byte-order evidence.

## Forbidden-token scan

New sim execution identifiers scanned — no forbidden semantic tokens in `accumulator_plan_tick.rs`.

## Tests added

**simthing-sim (8 new GPU tests):** GPU vertical seed execution, `[20,10]` output, CPU/GPU parity, validation errors, adapter absence handling, no Studio/Bevy/bespoke kernel.

**simthing-mapeditor (2 new guards):** Studio app GPU dispatch fence, load path sim GPU tick fence.

## Commands run

> **Evidence correction (SIM-GPU-RESIDENT-ACCUMULATOR-TICK-0):** This report originally recorded a partial validation sweep. The full required command set was run and recorded in `docs/tests/sim_gpu_resident_accumulator_tick_0_results.md`.

Partial sweep recorded at BACKEND-0 landing:

```text
cargo fmt --all -- --check
cargo check -p simthing-sim
cargo test -p simthing-sim
cargo test -p simthing-driver --test structural_link_accumulator_compile
cargo test -p simthing-gpu --test accumulator_op_sum_input_list_convergence
cargo test -p simthing-mapeditor --test accumulator_convergence_1_guards
cargo test -p simthing-clausething --test stead_spatial_contract_guards
git diff --check
```

## Windows / resource-limit notes

No paging-file or linker failures observed.

## Files changed

- `crates/simthing-sim/src/accumulator_plan_tick.rs` — GPU backend, backend enum, extended errors
- `crates/simthing-sim/src/lib.rs` — exports
- `crates/simthing-sim/tests/accumulator_plan_tick_convergence.rs` — GPU proofs
- `crates/simthing-mapeditor/tests/accumulator_convergence_1_guards.rs` — Studio GPU dispatch guards
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/sim_gpu_accumulator_backend_0_results.md`

## Deferred work

Full runtime terran-pirate play-out, Gu-Yang/PALMA surfaces, big-endian byte proofs, RF/MF execution, heatmap rendering.

## DA status

**PROBATION** — sim-owned GPU backend complete; real-adapter evidence observed; awaiting owner sign-off.