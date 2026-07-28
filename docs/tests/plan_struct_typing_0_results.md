# PLAN-STRUCT-TYPING-0 results (Remand 3 evidence)

- Track: 0.0.8.7 RF arena modernization (rung 4.2)
- Status: **PROBATION / proof-present / DA-review-pending** (remand 5109354146 discharged; draft retained)
- HD-RECEIPT: `769199b7423f`
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- ANCHOR-ACK: `simthing-0087-pillars@42b6ba6442aa`
- ANCHOR-ACK: `simthing-0087-binding-laws@91270dd77e96`
- ANCHOR-ACK: `rf-arena-substrate@17b5f1e5c2ba`
- Remand: Board comment `5109354146` (exact prior head `d7cc15ba1ef4461ccc8ec76adb02e28c6136e619`)
- Prior remands: `5108706100` / landing `5109250521`; `5108383145` / landing `5108597299`
- base_sha: `0342a28cce8ca891bc283e8ad88d1264d7eee2ba`
- tested_code_sha: `3b30c5bf608903dc7cdc757ae93cb6bceb002455`
- implementation_code_sha: `3b30c5bf608903dc7cdc757ae93cb6bceb002455`
- final_head_sha: PR-body-bound (see PR #1480 `final_head_sha` after push; this file does not self-hash)
- clearance_pr_head: PR-body-bound (see PR #1480 `clearance_pr_head` after push; this file does not self-hash)
- coverage_basis: focused plan_struct_typing_0 + region-field admit referees + authored_admit_door unit + seven-arm census + adapter-pinned kernel/sim/full driver
- ci_green: local inventory drift PASS; Remand-2 batteries retained; hosted Doctrine Scan/Exec/Clearance re-bound on the evidence tip after push
- expected_route: `DA-RESERVE(gate-wiring)`
- CLEARANCE-VERDICT: `DA-RESERVE(gate-wiring)`

## Remand 3 (evidence-only)

1. Ledgered permanent unit row for `authored_admit_door_rejects_out_of_range_and_preserves_in_range` in `scripts/ci/test_inventory.tsv` (no referee deletion/weaken).
2. Stopped self-referential tip rebind: `tested_code_sha` / `implementation_code_sha` stay on the Remand-2 code-bearing identity `3b30c5bf…`; `final_head_sha` / `clearance_pr_head` live only in the PR body after the final evidence commit.

## Remand 2 substantive (preserved; no production Rust edits this remand)

1. Typed region-field compiled columns via bounded `try_from_admitted_authored`; first_slice encode-only.
2. Census arm 7 — Family-B plan column identities are typed.
3. Exact-head fields carried in PR body with normal Markdown backticks.

## Scope Ledger

| Path family | Why necessary |
|---|---|
| `scripts/ci/test_inventory.tsv` | Ledger missing admit-door unit (TEST-INVENTORY-DRIFT) |
| `docs/tests/plan_struct_typing_0_results.md` | Evidence identity cleanup only |
| Remand-2 production surfaces | Unchanged |

## Permanent referees

| Referee | Regression caught |
|---|---|
| kernel/driver `plan_struct_typing_0` suite | Wire boundary + arena range authority |
| `authored_admit_door_rejects_out_of_range_and_preserves_in_range` | Authored admit door admits OOR / loses in-range bits |
| `authored_region_field_columns_admit_as_typed_column_index` | Compiled region-field cols stay raw `u32` |
| `authored_region_field_columns_out_of_range_are_rejected` | Out-of-range authored cols admitted |
| `scripts/ci/plan_struct_typing_census.sh` | Seven-arm authority + plan-field census |

## Proof (local Remand 3)

| Check | Result |
|---|---|
| `test_inventory_drift_check` | PASS after ledger row |
| production Rust | unchanged |

## Fences held

- Zero production Rust / test semantic edits
- Zero Family C / 9.2 mint-sweep / WGSL / authored-serde change
- PR remains draft; no merge; no next-rung dispatch
