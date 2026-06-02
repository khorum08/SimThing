# GAMEPLAY-0080-0 — Local Patrol Economy Read-Only Observation Opening Spec

> **Status: OPENING SPEC / NO IMPLEMENTATION.**
> - `SCENARIO-0080-0` (Local Patrol Economy) is **ACCEPTED**.
> - `PRODUCTION-PATH-0080-0` is **IMPLEMENTED / PASS** (`run_production_path_0080_0`).
> - `DEFAULT-SCHEDULE-0080-0` is **IMPLEMENTED / PASS** (1A schedule + patrol, 1B bounded pirate loop;
>   deterministic cat-and-mouse).
> - `GAMEPLAY-0080-0` is **OPEN only as a narrowed read-only observation gate** (docs/design).
> - **This PR does not implement the observation surface.** A separate authorized PR may implement the
>   named slice.
>
> Verdict: **OPEN WITH NARROWING (Option A)** — read-only, scenario-scoped, opt-in, reversible,
> non-interactive. **Not gameplay-as-control.**

---

## 0. Naming caution (design-authority note)

The ladder id is historically `GAMEPLAY-0080-0`, but this gate authorizes **only read-only observation**
of the already-implemented Local Patrol Economy schedule. The word "gameplay" in the gate name **does
not** authorize player commands, control inputs, a UI framework, or a real-time loop — those remain a
**separate, CLOSED** concern. Any future PR that reads "gameplay" as license to add control input is
**out of scope and must stop-and-escalate.** This gate is an *observer*, not a *controller*.

---

## 1. Why observation is now a product need

Local Patrol Economy is no longer just substrate proof. `DEFAULT-SCHEDULE-0080-0` now produces
**deterministic, scheduled** patrol/pirate/economy behavior, including an emergent cat-and-mouse pattern,
all captured in `DefaultSchedule0080RunReport`. Today that behavior is only legible by reading raw test
structs. A product-facing observer lets a designer/user **inspect the simulation's emergent behavior**
in a stable, product-readable form.

This is **execution-unlocking product visibility** — the first product-facing *consumer* of the
0.0.8.0 Local Patrol Economy stack — **not** another passive proof wrapper. It renders an existing,
deterministic report; it adds no proof, no soak, no accounting variant, and no new simulation behavior.

---

## 2. Scope

- Local Patrol Economy only.
- **Read-only** consumer of `DefaultSchedule0080RunReport` (and its `step_reports`).
- One owner; one or a few patrols; one bounded pirate; two or a few local locations — exactly as the
  schedule already produces them.
- The observer **does not run the simulation** other than by invoking the existing opt-in schedule
  (`run_default_schedule_0080_0` / `replay_default_schedule_0080_0`) it observes; it never mutates
  scenario state itself.

---

## 3. Narrowed gameplay definition

`GAMEPLAY-0080-0` is defined as:
- **read-only observation** of Local Patrol Economy;
- deterministic scenario **transcript / export / report**;
- **no player command input**;
- **no interactive UI framework**;
- **no real-time / wall-clock loop**;
- **no gameplay scheduler** (it consumes the existing opt-in schedule; it registers none);
- **no new simulation behavior**;
- **no default-on path** (opt-in only).

---

## 4. Allowed observation content

The future implementation may expose (all already present, deterministic, in the schedule report):

- tick / step index;
- source / destination location summaries;
- patrol location / membership / economy participation;
- pirate location;
- `supply`; `maintenance`; `local_output`; `local_security`; `disruption`;
- threshold accepted / event emitted / boundary request materialized;
- production path invoked;
- patrol relocation;
- pirate relocation;
- pirate supply drain;
- pirate disruption added;
- target score terms (`supply` / `disruption` / `local_security` evasion term);
- deterministic replay checksum (`deterministic_replay_checksum`);
- cat-and-mouse observed flag (`cat_and_mouse_pattern_observed`).

---

## 5. Forbidden content

The future implementation **must not** add:
- player controls;
- designer-authored move requests;
- CPU planner / urgency / commitment emission;
- real-time game loop;
- global default schedule;
- semantic / raw WGSL;
- new shader or GPU kernel;
- hard currency;
- markets / trade / `ai_budget`;
- nested Resource Flow;
- multi-faction economy;
- ClauseThing implementation;
- `simthing-spec` alteration for ClauseThing;
- invariant edits;
- new substrate;
- closed-ladder reopen.

