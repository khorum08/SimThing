# C-5 WeightedMean / Mean Reduction Design Gate (Opus audit + design memo)

**Author:** Opus 4.7
**Date:** 2026-05-25
**Gate for:** Codex 5.5 implementation PR — `feat(gpu): C-5 WeightedMean/Mean soft reductions → AccumulatorOp`
**Status:** Accepted (design); implementation PR follows separately
**Implementer:** **Codex 5.5** (mechanical execution from this memo)
**Companion:** `docs/adr_accumulator_op_v2.md`, `docs/design_v7.md` §3 (SoftAggregateGuard) + §4 (pipeline), `docs/accumulator_op_v2_production_plan.md` PR C-5, `docs/workshop/pivot_forward_implementation_policy.md`, `docs/workshop/soft_aggregate_tolerance_audit.md` (A-4), `docs/workshop/c4_overlay_orderband_compiler_design.md` (C-4)

---

## TL;DR

> **Audit result (re-verified):** zero existing production exposure to soft
> aggregates feeding hard structural triggers. The A-4 finding still holds
> verbatim: every `FissionTrigger` / `FusionTrigger` / `PropertyExpiry` /
> `CapabilityUnlock` registers on `THRESH_BUF_VALUES` (pre-reduction); the
> only `THRESH_BUF_OUTPUT` consumer is `AggregateAlert`, which is purely
> informational. The A-4 validator
> (`assert_no_hard_trigger_on_soft_aggregate`) already covers
> `WeightedMean` AND `Mean`, and it continues to be the forward gate.
> **No new guards or property-level changes are required for the
> currently-shipped property set.**
>
> **Architecture:** preserve the two-buffer model. Reductions migrate
> behind the flag into a dedicated `ReductionSoft` `AccumulatorOpSession`
> that **reads from and writes to `output_vectors` only**. The leaf-init
> step is a `copy_buffer_to_buffer(values → output_vectors)` (replaces
> legacy Pass 4). Reduction ops are dispatched per depth bucket via
> `OrderBand(depth)`, ascending from leaves to root. `THRESH_BUF_OUTPUT`
> semantics and the legacy oracle harness are unaffected.
>
> **Kernel:** linear-loop gather (not workgroup shared memory) for
> determinism. One thread per parent slot. Same execution shape as the
> existing `COMBINE_SUM + SOURCE_SLOT_RANGE` path C-3 already uses.
>
> **Determinism:** GPU-to-GPU bit-identical (three runs); CPU-oracle
> abs-tolerance `1e-5` (looser than the workshop's measured `3e-6` to
> leave portability headroom). **Not** required to be CPU-bit-exact.
>
> **Scope:** WeightedMean + Mean only. Sum / Max / Min are C-6. The C-5
> kernel additions are deliberately narrow; reusing C-3/C-4
> infrastructure means Codex 5.5 can execute mechanically.

---

## 1. Existing exposure audit (re-verified for C-5)

This audit re-runs A-4 (`docs/workshop/soft_aggregate_tolerance_audit.md`)
against the post-C-4 codebase. The conclusion is unchanged.

### 1.1 Threshold registration paths

| Path | Where | Buffer | Semantic | Hard trigger? | Soft exposure |
|---|---|---|---|---|---|
| Fission per property | `threshold_registry.rs::walk` | `VALUES` | `FissionTrigger` | **Yes** | None |
| Decay → expiry (3 variants) | `push_decay_thresholds` | `VALUES` | `PropertyExpiry` | **Yes** | None |
| Fusion lineage | `push_fusion_lineage` | `VALUES` | `FusionTrigger` | **Yes** | None |
| Capability unlock | `push_capability_unlocks` | `VALUES` | `CapabilityUnlock` | Semi (durable state) | None |
| Scripted event trigger | `push_scripted_event_triggers` | `VALUES` | `ScriptedEventTrigger` | Variable | None |
| Velocity alert | `push_velocity_alerts` | `VALUES` | `VelocityAlert` | No (informational) | None |
| **Aggregate alert** | `push_aggregate_alerts` | **`OUTPUT`** | `AggregateAlert` | **No** (informational) | Reads post-reduction |

`AggregateAlert` remains the only path that reads post-reduction values,
and its downstream consumer set is still empty:

```
$ grep -rn "AggregateAlert\b" crates/simthing-driver/src crates/simthing-spec/src
(no matches)
```

No driver / spec consumer reads `AggregateAlertEvent` as a trigger. The
events flow to the delta log and to AI/observability hooks per the v6.5
design — informational only.

### 1.2 CPU-side reads of reduced output

The three CPU sites that drive structural decisions all read per-slot
`values_shadow`, not `ReducedField`:

| Site | Reads | Used for |
|---|---|---|
| `fission.rs::check_secondary` | `values_shadow[s * n_dims + col]` | Fission secondary condition |
| `overlay_lifecycle.rs::read_sub_field` | `shadow[base + col]` | `DissolveCondition::PropertyReaches/PropertyBelow` |
| `property_expiry.rs::cpu_decay_collect_node` | `values_shadow[slot * n_dims + col]` | `TowardZero` decay sweep |

