# SimThing — Invariants

These are the non-negotiable structural rules of the codebase. Violations will silently produce
incorrect GPU output, non-deterministic evaluation, or hidden simulation state that neither the
player nor the AI can read. If you are modifying any file in `simthing-core`, check this list first.

---

## I1 — Column arithmetic has exactly one home

**Rule:** No code outside of `PropertyLayout` and `PropertyColumnRange` may compute a GPU column
index. No code may compute `slot * N_DIMS + dim` except the WGSL shaders themselves.

**The two permitted arithmetic sites:**
- `PropertyLayout::offset_of(role)` — local offset within a property's column block
- `PropertyColumnRange::col_for_role(role, layout)` — global GPU column = `range.start + offset_of`

**Consequences of violation:** An overlay that hardcodes column index 3 for "grievance_inertia"
will silently read/write the wrong column if any property ahead of it in the registry changes its
layout. The simulation produces no error — it produces wrong numbers.

---

## I2 — Stride is computed, never stored

**Rule:** `PropertyLayout::stride()` is computed from the sum of sub-field widths on every call.
It is not a field. Do not cache it.

**Why:** The GPU column count for a property is derived at registration time by calling `stride()`.
If stride were stored and the sub-fields were later edited (e.g. promoting a width-1 to width-3),
the stored stride would be wrong while the sub-fields were correct. Computing it eliminates the
possibility of this class of stale-data bug.

---

## I3 — Velocity is pinned at saturated boundaries

**Rule:** When `PropertyValue::integrate` drives a governed sub-field to its floor or ceiling, the
governing sub-field's value is immediately clamped to zero in the saturated direction.

**Why:** A cohort pinned at the loyalty floor for 60 days must not accumulate 60 days of negative
velocity debt. When suppression lifts and conditions improve, that debt would resist recovery in a
way that is invisible to the player, unattributable by the AI, and not readable from any overlay
stack. Persistent inertia from long-term suppression belongs in `Named("grievance_inertia")` —
a named vector component that is observable, queryable, and attributable.

**Implementation site:** `PropertyValue::integrate` in `property.rs`. The floor/ceiling check uses
`ClampBehavior::at_floor` / `at_ceiling` on the spec of the governed sub-field.

---

## I4 — Index constants are banned outside PropertyLayout

**Rule:** The constants `AMOUNT_IDX`, `VELOCITY_IDX`, `INTENSITY_IDX`, `VECTOR_START_IDX` do not
exist in this codebase. They were removed during the property generalization refactor. Do not
re-introduce them.

**Replacement:** All access to sub-field values goes through:
```rust
let offset = layout.offset_of(&SubFieldRole::Amount).unwrap();
value.data[offset]
```
or through `PropertyValue::get_role(role, layout)` for single reads.

---

## I5 — Overlay transforms reference sub-fields by role, never by column index

**Rule:** `PropertyTransformDelta::sub_field_deltas` contains `Vec<(SubFieldRole, TransformOp)>`.
Column indices are never stored in overlays. Column resolution happens in the CPU preparation pass
via `col_for_role`, immediately before GPU dispatch.

**Why:** An overlay authored for "loyalty garrison suppresses intensity" must work correctly
regardless of which GPU column intensity currently occupies. Column assignment is a registry
concern. Overlay authoring is a gameplay concern. They must not be coupled.

---

## I6 — `governed_by` is the only integration relationship

**Rule:** The only way a sub-field evolves over time via integration is through
`SubFieldSpec::governed_by`. There is no other mechanism. The evaluator iterates sub-fields,
finds those with a non-None `governed_by`, and integrates them using the governing sub-field's
current value.

**Corollary:** A sub-field with `governed_by: None` is never touched by `integrate()`. It changes
only when an overlay transform is applied to it. This is the correct behavior for velocity
sub-fields themselves, for named vector components like `grievance_inertia`, and for any
sub-field the designer declares as "externally driven only."

---

## I7 — Structural mutations occur only at the day boundary

**Rule:** The following operations may only execute during the day boundary protocol:
- Adding or removing children from the SimThing tree
- Inserting or removing entries from any SimThing's property map
- Tombstoning or restoring registry columns
- Registering or unregistering threshold entries
- Fission and fusion events
- Slot allocation and deallocation in the GPU buffer

