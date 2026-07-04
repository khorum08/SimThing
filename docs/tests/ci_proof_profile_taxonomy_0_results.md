# CI-PROOF-PROFILE-TAXONOMY-0 Results

## Status

**PROBATION / DA-OWNER REVIEW**

This rung retires or reframes stale Track-D-era Doctrine Exec profiles after Track D closeout. It does not reopen Track D, delete tests, modify product code, edit workflows, or normalize full-workspace proof as routine.

Branch: `ci-proof-profile-taxonomy-0` (pending PR).

## Baseline

| Item | Value |
|---|---|
| #1126 merged | yes @ `86205e7d1c1027ef745a53d1f25de318d1dd8008` |
| Track D | **CLOSED** |
| Inventory | **731 rows** |
| Lifecycle doctrine | Scoped borrow assumed deleted at birth-track closure |

## Scope

- Retire stale `test-pare-*` / `test-deletion-*` executable profiles from `doctrine_exec_profiles.tsv`
- Add profile-lint guard forbidding Track-D deletion profiles in executable table
- Delete uninvoked `protected_class_audit.py`
- Update CI scaffolding docs

Out of scope: workflows, product code, test deletion, full-workspace cargo test.

## Profile taxonomy audit

| profile_id | profile_class | risk_class | current_command_count | references_deleted_or_unknown_tests | live_role_after_track_d | action_taken | post_rung_profile_id | proof_scope | notes |
|---|---|---|---:|---|---|---|---|---|
| ci-b-webchat-smoke | smoke | webchat-orchestration | 3 | no | live-routine-smoke | KEPT | ci-b-webchat-smoke | PR default smoke + profile lint + plan | Current non-owner-deep default |
| seal-residue | targeted | seal-residue | 2 | no | live-targeted-proof | KEPT | seal-residue | kernel/spec `--doc` seal proof | `gpu_required=yes`; INSPECT if GPU missing — owner/deep-safe targeted doc proof |
| data-deliverable | targeted | data-deliverable | 2 | no | live-targeted-proof | KEPT | data-deliverable | Named TP deliverable binaries | `tp_ownership_columns_0`, `tp_planet_surface_payload_0` exist on tree |
| timeout-finalize-proof | probe | timeout-finalize | 1 | no | live-profile-lint/proof-seal | KEPT | timeout-finalize-proof | Probe infrastructure | Expected FAIL probe |
| owner-deep-full-cpu-quarantined | owner-deep | general-owner-deep | 12 | no | live-owner-deep-quarantine | KEPT | owner-deep-full-cpu-quarantined | workflow_dispatch artillery only | Full-crate batteries quarantined |
| tests-compile-floor-non-bevy | targeted | gate-state/tests-compile-floor | 5 | no | live-targeted-proof | KEPT | tests-compile-floor-non-bevy | Narrow non-Bevy `cargo check --tests` floor | Not closure certificate; complements lifecycle gates |
| test-pare-clausething | targeted | test-deletion-clausething | 2 | yes | retired-track-d-residue | RETIRED | — | — | Track-D deletion wave; spent |
| test-pare-spec | targeted | test-deletion-spec | 70+ | yes | retired-track-d-residue | RETIRED | — | — | Pre-#1122 spec battery; many binaries deleted |
| test-pare-tier2-cpu-admission-collapse | targeted | test-deletion-tier2-admission-collapse | 7 | yes | retired-track-d-residue | RETIRED | — | — | Spent admission-collapse wave |
| test-pare-mapgenerator-admission-collapse | targeted | test-deletion-tier2-admission-collapse | 13 | partial | retired-track-d-residue | RETIRED | — | — | Spent mapgenerator wave |
| test-pare-cpu-safe-boundary-sweep | targeted | test-deletion-cpu-safe-boundary-sweep | 1 | unknown | retired-track-d-residue | RETIRED | — | — | Spent sweep profile |
| test-pare-broken-clausething-admission-residue | targeted | test-deletion-broken-clausething-admission-residue | 3 | partial | retired-track-d-residue | RETIRED | — | — | Spent residue wave |
| test-pare-src-unit-fossil-residue | targeted | test-deletion-src-unit-fossil-residue | 0 | n/a | retired-track-d-residue | RETIRED | — | — | Crate-check shell only; wave spent |
| test-pare-gpu-bevy-residue | targeted | test-deletion-gpu-bevy-residue | 4 | partial | retired-track-d-residue | RETIRED | — | — | Spent GPU-named wave |
| test-pare-protected-pare-delete | targeted | test-deletion-protected-pare | 0 | n/a | retired-track-d-residue | RETIRED | — | — | Crate-check shell; wave spent |
| test-pare-conservative-survivor-delete | targeted | test-deletion-conservative-survivor | 0 | n/a | retired-track-d-residue | RETIRED | — | — | Crate-check shell; wave spent |
| test-consolidate-classifier-families | targeted | test-deletion-classifier-consolidation | 1 | yes | retired-track-d-residue | RETIRED | — | — | Hygiene test deleted in #1122 |

