# 0.0.8.6 — Studio Live Ops (Phase 9)

> **Status: OPEN / Owner-directed (2026-07-09).** Production Studio track for **realtime observation and
> control** of loaded scenarios (including clause-ingested StructuralRebindReady sessions). Sits **after**
> 0.0.8.5 Phase 8 open/hydrate spine (transpile → API → picker) and **beside** (does not close) the
> Terran-Pirate scenario envelope. **Not closeout of 0.0.8.5.**
>
> **Purpose.** Give Studio an operator-grade live path: pause/play, 2×/4× acceleration, max ticks/second,
> observation of live state, and a separate modal scenario library (load/create/save) that pauses the sim
> while visible. UI is presentation; ScenarioSpec remains authority; Bevy/GPU/clock UI never become model
> authority or a CPU planner.
>
> **Prerequisite (landed, do not re-derive):**
> - Clause → StructuralRebindReady Spec → Studio session hydrate (`TP-STUDIO-CLAUSE-API-1`, #1230)
> - Open ClauseScript Scenario… picker (`TP-STUDIO-CLAUSE-PICKER-0`, #1239)
> - Workshop live-run multi-tick proof (`TP-LIVE-RUN-0`, #1217) — residue for production bridge only
>
> **Baseline client:** debug `simthing-studio` on master (rebuild: `cargo build -p simthing-mapeditor --bin simthing-studio`).

---

## 0. Track harness header (constitution §0.5 Rule 1)

**Fixed base (durable — hold every rung):**

1. [`simthing_core_design.md`](simthing_core_design.md) **§1.2 / §1.2.1** — admission ladder; residue-as-tripwire.
2. [`design_0_0_8_3.md`](design_0_0_8_3.md) **§0** — constitution (anti-flattening, STEAD, closed-lowering).
3. [`design_0_0_8_3_studio_production.md`](design_0_0_8_3_studio_production.md) — Studio doctrine: Spec authority; Bevy/UI presentation-only.
4. **This file** — 0.0.8.6 canonical design / PR ladder.
5. [`stead_spatial_contract.md`](stead_spatial_contract.md) — STEAD invariants for any live spatial tick.
6. [`ci_screening_surface.md`](ci_screening_surface.md) + [`handoff_template.md`](handoff_template.md) + [`agent_onboarding.md`](agent_onboarding.md).
7. Prior product spine: [`design_0_0_8_5_clausescript_terran_pirate_galaxy.md`](design_0_0_8_5_clausescript_terran_pirate_galaxy.md) Phase 8 (hydrate/picker) — consume, do not re-open unless regression.

**Established decisions — do NOT re-derive:**

- **ScenarioSpec is the sole model authority.** Studio projections, Bevy ECS, camera, dialogs, clocks, FPS/VRAM telemetry are **presentation** (studio production doctrine).
- **Decisions remain FIELD_POLICY / threshold crossings** — no CPU planner, no “AI tick” beside the tree.
- **Clock UI never invents gameplay outcomes** — it only schedules admitted sim ticks / pauses them.
- **Clause load stays on production ingest/picker** — library reuses API; no TP fixture defaults in production.
- **Modal scenario library ⇒ sim paused** while visible (binding UX law for this track).
- **Gate-wiring remains DA-reserve.** Clearance class for live-ops lands only after UI shape stabilizes.
- **GHA does not run Bevy/GPU Studio.** Owner-local / desktop proof remains citable; CI is greppable + targeted tests.
- **0.0.8.5 is not closed by this track.** Closeout of Terran-Pirate remains Owner-triggered only.

---

## 1. Root cause this track closes

Phase 8 delivered **open and hydrate** (clause → StructuralRebindReady → Studio session). Operators still cannot **run and control** that session in realtime:

| Gap | Effect |
|---|---|
| No Studio sim clock (pause/play/rate/TPS) | Loaded scenario is structural view only |
| Live-run proof is workshop/theater-homed | Not production Studio operator path |
| Scenario I/O scattered | No modal library that freezes the world while authoring I/O |
| Observation of live tick state ad hoc | Cannot verify “it’s running” from UI |

This track closes the **operator live-ops** gap without reopening combat/diplomacy subsystems or atlas full-galaxy scheduling.

---

## 2. Bevy / presentation law (anchor surface)

## 2.1 Bevy and Studio UI are non-authority

> **Binding.** Bevy entities, transforms, materials, egui dialogs, camera, render caches, FPS/VRAM
> telemetry, and **sim clock presentation widgets** are **not** ScenarioSpec authority and must not
> become the production decision engine. Model edits and structural truth land on ScenarioSpec first;
> Studio rebuilds projection/render from authority. A tick scheduled by the clock must invoke the
> **admitted sim/driver path**, not a Bevy-side planner.

Trigger domains for anchors: `bevy-presentation`, `studio-ui`, `sim-clock`.

## 2.2 Modal library pause law

> **Binding.** While the scenario library window is **visible / modal**, the live sim clock is
> **paused**. Closing the library does not auto-Play; the operator must press Play. Load/create/save
> use existing authority I/O and production clause ingest — no silent TP defaults.

---

## 3. Success metrics (falsifiable)

| Metric | Target |
|---|---|
| Loaded clause/JSON scenario advances under Play | Multi-tick identity held; pause freezes |
| 2× / 4× | Rate ratio vs 1× within documented tolerance under TPS cap |
| Max TPS selector | Cap holds (no unbounded tick storm) |
| Library modal | Open ⇒ paused; load/save via authority + clause API |
| Bevy/UI | No ScenarioSpec mutation from pure render/camera |
| Clearance | Live-ops class clearable after shape stable; gate-wiring for harness only |

---

## 4. Phase 9 PR ladder

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 9.0 | `STUDIO-LIVE-OPS-READINESS-0` | **Docs only.** Map: tick path today (render vs SimSession/driver), clock ownership, pause semantics, modal pause contract, observation surfaces, whether GameMode/RF attach is still a production blocker for “live” vs structural. Non-goals explicit. | Readiness report in `docs/tests/studio_live_ops_readiness_0_results.md`; Owner/DA may admit clock+library as production UI after this | Tier-1 |
| 9.1 | `STUDIO-SIM-CLOCK-0` | **Sim clock substrate** (presentation + driver bind): pause / play / 1× / 2× / 4×; **max ticks per second**; deterministic ordering under accel; clock does not invent decisions. | Targeted tests: pause freezes; rate ratios; TPS cap | Tier-2 |
| 9.2 | `STUDIO-SIM-CLOCK-UI-0` | **Transport UI:** Pause, Play, 2×, 4×, TPS selector; readout (tick index, effective rate, paused). | UI drives clock; programmatic hooks for CI | Tier-2 |
| 9.3 | `STUDIO-LIVE-SESSION-BRIDGE-0` | **Wire loaded StudioSession → live tick path** (elevate workshop live-run policy only as needed). Prefer production driver/session. Bounded theater policy from readiness if required. No new gameplay systems. | Loaded clause/JSON multi-tick under Play; STEAD/session identity held | Tier-2 |
| 9.4 | `STUDIO-LIVE-OBSERVE-0` | **Observation surfaces:** tick, pause, optional tree-local summaries already available — no CPU planner. | Values update while running; freeze on pause | Tier-1 |
| 9.5 | `STUDIO-SCENARIO-LIBRARY-UI-0` | **Toggled/hidden library window:** load / create / save (JSON authority + clause open reusing production ingest/picker). **Modal/visible ⇒ pause.** | Modal open → paused; I/O through existing APIs | Tier-2 |
| 9.6 | `STUDIO-SCENARIO-LIBRARY-CREATE-0` | Create-new / blank or template from library (scope from readiness). | Create → loadable session; no TP hardcodes | Tier-2 |
| 9.7 | `STUDIO-LIVE-OPS-CLASS-0` | **Gate-wiring.** Precedented class for live-ops UI + clock shape. | Clearance selftests clearable + envelope rejects | Tier-2 |
| 9.8 | `STUDIO-LIVE-OPS-HARDENING-0` | Polish: cancel modal, double-open, rapid rate change, save-while-paused, no tick on modal. | Regression battery | Tier-1 |

**Dependency order:** 9.0 → 9.1 → (9.2 ∥ 9.3) → 9.4 / 9.5 → 9.6 → 9.7 → 9.8.

**Admission:** After 9.0, Owner/DA may issue a short admission stamp for “Studio live clock + library UI” before 9.1/9.5 if required by process; default is to treat 9.0 report as the gate.

---

## 5. Explicit non-goals

- Closing 0.0.8.5 Terran-Pirate / `TP-DA-CLOSEOUT-0` without Owner declaration  
- Atlas full-galaxy scheduler  
- New combat/diplomacy/AI subsystems  
- Auto-Play on load or on library close  
- GHA Bevy/desktop GPU proof  
- Parallel authority model or “Bevy as source of truth”

---

## 6. Birth track / inventory

New tests under this track use `birth_track = 0.0.8.6-studio-live-ops` once the lifecycle track row is registered. Do not put live-ops tests under `0.0.8.5-terran-pirate` unless they are TP-scenario residue.

---

## 7. Park / open posture

| Item | State |
|---|---|
| Active track | This file (after `--open`) |
| First open rung | `STUDIO-LIVE-OPS-READINESS-0` |
| Debug baseline | `cargo build -p simthing-mapeditor --bin simthing-studio` |
| Clause load baseline | Production picker + API (explicit resolver if `{{…}}`) |

**Park instruction for agents:** do not implement 9.1+ until 9.0 readiness is landed (or Owner explicitly parallelizes). Do not treat orientation pointer as closeout of 0.0.8.5.
