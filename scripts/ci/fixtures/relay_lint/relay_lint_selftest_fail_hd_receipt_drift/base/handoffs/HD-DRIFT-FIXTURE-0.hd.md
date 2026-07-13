---
rung: HD-DRIFT-FIXTURE-0
kind: rung
track: fixture
base_sha: 2222222222222222222222222222222222222222
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
- Fixture base object.
## FENCES
- Fixture only.
## EXIT-PROOF
- Receipt must not drift.
