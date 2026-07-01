# TP-SCALE-ENVELOPE-0 / 0R / 0R2 Results

## Status

**PROBATION** - `TP-SCALE-ENVELOPE-0` remains **HELD** for orchestrator review. Repair rung `TP-SCALE-ENVELOPE-0R2` updates PR #1073 on branch `tp-scale-envelope-0`. The swallowed-panic false-green path remains removed; the 0R reduction-topology repair remains intact; 0R2 repairs the next live-adapter velocity-op upload failure exposed by the 1500-star session proof.

## Original HOLD

`TP-SCALE-ENVELOPE-0` proved generate -> lattice -> RF-budget -> link -> `install_atomic` at 1500-star scale, but the terminal exit proof was false-green:

- On a real adapter, `SimSession::open_from_spec` panicked in `initial_gpu_sync -> WorldGpuState::upload_reduction_topology` (`Queue::write_buffer` overran a **24-byte** `column_rules` destination buffer).
- The test used `catch_unwind`, printed `skipping live adapter session proof; initial_gpu_sync reduction-topology upload rejected the scale shape`, and returned green without reaching `assert!(session.mapping.is_none())`.

Root cause: `WorldGpuState::rebuild_for_slots` grew `n_slots` and `n_dims` together but did **not** reallocate `column_rules` (sized only at `WorldGpuState::new` from the placeholder registry). `rebuild_for_registry` already resized `column_rules`; the slot-growth path did not.

## 0R repair

- **`crates/simthing-kernel/src/world_state.rs`:** `rebuild_for_slots` reallocates `column_rules` (and resets reduction sidecar buffers when the value-preservation path does not apply) whenever slot capacity grows with a widened registry.
- **`crates/simthing-kernel/src/accumulator_op/runtime.rs`:** `ensure_velocity_session` recreates the session when `n_slots` / `n_dims` no longer match (stale-session guard after shape sync).
- **`crates/simthing-clausething/tests/tp_scale_envelope.rs`:** removed the `catch_unwind` swallow-and-return path; session open is a direct `expect("open TP scale session")` plus `assert!(session.mapping.is_none())`.

## 0R2 repair

The 0R2 reproduction reached the next failure:

- Stack: `SimSession::open_from_spec -> initial_gpu_sync -> sync_gpu_buffers -> upload_velocity_ops_with_bands -> WorldAccumulatorRuntime::upload_velocity_ops -> AccumulatorOpSession::write_op_bytes -> Queue::write_buffer`.
- Shape at failing upload: `n_slots=7505`, `n_dims=43773`, `governed_pairs=14585`, `emission_capacity=4096` for the velocity session, and RF `budget_gpu_slots=7505`.
- Old velocity planner materialized one op per `(slot, governed pair)`: `109,460,425` ops.
- Old upload byte count: `10,946,042,500` bytes into a freshly grown velocity op buffer, which failed locally as `Queue::write_buffer: Not enough memory left`.

Repair:

- **`crates/simthing-kernel/src/velocity_accumulator.rs`:** `plan_velocity_integration` now uploads one compact C-7 op per governed pair and stores the slot span in `source_count`; the band-targeted governed planner remains expanded so E-11/order-band semantics are not changed.
- **`crates/simthing-kernel/src/shaders/accumulator_op.wgsl`:** velocity dispatch expands compact pair ops across slots on GPU invocations, preserving amount integration and velocity pinning behavior.
- **`crates/simthing-kernel/src/accumulator_op/session.rs`:** velocity encoding submits compact dispatches in bounded chunks and op/input-list upload byte sizing uses checked arithmetic with typed session errors.
- **`crates/simthing-kernel/src/velocity_accumulator.rs`:** `plan_velocity_integration_compacts_scale_upload` pins the TP-scale shape to pair-count upload: under 2 MB compact, over 10 GB if re-expanded.

## 0R2 parity proof

The existing CPU-oracle velocity parity tests pass against the compact velocity execution path:

- `velocity_integration_matches_cpu_oracle_dt_one`
- `velocity_integration_matches_cpu_oracle_fractional_dt`

## Terminal session proof

Real-adapter `SimSession::open_from_spec` succeeded; `session.mapping.is_none()` asserted.

Preserved legs unchanged: seed `770421`, 1500-star connected disc, lattice hierarchy, structural frame above dense-field cap, budgeted RF admission (3000 participants / 2 arenas), typed atlas deferral, `install_atomic` through 7505-slot footprint.

## Load-bearing proofs

| Check | Result |
|---|---|
| `cargo check -p simthing-kernel` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo check -p simthing-clausething` | PASS |
| `cargo test -p simthing-kernel rebuild_for_slots_expands_column_rules_when_dims_grow` | PASS |
| `cargo test -p simthing-kernel velocity_integration_matches_cpu_oracle -- --nocapture` | PASS (dt=1.0 and fractional dt) |
| `cargo test -p simthing-kernel plan_velocity_integration_compacts_scale_upload` | PASS |
| `cargo test -p simthing-mapgenerator --test topology_stead` | PASS (9/9) |
| `cargo test -p simthing-mapgenerator --test connectivity` | PASS (7/7) |
| `cargo test -p simthing-clausething --test tp_scale_envelope tp_scale_envelope_disc_1500_admits_installs_with_budget -- --nocapture` | PASS (real adapter; terminal `mapping.is_none()` assertion reached) |
| `bash scripts/ci/gen_digest.sh --check` | PASS |
| `bash scripts/ci/doctrine_scan.sh` | PASS - failures=0 inspect=0 |

## INSPECT / triage

None for this repair rung (`scripts/ci/triage_log.tsv` has no `TP-SCALE-ENVELOPE-0R2` rows).

## Scope Ledger

| Element | State |
|---|---|
| Remove catch_unwind false-green session path | preserved |
| Fix reduction-topology `column_rules` sizing on slot growth | preserved |
| Repair velocity op upload at 7505-slot / 14585-pair shape | implemented |
| CPU-oracle velocity parity for compact GPU execution | passed (dt=1.0 and fractional dt) |
| Preserve 1500-star / RF-budget / install_atomic legs | implemented |
| Terminal `mapping.is_none()` on local adapter | reached and asserted |
| Phase 1 content / scanners / allowlists / new AccumulatorRole | held (untouched) |
| `TP-SCALE-ENVELOPE-0` self-marked COMPLETE | held (orchestrator only) |

## Graduation routing

```
Graduation routing (for orchestrator review - why PROBATION, not COMPLETE):
  CI verdict:          PASS-RELIABLE
  Triage entries:      none
  Risk class:          kernel/WGSL compact velocity parity + scale-proof
  Falsification check: Verify compact velocity GPU execution matches CPU oracle for dt=1.0 and fractional dt; verify real-adapter SimSession::open_from_spec reaches mapping.is_none(); verify compact upload stays bounded; verify no Phase 1 content, new AccumulatorRole, per-tick allocation, scanner/allowlist edit, or catch_unwind false-green path.
  Recommended posture: deep - this is the final semantic-preservation proof for a WGSL/kernel execution change inherited by later Terran-Pirate rungs.
```

## Known gaps / next

- PR #1073 remains PROBATION and must not merge without orchestrator review.
- Phase 1+ remains blocked behind scale-envelope closeout.
