# SimThing — Design v7

> **Status:** Active specification. Living document updated by each migration
> PR. Supersedes `design_v6.md` §10 (GPU pipeline). All other sections of
> `design_v6.md` and `design_v6.5.md` remain authoritative until explicitly
> superseded here.
>
> **Companion documents:**
> - `docs/adr_accumulator_op_v2.md` — decision rationale and evidence
> - `docs/accumulator_op_v2_production_plan.md` — PR ladder
> - `docs/design_v6.md` — previous specification (§10 superseded)
> - `docs/design_v6.5.md` — session parking doc (unchanged)

---

## 1. What changed in v7

v7 introduces one new foundational primitive and an economic substrate built
on it. Everything else in v6/v6.5 is unchanged.

**New: AccumulatorOp v2** — a unified GPU gather/combine/gate/scatter primitive
that replaces the 8-pass specialised pipeline with a 3-pass architecture. It is
the substrate for resource interactions, overlay application, reduction,
threshold scanning, and EML-combined updates.

**New: Economic substrate** — transfer, debt-band emission, and conjunctive
production recipes as first-class AccumulatorOp registrations. These are the
operations the foundational retrospective identified as the missing primitive
for how values move between SimThings.

**New: Logging tiers** — summary/checksum default production tier; compact
emission records for audit and replay; full state readback for debug only.

**Unchanged from v6/v6.5:**
- SimThing recursive type (`SimThing { properties, overlays, children }`)
- OverlayLifecycle (`Permanent | Transient | Suspended`)
- BoundaryRequest protocol
- FissionTemplate and fission/fusion semantics
- DimensionRegistry and SubFieldRole invariants
- spec layer / simthing-driver / simthing-spec contracts
- Replay v3 (`SpecSnapshot`, `SpecDelta`, logical-key invariant)
- All production tests in `simthing-sim` (they pass throughout the migration)

---

## 2. The v7 constitution

```
One recursive type:
  SimThing { properties, overlays, children }

One mechanism for change:
  overlay TransformDelta, referencing SubFieldRole by name

One mechanism for resource interaction:
  AccumulatorOp { source, combine, gate, scale, consume, targets }
  — the universal primitive for accumulation, threshold-gated emission,
    value transfer, and multi-input production recipes

One mechanism for differentiation:
  intensity threshold registered in DimensionRegistry

One execution model:
  GPU-resident AccumulatorOpSession, persistent across ticks,
  summary/checksum readback by default

  B-1 note: AccumulatorOpSession in simthing-gpu is a non-integrated bootstrap
  skeleton (non-contended Identity/Sum/clamped transfer only). Production
  AccumulatorOp semantics lock only as C-family migrations pass parity.

  B-2 note: the kernel is production-shaped for persistent buffers and compact
  event readback (Identity EmitEvent), but operation-family semantics are only
  authoritative once their C/E migration PRs pass parity. Bootstrap validation
  rejects any same-runtime-cell contention, including Always-vs-OrderBand
  aliasing, until production contention semantics are implemented.

  B-3 note: `AccumulatorOpSession` exposes optional timestamp measurement for
  the execute pass via `last_pass_time_us()`. This is instrumentation only and
  does not affect operation semantics.

One retained operation:
  snapshot — copy_buffer_to_buffer (memcpy; not a per-slot write)

One source of truth:
  GPU dense matrices + CPU semantic interpretation at boundaries

One place to edit:
  DimensionRegistry — SubFieldSpec governs column layout
```

AccumulatorOp declares:

```
source    where value comes from
          Constant | SlotValue | SlotRange | ConjunctiveCrossing

combine   how N inputs collapse to a write value
          Identity | Sum | Mean | Max | Min | WeightedMean* | Product |
          LastByPriority | IntegrateWithClamp | CrossingFormula |
          MinAcrossInputs | EvalEML†

gate      when the write fires
          Always | Threshold | LifecycleActive | DirtyOnly | OrderBand

consume   what happens to the source after the write
          None | SubtractFromSource | SubtractFromAllInputs |
          ResetTarget | ScaleTarget | EmitEvent

targets   where the result goes (up to 4 slots × cols)

* WeightedMean: soft aggregate; tolerance policy from ADR applies
† EvalEML: requires whitelist entry; no transcendentals; ≤16 nodes
```

Velocity integration, overlay application, reduction, transfer, emission —
all AccumulatorOp instances. The CPU receives only what cannot be
AccumulatorOp: structural mutations (fission, overlay lifecycle, property
expiry, capability unlock).

