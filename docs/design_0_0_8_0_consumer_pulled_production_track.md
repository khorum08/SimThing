# SimThing — Design 0.0.8.0 Consumer-Pulled Production Track

> **Local Patrol Economy 0.0.8.0 vertical slice COMPLETE / PARKED after `DEMO-0080-0`** (2026-06-02).
> Scenario → production path → schedule → observation → control → demo/export are all IMPLEMENTED / PASS.
> **No further work opens on this slice without a new named product scenario or explicit product
> authorization.** Closeout: [`tests/phase_local_patrol_economy_0080_closeout_results.md`](tests/phase_local_patrol_economy_0080_closeout_results.md).
>
> **Second scenario COMPLETE / PARKED after `SCENARIO-0080-1-CLOSE-0`** (2026-06-02).
> `SCENARIO-0080-1` — **Nested Starmap (Terran/Pirate multi-theater)** — is complete end-to-end:
> scenario → atlas → econ-scale → production path → schedule → observation → control → demo/export are all
> IMPLEMENTED / PASS (155-test chain green at close). It opened and resolved two previously-parked substrate
> gates: `ATLAS-0080-0` (sparse-residency nested mapping) and `ECON-SCALE-0080-0` (faction-index ECON,
> pirate as full economy faction), both **IMPLEMENTED / PASS**. **No further work opens on this slice
> without a new named product scenario or explicit product authorization.** Closeout:
> [`tests/phase_scenario_0080_1_closeout_results.md`](tests/phase_scenario_0080_1_closeout_results.md).
> Packet: [`scenarios/scenario_0080_1_admission_packet.md`](scenarios/scenario_0080_1_admission_packet.md);
> review: [`tests/phase_scenario_0080_1_opening_review_results.md`](tests/phase_scenario_0080_1_opening_review_results.md).
>
> **Status:** OPEN. `SCENARIO-0080-0` (Local Patrol Economy) **ACCEPTED** (2026-06-02);
> `PRODUCTION-PATH-0080-0` **IMPLEMENTED / PASS** as a Local Patrol Economy opt-in production path
> scoped to Local Patrol Economy on the 0.0.7.9 mobility/transfer substrate; opening spec:
> [`production_paths/production_path_0080_0_opening_spec.md`](production_paths/production_path_0080_0_opening_spec.md).
> Implementation report:
> [`tests/phase_production_path_0080_0_impl_results.md`](tests/phase_production_path_0080_0_impl_results.md).
> `DEFAULT-SCHEDULE-0080-0` is **IMPLEMENTED / PASS - 1A schedule + patrol and 1B bounded pirate loop**
> as a scenario-scoped schedule (reports: [`tests/phase_default_schedule_0080_0_impl_1a_results.md`](tests/phase_default_schedule_0080_0_impl_1a_results.md),
> [`tests/phase_default_schedule_0080_0_impl_1b_results.md`](tests/phase_default_schedule_0080_0_impl_1b_results.md));
> **not** a global default schedule.
>
> **Active constitution:** [`design_0_0_8_0.md`](design_0_0_8_0.md).
>
> **0.0.7.9 mobility/transfer substrate:** COMPLETE and PARKED
> ([`design_v7_9_mobility_transfer_allocation_production_track.md`](design_v7_9_mobility_transfer_allocation_production_track.md)).
> No parked substrate moves to production until a **named product scenario** pulls it.

---

## 1. Purpose

0.0.8.0 redirects effort from **substrate-ahead-of-need** to **consumer-pulled integration**
(`design_0_0_8_0.md` §1, §5). The 0.0.7.9 track proved large, correct, opt-in/default-off substrate
that nothing in production consumed — and generated hygiene loops (recombination soaks, accounting-over-
accounting, passive proof wrappers) while waiting for a consumer.

**This track operationalizes SCENARIO-FIRST authoring:** the next artifact is a **named product
scenario**, not another fixture, soak, replay, accounting summary, manifest, or substrate expansion.
A substrate's production-path gate opens **only because a named scenario consumes it** — never
speculatively.

This is not implementation. This is not a substrate track. This is not another proof wrapper.

**ClauseThing / ClauseScript** remains a horizontal future authoring surface. Scenario admission
must not assume ClauseThing support, but should avoid choices that would make a future ClauseThing
front-end impossible.

---

## 2. Operating doctrine summary

- **Guardrails at designer/spec admission; runtime enforces hard safety** (`design_0_0_8_0.md` §2.1;
  `invariants.md` two-layer guardrail placement).
- **Tier-1 fast-lane** — accepted, generic/semantic-free, opt-in/default-off, parity-backed,
  reversible work ships as one implementation PR + one test report + one status-row update
  (`phase_m_gating_and_doc_policy.md` §1).
- **Tier-2 gate** — new scenario gates, default `SimSession`, default schedule, gameplay surface,
  semantic WGSL, invariant edits, new architecture, or closed-ladder reopen.
- **Proven-capability stop rule** — no passive proof wrappers unless they unlock execution
  (`phase_m_gating_and_doc_policy.md` §6; 0.0.7.9 track §2.2).
- **One principle per class; no per-slice accretion** (`design_0_0_8_0.md` §2.5).

---

## 3. ClauseThing horizon non-goal

ClauseThing / ClauseScript is a **horizontal future designer-facing aspiration** — not active
scope for this track.

- This track does **not** open ClauseThing.
- This track does **not** require ClauseScript expressiveness.
- This track does **not** alter `simthing-spec`.
- Scenario packets should target the **current accepted `simthing-spec` admission surface**
  (L0/L1/L2 / CLAUSE-SPEC).
- Future ClauseThing compatibility is a **non-blocking horizon concern only** — avoid admission
  choices that would foreclose a later front-end, but do not design for ClauseScript now.

---

## 4. Parked substrate inventory

Compact reference only — do not re-litigate. Each row is **complete or accepted at first slice,
opt-in/default-off, parked** until a named scenario pulls it.

