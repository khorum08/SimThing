# Resource Flow Model — Feasibility Evaluation

**For:** Opus handoff response  
**From:** Opus evaluation, 2026-05-26  
**Status:** Feasibility evaluation (no implementation), grounded in
`crates/simthing-core/src/accumulator_op.rs`, `property.rs`, `registry.rs`,
`reduction.rs`; `crates/simthing-sim/src/boundary.rs`, `threshold_registry.rs`;
`crates/simthing-gpu/src/shaders/accumulator_op.wgsl`,
`reduction_orderband.rs`, `transfer_accumulator.rs`,
`accumulator_op/input_list_table.rs`; and the workshop kernel
`crates/simthing-workshop/src/transfer_contention_gpu.wgsl`. All claims about
landed semantics are verified against source as of `master` on the local
worktree.  
**Target file:** `docs/workshop/resource_flow_feasibility_opus.md`  
**Verdict (one line):** PROCEED TO ADR WITH MODIFICATIONS. D-1 substantially
eliminated for the continuous-flow case; preserved at narrowed scope for the
discrete-transaction case. The proposition is constitutionally compatible.

**Successor:** `docs/workshop/resource_flow_adr_shaping.md` — the ADR-shaping
draft that supersedes this document. This document is retained for traceability
of the feasibility analysis and grounding evidence.

---

## 0. Scope reminder

The proposition replaces *per-tick pull contention* — many requesters
debiting one pool at tick time — with *per-tick parallel rate reduction* +
*per-boundary budget allocation*. The continuous-flow case is the one D-1
was scoped to solve. The proposition argues D-1 dissolves because the
contention regime it targets no longer exists in the steady state.

I confirm the architectural reframe. I disagree that D-1 fully disappears.
Discrete transactions (construction commit, treaty payments, emergency
spend, mass policy action) remain in the explicit-transfer regime and
preserve a narrower D-1 case. The recommendation is to **rescope** D-1, not
delete it.

---

## 1. Invariant audit

Every relevant invariant from `docs/invariants.md` walked explicitly. PASS
= no conflict. CONCERN = needs explicit clarification before ADR. NO PASS
= the proposition violates the invariant as stated.

