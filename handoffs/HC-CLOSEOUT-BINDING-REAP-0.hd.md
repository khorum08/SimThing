---
rung: HC-CLOSEOUT-BINDING-REAP-0
kind: rung
track: 0.0.8.4.8.4.1
base_sha: 170add2154816198c9ee82bb9353bae242b0de75
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Fully-automated stack: coder = Grok CLI (grok-4.5), DA-driven. Reap only CLOSED tracks' discharged rows; never touch open/parked tracks. Owner carries progression."
surfaces: ["scripts/ci/track_closeout.sh", "scripts/ci/binding_conditions.tsv", "docs/track_closeout_protocol.md"]
forbidden: ["crates/**", "Studio/UI", "new tables", "clearance router changes", "gen_orientation changes", "reaping open/parked-track rows"]
required_checks: ["track-closeout-prove", "doctrine-selftest", "agent-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "open-or-parked-row-reaped"]
---
## BUILD
- `track_closeout.sh --apply` reaps the CLOSING track's `discharged` `binding_conditions.tsv` rows
  as part of close, reported in the CLOSEOUT-RECEIPT (count + rungs). Transactional (HD-6
  preflight/staged/rollback) with a rollback fixture.
- Retire the 10 existing dead rows now: the `discharged` rows from CLOSED tracks (TP×4, HU×2,
  OC×2, HD×2). Leave `HC-TRACK-OPEN-0` (discharged but THIS open track's) and `active` HC-CLOSEOUT-0.
## FENCES
- **Never reap an open or parked track's row.** Key off closed-track membership, not status alone —
  a `discharged` row of an OPEN track stays. Parked rows live in the park block, out of scope.
- 0.0.8.6 `--park`/`--unpark` stays byte-exact (§3a); the reaper must not touch the parked block.
- CLOSEOUT-RECEIPT shape change ⇒ update its selftest fixtures same PR (§3a). No crates, no
  `gen_orientation`, no router, no new tables. Net TSV rows DECREASE (retirement).
## EXIT-PROOF
- Falsifier that BITES (ruling 3): a fixture close with a closed-track discharged row present shows
  it REMOVED after `--apply`, PRESENT before — pre-fix leaves it, fixed reaps it. Negative control:
  an open-track discharged row is NOT reaped. Green-both-ways proves nothing.
- `binding_conditions.tsv` drops 12 rows → 2; `track_closeout.sh --prove` green; doctrine-selftest,
  agent-scan, orientation-check, doc-budget green; 0.0.8.6 `--unpark` re-proved (`19e0e85c8d3f`).
- PROBATION LEADS the HC-3 cell in-diff; orientation regenerated; DA stamps graduation at merge.