| Capability | State | Opens when… |
|---|---|---|
| **0.0.7.9 mobility/transfer substrate** (ALLOC, REENROLL, IDROUTE, ECON, OWNER + RUNTIME-0/1A/1B + semantic-free GPU kernel substrate) | COMPLETE + PARKED | a named scenario needs mobility/ownership/economy in the default `SimSession` path |
| **Line A — nested Resource Flow (A-0)** | ACCEPTED, static nested first slice; production posture is `FlatStarResourceFlow` | a named economy needs depth>2 nested fanout |
| **Line B — discrete hard-currency ordering (B-0)** | ACCEPTED, narrow smoke; no B-1 | a named multi-transaction hard-currency workload |
| **Line C — atlas / multi-theater mapping (C-0/C-1/C-2)** | ACCEPTED; map batching CLOSED at designer surface | a named multi-theater scenario opens the atlas production runtime gate |
| **simthing-spec / CLAUSE-SPEC (L0/L1/L2)** | ACCEPTED designer-admission substrate | engine of this track's scenario authoring |
| **ClauseThing / ClauseScript (L3)** | PARKED pending product authorization | product authorizes the front-end |
| **Deferred-by-design** | E-11B-5, atlas production runtime, B-1, Hybrid-Strata/faction-index ECON scaling, FrontierV2-5, ACT/EVENT/OBS/PIPE | each requires its own named product scenario or product authorization — not open questions |

---

## 5. First gate: SCENARIO-0080-0

| Field | Value |
|---|---|
| **Gate ID** | `SCENARIO-0080-0` |
| **Type** | Tier-2 scenario/admission gate |
| **Deliverable** | Named product-scenario / admission packet |
| **Packet** | [`scenarios/scenario_0080_0_admission_packet.md`](scenarios/scenario_0080_0_admission_packet.md) — **Local Patrol Economy** — **ACCEPTED 2026-06-02** ([acceptance review](tests/phase_scenario_0080_0_acceptance_review_results.md)) |
| **Design-authority enrichment** | Patrol relocate decision sourced from the accepted GPU-resident SEAD `Threshold`+`EmitEvent`→`BoundaryRequest` posture (not a CPU planner; no new substrate pulled) — scenario exercises SEAD + Ownership + Flow |
| **Runtime implementation** | **No** |
| **On acceptance** | Opened **only** the mobility/transfer production-path gate (`PRODUCTION-PATH-0080-0`, below) |

---

## 6. SCENARIO-0080-0 admission packet — required declarations

The admission packet must declare:

1. **Scenario name and product purpose**
2. **User-facing / product behavior** the scenario is meant to enable
3. **Which one parked substrate** it consumes (exactly one)
4. **Why that substrate is required now** (consumer pull — not speculative substrate build)
5. **Bounds and scale**
6. **Designer/spec admission vocabulary**
7. **Rejection vocabulary and diagnostics**
8. **Runtime path requested**, if any
9. **Whether production default path is requested**
10. **Whether default schedule is requested**
11. **Whether gameplay-facing integration is requested**
12. **Test/evidence needed for admission**
13. **Explicit non-goals**
14. **Exit criteria for acceptance**

Acceptance is **design-authority + product only**. No self-acceptance.

---

## 7. Recommended first candidate (not authorization)

**Most-ready candidate:** 0.0.7.9 mobility/transfer substrate.

**Reason:** Its production-path gate is already mapped as the first non-test-support default
`SimSession` path / default schedule (0.0.7.9 track top status; MOBILITY-SCENARIO-0 accepted the
scenario discipline).

**Constraint:** It must still be **pulled by a named product scenario**. Do not open
`PRODUCTION-PATH-0080-0` speculatively.

---

## 8. Stop conditions

Reject the scenario if it requires any of:

- owner-entity as spatial parent
- capture-as-reparenting
- semantic/raw WGSL
- default-on behavior without a production gate
- hard-currency through Resource Flow
- CPU planner / urgency / commitment emission
- reopening atlas runtime, E-11B-5, B-1, ClauseThing/L3 front-end, FrontierV2-5, ACT/EVENT/OBS/PIPE,
  Hybrid-Strata/faction-index scaling, or any closed ladder without its own product authorization
- passive proof wrappers that do not unlock execution

---

## 9. Ladder index

