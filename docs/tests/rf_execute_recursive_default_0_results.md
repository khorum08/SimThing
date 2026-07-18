# RF-EXECUTE-RECURSIVE-DEFAULT-0 Results

## Status

**PROBATION — remand implementation and local evidence complete; ready for orchestration review** (2026-07-17).

Tested code commit: `6aa302b59ad29cded8060434b519303531be9ff8`.

Clearance routing observed on the reviewed head: `DA-RESERVE(unclassified-scope)`.

The admitted Arena Resource Flow plan is now the `GameModeSpec` execution-profile default. Ordinary
`SimSession::open_from_spec` plus `step_once` executes the already-materialized flat or nested topology
through existing OrderBands, writes disburse-down results to `AllocatedFlow`, and reports
`economy_execution_deferred=false`. Explicit `DefaultDisabled` remains the opt-out.

## Load-bearing execution proof

The workshop-homed live fixture authors a neutral D=3 hierarchy with a real sibling aggregate:

`session root -> named Owner aggregate -> {selected child, fixed sibling A, fixed sibling B}`

It leaves `ResourceFlowOptInMode` disabled and uses `ResourceFlowExecutionProfile::default()`, proving
the default profile rather than the old `FlatStarOptIn` route. Five fresh GPU sessions run through
ordinary `step_once`: positive, bit-exact replay, selected-contribution control, governed-Balance
disconnect, and explicit `DefaultDisabled`. The selected-contribution control changes only the selected
child's intrinsic contribution from `5.5` to `0`; sibling values, weights, topology, root budget, profile,
and execution path remain bit-identical.

Observed output:

```text
RF2-EXECUTED-DEFAULT: depth=3 bands=8 named_child_marginal=5.5 sibling_aggregate=5.375 owner_aggregate_with=10.875 owner_aggregate_without=5.375 leaf_allocations_with=[4.5750003, 12.200001, 6.1000004] leaf_allocations_without=[3.4750001, 9.266667, 4.6333337] owner_residual=-0.0000019073486 owner_balance_delta=-0.0000019073486 rf1_allocator=PASS rf1_structural=PASS rf1_recipe=VACUOUS deterministic_bits=PASS economy_execution_deferred=false
RF2-RUNTIME-BALANCE-REMOVED: owner_budget=22.875 leaf_allocations=[4.5750003, 12.200001, 6.1000004] residual=-0.0000019073486 owner_rate=-0.0000019073486 actual_owner_delta=0 result=ResidualNotIntegrated
RF2-DEFAULT-DISABLED: flag_source=DefaultDisabled rf_active=false bands=0 owner_aggregate=0 owner_allocation=0 leaf_allocations=[0.0, 0.0, 0.0]
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The fixed siblings contribute `5.375`, so the Owner aggregate remains non-zero in the control. The
positive aggregate is `5.375 + 5.5 = 10.875`, and the positive/control aggregate differential is exactly
the selected marginal `5.5`. Every control leaf retains a non-zero allocation; the total downstream
`AllocatedFlow` differential matches the selected marginal within the declared RF f32 bound.

## RF-1 judge

The independent RF-1 oracle remains unchanged. The D=3 proof reads the actual Balance and governed-rate
columns before and after ordinary `step_once` for both intermediate allocators. It supplies the measured
child disbursements and measured Balance deltas to RF-1, plus structural evidence built from admitted
participant IDs, declared intrinsic-source IDs, and parent-disbursement recipients. The composite report
passes allocator and structural/no-orphan checks; no recipe executes, so recipe exactness is explicitly
vacuous.

The executed Owner allocator has budget `22.875` and a deterministic non-zero arithmetic residual
`-0.0000019073486`, within `O(epsilon * 3)`. Its actual measured Balance delta is
`-0.0000019073486`. The paired runtime negative removes only the Balance `governed_by` connection; its
topology, weights, intrinsic inputs, residual rate, and GPU disbursements are unchanged, but the actual
Balance delta is `0`, so RF-1 returns `ResidualNotIntegrated`.

## Explicit opt-out

The same authored Arena run with `ResourceFlowExecutionProfile::DefaultDisabled` reports
`flag_source=DefaultDisabled`, `rf_active=false`, and `bands=0`. Its Owner aggregate, Owner
`AllocatedFlow`, and all leaf `AllocatedFlow` cells remain zero, proving the accepted opt-out is not
bypassed by unconditional execution wiring.

## Verification

- `cargo check -p simthing-spec` — **PASS** (pre-existing warnings only).
- `cargo check -p simthing-driver` — **PASS** (pre-existing warnings only).
- `cargo test -p simthing-workshop --test rf_execute_recursive_default_0 -- --nocapture` — **PASS**, 1 passed; sibling aggregate, live RF-1 judge, runtime disconnect, and opt-out outputs above.
- `cargo test -p simthing-driver --test rf_conservation_oracle_0 -- --nocapture` — **PASS**, 2 passed.
- `cargo test -p simthing-driver --lib` — **PASS**, 8 passed.
- `cargo test -p simthing-spec --test runtime_rf_tick -- --nocapture` — **PASS**, 1 passed.
- `cargo test -p simthing-spec --test runtime_tick_shell -- --nocapture` — **PASS**, 1 passed.
- `cargo test -p simthing-spec --test runtime_tick_history -- --nocapture` — **PASS**, 1 passed.
- `bash scripts/ci/test_inventory_drift_check.sh` — **PASS**, unledgered `0`, stale `0`.
- Exact-head doctrine scan is rerun after the final evidence commit and carried in the coder relay.
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
- Harness ACKs required by the remand: orientation-harness-core `8a365d1c0864`;
  scanner-selftest-delta-gate `34fb2662baae`.
