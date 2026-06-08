# CONTROL-0080-0 — Local Patrol Economy Bounded Command Admission Opening Spec

> **Status: IMPLEMENTED / PASS — bounded command admission (`admit_control_0080_0`).**
> - `SCENARIO-0080-0` (Local Patrol Economy) is **ACCEPTED**.
> - `PRODUCTION-PATH-0080-0` is **IMPLEMENTED / PASS** (`run_production_path_0080_0`).
> - `DEFAULT-SCHEDULE-0080-0` is **IMPLEMENTED / PASS** (1A schedule + patrol, 1B bounded pirate loop).
> - `GAMEPLAY-0080-0` is **IMPLEMENTED / PASS** — read-only observation export (`observe_gameplay_0080_0`).
> - `CONTROL-0080-0` is **IMPLEMENTED / PASS** — bounded command admission.
> - **Direct movement control / player command loop / UI framework / real-time loop remain CLOSED.**
>
> Implementation report: [`../tests/phase_control_0080_0_impl_results.md`](../tests/phase_control_0080_0_impl_results.md).

---

## 0. Naming caution (design-authority note)

`CONTROL-0080-0` authorizes **bounded command *admission*** — a designer may select from a tiny validated
vocabulary that writes only **already-accepted scenario input/config values** (the existing
`DefaultSchedule0080Input` fields) before the existing schedule runs. It does **not** authorize direct
control of movers, a player command bus, or free-form scripting. A command never moves a patrol or
pirate, never emits a `BoundaryRequest`, and never bypasses FIELD_POLICY: movement still **emerges** from the
already-accepted GPU-resident `Threshold`+`EmitEvent`→`BoundaryRequest` path. Reading "control" as license
for direct movement or a gameplay loop is **out of scope and a stop-and-escalate.**

---

## 1. Why bounded command admission may now be product-relevant

The scenario now runs deterministically (`DEFAULT-SCHEDULE-0080-0`) and is legible in product-readable
form (`GAMEPLAY-0080-0`). The next product question is narrow: may a designer/user **select from a tiny
admitted command vocabulary** to set up a scenario configuration and then observe the deterministic
result? The vocabulary steers only **accepted scenario parameters / admitted FIELD_POLICY threshold inputs** —
never direct movement mutation. This is **not** a CPU planner, not gameplay AI, not free-form scripting,
and not ClauseThing.

---

## 2. Narrowed control definition

`CONTROL-0080-0` is defined as:
- **bounded command/admission only**;
- Local Patrol Economy only;
- **opt-in / default-off**;
- **deterministic**;
- read/write **only** through admitted scenario configuration or already-accepted FIELD_POLICY threshold inputs;
- **no direct entity relocation command**;
- **no externally-scripted move request**;
- **no free-form player command bus**;
- **no UI framework**;
- **no real-time loop**;
- **no global default schedule**.

---

## 3. Possible minimal command vocabulary (named, not implemented)

Each targets an existing `DefaultSchedule0080Input` bounded value/config field:

- `set_source_disruption(value)`
- `set_destination_disruption(value)`
- `set_source_supply(value)`
- `set_destination_supply(value)`
- `set_source_local_security(value)`
- `set_destination_local_security(value)`
- `set_step_count(value)`
- `set_patrol_disruption_reduction(value)`
- `run_observed_scenario`
- `export_transcript`

Each command must: be deterministic; be admitted through explicit validation; target existing bounded
values only; produce a reproducible observation export; and **never directly issue a move request**.

---

## 4. Forbidden commands / stop conditions

Reject any command that would: move patrol directly; move pirate directly; create an externally-scripted
`BoundaryRequest`; bypass threshold/event/boundary; emit CPU planner / urgency / commitment; create a
player-command gameplay loop; add a UI framework; add a real-time loop; register a global default
schedule; add semantic/raw WGSL; add a new shader/GPU kernel; add hard currency; add markets/trade/
`ai_budget`; add nested Resource Flow; add multi-faction economy; implement ClauseThing; alter
`simthing-spec` for ClauseThing; edit invariants; or reopen any closed ladder.

---

## 5. Future implementation slice (if opened)

The next handoff **may**:
- add a bounded command/admission module for Local Patrol Economy;
- accept a tiny validated command list (§3);
- mutate only the allowed scenario input/config values before invoking the existing schedule/observation
  path (`run_default_schedule_0080_0` → `observe_gameplay_0080_0`);
- return the existing read-only observation export after command application;
- keep all existing production/schedule/gameplay observation tests green;
- add rejection tests for forbidden direct-control paths.

It **must not**: implement direct movement control; create gameplay UI; create a real-time loop; create a
general command system; create a general scenario editor; or implement ClauseThing.

**Discipline.** The admission layer is pure Rust validation over the existing input struct; it touches no
shader text and no GPU kernel (WGSL ban, invariants row 169/194), and it adds no decision logic — FIELD_POLICY
remains the sole mover-decision source.

---

## 6. Future required tests (named, not implemented)

- `control_0080_0_explicit_opt_in_only`
- `control_0080_0_accepts_bounded_scenario_value_commands`
- `control_0080_0_runs_observed_scenario_after_admitted_command`
- `control_0080_0_exports_transcript_after_command`
- `control_0080_0_replay_after_command_deterministic`
- `control_0080_0_rejects_direct_patrol_move`
- `control_0080_0_rejects_direct_pirate_move`
- `control_0080_0_rejects_external_boundary_request`
- `control_0080_0_rejects_cpu_planner_or_commitment`
- `control_0080_0_rejects_player_command_loop`
- `control_0080_0_rejects_ui_framework`
- `control_0080_0_rejects_realtime_loop`
- `control_0080_0_rejects_global_default_schedule`
- `control_0080_0_rejects_semantic_or_raw_wgsl`
- `control_0080_0_rejects_hard_currency_markets_trade_aibudget`
- `control_0080_0_rejects_nested_resource_flow`
- `control_0080_0_rejects_clausething_dependency`
- `control_0080_0_docs_status_matches_gate`

---

## 7. Exit criteria (this opening PR)

- [x] Design authority chose **Option A** (open with narrowing).
- [x] Opening spec exists; `CONTROL-0080-0` marked OPEN WITH NARROWING (bounded command admission).
- [x] Scope is Local Patrol Economy only; writes only `DefaultSchedule0080Input` bounded values/config.
- [x] Future implementation slice is named but not implemented.
- [x] `PRODUCTION-PATH-0080-0`, `DEFAULT-SCHEDULE-0080-0`, `GAMEPLAY-0080-0` remain IMPLEMENTED / PASS.
- [x] Direct movement control, UI framework, real-time loop, global default schedule remain CLOSED.
- [x] Mapping guidance + worklog updated.
- [x] No code changed.

---

## 8. Pointers
- Active constitution: [`../design_0_0_8_0.md`](../design_0_0_8_0.md)
- Production track: [`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md)
- Schedule spec: [`../production_paths/default_schedule_0080_0_opening_spec.md`](../production_paths/default_schedule_0080_0_opening_spec.md)
- Observation spec: [`gameplay_0080_0_opening_spec.md`](gameplay_0080_0_opening_spec.md)
- Binding rules: [`../invariants.md`](../invariants.md)
- Visibility report: [`../tests/phase_control_0080_0_opening_review_results.md`](../tests/phase_control_0080_0_opening_review_results.md)
