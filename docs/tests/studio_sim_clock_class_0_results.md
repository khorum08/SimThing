# STUDIO-SIM-CLOCK-CLASS-0 Results

## Status

**PROBATION / proof-present / DA-review-pending** — gate-wiring class registration. Expected clearance for this PR: `DA-RESERVE(gate-wiring)`. Do not self-merge.

## Identity

| Field | Value |
|---|---|
| Rung | `STUDIO-SIM-CLOCK-CLASS-0` |
| Track | `0.0.8.6-studio-live-ops` |
| Kind | clearance-router class-hardening (gate-wiring) |
| Blocked PR | [#1258](https://github.com/khorum08/SimThing/pull/1258) (`DA-RESERVE(admitted-scope-router-gap)`) |
| New class | `studio-sim-clock-substrate` |

## Changed files

| Path | Role |
|---|---|
| `scripts/ci/precedented_classes.tsv` | Class row |
| `scripts/ci/class_predicates.tsv` | Predicate / envelope |
| `scripts/ci/clearance_check.sh` | Register 11 new selftest fixtures |
| `scripts/ci/fixtures/clearance/clearance_selftest_studio_clock_class_*` | Pos/neg/non-regression fixtures |
| `scripts/ci/test_inventory.tsv` | Fixture ledger rows |
| `docs/tests/studio_sim_clock_class_0_results.md` | This evidence |
| `docs/design_0_0_8_6_studio_live_ops.md` | 9.1h adjacency rung |
| `docs/orchestrator_orientation.md` | Regenerated |

No product implementation files changed.

## Class row summary

```text
class_id: studio-sim-clock-substrate
envelope: 0.0.8.6-studio-live-ops
requirements: tested_code_sha|coverage_basis|ci_green
status: active
promotion_blocker: STUDIO-SIM-CLOCK-CLASS-0
```

Scope covers mapeditor `StudioSimClock` substrate + headless tests + results + inventory/triage + rolled-in 9.0 readiness/design/orientation only.

## Predicate row summary

```text
detect_mode: any_then_envelope
priority: 30
match_any:
  crates/simthing-mapeditor/src/studio_sim_clock.rs
  | crates/simthing-mapeditor/tests/studio_sim_clock_*.rs
  | docs/tests/studio_sim_clock_*_results.md
forbidden:
  app/ui.rs | app/mod.rs | scenario_library* | studio_live_session*
  | simthing-driver/** | simthing-sim/** | simthing-gpu/** | simthing-kernel/**
```

## Positive fixture summary

| Fixture | Expected |
|---|---|
| `clearance_selftest_studio_clock_class_clearable` | `ORCHESTRATOR-CLEARABLE` (#1258 shape) |

## Negative fixture summary

| Fixture | Expected |
|---|---|
| `..._missing_tested_sha` | `FAIL(missing-tested-code-sha...)` |
| `..._missing_coverage` | `FAIL(missing-tested-code-sha...)` |
| `..._missing_ci_green` | `FAIL(ci-not-green...)` |
| `..._rejects_transport_ui` | `DA-RESERVE(class-envelope-violation)` |
| `..._rejects_live_bridge` | `DA-RESERVE(class-envelope-violation)` |
| `..._rejects_library_ui` | `DA-RESERVE(class-envelope-violation)` |
| `..._rejects_runtime_src` | `DA-RESERVE(class-envelope-violation)` |
| `..._gate_wiring` | `DA-RESERVE(gate-wiring)` |

## Non-regression fixture summary

| Fixture | Expected |
|---|---|
| `..._api_nonregression` | `ORCHESTRATOR-CLEARABLE` (`tp-admitted-clause-api-composition`) |
| `..._picker_nonregression` | `ORCHESTRATOR-CLEARABLE` (`tp-studio-clause-picker`) |

## Selftest result

```text
Previous: CLEARANCE-SELFTEST: PASS (61 fixtures)
New:      CLEARANCE-SELFTEST: PASS (72 fixtures)
```

## Expected clearance for this PR

```text
CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)
```

## Blocked PR after this lands

After DA merges this class-hardening PR, re-run clearance on #1258:

```text
expected_blocked_pr_after_class: ORCHESTRATOR-CLEARABLE
```

## Falsification held

- No product implementation files changed
- No 9.2 transport UI / 9.3 live bridge / 9.5 library UI implemented
- No runtime/GPU/kernel/engine authority surfaces touched
- Class does not self-clear this gate-wiring PR
- Existing TP API and picker classes still clear known positives