| Gate | Description | Status |
|---|---|---|
| `SCENARIO-0080-0` | Scenario/admission packet (Local Patrol Economy) | **ACCEPTED (2026-06-02)** |
| `PRODUCTION-PATH-0080-0` | First substrate production-path gate | **IMPLEMENTED / PASS - Local Patrol Economy opt-in production path** — scoped to: *Local Patrol Economy using the 0.0.7.9 mobility/transfer substrate*, patrol relocate decision sourced from the accepted GPU-resident SEAD `Threshold`+`EmitEvent`→`BoundaryRequest` path (mobility/transfer is the only substrate wired; no CPU planner; no new SEAD production gate; no global default schedule) |
| `DEFAULT-SCHEDULE-0080-0` | Scenario-scoped schedule for Local Patrol Economy | **IMPLEMENTED / PASS - 1A schedule + patrol and 1B bounded pirate loop** — deterministic opt-in step driver routes GPU-resident SEAD threshold/event/`BoundaryRequest` decisions into `PRODUCTION-PATH-0080-0`; pirate is a second IDROUTE identity, not a second economy owner; pirate disruption, supply drain, threshold relocation, `local_security` evasion scoring, and deterministic cat-and-mouse assertions are implemented; **not a global default schedule**. Spec: [`production_paths/default_schedule_0080_0_opening_spec.md`](production_paths/default_schedule_0080_0_opening_spec.md); reports: [`tests/phase_default_schedule_0080_0_impl_1a_results.md`](tests/phase_default_schedule_0080_0_impl_1a_results.md), [`tests/phase_default_schedule_0080_0_impl_1b_results.md`](tests/phase_default_schedule_0080_0_impl_1b_results.md). |
| `GAMEPLAY-0080-0` | Read-only Local Patrol Economy observation surface | **IMPLEMENTED / PASS — read-only Local Patrol Economy observation export** — consumes `DefaultSchedule0080RunReport`; exports deterministic tick transcript + summary via `observe_gameplay_0080_0`; explicit opt-in/default-off. **Player control / command input / UI framework / real-time loop remain CLOSED.** Spec: [`gameplay/gameplay_0080_0_opening_spec.md`](gameplay/gameplay_0080_0_opening_spec.md); impl: [`tests/phase_gameplay_0080_0_impl_results.md`](tests/phase_gameplay_0080_0_impl_results.md). |
| `CONTROL-0080-0` | Bounded Local Patrol Economy command admission | **IMPLEMENTED / PASS — bounded Local Patrol Economy command admission** — opt-in/default-off command vocabulary writes only existing `DefaultSchedule0080Input` bounded values/config, then runs schedule→`observe_gameplay_0080_0`; commands never move a mover, emit a `BoundaryRequest`, or bypass SEAD. **Direct movement control / player command loop / UI framework / real-time loop remain CLOSED.** Spec: [`gameplay/control_0080_0_opening_spec.md`](gameplay/control_0080_0_opening_spec.md); impl: [`tests/phase_control_0080_0_impl_results.md`](tests/phase_control_0080_0_impl_results.md). |
| `DEMO-0080-0` | Headless Local Patrol Economy demo/export packaging | **IMPLEMENTED / PASS — headless Local Patrol Economy demo/export library helper** — deterministic opt-in/default-off; canonical `Control0080CommandBatch::canonical_run()`; existing control→schedule→observation/export path via `run_demo_0080_0`; day-to-day patrol/pirate movement record in impl report. **No CLI binary.** Direct movement control, player command loop, UI framework, real-time loop, global default schedule remain CLOSED. Spec: [`gameplay/demo_0080_0_opening_spec.md`](gameplay/demo_0080_0_opening_spec.md); impl: [`tests/phase_demo_0080_0_impl_results.md`](tests/phase_demo_0080_0_impl_results.md). |
| `SCENARIO-0080-1` | Second scenario — Nested Starmap (Terran/Pirate multi-theater) | **ACCEPTED (2026-06-02)** — nested `session → starmap(10×10) → 10 starsystems(10×10) → planet(10×10 submap)`; owner overlays inherit personality/policy weights broadcast from faction-owner simthings (OWNER down-broadcast); ownership up-aggregation (planet→starsystem) as a derived overlay; Terran patrol + pirate as a **full economy faction**; SEAD-sourced composite-gap decision; opt-in/default-off. Pulls two parked substrates (below). Packet: [`scenarios/scenario_0080_1_admission_packet.md`](scenarios/scenario_0080_1_admission_packet.md). |
| `ATLAS-0080-0` | Atlas production runtime / sparse-residency nested mapping | **IMPLEMENTED / PASS — scenario-scoped sparse-residency nested mapping runtime for Nested Starmap** — opened by `SCENARIO-0080-1` as the named multi-theater consumer (the *first-slice gating* the invariant contemplates); opt-in sparse residency + nested theater descent/ascent; residency is a strict value no-op (I8 parity); **no default session pass-graph wiring**. Spec: [`production_paths/atlas_0080_0_opening_spec.md`](production_paths/atlas_0080_0_opening_spec.md); report: [`tests/phase_atlas_0080_0_impl_results.md`](tests/phase_atlas_0080_0_impl_results.md). |
| `ECON-SCALE-0080-0` | Multi-faction (Hybrid-Strata/faction-index) ECON scaling | **IMPLEMENTED / PASS — bounded faction-indexed contended ECON scaling for Nested Starmap** — opt-in/default-off; Terran + Pirate fixed bounded faction set; pirate is a full economy faction (adversarial participant in starsystem resource flow, extracts not merely disrupts); deterministic integer contended clearing with a CPU parity oracle; subsidiarity / FlatStar posture preserved; **no hard currency / markets / trade / `ai_budget`, no nested RF, no unbounded factions**. Default single-owner ECON unchanged when disabled. Spec: [`production_paths/econ_scale_0080_0_opening_spec.md`](production_paths/econ_scale_0080_0_opening_spec.md); impl: [`tests/phase_econ_scale_0080_0_impl_results.md`](tests/phase_econ_scale_0080_0_impl_results.md). |
| `PRODUCTION-PATH-0080-1` | `SCENARIO-0080-1` opt-in production path | **IMPLEMENTED / PASS — opt-in Nested Starmap production-path composition** — composes implemented/pass `ATLAS-0080-0` (sparse residency) + `ECON-SCALE-0080-0` (faction-index ECON) reports into one inspectable scenario report; owner-overlay inheritance + ownership up-aggregation are numeric summaries; SEAD composite-gap terms are read-only; **no schedule/movement, no new substrate**. Spec: [`production_paths/production_path_0080_1_opening_spec.md`](production_paths/production_path_0080_1_opening_spec.md); impl: [`tests/phase_production_path_0080_1_impl_results.md`](tests/phase_production_path_0080_1_impl_results.md). |
| `DEFAULT-SCHEDULE-0080-1` | `SCENARIO-0080-1` schedule / movement | **IMPLEMENTED / PASS — scenario-scoped Nested Starmap SEAD-sourced schedule/movement** — deterministic opt-in/default-off step driver that consumes `PRODUCTION-PATH-0080-1` and turns read-only SEAD composite-gap terms into live movement via `Threshold + EmitEvent → BoundaryRequest`, routed through the existing mobility/transfer substrate posture (Terran + Pirate ships); preserves identity + owner overlays, updates membership without reparenting; **not a global default schedule; no observation/control/demo; no direct move; no new substrate**. Spec: [`production_paths/default_schedule_0080_1_opening_spec.md`](production_paths/default_schedule_0080_1_opening_spec.md); impl: [`tests/phase_default_schedule_0080_1_impl_results.md`](tests/phase_default_schedule_0080_1_impl_results.md). |
| `GAMEPLAY-0080-1` | `SCENARIO-0080-1` read-only observation/export | **IMPLEMENTED / PASS - read-only Nested Starmap observation/export** - read-only consumer of `DEFAULT-SCHEDULE-0080-1` run reports (`DefaultSchedule0081RunReport`) via `observe_gameplay_0080_1`; exports deterministic atlas residency, faction-index ECON, owner-overlay/up-aggregation, SEAD movement trace, and Terran/Pirate movement rows; opt-in/default-off, non-interactive, mutates nothing beyond optional explicit schedule invocation. No control/command input, demo packaging, UI, or real-time loop. Control/demo for `0080-1` remain not opened; direct movement control, external boundary requests, CPU planner, global default schedule, semantic WGSL, new shader/GPU kernel, hard currency, nested RF, ClauseThing/L3, UI/realtime, and parked ladders remain closed/parked. Spec: [`gameplay/gameplay_0080_1_opening_spec.md`](gameplay/gameplay_0080_1_opening_spec.md); impl: [`tests/phase_gameplay_0080_1_impl_results.md`](tests/phase_gameplay_0080_1_impl_results.md). |
| `CONTROL-0080-1` | `SCENARIO-0080-1` bounded command admission | **IMPLEMENTED / PASS - bounded Nested Starmap command admission** - opt-in/default-off deterministic command vocabulary that writes only existing `DefaultSchedule0081Input` bounded schedule values plus bounded Nested Starmap control config, then runs the existing `DEFAULT-SCHEDULE-0080-1` -> `GAMEPLAY-0080-1` path. Commands never move a ship, emit an external `BoundaryRequest`, or bypass SEAD (movement still emerges from the implemented `Threshold + EmitEvent -> BoundaryRequest` schedule). No direct movement control, player command loop, UI framework, real-time loop, demo packaging, or global default schedule. Spec: [`gameplay/control_0080_1_opening_spec.md`](gameplay/control_0080_1_opening_spec.md); impl: [`tests/phase_control_0080_1_impl_results.md`](tests/phase_control_0080_1_impl_results.md). |
| `DEMO-0080-1` | `SCENARIO-0080-1` headless demo/export packaging | **IMPLEMENTED / PASS — headless Nested Starmap demo/export library helper** — deterministic opt-in/default-off; canonical `Control0081CommandBatch::canonical_run()`; existing `control → DEFAULT-SCHEDULE-0080-1 → GAMEPLAY-0080-1` path via `run_demo_0080_1`; report includes atlas residency, faction-index ECON, owner-overlay/up-aggregation, SEAD movement trace, Terran/Pirate movement rows, command transcript, and replay checksum. **No CLI binary.** Direct movement control, player command loop, UI framework, real-time loop, and global default schedule remain CLOSED. Spec: [`gameplay/demo_0080_1_opening_spec.md`](gameplay/demo_0080_1_opening_spec.md); impl: [`tests/phase_demo_0080_1_impl_results.md`](tests/phase_demo_0080_1_impl_results.md). |
| `SEMANTIC-WGSL-0080-0` | Semantic shader surface | **CLOSED** |
| `CLAUSETHING-L3-0080-0` | Front-end / parser / product authoring surface | **PARKED** pending product authorization |

