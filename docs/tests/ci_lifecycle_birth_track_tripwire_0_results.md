# CI-LIFECYCLE-BIRTH-TRACK-TRIPWIRE-0 Results

## Status

**DONE / DA-OWNER REVIEW**

Track-A / ledger-layer text analysis only. No §3B executable proof, no workflow, no cargo test, no auto-deletion, no semantic note-truth scan.

## Identity

| Field | Value |
|---|---|
| PR | #1131 |
| Branch | `ci-lifecycle-birth-track-tripwire-0` |
| Head SHA | `324b236ee0e076c3e504c54f4c240924c0f142fe` |
| Base SHA / base ref | `fc5dc16e1672b6e938ecc62620abf127b6477927` / `origin/master` |

## Files changed

- `scripts/ci/test_inventory.tsv`
- `scripts/ci/test_inventory_check.sh`
- `scripts/ci/test_inventory_drift_check.sh` (mechanical header companion: `birth_track` column)
- `scripts/ci/test_lifecycle_expiry_check.sh` (new)
- `scripts/ci/test_lifecycle_tracks.tsv` (new)
- `docs/design_0_0_8_4_6_ci_scaffolding.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/ci_lifecycle_birth_track_tripwire_0_results.md` (this file)

## Implemented

- **birth_track column:** added as tenth column on all 731 inventory rows.
- **backfill rule used:** `scripts-ci` → `0.0.8.4.6-ci-scaffolding`; `tp_*` / terran-pirate paths → `0.0.8.5-terran-pirate`; all other rows → `pre-lifecycle` (644 + 54 + 33).
- **lifecycle tracks table:** `scripts/ci/test_lifecycle_tracks.tsv` with `pre-lifecycle` (closed), `0.0.8.4.6-ci-scaffolding` (open), `0.0.8.5-terran-pirate` (open).
- **expiry checker:** `scripts/ci/test_lifecycle_expiry_check.sh` modes `--schema`, `--track-closeout <track_id>`, `--scheduled`, `--prove`; footer `LIFECYCLE-EXPIRY-VERDICT: PASS|INSPECT|FAIL expired=N mode=...`; durable classes + `compile_fail`/`trybuild` immune; `downstream-utility:` clause suppresses surfacing without judging truth.
- **inventory-check schema wiring:** `test_inventory_check.sh` invokes `test_lifecycle_expiry_check.sh --schema` only (no `--scheduled` on every PR).

## Proof

### bash -n scripts/ci/test_lifecycle_expiry_check.sh

```
(exit 0, no output)
```

### test_lifecycle_expiry_check.sh --schema

```
LIFECYCLE-EXPIRY SCHEMA CHECK
  inventory rows: 731
  lifecycle tracks: 3
LIFECYCLE-EXPIRY-VERDICT: PASS expired=0 mode=schema
```

### test_lifecycle_expiry_check.sh --prove

```
LIFECYCLE-EXPIRY PROVE REPORT
  all synthetic prove cases passed
LIFECYCLE-EXPIRY-VERDICT: PASS expired=0 mode=prove
```

### test_inventory_check.sh

```
TEST-INVENTORY-CHECK-VERDICT: INSPECT
```
(pre-existing 2 extra dependency-floor fixture rows; lifecycle expiry schema PASS)

### test_inventory_drift_check.sh

```
TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS
```

### test_lifecycle_boundary_check.sh

```
TEST-LIFECYCLE-BOUNDARY-CHECK-VERDICT: PASS
```

### doctrine_scan.sh

```
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
```

### gen_digest.sh --check

```
gen_digest --check: PASS
```

### git diff --check origin/master...HEAD

```
(exit 0, no whitespace errors)
```

## INSPECT behavior

- **synthetic expired non-durable candidate:** closed `pre-lifecycle` + `behavior-regression` without `downstream-utility:` → `INSPECT` (prove `--scheduled` and `--track-closeout`).
- **durable immunity:** `seal-proof` class on closed track → `PASS`.
- **downstream-utility immunity:** non-durable with `downstream-utility:` note on closed track → `PASS`.
- **open-track immunity:** non-durable on `open-track` → `PASS`.
- **unknown birth_track:** schema → `FAIL`.
- **empty birth_track:** schema → `FAIL`.

## Scope ledger

| Item | Touched? |
|---|---|
| product code | no |
| workflows | no |
| doctrine_exec_profiles.tsv | no |
| doctrine_tests | no |
| scans/allowlists | no |
| cargo/workspace test run | no |
| auto-deletion implemented | no |
| semantic note-truth scan implemented | no |

## Graduation routing

- **DONE / DA-OWNER REVIEW**
- gate-state + data-deliverable
- DA deep review required
- not self-mergeable