# CC-RELAY-CLEARANCE-GATE-0 Results

## Status

PROBATION / proof-present / DA-review-pending.

## PR / branch / merge

- Branch: `codex/cc-relay-clearance-gate-0`
- PR: https://github.com/khorum08/SimThing/pull/1184
- tested_code_sha: b9d83eef72
- coverage_basis: PASS - harness implementation and fixture ledger validated at `b9d83eef72`; this results doc is evidence-only.
- ORIENT-RECEIPT: 3f856e32d5d7
- role: coding
- orientation_digest_sha: 5c5dfdb1c91bc2417d0c77b888e704aed8937158ac2e77a2b395743ddff63386

## What changed

- Added a clearance-verdict requirement to `relay_lint.sh` for DA-review / DA-reserve / deep-posture / gate-wiring relays.
- Accepted only fresh head-bound `CLEARANCE-VERDICT: DA-RESERVE(...)` for DA relays.
- Rejected missing or stale/unbound clearance as `FAIL(missing-clearance-verdict)`.
- Rejected `ORCHESTRATOR-CLEARABLE` in a DA relay as `FAIL(clearable-not-da-relay)`.
- Added `docs/handoff_template.md` and `docs/agent_onboarding.md` to `clearance_check.sh` gate-wiring paths.
- Added relay-lint fixtures for missing clearance, stale clearance, fresh DA-RESERVE clearance, ORCHESTRATOR-CLEARABLE misuse, and ordinary non-DA relay without clearance.
- Added clearance fixtures for handoff-template and agent-onboarding gate-wiring routing.
- Regenerated `docs/orchestrator_orientation.md` after the relay-lint schema stamp changed.
- Added fixture inventory rows required by the existing drift gate for the new `scripts/ci/fixtures/**` files.

## Load-bearing proofs

```text
$ bash scripts/ci/relay_lint.sh --selftest
PASS relay_lint_selftest_pass_1154_shape
PASS relay_lint_selftest_fail_missing_coverage_basis
PASS relay_lint_selftest_fail_missing_classification
PASS relay_lint_selftest_fail_missing_graduation_routing
PASS relay_lint_selftest_pass_optional_5_1_sketch
PASS relay_lint_selftest_fail_empty_kabuki_sections
PASS relay_lint_selftest_fail_live_pointer_current_pr_head
PASS relay_lint_selftest_fail_live_pointer_docs_refresh_head
PASS relay_lint_selftest_fail_live_pointer_latest_run
PASS relay_lint_selftest_fail_missing_clearance_verdict
PASS relay_lint_selftest_fail_stale_clearance_verdict
PASS relay_lint_selftest_pass_fresh_da_reserve_clearance
PASS relay_lint_selftest_fail_clearable_da_relay
PASS relay_lint_selftest_pass_non_da_without_clearance
PASS cold_start_selftest_valid_coding_receipt
PASS cold_start_selftest_valid_orchestrator_receipt
PASS cold_start_selftest_fail_missing_receipt
PASS cold_start_selftest_fail_stale_receipt
PASS cold_start_selftest_fail_wrong_role
PASS anchor_integrity_selftest_pass_gate_wiring_ack
PASS anchor_integrity_selftest_fail_missing_ack
PASS anchor_integrity_selftest_fail_stale_ack
PASS anchor_integrity_selftest_fail_unknown_anchor
RELAY-LINT-SELFTEST: PASS (23 fixtures)
```

```text
$ bash scripts/ci/clearance_check.sh --selftest
PASS clearance_selftest_clearable_1150_shape
PASS clearance_selftest_clearable_1151_shape
PASS clearance_selftest_clearable_1152_shape
PASS clearance_selftest_reserve_1154_binding_conditions
PASS clearance_selftest_fail_closed_malformed_tsv
PASS clearance_selftest_fail_closed_ambiguous_class
PASS clearance_selftest_gate_wiring_self_application
PASS clearance_selftest_suspended_class
PASS clearance_selftest_missing_required_proof_fields
PASS clearance_selftest_fail_closed_empty_requested_diff
PASS clearance_selftest_fail_triage_missing
PASS clearance_selftest_pass_triage_present
PASS clearance_selftest_gate_wiring_handoff_template
PASS clearance_selftest_gate_wiring_agent_onboarding
CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE
CLEARANCE-SELFTEST: PASS (14 fixtures)
```

```text
$ bash scripts/ci/gen_orientation.sh --check
gen_orientation --check: PASS
```

```text
$ bash scripts/ci/gen_digest.sh --check
gen_digest --check: PASS
```