Opening spec of record: [`production_path_0080_0_opening_spec.md`](production_paths/production_path_0080_0_opening_spec.md).
Implementation report: [`phase_production_path_0080_0_impl_results.md`](tests/phase_production_path_0080_0_impl_results.md).
`DEFAULT-SCHEDULE-0080-0` is **IMPLEMENTED / PASS - 1A schedule + patrol and 1B bounded pirate loop**
as an opt-in scenario-scoped schedule (reports: [`phase_default_schedule_0080_0_impl_1a_results.md`](tests/phase_default_schedule_0080_0_impl_1a_results.md),
[`phase_default_schedule_0080_0_impl_1b_results.md`](tests/phase_default_schedule_0080_0_impl_1b_results.md)).
`GAMEPLAY-0080-0` is **IMPLEMENTED / PASS — read-only Local Patrol Economy observation export**
(spec: [`gameplay/gameplay_0080_0_opening_spec.md`](gameplay/gameplay_0080_0_opening_spec.md);
impl: [`tests/phase_gameplay_0080_0_impl_results.md`](tests/phase_gameplay_0080_0_impl_results.md)).
`CONTROL-0080-0` is **IMPLEMENTED / PASS — bounded Local Patrol Economy command admission**
(spec: [`gameplay/control_0080_0_opening_spec.md`](gameplay/control_0080_0_opening_spec.md);
impl: [`tests/phase_control_0080_0_impl_results.md`](tests/phase_control_0080_0_impl_results.md)) —
commands write only existing bounded scenario input/config and never bypass SEAD or move a mover.
`DEMO-0080-0` is **IMPLEMENTED / PASS — headless Local Patrol Economy demo/export library helper**
(spec: [`gameplay/demo_0080_0_opening_spec.md`](gameplay/demo_0080_0_opening_spec.md);
impl: [`tests/phase_demo_0080_0_impl_results.md`](tests/phase_demo_0080_0_impl_results.md);
**No CLI binary**). **Direct movement control / externally-scripted move requests / player command loop**, UI framework, real-time loop, CLI binary, semantic WGSL, **global default
schedule**, new shader/GPU kernel, ClauseThing/L3, Hybrid-Strata/faction-index scaling, atlas runtime,
E-11B-5, B-1, FrontierV2-5, and ACT/EVENT/OBS/PIPE remain closed/parked.

---

## 10. Evidence / test-report policy

- **This track file:** no test report required.
- **Future SCENARIO-0080-0 admission:** one compact admission report only:
  [`docs/tests/phase_scenario_0080_0_admission_results.md`](tests/phase_scenario_0080_0_admission_results.md)
  — **authored** for Local Patrol Economy proposal; verdict PROPOSED / READY-FOR-DESIGN-AUTHORITY.
- **PRODUCTION-PATH-0080-0 opening spec:** one compact visibility report:
  [`docs/tests/phase_production_path_0080_0_opening_spec_results.md`](tests/phase_production_path_0080_0_opening_spec_results.md)
  — **authored**; no implementation.
- **PRODUCTION-PATH-0080-0 implementation:** one compact implementation report:
  [`docs/tests/phase_production_path_0080_0_impl_results.md`](tests/phase_production_path_0080_0_impl_results.md)
  — **PASS**; opt-in/default-off Local Patrol Economy only.
