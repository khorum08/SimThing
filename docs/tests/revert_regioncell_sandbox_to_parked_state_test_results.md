# Revert RegionCell sandbox to parked state — test results

**Date/time:** 2026-05-27 (recovered after session crash)  
**Base HEAD (before revert branch):** `416c6fc` — RegionCell sandbox merge + workshop HEAD update (PR #197)  
**Revert branch commits:** `08f43e9` (revert workshop HEAD), `b454221` (revert PR #197 merge)  
**Final commit SHA:** `ee33e89`  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell  
**GPU availability:** Local GPU present — E-11B nested hierarchy GPU parity 12/12 PASS.

---

## Commands

| Command | Result |
|---------|--------|
| `git status --short` | PASS — revert branch; doc updates staged pending commit |
| `git rev-parse HEAD` | PASS — `b454221` (pre-docs commit) |
| `rustc --version` | PASS |
| `cargo --version` | PASS |
| `cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-driver --test e11b_nested_materialization_ron_session -- --nocapture` | **PASS** — 3/3 |
| `cargo test -p simthing-driver --test e11b_nested_materialization -- --nocapture` | **PASS** — 10/10 |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | **PASS** — 12/12 |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture` | **PASS** — 13/13 |
| `cargo check --workspace` | **PASS** |
| `cargo test -p simthing-driver --test mapping_regioncell_field_intelligence_sandbox` | **EXPECTED REMOVAL** — `no test target named mapping_regioncell_field_intelligence_sandbox` |
| `cargo test --workspace` | **PASS** |

---

## Removals verified

- `crates/simthing-driver/tests/mapping_regioncell_field_intelligence_sandbox.rs` — **deleted**
- `docs/tests/mapping_regioncell_field_intelligence_sandbox_test_results.md` — **deleted**
- `docs/tests/mapping_regioncell_field_intelligence_sandbox_full.log` — not present

---

## Preserved (unchanged by revert)

- `ExplicitParticipantSpec.parent_subtree_root_id`
- `ExplicitParticipantSpec::flat` / `::nested`
- `materialize_arena_participants` nested topology
- E-11B-1 materialization tests
- `resource_flow_nested_participant_roundtrip.rs`
- `e11b_nested_materialization_ron_session.rs`
- E-11B nested hierarchy GPU + fission/gap tests

---

## Posture restored

- Implementation parked after E-11B-1 and E-11B RON smoke.
- Sparse RegionCell field-intelligence sandbox reverted; concept validated externally only.
- Mapping/location architecture remains provisional.
- FlatStarResourceFlow bounded posture unchanged.
- `PipelineFlags::default().use_accumulator_resource_flow` remains **false**.

---

## Final verdict

**PASS** — RegionCell sandbox removed; E-11B landed work intact; regressions green; repo returned to parked state.

---

## Re-run (2026-05-28 06:45:54 -05:00)

**HEAD at re-run:** `f2807a1` — workshop HEAD update after revert merge (PR #198)  
**Note:** Test result files preserved; this section appends a fresh verification pass.

| Command | Result |
|---------|--------|
| `git status --short` | PASS — unrelated workshop report churn only |
| `git rev-parse HEAD` | PASS — `f2807a1` |
| `rustc --version` | PASS — `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | PASS — `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-driver --test e11b_nested_materialization_ron_session -- --nocapture` | **PASS** — 3/3 |
| `cargo test -p simthing-driver --test e11b_nested_materialization -- --nocapture` | **PASS** — 10/10 |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | **PASS** — 12/12 |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture` | **PASS** — 13/13 |
| `cargo check --workspace` | **PASS** |
| `cargo test -p simthing-driver --test mapping_regioncell_field_intelligence_sandbox` | **EXPECTED REMOVAL** — `no test target named mapping_regioncell_field_intelligence_sandbox` |
| `cargo test --workspace` | **PASS** (~206s) |

**Full log:** [`revert_regioncell_sandbox_to_parked_state_full.log`](revert_regioncell_sandbox_to_parked_state_full.log)

**Re-run verdict:** **PASS** — parked state confirmed; E-11B regressions 40/40; sandbox target still absent; workspace green.

