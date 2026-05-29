# Phase M FirstSliceScenarioSpec-R1 Hygiene Test Results

Date: 2026-05-28

## Base

- Base HEAD: `1f56dd8eb9e11dd0ae8af014b88541ee0a79ab2e` (FirstSliceScenarioSpec merge)
- Final commit SHA: not known at report authoring; branch commit/merge records the final SHA.

## Files Changed

- `crates/simthing-spec/src/compile/first_slice_scenario_admission.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/src/first_slice_scenario_fixture.rs` (removed)
- `crates/simthing-driver/tests/support/first_slice_scenario_fixture.rs` (added)
- `crates/simthing-driver/tests/support/mod.rs`
- `crates/simthing-driver/tests/phase_m_first_slice_scenario_spec.rs`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/todo.md`
- `docs/worklog.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/tests/phase_m_first_slice_scenario_spec_r1_hygiene_test_results.md`

## What Was Cleaned

1. **Public/test-only boundary (Path A):** Removed `FirstSliceScenarioFixtureSession` from production `simthing-driver/src` and `lib.rs` exports. Moved equivalent helper to `crates/simthing-driver/tests/support/first_slice_scenario_fixture.rs`. Production retains narrow `FirstSliceMappingSession::open_from_scenario_preview` only.
2. **Budget estimate handling:** `compile_first_slice_scenario_preview` now propagates `estimate_region_field_budget` failures as `SpecError::RegionFieldAdmission` instead of silently dropping with `.ok()`. Successful previews always retain `budget_estimate_bytes: Some(...)`.
3. **Tests:** Added `scenario_production_test_boundary`; strengthened admission/budget assertions; preserved all prior scenario GPU-resident behavior tests.

## Helper Disposition

**Path A — test-local.** `FirstSliceScenarioFixtureSession` is integration-test support only, not a public production API.

## Budget-Estimate Handling Change

Before:

```rust
estimate_region_field_budget(...).ok().map(|b| b.estimated_bytes)
```

After:

```rust
let budget = estimate_region_field_budget(...).map_err(|err| SpecError::RegionFieldAdmission { ... })?;
budget_estimate_bytes: Some(budget.estimated_bytes)
```

Over-budget scenarios still reject deterministically via nested `RegionFieldSpec` admission (`VRAM budget exceeded`). Scenario-level compile also rejects over-budget specs. Successful admits no longer lose budget estimate silently.

## Crash / Build-Run Addendum

Documented from the original FirstSliceScenarioSpec landing session (PR #238):

| Event | Class | Rust test failure? | Notes |
|---|---|---|---|
| Initial `cargo test -p simthing-driver --test phase_m_first_slice_scenario_spec` | Agent/tool interruption | No | User interrupted after ~508s; no failing assertion observed in partial output |
| `gh pr merge 238` (first attempt) | Agent/tool interruption | No | Interrupted before merge completed; repo state unchanged |
| Compile error: wrong `RegionFieldBudgetSpec` field name | Compile failure (fixed pre-test) | N/A | Fixed before clean test run |
| Compile error: `diagnostic_readback_reduction_eml` needed `&mut self` | Compile failure (fixed pre-test) | N/A | Fixed before clean test run |
| Final scenario spec test run | Clean pass | No | 8/8 |
| Final subset + `cargo check --workspace` | Clean pass | No | All required targets green |
| Final `cargo test --workspace -j 1` | Clean pass | No | Exit code 0 (~403s) |
| PR #238 merge (recovery run) | Clean pass | No | Merged to `1f56dd8` |

No GPU/device loss, OOM, or cargo artifact format errors were observed in the final clean runs. No Rust test assertion failures were observed for FirstSliceScenarioSpec work.

## Commands Run (R1)

| Command | Result |
|---|---|
| `git rev-parse HEAD` | PASS; `1f56dd8eb9e11dd0ae8af014b88541ee0a79ab2e` |
| `rustc --version` | PASS; `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | PASS; `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `cargo test -p simthing-driver --test phase_m_first_slice_scenario_spec -- --nocapture` | PASS; 9/9 |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_commitment_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture` | PASS; 28/28 |
| `cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture` | PASS; 11/11 |
| `cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge -- --nocapture` | PASS; 2/2 |
| `cargo check --workspace` | PASS |
| `cargo test --workspace -j 1` | PASS |

## Posture Summary

Phase M FirstSliceScenarioSpec-R1 hygiene landed.
The scenario-level RON wrapper remains opt-in and GPU-resident. The public/test-only boundary was clarified, scenario budget estimate handling was hardened, and the prior build/test crash history was documented with a final clean verification run.
No default SimSession wiring was introduced.
No CPU-side AI planner was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

## Known Caveat

First-slice bridge uses queue writes for child resource values and parent weights. This is acceptable for the 10x10 first-slice scenario fixture. Future multi-field/atlas scale must replace per-slot resource writes with a generic preinitialized resource column, fill helper, or GPU fill kernel after a separate measured design step.

## Final Verdict

PASS — Phase M FirstSliceScenarioSpec-R1 hygiene landed; the scenario-level RON wrapper remains opt-in and GPU-resident, the public/test-only boundary was clarified, scenario budget estimate handling was hardened, and prior build/test crash history was documented with a final clean verification run, without introducing default SimSession wiring, atlas, semantic WGSL, source_mask, perception, map residency, or CPU-side AI planning.
