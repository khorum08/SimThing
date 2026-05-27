# RF-T4 — Resource Flow scenario-class default-on test results

| Field | Value |
|-------|-------|
| **Date/time** | 2026-05-19 (local verification run) |
| **Base HEAD (before commit)** | `73cbacd4d200e89ceb04990881250fc211a2218e` |
| **Final commit SHA** | `ebef7a1` |
| **rustc** | `1.95.0 (59807616e 2026-04-14)` |
| **cargo** | `1.95.0 (f2d3ce0bd 2026-03-21)` |
| **Platform/OS** | Windows 10 (win32 10.0.26200), PowerShell |
| **GPU availability** | Local wgpu adapter available — RF-T4 GPU scenario-class paths executed (not skipped) |

Full command output: [`resource_flow_scenario_class_default_on_full.log`](resource_flow_scenario_class_default_on_full.log)

## Commands and results

| Command | Result |
|---------|--------|
| `git status --short` | PASS — RF-T4 spec/driver/test changes |
| `git rev-parse HEAD` | `73cbacd` (pre-commit base) |
| `rustc --version` | PASS |
| `cargo --version` | PASS |
| `cargo test -p simthing-driver --test resource_flow_scenario_class_default_on -- --nocapture` | PASS — 16/16 |
| `cargo test -p simthing-driver --test resource_flow_opt_in_product_soak -- --nocapture` | PASS — 13/13 |
| `cargo test -p simthing-driver --test resource_flow_opt_in_telemetry -- --nocapture` | PASS — 6/6 |
| `cargo test -p simthing-driver --test resource_flow_opt_in_burn_in -- --nocapture` | PASS — 15/15 |
| `cargo test -p simthing-driver --test resource_flow_opt_in -- --nocapture` | PASS — 13/13 |
| `cargo test -p simthing-spec --test resource_flow_opt_in_roundtrip -- --nocapture` | PASS — 3/3 |
| `cargo test -p simthing-driver --test e2b5_dynamic_enrollment_soak -- --nocapture` | PASS — 12/12 |
| `cargo test -p simthing-driver --test e2b5_dynamic_fission_enrollment -- --nocapture` | PASS — 21/21 |
| `cargo test -p simthing-driver --test resource_flow_enrollment_session -- --nocapture` | PASS — 3/3 |
| `cargo test -p simthing-driver --test resource_flow_enrollment_compile -- --nocapture` | PASS — 9/9 |
| `cargo test -p simthing-driver --test e11_resource_flow_soak -- --nocapture` | PASS — 6/6 |
| `cargo test -p simthing-driver --test e11_burn_in_scenarios -- --nocapture` | PASS — 6/6 |
| `cargo test -p simthing-driver --test e11_burn_in -- --nocapture` | PASS — 4/4 |
| `cargo test -p simthing-driver --test e11_arena_allocation -- --nocapture` | PASS — 14/14 |
| `cargo test -p simthing-gpu accumulator_op -- --nocapture` | PASS — 72/72 (+89 filtered) |
| `cargo check --workspace` | PASS |
| `cargo test --workspace` | PASS |

## RF-T4 highlights

- **`ResourceFlowExecutionProfile`** on `GameModeSpec` (`DefaultDisabled`, `FlatStarResourceFlow`).
- Session open precedence: spec `FlatStarOptIn` → `SpecFlatStarOptIn`; profile `FlatStarResourceFlow` with spec disabled → `ScenarioClassDefaultOn`.
- **`ResourceFlowFlagSource::ScenarioClassDefaultOn`** + `execution_profile_name` in telemetry.
- Global `PipelineFlags::default().use_accumulator_resource_flow` remains **false**.
- No WGSL, no new AccumulatorRole variants, no CPU fallback, `simthing-sim` remains arena-ignorant.

## Notable stdout

```
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out  (scenario class)
```

## Final verdict

**PASS** — RF-T4 limited scenario-class default-on; GPU paths exercised locally; regression suites green.
