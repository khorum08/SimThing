# C-4 Overlay Multiply/Set + Dirty/Cached OrderBand Compiler (Opus design memo)

**Author:** Opus 4.7
**Date:** 2026-05-25 (erratum: 2026-05-25)
**Gate for:** Composer/Codex implementation PR — `feat(gpu): C-4 overlay Mul/Set + dirty OrderBand compiler`
**Status:** Accepted (design); implementation PR follows separately
**Implementer:** **Codex 5.5** (mechanical execution from this memo's specifications)

> ⚠️ **2026-05-25 ERRATUM**: §7.1 supersedes §7.2's `write_target`
> sketch. The original sketch used plain `values[idx] = ...` assignment,
> which does not compile because `values: array<atomic<i32>>` in the
> actual shader. Codex must use the `atomicLoad` + `atomicStore` form
> from §7.1, scoped to the per-band single-writer invariant. The CAS
> loop in `atomic_add_f32_at` is not required here (only one writer
> per cell per dispatch) and would impose unnecessary overhead in the
> high-density path the workshop regression hit.
**Companion:** `docs/adr_accumulator_op_v2.md`, `docs/design_v7.md` §2/§4, `docs/accumulator_op_v2_production_plan.md` PR C-4, `docs/workshop/pivot_forward_implementation_policy.md`, `docs/workshop/slot_summary_b4_design.md`

---

## TL;DR

> **Reuse `build_overlay_deltas` unchanged.** It already does the tree walk,
> lifecycle filtering, ancestor-before-local ordering, fission-clone
> handling, and per-property/per-sub-field column resolution. C-4 is a
> downstream planner that consumes `(Vec<OverlayDelta>, Vec<SlotDeltaRange>)`,
> exactly like C-3, and handles mixed Add/Mul/Set with the same per-cell
> OrderBand pattern.
>
> **Reject the user-proposed `OverlayCompileCache { slot_bands,
> dirty_slot_bands, overlay_to_slots, ... }` shape** as overengineered:
> partial recompilation introduces ancestor-ordering bugs, and the actual
> high-density perf cost is GPU buffer upload, not CPU planner work.
>
> **Adopt the two-tier cache:**
> 1. `overlay_compile_revision: u64` on `BoundaryProtocol`, bumped on
>    overlay-affecting events (attach, dissolve, structural mutation,
>    activate/suspend, property add/remove).
> 2. Bytewise equality check of `(deltas, ranges)` against the cache as
>    defense-in-depth. Even when revision differs but the output is
>    identical, skip the GPU re-upload.
>
> When revision matches: skip `build_overlay_deltas`, skip planning, skip
> upload. Reuse the GPU buffer. This is the high-density steady-state win
> the workshop demanded.
>
> **CombineFn / ConsumeMode mapping for overlay write-target semantics:**
>
> | overlay op | `combine` | `consume` | shader effect |
> |---|---|---|---|
> | `OP_ADD` | `Identity` | `AddToTarget` *(new)* | `values[idx] += write_value` |
> | `OP_MULTIPLY` | `Identity` | `ScaleTarget` *(exists, impl-needed)* | `values[idx] *= write_value` |
> | `OP_SET` | `Identity` | `ResetTarget` *(exists, impl-needed)* | `values[idx] = write_value` |
>
> `ConsumeMode::AddToTarget` is a new enum variant added in C-4. Existing
> C-3 `(Identity, None)` overlay-Add ops are migrated to `(Identity,
> AddToTarget)` in the same PR — removes the C-3 semantic hack where
> `Identity` was overloaded to mean "add to target" in the shader.
>
> **Legacy Pass 3 fallback shrinks to zero.** After C-4 lands and defaults
> on, S-3 deletes `overlay_prep.rs`'s atomic-write WGSL and the legacy
> overlay path. No mixed-batch fallback remains.

---

## 1. Scope

### In scope

- Full Add/Multiply/Set support in the AccumulatorOp overlay path
- Bit-exact preservation of legacy `Evaluator::evaluate_node` ordering
  (ancestor-before-local; same-cell ops in source-position order)
- Dirty cache that skips re-compilation + re-upload on overlay-unchanged
  ticks
- New `ConsumeMode::AddToTarget` enum variant (semantic cleanup)
- Shader implementations of `ScaleTarget` and `ResetTarget`
- Replacement of the C-3 `(Identity, None) ≡ add` convention with
  `(Identity, AddToTarget)`
- Removal of `OverlayAddPlan::FallbackNonAdd` — C-4 plans all overlay
  batches, no fallback
- B-4 world summary still triggered after the overlay pass when active
- Tests for parity, dirtiness, fission-clone, suspended-overlay,
  high-density no-recompile

### Out of scope (deferred PRs)

- `CombineFn::WeightedMean` / `Mean` reductions — C-5
- `CombineFn::Sum / Max / Min` reductions — C-6
- Velocity integration — C-7
- `EvalEML` / transfer / conjunctive — C-8 / E-family
- S-3 deletion of legacy overlay path — separate PR after C-4 default-on
- Cross-pool / hot-pool allocator — Phase D
- Per-(slot, band) granular dirty tracking — not needed; coarse revision
  is sufficient

### Pivot posture

```
AccumulatorOp path is the intended production path.
Legacy Pass 3 is oracle/fallback only.
This PR REMOVES the mixed-batch legacy fallback. After C-4, the only
remaining legacy interaction is the parity oracle in tests; S-3 deletes
the legacy overlay code entirely.

Sunset target: S-3 — delete `crates/simthing-gpu/src/overlay_prep.rs`,
delete the legacy Pass 3 WGSL, retire `OverlayDelta`/`SlotDeltaRange`
upload to the legacy GPU path.

Legacy interaction allowed: oracle only.
Legacy interaction forbidden: no new features, no optimization, no
semantic expansion.
```

---

## 2. Why reuse `build_overlay_deltas` instead of writing a new compiler

The user handoff framed C-4 as a "compiler that handles ancestor ordering,
fission clones, lifecycle, etc." That framing is misleading.
`build_overlay_deltas` in `crates/simthing-gpu/src/overlay_prep.rs`
**already does** every one of those things:

| Concern | Where it lives today |
|---|---|
| Ancestor-before-local ordering | `build_node` — passes `local_transforms` recursively |
| Active-overlay filtering (Suspended out, Permanent/Transient-active in) | `if !overlay.is_active() { continue; }` |
| Per-property gating (overlay only fires if node has the property) | `if !node.properties.contains_key(&transform.property_id) { continue; }` |
| Per-sub-field column resolution | `col_for_role(role, layout)` |
| Op-kind tagging | `OP_ADD / OP_MULTIPLY / OP_SET` written into `OverlayDelta.op_kind` |
| Fission clones (correct overlays propagate to clones) | Cloned subtree is part of the tree walk; ancestor stack is inherited via the parent's `local_transforms` |
| Slot allocation | Reads `allocator.slot_of(node.id)` |

The output `(Vec<OverlayDelta>, Vec<SlotDeltaRange>)` is the **canonical
compiled form of overlay state** — already deterministic, already correct,
already covers every case the user handoff listed. C-3's planner
(`plan_overlay_add_accumulator`) is a thin transform from this form to
`AccumulatorOpGpu` ops. C-4 extends that transform to cover Mul/Set; it
does NOT replace `build_overlay_deltas`.

This is the central design choice and it diverges from the user's
preliminary recommendation. The reasons:

1. **Correctness.** Re-implementing the tree walk in a per-(slot, band)
   cache is a vector for regressions. `build_overlay_deltas` has months of
   parity testing behind it.
2. **Lifecycle / fission concerns disappear.** They're handled upstream by
   the existing tree walk; the C-4 planner sees only the already-filtered
   delta list.
3. **The actual perf bottleneck is GPU upload, not CPU planner work.** At
   density=1.0 with 10k slots × 8 overlays/slot, the GPU buffer upload is
   ~5 MB; the planner walk is a few hundred microseconds. Caching the
   *upload* is what the workshop result demanded; per-(slot, band)
   granularity in the CPU cache solves the wrong cost.

---

## 3. Data flow

```text
SimThing tree mutations (boundary)
        │
        ▼
build_overlay_deltas(root, registry, allocator)   ← unchanged from today
        │
        ▼ (Vec<OverlayDelta>, Vec<SlotDeltaRange>)
        │
        ▼
plan_overlay_orderband(deltas, ranges, n_slots)   ← C-4 replaces plan_overlay_add_accumulator
        │
        ▼ Vec<AccumulatorOpGpu> + n_bands
        │
        ▼
WorldAccumulatorRuntime::upload_overlay_ops(ops, n_bands)
        │
        ▼
Pipelines::run_tick_pipeline_with_accumulator_overlay(...)
   - dispatches one band at a time via AccumulatorOp execute kernel
   - bands 0..n_bands in ascending order
   - same encoder as world pipeline (one submit per tick — C-1 pattern)
        │
        ▼
WorldSummaryRuntime encode_into(same encoder, values)
        │
        ▼
queue.submit()  → one submission per tick
```

The dirty cache wraps the top two stages: when no overlay-affecting event
happened, both `build_overlay_deltas` and `plan_overlay_orderband` are
skipped, and the previously-uploaded GPU op buffer is reused.

---

## 4. The OrderBand assignment algorithm

Identical pattern to C-3, extended to mixed op kinds. The `(deltas,
ranges)` list is already in the legacy evaluation order; we just need to
assign each delta a band such that **same-cell deltas execute in
source-position order** and **different-cell deltas can execute in
parallel**.

### Algorithm

```rust
pub fn plan_overlay_orderband(
    deltas:  &[OverlayDelta],
    ranges:  &[SlotDeltaRange],
    n_slots: u32,
) -> OverlayOrderBandPlan {
    let mut next_band: HashMap<(u32, u32), u32> = HashMap::new();
    let mut ops = Vec::with_capacity(deltas.len());
    let mut n_bands = 0u32;

    for slot in 0..n_slots as usize {
        if slot >= ranges.len() { break; }
        let range = ranges[slot];
        for i in range.offset as usize..(range.offset + range.length) as usize {
            if i >= deltas.len() { break; }
            let delta = deltas[i];
            let cell  = (slot as u32, delta.col);
            let band  = *next_band.get(&cell).unwrap_or(&0);
            next_band.insert(cell, band + 1);
            n_bands   = n_bands.max(band + 1);

            ops.push(make_overlay_op(cell.0, cell.1, delta.value, delta.op_kind, band));
        }
    }

    OverlayOrderBandPlan { ops, n_bands }
}
```

This is **structurally identical to `plan_overlay_add_accumulator`**
modulo the `op_kind` switch in `make_overlay_op`. There is no separate
fallback variant — every batch plans cleanly.

### Why per-cell bands work for mixed ops

The legacy Pass 3 applies ops in `delta_array` position order for each
cell, with later ops overwriting / scaling / adding to earlier ops' output.
By assigning each cell's `k`-th delta to `band = k`, and dispatching bands
in ascending order, we get the same effect:

- Band 0: first delta on every cell that has one (e.g. ancestor Add on
  cell A, ancestor Add on cell B). These execute in parallel because
  different cells.
- Band 1: second delta on every cell that has one. Cell A's local Mul,
  cell C's first ancestor Add (cell C wasn't touched in band 0 — fine).
