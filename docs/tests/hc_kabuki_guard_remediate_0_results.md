# HC-KABUKI-GUARD-REMEDIATE-0 Results

## Status

**PROBATION / proof-present / DA-review-pending.** Remediation rung; do not self-graduate, undraft, merge, or advance progression.

Rung: HC-KABUKI-GUARD-REMEDIATE-0
HD-RECEIPT: 2367edab6be6
ORIENT-RECEIPT: 5e5bc265ae7b
Classification: DA-RESERVE(gate-wiring) expected route; crates/** in scope for kabuki deletion only
ANCHOR-ACK: orientation-harness-core

## What changed

- Deleted DEAD kabuki guard `simthing-gpu::scan_for_forbidden_validation_tokens` and its
  `crates/simthing-gpu/src/lib.rs` re-export (zero callers anywhere).
- Deleted #1355-shape self-scan `simthing-mapeditor::observe_module_source_forbids_workshop_residue`
  and its `lib.rs` re-export; removed the self-scan assertion from
  `tests/studio_live_observe_0.rs` while **keeping** the Cargo.toml workshop-dependency assert.
- RETIRED the two `GUARD-KABUKI-TRIPWIRE` rows in `scripts/ci/triage_log.tsv` and
  `scripts/ci/inspect_justifications.tsv` (net ledger −4). No HORIZON-ENTRY added — these were
  kabuki, not future API.
- HC-7 Exit-proof stamped **PROBATION**-leading; orientation regenerated.

## Falsifier (bites)

| control | pre-fix | fixed |
|---|---|---|
| GUARD-KABUKI-TRIPWIRE live sites (deterministic) | 2 INSPECT sites | 0 (PASS count 0) |
| cargo check -p simthing-gpu | green (fn uncalled) | green after delete |
| cargo check -p simthing-mapeditor | green | green after delete + test trim |

Build-green-after-delete is the bite: a naive delete that broke callers would fail cargo check.
Nothing depended on either deleted fn.

## Validation

- PASS: `cargo check -p simthing-gpu`
- PASS: `cargo check -p simthing-mapeditor`
- PASS: `bash scripts/ci/doctrine_selftest.sh`
- PASS: `bash scripts/ci/agent_scan.sh`
- PASS: `bash scripts/ci/gen_orientation.sh --check`
- PASS: `bash scripts/ci/doc_budget_check.sh --check`
- PASS: whole-tree doctrine scan reports `GUARD-KABUKI-TRIPWIRE  PASS  0` (live-hit 2 → 0)
- PASS: live 0.0.8.6 `--unpark` re-proved in disposable sandbox —
  `LIVE-0086-UNPARK-PROOF: PASS receipt=19e0e85c8d3f restored_rows=1 restored_handoffs=1 active_pointer=docs/design_0_0_8_6_studio_live_ops.md`

## Scope Ledger

- `crates/simthing-gpu/src/structural_validation.rs` — delete dead source-scan guard
- `crates/simthing-gpu/src/lib.rs` — drop re-export
- `crates/simthing-mapeditor/src/studio_live_observe.rs` — delete self-scan guard
- `crates/simthing-mapeditor/src/lib.rs` — drop re-export
- `crates/simthing-mapeditor/tests/studio_live_observe_0.rs` — drop self-scan assert only
- `scripts/ci/triage_log.tsv` — retire 2 GUARD-KABUKI rows (−2)
- `scripts/ci/inspect_justifications.tsv` — retire 2 GUARD-KABUKI rows (−2)
- design ladder HC-7 Exit-proof **PROBATION**-leading; orientation regenerated
- this results doc + evidence index pointer

Forbidden surfaces not touched: Studio/UI runtime behavior, gen_orientation logic, scans.tsv
scan defs, clearance router, new tables, HORIZON-ENTRY dodge. Net ledger −4 (retirement).

## Graduation routing

CI verdict: local required-check battery green at committed head.
Risk class: DA-reserve / gate-wiring expected route (remediation of HC-2 finds).
Recommended posture: PROBATION / proof-present / DA-review-pending.
DA stamps graduation at merge; do not self-graduate.