---

## 3. The v7 SimThing type

Unchanged from v6:

```rust
SimThing {
    id:         SimThingId,
    kind:       SimThingKind,
    properties: BTreeMap<SimPropertyId, PropertyValue>,
    overlays:   Vec<Overlay>,
    children:   Vec<SimThingId>,
}
```

The only addition: properties that participate in the economic substrate carry
an `AccumulatorSpec` in their `SubFieldSpec`:

```rust
SubFieldSpec {
    role:                   SubFieldRole,
    width:                  usize,
    governed_by:            Option<SubFieldRole>,
    clamp:                  ClampBehavior,
    reduction_rule:         Option<ReductionRule>,
    soft_aggregate_guard:   Option<SoftAggregateGuard>,   // NEW in v7
    accumulator_spec:       Option<AccumulatorSpec>,       // NEW in v7
}

struct AccumulatorSpec {
    /// Which combine function this sub-field uses when participating in
    /// emission or production recipes.
    combine_hint:   CombineFn,
    /// Logging tier override for this field. Default: Summary.
    log_tier:       LogTier,
}

enum SoftAggregateGuard {
    Unguarded,
    Quantized { step: f32 },
    Hysteresis { band: f32 },
}
```

`SoftAggregateGuard` is required on any sub-field using `WeightedMean` or
`Mean` reduction that feeds a threshold registration. Validated at
registration time by `assert_no_hard_trigger_on_soft_aggregate()`.

---

## 4. GPU pipeline (v7 — replaces design_v6.md §10)

> This section is maintained as a living document. Each migration PR updates
> it to reflect which passes have moved to AccumulatorOp and which remain.
> Current state: pre-migration baseline (all passes still exist).

### 4.1 The target 3-pass architecture (post-migration)

```
Pass 0:  Snapshot
         copy_buffer_to_buffer(values → previous_values)
         Retained permanently. Not a per-slot write.

Pass B:  AccumulatorOp
         Unified gather/combine/gate/scatter kernel.
         Dispatched once per OrderBand in ascending order.
         One WGSL file: accumulator_op.wgsl
         Handles: velocity integration, overlay application, all reductions,
                  threshold-gated events, transfer, debt-band emission,
                  conjunctive production, EML-combined updates.

Pass C:  Event readback
         GPU atomic counter + compact EmissionRecord buffer.
         CPU reads: emission_count (4 bytes), then emissions (count × 8 bytes).
         Structural events (fission, capability unlock, expiry) route here.
         Pure numeric emissions resolved GPU-side do not reach CPU.
```

### 4.2 Current state during migration

The following flags control which path runs for each operation family.
All default to `false` (existing path) until each migration PR flips them
to `true` and the corresponding sunset PR removes the old code.

```rust
pub struct PipelineFlags {
    pub use_accumulator_threshold_scan: bool,  // C-1 → S-6
    pub use_accumulator_intent:         bool,  // C-2 → S-1
    pub use_accumulator_overlay_add:    bool,  // C-3 → S-3
    pub use_accumulator_overlay_mul_set: bool, // C-4 → S-3
    pub use_accumulator_weighted_mean:  bool,  // C-5 → S-4
    pub use_accumulator_sum_max_min:    bool,  // C-6 → S-4
    pub use_accumulator_velocity:       bool,  // C-7 → S-5
    pub use_accumulator_eml_transfer:   bool,  // C-8 → S-2
}
```

### 4.3 Pass descriptions (current baseline — update as migration PRs land)

**Pass 0 — Snapshot (permanent)**
- `encoder.copy_buffer_to_buffer(values_buffer, previous_values_buffer)`
- No kernel dispatch. Hardware DMA. Not subject to migration.

**Pass 1 — Velocity integration (migrate → C-7, sunset → S-5)**
- WGSL: `velocity_integration.wgsl`
- Per-slot: `amount += velocity * dt`, clamped to `vel_max`
- `GovernedPair` drives per-property parameters
- Post-migration: replaced by `IntegrateWithClamp` combine + MultiTarget

**Pass 2 — Intensity update (migrate → C-8, sunset → S-2)**
- WGSL: `intensity_update.wgsl`
- Per-slot: piecewise `build` or `decay` based on `|velocity| > threshold`
- Post-migration: replaced by `EvalEML` combine with whitelisted intensity tree

