# PRODUCTION-PATH-0080-1 — Nested Starmap Production Path Opening Spec

> **Status: OPENING SPEC / NO IMPLEMENTATION.**
> - `SCENARIO-0080-1` (Nested Starmap) is **ACCEPTED**.
> - `ATLAS-0080-0` is **IMPLEMENTED / PASS** (`run_atlas_0080_0`).
> - `ECON-SCALE-0080-0` is **IMPLEMENTED / PASS** (`run_econ_scale_0080_0`).
> - `PRODUCTION-PATH-0080-1` is **OPEN only as an opt-in/default-off Nested Starmap production-path gate**.
> - **This PR does not implement the production path.**
>
> Verdict: **OPEN WITH NARROWING (Option A)** — composition of two already-green substrates for one named
> scenario; opt-in, deterministic, reversible. **No schedule, no movement, no new substrate.**

---

## 1. Why this gate now unlocks execution

Nested Starmap's two required substrates are both implemented/pass: `ATLAS-0080-0` (sparse-residency
nested mapping) and `ECON-SCALE-0080-0` (bounded Terran/Pirate faction-indexed contended ECON). The next
product step is their **composition into a single opt-in production scenario surface** — instantiate the
accepted scenario, drive both substrates with explicit opt-in, and compose one inspectable scenario-level
report. This is **not** another substrate proof wrapper; it composes two implemented capabilities for the
named scenario. Movement (schedule) is explicitly a later slice.

---

## 2. Narrowed production-path definition

`PRODUCTION-PATH-0080-1` is defined as:
- `SCENARIO-0080-1` only; explicit **opt-in / default-off**; deterministic;
- bounded to accepted Nested Starmap dimensions: 10×10 starmap; 10 deterministic starsystems; each
  starsystem a 10×10 subfield; each starsystem one planet; each planet a 10×10 submap; ≈ 2,100 logical
  location simthings;
- uses **`ATLAS-0080-0`** for sparse residency and nested descent/ascent;
- uses **`ECON-SCALE-0080-0`** for bounded Terran/Pirate faction-indexed contended clearing;
- composes **owner-overlay inherited numeric weights**;
- exposes **SEAD composite-gap inputs / read-only terms**;
- **no schedule yet** (no movement execution).

---

## 3. Required future implementation behavior (not implemented here)

The future implementation **may**:
- add a narrow `production_path_0080_1` module (`simthing-driver`);
- instantiate the accepted Nested Starmap scenario (§4.1 of the admission packet: 6/10 Terran stars,
  4 neutral; 3 Terran ships at 3 distinct Terran stars; 3 pirate ships at distinct neutral stars; pirate
  owns only its ships);
- call `run_atlas_0080_0` with explicit opt-in;
- call `run_econ_scale_0080_0` with explicit opt-in;
- validate that **both** reports are `admitted` and implemented/pass; **reject** if either substrate
  report is disabled / rejected / not admitted;
- compose a scenario-level report: starmap shape; starsystem/planet structure; active/resident theaters;
  Terran/Pirate fixed faction set; pirate full-economy participation; contended clearing reports; derived
  ownership up-aggregation summary; inherited overlay weight summary; SEAD composite-gap read-only term
  summary; deterministic replay checksum;
- keep all substrate reports intact and inspectable.

It **must not**: implement schedule/observation/control/demo for `0080-1`; emit movement; add a default
session pass-graph wiring; or anything in §10.

---

## 4. Owner / overlay contract

- Faction owner simthings are **session siblings**, not spatial parents.
- Location owner overlays **inherit numeric personality/policy weights** from owning faction owner
  simthings (OWNER down-broadcast — proven substrate).
- Ships inherit owner overlays from their faction.
- Planet → starsystem **ownership up-aggregation is a derived owner overlay**, not reparenting.
- **Owner-entity-as-spatial-parent remains rejected; capture-as-reparenting remains rejected.**
- In this first production path these may be represented as **numeric overlay summaries**; **no new owner
  substrate is opened.**

---

## 5. SEAD composite-gap contract

- Movement decisions remain **SEAD-sourced in later schedule slices** — not here.
- This production path **may compute/report read-only composite-gap inputs**: `current(space) −
  inherited_setpoint(space)`; supply/security gap; bilateral relational gap; sum/composite vector terms.
- **No** CPU planner / urgency / commitment emission; **no** direct move requests; **no** externally
  scripted `BoundaryRequest`; **no** new SEAD substrate; **no** schedule execution.

