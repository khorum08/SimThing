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
    └── simthing-core/
        └── src/
            ├── lib.rs                 public re-exports
            ├── ids.rs                 SimThingId, SimPropertyId, OverlayId
            ├── property.rs            PropertyValue, PropertyLayout, SubFieldSpec,
            │                          ClampBehavior, SubFieldRole, SimProperty,
            │                          IntensityBehavior, DecayBehavior, fission types
            ├── registry.rs            DimensionRegistry, PropertyColumnRange
            ├── overlay.rs             Overlay, PropertyTransformDelta, TransformOp
            ├── simthing.rs            SimThing, SimThingKind
            └── evaluate.rs            Evaluator, TransformStack, FieldSnapshot (CPU oracle)
```

Future crates (not yet created):
- `simthing-gpu` — wgpu buffers, GPU passes, EvaluationBatch builder (Week 2)
- `simthing-feeder` — feeder thread, work queue (Week 3)
- `simthing-sim` — day boundary orchestration, fission/fusion execution (Week 3)

---

## Current implementation state

**All of Week 1 is complete, including the property generalization refactor.**

What's in the code right now:
- `PropertyLayout` is fully declarative: `Vec<SubFieldSpec>` with computed stride
- `SubFieldSpec` has role, width, ClampBehavior, velocity_max, default, governed_by
- All index arithmetic lives in `PropertyLayout::offset_of` and `PropertyColumnRange::col_for_role`
- No global index constants (`AMOUNT_IDX` etc.) — they were removed
- `PropertyValue::integrate` uses `governed_by` to know what evolves what
- Velocity is pinned at saturated boundaries (see Invariant I3)
- `TransformStack::apply_to` and `PropertyTransformDelta::apply_to_data` take `&layout`
- 14 tests passing, zero warnings

The `evaluate.rs` `Evaluator` is the CPU reference oracle for Week 2 GPU verification.

**What is NOT yet built:**
- GPU buffers and compute shaders (Week 2)
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

All 14 tests must pass with zero warnings before any commit. The test suite includes:
- `registry::column_assignment_is_contiguous` — column layout correctness
- `registry::col_for_role_multi_property` — global column arithmetic across properties
- `evaluate::velocity_integration` — amount evolves at velocity * dt
- `evaluate::ancestor_transform_propagates` — world-level overlay reaches cohort
- `evaluate::deterministic` — two identical evaluations produce bit-identical output
- `evaluate::snapshot_round_trip` — JSON serialize/deserialize is lossless
- `property::velocity_clamped_at_floor/ceiling` — velocity pinning at boundaries
- `property::integrate_mid_range_unchanged` — no spurious clamping mid-range
- `property::custom_layout_ethics_axis` — designer-defined layout, drift governor, width-3 vector

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
    local_xforms:    GpuMatrix,  // [N_slots × N_dims × N_dims]
    governed_pairs:  GpuBuffer,  // [(governed_col, governing_col, clamp_min, clamp_max, vel_max)]
    reduction_map:   GpuBuffer,
}
```

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
Pass 3: transform application     ← reads ancestor_xforms × local_xforms, writes output_vectors
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
   - `ancestor_transforms`: same layout
   - `output_vectors`: per-slot output after reduction
   - `governed_pairs`: flat array of `(governed_col, governing_col, clamp_min, clamp_max, vel_max)`
   - `threshold_registry`: flat array of threshold registrations
   - `event_candidates`: sparse output from Pass 7

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
