# TESTS-COMPILE-FLOOR-NON-BEVY-0 Results

## Status

**PROBATION / DA REVIEW after 0R**. This rung is gate-state. It is not orchestrator-mergeable. Fable/DA or Owner clearance is required before merge.

## Mission

Add a standing GitHub Actions compile floor for test binaries that are safe to compile in non-owner-deep GHA:

```bash
cargo check -p <crate> --tests
```

The purpose is to prevent the dead-binary class from re-forming after `TEST-REPAIR-OR-REPLACE-TRUE-ORACLE-BINARIES-0` (#1106) and `TEST-PARE-PROTECTED-REPRESENTATIVE-RESTORE-0` (#1111).

## Scope

- Branch: `grok/tests-compile-floor-non-bevy-0`
- Base: `master` after #1111 merge (`c49abf044950132da76e365357c7f6320ca498ed`)
- Allowed edits: `scripts/ci/doctrine_exec_profiles.tsv`, evidence/design/index docs only
- No `crates/**` edits
- No inventory/boundary/scope/allowlist/scanner edits
- No workflow edits (profile table sufficient)

## Included crates

Profile `tests-compile-floor-non-bevy` runs `cargo check -p <crate> --tests` for:

| crate | local `cargo check -p <crate> --tests` | GHA-safe? | included? | reason | blocked surface | follow-on |
|---|---|---|---|---|---|---|
| simthing-core | PASS | yes | yes | CPU-only; no Bevy/desktop/typeface dev-deps | — | — |
| simthing-kernel | PASS | yes | yes | authority crate; compile-only; no runtime GPU proof | — | — |
| simthing-sim | PASS | yes | yes | CPU-oracle parity tests compile without desktop bootstrap | — | — |
| simthing-driver | blocked (doctrine) | no | no | test dependency graph reaches forbidden desktop/audio path on Linux; doctrine block, not a missing package | ALSA/audio + GPU runtime integration surface | split/pare driver test graph or local/owner-deep posture only |
| simthing-workshop | PASS | yes | yes | CPU workshop tests compile without Bevy mapeditor/tools drag | — | — |
| simthing-mapgenerator | PASS | yes | yes | CPU mapgenerator tests compile without Studio/Bevy | — | — |
| simthing-spec | FAIL (ResourceFlowSpec `capacity_budget` drift in deferred admission tests) | n/a | no | non-compiling deferred admission-substrate test binaries on live master | Admission Substrate deferred corpus | `TEST-PARE-ADMISSION-SUBSTRATE-DEAD-BINARIES-0` |
| simthing-clausething | FAIL (`ct_3b_4a_session_loop` seal/private-field drift) | n/a | no | doc-named enforcement binary does not compile after seal adoption | Admission Substrate + doc-named Clausething row | repair-or-replace or admission-substrate adjudication |
| simthing-gpu | PASS locally | no | no | wgpu/GPU runtime test surface; owner-deep class | GPU runtime proof | `TESTS-COMPILE-FLOOR-GPU-OWNER-DEEP-0` |
| simthing-mapeditor | FAIL (Bevy `E0432`/`E0063` on lib tests) | no | no | Bevy + egui desktop Studio shell | Bevy/desktop | `TESTS-COMPILE-FLOOR-STUDIO-OWNER-DEEP-0` |
| simthing-tools | FAIL (`typeface_lr5` missing symbol) | no | no | Bevy/winit/typeface owner-deep dev-deps | typeface/Bevy | `TEST-PARE-STUDIO-TYPEFACE-OWNER-DEEP-0` |

## Profile changes

Added to `scripts/ci/doctrine_exec_profiles.tsv`:

```text
profile_id: tests-compile-floor-non-bevy
profile_class: targeted
risk_class: gate-state/tests-compile-floor
crate_checks: -
tests: cargo check -p simthing-core --tests; cargo check -p simthing-kernel --tests; cargo check -p simthing-sim --tests; cargo check -p simthing-workshop --tests; cargo check -p simthing-mapgenerator --tests
doc_tests: -
gpu_required: no
expected_verdict_if_gpu_missing: PASS
```

The profile does not run `cargo test`, Atlas, Bevy, mapeditor desktop runtime, tools typeface runtime, GPU runtime proof, `apt-get`, or x11/wayland/ALSA setup.

`simthing-driver` is blocked from the non-owner-deep GHA compile floor because its test dependency graph reaches a forbidden desktop/audio dependency path on Linux. This is a doctrine block, not a missing package. No ALSA/libasound installation, X setup, desktop bootstrap, or GHA workaround is permitted. Future work must either split/pare the driver test graph so a non-desktop compile floor is possible, or keep driver coverage in a local/owner-deep posture.

`scripts/ci/doctrine_exec_profile_lint.sh` now fails lint for `owner_deep=false` executable profile commands containing forbidden desktop/audio/windowing/GPU tokens (ALSA, libudev/udev, X/Xvfb, Wayland, xkbcommon, xcb, EGL/GLX, Mesa/Vulkan, Bevy, winit/wininit, wgpu, mapeditor, typeface, apt-get) or blocked crates (`simthing-driver`, `simthing-gpu`, `simthing-mapeditor`, `simthing-tools`) in any `-p`, `-p=`, `--package`, or `--package=` form (tokenized `shlex` check).

## Proof

Local (branch head):

- `bash scripts/ci/doctrine_scan.sh`: PASS `failures=0 inspect=0`
- `bash scripts/ci/gen_digest.sh --check`: PASS
- `bash scripts/ci/doctrine_exec_profile_lint.sh`: PASS
- `bash scripts/ci/doctrine_exec_plan.sh --profile tests-compile-floor-non-bevy`: PASS
- Included `cargo check -p <crate> --tests` commands: all PASS locally for the five included crates
- `git diff --check origin/master...HEAD`: PASS

Process finding: an earlier attempt incorrectly probed `simthing-driver` on GHA and hit `alsa-sys`. That attempt is invalid proof and must not be repeated. The corrected profile excludes driver and records it as blocked by doctrine.

Latest-head proof: pending after stale-proof/lint-hardening 0R push (replace with fresh run/job IDs on new head).

## Known gaps / follow-ons

1. `simthing-spec` and `simthing-clausething` test compile floors deferred until admission-substrate / doc-named repair adjudication.
2. `simthing-gpu`, `simthing-mapeditor`, `simthing-tools` require owner-deep follow-on rungs.
3. `TEST-PARE-STUDIO-TYPEFACE-OWNER-DEEP-0` remains DA-held for Studio/typeface fossil review.
4. `TEST-PARE-ADMISSION-SUBSTRATE-DEAD-BINARIES-0` remains owner/Fable-gated.

## Graduation routing

```text
Graduation routing:
  CI verdict:          latest-head Doctrine Scan + targeted profile proof recorded below
  Triage entries:      none expected (gate-state profile; compile-only commands)
  Risk class:          gate-state / tests-compile-floor
  Falsification check: plan mode lists exactly five cargo check --tests commands; profile lint + forbidden-desktop-deps guard + GHA proof seal PASS; no driver/mapeditor/tools/gpu in executable commands; excluded crates documented as doctrine blocks
  Recommended posture: PROBATION / DA REVIEW — not orchestrator-mergeable
```