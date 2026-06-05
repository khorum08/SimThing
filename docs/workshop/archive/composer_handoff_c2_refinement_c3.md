# Cursor Composer Handoff: C-2 Refinements + C-3 Overlay Add GPU-Resident Migration

## Goal

Two parts, in order:

**Part 1 — C-2 refinements:** Fix three real bugs found after the C-2 intent
migration merged. These are genuine correctness issues.

**Part 2 — C-3:** Migrate `TransformOp::Add` overlays to AccumulatorOp using
the GPU-resident atomic path. One `AccumulatorOp` registration per overlay
delta, dispatched in parallel via `atomic_add_f32`. No CPU-side folding. No
fallback when Multiply/Set are present — those op kinds stay on old Pass 3
while Add moves to the AccumulatorOp session in the same tick.

Do not delete old shaders or old pass code. Old paths remain until S-phase
sunset.

---

## Current state

Already merged:
- **C-1:** Threshold scan → AccumulatorOp behind `use_accumulator_threshold_scan`
- **C-2:** Intent delta → AccumulatorOp behind `use_accumulator_intent`, using
  `COMBINE_AFFINE_INTENT` (`value = value * mul + add`). CPU fold unchanged.

`world_state.rs` already has:
- `overlay_add_accumulator: Option<AccumulatorOpSession>`
- `accumulator_overlay_add_active: bool`
- `ensure_overlay_add_accumulator()`
- `clear_accumulator_sessions()` called in `rebuild_for_registry` and
  `rebuild_for_slots`
- `c2_registry_growth_recreates_accumulator_sessions` test passing

---

## Part 1 — C-2 refinements

### Fix 1: Add integrated intent timestamp finish

**Problem:** `run_tick_pipeline_with_accumulators()` calls
`finish_threshold_scan()` but not the equivalent for intent. The intent
session's `last_pass_time_us()` never updates.

**Fix:** Add to `AccumulatorOpSession`:

```rust
/// Finish the intent timestamp query if timestamps are supported.
/// Call immediately after the submission that drove encode_intent_into.
pub fn finish_intent(&mut self, ctx: &GpuContext) {
    self.read_execute_pass_timestamp(ctx);
}
```

Then in `Pipelines::run_tick_pipeline_with_accumulators()`, after
`run_tick_pipeline_internal(...)`:

```rust
if let Some(session) = sessions.intent.as_mut() {
    session.finish_intent(&state.ctx);
}
if let Some(session) = sessions.threshold.as_mut() {
    session.finish_threshold_scan(&state.ctx);
}
```

**Test:**

```rust
#[test]
fn c2_integrated_intent_timestamp_finishes_when_supported() {
    // ... run one tick with use_accumulator_intent = true ...
    let session = state.intent_accumulator.as_ref().unwrap();
    if session.timestamp_supported() {
        assert!(session.last_pass_time_us().is_some());
    } else {
        assert_eq!(session.last_pass_time_us(), None);
    }
}
```

---

### Fix 2: Surface AccumulatorOp threshold readback errors

**Problem:** Current code silently swallows readback errors:

```rust
state
    .threshold_accumulator
    .as_mut()
    .and_then(|s| s.readback_threshold_events(&state.ctx).ok())
    .unwrap_or_default()
```

Overflow or readback failure becomes "no events." This is silent data loss.

**Fix — structured error on `TickOutcome`:**

Find `TickOutcome`. Add:

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TickGpuError {
    AccumulatorThresholdReadback(String),
}
```

Add to `TickOutcome`:

```rust
pub gpu_error: Option<TickGpuError>,
```

Update all `TickOutcome` construction sites with `gpu_error: None`.

In `DispatchCoordinator::tick()`:

```rust
let mut gpu_error = None;

