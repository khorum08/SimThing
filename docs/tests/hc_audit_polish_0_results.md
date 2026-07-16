# HC-AUDIT-POLISH-0 Results

## Status

**PROBATION / proof-present / DA-review-pending.** Gate-wiring rung; do not self-graduate, undraft, merge, or advance progression.

Rung: HC-AUDIT-POLISH-0
HD-RECEIPT: 3a20bcab770d
ORIENT-RECEIPT: 5e5bc265ae7b
Classification: gate-wiring / DA-RESERVE(gate-wiring)
ANCHOR-ACK: orientation-harness-core

## What changed

1. **Horizon staleness single-source.** `gen_orientation.sh` now exports
   `HORIZON_ENTRY_STALE_DAYS="${HORIZON_ENTRY_STALE_DAYS:-90}"` (same pattern as
   `doctrine_scan.sh`). Lifecycle assess honors the env; the selftest no longer
   hard-codes `=90` over a caller override.
2. **Board push-sync live-handoff render.** `.github/workflows/clearance.yml`
   push-to-master path resolves via graceful
   `bash scripts/ci/handoff_dispatch.sh --current-handoff` (#1342: empty = none)
   before `--board-json`. Live dispatch renders; genuine none stays none.
   `current_handoff_path` honors `HD_HANDOFFS_DIR` for fixture isolation.
3. **GUARD-KABUKI evasion residue documented** in `scans.tsv` note column and
   one compressed line in `docs/ci_screening_surface.md` (DOC-BUDGET at ceiling;
   regex intentionally NOT widened). DA review remains the backstop.

## Falsifier (bites)

| control | pre-fix | fixed |
|---|---|---|
| mid-age (30d) marker under `HORIZON_ENTRY_STALE_DAYS=10` | lifecycle hard-code 90 keeps mid FRESH while scan flags INSPECT (diverge) | both scan + lifecycle treat mid as STALE; `window_days=10` |
| mid-age under default/unset window | n/a | both treat mid as FRESH; `window_days=90` |
| push-to-master board sync with live .hd | handoff stays empty → `current_handoff: none` | `--current-handoff` resolves live .hd → board renders rung |
| push with no live .hd | none | none (graceful empty) |

`doctrine_selftest.sh` records:
`horizon entry falsifier: PASS (... single_source_override=scan+lifecycle agree pre_fix_hardcode_diverges)`.

`handoff_dispatch.sh --selftest` records:
`push-sync-pre-fix-renders-none`, `push-sync-resolves-live-current-handoff`,
`push-sync-live-handoff-renders-on-board`, `push-sync-genuine-none-stays-none`.

## Verify-only (no action; deferred to HC-C)

- **anchor_reach_log prune readiness:** `librarian.sh --staleness` →
  `ANCHOR-QUERY-PRUNE: DRY removed=0 kept=46 days=30` (ready; zero stale rows).
- **leased-.hd-set coherence:** 7 leased graduated `.hd` rows in
  `closeout_artifacts.tsv`; live `handoffs/` holds those 7 + current
  `HC-AUDIT-POLISH-0.hd.md` (not yet leased). Coherent; no expiry/cruft
  (`ARTIFACT-EXPIRY-VERDICT: PASS expired=0 cruft=0`). Action deferred to HC-C.

## Validation

- PASS: `bash scripts/ci/doctrine_selftest.sh` (incl. single-source override falsifier)
- PASS: `bash scripts/ci/gen_orientation.sh --selftest` (incl. env-honoring lifecycle)
- PASS: `bash scripts/ci/handoff_dispatch.sh --selftest` (incl. push-sync resolution branch)
- PASS: `bash scripts/ci/agent_scan.sh`
- PASS: `bash scripts/ci/gen_orientation.sh --check`
- PASS: `bash scripts/ci/doc_budget_check.sh --check` (`ci_screening_surface.md` 525/525)
- PASS: live 0.0.8.6 `--unpark` re-proved in disposable sandbox —
  `LIVE-0086-UNPARK-PROOF: PASS receipt=19e0e85c8d3f restored_rows=1 restored_handoffs=1 active_pointer=docs/design_0_0_8_6_studio_live_ops.md`

## Scope Ledger

- `scripts/ci/gen_orientation.sh` — export + honor `HORIZON_ENTRY_STALE_DAYS`; selftest override + pre-fix diverge
- `scripts/ci/doctrine_selftest.sh` — dual-path scan+lifecycle single-source falsifier
- `.github/workflows/clearance.yml` — push-sync `--current-handoff` resolution branch
- `scripts/ci/handoff_dispatch.sh` — `HD_HANDOFFS_DIR` for current-handoff; push-sync selftest mirror
- `scripts/ci/scans.tsv` — GUARD-KABUKI note: accepted private-fn / var-bound residue
- `docs/ci_screening_surface.md` — one-line residue note (no line growth)
- design ladder HC-8 Exit-proof **PROBATION**-leading (no pipe in cell); orientation regenerated
- this results doc

Forbidden surfaces not touched: `crates/**`, Studio/UI, tripwire regex widening,
clearance router logic, new tables. 0.0.8.6 park block byte-exact (re-proved).

## Graduation routing

CI verdict: local required-check battery green at committed head.
Risk class: DA-reserve / gate-wiring.
Recommended posture: PROBATION / proof-present / DA-review-pending.
DA stamps graduation at merge; do not self-graduate.
