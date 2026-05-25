# SimThing Core — Structural Invariants

These rules are enforced by the type system and code structure, not by convention.
A violation is either a compile error or a test failure. If you find yourself
working around one of these, stop and reconsider the design.

---

## Property Layout

| Rule | Enforced by |
|---|---|
| `stride` is computed, never stored | `PropertyLayout::stride()` method; no `stride` field on the struct |
| Local index arithmetic has one home | `PropertyLayout::offset_of()` only — no raw index arithmetic anywhere else |
| Global column arithmetic has one home | `PropertyColumnRange::col_for_role(layout)` only |
| `PropertyValue::data` indices never hardcoded | All callers go through `offset_of` |
| Integration relationships are explicit | `SubFieldSpec::governed_by` — designer declares which sub-field governs which |
| Clamping is per-sub-field | `SubFieldSpec::clamp: ClampBehavior` — no property-level `valid_range` |
| Sub-field roles are named, not positional | `SubFieldRole::Named(String)` replaces `VectorComponent(usize)` |

## Registry

| Rule | Enforced by |
|---|---|
| Property definitions are registered once per session | `register()` panics on duplicate namespace+name |
| Columns are append-only within a session | `DimensionRegistry::total_columns` only increases |
| Tombstoned columns stay indexed | `active: Vec<bool>` marks inactive; no column removal |
| Column owners are recorded at registration | `column_owners: Vec<(SimPropertyId, usize)>` built during `register()` |

## Evaluation

| Rule | Enforced by |
|---|---|
| Transforms reference sub-fields by role, not column | `PropertyTransformDelta::sub_field_deltas: Vec<(SubFieldRole, TransformOp)>` |
| Column resolution happens in the CPU prep pass | `apply_to_data` takes `layout`; GPU receives only resolved indices |
| Transforms apply after velocity integration | Evaluation step order in `evaluate_node`; documented as intentional |
| Evaluator does not mutate the SimThing tree | `evaluate_node` takes `&SimThing`, returns snapshot; fission/fusion belong to day-boundary protocol |
| Determinism is bit-exact | Tests use `f32::to_bits()` comparison, not approximate equality |

## State Authority

| Rule | Enforced by |
|---|---|
| Within-day CPU shadow writes do not perform stale read-modify-write | `TransformPatcher` applies only `Set` immediately; `Add`/`Multiply` increment `unsafe_rmw_skipped` |
| Boundary lifecycle decisions read GPU-integrated values | `BoundaryProtocol::execute` reads `WorldGpuState::values` into `coord.shadow` before expiry/fission/structural work |
| CPU `TowardZero` expiry reads synchronized shadow | `resolve_property_expiry(root, registry, allocator, shadow, n_dims, ...)` resolves slot+column and reads `shadow` |
| Registry tombstoning is whole-tree scoped | CPU expiry collects removals first, then checks liveness from the root before `registry.tombstone(pid)` |
| Structural slot churn scrubs dense state | `AddChild` zeroes and projects initialized subtree properties; `Remove` zeroes rows before tombstoning slots |
| Fission secondary checks use the triggering property | `check_secondary(..., triggering_pid, ...)` reads Amount/Intensity from that property's shadow columns |

## SimProperty Identity

| Rule | Enforced by |
|---|---|
| `SimProperty` equality is on `namespace + name` only | Manual `PartialEq`, `Eq`, `Hash` impls that exclude all other fields |
| Metadata fields do not participate in key comparison | Verified by: two properties with same identity but different layouts compare equal |

## AccumulatorOp v2

| Rule | Enforced by |
|---|---|
| Exact operations never use soft-aggregate combine fns | Code review gate; `WeightedMean` / `Mean` may not appear in conservation-critical registration paths |
| `EvalEML` combine requires a whitelist entry | `EmlExpressionRegistry::assert_whitelisted(tree_id)` checked at registration |
| `SubtractFromSource` is the only transfer mechanism | No two-overlay transfers; `TransformOp::Add` on two separate slots for the same logical transfer is a violation |
| Emission records are produced for every GPU-resolved emission | `EmissionRecord { reg_idx, emit_count }` written to compact buffer; read back for delta log |
| Persistent GPU buffer is the residency model | `AccumulatorOpSession` is created at session open and closed at session close; no per-tick device creation |
| Timestamp queries are required for performance claims | Any PR claiming a performance win must include timestamped GPU pass measurements, not just wall-clock |
| Old pass code is never deleted without a green CI run at default-on flag | Sunset PR checklist; enforced before deletion |
| `design_v7.md` §4 is updated by each migration PR | PR template checklist item |
| `SoftAggregateGuard` on WeightedMean columns feeding thresholds | `assert_no_hard_trigger_on_soft_aggregate()` at registration |
| `simthing-sim` never knows recipe semantics | No recipe strings, costs, or economic types in `simthing-sim` |

---

## The Proof Test

`custom_layout_ethics_axis` in `property.rs` is the invariant proof for the
generalization. It constructs a non-standard layout — signed ethics axis with a
drift governor and a width-3 bonus vector — and verifies:

1. `stride()` = sum of sub-field widths (1 + 1 + 3 = 5)
2. `offset_of` returns correct local indices for each role
3. `width_of` returns 3 for the Named("ethics_bonus") vector sub-field
4. `default_data()` produces correct defaults including the 1.0 neutral bonus values
5. Integration advances the governed sub-field using the governing sub-field's value
6. Non-governed sub-fields are untouched by integration

If this test passes, the full generalization works. If a future change breaks it,
something about the sub-field layout invariants has been violated.

---

## What These Invariants Buy

**Correctness by construction.** A designer editing a `SubFieldSpec` cannot accidentally
produce inconsistent column arithmetic — there is only one place column arithmetic
lives, and it reads the layout at runtime.

**Refactoring safety.** Renaming a sub-field from `Named("vec_0")` to `Named("grievance_inertia")`
requires updating the `SubFieldSpec` role and any overlay `sub_field_deltas` that
reference it by name. The compiler will catch the latter via exhaustive match if
`SubFieldRole` is a closed enum for your game's named fields — or the test suite
will catch it via the observability query tests if `Named` remains open.

**GPU/CPU parity.** The CPU preparation pass and the CPU reference evaluator both go
through `offset_of` and `col_for_role`. If the GPU output diverges from the CPU
oracle in Week 2 tests, the bug is in shader arithmetic or buffer layout — not in
a disagreement about what column a sub-field occupies, because both sides read the
same registry.

**Designer safety.** The one-edit guarantee holds as long as all callers use
`offset_of` and `col_for_role`. Any direct `data[N]` access outside of
`PropertyLayout` methods is a violation of the invariants above and should be
treated as a bug regardless of whether it produces correct output today.
