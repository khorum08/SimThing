# Revert SEAD sandbox to parked state — test results

**Date/time:** 2026-05-19  
**Base HEAD (before revert branch):** `79edd35` — SEAD field-intelligence feasibility probe merge (PR #200)  
**Revert branch commit:** `84ecc99` — revert PR #200 merge  
**Final commit SHA:** `b45dace` (revert merge PR #201)  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell  
**GPU availability:** Local GPU present — E-11B nested hierarchy GPU parity 12/12 PASS.

---

## Commands

| Command | Result |
|---------|--------|
| `git status --short` | PASS — revert branch; preserve docs restored |
| `git rev-parse HEAD` | PASS — pre-merge revert branch |
| `rustc --version` | PASS |
| `cargo --version` | PASS |
| `cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-driver --test e11b_nested_materialization_ron_session -- --nocapture` | **PASS** — 3/3 |
| `cargo test -p simthing-driver --test e11b_nested_materialization -- --nocapture` | **PASS** — 10/10 |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | **PASS** — 12/12 |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture` | **PASS** — 13/13 |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` | **PASS** (~166s) |
| `cargo test -p simthing-driver --test sead_field_intelligence_sandbox` | **EXPECTED REMOVAL** — `no test target named sead_field_intelligence_sandbox` |

**Full log:** [`revert_sead_sandbox_to_parked_state_full.log`](revert_sead_sandbox_to_parked_state_full.log)

---

## Removals verified

- `crates/simthing-driver/tests/sead_field_intelligence_sandbox.rs` — **deleted**

---

## Preserved (unchanged by revert)

- `docs/workshop/sead_sandbox_code_preserve.rs`
- `docs/tests/sead_field_intelligence_sandbox_test_results.md`
- `docs/tests/sead_field_intelligence_sandbox_full.log`
- `ExplicitParticipantSpec.parent_subtree_root_id`
- `materialize_arena_participants` nested topology
- E-11B-1 materialization tests
- E-11B RON smoke tests
- E-11B nested hierarchy GPU + fission/gap tests

---

## Posture restored

- SEAD field-intelligence sandbox reverted; decision-gate evidence preserved externally in docs.
- Overall probe verdict remains **PARTIAL** (see [`sead_field_intelligence_sandbox_test_results.md`](sead_field_intelligence_sandbox_test_results.md)).
- Mapping/location architecture remains provisional.
- Implementation parked until mapping doc is ready or product names a concrete non-mapping scenario.
- FlatStarResourceFlow bounded posture unchanged.
- `PipelineFlags::default().use_accumulator_resource_flow` remains **false**.

---

## Final verdict

**PASS** — SEAD sandbox production test removed; E-11B landed work intact; regressions green; preserved artifacts retained; repo returned to parked state.