- **DEFAULT-SCHEDULE-0080-0 implementation 1A:** one compact implementation report:
  [`docs/tests/phase_default_schedule_0080_0_impl_1a_results.md`](tests/phase_default_schedule_0080_0_impl_1a_results.md)
  — **PASS**; scenario-scoped schedule + patrol loop only.
- **DEFAULT-SCHEDULE-0080-0 implementation 1B:** one compact implementation report:
  [`docs/tests/phase_default_schedule_0080_0_impl_1b_results.md`](tests/phase_default_schedule_0080_0_impl_1b_results.md)
  — **PASS**; bounded pirate loop, second IDROUTE identity only, `local_security` evasion, deterministic cat-and-mouse included.
- Do not create reports for passive proof wrappers.
- Do not create per-slice ceremony.

---

## 11. SCENARIO-0080-1 PR ladder (Codex develops; Opus design-authority at the gates)

**Gate protocol.** Opus (design authority) authors/adjudicates every **OPEN** rung (the opening spec is
the gate: scope, bounds, stop conditions, named-but-unimplemented tests) and adjudicates every
**ACCEPT** review. Codex develops the **IMPL** rungs between gates. Each IMPL rung must: implement only
within the accepted opening spec; keep all prior `0080-0` and `0080-1` regression suites green; hold
CPU-oracle bit-exact parity (I8) for any GPU-resident field; ship one compact impl report; update the
ladder index + mapping guidance + worklog; and **stop-and-escalate to Opus** on any stop-condition
crossing (default-on session wiring, real-time loop, UI framework, direct movement control, hard
currency/markets/trade/`ai_budget`, nested Resource Flow depth, semantic/raw WGSL, capture-as-reparenting,
owner-as-spatial-parent, ClauseThing, invariant edit) rather than proceeding. Openings are docs-only;
implementations are opt-in/default-off and reversible.

| Rung | PR id | Deliverable (Codex, unless OPEN) | Gate (Opus) | Depends on |
|---|---|---|---|---|
| 1 | `ATLAS-0080-0-IMPL-0` | Opt-in sparse-residency nested-mapping runtime **+ deterministic nested structure scaffold** (`session → starmap(10×10) → 10 seeded starsystems(10×10) → one planet(10×10 submap) each`); theater descent/ascent; residency a strict value no-op with I8 parity; no default session pass-graph wiring | **ACCEPT** vs `atlas_0080_0_opening_spec.md` | ATLAS opening spec (merged) |
| 2 | `ECON-SCALE-0080-0-IMPL-0` | Opt-in faction-indexed contended resource flow; bounded fixed faction set; subsidiarity preserved; I8 parity; no hard currency / nested RF | **ACCEPT** vs `econ_scale_0080_0_opening_spec.md` | rung 1 |
| 3 | `PRODUCTION-PATH-0080-1-OPEN-0` | *(Opus authors)* Opening spec: initial conditions (§ scenario packet), owner overlays + personality/policy down-broadcast, ownership up-aggregation (planet→starsystem, derived overlay), ships-as-movers, pirate adversarial RF on starsystem entry, SEAD-sourced composite-gap decisions | **OPEN** (author + adjudicate) | rungs 1–2 |
| 4 | `PRODUCTION-PATH-0080-1-IMPL-0` | Implement the scenario production path per rung-3 spec (opt-in) | **ACCEPT** vs rung-3 spec | rung 3 |
| 5 | `DEFAULT-SCHEDULE-0080-1-OPEN-0` | *(Opus authors)* Opening spec: deterministic multi-tick schedule (ships move, pirates raid/contend, ownership + up-aggregation update per tick) | **OPEN** (author + adjudicate) | rung 4 |
| 6 | `DEFAULT-SCHEDULE-0080-1-IMPL-0` | Implement the deterministic schedule (bounded steps, deterministic replay) | **ACCEPT** vs rung-5 spec | rung 5 |
| 7 | `GAMEPLAY-0080-1-IMPL-0` | Read-only observation/export of the nested-starmap run (reuse the proven `observe_*` pattern; opening folded — pattern already proven at `0080-0`) | **ACCEPT** | rung 6 |
| 8 | `CONTROL-0080-1-OPEN-0` *(optional)* | *(Opus authors)* Opening spec: bounded command admission **+ player-orders as a weighted overlay term** on the action vector (never direct-move, never the currency mechanism) | **OPEN** (author + adjudicate) | rung 7 |
| 9 | `DEMO-0080-1-IMPL-0` | Headless demo/export library helper (default **No CLI binary**) | **ACCEPT** | rung 7 (or 8) |
| 10 | `SCENARIO-0080-1-CLOSE-0` | *(Opus adjudicates)* Closeout / park review of the full vertical slice | **OPEN** (adjudicate close/park) | rungs complete |

**Sequencing note (design authority):** implement the two parked substrates **serially, not in parallel** —
rung 1 (atlas) is the structural prerequisite the scenario needs to exist at all; rung 2 (econ-scale) is the
heavier lift. If pace demands, rungs 3–7 may prove out the nested structure + inherited-overlay decisions on
atlas first, and fold the pirate's **full-economy** contention (rung 2's consumer) into a later production-path
sub-slice — keeping each parity surface clean.

---

## 12. Next track: full-vertical SimThing dress rehearsal (SCENARIO-0080-2 → engine)

**Why.** Every 0080 slice so far — `0080-0` Local Patrol Economy, `0080-1` Nested Starmap, and the
`SCENARIO-0080-2` Pirate Gradient Pathfinding rung ladder — was **proven at the math/behavioral
layer**: the recurrences, fields, dual-output gradient, SEAD threshold-gated movement, and bit-exact
deterministic replay all hold and remain valid as **CPU oracles**. Under the constitution's
`invariants.md` "Scenario Proof" bar, none is yet proven **through a real SimThing reduction** — they
run on plain `Vec`/struct math with no `SimThing`/`SimProperty`/`Overlay`/`BoundaryProtocol`. This
track closes exactly that gap and validates SimThing **verticality in totality** in one assembled,
opt-in/default-off session.

