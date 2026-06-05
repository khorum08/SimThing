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
| **Design-authority enrichment** | Patrol relocate decision sourced from the accepted GPU-resident FIELD_POLICY `Threshold`+`EmitEvent`→`BoundaryRequest` posture (not a CPU planner; no new substrate pulled) — scenario exercises FIELD_POLICY + Ownership + Flow |
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
| `PRODUCTION-PATH-0080-0` | First substrate production-path gate | **IMPLEMENTED / PASS - Local Patrol Economy opt-in production path** — scoped to: *Local Patrol Economy using the 0.0.7.9 mobility/transfer substrate*, patrol relocate decision sourced from the accepted GPU-resident FIELD_POLICY `Threshold`+`EmitEvent`→`BoundaryRequest` path (mobility/transfer is the only substrate wired; no CPU planner; no new FIELD_POLICY production gate; no global default schedule) |
| `DEFAULT-SCHEDULE-0080-0` | Scenario-scoped schedule for Local Patrol Economy | **IMPLEMENTED / PASS - 1A schedule + patrol and 1B bounded pirate loop** — deterministic opt-in step driver routes GPU-resident FIELD_POLICY threshold/event/`BoundaryRequest` decisions into `PRODUCTION-PATH-0080-0`; pirate is a second IDROUTE identity, not a second economy owner; pirate disruption, supply drain, threshold relocation, `local_security` evasion scoring, and deterministic cat-and-mouse assertions are implemented; **not a global default schedule**. Spec: [`production_paths/default_schedule_0080_0_opening_spec.md`](production_paths/default_schedule_0080_0_opening_spec.md); reports: [`tests/phase_default_schedule_0080_0_impl_1a_results.md`](tests/phase_default_schedule_0080_0_impl_1a_results.md), [`tests/phase_default_schedule_0080_0_impl_1b_results.md`](tests/phase_default_schedule_0080_0_impl_1b_results.md). |
| `GAMEPLAY-0080-0` | Read-only Local Patrol Economy observation surface | **IMPLEMENTED / PASS — read-only Local Patrol Economy observation export** — consumes `DefaultSchedule0080RunReport`; exports deterministic tick transcript + summary via `observe_gameplay_0080_0`; explicit opt-in/default-off. **Player control / command input / UI framework / real-time loop remain CLOSED.** Spec: [`gameplay/gameplay_0080_0_opening_spec.md`](gameplay/gameplay_0080_0_opening_spec.md); impl: [`tests/phase_gameplay_0080_0_impl_results.md`](tests/phase_gameplay_0080_0_impl_results.md). |
| `CONTROL-0080-0` | Bounded Local Patrol Economy command admission | **IMPLEMENTED / PASS — bounded Local Patrol Economy command admission** — opt-in/default-off command vocabulary writes only existing `DefaultSchedule0080Input` bounded values/config, then runs schedule→`observe_gameplay_0080_0`; commands never move a mover, emit a `BoundaryRequest`, or bypass FIELD_POLICY. **Direct movement control / player command loop / UI framework / real-time loop remain CLOSED.** Spec: [`gameplay/control_0080_0_opening_spec.md`](gameplay/control_0080_0_opening_spec.md); impl: [`tests/phase_control_0080_0_impl_results.md`](tests/phase_control_0080_0_impl_results.md). |
| `DEMO-0080-0` | Headless Local Patrol Economy demo/export packaging | **IMPLEMENTED / PASS — headless Local Patrol Economy demo/export library helper** — deterministic opt-in/default-off; canonical `Control0080CommandBatch::canonical_run()`; existing control→schedule→observation/export path via `run_demo_0080_0`; day-to-day patrol/pirate movement record in impl report. **No CLI binary.** Direct movement control, player command loop, UI framework, real-time loop, global default schedule remain CLOSED. Spec: [`gameplay/demo_0080_0_opening_spec.md`](gameplay/demo_0080_0_opening_spec.md); impl: [`tests/phase_demo_0080_0_impl_results.md`](tests/phase_demo_0080_0_impl_results.md). |
| `SCENARIO-0080-1` | Second scenario — Nested Starmap (Terran/Pirate multi-theater) | **ACCEPTED (2026-06-02)** — nested `session → starmap(10×10) → 10 starsystems(10×10) → planet(10×10 submap)`; owner overlays inherit personality/policy weights broadcast from faction-owner simthings (OWNER down-broadcast); ownership up-aggregation (planet→starsystem) as a derived overlay; Terran patrol + pirate as a **full economy faction**; FIELD_POLICY-sourced composite-gap decision; opt-in/default-off. Pulls two parked substrates (below). Packet: [`scenarios/scenario_0080_1_admission_packet.md`](scenarios/scenario_0080_1_admission_packet.md). |
| `ATLAS-0080-0` | Atlas production runtime / sparse-residency nested mapping | **IMPLEMENTED / PASS — scenario-scoped sparse-residency nested mapping runtime for Nested Starmap** — opened by `SCENARIO-0080-1` as the named multi-theater consumer (the *first-slice gating* the invariant contemplates); opt-in sparse residency + nested theater descent/ascent; residency is a strict value no-op (I8 parity); **no default session pass-graph wiring**. Spec: [`production_paths/atlas_0080_0_opening_spec.md`](production_paths/atlas_0080_0_opening_spec.md); report: [`tests/phase_atlas_0080_0_impl_results.md`](tests/phase_atlas_0080_0_impl_results.md). |
| `ECON-SCALE-0080-0` | Multi-faction (Hybrid-Strata/faction-index) ECON scaling | **IMPLEMENTED / PASS — bounded faction-indexed contended ECON scaling for Nested Starmap** — opt-in/default-off; Terran + Pirate fixed bounded faction set; pirate is a full economy faction (adversarial participant in starsystem resource flow, extracts not merely disrupts); deterministic integer contended clearing with a CPU parity oracle; subsidiarity / FlatStar posture preserved; **no hard currency / markets / trade / `ai_budget`, no nested RF, no unbounded factions**. Default single-owner ECON unchanged when disabled. Spec: [`production_paths/econ_scale_0080_0_opening_spec.md`](production_paths/econ_scale_0080_0_opening_spec.md); impl: [`tests/phase_econ_scale_0080_0_impl_results.md`](tests/phase_econ_scale_0080_0_impl_results.md). |
| `PRODUCTION-PATH-0080-1` | `SCENARIO-0080-1` opt-in production path | **IMPLEMENTED / PASS — opt-in Nested Starmap production-path composition** — composes implemented/pass `ATLAS-0080-0` (sparse residency) + `ECON-SCALE-0080-0` (faction-index ECON) reports into one inspectable scenario report; owner-overlay inheritance + ownership up-aggregation are numeric summaries; FIELD_POLICY composite-gap terms are read-only; **no schedule/movement, no new substrate**. Spec: [`production_paths/production_path_0080_1_opening_spec.md`](production_paths/production_path_0080_1_opening_spec.md); impl: [`tests/phase_production_path_0080_1_impl_results.md`](tests/phase_production_path_0080_1_impl_results.md). |
| `DEFAULT-SCHEDULE-0080-1` | `SCENARIO-0080-1` schedule / movement | **IMPLEMENTED / PASS — scenario-scoped Nested Starmap FIELD_POLICY-sourced schedule/movement** — deterministic opt-in/default-off step driver that consumes `PRODUCTION-PATH-0080-1` and turns read-only FIELD_POLICY composite-gap terms into live movement via `Threshold + EmitEvent → BoundaryRequest`, routed through the existing mobility/transfer substrate posture (Terran + Pirate ships); preserves identity + owner overlays, updates membership without reparenting; **not a global default schedule; no observation/control/demo; no direct move; no new substrate**. Spec: [`production_paths/default_schedule_0080_1_opening_spec.md`](production_paths/default_schedule_0080_1_opening_spec.md); impl: [`tests/phase_default_schedule_0080_1_impl_results.md`](tests/phase_default_schedule_0080_1_impl_results.md). |
| `GAMEPLAY-0080-1` | `SCENARIO-0080-1` read-only observation/export | **IMPLEMENTED / PASS - read-only Nested Starmap observation/export** - read-only consumer of `DEFAULT-SCHEDULE-0080-1` run reports (`DefaultSchedule0081RunReport`) via `observe_gameplay_0080_1`; exports deterministic atlas residency, faction-index ECON, owner-overlay/up-aggregation, FIELD_POLICY movement trace, and Terran/Pirate movement rows; opt-in/default-off, non-interactive, mutates nothing beyond optional explicit schedule invocation. No control/command input, demo packaging, UI, or real-time loop. Control/demo for `0080-1` remain not opened; direct movement control, external boundary requests, CPU planner, global default schedule, semantic WGSL, new shader/GPU kernel, hard currency, nested RF, ClauseThing/L3, UI/realtime, and parked ladders remain closed/parked. Spec: [`gameplay/gameplay_0080_1_opening_spec.md`](gameplay/gameplay_0080_1_opening_spec.md); impl: [`tests/phase_gameplay_0080_1_impl_results.md`](tests/phase_gameplay_0080_1_impl_results.md). |
| `CONTROL-0080-1` | `SCENARIO-0080-1` bounded command admission | **IMPLEMENTED / PASS - bounded Nested Starmap command admission** - opt-in/default-off deterministic command vocabulary that writes only existing `DefaultSchedule0081Input` bounded schedule values plus bounded Nested Starmap control config, then runs the existing `DEFAULT-SCHEDULE-0080-1` -> `GAMEPLAY-0080-1` path. Commands never move a ship, emit an external `BoundaryRequest`, or bypass FIELD_POLICY (movement still emerges from the implemented `Threshold + EmitEvent -> BoundaryRequest` schedule). No direct movement control, player command loop, UI framework, real-time loop, demo packaging, or global default schedule. Spec: [`gameplay/control_0080_1_opening_spec.md`](gameplay/control_0080_1_opening_spec.md); impl: [`tests/phase_control_0080_1_impl_results.md`](tests/phase_control_0080_1_impl_results.md). |
| `DEMO-0080-1` | `SCENARIO-0080-1` headless demo/export packaging | **IMPLEMENTED / PASS — headless Nested Starmap demo/export library helper** — deterministic opt-in/default-off; canonical `Control0081CommandBatch::canonical_run()`; existing `control → DEFAULT-SCHEDULE-0080-1 → GAMEPLAY-0080-1` path via `run_demo_0080_1`; report includes atlas residency, faction-index ECON, owner-overlay/up-aggregation, FIELD_POLICY movement trace, Terran/Pirate movement rows, command transcript, and replay checksum. **No CLI binary.** Direct movement control, player command loop, UI framework, real-time loop, and global default schedule remain CLOSED. Spec: [`gameplay/demo_0080_1_opening_spec.md`](gameplay/demo_0080_1_opening_spec.md); impl: [`tests/phase_demo_0080_1_impl_results.md`](tests/phase_demo_0080_1_impl_results.md). |
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
commands write only existing bounded scenario input/config and never bypass FIELD_POLICY or move a mover.
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
| 3 | `PRODUCTION-PATH-0080-1-OPEN-0` | *(Opus authors)* Opening spec: initial conditions (§ scenario packet), owner overlays + personality/policy down-broadcast, ownership up-aggregation (planet→starsystem, derived overlay), ships-as-movers, pirate adversarial RF on starsystem entry, FIELD_POLICY-sourced composite-gap decisions | **OPEN** (author + adjudicate) | rungs 1–2 |
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

