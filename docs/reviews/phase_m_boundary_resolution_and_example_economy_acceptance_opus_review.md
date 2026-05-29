# Opus/Product Acceptance Review — Phase M Abstract Boundary Resolution + Example Economy (R2 Vocabulary)

**Date:** 2026-05-29
**Authority:** Opus 4.8, mapping/SEAD design authority under human delegation (authority to raise
guardrails up to the Designer-facing / RON / Scenario layer).
**Decision type:** Acceptance review — **not** an implementation handoff. No code changed.
**Reviews:** packet `docs/reviews/phase_m_boundary_resolution_and_example_economy_review_packet.md`;
reports `phase_m_boundary_resolution_doctrine_r2_terminology_test_results.md`,
`phase_m_boundary_resolution_review_packet_test_results.md`,
`phase_m_daily_economy_fixture_test_results.md`,
`phase_m_boundary_cadence_doctrine_audit.md`.
**Builds on:** the first-slice vertical proof acceptance
(`docs/reviews/phase_m_first_slice_vertical_proof_acceptance_opus_review.md`).

---

## 1. Executive verdict

**PASS WITH CONDITIONS.** The boundary-resolution doctrine is **accepted**:
SimThing exposes deterministic `tick`/`boundary`/`day` resolution; `day_index` is the boundary
counter (one host/spec interpretation is a calendar day); `ticks_per_day` is the cadence field;
`pause`/`speed` are host/UI orchestration (the sim advances only when the host requests ticks).

**Naming preference (product, 2026-05-29 — supersedes the R1/R2 "abstract cadence" framing):**
`tick`, `boundary`, `day`, `day_index`, and `ticks_per_day` are the **preferred, endorsed names for
their legibility**. We do **not** churn them toward abstract/illegible alternatives. The line we
hold is on *semantics*, not vocabulary: avoid Clausewitz/calendar **semantics** (calendar
arithmetic, `Calendar`/month/year/season types, leap/date math, a sim-side pause flag,
`DailyResolutionBoundary`) — **not** the legible day-flavored names. Calendar/turn/frame/season
*meaning* lives at the host/spec/boundary-handler layer.

**Daily Economy Fixture V1 is accepted as one example/product fixture only** — it does not make
daily cadence canonical. The `ResourceEconomySpec` (discrete boundary banking) vs Resource Flow
E-11 (continuous, default-off) distinction is accepted. The future-agent guardrails are made
**binding**.

The condition (below) is a **terminology-precision** fix, not a behavioral one: the
doctrine's "no day semantics in `simthing-sim`" claim must be stated precisely — it is about
*semantics*, not the legible day-flavored names, which `simthing-sim` already uses throughout and
which product now affirmatively **prefers**. The substance is correct; the wording must not be
falsifiable by a source grep, and must not read as discouraging the legible names.

---

## 2. Evidence reviewed

Packet + all four reports read in full. **Independently verified in code, not taken on the
reports' word:**

- **Cadence is abstract and host-driven.** `crates/simthing-feeder/src/dispatcher.rs`:
  `ticks_per_day` (must be > 0), `boundary_reached = tick_in_day >= ticks_per_day`, `day_index`
  "bumps once per `ticks_per_day` ticks." `SimSession::run`/`tick` are pull-based; nothing advances
  unless the host calls a tick (confirmed: no `thread::spawn`/wall-clock scheduler in the sim).
- **Forbidden primitive genuinely absent.** `DailyResolutionBoundary` appears **only** in two
  negative-assertion source-scan tests (`phase_m_boundary_cadence_doctrine.rs`,
  `phase_m_daily_economy_fixture.rs`) — never as a real type. Its absence is regression-guarded.
- **No calendar/pause *types* in the sim.** No `Calendar`, no month/season arithmetic, no pause
  flag in `crates/simthing-sim/src`. (See the condition for the naming nuance.)
- **Economy is opt-in and discrete.** The daily fixtures run a `TransferOnly` game mode
  (`use_accumulator_transfer=true`, `use_accumulator_emission=false`,
  `use_accumulator_resource_flow=false` — default-off). Income = recipe production + conservation-
  exact discrete transfer into a persistent treasury column; upkeep = transfer out; deficit fires
  a threshold `EmitOnThreshold` event over resolved storage. Storage persists in the GPU values
  buffer across boundaries with no per-day re-upload.

