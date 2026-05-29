# Phase M Abstract Boundary Resolution + Example Economy — Review / Parking Packet

> **Audience:** Opus / product review; future agents  
> **Status:** **PARKED FOR REVIEW** — docs/review packaging only; no runtime behavior changes in this packet.  
> **Date:** 2026-05-29  
> **Master baseline at parking:** `1855ccd0a2c5e982d0976508842fb224dc7c85a9` (Boundary Resolution Doctrine R1)

---

## 1. Executive verdict

**Phase M boundary resolution and example economy work is parked for review.**

The substrate supports **abstract tick/boundary resolution**. A host/spec may interpret a boundary as a day, turn, frame, season, orbital step, market close, learning epoch, or other semantic unit.

**Daily Economy Fixture V1 is an example fixture only**; it does not make daily cadence canonical for SimThing.

No runtime behavior changed in this parking pass. No `DailyResolutionBoundary` primitive was introduced. No Day/Calendar/Pause semantic was added to `simthing-sim`. No default SimSession mapping wiring was introduced. No atlas batching landed. No semantic WGSL landed. `simthing-sim` remains map-free. Defaults unchanged.

**Related prior work:**

- Boundary Resolution Doctrine audit: [`../tests/phase_m_boundary_cadence_doctrine_audit.md`](../tests/phase_m_boundary_cadence_doctrine_audit.md)
- Boundary Resolution Doctrine R1 (terminology correction): [`../tests/phase_m_boundary_resolution_doctrine_r1_test_results.md`](../tests/phase_m_boundary_resolution_doctrine_r1_test_results.md)
- Daily Economy Fixture V1: [`../tests/phase_m_daily_economy_fixture_test_results.md`](../tests/phase_m_daily_economy_fixture_test_results.md)
- First-slice vertical proof (separate track): [`phase_m_first_slice_vertical_proof_review_packet.md`](phase_m_first_slice_vertical_proof_review_packet.md)

---

## 2. Abstract boundary doctrine

SimThing exposes **abstract deterministic tick/boundary resolution**:

| Term | Meaning |
|---|---|
| **tick** | Deterministic substrate advancement (`DispatchCoordinator::tick`) |
| **boundary** | Synchronization point at which resolved summaries, events, and metadata may be consumed by host/spec/boundary-handler code |
| **day_index** | Current boundary counter / host-spec interpreted index |
| **ticks_per_day** | Current cadence field controlling how many ticks occur before a boundary |
| **pause/speed** | Host/UI orchestration; the sim advances only when the host requests ticks |

**Naming caveat:** The current names `ticks_per_day` and `day_index` are retained because they are already legible and widely used in fixtures/tests. Constitutionally, they do **not** make "day" a hardcoded substrate semantic. A host may interpret `day_index` as a day, turn, frame, season, orbital step, market close, learning epoch, or other unit.

**Allowed:**

- A scenario fixture uses `ticks_per_day=1` and calls that "daily" in fixture labels.
- A game host interprets `day_index` as a calendar day.
- Resource economy fixtures demonstrate example boundary banking.

**Not allowed in doctrine:**

- SimThing doctrine says boundary == day.
- `simthing-sim` gains Day/Calendar/Pause semantics.
- WGSL gains day/economy/calendar semantics.
- Mapping/runtime code assumes daily cadence by default.

---

## 3. Example daily economy fixture

**Daily Economy Fixture V1** demonstrates that a game **can choose** `ticks_per_day=1` and interpret each boundary as one day.

It uses discrete `ResourceEconomySpec` recipes/transfers:

1. **Production recipe** — conjunctive inputs → producer stock
2. **Bank transfer** — producer → treasury/storage
3. **Upkeep transfer** — treasury → upkeep sink
4. **Threshold/event** — `EmitOnThreshold` over resolved storage column

**Fixture locations:**

- Surplus RON: `crates/simthing-driver/tests/fixtures/daily_economy_banking_scenario.ron`
- Deficit RON: `crates/simthing-driver/tests/fixtures/daily_economy_banking_deficit_scenario.ron`
- Tests: `crates/simthing-driver/tests/phase_m_daily_economy_fixture.rs`

**Measured example (surplus):**

```text
initial treasury: 100
+10 income (recipe + bank transfer per boundary)
-3 upkeep
five-day trace: [107, 114, 121, 128, 135]
```

**Measured example (deficit):**

```text
initial treasury: 100
+2 income
-8 upkeep
treasury after one boundary: 94
exactly one low_storage_event emitted below threshold 95 (event_kind 0x4C4F5754)
```

**Important implementation note:** C-8d `ResourceEmissionSpec` uses `ConsumeMode::EmitEvent` and is **not** hard-currency banking. The fixture uses recipe production + discrete transfers, not emission-as-income.

---

## 4. What this proves

| Claim | Evidence |
|---|---|
| Abstract tick/boundary cadence exists | `phase_m_boundary_cadence_doctrine.rs`; feeder integration tests |
| Boundary can act as logical resolution unit | `boundary_reached`, `BoundaryProtocol`, boundary hook |
| Host/spec can interpret boundary cadence as a day without adding day semantics to simthing-sim | Daily economy fixture at `ticks_per_day=1`; no Day/Calendar/Pause in simthing-sim |
| Discrete `ResourceEconomySpec` can implement example boundary banking | Daily economy fixture 7/7 |
| Storage persists in GPU values across boundaries | Multi-day accumulation test; no per-day re-upload required |
| Threshold events can fire over resolved storage through existing substrate | Deficit fixture; `EmitOnThreshold` GPU path |
| CPU can consume resolved boundary outputs without recomputing economy state | Treasury readback from GPU `read_values()`; no CPU planner events |

