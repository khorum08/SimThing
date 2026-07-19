# RF-NEED-BINDING-AUTHORING-0 (RF-5A) Results

## Status

**PROBATION / remand-remediated / DA-review-pending** — 2026-07-19 (coder=Codex).

Remander `5014412811` is implemented on the existing draft PR #1416 branch. RF-5/#1414 remains **BLOCKED-PAUSED**. No merge, DA relay, OVL, Studio production change, or canonical TP change is claimed.

## Identity

| Field | Value |
|---|---|
| Rung | `RF-NEED-BINDING-AUTHORING-0` (RF-5A) |
| PR | #1416 (draft) |
| Base | `cecde41f7ed73287f309020eec6db8f57776feb9` |
| Pre-remand head | `4b5d7f0da3be63a0edf0916b332045d90e3dbd9f` |
| HD-RECEIPT | `5844fdbd66f4` |
| DA ruling | `5014178941` Modified Option A |
| Remander | `5014412811` |
| ORIENT-RECEIPT | `2c9fde39d1d6` |
| implementation_sha / tested_code_sha | `99586581272ec69a839034598c399bddddb7bc5c` |
| evidence_sha | self (reported in the relay; a Git commit cannot embed its own hash) |
| coverage_basis | RF-5A workshop 24/24 plus targeted RF/RF-4/OrderBand/adapter regressions |

## Remander remedies

| # | Finding | Remediation and falsifier |
|---|---|---|
| 1 | LIVE mutated registrations after open | LIVE uses an authored 0.25 transfer and authored install defaults before `SimSession::open`; no post-open registration mutation. |
| 2 | Host-qualified materialization could select first DFS owner | Transfer source/target, emission, and threshold host identities survive hydrate → compile → session materialization. Qualified rows resolve the named install target and its property cell directly. Conflicting explicit economy hosts fail closed. |
| 3 | Host failures lacked actual source spans | Location/owner host spans survive hydration. Missing, ambiguous, conflicting, and named-host-without-property tests assert the actual authored span token. |
| 4 | Same-participant bindings could collide in staging | Each participant receives a cursor-allocated, capacity-checked staged slice. The paired-binding proof asserts disjoint cells and independent GPU values (`3.0`, `1.0`). |
| 5 | Need staging could collide with gated-rate bands | Additive dependency map is producer `0` → stage `1` → need EvalEML `2` → arena `3`; the coexistence test asserts this order. |
| 6 | Three-control proof was relational | LIVE, DISCONNECT, and STATIC assert the exact t1/t2 closed forms below. |
| 7 | Disconnect switch was production-visible | Disconnect resync exists only behind the workshop-enabled `rf-test-harness` feature; the internal options path is crate-private. |
| 8 | Evidence chain was stale | Implementation and evidence SHAs are rebound by ordered commits after all gates pass. |

## Authoritative cell path

```text
authored source cell on named economy host
  -- ordinary transfer, producer band 0 --> current source value
  -- staged identity projection, band 1 --> participant-private stage cell
  -- need EvalEML, band 2 --> participant need cell
  -- arena execution begins at band 3
```

Property identity selects a column; the authored economy host selects the row. An unrelated overlay-held copy of the same property may coexist, but two explicit economy placements for the same property on different hosts are rejected.

## Exact three-control measurements

Budgeted authored transfer: `0.25` per tick. Initial source/stage/need: `3.00/3.00/3.00`.

| Control | t1 source/stage/need | t2 source/stage/need | Required bite |
|---|---|---|---|
| LIVE | `2.75/2.75/2.75` | `2.50/2.50/2.50` | Stage and need follow both ordinary GPU transfer steps. |
| DISCONNECT | `2.75/2.75/2.75` | `2.50/2.75/2.75` | Actual runtime resync omits only staged projections; source continues while stage/need freeze. |
| STATIC | `3.00/3.00/3.00` | `3.00/3.00/3.00` | Pre-open disabled economy holds every cell. |

Observed focused output:

```text
running 24 tests
DISCONNECT exact: t1 source/stage/need=2.75/2.75/2.75; t2=2.50/2.75/2.75
STATIC exact: t1 source/stage/need=3.00/3.00/3.00; t2=3.00/3.00/3.00
LIVE exact: t1 source/stage/need=2.75/2.75/2.75; t2=2.50/2.50/2.50
test result: ok. 24 passed; 0 failed; 0 ignored
```

## Targeted verification

| Command | Result |
|---|---|
| `cargo check -p simthing-spec` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo check -p simthing-workshop --tests` | PASS |
| `cargo test -p simthing-workshop --test rf_need_binding_authoring_0 -- --nocapture` | PASS, 24/24 with exact values above |
| `cargo test -p simthing-driver --lib gated_rate_and_need_binding_bands_are_dependency_ordered -- --nocapture` | PASS, 1/1 |
| `cargo test -p simthing-driver --lib governed_adapter_preserves_authored_targets_and_orderband -- --nocapture` | PASS, 1/1 |
| `cargo test -p simthing-kernel --lib governed_integration_executes_only_on_its_authored_orderband -- --nocapture` | PASS, 1/1 |
| `cargo test -p simthing-workshop --test rf_execute_recursive_default_0 -- --nocapture` | PASS, 1/1 |
| `cargo test -p simthing-driver --test rf_conservation_oracle_0 -- --nocapture` | PASS, 2/2 |
| `cargo test -p simthing-driver rf_conservation_oracle::tests --lib -- --nocapture` | PASS, 3/3 |
| `cargo test -p simthing-workshop --test tp_rf_reduce_up_golden_0 -- --nocapture` | PASS, 2/2 |
| `cargo test -p simthing-spec --test runtime_rf_tick -- --nocapture` | PASS, 1/1 |
| `cargo test -p simthing-spec --test runtime_tick_shell -- --nocapture` | PASS, 1/1 |
| `cargo test -p simthing-spec --test runtime_tick_history -- --nocapture` | PASS, 1/1 |
| `cargo test -p simthing-mapeditor --lib studio_gpu_adapter_policy::tests -- --nocapture` | PASS, 3/3; CPU policy proof accepts supported non-DX12 backends and blocks DX12 |
| `cargo test -p simthing-mapeditor --test studio_field_session_elevate_0 -- --nocapture` | PASS, 8/8 owner-local GPU proof; not a GHA claim |

## SHA-bound governance

`bash scripts/ci/doctrine_pr_scan.sh cecde41f7ed73287f309020eec6db8f57776feb9 99586581272ec69a839034598c399bddddb7bc5c`:

```text
TEST-BUDGET  INSPECT  1
WORKSHOP-HOMING-DETECTION  PASS  0
TEST-INVENTORY-DRIFT  PASS  0
DOC-BUDGET  PASS  0
hard failures: 0   inspect flags: 1
```

The single heuristic TEST-BUDGET inspection is discharged by additive `inspect_justifications.tsv` and `triage_log.tsv` rows bound to implementation SHA `99586581272ec69a839034598c399bddddb7bc5c`. The 24 named proofs cover distinct load-bearing remand rows: authored LIVE/DISCONNECT/STATIC execution, host/span admission failures, same-participant slice separation, cross-row projection, exact event counts, and authority/non-invention boundaries.

## Holds

Draft #1416 only · RF-5/#1414 blocked-paused · no merge · no DA relay · no OVL/Studio/TP scope expansion.
