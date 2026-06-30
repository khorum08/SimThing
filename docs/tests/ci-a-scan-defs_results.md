# CI-A-SCAN-DEFS-0 + CI-A-RUNNER-0 Results

## Status

**PROBATION** — scan data home and thin runner landed; self-reported positive control clean (zero hard FAIL). Gate 1 pending independent DA re-run/allowlist legitimacy audit. This doc does **not** claim DA acceptance.

## PR / branch / merge

- Branch: `ci-a-scan-defs-runner-0`
- PR: (pending)
- Merge: (pending)

## What changed

- Added `scripts/ci/scans.tsv` — seven RELIABLE blocklist + four HEURISTIC scans seeded from design §5.
- Added `scripts/ci/allow/sealed_producers.txt`, `inert_buffer_handles.txt`, `kernel_surface.txt` — populated from current `simthing-kernel` `lib.rs` surface (192 export symbols + 9 sealed/inert doors).
- Added `scripts/ci/README.md` — data layout, add-scan/onboarding rules, retirement discipline.
- Added `scripts/ci/doctrine_scan.sh` — thin runner: validates data, runs `rg -U`, handles rg exit 0/1/2, emits §1 report + stable footer.
- Updated `docs/design_0_0_8_4_6_ci_scaffolding.md` — `CI-A-SCAN-DEFS-0` and `CI-A-RUNNER-0` → PROBATION.
- Added evidence index row.

## Load-bearing proofs

| Proof | Command / observation | Catches |
|---|---|---|
| Positive control | `bash scripts/ci/doctrine_scan.sh` | Seeded scans false-failing current tree; report emitted |
| Footer exactly once | grep output below | Orchestrator parse failure |
| Malformed scans.tsv | Append `BAD \| RELIABLE \| x \| y \| \| ref \| blocker` → `scanner/data error: scans.tsv:… malformed record`; exit 1 | Silent skip of bad scan rows |
| Malformed allowlist | Append `forge_x \| read \| bad rationale \| retire never` to `sealed_producers.txt` → `symbol 'forge_x' does not match door-class 'read' grammar`; exit 1 | Rustified onboarding heuristic not enforced |
| rg no-match = PASS | RELIABLE scans with no hits (e.g. B3, FORGE-MINTERS) report `PASS  0` | Naive `set -e` treating rg exit 1 as shell failure |
| rg error path | Engine maps rg exit 2 to `scanner/data error` (not exercised on clean tree; code path in `run_rg_scan`) | Always-green scanner on rg failure |

## Scope Ledger

| Path | Why touched |
|---|---|
| `scripts/ci/scans.tsv` | Scan definition data home |
| `scripts/ci/allow/sealed_producers.txt` | Sealed producer admission records |
| `scripts/ci/allow/inert_buffer_handles.txt` | Inert buffer utility admission records |
| `scripts/ci/allow/kernel_surface.txt` | Kernel export closed-set data (192 symbols from `lib.rs`) |
| `scripts/ci/README.md` | Data layout + onboarding documentation |
| `scripts/ci/doctrine_scan.sh` | Thin scan runner |
| `docs/tests/ci-a-scan-defs_results.md` | Implementer proof (this file) |
| `docs/tests/current_evidence_index.md` | Evidence index row |
| `docs/design_0_0_8_4_6_ci_scaffolding.md` | Lifecycle rows → PROBATION |

**Not touched:** `crates/**`, `.github/workflows/**`, fixtures, self-test, hook installer, Track B.

## Known gaps / next

- `CI-A-ALLOWLIST-SCANS-0` — closed-set allowlist enforcement (not in this rung).
- `CI-A-FIXTURES-0` / `CI-A-SELF-TEST-0` — scanner self-test currently `SKIPPED`.
- `CI-A-WORKFLOW-0` — authoritative GitHub gate.
- Allowlist gaps for DA audit: `readback_threshold_events` / `readback_threshold_emissions` (`readback_*` vs `read_*` grammar); `validate_and_mint_placed_participants_by_location_id` (core re-export, no door-class prefix).
- HEURISTIC INSPECT flags on `fission.rs` / `boundary.rs` (`.kind` + semantic words in tests) — expected soft flags for DA classification.

## DOCTRINE SCAN REPORT (positive control)

```
DOCTRINE SCAN REPORT  (commit cbd328f4d5, 2026-06-30T16:34:17Z)
  scanner self-test: SKIPPED
  --- results ---
  B3-BUFFER-ESCAPE  PASS  0  design §5 B3 buffer escape
  FORGE-MINTERS  PASS  0  design §5 forge minters
  UNSAFE-FN  PASS  0  design §5 unsafe fn
  UNSAFE-ALLOW-ATTR  PASS  0  design §5 allow unsafe attr
  UNSAFE-FORBID-ATTR  PASS  0  design §5 forbid unsafe attr
  AS5-COLUMN-ALIAS  PASS  0  design §5 AS-5 ColumnIndex alias
  DENY-TOML-STUB  PASS  0  design §5 deny.toml stub
  RAW-DATA-INDEX  PASS  0  design §5 raw data[N] index
  SIM-KIND-READ  INSPECT  8  design §5 sim .kind read
  SEMANTIC-WORDS  INSPECT  73  design §5 semantic words below spec
  SPEC-STRING-CHANNEL  PASS  0  design §5 stringly channel identity
  --- summary ---
  hard failures: 0   inspect flags: 81   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only
DOCTRINE-SCAN-VERDICT: INSPECT  failures=0 inspect=81 selftest=SKIPPED
```