## Profile actions

### Retired profiles (11)

`test-pare-clausething`, `test-pare-spec`, `test-pare-tier2-cpu-admission-collapse`, `test-pare-mapgenerator-admission-collapse`, `test-pare-cpu-safe-boundary-sweep`, `test-pare-broken-clausething-admission-residue`, `test-pare-src-unit-fossil-residue`, `test-pare-gpu-bevy-residue`, `test-pare-protected-pare-delete`, `test-pare-conservative-survivor-delete`, `test-consolidate-classifier-families`

### Retained profiles (6)

`ci-b-webchat-smoke`, `seal-residue`, `data-deliverable`, `timeout-finalize-proof`, `owner-deep-full-cpu-quarantined`, `tests-compile-floor-non-bevy`

### Renamed/reframed

None — retire-only rung.

### Owner-deep quarantine check

`owner-deep-full-cpu-quarantined` remains `profile_class=owner-deep`; default profile `ci-b-webchat-smoke` is smoke. GHA proof-seal prove still PASS.

### Closure-certificate check

`cargo test --workspace --all-targets` is **not** in `doctrine_exec_profiles.tsv`. Closure certificate remains exceptional one-time proof only.

## Protected-class audit cleanup

`scripts/ci/protected_class_audit.py` was deleted as uninvoked Track-D residue; its historical evidence remains in git history and prior docs/tests artifacts, not active CI.

| Item | Disposition |
|---|---|
| `protected_class_audit.py` | **DELETED** — no live script/workflow invocation |
| Generated TSVs in `docs/tests/` | **RETAINED** as historical evidence (not active CI) |
| Remaining references | Historical results docs only |

## Validation

| Gate | Result |
|---|---|
| `doctrine_scan.sh` | **PASS** (failures=0 inspect=0) |
| `gen_digest.sh --check` | **PASS** |
| `doctrine_exec_profile_lint.sh` | **PASS** (profiles=6) |
| `doctrine_exec_profile_lint.sh --prove-gha-proof-seal` | **PASS** |
| `doctrine_exec_profile_lint.sh --prove-no-track-d-deletion-profiles` | **PASS** |
| `doctrine_exec_plan.sh --profile ci-b-webchat-smoke` | **PASS** |
| `test_inventory_check.sh` | **INSPECT** (2 dependency-floor fixtures; expected) |
| `test_inventory_drift_check.sh` | **PASS** |
| `test_lifecycle_boundary_check.sh` | **PASS** |
| fossil rg (`scripts/ci`) | **PASS** — no executable `test-pare-*` profiles; lint guard references only |
| `git diff --check origin/master...HEAD` | **PASS** |
| `cargo test --workspace --all-targets` | **not run** (forbidden) |

## Scope Ledger

| Bucket | Items |
|---|---|
| specified | Profile audit; retire test-pare-*; lint guard; protected_class_audit cleanup; docs |
| implemented | All specified items |
| proxied | Live tree verification; binary existence check for data-deliverable |
| deferred | `CI-COMMAND-ERGONOMICS-0`; `CI-HANDOFF-LIFECYCLE-ENFORCEMENT-0` |

## Graduation routing

| Field | Value |
|---|---|
| Risk class | gate-state |
| CI verdict | PROBATION / DA-OWNER REVIEW |
| Falsification check | Executable profiles contain only current proof surfaces; no test-pare-* IDs; protected_class_audit deletion removed only uninvoked residue |
| Recommended posture | Deep review — DA/Owner-held |

## Follow-ons

| ID | Scope |
|---|---|
| `CI-COMMAND-ERGONOMICS-0` | Copy-paste proof blocks for routine/gate-state/closure PRs |
| `CI-HANDOFF-LIFECYCLE-ENFORCEMENT-0` | Birth-track template hardening if needed |
| Track B resumption | After DA merges this PR |