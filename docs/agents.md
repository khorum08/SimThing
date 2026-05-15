# SimThing — Agent Briefing

This document is for AI agents picking up work on this project. Read it before touching any code.

---

## What this is

SimThing is a GPU-native grand strategy simulation kernel. The central idea: every entity in the
simulation — world, faction, star system, location, cohort — is the same recursive type (`SimThing`),
and the entire world state lives in GPU dense matrices that are evaluated continuously. The CPU
interprets GPU output as events; it does not drive the simulation.

The full design specification is in `docs/design_v4.md`. Read it. The key ideas are:

- **One type:** `SimThing { properties, overlays, children }`
- **One mechanism for change:** overlay a `PropertyTransformDelta` on a SimThing
- **One mechanism for differentiation:** intensity threshold in the registry
- **One place to edit any property:** the `DimensionRegistry`

If you find yourself adding a special case for "rebel cohorts" or "civil war state" or "ethics
system flags," stop. Those are properties with thresholds, not special cases.

---

## Repository layout

```
SimThing/
├── Cargo.toml                         workspace manifest
├── docs/
│   ├── design_v4.md                   complete architecture specification (read this)
│   ├── invariants.md                  non-negotiable code rules (read this too)
│   └── agents.md                      this file
└── crates/
    ├── simthing-core/
    │   └── src/
    │       ├── lib.rs                 public re-exports
    │       ├── ids.rs                 SimThingId, SimPropertyId, OverlayId
    │       ├── property.rs            PropertyValue, PropertyLayout, SubFieldSpec,
    │       │                          ClampBehavior, SubFieldRole, SimProperty,
    │       │                          IntensityBehavior, DecayBehavior, fission types
    │       ├── registry.rs            DimensionRegistry, PropertyColumnRange
    │       ├── overlay.rs             Overlay, PropertyTransformDelta, TransformOp
    │       ├── simthing.rs            SimThing, SimThingKind
    │       └── evaluate.rs            Evaluator, TransformStack, FieldSnapshot (CPU oracle)
    └── simthing-gpu/
        └── src/
            ├── lib.rs                 public re-exports
            ├── context.rs             GpuContext — device/queue/adapter init
            │                          new_blocking() and async new() entry points
            │                          primary backends (DX12 on Windows)
            └── world_state.rs         GovernedPair (#[repr(C)] Pod, 24 bytes)
                                       build_governed_pairs(&DimensionRegistry)
                                       WorldGpuState — owns GpuContext + 6 buffers
                                       upload/download helpers
```

Future crates (not yet created):
- `simthing-feeder` — feeder thread, work queue (Week 3)
- `simthing-sim` — day boundary orchestration, fission/fusion execution (Week 3)

---

## Current implementation state

**Week 1 complete. Week 2 in progress — WorldGpuState built, shaders not yet written.**

### simthing-core (complete)
- `PropertyLayout` fully declarative: `Vec<SubFieldSpec>` with computed stride
- `SubFieldSpec`: role, width, ClampBehavior, velocity_max, default, governed_by
- All index arithmetic in `PropertyLayout::offset_of` and `PropertyColumnRange::col_for_role`
- No global index constants — removed
- `PropertyValue::integrate` — governed_by driven, velocity pinning at boundaries (I3)
- `TransformStack::apply_to` and `PropertyTransformDelta::apply_to_data` take `&layout`
- 14 tests passing, zero warnings

`evaluate.rs::Evaluator` is the CPU reference oracle. GPU output must match it to the float bit.

### simthing-gpu (partial — WorldGpuState built, shaders pending)

**`context.rs` — `GpuContext`:**
- Device/queue/adapter init with `new_blocking()` and `async new()` entry points
- Primary backends (DX12 on Windows), default limits, no special features

**`world_state.rs`:**
- `GovernedPair` — `#[repr(C)]` Pod struct, 24 bytes:
  `(governed_col, governing_col, clamp_min, clamp_max, vel_max, clamp_kind)`
  Encodes `ClampBehavior` as u32 tag with sentinel `±INFINITY` for Floored/Unbounded
- `build_governed_pairs(&DimensionRegistry)` — walks active properties, skips tombstoned,
  emits one pair per sub-field with `governed_by: Some(_)`. Column resolution via `col_for_role` only (I1)
