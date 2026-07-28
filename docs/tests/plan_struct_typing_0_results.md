# PLAN-STRUCT-TYPING-0 results

- Track: 0.0.8.7 RF arena modernization (rung 4.2)
- Status: **PROBATION / proof-present / DA-review-pending**
- HD-RECEIPT: `769199b7423f`
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- ANCHOR-ACK: `simthing-0087-pillars@42b6ba6442aa`
- ANCHOR-ACK: `simthing-0087-binding-laws@91270dd77e96`
- ANCHOR-ACK: `rf-arena-substrate@17b5f1e5c2ba`
- base_sha: `0342a28cce8ca891bc283e8ad88d1264d7eee2ba`
- expected_route: `DA-RESERVE(gate-wiring)` (handoff); coding leaves the draft PR unmerged for DA clearance

## What landed

- Family B plan/compile intermediates carry `ColumnIndex` end-to-end: arena `NodeColumnRefs`, resource-economy `CompiledResource*`, transfer/emission/intensity registrations, `StructuralScalarChannel`, and compiled region-field stencil/commitment cols.
- `wgsl_encode::{encode_column, column_from_wire}` is the sole production raw drop / GPU round-trip rematerialize boundary; AccumulatorOp encode, overlay prep, packed upload, need-binding EML wire, and child-share slot encode route through it.
- Production census: zero `ColumnIndex::from_gpu_round_trip` outside `column_index.rs` + `wgsl_encode.rs`.
- Authored/wire serde (`RegionFieldSpec`, scenario channels) stays `u32`; Family C (`gated_rates`, `first_slice_mapping_runtime` magic) and 9.2 mint-sweep untouched.
- Ladder 4.2 PROBATION; Active open → `ANCHOR-DISPOSITION-ADMISSION-0`.

## Permanent referees

| Referee | Regression caught |
|---|---|
| `encode_column_and_column_from_wire_round_trip_bits` | Boundary helpers lose wire bits or sentinel `u32::MAX` |
| `governed_pair_wire_bytes_drop_only_through_encode_column` | GovernedPair POD size/layout or encode-path drift |
| `accumulator_op_encode_preserves_typed_column_wire_bits` | AccumulatorOp→GPU column remint / bit drift |
| `resolve_node_columns_returns_typed_column_index` | Arena resolve falls back to raw `u32` plan fields |
| `arena_plan_ops_carry_resolved_column_index_without_remint` | Arena planner remints resolved cols |
| `node_column_refs_optional_sentinel_stays_option_until_encode` | Optional balance cols collapse to raw sentinel early |
| `scripts/ci/plan_struct_typing_census.sh` | Production `from_gpu_round_trip` escapes `wgsl_encode` |

## Proof (local)

| Check | Result |
|---|---|
| NEW kernel `plan_struct_typing_0` | 3 passed / 0 failed |
| NEW driver `plan_struct_typing_0` | 3 passed / 0 failed |
| `plan_struct_typing_census.sh` | PASS |
| `cargo build --workspace` | PASS |
| `cargo test -p simthing-core` | PASS — 36 passed / 0 failed (14 unit + 22 doc) |
| live-pinned `cargo test -p simthing-kernel` | PASS — 81 passed / 0 failed (40 unit + 3 plan_struct + 38 doc) |
| live-pinned `cargo test -p simthing-sim` | PASS — 35 passed / 0 failed |
| live-pinned `cargo test -p simthing-driver` | PASS — **122 passed / 0 failed / 13 ignored / 64 harnesses** |
| GPU selection | PASS — adapter match required (`4080` / Vulkan) |
| Command | `$env:WGPU_BACKEND='vulkan'; $env:SIMTHING_GPU_ADAPTER_CONTAINS='4080'; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH='1'; cargo test -p simthing-driver` |

Final SHA / hosted Doctrine Scan+Exec IDs: bound in draft PR body after batteries land.

## Fences held

- Zero Family C / hardcoded-slice deletion
- Zero 9.2 legacy `ColumnIndex::new` / exclusion-list retirement
- Zero WGSL semantic or `repr(C)` layout edits
- Zero authored slot/column serde changes
- PR remains draft; coding does not merge; no next-rung dispatch
