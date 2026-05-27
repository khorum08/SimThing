# E-11B Nested Hierarchy GPU Test Results

Date/time: 2026-05-27T14:15:42.8312831-05:00 to 2026-05-27T14:19:23.6224500-05:00

Base HEAD before commit: `4780cdbdfba1301d95902f8d633c8e7e471b357a`

Final commit SHA after commit: not known at report generation time; see the commit containing this report.

Toolchain:
- `rustc 1.95.0 (59807616e 2026-04-14)`
- `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`

Platform/OS: Microsoft Windows NT 10.0.26200.0

GPU availability notes: GPU path was available locally. The E-11B D=3/D=4 parity tests and replay tests ran instead of taking the no-GPU skip branch.

Full log: [`e11b_nested_hierarchy_gpu_full.log`](e11b_nested_hierarchy_gpu_full.log)

## Commands

| Command | Result |
|---|---|
| `git status --short` | PASS |
| `git rev-parse HEAD` | PASS |
| `rustc --version` | PASS |
| `cargo --version` | PASS |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | PASS, 12 passed |
| `cargo test -p simthing-driver --test resource_flow_scenario_class_burn_in -- --nocapture` | PASS, 16 passed |
| `cargo test -p simthing-driver --test resource_flow_scenario_class_default_on -- --nocapture` | PASS, 16 passed |
| `cargo test -p simthing-driver --test resource_flow_opt_in_product_soak -- --nocapture` | PASS, 13 passed |
| `cargo test -p simthing-driver --test resource_flow_opt_in_telemetry -- --nocapture` | PASS, 6 passed |
| `cargo test -p simthing-driver --test resource_flow_opt_in_burn_in -- --nocapture` | PASS, 15 passed |
| `cargo test -p simthing-driver --test resource_flow_opt_in -- --nocapture` | PASS, 13 passed |
| `cargo test -p simthing-spec --test resource_flow_opt_in_roundtrip -- --nocapture` | PASS, 3 passed |
| `cargo test -p simthing-driver --test e2b5_dynamic_enrollment_soak -- --nocapture` | PASS, 12 passed |
| `cargo test -p simthing-driver --test e2b5_dynamic_fission_enrollment -- --nocapture` | PASS, 21 passed |
| `cargo test -p simthing-driver --test resource_flow_enrollment_session -- --nocapture` | PASS, 3 passed |
| `cargo test -p simthing-driver --test resource_flow_enrollment_compile -- --nocapture` | PASS, 9 passed |
| `cargo test -p simthing-driver --test e11_resource_flow_soak -- --nocapture` | PASS, 6 passed |
| `cargo test -p simthing-driver --test e11_burn_in_scenarios -- --nocapture` | PASS, 6 passed |
| `cargo test -p simthing-driver --test e11_burn_in -- --nocapture` | PASS, 4 passed |
| `cargo test -p simthing-driver --test e11_arena_allocation -- --nocapture` | PASS, 14 passed |
| `cargo test -p simthing-driver --test resource_economy_designer_ron_session -- --nocapture` | PASS, 3 passed |
| `cargo test -p simthing-driver --test resource_economy_burn_in -- --nocapture` | PASS, 5 passed |
| `cargo test -p simthing-gpu accumulator_op -- --nocapture` | PASS, 72 passed |
| `cargo check --workspace` | PASS |
| `cargo test --workspace` | PASS |

## Important Excerpts

E-11B focused suite:

```text
running 12 tests
test e11b_d3_static_nested_cpu_gpu_parity ... ok
test e11b_d4_static_nested_cpu_gpu_parity ... ok
test e11b_nested_execution_plan_has_depth_ordered_bands ... ok
test e11b_nested_preserves_participant_identity ... ok
test e11b_nested_rejects_gap_only_flat_star_leaf_claim ... ok
test e11b_nested_no_boundary_slot_compaction ... ok
test e11b_nested_replay_same_seed_same_frames ... ok
test e11b_nested_flat_star_regressions_unchanged ... ok
test e11b_nested_no_simthing_sim_arena_imports ... ok
test e11b_nested_no_new_wgsl ... ok
test e11b_nested_flag_default_false ... ok
test result: ok. 12 passed; 0 failed; 0 ignored
```

Workspace verification:

```text
cargo check --workspace
PASS

cargo test --workspace
PASS
```

Warnings: existing unused/deprecated test warnings remain visible in the full log; no command failed.

Substitutions: none. All requested target names existed and were run as requested.

Final verdict: PASS.