`ReducedField` (CPU view of `output_vectors`) has no in-tree caller in
`crates/simthing-driver` or `crates/simthing-sim` outside tests.

### 1.3 Current production property usage

Surveyed by `grep -rn "reduction_override" crates/simthing-spec/src`:

| Path | Override | Reduction class |
|---|---|---|
| `simthing-spec::compile::capability` (category property builder) | `Some(ReductionRule::Max)` | **Exact** |

Every other production `SubFieldSpec` uses `reduction_override: None` and
inherits the role default:

| Role | Default rule | Class |
|---|---|---|
| `Amount` | `Mean` | **Soft** |
| `Velocity` | `Mean` | **Soft** |
| `Intensity` | `Max` | Exact |
| `Named(_)` | `Mean` | **Soft** |
| `Custom(_)` | `Mean` | **Soft** |

So `Mean` reduction is implicit on every default `Amount` / `Velocity` /
`Named` column. But — per §1.1 — no hard-trigger registration reads
`THRESH_BUF_OUTPUT` for any of those columns. The post-reduction value is
never structurally consequential under the current architecture.

### 1.4 Conclusion

**Zero unguarded hard-trigger paths feeding from soft-aggregate values
exist today.** The A-4 validator continues to be a forward gate. C-5's
migration of `WeightedMean` / `Mean` into AccumulatorOp does not change
the audit conclusion because:

1. Reduction output continues to live in `output_vectors` (or its C-5
   AccumulatorOp equivalent) — same buffer the existing
   `THRESH_BUF_OUTPUT` reads from.
2. The A-4 validator already covers `Mean` and `WeightedMean` per its
   `is_soft_aggregate_rule` predicate.
3. The current property set has no hard-trigger registrations on
   post-reduction columns.

C-5 is therefore not introducing a new error category. It is moving the
reduction computation from fixed-pipeline WGSL to AccumulatorOp WGSL.

---

## 2. Guard policy (no change to A-4)

A-4 specified, and C-5 preserves:

```
assert_no_hard_trigger_on_soft_aggregate(semantic, pid, sub_field, buffer, dim_reg)
  → Err(HardTriggerOnUnguardedSoftAggregate) when ALL of:
    1. is_hard_structural_trigger(semantic)
    2. buffer == THRESH_BUF_OUTPUT
    3. resolved_reduction(sub_field) ∈ {Mean, WeightedMean}
    4. soft_aggregate_guard ∈ {None, Some(Unguarded)}
```

C-5 changes nothing about this contract. The validator is already wired
into `ThresholdBuilder::push_*` for the four hard-trigger arms. C-5
**does not need to extend or relax** the validator.

**`Unguarded` policy clarification (Codex must enforce in docs only):**
`Some(SoftAggregateGuard::Unguarded)` is explicitly the
"informational-field-acknowledged" opt-out. It is **valid** for fields
that never feed a `THRESH_BUF_OUTPUT` hard-trigger registration —
which today is every soft-aggregate field. It is **invalid** for hard
triggers (the validator rejects). Codex should not change this; it is
A-4's accepted contract.

### 2.1 What C-5 must NOT do to the guard

- Do not weaken `assert_no_hard_trigger_on_soft_aggregate` to allow soft
  aggregates without guards.
- Do not add `Mean`/`WeightedMean` to any conservation-critical
  registration path without a guard.
- Do not skip the validator call when the threshold buffer is `VALUES`
  (the validator returns `Ok(())` in that case but the call site must
  remain so future PRs that change the buffer trigger the gate).

---

## 3. Default property recommendations

**Recommendation: no production property changes required for C-5.**

The reasoning chain:

1. The production property set uses `Mean` (default for Amount/Velocity/
   Named) extensively, but no `THRESH_BUF_OUTPUT` registration reads
   those columns for a hard trigger.
2. The one production property with an explicit `reduction_override` is
   the capability tree category property (`Max`) — exact, no guard
   needed.
3. The A-4 validator stays in force: any future property authoring that
   *does* register a hard trigger on a soft-aggregate post-reduction
   column will fail at session open with a clear error pointing at
   `SoftAggregateGuard`.

So C-5 does not need to ship any `Some(Quantized { step })` or
`Some(Hysteresis { band })` annotations on existing properties. The
forward-protection from A-4 is sufficient.

### 3.1 If a modder adds a new property with hard-trigger + soft-aggregate

The validator panic message (from A-4) tells them what to do:

```
soft-aggregate column registered as hard structural trigger without
guard: semantic=FissionTrigger, property=…, sub_field=…, rule=Mean.
Set SubFieldSpec::soft_aggregate_guard to
Some(Quantized { step }) or Some(Hysteresis { band })
— see docs/workshop/soft_aggregate_tolerance_audit.md.
```

Codex should add a short modder-facing note to
`docs/workshop/simthing_modder_object_guide.md` (or equivalent)
explaining the guard policy. Optional; primarily a docs polish item.

---

## 4. AccumulatorOp registration shape

### 4.1 Per-parent registration

