---
rung: UNKNOWN-KEY-HANDOFF-DISPATCH-FIXTURE-0
kind: rung
track: 0.0.8.4.8.4
base_sha: fd022256b82c30c42da7d51e041128494bf3dd0a
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: ""
surfaces: ["scripts/ci/handoff_dispatch.sh"]
forbidden: ["crates/**"]
required_checks: ["handoff-dispatch-selftest"]
stop_conditions: ["scope-widening"]
extra: no
---
## BUILD
- Unknown key fails.
## FENCES
- Fixture only.
## EXIT-PROOF
- Lint fails.
