# HC-GUARD-KABUKI-TRIPWIRE-0 Results

## Status
**PROBATION / proof-present / DA-review-pending.** Gate-wiring rung; no self-graduation or merge.

Rung: HC-GUARD-KABUKI-TRIPWIRE-0  
HD-RECEIPT: 199404e38fe5  
ORIENT-RECEIPT: c866259aba5e  
Classification: gate-wiring  
ANCHOR-ACK: orientation-harness-core@8a365d1c0864

## What changed
- Added `GUARD-KABUKI-TRIPWIRE` as an INSPECT-only HEURISTIC scan for bespoke source/path guard shapes and direct test-side `include_str!("../src/").contains/find/matches/lines(...)` source greps.
- Generated the HC-2 source, realistic path-read, direct include_str controls, ordinary source/path traps, and row-removal falsifier inside the selftest sandbox at runtime, with no checked-in fixture files.
- Demoted handoff-template anti-kabuki prose to a pointer to the scan, documented the scan in the screening surface within the doc budget, and stamped the HC-2 ladder row PROBATION.

## Existing-Tree Census
`bash scripts/ci/doctrine_scan.sh` reports `GUARD-KABUKI-TRIPWIRE  INSPECT  37`; the count is `rg` line-fragment output from multiline matches. The deterministic site census from the same scan regex is:

| site | kind | disposition |
|---|---|---|
| `crates/simthing-gpu/src/structural_validation.rs:scan_for_forbidden_validation_tokens` | `source: &str` source guard | green triage; validation-source proof residue until admission-typed |
| `crates/simthing-mapeditor/src/studio_live_observe.rs:observe_module_source_forbids_workshop_residue` | `source: &str` source guard | green triage; observe residue boundary until admission-typed |

Plain source inclusion no longer counts; the `include_str!("../src/")` arm catches direct proof-surrogate scanning expressions only. The two surviving sites are accounted in `inspect_justifications.tsv` and `triage_log.tsv`; the scan remains visible and never hard-fails legitimate cases.

## Falsifier
`doctrine_selftest.sh` first proves the generated bad `source: &str` scanner reports `INSPECT` with the row present, then removes the `GUARD-KABUKI-TRIPWIRE` row in the same sandbox and records the expected row-removed failure to catch it: `head=INSPECT row_removed_expected_FAIL scan=MISSING count=0 exit=0`.

## Validation
- `bash scripts/ci/doctrine_selftest.sh` - PASS, including `GUARD-KABUKI-TRIPWIRE` source-scan/path-read/include_str controls, ordinary source/path traps, and row-removal falsifier.
- `bash scripts/ci/agent_scan.sh` - `AGENT-SCAN-VERDICT: PASS delta_inspect=0`.
- `bash scripts/ci/doctrine_scan.sh` - `DOCTRINE-SCAN-VERDICT: INSPECT failures=0 inspect=456`; `GUARD-KABUKI-TRIPWIRE  INSPECT  37` line fragments, 2 deterministic sites.
- `bash scripts/ci/gen_orientation.sh --check` - PASS; `bash scripts/ci/gen_digest.sh --check` - PASS; `bash scripts/ci/doc_budget_check.sh --check` - PASS; `bash scripts/ci/test_inventory_drift_check.sh` - PASS; `git diff --check` - PASS.
- 0.0.8.6 live unpark proof - `LIVE-0086-UNPARK-PROOF: PASS receipt=19e0e85c8d3f restored_rows=1 restored_handoffs=1 active_pointer=docs/design_0_0_8_6_studio_live_ops.md`.
- `cargo check -p simthing-spec`, `simthing-clausething`, and `simthing-mapeditor` were not runnable in this desktop shell because `cargo` is not on PATH.

## Scope Ledger
No crates, Studio/UI, clearance router, new tables, or durable selftest fixture files were changed. `scans.tsv` grows by one HEURISTIC row under the HC-2 prose-to-scan carve-out.

## Graduation Routing
CI verdict: INSPECT(37 `GUARD-KABUKI-TRIPWIRE` line fragments, 2 deterministic sites; total whole-tree INSPECT baseline includes older HEURISTIC debt)
Triage entries: `GUARD-KABUKI-TRIPWIRE:green` structural_validation + studio_live_observe
Risk class: gate-wiring  
Falsification check: remove the scan row inside `run_guard_kabuki_falsifier_test`; the generated bad source scanner is no longer caught and the harness records `row_removed_expected_FAIL`.
Recommended posture: deep - new gate wiring plus INSPECT census requires DA review.