let events = if use_accumulator_threshold {
    match state.threshold_accumulator.as_mut() {
        Some(s) => match s.readback_threshold_events(&state.ctx) {
            Ok(events) => events,
            Err(err) => {
                gpu_error = Some(TickGpuError::AccumulatorThresholdReadback(
                    err.to_string(),
                ));
                Vec::new()
            }
        },
        None => Vec::new(),
    }
} else {
    /* existing path */
};
```

Include `gpu_error` in `TickOutcome`.

**Acceptable fallback if `TickOutcome` modification is too invasive:**

```rust
.expect("AccumulatorOp threshold event readback failed")
```

Still add the `TickGpuError` type — it will be wired properly in a follow-up.

**Test:**

```rust
#[test]
fn c1_threshold_accumulator_readback_error_surfaces_in_tick_outcome() {
    // Cause a readback error (e.g., trigger overflow by underallocating
    // emission_capacity), assert TickOutcome.gpu_error is Some.
}
```

---

### Part 1 acceptance

```powershell
cargo test -p simthing-gpu accumulator_op -- --nocapture
cargo test -p simthing-sim c2_intent -- --nocapture
cargo test -p simthing-sim c1_threshold -- --nocapture
cargo check --workspace
```

Expected: C-1 and C-2 parity still pass. Intent timestamp finishes when
supported. Threshold readback errors are no longer silently swallowed.

---

## Part 2 — C-3 Overlay Add GPU-Resident Migration

### The architectural intent — READ THIS FIRST

The AccumulatorOp kernel uses `atomic_add_f32` for all writes. Multiple
overlay `Add` deltas selection the same `(slot, col)` dispatch in parallel and
the CAS loop resolves contention. **This is by design. Do not fold Add deltas
on the CPU before upload.** The workshop transfer/emission contention tests
validated this exact pattern — 100k factory queues each with hundreds of
concurrent atomic adds to faction pool columns — and conservation held exactly.

One `AccumulatorOp` registration per overlay delta. Same number of registrations
as the old `OverlayDelta` array. Let the GPU sort out the same-cell writes via
atomics.

### Overlay ordering

Upload in the **same tree-walk order** that `build_overlay_deltas` produces
today — ancestor overlays before local overlays. This preserves f32
non-associativity parity in existing tests. Iterate the flat `overlay_deltas`
array and `slot_delta_ranges` in slot order, emit one `AccumulatorOp` per
`OverlayDelta` where `op_kind == OP_ADD`, preserving array order exactly.

### Multiply and Set are NOT affected

`TransformOp::Multiply` and `TransformOp::Set` overlays stay on old Pass 3
regardless of the flag. When the flag is on:

- Add overlays → `overlay_add_accumulator` session (AccumulatorOp, atomic)
- Multiply and Set overlays → filtered to old Pass 3

Both run in the same tick, in this order:
1. AccumulatorOp overlay Add dispatch (before reduction)
2. Old Pass 3 with Multiply/Set only

This preserves Add-before-Multiply semantics. If there are no Multiply/Set
overlays, old Pass 3 is a no-op (zero-length delta array — already handled).

**No fallback, no mixed-batch detection.** The separation is unconditional
when the flag is on.

---

### Implementation

#### 1. Add `use_accumulator_overlay_add` flag

Find `PipelineFlags`. Add:

```rust
/// C-3: Route TransformOp::Add overlays through AccumulatorOp instead of
/// old Pass 3. Multiply and Set overlays still use old Pass 3 in the same tick.
/// Default: false.
pub use_accumulator_overlay_add: bool,
```

#### 2. Add encoder method for overlay Add

Add to `AccumulatorOpSession`:

```rust
/// Encode overlay Add ops into the command encoder at the overlay position.
/// Does NOT submit — caller owns the encoder and submits with other passes.
/// Returns Ok(()) immediately if ops is empty.
pub fn encode_overlay_add_into(
    &mut self,
    ctx: &GpuContext,
    encoder: &mut wgpu::CommandEncoder,
    ops: &[AccumulatorOpGpu],
    values: &wgpu::Buffer,
    label: &'static str,
) -> Result<(), AccumulatorOpSessionError>
```

#### 3. Build Add-only AccumulatorOp registrations

Add a free function:

```rust
/// Convert flat overlay_deltas + slot_delta_ranges into one AccumulatorOpGpu
/// per OverlayDelta where op_kind == OP_ADD. Preserves slot/delta traversal
/// order. Multiply and Set deltas are skipped.
pub fn build_overlay_add_ops(
    deltas: &[OverlayDelta],
    ranges: &[SlotDeltaRange],
    n_slots: u32,
) -> Vec<AccumulatorOpGpu> {
    let mut ops = Vec::new();
    for slot in 0..n_slots as usize {
        let range = ranges[slot];
        for i in range.offset as usize..(range.offset + range.length) as usize {
            let delta = deltas[i];
            if delta.op_kind != OP_ADD {
                continue;
            }
            ops.push(AccumulatorOpGpu {
                source_kind:  0,           // Constant
                combine_fn:   0,           // Identity
                combine_p0:   delta.value, // add value in combine_p0
                gate_kind:    4,           // OrderBand
                gate_band:    0,
                scale_kind:   0,
                scale_value:  1.0,
                consume_mode: 0,           // None
                target_slots: [slot as u32, 0, 0, 0],
                target_cols:  [delta.col, 0, 0, 0],
                target_count: 1,
                ..Default::default()
            });
        }
    }
    ops
}
```

Verify `combine_p0` is where the WGSL kernel reads the constant value for
`source_kind=0`. If it uses a different field, update to match.

#### 4. Build Multiply/Set-only overlay batch for old Pass 3

```rust
/// Filter overlay_deltas to Multiply and Set only.
pub fn filter_multiply_set_deltas(
    deltas: &[OverlayDelta],
    ranges: &[SlotDeltaRange],
    n_slots: u32,
) -> (Vec<OverlayDelta>, Vec<SlotDeltaRange>) {
    let mut new_deltas = Vec::new();
    let mut new_ranges = vec![SlotDeltaRange::default(); n_slots as usize];
    for slot in 0..n_slots as usize {
        let range = ranges[slot];
        let start = new_deltas.len() as u32;
        for i in range.offset as usize..(range.offset + range.length) as usize {
            let delta = deltas[i];
            if delta.op_kind != OP_ADD {
                new_deltas.push(delta);
            }
        }
        new_ranges[slot] = SlotDeltaRange {
            offset: start,
            length: new_deltas.len() as u32 - start,
        };
    }
    (new_deltas, new_ranges)
}
```

#### 5. Pipeline integration — the compute pass split

**This is the critical correctness requirement.** AccumulatorOp Add must
execute at the overlay position — after intensity, before reduction.

Encoder order (single command buffer, one submit):

```
1. [optional] AccumulatorOp intent encode_intent_into        (C-2, before snapshot)
2. Snapshot pass (copy_buffer_to_buffer)                     (Pass 0)
3. Velocity integration compute pass                         (Pass 1)
4. Intensity update compute pass                             (Pass 2)
5. [if use_accumulator_overlay_add]
      AccumulatorOp overlay_add encode_overlay_add_into      (C-3)
   old Pass 3 with filtered Multiply/Set deltas              (always; no-op if empty)
