---
rung: OWNER-NOTES-HANDOFF-DISPATCH-FIXTURE-0
kind: rung
track: 0.0.8.4.8.4
base_sha: fd022256b82c30c42da7d51e041128494bf3dd0a
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Owner note exact words: do not paraphrase."
surfaces: ["scripts/ci/handoff_dispatch.sh", "scripts/ci/relay_lint.sh", ".github/workflows/clearance.yml"]
forbidden: ["crates/**"]
required_checks: ["handoff-dispatch-selftest"]
stop_conditions: ["scope-widening"]
---
## BUILD
- Preserve owner notes verbatim.
## FENCES
- Active directives render; retired directives do not.
## EXIT-PROOF
- All three roles carry the owner channel.
