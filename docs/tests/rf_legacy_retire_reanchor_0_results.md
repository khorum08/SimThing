# RF-LEGACY-RETIRE-REANCHOR-0 Results

## Status and binding

**DA-GRADUATED / merged [#1412](https://github.com/khorum08/SimThing/pull/1412) @ `d42b9109c032f96a66784a9274b5812107a32e45`** — 2026-07-18.

Tested code SHA: `63d89ff9d4994f390f6bfede08f0d5b0823c7d66`.
Evidence head SHA: `267e0b2c1fe05fc6c7e98fc163dd59dd78a9996a`.

DA returned merge authority to orchestration with expected-head protection. RF-4 (`STUDIO-FIELD-SESSION-ELEVATE-0`, 12.9) resumes on the executing recursive RF substrate; #1405 remains open as salvage provenance until RF-4 mines it. Owner retains [OVL] screenshot closure.

Coverage basis: live-adapter ct_2a and ct_2c execution through ordinary recursive-default
`SimSession::step_once`; unchanged independent RF-1 conservation judgment over measured GPU Balance;
the RF-2 D=3 sibling marginal, runtime governed-Balance disconnect, deterministic replay, and explicit
`DefaultDisabled` control; serde compatibility proof; touched-crate compile checks; exact-head harness
scans. No new test was added.

## Retirement and re-anchor

- `ResourceFlowExecutionProfile::RecursiveArenaResourceFlow` is the canonical default and serialized
  spelling. Historical `FlatStarResourceFlow` input deserializes only as an alias for that variant and
  reserializes canonically; there is no legacy profile variant or dispatch arm to reactivate.
- Session enablement now names the admitted Arena path (`enables_arena_resource_flow`). The materialized
  participant tree selects flat or nested topology within the same plan.
- `DefaultDisabled` remains the explicit opt-out. The RF-2 control reports `rf_active=false`, zero bands,
  and zero allocation cells.
- The Resource Flow ADR and founding allocation anchor now state recursive-default execution plus
  independent RF-1 judgment. RR-3/RR-4 module framing identifies those modules as falsification
  rehearsals, never `step_once` authority.
- RF-2A and RF-2 remain DA-graduated and merged in `#1411` at `c206b0ef`; RF-3 is DA-graduated and
  merged in `#1412` at `d42b9109`; RF-4 is the active resumed rung.

## ct_2a / ct_2c fail-then-pass

Before repair, both live tests reached the GPU-backed session but planned from the stale authoring tree:

```text
ct_2a execution plan: EmptyParticipants { arena: "ct2a_food" }
ct_2c execution plan: EmptyParticipants { arena: "settlement_energy" }
```

The tests now enroll a root plus three leaves through existing `ExplicitParticipantSpec` materialization
and build the plan from the admitted `session.proto.root`. They leave spec opt-in disabled and assert
`ScenarioClassDefaultOn`, so ordinary default execution is load-bearing. Adapter/session acquisition uses
`expect`; the former `open_from_spec_or_skip` / `NoAdapter` return path is gone, so NoAdapter/Unsupported is
a failing test result rather than a passing skip.

Observed live pass output:

```text
RF3-CT2A: participants=4 disbursed=[2.2857144, 4.571429, 1.1428572] residual=-0.0000009536743 balance_delta=-0.0000009536743 rf1=PASS flag_source=ScenarioClassDefaultOn
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

RF3-CT2C: participants=4 disbursed=[2.1000001, 5.6000004, 2.8000002] residual=-0.0000009536743 balance_delta=-0.0000009536743 rf1=PASS flag_source=ScenarioClassDefaultOn
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Both fixtures independently compute a deterministic non-zero f32 allocator residual and read the actual
root Balance before and after GPU execution. The measured delta exactly matches that residual and passes
unchanged RF-1; neither proof can collapse to zero/zero.

## Load-bearing falsification

The unchanged RF-2 recursive-default test remains the runtime-path falsifier. With the governed Balance
connection removed while budget, weights, topology, and disbursements remain fixed, actual GPU Balance
delta is zero and RF-1 bites:

```text
RF2-EXECUTED-DEFAULT: depth=3 bands=12 named_child_marginal=5.5 sibling_aggregate=5.375 owner_aggregate_with=10.875 owner_aggregate_without=5.375 leaf_allocations_with=[4.5750003, 12.200001, 6.1000004] leaf_allocations_without=[3.4750001, 9.266667, 4.6333337] owner_residual=-0.0000019073486 arena_generated_owner_rate=-0.0000019073486 owner_balance_delta=-0.0000019073486 rf1_allocator=PASS rf1_structural=PASS rf1_recipe=VACUOUS deterministic_bits=PASS economy_execution_deferred=false
RF2-RUNTIME-BALANCE-REMOVED: owner_budget=22.875 leaf_allocations=[4.5750003, 12.200001, 6.1000004] residual=-0.0000019073486 owner_rate=0 actual_owner_delta=0 result=ResidualNotIntegrated
RF2-DEFAULT-DISABLED: flag_source=DefaultDisabled rf_active=false bands=0 owner_aggregate=0 owner_allocation=0 leaf_allocations=[0.0, 0.0, 0.0]
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The independent RF-1 integration control also remains green and retains its real Balance-disconnect bite:

```text
RF-MEASURED-BALANCE: budget=10 sum_disbursed=10.000001 residual=-0.0000009536743 bound=0.0000667572 root_rate=-0.0000011920929 root_delta=-0.0000011920929 leaf_deltas=[Some(0.0), Some(0.0), Some(0.0), Some(0.0), Some(0.0), Some(0.0), Some(0.0)]
RF-RUNTIME-BALANCE-REMOVED: budget=10 sum_disbursed=10.000001 residual=-0.0000009536743 root_rate=-0.0000011920929 actual_root_delta=0 result=ResidualNotIntegrated { arithmetic_residual: -9.536743e-7, reported_balance_residual: Some(0.0) }
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Verification

- `cargo check -p simthing-spec` — PASS (pre-existing warnings only).
- `cargo check -p simthing-driver` — PASS (pre-existing warnings only).
- ct_2a focused live test — PASS, output above.
- ct_2c focused live test — PASS, output above.
- `cargo test -p simthing-workshop --test rf_execute_recursive_default_0 -- --nocapture` — PASS,
  recursive positive/replay/marginal/disconnect/opt-out and alias compatibility.
- `cargo test -p simthing-driver --test rf_conservation_oracle_0 -- --nocapture` — PASS, 2 passed.
- `bash scripts/ci/test_inventory_drift_check.sh` — PASS, unledgered `0`, stale `0`.
- `bash scripts/ci/test_lifecycle_expiry_check.sh --schema` — PASS, expired `0`, audit `0`.
- `bash scripts/ci/agent_scan.sh` at tested code SHA — PASS, delta INSPECT `0`.
- `bash scripts/ci/doctrine_pr_scan.sh c206b0ef6b6ef99cfdaac6361c32db529b115b1f 63d89ff9d4994f390f6bfede08f0d5b0823c7d66` — PASS, failures `0`, INSPECT `0`,
  `WORKSHOP-HOMING-DETECTION PASS 0`, test inventory drift PASS.
- `bash scripts/ci/anchor_check.sh --check` — PASS.
- `bash scripts/ci/gen_orientation.sh --check` — PASS.
- `bash scripts/ci/doc_budget_check.sh --check` — PASS.

## Scope and receipts

No kernel/WGSL/AccumulatorOp primitive, grammar, planner, scenario API, Studio/12.9 work, TP-specific
sealed-crate code, RF-1 semantic change, or RR-3/RR-4 production import was added. The pre-existing
untracked generated scenario output was left untouched.

- `HD-RECEIPT 11035f9c5602`
- `ORIENT-RECEIPT 2c9fde39d1d6`
- Anchor ACKs: field-policy `ae2d4c2c0c7d`; founding `46802793fba7`; one-tree
  `c88002b72898`; orientation harness `8a365d1c0864`; peripheral scope `7c85dbf18e48`;
  property-value/RF overlays `084ee935326b`; scanner selftest `34fb2662baae`; session lifecycle
  `d73fe5a83f25`; STEAD `b4a112cd02e8`; structural convergence `17fa0732f44d`; workshop homing
  `3e584f0ad175`.
