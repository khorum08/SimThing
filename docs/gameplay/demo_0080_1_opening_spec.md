# DEMO-0080-1 — Nested Starmap Headless Demo / Export Opening Spec

> **Status: OPENING SPEC / NO IMPLEMENTATION.**
> - `SCENARIO-0080-1` (Nested Starmap) is **ACCEPTED**.
> - `ATLAS-0080-0` is **IMPLEMENTED / PASS** (`run_atlas_0080_0`).
> - `ECON-SCALE-0080-0` is **IMPLEMENTED / PASS** (`run_econ_scale_0080_0`).
> - `PRODUCTION-PATH-0080-1` is **IMPLEMENTED / PASS** (`run_production_path_0080_1`).
> - `DEFAULT-SCHEDULE-0080-1` is **IMPLEMENTED / PASS** (`run_default_schedule_0080_1`).
> - `GAMEPLAY-0080-1` is **IMPLEMENTED / PASS** (`observe_gameplay_0080_1`).
> - `CONTROL-0080-1` is **IMPLEMENTED / PASS** (bounded command admission).
> - `DEMO-0080-1` is **OPEN only as a headless demo/export packaging gate**.
> - **This PR does not implement the demo.**
>
> Verdict: **OPEN WITH NARROWING (Option A)** — deterministic, opt-in, headless, non-interactive
> packaging over the *already-implemented* path. **CLI decision: `No CLI binary`. No new simulation
> behavior.**

---

## 0. Naming caution (design-authority note)

`DEMO-0080-1` is **packaging / usability** for the completed Nested Starmap vertical slice. It adds no
simulation feature and no decision logic: it applies a canonical bounded `CONTROL-0080-1` command batch,
runs the existing `control → schedule → observation` path, and emits the existing deterministic
transcript/export plus a compact demo report. "Demo" does **not** authorize a UI, an interactive runner, a
real-time loop, or any player command loop — those remain CLOSED and a stop-and-escalate.

---

## 1. Why headless demo/export may now be product-relevant

Nested Starmap can now instantiate (`PRODUCTION-PATH-0080-1`), schedule SEAD-sourced movement
(`DEFAULT-SCHEDULE-0080-1`), observe/export results (`GAMEPLAY-0080-1`), and accept bounded command
admission (`CONTROL-0080-1`). The remaining product need is a **stable, reusable, non-interactive headless
demo surface** that exercises the completed path end-to-end without requiring a reader to assemble raw
structs manually. This is packaging of already-implemented capability into a reproducible demo/export path
— **not** new simulation behavior and **not** a passive proof wrapper.

---

## 2. Narrowed demo definition

`DEMO-0080-1` is defined as: `SCENARIO-0080-1` only; explicit **opt-in / default-off**; deterministic;
headless; **non-interactive**; **library helper unless Opus explicitly authorizes a CLI** (see §4); uses
existing `CONTROL-0080-1` command admission; uses existing `DEFAULT-SCHEDULE-0080-1`; uses existing
`GAMEPLAY-0080-1` observation/export; **no** new simulation behavior; **no** new schedule; **no** new
control vocabulary beyond a canonical bounded command batch; **no** UI; **no** real-time loop; **no**
global default schedule.

---

## 3. CLI / binary decision

**Decision: `No CLI binary`.** The demo is implemented only as a **library helper + tests** (emitting the
existing deterministic export + a compact demo report, with an optional golden transcript). No binary
target is authorized under this gate. A demo binary/CLI would be a separate, explicitly-authorized future
decision; adding one under this gate is a stop-and-escalate.

---

## 4. Future implementation slice (if opened)

The next handoff **may**: add a narrow `demo_0080_1` module; create a canonical bounded `CONTROL-0080-1`
command batch; run the existing `control → schedule → observation/export` path
(`admit/control_0080_1` → `run_default_schedule_0080_1` → `observe_gameplay_0080_1`); emit the existing
deterministic text transcript/export; emit a compact demo report containing scenario id/name, starmap
shape, atlas residency summary, faction-index ECON summary, owner-overlay/up-aggregation summary, SEAD
movement trace, Terran/Pirate movement rows, command transcript rows, and replay checksum; optionally add
a stable golden/sample transcript; preserve all control/gameplay/schedule/production/atlas/econ
regressions; update docs/report.

