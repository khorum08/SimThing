# KERNEL-EMISSION-SEAL-0 Results

## Status

**PROBATION** — decision/emission record types are constructor-sealed; external crates cannot struct-literal or named-constructor forge authoritative decision output. DA re-review required before DONE.

## PR / branch / merge

- Branch: `codex/kernel-emission-seal-0` (+ `codex/kernel-emission-seal-0r` remediation)
- PR: https://github.com/khorum08/SimThing/pull/990 (+ https://github.com/khorum08/SimThing/pull/992 0R)
- Merge: `cb3a509077` (0), `edc276495a` (0R remediation)

## What changed

- Sealed `EmissionRecord` and `ThresholdEmission` in `accumulator_op/types.rs`: private fields, public accessors, `pub(crate)` kernel/readback/oracle minters.
- Split `ThresholdEvent` from GPU transport: `ThresholdEventGpu` (`#[repr(C)]` Pod for Pass 7 buffer) vs sealed `ThresholdEvent` (authoritative decision record with accessors).
- Kernel session readback, CPU oracle, and emission accumulator paths mint records only through named constructors.
- Added `cpu_oracle_emission_records()` for driver parity burn-in (oracle twin inside `simthing-gpu`).
- Migrated call sites in `simthing-sim`, `simthing-driver`, and `simthing-spec` to accessors and sanctioned constructors.

## 0R remediation — public named-constructor forge vector

**DA hold source:** public `ThresholdEvent::from_boundary_delivery(slot, col, value, event_kind)` minted arbitrary authoritative events from raw primitives.

**Exact fix:**
- Removed public `from_boundary_delivery` entirely (dead after migration).
- Added public `cpu_oracle_threshold_events(...)` — CPU-oracle twin that mints events only when buffer state crosses registered thresholds (not arbitrary tuples).
- Added named-constructor `compile_fail` doctests on `ThresholdEvent`, `EmissionRecord`, and `ThresholdEmission`.
- Migrated unit/integration test fixtures to CPU-oracle crossings via `threshold_event_test_fixtures` (`#[cfg(test)]` in `simthing-sim`) or inline oracle helpers.

**Constructor audit (post-0R):**

| Type | Public minters returning sealed type | Verdict |
|---|---|---|
| `EmissionRecord` | None (only accessors public); `cpu_oracle_emission_records` computes from flat + registrations | OK — not raw forge |
| `ThresholdEmission` | None | OK |
| `ThresholdEvent` | None; `cpu_oracle_threshold_events` requires threshold crossings | OK — not raw forge |

**0R proofs rerun:** `cargo test -p simthing-gpu --doc` (17 compile_fail incl. named constructors), `cargo test -p simthing-gpu threshold --lib`, `cargo test -p simthing-sim --test s6_threshold_sunset --test c1_threshold_perf --test c8d_emission_accumulator_parity`, fission/boundary/pr10 tests green.

**0R scope:** `world_state.rs`, `types.rs`, `passes.rs`, `lib.rs`, `threshold_event_test_fixtures.rs`, test call sites in `simthing-sim` / `simthing-driver` / `simthing-spec`.

## Sealed emission surfaces

| Type | Public fields | External struct literal | Sanctioned minters |
|---|---|---|---|
| `EmissionRecord` | None (accessors only) | `compile_fail` | `from_kernel_emit_event`, `from_gpu_readback`, `from_cpu_oracle` (`pub(crate)`); `cpu_oracle_emission_records()` for driver oracle |
| `ThresholdEmission` | None (accessors only) | `compile_fail` | `from_kernel_emit_event`, `from_gpu_readback`, `from_cpu_oracle` (`pub(crate)`) |
| `ThresholdEvent` | None (accessors only) | `compile_fail` (struct + named) | `from_kernel_pass7_readback`, `from_gpu_readback` (`pub(crate)`); production via kernel readback; tests via `cpu_oracle_threshold_events` |
| `EmissionRecordGpu`, `ThresholdEmissionGpu`, `ThresholdEventGpu`, `ThresholdRegistration`, `AccumulatorOpGpu` | Public POD layout | N/A — transport only, not authoritative decision records |

## Sanctioned channels preserved

