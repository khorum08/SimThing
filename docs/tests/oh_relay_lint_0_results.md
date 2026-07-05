# OH-RELAY-LINT-0 Results

## Status

**PROBATION / gate-wiring — not self-mergeable.** M3 relay/handoff linter; DA clearance required.

## PR / branch / merge

| Field | Value |
|---|---|
| PR | _(pending push)_ |
| Branch | `oh-relay-lint-0` |
| Base | `master` @ `39802af5` |
| Rung | `OH-RELAY-LINT-0` |

## What changed

- Added `scripts/ci/relay_lint.sh` — emits `RELAY-LINT-VERDICT: PASS | FAIL(reason) | INSPECT(reason)` plus `relay_lint_class`, `sketch`, `target` tags.
- Six selftest fixtures under `scripts/ci/fixtures/relay_lint/`.
- Wired `/relay-lint` on `doctrine-exec-commands.yml` (collaborator-only, fork-safe, sticky comment).
- Added `doctrine_exec_relay_lint.sh` + `doctrine_exec_relay_lint_comment.sh`.
- Ledgered 12 fixture files in `test_inventory.tsv` (drift-gate discipline).

## Load-bearing proofs

| Proof | Command / fixture | Catches |
|---|---|---|
| Selftest battery | `bash scripts/ci/relay_lint.sh --selftest` | All six regression classes |
| #1154-shaped PASS | `relay_lint_selftest_pass_1154_shape` | Known-good historical relay accepted |
| Missing coverage_basis | `relay_lint_selftest_fail_missing_coverage_basis` | Proof identity omission |
| Missing classification | `relay_lint_selftest_fail_missing_classification` | DA cannot route risk posture |
| Missing graduation routing | `relay_lint_selftest_fail_missing_graduation_routing` | PROBATION without DA routing data |
| Optional §5.1 sketch | `relay_lint_selftest_pass_optional_5_1_sketch` | LED sketch tagged, not required |
| Empty kabuki sections | `relay_lint_selftest_fail_empty_kabuki_sections` | Template-shaped empty relays rejected |

### Remediation proof output (owner-local)

**doctrine_selftest.sh**
```
positive control: PASS
inventory drift proof: PASS
DOCTRINE-SELFTEST-VERDICT: PASS
```

**gen_digest.sh --check**
```
gen_digest --check: PASS
```

**doctrine_scan.sh**
```
TEST-INVENTORY-DRIFT  PASS  0
DOCTRINE-SCAN-VERDICT: INSPECT  failures=0 inspect=415
```

### Selftest output (owner-local)

```
PASS relay_lint_selftest_pass_1154_shape
PASS relay_lint_selftest_fail_missing_coverage_basis
PASS relay_lint_selftest_fail_missing_classification
PASS relay_lint_selftest_fail_missing_graduation_routing
PASS relay_lint_selftest_pass_optional_5_1_sketch
PASS relay_lint_selftest_fail_empty_kabuki_sections
RELAY-LINT-SELFTEST: PASS (6 fixtures)
```

## Scope Ledger

| Path | Classification | Notes |
|---|---|---|
| `scripts/ci/relay_lint.sh` | gate-wiring harness | M3 linter |
| `scripts/ci/doctrine_exec_relay_lint*.sh` | gate-wiring harness | GHA helpers |
| `scripts/ci/fixtures/relay_lint/**` | seal-proof fixtures | selftest corpus |
| `scripts/ci/test_inventory.tsv` | inventory ledger | +12 rows (drift-gate discipline) |
| Engine crates | untouched | no engine edits |

## Conformance (spine/D-directives held)

- Admission behavior, not governance artifact (D8): lint rejects missing blocks with named FAIL.
- No engine-crate edits; no new Rust crate; no GPU/bevy/desktop GHA execution.
- M7 dual-mode: local script + `/relay-lint` on existing doctrine-exec carrier.

## Homing Boundary Classification

| Surface | Classification | Action |
|---|---|---|
| `relay_lint.sh` | CI gate-wiring harness | keep in `scripts/ci/` |
| engine crates | untouched | no engine edits |

tested_code_sha: 39802af5f13d96476c2e228390e912f30de524cc
coverage_basis: PASS — scripts-only gate-wiring rung; no binary-affecting paths touched

## Known gaps / next

- Advisory-only default (`--mode advisory`); blocking mode deferred to post-one-clean-cycle per design M3.
- No ORIENT-RECEIPT / ANCHOR-ACK enforcement (deferred rungs 2b/2c).
- `/relay-lint` GHA result recorded after PR opens.

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | _(GHA pending — update after push)_ |
| Triage entries | none expected |
| Risk class | gate-wiring |
| Falsification check | Mutate `coverage_basis` / classification / graduation routing in fixtures → named FAILs; post `/relay-lint` on PR |
| Recommended posture | deep — new gate surface and command wiring |