- `WorldGpuState` — owns `GpuContext` + 6 buffers:
  - `values`, `previous_values`, `output_vectors`: `n_slots × n_dims × 4B` each
  - `local_transforms`, `ancestor_transforms`: `n_slots × n_dims² × 4B` each
  - `governed_pairs`: `n_pairs × 24B`
  - All buffers: `STORAGE | COPY_SRC | COPY_DST`
  - Empty governed-pair set allocates one zeroed slot (bindable even with zero properties)
- Upload/download helpers: `write_values`, `read_values`, `read_previous_values`,
  `read_governed_pairs`. Read uses staging buffer + `map_async` + `device.poll(Maintain::Wait)`

**6 new tests, 20/20 total passing, zero warnings:**
- `governed_pairs_from_standard_layout` — amount↔velocity pair encoding
- `governed_pairs_skip_tombstoned_properties` — tombstoned props contribute zero pairs
- `governed_pairs_offset_across_multiple_properties` — multi-property column offsets
- `write_read_values_roundtrip` — values buffer bit-exact roundtrip
- `governed_pairs_upload_roundtrip` — pair buffer bit-exact roundtrip
- `empty_governed_pairs_buffer_is_bindable` — placeholder allocation works

**Not yet built in simthing-gpu:**
- `intensity_params` buffer (Pass 2 needs per-property velocity_threshold/build_coefficient/decay_coefficient)
- `EvaluationBatch` builder (CPU tree-walk → local_transforms/ancestor_transforms upload)
- WGSL shaders for Passes 0, 1, 2
- CPU-oracle parity harness
- `threshold_registry` + `event_candidates` (deferred to Pass 7)

**Not yet built in any crate:**
- Feeder thread (Week 3)
- Day boundary protocol execution (Week 3)
- Fission/fusion execution (Week 3)
- Player input handling (Week 4)

---

## How to run tests

```
cd C:\Users\mvorm\SimThing
cargo test
```

All 20 tests must pass with zero warnings before any commit.

**simthing-core tests:**
- `registry::column_assignment_is_contiguous` — column layout correctness
- `registry::col_for_role_multi_property` — global column arithmetic across properties
- `evaluate::velocity_integration` — amount evolves at velocity * dt
- `evaluate::ancestor_transform_propagates` — world-level overlay reaches cohort
- `evaluate::deterministic` — two identical evaluations produce bit-identical output
- `evaluate::snapshot_round_trip` — JSON serialize/deserialize is lossless
- `property::velocity_clamped_at_floor/ceiling` — velocity pinning at boundaries
- `property::integrate_mid_range_unchanged` — no spurious clamping mid-range
- `property::custom_layout_ethics_axis` — designer-defined layout, drift governor, width-3 vector

**simthing-gpu tests:**
- `world_state::governed_pairs_from_standard_layout`
- `world_state::governed_pairs_skip_tombstoned_properties`
- `world_state::governed_pairs_offset_across_multiple_properties`
- `world_state::write_read_values_roundtrip`
- `world_state::governed_pairs_upload_roundtrip`
- `world_state::empty_governed_pairs_buffer_is_bindable`

The `custom_layout_ethics_axis` test is the proof that the generalization works beyond the
standard amount/velocity/intensity layout. If you add a new layout capability, add a test in
this pattern.

---

## The invariants

`docs/invariants.md` has the full list. The ones most likely to be violated accidentally:

**I1:** Column arithmetic has exactly one home.
`PropertyLayout::offset_of` for local offsets. `PropertyColumnRange::col_for_role(role, layout)`
for global columns. Nothing else does column math. No exceptions.

**I3:** Velocity pinning at boundaries.
This is in `PropertyValue::integrate`. Don't move it. Don't add a flag to disable it.
Hidden velocity debt is not a feature.

**I4:** No index constants.
`AMOUNT_IDX`, `VELOCITY_IDX`, `INTENSITY_IDX`, `VECTOR_START_IDX` are banned.
Access sub-fields via `layout.offset_of(&SubFieldRole::Amount)`.

**I5:** Overlays use roles, not column indices.
`PropertyTransformDelta` stores `SubFieldRole`, not `usize`. Column resolution happens in the
CPU preparation pass at dispatch time.

**I7:** Structural mutations only at the day boundary.
This is not yet enforced programmatically (day boundary protocol is Week 3). Until then:
no test or benchmark should mutate the SimThing tree mid-evaluation.

---

## Design decisions already made — don't relitigate

**IntensityBehavior uses linear coefficients, not function pointers.**
Reason: function pointers don't serialize; linear coefficients map directly to WGSL uniforms.
If you need non-linear intensity dynamics, model it as a different property with a different
governed_by relationship, not as a function pointer.