| Invariant | Status | Note |
|---|---|---|
| `simthing-sim never knows recipe semantics` | **PASS** | The allocator solver belongs in `simthing-driver` (session assembly). It reads `WorldGpuState::read_output_vectors()` and produces `AccumulatorOp` registrations via existing builders. `simthing-sim` continues to see only `AccumulatorOp` structs. No recipe strings, costs, or economic types cross the boundary. |
| `SubtractFromSource is the only transfer mechanism` | **CONCERN — needs reframing** | The allocator's *inbound Flow writes* are not transfers in the source-debit sense; they are allocator disbursements writing to a participant's own Flow column. The natural `ConsumeMode` is `ResetTarget` (overwrite this boundary's Flow value) or `AddToTarget` (accumulate disbursements across the same band). Conservation in this path is enforced by the allocator's pool-level accounting (Σ disbursed ≤ root surplus), not by a per-op `SubtractFromSource` invariant. The ADR should state explicitly: *"`SubtractFromSource` is the transfer mechanism for source-debit transactions; allocator disbursements use `ResetTarget`/`AddToTarget` on independent target slots and conservation is enforced by allocator pool accounting."* This is a clarification, not a relaxation — no two-overlay hack is introduced. |
| `SoftAggregateGuard on WeightedMean columns feeding thresholds` | **PASS** | Flow reduces via `ReductionRule::Sum` (exact). Need is integrated locally per slot via `IntegrateWithClamp`, not reduced. Debt-band thresholds on Need read `THRESH_BUF_VALUES`. The `assert_no_hard_trigger_on_soft_aggregate` validator in `simthing-sim/src/threshold_registry.rs` returns `Ok(())` for both paths. No guard required. |
| `No hardcoded column indices` | **PASS** | `NumCountSource::Column { property_id, role }` resolves via `PropertyColumnRange::col_for_role(role, layout)` at session sync, never at registration construction time. The pattern matches existing `VelocityAlertRegistration` resolution in `ThresholdBuilder::push_velocity_alerts`. |
| `Persistent session per session lifetime` | **PASS** | The allocator does not create or tear down `AccumulatorOpSession`. It writes new ops to the existing `op_buffer` via the existing `WorldGpuState` per-runtime upload paths (see `sync_accumulator_transfer_session` in `boundary.rs`). The persistent-buffer residency model is preserved. |
| `Stride is computed, never stored` / `Sub-field roles are named, not positional` / `col_for_role` is sole authority for global column arithmetic | **PASS** | Flow and Need fit `SubFieldRole::Named` without disturbing any of these. See §3. |
| `Exact operations never use soft-aggregate combine fns` | **PASS** | Flow reduction uses `Sum`. Need integration uses `IntegrateWithClamp`. Recipe production uses `MinAcrossInputs` + `SubtractFromAllInputs`. None of these are soft-aggregate. |
| `Emission records are produced for every GPU-resolved emission` | **PASS** | Debt-band emission of Need crossings already lands compact `EmissionRecord` via `maybe_emit_event` in `accumulator_op.wgsl` (CONSUME_EMIT_EVENT path). Recipe production via `MinAcrossInputs` writes to a target with `SubtractFromAllInputs`; the emit_count is the target write value, which the existing compact log machinery can capture by adding an `EmitEvent` consume on the same op (already supported — see the `targets.len() == 0` allowance in `AccumulatorOp::validate` for `OrderBand + EmitEvent`). |
| `Old pass code never deleted without green CI at flag=on` / `design_v7.md §4 updated by migration PRs` | **PASS** | This proposition adds new ops; it does not delete any. |

**Audit result:** one concern about the wording of the `SubtractFromSource`
invariant. No fundamental blockers.

---

## 2. Conservation verification

The proposition makes two distinct conservation claims, which must be
evaluated separately.

### 2.1 Per-recipe conservation (Pattern 3): EXACT — confirmed

Claim: `Σ_j ΔNeed_j + emit_count × Σ_j c_j = 0` per recipe invocation.

Verified directly against the WGSL kernel in `accumulator_op.wgsl`:

1. `gather_min_across_inputs` computes
   `emit_count = floor(min_j(available_j / unit_cost_j))` *before*
   consumption.
2. `apply_consume` with `CONSUME_SUBTRACT_FROM_ALL_INPUTS` iterates inputs
   and subtracts `emit_count × unit_cost_j` per channel, clamped to ≥0.0.
3. By construction of step 1, `emit_count × unit_cost_j ≤ available_j` for
   every input. The `max(0.0)` clamp in step 2 is defensive — it can never
   fire under a correctly-planned op.
4. `plan_transfer_ops` in `transfer_accumulator.rs` rejects same-band
   consumed-input contention (`ContendedConsumedInput` error). Within a
   band, only one op may debit any (slot, col), so the available value
   read at gather time is identical to the value subtracted at consume
   time.

**Conclusion:** conservation is exact in the landed kernel, not just in
spec, for correctly-planned ops. The planner is the conservation enforcer;
the kernel's clamp is forward-defense only.

### 2.2 Global per-tick conservation: NOT REQUIRED — and not desirable

Claim under evaluation: `Σ_{i ∈ P} Flow_i × Δt = 0` per tick across the
whole arena.

**This invariant should not hold and should not be required.** Flow is a
*rate signal*, not a *stock*. The conserved quantity is Need. Three
arguments:

1. **The arena can be in surplus or deficit.** If every supplier has +Flow
   and every demander has 0 Flow (because no allocation has happened yet),
   `Σ Flow > 0`. That is a meaningful signal — the root reduces it and
   the allocator sees "this arena produces faster than it consumes". A
   global zero-sum constraint would erase that signal.
2. **The allocator is the budget-balancer.** It reads root Flow, computes
   the disbursable surplus, and writes inbound Flow to deficit
   participants such that `Σ disbursed ≤ root surplus`. This makes the
   allocator the locus of the budget invariant — exactly where it
   should be, since it is the only step with cross-participant visibility.
3. **Conservation already lives at the Need layer.** Recipes consume
   Need across inputs and produce Need at the output (or, where the
   recipe terminates, produce discrete units). The cross-channel Σ ΔNeed
   bookkeeping in §2.1 is the only conservation the system needs at the
   substrate level.

**Recommendation:** the ADR should state explicitly that *Flow is a rate
signal whose conservation is enforced per-boundary by the allocator's
pool accounting; the per-tick global Flow sum is not required to be
zero*. This is the conceptual hinge that justifies dropping D-1's
tick-time allocator.

---

## 3. Sub-field role assessment: `Flow` and `Need`

The handoff asks whether `Flow` and `Need` should be first-class
`SubFieldRole` variants or `Named` values.

**Recommendation: `Named`. Not first-class.**

Rationale grounded in the actual code:

- `SubFieldRole` in `crates/simthing-core/src/property.rs` is
  `{ Amount, Velocity, Intensity, Named(String), Custom(String) }`. The
  three first-class variants exist because they have *behavioral
  privilege*: Amount is integrated, Velocity governs Amount,
  Intensity has its own `IntensityBehavior` evolution path. They are
  matched exhaustively in `evaluate.rs`, `reduction.rs`
  (`default_for_role`), and the GPU encoder.
- Flow and Need do *not* have such privilege under the proposition. Flow
  is a Sum-reduced rate signal — structurally identical to any other
  Sum-reduced Named column. Need is a `governed_by`-integrated stock —
  structurally identical to Amount when configured the same way. Their
  semantics are configured through existing `SubFieldSpec` fields
  (`reduction_override = Some(Sum)`, `governed_by = Some(Named("flow"))`),
  not through enum-level distinction.
- Adding first-class variants touches every exhaustive `match` on
  `SubFieldRole`, including the C-7 velocity migration's
  `IntegrateWithClamp` planner, the C-5 reduction planner, the boundary
  hook context, the spec serialization layer, and every modder-authored
  property. The cost is high and the benefit is zero — `Named` is the
  established route for designer-defined semantics.
- The role-default reduction for `Named` is `Mean`. Resource properties
  set `reduction_override = Some(Sum)` explicitly on the Flow sub-field.
  This is the same pattern that `population.headcount` would use, and
  is already supported in `SubFieldSpec::resolved_reduction()` (see
  `reduction.rs` test `override_resolves_via_subfield_spec`).

**The `Flow` integration into `Need` reuses `IntegrateWithClamp` (C-7) with
different parameters, not a new combine function.** Specifically: the
governing sub-field is `Named("flow")` instead of `Velocity`, and the
governed sub-field is `Named("need")` instead of `Amount`. The combine
operates on (governed_offset, governing_offset, dt, clamp_bounds) — none
of these are role-name-dependent. C-7 currently compiles only
Amount-governed-by-Velocity pairs; the proposition requires the C-7
planner to be extended to compile *any* `governed_by` pair, which is a
small generalization of the existing path. No kernel change needed.

---

## 4. Registration feasibility

### 4.1 Resource property registration

`DimensionRegistry::register(prop: SimProperty)` is the only registration
entry point. It does not need to grow a `register_resource_property`
variant. The spec layer's *builder* — likely
`simthing-spec/src/spec/property.rs` — is where the convention lives:
construct the `SubFieldSpec` for the Flow sub-field with
`reduction_override = Some(ReductionRule::Sum)` and the Need sub-field
with `governed_by = Some(SubFieldRole::Named("flow".into()))`. This keeps
the runtime `DimensionRegistry` API surface unchanged.

The reduction planner `plan_reduction_orderband` in
`crates/simthing-gpu/src/reduction_orderband.rs` already dispatches
`ReductionRule::Sum` to `combine_kind::SUM` via the kernel's
`COMBINE_SUM` path. No reduction-planner change required.

Collision concern: none. Multiple properties may set
`reduction_override = Some(Sum)` for unrelated reasons (e.g.
`population.headcount`). The planner treats them identically and writes
to the parent's `output_vectors` column. No name-based filtering exists
or is needed.

### 4.2 `accumulator_spec` on `SubFieldSpec`

Important note: **the `accumulator_spec` field that `design_v7.md` §3
calls "NEW in v7" is not yet in the landed source**. `SubFieldSpec` in
`crates/simthing-core/src/property.rs` currently has: role, width, clamp,
velocity_max, default, display_name, display_range, governed_by,
reduction_override, soft_aggregate_guard. No `accumulator_spec`.

The proposition's `NeedSpec` could land in two ways:

1. **As `accumulator_spec: Option<AccumulatorSpec>` on `SubFieldSpec`,**
   which is the path `design_v7.md` §3 already plans. The
   `AccumulatorSpec` would carry `NeedSpec` for Need-role sub-fields and
   nothing for Flow-role sub-fields.
2. **As a sibling registry,** e.g.
   `simthing-driver/src/resource_arena_registry.rs`, keyed by
   (`property_id`, `SubFieldRole`). This isolates resource semantics
   from the core property layout.

I lean to (1) — the `accumulator_spec` field is already planned in
`design_v7.md` and other ADR-track plans (B-4 summary integration,
C-8d emission) implicitly assume `SubFieldSpec` is the place to hang
GPU-affecting per-sub-field metadata. Option (2) doubles the lookup
machinery for marginal isolation gain.

### 4.3 `num_count_source` resolution at boundary

`NumCountSource::Column { property_id, role }` resolves to a global column
index via `dim_reg.column_range(property_id).col_for_role(&role, layout)`,
exactly the pattern used by `ThresholdBuilder::push_velocity_alerts`,
`push_capability_unlocks`, and the C-8b intensity EML setup. Resolution
must happen *at session sync* (boundary step 13), not at registration
construction, because slot allocation can change between boundaries.

Re-registration when `num_count` changes: the natural place is the
boundary hook (`BoundaryHookContext`), which runs after GPU value
readback and before structural mutations. The hook reads the current
`num_count` from `coord.shadow` (the post-readback CPU shadow), computes
`new_threshold = -((num_count - 1) * unit_cost)`, and emits a
`BoundaryRequest` (or a direct registration update through a future
session API). This is one step after step 2 in the boundary sequence
documented in `boundary.rs` lines 11–32.

Reading `num_count` from the GPU summary buffer at boundary time is *not*
sufficient — the summary is a checksum, not a value. The path is the
existing `coord.shadow = state.read_values()` at step 0 of the boundary,
which is already running today. No new readback channel required.

---

## 5. Allocator solver shape

Three open questions.

### 5.1 Reading root reduced Flow

`WorldGpuState::read_output_vectors()` is the existing readback path.
`BoundaryProtocol::read_reduced_field` already exposes it via
`ReducedField`. The allocator at boundary time would call this once,
locate the root slot (slot 0 by convention — `SlotAllocator` assigns
monotonically and World is the root), and read column
`column_range(flow_property).col_for_role(Named("flow"), layout)` from
that slot.

This is a clean read path. No changes to `output_vectors` exposure
required. The reduction must have completed before the allocator runs,
which is guaranteed because reduction is part of the per-tick pipeline
and the boundary executes after the last tick.

### 5.2 Generating N registrations at boundary

The existing `upload_ops` / `boundary_sync` path absorbs new
registrations every boundary. Per-boundary regeneration of N inbound-Flow
registrations is well within the envelope demonstrated by the C-8c
transfer planner and the threshold rebuild path. Order of magnitude:

- `AccumulatorOpGpu` is approximately 92 bytes (24 u32 fields + 2 padding).
- 1000 deficit participants × 16 arenas × 92 bytes ≈ 1.5 MB per boundary.
- `WorldGpuState::sync_gpu_buffers` already moves ~1.0 MB on
  `fission_stress`-style boundaries with full threshold rebuild.

The new upload is comparable in scale to existing per-boundary traffic.
The C-8c `AccumulatorInputListTable` upload skip-on-unchanged pattern
(`uploaded_generation == self.generation`) is the right model: cache the
allocator's prior decisions and skip the upload when the arena's deficit
shape is unchanged.

### 5.3 Participant list residency

Two options.

**Option A — Walk the tree at boundary time:** Find all SimThings carrying
the resource property by traversing `root.children` recursively. Cost:
O(|tree|) per arena per boundary.

**Option B — Maintain a side-structure at session sync:** Add
`arena_participants: HashMap<SimPropertyId, Vec<SlotId>>` to
`DimensionRegistry` (or to a sibling resource-arena registry, per §4.2).
Updated only when structural mutations attach/detach the resource
property — boundary step 10 (`apply_structural_mutations`). Cost: O(|Δ
participants|) per boundary.

I recommend **Option B**. `DimensionRegistry::column_owners` already
records property-column ownership; an analogous arena-participant list
is the natural complement. Tree walks are repeated work; the session
already maintains structural caches (`cached_topology_state` for
reduction). Maintaining one more list at the same update points is
cheap and aligns with the existing pattern.

> **Successor note:** §5.3 above was **reversed** by the ADR-shaping draft
> after Gemini synthesis. Explicit enrollment via spec-author selectors
> (not implicit by property possession) is now the constitutional rule.
> See `resource_flow_adr_shaping.md` §§3.1, 4.1, 5.2.

Coherence with the session model: the participant list is session-scoped
(reset at session open), updated at boundary structural mutations
(same trigger as topology rebuild), and read at boundary allocator
time. No persistence required beyond the session.

---

## 6. Conjunctive recipe confirmation

The proposition claims Pattern 3 (`ConjunctiveCrossing + MinAcrossInputs
+ SubtractFromAllInputs`) is sufficient for multi-resource dependency.

**Confirmed.** The mechanism is landed (C-8c block) and behaves as
specified. Three sub-questions:

### 6.1 Is Pattern 3 semantically sufficient for the dependency chain?

Yes. The recipe IS the dependency. `MinAcrossInputs` computes the rate
of joint production, `SubtractFromAllInputs` debits each input's Need
proportionally, and the target write applies the produced amount to the
output's Need. The kernel uses `atomic_add_f32_at` on the target by
default for `CONSUME_SUBTRACT_FROM_ALL_INPUTS`, so the output's Need
accumulates per tick — exactly the "Flow column contribution rather
than discrete units" behavior the handoff asks about.

### 6.2 `ConsumeMode` for continuous-flow output

`SubtractFromAllInputs` is the right shape. It triggers the multi-input
subtraction and writes the produced amount to the target via additive
atomic write. `EmitEvent` is for discrete-emission cases (debt-band
crossing where the CPU needs to know "N units were produced this
boundary"). For continuous dependency where the output is just a Need
contribution, `SubtractFromAllInputs` alone is correct, with the target
being the output's Need column.

If the dependency *also* needs to surface discrete production events
(e.g. "this recipe produced 3 frigates this boundary, log them"), the
op can combine: `SubtractFromAllInputs + EmitEvent` — the kernel
checks `consume == CONSUME_SUBTRACT_FROM_ALL_INPUTS` in `apply_consume`
and `consume == CONSUME_EMIT_EVENT` in `maybe_emit_event` as
separate dispatches. Looking at the kernel's `execute_ops`:
```
write_value = gather_value(op);
apply_targets(target_value, op);
apply_consume(write_value, op);
maybe_emit_event(op_idx, write_value, op);
```
`apply_consume` and `maybe_emit_event` are independent. **However**, the
current kernel structure does *both* only when `consume` matches each
arm respectively — there is no joint case. The current shape requires
either one or the other. The ADR should accept this limitation: a
recipe writes to Need (continuous) *or* emits a discrete event, but not
both in one op. Two paired ops handle the rare both-needed case.

### 6.3 `AccumulatorInputListTable` extension beyond N=4 inputs

**Small change, not significant refactor.** Verified directly:

- `SOURCE_INPUT_LIST` is already a kernel source kind (`accumulator_op.wgsl`
  line 88).
- `transfer_accumulator.rs::plan_transfer_ops` emits ConjunctiveCrossing
  ops that get encoded with `source_kind = SOURCE_INPUT_LIST` and
  references into the persistent `AccumulatorInputListTable` (binding
  10). Already operates for arbitrary N.
- The bottleneck is the CPU-side `AccumulatorOp::validate()` rule
  `inputs.len() > 4 → TooManyConjunctiveInputs(5)` in
  `crates/simthing-core/src/accumulator_op.rs`.
- The 4-element bound is a holdover from the pre-input-list inline-array
  layout. With the input list table in production, the limit is the
  table's capacity (4096 default in `DEFAULT_INPUT_LIST_CAPACITY`,
  grows on demand via `ensure_capacity`).

The CPU-side validate limit is the only blocker, and lifting it is a
one-line change plus an updated test. The kernel and the planner
already support N inputs.

---

## 7. D-1 verdict

**Substantially eliminated for the continuous-flow case. Preserved at
narrowed scope for the discrete-transaction case.**

### 7.1 What D-1 was solving

The workshop scenario: 16 pools × 100k requesters at *tick time*, each
requester wanting an amount from its pool. The v1 allocator
(`transfer_contention_gpu.wgsl::resolve_transfer_contention_tick`) is
one workgroup invocation per pool, walking requesters linearly in
authored order. At 16 pools × 6250 requesters per pool, 16 GPU threads
do the work; CPU multi-core wins. Hence the measured 0.14× CPU
(7× slower) at hotspot.

### 7.2 Why the continuous-flow model eliminates this regime

Under the proposition:

- Per-tick: Sum reduction on Flow column. Fully parallel via the
  AccumulatorOp reduction OrderBand (C-5/C-6 landed). O(depth) ≈ O(log N)
  serial dispatches per boundary, each fully parallel within its band.
  No shared-slot writes at tick time.
- Per-tick: Need integration. `IntegrateWithClamp` is one op per
  (slot, governed pair), writing to two atomic cells (Amount + Velocity
  pinning, or Need + Flow pinning). No cross-slot contention.
- Per-tick: Recipe execution. Same-band consumed-input contention is
  rejected at plan time (`ContendedConsumedInput` in
  `transfer_accumulator.rs`). Each recipe op owns its inputs.
- Per-boundary: Allocator runs once. N ops are uploaded each targeting
  an independent (slot, col). The kernel's
  `atomic_add_single_writer_f32_at` (used for OrderBand-gated adds)
  bypasses the CAS loop because OrderBand guarantees a single writer per
  (band, slot, col).

The 16-pool/100k-requester hotspot does not arise: there is no shared
pool slot being written at tick time. The proof obligation is that the
allocator's boundary-time work is bounded — which it is, by N (number
of arena participants), uploaded once per boundary, not per tick.

