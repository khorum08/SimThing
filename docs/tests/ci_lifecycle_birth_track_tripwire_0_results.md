# CI-LIFECYCLE-BIRTH-TRACK-TRIPWIRE-0 Results

## Status

**DONE — DA-APPROVED (2026-07-04, executive DA deep review, R3).** All three defects from the R2 hold are
verified fixed against the tree: (1) the 8 Necessity-Test survivor guards are immune (`survivor-set
expired: 0`); (2) `downstream-utility:` is structured + auditable (not a bare-string silent skip); (3) the
owner-mandated rising-cost DSU lease is present with DA-tunable tiers; R3's global closure-gate is proven by
the cross-track meta-proof. The 137 expired `cfg_test_mod::tests` module markers are expected pre-lifecycle
backlog (scheduled path only, never blocking a PR). Merge-cleared.

Track-A / ledger-layer text analysis only. No §3B executable proof, no workflow, no cargo test, no auto-deletion, no semantic note-truth scan.

### 0R repair

- `--prove` now asserts footer verdict and expired count, not just exit code.
- This closes the PASS-vs-INSPECT false-green lane.
- Meta-proof `meta-false-green-lane` verifies the harness rejects a PASS footer when INSPECT is expected.

### 0R2 repair

- Defect 1: live-anchor Necessity-Test survivors no longer flag; 8 invariants.md-named guards handled by durable anchor/reclass.
- Defect 2: downstream-utility immunity requires structured consumer justification and emits a DA audit surface.
- Defect 3: downstream-utility is now a rising-cost lease via dsu_survivals and configurable audit tiers; closure-gate mode surfaces renewals and delete-or-promote pressure.

### 0R3 repair

- Closure-gate renewal audit now surfaces all structured downstream-utility rows across the inventory, not only rows born under the closing track.
- Added cross-track prove case: closure-gate pre-lifecycle audits DSU rows from pre-lifecycle and open-track, sorted by dsu_survivals descending.
- Design DoD row updated to reflect dsu_survivals, audit=N, closure-gate mode, rising-cost lease, and delete-or-promote pressure.

## Identity

| Field | Value |
|---|---|
| PR | #1131 |
| Branch | `ci-lifecycle-birth-track-tripwire-0` |
| Proof | tested_code_sha binding per coverage_basis |
| Base SHA / base ref | `fc5dc16e1672b6e938ecc62620abf127b6477927` / `origin/master` |

## Files changed

- `scripts/ci/test_inventory.tsv` (`birth_track`, `dsu_survivals`)
- `scripts/ci/test_inventory_check.sh`
- `scripts/ci/test_inventory_drift_check.sh`
- `scripts/ci/test_lifecycle_expiry_check.sh`
- `scripts/ci/test_lifecycle_tracks.tsv`
- `scripts/ci/test_lifecycle_dsu_tiers.tsv` (0R2)
- `docs/design_0_0_8_4_6_ci_scaffolding.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/ci_lifecycle_birth_track_tripwire_0_results.md` (this file)

## Implemented

- **birth_track column:** added as tenth column on all 731 inventory rows.
- **backfill rule used:** `scripts-ci` → `0.0.8.4.6-ci-scaffolding`; `tp_*` / terran-pirate paths → `0.0.8.5-terran-pirate`; all other rows → `pre-lifecycle` (644 + 54 + 33).
- **lifecycle tracks table:** `scripts/ci/test_lifecycle_tracks.tsv` with `pre-lifecycle` (closed), `0.0.8.4.6-ci-scaffolding` (open), `0.0.8.5-terran-pirate` (open).
- **expiry checker:** modes `--schema`, `--track-closeout`, `--scheduled`, `--closure-gate`, `--prove`; footer `LIFECYCLE-EXPIRY-VERDICT: PASS|INSPECT|FAIL expired=N audit=N max_dsu_survivals=N mode=...`; live-anchor durability (8 guards); structured `downstream-utility: <consumer>` only; `dsu_survivals` lease tiers; promotion-not-perpetual-renewal language at closure-gate.
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
  dsu tiers: 3
LIFECYCLE-EXPIRY-VERDICT: PASS expired=0 audit=0 max_dsu_survivals=0 mode=schema
```

### test_lifecycle_expiry_check.sh --scheduled

```
survivor-set expired: 0
justified-closed (audit): 0
LIFECYCLE-EXPIRY-VERDICT: INSPECT expired=137 audit=0 max_dsu_survivals=0 mode=scheduled
```
(137 pre-lifecycle AUDIT rows; zero of 592 survivor set)

### test_lifecycle_expiry_check.sh --track-closeout pre-lifecycle

```
survivor-set expired: 0
LIFECYCLE-EXPIRY-VERDICT: INSPECT expired=137 audit=0 max_dsu_survivals=0 mode=track-closeout
```

### test_lifecycle_expiry_check.sh --closure-gate pre-lifecycle

```
downstream-utility renewal audit: 0
LIFECYCLE-EXPIRY-VERDICT: PASS expired=0 audit=0 max_dsu_survivals=0 mode=closure-gate
```

### test_lifecycle_expiry_check.sh --prove

```
LIFECYCLE-EXPIRY PROVE REPORT
  8 live-anchor guard keys: (listed in output)
  all synthetic and live prove cases passed
LIFECYCLE-EXPIRY-VERDICT: PASS expired=0 audit=0 max_dsu_survivals=0 mode=prove
```

(0R/0R2: strict footer verdict, expired, audit, max_dsu_survivals, mode, and exit code asserted per case.)

Proof environment: Git Bash on Windows; `PYTHON_BIN=C:\Users\mvorm\AppData\Local\Programs\Python\Python313\python.exe` set explicitly because Python is not on default Git Bash PATH.

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

## 8 live-anchor guard rows (Defect 1)

| crate | file | test_name |
|---|---|---|
| simthing-driver | child_share_eml.rs | child_share_cpu_zero_weight_is_zero_not_nan |
| simthing-driver | phase_m_c0_m4_atlas_protocol_oracle.rs | c0_mapping_profile_default_remains_disabled |
| simthing-sim | property_expiry.rs | cpu_decay_keeps_registry_live_when_sibling_still_has_property |
| simthing-sim | c8b_intensity_eml_parity.rs | c8b_intensity_runs_after_velocity_before_overlay |
| simthing-sim | c8c_transfer_accumulator_parity.rs | c8c_conjunctive_transfer_min_across_inputs |
| simthing-sim | c8d_emission_accumulator_parity.rs | c8d_mismatched_registration_tree_id_rejected |
| simthing-sim | protected_representative_restore.rs | assert_no_hard_trigger_on_soft_aggregate |
| simthing-sim | protected_representative_restore.rs | clone_capability_children |

Handled via live-anchor durability rule (`invariants.md` / `stead_spatial_contract` / cited escaped-bug anchor); inventory class unchanged to preserve boundary-row sync.

## INSPECT behavior (strict prove assertions)

- **8 live-anchor guards:** not flagged (`survivor-set expired: 0` on live `--scheduled`).
- **generic non-anchor behavior-regression:** still flaggable (prove `expired-non-durable`).
- **bare downstream-utility:** flagged (prove `bare-dsu-flags`).
- **structured downstream-utility:** immunity + `audit=1` (prove `structured-dsu-audited`).
- **closure cycles 1–5:** advisory → rejustify → presumed-stale delete-or-promote (prove `closure-cycle-*`).

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

- **PROBATION / DA-OWNER REVIEW — R3 complete**
- gate-state + data-deliverable + lifecycle authority
- DA deep review required
- not self-mergeable