# DEMO-0080-0 — Local Patrol Economy Headless Demo / Export Opening Spec

> **Status: IMPLEMENTED / PASS — headless demo/export library helper (`run_demo_0080_0`).**
> - `SCENARIO-0080-0` (Local Patrol Economy) is **ACCEPTED**.
> - `PRODUCTION-PATH-0080-0` is **IMPLEMENTED / PASS** (`run_production_path_0080_0`).
> - `DEFAULT-SCHEDULE-0080-0` is **IMPLEMENTED / PASS** (1A schedule + patrol, 1B bounded pirate loop).
> - `GAMEPLAY-0080-0` is **IMPLEMENTED / PASS** — read-only observation export (`observe_gameplay_0080_0`).
> - `CONTROL-0080-0` is **IMPLEMENTED / PASS** — bounded command admission (`admit_control_0080_0`).
> - `DEMO-0080-0` is **IMPLEMENTED / PASS** — headless demo/export library helper.
> - **No CLI binary.** UI framework, real-time loop, player command loop, direct movement control remain CLOSED.
>
> Implementation report: [`../tests/phase_demo_0080_0_impl_results.md`](../tests/phase_demo_0080_0_impl_results.md).

---

## 0. Naming caution (design-authority note)

`DEMO-0080-0` is **packaging / usability** for the completed Local Patrol Economy vertical slice. It
adds no simulation feature and no decision logic: it applies a canonical bounded `CONTROL-0080-0` command
batch, runs the existing `control → schedule → observation` path, and emits the existing deterministic
transcript/export. "Demo" does **not** authorize a UI, an interactive runner, a real-time loop, or any
player command loop — those remain CLOSED and a stop-and-escalate.

---

## 1. Why headless demo/export may now be product-relevant

The vertical slice is runnable through bounded command admission (`CONTROL-0080-0`) and legible through
observation export (`GAMEPLAY-0080-0`). The remaining product need is a **stable, reusable,
non-interactive headless demo surface** that exercises the completed slice end-to-end without requiring a
reader to assemble raw structs manually. This is packaging of already-implemented capability into a
reproducible demo/export path — **not** a new simulation feature and **not** a passive proof wrapper.

---

## 2. Narrowed demo definition

`DEMO-0080-0` is defined as: Local Patrol Economy only; explicit **opt-in / default-off**; deterministic;
headless; **non-interactive**; **no** player command loop; **no** UI framework; **no** real-time loop;
**no** global default schedule; **no** direct movement control; **no** externally-scripted boundary
request; **no** new schedule implementation; **no** new simulation behavior; **no** semantic/raw WGSL;
**no** ClauseThing.

---

## 3. Future implementation slice (if opened)

The next handoff **may**:
- add a headless demo/export **helper** for Local Patrol Economy;
- use existing `CONTROL-0080-0` command admission (`admit_control_0080_0`, `Control0080CommandBatch`);
- apply a **canonical bounded command batch** (`Control0080CommandBatch::canonical_run()` or equivalent);
- run the existing `control → schedule → observation/export` path
  (`admit_control_0080_0` → `run_default_schedule_0080_0` → `observe_gameplay_0080_0`);
- emit the existing deterministic text transcript/export;
- optionally add a stable sample output / golden transcript if useful;
- preserve all control/gameplay/schedule/production regression tests;
- update docs/report.

It **must not**: add a CLI binary (see §5); add a UI framework; add interactive player commands; add a
real-time loop; add a global default schedule; add direct movement control; add new WGSL/shader/kernel;
add new simulation behavior; or add ClauseThing.

**Discipline.** The demo is pure Rust read/orchestration over existing seams; it touches no shader text
and no GPU kernel (WGSL ban, invariants row 169/194) and adds no decision logic — SEAD remains the sole
mover-decision source.

---

## 4. CLI / binary decision

**Decision: `No CLI binary`.** The demo is implemented only as a **library helper + tests** (returning
the existing deterministic export, with an optional golden transcript). No binary target is authorized
under this gate. A demo binary/CLI would be a separate, explicitly-authorized future decision; adding one
under this gate is a stop-and-escalate.

---

## 5. Future required tests (named, not implemented)

- `demo_0080_0_explicit_opt_in_only`
- `demo_0080_0_runs_canonical_control_batch`
- `demo_0080_0_emits_observation_export`
- `demo_0080_0_export_replay_deterministic`
- `demo_0080_0_uses_existing_control_schedule_observation_path`
- `demo_0080_0_no_direct_movement_command`
- `demo_0080_0_no_external_boundary_request`
- `demo_0080_0_no_player_command_loop`
- `demo_0080_0_no_ui_framework`
- `demo_0080_0_no_realtime_loop`
- `demo_0080_0_no_global_default_schedule`
- `demo_0080_0_no_semantic_or_raw_wgsl`
- `demo_0080_0_no_hard_currency_markets_trade_aibudget`
- `demo_0080_0_no_clausething_dependency`
- `demo_0080_0_docs_status_matches_gate`

---

## 6. Stop conditions

Stop (do not implement under this gate) if it would require: direct movement control; externally-scripted
boundary request; player command loop; UI framework; real-time loop; global default schedule; new
schedule implementation; semantic/raw WGSL; new shader/GPU kernel; CPU planner / urgency / commitment
emission; hard currency; markets/trade/`ai_budget`; nested Resource Flow; multi-faction economy;
ClauseThing implementation; `simthing-spec` alteration for ClauseThing; invariant edit; passive proof
wrapper; general scenario editor; general command framework; a CLI binary (§5); or reopening any closed
ladder.

---

## 7. Exit criteria (this opening PR)

- [x] Design authority chose **Option A** (open with narrowing); CLI decision = **No CLI binary**.
- [x] Opening spec exists; `DEMO-0080-0` marked OPEN WITH NARROWING.
- [x] Scope is Local Patrol Economy only; packaging over the existing control→schedule→observation path.
- [x] Future implementation slice is named but not implemented.
- [x] All prior gates remain IMPLEMENTED / PASS.
- [x] UI framework, real-time loop, direct movement control, global default schedule remain CLOSED.
- [x] Mapping guidance + worklog updated.
- [x] No code changed.

---

## 8. Pointers
- Active constitution: [`../design_0_0_8_0.md`](../design_0_0_8_0.md)
- Production track: [`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md)
- Control spec: [`control_0080_0_opening_spec.md`](control_0080_0_opening_spec.md)
- Observation spec: [`gameplay_0080_0_opening_spec.md`](gameplay_0080_0_opening_spec.md)
- Binding rules: [`../invariants.md`](../invariants.md)
- Visibility report: [`../tests/phase_demo_0080_0_opening_review_results.md`](../tests/phase_demo_0080_0_opening_review_results.md)
