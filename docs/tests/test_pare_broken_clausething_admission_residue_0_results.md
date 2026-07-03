# TEST-PARE-BROKEN-CLAUSETHING-ADMISSION-RESIDUE-0 Results

## Status

**DONE — merged #1098**. Live inventory after merge: **5671**.

## Ruling applied

`DA-RULING: TRACK-D-OWNER-DEEP-RESIDUE-PARE` Option A — Wave 1: broken compile and adapter-bound status are not retention reasons for admission duplicates with live representatives.

## Predecessor closeout (#1097)

- **#1097 `TEST-PARE-CPU-SAFE-BOUNDARY-SWEEP-0`**: **DONE — merged #1097**; merge commit `cfe934f17b5b62c6c3e617529cbbb088e901e043`; inventory was unchanged at 5675 before this wave.

## Mission

Delete broken clausething Tier 2 admission duplicate fossils blocked in #1097.

## Rows deleted

| File | Test | Representative kept |
|---|---|---|
| `ct_2a_intrinsic_flow.rs` | `unsupported_produces_field_hard_errors` | `ct_1a_entity::unsupported_entity_field_is_hard_error` |
| `ct_2c_category_economy.rs` | `economic_key_decoder_rejects_ambiguity` | `ct_0c_expansion::recursive_inline_script_is_rejected` |
| `ct_2c_category_economy.rs` | `rejected_key_forms_hard_error_with_spans` | `bh3_authoring_parse::bh3_authoring_rejects_invalid_chi_literal` |
| `mapgenerator_cli_pr11_scale_envelope.rs` | `generated_1000_star_admission_status_is_honest` | `ct_0c_expansion::recursive_inline_script_is_rejected` |

- rows considered: 7 (4 deleted targets + 3 representatives)
- rows deleted: 4
- inventory: 5675 → **5671**

## Scope table

Added `test-pare-broken-clausething-admission-residue` to `track-d-clausething-tests` authorized profiles (expiring with Track D paring).

## Targeted proof

Representative binaries only (no broken deleted binaries):

- `cargo test -p simthing-clausething --test ct_1a_entity -- --nocapture` — PASS
- `cargo test -p simthing-clausething --test ct_0c_expansion -- --nocapture` — PASS
- `cargo test -p simthing-clausething --test bh3_authoring_parse -- --nocapture` — PASS

## Required gates

Local:

- `bash scripts/ci/test_edit_scope_check.sh --prove` — PASS
- `bash scripts/ci/test_inventory_check.sh` — PASS; rows 5671
- `bash scripts/ci/test_pare_boundary_check.sh` — PASS
- `bash scripts/ci/test_inventory_drift_check.sh` — PASS
- `bash scripts/ci/doctrine_scan.sh` — PASS
- `bash scripts/ci/gen_digest.sh --check` — PASS
- `bash scripts/ci/doctrine_exec_profile_lint.sh` — PASS

## GitHub Doctrine Exec

Profile: `test-pare-broken-clausething-admission-residue`

Trigger: `/seal-proof profile=test-pare-broken-clausething-admission-residue`

## Graduation routing

- CI verdict: pending GitHub proof
- Risk class: admission duplicate deletion under owner-deep residue ruling
- Falsification check: four deleted rows are admission variants with named representatives; no never-pare row touched; no product code edited
- Recommended posture: deep — small inventory delta; enables Wave 2/3 residue processing

## Known gaps / next

- `ct_2a_intrinsic_flow` / `ct_2c_category_economy` remaining GPU session tests still do not compile (SlotIndex / missing import drift) — Wave 3 GPU residue classification
- PR11 scale behavior tests in `mapgenerator_cli_pr11_scale_envelope.rs` — Wave 3 Class 1/2 review
