# Revert SEAD tensor/stencil WGSL sandbox to parked state — test results

**Date/time:** 2026-05-19  
**Base HEAD (before revert branch):** `cd99ff6` — SEAD tensor/stencil WGSL probe merge (PR #206)  
**Revert branch commit:** `c8a3328` (revert PR #206 merge)  
**Final commit SHA:** `a146462` (revert merge PR #207)  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell  
**GPU availability:** Local GPU present — E-11B nested hierarchy GPU parity 12/12 PASS.

---

## Commands

| Command | Result |
|---------|--------|
| `git status --short` | PASS — revert branch; preserve docs restored |
| `rustc --version` | PASS |
| `cargo --version` | PASS |
| `cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-driver --test e11b_nested_materialization_ron_session -- --nocapture` | **PASS** — 3/3 |
| `cargo test -p simthing-driver --test e11b_nested_materialization -- --nocapture` | **PASS** — 10/10 |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | **PASS** — 12/12 |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture` | **PASS** — 13/13 |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` | **PASS** |
| `cargo test -p simthing-driver --test sead_tensor_stencil_wgsl_sandbox` | **EXPECTED REMOVAL** — `no test target named sead_tensor_stencil_wgsl_sandbox` |

**Full log:** [`revert_sead_tensor_stencil_wgsl_sandbox_to_parked_state_full.log`](revert_sead_tensor_stencil_wgsl_sandbox_to_parked_state_full.log)

---

## Removals verified

- `crates/simthing-driver/tests/sead_tensor_stencil_wgsl_sandbox.rs` — **deleted**
- `crates/simthing-gpu/src/sead_tensor_stencil_prototype.rs` — **deleted**
- `crates/simthing-gpu/src/shaders/sead_tensor_stencil_prototype.wgsl` — **deleted**
- `simthing-gpu` lib exports for stencil prototype — **removed**

---

## Preserved (unchanged by revert)

- `docs/workshop/archive/sead/sead_tensor_stencil_wgsl_sandbox_code_preserve.rs`
- `docs/workshop/archive/sead/sead_tensor_stencil_prototype.wgsl`
- `docs/workshop/archive/sead/sead_tensor_stencil_prototype_notes.md`
- `docs/workshop/archive/sead/sead_tensor_stencil_*_prototype.wgsl` variant copies
- `docs/tests/sead_tensor_stencil_wgsl_sandbox_test_results.md`
- `docs/tests/archive/sead/sead_tensor_stencil_wgsl_sandbox_full.log`
- Prior SEAD probe preserved artifacts
- E-11B-1 materialization + RON smoke + GPU parity + fission/gap tests

---

## Posture restored

- SEAD tensor/stencil WGSL prototype sandbox reverted; decision-gate evidence preserved externally in docs.
- Mapping/location architecture remains provisional.
- No mapping runtime, Scatter/Gather, wavefront propagation, dynamic nested enrollment, D-2a, E-11B-5, production WGSL, new AccumulatorRole variants, CPU fallback, slot compaction, or simthing-sim arena awareness landed.
- FlatStarResourceFlow remains the accepted bounded production Resource Flow posture.
- Global `PipelineFlags::default().use_accumulator_resource_flow` remains false.
- Presence of `ResourceFlowSpec` alone does not enable GPU execution.

---

## Verdict

**PASS** — SEAD tensor/stencil WGSL sandbox production files removed; E-11B landed work intact; regressions green; preserved artifacts retained; repo returned to parked state.
