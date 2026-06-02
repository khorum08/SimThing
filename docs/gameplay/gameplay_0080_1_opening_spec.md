# GAMEPLAY-0080-1 — Nested Starmap Read-Only Observation Opening Spec

> **Status: IMPLEMENTED / PASS - read-only Nested Starmap observation/export.**
> - `SCENARIO-0080-1` (Nested Starmap) is **ACCEPTED**.
> - `ATLAS-0080-0` is **IMPLEMENTED / PASS** (`run_atlas_0080_0`).
> - `ECON-SCALE-0080-0` is **IMPLEMENTED / PASS** (`run_econ_scale_0080_0`).
> - `PRODUCTION-PATH-0080-1` is **IMPLEMENTED / PASS** (`run_production_path_0080_1`).
> - `DEFAULT-SCHEDULE-0080-1` is **IMPLEMENTED / PASS** (`run_default_schedule_0080_1`).
> - `GAMEPLAY-0080-1` is implemented only as a read-only observation/export gate.
> - Implementation result: [`../tests/phase_gameplay_0080_1_impl_results.md`](../tests/phase_gameplay_0080_1_impl_results.md).
>
> Verdict: **OPEN WITH NARROWING (Option A)** — read-only, opt-in/default-off, scenario-scoped,
> non-interactive observation over the schedule run report. **Observation only — not control.**

---

## 0. Naming caution (design-authority note)

`GAMEPLAY-0080-1` authorizes **read-only observation** of the Nested Starmap run, exactly as
`GAMEPLAY-0080-0` did for Local Patrol Economy. The "gameplay" name does **not** authorize control,
command input, player command loop, UI, or a real-time loop — those remain a **separate, CLOSED** concern
(`CONTROL-0080-1` is a later, separate gate). Reading "gameplay" as license for control input is out of
scope and a stop-and-escalate. This gate is an *observer*, not a *controller*.

---

## 1. Why observation now unlocks product visibility

Nested Starmap now has **live SEAD-sourced movement** (`DEFAULT-SCHEDULE-0080-1`). Its run report
(`DefaultSchedule0081RunReport`) is not yet product-readable enough for designer/user inspection. A
read-only observation/export surface makes the nested theater, faction ECON, ownership overlays, SEAD
decisions, and ship movement **legible without changing simulation state**. This is the first
product-facing consumer of the `0080-1` stack — **not** a control gate, **not** a UI framework, and
**not** a passive proof wrapper.

---

## 2. Narrowed observation definition

`GAMEPLAY-0080-1` is defined as:
- `SCENARIO-0080-1` only; explicit **opt-in / default-off**; **read-only**; deterministic;
- consumes `DEFAULT-SCHEDULE-0080-1` reports (`run_default_schedule_0080_1` → `DefaultSchedule0081RunReport`);
- exports a stable transcript/summary;
- **does not mutate simulation state** except by invoking the existing explicit opt-in schedule path it
  observes;
- **no** control; **no** command input; **no** demo packaging; **no** UI; **no** real-time loop; **no**
  global default schedule.

---

## 3. Allowed observation content

The future implementation may expose: scenario id + name; starmap shape; active/resident theaters;
Terran/Pirate fixed faction set; pirate full-economy participation; contended ECON summary; owner-overlay
inheritance summary; ownership up-aggregation summary; step index; mover id; mover faction; start
starsystem/theater; end starsystem/theater; threshold-accepted flag; event-emitted flag;
`BoundaryRequest`-materialized flag; identity-preserved flag; owner-overlay-preserved flag;
membership-updated-without-reparenting flag; replay checksum; guardrail flags confirming no
direct-movement/control/UI/realtime/global-schedule.

---

## 4. Forbidden content

The future implementation **must not** add: control; command input; player command loop; UI framework;
real-time loop; global default schedule; direct movement commands; externally-scripted `BoundaryRequest`s;
CPU planner / urgency / commitment; semantic/raw WGSL; new shader/GPU kernel; hard currency;
markets/trade/`ai_budget`; nested Resource Flow; unbounded factions; owner-entity as spatial parent;
capture-as-reparenting; ClauseThing implementation; `simthing-spec` alteration for ClauseThing; invariant
edit; passive proof wrapper; general gameplay framework.

---

## 5. Future implementation slice (if opened)

The next handoff **may**: add a narrow `gameplay_0080_1` module; consume `DefaultSchedule0081RunReport`;
optionally call `run_default_schedule_0080_1` through explicit opt-in to obtain the report; produce a
stable Rust observation report + deterministic text export; include a movement transcript table/section;
preserve all schedule/production/atlas/econ regression tests; update docs/report.

