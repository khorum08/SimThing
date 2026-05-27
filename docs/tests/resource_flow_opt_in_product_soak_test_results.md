# RF-T3 — Resource Flow opt-in product soak / telemetry test results

| Field | Value |
|-------|-------|
| **Date/time** | 2026-05-19 (local verification run) |
| **Base HEAD (before commit)** | `70c2d3465016219920c2c10e277ce83cc3fd280d` |
| **Final commit SHA** | `d920164` |
| **rustc** | `1.95.0 (59807616e 2026-04-14)` |
| **cargo** | `1.95.0 (f2d3ce0bd 2026-03-21)` |
| **Platform/OS** | Windows 10 (win32 10.0.26200), PowerShell |
| **GPU availability** | Local wgpu adapter available — RF-T3 GPU product soak paths executed (not skipped) |

Full command output: [`resource_flow_opt_in_product_soak_full.log`](resource_flow_opt_in_product_soak_full.log)

## Commands and results

| Command | Result |
|---------|--------|
| `git status --short` | PASS — RF-T3 driver changes only (+ unrelated local workshop noise) |
| `git rev-parse HEAD` | `70c2d3465016219920c2c10e277ce83cc3fd280d` (pre-commit base) |
| `rustc --version` | PASS |
| `cargo --version` | PASS |
| `cargo test -p simthing-driver --test resource_flow_opt_in_telemetry -- --nocapture` | PASS — 6/6 |
| `cargo test -p simthing-driver --test resource_flow_opt_in_product_soak -- --nocapture` | PASS — 13/13 |
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
| `cargo test -p simthing-gpu accumulator_op -- --nocapture` | PASS — 72/72 (+89 filtered) |
| `cargo check --workspace` | PASS |
| `cargo test --workspace` | PASS — workspace green (160+ driver tests; 1 ignored in unrelated suite) |

## RF-T3 highlights

- **`ResourceFlowOptInTelemetryReport`** + **`ResourceFlowFlagSource`** (`DefaultDisabled`, `SpecFlatStarOptIn`, `TestOverride`) on `SimSession`.
- Product-like fixtures: 128/256 static, dynamic fission cadence, multi-arena, multi-session replay, disabled diagnostics, rejection telemetry, repeated resync.
- Telemetry surfaces flag source, arenas/participants, ops/bands, generation, dynamic admissions/rejections, sync count, max error, replay bit-exact.
- Global `PipelineFlags::default().use_accumulator_resource_flow` remains **false**.
- No WGSL, no new AccumulatorRole variants, no CPU fallback, `simthing-sim` remains arena-ignorant.

## Notable stdout

```
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out   (telemetry)
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out  (product soak)
```

No Windows linker retries observed during this run.

## Final verdict

**PASS** — RF-T3 product-like opt-in soak and telemetry surfacing; GPU paths exercised locally; regression suites green.
