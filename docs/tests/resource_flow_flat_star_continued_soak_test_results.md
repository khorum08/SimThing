# Continued flat-star Resource Flow soak checkpoint — test results

**Date/time:** 2026-05-27 (local verification run)  
**Base HEAD (before commit):** `3d500986c7f2bd42774b18c35dd2ec28c10897b5`  
**Final commit SHA:** `05f58fe` (branch `continued-flat-star-soak-checkpoint`)  
**rustc:** 1.95.0 (59807616e 2026-04-14)  
**cargo:** 1.95.0 (f2d3ce0bd 2026-03-21)  
**Platform/OS:** Windows 10 (win32 10.0.26200), x86_64  
**GPU availability:** Local GPU available — all GPU soak tests executed (not skipped)

## Scope

Continued flat-star Resource Flow soak checkpoint. Reuses RF-T5/RF-T6 `FlatStarResourceFlow` infrastructure. Adds `FlatStarContinuedSoakSummary` and `resource_flow_flat_star_continued_soak` test suite (12 tests). **512-participant static fixture used as specified** (no substitution required; ~5s suite runtime locally).

Full regression log: [`resource_flow_flat_star_continued_soak_full.log`](resource_flow_flat_star_continued_soak_full.log) (partial; completed via sequential retry after transient GPU process exit during batch run)

## Commands and results

| Command | Result |
|---------|--------|
| `git status --short` | PASS — implementation + docs changes |
| `git rev-parse HEAD` | `3d500986c7f2bd42774b18c35dd2ec28c10897b5` (pre-commit base) |
| `rustc --version` | PASS |
| `cargo --version` | PASS |
| `cargo test -p simthing-driver --test resource_flow_flat_star_continued_soak -- --nocapture` | **PASS** — 12/12 |
| `cargo test -p simthing-driver --test resource_flow_scenario_class_burn_in -- --nocapture` | **PASS** — 16/16 |
| `cargo test -p simthing-driver --test resource_flow_scenario_class_default_on -- --nocapture` | **PASS** — 16/16 |
| `cargo test -p simthing-driver --test resource_flow_opt_in_product_soak -- --nocapture` | **PASS** — 13/13 |
| `cargo test -p simthing-driver --test resource_flow_opt_in_telemetry -- --nocapture` | **PASS** — 6/6 |
| `cargo test -p simthing-driver --test resource_flow_opt_in_burn_in -- --nocapture` | **PASS** — 15/15 (retry after transient batch crash) |
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
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | **PASS** — 12/12 |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture` | **PASS** — 13/13 |
| `cargo test -p simthing-driver --test resource_economy_designer_ron_session -- --nocapture` | **PASS** — 3/3 |
| `cargo test -p simthing-driver --test resource_economy_burn_in -- --nocapture` | **PASS** — 5/5 |
| `cargo test -p simthing-gpu accumulator_op -- --nocapture` | **PASS** — 72/72 |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` | **PASS** |

## Continued soak suite (12 tests)

All **PASS**:

- `rf_flat_star_continued_static_512_participants_1000_ticks`
- `rf_flat_star_continued_static_skewed_weights_1000_ticks`
- `rf_flat_star_continued_dynamic_policy_a_1000_ticks`
- `rf_flat_star_continued_multi_arena_no_coupling_1000_ticks`
- `rf_flat_star_continued_replay_same_seed_same_summary`
- `rf_flat_star_continued_telemetry_has_flag_source_and_profile`
- `rf_flat_star_continued_global_flag_default_false`
- `rf_flat_star_continued_populated_spec_without_opt_in_inactive`
- `rf_flat_star_continued_does_not_enable_transfer_or_emission`
- `rf_flat_star_continued_no_simthing_sim_arena_imports`
- `rf_flat_star_continued_no_new_wgsl`
- `rf_flat_star_continued_flat_star_only_no_nested_claims`

## Important excerpts

Primary continued soak suite:

```
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.31s
```

512-participant static @ 1000 ticks completed within continued soak suite (no participant-count substitution).

## Posture confirmation

- FlatStarResourceFlow remains bounded production posture
- No Resource Flow semantics expansion
- E-11B remains paused; E-11B-5 unauthorized without named scenario
- Global flag default false; spec presence alone inactive
- Transfer/emission flags remain disabled in soak paths
- No new WGSL; `simthing-sim` remains arena-ignorant

## Final verdict

**PASS** — continued flat-star soak checkpoint and full regression ladder green on local GPU.
