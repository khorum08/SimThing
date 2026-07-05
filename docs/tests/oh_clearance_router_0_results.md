# OH-CLEARANCE-ROUTER-0 Results

## Status

**PROBATION / gate-wiring — not self-mergeable.** Rung 0 + 0R remediation; DA clearance required before graduation.

## PR / branch / merge

| Field | Value |
|---|---|
| PR | [#1162](https://github.com/khorum08/SimThing/pull/1162) |
| Branch | `oh-clearance-router-0` |
| Base | `master` @ `6946d8adfe` |
| Rung | `OH-CLEARANCE-ROUTER-0` + `OH-CLEARANCE-ROUTER-0R` |

## What changed

### Rung 0
- Added `scripts/ci/clearance_check.sh` — emits exactly one `CLEARANCE-VERDICT:` line per invocation.
- Added rule TSVs: `precedented_classes.tsv`, `binding_conditions.tsv`, `clearance_ledger.tsv`.
- Added nine committed selftest fixtures under `scripts/ci/fixtures/clearance/`.
- Wired `/clearance` on existing `doctrine-exec-commands.yml` carrier.

### Rung 0R (remediation)
- Ledgered all 33 clearance fixture files in `scripts/ci/test_inventory.tsv` (`scripts-ci` / `seal-proof` / `birth_track=0.0.8.4.7-orchestration-harness`).
- Fixes GHA Doctrine Scan failure at inventory drift gate (unledgered `scripts/ci/fixtures/clearance/**` files).

## Load-bearing proofs

| Proof | Command / fixture | Catches |
|---|---|---|
| Router selftest battery | `bash scripts/ci/clearance_check.sh --selftest` | All nine regression classes |
| Inventory drift (0R) | `doctrine_selftest.sh` → `inventory drift proof: PASS` | Unledgered clearance fixtures cannot break Doctrine self-test |

### Remediation proof output (owner-local, 2026-07-05)

**clearance_check.sh --selftest**
```
PASS clearance_selftest_clearable_1150_shape
PASS clearance_selftest_clearable_1151_shape
PASS clearance_selftest_clearable_1152_shape
PASS clearance_selftest_reserve_1154_binding_conditions
PASS clearance_selftest_fail_closed_malformed_tsv
PASS clearance_selftest_fail_closed_ambiguous_class
PASS clearance_selftest_gate_wiring_self_application
PASS clearance_selftest_suspended_class
PASS clearance_selftest_missing_required_proof_fields
CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE
CLEARANCE-SELFTEST: PASS (9 fixtures)
```

**doctrine_selftest.sh**
```
positive control: PASS
inventory drift proof: PASS
DOCTRINE-SELFTEST-VERDICT: PASS
```

**gen_digest.sh --check**
```
gen_digest --check: PASS
```

**doctrine_scan.sh**
```
TEST-INVENTORY-DRIFT  PASS  0
DOCTRINE-SCAN-VERDICT: INSPECT  failures=0 inspect=415 selftest=SKIPPED
```

## Scope Ledger

| Path class | Touched (0R) | Notes |
|---|---|---|
| `scripts/ci/test_inventory.tsv` | yes | +33 fixture rows |
| `docs/tests/oh_clearance_router_0_results.md` | yes | evidence repair |
| Engine crates | **no** | |
| Drift gate logic | **no** | ledger-only fix |

## Known gaps / next

- `OH-RELAY-LINT-0` (M3) not started.
- Pre-existing `SPEC-LOWERER-KIND-READ` INSPECT(415) unchanged — not introduced by this PR.
- GHA post-0R results recorded below once settled.

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | PASS-RELIABLE — `failures=0`; pre-existing HEURISTIC INSPECT(415) only |
| Triage entries | none |
| Risk class | gate-wiring |
| Falsification check | `bash scripts/ci/doctrine_selftest.sh` → `inventory drift proof: PASS`; remove one ledger row → drift gate FAIL |
| Recommended posture | **deep** — gate-wiring audit unchanged; 0R confirms inventory discipline held |

### GHA (post-0R push)

_(updated after push)_