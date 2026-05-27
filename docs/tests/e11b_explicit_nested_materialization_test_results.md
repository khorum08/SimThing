# E-11B-1 Explicit Nested Participant Materialization — Test Results

**Date/time:** 2026-05-27 (local)  
**Base HEAD before commit:** `5c65c1a60a3c653a5e30fb92138e92b3d8d8bf3c`  
**Final commit SHA:** `2fe88db`  
**rustc:** 1.95.0 (59807616e 2026-04-14)  
**cargo:** 1.95.0 (f2d3ce0bd 2026-03-21)  
**Platform/OS:** Windows 10 (win32 10.0.26200)  
**GPU availability:** Present — E-11B nested GPU regression suites executed on GPU; E-11B-1 materialization tests are CPU/topology only.

## Commands

| Command | Result |
|---------|--------|
| `cargo fmt` | PASS |
| `cargo check --workspace` | PASS |
| `cargo test -p simthing-driver --test e11b_nested_materialization -- --nocapture` | PASS (10/10) |
| `cargo test -p simthing-driver --test e11_arena_allocation -- --nocapture` | PASS (14/14) |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | PASS (12/12) |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture` | PASS (13/13) |
| `cargo test -p simthing-driver --test resource_flow_scenario_class_burn_in -- --nocapture` | PASS (16/16) |
| `cargo test -p simthing-driver --test resource_flow_flat_star_continued_soak -- --nocapture` | PASS (12/12) |
| `cargo test -p simthing-spec --test resource_flow_opt_in_roundtrip -- --nocapture` | PASS (3/3) |
| `cargo test --workspace` | PASS |

## Notable excerpts

- `e11b_explicit_nested_materialization_d3_contiguous_per_parent` — `max_depth == 3`, per-parent child contiguity verified.
- `e11b_explicit_nested_materialization_d4_contiguous_per_parent` — `max_depth == 4`, two arena-root participant trees, contiguity verified.
- `e11b_explicit_nested_materialization_flat_star_regression` — flat-star `max_depth == 2` unchanged with `parent_subtree_root_id: None`.
- `e11b_nested_hierarchy_gpu` regression suite — 12/12 green (GPU path).
- Missing-parent and cycle cases reject with `SpecError::UnknownExplicitParticipantParent` / `SpecError::ExplicitParticipantParentCycle`.

## Verdict

**PASS**
