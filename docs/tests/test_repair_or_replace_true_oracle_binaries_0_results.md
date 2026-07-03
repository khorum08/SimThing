# TEST-REPAIR-OR-REPLACE-TRUE-ORACLE-BINARIES-0 Results

## Status

INTERVENTION 0I APPLIED / PROBATION - DA review required.

#1106 is not merge-authorized. Passing local checks or a green CI result is proof input only, not clearance.

INTERVENTION 0I applied: #1106 was corrected after scope review. CI/gate-state edits were reverted or neutralized; admission-substrate deletions were restored/deferred; simthing-kernel remained untouched; deletion continued aggressively only across allowed non-kernel, non-admission test surfaces. PR remains PROBATION / DA review required. No merge authorization.

## PR / branch / merge

- Branch: `test-repair-or-replace-true-oracle-binaries-0`
- PR: #1106
- Base SHA: `6c637fcd89f2293408656825186e515618737c57`
- Prior intervention head: `f26682eae33931e7d29dd293bb829909490f9724`
- Merge SHA, if merged: none

## Scope correction

- Reverted CI/gate-state profile/scope edits: `scripts/ci/doctrine_exec_profiles.tsv` and `scripts/ci/test_edit_scope.tsv` are restored to the master/base state. This PR no longer adds `test-repair-or-replace-true-oracle-binaries`, changes `sentinel-core`, or carries `td-torb-*` scope authority.
- Restored/deferred admission-substrate files: 9 files / 30 inventory rows under `crates/simthing-spec/tests/**` and `crates/simthing-clausething/tests/**`.
- Kernel files touched: no.
- Allowed-surface deletions retained: 115 non-kernel, non-admission fossil test binaries / 895 inventory rows across `simthing-driver`, `simthing-sim`, and `simthing-gpu`.
- New replacement binaries added by the aborted 0R path: none.
- Scanner/allowlist edits: none.

## Deferred admission-substrate deletion candidates

Follow-on rung: `TEST-PARE-ADMISSION-SUBSTRATE-DEAD-BINARIES-0`.

| Path | Reason discovered | Why deletion is deferred | Recommended follow-on |
|---|---|---|---|
| `crates/simthing-clausething/tests/ct_1b_recalc.rs` | compile-floor/dead-binary sweep | admission-substrate protected from this sweep | `TEST-PARE-ADMISSION-SUBSTRATE-DEAD-BINARIES-0` |
| `crates/simthing-clausething/tests/ct_1c_tradition.rs` | compile-floor/dead-binary sweep | admission-substrate protected from this sweep | `TEST-PARE-ADMISSION-SUBSTRATE-DEAD-BINARIES-0` |
| `crates/simthing-clausething/tests/ct_3b_4a_gpu_projection.rs` | compile-floor/dead-binary sweep | admission-substrate protected from this sweep | `TEST-PARE-ADMISSION-SUBSTRATE-DEAD-BINARIES-0` |
| `crates/simthing-clausething/tests/ct_3b_4a_headline.rs` | compile-floor/dead-binary sweep | admission-substrate protected from this sweep | `TEST-PARE-ADMISSION-SUBSTRATE-DEAD-BINARIES-0` |
| `crates/simthing-clausething/tests/ct_3b_4a_session_loop.rs` | compile-floor/dead-binary sweep | admission-substrate protected from this sweep | `TEST-PARE-ADMISSION-SUBSTRATE-DEAD-BINARIES-0` |
| `crates/simthing-clausething/tests/ct_rf_eml_rate.rs` | compile-floor/dead-binary sweep | admission-substrate protected from this sweep | `TEST-PARE-ADMISSION-SUBSTRATE-DEAD-BINARIES-0` |
| `crates/simthing-spec/tests/owner_silo_disburse_down.rs` | compile-floor/dead-binary sweep | admission-substrate protected from this sweep | `TEST-PARE-ADMISSION-SUBSTRATE-DEAD-BINARIES-0` |
| `crates/simthing-spec/tests/owner_silo_runtime_writeback.rs` | compile-floor/dead-binary sweep | admission-substrate protected from this sweep | `TEST-PARE-ADMISSION-SUBSTRATE-DEAD-BINARIES-0` |
| `crates/simthing-spec/tests/resource_flow_opt_in_roundtrip.rs` | compile-floor/dead-binary sweep | admission-substrate protected from this sweep | `TEST-PARE-ADMISSION-SUBSTRATE-DEAD-BINARIES-0` |