- Band k: k-th delta per cell.

Within one band, no two ops target the same cell. Within a single cell,
band ordering is monotonic. Across cells, parallelism is preserved.

### Same-cell mixed op example

Source deltas (one slot, one col):
```
[0] OP_ADD       value=5    → band 0
[1] OP_MULTIPLY  value=2    → band 1
[2] OP_ADD       value=3    → band 2
[3] OP_SET       value=0    → band 3
```

GPU execution:
- Initial value `v0`
- Band 0 dispatch: `v1 = v0 + 5`
- Band 1 dispatch: `v2 = v1 * 2`
- Band 2 dispatch: `v3 = v2 + 3`
- Band 3 dispatch: `v4 = 0`

For `v0 = 1`: `v1=6, v2=12, v3=15, v4=0`. Matches `evaluate_node`'s
left-to-right transform application. Bit-exact for `f32` because no
reordering across bands and no atomic CAS races within a band.

---

## 5. CombineFn / ConsumeMode mapping

The shader needs to know *what to do with the target* per op. Three
options were considered:

### Option α (rejected): overload `combine_kind`

C-3 currently uses `CombineFn::Identity` to mean "add to target" in the
shader's `write_target`. This is a semantic hack: `Identity` should mean
"pass the value through unchanged." Extending the hack to also overload
`Product` for Mul and `LastByPriority` for Set works but compounds the
semantic debt — every future PR that touches the shader has to remember
the overload rules.

