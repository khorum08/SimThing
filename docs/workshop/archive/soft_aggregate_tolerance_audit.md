# Soft-Aggregate Tolerance Audit (PR A-4)

**Author:** Opus 4.7
**Date:** 2026-05-25
**Gate for:** Implementation half of PR A-4 (`SoftAggregateGuard` field on `SubFieldSpec` and `assert_no_hard_trigger_on_soft_aggregate`)
**Status:** Accepted (analysis complete; implementation lands in this PR)

---

## TL;DR

> **Existing production exposure: zero structural hard triggers are currently
> driven by soft-aggregate values.** Every existing fission, fusion, property
> expiry, and capability unlock threshold registers against `THRESH_BUF_VALUES`
> (the per-slot, pre-reduction buffer). The only path that registers on
> `THRESH_BUF_OUTPUT` (post-reduction, where soft-aggregate drift lives) is
> `AggregateAlert`, which is informational only — no consumer in
> `simthing-driver` or `simthing-spec` uses it as a trigger.
>
> The ADR's tolerance policy is therefore **forward-protecting**, not a
> remediation of an existing wrong. We ship the guard now, before C-5
> (WeightedMean migration) or E0 (economic substrate) introduces a way for
> soft aggregates to flow into structural decisions.

---

## 1. Method

PR A-4 (`docs/accumulator_op_v2_production_plan.md`) asks for:

> Review `boundary.rs`, `threshold_registry.rs`, and `overlay_lifecycle.rs`
> for any code path that reads a reduced value and uses it in a structural
> decision (fission trigger, overlay lifecycle, property expiry).
> Cross-reference against the `DimensionRegistry` to find which properties
> use `ReductionRule::WeightedMean`.

I read the production code rather than tests, and traced every site where:

1. A `ThresholdRegistration` is constructed (which buffer? which semantic?)
2. A `ThresholdEvent` is dispatched (does it gate a structural mutation?)
3. CPU code reads `values_shadow` or `output_vectors` and feeds the value
   into a `BoundaryRequest`, `FissionOutcome`, `ExpiryOutcome`, or
   `LifecycleOutcome`.

The two-buffer distinction (`THRESH_BUF_VALUES` vs `THRESH_BUF_OUTPUT`,
defined in `simthing-gpu/src/world_state.rs:188-198`) is the decisive
mechanical signal: the reduction passes (4-6) write to `output_vectors`;
everything else reads `values`. Soft-aggregate drift only enters at
`output_vectors`.

I also widened the audit beyond the production plan's framing: the plan
mentions only `WeightedMean`, but `ReductionRule::Mean` is the default for
`Amount`, `Velocity`, `Named(_)`, and `Custom(_)` roles (see
`simthing-core/src/reduction.rs:61-69`). **`Mean` is in the soft class** —
both the ADR semantic scope section and `CombineFn::is_soft_aggregate()`
(`simthing-core/src/accumulator_op.rs:100-102`) treat it that way. Any
guard policy must cover both rules.

---

## 2. Existing production exposure

### 2.1 Threshold registration paths, classified

| Path | Where | Buffer | Semantic | Hard trigger? | Soft exposure |
|---|---|---|---|---|---|
| Fission per property | `threshold_registry.rs::walk` (615-631) | `VALUES` | `FissionTrigger` | **Yes** | None |
| Decay → expiry (3 variants) | `push_decay_thresholds` (654-723) | `VALUES` | `PropertyExpiry` | **Yes** | None |
| Fusion lineage | `push_fusion_lineage` (561-596) | `VALUES` | `FusionTrigger` | **Yes** | None |
| Capability unlock | `push_capability_unlocks` (462-496) | `VALUES` | `CapabilityUnlock` | Semi (durable state) | None |
| Scripted event trigger | `push_scripted_event_triggers` (442-460) | `VALUES` | `ScriptedEventTrigger` | Variable (designer-defined effects) | None |
| Velocity alert | `push_velocity_alerts` (726-760) | `VALUES` | `VelocityAlert` | No (informational) | None |
| **Aggregate alert** | `push_aggregate_alerts` (762-796) | **`OUTPUT`** | `AggregateAlert` | No (informational) | **Reads reduced** |

`AggregateAlert` is the only existing path that targets `THRESH_BUF_OUTPUT`.

### 2.2 `AggregateAlert` downstream consumers

Tracing `AggregateAlertEvent` through the workspace:

