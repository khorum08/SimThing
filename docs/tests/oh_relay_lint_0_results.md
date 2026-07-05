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

| Path | Touched | Notes |
|---|---|---|
| `scripts/ci/relay_lint.sh` | yes | M3 linter |
| `scripts/ci/doctrine_exec_relay_lint*.sh` | yes | GHA helpers |
| `scripts/ci/doctrine_exec_commands.sh` | yes | `/relay-lint` parse |
| `scripts/ci/fixtures/relay_lint/**` | yes | selftest fixtures |
| `scripts/ci/test_inventory.tsv` | yes | +12 rows (mandatory drift-gate ledger) |
| `.github/workflows/doctrine-exec-commands.yml` | yes | relay-lint job |
| Engine crates | **no** | |

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