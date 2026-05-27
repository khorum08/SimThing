# E-11B Static Nested Participant RON Smoke — Test Results

**Date/time:** 2026-05-27 (local)  
**Base HEAD before commit:** `b8d181053432f1e72120907b38e85fec60050ac5`  
**Final commit SHA:** pending at report write  
**rustc:** 1.95.0 (59807616e 2026-04-14)  
**cargo:** 1.95.0 (f2d3ce0bd 2026-03-21)  
**Platform/OS:** Windows 10 (win32 10.0.26200)  
**GPU availability:** Present — E-11B nested GPU regression suites executed on GPU; new RON smoke tests are CPU/materialization only.

## Commands

| Command | Result |
|---------|--------|
| `cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip -- --nocapture` | PASS (2/2) |
| `cargo test -p simthing-driver --test e11b_nested_materialization_ron_session -- --nocapture` | PASS (3/3) |
| `cargo test -p simthing-driver --test e11b_nested_materialization -- --nocapture` | PASS (10/10) |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | PASS (12/12) |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture` | PASS (13/13) |
| `cargo check --workspace` | PASS |
| `cargo test --workspace` | PASS |

## Notable excerpts

- `resource_flow_nested_participant_parent_field_roundtrips_ron` — `parent_subtree_root_id` survives GameModeSpec RON serialize/parse/reserialize.
- `resource_flow_nested_participant_missing_parent_field_defaults_none_ron` — omitted field defaults to `None`; flat participant RON omits `parent_subtree_root_id`.
- `e11b_nested_materialization_from_ron_d3_reaches_nested_layout` — RON-parsed D=3 spec → `max_depth == 3`, nested contiguity verified.
- `e11b_nested_materialization_from_ron_d4_reaches_nested_layout` — RON-parsed D=4 spec → `max_depth == 4`, two participant roots.
- `e11b_nested_materialization_ron_flat_star_regression` — flat-star `max_depth == 2` unchanged; all participants have `parent_subtree_root_id: None`.
- Deleted inspected artifact: `docs/tests/e11b_explicit_nested_materialization_test_results.md`.

## Verdict

**PASS**