**Pass 3 — Overlay application (migrate → C-3/C-4, sunset → S-3)**
- WGSL: inline in `overlay_prep.rs`
- `Add`, `Multiply`, `Set` ops in ancestor-then-local order
- **C-3 landed:** `use_accumulator_overlay_add` routes Add-only overlay batches
  through AccumulatorOp. If any active Multiply or Set overlay is present, the
  full overlay batch remains on legacy Pass 3 until C-4. For Add-only batches,
  C-3 emits one AccumulatorOp per Add delta with an OrderBand equal to that
  cell's local Add sequence index; the pipeline dispatches all overlay Add bands
  in ascending order within the same command buffer, preserving legacy f32
  operation order. This fallback is temporary — C-4 owns the full order-band
  compiler and S-3 deletes the old overlay path after C-3/C-4 pass parity.
- Post-migration: `Identity+OrderBand(0)`, `Product+OrderBand(1)`,
  `LastByPriority+OrderBand(1)` combines

**Passes 4–6 — Reduction (migrate → C-5/C-6, sunset → S-4)**
- WGSL: `reduction.wgsl`
- Depth-bucketed `Sum`, `Mean`, `WeightedMean`, `Max`, `Min`
- Post-migration: `SlotRange` source + named combine per parent, dispatched
  in depth-bucket OrderBand sequence

**Pass 7 — Threshold scan (migrate → C-1, sunset → S-6)**
- WGSL: `threshold_scan.wgsl` (legacy path; default via `use_accumulator_threshold_scan: false`)
- Reads `previous_values` vs `values`, detects crossings, writes events
- **C-1 landed:** `use_accumulator_threshold_scan` on `BoundaryProtocol` wires
  `Threshold` gate + `EmitEvent` via `WorldGpuState::threshold_accumulator`;
  compact `ThresholdEmissionGpu` readback replaces Pass 7 when flag is true.
  Parity: `c1_threshold_scan_parity.rs` (fission_stress 20k × 100 ticks).
- Post-sunset (S-6): delete `threshold_scan.wgsl` and the flag; Pass 7 entry removed.

**Pre-Pass 0 — Intent delta application (migrate → C-2, sunset → S-1)**
- WGSL: `intent_delta.wgsl` (legacy path; default via `use_accumulator_intent: false`)
- Applies folded CPU `IntentDelta { mul, add }` as `values = values * mul + add`
- **C-2 landed:** `use_accumulator_intent` on `BoundaryProtocol` wires folded
  intent rows as `COMBINE_AFFINE_INTENT` ops via `WorldGpuState::intent_accumulator`;
  encoded in the same tick command buffer before snapshot when flag is true.
  Parity: `c2_intent_accumulator_parity.rs` (10 scenarios + combined C-1/C-2 ordering).
- Post-sunset (S-1): delete `intent_delta.wgsl` and the flag.

---

## 5. Economic substrate

### 5.1 The three canonical patterns

Every resource interaction in SimThing is one of three AccumulatorOp
registration patterns:

**Pattern 1: Resource transfer**

```
Transfer from faction pool to factory queue at a fixed rate per tick.

source:   SlotValue(faction_pool_slot, amount_col)
combine:  Identity
gate:     Always
consume:  SubtractFromSource(rate)
target:   factory_queue_slot, amount_col
```

Conservation is structurally enforced. The source decrement and target
increment are atomic within a single registration. No two-overlay hack.

**Pattern 2: Debt-band emission**

```
Factory queue accumulates value; when it crosses a band boundary,
emit_count units are produced and the queue is decremented accordingly.

source:   SlotValue(queue_slot, amount_col)
combine:  CrossingFormula { unit_cost }
gate:     Threshold { value: -((queued_count - 1) * unit_cost), direction: Downward }
consume:  SubtractFromSource
target:   units_produced_slot, amount_col
```

On emission: the CPU boundary hook reads `emit_count` from the emission record,
decrements `queued_count`, and re-registers the next threshold band. The GPU
handles the arithmetic; the CPU handles the state bookkeeping.

**Pattern 3: Conjunctive production recipe**

```
One unit requires ALL of: 5 iron + 3 energy + 2 labor.
Factory emits only when all three channels have accumulated enough.

source:   ConjunctiveCrossing { inputs: [
            (queue_slot, iron_col,   unit_cost=5.0),
            (queue_slot, energy_col, unit_cost=3.0),
            (queue_slot, labor_col,  unit_cost=2.0),
          ]}
combine:  MinAcrossInputs
gate:     Always
consume:  SubtractFromAllInputs
target:   units_produced_slot, amount_col
```

The recipe IS the registration. Conservation is structurally enforced across
all three channels atomically. No CPU correlation state.

