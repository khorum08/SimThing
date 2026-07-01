# TP-SCALE-ENVELOPE-0 / 0R Results

## Status

**PROBATION** — `TP-SCALE-ENVELOPE-0` remains **HELD**; repair rung `TP-SCALE-ENVELOPE-0R` landed on branch `tp-scale-envelope-0` (PR #1073 retitled). The swallowed-panic false-green path is removed. The reduction-topology upload defect is repaired; orchestrator review required before promoting the rung.

## Original HOLD

`TP-SCALE-ENVELOPE-0` proved generate → lattice → RF-budget → link → `install_atomic` at 1500-star scale, but the terminal exit proof was false-green:

- On a real adapter, `SimSession::open_from_spec` panicked in `initial_gpu_sync → WorldGpuState::upload_reduction_topology` (`Queue::write_buffer` overran a **24-byte** `column_rules` destination buffer).
- The test used `catch_unwind`, printed `skipping live adapter session proof; initial_gpu_sync reduction-topology upload rejected the scale shape`, and returned green without reaching `assert!(session.mapping.is_none())`.

Root cause: `WorldGpuState::rebuild_for_slots` grew `n_slots` and `n_dims` together but did **not** reallocate `column_rules` (sized only at `WorldGpuState::new` from the placeholder registry). `rebuild_for_registry` already resized `column_rules`; the slot-growth path did not.

## 0R repair

- **`crates/simthing-kernel/src/world_state.rs`:** `rebuild_for_slots` now reallocates `column_rules` (and resets reduction sidecar buffers when the value-preservation path does not apply) whenever slot capacity grows with a widened registry.
- **`crates/simthing-kernel/src/accumulator_op/runtime.rs`:** `ensure_velocity_session` recreates the session when `n_slots` / `n_dims` no longer match (stale-session guard after shape sync).
- **`crates/simthing-clausething/tests/tp_scale_envelope.rs`:** removed the `catch_unwind` swallow-and-return path; session open is a direct `expect("open TP scale session")` plus `assert!(session.mapping.is_none())`.

## Terminal session proof

**Local real-adapter run (2026-07-01, Windows):**

- Reduction-topology upload no longer fails on the 24-byte `column_rules` buffer; `upload_reduction_topology` completes.
- `SimSession::open_from_spec` still fails later in the same `initial_gpu_sync` at `upload_velocity_ops_with_bands → write_op_bytes` with wgpu `Queue::write_buffer: Not enough memory left` (honest panic — not swallowed).
- **Not yet claimed:** `session.mapping.is_none()` on this host. Orchestrator should re-run on hardware with sufficient GPU memory or adjudicate whether velocity op upload at 7505-slot headroom needs a follow-on sizing rung.

Preserved legs unchanged: seed `770421`, 1500-star connected disc, lattice hierarchy, structural frame above dense-field cap, budgeted RF admission (3000 participants / 2 arenas), typed atlas deferral, `install_atomic` through 7505-slot footprint.

## Load-bearing proofs

| Check | Result |
|---|---|
| `cargo check -p simthing-kernel` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo check -p simthing-clausething` | PASS |
| `cargo test -p simthing-kernel rebuild_for_slots_expands_column_rules_when_dims_grow` | PASS |
| `cargo test -p simthing-mapgenerator --test topology_stead` | PASS (9/9) |
| `cargo test -p simthing-mapgenerator --test connectivity` | PASS (7/7) |
| `cargo test -p simthing-clausething --test tp_scale_envelope tp_scale_envelope_disc_1500_admits_installs_with_budget` | FAIL (honest) — velocity `write_op_bytes` wgpu validation on local adapter; no skip message |
| `bash scripts/ci/gen_digest.sh --check` | skipped — WSL/bash unavailable locally |
| `bash scripts/ci/doctrine_scan.sh` | skipped — WSL/bash unavailable locally |

## INSPECT / triage

None for this repair rung (`scripts/ci/triage_log.tsv` unchanged).

## Scope Ledger

| Element | State |
|---|---|
| Remove catch_unwind false-green session path | implemented |
| Fix reduction-topology `column_rules` sizing on slot growth | implemented |
| Preserve 1500-star / RF-budget / install_atomic legs | implemented |
| Terminal `mapping.is_none()` on local adapter | **not reached** — velocity upload blocks session open (honest failure) |
| Phase 1 content / scanners / allowlists / new AccumulatorRole | held (untouched) |
| `TP-SCALE-ENVELOPE-0` self-marked COMPLETE | held (orchestrator only) |

## Graduation routing

```
Graduation routing (for orchestrator review — why PROBATION, not COMPLETE):
  CI verdict:          PASS-RELIABLE | INSPECT(n) | FAIL (pending live GitHub Doctrine Scan on PR #1073)
  Triage entries:      none
  Risk class:          kernel/driver GPU buffer sizing + scale-proof
  Falsification check: Verify the 1500-star scale test no longer has a catch_unwind-swallow passing path;
                       verify upload_reduction_topology no longer overruns the 24-byte column_rules buffer;
                       verify install_atomic still exercises the accepted RF budget at 7505-slot scale;
                       verify no Phase 1 content, semantic runtime leakage, new AccumulatorRole,
                       per-tick allocation, atlas scheduler, or scanner/allowlist edits.
  Recommended posture: deep — repairs a false-green scale gate and touches GPU/session capacity behavior
                       inherited by later Terran-Pirate rungs.
```

## Known gaps / next

- Orchestrator adjudication: confirm full `SimSession::open_from_spec` + `mapping.is_none()` on a capable GPU, or scope a follow-on sizing rung if velocity op upload at budget headroom is the remaining scale defect.
- Phase 1+ remains blocked behind scale-envelope closeout.