- `boundary.rs:1066-1082` constructs them from threshold events.
- `boundary.rs:247` attaches them to `BoundaryOutcome.aggregate_alerts`.
- `delta_log.rs:228` serialises them as `BoundaryDeltaEntry::AggregateAlert`
  for the replay log.
- `simthing-driver` contains **no** references to `aggregate_alerts` or
  `AggregateAlertEvent` (grep confirms zero matches across `src/`).
- `simthing-spec` contains no consumers either.
- Tests reference them only to assert presence (`boundary_integration.rs:1129`).

**No fission, fusion, expiry, lifecycle, capability, or scripted-event
handler reads an aggregate alert.** They surface to the AI/observability
layer (`design_v5.md §6`) and to the replay log. The current architecture
treats post-reduction crossings as advisory.

### 2.3 CPU-side reads of `values_shadow` for structural decisions

These are the three sites where CPU code reads from the shadow and feeds a
structural mutation:

| Site | Reads | Used for |
|---|---|---|
| `fission.rs::check_secondary` (216) | `values_shadow[s * n_dims + col]` | Fission secondary condition (Intensity/Amount above/below) |
| `overlay_lifecycle.rs::read_sub_field` (212-224) | `shadow[base + col]` (per-slot) | `DissolveCondition::PropertyReaches/PropertyBelow` |
| `property_expiry.rs::cpu_decay_collect_node` (222) | `values_shadow[slot * n_dims + col]` | `TowardZero` decay sweep |

**All three read per-slot offsets** (`slot * n_dims + col`), which is the
pre-reduction shadow populated by `state.read_values()` at boundary step 0.
None of them touch `ReducedField` or `output_vectors`.

### 2.4 What about `ReducedField`?

`simthing-sim::reduced_field::ReducedField` is the CPU view of
`output_vectors`. It is produced by `BoundaryProtocol::read_reduced_field`
on demand (UI cadence, not boundary cadence). A workspace-wide grep for
`read_reduced_field` returns zero in-tree callers outside tests: it is
present for the eventual Studio / observability layer but no production
boundary path consumes it.

### 2.5 Today's `ReductionRule::WeightedMean` usage

`WeightedMean { by }` is **declared** in `simthing-core/src/reduction.rs` and
implemented in the GPU shader (`crates/simthing-gpu/src/reduction.rs`), but
no production property in the current default property set uses it as a
`SubFieldSpec::reduction_override`. A grep across `crates/` shows
`WeightedMean` appearing only in:

- The workshop A/B test fixtures (`simthing-workshop/`)
- The reduction GPU oracle and shader
- The CPU oracle test suite
- The new `CombineFn::WeightedMean` variant (A-2)

No `register(SimProperty { ... fission_templates: [..], ... })` in
production passes a layout where a `WeightedMean`-reduced sub-field is also
the `FissionThreshold::sub_field`. That combination would require a
designer to explicitly set both `reduction_override: Some(WeightedMean { by })`
and `fission_templates: [FissionThreshold { sub_field: <same role>, .. }]`
on the same property — possible in principle, absent in practice.

### 2.6 Conclusion of audit

The current architecture is **structurally separated**:

- Hard triggers register against `VALUES` (pre-reduction).
- Soft aggregates exist only in `output_vectors` (post-reduction).
- The CPU code paths that drive structural mutations all read from
  per-slot shadow, never from `ReducedField`.

The boundary between "exact" and "soft" coincides with the boundary
between `VALUES` and `OUTPUT`. **This is a load-bearing accident, not an
invariant.** Nothing in the type system today prevents:

1. A future `AggregateAlert` consumer from reading the event and routing
   it to a `BoundaryRequest::Remove` (informational becomes structural).
2. A new `ThresholdSemantic` arm emitted on `THRESH_BUF_OUTPUT` for a
   structural action.
3. The C-5 WeightedMean migration introducing a registration that targets
   `OUTPUT` and feeds a hard trigger.
4. A designer adding `reduction_override: Some(Mean)` on a sub-field that
   also has a `FissionThreshold` and a future pipeline (E0 economic
   transfers or post-AccumulatorOp routing) that reads the reduced value
   instead of the slot value.

The guard's job is to make this boundary explicit and enforceable.

---

## 3. Recommended guard pattern

### 3.1 Three candidate patterns

The production plan and ADR mention three implementations:

