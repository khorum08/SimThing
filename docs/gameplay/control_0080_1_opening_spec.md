# CONTROL-0080-1 — Nested Starmap Bounded Command Admission Opening Spec

> **Status: IMPLEMENTED / PASS - bounded command admission.**
> - `SCENARIO-0080-1` (Nested Starmap) is **ACCEPTED**.
> - `ATLAS-0080-0` is **IMPLEMENTED / PASS** (`run_atlas_0080_0`).
> - `ECON-SCALE-0080-0` is **IMPLEMENTED / PASS** (`run_econ_scale_0080_0`).
> - `PRODUCTION-PATH-0080-1` is **IMPLEMENTED / PASS** (`run_production_path_0080_1`).
> - `DEFAULT-SCHEDULE-0080-1` is **IMPLEMENTED / PASS** (`run_default_schedule_0080_1`).
> - `GAMEPLAY-0080-1` is **IMPLEMENTED / PASS** — read-only observation/export (`observe_gameplay_0080_1`).
> - `CONTROL-0080-1` is **IMPLEMENTED / PASS only as a bounded command-admission gate**.
> - Implementation report: [`../tests/phase_control_0080_1_impl_results.md`](../tests/phase_control_0080_1_impl_results.md).
>
> Verdict: **IMPLEMENTED / PASS (Option A)** — bounded, opt-in, deterministic, parameter-admission only.
> **Not** player control, not a CPU planner, not direct movement, not a general command system.

---

## 0. Naming caution (design-authority note)

`CONTROL-0080-1` authorizes **bounded command *admission*** — a designer may select from a tiny validated
vocabulary that writes only **already-accepted `DefaultSchedule0081Input` / Nested Starmap bounded
scenario/config values** before the existing schedule runs. It does **not** authorize direct ship
movement, a player command bus/loop, or free-form scripting. A command never moves a Terran or Pirate
ship, never emits a `BoundaryRequest`, and never bypasses SEAD: movement still **emerges** from the
already-implemented GPU-resident `Threshold + EmitEvent → BoundaryRequest` posture in
`DEFAULT-SCHEDULE-0080-1`. Reading "control" as license for direct movement or a gameplay loop is **out of
scope and a stop-and-escalate.**

---

## 1. Why bounded command admission may now be product-relevant

Nested Starmap now runs deterministically (`DEFAULT-SCHEDULE-0080-1`) and is legible in product-readable
form (`GAMEPLAY-0080-1`). The next product question is narrow: may a designer/user **select from a tiny
admitted command vocabulary** to set up a scenario configuration and then observe the deterministic
result? The vocabulary steers only **accepted scenario/config values or admitted schedule parameters** —
never direct movement mutation. This is **not** a CPU planner, not player control, not UI, not a general
command bus, and not ClauseThing.

---

## 2. Narrowed control definition

`CONTROL-0080-1` is defined as:
- `SCENARIO-0080-1` only; explicit **opt-in / default-off**; deterministic;
- **bounded command admission only**;
- writes only existing `DefaultSchedule0081Input` / Nested Starmap bounded scenario/config values;
- after admission, runs the existing **schedule → observation** path
  (`run_default_schedule_0080_1` → `observe_gameplay_0080_1`);
- **no** direct ship movement command; **no** externally-scripted `BoundaryRequest`; **no** SEAD bypass;
- **no** CPU planner / urgency / commitment; **no** player command loop; **no** UI; **no** real-time loop;
- **no** global default schedule; **no** demo packaging.

---

## 3. Implemented minimal command vocabulary

Each targets an existing `DefaultSchedule0081Input` / Nested Starmap bounded value/config field:

- `set_step_count(value)`
- `set_terran_threshold(value)`
- `set_pirate_threshold(value)`
- `set_terran_source_starsystem(value)`
- `set_terran_candidate_starsystem(value)`
- `set_pirate_source_starsystem(value)`
- `set_pirate_candidate_starsystem(value)`
- `set_supply_security_gap(value)`
- `set_bilateral_relational_gap(value)`
- `set_composite_gap_sum(value)`
- `run_observed_scenario`
- `export_transcript`

Each command must: be deterministic; be explicitly validated; target existing bounded input/config values
only; produce a reproducible observation export; **never directly issue a move request**; **never create
or emit a `BoundaryRequest`**; **never change identity, owner overlay, membership, or ownership directly**.

---

## 4. Forbidden commands / stop conditions

Reject any command that would: move a Terran ship directly; move a Pirate ship directly; create an
externally-scripted `BoundaryRequest`; bypass threshold/event posture; bypass SEAD; emit CPU planner /
urgency / commitment; create a player command loop; add a UI framework; add a real-time loop; register a
global default schedule; add semantic/raw WGSL; add a new shader/GPU kernel; add hard currency; add
markets/trade/`ai_budget`; add nested Resource Flow; add unbounded factions; make owner-entity a spatial
parent; implement capture-as-reparenting; implement ClauseThing; alter `simthing-spec` for ClauseThing;
edit invariants; reopen closed ladders; or add passive proof wrappers.