### 12.0 Harness handoff — canonical citations (cite on every handoff)

**Codex cites this high-signal set on every rung handoff (keep to these; everything else is reachable
from them). They anchor each handoff to the three things that must not drift — FIELD_POLICY principles, the
GPU-resident Accumulator resource-flow notion, and the anticipated emergent behaviors:**
1. **[`design_0_0_8_0.md`](design_0_0_8_0.md) §0** — transient constitution: conformance; **all conflict
   is resource flow**; recursive allocation; **FIELD_POLICY = GPU-resident threshold crossings, no CPU planner**
   (§0.0, §0.5 #4); §0.5 harness discipline.
2. **[`invariants.md`](invariants.md)** — binding rules: **"Scenario Proof"**; **AccumulatorOp v2** +
   **Resource Flow Substrate** sections (the GPU-resident flow contract); FIELD_POLICY/JIT closure-posture.
3. **This file, §12–§12.5** — rehearsal + pre-rehearsal design (architecture, EC1/EC2, nested-grid,
   ATLAS-BATCH-0, retirement map + coverage). **§12.4 links the OWNER design of record + parked
   `mobility_owner0.rs`** (masked reduction — reachable here, not duplicated as a top-level link).
4. **[`scenarios/scenario_0080_2_dress_rehearsal_spec.md`](scenarios/scenario_0080_2_dress_rehearsal_spec.md)**
   — the **concrete scenario** (13 systems, factory/pop/starport economy, numbers, disruption-as-blockade)
   **and the anticipated emergent behaviors (§8.1)** the closing report verifies.
5. **[`../crates/simthing-core/src/accumulator_op.rs`](../crates/simthing-core/src/accumulator_op.rs)** —
   the **GPU-resident Accumulator primitive**: `SourceSpec` / `CombineFn` / `GateSpec` / `ScaleSpec` /
   `ConsumeMode` — the vocabulary every arena (labor / production / disruption / combat) compiles down to.
6. **[`workshop/field_policy_track.md`](workshop/field_policy_track.md)** — the **FIELD_POLICY charter /
   principles**: field-as-policy; decisions are GPU-resident threshold crossings → `BoundaryRequest`;
   no CPU planner.

**Anchors (every handoff holds all three):** **FIELD_POLICY principles** → links 1, 2, 6; **GPU-resident
Accumulator resource flow** → links 1, 2, 5; **anticipated emergence** → link 4 (§8.1). OWNER masked
reduction is reachable via §12.4 (link 3).

**Established decisions (do not re-derive — implement within these):**
- `Location` is the FIELD_POLICY field primitive (gridcell); **non-`Location` SimThings participate in resource
  flow normally.** `StarSystem`/`Station` are **deprecated — do not use.** `kind` is an install-time
  selector only, never a runtime branch (§0.1).
- A Location arranges its gridcell children's flow data in **dense, grid-ordered 2-D cells**
  (`cell(x,y) = map_base + y·width + x`); the buffer-is-the-map view is free (§12.2).
- The **cell is its own slot**; a planet (feature) and patrol/pirate (movers) are **occupants** that
  contribute *into* it — **per-channel/per-owner, never merged** (§12.3 EC-A3).
- Multi-owner routing at a cell = the **OWNER masked reduction** (mask-then-sum per identity); a
  faction's overlays mask **down** from the GameSession owner-entities; **capture = owner-column flip,
  never reparenting** (§12.4). **The 2-D arrangement does not alter the OWNER directives — addressing
  (where a cell sits) and identity/masking (how flows route) compose; they do not conflict.**
- Dense field/heatmap + **sparse REENROLL movers**; settling depth is **emergent** (§0.2).
- Build atlas batch allocation on a **static** map first (ATLAS-BATCH-0, §12.3); the **sparse-residency
  scheduler (M-4A) and REENROLL stay parked** until R5.
- **Rung sequence + which parked phase each rung proves/closes: §12.5** (one parked phase per rung).

---

**Why.** Every 0080 slice so far — `0080-0` Local Patrol Economy, `0080-1` Nested Starmap, and the
`SCENARIO-0080-2` Pirate Gradient Pathfinding rung ladder — was **proven at the math/behavioral
layer**: the recurrences, fields, dual-output gradient, FIELD_POLICY threshold-gated movement, and bit-exact
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
| FIELD_POLICY field-as-policy movement (0080-2 rung 4; 0080-0/1 patrol/ship FIELD_POLICY) | pirate movement via `Threshold`+`EmitEvent`→`BoundaryRequest`, one step/boundary, no CPU planner |
| Disruption/desirability as faction-economy signals (0080-0/0080-1) | gridcell columns read by faction overlays; pirate as adversarial participant |
| Deterministic replay / I8 bit-exact parity | same inputs → identical resolved GPU/CPU values across two runs of the assembled session |

**This is the first scenario authored to satisfy the new "Scenario Proof" gate.** Tier-2 (new
assembled session; gridcell columns wired into a session pass graph). §8 stop conditions still bind:
it pulls the mapping/RegionCell + AccumulatorOp + FIELD_POLICY substrates through one tree; it does **not**
open atlas production runtime, nested-RF depth, hard currency, ClauseThing/L3, or a real-time/UI loop.

> Dense per-cell temporal memory stays separately gated. The gridcell `disruption` column here is
> per-cell **state advanced across boundaries** by the standard AccumulatorOp + root-overlay decay
> (the bounded-feedback contract) — i.e. the sparse-per-node math of 0080-2 now expressed over real
> cell simthings, **not** the deferred dense-temporal VRAM gate.

### 12.1 Provisional findings (2026-06-03 audit — to be firmed up before the dress-rehearsal opens)

> **Status: PROVISIONAL.** Captured from a design-authority audit of the prior mapping/FIELD_POLICY/0080 work.
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
- **F4 — FIELD_POLICY never consumed a heatmap for pathing/critical-path.** FIELD_POLICY-OBS scores an entity's **own
  overlays** (`ExactQ16WeightedSum`), not a spatial field; FIELD_POLICY-EVENT/PIPE/ACT are event→proposal
  downstream; FrontierV1-4's "FIELD_POLICY route" `validate_field_policy_v1_consumed()` only **asserts two kernel
  descriptors are registered** (field computed in the same fixture but never read by FIELD_POLICY); 0080
  "pathing" was a scalar `supply*100 − disruption*10 − security` over 2 nodes. `field_urgency`
  critical-path existed as plumbing over hand-seeds, never a gameplay-driven agent decision.
- **F5 — the connecting tissue was never implemented.** Both halves (*field → heatmap* and *FIELD_POLICY →
  action*) were built and "proven" separately; the loop **field → diffuse → gradient → FIELD_POLICY reads local
  cell → action** was never wired. Every prior "pass" satisfied the two ends and never the connection.

**Provisional design resolutions (from the §12 discussion — confirm tomorrow):**

- Falloff is **stencil diffusion**, not arena enrollment; **two-column model** — `disruption` (source,
  arena accumulator, BoundedFeedback decay) → `location_status` (sink, stencil-written falloff,
  strict-sink `source_col != target_col`). Neighbors are **not enrolled**; the dense pass sweeps the
  whole grid. Falloff is a property field, **not** an overlay (the decaying *ownership* signal is the overlay).
- **Sparse arenas + dense diffusion**: arenas instance only on occupied cells; one stencil pass covers
  all 100 cells; they meet at the `disruption` column.
- **Diffusion horizon = FIELD_POLICY sight radius** (myopic local read sees H hops because diffusion pre-bakes
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
- **EC2 (FIELD_POLICY consumption):** a mover's FIELD_POLICY evaluation **reads the diffused heatmap gradient at its own
  cell**, and the **emitted action is a function of that gradient** (verified against a CPU oracle) —
  **not** a hand-seeded field or a registration-only stand-in. The field → gradient → FIELD_POLICY → action loop
  is closed end-to-end through real SimThings.

### 12.2 Key concept — the recursive nested-grid field hierarchy (design note)

> **Status: PROVISIONAL design note (2026-06-03).** The substrate concept the EC1/EC2 exit criteria
> build on. Not yet a gate.

**The idea.** `Location`-kind SimThings are the FIELD_POLICY field primitives ("gridcells"). Any non-Location
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
  star 10×10 subgrids + planet/moon subgrids). **Clarification (2026-06-03):** the **closed**
  `SCENARIO-0080-2 ATLAS-BATCH-0-GEN` fixture is the **economy-rehearsal live map descriptor** from
  [`scenarios/scenario_0080_2_dress_rehearsal_spec.md`](scenarios/scenario_0080_2_dress_rehearsal_spec.md):
  **20×20 galactic grid, 13 systems (10 Terran + 3 Pirate)**, 10×10 system/planet surfaces — not the
  100×100 / ~1000-star stress atlas. The older 100×100 language remains a **stress/scale atlas-batching
  target** for PACK and M-4 acceptance; LOC/PACK/STORE for the dress rehearsal must consume the **20×20 /
  13-system** descriptor unless Opus authors a separate scale-stress fixture contract.
- **EC-A2 (split by design authority 2026-06-03):**
  - **EC-A2a (PACK, CPU-provable):** the batcher packs the homogeneous tiles with `G=0` algebraic
    masking within `V78AtlasVramBudget`, reports a **numeric** VRAM multiplier, and a **CPU oracle**
    proves the `G=0` no-inter-tile-bleed property + tile-local↔atlas coordinate round-trip.
  - **EC-A2b (PACK-GPU):** one **batched GPU dispatch per homogeneous tile class** (`AtlasMaskGpuOp`,
    `TileLocalMaskG0`) matches the CPU oracle within the established **`GpuVerified` tolerance
    (full-tile L∞ ≤ 1e-4)**, with `G=0` tile-local no-bleed proven. *(Design-authority correction
    2026-06-03: redefined from "bit-exact" — the f32 atlas stencil is `GpuVerified`, not
    `ExactDeterministic`; the atlas-mask GPU primitive + CPU oracle already exist in
    `simthing-gpu/src/atlas_mask.rs`.)* Contract: [`handoffs/dress_rehearsal_codex_handoff_4_atlas_batch_0_pack_gpu.md`](handoffs/dress_rehearsal_codex_handoff_4_atlas_batch_0_pack_gpu.md).
  - **EC-A2b-exact (DEFERRED — separate exact-arithmetic track):** true **bit-exact** (`f32::to_bits()`)
    GPU=CPU parity. Requires a **pinned fixed-point stencil**; not achievable on the f32 path. Not PACK-GPU.
- **EC-A3 (STORE, CPU storage shape):** a Location stores its gridcell children's flow results in the
  correct `(location, cell, channel, owner)` slots, and **co-located children at one `(x,y)` are
  preserved per-channel/per-owner and never blind-summed by position** — proven on a CPU oracle (the
  10-pirate-shared-cell case + a constructed planet+patrol+pirate cell). *(Design-authority split
  2026-06-03: STORE = CPU storage-shape proof; the **live OWNER masked-reduction runtime** — `EvalEML`
  `CMP_EQ` + `Sum` over owner-indexed columns — is proven on GPU as **EC-A3-gpu /
  `ATLAS-BATCH-0-STORE-GPU` (PASS, fixture composition only)**; the OWNER masked-reduction *runtime*
  and R3 remain parked.)* Contract: [`handoffs/dress_rehearsal_codex_handoff_5_atlas_batch_0_store.md`](handoffs/dress_rehearsal_codex_handoff_5_atlas_batch_0_store.md).
  The CPU oracle here is the reference the STORE-GPU slice checks against (§12.4 OWNER masked reduction).
- **EC-A4:** the residency scheduler (M-4A) and REENROLL remain unbuilt/parked — the slice is static.

**Rungs (provisional):**
1. `ATLAS-BATCH-0-GEN` — static map generator (fixture producer).
2. `ATLAS-BATCH-0-LOC` — Location-kind gridcell primitive + grid-placement slot allocation + multi-channel cell.
3. `ATLAS-BATCH-0-PACK` — atlas batch allocation + `G=0` mask + VRAM-multiplier + batched dispatch + CPU parity.
4. `ATLAS-BATCH-0-STORE` — children's flow results into 2-D map slots; the co-located-not-merged test (EC-A3).
5. `ATLAS-BATCH-0-CLOSE` — **CLOSED / PASS (2026-06-04)**; design-authority accept; residency scheduler + REENROLL confirmed parked. Report: [`tests/scenario_0080_2_atlas_batch_0_close_report.md`](tests/scenario_0080_2_atlas_batch_0_close_report.md).

> **`ATLAS-BATCH-0-GEN` closure (2026-06-03).** `SCENARIO-0080-2` GEN is **closed / PASS** as a pure
> 20×20 / 13-system dress-rehearsal topology descriptor (no GPU, no `Location` materialization, no
> production wiring). Evidence:
> [`crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_gen.rs`](../crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_gen.rs);
> [`crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_gen.rs`](../crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_gen.rs);
> [`tests/scenario_0080_2_atlas_batch_0_gen_report.md`](tests/scenario_0080_2_atlas_batch_0_gen_report.md);
> [`tests/scenario_0080_2_atlas_batch_0_gen_cargo_test_2026_06_03.txt`](tests/scenario_0080_2_atlas_batch_0_gen_cargo_test_2026_06_03.txt).
> Command: `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_gen` → **6 passed; 0 failed**.

> **`ATLAS-BATCH-0-LOC` closure (2026-06-03).** `SCENARIO-0080-2` LOC is **implemented / PASS** as
> fixture-only gridcell-primitive layout + occupant placement + typed channel descriptors (27 Locations,
> 56 occupants, `total_cell_slots = 3000`, single `cell_index` home). Does **not** implement PACK, STORE,
> GPU dispatch, owner masked-reduction runtime, economy, FIELD_POLICY movement, or `simthing-sim` semantics.
> Evidence:
> [`crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_loc.rs`](../crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_loc.rs);
> [`crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_loc.rs`](../crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_loc.rs);
> [`tests/scenario_0080_2_atlas_batch_0_loc_report.md`](tests/scenario_0080_2_atlas_batch_0_loc_report.md);
> [`tests/scenario_0080_2_atlas_batch_0_loc_cargo_test_2026_06_03.txt`](tests/scenario_0080_2_atlas_batch_0_loc_cargo_test_2026_06_03.txt).
> Command: `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_loc` → **9 passed; 0 failed**.

> **`ATLAS-BATCH-0-PACK` closure (2026-06-03) — EC-A2a only.** `SCENARIO-0080-2` PACK is **implemented /
> PASS** for **EC-A2a**: CPU pack-plan descriptor (3 homogeneous tile classes, 27 packed tiles), algebraic
> tile-local **G=0** CPU oracle (`g_zero_sample`), and numeric **VRAM report** (multiplier **1.0**,
> `budget_pass` vs `V78AtlasVramBudget`). **Does NOT** implement batched GPU dispatch or GPU=CPU bit-exact
> parity — that is **EC-A2b**, deferred to **`ATLAS-BATCH-0-PACK-GPU`**. Does not implement STORE, owner
> masked-reduction runtime, economy, or FIELD_POLICY. Evidence:
> [`crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_pack.rs`](../crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_pack.rs);
> [`crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_pack.rs`](../crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_pack.rs);
> [`tests/scenario_0080_2_atlas_batch_0_pack_report.md`](tests/scenario_0080_2_atlas_batch_0_pack_report.md);
> [`tests/scenario_0080_2_atlas_batch_0_pack_cargo_test_2026_06_03.txt`](tests/scenario_0080_2_atlas_batch_0_pack_cargo_test_2026_06_03.txt).
> Command: `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_pack` → **9 passed; 0 failed**.
> **STORE remains unimplemented.** Sparse-residency scheduler (M-4A) and REENROLL remain parked.

> **`ATLAS-BATCH-0-PACK-GPU` closure (2026-06-03) — EC-A2b GpuVerified.** `SCENARIO-0080-2` PACK-GPU is
> **implemented / PASS** for **EC-A2b**: one batched `AtlasMaskGpuOp` dispatch path per homogeneous tile class
> (`Galactic20x20`, `StarSystem10x10`, `PlanetSurface10x10`) with `TileLocalMaskG0` matches the
> caller-managed CPU oracle within **`GpuVerified` tolerance (full-tile L∞ ≤ 1e-4)**; `G=0` cross-tile /
> out-of-atlas isolation proven. Uses existing `simthing-gpu/src/atlas_mask.rs` only (no new WGSL). **Does
> NOT** claim bit-exact / `f32::to_bits()` parity — **EC-A2b-exact remains DEFERRED** (pinned fixed-point
> stencil track). Does not implement STORE, owner masked-reduction runtime, economy, disruption, or FIELD_POLICY.
> Evidence:
> [`crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_pack_gpu.rs`](../crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_pack_gpu.rs);
> [`crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_pack_gpu.rs`](../crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_pack_gpu.rs);
> [`tests/scenario_0080_2_atlas_batch_0_pack_gpu_report.md`](tests/scenario_0080_2_atlas_batch_0_pack_gpu_report.md);
> [`tests/scenario_0080_2_atlas_batch_0_pack_gpu_cargo_test_2026_06_03.txt`](tests/scenario_0080_2_atlas_batch_0_pack_gpu_cargo_test_2026_06_03.txt);
> [`tests/scenario_0080_2_atlas_batch_0_pack_gpu_parity_2026_06_03.txt`](tests/scenario_0080_2_atlas_batch_0_pack_gpu_parity_2026_06_03.txt).
> Command: `$env:SIMTHING_RUN_GPU_TESTS=1; cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_pack_gpu`
> → **8 passed; 0 failed** (GPU tier ran; adapter **Intel(R) RaptorLake-S Mobile Graphics Controller**).
> **STORE remains unimplemented.** M-4A sparse-residency scheduler and REENROLL remain parked.

> **`ATLAS-BATCH-0-STORE` closure (2026-06-03) — EC-A3 CPU storage shape.** `SCENARIO-0080-2` STORE is
> **implemented / PASS** for **EC-A3**: generic child contributions aggregate into dense
> `(location_id, cell_index, channel, owner)` slots via LOC `cell_index`; co-located occupants never
> blind-summed by position. Proven: **10 canonical pirate fleets** on one galactic cell (pirate channels
> only); **constructed planet+patrol+pirate** at one system cell (distinct channel/owner entries). **CPU-only**
> — does **not** run live OWNER masked reduction (**STORE-GPU deferred**; runtime remains parked until
> STORE-GPU / R3). Does not implement R1/R2/R3/R4, economy, disruption, FIELD_POLICY, or combat. Evidence:
> [`crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_store.rs`](../crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_store.rs);
> [`crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_store.rs`](../crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_store.rs);
> [`tests/scenario_0080_2_atlas_batch_0_store_report.md`](tests/scenario_0080_2_atlas_batch_0_store_report.md);
> [`tests/scenario_0080_2_atlas_batch_0_store_cargo_test_2026_06_03.txt`](tests/scenario_0080_2_atlas_batch_0_store_cargo_test_2026_06_03.txt).
> Command: `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store` → **11 passed; 0 failed**.
> M-4A sparse-residency scheduler and REENROLL remain parked.

> **`ATLAS-BATCH-0-STORE-GPU` closure (2026-06-03) — EC-A3-gpu ExactDeterministic bit-exact.** `SCENARIO-0080-2`
> STORE-GPU is **implemented / PASS** for **EC-A3-gpu**: whitelisted `EvalEML` (`CMP_EQ`/`SELECT`)
> owner+channel mask + contiguous `Sum` on `AccumulatorOpSession` matches the accepted CPU **`StoreOracle`**
> (**38/38** entries, `f32::to_bits()`). Proven on GPU: **10-pirate shared cell**; **constructed
> planet+patrol+pirate**. **Fixture composition only** — OWNER masked-reduction is **not** wired into a
> session pass graph; **R3/runtime remains parked**. Parity standard: **ExactDeterministic bit-exact** (no
> GpuVerified fallback). Does not implement R1/R2/R3/R4, economy, disruption, FIELD_POLICY, movement, combat, or
> M-4A/REENROLL. Evidence:
> [`crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_store_gpu.rs`](../crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_store_gpu.rs);
> [`crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_store_gpu.rs`](../crates/simthing-driver/tests/dress_rehearsal_atlas_batch_0_store_gpu.rs);
> [`tests/scenario_0080_2_atlas_batch_0_store_gpu_report.md`](tests/scenario_0080_2_atlas_batch_0_store_gpu_report.md);
> [`tests/scenario_0080_2_atlas_batch_0_store_gpu_cargo_test_2026_06_03.txt`](tests/scenario_0080_2_atlas_batch_0_store_gpu_cargo_test_2026_06_03.txt);
> [`tests/scenario_0080_2_atlas_batch_0_store_gpu_parity_2026_06_03.txt`](tests/scenario_0080_2_atlas_batch_0_store_gpu_parity_2026_06_03.txt).
> Command: `$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu`
> → **10 passed; 0 failed; 0 ignored** (GPU tier ran on discrete **NVIDIA GeForce RTX 4080 Laptop GPU**; `gpu_adapter_is_discrete_rtx_target` fails Intel).
> **Prior Intel-only STORE-GPU log is superseded for EC-A3-gpu;** PACK-GPU Intel evidence remains Intel-only (EC-A2b).

### 12.4 Established mechanism — OWNER routing (multi-owner flows in one cell)

> **Already-designed + parked substrate; the harness pulls it, does not re-derive it.** This is the
> routing mechanism for resource flows from multiple owners sharing one spatial location, and it is
> what implements §12.3 EC-A3 ("co-located children never merged").

**Design of record:** [`workshop/mobility_and_transfer_allocation.md`](workshop/mobility_and_transfer_allocation.md)
**§11** — *"the identity/ownership overlay: directing flows by property, not by structure"* (esp. **§11.1**
the masked reduction; **§11.5** the session clearinghouse topology). Acceptance review:
[`reviews/transfer_emission_registration_ownership_opus_review.md`](reviews/transfer_emission_registration_ownership_opus_review.md).
**Parked substrate code:**
[`../crates/simthing-spec/src/designer_admission/mobility_owner0.rs`](../crates/simthing-spec/src/designer_admission/mobility_owner0.rs)
(`MOBILITY-OWNER-0` — owner relations as explicit columns `{Faction, Species, Blueprint, Tech}`, latched
modifier overlays via deterministic owner-column matching; metadata/testable substrate, no production
runtime). Masked-reduction primitives:
[`../crates/simthing-core/src/accumulator_op.rs`](../crates/simthing-core/src/accumulator_op.rs)
(`EvalEML` select/`CMP_EQ` + `Sum` + `ScaleSpec::ByColumn` — **no new WGSL / `CombineFn` /
`AccumulatorRole`**).

**How it works (as designed — do not reinvent):**
- **Owner-entities live under the GameSession root, not in the spatial tree** (§11.5):
  `GameSession → { Faction A, Faction B, …, SpeciesRegistry, WorldStateMap }`. A faction owner-entity
  holds its capability trees, policies, stockpile, and **effective overlay set**; the WorldStateMap is
  pure spatial containment; cells/holdings/cohorts carry **owner-columns**. **Capture = owner-column
  flip, never reparenting.**
- **Modifier overlays mask down by owner-column matching** (latched, `DirtyOnly`, per-owner layered):
  a faction's effective overlays (combat bonus, tech, fight-or-flight policy) broadcast from its
  GameSession-level owner-entity onto each spatial SimThing's owner overlay, applied where the
  owner-column matches. This **is** the "inherited from the gamesession, masked onto each simthing's
  ownership overlay" path.
- **Flows route by masked reduction, not by structure** (§11.1): for each identity `F` present in a
  cell, `masked = value · (owner_column == F)` via an `EvalEML` select/`CMP_EQ` mask, then a contiguous
  `SlotRange Sum` of the masked values into a **per-identity column** on the cell. A planet, a patrol,
  and a pirate in one `(x,y)` are summed **per owner-identity into distinct columns** — never collapsed.
  EC-A3 *is* this masked reduction applied to co-located occupants.
- **Where the flow balances is emergent** (§0.2): the masked reduction climbs the spine to wherever
  supply meets demand (combat nets at the cell; an empire economy nets at the GameSession root) — one
  mechanism, emergent settling depth, not a per-relation `D` assignment.

**Harness establishes:** ATLAS-BATCH-0 (§12.3) materializes owner-columns on cells and proves the masked
per-owner reduction keeps co-located occupants distinct (EC-A3); the dress rehearsal (§12) reuses the
same OWNER routing for disruption / combat / economy flows. **Parked, not pulled here:** OWNER
*production-runtime gameplay*, Hybrid-Strata/faction-index ECON scaling, and any capture beyond
column-flip — each its own gate.

### 12.5 Rehearsal rung ladder + parked-phase retirement map

> **Handoffs (2026-06-03):** [`handoffs/dress_rehearsal_codex_handoff_0.md`](handoffs/dress_rehearsal_codex_handoff_0.md)
> (orientation + **`ATLAS-BATCH-0-GEN`** — closed/PASS) · [`handoffs/dress_rehearsal_codex_handoff_1_remedial.md`](handoffs/dress_rehearsal_codex_handoff_1_remedial.md)
> (GEN spacing-band remedial — resolved) · [`handoffs/dress_rehearsal_codex_handoff_2_atlas_batch_0_loc.md`](handoffs/dress_rehearsal_codex_handoff_2_atlas_batch_0_loc.md)
> (**`ATLAS-BATCH-0-LOC`** — closed/PASS) · [`handoffs/dress_rehearsal_codex_handoff_3_atlas_batch_0_pack.md`](handoffs/dress_rehearsal_codex_handoff_3_atlas_batch_0_pack.md)
> (**`ATLAS-BATCH-0-PACK`** — closed/PASS for EC-A2a; EC-A2b/GPU deferred) ·
> [`handoffs/dress_rehearsal_codex_handoff_4_atlas_batch_0_pack_gpu.md`](handoffs/dress_rehearsal_codex_handoff_4_atlas_batch_0_pack_gpu.md)
> (**`ATLAS-BATCH-0-PACK-GPU`** — closed/PASS for EC-A2b GpuVerified; EC-A2b-exact deferred) ·
> [`handoffs/dress_rehearsal_codex_handoff_5_atlas_batch_0_store.md`](handoffs/dress_rehearsal_codex_handoff_5_atlas_batch_0_store.md)
> (**`ATLAS-BATCH-0-STORE`** — closed/PASS for EC-A3 CPU storage shape; STORE-GPU deferred) ·
> [`handoffs/dress_rehearsal_codex_handoff_6_store_remedial.md`](handoffs/dress_rehearsal_codex_handoff_6_store_remedial.md)
> (STORE remedial — resolved) · [`handoffs/dress_rehearsal_codex_handoff_7_atlas_batch_0_store_gpu.md`](handoffs/dress_rehearsal_codex_handoff_7_atlas_batch_0_store_gpu.md)
> (**`ATLAS-BATCH-0-STORE-GPU`** contract — authored + accepted; Option B fixture harness over existing
> `AccumulatorOp`/`EvalEML CMP_EQ`/`Sum` masked reduction; bit-exact (ExactDeterministic) parity over
> integer masked sums vs the STORE oracle, GpuVerified fallback; OWNER masked-reduction *runtime* + R3 stay parked).
>
> **ATLAS-BATCH-0 — COMPLETE (CLOSED / PASS, `ATLAS-BATCH-0-CLOSE` 2026-06-04).** `GEN`, `LOC`,
> `PACK` (EC-A2a), **`PACK-GPU` (EC-A2b GpuVerified)**, **`STORE` (EC-A3)**, **`STORE-GPU` (EC-A3-gpu
> ExactDeterministic)** all closed/PASS, NVIDIA RTX 4080-validated, full workspace green. Closeout:
> [`tests/scenario_0080_2_atlas_batch_0_close_report.md`](tests/scenario_0080_2_atlas_batch_0_close_report.md).
> **`R1` — ACCEPTED / CLOSED / IMPLEMENTED-PASS — Disruption Heatmap / EC1 (2026-06-04, Opus).**
> `SCENARIO-0080-2-R1-IMPL-0` (PR #511) implemented from the R1 opening
> spec: [`scenarios/scenario_0080_2_r1_disruption_heatmap_opening_spec.md`](scenarios/scenario_0080_2_r1_disruption_heatmap_opening_spec.md);
> evidence [`tests/scenario_0080_2_r1_disruption_heatmap_report.md`](tests/scenario_0080_2_r1_disruption_heatmap_report.md);
> **accepted** in [`tests/scenario_0080_2_r1_acceptance_review.md`](tests/scenario_0080_2_r1_acceptance_review.md)
> (Opus re-ran all test evidence: R1 34/34, gen 6, loc 9, store 11, demo_0080_1 24, default_schedule_0080_1 30,
> `cargo check --workspace` clean). The fixture builds an occupant-produced disruption heatmap over the
> **galactic 20×20** gridcell SimThings (fleets/systems as contributors), applies the pinned
> `BoundedFeedback` recurrence (`clamp(prev*0.80 + input, 0, 100)`; `PIRATE_EMIT=20`, `PATROL_SUPPRESS=15`),
> diffuses into the strict sink `location_status`, reduces a starmap summary, and emits a deterministic
> 400-cell artifact with CPU-oracle parity. **CPU oracle primary; no new GPU/shader code; no f32 bit-exact
> claim.** `R2` is now **IMPLEMENTED / PASS (2026-06-04)** as recursive allocation + faction economy +
> blockade/divert over the accepted R1 heatmap: [`tests/scenario_0080_2_r2_recursive_allocation_report.md`](tests/scenario_0080_2_r2_recursive_allocation_report.md).
> `R3` is now **IMPLEMENTED / PASS (2026-06-04)** as capability-tree modifier overlays masked down
> by owner-column: [`tests/scenario_0080_2_r3_capability_mask_down_report.md`](tests/scenario_0080_2_r3_capability_mask_down_report.md).
> `R4` is now **IMPLEMENTED / PASS (2026-06-04)** as FIELD_POLICY field-consumption + exact sqrt EC2:
> [`tests/scenario_0080_2_r4_field_policy_consumption_report.md`](tests/scenario_0080_2_r4_field_policy_consumption_report.md).
> `R5` is now **IMPLEMENTED / PASS (2026-06-04)** as movement via BoundaryRequest + REENROLL + mobility substrate:
> [`tests/scenario_0080_2_r5_movement_reenroll_report.md`](tests/scenario_0080_2_r5_movement_reenroll_report.md).
> R1-R6B remain single-galactic-tier, opt-in/default-off, CPU-oracle primary (GPU-shaped row ops).
> **R6C is now IMPLEMENTED / PASS (2026-06-04)** as the integrated 100-tick mutable run:
> one canonical seed, live feedback write-back, detector table, race curve, and stable checksum
> `1bba891c779190a4`; first blockade tick 2, first movement-produced combat tick 44, first
> production reinforcement tick 49. Report:
> [`tests/scenario_0080_2_r6c_integrated_run_report.md`](tests/scenario_0080_2_r6c_integrated_run_report.md).
> **R7 is now CLOSED / PASS (2026-06-04)** with the R6C evidence replacing the reopened interim
> "not yet emerged" finding: [`tests/scenario_0080_2_r7_closeout_report.md`](tests/scenario_0080_2_r7_closeout_report.md).
> M-4A sparse-residency scheduler remains parked.
>
> **`SCENARIO-0080-2` is COMPLETE (`SCENARIO-0080-2-COMPLETE-0`, 2026-06-04, design authority).** The
> rehearsal proves *and runs* a single-galactic-tier, opt-in/default-off integrated SimThing dress
> rehearsal: occupant-produced disruption, economy/blockade, capability mask-down, FIELD_POLICY field
> consumption, movement/REENROLL, fleet-cohort Resource Flow combat, and ship production/fusion all
> operate through one mutable 100-tick session with write-back. The run is **GPU-conformant and
> CPU-oracle verified**; GPU execution for the rehearsal rungs remains an unmeasured follow-on
> diagnostic. Closed **with caveats recorded as findings** (not defects): Terran patrols never crossed
> the movement threshold (one-sided contest), the production/attrition race is unresolved in 100 ticks,
> and front/standoff + self-sustaining-loop behaviors only partially emerged. Those findings feed a
> **future** `SCENARIO-0080-3 — richer multi-hotspot emergence run`, which is **not** opened now.
>
> **`GPU-MEASURE-0080-0` is IMPLEMENTED / PASS (2026-06-04)** (opening:
> [`handoffs/gpu_measure_0080_0_opening.md`](handoffs/gpu_measure_0080_0_opening.md), report:
> [`tests/gpu_measure_0080_0_results.md`](tests/gpu_measure_0080_0_results.md)). The rehearsal's
> accepted row/mask/threshold/emission-band shapes were measured on the discrete GPU via existing
> generic AccumulatorOp / structured-field / Candidate-F paths: R1, R2, R6, and R6B are
> `GPU-measured (integer bit-exact)`; R4 is
> `GPU-measured (verified-approximate, within accepted f32 bound)` with max abs delta `3.0994415e-6`
> against bound `1.0e-4`. R6C whole-run execution keeps the exact posture
> `GPU-conformant; GPU execution not yet measured`. No new semantic WGSL, no new op, no new invariant,
> no pinned-number change, no default SimSession wiring, and no reopen of 0080-2.
>
> **`RUNTIME-0080-0-R0` is IMPLEMENTED / PARTIAL (R0A remedial, 2026-06-05)** (design opening:
> [`production_paths/runtime_0080_0_opening_spec.md`](production_paths/runtime_0080_0_opening_spec.md);
> report: [`tests/runtime_0080_0_r0_results.md`](tests/runtime_0080_0_r0_results.md)). Single-theater
> mirror-dispatch scheduler for R6C: persistent GPU buffer + per-tick R1/R2/R4/R6/R6B shape dispatches
> preserved (`inter_tick_world_readbacks=0`), but **CPU R6C remains tick authority** — GPU state does
> not yet drive tick N+1. CPU-oracle parity incl. checksum `1bba891c779190a4` unchanged. **R6C whole-run
> posture (corrected):** `R6C whole-run remains GPU-conformant; per-tick shapes GPU-dispatched against
> CPU-authoritative R6C; GPU-resident next-tick authority not yet implemented`. Substrate gap for true
> PASS: GPU-resident cross-tick world transition for full R6C loop (new runtime primitive rung). No
> `request_atlas_batching`, no M-4A masking-at-scale, no new semantic WGSL/op/invariant, no scenario
> reopen. Adapter: RTX 4080 Laptop GPU.
>
> **`RUNTIME-0080-0-R1a` is IMPLEMENTED / PASS (Outcome A hardened, 2026-06-05)** — Tier-A GPU-side
> next-tick source-of-truth on `WorldGpuState` + `Pipelines` with **no covered-column oracle-fed replay**;
> per-tick inputs are derived from `R1aBoundaryWitness` + resident GPU readback + static constants; exact-bit
> proofs for stockpiles, construction_progress, existing-slot `num_ships`, and blockade/divert code; disabled-transform
> parity check earned at bit level; report checksum `f0244d3d9106900d`. See
> [`docs/tests/runtime_0080_0_r1a_next_tick_authority_results.md`](tests/runtime_0080_0_r1a_next_tick_authority_results.md).
> (design spec: [`production_paths/runtime_0080_0_r1_next_tick_authority_spec.md`](production_paths/runtime_0080_0_r1_next_tick_authority_spec.md) §14;
> handoff: [`handoffs/runtime_0080_0_r1a_remedial_opening.md`](handoffs/runtime_0080_0_r1a_remedial_opening.md)).
> R1 defines the substrate primitive **`GPU-STATE-AUTH-0`** — GPU-resident world state as the **input
> authority for tick N+1**. The accepted R1a implementation deletes the old CPU-injected journal producer
> and registers the Tier-A R6C transforms as resident `AccumulatorOp` / generic GPU helper work on the
> production substrate: the GPU computes `state_N+1`, tick-boundary swap promotes NEXT to CURRENT, and the
> CPU oracle is comparison-only (not a production input feeder). The anti-faking protocol is earned by
> independence, disabled-transform parity check, measured counters, per-column parity from GPU readback bits,
> and source-shape guards against oracle-fed replay.
>
> **Opcode/WGSL-gate clarification (Opus, 2026-06-05):** the remedial's blanket "no new WGSL/opcode"
> stop-line was hygiene theater — stricter than `design_0_0_8_0.md` §2.3 (*"New generic WGSL is a Tier-2
> gate, not a prohibition"*). The constitution now resolves the EML-interpreter-opcode ambiguity (§2.4 +
> the invariants companion row), and the handoff gains a **§4a admission gate**: adding a **generic,
> semantic-free** `EvalEML` opcode / `AccumulatorOp` combine fn / kernel is **permitted** for Outcome A
> (semantic-free, reusable, CPU-oracle bit-exact parity, opt-in/default-off). **Semantic** WGSL/opcodes
> stay banned and the anti-faking protocol is unchanged — lifting the generic-op ban only changes *how* the
> GPU may truly compute the transition, not the bar for proving it did. R1a uses that gate for a generic
> `Floor` EvalEML opcode and a generic Candidate-F max-magnitude WGSL helper; both are semantic-free,
> reusable, opt-in/default-off, and parity-tested.
> Per **SimThing Maximality**, any transition already expressed as
> row/mask/reduce/disburse/threshold/emission-band is promoted toward resident execution; the CPU may
> remain oracle/inspector/save-writer but may **not** be the hidden authority for state_N+1 when the
> track claims GPU-resident execution. The CPU shadow is **retained** as parity witness (not deleted —
> deleting it would break semantic-GPU parity); only authority moves to the resident buffer, reconciled
> at the **tick boundary** (the save/pause/stable-state point). First IMPL sub-rung
> **`RUNTIME-0080-0-R1a`** promotes the already-measured **Tier-A field columns** (disruption,
> location_status, stockpiles, construction_progress, per-cohort `num_ships` value, blockade/divert code,
> R4 magnitude) to a resident double-buffered next-tick authority — **no semantic op / semantic WGSL /
> atlas batching / M-4A**. **Tier-B** structural changes (REENROLL membership scatter, cohort birth/removal,
> fusion lineage) are applied by a **bounded CPU boundary-maintenance pass driven by a GPU-written event
> journal** (the boundaryEvent dispatch), which is GPU-decided / CPU-applied — **not** a CPU planner.
> **R0A is CLOSED as PARTIAL / informative** (correct honest outcome; no PR #531 change required).
>
> **R1b (`RESIDENT-EVENTLOG-0`) — IMPLEMENTED / PARTIAL (full journal parity) 2026-06-05:** [`runtime_0080_0_r1b.rs`](../crates/simthing-driver/src/runtime_0080_0_r1b.rs) adds a GPU-staged resident event journal on the R1a substrate. **All seven structural event kinds** (`MoveRequest`, `DamageDelta`, `ZeroCohort`, `ShipCountDelta`, `LocalBirthRequest`, `FusionRequest`, `OwnerCodeFlip`) are journalized and consumed by a bounded CPU boundary pass with **exact aggregate and per-tick oracle parity**, no movement/combat/production/blockade tick rederivation, and R1a Tier-A + R6C checksum preserved. The GPU-written resident event journal now drives bounded CPU structural boundary maintenance. **Resident structural scatter/compact is NOT yet claimed** — the structural *decisions* are still computed by a self-consistent CPU decision witness and staged into the GPU journal (`structural_decisions_gpu_emitted = false`); moving that decision authority resident is `R1c`. A structural-drift failure mode (reconstructing the CPU witness from partial value-only Tier-A readback) was found and fixed by making the witness self-consistent while the GPU value loop runs resident in parallel (the GPU is not starved waiting on the CPU). **Load-bearing invariant (carried into R1c):** the CPU shadow stays a *complete, serializable, pausable* world mirror in CPU land even after the GPU holds authority — every Tier-A value and Tier-B structural field lives in the shadow, a boundary save is whole and GPU-independent (reloads/continues bit-identically on a CPU-only host), and resident tables are an acceleration of the shadow, never its replacement. Report: [`tests/runtime_0080_0_r1b_resident_event_journal_results.md`](tests/runtime_0080_0_r1b_resident_event_journal_results.md); anti-drift + GPU-non-starvation contract: [`handoffs/runtime_0080_0_r1c_resident_decision_opening.md`](handoffs/runtime_0080_0_r1c_resident_decision_opening.md).
>
> **R1c (`RESIDENT-REENROLL-0`) — IMPLEMENTED / PARTIAL (STOP-LINE) 2026-06-05:** [`runtime_0080_0_r1c.rs`](../crates/simthing-driver/src/runtime_0080_0_r1c.rs) records the honest resident-decision gate instead of overclaiming PASS. It preserves R1b as the earned predecessor (`247` GPU journal rows exactly matching the oracle), keeps `structural_decisions_gpu_emitted = false`, proves the complete CPU-shadow contract with a deterministic serialize→reload→continue snapshot round-trip, and documents that resident REENROLL scatter, birth/removal authority, and fusion compaction remain behind the free-list-scatter / compaction stop-lines. R1c names the smaller executable next rung: `R1c-a resident free-list mark-only / no compaction`. Report: [`tests/runtime_0080_0_r1c_resident_decision_results.md`](tests/runtime_0080_0_r1c_resident_decision_results.md).
>
> **R1c-a (`RESIDENT-FREELIST-MARK-ONLY-0`) — IMPLEMENTED / PASS 2026-06-05:** [`runtime_0080_0_r1c_a.rs`](../crates/simthing-driver/src/runtime_0080_0_r1c_a.rs) implements the smaller resident free-list mark-only rung. R1b now exposes a compact mark-source projection from its **GPU-read** journal rows; R1c-a copies those source marks through a resident GPU slot bitmap, measures GPU-vs-oracle mark parity, and proves a disabled-marker negative control. It claims only `resident_free_list_mark_authority = true`; allocation into free slots, REENROLL scatter, birth/removal authority, fusion compaction, and `structural_decisions_gpu_emitted` remain false. Report: [`tests/runtime_0080_0_r1c_a_free_list_mark_results.md`](tests/runtime_0080_0_r1c_a_free_list_mark_results.md).
>
> **Next horizon:** `R1c-b` (resident allocation into marked free slots / no compaction); then resident scatter/compact for membership + cohort table behind the §11 / free-list-scatter stop-lines; then multi-atlas batching + M-4A masking
> (§11 gate); richer emergence (`SCENARIO-0080-3`); multi-faction ECON; system→planet recursion.
>
> > **✓ Adapter-scope caveat RESOLVED (2026-06-04, design authority).** `GpuContext` now **always selects
> > a discrete GPU when present** (`context.rs`), and the **full NVIDIA RTX 4080 validation ladder is
> > complete** — `docs/nvidia_fp_determinism_test.md` (Batteries 01–13). **`cargo test --workspace` is
> > green on the discrete RTX 4080** (60 binaries, 0 failed); the priority f32 `GpuVerified` rungs
> > (PACK-GPU, structured-field, m5 gradients, first-slice, f32 c-series) re-passed on the RTX, and
> > STORE-GPU's integer bit-exact held cross-adapter. **`ATLAS-BATCH-0-CLOSE` may proceed without the
> > adapter caveat.** *(Original caveat: prior to 2026-06-03 all GPU parity ran on the Intel iGPU because
> > `PowerPreference::default()` selected integrated; now fixed + re-validated on the discrete target.)*
>
> **Sequencing discipline (§0.5, §5):** one parked phase proved-and-closed per rung. The rehearsal is
> the **convergent consumer that retires the parked backlog one rung at a time** — not a big-bang pull.
> ATLAS-BATCH-0 (§12.3) is the pre-rehearsal prerequisite; R1–R7 are the full rehearsal. Each rung is
> proven **through a real reduction** (Scenario Proof), CPU-oracle parity, opt-in/default-off.

| Rung | Deliverable | Parked phase proved / closed | Pulls |
|---|---|---|---|
| **Open — scenario admission** | admit the rehearsal scenario through the accepted simthing-spec / CLAUSE-SPEC **L0/L1/L2** designer-admission layer (scenario spec, bounds, rejection vocabulary) — the Tier-2 gate that opens the rehearsal | **simthing-spec / CLAUSE-SPEC (L0/L1/L2)** consumed as the authoring engine | CLAUSE-SPEC admission |
| **Pre — `ATLAS-BATCH-0`** (§12.3) | static map gen + Location gridcell primitive + atlas batch allocation + 2-D-map storage | **Atlas batch allocation (C / M-4)**; OWNER masked-reduction storage | atlas runtime; `mobility_owner0` masked reduction |
| **R1 — Disruption heatmap (EC1)** — *ACCEPTED / CLOSED / IMPLEMENTED-PASS 2026-06-04 ([spec](scenarios/scenario_0080_2_r1_disruption_heatmap_opening_spec.md), [report](tests/scenario_0080_2_r1_disruption_heatmap_report.md), [acceptance](tests/scenario_0080_2_r1_acceptance_review.md))* | pirate/patrol presence → `disruption` column on gridcell SimThings → BoundedFeedback decay → diffuse to `location_status` → reduce up to the starmap heatmap; vs CPU oracle; emitted deterministic artifact | **EML Tier-2 `BoundedFeedback`/`Decay`** (first real consumer); EC1 | EML temporal gadgets; stencil diffusion; SlotRange reduce |
| **R2 — Recursive allocation + faction economy + blockade/divert** — *IMPLEMENTED / PASS 2026-06-04 ([spec](scenarios/scenario_0080_2_r2_recursive_reduce_opening_spec.md), [opening review](tests/scenario_0080_2_r2_opening_review.md), [report](tests/scenario_0080_2_r2_recursive_allocation_report.md))* | reduce-up **+ disburse-down** (one §0.2 behavior): production reduces up to per-faction stockpiles (OWNER-masked, never merged) + subsidiarity disburse-down to deficit systems; **blockade `≥100` gates outflow + divert flips the production owner-column to the blockader (column flip, not reparenting, no occupant moved)** — consuming the accepted R1 disruption field. **Single galactic tier** (system→planet recursion = named build fork); excludes R3/R5/R6. opt-in/default-off; CPU oracle | **A-0 nested Resource Flow (disburse-down)** off `FlatStarResourceFlow`; **ECON clearinghouse (subsidiarity) + faction-index contention (ECON-SCALE reuse)**; the §6 blockade/divert mechanic | A-0 nested RF; ECON Balance ledger / faction-index; OWNER masked reduction; AccumulatorOp recipe |
| **R3 — Capability-tree mask-down** — *IMPLEMENTED / PASS 2026-06-04 ([report](tests/scenario_0080_2_r3_capability_mask_down_report.md))* | Terran/Pirate capability trees resolve → modifier overlays (decay resistance, patrol suppression, combat bonus placeholder) masked **down** by owner-column onto cells/occupants; read-side only; no reparenting or combat resolution | **Capability-tree → modifier-overlay substrate** (first real consumer); OWNER mask-down end-to-end | capability-tree substrate; OWNER latched overlays |
| **R4 — FIELD_POLICY field-consumption + exact sqrt (EC2)** — *IMPLEMENTED / PASS 2026-06-04 ([report](tests/scenario_0080_2_r4_field_policy_consumption_report.md))* | a moving child (fleet/patrol) reads the parent grid heatmap **at its own cell** — a composite intersecting **patrol-presence × disruption × its own (masked) disposition** — computes the gradient, evaluates **Euclidean magnitude via exact sqrt Candidate F**, and threshold-gates: **sit still vs step to the next opportunity** | **FIELD_POLICY ladder field-consumption (EC2)** — closes the audit gap; **exact sqrt Candidate F** (named consumer for the orphaned artifact) | FIELD_POLICY OBS/EVENT/PIPE/ACT; `m_jit_mag2_fixed_exact` → `m_jit_mag_f_from_exact_mag2` (Candidate F); `GradientXY` |
| **R5 — Movement: REENROLL + mobility substrate (+ ship fission)** — *IMPLEMENTED / PASS 2026-06-04 ([report](tests/scenario_0080_2_r5_movement_reenroll_report.md))* | the R4 move event (`Threshold`+`EmitEvent`→`BoundaryRequest`) relocates the mover — deregister from cell A's arenas, register into cell B's — routed through the 0.0.7.9 mobility/transfer substrate (`compose_mobility_runtime0`, opt-in harness); **starport→ship emission instantiates a new `Fleet` via gated ALLOC arrival** and enrolls it | **REENROLL**; **0.0.7.9 mobility/transfer substrate composition** (dress-rehearsal consumer); **ALLOC arrival fission** (starport ship instantiation) | REENROLL; mobility ALLOC/IDROUTE/OWNER; `plan_mobility_alloc0` arrival |
| **R6 — Combat as fleet-cohort Resource Flow arena** — *IMPLEMENTED / PASS 2026-06-04 ([report](tests/scenario_0080_2_r6_combat_hp_damage_report.md); R6A fleet-cohort correction)* | co-located hostile **fleet cohorts** (10 ships × 100 HP/ship, 50 damage/ship/tick) in a local gridcell arena: damage **reduce-up** by owner channel, **disburse-down** to hostile cohorts, emission-band `ships_destroyed = floor(received/hp_per_ship)`, removal only at `num_ships_after == 0` via MOBILITY-ALLOC-0 `Departure`; consumes R5 post-move membership + upstream R1–R4 | **§0.3 all-conflict-is-resource-flow** — combat as adversarial Resource Flow, not a bespoke combat engine | combat arena; masked reduction / disbursement (ATLAS-BATCH-0 EC-A3 shape) |
| **R6B — Ship production threshold emission + cohort reinforcement/fusion** — *IMPLEMENTED / PASS 2026-06-04 ([report](tests/scenario_0080_2_r6b_ship_cohort_reinforcement_report.md))* | starport/planetary production accumulates `construction_progress`; threshold emits `ship_count_delta`; owner/cell/profile masked selection reinforces compatible friendly cohort (`num_ships` increment, no movement `BoundaryRequest`) or locally births/enrolls a Fleet via ALLOC arrival; friendly co-located compatible cohorts fuse by masked reduction; `hp_to_retire` and `damage_output` recompute from `num_ships`; R6A combat consumes updated cohort sizes; CPU oracle verifies row ops only | **§0.2 reduce/disburse + emission-band** — production growth as Resource Flow, not CPU fleet manager | construction threshold; cohort compaction; MOBILITY-ALLOC-0 Arrival/Departure for enrollment coherence |
| **R6C — Integrated multi-tick run (the ladder's culmination)** — *IMPLEMENTED / PASS 2026-06-04 ([opening spec](scenarios/scenario_0080_2_r6c_integrated_run_opening_spec.md), [report](tests/scenario_0080_2_r6c_integrated_run_report.md))* | assembled R1→R6B into **one mutable session-state** and ticked it **100 times with feedback**: disruption (R1) → economy/blockade/divert (R2) → overlays (R3) → FIELD_POLICY field read (R4) → movement/REENROLL + fission (R5) → combat on **movement-produced co-location** (R6) → production/reinforcement/fusion (R6B), then wrote positions, ship counts, stockpiles, and disruption forward. Emits trace excerpts, §8.1 detector table, race curve, conservation rows, and stable checksum `1bba891c779190a4`. opt-in/default-off; CPU-oracle primary; **no** default SimSession wiring, **no** CPU planner, **no** new invariant; GPU-MEASURE-0080-0 measured constituent shapes; RUNTIME-0080-0-R0A per-tick GPU shape dispatch against CPU-authoritative run (whole-run remains `GPU-conformant; GPU execution not yet measured`, report [`tests/runtime_0080_0_r0_results.md`](tests/runtime_0080_0_r0_results.md)) | **the integrated emergence run** — closes the §8.1 observation gap; first blockade tick 2, movement-produced combat tick 44, production reinforcement tick 49 | R1–R6B rungs; mobility runtime tick composition; FrontierV2 closed-loop feedback pattern |
| **R7 — CLOSE + closeout integrity + report** — *CLOSED / PASS 2026-06-04 ([closeout](tests/scenario_0080_2_r7_closeout_report.md), [human report](gameplay/scenario_0080_2_pirate_gradient_pathfinding_results.md))* | design-authority closeout + human/layman report reclosed after R6C. Mechanism-chain proof (R1–R6B) stands and R6C supplies integrated-run evidence for §8.1. Numeric-only reconciliations carry forward; R4 tie-breaker is not attributed as emergence; movement remains greedy local FIELD_POLICY, not route search; R6C whole-run GPU posture corrected by R0A remedial (CPU tick authority; per-tick shapes GPU-dispatched) | the **closeout-integrity** meta-opportunity; the **proof+emergence narrative** | R6C run trace |

**R4 detail (exact-sqrt chain — design authority).** The FIELD_POLICY gradient magnitude must be
**exact-authoritative** so move/sit decisions are deterministic across GPU adapters (I8). Chain:
fixed-point `dx/dy` → **exact pre-sqrt mag2** (`m_jit_mag2_fixed_exact` / `ExactFixedPointDxDy`) →
**Candidate F sqrt** (`m_jit_mag_f_from_exact_mag2`, artifact hash `e2e9e27601ee2e13`) → exact Euclidean
magnitude → threshold. Raw f32 `dx/dy` magnitude is `ApproximateDiagnostic` and **may not gate the
commitment** (invariants: "Exact Euclidean magnitude requires exact pre-sqrt mag2"; "Exact sqrt authority
is artifact-backed (Candidate F)"). The composite the mover reads is the **multi-channel cell weighted by
its masked-down disposition** (R3): a pirate weights low-patrol + high-opportunity (move toward clean
systems, through disruption it can pass); a patrol weights high-disruption (move *toward* it to suppress).
Same machinery — disposition is just the weight vector; sit-still is the below-threshold case.

**R5 detail.** Movement *is* the mobility substrate exercised in a real session pass: the FIELD_POLICY event
materializes a `BoundaryRequest` that re-enrolls the mover (REENROLL) and routes it via the parked
0.0.7.9 mobility/transfer substrate (IDROUTE identity preserved, no reparenting). This is the
"first non-test-support default `SimSession` path" the mobility gate was mapped to — coincident with the
movement rung rather than a standalone slice.

**Parked-inventory coverage audit (every constitution §3 parked track has a test home or a reason to stay parked):**

| Parked track (constitution §3 / §4) | Test home |
|---|---|
| 0.0.7.9 mobility/transfer: REENROLL, ALLOC, IDROUTE, GPU kernel, RUNTIME | **R5** |
| 0.0.7.9 mobility/transfer: OWNER | **ATLAS-BATCH-0** (masked-reduction storage) + **R3** (mask-down) |
| 0.0.7.9 mobility/transfer: ECON (clearinghouse + faction-index contention) | **R2** |
| Line A — nested Resource Flow (A-0, depth>2) | **R2** |
| Line C — atlas / multi-theater (batch allocation) | **ATLAS-BATCH-0** |
| simthing-spec / CLAUSE-SPEC (L0/L1/L2) | **Open** (scenario admission) |
| EML Tier-2 temporal (`BoundedFeedback`/`Decay`, `VelocityMonitor`) | **R1** |
| `field_urgency` / `field_pressure` | **R2** |
| Capability-tree → modifier-overlay | **R3** |
| FIELD_POLICY ladder field-consumption (OBS/EVENT/PIPE/ACT) | **R4** |
| Exact sqrt Candidate F (+ `mag2_fixed_exact`) | **R4** |
| `GradientXY` (landed) | **R1 / R4** (consumed) |
| E-11B-5 / E-2B-5 fission-enrollment | **R5** (starport→ship) |
| Conflict-as-resource-flow (combat HP/Damage) | **R6** |
| Ship production reinforcement / friendly cohort fusion | **R6B** |
| Closeout integrity (FrontierV1, mapping first-slice) | **R7** |

**Stay gated — no consumer in this scenario (correct to leave parked):** B-1 hard currency (no hard
currency); ClauseThing/ClauseScript L3 (no front-end); dense per-cell temporal memory (bounded-feedback
per-cell, not dense temporal); atlas sparse-residency scheduler / M-4A (static map); FrontierV2-5;
Hybrid-Strata ECON scaling beyond the 2-faction set (Terran/Pirate ECON-SCALE is reused, not extended).

---

## 13. Pointers

- Active constitution: [`design_0_0_8_0.md`](design_0_0_8_0.md)
- Parked 0.0.7.9 mobility/transfer track: [`design_v7_9_mobility_transfer_allocation_production_track.md`](design_v7_9_mobility_transfer_allocation_production_track.md)
- Gating mechanics + proven-capability stop rule: [`workshop/phase_m_gating_and_doc_policy.md`](workshop/phase_m_gating_and_doc_policy.md)
- Binding structural rules: [`invariants.md`](invariants.md)
- Active status table + read order: [`workshop/mapping_current_guidance.md`](workshop/mapping_current_guidance.md)