**Tree under test:**

```
gamesession (root simthing)
  ├─ Terran faction       (+ techtree capability tree)
  ├─ worldstate
  │     └─ starmap gridcell simthings   (disruption + desirability columns live here)
  └─ Pirate faction       (+ techtree capability tree)
```

- **gamesession (root):** carries the global decay parameters as a root **overlay** — read-side
  weights the opt-in columns READ, not a column-wide write and not a global default schedule (the
  approved "gravity to zero" posture). Owns the boundary cadence.
- **worldstate → gridcell simthings:** the starmap. `disruption` is a real `SimProperty` **column on
  each gridcell simthing**, advanced by `AccumulatorOp` (pirate presence emits +, patrol presence
  emits −, root overlay decays it, clamp ceiling). The compound **desirability** is a derived column
  read-only over `disruption`; the **`GradientXY`** kernel runs over the worldstate gridcell slot range.
- **Terran faction (+ techtree):** capability-tree unlocks contribute **retention / suppression
  modifiers** as read-side overlay weights (≤1, acceleration-only on decay) and patrol presence —
  validating capability tree → overlay → column influence with no destructive writes.
- **Pirate faction (+ techtree):** the mover. Movement **emerges** from the gridcell desirability
  gradient via `Threshold`+`EmitEvent`→`BoundaryRequest` (exactly one step per boundary, no CPU
  planner); its techtree modifies disruption emission and movement threshold.

**Principles carried from 0080 (proved as math; to be re-proved through the engine):**

| Principle (0080 origin) | Re-validated through the engine as… |
|---|---|
| BoundedFeedback disruption decay (0080-2 rung 1) | `disruption` `SimProperty` column on gridcell simthings; `AccumulatorOp` recurrence; root-overlay decay weight |
| Decay/patrol modifiers as read-side params (0080-2 rung 1) | faction **techtree capability** → overlay weights composed onto the column (≤1, acceleration-only) |
| Compound desirability field (0080-2 rung 2) | derived desirability column, read-only over `disruption`, per gridcell |
| Dual-output `GradientXY` (0080-2 rung 3) | `StructuredFieldStencilOp::GradientXY` over the worldstate gridcell slot range, now wired into a session |
| SEAD field-as-policy movement (0080-2 rung 4; 0080-0/1 patrol/ship SEAD) | pirate movement via `Threshold`+`EmitEvent`→`BoundaryRequest`, one step/boundary, no CPU planner |
| Disruption/desirability as faction-economy signals (0080-0/0080-1) | gridcell columns read by faction overlays; pirate as adversarial participant |
| Deterministic replay / I8 bit-exact parity | same inputs → identical resolved GPU/CPU values across two runs of the assembled session |

**This is the first scenario authored to satisfy the new "Scenario Proof" gate.** Tier-2 (new
assembled session; gridcell columns wired into a session pass graph). §8 stop conditions still bind:
it pulls the mapping/RegionCell + AccumulatorOp + SEAD substrates through one tree; it does **not**
open atlas production runtime, nested-RF depth, hard currency, ClauseThing/L3, or a real-time/UI loop.

> Dense per-cell temporal memory stays separately gated. The gridcell `disruption` column here is
> per-cell **state advanced across boundaries** by the standard AccumulatorOp + root-overlay decay
> (the bounded-feedback contract) — i.e. the sparse-per-node math of 0080-2 now expressed over real
> cell simthings, **not** the deferred dense-temporal VRAM gate.

### 12.1 Provisional findings (2026-06-03 audit — to be firmed up before the dress-rehearsal opens)

> **Status: PROVISIONAL.** Captured from a design-authority audit of the prior mapping/SEAD/0080 work.
> Not yet ratified into spec; the dress rehearsal exists to resolve them. Work continues here next.

**Gap findings — what prior "passing" work did and did not do:**

- **F1 — 0080 modeled no spatial structure.** The 0080 scenarios ran a **1-D line of 4–5 scalar nodes**
  (source comment: *"simple 1-D line"*; `y` always 0). No grid, no field, no heatmap. The token
  "heatmap" appears **nowhere** in the code.
- **F2 — the mapping track built real 2-D field machinery but never demoed a heatmap.** 10×10 grid,
  stencil diffusion, gradient extraction (M-5A–E), 100-cell→parent reduction via `field_urgency`, all
  **GPU/CPU bit-exact**. But the field was **hand-seeded** (`CallerManagedOneShotSeedThenZero`, not
  gameplay-produced), **never exported/rendered/demoed** as a heatmap (deliverable = parity asserts),
  and **never run through SimThing cells** (flat slot-range `0..100 → 100`, not children of a starmap
  SimThing). The acceptance bar was *numeric-pipeline correctness*, not a heatmap artifact.
- **F3 — engine and consumer never met.** Mapping proved the machinery and closed at "primitive proven,
  unconsumed"; the consumer that should have pulled it (0080-2) **bypassed it with the 1-D toy**.
- **F4 — SEAD never consumed a heatmap for pathing/critical-path.** SEAD-OBS scores an entity's **own
  overlays** (`ExactQ16WeightedSum`), not a spatial field; SEAD-EVENT/PIPE/ACT are event→proposal
  downstream; FrontierV1-4's "SEAD route" `validate_sead_v1_consumed()` only **asserts two kernel
  descriptors are registered** (field computed in the same fixture but never read by SEAD); 0080
  "pathing" was a scalar `supply*100 − disruption*10 − security` over 2 nodes. `field_urgency`
  critical-path existed as plumbing over hand-seeds, never a gameplay-driven agent decision.
- **F5 — the connecting tissue was never implemented.** Both halves (*field → heatmap* and *SEAD →
  action*) were built and "proven" separately; the loop **field → diffuse → gradient → SEAD reads local
  cell → action** was never wired. Every prior "pass" satisfied the two ends and never the connection.

**Provisional design resolutions (from the §12 discussion — confirm tomorrow):**

