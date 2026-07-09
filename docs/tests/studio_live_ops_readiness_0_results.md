# STUDIO-LIVE-OPS-READINESS-0 Results

## Status

**PROBATION / proof-present** — readiness map only. No production clock, transport UI, live bridge, library modal, scenario create, clearance class, or hardening in this PR.

## Identity

| Field | Value |
|---|---|
| Rung | `STUDIO-LIVE-OPS-READINESS-0` |
| Track | `0.0.8.6-studio-live-ops` (`docs/design_0_0_8_6_studio_live_ops.md`) |
| Kind | readiness / docs-report (repo inspection) |
| Canonical design | `docs/design_0_0_8_6_studio_live_ops.md` §4 rung 9.0 |
| Inspected HEAD (pre-PR) | `1a3f6b86820838b7c5026f4686fe79d086310de9` |

## Inspection spine (kept tight)

| Surface | Why |
|---|---|
| `crates/simthing-mapeditor/src/app/mod.rs` | Bevy `Update` schedule = current Studio “tick” |
| `crates/simthing-mapeditor/src/session.rs` | `StudioSession` authority + summary identity |
| `crates/simthing-mapeditor/src/app/ui.rs` + `app/scenario_io.rs` | Scenario/clause I/O presentation; no transport clock |
| `crates/simthing-mapeditor/src/scenario_runtime_saveload_ui.rs` | Driver compile/report presentation; non-authority flags |
| `crates/simthing-driver/src/session.rs` | `SimSession` open/run authority for admitted ticks |
| `crates/simthing-workshop/src/live_run_post_hydration.rs` + `docs/tests/tp_live_run_0_results.md` | Prior live-run residue (workshop-only) |
| Prior Phase 8 readiness | `docs/tests/tp_studio_ingest_readiness_0_results.md`, `tp_studio_stead_rebind_readiness_0_results.md` |

No additional crates were required beyond this set for the eight mission questions.

---

## 1. Current Studio tick / update path

**There is no Studio sim clock today.** The only continuous “tick” is the Bevy frame loop.

Observed schedule in `crates/simthing-mapeditor/src/app/mod.rs`:

```text
Startup / PostStartup
  → scene, window, telemetry, typeface shell
EguiPrimaryContextPass
  → ui::studio_ui_system          (egui panels / dialogs)
Update (chained)
  → begin_main_update_timing
  → panel opacity, camera, AA, falloff, picking, star/nameplate sync,
    FPS/VRAM/window GPU telemetry
  → finalize_main_update_timing
PostUpdate
  → hyperlane color sync
```

What advances each frame:

- Camera / selection / render projection sync (presentation).
- egui UI (presentation).
- Performance telemetry (`last_update_frame`, FPS, VRAM) — **render frame counters**, not sim ticks.

What does **not** advance:

- No `SimSession::run` / hot-cycle call from Studio.
- No pause/play/rate/TPS state on `StudioAppState` or `StudioSession`.
- Loaded `StudioSession` is structural/view hydrate only until a later bridge rung.

---

## 2. Where render/UI ends and SimSession / driver authority begins

| Layer | Owner | Authority? |
|---|---|---|
| Bevy ECS entities, camera, materials, egui, FPS/VRAM | `app/*`, render helpers | **No** — presentation |
| `StudioAppState` (paths, dialogs, dirty flags, status strings) | `app/mod.rs` | **No** — presentation |
| `StudioSession.scenario_authority: SimThingScenarioSpec` | `session.rs` | **Yes** — model authority |
| Projections / view_model / GPU residency readiness | derived from Spec | **No** — rebuild from Spec |
| Driver compile/report plans (runtime save-load status) | `scenario_runtime_saveload_ui.rs` via `simthing_driver` / `simthing_spec` | **Reports only**; flags declare UI/Bevy/GPU non-authority |
| `SimSession` (`simthing-driver`) | `session.rs` `open` / `open_from_spec` / `run` | **Yes** — admitted sim execution |

Boundary today:

```text
ScenarioSpec (StudioSession.scenario_authority)
  → Studio hydrate / structural projection / Bevy scene   [presentation]
  → (optional) driver compile plans for readiness status  [presentation reports]
  ✗ no live SimSession held by StudioAppState
```

`scenario_runtime_saveload_ui.rs` already encodes the non-authority contract (`ui_state_is_authority: false`, `bevy_state_is_authority: false`, …). Live-ops must keep that law: clock UI schedules admitted driver ticks; it never mutates Spec from Bevy.

---

## 3. What should own sim clock state (pause / play / rate / TPS)

**Recommended owner:** a dedicated Studio-side **sim clock substrate** object (rung 9.1), held beside — not inside — Bevy presentation state, and bound to an optional live driver/session handle.

