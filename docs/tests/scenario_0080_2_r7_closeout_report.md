# SCENARIO-0080-2-R7 — Dress Rehearsal Closeout & Claim-Boundary Report

> **⚠ REOPENED (2026-06-04, design authority).** This docs-only closeout was **premature**. The ladder's
> intended culmination — spec §8.1: *observe which emergent behaviors actually appear over a run* — was
> never given an implementation rung; R1–R6B are single-pass mechanism fixtures and there was no rung that
> ticks the assembled session. A new rung **`R6C — Integrated multi-tick run`** is inserted before R7 to
> assemble the rungs into one ticking session with feedback and actually run it (~100 ticks). **R7 will
> re-close only after R6C produces the integrated-run trace**, replacing the interim "not yet emerged"
> finding (§3) with run evidence. **Everything below remains valid as the mechanism-chain claim boundary**
> — the only correction is that "emergence not observed" was a *missing rung*, not a final verdict.
> Opening spec: [`../scenarios/scenario_0080_2_r6c_integrated_run_opening_spec.md`](../scenarios/scenario_0080_2_r6c_integrated_run_opening_spec.md).

**Verdict:** PASS *as mechanism-chain claim boundary* (closeout REOPENED pending R6C integrated run)
**Date:** 2026-06-04
**Gate:** R7 — CLOSE + closeout integrity + claim boundary
**Authority:** Opus design authority (SCENARIO-0080-2-R7-REVIEW-0; reopened same day)
**Scope:** Docs-only. No code behavior change, no invariant edit.

---

## 1. Ruling

**A — `SCENARIO-0080-2` closes as a vertical *proof* after this report.**

The full R1→R6B chain has landed; each rung consumes the prior rung's **real artifact** (pinned
checksums), not a re-derived stand-in. There is no missing capability for a vertical proof and no
overclaim that cannot be bounded in this report. The R4 spatial tie-breaker is handled by wording (§5),
not a code correction.

This is a **claim-boundary closeout**, not another acceptance ritual and not a new implementation gate.

---

## 2. Strongest accurate claim

> **`SCENARIO-0080-2` proves a vertical SimThing *slice*, mechanism-by-mechanism and consumption-proven:**
> occupant-produced disruption field (R1) → recursive owner-masked economy with blockade/divert (R2) →
> capability mask-down overlays (R3) → SEAD composite-field consumption with `GradientXY` + exact-sqrt
> threshold (R4) → movement via `Threshold`/`EmitEvent`→`BoundaryRequest`→REENROLL on the mobility
> substrate (R5) → local combat as adversarial Resource Flow with emission-band ship attrition (R6/R6A)
> → ship production as threshold emission into fleet-cohort reinforcement/fusion (R6B).
> **Every stage reads the prior stage's real emitted artifact. No bespoke combat/economy/movement
> subsystem was introduced — all of it compiles to SimThings + overlays + `AccumulatorOp`.**

### Required qualifiers (must travel with the claim)

- It is a **vertical slice / chain**, *not* a self-sustaining multi-tick **loop**. The rungs are
  single-pass fixtures wired in series; the closed strategic loop (§3 items 12–13) is **not** demonstrated.
- **Single galactic tier**, 13 systems on the 20×20 canonical field, **opt-in / default-off**, verified
  against the **CPU oracle** (the determinism reference for the GPU path; GPU residency remains the target,
  §6). System→planet recursion remains a named build fork (not proven depth).
- "Pathfinding" is **not** solved and was never solved earlier: movement is **greedy per-step local SEAD
  gradient reads**, never route search or lookahead (`multi_step_pathfinding` stays rejected).
- Combat is a **Resource-Flow proof** with fixture-staged co-location, verified via the CPU oracle — **not**
  production-ready; its GPU-resident execution is a follow-on measurement, not yet run here (§6).

The candidate claim in the review prompt is **accurate but slightly too strong on one word**: "loop" →
"slice/chain". With that and the qualifiers above, it is authorized.

---

## 3. Emergence classification (Opus ruling — R7 binding)

Axis: **Emerged** = arises from substrate dynamics unaided; **Partially emerged** = real mechanism runs
on real upstream data but is staged/assisted in one key way; **Fixture-proven only** = mechanism proven
on staged inputs; **Not yet emerged** = not demonstrated; **Parked** = out of scope by design.