---

## 6. Future implementation slice (if opened)

The next handoff **may**:
- add a **read-only** observation / export module for Local Patrol Economy;
- consume `DefaultSchedule0080RunReport`;
- produce a stable, product-readable **transcript or summary** (text/structured export);
- include deterministic **golden / text snapshot tests** if useful;
- preserve all existing schedule and production-path tests;
- update docs and report results.

It **must not**:
- mutate simulation state beyond calling the existing opt-in schedule;
- add a UI framework;
- add command inputs;
- add a global schedule.

**Read-side discipline.** The observer is pure Rust read-side over an existing report; it touches no
shader text and no GPU kernel, so the WGSL ban (invariants row 169/194) is satisfied trivially — any
need for new shader/kernel here is a stop-and-escalate, not an implementation choice.

---

## 7. Future required tests (named, not implemented)

- `gameplay_0080_0_readonly_observation_explicit_opt_in_only`
- `gameplay_0080_0_consumes_default_schedule_report`
- `gameplay_0080_0_exports_tick_transcript`
- `gameplay_0080_0_includes_patrol_pirate_economy_state`
- `gameplay_0080_0_includes_threshold_event_boundary_trace`
- `gameplay_0080_0_includes_cat_and_mouse_summary`
- `gameplay_0080_0_replay_transcript_deterministic`
- `gameplay_0080_0_no_player_commands`
- `gameplay_0080_0_no_realtime_loop`
- `gameplay_0080_0_no_global_default_schedule`
- `gameplay_0080_0_no_semantic_or_raw_wgsl`
- `gameplay_0080_0_no_cpu_planner_or_external_move_script`
- `gameplay_0080_0_no_hard_currency_markets_trade_aibudget`
- `gameplay_0080_0_no_clausething_dependency`
- `gameplay_0080_0_docs_status_matches_gate`

---

## 8. Stop conditions

Stop (do not implement under this gate) if it would require: player controls; gameplay UI framework;
real-time loop; global default schedule; semantic / raw WGSL; new shader / GPU kernel; CPU planner /
urgency / commitment emission; externally-scripted movement; hard currency; markets / trade /
`ai_budget`; nested Resource Flow; multi-faction economy; ClauseThing implementation; `simthing-spec`
alteration for ClauseThing; invariant edits; passive proof wrappers; a general scheduler / runtime; or
reopening any closed ladder.

---

## 9. Exit criteria (this opening PR)

- [x] Design authority chose **Option A** (open with narrowing).
- [x] Opening spec exists; `GAMEPLAY-0080-0` marked OPEN WITH NARROWING (read-only observation only).
- [x] Scope is Local Patrol Economy only; read-only consumer of `DefaultSchedule0080RunReport`.
- [x] Future implementation slice is named but not implemented.
- [x] `PRODUCTION-PATH-0080-0` and `DEFAULT-SCHEDULE-0080-0` remain IMPLEMENTED / PASS.
- [x] Player control, semantic WGSL, and global default schedule remain closed.
- [x] Mapping guidance + worklog updated.
- [x] No code changed.

---

## 10. Pointers
- Active constitution: [`../design_0_0_8_0.md`](../design_0_0_8_0.md)
- Production track: [`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md)
- Scenario packet: [`../scenarios/scenario_0080_0_admission_packet.md`](../scenarios/scenario_0080_0_admission_packet.md)
- Production path spec: [`../production_paths/production_path_0080_0_opening_spec.md`](../production_paths/production_path_0080_0_opening_spec.md)
- Schedule spec: [`../production_paths/default_schedule_0080_0_opening_spec.md`](../production_paths/default_schedule_0080_0_opening_spec.md)
- Schedule impl reports: [`../tests/phase_default_schedule_0080_0_impl_1a_results.md`](../tests/phase_default_schedule_0080_0_impl_1a_results.md), [`../tests/phase_default_schedule_0080_0_impl_1b_results.md`](../tests/phase_default_schedule_0080_0_impl_1b_results.md)
- Binding rules: [`../invariants.md`](../invariants.md)
- Visibility report: [`../tests/phase_gameplay_0080_0_opening_review_results.md`](../tests/phase_gameplay_0080_0_opening_review_results.md)