**`SimProperty` equality and hashing are on namespace+name only.**
Reason: the registry key must be stable across layout changes (version migrations). Metadata
does not participate in key comparison.

**`stride()` is computed, not stored.**
Reason: eliminates the class of bugs where stored stride diverges from actual sub-field widths.

**Velocity pinning at floor/ceiling, not velocity clamping.**
Reason: velocity that pushes in the recovery direction must always be permitted through.
Only velocity that would push further into the already-saturated direction is zeroed.

**`GovernedPair` encodes `ClampBehavior` as a u32 tag with sentinel float values.**
Reason: WGSL structs must be `#[repr(C)]` with fixed-size fields. `ClampBehavior` is a Rust
enum which cannot be sent to the GPU directly. Encoding uses `clamp_kind: u32`
(0=Bounded, 1=Floored, 2=Unbounded) with `±INFINITY` sentinels in `clamp_min`/`clamp_max`
for the cases where bounds are not meaningful. The WGSL shader reads `clamp_kind` and branches.

**`threshold_registry` and `event_candidates` deferred to Pass 7.**
Reason: their shape depends on threshold registration (fission thresholds, velocity thresholds,
decay conditions) which doesn't exist yet. Adding empty placeholder buffers now produces
untestable dead code. Add them when threshold registration API is designed.

**`intensity_params` buffer not yet in `WorldGpuState`.**
Reason: Pass 2 needs per-property `velocity_threshold`, `build_coefficient`, `decay_coefficient`
from `IntensityBehavior`. This is a property-level buffer, not slot-level. Build it alongside
the Pass 2 shader. Do not add it to `WorldGpuState` before the shader exists.

---

## FMA divergence — decision required before writing Pass 1

WGSL allows `mul`+`add` fusion into FMA (fused multiply-add) at the compiler's discretion.
The Pass 1 integration expression `position + velocity * dt` may FMA-fuse on GPU but will
not on the CPU oracle (which uses standard sequential `f32` arithmetic). On some hardware
this produces 1-ULP divergence, which fails the `to_bits()` parity test (Invariant I8).

**Choose one approach before writing the Pass 1 shader. Do not defer this.**

**Option A — CPU uses `f32::mul_add` to match GPU FMA:**
Update `PropertyValue::integrate` to use `f32::mul_add(velocity, dt, current_value)`.
CPU oracle now produces FMA-equivalent results. GPU can fuse freely.
Pro: GPU runs at full hardware speed.
Con: CPU oracle no longer matches naive f32 arithmetic; may surprise future contributors.

**Option B — WGSL shader explicitly prevents FMA fusion:**
Write integration as two separate assignments: `let scaled = velocity * dt; position = position + scaled;`
WGSL spec: intermediate `let` bindings prevent FMA. GPU matches naive CPU f32.
Pro: CPU oracle needs no changes.
Con: marginally slower on FMA-capable hardware (negligible at this workload scale).

**Recommendation: Option B.** The performance difference is negligible. Explicit FMA prevention
is a one-line auditable shader decision. Changing the CPU oracle to use `mul_add` silently
alters the behavior of the authoritative reference path and may mask future precision bugs.

**Outcome (Week 2):** Option B implemented and bit-exact verified on naga + DX12.
The `velocity_integration_matches_cpu_oracle_fractional_dt` test stresses `dt = 0.5`
with non-power-of-2 inputs; `to_bits()` parity holds. If a future driver fuses despite
the `let` bindings, that test fails loudly and the fallback is `f32::mul_add` on the
CPU side + WGSL `fma()` in the shader.

---

## Transform application — iterative on GPU (decided)

`TransformOp::{Add, Multiply, Set}` is not a closed group under N×N matrix
multiplication. `Multiply(k)` is linear (diagonal entry `k`); `Add(k)` is a
translation (needs a bias term); `Set(k)` discards the input. An earlier draft
proposed affine `(M, b)` composition on the CPU prep pass with a single matmul
on the GPU. **That approach was considered and rejected.** Pass 3 instead
applies overlays **iteratively on the GPU**.

### Why iterative

- **Bit-exact parity is trivial.** Both `Evaluator::apply_to_data` and the
  Pass 3 shader walk a list of `(col, op, value)` deltas in stack order and
  apply each op the same way. No composition step means no rounding-order
  divergence. The `Evaluator` stays as-is.
