# EVENT-GENERATION-STAMP-0 results

- Track: 0.0.8.7 RF arena modernization (rung 6.1)
- Status: **PROBATION / proof-present / DA-review-pending** (Remand 1 discharge)
- ORIENT-RECEIPT: `4a101ed6652d`
- orientation_rule_stamp: `abd646955a48aa4a`
- HD-RECEIPT: `22c8f88826dd`
- Dispatch: Board comment `5165742228`
- Remand: Board comment `5165983417`
- base_sha (handoff): `49bc1d4a`
- draft PR: #1596
- expected_route: `DA-RESERVE(gate-wiring)`
- Scope: **6.1 ONLY** (no 6.1b / 6.2 / 6.2b / 6.3)

## Remand 1 repairs (production seams)

| # | Defect | Repair |
|---|---|---|
| 1 | Event stamps optional | Sealed mint **requires** generation; GPU readback stamps by construction via session/world generation; `production_sealed` flag; unsealed strip REDs at egress |
| 2 | Reduce-up optional stamp | `reduce_owner_channel_rf(..., generation)` returns `StampedReduceUpProduct`; raw report integrate door hard-errors |
| 3 | Replay only receipts | `ParentRfIntegrationState` + `replay_reduce_up_schedule` bit-exact integrated RF state |
| 4 | Isolated ring | Production `push_emissions_into_production_egress` on session; kernel egress rejects unsealed |
| 5 | No wait mutant | `plant_wait_for_fresh_child_mutant` REDs N+3<-N; restored path green |
| 6 | Dispatch dissolve optional | `deliver_routed_overlay` admits Event/System Instruction/Custom only with `UntilDissolvedWith` |

## Local proof

```text
simthing-core event_generation_stamp_0: 4 passed
simthing-core simthing_automaton_intrinsic_0: 3 passed
simthing-spec event_generation_stamp_reduce_up_0: 4 passed
simthing-kernel generation_stamp_tests: 2 passed
simthing-driver owner_channel_intrinsic_reduce_up_0: 4 passed
simthing-driver simthing_automaton_rf_reception_0: 2 passed
agent_scan: hard FAIL 0 (TEST-BUDGET INSPECT justified)
test_inventory_drift: PASS
```

## Posture

**PROBATION / proof-present / DA-review-pending** under `DA-RESERVE(gate-wiring)`.

Coder does **not** invoke `/clearance` or merge.