---

## 5. Implementation slice

The implementation adds a narrow `control_0080_1` module; accepts the tiny validated command list (§3);
mutates only allowed `DefaultSchedule0081Input` / Nested Starmap bounded input/config values; invokes the
existing `DEFAULT-SCHEDULE-0080-1` → `GAMEPLAY-0080-1` path; returns the existing read-only observation
export after admitted commands; preserves all schedule/observation/production/atlas/econ regressions; and
adds rejection tests for forbidden direct-control paths.

It **must not**: implement direct movement control; implement external boundary requests; implement
gameplay UI; implement a real-time loop; create a general command system; create a general scenario
editor; implement ClauseThing.

**Discipline.** The admission layer is pure Rust validation over the existing input struct; it touches no
shader text and no GPU kernel (WGSL ban, invariants row 169/194), and it adds no decision logic — SEAD
remains the sole mover-decision source.

---

## 6. Required tests (implemented)

- `control_0080_1_explicit_opt_in_only`
- `control_0080_1_accepts_bounded_schedule_value_commands`
- `control_0080_1_runs_observed_scenario_after_admitted_command`
- `control_0080_1_exports_transcript_after_command`
- `control_0080_1_replay_after_command_deterministic`
- `control_0080_1_rejects_direct_terran_move`
- `control_0080_1_rejects_direct_pirate_move`
- `control_0080_1_rejects_external_boundary_request`
- `control_0080_1_rejects_sead_bypass`
- `control_0080_1_rejects_cpu_planner_or_commitment`
- `control_0080_1_rejects_player_command_loop`
- `control_0080_1_rejects_ui_framework`
- `control_0080_1_rejects_realtime_loop`
- `control_0080_1_rejects_global_default_schedule`
- `control_0080_1_rejects_semantic_or_raw_wgsl`
- `control_0080_1_rejects_new_shader_or_gpu_kernel`
- `control_0080_1_rejects_hard_currency_markets_trade_aibudget`
- `control_0080_1_rejects_nested_resource_flow`
- `control_0080_1_rejects_clausething_dependency`
- `control_0080_1_docs_status_matches_gate`

---

## 7. Stop conditions

Stop if this gate would require: direct ship movement control; externally-scripted `BoundaryRequest`;
SEAD bypass; CPU planner / urgency / commitment; player command loop; UI framework; real-time loop; global
default schedule; demo packaging; semantic/raw WGSL; new shader/GPU kernel; hard currency;
markets/trade/`ai_budget`; nested Resource Flow; unbounded factions; owner-entity as spatial parent;
capture-as-reparenting; ClauseThing implementation; `simthing-spec` alteration for ClauseThing; invariant
edit; passive proof wrapper; or a general command system / scenario editor.

---

## 8. Exit criteria (this opening PR)

- [x] Design authority chose **Option A** (open with narrowing).
- [x] Opening spec exists; `CONTROL-0080-1` marked IMPLEMENTED / PASS (bounded command admission).
- [x] Scope is `SCENARIO-0080-1` only; writes only `DefaultSchedule0081Input` bounded values/config.
- [x] Implementation slice landed in `control_0080_1`.
- [x] `GAMEPLAY-0080-1` and all prior `0080-1` gates remain IMPLEMENTED / PASS.
- [x] Demo for `0080-1` remains not opened.
- [x] Mapping guidance + worklog updated.
- [x] Code changed only for bounded command admission and tests.

---

## 9. Pointers
- Scenario: [`../scenarios/scenario_0080_1_admission_packet.md`](../scenarios/scenario_0080_1_admission_packet.md)
- Observation spec / impl: [`gameplay_0080_1_opening_spec.md`](gameplay_0080_1_opening_spec.md), [`../tests/phase_gameplay_0080_1_impl_results.md`](../tests/phase_gameplay_0080_1_impl_results.md)
- Schedule spec / impl: [`../production_paths/default_schedule_0080_1_opening_spec.md`](../production_paths/default_schedule_0080_1_opening_spec.md), [`../tests/phase_default_schedule_0080_1_impl_results.md`](../tests/phase_default_schedule_0080_1_impl_results.md)
- Prior bounded command-admission precedent: [`control_0080_0_opening_spec.md`](control_0080_0_opening_spec.md)
- Production track + PR ladder: [`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md)
- Binding rules: [`../invariants.md`](../invariants.md)
- Visibility report: [`../tests/phase_control_0080_1_opening_review_results.md`](../tests/phase_control_0080_1_opening_review_results.md)
- Implementation report: [`../tests/phase_control_0080_1_impl_results.md`](../tests/phase_control_0080_1_impl_results.md)