It **must not**: add a CLI binary (§3); add direct movement control; add external `BoundaryRequest`s;
bypass SEAD; add command input beyond the canonical bounded demo batch; add UI; add a real-time loop; add
a global default schedule; add semantic/raw WGSL; add a new shader/GPU kernel; add hard currency; add
nested Resource Flow; implement ClauseThing.

**Discipline.** The demo is pure Rust read/orchestration over existing seams; it touches no shader text and
no GPU kernel (WGSL ban, invariants row 169/194) and adds no decision logic — SEAD remains the sole
mover-decision source.

---

## 5. Future required tests (named, not implemented)

- `demo_0080_1_explicit_opt_in_only`
- `demo_0080_1_runs_canonical_control_batch`
- `demo_0080_1_uses_existing_control_schedule_observation_path`
- `demo_0080_1_emits_nested_starmap_export`
- `demo_0080_1_includes_command_transcript`
- `demo_0080_1_includes_terran_and_pirate_movement_rows`
- `demo_0080_1_includes_atlas_residency_summary`
- `demo_0080_1_includes_faction_index_econ_summary`
- `demo_0080_1_includes_owner_overlay_and_up_aggregation_summary`
- `demo_0080_1_replay_deterministic`
- `demo_0080_1_no_cli_binary_unless_authorized`
- `demo_0080_1_no_direct_movement_command`
- `demo_0080_1_no_external_boundary_request`
- `demo_0080_1_no_sead_bypass`
- `demo_0080_1_no_player_command_loop`
- `demo_0080_1_no_ui_framework`
- `demo_0080_1_no_realtime_loop`
- `demo_0080_1_no_global_default_schedule`
- `demo_0080_1_no_semantic_or_raw_wgsl`
- `demo_0080_1_no_new_shader_or_gpu_kernel`
- `demo_0080_1_no_hard_currency_markets_trade_aibudget`
- `demo_0080_1_no_nested_resource_flow`
- `demo_0080_1_no_clausething_dependency`
- `demo_0080_1_docs_status_matches_gate`

---

## 6. Stop conditions

Stop if this gate would require: direct movement command; externally-scripted `BoundaryRequest`; SEAD
bypass; CPU planner / urgency / commitment; player command loop; UI framework; real-time loop; global
default schedule; semantic/raw WGSL; new shader/GPU kernel; hard currency; markets/trade/`ai_budget`;
nested Resource Flow; unbounded factions; owner-entity as spatial parent; capture-as-reparenting;
ClauseThing implementation; `simthing-spec` alteration for ClauseThing; invariant edit; passive proof
wrapper; general scenario editor; general command framework; general gameplay framework; a CLI binary
(§3); or reopening any closed ladder.

---

## 7. Exit criteria (this opening PR)

- [x] Design authority chose **Option A** (open with narrowing); CLI decision = **No CLI binary**.
- [x] Opening spec exists; `DEMO-0080-1` marked OPEN WITH NARROWING.
- [x] Scope is `SCENARIO-0080-1` only; packaging over the existing control→schedule→observation path.
- [x] Future implementation slice named, not implemented.
- [x] All prior `0080-1` gates remain IMPLEMENTED / PASS.
- [x] UI, real-time loop, direct movement control, global default schedule remain CLOSED.
- [x] Mapping guidance + worklog updated.
- [x] No code changed.

---

## 8. Pointers
- Scenario: [`../scenarios/scenario_0080_1_admission_packet.md`](../scenarios/scenario_0080_1_admission_packet.md)
- Control spec / impl: [`control_0080_1_opening_spec.md`](control_0080_1_opening_spec.md), [`../tests/phase_control_0080_1_impl_results.md`](../tests/phase_control_0080_1_impl_results.md)
- Observation spec / impl: [`gameplay_0080_1_opening_spec.md`](gameplay_0080_1_opening_spec.md), [`../tests/phase_gameplay_0080_1_impl_results.md`](../tests/phase_gameplay_0080_1_impl_results.md)
- Prior headless demo precedent: [`demo_0080_0_opening_spec.md`](demo_0080_0_opening_spec.md)
- Production track + PR ladder: [`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md)
- Binding rules: [`../invariants.md`](../invariants.md)
- Visibility report: [`../tests/phase_demo_0080_1_opening_review_results.md`](../tests/phase_demo_0080_1_opening_review_results.md)
