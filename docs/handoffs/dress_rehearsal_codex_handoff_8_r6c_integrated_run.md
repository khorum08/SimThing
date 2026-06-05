# Dress Rehearsal Codex Handoff 8 — R6C Integrated Multi-Tick Run

> **Paste-ready implementation prompt for Cursor / Codex5.5max.** Opus (design authority) has opened the
> gate and authored the spec; this handoff is the implementation instruction. Implement, push, and merge
> one focused PR.

---

Intended recipient: Cursor / Codex5.5max
Role: production implementation agent

## SCENARIO-0080-2-R6C-IMPL-0 — integrated multi-tick run (the dress-rehearsal culmination)

Repo: khorum08/SimThing
Branch from latest `origin/master` (after PR #524). Open one focused implementation PR.

### Operating rules (binding)
- **PowerShell fail mode:** do **not** pipe bare `2>&1` into `cargo`. Run final verification in the
  **foreground** with a normal blocking shell (long enough for compile + test). Diagnostics may stream,
  but the final gate run is foreground.
- **opt-in / default-off.** No default `SimSession` pass-graph wiring. No global schedule.
- **No CPU planner. No scripted behavior.** Every decision is `Threshold`→`EmitEvent` over the masked
  field (design §0). If a behavior needs a planner to appear, it does not appear — report it as a finding.
- **No new `AccumulatorOp`, no semantic WGSL, no invariant edit** (`docs/invariants.md` is off-limits).
- **Verify against the CPU oracle** (the determinism reference for the GPU path). GPU residency is the
  **target** (design §0): write the rung GPU-shaped (row/mask/threshold/emission-band, no CPU-only control
  flow) so GPU execution is a later *measurement*, not a rewrite. A GPU execution diagnostic is **not**
  required for R6C PASS; report "GPU-conformant; GPU execution not yet measured," never "validated on GPU."
- Single galactic tier (no system→planet nesting). No UI/realtime/hard-currency/ClauseThing.
- If any of the above must be violated to make the loop run, **STOP and escalate** — that is itself a
  finding about mechanism sufficiency.

### Read first
1. `docs/scenarios/scenario_0080_2_r6c_integrated_run_opening_spec.md` — **the authoritative spec for this
   rung** (loop order, feedback, field, detectors, acceptance, stop conditions). Implement to it.
2. `docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md` §2 (topology/placement), §4–§9 (arenas,
   movement model, §8.1 anticipated emergent behaviors, §9 pinned economy numbers).
3. `docs/design_0_0_8_0.md` §0/§0.2/§0.3 (emergence-from-threshold, reduce-up/disburse-down, all-conflict-
   is-Resource-Flow).
4. The existing rung sources you are composing:
   `crates/simthing-driver/src/dress_rehearsal_r1_disruption_heatmap.rs`,
   `…_r2_recursive_allocation.rs`, `…_r3_capability_mask_down.rs`,
   `…_r4_sead_field_consumption.rs`, `…_r5_movement_reenroll.rs`,
   `…_r6_combat_hp_damage.rs`, `…_r6b_ship_cohort_reinforcement.rs`.
5. FrontierV2 closed-loop precedent: `docs/tests/phase_m_frontier_v2_0_closed_loop_consumer_results.md`
   (and v2_1..v2_4) — the multi-tick feedback pattern, at fixture level.

### What to build
Compose the **existing** rung logic into one ticking session. Do not re-derive mechanisms; reuse the rung
functions/structs, refactoring shared logic into pure transforms over a single world-state where needed.

1. **`crates/simthing-driver/src/dress_rehearsal_r6c_integrated_run.rs`**
   - `DressRehearsalR6cWorld`: the single mutable session-state (per-cell disruption/location_status;
     per-faction stockpiles; per-system production/labor; live fleet cohorts {id, owner, cell, num_ships,
     hp_per_ship, dmg_per_ship}; per-system blockade/divert owner-column state). Seed once from the
     canonical ATLAS-BATCH-0 §2/§9 layout (Pirate 10 ships, Terran 3 patrols, Terran spacing ≥2–4, Pirate
     within 1 cell of a Terran system).
   - `run_dress_rehearsal_r6c_integrated_run(input) -> DressRehearsalR6cReport` with the per-tick order in
     spec §3, `N = 100` canonical, write-back closing the loop each tick. Movement = ≤speed greedy SEAD
     sub-steps with per-sub-step REENROLL (Pirate 3 / Patrol 2). Combat runs only on co-locations produced
     by movement — **never hand-place a co-location**.
   - **Field non-degeneracy:** drop the R4 tie-breaker (`cell_index*0.01 + x*0.001`) **or** assert it is
     dominated by the real signal at every committed step; record which. Movement direction must be
     field-attributable.
   - **§8.1 detectors:** deterministic per-behavior detector returning first-occurrence tick or
     not-observed (table in spec §4), plus the race-equilibrium ship-count curve.
   - CPU oracle + deterministic replay (bit-identical trace + pinned checksum).
2. **`crates/simthing-driver/tests/dress_rehearsal_r6c_integrated_run.rs`** — capability tests:
   single mutable world (no per-rung static fixture inside the loop); write-back actually changes next-tick
   inputs; movement is field-attributable (tie-breaker dropped/dominated assertion); combat co-location is
   movement-produced; conservation (ships change only via combat/production); identity/owner preserved
   across REENROLL; no reparenting/teleport; deterministic replay + checksum pin; opt-in default-off;
   each §8.1 detector exercised; upstream R1–R6B suites still green.
3. **`docs/tests/scenario_0080_2_r6c_integrated_run_report.md`** — run summary, the §8.1 detector table
   (first-seen tick / not-observed + cause), the race-equilibrium curve, identity/conservation evidence,
   checksum, exact foreground test commands + results.
4. **Re-close R7:** update `docs/tests/scenario_0080_2_r7_closeout_report.md` §3 — replace the interim
   "not yet emerged" rows with **run evidence** from R6C (emerged / partially / not-observed per the
   detectors); remove the REOPENED banner once R6C is PASS. Update the human-facing
   `docs/gameplay/scenario_0080_2_pirate_gradient_pathfinding_results.md` "what did NOT emerge" section
   with what the run actually showed.
5. **Docs:** flip track §12.5 **R6C → IMPLEMENTED/PASS** and **R7 → CLOSED/PASS**; update the top status
   note, `docs/worklog.md`, and `docs/workshop/mapping_current_guidance.md`. Export R6C from
   `crates/simthing-driver/src/lib.rs`.

### Acceptance (spec §5)
A 100-tick run over one mutable session-state with all 8 steps wired + write-back; movement
field-attributable; deterministic replay + CPU-oracle parity; §8.1 detector table populated from the
trace; conservation/identity hold; upstream suites green; `cargo check --workspace` clean. **No behavior
is required to emerge** — only that the run is faithful and the trace is honestly reported.

### Honesty rule
Do not tune the pinned §9 economy numbers to force a behavior. If the canonical numbers don't produce a
§8.1 behavior, report it as the finding (parameter vs model gap). Changing pinned numbers is a
design-authority call — **STOP and escalate** (spec §7).

### Test command discipline (PowerShell, foreground for the final gate)
```powershell
cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run
cargo test -p simthing-driver --test dress_rehearsal_r6b_ship_cohort_reinforcement
cargo test -p simthing-driver --test dress_rehearsal_r6_combat_hp_damage
cargo test -p simthing-driver --test dress_rehearsal_r5_movement_reenroll
cargo test -p simthing-driver --test dress_rehearsal_r4_sead_field_consumption
cargo test -p simthing-driver --test dress_rehearsal_r3_capability_mask_down
cargo test -p simthing-driver --test dress_rehearsal_r2_recursive_allocation
cargo test -p simthing-driver --test dress_rehearsal_r1_disruption_heatmap
cargo check --workspace
```

### Deliver
One PR titled `SCENARIO-0080-2-R6C-IMPL-0: integrated multi-tick run`, merged to `master` after green
foreground tests. Then `SCENARIO-0080-2` closes for real — mechanism chain **and** the observed-emergence
run.
