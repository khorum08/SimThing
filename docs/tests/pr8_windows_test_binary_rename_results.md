# PR8 Windows Test Binary Rename Results

> **Artifact lifecycle: PROBATION** (hygiene visibility for PR8-WIN-HYGIENE; review at PR9).

## Verdict

**PASS** — renaming the PR8 driver integration test removes the Windows UAC/elevation block without
changing PR8 proof semantics.

## Change

| Before | After |
|---|---|
| `crates/simthing-driver/tests/ct_bh3_closeout_sample_install.rs` | `crates/simthing-driver/tests/ct_bh3_closeout_sample_driver.rs` |
| `cargo test -p simthing-driver --test ct_bh3_closeout_sample_install` | `cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver` |

Cargo previously emitted `ct_bh3_closeout_sample_install-<hash>.exe`. The `install` substring triggered
Windows installer-detection (`os error 740`: "The requested operation requires elevation") unless
`__COMPAT_LAYER=RunAsInvoker` was set. The renamed binary runs without that workaround.

## Tests run (Windows, no UAC workaround)

| Command | Result |
|---|---|
| `cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver` | 2 passed; 0 failed; no UAC prompt |
| `cargo test -p simthing-clausething --test ct_scenario_container` | 45 passed |
| `cargo fmt --all -- --check` | pass |
| `git diff --check` | pass |

## Semantic scope

Rename and active-doc command updates only. No `simthing-sim`, driver runtime, GPU kernel, or
ClauseThing lowering changes.

## Lifecycle classification

| Artifact | Classification |
|---|---|
| `docs/tests/pr8_windows_test_binary_rename_results.md` | PROBATION |
| `ct_bh3_closeout_sample_driver.rs` | LIVE_GUARDRAIL (unchanged proof; new binary name) |
