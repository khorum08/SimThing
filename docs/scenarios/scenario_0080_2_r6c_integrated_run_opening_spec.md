# SCENARIO-0080-2-R6C — Integrated Multi-Tick Run (Opening Spec)

> **Status: OPEN (2026-06-04, Opus design authority).** This is the **culmination rung** of the
> `SCENARIO-0080-2` dress rehearsal: the one that was implied by spec §8.1 but never given an
> implementation rung. R1–R6B proved each **mechanism** in isolation against the prior rung's frozen
> artifact; **R6C assembles them into one ticking session with feedback and actually runs it.** Only after
> R6C produces a trace does **R7** re-close and report which §8.1 behaviors emerged.
>
> **Harness/authority:** `design_0_0_8_0.md` §0 (decisions emerge as threshold crossings, no CPU planner),
> §0.2 (reduce-up/disburse-down), §0.3 (all conflict is Resource Flow); `invariants.md` (Scenario Proof);
> dress-rehearsal spec `scenario_0080_2_dress_rehearsal_spec.md` §4–§9; FrontierV2-0..4 closed-loop
> feedback pattern (the precedent for multi-tick feedback at fixture level).

---

## 1. Why this rung exists

The R1–R6B rungs are **single-pass**: each `run_*(input) -> report` consumes the previous rung's
**static** artifact and emits an immutable report. Nothing carries world-state forward across ticks, so
the marquee questions of the scenario — *does pirate overmatch hold or does Terran out-build it? do raids
chain across systems? do fleets intercept in transit?* — **cannot be observed**, because there is no run.

R6C closes that gap. It does **not** add a new mechanism; it **wires the proven mechanisms into a loop**.

### Non-goals (explicitly out of scope, stay parked)
- No default `SimSession` pass-graph wiring; R6C is an **opt-in/default-off** driver helper.
- No CPU planner / no scripted behavior. Every decision still comes from `Threshold`→`EmitEvent` over the
  masked field (§0). If a behavior needs a planner to appear, it does **not** appear — that is a finding.
- No new `AccumulatorOp`, no semantic WGSL, no new invariant.
- No nested system→planet depth (stays the named build fork); single galactic tier.
- No real-time loop / UI / player control.
- CPU-oracle primary. GPU residency is **shape-only**; do not claim GPU kernels.

---

## 2. The four things R6C must build (the blockers identified at closeout)

1. **One shared mutable session-state.** A single `DressRehearsalR6cWorld` seeded once from the canonical
   ATLAS-BATCH-0 layout, holding the *live* mutable state every rung reads and writes:
   - per-cell `disruption` / `location_status`;
   - per-faction stockpiles + per-system production/labor;
   - the live set of **fleet cohorts** (id, owner, cell, `num_ships`, hp/ship, dmg/ship);
   - per-system blockade/divert owner-column state.
   Rungs become **pure transforms over this world**, not artifact emitters.

2. **Feedback / write-back (close the loop).** At the end of each tick the mutated world becomes the next
   tick's input. Concretely: R5's REENROLL'd positions feed R1's next disruption pass; R6 casualties and
   R6B's new/fused ships update the cohort set; R2 stockpile/divert carries forward. **Combat co-location
   is produced by R5 movers converging — never hand-placed.**

3. **A tick driver.** `for tick in 0..N { r1; r2; r3; r4; r5(multi-step by speed); r6; r6b; write_back }`
   with `N = 100` canonical. Movement is **≤ speed greedy SEAD sub-steps per tick** (Pirate 3 / Patrol 2),
   each sub-step a fresh local gradient read + threshold (spec §8) — re-enroll → re-evaluate → step, or
   stop early. Per-sub-step re-enrollment is what enables **interception in an intermediate cell**.

4. **A non-degenerate field.** The canonical field has one saturated hotspot; over many ticks a flat
   gradient is meaningless. R6C must make the field carry direction:
   - seed the canonical 13-system / starting-fleet layout from spec §2/§9 (Pirate 10 ships, Terran 3
     patrols, ≥2–4 cell Terran spacing, Pirate within 1 cell of a Terran system) so that **multiple**
     competing gradients exist as the sim evolves;
   - **drop the R4 fixture tie-breaker** (`cell_index*0.01 + x*0.001`) **or prove it is dominated** by the
     real field signal (assert real-signal magnitude ≫ tie-breaker at every committed step). Movement
     direction must be attributable to the field, not the tie-breaker. Record which choice was made.