### 5.2 Builder API

The spec layer provides three builders that produce correctly-formed
`AccumulatorOp` registrations. Modders and Studio use these; they do not
construct `AccumulatorOp` directly.

```rust
// In simthing-spec:

/// Transfers `rate` units per tick from `source` to `target`.
pub fn resource_transfer(
    source: ResourceRef,
    target: ResourceRef,
    rate:   f32,
) -> AccumulatorOp

/// Emits units when the debt-band accumulator crosses a band boundary.
pub fn emit_on_threshold(
    accumulator: ResourceRef,
    unit_cost:   f32,
    queued_count: u32,
    max_per_tick: u32,
    target:       ResourceRef,
) -> AccumulatorOp

/// Emits one unit when ALL inputs have accumulated enough for one recipe.
pub fn conjunctive_recipe(
    inputs:      &[RecipeInput],   // (resource, unit_cost) up to 4
    target:      ResourceRef,
    max_per_tick: u32,
) -> AccumulatorOp
```

### 5.3 RON authoring format

Economic properties in modder-facing RON:

```ron
// Example: iron ore resource property
(
    property: "iron_ore",
    namespace: "economy",
    sub_fields: [
        (
            role: Amount,
            accumulator_spec: Some((
                combine_hint: CrossingFormula,
                log_tier: Summary,
            )),
        ),
        (
            role: Velocity,  // transfer rate
        ),
    ],
)

// Example: factory recipe
(
    recipe: "basic_unit",
    inputs: [
        (resource: "iron_ore",  unit_cost: 5.0),
        (resource: "energy",    unit_cost: 3.0),
        (resource: "labor",     unit_cost: 2.0),
    ],
    output: "units_produced",
    max_per_tick: 4,
)
```

The `simthing-driver` session assembly translates these into `AccumulatorOp`
registrations at session open. `simthing-sim` sees only `AccumulatorOp`
structs; it never knows what "iron_ore" or "basic_unit" means.

### 5.4 Conservation guarantee

The following invariant holds exactly for all economic registrations:

```
faction_pool_decrease = factory_queue_increase + (total_emissions × Σ unit_costs)

where:
  faction_pool_decrease = Σ(ch) pool_initial[ch] - pool_final[ch]
  factory_queue_increase = Σ(f, ch) queue_final[f,ch] - queue_initial[f,ch]
  total_emissions = sum of all emit_count values from compact emission buffer
  Σ unit_costs = sum of unit_costs across all channels in the recipe

Tolerance: ±0.01 × faction_pool_decrease (floating-point drift only,
           no structural loss)
```

This invariant is verified by the boundary handler after each tick using
the summary/checksum readback.

---

## 6. Logging tiers

### 6.1 Tier definitions

```
Summary (default production):
  GPU writes SlotSummary { slot, checksum } for every modified slot.
  CPU reads the summary buffer after each tick.
  Used for: boundary skip decisions, change detection, basic audit.
  Volume: 8 bytes × n_slots per tick.

  B-1 ships checksum-only SlotSummary as a provisional bootstrap tier.
  B-2 continues to use this checksum-only tier for parity.
  B-4 remains responsible for the final summary/checksum/coarse-value design.

Compact records (production audit / selective replay):
  GPU writes EmissionRecord { registration_idx, emit_count } for every
  GPU-resolved emission event.
  CPU reads the compact emission buffer after each tick.
  Used for: delta log integration, replay checkpoints, resource audit.
  Volume: 8 bytes × n_emissions per tick (sparse).

Full readback (debug only):
  CPU reads entire values buffer.
  AccumulatorOpSession::readback_full_values() logs a warning when called.
  Never called in production unless behind an explicit debug flag.
  Volume: 4 bytes × n_slots × n_dims per tick.
```

### 6.2 Replay model

Replay uses compact emission records as the authoritative source:

1. Start from a `SpecSnapshot` (initial session state)
2. For each tick, apply `AccumulatorEmission` delta entries from the compact log
3. Assert final resource totals match the recorded summary checksums

This is sufficient for conservation-exact replay because:
- Transfer and emission are the only value-changing operations that are not
  fully deterministic from initial state + ops alone
- Summary checksums detect any divergence from the expected post-tick state
- Full-state readback is available for debugging specific divergences

GPU-resolved WeightedMean and EML outputs are NOT in the compact log. They are
reproduced by re-running the same AccumulatorOp registrations from the
same initial state. This requires that the session's `AccumulatorOp`
registration buffer is stored in the `SpecSnapshot`.