| Candidate | Verdict |
|---|---|
| Bevy `Time` / frame delta | **Reject** — would make render rate authoritative |
| egui widget local state | **Reject** — presentation only; cannot be the freeze source of truth |
| `StudioSession` / ScenarioSpec fields | **Reject** — Spec is model authority, not transport schedule |
| New `StudioSimClock` (or equivalent) in mapeditor, with programmatic API | **Accept** — owns `paused`, `rate` (1×/2×/4×), `max_tps`, `tick_index` / scheduled-tick counter; UI (9.2) only drives it |
| `SimSession` itself | Owns **execution** of admitted ticks when invoked; does not own operator transport UX |

Binding: clock schedules N admitted ticks per wall interval under TPS cap; each tick invokes the admitted driver/session path (9.3), never a Bevy-side planner.

Testability: substrate unit/integration tests can advance a fake clock + count scheduled ticks **without** opening a Bevy window or GPU adapter.

---

## 4. Exact pause semantics (including modal scenario-library pause)

**Today:** no sim pause exists. Closest presentation analogues:

- `performance_diagnostic_freeze_camera` — freezes **camera input**, not sim.
- `generation_name_dialog_visible` — modal for generation naming only; does not pause a sim clock (none exists).
- Left-panel New/Load/Save buttons are still inactive stubs; real I/O lives under “Scenario / runtime save-load” + ClauseScript picker (non-modal panel section).

**Track law (design §2.2) — required semantics for 9.1/9.5:**

| Condition | Required behavior |
|---|---|
| Scenario library window **visible / modal** | Sim clock **paused** (no scheduled ticks) |
| Operator closes library | Remains **paused**; does **not** auto-Play |
| Operator presses Play | Resumes scheduling under current rate/TPS |
| Pause button | Freezes scheduling; tick index held |
| “Pause visually but tick underneath” | **Forbidden** |

Library I/O must reuse production JSON load/save + clause ingest/picker (`clause_scenario_ingest` / `clause_scenario_picker`) with **explicit** resolver entries — no silent TP fixture defaults (picker module already documents this).

---

## 5. Observation surfaces already present

| Concern | Existing surface | Live-ops gap |
|---|---|---|
| Scenario identity | `StudioSession.galaxy_name()` / `scenario_authority.scenario_id`; `StudioScenarioSummary.scenario_id` | Present |
| Structural summary | `scenario_summary` (system/link counts, STEAD/links valid, `rf_ready`, heatmap readiness) | Present; static until re-hydrate |
| Status strings | `status_message`, `last_scenario_io_status`, clause/runtime messages | Present; not tick-indexed |
| Authority digest / runtime readiness | `StudioScenarioRuntimeSaveLoadStatus` (`loaded_scenario_digest`, STEAD/RF/report-chain flags) | Present as **compile/report** snapshot, not live tick |
| Frame / GPU telemetry | `StudioPerformanceTelemetry` (`last_update_frame`, FPS, VRAM, main_update_ms) | **Render** observation only |
| Sim tick index | — | **Missing** |
| Pause / rate / effective TPS readout | — | **Missing** |
| Pause-freeze verification of sim state | — | **Missing** (needs clock + bridge) |

Rung 9.4 should wire readouts to the clock substrate (+ optional tree-local summaries already available), not invent a CPU planner.

---

## 6. Is GameMode / RF attach still a blocker for “live” Studio?

**No — not a blocker for 9.1–9.3 structural / live-session ticks.**

Evidence:

- Phase 8 already hydrates StructuralRebindReady Spec into `StudioSession` without GameMode/RF attach (picker/API results explicitly ban GameMode/RF/live-run in that scope).
- Prior readiness (`tp_studio_stead_rebind_readiness_0_results.md`) separates: STEAD grid required for Studio inspect; `GameModeSpec` / RF columns required for **driver RF live-run**, not for mapeditor hydrate.
- Workshop `TP-LIVE-RUN-0` proves multi-tick `SimSession::open_from_spec` + RF/STEAD in **workshop theater** — residue for production bridge policy, not a requirement to attach full TP GameMode before Studio clock exists.
- Driver also exposes `SimSession::open(scenario)` (no GameMode) for non-RF session shells; RF/mapping attach is opt-in via `open_from_spec` + authored profile.

**Implication for 9.1–9.3:**

| Rung | Can proceed without new GameMode/RF gameplay systems? |
|---|---|
| 9.1 clock substrate | **Yes** — schedule/pause/rate/TPS only |
| 9.2 transport UI | **Yes** — drives clock |
| 9.3 live session bridge | **Yes** for structural / session-identity multi-tick under Play, elevating workshop live-run **policy** only as needed; full TP RF combat theater remains optional/bounded, not a new gameplay subsystem |

GameMode/RF attach remains a **later product depth** choice (bounded theater vs structural ticks), not a gate that blocks clock admission.

---

## 7. Files each follow-on rung should touch

### 9.1 `STUDIO-SIM-CLOCK-0`

| Likely touch | Role |
|---|---|
| `crates/simthing-mapeditor/src/` new module (e.g. `studio_sim_clock.rs`) | Clock substrate: pause/play/rate/TPS, tick counter, schedule API |
| `crates/simthing-mapeditor/src/lib.rs` | Export |
| `crates/simthing-mapeditor/tests/` new focused test | Pause freeze, rate ratios, TPS cap (headless) |
| `docs/tests/studio_sim_clock_0_results.md` | Evidence |
| `scripts/ci/test_inventory.tsv` | Inventory row (`birth_track=0.0.8.6-studio-live-ops`) |