---

## 3. Per-tick order of operations (one tick)

```
for tick in 0..100:
  1. R1  disruption recurrence from CURRENT fleet positions → disruption/location_status
  2. R2  labor→production; reduce-up to faction stockpiles; disburse-down to deficits;
         blockade(≥100) gates + diverts owner-column at disrupted systems
  3. R3  resolve capability overlays; mask down by owner (read-side)
  4. R4  each fleet reads composite field at its cell → GradientXY → exact-sqrt magnitude → threshold
  5. R5  for each fleet: up to `speed` greedy SEAD sub-steps (REENROLL each sub-step);
         starport production ≥100 → fission a new Fleet at the starport cell
  6. R6  for every cell with hostile co-located cohorts: adversarial Resource-Flow combat,
         emission-band attrition, remove cohort at num_ships==0 (Departure)
  7. R6B starport/planet production threshold → ship_count_delta → reinforce compatible cohort,
         else birth; fuse friendly co-located compatible cohorts
  8. WRITE BACK: positions, num_ships, stockpiles, disruption, divert-state → next tick
```

Determinism: fixed iteration order (cell-index, then stable cohort-id); no wall-clock; bit-stable replay.

---

## 4. Emergence observations (the R6C trace feeds R7)

R6C records a per-tick trace and, for **each** spec §8.1 behavior, a deterministic detector with the tick
it first occurred (or "not observed"):

| §8.1 behavior | Detector (deterministic, from the run trace) |
|---|---|
| Pirate raiding waves | a pirate cohort moves toward and raises disruption at successive Terran systems |
| Self-disruption migration | a cohort departs a cell it saturated (≥100) toward a lower-disruption target |
| Terran patrol redistribution | a patrol moves toward a disrupted owned system and lowers its disruption |
| Blockade-divert ownership flips | a Terran system's production owner-column flips to Pirate for ≥1 tick, then back |
| Interception / attrition | combat resolves in a cell reached **by movement this run** (not a seeded co-location) |
| **Race equilibrium (headline)** | final-tick `pirate_ships` vs `terran_ships` trajectory: which side trends up; record the curve |
| Front / standoff formation | a cell/boundary where patrol suppression ≈ pirate disruption holds across ≥K ticks |

**Honesty rule (spec §8.1):** any behavior not detected is reported as **not observed**, with the likely
cause (parameter vs model gap). R6C does not tune parameters to force a behavior; if the canonical §9
numbers don't produce it, that is the finding R7 reports.

---

## 5. Acceptance criteria (what makes R6C PASS)

R6C is PASS when:
1. A 100-tick run executes over one mutable session-state with all 8 steps wired and write-back closing
   the loop (no per-rung static fixtures inside the loop; no hand-placed combat co-location).
2. Movement is field-attributable (tie-breaker dropped or proven dominated; assertion in tests).
3. Deterministic replay: two runs produce a bit-identical trace + pinned checksum; CPU-oracle parity.
4. The §8.1 detector table is populated from the trace (each behavior: tick-first-seen or not-observed).
5. Conservation/identity hold across ticks: ships only change via combat/production; owner overlays and
   identity lanes preserved across REENROLL; no reparenting; no occupant teleport.
6. Upstream R1–R6B unit suites still green; `cargo check --workspace` clean. opt-in/default-off; no
   default wiring, planner, semantic WGSL, or invariant edit.

R6C does **not** require any specific behavior to emerge — only that the run is faithful and the trace is
honestly reported.

## 6. Deliverables
- `crates/simthing-driver/src/dress_rehearsal_r6c_integrated_run.rs` (world-state + tick driver + detectors + CPU oracle)
- `crates/simthing-driver/tests/dress_rehearsal_r6c_integrated_run.rs`
- `docs/tests/scenario_0080_2_r6c_integrated_run_report.md` (run summary, §8.1 detector table, race curve, checksum)
- track §12.5 R6C row → IMPLEMENTED/PASS; then **R7 re-close** updating its §3 with run evidence.

## 7. Stop conditions (escalate, do not improvise)
- If faithful wiring requires a CPU planner or a new `AccumulatorOp`/invariant to make the loop run → stop, escalate (the mechanism set may be insufficient — itself a finding).
- If determinism cannot hold across ticks without nondeterministic ordering → stop, escalate.
- If forcing the field non-degenerate requires changing pinned §9 economy numbers → stop, escalate (parameter ratification is a design-authority call, not an implementation one).