---

## 7. EML integration in v7

EML is the expression compiler for designer-tunable combine functions.
It operates at three stages:

**Stage 1 — Boundary prep (CPU, every boundary)**
When an overlay's `value` or a threshold's `threshold_value` comes from a
designer formula, EML compiles to a scalar at boundary prep time. The GPU
sees a constant.

**Stage 2 — Registration parameter generation (CPU, session open or recipe change)**
Per-property parameters like intensity build/decay coefficients compile via
EML to `f32` values stored in `AccumulatorOp.combine_p0..combine_p3`. One
compile per recipe class, not per slot.

**Stage 3 — GPU EvalEML combine (GPU, per tick)**
For whitelisted formulas (no transcendentals, ≤16 nodes), the kernel evaluates
the EML tree against per-slot inputs inline. Validated: ~1.6× overhead vs
hardcoded at 100k slots; bit-exact for the tested formula class; deterministic.

The intensity update formula is the canonical Stage 3 example:
```
if |velocity| > threshold:
    intensity += build_coeff × |velocity| × dt
else:
    intensity -= decay_coeff × intensity × dt
intensity = clamp(intensity, 0.0, 1.0)
```
16 nodes, no transcendentals, bit-exact on CPU/GPU comparison.

---

## 8. Performance model

The v7 performance model is defined by persistent GPU execution with
summary-tier readback. Total-validation readback is not the performance
baseline; it is a debug tool.

### Reference benchmarks (from workshop persistent-buffer test)

| Scenario | Timestamped GPU pass time | vs current GPU envelope |
|---|---|---|
| Distributed 100k | 0.275 ms/tick (17.57 ms / 64 ticks) | 8.6× faster |
| Sparse 100k | 0.068 ms/tick (4.33 ms / 64 ticks) | 46.3× faster |
| Hotspot 100k (v1 allocator) | 3.066 ms/tick (196 ms / 64 ticks) | 3.0× faster |

Hotspot performance is limited by the v1 allocator. The v2 allocator (Phase D)
is expected to bring hotspot closer to the distributed case.

### tick_event_readback_ms retirement

`tick_event_readback_ms` (~21ms at `fission_stress` scale) is the dominant
cost in the current pipeline. Under v7, it is replaced by:
- `pass_b_dispatch_us` — timestamped GPU dispatch time for Pass B
- `pass_c_readback_us` — readback of the compact emission buffer only
  (4 bytes for count + count × 8 bytes for records)

At 100k slots with typical 10–20% crossing rate, `pass_c_readback_us` is
expected to be under 1ms — approximately 20× improvement over the current
`tick_event_readback_ms`.

---

## 9. Invariants (v7 additions)

These additions to `docs/invariants.md` are enforced with the same weight as
existing invariants:

| Rule | Enforced by |
|---|---|
| Exact operations never use soft-aggregate combine fns | Code review; `WeightedMean`/`Mean` banned from conservation-critical paths |
| `EvalEML` requires whitelist entry | `EmlExpressionRegistry::assert_whitelisted()` at registration |
| Transfer uses `SubtractFromSource` only | No two-overlay transfers anywhere in the codebase |
| Emission records written for every GPU-resolved emission | `EmissionRecord` in compact buffer; count checked against emission_capacity |
| Persistent session per session lifetime | No `AccumulatorOpSession::new()` in hot path |
| Timestamp queries required for perf claims | PR template checklist |
| Old pass deleted only after CI green at flag=on | Sunset PR checklist |
| `design_v7.md §4` updated by each migration PR | PR template checklist |
| `SoftAggregateGuard` on WeightedMean columns feeding thresholds | `assert_no_hard_trigger_on_soft_aggregate()` at registration |
| `simthing-sim` never knows recipe semantics | No recipe strings, costs, or economic types in `simthing-sim` |

---

## 10. Read order for new agents

1. `docs/invariants.md` — hard rules, always read first
2. `docs/adr_accumulator_op_v2.md` — the decision and evidence
3. `docs/design_v7.md` §2 (constitution) and §4 (current pipeline state)
4. `docs/design_v7.md` §5 (economic substrate) if working on resource/economic code
5. `docs/eml_integration_guidance.md` if working on EML expressions
6. `docs/accumulator_op_v2_production_plan.md` for the PR you're implementing
7. `docs/design_v6.md` for anything not yet superseded by v7
8. `docs/design_v6.5.md` (session parking doc — spec/driver layer)

Do not implement from `design_v6.md §10`. It is superseded.
