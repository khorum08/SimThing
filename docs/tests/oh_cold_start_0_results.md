# OH-COLD-START-0 Results

## Status

**PROBATION / proof-present / DA-review-pending** — orientation receipts + cold-agent admission; DA clearance required (gate-wiring).

## PR / branch / merge

| Field | Value |
|---|---|
| PR | pending |
| Branch | `oh-cold-start-0` |
| Base | `master` @ `eee9d4714` (#1165 merge) |
| Rung | `OH-COLD-START-0` |

## What changed

- Added `scripts/ci/orient.sh` — role-keyed orientation landing (`coding|orchestrator|da`) emitting `ORIENT-RECEIPT`, `orientation_digest_sha`, `source_stamp`; `--selftest`.
- Extended `scripts/ci/relay_lint.sh` — receipt validation (`missing-orient-receipt`, `stale-orient-receipt`, `wrong-orient-role`); five cold-start fixtures in selftest battery (11 total).
- Rewired `scripts/ci/doctrine_exec_orient.sh` — `/orient` sticky output carries `ORIENT-REPORT: OK` + receipt fields + head/base metadata.
- Extended `scripts/ci/gen_orientation.sh` — receipt schema, role meanings, freshness rules, `orient.sh` usage in inner loop; regenerated `docs/orchestrator_orientation.md`.
- Added `docs/handoff_template.md` §10b receipt slot (enforced via relay-lint when `required_receipt_role.txt` present).
- Ledgered 17 cold-start fixture files in `test_inventory.tsv`.
- Flipped #1165 exit-proof rows to DA-GRADUATED @ `eee9d4714` (gate 0).

## Load-bearing proofs

| Proof | Command / fixture | Catches |
|---|---|---|
| Coding receipt | `bash scripts/ci/orient.sh --role=coding` | Role-keyed receipt emission |
| Orchestrator receipt | `bash scripts/ci/orient.sh --role=orchestrator` | Full-orientation receipt |
| DA receipt | `bash scripts/ci/orient.sh --role=da` | DA-slice receipt |
| Orient selftest | `bash scripts/ci/orient.sh --selftest` | Receipt stability regression |
| Freshness gate | `bash scripts/ci/gen_orientation.sh --check` | Hand-edit of orientation digest |
| Relay selftest | `bash scripts/ci/relay_lint.sh --selftest` | All 11 fixtures incl. cold-start |
| Valid coding receipt | `cold_start_selftest_valid_coding_receipt` | Fresh receipt accepted |
| Valid orchestrator receipt | `cold_start_selftest_valid_orchestrator_receipt` | Orchestrator receipt accepted |
| Missing receipt | `cold_start_selftest_fail_missing_receipt` | `FAIL(missing-orient-receipt)` |
| Stale receipt | `cold_start_selftest_fail_stale_receipt` | `FAIL(stale-orient-receipt)` |
| Wrong role | `cold_start_selftest_fail_wrong_role` | `FAIL(wrong-orient-role)` |
| Doctrine selftest | `bash scripts/ci/doctrine_selftest.sh` | Inventory drift + harness regression |
| Doctrine scan | `bash scripts/ci/doctrine_scan.sh` | TEST-INVENTORY-DRIFT + spine |

### Owner-local proof output

**orient.sh (all roles)**
```
ORIENT-RECEIPT: c32eb2b8313c  role: coding
ORIENT-RECEIPT: 8e90ecb077f1  role: orchestrator
ORIENT-RECEIPT: 5e526a904225  role: da
orientation_digest_sha: 0a992c63c51c79012c3807d82c35cde17f13e29c56f2050028bf105788060ba5
source_stamp: d294e4224fa9121e
```

**gen_orientation.sh --check**
```
gen_orientation --check: PASS
```

**relay_lint.sh --selftest**
```
RELAY-LINT-SELFTEST: PASS (11 fixtures)
```

**doctrine_selftest.sh**
```
inventory drift proof: PASS
DOCTRINE-SELFTEST-VERDICT: PASS
```

**doctrine_scan.sh**
```
TEST-INVENTORY-DRIFT  PASS  0
DOCTRINE-SCAN-VERDICT: INSPECT  failures=0 inspect=415
```

### Targeted mutations (falsification)

| Mutation | Expected | Observed |
|---|---|---|
| Stale digest hash in receipt fixture | `stale-orient-receipt` | PASS fixture |
| Remove receipt from relay fixture | `missing-orient-receipt` | PASS fixture |
| coding required, da receipt present | `wrong-orient-role` | PASS fixture |
| Hand-edit `orchestrator_orientation.md` | `gen_orientation --check` FAIL | verified locally |

## Post-merge /orient smoke

- PR used: pending
- comment/run: pending
- result: pending
- observed ORIENT-REPORT: pending
- role: orchestrator

## Scope Ledger

| Path | Classification | Notes |
|---|---|---|
| `scripts/ci/orient.sh` | gate-wiring harness | M2b receipt emitter |
| `scripts/ci/doctrine_exec_orient.sh` | gate-wiring harness | `/orient` GHA carrier |
| `scripts/ci/relay_lint.sh` | gate-wiring harness | receipt validation |
| `scripts/ci/gen_orientation.sh` | gate-wiring harness | receipt schema in digest |
| `scripts/ci/fixtures/cold_start/**` | seal-proof fixtures | 6 fixture dirs, 17 files |
| `scripts/ci/test_inventory.tsv` | inventory ledger | +17 rows |
| `docs/handoff_template.md` | harness template | §10b receipt slot |
| `docs/orchestrator_orientation.md` | generated digest | regen only |
| `clearance_check.sh` | deferred | router hook not clean/narrow; defer to future rung |
| Engine crates | untouched | no engine edits |

## Known gaps / next

- `clearance_check.sh` router integration deferred — no clean narrow hook for receipt admission on harness-class relays; document future hook at `DA-RESERVE(harness-error)` boundary.
- Merge-hold active: DA/Owner clearance required before merge (gate-wiring).
- ANCHOR-ACK / `/anchor` / `--since` delta receipt mode explicitly out of scope (rung 2c).

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | PASS-RELIABLE if Doctrine Scan and Doctrine Exec green on GHA |
| Triage entries | none unless new INSPECT delta appears |
| Risk class | gate-wiring |
| Falsification check | mutate receipt hash / remove receipt / wrong role / hand-edit orientation digest |
| Recommended posture | deep — receipt admission and command wiring |