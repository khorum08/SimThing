# GHA-PROOF-SEAL-0 Results

## Status

**DONE — merged #1101**. Merge commit `e49c8a258e4bd58d9c78b6c82b698cd5650dbaca`; head `317aba88f649a027fcd2c9997b182a7c27005cce`.

## Mission

Bind the #1100 remedial lesson: GitHub Doctrine Exec targeted/smoke/probe profiles must remain CPU-only. Atlas, Bevy, GPU-runtime, mapeditor/tools desktop, and desktop bootstrap proof belong on local/owner-deep artillery only.

## Implementation

| Artifact | Change |
|---|---|
| `scripts/ci/doctrine_exec_gha_proof_seal.sh` | New checker: non-`owner-deep` profiles fail if `tests`/`doc_tests` contain forbidden Atlas/Bevy/GPU/desktop tokens |
| `scripts/ci/doctrine_exec_profile_lint.sh` | Calls GHA proof seal after existing profile lint; `--prove-gha-proof-seal` runs synthetic prove battery |
| `scripts/ci/doctrine_exec_profiles.tsv` | Remediated `test-pare-src-unit-fossil-residue`: removed `mapeditor_linux_cargo_check.sh`; `simthing-mapeditor` moved to `cargo check` floor only |

## Forbidden on GHA (non-owner-deep)

- `atlas_0080_0`, `cargo test -p simthing-driver --test atlas*`
- `mapeditor_linux_cargo_check.sh`, `cargo test -p simthing-mapeditor|tools|gpu`
- `cargo test -p simthing-workshop --test …`, `typeface`, `studio_ingestion`
- `wgpu`, `bevy`, `apt-get`, `x11`, `wayland` in executable profile commands

## Allowed on GHA

- `cargo check -p` floors (including `simthing-gpu`, `simthing-driver`, `simthing-mapeditor`)
- Cheap exact CPU representatives (`simthing-clausething`, `simthing-spec`, `simthing-mapgenerator`, …)
- Inventory/boundary/drift/edit-scope/doctrine_scan/gen_digest/profile_lint gates

## Prove cases

`bash scripts/ci/doctrine_exec_profile_lint.sh --prove-gha-proof-seal`:

- BAD: atlas_0080_0, mapeditor_linux_cargo_check.sh, typeface_lr4, bh1_choke_readout
- GOOD: cargo check floors, clausething/mapgenerator CPU reps, owner-deep atlas exception

## Historical note (#1100)

`atlas_0080_0` timed out on GHA (run `28636114868`, 300s) and was removed from the GitHub profile. Do not re-add atlas to GHA targeted proof. Class 1 survivor proof remains local/owner-deep only.

## Validation

- `bash scripts/ci/doctrine_exec_profile_lint.sh` — PASS
- `bash scripts/ci/doctrine_exec_profile_lint.sh --prove-gha-proof-seal` — PASS
- `bash scripts/ci/test_edit_scope_check.sh --prove` — PASS
- `bash scripts/ci/doctrine_scan.sh` — PASS
- `bash scripts/ci/gen_digest.sh --check` — PASS

## Standing instruction

For `TEST-PARE-PROTECTED-CLASS-AUDIT-0` and `TEST-PARE-PROTECTED-RESIDUE-0`: GitHub proves ledgers, coverage maps, `cargo check` floors, and cheap CPU representatives only. Survivors needing Atlas/Bevy/GPU/desktop proof use `proof_mode=local-owner-deep`.
