# Resource Flow Limited Scenario-Class Posture

**Status:** Active production-facing guide for the accepted bounded `FlatStarResourceFlow` posture.

This guide explains the Resource Flow posture accepted by
[`docs/reviews/resource_flow_limited_scenario_class_production_posture.md`](reviews/resource_flow_limited_scenario_class_production_posture.md).
It is documentation and observability guidance only. It does not define new
runtime behavior.

## Summary

Limited scenario-class `FlatStarResourceFlow` is the current bounded production
Resource Flow posture.

That means a scenario or game mode may enable the existing flat-star Resource
Flow GPU path only by explicitly selecting
`ResourceFlowExecutionProfile::FlatStarResourceFlow`, or by using the older
spec-level `ResourceFlowOptInMode::FlatStarOptIn`.

This does not authorize global default-on.
`PipelineFlags::default().use_accumulator_resource_flow` remains false, and a populated
`ResourceFlowSpec` alone does not enable GPU execution.

## What FlatStarResourceFlow Means

`FlatStarResourceFlow` is a scenario-class / execution-profile declaration. At
session open, it enables the existing E-11 flat-star Resource Flow path for
scenarios that fit the burned-in shape:

- Flat-star D=2 allocation.
- Explicit bounded participation.
- Static E-2B enrollment, E-2B-5 Policy A dynamic fission enrollment, or both.
- Existing AccumulatorOp v2 registrations.
- Driver/session-owned `ArenaRegistry` and `ArenaParticipant` artifacts.
- `simthing-sim` remains arena-ignorant.

The profile is not a new Resource Flow mode. It is an explicit way to select
the already-soaked bounded path.

## Not Global Default-On

Global default-on would mean Resource Flow GPU execution is enabled by default
for all sessions or all authored Resource Flow specs. That remains rejected.

Current invariant:

```rust
PipelineFlags::default().use_accumulator_resource_flow == false
```

`FlatStarResourceFlow` does not change that default. It is a named
scenario-class/profile opt-in at session open.

## Not ResourceFlowSpec Presence

`ResourceFlowSpec` presence stages authoring/session artifacts. It does not
mean the GPU Resource Flow path is active.

When a spec is populated but no explicit enablement exists:

- The registry/scaffold can still be built.
- Dynamic enrollment can update driver/session state according to existing
  Policy A behavior.
- GPU Resource Flow ops stay inactive.
- Telemetry should report `DefaultDisabled`.

This separation is intentional. Spec presence is structure. Execution requires
explicit posture.

## Relationship to Spec FlatStarOptIn

`ResourceFlowOptInMode::FlatStarOptIn` remains supported.

If a spec explicitly uses `FlatStarOptIn`, it is still the source of enablement
and takes precedence over scenario-class profile attribution. Telemetry should
show `SpecFlatStarOptIn`, not `ScenarioClassDefaultOn`, when both are present.

Use `FlatStarOptIn` when the authored spec itself is declaring execution
intent. Use `FlatStarResourceFlow` when a bounded product scenario class or
execution profile is declaring that same flat-star execution intent.

## Accepted Scenario Classes

`FlatStarResourceFlow` may be used for:

- Bounded flat-star D=2 continuous Resource Flow scenarios.
- Static participant sets resolved by E-2B enrollment.
- Policy A dynamic fission inheritance where children append as arena-root
  sibling participants.
- Multi-arena scenarios where each arena remains independently flat-star and
  no coupling-heavy semantics are required.
- Scenarios that accept approximate-deterministic continuous-flow conservation.
- Scenarios whose operational diagnosis can rely on existing flag-source,
  execution-profile, admission/rejection, sync, band/op, error, and replay
  telemetry.

The covered execution paths are only:

- E-11 flat-star.
- E-2B static enrollment.
- E-2B-5 Policy A dynamic enrollment.

## Blocked Scenario Classes

Keep these blocked from `FlatStarResourceFlow` production posture:

- Nested dynamic enrollment (E-11B-5) — static nested D=3/D=4 materialization is landed for explicit nested tests, but nested dynamic admission is paused/deferred until a named product scenario requires it.
- Policy B `Reevaluate` selector re-run.
- Wildcard or unbounded dynamic selector expansion.
- Coupling-heavy arena graphs requiring product-scale delay semantics.
- Exact hard-currency transfer.
- Any scenario that needs CPU production allocation fallback.
- Any scenario that requires boundary-time slot compaction or indirection-list
  `SlotRange` replacement.
- Any scenario that needs `simthing-sim` to understand arenas.
- Any scenario whose safety depends on a redesigned designer/RON/spec guardrail
  layer.

## Telemetry Fields

Resource Flow telemetry is collected in `ResourceFlowOptInTelemetryReport`.
Interpret fields as follows:

| Field | Meaning |
|-------|---------|
| `scenario_name` | Human-readable scenario / fixture name used for reporting. |
| `opt_in_mode` | Spec-level Resource Flow opt-in mode. `Disabled` may still appear for scenario-class enablement. |
| `flag_source` | Why the session flag was set or left disabled. This is the first field to inspect. |
| `execution_profile_name` | Execution profile name, such as `DefaultDisabled` or `FlatStarResourceFlow`. |
| `resource_flow_enabled` | Whether the session flag is true. This reflects execution enablement, not spec presence. |
| `arenas_planned` | Arena count in the session registry. Nonzero does not prove GPU execution. |
| `participants_planned` | Participant count in the session registry. Nonzero does not prove GPU execution. |
| `total_ops` | Uploaded Resource Flow AccumulatorOp count. Should be positive for active flat-star GPU scenarios. |
| `n_bands` | Resource Flow order-band count. Should be positive for active flat-star GPU scenarios. |
| `generation_start` / `generation_end` | Arena registry generation window observed by telemetry. |
| `dynamic_admissions` | Boundary-time dynamic participant admissions observed. |
| `dynamic_rejections` | Boundary-time dynamic participant rejections observed. |
| `sync_count` | Observed Resource Flow sync count. Repeated resync fixtures use this to detect churn/regression. |
| `max_abs_error` | Burn-in/oracle maximum absolute error when a burn report is attached. |
| `replay_bit_exact` | Whether paired burn-in/replay evidence was bit-exact. |

## Flag Source Interpretation

| `ResourceFlowFlagSource` | Interpretation |
|--------------------------|----------------|
| `DefaultDisabled` | Resource Flow GPU execution is inactive. Populated spec state may still exist. |
| `SpecFlatStarOptIn` | Spec `ResourceFlowOptInMode::FlatStarOptIn` enabled the flat-star path. This takes precedence over profile attribution. |
| `ScenarioClassDefaultOn` | `ResourceFlowExecutionProfile::FlatStarResourceFlow` enabled the bounded scenario-class flat-star path. |
| `TestOverride` | A test or harness forced the flag. Do not treat as product posture evidence. |

## Operator Checklist

For a scenario expected to run bounded Resource Flow:

1. Confirm `resource_flow_enabled == true`.
2. Confirm `flag_source` is `ScenarioClassDefaultOn` or `SpecFlatStarOptIn`.
3. Confirm `execution_profile_name == "FlatStarResourceFlow"` for scenario-class posture.
4. Confirm `total_ops > 0` and `n_bands > 0`.
5. Confirm planned arenas and participants match scenario expectations.
6. Check dynamic admissions and rejections after fission-heavy boundaries.
7. Check `sync_count` for unexpected repeated upload churn.
8. Check `max_abs_error` and `replay_bit_exact` when burn-in evidence is attached.
9. Confirm transfer/emission flags are not being enabled by Resource Flow posture.
10. Confirm global defaults remain off in new tests or startup diagnostics.

For a scenario expected to stay inactive:

1. Confirm `resource_flow_enabled == false`.
2. Confirm `flag_source == DefaultDisabled`.
3. Treat nonzero `arenas_planned` or `participants_planned` as staged session
   state, not GPU execution.
4. Confirm `total_ops == 0` and `n_bands == 0`.

## Stop Conditions

Stop bounded `FlatStarResourceFlow` work and route to a separate gate if the
scenario requires any of the following:

- Global Resource Flow default-on.
- GPU execution inferred from `ResourceFlowSpec` presence.
- E-11B nested dynamic enrollment (E-11B-5).
- Policy B `Reevaluate` selector re-run.
- Wildcard or unbounded dynamic selector expansion.
- New WGSL.
- New `AccumulatorRole` variants.
- CPU production allocation fallback.
- `simthing-sim` arena awareness.
- Boundary-time slot compaction.
- Indirection-list `SlotRange` replacement.
- Hard-currency transfer through Resource Flow.
- Immediate designer/RON/spec guardrail redesign inside the RF track.

Route nested dynamic enrollment requirements to E-11B-5 (E-11B track paused until product names a scenario). Route hard-currency ordering
requirements to D-2a. Route authoring-safety and product-UX guardrail
requirements to the future simthing-spec/RON/Designer rebuild track.

## Regression Checklist

Keep these invariants visible in future RF changes:

- Global Resource Flow flag default remains false.
- Populated `ResourceFlowSpec` with default profile remains inactive.
- Spec `FlatStarOptIn` still works and takes precedence.
- Scenario-class `FlatStarResourceFlow` records `ScenarioClassDefaultOn`.
- E-11 flat-star, E-2B static enrollment, and E-2B-5 Policy A remain the only
  covered execution paths.
- No WGSL, role, CPU fallback, slot compaction, or simthing-sim arena import is
  introduced by posture polish.