| Variant | Semantic | When to use |
|---|---|---|
| `Unguarded` | Field is explicitly approved as soft-aggregate-without-guard | Aggregate alerts; AI summary scores; UI display columns. Never feeds a structural trigger. |
| `Quantized { step }` | Round the reduced value to nearest `step` before threshold comparison | Coarse-grained categorical decisions (loyalty zones at 0.1 step). Eliminates sub-step drift from triggering re-evaluation. |
| `Hysteresis { band }` | Threshold fires only when the value exits a `band` around the last commit | Bi-stable decisions (capability stays unlocked until clearly re-locked). Best for state that should latch. |

**Recommendation:** **Adopt all three; let the designer pick per-field.**

The choice depends on the semantic of the value, not on the pipeline.
`Quantized` is appropriate for monotonically-trending values (efficiency
scores that climb or fall over many ticks); `Hysteresis` is appropriate for
oscillating values where a single crossing would chatter (faction-level
stability that hovers around an unlock band); `Unguarded` is the explicit
opt-out that documents "this field is informational and the designer has
accepted the drift."

### 3.2 Why not state on the variant?

The production plan text says `Hysteresis { band: f32, last_committed: f32 }`,
embedding mutable state inside the type spec. The accepted v7 design
(`design_v7.md §3`) uses `Hysteresis { band: f32 }` — stateless. **I
recommend the v7 form** and the implementation in `simthing-core/src/accumulator_op.rs`
already follows it (lines 185-196). Reasons:

1. `SubFieldSpec` is part of the schema, not the runtime. Schema types must
   serialise stably; embedding `last_committed` would make every snapshot
   churn the schema bytes each tick.
2. The "last committed" cursor is per-(slot, registration), not per-spec.
   A single property's `Hysteresis { band }` declares the policy; the
   actual cursor lives on the AccumulatorOp registration state or on a
   sidecar map in the boundary protocol (deferred to C-5 implementation).
3. Quantization is stateless. Putting state on `Hysteresis` only would
   introduce an asymmetry between the variants with no upside.

### 3.3 Where to enforce

The guard must be enforced at **threshold-registration time**, not at
**event-firing time**, because:

- A misconfigured registration is an authoring bug. Catching it at the
  fire site means the error surfaces stochastically (only when the
  threshold actually crosses), making debugging painful.
- Registration happens at session open and at boundary topology-rebuild,
  not per-tick. A panic here is a cheap, deterministic gate.
- The validator's input (`ThresholdSemantic` + `SimPropertyId` +
  `SubFieldRole` + buffer hint) is fully known at registration.

`ThresholdBuilder` is the chokepoint: every GPU threshold registration
flows through one of its `push_*` helpers. Adding the validator call
inside the four "hard structural trigger" helpers (`push_*` for
`FissionTrigger`, `FusionTrigger`, `PropertyExpiry`, `CapabilityUnlock`)
covers every existing path without disrupting `VelocityAlert` /
`AggregateAlert` / `ScriptedEventTrigger`, which are not in the
hard-trigger class.

### 3.4 Validator scoping rule

To avoid breaking every existing fission test (which uses `Amount` =
default-`Mean` reduction with `FissionThreshold` registered on the
`Amount` column), the validator panics only when **all** of these hold:

1. The semantic is a hard structural trigger
   (`FissionTrigger | FusionTrigger | PropertyExpiry | CapabilityUnlock`).
2. The registered buffer is `THRESH_BUF_OUTPUT` (post-reduction).
3. The resolved reduction for the registered `(property, sub_field)` is
   `Mean` or `WeightedMean`.
4. The sub-field's `soft_aggregate_guard` is `None` or `Some(Unguarded)`.

This is the **precise risk profile**: only post-reduction reads of a
soft-aggregate column selection a hard trigger. Today no path satisfies
condition 2 + 3 simultaneously for a hard trigger. The validator is a
forward gate.

`Unguarded` is intentionally rejected for hard triggers. Per its docstring
("Only valid for fields that never feed threshold registrations"), it
documents an explicit decision that the field is informational. If a
designer attaches a hard trigger to an `Unguarded` soft-aggregate column,
that is exactly the bug the gate exists to catch.

---

## 4. Exact type signature for the guard

### 4.1 Schema field on `SubFieldSpec` (`simthing-core/src/property.rs`)

```rust
pub struct SubFieldSpec {
    // ...existing fields...
    pub reduction_override: Option<ReductionRule>,
    /// Tolerance policy for soft-aggregate reductions. Required (must be
    /// `Some(Quantized { .. })` or `Some(Hysteresis { .. })`) when this
    /// sub-field's resolved reduction is `Mean` or `WeightedMean` AND
    /// the sub-field is registered as a hard structural threshold reading
    /// from the post-reduction buffer (`THRESH_BUF_OUTPUT`).
    ///
    /// `None` and `Some(Unguarded)` are equivalent: no guard applied.
    /// Enforced at threshold-registration time by
    /// `assert_no_hard_trigger_on_soft_aggregate`.
    #[serde(default)]
    pub soft_aggregate_guard: Option<SoftAggregateGuard>,
}
```

