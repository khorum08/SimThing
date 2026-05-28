# Revert SEAD strategic horizon sandbox to parked state — test results

**Date/time:** 2026-05-19  
**Base HEAD (before revert branch):** `0878c39` — SEAD strategic horizon probe merge (PR #202)  
**Revert branch commit:** `3ce06b4` — revert PR #202 merge  
**Final commit SHA:** `9e25d4f` (revert merge PR #203)  
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
| `cargo test -p simthing-driver --test sead_strategic_horizon_sandbox` | **EXPECTED REMOVAL** — `no test target named sead_strategic_horizon_sandbox` |

**Full log:** [`revert_sead_strategic_horizon_sandbox_to_parked_state_full.log`](revert_sead_strategic_horizon_sandbox_to_parked_state_full.log)

---

## Removals verified

- `crates/simthing-driver/tests/sead_strategic_horizon_sandbox.rs` — **deleted**

---

## Preserved (unchanged by revert)

- `docs/workshop/sead_strategic_horizon_sandbox_code_preserve.rs`
- `docs/tests/sead_strategic_horizon_sandbox_test_results.md`
- `docs/tests/sead_strategic_horizon_sandbox_full.log`
- First SEAD probe preserved artifacts (`sead_sandbox_code_preserve.rs`, etc.)
- E-11B-1 materialization + RON smoke + GPU parity + fission/gap tests

---

## Posture restored

- SEAD strategic horizon / velocity / PF-skip sandbox reverted; decision-gate evidence preserved.
- Overall probe verdict remains **PARTIAL** (see [`sead_strategic_horizon_sandbox_test_results.md`](sead_strategic_horizon_sandbox_test_results.md)).
- Mapping/location architecture remains provisional.
- Implementation parked until mapping doc is ready or product names a concrete non-mapping scenario.

---

## Final verdict

**PASS** — Strategic horizon sandbox production test removed; E-11B intact; regressions green; preserved artifacts retained; repo returned to parked state.
