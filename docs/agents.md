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

**`intensity_params` buffer is in `WorldGpuState`.**
Per-property entry: `(velocity_col, intensity_col, velocity_threshold, build_coefficient,
decay_coefficient)` plus padding. Built from the registry by iterating active properties
with `intensity_behavior: Some(_)` and both Velocity and Intensity sub-fields in their
layout. Property-level (one entry per property), not slot-level. Pass 2 dispatches one
thread per `(slot, intensity_param)`.

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

**Outcome (Week 2):** Option B implemented and bit-exact verified on naga + DX12. The
`velocity_integration_matches_cpu_oracle_fractional_dt` test stresses the case with
`dt = 0.5` and non-power-of-2 inputs; `to_bits()` parity holds. If a future driver
fuses despite the `let` bindings, that test will fail loudly and the fallback is
`f32::mul_add` on the CPU side + WGSL `fma()` in the shader.

---

## Transform encoding — affine (decided)

`TransformOp::{Add, Multiply, Set}` is not a linear operation. `Multiply(k)` on
column `c` is linear (diagonal matrix entry `k`), but `Add(k)` is a translation
(needs a bias term), and `Set(k)` is neither linear nor affine in the standard
sense — it discards the input value entirely.

The transform pipeline therefore uses **affine** representation, not pure
N×N matrix multiplication. Each transform is a pair:

```rust
struct AffineTransform {
    matrix: GpuMatrix,  // M : [N_dims × N_dims]
    bias:   GpuVector,  // b : [N_dims]
}
```

Applied as `output = M · x + b`. Composition is:

```
(M2, b2) ∘ (M1, b1)  =  (M2 · M1, M2 · b1 + b2)
```

This is what `EvaluationBatch::ancestor_xforms` / `ancestor_bias` accumulate as
the tree walk descends from root to leaf.

### Encoding `TransformOp` per column

For a sub-field's column `c`, with previous value `x[c]`:

| `TransformOp`        | `M[c, c]` | `b[c]` | Effect: `M[c,c] * x[c] + b[c]` |
|----------------------|-----------|--------|--------------------------------|
| identity (no-op)     | 1         | 0      | `x[c]`                         |
| `Multiply(k)`        | `k`       | 0      | `k * x[c]`                     |
| `Add(k)`             | 1         | `k`    | `x[c] + k`                     |
| `Set(k)`             | 0         | `k`    | `k`                            |

Off-diagonal `M` entries stay zero — `TransformOp` is per-column, so transforms
never mix columns. (If a future op needs to mix columns — e.g., a vector
rotation or a cross-property pressure — it writes off-diagonal entries
directly and the same composition formula applies.)

### Why not the alternatives

- **Homogeneous coordinates** `(N+1)×(N+1)` with extended vector: single matmul,
  bias in the last column. But WGSL indexing for `(N+1)`-sized matrices is
  awkward, and at endgame scale the extra row+column costs ~6 MB per transform
  matrix. Affine pays a smaller fixed cost (one `[N_slots × N_dims]` bias
  buffer per transform, ~2.8 MB at endgame) with clearer arithmetic.
- **Iterative (apply each overlay's delta sequentially on GPU)**: contradicts
  the design doc's single-matmul Pass 3 promise, can't pre-compose the ancestor
  stack on the CPU prep pass, and complicates the reduction passes.

### `WorldGpuState` buffer additions (Pass 3 work)

Two new buffers will be added alongside `local_transforms` and
`ancestor_transforms`:

```
local_bias    : [N_slots × N_dims]
ancestor_bias : [N_slots × N_dims]
```

Each pairs with its matrix buffer. Memory budget at endgame (64 dims, 11,520
slots): adds ~5.6 MB total — negligible against the ~728 MB matrix budget.

### Pass 3 dispatch

```wgsl
// For each slot, for each output column j:
//   tmp[j] = sum over k of local_M[slot, j, k] * (
//              sum over m of ancestor_M[slot, k, m] * base_x[slot, m]
//              + ancestor_b[slot, k]
//            ) + local_b[slot, j]
//
// Equivalent to applying composed affine (M_local · M_ancestor,
// M_local · b_ancestor + b_local) to base_x. CPU prep pass composes once;
// shader applies the composed affine.
```

Composition happens during CPU prep — one `EvaluationBatch` carries the
already-composed `(M, b)` per slot for both ancestor and local. The shader
just applies the composed affine.

### Identity / no-overlay slot

An overlay slot with no overlay applied holds `(M = I, b = 0)`. Affine
composition with identity is a no-op: `I · x + 0 = x`. This preserves the
"transient overlay slot costs zero CPU writes when unused" invariant from
the design doc (§5: Instruction Overlays As Standing Registers).

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
    base_vectors:    GpuMatrix,  // [N_slots × N_dims]
    ancestor_xforms: GpuMatrix,  // [N_slots × N_dims × N_dims]
    ancestor_bias:   GpuBuffer,  // [N_slots × N_dims]            (affine bias)
    local_xforms:    GpuMatrix,  // [N_slots × N_dims × N_dims]
    local_bias:      GpuBuffer,  // [N_slots × N_dims]            (affine bias)
    governed_pairs:  GpuBuffer,  // [(governed_col, governing_col, clamp_min, clamp_max, vel_max)]
    reduction_map:   GpuBuffer,
}
```

Note: each transform is the affine pair `(M, b)` — see the "Transform
encoding — affine" section above. The CPU prep pass composes overlays into
`(M, b)` before upload; Pass 3 applies the composed affine.

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
Pass 3: transform application     ← reads composed affine (M, b), writes output_vectors
Pass 4–6: reduction
Pass 7: threshold scan
```

---

Add `wgpu = "22"` and `rayon = "1"` to `[workspace.dependencies]` in `Cargo.toml`.

Create `crates/simthing-gpu/` with:

1. **`WorldGpuState`** — owns the wgpu device/queue and all GPU buffers:
   - `values`: `[slot * N_DIMS + col]` — current property values
   - `previous_values`: snapshot from Pass 0
   - `local_transforms`: per-slot transform matrices `[slot * N_DIMS * N_DIMS + ...]`
   - `local_bias`: per-slot affine bias `[slot * N_DIMS + ...]` — pairs with `local_transforms`
   - `ancestor_transforms`: same layout as `local_transforms`
   - `ancestor_bias`: same layout as `local_bias`
   - `output_vectors`: per-slot output after reduction
   - `governed_pairs`: flat array of `(governed_col, governing_col, clamp_min, clamp_max, vel_max)`
   - `intensity_params`: flat array of per-property IntensityBehavior coefficients
   - `threshold_registry`: flat array of threshold registrations *(deferred — see Pass 7)*
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