Serde-default `None` means every existing RON file deserialises unchanged.

### 4.2 Validator function (`simthing-sim/src/threshold_registry.rs`)

```rust
/// Validation outcome for a single threshold registration.
#[derive(Clone, Debug, PartialEq)]
pub enum SoftAggregateViolation {
    /// A hard structural trigger reads from the post-reduction buffer
    /// against a soft-aggregate sub-field without a guard.
    HardTriggerOnUnguardedSoftAggregate {
        property_id:  SimPropertyId,
        sub_field:    SubFieldRole,
        rule:         ReductionRule,
        semantic_kind: &'static str,
    },
}

/// Verify that `(semantic, property, sub_field, buffer)` does not
/// constitute an un-guarded hard structural trigger on a soft-aggregate
/// column. Returns `Ok(())` for safe registrations; returns a
/// `SoftAggregateViolation` for unsafe ones.
///
/// The four classifying conditions are documented in
/// `docs/workshop/soft_aggregate_tolerance_audit.md §3.4`.
pub fn assert_no_hard_trigger_on_soft_aggregate(
    semantic:    &ThresholdSemantic,
    property_id: SimPropertyId,
    sub_field:   &SubFieldRole,
    buffer:      u32,
    dim_reg:     &DimensionRegistry,
) -> Result<(), SoftAggregateViolation> { ... }
```

The validator is called from `ThresholdBuilder::push_*` for hard-trigger
arms. The caller pattern uses `.expect("assert_no_hard_trigger_on_soft_aggregate")`
to surface the violation as a panic at registration time, matching the
production plan wording. A `Result`-returning function lets the spec layer
or future tools surface the violation as a recoverable error if they
prefer to validate before commit.

### 4.3 Wiring

- `push_capability_unlocks` calls the validator before each push.
- `push_decay_thresholds` calls it for each variant before pushing.
- `walk` calls it for each `FissionThreshold` push.
- `push_fusion_lineage` calls it before each FusionTrigger push.
- `push_velocity_alerts`, `push_aggregate_alerts`,
  `push_scripted_event_triggers` do **not** call it (they are not in the
  hard-trigger class).

Today's paths all use `THRESH_BUF_VALUES`, so the validator's condition 2
fails and it returns `Ok(())` unconditionally. The wiring is in place so
that future PRs (C-5, E-1..E-3) that introduce `OUTPUT` registrations for
hard triggers will fail loudly without needing further code changes.

---

## 5. Out of scope for this PR

- Wiring the guard cursor for `Hysteresis` (the per-registration
  `last_committed` cursor). That belongs in C-5 alongside the actual
  WeightedMean AccumulatorOp migration.
- Quantization application at threshold-comparison time. Same reason —
  C-5 is where the threshold compare path becomes soft-aggregate-aware.
- Studio / RON authoring affordances for the new field. Modder docs land
  in E-6.
- The post-reduction summary checksum readback design (B-4 Opus gate).
  Independent question.

---

## 6. Sign-off checklist

Per A-4 acceptance criteria:

- [x] Survey of `boundary.rs`, `threshold_registry.rs`, `overlay_lifecycle.rs`
- [x] DimensionRegistry / WeightedMean cross-reference
- [x] (a) Existing production exposure documented (§2)
- [x] (b) Recommended guard pattern justified (§3)
- [x] (c) Exact type signature specified (§4)
- [ ] Human sign-off on the guard pattern before merge (PR description requests it)

---

## References

- `docs/adr_accumulator_op_v2.md` — ADR §Semantic scope, soft-aggregate policy
- `docs/design_v7.md` §3, §9 — `SubFieldSpec` extension; invariant row 9
- `docs/accumulator_op_v2_production_plan.md` PR A-4 — task spec
- `crates/simthing-core/src/accumulator_op.rs` — `SoftAggregateGuard`,
  `CombineFn::is_soft_aggregate`
- `crates/simthing-core/src/reduction.rs` — `ReductionRule` defaults
- `crates/simthing-sim/src/threshold_registry.rs` — registration paths
- `crates/simthing-gpu/src/world_state.rs` — `THRESH_BUF_VALUES/OUTPUT`
