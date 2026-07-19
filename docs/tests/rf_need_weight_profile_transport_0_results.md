# RF-NEED-WEIGHT-PROFILE-TRANSPORT-0 (RF-5) Results

## Status

**IMPLEMENTED / local GPU proofs green / awaiting orchestrator acceptance** — 2026-07-19.

Dispatch `5016103467` resumed RF-5 after RF-5A graduation. Executable freeze, owner OVL,
and screenshot C remain blocked until orchestration accepts the implementation and proofs.

## Identity

| Field | Value |
|---|---|
| Rung | `RF-NEED-WEIGHT-PROFILE-TRANSPORT-0` (RF-5) |
| PR / branch | #1414 / `coder/rf-need-weight-profile-transport-0` |
| RF-5 pre-resume head | `39b4b39281aabcba7a7e96c3ee9eafe23f0e9bd7` |
| RF-5A graduated merge / master | `3ad5add387e17bd2db22565cc102287b11e96484` |
| RF-5A implementation / evidence | `d53d5253aad066e0fe8b187e1b2dc21ed7279e7c` / `79e5ed4c666522b5fb97c50c6b381e3b89ea78e4` |
| RF-5A integration commit | `5be1012a7eec60d21886915b76b3810ef306603e` |
| tested / implementation head | `9ef5ba0cb56b77ab5acf970e7925ceab85b38a51` |
| evidence commit | `4a962f6987ea272e0546441b68bde854f59568f3` |
| ORIENT-RECEIPT | `2c9fde39d1d6` (role=coding) |

## Implemented surface

- Canonical `scenarios/terran_pirate_galaxy.clause` now authors a complete generic
  `need_binding = terran_expansion_need`, joined by id to the existing
  `weight_profile` and explicitly naming participant, arena, semantic full-cell input,
  semantic full-cell weight, threshold, and event kind.
- Studio composition preserves `ResourceFlowSpec.need_bindings`; it no longer creates a
  companion binding, chooses a first owner, re-homes sources, or treats `AdmissionGap`
  as a successful runtime state.
- Studio telemetry is read-only: scenario/tick come from the live bridge, while profile
  id/kind, actual GPU weight value, actual GPU need value, threshold, and binding-specific
  sealed event count come from the admitted runtime/session readout.
- Canonical high/low variants differ only in the authored weight scalar (`0.02` versus
  `0.005`). The profile, topology, input, participant, threshold, and event kind are identical.
- Missing profile join fails during hydrate. A misbound semantic property fails during
  production session admission. Neither becomes an empty or neutral binding.
- Scenario-specific proof code is homed in `simthing-workshop`; production crates remain generic.

## Load-bearing local GPU proof

Command:

```text
cargo test -p simthing-workshop --test tp_field_session_elevate_0 canonical_tp_generic_need_binding_live_weight_controls_need_and_field_policy -- --nocapture
```

Observed output:

```text
RF-5 LIVE scenario=terran_pirate_galaxy tick=1 profile=Some("terran_expansion_need") weights=Some("terran=0.020000") need=Some(2.02) threshold=Some(1.0) result=Some("event") field_policy_events=1
RF-5 LIVE scenario=terran_pirate_galaxy tick=1 profile=Some("terran_expansion_need") weights=Some("terran=0.005000") need=Some(0.505) threshold=Some(1.0) result=Some("no-event") field_policy_events=0
test canonical_tp_generic_need_binding_live_weight_controls_need_and_field_policy ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out
```

The crossing result is not recomputed in Studio. `field_policy_events` counts the
binding's event kind in the sealed threshold-event readback from the completed GPU tick.

## Fail-closed and neutral proofs

```text
cargo test -p simthing-workshop --test tp_field_session_elevate_0 canonical_tp_need_binding_removed_or_misbound_fails_closed -- --nocapture
running 1 test
test canonical_tp_need_binding_removed_or_misbound_fails_closed ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out
```

RF-5A's scenario-agnostic suite, including the neutral AQUEDUCT scenario on the same
generic path, remains green:

```text
cargo test -p simthing-workshop --test rf_need_binding_authoring_0 -- --nocapture
running 24 tests
test aqueduct_second_scenario_same_generic_path ... ok
test open_step_paired_need_exact_event_counts ... ok
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## RF-4 exact non-regression

```text
RF4_LIVE loaded_owner_aggregate=0 live_owner_aggregate=15 disabled_aggregate=10 named_marginal=5 budget=27.1 sum_disbursed=27.100002 arithmetic_residual=-0.0000019073486 measured_balance_delta=-0.0000019073486 bound=0.000077533725
RF4_RUNTIME_NEGATIVE governed_balance=disconnected actual_gpu_balance_delta=0 violation=ResidualNotIntegrated { arithmetic_residual: -1.9073486e-6, reported_balance_residual: Some(0.0) }
test canonical_recursive_rf_bites_with_real_owner_aggregate_and_runtime_balance_negative ... ok
```

The exact NVIDIA RTX 4080 Laptop GPU adapter identity policy and the owner-authorized
non-DX12 backend requirement are unchanged by RF-5.

## Verification

```text
cargo check -p simthing-spec
Finished `dev` profile [optimized + debuginfo] target(s)

cargo check -p simthing-driver
Finished `dev` profile [optimized + debuginfo] target(s)

cargo check -p simthing-mapeditor
Finished `dev` profile [optimized + debuginfo] target(s)

cargo test -p simthing-mapeditor --test studio_field_session_elevate_0 -- --nocapture
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

bash scripts/ci/agent_scan.sh
TEST-BUDGET  PASS  0
WORKSHOP-HOMING-DETECTION  PASS  0
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
AGENT-SCAN-VERDICT: PASS delta_inspect=0
```

GPU/desktop proofs above are owner-local by design; GitHub Actions has no GPU or desktop context.

## Fences held

- No new binding type, grammar family, kernel/WGSL/EvalEML/accumulator primitive, or staged projection.
- No execution-default flip, raw column mint, Studio arithmetic, feeder/mirror, or proof-only production API.
- No first-DFS/first-owner or positional property mapping.
- No scenario-specific production code and no `§12.10` claim.
- No executable freeze, screenshot C, PR-body edit, clearance edit, PR creation, or merge.