6. Reduction passes 4-6
7. [if use_accumulator_threshold_scan]
      AccumulatorOp threshold encode_threshold_into          (C-1)
   else old Pass 7
8. submit once
```

Do not add a second `queue.submit()`. All accumulator passes encode into the
same encoder. GPU executes in encoder order.

Extend `AccumulatorPipelineSessions`:

```rust
pub struct AccumulatorPipelineSessions<'a> {
    pub intent:      Option<&'a mut AccumulatorOpSession>,
    pub overlay_add: Option<&'a mut AccumulatorOpSession>,  // NEW
    pub threshold:   Option<&'a mut AccumulatorOpSession>,
}
```

#### 6. Session sync in BoundaryProtocol

Near overlay delta upload:

```rust
if self.flags.use_accumulator_overlay_add {
    state.ensure_overlay_add_accumulator();

    let add_ops = build_overlay_add_ops(&deltas, &ranges, state.n_slots);
    if let Some(session) = state.overlay_add_accumulator.as_mut() {
        session.upload_ops(&state.ctx, bytemuck::cast_slice(&add_ops))
               .expect("overlay Add op upload failed");
    }
    state.accumulator_overlay_add_active = !add_ops.is_empty();

    let (ms_deltas, ms_ranges) = filter_multiply_set_deltas(&deltas, &ranges, state.n_slots);
    state.upload_overlay_deltas(&ms_deltas, &ms_ranges);
} else {
    state.upload_overlay_deltas(&deltas, &ranges);
}
```

---

### C-3 tests

#### Unit tests

```rust
#[test]
fn c3_build_overlay_add_ops_produces_one_op_per_add_delta() {
    // 2 slots: slot 0 has 2 Add deltas, slot 1 has 1 Multiply delta.
    // Expected: 2 AccumulatorOpGpu entries for slot 0 only.
    let deltas = vec![
        OverlayDelta { col: 0, op_kind: OP_ADD,      value: 3.0, _pad: 0 },
        OverlayDelta { col: 1, op_kind: OP_ADD,      value: 1.5, _pad: 0 },
        OverlayDelta { col: 0, op_kind: OP_MULTIPLY, value: 2.0, _pad: 0 },
    ];
    let ranges = vec![
        SlotDeltaRange { offset: 0, length: 2 },
        SlotDeltaRange { offset: 2, length: 1 },
    ];
    let ops = build_overlay_add_ops(&deltas, &ranges, 2);
    assert_eq!(ops.len(), 2);
    assert_eq!(ops[0].target_slots[0], 0);
    assert_eq!(ops[0].target_cols[0], 0);
    assert_eq!(ops[1].target_cols[0], 1);
}

