# simthing-workshop — Multichannel Accumulator Test Battery

## Implementation-Ready Specification

> **Status: PROVISIONAL WORKSHOP PLAN**
>
> This document defines a standalone benchmark/test crate intended to measure whether the proposed accumulator/emission model justifies an eventual `AccumulatorWrite` pivot. It is not an implementation plan for production GPU migration.

This document integrates the original multichannel factory benchmark plan with additional pivot-readiness gates: fit matrix, full residual conservation, debt-band testing, replay/delta-log checks, hierarchical aggregation parity, contention profiles, and explicit decision thresholds.

The battery now answers two different questions:

1. **Economic throughput:** Are accumulator/emission workloads faster or simpler under interventions or a Pass B' prototype?
2. **Foundational pivot readiness:** Is `AccumulatorWrite` expressive, deterministic, replayable, and parity-safe enough to become a future GPU execution primitive?

Passing the factory benchmark alone is **not** enough to justify the pivot.

---

## Scope discipline

**IS:**

- A standalone crate at `crates/simthing-workshop`.
- Three test scenarios: `1k`, `10k`, and `100k` factories.
- Three implementations:
  - **A:** current baseline.
  - **B:** current pipeline with intervention/template improvements.
  - **C:** isolated Pass B' pivot prototype.
- A fixed harness producing `target/workshop/report.json` and `target/workshop/report.md`.
- Pivot-readiness checks for fit, replay, hierarchy, conservation, contention, and decision thresholds.
- A minimal Pass B' compute shader handling exactly three prototype variants:
  - transfer;
  - conjunctive emit;
  - velocity integrate.

**IS NOT:**

- A migration of any existing production pass.
- A replacement for `fission_stress` or existing tests.
- A production dependency of `simthing-sim`, `simthing-gpu`, or `simthing-driver`.
- A proof that the whole GPU pipeline should be replatformed merely because ImplC is fast.

**Hard rules:**

- `simthing-workshop` depends on production crates, but production crates never depend on it.
- Pass B' lives entirely under `crates/simthing-workshop/src/pass_b/`.
- The workshop crate imports only public APIs.
- ImplC is an idealized pivot prototype, not production-compatible proof by itself.

---

## Phase 0 — AccumulatorWrite fit matrix and non-distortion review

**Goal:** Determine whether `AccumulatorWrite` can express existing GPU operations without semantic distortion before prototype numbers are overinterpreted.

Create `docs/workshop/accumulator_write_fit_matrix.md` with one row per current operation:

| Operation | Current location | AccumulatorWrite form | Fit | Risk |
|-----------|------------------|-----------------------|-----|------|
| Velocity integration | Pass 2 | `source=Velocity`, `target=Amount`, `scale=dt`, `gate=Always` | likely | dt/source semantics |
| Overlay Add | Pass 3 | `source=Constant`, `target=col`, `scale=Identity` | likely | lifecycle gating |
| Overlay Multiply | Pass 3 | special `ScaleSource` or retained op | uncertain | not additive |
| Overlay Set | Pass 3 | special `SetTarget` or retained op | uncertain | not additive |
| Reduction Sum | Passes 4–6 | child writes to parent | likely | ordering/atomic contention |
| Reduction Mean | Passes 4–6 | sum + count / derived write | partial | needs count |
| Reduction WeightedMean | Passes 4–6 | weighted sum + weight sum | partial | parity/order |
| Reduction Min/Max | Passes 4–6 | atomic min/max equivalent | partial | f32 atomics/ties |
| Threshold scan | Pass 7 | `gate=Threshold`, output event/write | likely | event contract |
| Debt-band emission | proposed | crossing formula + emit count | likely | metadata/replay |
| Transfer with consume | proposed | target add + source subtract | likely | conservation |
| Intensity update | Pass 1 | TBD | risky | nonlinear build/decay |
| Velocity snapshot / previous values | Pass 0 | TBD | risky | history-buffer semantics |

Fit labels:

```text
Clean:
  Can be represented directly as source/gate/scale/consume.

Partial:
  Can be represented only with additional helper state, multiple writes,
  or retained companion pass.

Risky:
  Might distort semantics, parity, or replay.

Retained:
  Should remain its own pass for now.
```

