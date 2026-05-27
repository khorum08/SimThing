# RF-T1 — Resource Flow opt-in flagging test results

| Field | Value |
|-------|-------|
| **Date/time** | 2026-05-19 (local verification run) |
| **Base HEAD (before commit)** | `d969a55a2b74d7723b1c5e728f1477ff36df6d10` |
| **Final commit SHA** | `e3d589e` |
| **rustc** | 1.95.0 (59807616e 2026-04-14) |
| **cargo** | 1.95.0 (f2d3ce0bd 2026-03-21) |
| **Platform/OS** | Windows 10.0.26200 (win32) |
| **GPU availability** | Local wgpu adapter available — flat-star upload, dynamic enrollment resync, and soak GPU paths executed (not skipped) |

Full command output: [`resource_flow_opt_in_flagging_full.log`](resource_flow_opt_in_flagging_full.log)

## Commands and results

| Command | Result |
|---------|--------|
| `git status --short` | PASS |
| `cargo test -p simthing-spec --test resource_flow_opt_in_roundtrip -- --nocapture` | PASS — 2/2 |
| `cargo test -p simthing-driver --test resource_flow_opt_in -- --nocapture` | PASS — 13/13 |
| `cargo test -p simthing-driver --test e2b5_dynamic_enrollment_soak -- --nocapture` | PASS — 12/12 |
| `cargo test -p simthing-driver --test e2b5_dynamic_fission_enrollment -- --nocapture` | PASS — 21/21 |
| `cargo test -p simthing-driver --test resource_flow_enrollment_session -- --nocapture` | PASS — 3/3 |
| `cargo test -p simthing-driver --test resource_flow_enrollment_compile -- --nocapture` | PASS — 9/9 |
| `cargo test -p simthing-driver --test e11_resource_flow_soak -- --nocapture` | PASS — 6/6 |
| `cargo test -p simthing-driver --test e11_burn_in_scenarios -- --nocapture` | PASS — 6/6 |
| `cargo test -p simthing-driver --test e11_burn_in -- --nocapture` | PASS — 4/4 |
| `cargo test -p simthing-driver --test e11_arena_allocation -- --nocapture` | PASS — 14/14 |
| `cargo test -p simthing-gpu accumulator_op -- --nocapture` | PASS — 72/72 |
| `cargo check --workspace` | PASS |
| `cargo test --workspace` | PASS |

## RF-T1 test highlights

- `resource_flow_opt_in_mode_roundtrips_ron` — FlatStarOptIn RON roundtrip
- `resource_flow_opt_in_default_disabled` — missing/explicit Disabled defaults
- `resource_flow_opt_in_disabled_keeps_flag_false` — Disabled mode leaves GPU sync off
- `resource_flow_opt_in_flat_star_enables_resource_flow_flag_only` — only `use_accumulator_resource_flow` enabled (transfer/emission remain false)
- `resource_flow_opt_in_populated_spec_without_opt_in_stays_inactive` — enrollment compiles; flag stays false
- `resource_flow_opt_in_flat_star_session_open_uploads_ops` — E-11 flat-star ops upload under opt-in
- `resource_flow_opt_in_dynamic_enrollment_resyncs_after_fission` — E-2B-5 Policy A resync under opt-in session
- `resource_flow_opt_in_no_nested_gpu_claims` — flat-star D=2 guard
- `resource_flow_opt_in_wildcard_rejected_at_session_open` — nested/wildcard path rejected

## Notable excerpts

```
test resource_flow_opt_in_flat_star_enables_resource_flow_flag_only ... ok
test resource_flow_opt_in_dynamic_enrollment_resyncs_after_fission ... ok
test resource_flow_opt_in_populated_spec_without_opt_in_stays_inactive ... ok
```

Regression suites (`e2b5_*`, `e11_*`, `resource_flow_enrollment_*`) remained green.

## Final verdict

**PASS** — RF-T1 implementation gate; GPU Resource Flow opt-in path exercised locally.
