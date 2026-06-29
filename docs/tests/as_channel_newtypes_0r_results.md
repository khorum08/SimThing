# AS-CHANNEL-NEWTYPES-0R Results

## Status

PASS — production/report RF channel identity surfaces adopt `OwnerRef` / `ResourceKey` / `ScopeId` / `ParentLocationId`; raw transposable `String` fields removed from `simthing-spec/src/spec`. Parent **AS-CHANNEL-NEWTYPES-0** → **PROBATION** pending DA re-review (not DA-APPROVED).

## PR / branch / merge

- Branch: `codex/as-channel-newtypes-0r`
- PR: (pending)
- Merge: (pending)

## What changed

- Added `ParentLocationId(u32)` — fourth RF channel axis (parent location id); distinct from string keys and from `OwnerRef`.
- Extended `channel_key.rs` compile_fail coverage: OwnerRef↔ResourceKey, ResourceKey↔ScopeId, OwnerRef↔ParentLocationId.
- Migrated all production/report RF channel structs under `crates/simthing-spec/src/spec/` from raw `String` / `Option<String>` to newtypes; `parent_location_id_raw` → `parent_location_id: ParentLocationId` in RF channel contexts.
- Added `as_channel_newtypes_production_adoption` lib test — fails if any spec module reintroduces raw channel identity fields.
- Compiler-forced driver compile-path updates (`*_compile.rs`) to compare via `.as_str()` / typed fields.
- Integration tests updated for typed comparisons; strings convert at assertion boundary only.

## Production adoption audit

| File / surface | Resolution |
|---|---|
| `loaded_scenario_recursive_rf_runtime.rs` | Migrated participant/channel rows |
| `owner_silo_disburse_down.rs` | Migrated demand/disburse structs + errors |
| `owner_silo_runtime_writeback.rs` | Migrated writeback input/result rows |
| `owner_silo_recursive_rf_source.rs` | Migrated demand bucket derivation |
| `local_effect_application.rs` | Migrated application records |
| `local_participant_effects.rs` | Migrated effect previews |
| `runtime_local_allocation.rs` | Migrated allocation states |
| `recursive_local_rf.rs` | Migrated arena/participant/settlement rows |
| `recursive_rf_reconciliation.rs` | Migrated reconciliation projection rows |
| `runtime_rf_tick_source.rs` | Migrated tick source rows |
| `runtime_participant_*_mutation*.rs` | Migrated mutation records |
| `semantic_*` report surfaces | Migrated output rows |
| `scenario_stead_map_roundtrip.rs` | Migrated RF metadata rows |
| `scenario_candidate_from_runtime.rs` | Migrated mutation records |
| `scenario_property_mutation_authority_boundary.rs` | Migrated records |
| `planet_child_location.rs` | `PlanetNonGridChildEntry.owner_ref` → `Option<OwnerRef>` |
| `planet_child_rf.rs` | Already typed (0A baseline) — unchanged |
| Authored JSON / scenario metadata readers | Strings wrapped at admission via `OwnerRef::new` / etc. — not stored raw in report structs |

## ParentLocationId decision

Handoff preferred `ParentLocationId(String)`; production RF parent-location axis is **raw gridcell/location id (`u32`)**, not a string channel key. Implemented:

```rust
pub struct ParentLocationId(u32); // new / raw / as_u32
```

Justification: matches every RF arena/participant grouping site; transposition with `OwnerRef`/`ResourceKey`/`ScopeId` is already uncompilable via distinct types + compile_fail doc-tests.

## Deviation Records

None — zero allowed deviations in adoption proof; no serialization-boundary raw-string report structs remain.

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `channel_owner_ref_rejects_resource_key_compile_fail` | ResourceKey passed as OwnerRef |
| `channel_resource_key_rejects_owner_ref_compile_fail` | OwnerRef passed as ResourceKey |
| `channel_resource_key_rejects_scope_id_compile_fail` | ScopeId passed as ResourceKey |
| `channel_scope_id_rejects_resource_key_compile_fail` | ResourceKey passed as ScopeId |
| `channel_owner_ref_rejects_parent_location_compile_fail` | ParentLocationId passed as OwnerRef |
| `channel_parent_location_rejects_owner_ref_compile_fail` | OwnerRef passed as ParentLocationId |
| `as_channel_newtypes_production_adoption` | Raw `owner_ref`/`resource_key`/`scope_id` String fields in spec production modules |
| `planet_child_rf_scope_key_groups_equivalently_after_newtypes` | Behavior preserved (0A) |
| `planet_child_rf_empty_owner_ref_still_rejects` | Admission hard-error preserved |
| `planet_child_rf_unknown_owner_ref_still_rejects` | Admission hard-error preserved |

## Scope Ledger

| File / area | Why touched |
|---|---|
| `crates/simthing-spec/src/spec/channel_key.rs` | ParentLocationId + adoption proof + compile_fail |
| `crates/simthing-spec/src/spec/*.rs` (19 production modules) | Production newtype adoption |
| `crates/simthing-spec/src/lib.rs` | Export ParentLocationId |
| `crates/simthing-spec/tests/*.rs` (19 files) | Typed test comparisons |
| `crates/simthing-driver/src/*_compile.rs` (7 files) | Compiler-forced compile-path bridges |
| `docs/design_0_0_8_4_admission_substrate.md` | AS-2 state note |
| `docs/tests/current_evidence_index.md` | 0R evidence row |
| `docs/tests/as_kind_out_of_tick_closeout_results.md` | AS-3 closeout prep (docs only) |
| `docs/tests/as_sim_semantic_free_closeout_results.md` | AS-4 closeout prep (docs only) |

**Guard retired:** raw channel identity `String` fields beside newtypes in production report structs (DA-held AS-2 gap).

**Not touched:** AS-5 index newtypes, AS-8 PackedUpload, simthing-sim/gpu semantic surface, AS-1 PropertyValue hardening.

## DA closeout status

- **AS-2 remediation:** production adoption complete; illegal transposable raw strings removed from report surfaces. Awaiting DA re-review — **PROBATION**, not DA-APPROVED.
- **AS-1:** unchanged (optional hardening not taken).
- **AS-3 / AS-4:** closeout ledgers prepared; code not broadened.

## Known gaps / next

- Driver compile proof structs still mirror spec types with newtypes (not raw String) — acceptable; not a second raw-string path.
- `simthing-mapeditor` studio surfaces may still use raw strings outside this remediation scope — not RF report production path in `simthing-spec`.
- DA re-review for AS-2 graduation; AS-3/AS-4 net-negative closeout confirmation remains Opus queue.
