# THRESHOLD-EVENT-REGRESSION-REPAIR-0 — Stage A2 results (INVARIANT SPLIT PENDING DA)

- Track: 0.0.8.7 RF arena modernization (rung 5.3c)
- Status: **COMPLETE — DA-GRADUATED / merged #1507 @ 462cc794** (DA ruling `5133576764`: invariant split APPROVED read-only with three binding guards; execution belongs to the TP-purge successor, which does NOT stop the 2026-08-11 clock)
- HD-RECEIPT (5.3c coding): `84786707d7c0`
- HD-RECEIPT (intervention transport): `36a4121e68f0`
- ORIENT-RECEIPT (coding, fresh): `56fe3e6032b0`
- orientation_rule_stamp: `1497628db25456ff`
- Exact master / reconcile base: `f64b27464e86630c1e89677e6887ade55f00f128`
- Prior Stage-A tip (superseded; not mechanical rebase): `6c52ae47b02b1f5043d422da26772ee9ad80d303`
- Stage-A2 tip / tested_code_sha: `3b889d8083c5da95fcc3056cc17487642445c5cf`
- Adapter pin: `WGPU_BACKEND=vulkan`, `SIMTHING_GPU_ADAPTER_CONTAINS=4080`, `SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1`

## Why Stage A was revised

#1517 made the Invariant Set the complete substrate proof surface and mechanized
Detachability. The prior 787-row blanket-REAP proposal is inadmissible: it had no
invariant classification and would have reaped `s6_threshold_events_match_cpu_golden`
(CPU/GPU parity). Corpus/fixture/generator input never validates substrate law.

## What was reconstructed (not mechanically rebased)

Master `f64b2746` remains authoritative for Invariant Set / Detachability /
lifecycle / `--rungclose` / orientation. Admitted 5.3c delta transplanted onto that gate:

1. Production: `prepare_threshold_scan` / `finish_threshold_scan` on
   `WorldGpuState::dispatch_accumulator_threshold_scan` (golden unedited).
2. Truthful rename → `canonical_tp_gpu_table_matches_admission_totality` + inventory
   lockstep; **`dsu_survivals=0`** (rename ≠ renewal; birth track unchanged).
3. Exact one-row `scripts/ci/authorized_renames.tsv` (`5126261563-DA`).
4. Deletion-guard ledger match + fail-closed selftests merged **with** master gate
   (note bypass stays REJECTED). Gate preservation: `track_closeout.sh` **+197 / −0**.

## Stage-A2 fences observed

- No reap/renew execution; no TP purge in #1507.
- Pickers classified only; not repaired/renewed.
- No `ci_green: PASS`, Remand discharge, final clearance, or full mapeditor exit claim.

## Detachability

```
DETACHABILITY-VERDICT: PASS production_coupling=0 proof_coupling=2 ceiling=2
DETACHABILITY-SELFTEST: PASS (4 fixtures)
```

Proof-coupling edges (must decrease toward 0 in TP-purge successor):
1. `simthing-driver` -> `simthing-clausething` [dev-dependencies]
2. `simthing-driver` -> `simthing-mapeditor` [dev-dependencies]

## Lifecycle invariant split (read-only; 2026-08-11 due set)

Artifact: `docs/tests/lifecycle_invariant_split_proposal_2026_08_11.tsv`
Generator: `scripts/ci/gen_lifecycle_reap_renew_proposal.py`

| metric | value |
|---|---|
| Due rows (closed-track, non-durable, due ≤ 2026-08-11) | **787** |
| PAIR-REAP | **296** |
| REPLACE-INLINE-INVARIANT | **222** |
| TP-PURGE-SUCCESSOR | **268** |
| RENEW-INVARIANT | **1** (`s6_threshold_events_match_cpu_golden`) |
| PROMOTION-EVAL | **0** |

### By invariant

| invariant | count |
|---|---|
| NONE | 357 |
| cpu-gpu-parity | 209 |
| determinism | 148 |
| residency-typing | 29 |
| admission-totality | 20 |
| boundedness | 14 |
| conservation | 10 |

### Named classifications (Remand 4)

| identity | invariant | input | disposition |
|---|---|---|---|
| `s6_threshold_events_match_cpu_golden` | cpu-gpu-parity | inline-constructed | **RENEW-INVARIANT** — consumer `WorldGpuState::dispatch_accumulator_threshold_scan`; planted defect = drop prepare/finish so GPU events=0 vs golden 1 |
| `canonical_tp_gpu_table_matches_admission_totality` | admission-totality | TP/corpus-coupled | **TP-PURGE-SUCCESSOR** (transition-only 5.3c rename; dsu=0; not executed here) |
| `picker_0_no_duplicate_parse_or_rebind_path` | NONE | TP/corpus-coupled | **TP-PURGE-SUCCESSOR** |
| `picker_0_no_gamemode_rf_live_run_closeout` | NONE | TP/corpus-coupled | **TP-PURGE-SUCCESSOR** |

## TP-purge successor census (separate; not conflated)

Artifact: `docs/tests/tp_purge_successor_census_2026_08_11.tsv`

| metric | value |
|---|---|
| Census rows (incl. 2 coupling edges) | **80** |
| Engine structural gates (YES) | **12** (includes 2 proof-coupling edges) |
| Projected detachability ceiling | **2 → 0** after successor removes both engine proof couplings |

## `--rungclose` (expected pre-graduation)

```
RUNGCLOSE-VERDICT: FAIL (4) rung=THRESHOLD-EVENT-REGRESSION-REPAIR-0
```

Expected only: missing DA-GRADUATED cells / merge stamp / still PROBATION / pointer still 5.3c.
No lifecycle-schema, orientation-freshness, binding-condition, or reap-clock defect.

## Stage-A2 matrix (adapter-pinned)

| command | result |
|---|---|
| s6 golden | **1/0** |
| write_door_band_delta_0 | **10/0** |
| anchor_table_surface_0 | **13/0** |
| rf_conservation_oracle_0 | **2/0** |
| arena RF-1+replay | **1/0** |
| canonical_anchor_materialization_0 | **4/0** |
| `--prove` | **PASS** |
| lifecycle `--scheduled` | **PASS** expired=0 |
| detachability + selftest | **PASS** |
| inventory drift / doc-budget / bypass / orient-check | **PASS** |
| `--rungclose` | **FAIL (4)** expected |

Full mapeditor zero-failure exit deferred until DA approves the revised split and successor boundary.
