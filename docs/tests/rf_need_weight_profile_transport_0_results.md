# RF-NEED-WEIGHT-PROFILE-TRANSPORT-0 (RF-5) Results

## Status

**DA Fork A corpus-red correction green locally / Owner OVL remains valid / draft-open hold** —
2026-07-19.

This tests/evidence-only correction does not change the accepted implementation, canonical scenario,
or frozen executable. No rebuild, re-freeze, or new Owner screenshot is required. PR #1414 remains
draft/open/unmerged pending fresh exact-head governance and DA adjudication.

## Identity

| Field | Value |
|---|---|
| Rung | `RF-NEED-WEIGHT-PROFILE-TRANSPORT-0` (RF-5) |
| PR / branch | #1414 / `coder/rf-need-weight-profile-transport-0` |
| RF-5A graduated merge / master | `3ad5add387e17bd2db22565cc102287b11e96484` |
| RF-5A implementation / evidence | `d53d5253aad066e0fe8b187e1b2dc21ed7279e7c` / `79e5ed4c666522b5fb97c50c6b381e3b89ea78e4` |
| RF-5A integration commit | `5be1012a7eec60d21886915b76b3810ef306603e` |
| RF-5 original implementation / evidence | `9ef5ba0cb56b77ab5acf970e7925ceab85b38a51` / `4a962f6987ea272e0546441b68bde854f59568f3` |
| remand-corrected tested implementation | `71005e59af05dc12105d63b585159b09ac0a8f6a` |
| corrected evidence commit | `afeaa4f15a22a48738caf97abba5a2a540837100` |
| DA Fork A test/evidence commit | `fd6d166276aeffded4e6b4418461878e511dafa5` |
| frozen source head | `85056ee19aab744507b4dfc2b8a4a2f641b718be` |
| frozen executable SHA-256 | `81d346791553be551323fb38e82945c91ece7fedb6b019e57d4047ea361d5a71` |
| Owner screenshot C closure | comment `5017017028` — remains valid under DA Fork A |
| ORIENT-RECEIPT | `2c9fde39d1d6` (role=coding) |

## DA Fork A corpus-red correction

- Orchestrator remand `5017072249` adjudicates the RF-5 single-pair `expansion-need`
  profile as intentional canonical truth: one admitted semantic `need_binding`, one input source,
  and one weight source.
- RF-5 intentionally amended the 12.8 canonical expansion profile from exact columns
  `[0, 1] / [10, 11] / 12` to `[0] / [10] / 12`, matching the admitted one-pair semantic
  `need_binding` and the accepted live oracle `101 × 0.020000 = 2.020000`.
- The 12.8 workshop assertion now requires exactly `input_cols == [0]`,
  `weight_cols == [10]`, and `output_col == Some(12)`. It remains an exact equality proof;
  no length, subset, `contains`, or shape-only weakening was introduced.
- Manufacturing and disruption profile assertions are unchanged. No production Rust, grammar,
  scenario, Studio, GPU, threshold, adapter-policy, planning, or doctrine surface changed.

Local corpus proof:

```text
cargo test -p simthing-workshop --test tp_clause_economy_author_0 -- --nocapture
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

cargo test -p simthing-workshop --test rf_need_binding_authoring_0 -- --nocapture
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## RF-5 implementation remand correction

- Removed `ensure_resource_economy_threshold_ops` and
  `materialize_authored_constant_emission_seeds` from the Studio bridge. The bridge no longer
  performs CPU full-buffer reads, dense Constant writes, previous-value manufacture, or either
  boundary install call.
- The admitted canonical input remains live through the generic property install. The authored
  emission-backed weight is zero after open and becomes live only through ordinary GPU execution;
  the derived need cell is likewise zero at open and is written by the admitted RF-5A stages.
- A source guard fails if either dense boundary-install call returns to the Studio bridge, while
  the runtime proof fails if the open-time weight/need values cease to be zero.
- Sealed threshold readback now returns `Result`. Missing runtime and readback errors put the bridge
  in `Errored` and return `ThresholdReadbackFailed`; an empty event list means `no-event` only after
  successful readback. The error-mapping unit proof asserts that failure cannot populate a
  successful no-event result.
- Canonical high/low variants still differ only in the authored weight scalar (`0.02` versus
  `0.005`). Profile, topology, input, participant, threshold, and event kind remain identical.
- Missing profile joins and misbound properties still fail closed. Scenario-specific proof code
  remains homed in `simthing-workshop`; the production bridge remains generic.

## Load-bearing local GPU proof

Command:

```text
cargo test -p simthing-workshop --test tp_field_session_elevate_0 -- --nocapture
```

Observed canonical pair:

```text
RF-5 LIVE scenario=terran_pirate_galaxy open_input=100 open_weight=0 open_need=0 tick=1 profile=Some("terran_expansion_need") weights=Some("terran=0.020000") need=Some(2.02) threshold=Some(1.0) result=Some("event") field_policy_events=1
RF-5 LIVE scenario=terran_pirate_galaxy open_input=100 open_weight=0 open_need=0 tick=1 profile=Some("terran_expansion_need") weights=Some("terran=0.005000") need=Some(0.505) threshold=Some(1.0) result=Some("no-event") field_policy_events=0
test canonical_tp_generic_need_binding_live_weight_controls_need_and_field_policy ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The open-time `weight=0` and `need=0` observations are actual GPU readbacks. A restored Studio
dense seed changes the high/low open weight and fails the proof. The event/no-event result is not
recomputed in Studio: it comes from successful sealed threshold-event readback after `step_once`.

