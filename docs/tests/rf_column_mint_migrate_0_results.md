# RF-COLUMN-MINT-MIGRATE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 9.2)
- Status: **PROBATION / proof-present / DA-review-pending**
- HD-RECEIPT: `11176c638ff7`
- ORIENT-RECEIPT: `a5dc59920dd4`
- orientation_rule_stamp: `61818ff7d4adda84`
- Board dispatch: comment `5356305326`
- expected_route: `DA-RESERVE(gate-wiring)`

## Mint census (live tree)

| Door | Production src | Disposition |
|---|---|---|
| `ColumnIndex::new` | none (alias deleted) | retired |
| `col_for_role` / `from_layout_role` | registry + comparative derived Amount | keep |
| `StructuralScalarChannel::into_plan_column` | owner-channel / silo / mapping / dummy insufficient comparative | keep |
| `from_gpu_round_trip` | `wgsl_encode::column_from_wire` only | keep (WGSL boundary) |
| `try_from_admitted_authored` | spec admission + mapping urgency fallback | keep (authored-wire) |
| `from_raw_for_oracle_or_rehearsal` | `#[cfg(test)]` / tests only | keep (oracle independence) |
| `COLUMN-INDEX-MINT` scan | deleted from `scans.tsv` | retired after sweep |

## Owner layout

`OwnerInterner` / `OwnerLayoutId` are session-local, not serde. GPU bucket plans sort by interned id (tree-walk first-seen), not `OwnerRef` lexical `Ord`. Seam identity remains `OwnerRef`.

## Falsifiers

| Production | Rival | Result |
|---|---|---|
| interned layout under `alpha -> zulu` (tree-walk compile + `OwnerInterner::rebind`) | lexical `OwnerRef` sort | interned ids stable; lexical permutes |
| independent sessions intern `alpha`/`zulu` in opposite order | same raw interned id | ids differ |
| GPU plan compile after forced `SlotAllocator::epoch_rebind` | order buckets by post-rebind physical `slot_of` | interned keys/ids stable; physical-row rival permutes |
| seam JSON with `OwnerRef` | extra `interned_owner_id` | deny_unknown_fields RED |
| comparative `col_for_role(Amount)` after a pad property | `from_gpu_round_trip` of pad Amount | production Amount is not GPU-wire column 0 |

## Blast radius (local, pre-push)

| Command | Result |
|---|---|
| `cargo test -p simthing-driver --test rf_column_mint_migrate_0` | 1/1 |
| `cargo test -p simthing-driver --lib` | 16/16 |
| `cargo test -p simthing-core --lib` | 44/44 |
| `cargo test -p simthing-kernel --lib` | 40/40 |
| `cargo test -p simthing-spec --lib` | 13/13 |
| `cargo test -p simthing-driver --test owner_channel_intrinsic_reduce_up_0` | 4/4 |
| `cargo test -p simthing-driver --test slot_logical_identity_0` | 3/3 |
| `cargo test -p simthing-driver --test gated_rates_eml_rewire_0` | 1/1 |
| `cargo test -p simthing-driver --test guyang_comparative_projections_0` | 5/5 |
| inventory-drift | PASS unledgered=0 stale=0 |
| detachability | PASS `production_coupling=0` |
| DOC-BUDGET | PASS |
| stemthing-slot-census | PASS |
| plan-struct-typing-census | PASS |
| gen_orientation --check | PASS |

`planet_id` / `star_system_gridcell_id_raw` absent from core/kernel/gpu/sim/driver `src`. `PlanetChildRfScopeKey` remains `type PlanetChildRfScopeKey = OwnerChannelScopeKey`.