Avoid: Bevy systems as authority; Spec mutation; workshop GameMode attach.

### 9.2 `STUDIO-SIM-CLOCK-UI-0`

| Likely touch | Role |
|---|---|
| `crates/simthing-mapeditor/src/app/ui.rs` | Transport controls + readout |
| `crates/simthing-mapeditor/src/app/mod.rs` | Hold clock resource / wire Update to ask clock for scheduled ticks |
| `crates/simthing-mapeditor/src/app/scenario_io.rs` or thin UI helper | Programmatic hooks for CI (press Play/Pause without GPU) |
| Focused UI/programmatic test + results doc | |

### 9.3 `STUDIO-LIVE-SESSION-BRIDGE-0`

| Likely touch | Role |
|---|---|
| New bridge module under mapeditor (or thin adapter) | Optional live handle: Spec → admitted driver/session tick path |
| `app/mod.rs` | On Play, invoke bridge N times per clock schedule |
| Workshop `live_run_post_hydration.rs` | **Read-only residue** — elevate policy only if bounded theater required; do not move TP combat into Studio by default |
| Focused multi-tick identity test (prefer headless / skip-without-adapter) | |

### 9.5 `STUDIO-SCENARIO-LIBRARY-UI-0`

| Likely touch | Role |
|---|---|
| `app/ui.rs` + possibly new `scenario_library_ui.rs` | Modal/toggled library window |
| `app/scenario_io.rs`, `clause_scenario_picker.rs`, `clause_scenario_ingest.rs` | Reuse only — load/save/open clause |
| Clock substrate | Modal visible ⇒ `paused = true`; close ⇒ stay paused |
| Focused tests: modal⇒paused; I/O via production APIs; no TP defaults | |

---

## 8. Tests each follow-on rung should add

| Rung | Load-bearing tests (catches) |
|---|---|
| **9.1** | `pause_freezes_tick_index` — Pause stops scheduled advances; `rate_2x_4x_ratios` — scheduled ticks vs 1× within tolerance; `max_tps_cap_holds` — no unbounded tick storm; optionally `clock_does_not_mutate_scenario_spec` |
| **9.2** | `ui_or_hook_sets_paused_play_rate_tps` — programmatic hooks drive clock; `readout_exposes_tick_rate_paused` — observation fields present without requiring desktop GPU |
| **9.3** | `loaded_session_multi_tick_under_play` — ≥N ticks with session/STEAD identity held; `pause_freezes_live_state` — no tick while paused; GPU legs `skip` without adapter |
| **9.5** | `library_visible_forces_pause`; `library_close_does_not_autoplay`; `load_save_clause_via_production_api_only` — no fixture-default resolver; `no_tick_while_modal_open` |

Do **not** battery-test type-system impossibilities. Prefer headless clock/bridge tests; cite owner-local desktop proof only when a later rung truly needs Bevy interaction.

---

## Falsification (bad paths rejected)

| Bad path | Falsified by |
|---|---|
| Hidden Bevy-authoritative sim path | Current Update loop is camera/UI/telemetry only; recommendation puts schedule ownership in `StudioSimClock` + driver ticks, not Bevy Time |
| New gameplay system required for 9.1 | 9.1 is pause/rate/TPS substrate only; GameMode/RF not required |
| Silent TP default in library/load path | Reuse `clause_scenario_ingest` / picker with explicit resolver; picker already forbids TP defaults |
| “Pause visually but tick underneath” | Modal/pause must gate **scheduling** on the clock substrate (design §2.2) |
| Clock owner untestable without desktop GPU | Substrate API is headless-testable; Bevy UI is 9.2 presentation |

---

## Recommendation

```text
READY_FOR_9_1: YES
production_clock_admission_needed: NO
library_ui_admission_needed: NO
gamemode_rf_blocker: NO
recommended_next_rung: STUDIO-SIM-CLOCK-0
```

Rationale: design already admits clock + library as the Phase 9 ladder after this readiness report; no separate Owner/DA production-admission stamp is required to start 9.1. Library UI waits for 9.5 (after clock exists so modal⇒pause is enforceable). GameMode/RF is not a 9.1 blocker.

## Non-goals confirmed

- No STUDIO-SIM-CLOCK-0 implementation in this PR  
- No transport UI, live bridge, library modal, create templates, clearance class, hardening  
- No runtime/GPU/kernel edits; no Bevy-as-authority; no CPU planner  
- 0.0.8.5 / TP closeout not re-opened  

## Local proof (this PR)

```text
ORIENT-RECEIPT: 258b76525e5c
cargo check -p simthing-mapeditor
bash scripts/ci/agent_scan.sh
bash scripts/ci/gen_orientation.sh --check
bash scripts/ci/test_inventory_drift_check.sh
```

(Commands executed on the PR branch; outputs recorded in the PR body.)