### Option β (rejected): per-op-kind `combine_kind` variants

Add `COMBINE_OVERLAY_ADD`, `COMBINE_OVERLAY_MUL`, `COMBINE_OVERLAY_SET`.
Works but clutters the `CombineFn` enum with overlay-specific variants
that have nothing to do with multi-input combination semantics. The
`CombineFn` namespace is reserved for "how do N gathered inputs collapse
to one write value" (ADR §AccumulatorOp v2 primitive).

### Option γ (selected): use `ConsumeMode` as the write-target semantic

`ConsumeMode` is documented (in `simthing-core::accumulator_op`) as "what
happens to the source after the write" — but two of its existing variants
already describe write-to-target behavior:

- `ConsumeMode::ScaleTarget` — "Multiply the target slot/col by the
  computed value. Used for Multiply overlays when the target is being
  scaled in place."
- `ConsumeMode::ResetTarget` — "Overwrite the target slot/col with the
  computed value (rather than adding). Used for Set overlays and clamp
  operations."

The third overlay op (Add) is missing. C-4 adds it:

- **New:** `ConsumeMode::AddToTarget` — "Add the computed value to the
  target slot/col. Used for Add overlays. Inverse of `ResetTarget`."

This is a minor `simthing-core` change with a clear semantic story.
Doc-update on `ConsumeMode` to explain that `None | ResetTarget |
ScaleTarget | AddToTarget` form a four-way write-to-target switch, and
`SubtractFromSource | SubtractFromAllInputs | EmitEvent` form the
side-effect-on-source axis. Mixing one of each is well-defined (e.g.
`combine: Sum, consume: ResetTarget` for a "compute sum, write to
target" reduction; `combine: Identity, consume: AddToTarget` for overlay
Add).

### Final mapping

| overlay op | `combine_kind` (WGSL) | `consume` (WGSL) | shader effect |
|---|---|---|---|
| `OP_ADD` | `IDENTITY` | `ADD_TO_TARGET` *(new)* | `values[idx] += write_value` |
| `OP_MULTIPLY` | `IDENTITY` | `SCALE_TARGET` *(impl C-4)* | `values[idx] *= write_value` |
| `OP_SET` | `IDENTITY` | `RESET_TARGET` *(impl C-4)* | `values[idx] = write_value` |

`make_overlay_op` constructs each `AccumulatorOpGpu` accordingly. Source
is `Constant(delta.value)` (bit-pattern in `source_slot`), gate is
`OrderBand(band)`, scale is `Identity`, single target `(slot, col)`.

### Migration of C-3's existing Add ops

In the same PR, the C-3 planner is **deleted** (replaced by the new C-4
planner). All Add ops now produce `(Identity, AddToTarget)` registrations
instead of `(Identity, None)`. The shader's old `write_target` branch
that treated `Identity` as "add" is removed; `Identity` now means
"assign," which matches its semantic across the rest of the codebase.

The bootstrap/intent/threshold paths that use `(Identity, None)` for
"assign to target" are unaffected because they always write to a target
whose existing value isn't read (intent deltas, threshold ops never write
through `write_target` because they go to the emission buffer).

If any bootstrap test starts failing because it relied on the
`Identity ≡ add` hack, the test was already wrong — fix it to use
explicit `AddToTarget`.

---

## 6. The dirty cache

### Tier 1: revision counter

Add to `BoundaryProtocol`:

```rust
pub struct BoundaryProtocol {
    // ... existing fields ...

    /// Bumped on every boundary mutation that can change the output of
    /// `build_overlay_deltas`. Compared against the C-4 cache's
    /// `compiled_at_revision` to decide whether to re-run the overlay
    /// compile pipeline. See `c4_overlay_orderband_compiler_design.md`
    /// for the invalidation rules.
    overlay_compile_revision: u64,
}
```

**Events that bump `overlay_compile_revision`:**

| Event | Where |
|---|---|
| `LifecycleOutcome.dissolved > 0` | After `resolve_overlay_lifecycle` |
| `LifecycleOutcome.overlays_attached > 0` | After `apply_structural_mutations` processes `AttachOverlay` |
| `LifecycleOutcome.after_ticks_decremented > 0` AND any dissolved | Same step — counted in `dissolved` |
| `FissionOutcome.fissions_executed > 0` | After fission step |
| `FissionOutcome.fusions_executed > 0` | After fusion step |
| `ExpiryOutcome.properties_removed > 0` | After expiry step |
| `ExpiryOutcome.cpu_side_removals > 0` | After expiry step |
| Any `BoundaryRequest::AddChild`, `Remove`, `ActivateOverlay`, `SuspendOverlay` processed | After `apply_structural_mutations` |
| Any property added via boundary request | Same |
| Initial sync (first `initial_gpu_sync` call) | Always — cache is unset |
| `WorldGpuState::rebuild` (dimension change, slot grow) | Cache must be invalidated explicitly because GPU buffers were re-created |

**Events that do NOT bump:**

- `LifecycleOutcome.after_ticks_decremented` alone (no dissolve) — the
  remaining counter changed but `is_active()` did not, so
  `build_overlay_deltas` produces the same output. Verify by writing a
  test that decrements an `AfterTicks { remaining: 5 }` overlay across
  multiple ticks without expiry and asserts no recompilation.