## Review table

Durable review table: `docs/tests/test_repair_or_replace_true_oracle_binaries_0_review.tsv`.

0I columns distinguish:

- `DEFERRED_ADMISSION_SUBSTRATE` for every restored spec/clausething row.
- `DELETED_ALLOWED_SURFACE` for retained deletions in allowed non-kernel, non-admission test surfaces.
- `OWNED_BY_EXISTING_COMPILING_TEST` where a deleted live surface has an existing current owner.
- `DEFERRED_FOR_DA` where a live surface needs DA review rather than a new replacement binary in this intervention.

## Inventory

- Before #1106: 5,147 rows.
- Before intervention: 4,222 rows.
- After intervention: 4,252 rows.
- Restored/deferred admission rows: 30.
- Deleted allowed-surface rows retained: 895.

## D2o KEEP surface dispositions

- PALMA field-not-route: re-keyed to the existing STEAD guard `crates/simthing-clausething/tests/stead_spatial_contract_guards.rs::palma_feedstock_indexes_structural_grid_and_emits_no_routes`; not rerun in this intervention because clausething is protected from this sweep.
- c4 lifecycle/fission inheritance: re-keyed to existing sim unit owners `crates/simthing-sim/src/tree_mutation.rs::activate_overlay_restores_parked_lifecycle` and `crates/simthing-sim/src/fission.rs::fission_clone_capability_children_remaps_affects_and_copies_shadow`.
- c5 SoftAggregateGuard surfaces: re-keyed/deferred against existing `threshold_registry.rs` validator machinery; no new replacement binary was added because this intervention does not add inventory-backed tests.

## Proof

0I local/static proof rerun:

- `bash scripts/ci/doctrine_scan.sh`: PASS, failures=0 inspect=0.
- `bash scripts/ci/gen_digest.sh --check`: PASS.
- `bash scripts/ci/test_inventory_check.sh`: PASS; rows=4,252, discovered=4,252, missing=0, extra=0.
- `bash scripts/ci/test_pare_boundary_check.sh`: PASS; live inventory rows=4,252, missing owning boundary=0.
- `bash scripts/ci/test_inventory_drift_check.sh`: PASS; unledgered=0, stale=0.
- `cargo check -p simthing-driver`: PASS.
- `cargo check -p simthing-sim`: PASS.
- `cargo check -p simthing-gpu`: PASS.
- `cargo check -p simthing-workshop`: PASS.
- `cargo check -p simthing-mapgenerator`: PASS.

No broad full-crate cargo tests were used.

## Scope Ledger

- Specified: correct #1106 scope; remove unauthorized profile/scope edits; restore/defer admission-substrate deletions; keep kernel untouched; continue deletion only in allowed surfaces.
- Implemented: profile/scope rows reverted, protected admission files restored, inventory/audit ledger reconciled to restored files, allowed non-admission deletions retained, evidence updated.
- Proxied: none.
- Deferred: admission-substrate dead-binary deletion to `TEST-PARE-ADMISSION-SUBSTRATE-DEAD-BINARIES-0`; DA review of live-surface owner equivalence; any CI compile-floor architecture repair.
- Out of scope: product logic changes, scanner/allowlist edits, new CI profiles, new GHA proof-seal logic, simthing-kernel/spec/clausething replacements, merge.

## Known gaps / next

- PR remains PROBATION pending DA review.
- No merge authorization.
- Any admission-substrate deletion belongs to `TEST-PARE-ADMISSION-SUBSTRATE-DEAD-BINARIES-0`.
- Any standing CI compile-floor repair belongs to a separate CI rung, not this intervention.

## Graduation routing

- CI verdict: pending/DA review; local proof only unless a live check is separately dispatched.
- Risk class: deletion wave + scope correction.
- Recommended posture: deep DA review.
- Merge authorization: none.
