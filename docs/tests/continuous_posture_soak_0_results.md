# CONTINUOUS-POSTURE-SOAK-0 results

- Track: 0.0.8.7 RF arena modernization (rung 6.3)
- Status: **PROBATION / proof-present / DA-review-pending**
- Implementation base: `63d6fc444bef8f4bff4b3c8bead7d43780c9a69f`
- ORIENT-RECEIPT: `6af1884543b0`
- orientation_rule_stamp: `5554b2613f8907ff`
- HD-RECEIPT: `34632db671e2`
- Dispatch: `5180698408`
- expected_route: `DA-RESERVE(gate-wiring)`
- Scope: **6.3 only**; no movement/contention/5.11/census/downstream work

## ANCHOR-ACK

`admission-ladder-necessity-test,core-gpu-residency,core-overlays,core-property-value-model,core-rf-arenas,eml-extension-ladder,exact-numeric-candidate-f,field-policy-time-decisions,field-sweep-preservation,founding-ontology-invariants,one-tree-owners-never-spatial,orientation-harness-core,rf-arena-substrate,scanner-selftest-delta-gate,seal-residue-cross-crate,simthing-0087-binding-laws,simthing-0087-pillars,stead-events-are-rf,stead-rejected-shapes,stead-shared-surface-ledger,stead-spatial-contract-core,structural-execution-convergence,workshop-candidate-homing`

## Landed surface

- `simthing_core::ExecutionPosture::{Paced, Continuous{batch_generations}}` — scheduling policy over ONE kernel. Default paced. Continuous batches pump the same `SimSession` hot-cycle + boundary path (`run` match arm); no second kernel, model, or resolution meaning. Continuous `batch_generations == 0` fails closed at admit/`set_execution_posture`/`run` — never a silent `Ok` with zero generations.
- `simthing_spec::AsyncStalenessColumn` — derived STEAD scalar `parent_generation - latest_integrated_child_stamp` in ONE f32 lane per admitted slot. Seeded only from retained `OwnerChannelRfCrossingFlow.boundary_simthing_id`s; horizon-bounded tree-neighbourhood sweep; inert world (`AsyncStalenessColumn::inert`) allocates zero bytes, zero registrations, zero dispatches. Missing latest child stamp fails closed (no fabricated zero freshness). Whole-lattice registration mutant is test-only (`cfg(test)`), not a production API.
- N-generation forced-lag soak over landed 6.0/6.1/6.2 surfaces: `reduce_owner_channel_rf` + `reconstruct_owner_channel_rf_map` growth measurement, `AsyncOwnerChannelRfSeam` + sole `IntegrationSchedule` replay, closed causal cycle receive → CostBand → EML → originate → route → receive once per generation.
- Dual `ResolutionSite` soak: ClosedLoop and CpuAuthoritative produce bit-identical velocity-alert and AttachOverlay `BoundaryRequest` streams over N generations from identical seeds; planted mode-divergence mutant REDs.

## Growth measurements (OwnerChannelRfSteadSurface)

Scaling matrix over 16 generations each; retained rows = own aggregates + ownership crossings:

| nodes | owners | resources | max own_aggregates | max crossing_flows |
|---|---|---|---|---|
| 32 | 4 | 1 | 32 | 3 |
| 64 | 8 | 2 | 128 | 7 |
| 128 | 8 | 3 | 384 | 7 |
| 256 | 8 | 2 | 512 | 7 |

Crossing rows stayed crossing-bounded across all generations. Planted product-form growth mutant (inflate crossings toward `nodes × owners × resources`) diverges from measured growth and REDs the bound.

## Biting proofs

| Proof | Result |
|---|---|
| Continuous posture | `ExecutionPosture::continuous(N)` batches N generations; default remains `Paced`; same kernel path |
| Zero continuous batch | admit/set/`SimSession::run` fail closed — never silent `Ok` with zero gens |
| Paced session run | default `SimSession::run(N)` retains prior boundary-count behavior |
| Forced-lag dual-site | N=16 generations: ClosedLoop ≡ CpuAuthoritative velocity + AttachOverlay streams; mode-divergence mutant RED |
| STEAD growth | scaling matrix measured; product-form mutant RED |
| Staleness representation | one derived f32 STEAD lane; seed+horizon sweep; missing stamp RED; whole-lattice mutant test-only RED |
| Inert world | zero column bytes, registrations, dispatches, side state |
| Causal cycle | once-per-generation receive→CostBand→EML→originate→route→receive under load; no authored cascade bound |
| Replay | forced-lag run replays bit-exactly from the existing integration schedule; empty/ambient second recorder RED |
| Inherited device-loss | reproduced at ≥1000 slots on this head (see below) — reportable, not a 6.3 failure |

## Inherited device-loss characterization (≥1000 slots)

**Historical evidence (not re-paid):** 6.2b telemetry at `748a8222` / `docs/tests/resolution_site_split_0_results.md` — `threshold_stress` at 500 slots completes; 1,000 slots device-lost at `Queue::submit` inside `read_threshold_emissions` (Windows TDR-shaped).

**Reproduction on 6.3 implementation head (this branch):**

```text
cargo run -p simthing-driver --release -- bench --scenario <tmp n_slots=1000> --days 2
→ panic: Error in Queue::submit: Validation Error
  Caused by: Parent device is lost
```

Profile matches the inherited finding: fused threshold/reduction scale at ≥1000 slots exceeds the Windows TDR window. The 6.3 soak proofs are synthetic CPU/dual-site referees and do **not** require clearing that GPU threshold path. No silent workload shrink and no unrelated threshold-pipeline redesign were attempted.

## Local evidence

```text
cargo test -p simthing-spec --test continuous_posture_soak_0: 5/0
cargo test -p simthing-sim --test continuous_posture_dual_site_0: 1/0
cargo test -p simthing-spec async_staleness --lib: 3/0
cargo test -p simthing-core continuous_zero_batch --lib: 1/0
cargo test -p simthing-driver continuous_posture_session_proofs --lib: 2/0
```

## Posture

**PROBATION / proof-present / DA-review-pending.** Coder does not `/clearance`, merge, move the pointer, or begin successor work.
