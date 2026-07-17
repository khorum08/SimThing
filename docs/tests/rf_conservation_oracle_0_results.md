# RF-CONSERVATION-ORACLE-0 Results

## Status

**REMAND 2 REMEDIATED — ready for orchestration/DA review** (2026-07-17).

Tested code commit: `675e1544ecb9746602339629426ec7b13feb10f8`.

The anti-cosplay gap is closed: the admitted FlatStarOptIn fixture now produces a deterministic non-zero f32 allocator residual, reads an actual non-zero governed root `Balance` delta from GPU state, and runs an identical GPU allocation with only the root governed integration omitted. That runtime-path negative reads an unchanged root `Balance` cell and fails as `ResidualNotIntegrated`. Remand-1 items 2–4 remain unchanged.

## Remand closure

| Required correction | Implemented proof |
|---|---|
| Use actual measured Balance | Seven equal child weights produce GPU disbursements summing to `10.000001` from budget `10`, so the arithmetic residual is non-zero (`-9.536743e-7`). The fixture reads root `Balance` before/after execution and observes non-zero delta `-1.1920929e-6`, within the declared `6.67572e-5` bound. |
| Falsify the runtime governed path | The paired run preserves budget, weights, bit-exact disbursements, participant topology, and residual rate, but omits the final root governed integration dispatch. Actual GPU root `Balance` delta is `0`; the oracle returns `ResidualNotIntegrated { arithmetic_residual: -9.536743e-7, reported_balance_residual: Some(0.0) }`. |
| Derive orphan status internally | `ArenaStructuralEvidence` carries intrinsic-source, inbound-coupling-endpoint, and parent-disbursement-recipient IDs. The oracle derives the lineage union and reports unmatched participant IDs. No caller orphan/lineage verdict remains. |
| Bind TP golden to canonical authored pack | Workshop code parses and hydrates `scenarios/terran_pirate_galaxy.clause`, identifies the Terran Owner and all ten descendant fleets, and derives selected marginal `40`, sibling contribution `360`, and exact Owner aggregate `400`. |
| Fail closed without GPU | Adapter/device acquisition returns a test failure; `SIMTHING_RF_FORCE_NO_ADAPTER=1` proves the former vacuous green path is absent. |

## Positive verification

- `cargo test -p simthing-driver --test rf_conservation_oracle_0 -- --nocapture` — **PASS**, 2 passed; live GPU output reported budget `10`, sum disbursed `10.000001`, arithmetic residual `-9.536743e-7`, bound `6.67572e-5`, residual rate `-1.1920929e-6`, and actual root Balance delta `-1.1920929e-6`.
- `cargo test -p simthing-driver rf_conservation_oracle::tests --lib -- --nocapture` — **PASS**, 3 passed.
- `cargo test -p simthing-workshop --test tp_rf_reduce_up_golden_0 -- --nocapture` — **PASS**, 2 passed; canonical golden reported Owner `terran`, selected `40`, siblings `360`, aggregate `400`, participants `10`.
- `cargo check -p simthing-spec` — **PASS** (pre-existing warnings only).
- `cargo check -p simthing-driver` — **PASS** (pre-existing warnings only).
- `bash scripts/ci/gen_orientation.sh --check` — **PASS**.
- `bash scripts/ci/doc_budget_check.sh --check` — **PASS**.
- `bash scripts/ci/doctrine_pr_scan.sh 7709eb867b4de01f9704a8d1432b421ab27c3e2a 675e1544ecb9746602339629426ec7b13feb10f8` — **PASS**, failures `0`, INSPECT `0`; `WORKSHOP-HOMING-DETECTION PASS 0`; test inventory drift **PASS**.
- `bash scripts/ci/agent_scan.sh` — **PASS**, delta INSPECT `0`.

## Falsification evidence

- Runtime-path negative inside `oracle_agrees_with_flat_star_opt_in_executed_rf` — **OBSERVED BITE**: with root governed integration omitted, actual GPU residual rate remained `-1.1920929e-6`, actual root Balance delta was `0`, and the oracle returned `ResidualNotIntegrated { arithmetic_residual: -9.536743e-7, reported_balance_residual: Some(0.0) }`.
- `SIMTHING_RF_BALANCE_DRIFT=1 ... oracle_agrees_with_flat_star_opt_in_executed_rf` — **EXPECTED FAIL** (`101`, secondary): `ResidualNotIntegrated { arithmetic_residual: -9.536743e-7, reported_balance_residual: Some(0.9999988) }`.
- `SIMTHING_RF_ORPHAN_DRIFT=1 ... oracle_agrees_with_flat_star_opt_in_executed_rf` — **EXPECTED FAIL** (`101`): `OrphanParticipants { orphan_ids: [7] }`.
- `SIMTHING_RF_FORCE_NO_ADAPTER=1 ... oracle_agrees_with_flat_star_opt_in_executed_rf` — **EXPECTED FAIL** (`101`): `simulated NoAdapter: RF conservation live proof fails closed`.
- `SIMTHING_RF_GOLDEN_DRIFT=1 ... canonical_tp_reduce_up_golden_is_analytically_derived` — **EXPECTED FAIL** (`101`): authored upkeep drift changed selected/siblings/aggregate from `40/360/400` to `60/540/600` before the golden assertion failed.

## Scope and residue

- No `resource_flow_execution_profile` change or default flip.
- No recursive RF source import or call.
- No new accumulator primitive.
- Terran/Pirate authored-pack interpretation remains in `simthing-workshop`.
- No test-budget INSPECT or triage row was required.
- Known pre-existing untracked generated scenario output was left untouched and excluded from the remediation commits.

## Orientation receipt

- `HD-RECEIPT 9772abd8fcac`
- `ORIENT-RECEIPT 46d89a04fc85`
- Doctrine ACKs: field-policy `ae2d4c2c0c7d`; founding `b960ed2d493d`; one-tree `c88002b72898`; property-value `084ee935326b`; stead `b4a112cd02e8`; structural `17fa0732f44d`; workshop `3e584f0ad175`.
