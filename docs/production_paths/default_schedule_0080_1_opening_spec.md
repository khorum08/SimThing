# DEFAULT-SCHEDULE-0080-1 — Nested Starmap Schedule / Movement Opening Spec

> **Status: OPENING SPEC / NO IMPLEMENTATION.**
> - `SCENARIO-0080-1` (Nested Starmap) is **ACCEPTED**.
> - `ATLAS-0080-0` is **IMPLEMENTED / PASS** (`run_atlas_0080_0`).
> - `ECON-SCALE-0080-0` is **IMPLEMENTED / PASS** (`run_econ_scale_0080_0`).
> - `PRODUCTION-PATH-0080-1` is **IMPLEMENTED / PASS** (`run_production_path_0080_1`).
> - `DEFAULT-SCHEDULE-0080-1` is **OPEN only as a scenario-scoped opt-in schedule/movement gate**.
> - **This PR does not implement schedule/movement.**
>
> Verdict: **OPEN WITH NARROWING (Option A)** — deterministic, opt-in/default-off, scenario-scoped
> SEAD-sourced movement cadence over the composed Nested Starmap report. **No new substrate; no direct
> movement; no observation/control/demo.**

---

## 1. Why this gate now unlocks execution

`PRODUCTION-PATH-0080-1` composes atlas sparse residency, bounded Terran/Pirate faction-indexed ECON,
owner-overlay summaries, ownership up-aggregation, and **read-only SEAD composite-gap terms** into one
deterministic scenario report. The next product step is to **execute deterministic movement decisions**
over that composed scenario. This is not another substrate proof wrapper; it turns the already-composed
scenario report into a bounded **SEAD-sourced movement cadence** — movement at last becomes *live*, but
only through the accepted `Threshold + EmitEvent → BoundaryRequest` posture.

---

## 2. Narrowed schedule definition

`DEFAULT-SCHEDULE-0080-1` is defined as:
- `SCENARIO-0080-1` only; explicit **opt-in / default-off**; deterministic;
- bounded to the accepted Nested Starmap dimensions (10×10 starmap; 10 deterministic starsystems; 10×10
  subfields; one planet each with a 10×10 submap; ≈ 2,100 logical locations);
- **consumes `PRODUCTION-PATH-0080-1`** (`run_production_path_0080_1`);
- runs a **bounded** number of deterministic steps;
- emits **zero or more `BoundaryRequest`s only through the SEAD threshold/event posture**;
- routes accepted boundary requests through the **existing mobility/transfer substrate posture**;
- updates only schedule-step reports and movement outcomes authorized by existing substrate behavior;
- **no observation/control/demo yet**; **not** a global default schedule.

---

## 3. Movement scope

The future implementation **may**:
- move **Terran ships** between Terran-owned or contended starsystems;
- move **Pirate ships** between neutral or weakly-defended starsystems;
- use the **composite-gap terms** (already reported read-only by `PRODUCTION-PATH-0080-1`) to choose a
  candidate destination;
- use active/resident atlas theaters from `ATLAS-0080-0`;
- use Terran/Pirate ECON state from `ECON-SCALE-0080-0`;
- preserve owner overlays; preserve identity through movement;
- update participant location/membership through the **existing mobility/transfer substrate posture**.

It **must not**: add direct movement commands; add externally-scripted `BoundaryRequest`s; add a CPU
planner; add pathfinding beyond bounded scenario-local candidate selection; implement
observation/control/demo.

---

## 4. SEAD contract

Movement decisions must be derived from: the **read-only composite-gap terms** from
`PRODUCTION-PATH-0080-1` (`current(space) − inherited_setpoint`, supply/security gap, bilateral relational
gap, composite gap sum) → **deterministic threshold comparison** → **event emission** → **materialized
`BoundaryRequest`**. No CPU planner, urgency, commitment, goal stack, or direct move scripting. This is
the same GPU-resident decision posture proven at `DEFAULT-SCHEDULE-0080-0`.

---

## 5. Atlas / residency contract

The schedule may reference active/resident theaters from `ATLAS-0080-0`. It **must not** create a global
mapping scheduler, wire atlas into the default session pass graph, or mutate atlas field values except
through existing movement-substrate outputs if the later implementation requires membership changes.
Residency remains a **value no-op**.

---

## 6. ECON / resource-flow contract

The schedule may consume `ECON-SCALE-0080-0` contended clearing reports. **No** hard currency, markets,
trade, `ai_budget`; **no** nested Resource Flow beyond FlatStar posture; **no** unbounded factions. Pirate
remains a bounded **Terran/Pirate fixed-set** economy participant.

---

## 7. Owner / overlay contract

