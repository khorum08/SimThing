# KERNEL-GPUREADBACK-CONSTRUCTORS-SEAL-0 Results

## Status

**PROBATION** — three public GPU readback constructors sealed; doctrine scan baseline restored. Self-reported implementer proof only; not DA acceptance.

## PR / branch / merge

- Branch: `kernel-gpureadback-constructors-seal-0`
- PR: pending
- Merge: pending

## Recipient Agent

Cursor

## Orchestrator / §1A Triage Agent

Codex

## Executive DA

Opus / Owner

## What changed

- `gpu_readback.rs`: `EmissionRecordReadback::new`, `ThresholdEmissionReadback::new`, `ThresholdEventCandidatesReadback::new` changed from `pub fn` to `pub(crate) fn`.
- All call sites are in-crate (`world_state.rs`, `accumulator_op/session.rs`); no external callers.
- No allowlist or scanner changes.

## Before / after doctrine scan

| State | ALLOW-SEALED-PRODUCERS | DOCTRINE-SCAN-VERDICT |
|---|---|---|
| Before (0R2 baseline @ `a1fc28babf`) | FAIL 3 — three public `new -> Self` | FAIL |
| After seal | PASS 0 | PASS |

## Load-bearing proofs

| Proof | Result |
|---|---|
| `bash scripts/ci/doctrine_scan.sh` | PASS — 0 hard FAIL |
| `python scripts/ci/verify_kernel_surface.py` | PASS — 195/195 |
| `cargo check -p simthing-kernel` | PASS |

## Negative control (local, reverted)

| Mutation | Expected | Observed |
|---|---|---|
| Temporarily restore `EmissionRecordReadback::new` to `pub fn` | ALLOW-SEALED-PRODUCERS FAIL | exit 1 — `new -> Self (EmissionRecordReadback)` |

## Scope Ledger

| Path | Touched |
|---|---|
| `crates/simthing-kernel/src/gpu_readback.rs` | yes |
| `docs/tests/kernel_gpureadback_constructors_seal_results.md` | yes |
| `docs/tests/current_evidence_index.md` | yes |
| `docs/tests/ci-a-allowlist-scans_results.md` | yes (baseline note) |
| `scripts/ci/**`, allowlists, workflows, fixtures | **no** |

## Known gaps / next

- `CI-A-FIXTURES-0` unblocked once this lands and master doctrine scan is PASS.
- `validate_and_mint_placed_participants_by_location_id` remains core re-export gap (unchanged).

## DOCTRINE SCAN REPORT

```
DOCTRINE SCAN REPORT  (commit pending, post-seal positive control)
  scanner self-test: SKIPPED
  --- results ---
  ALLOW-SEALED-PRODUCERS  PASS  0  design §5 sealed producer allowlist
  ALLOW-BUFFER-HANDLES  PASS  0  design §5 buffer handle allowlist
  ALLOW-KERNEL-SURFACE  PASS  0  design §5 kernel surface allowlist
  --- summary ---
  hard failures: 0   inspect flags: 0   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
```
