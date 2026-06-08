# V7.6 StructuredFieldStencilOp Guardrail Hardening — Test Results

| Field | Value |
|---|---|
| Date/time | 2026-05-19 (local verification run) |
| Base HEAD | `932d65f7b596474f045987f69939520e58b4d427` |
| Final commit SHA | `adccd5f` |
| rustc | 1.95.0 (59807616e 2026-04-14) |
| cargo | 1.95.0 (f2d3ce0bd 2026-03-21) |
| Platform | Windows 10 (win32 10.0.26200) |
| GPU | Available (WGSL stencil tests executed) |

---

## Fixes applied

1. **Execution horizon enforcement** — `run_ping_pong` / `dispatch_ping_pong` return `ExecutionHorizonExceedsConfig` when `steps > config.horizon`; added `run_configured_horizon`.
2. **Source policy clarity** — renamed to `CallerManagedOneShotSeedThenZero`; test proves primitive does not auto-zero sources.
3. **Source-cap test indexing** — `test_d` uses `idx(slot, source_col, n_dims)` with source-column assertions.
4. **Clamp boundary parity** — CPU oracle implements `BoundaryMode::Clamp`; GPU/CPU parity test added.
5. **Active mask provisional** — renamed to `ActiveOnlyExperimentalNoHalo` with explicit test.
6. **Inertness** — strengthened string guards for `passes.rs`, `simthing-sim`, `simthing-driver` session paths.

---

## Commands run

| Command | Result |
|---|---|
| `cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture` | **PASS** (11/11) |
| `cargo test -p simthing-driver --test structured_field_stencil_parent_eml -- --nocapture` | **PASS** (2/2) |
| `cargo test -p simthing-spec --test eml_field_formula_admission -- --nocapture` | **PASS** (2/2) |
| `cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip -- --nocapture` | **PASS** |
| `cargo test -p simthing-driver --test e11b_nested_materialization_ron_session -- --nocapture` | **PASS** |
| `cargo test -p simthing-driver --test e11b_nested_materialization -- --nocapture` | **PASS** |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | **PASS** |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture` | **PASS** |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` (`CARGO_BUILD_JOBS=1`) | **PASS** |

---

## V7.6 Hardening Decision Summary

| Area | PASS / PARTIAL / FAIL | Evidence |
|---|---|---|
| Execution horizon enforcement | PASS | `ExecutionHorizonExceedsConfig`; `run_configured_horizon` |
| Source policy no longer misleading | PASS | `CallerManagedOneShotSeedThenZero` + caller-managed test |
| Source-cap test indexing | PASS | Correct `idx()` usage + column assertions |
| Clamp boundary parity | PASS | CPU oracle clamp + GPU/CPU test |
| Active mask provisional | PASS | `ActiveOnlyExperimentalNoHalo` |
| Inert by default | PASS | passes/sim/driver/simthing-sim string guards |
| E-11B regressions | PASS | Five regression targets green |
| Workspace check/test | PASS | `cargo check/test --workspace` |

## Final verdict

**PASS**

## Notes

- Remedial hardening only; V7.6 promotion and primitive remain in place.
- No mapping runtime; no production pass graph wiring.
- Resource Flow defaults unchanged; simthing-sim remains semantic-free.
