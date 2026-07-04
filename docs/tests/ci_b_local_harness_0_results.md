# CI-B-LOCAL-HARNESS-0 Results

## Status

**PROBATION / DA-OWNER REVIEW** — owner-local Track B executable harness wired. Remedial **0R** repairs proof freshness and strict footer self-proof. Not self-mergeable; DA/Owner clearance required.

## PR / branch / merge

- Branch: `ci-b-local-harness-0`
- PR: [#1129](https://github.com/khorum08/SimThing/pull/1129)
- Base: `origin/master` @ `b738e76f586f5b0d86aa4fd2c923a74fdde1ab7f` (post-#1128)
- Head: `e75e05f9b3d517c5b502953014f15e40ee09fdb9`

## Files changed

- `scripts/ci/doctrine_tests.sh` (new)
- `scripts/ci/doctrine_tests_profiles.tsv` (new)
- `docs/tests/ci_b_local_harness_0_results.md` (new)
- `docs/tests/current_evidence_index.md` (one Track B row)
- `docs/design_0_0_8_4_6_ci_scaffolding.md` (rung 1 status → PROBATION)

## Implemented harness modes

| Mode | Behavior |
|---|---|
| `--list` | Lists owner-local profiles from `doctrine_tests_profiles.tsv` |
| `--plan --profile owner-local-gpu-bevy` | Resolves live inventory-backed commands; emits §1 report; does not execute |
| `--profile owner-local-gpu-bevy` | Batch-executes resolved commands sequentially on owner machine |
| `--prove-report` | Proves footer format, malformed TSV rejection, GHA overlap guard, prerequisite blocking |

Resolver token: `RESOLVE:inventory-owner-local-gpu` reads `scripts/ci/test_inventory.tsv` only (not `doctrine_exec_profiles.tsv`). GPU legs batch one `cargo test -p <crate> --test <binary> -- --nocapture` per integration binary (never per-test-name spawn). Desktop/studio legs: `simthing-mapeditor` and `simthing-tools` integration binaries under `tests/`. `no_gpu` test names excluded from GPU leg resolution.

## Planned owner-local proof commands (49 binaries)

Resolved from live inventory in the PR tree:

- **simthing-clausething** (4): `ct_2a_intrinsic_flow`, `ct_2c_category_economy`, `ct_3b_4a_gpu_projection`, `mapgen_constitution_guards`
- **simthing-driver** (20): GPU/oracle integration binaries including `dress_rehearsal_atlas_batch_0_pack_gpu`, `gpu_exec0_readiness_fixture`, `gpu_measure_0080_0`, `mobility_gpu_kernel{0..11}_*`, `mobility_runtime1b_gpu_passgraph_fixture`, `owner_silo_gpu_tick`, `phase_m_frontier_v1_2_gpu_replay_acceptance`, `phase_m_jit_evaleml_wgsl_prototype`, `terran_pirate_skeleton_resident_tick`
- **simthing-gpu** (7): `bh0_saturating_flux`, `bh1_choke_readout`, `bh1r_choke_threshold`, `bh1r_scale_parallel_reduction`, `bh2_w_composition`, `bh2s_overlap_stress`, `min_plus_stencil`
- **simthing-sim** (2): `c8a_eml_infrastructure`, `c8b_intensity_eml_parity`
- **simthing-workshop** (5): `eml_phase5_intensity`, `multitarget_replay`, `overlay_order_semantics`, `persistent_bench`, `weighted_mean_parity`
- **simthing-mapeditor** (6): `accumulator_convergence_1_guards`, `canonical_scenario_load_save_display`, `runtime_vertical_seed`, `studio_ingestion_admission_report`, `terran_pirate_skeleton`, `tp_base_disc_gen`
- **simthing-tools** (5): `typeface_lr4` … `typeface_lr7`

Full plan transcript available via `bash scripts/ci/doctrine_tests.sh --plan --profile owner-local-gpu-bevy`.

## Mechanical proof

| Proof | Result |
|---|---|
| `bash -n scripts/ci/doctrine_tests.sh` | PASS |
| `doctrine_tests.sh --list` | PASS — `owner-local-gpu-bevy` profile listed |
| `doctrine_tests.sh --plan --profile owner-local-gpu-bevy` | PASS — 49 commands; footer `DOCTRINE-TESTS-VERDICT: INSPECT failures=0 inspect=1` (plan-only) |
| `doctrine_tests.sh --prove-report` | PASS — `DOCTRINE-TESTS-PROVE-REPORT: PASS` (malformed TSV negative + strict footer negatives) |
| `doctrine_scan.sh` | PASS — `DOCTRINE-SCAN-VERDICT: PASS failures=0 inspect=0` |
| `gen_digest.sh --check` | PASS |
| `doctrine_exec_profile_lint.sh` | PASS — `PROFILE-LINT: PASS profiles=6` |
| `doctrine_exec_profile_lint.sh --prove-gha-proof-seal` | PASS — `GHA-PROOF-SEAL: PASS prove` |
| `doctrine_exec_profile_lint.sh --prove-no-track-d-deletion-profiles` | PASS — `NO-TRACK-D-PROFILE-PROVE: PASS` |
| `git diff --check origin/master...HEAD` | PASS (no conflict markers) |

## Owner-local execution

Agent host (Windows, no `DOCTRINE_TESTS_GPU_OK=1`):

```
bash scripts/ci/doctrine_tests.sh --profile owner-local-gpu-bevy
```

Verdict: **INSPECT** — `owner-local GPU prerequisites not confirmed (set DOCTRINE_TESTS_GPU_OK=1 on owner machine with real adapter)`. Commands resolved and listed; no silent PASS. Real GPU/Bevy/desktop PASS remains owner-local obligation.

## INSPECT remaining

- Owner-local GPU/Bevy/desktop execution not run on agent host (expected; prerequisites not confirmed).
- Harness rung itself is **PROBATION / DA-OWNER REVIEW** until Owner/DA graduates.

## Scope ledger

| Fence | Touched? |
|---|---|
| Product code (`crates/**`) | no |
| Workflows (`.github/workflows/**`) | no |
| `doctrine_exec_profiles.tsv` | no |
| Inventory / lifecycle rows | no |
| Scans / allowlists | no |
| Tests added/deleted/restored | no |
| `cargo test --workspace --all-targets` | no |

## Forbidden proof avoided

- No `cargo test --workspace --all-targets`
- No GHA wiring of owner-local harness
- No `atlas_0080_0`, mapeditor_linux_cargo_check, studio_ingestion GHA probes, apt-get/x11/wayland/ALSA bootstrap
- No Track-D `test-pare-*` / deletion profile resurrection
- No test inventory / lifecycle / workflow edits

## CI-B-LOCAL-HARNESS-0R remedial repairs

| HOLD | Repair |
|---|---|
| HOLD-1 stale result-doc head SHA | `Head:` and plan wording now bind to live PR head after 0R commit |
| HOLD-2 weak malformed-footer proof | `--prove-report` uses strict `FOOTER_PATTERN`; rejects footers missing `failures=`, `inspect=`, `profile=`, `owner_local=true`, or `head_sha=` |

0R proof (at head `e75e05f9b3d517c5b502953014f15e40ee09fdb9`):

- `bash -n scripts/ci/doctrine_tests.sh`: PASS
- `--prove-report`: PASS (valid footer accepted; 5 malformed negatives rejected)
- `doctrine_scan.sh`: see mechanical proof table above
- Scope: only `scripts/ci/doctrine_tests.sh` and this results doc edited

## Graduation routing

- Return to orchestrator re-review
- Then DA/Owner review if clean
- **gate-state / PROBATION / DA-OWNER REVIEW** — not self-mergeable
- Next rung after clearance: **CI-B-TRIPWIRE-TAGS-0**
