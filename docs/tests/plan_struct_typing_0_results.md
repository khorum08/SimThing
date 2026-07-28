# PLAN-STRUCT-TYPING-0 results (Remand 2 discharge)

- Track: 0.0.8.7 RF arena modernization (rung 4.2)
- Status: **PROBATION / proof-present / DA-review-pending** (remand 5108706100 discharged; draft retained)
- HD-RECEIPT: `769199b7423f`
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- ANCHOR-ACK: `simthing-0087-pillars@42b6ba6442aa`
- ANCHOR-ACK: `simthing-0087-binding-laws@91270dd77e96`
- ANCHOR-ACK: `rf-arena-substrate@17b5f1e5c2ba`
- Remand: Board comment `5108706100` (exact prior head `c659c0c27a9b52959446b184e51f8be03328c416`)
- Prior remand: `5108383145` / landing `5108597299`
- base_sha: `0342a28cce8ca891bc283e8ad88d1264d7eee2ba`
- tested_code_sha: `27ccf329e3e684cae55cefc8145dc9d5b6bf317d`
- implementation_code_sha: `27ccf329e3e684cae55cefc8145dc9d5b6bf317d`
- final_head_sha: `27ccf329e3e684cae55cefc8145dc9d5b6bf317d`
- clearance_pr_head: `27ccf329e3e684cae55cefc8145dc9d5b6bf317d`
- coverage_basis: focused plan_struct_typing_0 + region-field admit referees + seven-arm census + workspace/core + adapter-pinned kernel/sim/full driver
- ci_green: local batteries green (region-field admit 2/0; plan_struct kernel 3/0 driver 4/0; census 7-arm PASS; doc-budget PASS; execution census mixed_ruled=0; agent_scan INSPECT justified; adapter-pinned driver 123/0/13/64); hosted Doctrine Scan/Exec re-bound after push
- expected_route: `DA-RESERVE(gate-wiring)`
- CLEARANCE-VERDICT: `DA-RESERVE(gate-wiring)`

## Remand 2 fixes

1. **Typed region-field compiled columns**
   - `ColumnIndex::try_from_admitted_authored(raw, bound)` — bounded authored→plan door (not a bare constructor).
   - `CompiledRegionFieldStencilSpec::{source_col,target_col}`, `SaturatingFlux::choke_output_col`, `CompiledRegionFieldReduction::{child_col,parent_col}`, `CompiledFirstSliceCommitmentThreshold::urgency_col` are `ColumnIndex` / `Option<ColumnIndex>`.
   - `first_slice_mapping_runtime` drops via `encode_column` only (no wire remint of already-typed plan cols).
   - Referees: in-range admit preserves typed identity; out-of-range authored columns rejected.

2. **Census arm 7** — Family-B compiled/plan records must not declare column identities as `u32`/`Option<u32>` (named production compile/plan files; authored serde and WGSL/POD wire excluded).

3. **Exact-head fields** — bound to the remedial tip in this doc and PR #1480 body (normal Markdown backticks).

## Scope Ledger (unchanged mechanical fallout from Remand 1, plus Remand 2)

| Path family | Why necessary |
|---|---|
| `simthing-spec` region_field_admission | Bounded admit + typed compiled column fields |
| `simthing-driver` first_slice_mapping_runtime | Family C type adaptation only — encode typed cols |
| Remand-1 fallout (sim/mapeditor/workshop) | Preserved |

## Permanent referees

| Referee | Regression caught |
|---|---|
| kernel/driver `plan_struct_typing_0` suite | Wire boundary + arena range authority |
| `authored_region_field_columns_admit_as_typed_column_index` | Compiled region-field cols stay raw `u32` |
| `authored_region_field_columns_out_of_range_are_rejected` | Out-of-range authored cols admitted |
| `scripts/ci/plan_struct_typing_census.sh` | Seven-arm authority + plan-field census |

## Proof (local)

| Check | Result |
|---|---|
| region-field admit unit referees | 2 passed / 0 failed |
| kernel `plan_struct_typing_0` | 3 passed / 0 failed |
| driver `plan_struct_typing_0` | 4 passed / 0 failed |
| `plan_struct_typing_census.sh` (7 arms) | PASS |
| `cargo build --workspace` | PASS |
| `cargo test -p simthing-core` | PASS |
| live-pinned `cargo test -p simthing-kernel` | PASS |
| live-pinned `cargo test -p simthing-sim` | PASS |
| live-pinned `cargo test -p simthing-driver` | PASS — **123 passed / 0 failed / 13 ignored / 63 harnesses** |
| GPU selection | PASS — `4080` / Vulkan |
| Command | `$env:WGPU_BACKEND='vulkan'; $env:SIMTHING_GPU_ADAPTER_CONTAINS='4080'; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH='1'; cargo test -p simthing-driver` |

## Fences held

- Zero Family C / hardcoded-slice deletion / gated_rates redesign
- Zero 9.2 mint-sweep / exclusion retirement
- Zero WGSL semantic or `repr(C)` layout edits
- Zero authored/serde slot/column changes
- PR remains draft; no merge; no next-rung dispatch
