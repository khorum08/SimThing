# PLAN-STRUCT-TYPING-0 results (Remand 1 discharge)

- Track: 0.0.8.7 RF arena modernization (rung 4.2)
- Status: **PROBATION / proof-present / DA-review-pending** (remand 5108383145 discharged; draft retained)
- HD-RECEIPT: `769199b7423f`
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- ANCHOR-ACK: `simthing-0087-pillars@42b6ba6442aa`
- ANCHOR-ACK: `simthing-0087-binding-laws@91270dd77e96`
- ANCHOR-ACK: `rf-arena-substrate@17b5f1e5c2ba`
- Remand: Board comment `5108383145` (exact prior head `fc377268`)
- base_sha: `0342a28cce8ca891bc283e8ad88d1264d7eee2ba`
- tested_code_sha: `36663d4efda7e2bad235e3442da02d5c7fc46be6`
- implementation_code_sha: `36663d4efda7e2bad235e3442da02d5c7fc46be6`
- final_head_sha: `36663d4efda7e2bad235e3442da02d5c7fc46be6`
- clearance_pr_head: `36663d4efda7e2bad235e3442da02d5c7fc46be6`
- coverage_basis: focused `plan_struct_typing_0` referees + expanded `plan_struct_typing_census.sh` + workspace build + `cargo test -p simthing-core` + adapter-pinned kernel/sim/full driver
- ci_green: local batteries green (hosted Doctrine Scan/Exec re-bound after push)
- expected_route: `DA-RESERVE(gate-wiring)` (handoff); coding leaves the draft PR unmerged for DA clearance
- CLEARANCE-VERDICT: deferred to DA after substantive remand conformance (coding does not self-clear)

## Remand 1 fixes

1. **Stopped production raw ColumnIndex mints**
   - Removed `StructuralScalarChannel::from_authored_channel`; plan-local `StructuralScalarChannel` + sealed `into_plan_column` / `from_structural_plan_channel`.
   - Region-field compiled stencil/commitment/reduction cols stay authored-wire `u32` (`CompiledRegionFieldReduction`); typed only at `column_from_wire` / `encode_column` in driver.
   - `arena_pressure` Named lanes use `PropertyColumnRange::col_for_role` (no oracle door + range arithmetic).
   - `disburse_op` no longer calls `column_from_wire(0)`; unused EvalEML source SlotValue reuses the typed target column.

2. **Preserved ColumnIndex vs RoleOffset**
   - `resolve_node_columns(range, layout, arena)` consumes authoritative `registry.column_range` (helper `resolve_node_columns_for_property`).
   - Referee `resolve_node_columns_honors_nonzero_authoritative_range_start` fails if nonzero start is ignored or a local lane substitutes for the global column.

3. **wgsl_encode as real wire boundary**
   - first_slice GPU config, emission oracle registration, boundary schedule, world_state transfer hash, studio observation bridges route drops through `encode_column` / rematerialize via `column_from_wire` only for real wire/admitted authored fields.
   - Plan-local `n_dims` uses `StructuralScalarChannel::raw()` (not ColumnIndex `.raw_u32()`).

4. **Expanded census** (`scripts/ci/plan_struct_typing_census.sh`)
   - zero production `from_gpu_round_trip` outside `wgsl_encode` (+ door definition)
   - zero production `from_raw_for_oracle_or_rehearsal` outside oracle/rehearsal/test
   - zero `from_authored_channel`
   - zero production `column_from_wire(<literal>)`
   - no fabricated `PropertyColumnRange { start: 0 }` registry substitute in resolve/pressure paths
   - zero targeted plan/WGSL POD `.raw_u32()` drops outside `wgsl_encode`

## Scope Ledger (unavoidable mechanical fallout)

| Path family | Why necessary |
|---|---|
| `simthing-spec` region_field_admission | Remove oracle-door `admit_authored_col`; keep authored wire as `u32` through compile |
| `simthing-sim` accumulator plan tick / fixtures | StructuralScalarChannel INPUT/OUTPUT + `into_plan_column` API fallout |
| `simthing-mapeditor` studio_live_session_bridge | `resolve_node_columns_for_property` + observation `encode_column` drops |
| `simthing-workshop` / clausething RF tests | Call-site signature update for authoritative range resolve |
| `first_slice_mapping_runtime` | Family C **type adaptation only** (wire encode/decode); no EML/magic redesign |

## Permanent referees

| Referee | Regression caught |
|---|---|
| `encode_column_and_column_from_wire_round_trip_bits` | Boundary helpers lose wire bits or sentinel `u32::MAX` |
| `governed_pair_wire_bytes_drop_only_through_encode_column` | GovernedPair POD size/layout or encode-path drift |
| `accumulator_op_encode_preserves_typed_column_wire_bits` | AccumulatorOp→GPU column remint / bit drift |
| `resolve_node_columns_returns_typed_column_index` | Arena resolve falls back to raw `u32` plan fields |
| `resolve_node_columns_honors_nonzero_authoritative_range_start` | Nonzero registry start ignored / local lane substitutes for global col |
| `arena_plan_ops_carry_resolved_column_index_without_remint` | Arena planner remints resolved cols |
| `node_column_refs_optional_sentinel_stays_option_until_encode` | Optional balance cols collapse to raw sentinel early |
| `scripts/ci/plan_struct_typing_census.sh` | Full 4.2 authority + wire-boundary census (six arms) |

## Proof (local)

| Check | Result |
|---|---|
| NEW kernel `plan_struct_typing_0` | 3 passed / 0 failed |
| NEW driver `plan_struct_typing_0` | 4 passed / 0 failed |
| `plan_struct_typing_census.sh` (full) | PASS (all six arms) |
| `cargo build --workspace` | PASS |
| `cargo test -p simthing-core` | PASS — 36 passed / 0 failed (14 unit + 22 doc) |
| live-pinned `cargo test -p simthing-kernel` | PASS — 81 passed / 0 failed (40 unit + 3 plan_struct + 38 doc) |
| live-pinned `cargo test -p simthing-sim` | PASS — 35 passed / 0 failed |
| live-pinned `cargo test -p simthing-driver` | PASS — **123 passed / 0 failed / 13 ignored / 63 harnesses** |
| GPU selection | PASS — adapter match required (`4080` / Vulkan) |
| Command | `$env:WGPU_BACKEND='vulkan'; $env:SIMTHING_GPU_ADAPTER_CONTAINS='4080'; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH='1'; cargo test -p simthing-driver` |
| `agent_scan` / doc-budget / anchors / execution census | PASS / INSPECT(COLUMN-INDEX-MINT at wgsl_encode only) / PASS / PASS |

## Fences held

- Zero Family C / hardcoded-slice deletion / gated_rates redesign
- Zero 9.2 legacy `ColumnIndex::new` / exclusion-list retirement
- Zero WGSL semantic or `repr(C)` layout edits
- Zero authored slot/column serde changes
- PR remains draft; coding does not merge; no next-rung dispatch