One `AccumulatorOp` per (parent_slot, column, reduction-rule) tuple:

```rust
AccumulatorOp {
    source:  SourceSpec::SlotRange {
        start: first_child_slot,   // contiguous range — see §4.2
        count: n_children,
    },
    combine: CombineFn::WeightedMean { weight_col }   // or Mean
                                                       // or other exact in C-6,
    gate:    GateSpec::OrderBand(depth_band),
    scale:   ScaleSpec::Identity,
    consume: ConsumeMode::ResetTarget,   // overwrite parent — see §4.3
    targets: vec![(parent_slot, target_col)],
}
```

### 4.2 Contiguous child slots

`SourceSpec::SlotRange { start, count }` requires children to occupy
contiguous slot indices. This is **already true** in production because
`SlotAllocator` allocates child slots after their parent contiguously
(the topology-builder relies on this). Verify by reading the existing
`TopologyState` / `build_topology` invariants; C-5 inherits that
guarantee. If a fission/AddChild path ever produces non-contiguous
children, the C-5 planner must fall back per band (one op per child
with `SourceSpec::SlotValue` and a per-cell band), but this is a
defensive future case — not an immediate concern for C-5.

### 4.3 Why `ConsumeMode::ResetTarget` for reductions

Reductions overwrite the parent's value with the computed aggregate —
not add to it. C-4 added `ConsumeMode::AddToTarget` as the explicit
overlay-Add semantic; reductions use the symmetric `ResetTarget`
(already exists in the enum, shader implementation landed with C-4
per the erratum).

This is the clean mapping established by C-4's semantic cleanup. The
four-way write-target axis is:

| consume | shader effect |
|---|---|
| `None` / `Identity` default | `values[idx] = write_value` (assign) |
| `ResetTarget` | `values[idx] = write_value` (assign, explicit) |
| `AddToTarget` | `values[idx] += write_value` |
| `ScaleTarget` | `values[idx] *= write_value` |