- Falloff is **stencil diffusion**, not arena enrollment; **two-column model** — `disruption` (source,
  arena accumulator, BoundedFeedback decay) → `location_status` (sink, stencil-written falloff,
  strict-sink `source_col != target_col`). Neighbors are **not enrolled**; the dense pass sweeps the
  whole grid. Falloff is a property field, **not** an overlay (the decaying *ownership* signal is the overlay).
- **Sparse arenas + dense diffusion**: arenas instance only on occupied cells; one stencil pass covers
  all 100 cells; they meet at the `disruption` column.
- **Diffusion horizon = SEAD sight radius** (myopic local read sees H hops because diffusion pre-bakes
  distant info into the local gradient); **recursion (reduce-up / broadcast-down) = multi-resolution
  escape from local optima** (coarse starmap field biases the fine cell gradient).
- Grid-of-simthings requires **contiguous, row-major slot allocation** for the stencil's neighbor
  arithmetic — prerequisite to confirm/build in `SlotAllocator`.
- Ownership is a **decaying owner overlay** (D=2), not an ownership tree-node (rejected D=3 — the
  canonical §0.0 conformance violation).

**Proposed hard exit criteria (provisional — these would have failed every prior "pass"):**

- **EC1 (heatmap):** the starmap SimThing holds a **non-trivial reduced disruption field over its 100
  child gridcell SimThings**, produced by **pirate/patrol presence (not hand-seeded)**, verified against
  a CPU oracle, and **emitted as an inspectable heatmap artifact**.
- **EC2 (SEAD consumption):** a mover's SEAD evaluation **reads the diffused heatmap gradient at its own
  cell**, and the **emitted action is a function of that gradient** (verified against a CPU oracle) —
  **not** a hand-seeded field or a registration-only stand-in. The field → gradient → SEAD → action loop
  is closed end-to-end through real SimThings.

### 12.2 Key concept — the recursive nested-grid field hierarchy (design note)

> **Status: PROVISIONAL design note (2026-06-03).** The substrate concept the EC1/EC2 exit criteria
> build on. Not yet a gate.

**The idea.** `Location`-kind SimThings are the SEAD field primitives ("gridcells"). Any non-Location
SimThing participates in resource flow like everything else. A gridcell knows its `(x, y)` within its
parent's grid; every gridcell enrolls in the `location_val` flow arena. **Every gridcell that is a
parent of gridcells maintains a 2-D map siloing its children's reduced values at each child's `(x, y)`.**
Aggregate / velocity evaluation happens at that tier on the 2-D map, and a summary reduces up to the
parent (gridcell or not). It is **sparse** — only gridcells with gridcell children materialize a map —
and **recursive**: planet surface → planet/moon map → star system → galactic starmap share one
reduction/evaluation behavior.

**The "for free" property (storage), and its one condition.** The value buffer is
`n_slots × n_dims × 4 B` and the stencil addresses cells as `slot = base + row·width + col`. **If slots
are laid out to mirror topology, the buffer *is* the nested 2-D maps — a view, not extra memory.** A
system's 100 child cells as one contiguous row-major block already *is* its 10×10 map; the 2000 system
slots in galactic-grid order already *are* the galactic map. So nested 2-D legibility is free in
**storage**, conditioned on **slot layout mirroring grid topology at every tier**.

**VRAM at 2000+ systems × 10×10 (the field is not the constraint):** ~200K leaf cells + ~2K systems ≈
202K field slots.

| `n_dims`/cell | Single buffer | Ping-pong (×2) |
|---|---|---|
| 8 | 6.4 MB | 12.8 MB |
| 16 | 12.9 MB | 25.6 MB |
| 32 | 25.6 MB | 51.2 MB |
| 64 | 51.2 MB | 102 MB |

A realistic cell (~16–24 cols) puts the whole galactic field at **~25–80 MB double-buffered** — trivial.
**Breadth (2000 systems) is free; depth (recursion) is the cost:** full recursion (a 10×10 planet
surface under *every* leaf) is ~20M cells ≈ 1.3 GB — which is exactly why **sparsity (only occupied
interior nodes materialize a deeper grid)** is the binding cost lever, not VRAM at the top tier.

**What is *not* free:** (a) the rigid grid-ordered contiguous slot layout — reserves full dense tiles
even when mostly empty, and resists REENROLL slot recycling; (b) per-tier compute (stencil / `GradientXY`
/ reduction / velocity passes) — cheap and parallel, but real; (c) reducing children to the parent
collapses 2-D to a scalar, so the parent keeps **both** the child block (free 2-D map) **and** a +1
summary column.

**Proven/parked service map:**
- per-system 10×10 tiles batched in one buffer = the parked **ATLAS** substrate (C-2 closed the
  designer surface for bounded **algebraic tile-local G=0**, homogeneous-square tiles — exactly 2000
  homogeneous 10×10 masked tiles). **This design is the named multi-theater consumer that opens the
  parked atlas production-runtime gate (§4).**
- fine atlas (leaf cells) + coarse galactic grid (system reductions) = the multi-resolution pair; both
  are single-grid stencil fields.
- `StructuredFieldStencilOp` diffusion, `GradientXY`, `SlotRange` reduction, `field_urgency` EvalEML,
  `VelocityMonitor` (explicit prev-column) — **all proven, reusable.**
- **One genuinely new primitive:** the reduction target is *a cell `(x,y)` in the coarser parent grid*
  (not a free-standing scalar), applied recursively — "silo the child block into the parent's 2-D map."

**Decision that determines everything — slot layout fork:**
- **Dense, grid-ordered, contiguous** (recommended default): free 2-D view, cheap slot-arithmetic
  stencil, atlas-batchable — but reserves full grids and resists REENROLL.
- **Sparse with explicit `(x,y)` coordinate columns:** saves slots, REENROLL-friendly — but needs a
  scatter/gather to assemble the 2-D map and forfeits slot-arithmetic neighboring.
- **Recommended split:** the **field/heatmap is dense** (per materialized tier, atlas-batched, free 2-D
  view), the **movers (fleets) are sparse** and REENROLL between cells, layered on top of the dense
  field. Sparsity lives at *which tiles materialize at all*, not within a tile.