#[test]
fn c3_filter_multiply_set_retains_non_add() {
    let deltas = vec![
        OverlayDelta { col: 0, op_kind: OP_ADD,      value: 3.0, _pad: 0 },
        OverlayDelta { col: 1, op_kind: OP_MULTIPLY, value: 2.0, _pad: 0 },
        OverlayDelta { col: 2, op_kind: OP_SET,      value: 0.5, _pad: 0 },
    ];
    let ranges = vec![SlotDeltaRange { offset: 0, length: 3 }];
    let (ms_deltas, ms_ranges) = filter_multiply_set_deltas(&deltas, &ranges, 1);
    assert_eq!(ms_deltas.len(), 2);
    assert_eq!(ms_ranges[0].length, 2);
    assert!(ms_deltas.iter().all(|d| d.op_kind != OP_ADD));
}

#[test]
fn c3_empty_overlay_add_produces_no_ops() {
    let deltas = vec![OverlayDelta { col: 0, op_kind: OP_MULTIPLY, value: 2.0, _pad: 0 }];
    let ranges = vec![SlotDeltaRange { offset: 0, length: 1 }];
    let ops = build_overlay_add_ops(&deltas, &ranges, 1);
    assert!(ops.is_empty());
}

#[test]
fn c3_overlay_add_op_value_in_correct_field() {
    let deltas = vec![OverlayDelta { col: 2, op_kind: OP_ADD, value: 7.5, _pad: 0 }];
    let ranges = vec![SlotDeltaRange { offset: 0, length: 1 }];
    let ops = build_overlay_add_ops(&deltas, &ranges, 1);
    assert_eq!(ops.len(), 1);
    assert_eq!(ops[0].combine_p0, 7.5);  // constant value in combine_p0
    assert_eq!(ops[0].target_cols[0], 2);
}
```

#### GPU session test

```rust
#[test]
fn c3_overlay_add_accumulator_applies_add_to_values() {
    // slot 0, col 0: initial 10.0 + Add 3.5 = 13.5
    let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return; }
    // ... setup session, values buffer, op, encoder, encode, submit, readback ...
    assert!((result[0] - 13.5).abs() < 1e-5, "expected 13.5, got {}", result[0]);
}
```

#### Integration parity tests

File: `crates/simthing-sim/tests/c3_overlay_add_accumulator_parity.rs`

Run each scenario with `use_accumulator_overlay_add = false` (old path) and
`= true` (new path). Assert bit-exact GPU values after tick, except scenario 4
(same-cell contention) which asserts within `f32::EPSILON * n_overlays`:

1. No overlays
2. Single Add overlay
3. Parent Add + child Add — ordering preserved
4. Multiple Add overlays on same slot/col — atomic path; assert within tolerance
5. Multiple columns
6. Lifecycle inactive/suspended overlay filtered before both paths
7. Mixed Add + Multiply — Add goes AccumulatorOp, Multiply goes old Pass 3; final values identical
8. Mixed Add + Set — same as 7 with Set
9. Add-only batch — old Pass 3 is a no-op; values identical
10. Combined C-1 + C-2 + C-3 all active simultaneously

Test 4 is the key atomics correctness test. Tests 7 and 8 are the key
split-path tests.

---

### Documentation updates

**`docs/design_v7.md` §4.2:** Add `use_accumulator_overlay_add: bool, // C-3 → S-3`