| # | Item | Classification | Basis |
|---|---|---|---|
| 1 | Pirate disruption hotspot formation | **Fixture-proven only** | One seeded saturated hotspot (10 pirate ships → 100.0 at cell 284); no dynamic formation across ticks |
| 2 | Patrol suppression | **Fixture-proven only** | Suppression term proven in the recurrence (2 patrols vs 1 pirate floors at 0); no dynamic contest over ticks |
| 3 | Economic consequence of disruption | **Fixture-proven only** | Blockade gate + diverted production proven at one cell; not emergent swings over time |
| 4 | Blockade / divert behavior | **Fixture-proven only** | Owner-column flip (owner→blockader) proven, no reparenting; single tick |
| 5 | Faction capability differences | **Fixture-proven only** (with real downstream consumer) | R3 overlays resolved + masked down; consumed by R6 combat modifier bps (Terran 10500 / Pirate 11500) |
| 6 | Field consumption by movers | **Partially emerged** | R4 genuinely computes `GradientXY` + exact-sqrt magnitude from the composite field; assisted by the spatial tie-breaker in the sparse field (§5) |
| 7 | Actual movement from SEAD threshold/event | **Partially emerged** | R5 movement is materialized from R4 `StepOpportunity`→event→`BoundaryRequest`→REENROLL (not scripted); direction leans on the tie-breaker in the canonical field |
| 8 | Combat from hostile co-location | **Fixture-proven only** | Co-location is fixture-arranged (Terran placed at 284), not produced by R5 movers converging |
| 9 | Fleet attrition as cohort ship loss | **Partially emerged** | Emission-band attrition (500→5 killed) is a true Resource-Flow consequence once combat runs; the engagement that triggers it is staged |
| 10 | Ship production reinforcing fleets | **Partially emerged** | Reinforcement of the real R5 fission fleet from real R2 production is a genuine chain; fusion/birth use seeded fixture cohorts |
| 11 | Friendly fleet fusion / cohort compaction | **Fixture-proven only** | Fusion cohorts (7+7→14) seeded at fixture cell 42 |
| 12 | Self-sustaining pirate pressure loop | **Not yet emerged** | No multi-tick closed loop; single-pass fixtures in series |
| 13 | Open-ended AI behavior | **Not yet emerged** (by design) | No CPU planner / no AI; deterministic fixtures. Emergence is intended from flow, not a planner |
| 14 | Modder-facing expressibility | **Partially emerged** | Substrate uniformity is genuinely demonstrated (combat, production, disruption all as `AccumulatorOp`/overlays, no bespoke subsystem); the authoring surface (ClauseScript L3) is parked |

**Headline finding (honest non-emergence):** the scenario's marquee questions — raiding waves, the
race equilibrium, interception in transit, front/standoff formation (spec §8.1) — **did not emerge**,
because the rehearsal is a series of single-pass fixtures over a sparse one-hotspot field, not a
multi-tick simulation. This is a model/parameter gap to be opened by a future multi-tick scenario, **not**
a defect of the rungs. Each rung proved its **mechanism**; the **gameplay** is not yet exercised.

---

## 4. Numeric-only → consumption-proven reconciliation

| Prior claim | Status now |
|---|---|
| First-slice runtime R1/R2/R3 heatmap "accepted (numeric)" | **Consumption-proven**: R1 heatmap is consumed by R2 (blockade/divert reads `final_disruption`), R3, and R4 |
| FrontierV1 "SEAD route" (registers descriptors only; never consumed a field to act) | **Consumption-proven via R4/R5**: SEAD now consumes the composite field → exact threshold → real movement. The earlier "SEAD route" remains **numeric-only / superseded** for the field→action loop |
| Mapping first-slice heatmap (hand-seeded, parity-only) | **Consumption-proven** as the disruption field consumed downstream; still **not** a gameplay-produced heatmap |
| "pathfinding solved" (any pre-R4/R5 wording) | **Downgraded**: never solved; movement is greedy local SEAD steps, no route search — both before and after R4/R5 |
| "economy generalized" | **Downgraded**: single galactic tier, 13 systems, single-tick fixture; nested depth parked |
| "combat production-ready" | **Downgraded**: Resource-Flow proof, fixture-staged co-location, CPU-oracle primary |

**Remaining parked (correctly):** multi-tick closed loop; system→planet nested depth (A-0 beyond D=2);
sparse-residency scheduler / M-4A; ClauseScript L3 authoring; hard currency; GPU-resident combat/production
kernels; AI planner.

---

## 5. R4 spatial-bias note (claim-boundary note)

R4 uses a small deterministic spatial bias (`cell_index * 0.01 + x * 0.001`) to keep `GradientXY`
non-degenerate in the sparse canonical field.

- It **must not** be presented as emergent behavior.
- It **is** a **fixture tie-breaker** that disambiguates an otherwise flat gradient.
- A future richer/multi-hotspot field **must drop it or prove it dominated** by the real field signal.
- The R5 report already records: *"R7 emergence narrative must not attribute movement to the R4 canonical
  tie-breaker bias."* This is honored here.