`ThresholdRegistration` / packed threshold upload → GPU threshold scan or `EmitEvent` op → readback from kernel-owned emission buffer → sealed `EmissionRecord` / `ThresholdEmission` / `ThresholdEvent` → `BoundaryProtocol` / `BoundaryRequest` handling.

CPU-oracle twin: `from_cpu_oracle` / `cpu_oracle_emission_records` inside `simthing-gpu`; driver delegates via `resource_economy_oracle`.

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `EmissionRecord` `compile_fail` rustdoc (`types.rs`) | External CPU-side `EmissionRecord { … }` forgery |
| `ThresholdEmission` `compile_fail` rustdoc (`types.rs`) | External CPU-side threshold emission forgery |
| `ThresholdEvent` `compile_fail` rustdoc (struct literal + `from_boundary_delivery`) | External struct-literal and named-constructor forgery |
| `EmissionRecord` / `ThresholdEmission` named-constructor `compile_fail` | External pub(crate) minter reach via public API |
| `cargo test -p simthing-gpu --lib threshold` (18 tests) | Kernel threshold scan + CPU-oracle parity unchanged |
| `cargo test -p simthing-sim --test s6_threshold_sunset --test c1_threshold_perf --test c8d_emission_accumulator_parity` | End-to-end threshold/emission readback + perf smoke |
| `cargo test -p simthing-sim as_sim_semantic_free_public_surface_audit --lib` | Semantic-free surface audit still green |
| `cargo test -p simthing-sim as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads --lib` | No runtime kind reads introduced |

## Value parity

No threshold semantics, event ordering, or `event_kind` mapping change intended. Existing GPU↔CPU-oracle parity tests green (`c1_threshold_gpu_matches_cpu_oracle`, `c8d_eval_eml_exact_emission_matches_cpu_oracle`, `passes::tests::threshold_scan_matches_cpu_oracle`, etc.).

## Performance parity

Zero-cost by construction: private fields + accessor inlining only; no runtime branch, heap allocation, dynamic dispatch, or side registry. `c1_threshold_perf::c1_accumulator_threshold_readback_smoke` green — no measured regression vs pre-seal baseline.

## Scope Ledger

| File / area | Why touched |
|---|---|
| `crates/simthing-gpu/src/accumulator_op/types.rs` | Seal `EmissionRecord`, `ThresholdEmission` + `compile_fail` docs |
| `crates/simthing-gpu/src/world_state.rs` | Split `ThresholdEventGpu` / sealed `ThresholdEvent`; readback mint path |
| `crates/simthing-gpu/src/accumulator_op/session.rs` | Readback/oracle use sealed constructors |
| `crates/simthing-gpu/src/accumulator_op/cpu_oracle.rs` | Oracle mint path |
| `crates/simthing-gpu/src/emission_accumulator.rs` | `cpu_oracle_emission_records()` |
| `crates/simthing-gpu/src/passes.rs` | Accessor migration in tests |
| `crates/simthing-gpu/src/lib.rs` | Re-exports |
| `crates/simthing-sim/src/{boundary,fission,legacy_oracle,property_expiry,threshold_registry}.rs` | Accessor + `from_boundary_delivery` migration |
| `crates/simthing-sim/tests/**` | Test fixture migration |
| `crates/simthing-driver/**` | Oracle delegation + accessor migration |
| `crates/simthing-spec/tests/pr10_scripted_event_thresholds.rs` | Test fixture migration |
| `docs/tests/kernel_emission_seal_0_results.md` | This ledger |
| `docs/tests/current_evidence_index.md` | Index row |
| `docs/design_0_0_8_4_5_simthing_kernel.md` | Rung 4 OPEN → PROBATION |

**Not touched:** participation seal, `simthing-kernel` crate extraction, `deny.toml`, `design_0_0_8_5`, new dependencies, threshold behavior redesign.

## Known gaps / next

- DA re-review: PROBATION → DONE.
- **`KERNEL-PARTICIPATION-SEAL-0`** — next seal rung (spatial arena placement proof).
- Raw GPU POD types remain public byte-layout mirrors; consumers must treat sealed records as authoritative, not POD casts (future crate boundary may tighten further at extraction).
