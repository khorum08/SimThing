# AS-COLUMN-ACCESS-0 Results

## Status

PROBATION — promoted §3 "no hardcoded `data[N]`" from prose to a Rust type boundary on `PropertyValue`.

## PR / branch / merge

- Branch: `codex/as-column-access-0`
- PR: (pending)
- Merge: (pending)

## What changed

- Made `PropertyValue.data` private; added role/layout accessors (`get_role`, `set_role`, `get_role_slice`, `set_role_slice`, `lane_at_offset`, `set_lane_at_offset`, `add_lane_at_offset`).
- Added explicit serialization escape hatch: `from_raw_lanes`, `raw_lanes`, `raw_lanes_mut`, `raw_lanes_for_serialization`.
- Exported `RoleOffset` and `ColumnIndex` type aliases.
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

- Test modules in `simthing-sim`, `simthing-gpu`, and integration tests still using `.data[...]` directly will fail when those test targets are compiled; migrate opportunistically on next touch or a dedicated test-harness rung.
- AS-2 channel newtypes remains OPEN.