- Property values mutated by intent/velocity/overlay (the values change,
  the structure does not).
- Threshold events fired (those go to the spec handler, not the overlay
  compile output).

### Tier 2: equality check

Even when the revision bumps, the compile output may be identical to the
cached version (e.g. an `AttachOverlay` that targeted a slot with no
matching property — no delta emitted). Defense in depth:

```rust
struct OverlayCompileCache {
    compiled_at_revision: u64,
    cached_deltas:        Vec<OverlayDelta>,
    cached_ranges:        Vec<SlotDeltaRange>,
    cached_n_bands:       u32,
    cached_op_buffer_uploaded_n_ops: u32,  // for dispatch
}
```

When `boundary.overlay_compile_revision != cache.compiled_at_revision`:
1. Re-run `build_overlay_deltas` → `(new_deltas, new_ranges)`.
2. Compare `new_deltas` and `new_ranges` against `cached_deltas` and
   `cached_ranges` (bytewise `Vec` equality on `Pod` types — fast).
3. If equal: bump `cache.compiled_at_revision`, skip the planner, skip
   the upload. The previously-uploaded GPU op buffer is still correct.
4. If different: run `plan_overlay_orderband`, upload the new ops via
   `runtime.upload_overlay_ops(ops, n_bands)`, update cache.

When `boundary.overlay_compile_revision == cache.compiled_at_revision`:
- Skip everything. The cached GPU buffer is used unchanged.

### Cache lives on `WorldAccumulatorRuntime`

Add to `WorldAccumulatorRuntime`:

```rust
pub struct WorldAccumulatorRuntime {
    // ... existing fields ...

    /// C-4 overlay compile cache. `None` until first overlay sync;
    /// reset to `None` by `clear_overlay_orderband`.
    pub overlay_compile_cache: Option<OverlayCompileCache>,
}
```

Reasons to put it here rather than `BoundaryProtocol`:
- The cache tracks GPU state (whether the op buffer's contents match the
  cached plan). `WorldAccumulatorRuntime` is the right owner of GPU
  session state.
- Disabling the overlay flag clears the runtime sessions; the cache
  should clear with them.
- `BoundaryProtocol` only owns the revision counter (CPU-side mutation
  tracking).

---

## 7. WGSL changes

> **READ §7.1 (ERRATUM) FIRST.** The original `write_target` sketch in
> §7.2 used plain `values[idx] = ...` assignment, which does not compile
> against the actual buffer type `array<atomic<i32>>`. Codex must use
> the corrected WGSL in §7.1.

### 7.1 ERRATUM — atomic buffer-type guidance for `write_target`

**Actual buffer types in `accumulator_op.wgsl` (as of 2026-05-25):**

```wgsl
@group(0) @binding(1) var<storage, read_write> values: array<atomic<i32>>;
@group(0) @binding(5) var<storage, read> previous_values: array<f32>;
```

`values` is `array<atomic<i32>>` — f32 values are stored bit-cast as i32
so that WGSL atomic ops can address them. `previous_values` is plain
`array<f32>` because the threshold scan only reads it, never writes.

**Existing helpers Codex must use (in the shader, already implemented):**

```wgsl
fn atomic_read_f32_at(idx: u32) -> f32 {
    return bitcast<f32>(atomicLoad(&values[idx]));
}

fn atomic_add_f32_at(idx: u32, val: f32) {
    let cell_ptr = &values[idx];
    loop {
        let old_bits = atomicLoad(cell_ptr);
        let new_bits = bitcast<i32>(bitcast<f32>(old_bits) + val);
        let result = atomicCompareExchangeWeak(cell_ptr, old_bits, new_bits);
        if (result.exchanged) { break; }
    }
}
```

C-4 must add two more helpers, scoped to **the per-band single-writer
invariant** that the OrderBand planner guarantees (one writer per `(slot,
col)` per dispatch). Because there is no in-dispatch contention on any
cell C-4 writes, **CAS loops are not required for Mul / Set / Add** —
plain `atomicLoad` + bitcast + arithmetic + `atomicStore` is race-free
within one dispatch, and meaningfully cheaper than the CAS loop that
`atomic_add_f32_at` uses.

```wgsl
/// Atomic store of an f32 value (bitcast to i32 for the underlying
/// atomic). Safe without CAS *only when the caller guarantees no other
/// thread in the current dispatch writes the same cell*. The C-4
/// OrderBand planner enforces this: one op per (slot, col) per band.
fn atomic_store_f32_at(idx: u32, val: f32) {
    atomicStore(&values[idx], bitcast<i32>(val));
}

/// Load-modify-store add. Same single-writer-per-dispatch precondition
/// as `atomic_store_f32_at`. Cheaper than `atomic_add_f32_at` because
/// no CAS loop. Codex MUST NOT use this from any path that does not
/// honor the OrderBand single-writer invariant.
fn atomic_add_single_writer_f32_at(idx: u32, val: f32) {
    let cell_ptr = &values[idx];
    let old = bitcast<f32>(atomicLoad(cell_ptr));
    atomicStore(cell_ptr, bitcast<i32>(old + val));
}

/// Load-modify-store multiply. Same single-writer-per-dispatch
/// precondition as the others.
fn atomic_mul_single_writer_f32_at(idx: u32, val: f32) {
    let cell_ptr = &values[idx];
    let old = bitcast<f32>(atomicLoad(cell_ptr));
    atomicStore(cell_ptr, bitcast<i32>(old * val));
}
```

