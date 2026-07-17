# RF-CONSERVATION-ORACLE-0 Results

## Status

**REMEDIATED — ready for orchestration/DA review** (2026-07-17).

Tested code commit: `2fd64ea32051a0b4341689e1399f5c928b5ec2e6`.

The remand is closed at the implementation/evidence boundary: live validation consumes GPU-read measured `Balance` deltas; missing or altered measurements fail as `ResidualNotIntegrated`; orphan status is derived from topology evidence; the TP Owner reduce-up golden is hydrated from the canonical ClauseScript pack; and the live GPU proof fails closed when no adapter is available.

## Remand closure

| Required correction | Implemented proof |
|---|---|
| Use actual measured Balance | The FlatStarOptIn proof reads root and leaf `Balance` before/after the GPU tick and passes those deltas to the oracle as `Option<f32>`. The helper never reconstructs a missing value from arithmetic residual. |
| Derive orphan status internally | `ArenaStructuralEvidence` carries intrinsic-source, inbound-coupling-endpoint, and parent-disbursement-recipient IDs. The oracle derives the lineage union and reports unmatched participant IDs. No caller orphan/lineage verdict remains. |
| Bind TP golden to canonical authored pack | Workshop code parses and hydrates `scenarios/terran_pirate_galaxy.clause`, identifies the Terran Owner and all ten descendant fleets, and derives selected marginal `40`, sibling contribution `360`, and exact Owner aggregate `400`. |
| Fail closed without GPU | Adapter/device acquisition returns a test failure; `SIMTHING_RF_FORCE_NO_ADAPTER=1` proves the former vacuous green path is absent. |

## Positive verification

- `cargo test -p simthing-driver --test rf_conservation_oracle_0 -- --nocapture` — **PASS**, 2 passed; live GPU measurement reported budget `10`, disbursed `10`, root Balance delta `0`, leaf Balance deltas `[0, 0]`.
- `cargo test -p simthing-driver rf_conservation_oracle::tests --lib -- --nocapture` — **PASS**, 3 passed.
- `cargo test -p simthing-workshop --test tp_rf_reduce_up_golden_0 -- --nocapture` — **PASS**, 2 passed; canonical golden reported Owner `terran`, selected `40`, siblings `360`, aggregate `400`, participants `10`.
- `cargo check -p simthing-spec -p simthing-driver -p simthing-workshop` — **PASS** (pre-existing warnings only).
- `bash scripts/ci/gen_orientation.sh --check` — **PASS**.
- `bash scripts/ci/doc_budget_check.sh --check` — **PASS**.
- `bash scripts/ci/doctrine_pr_scan.sh 7709eb867b4de01f9704a8d1432b421ab27c3e2a 2fd64ea32051a0b4341689e1399f5c928b5ec2e6` — **PASS**, failures `0`, INSPECT `0`; test inventory drift **PASS**.
- `bash scripts/ci/agent_scan.sh` — **PASS**, delta INSPECT `0`.

## Falsification evidence

- `SIMTHING_RF_BALANCE_DRIFT=1 ... oracle_agrees_with_flat_star_opt_in_executed_rf` — **EXPECTED FAIL** (`101`): `ResidualNotIntegrated { arithmetic_residual: 0.0, reported_balance_residual: Some(1.0) }`.
- `SIMTHING_RF_ORPHAN_DRIFT=1 ... oracle_agrees_with_flat_star_opt_in_executed_rf` — **EXPECTED FAIL** (`101`): `OrphanParticipants { orphan_ids: [7] }`.
- `SIMTHING_RF_FORCE_NO_ADAPTER=1 ... oracle_agrees_with_flat_star_opt_in_executed_rf` — **EXPECTED FAIL** (`101`): `simulated NoAdapter: RF conservation live proof fails closed`.
- `SIMTHING_RF_GOLDEN_DRIFT=1 ... canonical_tp_reduce_up_golden_is_analytically_derived` — **EXPECTED FAIL** (`101`): authored upkeep drift changed selected/siblings/aggregate from `40/360/400` to `60/540/600` before the golden assertion failed.

## Scope and residue

- No `resource_flow_execution_profile` change or default flip.
- No recursive RF source import or call.
- No new accumulator primitive.
- Terran/Pirate authored-pack interpretation remains in `simthing-workshop`.
- No test-budget INSPECT or triage row was required.
- Known pre-existing untracked generated scenario output was left untouched and excluded from both commits.

## Orientation receipt

- `HD-RECEIPT 9772abd8fcac`
- `ORIENT-RECEIPT 46d89a04fc85`
- Doctrine ACKs: field-policy `ae2d4c2c0c7d`; founding `b960ed2d493d`; one-tree `c88002b72898`; property-value `084ee935326b`; stead `b4a112cd02e8`; structural `17fa0732f44d`; workshop `3e584f0ad175`.