**This is sufficient as documentation. No pre-R7 code correction is required.**

---

## 6. GPU / substrate residency — evidence status (not a guardrail)

> **This is an evidence statement, not a binding guardrail.** Guardrails live at admission/authoring
> (`simthing-spec` / CLAUSE-SPEC) and the runtime last line per `invariants.md` — a test report does not
> mint binding constraints. And per the founding premise (design §0/§0.1), **GPU residency is the target**:
> the whole point is to keep as much calculation as possible resident on the GPU as uniform automata. So
> "GPU-shaped" below is **conformance to that target — a positive result**, not a claim to be suppressed.
> The only honest limitation is a **measurement gap**: these rungs were verified against the CPU oracle and
> have **not yet been executed on the GPU** in this vertical.

- **Conformance (positive):** R1–R6B are written as row / mask / threshold / emission-band operations —
  exactly the shape that lowers to GPU-resident `AccumulatorOp` automata. They carry **no** bespoke
  CPU-only control flow that would block GPU residency. This is the rehearsal doing what §0 asks.
- **Verification method:** the **CPU oracle is the determinism reference** the GPU path is checked against
  (it is how every accepted GPU rung in this project is validated — CPU oracle ↔ GPU bit-exact). R4 runs
  `GradientXY` via `simthing-gpu`'s `cpu_horizon`; exact magnitude via Candidate-F (`sqrt_cr_f_bits`,
  artifact `e2e9e27601ee2e13`). Using the oracle is **correct practice**, not a posture against GPU.
- **Measurement gap (the only limitation):** a GPU *execution* diagnostic has **not yet been run** for
  R4/R6/R6B in this vertical. So the accurate phrasing is **"GPU-conformant; GPU execution not yet
  measured here"** — not "validated on GPU" and not "CPU is the target." Closing this gap is a follow-on
  measurement, not a redesign.
- **Already measured on discrete GPU:** **ATLAS-BATCH-0 STORE-GPU** — integer bit-exact, validated
  cross-adapter on the RTX 4080 ladder. That is the standing proof that this substrate's masked-reduction
  shape executes GPU-resident and bit-exact; the rehearsal rungs are the same shape awaiting the same
  measurement.

---

## 7. Per-rung consumption-proof status

| Rung | Checksum | Consumes | Status |
|---|---|---|---|
| R1 | `17de0080304b3da7` | ATLAS-BATCH-0 layout | consumption-proven (downstream by R2/R3/R4) |
| R2 | `4fe0590589ddd975` | R1 disruption | consumption-proven |
| R3 | `28afb4a204d101d2` | R1, R2 | consumption-proven (combat modifier in R6) |
| R4 | `f0acbe2ccb98badb` | R1, R2, R3 | consumption-proven (movement in R5) |
| R5 | `5308a1eb1b7ae5fb` | R1–R4 | consumption-proven (membership in R6) |
| R6/R6A | `68b5c8e2e8f3b801` | R1–R5 | consumption-proven (cohorts in R6B) |
| R6B | `f9d334bd21ed5097` | R5, R6A | consumption-proven (overrides re-enter R6 combat) |

---

## 8. Test-command rollup (foreground, plain `cargo test`)

Re-verified on `master` @ `c7de646` for this closeout:

```text
cargo test -p simthing-driver --test dress_rehearsal_r6b_ship_cohort_reinforcement  → 24 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r6_combat_hp_damage            → 25 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r5_movement_reenroll           → 17 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r4_sead_field_consumption      → 16 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r3_capability_mask_down        → 13 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r2_recursive_allocation        → 13 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r1_disruption_heatmap          → 34 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store            → 11 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu        → 10 passed; 0 failed
cargo test -p simthing-spec   --test mobility_reenroll0_substrate                   → 16 passed; 0 failed
cargo test -p simthing-spec   --test mobility_runtime0_composition                  → 23 passed; 0 failed
cargo check --workspace                                                             → PASS (pre-existing warnings only)
```

**Skipped / not run honestly:** no GPU diagnostic for R4–R6B (CPU-oracle primary by design); the only
discrete-GPU evidence is ATLAS STORE-GPU (10/10). No multi-tick simulation run exists — by scope.

---

## 9. Closeout disposition

- `SCENARIO-0080-2` is **CLOSED as a vertical proof** (mechanism chain consumption-proven; gameplay
  emergence explicitly *not yet* demonstrated and recorded as a finding).
- Human/modder-facing companion: [`../gameplay/scenario_0080_2_pirate_gradient_pathfinding_results.md`](../gameplay/scenario_0080_2_pirate_gradient_pathfinding_results.md).
- No invariant edit. No code behavior change. No new rung opened.
