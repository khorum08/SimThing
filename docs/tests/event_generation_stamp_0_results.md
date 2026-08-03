# EVENT-GENERATION-STAMP-0 results

- Track: 0.0.8.7 RF arena modernization (rung 6.1)
- Status: **PROBATION / proof-present / DA-review-pending** (Remand 2 + DA addendum)
- ORIENT-RECEIPT: `8bcf881f793a`
- orientation_rule_stamp: `2e9a349eddfe2d31`
- HD-RECEIPT: `9df0629526ec` (re-issued; supersedes `22c8f88826dd`)
- Dispatch: `5165742228`
- Remand 1: `5165983417` / Remand 2: `5166255331` / DA addendum: `5166364435`
- draft PR: #1596
- expected_route: `DA-RESERVE(gate-wiring)`
- Scope: **6.1 ONLY**

## Remand 2 + DA repairs

| # | Requirement | Repair |
|---|---|---|
| 1 | Wait path out of production | `WAIT_FOR_*` / plant / wait-mutant integrate are `#[cfg(test)]` only; production `integrate_stamped_reduce_up` has zero wait branch |
| 2 | Generation authority live | `WorldGpuState::bind_production_generation` at driver hot-cycle + boundary + post-boundary; stamps all AccumulatorOp sessions |
| 3 | Oracle not gen-0 blind | `execute_*_with_emissions` / `cpu_oracle_emission_records` / burn-in require generation; parity asserts stamp match |
| 4 | Live ring egress sequence | Production-sequence unit proof: gen 1→2 stamps + ring forced lag; unsealed REDs |
| 5 (DA) | Schedule per-product full gen set | `IntegrationSchedule` append-only per-product rows; proof keeps both gens for same product_key |

## Local proofs

```text
spec event_generation_stamp_reduce_up_0: 4/0
spec wait_mutant_proof (lib): 1/0
kernel generation_stamp_tests: 3/0
core event_generation_stamp_0: 4/0
core automaton intrinsic: 3/0
driver owner_channel_intrinsic_reduce_up_0: 4/0
core cargo test --doc: 23/0
agent_scan hard FAIL 0 (TEST-BUDGET INSPECT)
inventory drift PASS
```

## Posture

**PROBATION / proof-present / DA-review-pending**. No `/clearance`. No merge.