---

## 5. What this does not prove

| Non-claim | Notes |
|---|---|
| Does not make day/calendar semantics canonical | Daily is one host interpretation only |
| Does not introduce `DailyResolutionBoundary` | Forbidden primitive; source-scanned |
| Does not prove month/year calendar authoring | No calendar system in substrate |
| Does not prove default economy gameplay | Fixture is opt-in; `ResourceEconomyOptInMode` required |
| Does not prove Resource Flow E-11 as daily economy | E-11 is continuous/high-frequency; default-off |
| Does not integrate daily economy with mapping/SEAD | Separate tracks; no combined fixture |
| Does not add default SimSession mapping wiring | `MappingExecutionProfile::default()` = Disabled |
| Does not authorize atlas | M-4 atlas remains provisional/unimplemented |
| Does not authorize dense grid readback at boundary | CPU boundary discipline preserved |

---

## 6. Resource Economy vs Resource Flow distinction

| Substrate | Role | Default posture |
|---|---|---|
| **`ResourceEconomySpec`** | Discrete boundary-banking **example** substrate (transfers, recipes, threshold emit) | Opt-in via `ResourceEconomyOptInMode`; not default-on |
| **Resource Flow E-11** | Continuous/high-frequency flow substrate | Separately opt-in; `use_accumulator_resource_flow` **default false** |

**Do not use Resource Flow as the default answer for discrete boundary banking.**

Discrete resource economy is the example substrate used by the daily economy fixture. It is **not** stated as the recommended model for all SimThing simulations.

---

## 7. Guardrails for future agents

When extending boundary or economy work, preserve:

```text
Do not add Day/Calendar/Pause semantic to simthing-sim.
Do not introduce DailyResolutionBoundary.
Do not equate boundary with day in doctrine.
Do not use CPU planner logic to emit events.
Do not recompute economy/threat/urgency on CPU at boundary.
Do not scan dense RegionCell grids at boundary by default.
Do not turn Resource Flow E-11 on by default.
Do not implement atlas as a side effect of boundary/economy work.
Do not add default SimSession mapping pass-graph wiring.
Do not add semantic WGSL for day/economy/calendar.
Do not rename ticks_per_day/day_index in this track without explicit product authorization.
```

**Constitutional posture (unchanged):**

- V7.7 Mapping ADR approved; first-slice vertical proof accepted
- SummaryValidity V1 + V1-R1 parked; Queue-Write/Map Residency landed
- `MappingExecutionProfile::default()` = Disabled
- `simthing-sim` remains map-free

---

## 8. Evidence table

| Test report | Purpose | Core result | Status |
|---|---|---|---|
| [`phase_m_boundary_cadence_doctrine_audit.md`](../tests/phase_m_boundary_cadence_doctrine_audit.md) | Original boundary resolution audit | Abstract cadence confirmed; no forbidden primitive | **PASS** |
| [`phase_m_boundary_resolution_doctrine_r1_test_results.md`](../tests/phase_m_boundary_resolution_doctrine_r1_test_results.md) | R1 terminology correction | Active guidance reframed; source scan added | **PASS** |
| [`phase_m_daily_economy_fixture_test_results.md`](../tests/phase_m_daily_economy_fixture_test_results.md) | Example daily economy fixture | Surplus/deficit/replay/posture 7/7 | **PASS** |
| [`phase_m_boundary_resolution_review_packet_test_results.md`](../tests/phase_m_boundary_resolution_review_packet_test_results.md) | This parking pass verification | Review packet created; targeted tests green | **PASS** |

---

## 9. Recommended next options

Options listed for Opus/product decision — **not implemented in this packet**:

| Option | Description | Recommendation |
|---|---|---|
| **A** | Accept and park boundary-resolution doctrine | **Recommended first** |
| **B** | Generic boundary-output packet for examples, still abstract and non-semantic | Acceptable if it stays abstract; **do not** let this become `DailyResolutionBoundary` by another name |
| **C** | Integrate one economy signal with the first-slice SEAD fixture, explicitly as a product fixture | Reasonable after A; keep opt-in and fixture-scoped |
| **D** | Improve resource-economy authoring ergonomics | Reasonable after A; no substrate semantic expansion |
| **E** | Return to mapping scale work (atlas/M-4) | **Do not do yet** — only after Opus/product accepts parked doctrine and names a multi-theater scenario |

**Suggested sequence:** **A first**, then **C or D**. Do not do **B** if it starts becoming `DailyResolutionBoundary` by another name. Do not do **E** / M-4 atlas yet.

---

## 10. Parking verdict

**PARKED FOR REVIEW** — Phase M abstract boundary-resolution + example economy work is ready for Opus/product acceptance of the abstract doctrine and example-fixture scope. No further runtime implementation is required for this parking pass.

Required acceptance wording if PASS:

```text
Phase M abstract boundary-resolution + example economy review packet landed.
The repo now distinguishes abstract substrate tick/boundary cadence from game-level daily interpretation. `ticks_per_day` and `day_index` remain the legible API names for boundary cadence and the boundary counter; despite the names, day/calendar semantics are not part of `simthing-sim`.
Daily Economy Fixture V1 remains a valid product/example fixture showing one game-level interpretation: one boundary as one day, with discrete ResourceEconomySpec banking.
No runtime behavior changed.
No DailyResolutionBoundary primitive was introduced.
No Day/Calendar/Pause semantic was added to simthing-sim.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.
```
