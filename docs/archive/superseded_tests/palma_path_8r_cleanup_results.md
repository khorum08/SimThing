# PALMA-PATH-8R-CLEAN — Remove legacy PALMA field-band aliases — Test Results

**Status: PASS** (2026-06-11)

## Scope

Remove remaining public PALMA/tick naming scaffold from production-facing API after PATH-8R tick removal. PALMA remains algebraic provenance in docs only.

## Removed

| Item | Action |
|---|---|
| `pub mod palma_min_plus_field_band` | Deleted module + `lib.rs` export |
| `palma_min_plus_field_band.rs` | Deleted |
| `PalmaMinPlusFieldBandSession` and related `PALMA_MIN_PLUS_*` re-exports | Removed from crate root |
| `TraversalFieldBandTickReport` type alias | Removed; use `TraversalFieldDispatchReport` |

## Production API (unchanged behavior)

- `TraversalFieldBandSession`
- `TraversalFieldDispatchReport`
- `TraversalFieldGpuInput` + `dispatch_gpu_resident`
- `resident_d_output()`
- Explicit diagnostic/compatibility dispatch methods

## Tests (targeted)

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo test -p simthing-gpu --test min_plus_stencil` | PASS |
| `cargo test -p simthing-driver --test palma_path_min_plus_oracle` | PASS |
| `cargo test -p simthing-driver --test palma_path_6_session_regionfield` | PASS |
| `cargo test -p simthing-driver --test palma_path_7_gpu_traversal_utility` | PASS |
| `cargo test -p simthing-driver --test palma_path_8_gpu_native_field_graph` | PASS |
| `cargo test -p simthing-driver --test palma_path_8r_remove_tick_scaffold` | PASS (6 tests incl. `legacy_palma_aliases_are_not_public`) |

## Boundaries preserved

No pathfinding engine, movement policy, mandatory CPU readback, or new runtime behavior.