**Independent verification run (this review, real GPU, this machine):**

| Suite | Result |
|---|---|
| `phase_m_boundary_cadence_doctrine` | **7/7** (incl. `host_pause_preserves_state_after_partial_advancement`, `doctrine_no_daily_resolution_boundary_primitive`) |
| `phase_m_daily_economy_fixture` | **7/7** (incl. `multi_day_accumulation_is_deterministic`, deficit threshold event) |
| `region_field_spec_admission` | **11/11** |

`cargo check --workspace` is recorded green in the R2 report; the full workspace was run in the
audit/fixture passes. The R2 pass itself was docs-only terminology, so the targeted subset is the
correct verification scope.

---

## 3. Acceptance decision (answers to the four questions)

1. **Abstract boundary doctrine — ACCEPT WITH CONDITIONS.** Correct and well-evidenced. Condition
   C-1 (terminology precision) applies.
2. **Daily fixture classification — ACCEPT.** Daily Economy Fixture V1 is an example/product fixture
   only: opt-in, default-off, regression-guarded against `DailyResolutionBoundary`, and explicitly
   non-canonical. It proves a *host interpretation*, not a substrate semantic.
3. **ResourceEconomy vs Resource Flow distinction — ACCEPT.** `ResourceEconomySpec` (discrete
   boundary banking) is the example substrate for boundary banking; Resource Flow E-11 is the
   continuous/high-frequency substrate, separately opt-in and **default-off**. Resource Flow must
   not be the default answer for discrete boundary banking. This matches the design guidance already
   given for the daily-cadence question.
4. **Future-agent guardrails — ACCEPT (made binding).** All eleven are sound and consistent with the
   constitution. Codified as a binding invariants section (see §8), with the one precision gloss in
   C-1 applied to the "no Day/Calendar/Pause semantics" item.

---

## 4. Conditions

- **C-1 (terminology precision — required).** `simthing-sim` already uses "day boundary" / `day`
  naming pervasively and predating this doctrine (e.g. `lib.rs` "day boundary orchestration",
  the numbered "step N of the day boundary" modules, `replay.rs` `day: u32`, and the Evaluation
  invariant "fission/fusion belong to day-boundary protocol"). The doctrine's claim is **about
  semantics, not vocabulary**: there is no calendar arithmetic, no `Calendar`/month/year/season
  type, no pause flag, and no `DailyResolutionBoundary`. The doctrine's naming caveat must say so
  explicitly — that the pre-existing `day` / "day boundary" terminology is the same legible-
  monotonic-counter naming as `day_index`/`ticks_per_day`, carrying no calendar interpretation — so
  the doctrine cannot be read as contradicted by a grep of the source. **The binding guardrail is
  "no calendar/pause *semantics* (arithmetic, types, interpretation, sim pause flag, or
  `DailyResolutionBoundary`)," not "no use of the word day."** (Applied in this pass — see §8.)
- **C-2 (prohibitions hold).** All §5 guardrails remain enforced; none relaxes except through its
  own separately-gated decision.
- **C-3 (legible naming is the product preference).** `tick` / `boundary` / `day` / `day_index` /
  `ticks_per_day` are the **preferred, endorsed names for their legibility** (product, 2026-05-29).
  Do **not** churn them toward abstract/illegible alternatives, and do not rename them in this track
  without explicit product authorization. The R1→R2→this-pass arc settled here: legible names are
  preferred; only Clausewitz/calendar *semantics* are out of scope.

---

## 5. Binding guardrails

Made binding (codified in `docs/invariants.md`, "Boundary resolution (tick / boundary / day)"
section):

