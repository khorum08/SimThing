# PLANET-CHILD-RF-GPU-PARTICIPANT-0 Results

> **Lifecycle: PROBATION** — planet/non-grid child RF participant GPU proof landed. Full owner-silo state mutation remains deferred. Pending owner DA approval.

## Status

**PASS** — focused validation complete; GPU proof REAL_ADAPTER_OBSERVED when adapter available.

## PR / branch / merge

| Item | Value |
|------|-------|
| Branch | `planet-child-rf-gpu-participant-0` |
| PR | PENDING (pre-merge) |
| Merge | PENDING (pre-merge) |
| Base | `master` after #794 (`8773cdcb`) |

## Mission

Connect admitted planet gridcells and planet non-grid children to existing owner-silo AccumulatorOp GPU proof path via metadata-driven RF participant rows.

## Constitutional alignment

- Owners remain GameSession children / RF channel scopes, not spatial parents.
- Planet gridcells and non-grid children contribute via metadata/properties/columns only.
- Recursive local-grid doctrine from #790/#791/#792 preserved.
- GPU output is proof/cache only; scenario authority is not mutated.

## Implemented changes

- **simthing-spec** `planet_child_rf.rs`: `PlanetChildRfParticipantInput`, `evaluate_planet_child_rf_admission`, `planet_child_rf_participant_inputs`; ingestion `ScenarioIngestionResult.planet_child_rf` and compile-readiness field.
- **simthing-driver** `planet_child_rf_accumulator_compile.rs`: `compile_planet_child_rf_gpu_tick_plan`, tick input helpers; reuses `compile_participant_channel_sum_plan` from owner-silo compile path.
- **Corpus** `scenarios/corpus/planet_child_rf_participants_admitted.simthing-scenario.json`.
- **Tests** `crates/simthing-driver/tests/planet_child_rf_gpu_tick.rs` (11 tests; 10 PASS, 1 ignored corpus writer).

## Participant admission model

- Planet gridcell participants use `planet_owner_ref` / `owner_flow_owner_ref` plus `owner_flow_surplus` / `owner_flow_deficit`.
- Non-grid child participants (Cohort, Fleet, Infrastructure) use `planet_non_grid_child_owner_ref` plus owner-flow surplus/deficit metadata.
- Active RF without owner/channel metadata rejects fail-closed (`MissingOwnerChannelForActiveRfParticipant`).
- Malformed surplus/deficit amounts reject fail-closed (`InvalidPlanetChildRfAmount`).
- Canonical fixture: 4 participants (1 planet gridcell + 3 non-grid children), surplus total 30, deficit total 13, owner channel `owner_a`.

## GPU-resident proof path

- Driver compiles two `CompiledAccumulatorOpPlan` values (surplus sum + deficit sum) with one aggregate slot.
- CPU oracle via `execute_accumulator_plan_tick_cpu`; GPU via `SimGpuAccumulatorTickState` + scoped `ProofReadback`.
- `full_state_mutation_deferred: true` on compile plan.
- GPU evidence: **REAL_ADAPTER_OBSERVED** when adapter available; honest SKIP otherwise.

## Boundary / non-goals

| Check | Status |
|-------|--------|
| No new GPU primitive/WGSL | PASS |
| No planet/economy/combat/orbit engine | PASS |
| No sim state mutation | PASS |
| No Studio GPU dispatch | PASS |
| No Terran Pirate fixture edits | PASS |
| No MapGenerator/ClauseThing changes | PASS |

## Validation commands

| Command | Status |
|---------|--------|
| `cargo fmt --all -- --check` | PASS (auto-fixed, re-checked) |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test planet_child_location_admission` | PASS (25) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS (12) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test planet_child_location_structural_readiness` | PASS (6) |
| `cargo test -p simthing-driver --test owner_silo_gpu_tick` | PASS (11) |
| `cargo test -p simthing-driver --test planet_child_rf_gpu_tick` | PASS (10; 1 ignored) |
| `git diff --check` | PENDING (pre-commit) |
| `git diff --name-only master...HEAD` | PENDING (pre-commit) |
| `cargo test` (all packages) | SKIP — focused validation only |

## Files changed

- `crates/simthing-spec/src/spec/planet_child_rf.rs` (new)
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-driver/src/planet_child_rf_accumulator_compile.rs` (new)
- `crates/simthing-driver/src/owner_silo_accumulator_compile.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/planet_child_rf_gpu_tick.rs` (new)
- `scenarios/corpus/planet_child_rf_participants_admitted.simthing-scenario.json` (new)
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/planet_child_rf_gpu_participant_0_results.md`

## Evidence lifecycle

| Artifact | Classification |
|----------|----------------|
| `docs/tests/planet_child_rf_gpu_participant_0_results.md` | PROBATION |
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER update |
| `docs/0.8.3 Simthing Studio Production.md` | living production synthesis |
| `scenarios/corpus/planet_child_rf_participants_admitted.simthing-scenario.json` | durable corpus fixture |

## Known gaps

- Full owner-silo state mutation (reduce-up/disburse-down writes) remains deferred.
- Studio presentation of `planet_child_rf` ingestion counts deferred (ingestion computes them; display not added).
- Local RF resolution / surplus-deficit reduce-up remains next after participant GPU proof.

## Deferred next rung

- Full owner-silo state mutation.
- Local RF resolution / surplus-deficit reduce-up.
- Star-system local-grid GPU operators.
- Fleet movement/combat execution.

## DA status

**N/A** — not DA-promoted.