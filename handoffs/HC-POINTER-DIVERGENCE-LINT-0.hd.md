---
rung: HC-POINTER-DIVERGENCE-LINT-0
kind: rung
track: 0.0.8.4.8.4.1
base_sha: e600f76694e13041dc89715f19e8a0b9e8b1549a
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Fully-automated stack: coder = Grok CLI (grok-4.5), DA-driven. Touches the pointer machinery (gen_orientation.sh) — that surface is IN scope here, unlike prior HC rungs. Owner carries progression."
surfaces: ["scripts/ci/gen_orientation.sh", "docs/owner_authoring_guide.md"]
forbidden: ["crates/**", "Studio/UI", "scans.tsv", "clearance router changes", "new tables"]
required_checks: ["gen-orientation-selftest", "agent-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "unpark-not-byte-exact"]
---
## BUILD
- `gen_orientation.sh --check` FAILs when the authoritative `Active open rung` row
  (`authoritative_active_pointer`) names a rung whose exit-proof cell already carries a
  graduation/finished stamp, OR names a rung absent from the ladder. Name the row + remedy.
- Fix the HC-3 false-positive at the source: `next_rung_pointer` uses `is_completed_exit(deliv)`, so
  a SCOPE cell that merely *describes* completion words (e.g. HC-4's own cell) false-completes the
  rung and skips the pointer. A completion stamp lives in the EXIT-PROOF cell, not the scope cell —
  scope-cell text must not mark a rung complete.
- §3a cascade: `--park` REFUSES a divergent pointer (authoritative row names a completed/absent
  rung), so `--unpark` can never restore divergence. Same family as its open-PR refusal.
- Document the two-source rule in `owner_authoring_guide.md`: stamping a ladder cell does NOT move
  an authoritative `Active open rung` pointer — both must be updated at graduation.
## FENCES
- Transactional/rollback discipline for any `--park` change (HD-6). 0.0.8.6 `--park`/`--unpark`
  round-trip stays byte-exact (§3a). No crates, no scans.tsv, no router, no new tables.
## EXIT-PROOF
- Falsifiers that BITE (ruling 3): graduated-rung-named-as-pointer FAILs; unknown-rung FAILs;
  scope-cell-with-completion-words does NOT false-complete (the HC-3 case); legitimate
  not-yet-dispatched next rung passes; `none`-form passes; `--park` refuses a divergent pointer.
  Prove each differs on the pre-fix tree.
- `gen_orientation.sh --selftest` + agent-scan + orientation-check + doc-budget green; live 0.0.8.6
  `--unpark` re-proved (`19e0e85c8d3f`).
- PROBATION LEADS the HC-4 cell in-diff; orientation regenerated; DA stamps graduation at merge.
