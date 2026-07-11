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
| 9.0 | `STUDIO-LIVE-OPS-READINESS-0` | **Docs only.** Map: tick path today (render vs SimSession/driver), clock ownership, pause semantics, modal pause contract, observation surfaces, whether GameMode/RF attach is still a production blocker for “live” vs structural. Non-goals explicit. | **ORCHESTRATOR-GRADUATED / merged [#1257](https://github.com/khorum08/SimThing/pull/1257) @ `4f8c250c`** — readiness report [`studio_live_ops_readiness_0_results.md`](tests/studio_live_ops_readiness_0_results.md); `READY_FOR_9_1: YES`; GameMode/RF not a 9.1 blocker. | Tier-1 |
| 9.1 | `STUDIO-SIM-CLOCK-0` | **Sim clock substrate** (presentation + driver bind): pause / play / 1× / 2× / 4×; **max ticks per second**; deterministic ordering under accel; clock does not invent decisions. | **DA-GRADUATED / merged [#1258](https://github.com/khorum08/SimThing/pull/1258)** — `StudioSimClock` headless substrate; pause freeze / 2×·4× ratios / max_tps cap / no Spec mutation; evidence [`studio_sim_clock_0_results.md`](tests/studio_sim_clock_0_results.md). **Next:** `STUDIO-SIM-CLOCK-UI-0` (∥ `STUDIO-LIVE-SESSION-BRIDGE-0`). | Tier-2 |
| 9.1h | `STUDIO-SIM-CLOCK-CLASS-0` | **Harness adjacency (clearance router).** Register precedented class `studio-sim-clock-substrate` so #1258-shaped StudioSimClock substrate diffs are ORCHESTRATOR-CLEARABLE; reject UI/bridge/library/runtime/GPU/kernel/gate-wiring. | **DA-GRADUATED / merged [#1259](https://github.com/khorum08/SimThing/pull/1259) @ `d7ceb754`** — selftests clearable + envelope rejects; evidence [`studio_sim_clock_class_0_results.md`](tests/studio_sim_clock_class_0_results.md). | Tier-2 |
| 9.2 | `STUDIO-SIM-CLOCK-UI-0` | **Transport UI:** Pause, Play, 2×, 4×, TPS selector; readout (tick index, effective rate, paused). | **DA-GRADUATED / merged [#1283](https://github.com/khorum08/SimThing/pull/1283)** — transport facade over landed `StudioSimClock` (6 headless proofs + substrate regression; ScenarioSpec non-mutation directly tested); evidence [`studio_sim_clock_ui_0_results.md`](tests/studio_sim_clock_ui_0_results.md). **Next:** `STUDIO-LIVE-SESSION-BRIDGE-0` (∥ observe/library). | Tier-2 |
| 9.3 | `STUDIO-LIVE-SESSION-BRIDGE-0` | **Wire loaded StudioSession → live tick path** (elevate workshop live-run policy only as needed). Prefer production driver/session. Bounded theater policy from readiness if required. No new gameplay systems. | **DA-GRADUATED / merged [#1285](https://github.com/khorum08/SimThing/pull/1285) @ `ab238657`** — `SimSession::step_once` shares `run` hot-cycle body; clock→bridge→production session; JSON + clause-shaped multi-tick; pause freeze; STEAD/session identity; no workshop import; evidence [`studio_live_session_bridge_0_results.md`](tests/studio_live_session_bridge_0_results.md). **Next:** `STUDIO-LIVE-OBSERVE-0`. | Tier-2 |
| 9.4 | `STUDIO-LIVE-OBSERVE-0` | **Observation surfaces:** tick, pause, optional tree-local summaries already available — no CPU planner. | **DA-GRADUATED / merged [#1287](https://github.com/khorum08/SimThing/pull/1287) @ `5d3ed74c`** — pure `StudioLiveObservationReadout` + UI over clock/bridge/session; update-while-running / freeze-on-pause; no tick/Spec mutation; 10 headless proofs; evidence [`studio_live_observe_0_results.md`](tests/studio_live_observe_0_results.md). **Next:** `STUDIO-SCENARIO-LIBRARY-UI-0`. | Tier-1 |
| 9.5 | `STUDIO-SCENARIO-LIBRARY-UI-0` | **Toggled/hidden library window:** load / create / save (JSON authority + clause open reusing production ingest/picker). **Modal/visible ⇒ pause.** | **DA-GRADUATED / merged [#1289](https://github.com/khorum08/SimThing/pull/1289) @ `d2493dc2`** — modal open pauses via `StudioSimClockTransport`; bridge freeze while visible; JSON authority + production Clause I/O; close/I/O no autoplay; Create deferred to 9.6; 12 headless proofs; evidence [`studio_scenario_library_ui_0_results.md`](tests/studio_scenario_library_ui_0_results.md). **Next:** `STUDIO-SCENARIO-LIBRARY-CREATE-0`. | Tier-2 |
| 9.6 | `STUDIO-SCENARIO-LIBRARY-CREATE-0` | Create-new / blank or template from library (scope from readiness). | **DA-GRADUATED / merged [#1291](https://github.com/khorum08/SimThing/pull/1291) @ `e42a248f`** — blank one-cell `SimThingScenarioSpec` + `from_loaded_scenario` hydrate; STEAD/links; authority save/load roundtrip; fail-loud atomic create; bridge detach on replace; modal pause/no-autoplay; 12 headless proofs; evidence [`studio_scenario_library_create_0_results.md`](tests/studio_scenario_library_create_0_results.md). **Next:** `STUDIO-LIVE-OPS-CLASS-0`. | Tier-2 |
| 9.7 | `STUDIO-LIVE-OPS-CLASS-0` | **Gate-wiring.** Precedented class for live-ops UI + clock shape. | **DA-GRADUATED / merged [#1293](https://github.com/khorum08/SimThing/pull/1293) @ `9a3c42eb`** — class `studio-live-ops-ui-clock` + predicate priority 40; 12 fixtures clearable + envelope rejects; substrate nonregression; live-bridge supersession bounded; selftest 90 PASS; evidence [`studio_live_ops_class_0_results.md`](tests/studio_live_ops_class_0_results.md). **Next:** `STUDIO-LIVE-OPS-HARDENING-0`. | Tier-2 |
| 9.8 | `STUDIO-LIVE-OPS-HARDENING-0` | Polish: cancel modal, double-open, rapid rate change, save-while-paused, no tick on modal. | **DA-GRADUATED / merged [#1295](https://github.com/khorum08/SimThing/pull/1295)** — 13-proof hardening battery (bridge-reset on all session-replacement paths, modal cancel keeps Pause + purges pending, double-open idempotent, save-while-paused no-tick, fail-loud save-error); presentation-only, zero forbidden surfaces; evidence [`studio_live_ops_hardening_0_results.md`](tests/studio_live_ops_hardening_0_results.md) | Tier-1 |

**Dependency order:** 9.0 → 9.1 → 9.1h (class for 9.1 merge) → (9.2 ∥ 9.3) → 9.4 / 9.5 → 9.6 → 9.7 → 9.8.
**Phase 9 status: COMPLETE (2026-07-10).** All 10 rungs graduated; the operator live-ops path
(clock → transport UI → production-session bridge → observe → scenario library → hardening) is landed.

---

## 4b. Phase 10 PR ladder — Studio UI / Control Refinement (OPEN)

> **Scope.** Presentation-only refinement of the landed Studio live-ops surface: transport controls,
> live readouts, scenario-library ergonomics, keyboard/mouse affordances, layout and state clarity.
> **All Phase-9 doctrine holds unchanged:** ScenarioSpec is sole authority; Bevy/egui/clock are
> presentation; no CPU planner; modal library ⇒ paused; no Auto-Play; no new gameplay/RF/GameMode.
> Phase-10 diffs are **`studio-live-ops-ui-clock`-clearable** (class scope widened at #1296 to the
> `studio_*_0.rs` test shape + evidence ledgers; Phase-11 widen adds clause ingest/picker/scenario_io
> + star/galaxy render so Tier-B presentation stays orch-mergeable). Forbidden globs still hard-reject
> driver/kernel/sim/gpu/spec/clause/**src**/wgsl/gate. A rung that must touch an authority crate
> (`simthing-spec` / `simthing-clausething` / `simthing-mapgenerator`) falls outside the envelope and
> reserves to DA by design.
>
> **Extensible.** This ladder grows on Owner direction; the DA scopes each rung from a stated tweak.
> Additional Phase ladders (11+) append to this file, never fork it. **Authoring rule:** Phase-10/11 UI
> rungs touch an already-enumerated live-ops detector path (`studio_*` live-ops modules, `app/ui` together
> with those modules, `star_render.rs` / `galaxy_render.rs`, or matching `studio_clause_*` /
> `studio_faction_*` / `studio_owned_*` / `studio_frosted_*` / `studio_live_ops_*` tests) so the class
> detector fires. Production ingest (`clause_scenario_ingest.rs` / picker / `scenario_io`) is
> **scope-admitted** once a live-ops detector fires — do not open a DA relay for envelope under-width
> alone; widen `class_predicates.tsv` scope instead. A brand-new UI module is added to class
> `match_any` as part of its own rung.

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 10.0 | `STUDIO-TRANSPORT-OBSERVE-REFINE-0` | Presentation polish of transport + live-observe surface (affordances/layout/shortcuts). | **PARKED** — superseded by Phase 11 (concrete UI scope landed 2026-07-11); reopen only if a transport-specific tweak is requested that Phase 11 doesn't cover. | Tier-1 |

**Dependency order:** 10.0 → (further UI rungs appended on Owner direction).

---

## 4c. Phase 11 PR ladder — Scenario Presentation & Faction Identity (OPEN, ACTIVE)

> **Owner-directed (2026-07-11)** from live debug-client review. Four concerns: (1) ClauseScript-only
> loader with the source-JSON resolver auto-selected off-screen and surfaced as read-only scenario
> telemetry; (2) stars are unnamed — the galaxy was generated without the star-naming pass; (3) owning
> factions gain identity fields (RGB color + faction name + alliance, placeholders for later) reflected
> in nameplates and an owned-star selection highlight; (4) window frosted-glass backgrounds need a real
> **performant** darken+blur.
>
> **Doctrine unchanged:** ScenarioSpec is authority; Bevy/egui/clock are presentation; no CPU planner.
> **Tiering:** data-model / generator rungs touch authority crates (spec/clausething/mapgenerator) and
> are **DA-reserve**; pure-mapeditor presentation rungs are **`studio-live-ops-ui-clock`-clearable**
> (including production ingest `source_base` wire in `clause_scenario_ingest.rs`, and nameplate /
> owned-star presentation in `star_render.rs` / `galaxy_render.rs`). **Do not escalate Tier-B to DA**
> solely for `class-envelope-violation` / `admitted-scope-router-gap` — that is class-hardening debt;
> widen `class_predicates.tsv` and re-run clearance. UI rungs depend on their data rung landing first
> (colors need the field; named nameplates need the pass).
>
> **Model tier (coding-agent selection):** **Std** = regular coding model (mechanical / precedented / presentation; the handoff is prescriptive). **Frontier** = fable/codex-class required — reserved for load-bearing subtlety only: **11.3** (seed-stable deterministic naming + golden base-disc regen) and **11.7** (frame-budget GPU blur / shader correctness). Everything else is Std; DA review is the safety net on all Tier-A regardless of model.

### Tier A — data / authority (DA-reserve)

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 11.1 | `STUDIO-CANONICAL-SCENARIO-0` | **Foundation.** Resolve clause `source_json`/includes **relative to the clause file's directory** (not process cwd) in `simthing-clausething::hydrate_scenario`; commit a self-contained `scenarios/terran_pirate_galaxy.clause` + sibling base-disc JSON that loads with an **empty resolver from any cwd**. Removes the `{{FIXTURE_JSON}}` test-harness leak from the operator path. | **DA-GRADUATED / merged [#1300](https://github.com/khorum08/SimThing/pull/1300) @ `46f6151e`** — `hydrate_scenario_with_source_base` + clause-dir resolve; portable scenarios pair; 4 proofs; token back-compat; no sibling output; evidence [studio_canonical_scenario_0_results.md](tests/studio_canonical_scenario_0_results.md). **Next:** `STUDIO-FACTION-IDENTITY-FIELDS-0`. | DA-reserve · Std |
| 11.2 | `STUDIO-FACTION-IDENTITY-FIELDS-0` | Owner/faction identity fields on the scenario spec + clause grammar + hydrate: **`color_rgb`** (required; drives UI), **`faction_name`**, **`faction_alliance`** (placeholder), reserved forward placeholders. TP owners (Terran/Pirate) carry distinct colors in the canonical scenario. Authority + admission only; no UI. | **DA-GRADUATED / merged [#1302](https://github.com/khorum08/SimThing/pull/1302) @ `f18efd1b`** — Spec identity props + clausething grammar/hydrate; Terran/Pirate distinct colors; fail-loud color rules; 8 proofs; evidence [studio_faction_identity_fields_0_results.md](tests/studio_faction_identity_fields_0_results.md). **Next:** `STUDIO-STAR-NAMING-PASS-0`. | DA-reserve · Std |
| 11.3 | `STUDIO-STAR-NAMING-PASS-0` | Galaxy generation runs the **star-naming pass** so every star system carries a display name; the committed TP base-disc is regenerated/repaired so `simthing_spec::star_system_display_name` resolves for all systems. Generator + data; deterministic naming (seed-stable). | **DA-GRADUATED / merged [#1304](https://github.com/khorum08/SimThing/pull/1304) @ `052cc192`** — isolated domain-separated naming; emitter non-blank names; 1,500-name canonical golden; structure and 11.1/11.2 held; 9 proofs; evidence [studio_star_naming_pass_0_results.md](tests/studio_star_naming_pass_0_results.md). **Next:** `STUDIO-CLAUSE-LOADER-SIMPLIFY-0`. | DA-reserve · **Frontier** |

### Tier B — presentation (`studio-live-ops-ui-clock`-clearable unless noted)

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 11.4 | `STUDIO-CLAUSE-LOADER-SIMPLIFY-0` | **Needs 11.1.** Scenario Library shows **only the ClauseScript loader**; the resolver/source field is removed from the load panel and **auto-populated off-screen** (sibling-file / canonical convention). The resolved source path + resolver state move to a **new "Scenario" dropdown in the Telemetry window**, alongside scenario telemetry (scenario id, star/owner counts, STEAD status, tick index, paused). Read-only; no Spec mutation. Wire production ingest `source_base`. | **DA-GRADUATED / merged [#1306](https://github.com/khorum08/SimThing/pull/1306) @ `786c6c0b`** — ClauseScript-only loader; empty operator resolver + clause-dir `source_base`; read-only Scenario telemetry; bridge reset + modal pause retained; 10 proofs; evidence [studio_clause_loader_simplify_0_results.md](tests/studio_clause_loader_simplify_0_results.md). **Next:** `STUDIO-FACTION-NAMEPLATES-0`. | Tier-2 · Std |
| 11.5 | `STUDIO-FACTION-NAMEPLATES-0` | **Needs 11.2 + 11.3.** Star + planet nameplates render the **star/system name** (from 11.3) in the **owning faction's `color_rgb`** (from 11.2); unowned = neutral. Presentation only over existing `GalaxyStarNameplate` path. | **PROBATION** [#1309](https://github.com/khorum08/SimThing/pull/1309) @ `f0b92b93` — faction-colored star nameplates from owner color_rgb; evidence [studio_faction_nameplates_0_results.md](tests/studio_faction_nameplates_0_results.md). Not graduated. | Tier-2 · Std |
| 11.6 | `STUDIO-OWNED-STAR-SELECT-BRIGHTEN-0` | **Needs 11.2.** Selecting a faction-owned star **brightens all stars that faction owns** to reflect the owned/selected set; deselect restores. Presentation-only render state; no Spec mutation, no selection-model authority. | NOT STARTED | Tier-2 · Std |
| 11.7 | `STUDIO-FROSTED-GLASS-0` | Window backgrounds get a real **frosted-glass** effect: slight darkening tint + backdrop **blur** of content behind the panel. **Performance is a hard exit criterion** — frame-budget-bound (e.g. one downsampled/low-radius separable blur target, not a full-res per-frame gaussian); record before/after frame-time. May add a presentation `*.wgsl` blur pass → **DA-reserve** (benign presentation shader, not kernel authority). | NOT STARTED | Tier-2 / DA-reserve if wgsl · **Frontier** |

**Dependency order:** 11.1 → (11.2 ∥ 11.3) → (11.4 needs 11.1 ; 11.5 needs 11.2+11.3 ; 11.6 needs 11.2) ; 11.7 independent. Tier-A rungs reserve to DA by envelope; Tier-B UI rungs are orchestrator-mergeable once sticky is `ORCHESTRATOR-CLEARABLE` (class envelope admits mapeditor presentation + ingest wire; WGSL on 11.7 still DA-reserve).

---

## 5. Explicit non-goals

- Reopening 0.0.8.5 Terran-Pirate (CLOSED 2026-07-09, #1256; consume its landed hydration, never re-derive)  
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
| Active open rung | `STUDIO-FACTION-NAMEPLATES-0` (Phase 11; 11.1–11.4 graduated) |
| Debug baseline | `cargo build -p simthing-mapeditor --bin simthing-studio` |
| Clause load baseline | Canonical `scenarios/terran_pirate_galaxy.clause` via production ingest `hydrate_scenario_with_source_base` (clause parent dir) |

**Park instruction for agents:** Phase 9 complete; Phase 10 parked. Phase 11 active at `STUDIO-FACTION-NAMEPLATES-0` (11.5, Std). Do not reopen 0.0.8.5.
