# DEFAULT-SCHEDULE-0080-0 — Local Patrol Economy Scenario-Scoped Schedule Opening Spec

> **Status: IMPLEMENTED / PASS - 1A scenario-scoped schedule + patrol loop.**
> - `SCENARIO-0080-0` (Local Patrol Economy) is **ACCEPTED**.
> - `PRODUCTION-PATH-0080-0` is **IMPLEMENTED / PASS** (opt-in/default-off; `run_production_path_0080_0`).
> - `DEFAULT-SCHEDULE-0080-0` 1A is **IMPLEMENTED / PASS** as a scenario-scoped schedule + patrol loop.
> - 1B pirate behavior remains **OPEN / FUTURE / NOT IMPLEMENTED**.
>
> Verdict: **OPEN WITH NARROWING (Option A)** — scenario-scoped, opt-in, reversible, non-gameplay,
> **not** a global default schedule.

Implementation result for 1A:
[`../tests/phase_default_schedule_0080_0_impl_1a_results.md`](../tests/phase_default_schedule_0080_0_impl_1a_results.md).
This result does not implement or close the 1B pirate-loop sub-slice.

---

## 1. Why a schedule is a product need now

`PRODUCTION-PATH-0080-0` exists and passes, but it runs a single evaluation when invoked. Local Patrol
Economy is a **dynamic** scenario: disruption and supply change over time, and the patrol's
GPU-resident SEAD decision (`Threshold`+`EmitEvent`→`BoundaryRequest`) only becomes meaningful when
there is a **repeatable scenario tick/step** to re-evaluate those thresholds and route the resulting
`BoundaryRequest`s into the production path. The schedule **unlocks execution cadence** — it is not a
proof wrapper, soak, or accounting variant.

This is **not** a global default schedule, a workspace scheduler, a gameplay loop, or a general
mobility runtime. It is a deterministic, opt-in, scenario-scoped step driver for one named scenario.

---

## 2. Scope (Local Patrol Economy only)