Exit criterion:

```text
≥90% of current GPU operations fit Clean or Partial,
and all Risky/Retained operations are explicitly bounded retained passes,
and no existing invariant is weakened merely to make the abstraction fit.
```

If intensity update or velocity snapshot requires contortions, do not force them into `AccumulatorWrite`. A five-pass hybrid can still be valuable; a leaky universal kernel is not.

---

## Crate layout

```text
crates/simthing-workshop/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── scenario.rs
│   ├── metrics.rs
│   ├── report.rs
│   ├── harness.rs
│   ├── impl_a_baseline.rs
│   ├── impl_b_intervention.rs
│   ├── impl_c_pivot.rs
│   └── pass_b/
│       ├── mod.rs
│       ├── registration.rs
│       └── kernel.wgsl
├── fixtures/
│   ├── factory_1k.ron
│   ├── factory_10k.ron
│   ├── factory_100k.ron
│   ├── factory_1k_starvation.ron
│   └── debt_band_1k.ron
├── tests/
│   ├── conservation.rs
│   ├── starvation.rs
│   ├── parity.rs
│   ├── debt_band.rs
│   ├── replay.rs
│   ├── hierarchy.rs
│   └── contention.rs
└── benches/
    └── run_battery.rs
```

---

## Pre-phase production-crate PRs

These should be separate PRs before the phases that consume them.

### PR W-1 — `ThresholdSemantic::EmitOnThreshold`

Add to `simthing_sim::threshold_registry::ThresholdSemantic`:

```rust
EmitOnThreshold {
    sim_thing_id: SimThingId,
    channel_idx: usize,
    unit_cost: f32,
}
```

Required production test: verify all `ThresholdSemantic` variants, including `EmitOnThreshold`, round-trip through serde.

### PR W-2 — templating / consume-mode intervention

The original proposal suggested adding `ConsumeMode` to `ThresholdRegistration` and a `TemplatedEmitOnThreshold` semantic. Treat this as an intervention experiment, not a required production migration.

Preferred safety split:

```text
B1:
  CPU-expanded templates, no production GPU scanner change.

B2:
  GPU-expanded templated threshold scan, only if B1 proves useful.
```

Do not change production GPU struct layout merely for the workshop unless the phase explicitly requires it and tests prove the change is isolated.

---

## Timing measurement contract

All implementations measure the same named points:

```text
T0: boundary begins
T1: GPU submit returns
T2: GPU readback begins
T3: GPU readback complete
T4: CPU boundary handler complete
T5: run_tick returns

boundary_total_us  = T5 - T0
readback_us        = T3 - T2
cpu_handler_us     = T4 - T3
gpu_dispatch_count = encoder.submit() calls between T0 and T1
bytes_uploaded     = queue.write_buffer byte count between T0 and T1
emissions_this_tick = units produced this tick across all factories
```

Additional required pivot-readiness metrics:

```rust
queue_residual_totals: BTreeMap<String, f32>
emissions_by_tick: Vec<u32>
threshold_registrations_active_max: u32
threshold_registrations_updated_total: u64
threshold_upload_bytes_total: u64
threshold_rebuild_mean_us: u64
```

Rationale:

```text
queue_residual_totals:
  required for the full conservation equation.

emissions_by_tick:
  required for starvation-window and burst/overshoot tests.

threshold_*:
  required to diagnose whether next-band re-registration is the bottleneck.
```

---

## Conservation invariant

The correct conservation check accounts for value in-flight in factory queues:

```text
faction_pool_decrease  = Σ(channel) faction_initial[ch] - faction_final[ch]
factory_queue_increase = Σ(factory, channel) queue_final[f,ch] - queue_initial[f,ch]
units_consumed_value   = total_emissions × Σ(channel) unit_costs[ch]

INVARIANT:
  faction_pool_decrease == factory_queue_increase + units_consumed_value
  tolerance: ±0.01 × faction_pool_decrease
```

Implementation consequence: `resource_totals()` must return final queue residuals, not just faction pool totals. A weaker bound check is not sufficient for pivot evidence.