---

## 6. ECON / resource-flow contract

- Use `ECON-SCALE-0080-0` report output (`run_econ_scale_0080_0`) as the contended ECON substrate.
- **No** hard currency, markets, trade, `ai_budget`; **no** nested Resource Flow beyond FlatStar posture;
  **no** unbounded factions. Pirate full-economy participation stays bounded to the **Terran + Pirate**
  fixed set.

---

## 7. Atlas / residency contract

- Use `ATLAS-0080-0` report output (`run_atlas_0080_0`) as the nested-mapping substrate.
- **No** default session pass-graph wiring; **no** global mapping scheduler; **no** real-time loop.
- Residency is a **value no-op**; sparse residency remains scenario-scoped.

---

## 8. Future required tests (named, not implemented)

- `production_path_0080_1_explicit_opt_in_only`
- `production_path_0080_1_requires_atlas_and_econ_scale_admitted`
- `production_path_0080_1_rejects_disabled_atlas`
- `production_path_0080_1_rejects_disabled_econ_scale`
- `production_path_0080_1_instantiates_nested_starmap_shape`
- `production_path_0080_1_composes_sparse_residency_report`
- `production_path_0080_1_composes_faction_index_econ_report`
- `production_path_0080_1_reports_owner_overlay_inheritance`
- `production_path_0080_1_reports_ownership_up_aggregation`
- `production_path_0080_1_reports_sead_composite_gap_terms_readonly`
- `production_path_0080_1_replay_deterministic`
- `production_path_0080_1_no_schedule_observation_control_demo`
- `production_path_0080_1_no_default_session_pass_graph_wiring`
- `production_path_0080_1_no_global_default_schedule`
- `production_path_0080_1_no_realtime_loop_or_ui`
- `production_path_0080_1_no_direct_movement_or_external_boundary_request`
- `production_path_0080_1_no_cpu_planner_or_commitment`
- `production_path_0080_1_no_semantic_or_raw_wgsl`
- `production_path_0080_1_no_hard_currency_markets_trade_aibudget`
- `production_path_0080_1_no_nested_resource_flow`
- `production_path_0080_1_no_clausething_dependency`
- `production_path_0080_1_docs_status_matches_gate`

---

## 9. Stop conditions

Stop if this gate would require: schedule execution; observation/control/demo for `0080-1`; direct
movement command; externally-scripted `BoundaryRequest`; CPU planner / urgency / commitment; default
session pass-graph wiring; global default schedule; real-time loop; UI framework; semantic/raw WGSL; new
shader/GPU kernel; hard currency; markets/trade/`ai_budget`; nested Resource Flow; unbounded factions;
owner-entity as spatial parent; capture-as-reparenting; ClauseThing implementation; `simthing-spec`
alteration for ClauseThing; invariant edit; passive proof wrapper; or a general production path beyond this
scenario.

---

## 10. Exit criteria (this opening PR)

- [x] Design authority chose **Option A** (open with narrowing).
- [x] Opening spec exists; `PRODUCTION-PATH-0080-1` marked OPEN WITH NARROWING.
- [x] Scope is `SCENARIO-0080-1` only; composition of `ATLAS-0080-0` + `ECON-SCALE-0080-0`.
- [x] Future implementation slice named, not implemented.
- [x] `ATLAS-0080-0` and `ECON-SCALE-0080-0` remain IMPLEMENTED / PASS.
- [x] Schedule/observation/control/demo for `0080-1` remain not opened.
- [x] Mapping guidance + worklog updated.
- [x] No code changed.

---

## 11. Pointers
- Scenario: [`../scenarios/scenario_0080_1_admission_packet.md`](../scenarios/scenario_0080_1_admission_packet.md)
- Atlas spec / impl: [`atlas_0080_0_opening_spec.md`](atlas_0080_0_opening_spec.md), [`../tests/phase_atlas_0080_0_impl_results.md`](../tests/phase_atlas_0080_0_impl_results.md)
- ECON-scale spec / impl: [`econ_scale_0080_0_opening_spec.md`](econ_scale_0080_0_opening_spec.md), [`../tests/phase_econ_scale_0080_0_impl_results.md`](../tests/phase_econ_scale_0080_0_impl_results.md)
- Production track + PR ladder: [`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md)
- Binding rules: [`../invariants.md`](../invariants.md)
- Visibility report: [`../tests/phase_production_path_0080_1_opening_review_results.md`](../tests/phase_production_path_0080_1_opening_review_results.md)