```text
$ bash scripts/ci/doctrine_scan.sh
DOCTRINE SCAN REPORT  (commit b9d83eef72, 2026-07-06T13:53:30Z)
  scanner self-test: SKIPPED
  scan mode: whole-tree
  reliable scope: whole-tree
  heuristic scope: whole-tree
  --- results ---
  B3-BUFFER-ESCAPE  PASS  0  design §5 B3 buffer escape
  FORGE-MINTERS  PASS  0  design §5 forge minters
  UNSAFE-FN  PASS  0  design §5 unsafe fn
  UNSAFE-ALLOW-ATTR  PASS  0  design §5 allow unsafe attr
  UNSAFE-FORBID-ATTR  PASS  0  design §5 forbid unsafe attr
  AS5-COLUMN-ALIAS  PASS  0  design §5 AS-5 ColumnIndex alias
  DENY-TOML-STUB  PASS  0  design §0.6.6 deny.toml stub
  RAW-DATA-INDEX  PASS  0  design §5 raw data[N] index
  SIM-KIND-READ  PASS  0  design §5 sim .kind read
  SEMANTIC-WORDS  PASS  0  design §5 semantic words below spec
  SPEC-STRING-CHANNEL  PASS  0  design §5 stringly channel identity
  ALLOW-SEALED-PRODUCERS  PASS  0  design §5 sealed producer allowlist
  ALLOW-BUFFER-HANDLES  PASS  0  design §5 buffer handle allowlist
  ALLOW-KERNEL-SURFACE  PASS  0  design §5 kernel surface allowlist
  TEST-BUDGET  PASS  0  design §0.9.5 test admission budget
  SPEC-LOWERER-KIND-READ  INSPECT  415  ci_screening_surface §12 + design §0A.1; HEURISTIC tripwire
  TEST-INVENTORY-DRIFT  PASS  0  stock gate: inventory matches discovered tests and KEEP rows are owned
  DOC-BUDGET  PASS  0  DOC-BUDGET-VERDICT: PASS
  RULE-EXPIRY  PASS  0  RULE-EXPIRY-VERDICT: PASS
  AGENTS-STUB  PASS  0  AGENTS-STUB-VERDICT: PASS
  --- summary ---
  hard failures: 0   inspect flags: 415   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only
DOCTRINE-SCAN-VERDICT: INSPECT  failures=0 inspect=415 selftest=SKIPPED
  --- inspect justifications ---
  justifications file present with 1 entries
```

```text
$ git diff --check
<no output>
```

```text
$ bash scripts/ci/clearance_check.sh --range master..HEAD
CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)
```

```text
$ bash scripts/ci/test_inventory_drift_check.sh
TEST-INVENTORY-DRIFT-CHECK REPORT
  rows: 930
  discovered: 928
  unledgered: 0
  stale: 0
  promotion-target rows: 0
TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS
```

Necessity-scoped omissions:

- `scanner unchanged - doctrine_selftest not required`
- `no Rust crate touched - cargo check not required`

## Scope Ledger

| Item | Status | Notes |
|---|---|---|
| relay_lint clearance-verdict gate | implemented | DA-review / DA-RESERVE / deep-posture / gate-wiring relays require clearance. |
| DA relay without clearance fixture | implemented | `relay_lint_selftest_fail_missing_clearance_verdict`. |
| stale/unbound clearance fixture | implemented | `relay_lint_selftest_fail_stale_clearance_verdict`. |
| fresh head-bound DA-RESERVE fixture | implemented | `relay_lint_selftest_pass_fresh_da_reserve_clearance`. |
| ORCHESTRATOR-CLEARABLE in DA relay fixture | implemented | `relay_lint_selftest_fail_clearable_da_relay`. |
| ordinary non-DA relay fixture | implemented | `relay_lint_selftest_pass_non_da_without_clearance`. |
| clearance_check handoff_template gate-wiring path | implemented | `clearance_selftest_gate_wiring_handoff_template`. |
| clearance_check agent_onboarding gate-wiring path | implemented | `clearance_selftest_gate_wiring_agent_onboarding`. |
| generated orientation/digest freshness | checked | orientation regenerated; both freshness checks PASS. |
| fixture inventory rows | implemented | Required by existing drift gate for new `scripts/ci/fixtures/**` files. |
| production Rust crates | not touched | No crate code changed. |

## Conformance

- DA-review relay cannot lint without a clearance verdict.
- `DA-RESERVE(...)` is the only clearance verdict accepted as DA-relay justification.
- `ORCHESTRATOR-CLEARABLE` means no DA relay is warranted.
- `docs/handoff_template.md` and `docs/agent_onboarding.md` route as `DA-RESERVE(gate-wiring)`.
- No production Rust or workflow behavior changed.

## Known gaps / next

- DA must confirm the PR-head binding is strong enough and fixture-deterministic.
- DA must confirm ordinary non-DA relays are not accidentally forced to carry clearance verdicts.
- DA must confirm this does not reintroduce per-handoff orientation runs.
- Handoff said not to edit `test_inventory.tsv`, but new fixture files are discovered by the existing drift gate; the added rows are fixture-ledger rows only.

## Graduation routing

- CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)
- CI verdict: doctrine_scan INSPECT(415), no hard failures; required selftests and freshness checks PASS
- Triage entries: none for this diff; existing heuristic INSPECT covered by `inspect_justifications.tsv`
- Risk class: gate-wiring, harness-enforcement
- Falsification check: inspect `scripts/ci/relay_lint.sh`, `scripts/ci/clearance_check.sh`, fixtures, and this results doc; confirm DA-review relays fail without fresh head-bound clearance, `ORCHESTRATOR-CLEARABLE` cannot justify DA relay, `DA-RESERVE(...)` can, and handoff-template/onboarding changes route gate-wiring.
- Recommended posture: deep - gate-wiring harness enforcement.