**Conformance note:** binding `Location` ⇒ field-primitive is conformant **iff** `kind` is only the
install-time selector (which cells get `location_val` + the 2-D-map column when they have gridcell
children); the runtime still reads behavior from properties/overlays/arena registrations and never
branches on `Location` (§0.0/§0.1).

### 12.3 Pre-Rehearsal track — `ATLAS-BATCH-0` (build + validate before the dress rehearsal)

> **Status: PROVISIONAL track definition (2026-06-03), design authority. Tier-2** (opens the parked
> atlas production-runtime gate — **batch allocation only**). To be laddered and accepted before the
> dress rehearsal (§12) opens.

**Purpose.** Stand up and prove **atlas batch allocation** on a **static, pre-generated** multi-theater
map — the named multi-theater consumer the M-4 / M-4A atlas runtime gate was parked for (constitution
§4; C-0/C-1/C-2 closed the designer surface). The static map deliberately **isolates batch allocation**
from the **sparse-residency scheduler (M-4A)** and from **REENROLL** — both stay parked (a static map
exercises neither). Establishes the Location-kind gridcell primitive and the 2-D-map storage that the
dress-rehearsal EC1/EC2 build on.

**Scope — what this track builds:**
1. **Simple static map generator (simulated).** Produces a fixed atlas at game start: a galactic
   **100×100 grid with ~1000 stars** dispersed (random or galaxy-shaped algorithm); each star system's
   **10×10 subgrid** with planet-system positions; each planet system's subgrid with moon/orbital
   positions. **All static at game start.** No procedural-generation runtime — it is a test/fixture
   producer the batcher consumes.
2. **Location-kind gridcell primitive.** `Location` SimThings carry grid `(x,y)` and a `width×height`
   dense map; grid-placement allocation reserves a dense contiguous cell-slot range with
   `cell(x,y) = map_base + y·width + x` (the free 2-D view, §12.2). Sparse by tier (only Locations with
   gridcell children materialize a map). `Location` is an **install-time selector only** — the runtime
   never branches on kind (§0.1).
3. **Atlas batch allocation.** Pack the ~1000 homogeneous 10×10 star tiles (+ planet/moon tiles, batched
   per homogeneous size-class) into batched buffers with **algebraic tile-local `G=0` masking** (no
   inter-tile bleed; systems couple only via the galactic-tier reduction), within the declared
   `V78AtlasVramBudget`, with **mandatory VRAM-multiplier reporting**. One batched stencil dispatch flows
   over all tiles of a class; CPU-oracle bit-exact parity.
4. **2-D-map storage of children's flow results** (the §12.2 reduction-target-is-a-cell primitive,
   refined by the binding constraint below).

**BINDING CONSTRAINT — co-located children are never merged.** A Location MAY have **multiple children
at the same `(x,y)`** (e.g. a planet, a patrol fleet, and a pirate fleet in one cell). This **refines and
corrects §12.2's "child is the cell" simplification**: the **cell is its own dense map slot** (the
position); features and movers are **occupants** that contribute *into* the cell. Co-located occupants
are distinguished and **must not be collapsed into one figure**:
- The cell is **multi-channel** (and where needed **owner/faction-indexed**): a planet writes
  food/labor channels; a patrol writes patrol-presence; a pirate writes pirate-presence/disruption —
  distinct channels at the same `(x,y)`.
- The batcher's reduction is **per-channel (and per-owner), never a blind sum-by-position.** Two pirate
  fleets in one cell *do* sum within the pirate-presence channel (correct — more presence); a planet and
  a pirate in one cell **never** sum across their channels.
- Therefore: dense **cell** slots (the map, grid-ordered) + **occupant** children (features + movers)
  that scatter/reduce into the cell's appropriate channel keyed on `(x,y)` + role/owner. This is the
  dense-field / sparse-occupant split, with the **planet now correctly an occupant, not the cell.**

**Exit criteria (provisional):**
- **EC-A1:** the static generator deterministically produces the fixed atlas (galactic 100×100 + ~1000
  star 10×10 subgrids + planet/moon subgrids).
- **EC-A2:** the batcher packs the homogeneous tiles with `G=0` masking within `V78AtlasVramBudget`,
  reports the VRAM multiplier, and one batched stencil dispatch over all tiles matches the CPU oracle
  bit-exactly.
- **EC-A3:** a Location stores its gridcell children's flow results in the correct `(x,y)` map slots, and
  **co-located children at one `(x,y)` are preserved per-channel/per-owner and never merged** — explicit
  test: planet + patrol + pirate in one cell → three distinct channel figures, verified vs CPU oracle.
- **EC-A4:** the residency scheduler (M-4A) and REENROLL remain unbuilt/parked — the slice is static.

**Rungs (provisional):**
1. `ATLAS-BATCH-0-GEN` — static map generator (fixture producer).
2. `ATLAS-BATCH-0-LOC` — Location-kind gridcell primitive + grid-placement slot allocation + multi-channel cell.
3. `ATLAS-BATCH-0-PACK` — atlas batch allocation + `G=0` mask + VRAM-multiplier + batched dispatch + CPU parity.
4. `ATLAS-BATCH-0-STORE` — children's flow results into 2-D map slots; the co-located-not-merged test (EC-A3).
5. `ATLAS-BATCH-0-CLOSE` — design-authority accept; confirm residency scheduler + REENROLL stay parked.

---

## 13. Pointers

- Active constitution: [`design_0_0_8_0.md`](design_0_0_8_0.md)
- Parked 0.0.7.9 mobility/transfer track: [`design_v7_9_mobility_transfer_allocation_production_track.md`](design_v7_9_mobility_transfer_allocation_production_track.md)
- Gating mechanics + proven-capability stop rule: [`workshop/phase_m_gating_and_doc_policy.md`](workshop/phase_m_gating_and_doc_policy.md)
- Binding structural rules: [`invariants.md`](invariants.md)
- Active status table + read order: [`workshop/mapping_current_guidance.md`](workshop/mapping_current_guidance.md)