Reductions use `ResetTarget` to make the semantic explicit (vs the
default's "assignment by happenstance"). Auditable.

### 4.4 OrderBand = depth

Parents reduce children at depth `d+1`; the result lands at depth `d`.
Dispatch bands in **leaf-first ascending order** (deepest depth → root):

```
Band 0  = leaves   (no reduction op; leaves already have output_vectors[slot]
                    from the init memcpy in §5.1)
Band 1  = parents whose children are at depth 0 (deepest internal)
...
Band k  = root
```

This is the legacy reduction order. The C-INF-1 `WorldAccumulatorRuntime`
and the C-4 multi-band dispatch pattern are reused; no new dispatch
infrastructure needed.

### 4.5 What goes in `combine_a..combine_d`

`WeightedMean { weight_col }` packs `weight_col: u32` into `combine_a`.
`Mean` uses no combine params. The encoder in
`crates/simthing-gpu/src/accumulator_op/encode.rs::encode_combine`
returns `(combine_kind, combine_a, combine_b, combine_c, combine_d)`;
extend the `match` arm for `CombineFn::WeightedMean { weight_col }` to
return `(combine_kind::WEIGHTED_MEAN, *weight_col, 0, 0, 0)` and add
`CombineFn::Mean → (combine_kind::MEAN, 0, 0, 0, 0)`.

---

## 5. Kernel design

### 5.1 Two-buffer model preserved; new `output_vectors` binding

C-5 adds **one new binding** to the AccumulatorOp shader and threads it
through `WorldAccumulatorRuntime` for the `ReductionSoft` session:

```wgsl
@group(0) @binding(8) var<storage, read_write> output_vectors: array<atomic<i32>>;
```

The ReductionSoft session's bind group routes binding 8 to the world's
`output_vectors` buffer. All other sessions (Intent, Threshold,
OverlayOrderBand) route binding 8 to a small zero-sized scratch buffer
or use a separate pipeline layout — see §8.

**Reduction kernel binding choice:** WeightedMean / Mean reductions
read from `output_vectors` (children, already populated by the leaf
init or by lower-band reductions) AND write to `output_vectors`
(parent). Both sides are `output_vectors`. The `values` binding is
unused by the reduction path.

### 5.2 Leaf init: `copy_buffer_to_buffer(values → output_vectors)`

Before dispatching any reduction band, encode:

```rust
encoder.copy_buffer_to_buffer(
    &state.values,           // source
    0,
    &state.output_vectors,   // dest
    0,
    n_slots * n_dims * 4,
);
```

This replaces legacy Pass 4 (the "snapshot values into output_vectors"
init). All slots (leaves and internal) start with their own per-slot
value; subsequent reduction bands overwrite internal nodes with reduced
values. The `previous_output_vectors` snapshot for threshold scan
remains a separate operation (encoded at Pass 0, unchanged).

### 5.3 Kernel: linear-loop gather (no workgroup shared memory)

For both `Mean` and `WeightedMean`, the kernel uses the same gather
pattern as the existing C-3 `COMBINE_SUM` path: one thread per parent
op, linear loop over children, single atomic store at the end. Add to
`crates/simthing-gpu/src/shaders/accumulator_op.wgsl::gather_value`:

```wgsl
if (op.combine_kind == COMBINE_MEAN && op.source_kind == SOURCE_SLOT_RANGE) {
    var sum = 0.0;
    for (var i: u32 = 0u; i < op.source_count; i = i + 1u) {
        sum = sum + atomic_read_f32_at(linear_idx(op.source_slot + i, op.source_col));
    }
    if (op.source_count == 0u) { return 0.0; }
    return sum / f32(op.source_count);
}

if (op.combine_kind == COMBINE_WEIGHTED_MEAN && op.source_kind == SOURCE_SLOT_RANGE) {
    let weight_col = op.combine_a;
    var weighted_sum = 0.0;
    var weight_total = 0.0;
    for (var i: u32 = 0u; i < op.source_count; i = i + 1u) {
        let child_slot = op.source_slot + i;
        let v = atomic_read_f32_at(linear_idx(child_slot, op.source_col));
        let w = atomic_read_f32_at(linear_idx(child_slot, weight_col));
        weighted_sum = weighted_sum + v * w;
        weight_total = weight_total + w;
    }
    if (weight_total == 0.0) { return 0.0; }
    return weighted_sum / weight_total;
}
```

**Important:** `atomic_read_f32_at` reads from the bound `values` array,
but for the reduction session we route binding 1 (`values`) to point at
`output_vectors`. From the kernel's perspective, "values" always means
"the bound input buffer" — for the reduction session, that's
`output_vectors`. See §8 for the binding-routing detail.

### 5.4 Write path: reuse C-4's `atomic_store_f32_at`

The reduction's `ConsumeMode::ResetTarget` reaches the C-4 single-writer
`write_target` switch (per the C-4 erratum §7.1):

```wgsl
case CONSUME_RESET_TARGET: {
    atomic_store_f32_at(idx, write_value);
}
```

The single-writer invariant holds for reductions because each parent
slot is written by exactly one op in its band (parents are unique).
The C-4 debug_assert in `plan_overlay_orderband` is overlay-specific;
C-5 adds a sibling assert in `plan_reduction_orderband`.

### 5.5 Why not workgroup shared memory?

The workshop measured ~3e-6 CPU-oracle drift on the parallel-reduction
WeightedMean shader. The drift comes from accumulation order divergence
(tree reduction vs linear sum). Linear-loop gather has the same order as
a CPU `for i in 0..n { sum += v[i] * w[i] }`, so the GPU and CPU produce
the **same f32** for the same inputs — when the kernel uses linear sum.

For C-5 production, linear sum is:
- Slower per-parent (O(n) work in one thread vs O(log n) in shared
  memory) — but the workload is small (typical parent has < 100
  children); the kernel launch overhead dominates anyway.
- Deterministic GPU-to-GPU (same f32 ops every run on any conformant
  IEEE 754 implementation).
- Comparable to CPU oracle within abs-tolerance `1e-5` (well above the
  workshop's measured 3e-6 drift, which itself was for a different
  workload; conservative bound).

The workshop's parallel reduction is the right optimization for the
single-property dense-aggregation case. If C-5 profiling on
`fission_stress` shows reduction dispatch is bottlenecked, swap to
workgroup-shared-memory tree reduction in a follow-up PR — but Codex
must not do it in C-5 (a future "tighter reduction" PR sits between
C-5 and S-4 for that).

---

## 6. Determinism policy

### 6.1 GPU-to-GPU bit-identical (REQUIRED)

Three runs on the same GPU device must produce **bit-identical**
`output_vectors`. Test asserts `f32::to_bits()` equality across runs.
The linear-loop kernel satisfies this trivially because:
- Loop bounds are uniform (same `source_count` each run)
- Memory layout is identical
- IEEE 754 ops are deterministic per-instruction on conformant
  hardware

### 6.2 CPU-oracle abs-tolerance (REQUIRED)

CPU oracle (the existing `cpu_reduce_oracle` in
`crates/simthing-gpu/src/reduction.rs`) uses linear left-to-right sum
for both `Mean` and `WeightedMean`. GPU kernel uses the same order.
Expected drift: 0 ULPs in the typical case; allow up to `1e-5` abs
tolerance to absorb any per-driver f32 rounding variance Codex hasn't
characterized yet.

The A-4 audit and ADR §Semantic scope class these as **soft aggregate**
regardless of measured drift. The "1e-5 vs 3e-6" tolerance argument is
not licensing C-5 to relax the A-4 guard — C-5's tolerance is the
test-side abs-epsilon for parity comparison, not a license for
unguarded hard-trigger registration.

### 6.3 Cross-GPU determinism (NOT REQUIRED, NOT TESTED)

C-5 does not assert bit-identical results across different GPU vendors
or drivers. The portability layer (wgpu) does not guarantee that, and
the production architecture does not require it. The audit policy says
"GPU-to-GPU deterministic" — meaning same-device-across-runs, not
cross-vendor.

### 6.4 Legacy vs AccumulatorOp parity

Test target: abs-tolerance `1e-5` between legacy `reduction.wgsl`
output and AccumulatorOp output for the same input. This is the
soft-aggregate parity bound. Failing this test means the kernel has a
real bug (wrong gather order, wrong weight handling, wrong slot
indexing); succeeding means the migration preserved tolerance class.

---

## 7. Production current-path tolerance confirmation

The ADR notes that the current production pipeline is already loose-
tolerance on `WeightedMean`. Codex must confirm this empirically before
the C-5 migration lands, via a new test added in the same PR:

```rust
// crates/simthing-sim/tests/c5_legacy_weighted_mean_oracle.rs
#[test]
fn legacy_weighted_mean_matches_cpu_oracle_within_1e_5() {
    // Build a small fixture with WeightedMean reduction on the inner-node
    // depth bucket. Run the legacy reduction pipeline. Read back
    // output_vectors. Compare against cpu_reduce_oracle. Assert
    // max_abs_error < 1e-5.
    //
    // Document the observed max_abs_error in an eprintln so the value
    // is captured in CI logs. The first time this runs, the printed
    // number is the empirical confirmation of the workshop's
    // ~3e-6 prediction in the production codebase.
}
```

Acceptance: the test passes. If it fails (production is tighter or
looser than expected), open an Opus review before the C-5 migration
proceeds — but do not block C-5 indefinitely; tighten the bound and
proceed.

---

## 8. Implementation file map (Codex 5.5-ready)

```
crates/simthing-core/src/accumulator_op.rs
  - CombineFn: NO change (Mean and WeightedMean variants exist)
  - AccumulatorOp::validate: extend the WeightedMean check to assert
    source is SlotRange (already there) AND the weight_col is < n_dims.
    Codex pulls n_dims from a per-context source (likely the encoder
    closure, not the AccumulatorOp itself; defer the actual n_dims
    bound check to the encoder).

crates/simthing-gpu/src/accumulator_op/types.rs
  - combine_kind module: add MEAN = <next-ordinal>, WEIGHTED_MEAN =
    <next-ordinal>. Verify ordinals match the corresponding CombineFn
    variants in simthing-core.

crates/simthing-gpu/src/accumulator_op/encode.rs
  - encode_combine: add arms for CombineFn::Mean (returns
    (MEAN, 0, 0, 0, 0)) and CombineFn::WeightedMean { weight_col }
    (returns (WEIGHTED_MEAN, weight_col, 0, 0, 0)).
  - validate_bootstrap_op / validate_no_contention: NO change — the
    reduction ops are band-gated; existing contention logic already
    treats one-writer-per-band-per-cell as the invariant. The C-5
    planner enforces single-writer the same way the C-4 planner does.

crates/simthing-gpu/src/shaders/accumulator_op.wgsl
  - Add COMBINE_MEAN and COMBINE_WEIGHTED_MEAN constants matching the
    ordinals in types.rs.
  - Extend `gather_value` per §5.3 (two new branches: Mean and
    WeightedMean).
  - The write path is already covered by the C-4 `write_target` switch
    on `consume` — reductions use `CONSUME_RESET_TARGET` which routes
    to `atomic_store_f32_at`. No write-target shader changes needed.
  - Add a comment block above the WeightedMean branch documenting the
    linear-loop choice and the C-5 design memo reference.

crates/simthing-gpu/src/accumulator_op/runtime.rs
  - Add `reduction_soft_session: Option<AccumulatorOpSession>` field
    (sibling of `intent_session`, `threshold_session`, `overlay_session`).
  - Add ensure_/take_/restore_/disable_/clear_reduction_soft methods
    matching the existing per-family pattern.
  - The session is constructed with `new_attached_to_buffer(ctx, n_slots,
    n_dims, output_vectors_buffer)` — a new constructor that points the
    session's bound `values` array at `output_vectors` rather than the
    world's `values`. Add this constructor in session.rs.
  - The `reduction_soft_ops: OpSetHandle` field already exists from
    C-INF-1 — wire it active when the session uploads ops.

crates/simthing-gpu/src/accumulator_op/session.rs
  - Add `AccumulatorOpSession::new_attached_to_buffer(ctx, n_slots, n_dims,
    bound_values: &Buffer) -> Self`. Same shape as `new_attached` but
    uses `bound_values` for binding 1 instead of allocating its own
    values buffer. The previous_values, emissions, etc. bindings get
    small placeholder buffers — they're unused by the reduction
    dispatch but the bind group layout requires them.
  - Reuse the C-3/C-4 single-submit encoder helpers for dispatch.

crates/simthing-gpu/src/reduction_orderband.rs   ← NEW
  - pub struct ReductionOrderBandPlan { ops: Vec<AccumulatorOpGpu>,
                                         n_bands: u32 }
  - pub fn plan_reduction_orderband(
        topology: &TopologyState,
        column_rules: &[ColumnRuleDescriptor],   // per-column reduction rule
        n_dims: u32,
    ) -> ReductionOrderBandPlan
  - Walk depth buckets from deepest to root. For each (parent, column)
    where column_rules[col] ∈ {Mean, WeightedMean}, emit one
    AccumulatorOp per §4.1 with band = depth.
  - Skip columns with exact rules (Sum/Max/Min/First) — those go
    through C-6.
  - debug_assert that no two ops in the same band target the same
    (slot, col).
  - Re-export from lib.rs.

crates/simthing-sim/src/boundary.rs
  - PipelineFlags: add `use_accumulator_reduction_soft: bool` (default
    false). The flag-clearing pattern from C-1/C-2/C-3/C-4 applies:
    flag-off calls `runtime.clear_reduction_soft()`.
  - No reduction-revision counter needed — the topology + column rules
    only change when WorldGpuState::rebuild runs (registry growth /
    slot grow), at which point the runtime is already torn down. The
    reduction plan is recomputed at the next gpu_sync.

crates/simthing-sim/src/gpu_sync.rs
  - When `use_accumulator_reduction_soft` is on AND the topology has
    been rebuilt this sync OR any column rule changed:
    1. Build column rule descriptors (existing helper).
    2. Call plan_reduction_orderband(topology, column_rules, n_dims).
    3. runtime.upload_reduction_soft_ops(ops, n_bands).
  - When flag is off: clear_reduction_soft().

crates/simthing-feeder/src/dispatcher.rs (or wherever per-tick dispatch lives)
  - Before the legacy reduction dispatch: if the AccumulatorOp
    reduction session is active, encode:
       (a) copy_buffer_to_buffer(values → output_vectors)    // leaf init
       (b) session.encode_dispatch_bands_into(encoder, 0..n_bands)
    Skip the legacy reduction passes when the flag is on. Single
    submit per tick preserved (C-1 pattern).

crates/simthing-sim/src/legacy_oracle.rs
  - Extend the harness with a reduction-family oracle: drive the
    legacy reduction passes against the same world state and compare
    output_vectors via the abs-tolerance check. The harness already
    has the comparison primitives from C-INF-2.

docs/accumulator_op_v2_production_plan.md
  - PR C-5: status → "Design landed; implementation PR follows"; link
    this memo.

docs/design_v7.md
  - §4.2: add `use_accumulator_reduction_soft: bool` to the flag table.
  - §4.3: Passes 4–6 description gets a "C-5 migration of soft
    reductions landed; sunset S-4 pending C-6" note.

docs/workshop/workshop_current_state.md
  - §2 (Landed) — add C-5 once it merges.
  - §6 — add this memo to active workshop documents.
  - §5 (Tests) — add C-5 test counts.

docs/agents.md
  - If it documents the reduction pass, add a C-5 note.
```

### 8.1 Bind group layout — handling the new `output_vectors` binding

The C-INF-1 runtime already supports multiple per-family sessions with
distinct bind groups. The cleanest path for C-5:

1. The AccumulatorOp shader gets binding 8 = `output_vectors`.
2. Non-reduction sessions (Intent, Threshold, Overlay) get a small
   read-only zero buffer at binding 8. The shader path for those
   families never touches it.
3. The reduction session gets the world's `output_vectors` buffer at
   binding 8 AND has its binding 1 (`values`) point at
   `output_vectors` too (so the existing `atomic_read_f32_at` helper
   reads from the right buffer).

Actually — wait. The simpler design is: **don't add binding 8 at all.**
Route the reduction session's binding 1 to `output_vectors`. The shader
sees `values` as the input/output buffer it always saw, just bound to
a different physical buffer for this session. The kernel doesn't need
to know it's writing reductions vs values; the `consume` switch handles
that.

The leaf-init memcpy ensures `output_vectors` starts the tick with leaf
values copied in from `values`. The reduction dispatch reads/writes the
same buffer (`output_vectors` via binding 1). Other sessions in the
same tick keep binding 1 = `values` (their own buffer). No cross-talk
because the sessions don't share bind groups within a single dispatch.

**Final decision: no new binding. Per-session bind groups already
isolate buffer choice. Codex follows the existing per-session pattern.**

Update §5.1 above: scrap the binding-8 plan. Reduction session binds
`output_vectors` to binding 1.

---

## 9. Test plan (concrete cases for Codex)

### 9.1 Audit / guard validator (reuses A-4 infrastructure)

| Test | Behavior |
|---|---|
| `c5_assert_no_hard_trigger_blocks_weighted_mean_without_guard` | Same scenario as A-4's existing test, just confirms WeightedMean is in the soft class. |
| `c5_assert_no_hard_trigger_allows_weighted_mean_with_quantized` | Add `SoftAggregateGuard::Quantized { step: 0.01 }` to the field; validator returns Ok. |
| `c5_assert_no_hard_trigger_allows_weighted_mean_with_hysteresis` | Same with `Hysteresis { band: 0.05 }`. |
| `c5_unguarded_allowed_for_aggregate_alert_path` | Soft-aggregate column registered as `AggregateAlert` (not hard trigger) on `THRESH_BUF_OUTPUT` with `Unguarded` — validator returns Ok because `is_hard_structural_trigger(AggregateAlert) == false`. |

### 9.2 Production current-path tolerance

| Test | What |
|---|---|
| `c5_legacy_weighted_mean_matches_cpu_oracle_within_1e_5` | Per §7. Records observed max_abs_error to CI logs. |
| `c5_legacy_mean_matches_cpu_oracle_within_1e_5` | Same for Mean. |

### 9.3 GPU determinism

| Test | What |
|---|---|
| `c5_accumulator_weighted_mean_three_runs_bit_identical` | Run the AccumulatorOp reduction three times; assert `f32::to_bits()` equality across all three. |
| `c5_accumulator_mean_three_runs_bit_identical` | Same for Mean. |

### 9.4 Legacy vs AccumulatorOp parity

| Test | What |
|---|---|
| `c5_weighted_mean_legacy_vs_accumulator_within_1e_5` | Run both paths on identical input; assert max_abs_error < 1e-5. **Not** bit-exact. |
| `c5_mean_legacy_vs_accumulator_within_1e_5` | Same for Mean. |

### 9.5 Production-scale stress

| Test | What |
|---|---|
| `c5_fission_stress_reduction_three_runs_identical` | 20k-slot `fission_stress` scenario, 100 ticks, AccumulatorOp reduction flag on. Three full runs. Final `output_vectors` bit-identical across runs. |
| `c5_multi_depth_bucket_dispatch_ordering` | Tree with 4+ depth buckets; deepest band dispatches first; root last; final root value matches legacy bit-by-bit-on-determinism / within 1e-5 vs CPU oracle. |
| `c5_weighted_mean_zero_weight_returns_zero` | All children have weight=0; parent value = 0 (matches CPU oracle's divide-by-zero handling). |

### 9.6 Guard integration

| Test | What |
|---|---|
| `c5_threshold_registration_on_unguarded_weighted_mean_output_panics` | Build a property with `Mean` reduction and a `FissionThreshold` on its `Amount` column; register on `THRESH_BUF_OUTPUT`; expect the A-4 validator panic. |
| `c5_threshold_registration_on_guarded_weighted_mean_output_succeeds` | Same as above with `Hysteresis { band: 0.05 }`; registration succeeds. |

### 9.7 B-4 world summary compatibility

| Test | What |
|---|---|
| `c5_world_summary_matches_full_values_after_weighted_mean_reduction` | After a reduction tick, `WorldSummaryRuntime` checksum over `values` matches CPU oracle. (Verifies reduction's `output_vectors` writes don't leak into `values` and corrupt the summary.) |

### 9.8 Combined pipeline

| Test | What |
|---|---|
| `c5_combined_c1_c2_c4_c5_all_flags_on_no_panic` | All four migration flags on for 100 ticks on `fission_stress`. Asserts no panic, summary still valid, threshold events flow correctly. **Does not** assert bit-exact vs legacy for soft reductions (per §6.4). |

### 9.9 GPU-resident path

| Test | What |
|---|---|
| `c5_production_path_no_cpu_mediated_reduction` | Drive 50 ticks with the reduction flag on; assert that `cpu_reduce_oracle` is NOT called from any production path (only from test oracle). Use a probe counter or a `#[cfg(test)]` instrumentation hook. |

### 9.10 Run-family-oracle harness

Reuse C-INF-2 / `legacy_oracle.rs` for all legacy-vs-AccumulatorOp
parity tests above. C-5 extends the harness with a reduction-family
adapter rather than rolling bespoke comparison code.

---

## 10. Composer/Codex implementation boundaries

For Codex 5.5:

```
Pivot posture:
  AccumulatorOp reduction is the intended production path.
  Legacy reduction passes (Pass 4-6) are oracle/flag-off only until S-4.
  This PR migrates WeightedMean + Mean soft reductions behind a
  default-false flag.
  It MUST NOT optimize, extend, or add new semantics to legacy
  reduction.

Sunset target:
  S-4 — delete legacy reduction passes after C-5 + C-6 default-on
  validation + 7 days CI green.

GPU residency:
  Production reduction must execute in accumulator_op.wgsl against
  WorldGpuState.output_vectors (via reduction session's binding 1
  routed to output_vectors per §8.1).
  CPU may compile registrations at boundary and read back for oracle /
  test only. No CPU-mediated production reduction.

Legacy interaction allowed:
  Oracle / parity tests only.

Legacy interaction forbidden:
  no new features · no optimization · no semantic expansion.

Acceptance gates (per §9):
  [ ] All §9.1 audit/guard tests pass
  [ ] §9.2 legacy tolerance confirmation captured in CI logs
  [ ] All §9.3 GPU determinism tests pass three consecutive runs
  [ ] All §9.4 legacy-vs-AccumulatorOp parity tests within 1e-5 abs
  [ ] §9.5 production-scale stress tests pass
  [ ] §9.6 guard integration tests pass
  [ ] §9.7 world summary compatibility passes
  [ ] §9.8 combined pipeline test passes (no bit-exactness claim)
  [ ] §9.9 GPU-residency probe asserts no CPU-mediated reduction
  [ ] Zero warnings, debug + release clean
  [ ] design_v7.md §4.2/§4.3 updated
  [ ] production plan C-5 entry status set to Landed
  [ ] workshop_current_state.md §2/§5 updated
  [ ] PR description fills the pivot-forward handoff template
```

---

## 11. S-4 sunset implications

After C-5 merges AND C-6 (Sum/Max/Min) merges AND both default on AND
CI is green for 7 days, S-4 becomes mechanical:

```
S-4 PR checklist:
- [ ] use_accumulator_reduction_soft + use_accumulator_reduction_exact
      both defaulted to true
- [ ] CI green at flag=on for 7+ days
- [ ] All parity tests pass with both flags on
- [ ] Delete:
      - crates/simthing-gpu/src/shaders/reduction.wgsl
      - crates/simthing-gpu/src/reduction.rs production code
        (cpu_reduce_oracle kept for tests)
      - Pipelines: reduction_pipeline + bind group layout
      - WorldGpuState fields: column_rules buffer if not used by
        another path (verify before deletion)
      - gpu_sync.rs: legacy reduction upload branch
- [ ] Update design_v7.md §4.3 to remove Passes 4–6 entry
- [ ] Add SUPERSEDED annotation to design_v6.md §10 Pass 4–6 entries
```

C-5 does NOT do S-4 deletion. C-5's job is to make S-4 mechanical for
the soft-reduction half. C-6 finishes the exact-reduction half.

### 11.1 C-6 prep (shared infrastructure, do NOT implement)

`plan_reduction_orderband` should be designed in C-5 to easily extend
for C-6's exact reductions. The signature already takes
`column_rules: &[ColumnRuleDescriptor]` — adding Sum/Max/Min cases in
the planner is one match-arm extension per rule. The kernel similarly
needs new combine-kind arms in `gather_value`. C-5 should leave
explicit `TODO(C-6)` markers where Sum/Max/Min would extend the
existing structure, but **must not** implement them.

---

## 12. Non-goals (explicit)

C-5 does NOT:

- Add `CombineFn::Sum`/`Max`/`Min` reduction kernel paths (C-6)
- Touch velocity integration (C-7)
- Touch EML / transfer / conjunctive emission (C-8 / E-family)
- Default the reduction flag to true (separate PR)
- Implement S-4 deletion
- Optimize the kernel with workgroup shared memory (see §5.5 — future
  optimization PR between C-5 and S-4 if profiling demands)
- Modify A-4's `SoftAggregateGuard` or its enforcement
- Modify B-4's `SlotSummaryGpu` shape
- Add per-property guards to the production property set (per §3, no
  changes needed for the current properties)
- Collapse the two-buffer model (`values` vs `output_vectors`) — that
  is a separate v7-aligned refactor, post-C-7 at earliest

---

## 13. Sign-off checklist (this memo)

- [x] Read pivot-forward policy + aligned recommendations to it
- [x] Re-verified A-4 audit against post-C-4 codebase (§1)
- [x] Confirmed `Mean` is default for Amount/Velocity/Named (§1.3)
- [x] Confirmed no production property uses explicit `WeightedMean` (§1.3)
- [x] Confirmed `AggregateAlert` is the only `THRESH_BUF_OUTPUT` path
      and has no production consumer (§1.1)
- [x] Guard policy preserved without modification (§2)
- [x] Default property recommendation: no changes needed (§3)
- [x] AccumulatorOp registration shape specified (§4)
- [x] Kernel design (linear-loop, no shared memory) justified (§5)
- [x] Determinism policy explicit (§6)
- [x] Tolerance confirmation plan specified (§7)
- [x] Implementation file map Codex-ready (§8)
- [x] Test plan with 18 named cases (§9)
- [x] Composer/Codex boundaries + acceptance gates (§10)
- [x] S-4 sunset implications + C-6 prep notes (§11)
- [x] Non-goals enumerated (§12)
- [ ] Human sign-off (this PR requests it)

---

## References

- `docs/adr_accumulator_op_v2.md` — semantic scope, soft aggregate policy
- `docs/design_v7.md` §3 (SoftAggregateGuard), §4 (pipeline)
- `docs/accumulator_op_v2_production_plan.md` PR C-5
- `docs/workshop/pivot_forward_implementation_policy.md`
- `docs/workshop/workshop_current_state.md`
- `docs/workshop/soft_aggregate_tolerance_audit.md` (A-4)
- `docs/workshop/c4_overlay_orderband_compiler_design.md` (C-4)
- `crates/simthing-sim/src/threshold_registry.rs` — A-4 validator
- `crates/simthing-gpu/src/world_state.rs` — `output_vectors` buffer
- `crates/simthing-gpu/src/shaders/reduction.wgsl` — legacy reduction (oracle)
- `crates/simthing-gpu/src/shaders/accumulator_op.wgsl` — C-5 target kernel
- `crates/simthing-gpu/src/accumulator_op/runtime.rs` — runtime envelope
- `crates/simthing-gpu/src/reduction.rs` — `cpu_reduce_oracle`, `TopologyState`,
  `ColumnRuleDescriptor`
