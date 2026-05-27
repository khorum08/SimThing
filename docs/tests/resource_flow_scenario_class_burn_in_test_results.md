# RF-T5 — Resource Flow scenario-class burn-in / telemetry soak test results

| Field | Value |
|-------|-------|
| **Date/time (local)** | 2026-05-19 (evening) |
| **Base HEAD (before commit)** | `bbc6df0c46111e9532917655d9966deff8338cc6` |
| **Final commit SHA** | `44a00e77fb41adbb9e0089858bc868c4480b0d1c` |
| **rustc** | `rustc 1.95.0 (59807616e 2026-04-14)` |
| **cargo** | `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| **Platform/OS** | Windows 10.0.26200 (win32), PowerShell |
| **GPU availability** | Local wgpu adapter available — RF-T5 GPU scenario-class soak paths executed (not skipped) |

Full command output: [`resource_flow_scenario_class_burn_in_full.log`](resource_flow_scenario_class_burn_in_full.log)

---

## Preflight

| Command | Result |
|---------|--------|
| `git status --short` | PASS — RF-T5 driver module/tests/docs; deleted RF-T4 test artifacts |
| `git rev-parse HEAD` | `bbc6df0c46111e9532917655d9966deff8338cc6` (pre-commit) |
| `rustc --version` | PASS |
| `cargo --version` | PASS |

---

## Targeted RF-T5 + regression suites

| Command | Result | Notes |
|---------|--------|-------|
| `cargo test -p simthing-driver --test resource_flow_scenario_class_burn_in -- --nocapture` | **PASS** | 16/16 (GPU paths exercised) |
| `cargo test -p simthing-driver --test resource_flow_scenario_class_default_on -- --nocapture` | **PASS** | 16/16 |
| `cargo test -p simthing-driver --test resource_flow_opt_in_product_soak -- --nocapture` | **PASS** | 13/13 |
| `cargo test -p simthing-driver --test resource_flow_opt_in_telemetry -- --nocapture` | **PASS** | 6/6 |
| `cargo test -p simthing-driver --test resource_flow_opt_in_burn_in -- --nocapture` | **PASS** | 15/15 |
| `cargo test -p simthing-driver --test resource_flow_opt_in -- --nocapture` | **PASS** | 13/13 |
| `cargo test -p simthing-spec --test resource_flow_opt_in_roundtrip -- --nocapture` | **PASS** | 3/3 |
| `cargo test -p simthing-driver --test e2b5_dynamic_enrollment_soak -- --nocapture` | **PASS** | 12/12 |
| `cargo test -p simthing-driver --test e2b5_dynamic_fission_enrollment -- --nocapture` | **PASS** | _(see full log)_ |
| `cargo test -p simthing-driver --test resource_flow_enrollment_session -- --nocapture` | **PASS** | 3/3 |
| `cargo test -p simthing-driver --test resource_flow_enrollment_compile -- --nocapture` | **PASS** | 9/9 |
| `cargo test -p simthing-driver --test e11_resource_flow_soak -- --nocapture` | **PASS** | 6/6 |
| `cargo test -p simthing-driver --test e11_burn_in_scenarios -- --nocapture` | **PASS** | 6/6 |
| `cargo test -p simthing-driver --test e11_burn_in -- --nocapture` | **PASS** | 4/4 |
| `cargo test -p simthing-driver --test e11_arena_allocation -- --nocapture` | **PASS** | 14/14 |
| `cargo test -p simthing-gpu accumulator_op -- --nocapture` | **PASS** | 72/72 |
| `cargo check --workspace` | **PASS** | |
| `cargo test --workspace` | **PASS** | First run hit transient `STATUS_STACK_BUFFER_OVERRUN` during parallel compile; immediate retry green |

---

## RF-T5 highlights

- New module [`resource_flow_scenario_class_burn_in.rs`](../../crates/simthing-driver/src/resource_flow_scenario_class_burn_in.rs) mirrors RF-T3 product soak but opens via `ResourceFlowExecutionProfile::FlatStarResourceFlow` with spec `opt_in_mode: Disabled`.
- Fixtures: `rf_t5_profile_static_128/256`, dynamic fission cadence, multi-arena, multi-session replay, disabled/default inactive, rejection telemetry, repeated resync.
- Telemetry contract: `ScenarioClassDefaultOn` flag source + `FlatStarResourceFlow` execution profile name.
- Deleted superseded RF-T4 local test artifacts (`resource_flow_scenario_class_default_on_test_results.md`, `_full.log`).
- Posture preserved: global `PipelineFlags::default().use_accumulator_resource_flow` remains false; spec presence alone does not enable GPU; FlatStarOptIn precedence; no WGSL/new roles/CPU fallback/simthing-sim arena awareness.

---

## Final verdict

**PASS** — RF-T5 scenario-class burn-in / telemetry soak; GPU paths exercised locally; targeted regression suites green; `cargo test --workspace` green on retry.

**Recommended next gate:** Resource Flow limited scenario-class production posture review (not global default-on).
