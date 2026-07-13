---
rung: HD-BOOTSTRAP-FIXTURE-0
kind: rung
track: fixture
base_sha: 1111111111111111111111111111111111111111
audience: coding
model_tier: std
expected_route: fixture
owner_approved: true
owner_notes: ""
surfaces: ["scripts/ci/relay_lint.sh"]
forbidden: ["crates/**"]
required_checks: ["relay-lint-selftest"]
stop_conditions: ["fixture-stop"]
---
## BUILD
- Fixture bootstrap.
## FENCES
- Fixture only.
## EXIT-PROOF
- Receipt matches added handoff.
