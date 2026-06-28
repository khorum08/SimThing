# AS-CHANNEL-NEWTYPES-0 Results

## Status

PROBATION — promoted §5.1 RF channel identity from interchangeable `String` fields to distinct newtypes on the planet-child RF admission path.

## PR / branch / merge

- Branch: `codex/as-channel-newtypes-0`
- PR: https://github.com/khorum08/SimThing/pull/951
- Merge: `3a6df374f8` (master)

## What changed

- Added `crates/simthing-spec/src/spec/channel_key.rs` with `OwnerRef`, `ResourceKey`, and `ScopeId` newtypes (`new`, `as_str`, `into_inner`; no implicit `From<String>`).
- Migrated `PlanetChildRfScopeKey` and `PlanetChildRfParticipantInput` to typed owner/resource/scope fields; authored metadata strings convert at the admission seam in `collect_rf_participant_from_node`.
- Compiler-forced `.as_str()` bridges at runtime writeback, recursive-local RF adaptation, reconciliation projection, and driver writeback compile boundaries (String-backed runtime rows unchanged).
- Repaired reduce-up test fixtures to parent gameplay children under the planet surface gridcell (behavior-preserving vs current location admission doctrine).
- Exported `OwnerRef`, `ResourceKey`, `ScopeId` from `simthing-spec`.

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `channel_owner_ref_rejects_resource_key_compile_fail` (`compile_fail` doc-test on `channel_key`) | `ResourceKey` cannot be passed where `OwnerRef` is required |
| `channel_resource_key_rejects_owner_ref_compile_fail` (`compile_fail` doc-test on `channel_key`) | `OwnerRef` cannot be passed where `ResourceKey` is required |
| `planet_child_rf_scope_key_groups_equivalently_after_newtypes` | Same admitted participants → same bucket/owner-channel counts and reduce-up totals after typing |
| `planet_child_rf_empty_owner_ref_still_rejects` | Whitespace-only authored owner ref still hard-errors at admission |
| `planet_child_rf_unknown_owner_ref_still_rejects` | Unknown owner ref still hard-errors at admission |

## Scope Ledger

| File / area | Why touched |
|---|---|
| `crates/simthing-spec/src/spec/channel_key.rs` | Rung implementation + compile_fail doc-tests |
| `crates/simthing-spec/src/spec/planet_child_rf.rs` | Primary migration: scope key, participant input, admission seam |
| `crates/simthing-spec/src/spec/mod.rs`, `src/lib.rs` | Export newtypes |
| `crates/simthing-spec/src/spec/owner_silo_runtime_writeback.rs` | Compiler-forced: scope key → `String` at runtime writeback boundary |
| `crates/simthing-spec/src/spec/recursive_local_rf.rs` | Compiler-forced: planet-child input → recursive row `String` bridge |
| `crates/simthing-spec/src/spec/recursive_rf_reconciliation.rs` | Compiler-forced: typed owner → projection row `String` |
| `crates/simthing-driver/src/owner_silo_runtime_writeback_compile.rs` | Compiler-forced: scope key vs runtime writeback input compare |
| `crates/simthing-driver/tests/planet_child_rf_reduce_up.rs` | Compiler-forced: `ResourceKey` read + fixture surface-tier parenting |
| `crates/simthing-spec/tests/planet_child_rf_reduce_up.rs` | Compiler-forced: `ScopeId` assertions + fixture surface-tier parenting |
| `crates/simthing-spec/tests/reduce_up_fixture.rs` | Compiler-forced: fixture surface-tier parenting (restores RF participant discovery) |
| `crates/simthing-spec/tests/as_channel_newtypes_0.rs` | Load-bearing behavior/admission proofs |

**Guard retired:** no arg-order runtime check or owner-vs-resource grep battery added; distinct types + two compile_fail proofs are the enforcement.

**Not touched:** AS-3..AS-6, simthing-sim/gpu semantic surface, global string taxonomy, RF arithmetic/grouping semantics.

## Known gaps / next

- Broader runtime/report rows (`RuntimeOwnerSiloWritebackInput`, recursive RF rows, etc.) still use `String` at downstream boundaries — intentional; AS-2 scoped to planet-child RF channel path only.
- `ParentLocationId` not introduced; `planet_id` on scope key remains `Option<String>` (display/spatial label, not channel transposition target).
- AS-3 kind-out-of-tick is next queued rung on the 0.0.8.4 ladder.
