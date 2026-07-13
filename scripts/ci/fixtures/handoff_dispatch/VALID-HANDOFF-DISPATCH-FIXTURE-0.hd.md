---
rung: VALID-HANDOFF-DISPATCH-FIXTURE-0
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
required_checks: ["handoff-dispatch-selftest", "relay-lint-selftest"]
stop_conditions: ["stale-orient-receipt", "scope-widening"]
---
## BUILD
- Build deterministic projections.
## FENCES
- Keep scope narrow.
## EXIT-PROOF
- Fixtures prove receipt stability.
