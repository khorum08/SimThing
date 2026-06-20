# OWNER-SILO-RUNTIME-WRITEBACK-0 Results

> **Lifecycle: PROBATION** — runtime owner-silo writeback from scoped reduce-up. Scenario authority unchanged; disburse-down deferred.

## Status

**PASS** — focused validation complete; GPU aggregate proof REAL_ADAPTER_OBSERVED when adapter available.

## PR / branch / merge

| Item | Value |
|------|-------|
| Branch | `owner-silo-runtime-writeback-0` |
| PR | #797 — OWNER-SILO-RUNTIME-WRITEBACK-0 |
| Merge | `3508d5789a3acd23364d466eef18331e2d95d135` |
| Base | `master` after #796 (`7aaeab1f`) |

## Mission

Apply scoped planet child RF reduce-up bucket net surplus/deficit into runtime-resident owner-silo state without mutating Scenario authority.

## Constitutional alignment

- Scenario SimThing remains serializable authority; writeback mutates runtime oracle state only.
- Owners remain GameSession children / RF channel scopes, not spatial parents.
- Planet-local buckets aggregate to owner/resource channels for writeback.
- Disburse-down and economic allocation remain deferred.

## Implemented changes

- **simthing-spec** `owner_silo_runtime_writeback.rs`: runtime types, `runtime_owner_silo_states_from_scenario`, `owner_silo_writeback_inputs_from_planet_child_reduce_up`, `apply_owner_silo_runtime_writeback_cpu`.
- **simthing-driver** `owner_silo_runtime_writeback_compile.rs`: `compile_owner_silo_runtime_writeback_plan`, GPU aggregate proof plans per owner/resource.
- **Ingestion** `owner_silo_runtime_writeback_ready` / `owner_silo_runtime_writeback_deferred` compile-readiness flags.
- **Tests** 4 spec + 13 driver tests; reuses `planet_child_rf_reduce_up_scoped` corpus fixture.

## Runtime writeback model

- Initial state from Owner `owner_silo_current` / `owner_silo_capacity` metadata.
- Planet-local buckets aggregate by `(owner_ref, resource_key)` before writeback.
- Canonical fixture: `owner_a` 50→62 (+12 net), `owner_b` 40→45 (+5 net).
- Capacity clamp and unmet deficit recorded deterministically.

## CPU oracle proof

- Checked arithmetic; no underflow; overflow rejects fail-closed.
- Clamp test: current 95 + net_surplus 12 @ capacity 100 → next 100, clamped_surplus 7.
- Unmet deficit test: current 5 − net_deficit 10 → next 0, unmet_deficit 5.

## GPU proof path

- GPU proof covers owner/resource aggregate net surplus/deficit sums via existing AccumulatorOp Sum-over-INPUT_LIST.
- CPU oracle applies runtime writeback semantics (clamp, unmet deficit).
- GPU evidence: **REAL_ADAPTER_OBSERVED** when adapter available; honest SKIP otherwise.

## Boundary / non-goals

| Check | Status |
|-------|--------|
| Runtime state only; no ScenarioSpec mutation | PASS |
| No new GPU primitive/WGSL | PASS |
| No disburse-down | PASS |
| No Studio GPU dispatch | PASS |
| No Terran Pirate fixture edits | PASS |
| Studio writeback display deferred | PASS (documented) |

## Validation commands

| Command | Status |
|---------|--------|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test planet_child_rf_reduce_up` | PASS (11; 1 ignored) |
| `cargo test -p simthing-spec --test owner_silo_runtime_writeback` | PASS (4) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS (12) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test planet_child_rf_reduce_up` | PASS (8) |
| `cargo test -p simthing-driver --test owner_silo_runtime_writeback` | PASS (13) |
| `cargo test -p simthing-driver --test owner_silo_gpu_tick` | PASS (11) |
| `git diff --check` | PASS |
| `git diff --name-only master...HEAD` | PASS (13 files) |
| `cargo test` (all packages) | SKIP — focused validation only |

## Files changed

- `crates/simthing-spec/src/spec/owner_silo_runtime_writeback.rs` (new)
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/tests/owner_silo_runtime_writeback.rs` (new)
- `crates/simthing-spec/tests/reduce_up_fixture.rs` (new)
- `crates/simthing-driver/src/owner_silo_runtime_writeback_compile.rs` (new)
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/owner_silo_runtime_writeback.rs` (new)
- `crates/simthing-driver/tests/reduce_up_fixture.rs` (new)
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/owner_silo_runtime_writeback_0_results.md`

## Evidence lifecycle

| Artifact | Classification |
|----------|----------------|
| `docs/tests/owner_silo_runtime_writeback_0_results.md` | PROBATION |
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER update |
| `docs/0.8.3 Simthing Studio Production.md` | living production synthesis |
| `scenarios/corpus/planet_child_rf_reduce_up_scoped.simthing-scenario.json` | reused corpus fixture |

## Known gaps

- Disburse-down / local allocation remains deferred.
- Runtime tick integration (resident sim state loop) remains deferred.
- Studio runtime writeback preview display deferred.

## Deferred next rung

- Owner-silo disburse-down / local allocation.
- Runtime tick integration after writeback/disburse-down boundary proof.
- Star-system local-grid GPU operators.
- Fleet movement/combat execution.

## DA status

**N/A** — not DA-promoted.