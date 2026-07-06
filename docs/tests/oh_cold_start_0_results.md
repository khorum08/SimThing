# OH-COLD-START-0 Results

## Status

**DA-GRADUATED** — merged #1166 @ `d5c76215e0a80ac54c4d1e79d8d4165a39f1e94c`.

## PR / branch / merge

| Field | Value |
|---|---|
| PR | [#1166](https://github.com/khorum08/SimThing/pull/1166) |
| Branch | `oh-cold-start-0` |
| Merge | `d5c76215e0a80ac54c4d1e79d8d4165a39f1e94c` |
| Base | `master` @ `eee9d4714` (#1165 merge) |
| Rung | `OH-COLD-START-0` |

## Closure

OH-COLD-START-0 DA-GRADUATED / merged #1166 @ `d5c76215e0a80ac54c4d1e79d8d4165a39f1e94c`.
Orientation receipts live; relay-lint validates missing/stale/wrong-role receipts; router hook deferred as named future hook.

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
ORIENT-RECEIPT: 831cb2d35fa7  role: coding
ORIENT-RECEIPT: 66f5bb9def50  role: orchestrator
ORIENT-RECEIPT: b99a1ee80f51  role: da
orientation_digest_sha: b139390d9bc7ad62d9ad8de286ffd8048e3afd47b097f97e16ced3d98914c52c
source_stamp: b7e084213e8382c0
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

- PR used: [#1166](https://github.com/khorum08/SimThing/pull/1166)
- comment/run: `/orient role=orchestrator` → workflow [28760980506](https://github.com/khorum08/SimThing/actions/runs/28760980506)
- result: **PASS** — sticky comment posted with receipt-bearing output
- observed ORIENT-REPORT: `ORIENT-REPORT: OK`; `ORIENT-RECEIPT: 8e90ecb077f1`; `role: orchestrator`; `orientation_digest_sha: 0a992c63…`; `head_sha: 1ef409da`; `base_sha: eee9d4714`
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

### GHA CI

| Check | Result | Run |
|---|---|---|
| Doctrine Exec | **PASS** | [28761158306](https://github.com/khorum08/SimThing/actions/runs/28761158306) |
| Doctrine Scan | **PASS** | [28761158315](https://github.com/khorum08/SimThing/actions/runs/28761158315) |
| Orientation digest freshness | **PASS** | `gen_orientation.sh --check` |
| Doctrine self-test | **PASS** | `doctrine_selftest.sh` |
| PR delta scan | **PASS** | doctrine-scan workflow |
| Triage spam check | **PASS** | doctrine-scan workflow |

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | PASS-RELIABLE — Doctrine Exec + Doctrine Scan green on GHA |
| Triage entries | none unless new INSPECT delta appears |
| Risk class | gate-wiring |
| Falsification check | mutate receipt hash / remove receipt / wrong role / hand-edit orientation digest |
| Recommended posture | deep — receipt admission and command wiring |