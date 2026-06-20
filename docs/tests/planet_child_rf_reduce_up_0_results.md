# PLANET-CHILD-RF-REDUCE-UP-0 Results

> **Lifecycle: PROBATION** — scoped local RF reduce-up over planet participants. Full owner-silo state mutation and disburse-down remain deferred. Pending owner DA approval.

## Status

**PASS** — focused validation complete; GPU proof REAL_ADAPTER_OBSERVED for all buckets when adapter available.

## PR / branch / merge

| Item | Value |
|------|-------|
| Branch | `planet-child-rf-reduce-up-0` |
| PR | #796 — PLANET-CHILD-RF-REDUCE-UP-0 |
| Merge | `c1eb325c9462e0d1f1ed54e0fb2fca7e1fb29adc` |
| Base | `master` after #795 (`17a1ee8b`) |

## Mission

Group admitted planet gridcell and non-grid child RF participants into scoped owner/resource/planet buckets and compute local surplus/deficit reduce-up summaries without mutating scenario authority.

## Constitutional alignment

- Owners remain GameSession children / RF channel scopes, not spatial parents.
- Planet-local scope defaults via `planet_id`; star-system id derived from spatial path metadata only.
- Recursive local-grid doctrine from #790–#792 preserved.
- GPU output is proof/cache only; scenario authority is not mutated.

## Implemented changes

- **simthing-spec** `planet_child_rf.rs`: `PlanetChildRfScopeKey`, `PlanetChildRfReduceUpBucket`, `PlanetChildRfReduceUpReport`, `evaluate_planet_child_rf_reduce_up`, `scope_key_from_participant`.
- **simthing-spec** `scenario_ingestion.rs`: `ScenarioIngestionResult.planet_child_rf_reduce_up`, `compile_readiness.planet_child_rf_reduce_up_ready`.
- **simthing-driver** `planet_child_rf_reduce_up_compile.rs`: per-bucket `compile_planet_child_rf_reduce_up_gpu_proof_plan` and CPU/GPU tick helpers.
- **Corpus** `scenarios/corpus/planet_child_rf_reduce_up_scoped.simthing-scenario.json`.
- **Tests** `planet_child_rf_reduce_up.rs` in simthing-spec (12 tests) and simthing-driver (8 tests).

## Scoped reduce-up model

- Bucket key: `(owner_ref, resource_key="generic", planet_id, star_system_gridcell_id_raw)`.
- Canonical fixture: `owner_a` / `terra_prime` (3 participants, surplus 20, deficit 8, net_surplus 12); `owner_b` / `border_moon` (1 participant, surplus 7, deficit 2, net_surplus 5).
- Same star system, different owners remain separate buckets.
- Checked arithmetic rejects overflow fail-closed.

## CPU oracle proof

- Per-bucket surplus/deficit totals match participant sums.
- `execute_accumulator_plan_tick_cpu` aggregate slot matches bucket totals for all buckets.

## GPU proof path

- Per-bucket `CompiledAccumulatorOpPlan` surplus/deficit sum plans via existing `compile_participant_channel_sum_plan`.
- All buckets GPU-proven when adapter available (`planet_child_rf_reduce_up_gpu_each_bucket_matches_cpu_when_adapter_available`).
- `full_state_mutation_deferred: true`.
- GPU evidence: **REAL_ADAPTER_OBSERVED** when adapter available; honest SKIP otherwise.

## Boundary / non-goals

| Check | Status |
|-------|--------|
| No new GPU primitive/WGSL | PASS |
| No planet/economy/combat/orbit engine | PASS |
| No sim state mutation | PASS |
| No disburse-down | PASS |
| No Studio GPU dispatch | PASS |
| No Terran Pirate fixture edits | PASS |
| Studio reduce-up display deferred | PASS (documented) |

## Validation commands

| Command | Status |
|---------|--------|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test planet_child_location_admission` | PASS (25) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS (12) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18) |
| `cargo test -p simthing-spec --test planet_child_rf_reduce_up` | PASS (11; 1 ignored) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test planet_child_rf_gpu_tick` | PASS (10; 1 ignored) |
| `cargo test -p simthing-driver --test planet_child_rf_reduce_up` | PASS (8) |
| `cargo test -p simthing-driver --test owner_silo_gpu_tick` | PASS (11) |
| `git diff --check` | PASS |
| `git diff --name-only master...HEAD` | PASS (12 files) |
| `cargo test` (all packages) | SKIP — focused validation only |

## Files changed

- `crates/simthing-spec/src/spec/planet_child_rf.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/tests/planet_child_rf_reduce_up.rs` (new)
- `crates/simthing-driver/src/planet_child_rf_reduce_up_compile.rs` (new)
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/planet_child_rf_reduce_up.rs` (new)
- `scenarios/corpus/planet_child_rf_reduce_up_scoped.simthing-scenario.json` (new)
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/planet_child_rf_reduce_up_0_results.md`

## Evidence lifecycle

| Artifact | Classification |
|----------|----------------|
| `docs/tests/planet_child_rf_reduce_up_0_results.md` | PROBATION |
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER update |
| `docs/design_0_0_8_3_studio_production.md` | living production synthesis |
| `scenarios/corpus/planet_child_rf_reduce_up_scoped.simthing-scenario.json` | durable corpus fixture |

## Known gaps

- Full owner-silo state mutation / writeback remains deferred.
- Disburse-down remains deferred.
- Studio `planet_child_rf_reduce_up` display deferred; ingestion computes report.

## Deferred next rung

- Owner-silo state mutation / writeback.
- Disburse-down after mutation/writeback.
- Star-system local-grid GPU operators.
- Fleet movement/combat execution.

## DA status

**N/A** — not DA-promoted.