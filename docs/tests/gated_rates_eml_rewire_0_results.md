# GATED-RATES-EML-REWIRE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 9.1)
- Status: **PROBATION / proof-present / DA-review-pending**
- HD-RECEIPT: `30b58045c5ed`
- ORIENT-RECEIPT: `a5dc59920dd4`
- orientation_rule_stamp: `61818ff7d4adda84`
- Board dispatch: comment `5342561390`
- expected_route: `DA-RESERVE(gate-wiring)`

## What landed

`gated_rates` keeps `ColumnIndex` from `col_for_role` / `resolve_node_columns_for_property`. It no longer remints via `ColumnIndex::new` or `flow_start + offset`. Trees register on the ordinary `EmlExpressionRegistry` and inherit `MAX_EML_TREE_NODES`. Gate formulas stay the existing CMP_GE staircase (no EXP/LN consumed; no third exact-consumer variant).

`crates/simthing-driver/src/first_slice_mapping_runtime.rs` is deleted. Live mapping session/callers moved to `mapping_runtime.rs`, which admits columns with `try_from_admitted_authored` / compiled `urgency_col`.

## Falsifiers

| Check | Production | Wrong implementation |
|---|---|---|
| Role-pathway columns | `col_for_role` / admitted authored | test-side `from_raw_for_oracle_or_rehearsal(1)` disagrees with admitted col 4 |
| No raw remint | grep absence of `ColumnIndex::new` | would restore Family C mint |
| Mapping file | `first_slice_mapping_runtime.rs` absent | file present |
| Mapping cols | no `let eml_resource_col = 1` | hardcoded magic |

EXP/LN necessity was not opened: production gated-rate trees do not contain EXP/LN opcodes.

## Blast radius

See PR body / board return for exact counts at `tested_code_sha`.