```text
PREFER the legible tick/boundary/day/day_index/ticks_per_day naming; do not churn it toward
  abstract/illegible alternatives (product preference, 2026-05-29).
Do not introduce DailyResolutionBoundary (or any equivalent runtime primitive).
Do not add day/calendar/pause SEMANTICS to simthing-sim — no calendar arithmetic, no
  Calendar/month/year/season type, no leap/date math, no sim-side pause flag. (The legible
  day/day_index/ticks_per_day/"day boundary" naming is endorsed; day_index is a monotonic
  boundary counter, not a calendar.)
Do not equate boundary with day in doctrine; day is one host/spec interpretation of day_index.
Do not use CPU planner logic to emit events; commitments are GPU Threshold + EmitEvent crossings.
Do not recompute economy/threat/urgency on the CPU at the boundary; the CPU consumes resolved
  summaries/events/metadata only.
Do not scan dense RegionCell grids at the boundary by default.
Do not turn Resource Flow E-11 on by default; use the discrete ResourceEconomy substrate for
  discrete boundary banking.
Do not implement atlas as a side effect of boundary/economy work.
Do not add default SimSession mapping pass-graph wiring (MappingExecutionProfile default Disabled).
Do not add semantic WGSL for day/economy/calendar.
Do not rename tick/boundary/day/ticks_per_day/day_index in this track without explicit product
  authorization (the standing preference is to keep them).
```

---

## 6. Recommended next implementation handoff

Using the handoff's option lettering (A–E):

**A is accepted/parked now** (this memo). The next *implementation* handoff should be **C — improve
resource-economy authoring ergonomics** (lower risk, broadly useful, no substrate-semantic
expansion) **or B — integrate one economy signal with the first-slice SEAD fixture, explicitly as a
product fixture** (opt-in, fixture-scoped). Either is acceptable; I lead with C.

- **D (generic boundary-output packet):** admissible **only if tightly bounded** — it must stay an
  abstract, semantic-free carrier of already-resolved summaries/events/metadata and must **not**
  become `DailyResolutionBoundary` by another name (no calendar fields, no CPU recomputation, no
  day arithmetic). If it starts to accrete day/economy meaning, stop and escalate.
- **E (mapping scale / M-4 atlas):** **not now** — only after a named multi-theater scenario, an
  approved VRAM budget, and a §11-gate-passing M-4 PR.

> **Note on option letters:** the review packet's §9 list assigns B/C/D differently from this
> handoff (packet B = boundary-output packet, C = economy+SEAD, D = authoring ergonomics). The
> substance is identical regardless of letter: authoring ergonomics or an economy+SEAD product
> fixture next; a boundary-output packet only if tightly bounded; not atlas. I follow the handoff's
> lettering here.

---

## 7. Stop conditions for the next handoff (escalate; do not land)

Whichever of B/C/D is taken next must not introduce any of:
- `DailyResolutionBoundary` or any runtime primitive that bakes "boundary == day";
- day/calendar/pause **semantics** in `simthing-sim` (arithmetic, calendar types, sim pause flag),
  or semantic WGSL for day/economy/calendar;
- CPU-side recomputation of economy/threat/urgency at the boundary, or CPU-planner event emission;
- dense RegionCell grid scanning at the boundary by default;
- Resource Flow E-11 default-on, or using it as the default discrete-banking substrate;
- atlas implementation as a side effect, or relaxing the `request_atlas_batching` admission rejection;
- default SimSession mapping pass-graph wiring;
- renaming `ticks_per_day` / `day_index` without explicit product authorization.

For **D specifically:** the boundary-output packet must be a read-only, abstract carrier of
already-resolved values; the moment it grows a calendar field, a CPU compute step, or day semantics,
it has become the forbidden primitive and must be rejected.

---

## 8. Doc / ADR updates made alongside this memo

- **New:** this memo.
- **`docs/reviews/phase_m_boundary_resolution_and_example_economy_review_packet.md`** — status
  flipped to **ACCEPTED (PASS WITH CONDITIONS)**; naming caveat (§2) extended to cover the
  pre-existing `simthing-sim` "day"/"day boundary" vocabulary (C-1).
- **`docs/invariants.md`** — **"Boundary resolution (tick / boundary / day)"** binding section
  codifying the doctrine and the eleven guardrails (with the C-1 precision gloss).
- **`docs/workshop/mapping_current_guidance.md`**, **`docs/workshop/workshop_current_state.md`**,
  **`docs/accumulator_op_v2_production_plan.md`**, **`docs/todo.md`** — status flipped from
  "parked for review" to **accepted**; next step = C or B (authoring ergonomics / economy+SEAD
  fixture), not D-unless-bounded, not atlas.
- **`docs/worklog.md`** — dated 2026-05-29 acceptance entry appended.

All updates are decision/classification only. No production code changed; `MappingExecutionProfile`
default remains `Disabled`; `simthing-sim` remains map-free; Resource Flow E-11 remains default-off;
`request_atlas_batching` stays rejected at admission.
