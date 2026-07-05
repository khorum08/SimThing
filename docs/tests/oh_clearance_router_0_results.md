# OH-CLEARANCE-ROUTER-0 Results

## Status

**PROBATION / gate-wiring — not self-mergeable.** Rung 0 delivers the M1 clearance-router verdict surface; DA clearance required before graduation.

## PR / branch / merge

| Field | Value |
|---|---|
| PR | _(pending push)_ |
| Branch | `oh-clearance-router-0` |
| Base | `master` @ `6946d8adfe` |
| Rung | `OH-CLEARANCE-ROUTER-0` |

## What changed

- Added `scripts/ci/clearance_check.sh` — emits exactly one `CLEARANCE-VERDICT:` line per invocation (`ORCHESTRATOR-CLEARABLE`, `DA-RESERVE(reason)`, or `FAIL(remedy)`).
- Added rule TSVs: `precedented_classes.tsv`, `binding_conditions.tsv`, `clearance_ledger.tsv` (PALMA→Phase 6.2 binding rows backfilled discharged; fixture reproduces open-row reserve).
- Added nine committed selftest fixtures under `scripts/ci/fixtures/clearance/`.
- Wired `/clearance` into `doctrine_exec_commands.sh` + `.github/workflows/doctrine-exec-commands.yml` (collaborator-only, fork-safe, sticky verdict comment, ledger append+commit on PR branch).
- Added `doctrine_exec_clearance.sh` + `doctrine_exec_clearance_comment.sh` helpers.

## Load-bearing proofs

| Proof | Command / fixture | Catches |
|---|---|---|
| Router selftest battery | `bash scripts/ci/clearance_check.sh --selftest` | All nine regression classes in one PASS footer |
| #1150-shaped clearable | `clearance_selftest_clearable_1150_shape` | Active precedented-class PR with complete proof routes `ORCHESTRATOR-CLEARABLE` |
| #1151-shaped clearable | `clearance_selftest_clearable_1151_shape` | Second accepted class does not regress into DA reserve |
| #1152-shaped clearable | `clearance_selftest_clearable_1152_shape` | PALMA reach clears when no open binding condition applies |
| #1154-shaped reserve | `clearance_selftest_reserve_1154_binding_conditions` | Open DA binding rows force `DA-RESERVE(binding-conditions)` |
| Malformed TSV fail-closed | `clearance_selftest_fail_closed_malformed_tsv` | Malformed rule data cannot silently clear |
| Ambiguous class fail-closed | `clearance_selftest_fail_closed_ambiguous_class` | Two matched classes cannot silently clear |
| Gate-wiring self-application | `clearance_selftest_gate_wiring_self_application` | Router refuses to clear edits to its own surface |
| Suspended class kill-switch | `clearance_selftest_suspended_class` | `status=suspended` → `DA-RESERVE(class-suspended)` |
| Missing proof fields | `clearance_selftest_missing_required_proof_fields` | Absent `tested_code_sha`/`coverage_basis` → named `FAIL` |

### Selftest output (owner-local, 2026-07-05)

```
PASS clearance_selftest_clearable_1150_shape
PASS clearance_selftest_clearable_1151_shape
PASS clearance_selftest_clearable_1152_shape
PASS clearance_selftest_reserve_1154_binding_conditions
PASS clearance_selftest_fail_closed_malformed_tsv
PASS clearance_selftest_fail_closed_ambiguous_class
PASS clearance_selftest_gate_wiring_self_application
PASS clearance_selftest_suspended_class
PASS clearance_selftest_missing_required_proof_fields
CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE
CLEARANCE-SELFTEST: PASS (9 fixtures)
```

## Scope Ledger

| Path class | Touched | Notes |
|---|---|---|
| `scripts/ci/clearance_check.sh` | yes | M1 router |
| `scripts/ci/precedented_classes.tsv` | yes | precedented-class rows |
| `scripts/ci/binding_conditions.tsv` | yes | PALMA→6.2 rows (discharged live; open in #1154 fixture) |
| `scripts/ci/clearance_ledger.tsv` | yes | telemetry header |
| `scripts/ci/fixtures/clearance/**` | yes | selftest fixtures |
| `scripts/ci/doctrine_exec_commands.sh` | yes | `/clearance` parse |
| `scripts/ci/doctrine_exec_clearance*.sh` | yes | GHA ledger + sticky comment |
| `.github/workflows/doctrine-exec-commands.yml` | yes | `/clearance` job only |
| `docs/design_0_0_8_4_7_orchestration_harness.md` | yes | Rung 0 status row |
| Engine crates | **no** | |
| New Rust crate | **no** | |
| GPU/bevy/desktop GHA execution | **no** | recorded-proof consumption only |

## Known gaps / next

- `OH-RELAY-LINT-0` (M3) is the next rung; router does not yet require orientation receipts or ANCHOR-ACK.
- Live `/clearance` on this PR will self-route `DA-RESERVE(gate-wiring)` by design (self-application law).
- Local `doctrine_selftest.sh` positive control reported `UNKNOWN` on this Windows host (python PATH); GHA Linux runner is authoritative for doctrine selftest/scan.

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | PASS-RELIABLE expected on GHA (gate-wiring scope only; no engine/cargo edits) |
| Triage entries | none expected |
| Risk class | gate-wiring |
| Falsification check | `bash scripts/ci/clearance_check.sh --selftest` → 9/9 PASS; perturb `precedented_classes.tsv` to drop a tab → `DA-RESERVE(harness-error)`; post `/clearance` on a fork PR → no execution; post on collaborator non-gate PR shaped like #1150 → `ORCHESTRATOR-CLEARABLE` |
| Recommended posture | **deep** — audit fail-closed behavior, self-application refusal, ledger integrity, `/clearance` trust boundary |