## Readback failure and boundary-seed negatives

```text
cargo test -p simthing-mapeditor --lib unit_smoke -- --nocapture
test studio_live_session_bridge::unit_smoke::field_bridge_forbids_dense_boundary_seeding ... ok
test studio_live_session_bridge::unit_smoke::threshold_observation_error_is_not_no_event ... ok
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out
```

## Generic neutral path and fail-closed proof

```text
cargo test -p simthing-workshop --test rf_need_binding_authoring_0 -- --nocapture
test aqueduct_second_scenario_same_generic_path ... ok
test open_step_paired_need_exact_event_counts ... ok
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```text
cargo test -p simthing-workshop --test tp_field_session_elevate_0 canonical_tp_need_binding_removed_or_misbound_fails_closed -- --nocapture
test canonical_tp_need_binding_removed_or_misbound_fails_closed ... ok
```

## RF-4 exact non-regression

```text
RF4_LIVE loaded_owner_aggregate=0 live_owner_aggregate=15 disabled_aggregate=10 named_marginal=5 budget=27.1 sum_disbursed=27.100002 arithmetic_residual=-0.0000019073486 measured_balance_delta=-0.0000019073486 bound=0.000077533725
RF4_RUNTIME_NEGATIVE governed_balance=disconnected actual_gpu_balance_delta=0 violation=ResidualNotIntegrated { arithmetic_residual: -1.9073486e-6, reported_balance_residual: Some(0.0) }
test canonical_recursive_rf_bites_with_real_owner_aggregate_and_runtime_balance_negative ... ok
```

## Verification

```text
cargo test -p simthing-workshop --test tp_field_session_elevate_0 -- --nocapture
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

cargo test -p simthing-driver --test rf_conservation_oracle_0 -- --nocapture
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

cargo test -p simthing-driver --test owner_silo_gpu_tick -- --nocapture
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

cargo test -p simthing-spec --test owner_silo_disburse_down -- --nocapture
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

cargo test -p simthing-mapeditor --test studio_field_session_elevate_0 -- --nocapture
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

cargo build -p simthing-mapeditor --bin simthing-studio
Finished `dev` profile [optimized + debuginfo] target(s) in 0.60s

bash scripts/ci/doctrine_pr_scan.sh 3ad5add387e17bd2db22565cc102287b11e96484 fd6d166276aeffded4e6b4418461878e511dafa5
TEST-BUDGET  PASS  0
WORKSHOP-HOMING-DETECTION  PASS  0
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED

bash scripts/ci/gen_orientation.sh --check
gen_orientation --check: PASS

bash scripts/ci/doc_budget_check.sh --check
DOC-BUDGET-VERDICT: PASS

cargo check -p simthing-spec
Finished `dev` profile [optimized + debuginfo] target(s) in 0.23s

cargo check -p simthing-driver
Finished `dev` profile [optimized + debuginfo] target(s) in 0.28s

cargo check -p simthing-mapeditor
Finished `dev` profile [optimized + debuginfo] target(s) in 1.70s

cargo build -p simthing-mapeditor --bin simthing-studio
Finished `dev` profile [optimized + debuginfo] target(s) in 21.29s

cargo test -p simthing-mapeditor --test studio_field_session_elevate_0 -- --nocapture
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

cargo test -p simthing-mapeditor --lib studio_gpu_adapter_policy::tests::exact_adapter_policy_accepts_non_dx12_backends_and_rejects_identity_or_dx12 -- --nocapture
test studio_gpu_adapter_policy::tests::exact_adapter_policy_accepts_non_dx12_backends_and_rejects_identity_or_dx12 ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 37 filtered out

bash scripts/ci/agent_scan.sh
DOCTRINE SCAN REPORT  (commit 71005e59, 2026-07-19T15:45:46Z)
TEST-BUDGET  PASS  0
WORKSHOP-HOMING-DETECTION  PASS  0
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
AGENT-SCAN-VERDICT: PASS delta_inspect=0 elapsed=29s
```

GPU/desktop proofs above are owner-local by design; GitHub Actions has no GPU or desktop context.
The exact NVIDIA RTX 4080 Laptop GPU identity policy and owner-authorized non-DX12 backend support
are unchanged by RF-5.

## Fences held

- No new binding type, grammar family, kernel/WGSL/EvalEML/accumulator primitive, or staged projection.
- No execution-default flip, raw column mint, Studio arithmetic, feeder/mirror, direct dense seed,
  boundary install, reopen, registration edit, or proof-only production API.
- No first-DFS/first-owner or positional property mapping.
- No scenario-specific production code and no `§12.10` claim.
- No rebuild, re-freeze, new screenshot, ready-for-review transition, or merge.