---

## Scenario model

`Scenario` contains:

```rust
pub struct Scenario {
    pub name: String,
    pub n_factories: u32,
    pub ticks: u32,
    pub channels: Vec<ChannelSpec>, // 1..=4
    pub recipe: Recipe,
    pub starvation_window: Option<StarvationWindow>,
}

pub struct ChannelSpec {
    pub name: String,
    pub initial_pool: f32,
    pub transfer_rate: f32,
}

pub struct Recipe {
    pub unit_costs: Vec<f32>,
    pub max_per_tick: u32,
}
```

Baseline fixture:

```ron
(
    name: "factory_1k",
    n_factories: 1000,
    ticks: 1000,
    channels: [
        (name: "iron",   initial_pool: 100000.0, transfer_rate: 1.0),
        (name: "energy", initial_pool: 100000.0, transfer_rate: 0.6),
        (name: "labor",  initial_pool: 100000.0, transfer_rate: 0.4),
    ],
    recipe: (
        unit_costs: [5.0, 3.0, 2.0],
        max_per_tick: 4,
    ),
    starvation_window: None,
)
```

Starvation fixture sets iron transfer rate to `0.05` for ticks `100..200`.

Debt-band fixture:

```ron
(
    name: "debt_band_1k",
    n_factories: 1000,
    ticks: 100,
    channels: [
        (name: "build", initial_pool: 1000000.0, transfer_rate: 55.0),
    ],
    recipe: (
        unit_costs: [20.0],
        max_per_tick: 100,
    ),
    // Workshop-specific debt-band init:
    // queued_count = 10
    // queued_build.Amount = -200
    starvation_window: None,
)
```

---

## Implementation A — baseline

Uses existing production GPU/session path, existing threshold event readback, and CPU boundary hook.

Key assertions:

```text
EmitOnThreshold lives in simthing-sim only.
Workshop imports it; it does not redefine it.
Boundary hook remains generic.
No workshop-specific type appears in production execute_with_boundary_hook.
Ready state clears on starvation invalidation.
Threshold re-registration happens through boundary requests, not direct ThresholdBuilder calls inside the hook.
```

Expected 1k/100 tick acceptance:

```text
emissions > 0
emissions within 10% of n_factories * ticks / limiting_cost_period
full conservation passes
```

---

## Implementation B — interventions

Tests current-pipeline improvements before full pivot.

Intervention candidates:

```text
B1:
  CPU-expanded template registrations.

B2:
  GPU-expanded templated registrations.

Optional:
  consume-mode metadata if needed, but avoid production struct changes until justified.
```

Acceptance:

```text
Emission counts match A within ±2%.
Full conservation passes.
threshold_registrations_active_max is materially lower than A if templating is active.
If B captures ≥80% of C's performance gain, prefer intervention over pivot.
```

---

## Implementation C — isolated Pass B' pivot prototype

ImplC owns a flat values buffer independent of `SimWorld`:

```text
slot 0:        faction pool
slots 1..=N:   factory queues
slot N+1:      units_produced counter

n_dims = channels.len() * 2 + 1
```

ImplC bypasses:

```text
BoundaryProtocol
SimWorld
overlay system
threshold registry
production GPU pass pipeline
```

This is intentional. ImplC is the idealized lower-bound / best-case prototype. Passing ImplC does not prove production integration.

Pass B' variants:

```text
Transfer:
  source pool -> factory queue, subtract from source.

ConjunctiveEmit:
  emit min(queue[ch] / unit_cost[ch]) across channels.

VelocityIntegrate:
  amount += velocity.
```

WGSL must document nondeterminism:

```text
CAS-loop interleaving may vary.
f32 floor near integer boundaries may vary by backend.
High contention can spin.
NaN inputs are invalid.
Negative pools are allowed but finite only.
```

Run repeat tests at least 10 times, not merely twice.

---

## Phase 9 — Cross-implementation tests

### Full conservation with residual queues

Every implementation reports:

```rust
initial_resource_totals
final_resource_totals
queue_residual_totals
total_emissions
```

The required assertion per channel:

```rust
let faction_pool_decrease = initial - final_pool;
let factory_queue_increase = queue_residual;
let units_consumed_value = total_emissions as f32 * unit_cost;

assert_approx_eq(
    faction_pool_decrease,
    factory_queue_increase + units_consumed_value,
    tolerance,
);
```

### Starvation-window emissions

`ReportEntry.emissions_by_tick` is required.

Do not use total emissions as the test. Assert directly over ticks `100..200`:

```text
during_starvation_emissions < 5% of full-rate estimate for that window
```

### Cross-implementation parity

```text
B emissions match A within ±2%.
C emissions match A within ±2%.
A/B/C all pass full residual conservation.
```

---

## Phase 9b — Debt-band negative accumulator fixture

Tests the exact one-column negative debt model:

```text
previous Amount = -200
current Amount = -145
unit_cost = 20
queued_count = 10
emit_count = floor((10 * 20 + -145) / 20) = 2
new queued_count = 8
Amount remains -145
next_threshold = -140
carryover = 15
```

Acceptance:

```text
All implementations agree on emit_count.
Carryover is preserved.
Next threshold points at the next debt band.
No positive-queue fallback is used.
```

---

## Phase 9c — Replay and delta-log determinism fixture

Required experiment:

```text
1. Run ImplC for 100 ticks from a fixed scenario.
2. Save initial scenario, per-tick emission records, final pools,
   final queues, and final units_produced.
3. Replay by applying recorded emission records to a CPU reference model.
4. Assert final pools, queues, and units match ImplC within tolerance.
5. Rerun ImplC from the same initial state 10 times without records.
6. Assert total_emissions is identical across all 10 runs.
```

Decision implication:

```text
If record-based replay works but rerun-only replay does not:
  GPU-resolved emissions require compact emission records.

If neither works:
  Do not promote GPU-resolved numeric emission.

If both work:
  ADR must still decide whether replay records or deterministic rerun are canonical.
```

---

## Phase 9d — Hierarchical aggregation and reduction parity fixture

Tests whether the pivot covers reduction/aggregation, not merely flat factory queues.

Scenario shape:

```text
100_000 factories
→ 100 districts
→ 10 planets
→ 1 faction
```

Required reductions:

```text
Sum:
  district.output = sum(factory.output)
  planet.output = sum(district.output)
  faction.output = sum(planet.output)

Mean:
  district.efficiency = mean(factory.efficiency)

WeightedMean:
  planet.stability = weighted_mean(district.stability, district.population)

Min/Max:
  faction.min_readiness / max_readiness across planets
```

Compare:

```text
current reduction path
CPU oracle
AccumulatorWrite-style prototype, if implemented
```

If this phase fails, the pivot may still be useful for transfer/emission but not as a full pipeline replacement.

---

## Phase 9e — Contention profile battery

Profiles:

```text
many_to_one:
  many factories write to one units counter.

one_to_many:
  one faction pool feeds many factory queues.

many_to_many:
  multiple pools feed multiple queues.

hierarchical_many_to_few_to_one:
  factories → districts → planets → faction.

sparse_active:
  1% of factories active each tick.

bursty_all_active:
  100% of factories active on the same tick.
```

Metrics:

```text
wall_clock_ms
boundary_p50_us
boundary_p99_us
gpu_dispatch_count
emission buffer count
atomic contention proxy: registrations writing to same target
repeat-run variance over 10 runs
```

Decision implication:

```text
If ImplC only wins in low-contention flat cases, do not generalize its result.
If hierarchical reduction reduces contention and wins, pivot case strengthens.
```

---

## Battery runner and report

The runner writes:

```text
target/workshop/report.json
target/workshop/report.md
```

Report table includes:

```text
Impl
Scenario
n_factories
wall_ms
p50_us
p99_us
readback_us
handler_us
emissions
threshold_registrations_active_max
threshold_registrations_updated_total
threshold_upload_bytes_total
threshold_rebuild_mean_us
```

The report must include a generated `decision_summary` section.

---

## Phase 10b — Pivot decision gates

The battery is not complete until it recommends one of:

```text
No pivot:
  current paradigm remains active; optimize locally only.

Intervention track:
  keep current passes, adopt selected optimizations/templates.

Hybrid track:
  implement transfer/emission as AccumulatorWrite-like sidecar only.

Full ADR track:
  open AccumulatorWrite foundational ADR.
```

Minimum gates:

```text
Gate 1 — Expressiveness:
  Phase 0 fit matrix shows ≥90% Clean/Partial fit.
  Risky/Retained operations are explicitly bounded.

Gate 2 — Correctness:
  A, B, C all pass full conservation with queue residuals.
  Debt-band fixture passes.
  Starvation-window per-tick test passes.
  Existing production tests remain green after W-1/W-2, if those PRs land.

Gate 3 — Parity:
  Hierarchical reduction parity passes against CPU oracle/current GPU path.
  ImplC emission counts match ImplA within 2% for economic fixtures.

Gate 4 — Replay:
  GPU-resolved emissions are replayable either by compact emission records
  or deterministic rerun. If not, no foundational pivot.

Gate 5 — Performance:
  ImplC must beat ImplB by at least 2× at 100k factories on boundary_p99_us
  or total wall time.
  If ImplB captures ≥80% of ImplC's win with less architectural risk,
  prefer Intervention track.

Gate 6 — Bottleneck:
  If readback/CPU handler is not a dominant cost after B/interventions,
  do not promote the pivot yet.
```

Mandatory report language:

```text
The factory throughput battery alone cannot justify a foundational pivot.
A foundational pivot requires passing fit, conservation, debt-band,
replay, hierarchy, contention, and performance gates.
```

---

## Critical observation checklist

| Phase | Hard check |
|-------|------------|
| 0 | Fit matrix reviewed before treating ImplC as pivot evidence |
| 1 | `cargo tree --invert simthing-workshop` shows zero parents |
| 2 | Fixtures load; channel order is stable and validated |
| 3 | Metrics include queue residuals, per-tick emissions, and threshold rebuild stats |
| 4 | Harness starvation restore reads `scenario.channels`, not impl state |
| 5 | `EmitOnThreshold` in `simthing-sim` only; ready-state clears on starvation invalidation |
| 6 | Template intervention does not silently require production scanner rewrite |
| 7 | Atomic helpers reviewed; no NaN inputs; overflow check fires pre-process |
| 8 | ImplC imports no `simthing_sim`, `BoundaryProtocol`, or `SimWorld` |
| 9 | Full conservation uses queue residuals; starvation checks per-tick window emissions |
| 9b | Debt-band negative accumulator fixture tests `-200 → -145` carryover |
| 9c | Replay fixture proves compact records or deterministic rerun |
| 9d | Hierarchical reduction parity passes or pivot is partial only |
| 9e | Contention profiles cover many-to-one, one-to-many, many-to-many, sparse, bursty |
| 10 | Two runs produce identical `total_emissions` in `report.json` |
| 10b | Report contains explicit pivot/intervention/no-pivot recommendation |

---

## Start-coding checklist

Before coding Phase 1, verify:

- [ ] PR W-1 (`EmitOnThreshold`) is open or merged.
- [ ] W-2 is either deferred or split into B1/B2 intervention PRs.
- [ ] `fixtures/factory_1k.ron` content is in the repo.
- [ ] `fixtures/debt_band_1k.ron` content is in the repo.
- [ ] The timing measurement contract has been read.
- [ ] The full conservation invariant has been read.
- [ ] `ReportEntry` includes `queue_residual_totals` and `emissions_by_tick`.
- [ ] Phase 0 fit matrix has been reviewed before treating ImplC as pivot evidence.
- [ ] Decision gates in Phase 10b have been read and accepted.

---

## Bottom line

This battery is designed to avoid a false-positive pivot decision.

It can prove:

```text
The accumulator/emission economic workload benefits from a pivot-shaped implementation.
```

It cannot prove a foundational GPU replatform unless the added fit, replay, hierarchy, contention, and decision gates also pass.

If ImplC wins only on flat factory throughput, the correct result is not “full pivot.” It is “hybrid or intervention track.”
