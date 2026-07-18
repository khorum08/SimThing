# RF-NEED-WEIGHT-PROFILE-TRANSPORT-0 (RF-5) Results

## Status

**PROBATION / proof-present / DA-review-pending** — 2026-07-18 (coder=Grok-CLI).

Remand `5013016439` corrections landed. Owner alone closes screenshot-C [OVL]. Coding agent does not graduate, mark ready, or merge.

## Identity

| Field | Value |
|---|---|
| Rung | `RF-NEED-WEIGHT-PROFILE-TRANSPORT-0` (RF-5) |
| PR | #1414 |
| Base / master | `99fa9d06898a1b03b490b7f876a17dfd4568e500` |
| RF-4 graduation | **DA-GRADUATED / merged #1413 @ `99fa9d06`** |
| HD-RECEIPT | `9132665903e6` |
| ORIENT-RECEIPT | `2c9fde39d1d6` (role=coding) |
| Expected route | `DA-RESERVE(gate-wiring)` |
| tested_code_sha | `4880101b6b4c91e2c7a86210b4e0fe56d2e5f504` |
| implementation_sha | `df55d7d218008484846e70306d6ab95e7ad5e6e6` |
| coverage_basis | PASS (workshop RF-5 4/4 + mapeditor RF-4 elevate 8/8 + AGENT-SCAN PASS) |

## Remand corrections

| Defect | Fix |
|---|---|
| Studio fabricates seeds/inputs/thresholds/owners | **Removed** `studio_need_weight_profile_bindings`. Studio only preserves already-admitted `GameModeSpec.resource_flow.need_weight_profiles`. |
| Threshold proof was `value >= thr` | Need thresholds inject into `resource_economy.emit_on_threshold`; sealed substrate proven via `rebuild_emit_on_threshold_ops` + `execute_threshold_ops_cpu` (same encode path as economy/Studio upload). Fabric post-RF rescan added for live GPU event path. |
| Synthetic `rf_need::*` property | **Removed.** Need writes existing Arena **AllocatorWeight** cell via `col_for_role(Named("weight"))`. |
| Missing falsifiers | Paired **clause** authorings (amount_add 0.2 vs 3.0 only); empty weight_properties fail-closed; misbind fail-closed; canonical TP multi-owner separation + neutral second scenario via same promoter. |
| scans.tsv self-exclusion | **Reverted.** ColumnIndex mints go through `column_range.col_for_role` (registry-excluded path only). |

## Where authority comes from

| Quantity | Origin |
|---|---|
| Weight profile stack | Clause `weight_profile` → hydrate → `HydratedFieldEconomyWeightProfile.stack` |
| Install target | Clause `owner_policy_overlay.owner` (ScenarioListed) |
| Weight Amount | Overlay `amount_add`/`amount_mult` on `targets_property` + emission Constant seeds on that property |
| Input Amount | Stockpile/current (or other existing Amount property) on the same install host |
| Live need cell | Existing Arena participant **AllocatorWeight** (`Named("weight")` on flow property) |
| Threshold / events | `NeedWeightProfileThresholdSpec` → injected `EmitOnThresholdRegistration` → sealed AccumulatorOp threshold encode |

## Load-bearing proofs

| Claim | Result |
|---|---|
| Paired clause authorings (overlay amount_add only) diverge live need | PASS |
| Below-threshold sealed oracle fires 0 events | PASS |
| Crossing sealed oracle fires ≥1 event | PASS |
| Empty weight_properties fail closed | PASS |
| Misbound install fail closed | PASS |
| Canonical TP multi-profile owners separated (terran + pirate) | PASS |
| Neutral second scenario same promoter path | PASS |
| No synthetic `rf_need` property | PASS |
| RF-4 mapeditor elevate battery | *(re-run at final head)* |
| agent_scan | *(re-run at final head)* |

## Fences held

- No new ClauseScript syntax or kernel/WGSL primitive.
- No Studio feeder/mirror/arithmetic inventing weights, inputs, thresholds, or owners.
- No synthetic need host property.
- No `value >= thr` presentation comparison as the event proof.
- No 12.10 macro-emergence claim.
- No scans.tsv feature self-exclusion.

## Owner [OVL]

**Blocked until orchestration accepts.** Do not freeze executable or request screenshot C until then.

## Graduation routing

- Risk class: gate-wiring / DA-reserve (expected)
- Falsification: workshop RF-5 4/4 + RF-4 elevate nonregression
- Recommended posture: DEEP-TREE after Owner OVL when orchestration unblocks