**§4.3 Pass 3:** Add note:
> Post C-3: TransformOp::Add overlays route through AccumulatorOp
> (one registration per delta, atomic_add_f32, GPU-resident, parallel).
> Multiply and Set overlays continue on old Pass 3 until C-4. AccumulatorOp Add
> encodes before old Pass 3 Multiply/Set in the same command buffer, preserving
> Add-before-Multiply semantics.

**`docs/accumulator_op_v2_production_plan.md` C-3 entry:** Add:
> Implementation note: one AccumulatorOp per OverlayDelta (op_kind==OP_ADD),
> in original tree-walk order. Same-cell atomic contention handled by
> atomic_add_f32 — no CPU-side folding. Multiply/Set filtered to old Pass 3
> in same tick.

---

### C-3 acceptance

```powershell
cargo test -p simthing-gpu accumulator_op -- --nocapture
cargo test -p simthing-sim c2_intent -- --nocapture
cargo test -p simthing-sim c3_overlay_add -- --nocapture
cargo test -p simthing-sim c1_threshold_scan_parity -- --nocapture
cargo check --workspace
cargo test --workspace
```

Expected:
- Workspace green
- C-1, C-2, C-3 parity all pass
- Same-cell contention test (scenario 4) passes within tolerance
- Combined C-1 + C-2 + C-3 scenario passes
- Feature flag default false
- Multiply/Set use old Pass 3 regardless of flag
- Single submit — no extra `queue.submit()`
- No old shader deletion

---

### Out of scope

Do not implement:
- Overlay Multiply or Set via AccumulatorOp (C-4, requires Opus design)
- LifecycleActive GPU gate (CPU already filters inactive overlays before upload)
- CPU-side fold of Add deltas (explicitly rejected — atomics handle contention)
- Same-cell contention fallback (atomic_add_f32 handles it correctly)
- S-phase sunset, default-on flags, WeightedMean, EvalEML, velocity migration

---

### Reviewer checklist

**C-2 fixes:**
- [ ] Intent timestamp finishes after dispatch
- [ ] Threshold readback errors surface in TickOutcome (not swallowed)

**C-3:**
- [ ] Flag defaults false
- [ ] One AccumulatorOp per OverlayDelta with `op_kind == OP_ADD` — no CPU folding
- [ ] Multiply/Set filtered to old Pass 3
- [ ] AccumulatorOp Add encodes before old Pass 3 Multiply/Set in same encoder
- [ ] Single submit — no extra `queue.submit()`
- [ ] Same-cell contention test passes within tolerance
- [ ] C-1 and C-2 parity still pass
- [ ] design_v7.md and production_plan.md updated

**The main C-3 risk is execution placement.** AccumulatorOp overlay Add must
encode before reduction in the same command buffer. Verify with the combined
C-1 + C-2 + C-3 scenario test.
