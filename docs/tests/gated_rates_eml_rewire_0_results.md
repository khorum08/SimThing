# GATED-RATES-EML-REWIRE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 9.1)
Status: **COMPLETE — DA-GRADUATED / merged #1789 @ `059b4896`** (5.14 reconciliation #1787, remand `5343274168`, substrate STOP ruling `5351400933`, graduation ruling on Board #1332)
- HD-RECEIPT: `30b58045c5ed`
- ORIENT-RECEIPT: `a5dc59920dd4`
- orientation_rule_stamp: `61818ff7d4adda84`
- Board dispatch: comment `5342561390`
- DA identity ruling: `5351400933`
- Orchestrator resume: `5351414730`
- expected_route: `DA-RESERVE(gate-wiring)`

## What landed

`gated_rates` keeps `ColumnIndex` from `col_for_role` / `resolve_node_columns_for_property`. Trees register on the ordinary `EmlExpressionRegistry` and inherit `MAX_EML_TREE_NODES`. Gate formulas stay the existing CMP_GE staircase (no EXP/LN).

`first_slice_mapping_runtime.rs` remains deleted. Mapping gadget lanes are named plan-local `StructuralScalarChannel` identities (`EML_RESOURCE`, `EML_WEIGHT_PRESSURE`, `EML_WEIGHT_RESOURCE`) with an explicit `raw < n_dims` check. They convert to `ColumnIndex` only through `into_plan_column()` on AccumulatorOp / EvalEML surfaces. `eml_output_col` still comes from compiled `commitment.urgency_col`.

## Executed oracles

| Case | Production | Oracle / rival | Result |
|---|---|---|---|
| Gate below / equal / above / ungated add / gated mult | `build_gated_rate_ops` + `eval_eml_cpu` | independent `trigger >= at_least` referee | bit-identical |
| Mapping successor | `field_urgency_eml_nodes` via named structural channels | pre-delete positional SLOT_VALUE 1/2/3 | bit-identical |
| Authored-admit mutant | named `EML_RESOURCE` | `try_from_admitted_authored(urgency_col=4)` as resource | RED (values disagree) |
| Always-on gate mutant | CMP_GE production | drop the gate | RED when trigger < at_least |
| Plan bound | `field_urgency_plan_channels(2)` | channel 2 in n_dims 2 | `PlanChannelOutOfGrid` |

EXP/LN necessity was not opened: production gated-rate trees do not contain EXP/LN opcodes.

## Blast radius

See PR body / board return for exact counts at `tested_code_sha`.
