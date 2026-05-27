# E-2B-5R Dynamic Fission Enrollment Atomicity Test Results

## Run metadata

| Field | Value |
|-------|-------|
| **Date/time** | 2026-05-19 (local) |
| **Git HEAD (pre-commit)** | `a7408452efc949c15946dedbb5ad25f8c3fee715` |
| **rustc** | 1.95.0 (59807616e 2026-04-14) |
| **cargo** | 1.95.0 (f2d3ce0bd 2026-03-21) |
| **Platform/OS** | Windows 10.0.26200 (win32) |
| **GPU availability** | GPU available — dynamic-enrollment GPU-path tests ran (no skip) |

## Scope

E-2B-5R remedial hardening: two-phase dynamic admission (`prepare_dynamic_arena_root_append` / `commit_dynamic_arena_root_append`), registry-failure rollback, visible boundary enrollment reports on `SimSession`, conditional Resource Flow sync after successful admissions only.

## Commands and results

| Command | Result |
|---------|--------|
| `git status --short` | PASS (expected modified driver/gpu + docs) |
| `git rev-parse HEAD` | PASS — `a7408452efc949c15946dedbb5ad25f8c3fee715` |
| `rustc --version` | PASS |
| `cargo --version` | PASS |
| `cargo test -p simthing-driver --test e2b5_dynamic_fission_enrollment -- --nocapture` | **PASS** — 21 passed |
| `cargo test -p simthing-driver --test resource_flow_enrollment_session -- --nocapture` | **PASS** — 3 passed |
| `cargo test -p simthing-driver --test resource_flow_enrollment_compile -- --nocapture` | **PASS** — 9 passed |
| `cargo test -p simthing-driver --test e11_resource_flow_soak -- --nocapture` | **PASS** — 6 passed |
| `cargo test -p simthing-driver --test e11_burn_in_scenarios -- --nocapture` | **PASS** — 6 passed |
| `cargo test -p simthing-driver --test e11_burn_in -- --nocapture` | **PASS** — 4 passed |
| `cargo test -p simthing-driver --test e11_arena_allocation -- --nocapture` | **PASS** — 14 passed |
| `cargo test -p simthing-driver --test e10r2_arena_participant -- --nocapture` | **PASS** — 7 passed |
| `cargo test -p simthing-driver --test e10r3_arena_participant_block -- --nocapture` | **PASS** — 6 passed |
| `cargo test -p simthing-driver --test e10r_resource_flow_preflight -- --nocapture` | **PASS** — 5 passed |
| `cargo test -p simthing-gpu accumulator_op -- --nocapture` | **PASS** — 72 passed |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` | **PASS** — all crates green (4 ignored tests in unrelated suites) |

## Key e2b5 remedial tests (all PASS)

- `e2b5_max_participants_exceeded_rejects_child_without_partial_mutation`
- `e2b5_registry_rejection_does_not_mutate_scaffold_or_tree`
- `e2b5_session_boundary_records_dynamic_enrollment_report`
- `e2b5_no_admission_does_not_change_resource_flow_upload_state`
- `e2b5_successful_admission_still_resyncs_when_flag_enabled`
- `e2b5_replay_same_seed_same_dynamic_enrollment`
- `e2b5_100_tick_flat_star_burn_in_after_dynamic_enrollment`

## Excerpts

```
running 21 tests
test e2b5_registry_rejection_does_not_mutate_scaffold_or_tree ... ok
test e2b5_max_participants_exceeded_rejects_child_without_partial_mutation ... ok
test e2b5_session_boundary_records_dynamic_enrollment_report ... ok
test e2b5_no_admission_does_not_change_resource_flow_upload_state ... ok
test e2b5_successful_admission_still_resyncs_when_flag_enabled ... ok
test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured
```

## Full log

See [`e2b5r_dynamic_fission_enrollment_atomicity_full.log`](e2b5r_dynamic_fission_enrollment_atomicity_full.log).

## Final verdict

**PASS** — E-2B-5R atomicity and visible diagnostics hardening verified; GPU dynamic-enrollment path exercised locally.