- **Per-tick GPU work is proportional to active overlays, not `n_dims²`.**
  At realistic overlay loads (~10–20 deltas per slot's stack), iterative is
  ~10 ops/slot; the affine matmul would have been ~4096 ops/slot at `n_dims = 64`.
- **GPU memory plummets.** The affine path would have needed two
  `n_slots × n_dims²` matrix buffers and two `n_slots × n_dims` bias buffers
  — ~370 MB at endgame scale. Iterative replaces all of that with a flat
  delta array (~4 MB) and a per-slot range table (~90 KB).
- **Cross-property / cross-column transforms still work.** A future op
  variant that mixes columns (e.g. rotation, cross-property pressure) is a
  new `TransformOp` variant the shader branches on. Same flexibility as
  affine, less infrastructure.

The trade-off is variable per-thread work — slots with longer overlay stacks
run more iterations than others. At our scale this is fine; if it ever
matters, batch by stack length or pad to a fixed max.

### Data shape

```rust
#[repr(C)] #[derive(Pod, Zeroable)]
struct OverlayDelta {
    col:     u32,   // global column index (resolved via col_for_role at prep time)
    op_kind: u32,   // 0=Multiply, 1=Add, 2=Set
    value:   f32,
    _pad:    u32,   // align stride to 16 bytes
}

#[repr(C)] #[derive(Pod, Zeroable)]
struct SlotDeltaRange {
    offset: u32,   // index into overlay_deltas
    length: u32,   // number of deltas to apply for this slot
}
```

`overlay_deltas` is the flat concatenation of every slot's ancestor + local
stack, in evaluation order. `slot_delta_ranges` is indexed by `slot_idx`.
A slot with no overlays has `length = 0` and the shader is a no-op for it.

### CPU prep pass

```
fn build_overlay_deltas(root, registry, allocator) -> (Vec<OverlayDelta>, Vec<SlotDeltaRange>):
    walk tree depth-first carrying an ancestor stack of overlays
    for each node:
        slot = allocator.slot_of(node.id)
        record offset = deltas.len()
        for overlay in ancestor_stack:
            for (role, op) in overlay.transform.sub_field_deltas:
                col = registry.col_for_role(overlay.transform.property_id, role)
                deltas.push(OverlayDelta { col, op_kind, value })
        for overlay in node.overlays:
            ...same emission...
        record length = deltas.len() - offset
    return (deltas, ranges)
```

Mirrors `TransformStack` semantics exactly: ancestor overlays apply first, in
push order; then local overlays in registration order.

### Pass 3 shader (sketch)

```wgsl
@compute @workgroup_size(64)
fn pass_3(@builtin(global_invocation_id) gid: vec3<u32>) {
    let slot = gid.x;
    if (slot >= n_slots) { return; }
    let range = slot_delta_ranges[slot];
    let base = slot * n_dims;

    for (var i = 0u; i < range.length; i = i + 1u) {
        let d = overlay_deltas[range.offset + i];
        let addr = base + d.col;
        switch (d.op_kind) {
            case 0u: { values[addr] = values[addr] * d.value; }    // Multiply
            case 1u: { values[addr] = values[addr] + d.value; }    // Add
            case 2u: { values[addr] = d.value; }                    // Set
            default: { /* unreachable */ }
        }
    }
}
```

One thread per slot. Each thread walks its slot's delta range and applies
ops in place to `values`. Pass 3 reads from and writes to `values` —
`output_vectors` is unused for now and is a Pass 4–6 (reduction) concern.

### Buffer changes in `WorldGpuState`

The earlier matrix-based plan reserved `local_transforms` /
`ancestor_transforms` (each `n_slots × n_dims² × 4B`). Those buffers are
**removed** in favor of:

```
overlay_deltas      : Vec<OverlayDelta>          uploaded each tick
slot_delta_ranges   : Vec<SlotDeltaRange>        uploaded each tick
```

Both are `STORAGE | COPY_SRC | COPY_DST`. Empty cases get a placeholder
allocation so the buffers remain bindable.

---

## Week 2 scope (what to build next)

### Architecture note — governed_pairs is a separate buffer, not part of the transform matrices

Pass 1 (velocity integration) is a **pre-transform step**. It advances governed sub-fields
*before* the transform matrices in Pass 3 are applied. Do not fold `governed_by` pairs into
the transform matrix representation — that would conflict with Pass 3's transform application
and produce double-application on the same tick.

`EvaluationBatch` must carry a distinct `governed_pairs` buffer:

```rust
struct EvaluationBatch {
    base_vectors:      GpuMatrix,  // [N_slots × N_dims]
    overlay_deltas:    GpuBuffer,  // flat [OverlayDelta], ancestor stack then local, in evaluation order
    slot_delta_ranges: GpuBuffer,  // [N_slots × SlotDeltaRange { offset, length }]
    governed_pairs:    GpuBuffer,  // [(governed_col, governing_col, clamp_min, clamp_max, vel_max)]
    reduction_map:     GpuBuffer,
}
```

`overlay_deltas` and `slot_delta_ranges` replace the earlier matrix-based
`ancestor_xforms` / `local_xforms` plan. See "Transform application —
iterative on GPU" above for the reasoning.

`governed_pairs` is built from the `DimensionRegistry` during the CPU preparation pass by
iterating all active properties, finding sub-fields where `governed_by` is `Some`, and calling
`col_for_role` on both the governed and governing roles. It is a property-level buffer (same
pairs apply to every slot) — not a per-slot buffer. Pass 1 dispatches one thread per pair,
not one thread per (slot × pair); each thread handles all slots for its pair in a loop, or
alternatively dispatch is `(N_pairs × N_slots)` with the pair index in the workgroup.

The pass ordering is therefore:
```
Pass 0: snapshot
Pass 1: velocity integration     ← reads governed_pairs, writes values[]
Pass 2: intensity update          ← reads values[] (post-integration velocity)
Pass 3: transform application     ← reads overlay_deltas + slot_delta_ranges, writes values[] (in place)
Pass 4–6: reduction
Pass 7: threshold scan
```

---

Add `wgpu = "22"` and `rayon = "1"` to `[workspace.dependencies]` in `Cargo.toml`.

Create `crates/simthing-gpu/` with:

1. **`WorldGpuState`** — owns the wgpu device/queue and all GPU buffers:
   - `values`: `[slot * N_DIMS + col]` — current property values
   - `previous_values`: snapshot from Pass 0
   - `output_vectors`: per-slot output after reduction (Pass 4–6 destination)
   - `governed_pairs`: flat array of `(governed_col, governing_col, clamp_min, clamp_max, vel_max)`
   - `intensity_params`: flat array of per-property IntensityBehavior coefficients
   - `overlay_deltas`: flat `[OverlayDelta]` — ancestor stack then local, in evaluation order
   - `slot_delta_ranges`: `[N_slots × SlotDeltaRange]` — `(offset, length)` per slot
   - `threshold_registry`: flat array of threshold registrations *(deferred — Pass 7)*
   - `event_candidates`: sparse output from Pass 7 *(deferred)*

2. **`EvaluationBatch` builder** — CPU preparation pass:
   - Walk the SimThing tree
   - For each node, compose ancestor transforms using `TransformStack`
   - Resolve `PropertyTransformDelta` sub-field roles → column indices via `col_for_role`
   - Build `governed_pairs` from registry: for each active property, for each sub-field with
     `governed_by: Some(role)`, emit `(col_for_role(governed), col_for_role(governing), clamp_params)`
   - Write to `WorldGpuState` buffers (delta upload only)

3. **GPU Pass 1** (velocity integration) — WGSL compute shader:
   - One thread per `(slot, governed_pair_index)`
   - Read governing col value, apply velocity_max clamp, integrate, apply ClampBehavior
   - Write velocity pin if at boundary

4. **GPU Pass 2** (intensity update) — WGSL compute shader:
   - One thread per `(slot, intensity_col)` pair
   - Apply IntensityBehavior linear coefficients

5. **Verification harness**: run `Evaluator` (CPU oracle) and GPU pipeline on identical initial
   state, compare all output values with `assert_eq!(cpu_val.to_bits(), gpu_val.to_bits())`.

GPU output must match CPU oracle to the float bit. This is not optional. See Invariant I8.

---

## What success looks like at the end of Week 2

```
cargo test  →  all tests pass
              + new tests: cpu_gpu_pass1_matches, cpu_gpu_pass2_matches
VRAM usage at 100 SimThings, 8 dimensions  →  within 5% of projected budget
GPU pass timing at 1000 SimThings, 64 dims →  logged and within 50ms boundary budget
```

---

## Code style notes

- No comments explaining what the code does. Names should do that.
- Comments only for non-obvious WHY: a hidden constraint, a specific invariant reference,
  a workaround for a wgpu behavior, a simulation design decision.
- Reference invariants by number when a code comment explains a rule: `// I3: velocity pin`.
- Tests live in the module they test (`#[cfg(test)] mod tests` at the bottom of each file).
- New types go in the module that owns them. Don't create new files for small additions.
- No `unwrap()` in non-test code without a comment explaining why the None case is impossible.
