# E-11B-4 nested fission / gap preservation hardening — test results

**Date/time:** 2026-05-27 (local verification run)  
**Base HEAD (before commit):** `14743135092e76060d4ff807f8165f9b434c350d`  
**Final commit SHA:** `31135b7afd08672ca54e0ea807b8b216335d74db`  
**rustc:** 1.95.0 (59807616e 2026-04-14)  
**cargo:** 1.95.0 (f2d3ce0bd 2026-03-21)  
**Platform/OS:** Windows 10 (win32 10.0.26200), x86_64  
**GPU availability:** Local GPU available — E-11B nested D=3/D=4 CPU/GPU parity tests executed (not skipped)

## Scope

E-11B-4 nested fission/gap preservation hardening. New suite: `crates/simthing-driver/tests/e11b_nested_fission_gap.rs` (13 tests). Narrow driver helpers only; no posture change.

Deleted superseded kickoff artifacts:
- `docs/tests/e11b_nested_hierarchy_gpu_test_results.md`
- `docs/tests/e11b_nested_hierarchy_gpu_full.log`

Full regression log: [`e11b_nested_fission_gap_full.log`](e11b_nested_fission_gap_full.log)

## Commands and results

| Command | Result |
|---------|--------|
| `git status --short` | PASS — E-11B-4 changes only (+ unrelated workshop txt noise excluded from commit) |
| `git rev-parse HEAD` | `14743135092e76060d4ff807f8165f9b434c350d` (pre-commit base) |
| `rustc --version` | PASS |
| `cargo --version` | PASS |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture` | **PASS** — 13/13 |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | **PASS** — 12/12 |
| `cargo test -p simthing-driver --test resource_flow_scenario_class_burn_in -- --nocapture` | **PASS** — 16/16 |
| `cargo test -p simthing-driver --test resource_flow_scenario_class_default_on -- --nocapture` | **PASS** — 16/16 |
| `cargo test -p simthing-driver --test resource_flow_opt_in_product_soak -- --nocapture` | **PASS** — 13/13 |
| `cargo test -p simthing-driver --test resource_flow_opt_in_telemetry -- --nocapture` | **PASS** — 6/6 |
| `cargo test -p simthing-driver --test resource_flow_opt_in_burn_in -- --nocapture` | **PASS** — 15/15 |
| `cargo test -p simthing-driver --test resource_flow_opt_in -- --nocapture` | **PASS** — 13/13 |
| `cargo test -p simthing-spec --test resource_flow_opt_in_roundtrip -- --nocapture` | **PASS** — 3/3 |
| `cargo test -p simthing-driver --test e2b5_dynamic_enrollment_soak -- --nocapture` | **PASS** — 12/12 |
| `cargo test -p simthing-driver --test e2b5_dynamic_fission_enrollment -- --nocapture` | **PASS** — 21/21 |
| `cargo test -p simthing-driver --test resource_flow_enrollment_session -- --nocapture` | **PASS** — 3/3 |
| `cargo test -p simthing-driver --test resource_flow_enrollment_compile -- --nocapture` | **PASS** — 9/9 |
| `cargo test -p simthing-driver --test e11_resource_flow_soak -- --nocapture` | **PASS** — 6/6 |
| `cargo test -p simthing-driver --test e11_burn_in_scenarios -- --nocapture` | **PASS** — 6/6 |
| `cargo test -p simthing-driver --test e11_burn_in -- --nocapture` | **PASS** — 4/4 |
| `cargo test -p simthing-driver --test e11_arena_allocation -- --nocapture` | **PASS** — 14/14 |
| `cargo test -p simthing-driver --test resource_economy_designer_ron_session -- --nocapture` | **PASS** — 3/3 |
| `cargo test -p simthing-driver --test resource_economy_burn_in -- --nocapture` | **PASS** — 5/5 |
| `cargo test -p simthing-gpu accumulator_op -- --nocapture` | **PASS** — 72/72 |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` | **PASS** — workspace green (pre-existing ignored tests only) |

## E-11B-4 suite (13 tests)

All **PASS**:

- `e11b_nested_reserved_gap_child_stays_outside_active_child_slotrange`
- `e11b_nested_gap_child_does_not_become_allocation_leaf`
- `e11b_nested_parent_child_contiguity_preserved_after_gap_claim`
- `e11b_nested_rejects_noncontiguous_active_children_without_compaction`
- `e11b_nested_gap_claim_preserves_d3_cpu_gpu_parity_for_active_tree`
- `e11b_nested_gap_claim_preserves_d4_cpu_gpu_parity_for_active_tree`
- `e11b_nested_gap_exhaustion_rejects_without_partial_mutation`
- `e11b_nested_replay_same_seed_same_gap_state`
- `e11b_nested_flat_star_gap_regression_unchanged`
- `e11b_nested_flat_star_regression_session_unchanged`
- `e11b_nested_no_simthing_sim_arena_imports`
- `e11b_nested_no_new_wgsl`
- `e11b_nested_flag_default_false`

## Important excerpts

E-11B fission/gap primary suite:

```
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s
```

Nested hierarchy GPU regression (unchanged):

```
test e11b_d3_static_nested_cpu_gpu_parity ... ok
test e11b_d4_static_nested_cpu_gpu_parity ... ok
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.73s
```

## Posture confirmation (tests)

- `PipelineFlags::default().use_accumulator_resource_flow` remains **false**
- No new WGSL files
- No `simthing-sim` arena imports
- Flat-star gap regression unchanged
- Gap exhaustion rejects without partial mutation
- Non-contiguous nested active children rejected without slot compaction

## Final verdict

**PASS** — E-11B nested fission/gap hardening tests and full regression ladder green on local GPU.
