# RF-EXECUTE-RECURSIVE-DEFAULT-0 Results

## Status

**PROBATION — implementation and local evidence complete; ready for orchestration/DA review** (2026-07-17).

Tested code commit: `61ab842ff9a844eb9d3eb1aa09348fa77503a1e9`.

The admitted Arena Resource Flow plan is now the `GameModeSpec` execution-profile default. Ordinary
`SimSession::open_from_spec` plus `step_once` executes the already-materialized flat or nested topology
through existing OrderBands, writes disburse-down results to `AllocatedFlow`, and reports
`economy_execution_deferred=false`. Explicit `DefaultDisabled` remains the opt-out.

## Load-bearing execution proof

The workshop-homed live fixture authors a neutral D=3 hierarchy:

`session root -> named Owner aggregate -> named child source`

It leaves `ResourceFlowOptInMode` disabled and uses `ResourceFlowExecutionProfile::default()`, proving
the default profile rather than the old `FlatStarOptIn` route. Three fresh GPU sessions run through
ordinary `step_once`: positive, bit-exact replay, and a control that changes only the named child's
intrinsic contribution from `5.5` to `0`.

Observed output:

```text
RF2-EXECUTED-DEFAULT: depth=3 bands=8 named_child_marginal=5.5 owner_aggregate_with=5.5 owner_aggregate_without=0 local_allocation_with=17.5 local_allocation_without=12 deterministic_bits=PASS economy_execution_deferred=false
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The control preserves budget, weights, participant topology, and execution path. Removing only the
child contribution makes the Owner aggregate exactly zero; the positive-control differential is
exactly `5.5`. The runtime `AllocatedFlow` writeback differential is also exactly `5.5` (`12 -> 17.5`).

## RF-1 judge

The independent RF-1 oracle remains unchanged and passes on the executed allocator primitives:

```text
RF-MEASURED-BALANCE: budget=10 sum_disbursed=10.000001 residual=-0.0000009536743 bound=0.0000667572 root_rate=-0.0000011920929 root_delta=-0.0000011920929 leaf_deltas=[Some(0.0), Some(0.0), Some(0.0), Some(0.0), Some(0.0), Some(0.0), Some(0.0)]
RF-RUNTIME-BALANCE-REMOVED: budget=10 sum_disbursed=10.000001 residual=-0.0000009536743 root_rate=-0.0000011920929 actual_root_delta=0 result=ResidualNotIntegrated { arithmetic_residual: -9.536743e-7, reported_balance_residual: Some(0.0) }
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

The recipe, allocator, and structural oracle families remain green; no recursive-source report module
is imported by the oracle or by the new live fixture.

## Verification

- `cargo check -p simthing-spec` — **PASS** (pre-existing warnings only).
- `cargo check -p simthing-driver` — **PASS** (pre-existing warnings only).
- `cargo test -p simthing-workshop --test rf_execute_recursive_default_0 default_step_once_recursively_reduces_named_child_and_writes_local_allocation -- --nocapture` — **PASS**, 1 passed; live D=3 output above.
- `cargo test -p simthing-driver --test rf_conservation_oracle_0 -- --nocapture` — **PASS**, 2 passed.
- `cargo test -p simthing-driver --lib` — **PASS**, 8 passed.
- `cargo test -p simthing-spec --test runtime_rf_tick -- --nocapture` — **PASS**, 1 passed.
- `cargo test -p simthing-spec --test runtime_tick_shell -- --nocapture` — **PASS**, 1 passed.
- `cargo test -p simthing-spec --test runtime_tick_history -- --nocapture` — **PASS**, 1 passed.
- `bash scripts/ci/test_inventory_drift_check.sh` — **PASS**, unledgered `0`, stale `0`.
- `bash scripts/ci/doctrine_pr_scan.sh e60082b1e0358fe793257e4151d037c03da250ef 61ab842ff9a844eb9d3eb1aa09348fa77503a1e9` — **PASS**, failures `0`, INSPECT `0`; `WORKSHOP-HOMING-DETECTION PASS 0`; test inventory drift **PASS**.
- `bash scripts/ci/gen_orientation.sh --check` — **PASS** after regeneration.
- `bash scripts/ci/doc_budget_check.sh --check` — **PASS**.

## Scope and fences

- Existing `AccumulatorOp`, governed integration, and OrderBand execution only; no kernel, WGSL, GPU,
  grammar, spec-role, or accumulator primitive was added.
- No RUNTIME-0080 RR-3/RR-4 transplant and no CPU runtime planner/decision path.
- No Studio-side RF arithmetic and no scenario-specific code or tests outside `simthing-workshop`.
- RF-1 source, tests, independence fence, and bite remain unchanged.
- The historical serialized profile name `FlatStarResourceFlow` remains compatible; the admitted tree
  already selects flat versus nested planning. RF-3 owns the broader legacy naming/doc sweep.
- The pre-existing untracked generated scenario output was left untouched.

## Orientation receipt

- `HD-RECEIPT 6a2771cb341f`
- `ORIENT-RECEIPT 46d89a04fc85`
- Doctrine ACKs: field-policy `ae2d4c2c0c7d`; founding `b960ed2d493d`; one-tree `c88002b72898`;
  property-value `084ee935326b`; stead `b4a112cd02e8`; structural `17fa0732f44d`; workshop `3e584f0ad175`.