Owner simthings remain **session siblings, not spatial parents**. Owner overlays **persist through
movement**. Planet→starsystem ownership up-aggregation remains a **derived overlay summary**, not
reparenting. **Owner-entity-as-spatial-parent and capture-as-reparenting remain rejected. No new owner
substrate.**

---

## 8. Future required tests (named, not implemented)

- `default_schedule_0080_1_explicit_opt_in_only`
- `default_schedule_0080_1_requires_production_path_admitted`
- `default_schedule_0080_1_rejects_disabled_production_path`
- `default_schedule_0080_1_runs_bounded_nested_starmap_steps`
- `default_schedule_0080_1_threshold_false_emits_no_boundary_request`
- `default_schedule_0080_1_threshold_true_emits_boundary_request`
- `default_schedule_0080_1_routes_boundary_request_through_mobility_substrate`
- `default_schedule_0080_1_moves_terran_ship_by_sead_gap`
- `default_schedule_0080_1_moves_pirate_ship_by_sead_gap`
- `default_schedule_0080_1_preserves_identity_and_owner_overlay`
- `default_schedule_0080_1_updates_membership_without_reparenting`
- `default_schedule_0080_1_consumes_atlas_residency_report`
- `default_schedule_0080_1_consumes_faction_index_econ_report`
- `default_schedule_0080_1_replay_deterministic`
- `default_schedule_0080_1_no_observation_control_demo`
- `default_schedule_0080_1_no_direct_movement_command`
- `default_schedule_0080_1_no_external_boundary_request`
- `default_schedule_0080_1_no_cpu_planner_or_commitment`
- `default_schedule_0080_1_no_default_session_pass_graph_wiring`
- `default_schedule_0080_1_no_global_default_schedule`
- `default_schedule_0080_1_no_realtime_loop_or_ui`
- `default_schedule_0080_1_no_semantic_or_raw_wgsl`
- `default_schedule_0080_1_no_new_shader_or_gpu_kernel`
- `default_schedule_0080_1_no_hard_currency_markets_trade_aibudget`
- `default_schedule_0080_1_no_nested_resource_flow`
- `default_schedule_0080_1_no_clausething_dependency`
- `default_schedule_0080_1_docs_status_matches_gate`

---

## 9. Stop conditions

Stop if this gate would require: observation/control/demo for `0080-1`; direct movement commands;
externally-scripted `BoundaryRequest`; CPU planner / urgency / commitment; default session pass-graph
wiring; global default schedule; real-time loop; UI framework; semantic/raw WGSL; new shader/GPU kernel;
hard currency; markets/trade/`ai_budget`; nested Resource Flow; unbounded factions; owner-entity as
spatial parent; capture-as-reparenting; ClauseThing implementation; `simthing-spec` alteration for
ClauseThing; invariant edit; passive proof wrapper; or a general scheduler beyond this scenario.

---

## 10. Exit criteria (this opening PR)

- [x] Design authority chose **Option A** (open with narrowing).
- [x] Opening spec exists; `DEFAULT-SCHEDULE-0080-1` marked OPEN WITH NARROWING.
- [x] Scope is `SCENARIO-0080-1` only; consumes `PRODUCTION-PATH-0080-1`; SEAD-sourced movement.
- [x] Future implementation slice named, not implemented.
- [x] `PRODUCTION-PATH-0080-1`, `ATLAS-0080-0`, `ECON-SCALE-0080-0` remain IMPLEMENTED / PASS.
- [x] Observation/control/demo for `0080-1` remain not opened.
- [x] Mapping guidance + worklog updated.
- [x] No code changed.

---

## 11. Pointers
- Scenario: [`../scenarios/scenario_0080_1_admission_packet.md`](../scenarios/scenario_0080_1_admission_packet.md)
- Production path spec / impl: [`production_path_0080_1_opening_spec.md`](production_path_0080_1_opening_spec.md), [`../tests/phase_production_path_0080_1_impl_results.md`](../tests/phase_production_path_0080_1_impl_results.md)
- Atlas / ECON-scale specs: [`atlas_0080_0_opening_spec.md`](atlas_0080_0_opening_spec.md), [`econ_scale_0080_0_opening_spec.md`](econ_scale_0080_0_opening_spec.md)
- Prior schedule (proven posture): [`default_schedule_0080_0_opening_spec.md`](default_schedule_0080_0_opening_spec.md)
- Production track + PR ladder: [`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md)
- Binding rules: [`../invariants.md`](../invariants.md)
- Visibility report: [`../tests/phase_default_schedule_0080_1_opening_review_results.md`](../tests/phase_default_schedule_0080_1_opening_review_results.md)
