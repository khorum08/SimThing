# RECURSIVE-LOCAL-RF-GPU-RESIDENCY-REMEDIATION-0R Results

## Status

PASS

## PR / branch / merge

- Branch: `recursive-local-rf-gpu-residency-remediation-0r`
- PR: TBD
- Merge SHA: TBD

## Mission

Repair recursive local RF aggregate proof coverage and reassert maximal GPU-residency doctrine. Include both direct participant rows and child Location outputs in per-arena AccumulatorOp proof sources; assert CPU/GPU aggregates match authoritative recursive settlement totals.

## Pre-flight metadata check

- #807 metadata verified on `master`: PR #807, merge `b5e0b611487016f89400f091eb8a56af1eced4d2`; no `TBD` placeholders in evidence index, result report, or production doc for #807.

## GPU-residency drift audit

#807 correctly implemented the recursive CPU oracle/shadow evaluator and preserved the previous RF/effect ladder.

The drift risk is authority framing and proof coverage:

- The recursive evaluator must not become a CPU-owned runtime simulation path.
- CPU may provide deterministic oracle/reference validation, semantic-side bookkeeping tables, compile-plan construction, and owner/user-facing reports.
- Runtime execution direction must remain maximal GPU residency through flat rows/tables and AccumulatorOp-compatible proof surfaces.
- The #807 driver aggregate proof selected only direct participant rows, while authoritative recursive settlement totals include direct participant rows plus child Location outputs.

## Proof gap found

Driver `compile_recursive_rf_aggregate_proof_plans` indexed only `arena.participant_rows` for AccumulatorOp inputs. Authoritative settlement totals are `total_surplus = direct_surplus_total + child_surplus_total` and `total_demand = direct_demand_total + child_deficit_total`. Child Location output rows were excluded from proof inputs.

## Proof gap closed

- Added `RecursiveLocalRfAggregateSourceRow` / `RecursiveLocalRfAggregateSourceKind` and `recursive_local_rf_aggregate_source_rows()` flattening direct participants and child Location outputs into GPU-compatible flat table rows.
- `RecursiveLocalRfPlan` now carries `aggregate_source_rows`; proof `source_indices` reference the full source table per arena/owner/resource.
- `recursive_local_rf_surplus_tick_inputs` / `recursive_local_rf_demand_tick_inputs` read surplus/demand from aggregate source rows (direct surplus/demand plus child net_surplus/net_deficit).
- GPU aggregate test asserts CPU/GPU AccumulatorOp results match `recursive_local_rf_cpu_surplus_total` / `recursive_local_rf_cpu_demand_total` settlement oracle totals.

## CPU role boundary

This remediation does not grant CPU production simulation authority. The recursive CPU evaluator remains a deterministic oracle/reference and semantic shadow table used to validate the GPU-compatible RF aggregate source rows. Runtime direction remains maximal GPU residency: recursive RF inputs are flattened into arena/owner/resource/source rows suitable for AccumulatorOp-compatible proof surfaces. CPU responsibilities remain Scenario ingestion/validation, compile-plan construction, deterministic oracle comparison, semantic-side bookkeeping, and owner/user-facing reports.

## GPU-compatible source table

- `RecursiveLocalRfAggregateSourceKind::DirectParticipant` rows carry participant surplus/demand.
- `RecursiveLocalRfAggregateSourceKind::ChildLocationOutput` rows carry child net_surplus/net_deficit.
- Rows are deterministically ordered by arena, owner, resource_key, source_kind, source id.
- Checked totals in tests confirm source-row sums equal settlement `total_surplus` / `total_demand`.

## Prior ladder compatibility proof

- `recursive_local_rf_preserves_previous_planet_child_rf_ladder_outputs` — PASS
- `recursive_local_rf_preserves_owner_silo_disburse_down_fixture_behavior` — PASS
- `recursive_local_rf_coexists_with_local_effect_application_without_changing_totals` — PASS
- `recursive_local_rf_coexists_with_semantic_local_effects_without_changing_totals` — PASS
- `recursive_local_rf_compile_does_not_alter_semantic_local_effects_totals` — PASS
- `recursive_local_rf_does_not_replace_tick_shell_rf_source` — PASS

## Resource-key / generic fallback proof

- `recursive_local_rf_supports_explicit_resource_key_metadata` — PASS
- `recursive_local_rf_preserves_generic_resource_key_fallback` — PASS
- `recursive_local_rf_aggregate_sources_preserve_resource_key` — PASS
- `recursive_local_rf_aggregate_sources_preserve_generic_fallback` — PASS

## Authority preservation proof

- `prove_recursive_local_rf_preserves_authority` — PASS
- `recursive_local_rf_proof_remediation_preserves_scenario_authority` — PASS
- `recursive_local_rf_proof_remediation_does_not_mutate_participant_properties` — PASS

## GPU proof path

- `recursive_local_rf_gpu_aggregate_matches_cpu_when_adapter_available` — PASS (REAL_ADAPTER_OBSERVED)
- `recursive_local_rf_gpu_skips_honestly_without_adapter` — PASS (SKIP path when no adapter)

## Boundary / non-goals

- No tick-shell RF source replacement.
- No semantic effect execution.
- No participant property mutation.
- No ScenarioSpec mutation.
- No savefile or persistent timeline mutation.
- No new GPU primitive or WGSL.
- No fused recursive RF kernel.
- No Studio GPU dispatch.
- No MapGenerator/ClauseThing/Terran Pirate fixture edits.

## Validation commands

| Command | Result |
|---------|--------|
| `cargo fmt --all -- --check` | PASS (after `cargo fmt --all`) |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test recursive_local_rf` | PASS (23 tests) |
| `cargo test -p simthing-spec --test planet_child_location_admission` | PASS |
| `cargo test -p simthing-spec --test runtime_local_allocation` | PASS |
| `cargo test -p simthing-spec --test local_effect_application` | PASS |
| `cargo test -p simthing-spec --test semantic_local_effects` | PASS |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test recursive_local_rf` | PASS (19 tests) |
| `cargo test -p simthing-driver --test local_effect_application` | PASS (11 tests) |
| `cargo test -p simthing-driver --test semantic_local_effects` | PASS (12 tests) |
| `cargo test -p simthing-driver --test owner_silo_gpu_tick` | PASS |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/recursive_local_rf.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/recursive_local_rf.rs`
- `crates/simthing-driver/src/recursive_local_rf_compile.rs`
- `crates/simthing-driver/tests/recursive_local_rf.rs`
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/recursive_local_rf_gpu_residency_remediation_0r_results.md`

## Evidence lifecycle

PROBATION — not DA-promoted.

## Known gaps

- Recursive RF evaluator not yet integrated into runtime tick shell as GPU-resident RF source.
- Planet-child RF ladder not yet reconciled with recursive evaluator outputs.
- Semantic effect execution remains deferred.

## Deferred next rung

1. Reconcile planet-child RF ladder with recursive local RF evaluator outputs.
2. Integrate recursive local RF evaluator into runtime tick shell as optional GPU-resident RF source.
3. Semantic effect execution authority remains deferred until recursive RF evaluator is integrated into tick shell.
4. Runtime tick persistent history/replay storage remains deferred.
5. Star-system local-grid GPU operators remain deferred.
6. Fleet movement/combat remains deferred.
7. Studio presentation of recursive RF proof reports remains deferred.

## DA status

Not DA-promoted.