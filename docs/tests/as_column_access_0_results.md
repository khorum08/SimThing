# AS-COLUMN-ACCESS-0 Results

## Status

PROBATION — promoted §3 "no hardcoded `data[N]`" from prose to a Rust type boundary on `PropertyValue`.

## PR / branch / merge

- Branch: `codex/as-column-access-0`
- PR: https://github.com/khorum08/SimThing/pull/949
- Merge: `ea31b2e1ab` (master)

## What changed

- Made `PropertyValue.data` private; added role/layout accessors (`get_role`, `set_role`, `get_role_slice`, `set_role_slice`, `lane_at_offset`, `set_lane_at_offset`, `add_lane_at_offset`).
- Added explicit serialization escape hatch: `from_raw_lanes`, `raw_lanes`, `raw_lanes_mut`, `raw_lanes_for_serialization`.
- Exported `RoleOffset` as a real newtype (0R); `ColumnIndex` remains a plain alias deferred to AS-5.
- Migrated compiler-forced callsites in scenario builders, GPU projection, sim shadow seeding, and metadata byte-lane helpers.
- Added one `compile_fail` doc-test proving external `value.data[0]` no longer builds.

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `property_value_data_field_is_private_compile_fail` (`compile_fail` doc-test on `property` module) | External direct field indexing is uncompilable |
| `property_value_role_access_preserves_existing_layout_values` | Role-mediated scalar/slice read/write lands on correct layout lanes |
| `scenario_builtin_seed_values_survive_column_access_refactor` | `rebellion_demo` and related builtin seeds unchanged after migration |

## Scope Ledger

| File / area | Why touched |
|---|---|
| `crates/simthing-core/src/property.rs` | Rung implementation + accessors + tests |
| `crates/simthing-core/src/lib.rs` | Export `RoleOffset` / `ColumnIndex` |
| `crates/simthing-core/src/evaluate.rs` | Overlay apply uses `raw_lanes_mut()` (layout-mediated internally) |
| `crates/simthing-driver/src/scenario.rs` | Primary builtin seed migration target |
| `crates/simthing-driver/src/{install,gated_rates,arena_participant}.rs` | Compiler-forced layout-offset writes |
| `crates/simthing-spec/src/spec/scenario.rs` | Metadata byte-lane encode/decode via `from_raw_lanes` / `raw_lanes_for_serialization` |
| `crates/simthing-spec/src/spec/scenario_property_mutation_authority_boundary.rs` | Compiler-forced property value construction/read |
| `crates/simthing-gpu/src/projection.rs` | GPU upload copies via `raw_lanes_for_serialization` |
| `crates/simthing-sim/src/{boundary,fission,reduced_field,tree_mutation}.rs` | Compiler-forced shadow/projection paths (`cargo check -p simthing-driver` dependency chain) |
| `crates/simthing-clausething/src/literal_install.rs` | Overlay apply uses `raw_lanes_mut()` |
| `crates/simthing-mapeditor/src/hydration.rs` | Compiler-forced (driver dev-dep): `from_raw_lanes` + test reads via `raw_lanes()` |

**Guard retired:** no new `data[` grep battery; the private field is the enforcement.

**Not touched:** AS-2..AS-6, 0.0.8.5 Terran-Pirate, GPU/JIT EML lowering, new validation registries.

## Known gaps / next

- Integration test targets under `crates/simthing-sim/tests/` may still use `.data[...]`; migrate opportunistically on next touch.
- AS-2 channel newtypes remains OPEN.
- AS-5 broad index-newtype sweep (`ColumnIndex`, `SlotIndex`) remains OPEN.

---

## 0R repair (AS-COLUMN-ACCESS-0R)

### What changed

- Replaced `pub type RoleOffset = usize` with `pub struct RoleOffset(/* private */ usize)`; only `PropertyLayout::offset_of` constructs it.
- Added `RoleOffset::lane()` for post-resolution numeric use (shadow/GPU column arithmetic).
- `PropertyLayout::offset_of` now returns `Option<RoleOffset>`; `lane_at_offset` / `set_lane_at_offset` reject bare integers.
- Added two `compile_fail` doc-tests: external `RoleOffset = 0usize` and external `set_lane_at_offset(0, ...)`.
- Migrated `simthing-sim` / `simthing-gpu` lib test modules that failed on private `data` or `RoleOffset` typing.
- `ResolvedGatedRate` stores `RoleOffset`; intrinsic offset resolved via `intrinsic_flow_offset` (layout path), not raw column index cast.

### Load-bearing proofs

| Proof | Catches |
|---|---|
| `property_value_data_field_is_private_compile_fail` | Original AS-1 direct-field boundary still holds |
| `role_offset_cannot_be_constructed_from_usize_compile_fail` | External `RoleOffset = 0usize` does not compile |
| `lane_at_offset_rejects_bare_usize_compile_fail` | External `set_lane_at_offset(0, …)` does not compile |
| `property_value_role_access_preserves_existing_layout_values` | Role-mediated access unchanged |
| `scenario_builtin_seed_values_survive_column_access_refactor` | Builtin seeds unchanged |
| `cargo test -p simthing-sim --lib` / `cargo test -p simthing-gpu --lib` | Lib test surface compiles and passes after migration |

### Scope Ledger (0R additions)

| File / area | Why touched |
|---|---|
| `crates/simthing-core/src/{property,overlay,registry}.rs` | `RoleOffset` newtype + `offset_of` return type |
| `crates/simthing-driver/src/{arena_hierarchy,arena_pressure,gated_rates,install,scenario}.rs` | `.lane()` at shadow/GPU boundaries; gated-rate offsets typed |
| `crates/simthing-sim/src/{fission,tree_mutation,property_expiry,threshold_registry}.rs` | Compiler-forced lib-test migration |
| `crates/simthing-gpu/src/{passes,projection,reduction}.rs` | Compiler-forced lib-test migration |

### Known gaps / next

- Same as parent section; 0R closed the sim/gpu **lib** test compile surface called out in AS-1 evidence.