It **must not**: mutate simulation state beyond invoking the existing opt-in schedule; add command/control
APIs; add UI or a real-time loop; add demo packaging; add a global schedule.

**Read-side discipline.** The observer is pure Rust read/orchestration over an existing report; it touches
no shader text and no GPU kernel (WGSL ban, invariants row 169/194) and adds no decision logic — SEAD
remains the sole mover-decision source. Any need for new shader/kernel here is a stop-and-escalate.

---

## 6. Future required tests (named, not implemented)

- `gameplay_0080_1_readonly_observation_explicit_opt_in_only`
- `gameplay_0080_1_consumes_default_schedule_report`
- `gameplay_0080_1_exports_nested_starmap_transcript`
- `gameplay_0080_1_includes_atlas_residency_summary`
- `gameplay_0080_1_includes_faction_index_econ_summary`
- `gameplay_0080_1_includes_owner_overlay_and_up_aggregation_summary`
- `gameplay_0080_1_includes_sead_movement_trace`
- `gameplay_0080_1_includes_terran_and_pirate_movement_rows`
- `gameplay_0080_1_replay_transcript_deterministic`
- `gameplay_0080_1_no_control_or_command_input`
- `gameplay_0080_1_no_demo_packaging`
- `gameplay_0080_1_no_ui_framework`
- `gameplay_0080_1_no_realtime_loop`
- `gameplay_0080_1_no_global_default_schedule`
- `gameplay_0080_1_no_direct_movement_or_external_boundary_request`
- `gameplay_0080_1_no_cpu_planner_or_commitment`
- `gameplay_0080_1_no_semantic_or_raw_wgsl`
- `gameplay_0080_1_no_new_shader_or_gpu_kernel`
- `gameplay_0080_1_no_hard_currency_markets_trade_aibudget`
- `gameplay_0080_1_no_nested_resource_flow`
- `gameplay_0080_1_no_clausething_dependency`
- `gameplay_0080_1_docs_status_matches_gate`

---

## 7. Stop conditions

Stop if this gate would require: control/command input; demo packaging; player command loop; UI framework;
real-time loop; global default schedule; direct movement command; externally-scripted `BoundaryRequest`;
CPU planner / urgency / commitment; semantic/raw WGSL; new shader/GPU kernel; hard currency;
markets/trade/`ai_budget`; nested Resource Flow; unbounded factions; owner-entity as spatial parent;
capture-as-reparenting; ClauseThing implementation; `simthing-spec` alteration for ClauseThing; invariant
edit; passive proof wrapper; or a general gameplay framework.

---

## 8. Exit criteria (this opening PR)

- [x] Design authority chose **Option A** (open with narrowing).
- [x] Opening spec exists; `GAMEPLAY-0080-1` marked OPEN WITH NARROWING (read-only observation only).
- [x] Scope is `SCENARIO-0080-1` only; read-only consumer of `DEFAULT-SCHEDULE-0080-1` reports.
- [x] Future implementation slice named, not implemented.
- [x] `DEFAULT-SCHEDULE-0080-1` and all prior `0080-1` gates remain IMPLEMENTED / PASS.
- [x] Control/demo for `0080-1` remain not opened.
- [x] Mapping guidance + worklog updated.
- [x] No code changed.

---

## 9. Pointers
- Scenario: [`../scenarios/scenario_0080_1_admission_packet.md`](../scenarios/scenario_0080_1_admission_packet.md)
- Schedule spec / impl: [`../production_paths/default_schedule_0080_1_opening_spec.md`](../production_paths/default_schedule_0080_1_opening_spec.md), [`../tests/phase_default_schedule_0080_1_impl_results.md`](../tests/phase_default_schedule_0080_1_impl_results.md)
- Production path / atlas / econ-scale: [`../production_paths/production_path_0080_1_opening_spec.md`](../production_paths/production_path_0080_1_opening_spec.md), [`../production_paths/atlas_0080_0_opening_spec.md`](../production_paths/atlas_0080_0_opening_spec.md), [`../production_paths/econ_scale_0080_0_opening_spec.md`](../production_paths/econ_scale_0080_0_opening_spec.md)
- Prior read-only observation precedent: [`gameplay_0080_0_opening_spec.md`](gameplay_0080_0_opening_spec.md)
- Production track + PR ladder: [`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md)
- Binding rules: [`../invariants.md`](../invariants.md)
- Visibility report: [`../tests/phase_gameplay_0080_1_opening_review_results.md`](../tests/phase_gameplay_0080_1_opening_review_results.md)
