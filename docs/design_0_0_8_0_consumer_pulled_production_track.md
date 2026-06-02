# SimThing — Design 0.0.8.0 Consumer-Pulled Production Track

> **Local Patrol Economy 0.0.8.0 vertical slice COMPLETE / PARKED after `DEMO-0080-0`** (2026-06-02).
> Scenario → production path → schedule → observation → control → demo/export are all IMPLEMENTED / PASS.
> **No further work opens on this slice without a new named product scenario or explicit product
> authorization.** Closeout: [`tests/phase_local_patrol_economy_0080_closeout_results.md`](tests/phase_local_patrol_economy_0080_closeout_results.md).
>
> **Second scenario opened:** `SCENARIO-0080-1` — **Nested Starmap (Terran/Pirate multi-theater)** —
> ACCEPTED (2026-06-02). It deliberately opens two previously-parked substrate gates:
> `ATLAS-0080-0` (atlas production runtime / sparse-residency nested mapping) is **IMPLEMENTED / PASS**;
> `ECON-SCALE-0080-0` remains open/not implemented (multi-faction faction-index ECON scaling, pirate as
> full economy faction). Opt-in/default-off. Packet: [`scenarios/scenario_0080_1_admission_packet.md`](scenarios/scenario_0080_1_admission_packet.md);
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
- **One principle per class; no per-slice accretion** (`invariants.md` governing doctrine).

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
| `ECON-SCALE-0080-0` | Multi-faction (Hybrid-Strata/faction-index) ECON scaling | **OPEN — docs/design gate; no implementation** — opened by `SCENARIO-0080-1` admitting the pirate as a full economy faction (adversarial participant in starsystem resource flow); bounded fixed faction set, faction-indexed contended clearing within the subsidiarity model; **no hard currency / markets / trade / `ai_budget`, no nested RF**. Spec: [`production_paths/econ_scale_0080_0_opening_spec.md`](production_paths/econ_scale_0080_0_opening_spec.md). |
| `PRODUCTION-PATH-0080-1` | `SCENARIO-0080-1` opt-in production path | **NOT YET OPENED** — opens after `ATLAS-0080-0` and `ECON-SCALE-0080-0` opening specs are accepted |
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

## 12. Pointers

- Active constitution: [`design_0_0_8_0.md`](design_0_0_8_0.md)
- Parked 0.0.7.9 mobility/transfer track: [`design_v7_9_mobility_transfer_allocation_production_track.md`](design_v7_9_mobility_transfer_allocation_production_track.md)
- Gating mechanics + proven-capability stop rule: [`workshop/phase_m_gating_and_doc_policy.md`](workshop/phase_m_gating_and_doc_policy.md)
- Binding structural rules: [`invariants.md`](invariants.md)
- Active status table + read order: [`workshop/mapping_current_guidance.md`](workshop/mapping_current_guidance.md)