**Corrected `write_target` for C-4 (use this; ignore §7.2's body):**

```wgsl
fn write_target(slot: u32, col: u32, write_value: f32, op: AccumulatorOpGpu) {
    let idx = linear_idx(slot, col);
    // ╔════════════════════════════════════════════════════════════════╗
    // ║ Single-writer-per-dispatch invariant (C-4 OrderBand planner): ║
    // ║   For every (band, slot, col), there is at most one op in the ║
    // ║   uploaded buffer. Each band is dispatched separately by the  ║
    // ║   pipeline; the implicit memory barrier between dispatches    ║
    // ║   makes the next band's load see the previous band's writes.  ║
    // ║   This is what licenses atomicLoad + atomicStore here instead ║
    // ║   of the CAS loop used by the multi-writer bootstrap path.    ║
    // ║ Debug-assert in plan_overlay_orderband enforces the invariant.║
    // ╚════════════════════════════════════════════════════════════════╝
    switch op.consume {
        case CONSUME_ADD_TO_TARGET: {
            atomic_add_single_writer_f32_at(idx, write_value);
        }
        case CONSUME_SCALE_TARGET: {
            atomic_mul_single_writer_f32_at(idx, write_value);
        }
        case CONSUME_RESET_TARGET: {
            atomic_store_f32_at(idx, write_value);
        }
        default: {
            // Identity / no-mutation-modifier consume modes write the
            // value through. Matches the bootstrap (Identity, None) path.
            atomic_store_f32_at(idx, write_value);
        }
    }
}
```

**For the C-3 Add-overlay migration in the same PR**: the existing
overlay-add path that calls `atomic_add_f32_at` (the CAS-loop helper)
should switch to `atomic_add_single_writer_f32_at` once the migrated
C-3 ops are dispatched as `(Identity, AddToTarget)` through the new
`write_target` path. The CAS-loop helper stays in the shader because
the multi-writer bootstrap path (B-2 contention) still uses it; C-4
must not delete it.

**Why not just use the CAS loop everywhere?** It works correctly under
the single-writer invariant (CAS will succeed on the first attempt,
because no other thread is racing), but it costs a `loop` construct
plus the `result.exchanged` branch per write. With ~n_active_overlays ×
n_slots writes per tick at density 1.0, that's measurable overhead
exactly in the path the workshop's high-density regression hit. The
single-writer helpers compile to one atomic load + one atomic store —
two driver-level ops, no loop. The performance delta is worth the
small amount of additional shader code.

### 7.2 (SUPERSEDED by 7.1) — original sketch, retained for context

The text below was the original §7 body before this erratum. **Do not
implement this sketch**; it omits the atomic buffer-type handling that
the actual shader requires. Kept inline so reviewers can see the
diff against the corrected version above.

Add constants:

```wgsl
const CONSUME_RESET_TARGET:   u32 = 6u;
const CONSUME_SCALE_TARGET:   u32 = 7u;
const CONSUME_ADD_TO_TARGET:  u32 = 8u;
```

(Verify the next available ordinal after the existing constants in the
shader; A-2's `simthing-core::ConsumeMode` discriminants must align with
these. Update `crates/simthing-gpu/src/accumulator_op/types.rs::consume_kind`
to match. **This bullet still applies** — both §7.1 and §7.2 need the
constants.)

~~Replace `write_target`:~~

```wgsl
// SUPERSEDED — `values` is atomic<i32>, not plain f32. Plain assignment
// does not compile. Use the §7.1 form instead.
fn write_target(slot: u32, col: u32, write_value: f32, op: AccumulatorOpGpu) {
    let idx = linear_idx(slot, col);
    switch op.consume {
        case CONSUME_ADD_TO_TARGET: { values[idx] = values[idx] + write_value; }
        case CONSUME_SCALE_TARGET:  { values[idx] = values[idx] * write_value; }
        case CONSUME_RESET_TARGET:  { values[idx] = write_value; }
        default:                    { values[idx] = write_value; }
    }
}
```

The pre-C-4 `if (op.combine_kind == COMBINE_IDENTITY) { values[idx] = values[idx] + write_value; }`
branch is **removed**. C-3 Add ops now reach this shader as `(Identity,
AddToTarget)` and dispatch into the `CONSUME_ADD_TO_TARGET` arm.
**(Still applies — the routing decision is correct; only the body is
updated by §7.1.)**

### 7.3 Verify per-band single-writer invariant

The planner's per-cell `next_band` HashMap guarantees one writer per
`(slot, col)` per band. Add a debug_assert at the end of
`plan_overlay_orderband`:

```rust
#[cfg(debug_assertions)]
{
    let mut seen: HashSet<(u32, u32, u32)> = HashSet::new();
    for op in &ops {
        let key = (op.gate_a, op.target0_slot, op.target0_col);
        assert!(seen.insert(key),
            "overlay OrderBand planner produced duplicate (band, slot, col): {key:?}");
    }
}
```

This is a correctness invariant; if it ever fires the planner has a bug,
**and the §7.1 single-writer helpers become unsafe** — the CAS-loop
helper would still be correct but the optimized helpers would race. Keep
the assert under `debug_assertions` to avoid runtime cost.

---

## 8. Implementation file map

```
crates/simthing-core/src/accumulator_op.rs
  - ConsumeMode: add `AddToTarget` variant
  - ConsumeMode doc: clarify the four-way write-target axis
                     (None, ResetTarget, ScaleTarget, AddToTarget)
                     vs the source-side-effect axis (SubtractFromSource,
                     SubtractFromAllInputs, EmitEvent)
  - bump the variant ordinal documentation in
    crates/simthing-gpu/src/accumulator_op/types.rs::consume_kind

crates/simthing-gpu/src/accumulator_op/types.rs
  - consume_kind module: add `ADD_TO_TARGET`, `RESET_TARGET`, `SCALE_TARGET`
  - Verify ordinals match `ConsumeMode` enum variants in simthing-core

crates/simthing-gpu/src/accumulator_op/encode.rs
  - encode_consume: add arms for ScaleTarget / ResetTarget / AddToTarget
  - validate_bootstrap_op: no change — overlay ops come through the C-4
    planner, not bootstrap_set
  - Existing `validate_no_contention`: no change — overlay ops are
    band-gated, the existing band-vs-band exclusion logic still applies

crates/simthing-gpu/src/overlay_add.rs
  - DELETE this file. Its contents (OverlayAddPlan, FallbackNonAdd,
    plan_overlay_add_accumulator) are replaced by the C-4 planner.
  - All call sites must be updated to use the new planner — see gpu_sync.rs
    below.

crates/simthing-gpu/src/overlay_orderband.rs   ← NEW
  - pub struct OverlayOrderBandPlan { pub ops: Vec<AccumulatorOpGpu>,
                                       pub n_bands: u32 }
  - pub fn plan_overlay_orderband(deltas, ranges, n_slots) -> OverlayOrderBandPlan
  - make_overlay_op(slot, col, value, op_kind, band) -> AccumulatorOpGpu
    - Switches consume by op_kind (OP_ADD → AddToTarget, etc.)
  - Re-export OverlayOrderBandPlan / plan_overlay_orderband from lib.rs
  - Move and adapt the existing C-3 unit tests; add same-cell mixed-op
    tests (see §9)

crates/simthing-gpu/src/lib.rs
  - Remove `pub use overlay_add::{OverlayAddPlan, plan_overlay_add_accumulator};`
  - Add `pub use overlay_orderband::{OverlayOrderBandPlan, plan_overlay_orderband};`

crates/simthing-gpu/src/shaders/accumulator_op.wgsl
  - Add CONSUME_* constants
  - Add three new helpers per §7.1: `atomic_store_f32_at`,
    `atomic_add_single_writer_f32_at`, `atomic_mul_single_writer_f32_at`.
    These are atomic load + atomic store (NOT CAS loops) because the
    OrderBand planner enforces one writer per (slot, col) per dispatch.
    Document the single-writer precondition inline. The existing CAS-loop
    helper `atomic_add_f32_at` STAYS (used by the multi-writer bootstrap
    path).
  - Replace write_target body with the §7.1 (NOT §7.2) form: switch on
    op.consume that dispatches into the three single-writer helpers
    plus `atomic_store_f32_at` for the default/assign case. `values` is
    `array<atomic<i32>>` — plain assignment does not compile.
  - Remove the old `if (op.combine_kind == COMBINE_IDENTITY) ... else ...`
    branch.
  - Migrate the existing C-3 overlay-add atomic_add_f32_at call site to
    atomic_add_single_writer_f32_at once C-3 ops are routed through the
    new (Identity, AddToTarget) consume mode.

crates/simthing-gpu/src/accumulator_op/runtime.rs
  - WorldAccumulatorRuntime: add `overlay_compile_cache: Option<OverlayCompileCache>`
  - Add OverlayCompileCache struct
  - Replace `upload_overlay_add_ops` with `upload_overlay_ops(ctx, ops, n_bands)`
    - Method body unchanged otherwise; just rename
  - `clear_overlay_add` → `clear_overlay_orderband`; clears cache too
  - `overlay_add_active` → `overlay_active`; reads from cache + handle
  - `overlay_add_bands` → `overlay_n_bands`

crates/simthing-sim/src/gpu_sync.rs
  - Replace `plan_overlay_add_accumulator` call site with the C-4 cache flow:
    1. If boundary.overlay_compile_revision == cache.compiled_at_revision: skip.
    2. Otherwise: build_overlay_deltas → compare against cache → maybe
       plan + upload.
  - Remove the FallbackNonAdd branch; C-4 plans every batch.
  - Legacy build_overlay_deltas upload to the legacy GPU path remains for
    oracle/parity tests only. Production runtime path goes through the
    AccumulatorOp planner.

crates/simthing-sim/src/boundary.rs
  - BoundaryProtocol: add overlay_compile_revision: u64
  - Bump revision after every event listed in §6 (Tier 1)
  - Initial value: 0; first sync sets cache.compiled_at_revision = 0
    via the equality-check path (which is also the all-empty case).
  - Pass revision into gpu_sync; gpu_sync compares against cache.

crates/simthing-sim/src/lib.rs
  - No changes expected unless a re-export pattern changes.

crates/simthing-gpu/src/passes.rs
  - `run_tick_pipeline_with_accumulator_overlay` (or equivalent
    integrated path used by the dispatcher): dispatch overlay bands
    0..n_bands in ascending order inside the same encoder as the rest
    of the world pipeline. One submit per tick.
  - Verify B-4 world summary still encodes after the overlay bands.

crates/simthing-feeder/src/dispatcher.rs
  - Verify the overlay session is taken/restored on the same .take()/.put()
    pattern as the other C-1/C-2 sessions.
  - Update any logging/counters that referenced `overlay_add_*` to the
    new names.

docs/accumulator_op_v2_production_plan.md
  - Update PR C-4 status to "Landed (this PR)"
  - Note: C-4 sunsets the C-3 fallback path; S-3 is now unblocked

docs/design_v7.md
  - §4.2: PipelineFlags entry `use_accumulator_overlay_mul_set` either
    folds into `use_accumulator_overlay_add` (rename to
    `use_accumulator_overlay`) OR stays separate. RECOMMENDATION: rename
    to `use_accumulator_overlay` and document that it gates the full
    Add/Mul/Set path. The separate `_add` and `_mul_set` flags were a
    pre-C-4 staging convenience that no longer applies.
  - §4.3: Pass 3 entry updated to "Migration C-3+C-4 landed, sunset S-3
    pending"

docs/agents.md
  - If it references the legacy overlay path or the C-3 fallback, update
    to reflect C-4 ownership.

docs/workshop/workshop_current_state.md
  - Update §2 (Landed) to add C-4
  - Update §2 (Open migration work) — C-4 done; S-3 next
  - Update §5 (Tests) — add C-4 test counts
```

---

## 9. Test plan (concrete cases for Codex)

### Unit tests (in `crates/simthing-gpu/src/overlay_orderband.rs`)

| Test name | Behavior |
|---|---|
| `c4_add_only_matches_c3_planner_output` | Same as `c3_add_only_batch_emits_one_op_per_add_delta`; ops match (modulo new `consume = AddToTarget`) |
| `c4_same_cell_add_mul_set_assigns_increasing_bands` | `[Add(5), Mul(2), Set(0)]` on one cell → bands 0, 1, 2; correct combine/consume mapping |
| `c4_different_cells_share_same_band_across_op_kinds` | `[Add on (0,0), Mul on (0,1), Set on (1,0)]` → all band 0 |
| `c4_ancestor_then_local_correct_band_order` | Build deltas list where index 0 is ancestor Mul, index 1 is local Add (same cell); ancestor → band 0, local → band 1 |
| `c4_empty_batch_emits_no_ops_one_band` | `n_bands = 0` for empty deltas |
| `c4_planner_no_duplicate_band_slot_col` | `debug_assert` invariant fires under intentional bug |

### Integration parity tests (in `crates/simthing-sim/tests/`)

| Test file | What |
|---|---|
| `c4_overlay_orderband_parity.rs::add_only_matches_legacy` | Drive 100 ticks of Add-only overlays through both legacy Pass 3 and C-4; assert bit-identical post-tick `values` via `f32::to_bits()` |
| `c4_overlay_orderband_parity.rs::mul_only_matches_legacy` | Same with Multiply overlays |
| `c4_overlay_orderband_parity.rs::set_only_matches_legacy` | Same with Set overlays |
| `c4_overlay_orderband_parity.rs::mixed_add_mul_set_matches_legacy` | Same with mixed batch — the central parity test |
| `c4_overlay_orderband_parity.rs::ancestor_local_mixed_matches_legacy` | Tree with ancestor Mul + local Add on the same column; same-cell order preserved |
| `c4_overlay_orderband_parity.rs::suspended_overlay_absent_then_activated` | Overlay starts `Suspended`; assert no delta. Activate via `BoundaryRequest::ActivateOverlay`; assert delta appears next tick |
| `c4_overlay_orderband_parity.rs::transient_after_ticks_expiry_dirties_cache` | `AfterTicks { remaining: 3 }` overlay; assert cache hit on ticks 1, 2 (no expiry); assert dirty on tick 3 (expiry); assert overlay absent from delta after expiry |
| `c4_overlay_orderband_parity.rs::fission_clone_inherits_overlays_correctly` | Fission a node with ancestor overlays; assert cloned child gets the same compiled deltas as a sibling that already had the parent's overlays |
| `c4_overlay_orderband_parity.rs::combined_c1_c2_c3_c4_still_passes` | Threshold + intent + overlay all on; full pipeline still produces bit-identical events and values vs the all-legacy path |

### Dirty-cache tests

| Test name | What |
|---|---|
| `c4_no_change_tick_does_not_recompile` | Run a tick, snapshot `overlay_compile_revision` and cache generation, run another tick with no mutations, assert revision/generation unchanged, assert `plan_overlay_orderband` was NOT called (via a test-only counter or by instrumenting the runtime) |
| `c4_after_ticks_decrement_alone_does_not_recompile` | Transient overlay with `AfterTicks { remaining: 5 }`; tick once; assert revision did not bump (it decremented but didn't dissolve); cache still valid |
| `c4_overlay_attach_bumps_revision` | Process `AttachOverlay` request; assert revision bumped |
| `c4_overlay_dissolve_bumps_revision` | Transient with `AfterTicks { remaining: 0 }`; assert revision bumped after lifecycle pass |
| `c4_fission_bumps_revision` | Trigger fission; assert revision bumped |
| `c4_equality_check_skips_upload_when_deltas_unchanged` | Bump revision artificially via a no-op mutation that doesn't change `build_overlay_deltas` output; assert tier-1 dirty, tier-2 equal, no GPU upload |

### High-density guard

| Test name | What |
|---|---|
| `c4_high_density_unchanged_no_recompile_no_upload` | 1000 slots × 8 overlays per slot (density 1.0); run 50 ticks with no mutations; assert ONE compile and ONE GPU upload over the entire 50-tick run |
| `c4_high_density_single_attach_recompiles_once` | Same setup; tick 25 attaches one new overlay; assert exactly two compiles and two uploads across 50 ticks |

### B-4 summary compatibility

| Test name | What |
|---|---|
| `c4_world_summary_matches_full_values_after_mixed_overlay` | After a mixed Add/Mul/Set tick, the world summary's `checksum_all` matches CPU oracle computed from the full values readback. Verifies the integrated single-submit encoder still encodes the summary AFTER the overlay bands. |

### Run-family-oracle harness

The user handoff recommended using `run_family_oracle` where possible.
For the new tests above, use the harness for the parity comparisons
(legacy vs AccumulatorOp). Do not block C-4 on refactoring C-1/C-2/C-3
tests — that's tracked as the "oracle refactor" item.

---

## 10. Sunset criteria for S-3

After C-4 merges and the flag defaults on for one release cycle:

```
S-3 PR checklist:
- [ ] use_accumulator_overlay defaulted to `true`
- [ ] CI green at flag=on for 7+ days
- [ ] All parity tests still pass with flag=on (legacy path is the
      oracle, not the runtime)
- [ ] Delete:
      - crates/simthing-gpu/src/overlay_prep.rs
      - crates/simthing-gpu/src/shaders/<legacy overlay WGSL>
      - WorldGpuState fields: overlay_deltas, slot_delta_ranges buffers
      - Pipelines field: overlay_pipeline + bind group layout
      - gpu_sync.rs: legacy overlay upload branch
- [ ] Update design_v7.md §4.3 to remove the Pass 3 entry
- [ ] Add SUPERSEDED annotation to design_v6.md §10 Pass 3 entry
```

C-4 does NOT do S-3 deletion. C-4's job is to make S-3 mechanical.

---

## 11. Evaluating the user's preliminary `OverlayCompileCache` recommendation

The user handoff proposed:

```rust
struct OverlayCompileCache {
    slot_bands: Vec<SlotBandState>,
    dirty_slots: BitSet,
    dirty_slot_bands: BitSet,
    overlay_to_slots: HashMap<OverlayId, SmallVec<[SlotId; N]>>,
    generation: u64,
}

struct SlotBandState {
    bands: Vec<CompiledOverlayBand>,
    generation: u64,
}
```

**Decision: rejected** in favor of the simpler revision-counter + equality
check design in §6. Reasons:

1. **Partial recompilation is a correctness hazard.** An overlay added at
   depth 3 affects the ancestor stack of every descendant — recompiling
   "only the affected slot" would either miss those descendants or
   require tracking a subtree's worth of state. `build_overlay_deltas`
   walks the whole tree precisely because the ancestor model demands it.
2. **`overlay_to_slots` requires its own walk to maintain.** Every overlay
   attach / dissolve / suspend has to update the map; that walk is the
   cost we were trying to avoid.
3. **`dirty_slot_bands` solves the wrong cost.** The high-density
   regression the workshop measured was driven by GPU dispatch overhead
   and (probably) buffer upload, not by CPU-side planner work. Coarse
   dirty caching of the *upload* solves that; per-(slot, band) CPU
   tracking does not.
4. **`SmallVec`-per-overlay overhead is not trivial.** At 10k+ slots and
   density=1.0 with multiple overlays per slot, the `overlay_to_slots`
   map carries ~tens of thousands of entries with associated SmallVec
   payloads. The revision-counter design has O(1) state.
5. **Correctness audit surface.** The revision-counter design's
   invariant is "every overlay-affecting event bumps the revision" — one
   sentence, auditable by reviewing the §6 event list. The
   per-(slot, band) design's invariant is "every overlay-affecting event
   correctly identifies and dirties exactly the affected (slot, band)
   pairs" — much harder to audit and to keep correct as the codebase
   evolves.

**If profiling after C-4 lands shows the equality-check is the bottleneck
at extreme scale (>100k slots, density=1.0), the right next step is to
add a hash-of-output check instead of a byte-vec compare** — still
simpler than per-(slot, band) tracking, with most of the same win.

---

## 12. Composer/Codex handoff checklist

For the Codex 5.5 implementer:

```
Pivot posture:
  AccumulatorOp overlay path is the intended production path.
  Legacy Pass 3 is oracle/fallback only. After C-4 lands, the mixed-batch
  legacy fallback is REMOVED; S-3 deletes the legacy code.

Sunset target:
  S-3 — delete legacy overlay prep + WGSL after C-4 default-on validation.

Legacy interaction allowed:
  Oracle / parity tests only.

Legacy interaction forbidden:
  no new features · no optimization · no semantic expansion.

Acceptance gates:
  [ ] All §9 tests pass on three consecutive runs
  [ ] No regression in C-1, C-2, C-3 parity tests
  [ ] Cache hit observed on no-change ticks (high-density test asserts this)
  [ ] Single submit per tick preserved (verify via test that submits ==
      tick count when overlay flag is on)
  [ ] World summary still bit-exact after mixed overlay tick
  [ ] Zero warnings, debug + release clean
  [ ] `docs/design_v7.md` §4.2 and §4.3 updated per §8 file map
  [ ] `docs/accumulator_op_v2_production_plan.md` C-4 status set to Landed
  [ ] `docs/workshop/workshop_current_state.md` §2 and §5 updated
  [ ] Production-plan handoff template fields in PR description filled in
```

---

## 13. Non-goals (explicit)

C-4 does NOT:

- Add `CombineFn::WeightedMean` / `Mean` to the kernel (C-5)
- Add reduction `Sum / Max / Min` paths (C-6)
- Touch velocity integration (C-7)
- Touch EML / transfer / conjunctive emission (C-8 / E-family)
- Implement S-3 deletion
- Default the flag to `true` (separate PR)
- Modify the legacy Pass 3 WGSL or `overlay_prep.rs` (read-only)
- Change `build_overlay_deltas` (used as-is)
- Add per-(slot, band) granular dirty tracking (revision counter +
  equality check is the C-4 cache; finer granularity is a deferred
  follow-up if profiling demands it)
- Modify A-4's `SoftAggregateGuard` or its enforcement
- Modify B-4's `SlotSummaryGpu` shape

---

## 14. Sign-off checklist (this memo)

- [x] Read pivot-forward policy and aligned recommendations to it
- [x] Read `workshop_current_state.md`, ADR, design_v7, production plan C-4
- [x] Read C-3 planner (`overlay_add.rs`) and `build_overlay_deltas`
- [x] Read `WorldAccumulatorRuntime` for cache-placement decisions
- [x] (1) Choose compiler shape — §2 (reuse `build_overlay_deltas`)
- [x] (2) Data structures — §3, §6 (`OverlayCompileCache`, revision counter)
- [x] (3) Dirtiness rules — §6
- [x] (4) Fission / clone / structural mutation handling — §6 (inherited
      via `build_overlay_deltas` + revision bump on structural events)
- [x] (5) Suspended overlay handling — already correct upstream; §9 test asserts
- [x] (6) Mixed Add/Mul/Set OrderBand semantics — §4, §5
- [x] (7) CombineFn / ConsumeMode mapping — §5 (Option γ selected)
- [x] (8) Parity tests — §9
- [x] (9) High-density guard tests — §9
- [x] (10) C-4 implementation boundaries + S-3 deletion criteria — §10, §13
- [x] Evaluated user's preliminary recommendation — §11 (rejected, justified)
- [ ] Human sign-off on the design (this PR requests it)

---

## References

- `docs/adr_accumulator_op_v2.md` — semantic scope, sunset policy
- `docs/design_v7.md` §2 (constitution), §4.2 (flags), §4.3 (pass entries)
- `docs/accumulator_op_v2_production_plan.md` PR C-4
- `docs/workshop/pivot_forward_implementation_policy.md` — legacy posture
- `docs/workshop/workshop_current_state.md` — current routing
- `docs/workshop/slot_summary_b4_design.md` — B-4 compatibility
- `crates/simthing-gpu/src/overlay_prep.rs` — `build_overlay_deltas` (read-only)
- `crates/simthing-gpu/src/overlay_add.rs` — C-3 planner (this PR replaces)
- `crates/simthing-gpu/src/accumulator_op/runtime.rs` — `WorldAccumulatorRuntime`
- `crates/simthing-sim/src/gpu_sync.rs` — overlay sync block
- `crates/simthing-sim/src/boundary.rs` — `BoundaryProtocol`
