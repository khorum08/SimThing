# Revert SEAD tensor/stencil WGSL refinement sandbox to parked state — test results

**Date/time:** 2026-05-19  
**Base HEAD (before revert branch):** `be564a3` — SEAD tensor/stencil refinement probe merge (PR #208)  
**Revert branch commit:** `cebb8cb` (revert PR #208 merge)  
**Final commit SHA:** `b1f0934` (revert merge PR #209)  
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
| `cargo test -p simthing-driver --test sead_tensor_stencil_refinement_sandbox` | **EXPECTED REMOVAL** — `no test target named sead_tensor_stencil_refinement_sandbox` |

**Full log:** [`revert_sead_tensor_stencil_refinement_sandbox_to_parked_state_full.log`](revert_sead_tensor_stencil_refinement_sandbox_to_parked_state_full.log)

---

## Removals verified

- `crates/simthing-driver/tests/sead_tensor_stencil_refinement_sandbox.rs` — **deleted**
- `crates/simthing-gpu/src/sead_tensor_stencil_refinement_prototype.rs` — **deleted**
- `crates/simthing-gpu/src/shaders/sead_tensor_stencil_refinement_prototype.wgsl` — **deleted**
- `simthing-gpu` lib exports for refinement prototype — **removed**

---

## Preserved (unchanged by revert)

- `docs/workshop/sead_tensor_stencil_refinement_sandbox_code_preserve.rs`
- `docs/workshop/sead_tensor_stencil_refinement_prototype.wgsl`
- `docs/workshop/sead_tensor_stencil_refinement_notes.md`
- `docs/workshop/sead_tensor_stencil_*_refinement.wgsl` variant copies
- `docs/tests/sead_tensor_stencil_refinement_sandbox_test_results.md`
- `docs/tests/sead_tensor_stencil_refinement_sandbox_full.log`
- Prior SEAD probe preserved artifacts
- E-11B-1 materialization + RON smoke + GPU parity + fission/gap tests

---

## Posture restored

- SEAD tensor/stencil WGSL refinement sandbox completed and was reverted to parked state.
- The sandbox source, WGSL prototype(s), notes, and decision-gate results are preserved in docs/workshop and docs/tests.
- No sandbox test/prototype remains in the production runtime/test suite.
- Mapping/location architecture remains provisional.
- Implementation remains parked until the mapping doc is ready or product names a concrete non-mapping scenario.
- No mapping runtime, Scatter/Gather, wavefront propagation, dynamic nested enrollment, D-2a, E-11B-5, production WGSL, new AccumulatorRole variants, CPU fallback, slot compaction, or simthing-sim arena awareness landed.
- FlatStarResourceFlow remains the accepted bounded production Resource Flow posture.
- Global `PipelineFlags::default().use_accumulator_resource_flow` remains false.
- Presence of `ResourceFlowSpec` alone does not enable GPU execution.

---

**PASS** — SEAD tensor/stencil refinement sandbox production files removed; E-11B landed work intact; regressions green; preserved artifacts retained; repo returned to parked state.
