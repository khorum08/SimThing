# HC-HORIZON-ENTRY-CONVENTION-0 Results

## Status

**PROBATION / proof-present / DA-review-pending.** Gate-wiring rung; do not self-graduate, undraft, merge, or advance progression.

Rung: HC-HORIZON-ENTRY-CONVENTION-0
HD-RECEIPT: 9661c3391deb
ORIENT-RECEIPT: 5e5bc265ae7b
Classification: gate-wiring / DA-RESERVE(gate-wiring)
ANCHOR-ACK: orientation-harness-core

## What changed

- Defined greppable dated marker `HORIZON-ENTRY(<YYYY-MM-DD>): <intended consumer / design ref>`
  for future-facing consumerless API (horizon entries, not kabuki). Documented in
  `docs/handoff_template.md` §H and `docs/owner_authoring_guide.md`.
- `GUARD-KABUKI-TRIPWIRE` EXEMPTS only a symbol bearing a well-formed **FRESH** marker
  (default window 90 days via `HORIZON_ENTRY_STALE_DAYS`). Unmarked, bare-token, malformed,
  or stale-dated sites stay FLAGGED. Never a silent forever-pass; never a bare void token.
- Lifecycle assessment in `gen_orientation.sh --park/--unpark`: stale/malformed markers
  FLAG to INSPECT as deletion candidates; **never auto-delete**.
- Selftest samples generated in sandbox only (no checked-in fixture files; no
  `test_inventory.tsv` growth).

## Falsifier (bites)

| control | pre-fix | fixed |
|---|---|---|
| same consumerless `source: &str` scanner, unmarked | INSPECT | INSPECT |
| same fn, bare `HORIZON-ENTRY` token | INSPECT | INSPECT (not an exemption) |
| same fn, well-formed STALE date | INSPECT | INSPECT (not exempt) |
| same fn, well-formed FRESH date | INSPECT (no exemption logic) | PASS / count 0 (EXEMPT) |

`doctrine_selftest.sh` records:
`horizon entry falsifier: PASS (unmarked=INSPECT stale=INSPECT bare=INSPECT fresh=PASS pre_fix_fresh=INSPECT)`.

Green-both-ways avoided: stripping the exemption block re-FLAGS the fresh-marked site.

## Validation

- PASS: `bash scripts/ci/doctrine_selftest.sh` (incl. horizon-entry falsifier + pre/post strip)
- PASS: `bash scripts/ci/gen_orientation.sh --selftest` (incl. `orientation_horizon_entry_assessment`)
- PASS: `bash scripts/ci/agent_scan.sh`
- PASS: `bash scripts/ci/gen_orientation.sh --check`
- PASS: `bash scripts/ci/doc_budget_check.sh --check`
- PASS: live 0.0.8.6 `--unpark` re-proved in disposable sandbox —
  `LIVE-0086-UNPARK-PROOF: PASS receipt=19e0e85c8d3f restored_rows=1 restored_handoffs=1 active_pointer=docs/design_0_0_8_6_studio_live_ops.md`

## Scope Ledger

- `scripts/ci/doctrine_scan.sh` — dated FRESH-marker exemption filter for GUARD-KABUKI
- `scripts/ci/scans.tsv` — doctrine-ref notes HC-6 exemption (exclude column unchanged)
- `scripts/ci/doctrine_selftest.sh` — sandbox falsifiers (no durable fixtures)
- `scripts/ci/gen_orientation.sh` — park/unpark lifecycle assess + selftest
- `docs/handoff_template.md`, `docs/owner_authoring_guide.md`, `docs/ci_screening_surface.md`
- design ladder HC-6 Exit-proof **PROBATION**-leading; orientation regenerated

Forbidden surfaces not touched: `crates/**`, Studio/UI, clearance router, new tables,
FAIL-class exemption. Net durable fixture ledger: 0.

## Graduation routing

CI verdict: local required-check battery green at committed head.
Risk class: DA-reserve / gate-wiring.
Recommended posture: PROBATION / proof-present / DA-review-pending.
DA stamps graduation at merge; do not self-graduate.
