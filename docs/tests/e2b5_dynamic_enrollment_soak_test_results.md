# E-2B-5 Dynamic Enrollment Soak Test Results

## Run metadata

| Field | Value |
|-------|-------|
| **Date/time** | 2026-05-19 (local) |
| **Base HEAD (pre-commit)** | `edad7d36ac1db9b3d4d8efde2eed00b6ebfba6db` |
| **Final commit SHA** | `e0a573c` (PR #178) |
| **rustc** | 1.95.0 (59807616e 2026-04-14) |
| **cargo** | 1.95.0 (f2d3ce0bd 2026-03-21) |
| **Platform/OS** | Windows 10.0.26200 (win32) |
| **GPU availability** | GPU available — dynamic-enrollment soak GPU-path tests ran (no skip) |

## Scope

Resource Flow dynamic enrollment soak gate for landed E-2B-5R Policy A path: static session-open enrollment, boundary dynamic fission enrollment, arena-root sibling append, atomic failure behavior, visible boundary reports, generation bump, conditional Resource Flow re-sync when flag enabled, 100–1000 tick opt-in burn-in, replay determinism.

## Commands and results

| Command | Result |
|---------|--------|
| `git status --short` | PASS |
| `git rev-parse HEAD` | PASS — `edad7d36ac1db9b3d4d8efde2eed00b6ebfba6db` |
| `rustc --version` | PASS |
| `cargo --version` | PASS |
| `cargo test -p simthing-driver --test e2b5_dynamic_enrollment_soak -- --nocapture` | **PASS** — 12 passed |
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
| `cargo test --workspace` | **PASS** |

## Key soak tests (all PASS)

- `e2b5_soak_single_fission_100_ticks_gpu_parity`
- `e2b5_soak_single_fission_1000_ticks_gpu_parity`
- `e2b5_soak_multiple_fissions_100_ticks_stable`
- `e2b5_soak_two_arenas_dynamic_enrollment_100_ticks`
- `e2b5_soak_reject_when_cap_full_no_partial_mutation`
- `e2b5_soak_contiguity_blocked_no_compaction`
- `e2b5_soak_flag_off_updates_registry_but_no_gpu_sync`
- `e2b5_soak_replay_same_seed_same_dynamic_enrollment_frames`
- `e2b5_soak_repeated_resync_after_dynamic_admissions_stable`

## Excerpts

```
running 12 tests
test e2b5_soak_single_fission_1000_ticks_gpu_parity ... ok
test e2b5_soak_reject_when_cap_full_no_partial_mutation ... ok
test e2b5_soak_repeated_resync_after_dynamic_admissions_stable ... ok
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured
```

## Full log

See [`e2b5_dynamic_enrollment_soak_full.log`](e2b5_dynamic_enrollment_soak_full.log).

## Final verdict

**PASS** — Resource Flow dynamic enrollment soak verified; E-2B-5R remained atomic under soak; GPU path exercised locally.