- One economic **owner** (the patrol's owner); one or a very small fixed number of patrols.
- Two or a few local **locations**.
- Bounded economy values only: `supply`, `maintenance`, `local_output`, `local_security`, `disruption`.
- **SEAD decision source remains GPU-resident** `Threshold`+`EmitEvent`→`BoundaryRequest` — no CPU
  planner, no externally-scripted movement.
- Mobility / ownership / flow continue to run through `PRODUCTION-PATH-0080-0`
  (`run_production_path_0080_0`, `ALLOC → REENROLL → IDROUTE → ECON → OWNER`). The schedule **drives**
  that path per step; it does not reimplement it.

### 2.1 Dynamic driver — the pirate/patrol loop (design-authority enrichment, bounded)

A per-tick schedule needs a dynamic to evaluate. Per design authority (2026-06-02), the scenario is
enriched with a **bounded predator/patrol loop** that makes the schedule a genuine **SEAD + Ownership
+ Flow + IDROUTE** testbed — using **no new substrate** (same mobility/transfer substrate, same
GPU-resident SEAD posture, same bounded economy values):

- A **pirate** is a *hostile disruptor entity* with its own identity (a second IDROUTE identity, `k=2`
  — within `max_factions_per_cell`). It is **not** a second economic faction: it carries no treasury,
  market, or economic clearinghouse. ECON stays single-owner.
- **Per tick** at the pirate's current location: pirate **raises `disruption`** and **consumes
  `local_supply` by an amount proportional to local `disruption`**.
- **Pirate relocation** is GPU-resident threshold-driven: when local `disruption ≥ 0.5 × local_supply`,
  the pirate emits a `Threshold`+`EmitEvent`→`BoundaryRequest` to relocate toward the **most attractive
  reachable location**, scored over the **existing bounded economy values** as
  *highest `supply` · lowest `disruption` · lowest `local_security`*. `local_security` is the
  patrol-influence proxy (patrol presence raises it where it sits), so this third term makes the pirate
  **prefer systems with the least patrol influence** — it evades the patrol rather than merely chasing
  supply. No new field, no new substrate: it is an argmax over GPU-resident fields, same posture as SEAD
  (not a CPU planner).
- **Emergent cat-and-mouse (design intent, not scripted).** Because the patrol relocates toward
  depleted/contested supply and raises `local_security` where it sits, while the pirate scores *against*
  `local_security` and *toward* supply, a pursuit/evasion movement pattern is expected to **emerge** from
  the two independent GPU-resident threshold rules — neither mover is given the other's plan or an
  explicit chase/flee script. The dynamic is observed, not authored.
- **Patrol** reduces `disruption` per tick at its current location; when its current location's supply
  is depleted (or another location's disruption crosses the patrol's response threshold), it emits a
  threshold-driven `BoundaryRequest` to relocate toward the depleted/contested location.
- Both movers preserve **identity**, **owner overlay continuity** (patrol), and **local economy
  reassociation** through the relocation — all via `PRODUCTION-PATH-0080-0`.

**Pace protection (authorized, not pace-blocking).** The implementation may land in two sub-slices so
the enrichment never stalls cadence:
- **1A (minimal):** the deterministic schedule + the patrol threshold loop driving the production path.
  Sufficient to close the gate's core contract.
- **1B (the loop):** the pirate disruptor + the predator/patrol dynamic above. The pirate's
  target score is additive: a minimal pirate (highest-`supply` · lowest-`disruption`) is correct on its
  own, and the **`local_security` patrol-avoidance term is the final additive increment** that makes the
  cat-and-mouse emerge. Per design authority, that evasion term **may be deferred to the tail of 1B if it
  would otherwise impair production pace** — it is a refinement of an already-correct target rule, not a
  prerequisite.
The opening spec authorizes **both**; if pace demands, 1A ships first and 1B is the immediate
follow-on. Neither sub-slice introduces new substrate, a new shader, or a new gate.

Exact numeric constants (disruption rate, supply-drain coefficient, patrol reduction rate, thresholds)
are scenario-authored values within the bounded economy — not new architecture.

---

## 3. Schedule contract

- **Opt-in schedule registration only.** No global default registration; no workspace-wide scheduler.
- **No gameplay loop; no wall-clock / real-time loop.** Deterministic tick/step only.
- **Deterministic replay** of scheduled steps (same scenario + seed + step count → identical reports).
- **Bounded number of steps** for tests.
- **Disabled / default path has no schedule registered and no path invocation.** The production path's
  `global_default_schedule_registered` must remain `false`.
- Each step: evaluate GPU-resident SEAD thresholds on the current scenario state → emit **zero or one**
  `BoundaryRequest` per mover per step → route it into `run_production_path_0080_0` → record a
  deterministic per-step report.

---

## 4. Implementation result and remaining slice

The 1A implementation PR adds:
- add a `DEFAULT-SCHEDULE-0080-0` **opt-in** schedule surface for Local Patrol Economy;
- call `run_production_path_0080_0` from deterministic scheduled steps;
- advance patrol-side scenario state with changing `disruption`;
- emit zero or one patrol `BoundaryRequest` per step depending on threshold state;
- preserve identity, owner overlay continuity, and economy reassociation on relocation;
- record deterministic per-step reports.

The 1B pirate driver remains future/not implemented.

**WGSL discipline (binding for this slice).** Pirate/patrol per-tick arithmetic and the target-attraction
score must be expressed over the **existing `EvalEML` opcode set** (invariants row 194). New shader text
is a **stop-and-escalate**, not an implementation choice — there is no admission path for semantic/raw
WGSL here, and the bounded arithmetic (add/subtract/scale + argmax over `supply`/`disruption`/`local_security`)
is already within the generic interpreter's reach.

It **must not**:
- register a global default schedule;
- generalize to all scenarios / become a general mobility runtime;
- become gameplay;
- introduce semantic/raw WGSL;
- introduce a CPU planner / urgency / commitment emission, or externally-scripted movement;
- introduce hard currency, markets, trade, `ai_budget`, nested Resource Flow, multi-faction **economy**,
  or Hybrid-Strata/faction-index scaling;
- alter `simthing-spec` for ClauseThing or open ClauseThing/L3.

---

## 5. Required tests

`default_schedule_0080_0_explicit_opt_in_only`,
`default_schedule_0080_0_default_path_has_no_schedule`,
`default_schedule_0080_0_no_global_default_schedule`,
`default_schedule_0080_0_runs_local_patrol_economy_steps`,
`default_schedule_0080_0_threshold_false_emits_no_boundary_request`,
`default_schedule_0080_0_threshold_true_emits_boundary_request`,
`default_schedule_0080_0_routes_boundary_request_to_production_path`,
`default_schedule_0080_0_no_cpu_planner_or_external_move_script`,
`default_schedule_0080_0_preserves_identity_owner_overlay_economy`,
`default_schedule_0080_0_bounded_local_economy_only`,
`default_schedule_0080_0_replay_deterministic`,
`default_schedule_0080_0_rejects_gameplay_surface`,
`default_schedule_0080_0_rejects_semantic_or_raw_wgsl`,
`default_schedule_0080_0_rejects_hard_currency_markets_trade_aibudget`,
`default_schedule_0080_0_rejects_nested_resource_flow`,
`default_schedule_0080_0_rejects_clausething_dependency`,
`default_schedule_0080_0_docs_status_matches_gate`.

The 1A tests above are implemented in
[`../tests/phase_default_schedule_0080_0_impl_1a_results.md`](../tests/phase_default_schedule_0080_0_impl_1a_results.md).

**Pirate/loop sub-slice (1B) future tests, named but not implemented:**
`default_schedule_0080_0_pirate_raises_disruption_and_drains_supply_per_tick`,
`default_schedule_0080_0_pirate_relocates_when_disruption_ge_half_supply`,
`default_schedule_0080_0_patrol_reduces_disruption_and_relocates_to_depleted_supply`,
`default_schedule_0080_0_pirate_is_second_identity_not_second_economy_owner`,
`default_schedule_0080_0_predator_patrol_loop_replay_deterministic`,
`default_schedule_0080_0_pirate_prefers_low_patrol_influence_high_supply_target`,
`default_schedule_0080_0_cat_and_mouse_pattern_emerges_deterministically`.

---

## 6. Stop conditions

Stop (do not implement under this gate) if it would require: a global default schedule; gameplay
UI/loop; semantic/raw WGSL; CPU planner / urgency / commitment emission; externally-scripted patrol or
pirate movement; hard currency; markets/trade/`ai_budget`; nested Resource Flow; multi-faction
**economy**; capture-as-reparenting; owner-entity as spatial parent; ClauseThing implementation;
`simthing-spec` alteration for ClauseThing; invariant edits; passive proof wrappers; a general
production mobility runtime; or reopening any closed ladder.

---

## 7. Exit criteria (this opening PR)

- [x] Opening spec exists.
- [x] Scope is Local Patrol Economy only (patrol + bounded pirate enrichment; one economic owner).
- [x] 1A schedule implementation is named and implemented.
- [x] 1B pirate loop remains named but not implemented.
- [x] Production track marks `DEFAULT-SCHEDULE-0080-0` 1A as implemented/pass.
- [x] `PRODUCTION-PATH-0080-0` remains IMPLEMENTED / PASS.
- [x] Gameplay and semantic WGSL remain closed.
- [x] Mapping guidance + worklog updated.
- [x] No code changed.

---

## 8. Pointers
- Active constitution: [`../design_0_0_8_0.md`](../design_0_0_8_0.md)
- Production track: [`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md)
- Scenario packet: [`../scenarios/scenario_0080_0_admission_packet.md`](../scenarios/scenario_0080_0_admission_packet.md)
- Production path spec: [`production_path_0080_0_opening_spec.md`](production_path_0080_0_opening_spec.md)
- Production path impl report: [`../tests/phase_production_path_0080_0_impl_results.md`](../tests/phase_production_path_0080_0_impl_results.md)
- Binding rules: [`../invariants.md`](../invariants.md)
- Visibility report: [`../tests/phase_default_schedule_0080_0_opening_spec_results.md`](../tests/phase_default_schedule_0080_0_opening_spec_results.md)
