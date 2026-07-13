---
rung: HD-DISPATCH-SUBSTRATE-0
kind: rung
track: 0.0.8.4.8.4
base_sha: fd022256b82c30c42da7d51e041128494bf3dd0a
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: ""
surfaces: ["scripts/ci/handoff_dispatch.sh", "scripts/ci/relay_lint.sh", ".github/workflows/clearance.yml"]
forbidden: ["crates/**", "Studio/UI", "HD-3+"]
required_checks: ["handoff-dispatch-selftest", "relay-lint-selftest", "agent-scan", "orientation-check"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "lease-writer-duplication"]
---
## BUILD
- Add the strict `.hd.md` object parser, role projections, `HD-RECEIPT`, and board JSON.
- Render coding, orchestrator, and DA views deterministically from this one source object.
- Add workflow sync for handoff ingress and the standing SimThing Board.
## FENCES
- Reuse anchor triggers and closeout artifact ledger; add no second trigger or lease table.
- Keep Owner approval, notes, and directives visible without restating doctrine prose.
- Do not touch crates, Studio/UI, HD-3+, new clearance classes, or new route verdicts.
## EXIT-PROOF
- `handoff_dispatch.sh --selftest`, relay-lint selftest, agent scan, inventory drift, doc budget, and orientation check pass.
- Live PR shows handoff sticky, board issue, clearance `DA-RESERVE(gate-wiring)`, and relay-lint bootstrap PASS.