**Within a day:** Only transform matrix coefficients and velocity values in GPU buffers are
written. The tree structure, property map keyset, and registry layout are read-only.

**Why:** The GPU pipeline assumes a stable slot table for the duration of each pass sequence.
A mid-day structural mutation would invalidate live GPU buffer pointers and produce
undefined evaluation results.

---

## I8 — The CPU reference evaluator is the oracle

**Rule:** `crate::evaluate::Evaluator` is the single-threaded, deterministic CPU implementation.
Its output is the ground truth. Every GPU pipeline output must match it exactly — to the float
bit — for the same input state and same seed.

**Consequence:** GPU pass verification tests must pass `x.to_bits() == y.to_bits()` comparisons,
not epsilon comparisons. Float non-determinism between CPU reference and GPU output is a bug,
not a tolerated approximation.

---

## I9 — The registry is append-only within a session

**Rule:** Properties are never removed from `DimensionRegistry` within a running session.
When a property's last instance expires, its columns are tombstoned. The `SimPropertyId` index
remains valid and the `SimProperty` definition remains accessible. Column slots may be reused
by newly registered properties, but the original registration is never overwritten.

**Why:** `SimPropertyId` values are embedded in serialized overlays, threshold registrations,
and replay logs. Removing a registration would invalidate those ids. Tombstoning preserves
id stability while freeing GPU column slots for reuse.

---

## I10 — `SimProperty` equality and hashing are on namespace+name only

**Rule:** `PartialEq`, `Eq`, and `Hash` for `SimProperty` compare and hash `namespace` and `name`
only. All other fields — layout, behaviors, metadata — do not participate.

**Why:** `SimProperty` is used as a registration key. Two definitions with the same
namespace+name are the same property regardless of layout differences (which would represent
a version migration). The `by_name` HashMap in the registry enforces uniqueness on this key.

---

## I11 — `SubFieldRole::Named` is identity by string value

**Rule:** `SubFieldRole::Named("grievance_inertia")` and `SubFieldRole::Named("grievance_inertia")`
are equal. Spelling and casing are the entire identity. There is no separate registration step
for Named roles.

**Consequence:** Typos in role names are silent misses, not errors. An overlay that references
`Named("grievence_inertia")` (misspelled) will produce `None` from `offset_of` and silently
do nothing. Tests for new overlay types must verify that the role name round-trips correctly
through a layout lookup before the overlay is considered correct.

---

## I12 — Dormant properties are free

**Rule:** A sub-field at its default value with `governed_by: None` and no overlay transforms
applied costs exactly: one multiply-add per GPU pass that produces zero net change. There is no
code path that skips dormant sub-fields; the GPU matrix is dense and operates uniformly.

**Consequence:** Do not add special-case logic to skip dormant properties. The cost model is
already correct. Special-casing would introduce branch divergence in GPU shaders and complexity
in the CPU preparation pass for a zero benefit.

---

## Invariant summary table

| # | Rule | Site | Violation consequence |
|---|------|------|-----------------------|
| I1 | Column arithmetic has one home | `PropertyLayout`, `PropertyColumnRange` | Wrong GPU output, no error |
| I2 | Stride computed, not stored | `PropertyLayout::stride()` | Stale column count at registration |
| I3 | Velocity pinned at boundary | `PropertyValue::integrate` | Hidden velocity debt, wrong recovery |
| I4 | No index constants | Banned from codebase | Breaks on any layout change |
| I5 | Overlays use roles, not columns | `PropertyTransformDelta` | Overlay breaks on registry change |
| I6 | governed_by is the only integration path | `SubFieldSpec::governed_by` | Unanticipated evolution |
| I7 | Structural mutations at boundary only | Day boundary protocol | GPU buffer invalidation |
| I8 | CPU evaluator is the oracle | `evaluate::Evaluator` | GPU divergence is a bug |
| I9 | Registry is append-only | `DimensionRegistry` | Id invalidation in serialized state |
| I10 | SimProperty eq/hash on identity only | `PartialEq`, `Hash` impls | Duplicate registration |
| I11 | Named role identity is string value | `SubFieldRole::Named` | Silent misses on typos |
| I12 | Dormant properties are free | GPU matrix | Don't skip them |
