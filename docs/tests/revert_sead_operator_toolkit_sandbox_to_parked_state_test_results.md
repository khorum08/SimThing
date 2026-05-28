# Revert SEAD operator toolkit sandbox to parked state — test results

**Date/time:** 2026-05-19  
**Base HEAD (before revert branch):** `ce44b92` — SEAD operator toolkit probe merge (PR #204)  
**Revert branch commits:** `91c1afc` (merge SHA record) + `3c7dcf2` (revert PR #204 merge)  
**Final commit SHA:** `3c7dcf2` _(updated after revert merge PR)_  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell  
**GPU availability:** Local GPU present — E-11B nested hierarchy GPU parity 12/12 PASS.

---

## Commands

| Command | Result |
|---------|--------|
| `git status --short` | PASS — revert branch; preserve docs restored |
| `git rev-parse HEAD` | PASS — see Final commit SHA above |
| `rustc --version` | PASS |
| `cargo --version` | PASS |
| `cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-driver --test e11b_nested_materialization_ron_session -- --nocapture` | **PASS** — 3/3 |
| `cargo test -p simthing-driver --test e11b_nested_materialization -- --nocapture` | **PASS** — 10/10 |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | **PASS** — 12/12 |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture` | **PASS** — 13/13 |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` | **PASS** |
| `cargo test -p simthing-driver --test sead_operator_toolkit_sandbox` | **EXPECTED REMOVAL** — `no test target named sead_operator_toolkit_sandbox` |

**Full log:** [`revert_sead_operator_toolkit_sandbox_to_parked_state_full.log`](revert_sead_operator_toolkit_sandbox_to_parked_state_full.log)

---

## Removals verified

- `crates/simthing-driver/tests/sead_operator_toolkit_sandbox.rs` — **deleted**

---

## Preserved (unchanged by revert)

- `docs/workshop/sead_operator_toolkit_sandbox_code_preserve.rs`
- `docs/tests/sead_operator_toolkit_sandbox_test_results.md`
- `docs/tests/sead_operator_toolkit_sandbox_full.log`
- Prior SEAD probe preserved artifacts (field-intelligence, strategic horizon)
- E-11B-1 materialization + RON smoke + GPU parity + fission/gap tests

---

## Posture restored

- SEAD operator toolkit sandbox reverted; decision-gate evidence preserved externally in docs.
- Mapping/location architecture remains provisional.
- No mapping runtime, Scatter/Gather, wavefront propagation, dynamic nested enrollment, D-2a, E-11B-5, new WGSL, new AccumulatorRole variants, CPU fallback, slot compaction, or simthing-sim arena awareness landed.
- FlatStarResourceFlow remains the accepted bounded production Resource Flow posture.
- Global `PipelineFlags::default().use_accumulator_resource_flow` remains false.
- Presence of `ResourceFlowSpec` alone does not enable GPU execution.

---

## Verdict

**PASS** — SEAD operator toolkit sandbox production test removed; E-11B landed work intact; regressions green; preserved artifacts retained; repo returned to parked state.