### 7.3 Why D-1 is not fully eliminated

Discrete transactions remain:

- Construction commits ("AI decides to build a fleet of 50 frigates;
  debit 50 × cost from the faction pool this boundary").
- Treaty payments (cross-faction transfers at boundary time).
- Emergency spend (player or AI action that bypasses the steady-state
  Flow model).
- Mass-action policy changes (one boundary triggers transfers across
  many participants).

These still use `SubtractFromSource` at boundary time and still create
hotspot contention if N consumers debit one pool simultaneously. The
question is *scale*: in practice, discrete transactions are dramatically
rarer than continuous flows. Construction commits in a typical 4X are
bounded by player/AI action count per boundary — order O(10²) per
faction, not O(10⁵). The 100k-requester hotspot is unlikely under
discrete transactions.

### 7.4 Recommended D-1 disposition

Rather than executing the original D-1 design work, **rescope D-1**:

- **D-1 (new):** Design memo evaluating whether discrete-transaction
  contention reaches a scale that justifies a GPU allocator at all.
  Likely outcome: CPU-side priority queue with `SubtractFromSource` ops
  is sufficient at realistic scales. The workshop's 16-pool/100k
  scenario is recognized as a continuous-flow stress test that has been
  architecturally addressed; new discrete-transaction stress tests
  should drive D-1's decision.
- **D-2 (deferred indefinitely):** Hot-pool allocator v2 implementation.
  Defer until a discrete-transaction workload demonstrates the need.
- **D-3 (unchanged):** Changed-only compact logs + replay. The compact
  emission record path is independent of the allocator question and
  proceeds as planned.
- **D-4 (unchanged):** Cross-pool queue contention gate. Still useful as
  a forward sentry, but now expected to pass trivially because per-tick
  pool contention is gone.

The cost saving is significant: D-1 was scoped as a complex Opus design
note + multi-PR implementation. Under the rescope, D-1 becomes a short
analysis memo.

---

## 8. E-phase recommendation

| PR | Recommendation | Reason |
|---|---|---|
| E-1 `emit_on_threshold` | **UNCHANGED** | Debt-band Pattern 2 is the existing mechanism (Threshold gate + EmitEvent consume). Landed via C-1 + C-8d. Builder is straightforward. |
| E-2 `resource_transfer` | **REVISE — split into two builders** | `resource_transfer_discrete(source, target, amount)` for boundary-time discrete transactions, retaining `SubtractFromSource`. `resource_flow_participant(slot, supply_rate)` for continuous-flow enrollment, retaining `ResetTarget`/`AddToTarget` against participants' Flow columns. The original per-edge SubtractFromSource for continuous flows is *replaced* by the allocator path. |
| E-3 `conjunctive_recipe` | **UNCHANGED** | Pattern 3 IS the dependency mechanism. Landed via C-8c. The 4-input CPU validation limit should be lifted in a small follow-up PR (`AccumulatorOp::validate` line `inputs.len() > 4`) to fully exploit the GPU input-list table — but this is not strictly E-3 scope. |
| E-4 RON + session integration | **RETHINK** | If E-2 splits, the RON authoring format needs to express discrete vs continuous resource semantics. The `resource` property kind in design_v7.md §5.3 already implies this distinction implicitly; the explicit form should declare arena participation alongside the property registration. |
| E-5 compact log integration | **UNCHANGED** | Compact emission records are generic. The Flow allocator's per-boundary ops do not produce emission records (they are not threshold-gated emissions); they are normal AccumulatorOp registrations whose effects are visible via the summary readback diff. Discrete-transaction `SubtractFromSource` ops *do* produce events under the existing C-8c/C-8d paths. |
| E-6 doc update | **FOLLOWS** | Updates to design_v7.md §5 are downstream of this evaluation. The §5.1 patterns need a fourth pattern: "Pattern 4 — Continuous resource flow" describing the Flow/Need/Allocator model. The §5.4 conservation guarantee should be split into per-recipe (exact) and per-arena (allocator-enforced) sub-claims. |

> **Successor note:** the E-phase matrix has been substantially expanded
> by the ADR-shaping draft to include E-7 (`governed_by` generalization),
> E-8 (`accumulator_spec` lands), E-9 (`ArenaRegistry`), E-10 (spec
> admission framework), E-11 (hierarchical allocation kernel). See
> `resource_flow_adr_shaping.md` §9 for the current matrix.

---

## 9. Assumptions in the proposition I have verified directly

The proposition makes several claims about codebase shape that I checked
against source rather than accepting on faith:

1. *"Pattern 3 (ConjunctiveCrossing + MinAcrossInputs + SubtractFromAllInputs)
   is the existing dependency mechanism"* — **VERIFIED.** Landed in C-8c.
   See `transfer_accumulator.rs::plan_transfer_ops` (conjunctive branch)
   and `accumulator_op.wgsl::apply_consume` (CONSUME_SUBTRACT_FROM_ALL_INPUTS
   branch).
2. *"The AccumulatorInputListTable (binding 10, C-8c) handles N > 4 inputs"*
   — **VERIFIED.** Landed in C-8c. CPU type still validates ≤ 4; GPU path
   supports N. The CPU limit is a one-line change.
3. *"output_vectors is the post-reduction readback path"* — **VERIFIED.**
   `WorldGpuState::read_output_vectors` exists; `BoundaryProtocol::read_reduced_field`
   exposes it.
4. *"upload_ops / boundary_sync is the per-boundary registration upload path"*
   — **VERIFIED.** See `sync_accumulator_*_session` methods on
   `BoundaryProtocol`.
5. *"governed_by + IntegrateWithClamp already exists for Amount/Velocity"*
   — **VERIFIED.** C-7 landed (flag default false; legacy
   `velocity_integration.wgsl` remains until S-5). The generalization to
   arbitrary `governed_by` pairs is a small planner change.
6. *"SubFieldSpec.accumulator_spec is planned but not landed"* —
   **VERIFIED.** `design_v7.md` §3 lists it as "NEW in v7" but the
   field is not present in `crates/simthing-core/src/property.rs` on
   the local worktree. This is a real implementation gap that the
   proposition's NeedSpec depends on.
7. *"SoftAggregateGuard validator exists and fires on hard-trigger
   soft-aggregate paths"* — **VERIFIED.** See
   `simthing-sim/src/threshold_registry.rs::assert_no_hard_trigger_on_soft_aggregate`,
   landed with A-4 forward-protection.

One assumption I could not verify because the document does not exist
on the worktree: the handoff references
`docs/workshop/resource_vector_primitive_workshop.md` as a companion
document. There is no such file in `docs/workshop/` or
`docs/workshop/archive/`. The proposition is treated as freestanding;
this evaluation does not depend on workshop content beyond what is in
the handoff.

---

## 10. Overall feasibility verdict

**PROCEED TO ADR WITH MODIFICATIONS.**

The proposition is constitutionally compatible with the SimThing
architecture as landed in `master` on 2026-05-26. No fundamental
blocker. Required modifications are:

1. State the rate-vs-stock distinction explicitly: Flow is a rate
   signal; Need is the conserved stock. Global per-tick Σ Flow = 0 is
   NOT required.
2. Clarify the `SubtractFromSource` invariant: source-debit transfers
   use it; allocator disbursements to participants' own Flow columns
   use `ResetTarget` / `AddToTarget`; conservation in the allocator
   path is enforced by pool-level allocator accounting, not per-op.
3. Adopt `SubFieldRole::Named("flow")` and `SubFieldRole::Named("need")`
   — not first-class variants.
4. Add `accumulator_spec: Option<AccumulatorSpec>` to `SubFieldSpec`
   (already planned in design_v7.md §3 but not yet landed). Hang
   `NeedSpec` and any future per-sub-field accumulator metadata here.
5. Rescope D-1 to a discrete-transaction contention analysis memo; defer
   D-2 implementation indefinitely; keep D-3 and D-4.
6. Split E-2 into discrete-transfer and continuous-flow-participant
   builders.

The ADR should be authored against these modifications and reviewed
against this evaluation. If the ADR can articulate (1) and (2) clearly,
the rest follows mechanically.

The architectural saving is large: the workshop's measured 0.14×-CPU
hotspot regime disappears for the typical continuous-flow case, and the
participant-tree model maps directly onto the existing reduction
substrate. This is the strongest version of the AccumulatorOp v2 vision —
the substrate generalizes from "transfer + emit" to "transfer + emit +
arena". The proposition is worth doing.

---

## Appendix A — file-level evidence index

For traceability if another agent re-checks the claims:

| Claim | File | Function / line |
|---|---|---|
| `SubFieldRole` is `{ Amount, Velocity, Intensity, Named, Custom }` | `crates/simthing-core/src/property.rs` | enum definition near top |
| `SubFieldSpec` does not yet have `accumulator_spec` | `crates/simthing-core/src/property.rs` | struct fields list |
| `ReductionRule::Sum` dispatches to `combine_kind::SUM` | `crates/simthing-gpu/src/reduction_orderband.rs` | `combine_for_rule` |
| Conjunctive recipe conservation is exact | `crates/simthing-gpu/src/shaders/accumulator_op.wgsl` | `gather_min_across_inputs` + `apply_consume` |
| Same-band consumed-input contention rejected at plan time | `crates/simthing-gpu/src/transfer_accumulator.rs` | `plan_transfer_ops`, `seen_consumed` set |
| `AccumulatorInputListTable` supports arbitrary N | `crates/simthing-gpu/src/accumulator_op/input_list_table.rs` | `ensure_capacity`, `upload_lists` |
| CPU-side 4-input limit | `crates/simthing-core/src/accumulator_op.rs` | `AccumulatorOp::validate`, `TooManyConjunctiveInputs` |
| `read_output_vectors` is the post-reduction readback path | `crates/simthing-gpu/src/world_state.rs` and `crates/simthing-sim/src/boundary.rs::read_reduced_field` | |
| Boundary sequence step ordering | `crates/simthing-sim/src/boundary.rs` | header comment lines 11–32 |
| `assert_no_hard_trigger_on_soft_aggregate` validator | `crates/simthing-sim/src/threshold_registry.rs` | function definition |
| V1 allocator is one-invocation-per-pool | `crates/simthing-workshop/src/transfer_contention_gpu.wgsl` | `resolve_transfer_contention_tick` |
| C-7 governed-pair integration combine | `crates/simthing-gpu/src/shaders/accumulator_op.wgsl` | `COMBINE_INTEGRATE_CLAMP` branch in `execute_ops` |

---

*End of evaluation. Total: 10 sections + appendix.*
