# RF-T2 — Resource Flow opt-in burn-in test results

| Field | Value |
|-------|-------|
| **Date/time** | 2026-05-19 (local verification run) |
| **Base HEAD (before commit)** | `79f5311b586b95ba51c9bbd2a3d61f98e593aa40` |
| **Final commit SHA** | `e13a4b4` |
| **rustc** | 1.95.0 (59807616e 2026-04-14) |
| **cargo** | 1.95.0 (f2d3ce0bd 2026-03-21) |
| **Platform/OS** | Windows 10.0.26200 (win32) |
| **GPU availability** | Local wgpu adapter available — RF-T2 GPU burn-in paths executed (not skipped) |

Full command output: [`resource_flow_opt_in_burn_in_full.log`](resource_flow_opt_in_burn_in_full.log)

## Commands and results

| Command | Result |
|---------|--------|
| `git status --short` | PASS |
| `cargo test -p simthing-driver --test resource_flow_opt_in_burn_in -- --nocapture` | PASS — 15/15 |
| `cargo test -p simthing-driver --test resource_flow_opt_in -- --nocapture` | PASS — 13/13 |
| `cargo test -p simthing-spec --test resource_flow_opt_in_roundtrip -- --nocapture` | PASS — 2/2 |
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
| `cargo test --workspace` | PASS (1st run: transient `link.exe` 1102 on `pr3_capability_builder`; retry PASS) |

## RF-T2 highlights

- Static FlatStarOptIn: 10-participant and 64-participant 1000-tick burn-in via `open_from_spec`
- Skewed-weight 1000-tick bit-exact
- Dynamic single/multi fission 1000-tick with generation bump + admissions
- Two-arena flat-star 100-tick (no coupling)
- Disabled populated spec stays inactive (no GPU sync)
- Wildcard FlatStarOptIn rejected at session open
- Repeated resync stable (100 cycles)
- Replay same-seed bit-exact parity
- Global `PipelineFlags::default().use_accumulator_resource_flow` remains false
- Transfer/emission flags not enabled under opt-in

## Notable excerpts

```
test rf_t2_static_flat_star_10_participants_1000_ticks ... ok
test rf_t2_dynamic_multi_fission_1000_ticks ... ok
test rf_t2_disabled_populated_spec_stays_inactive ... ok
test rf_t2_flat_star_opt_in_rejects_wildcard_or_nested_claim ... ok
test result: ok. 15 passed; 0 failed
```

## Final verdict

**PASS** — RF-T2 implementation gate; GPU opt-in burn-in path exercised locally.
