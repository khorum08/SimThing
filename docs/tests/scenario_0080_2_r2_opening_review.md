# SCENARIO-0080-2-R2-OPEN-0 — R2 Opening Review

**Verdict:** **R2 OPEN / AUTHORED** (Option A — full §12.5 scope, with rung-identity boundaries)
**Gate:** `R2` — Recursive allocation + faction economy + blockade/divert (single galactic tier)
**Date:** 2026-06-04
**Authoring authority:** Opus (design authority)
**Method:** docs/design-only gate; no code; grounded in the accepted R1 output + existing economy machinery.

## 1. Verdict

R2 is **opened** as a docs/design gate. It opens at the **full §12.5 R2 unit** — the recursive allocation
loop (reduce-up + disburse-down), the faction economy (production, subsidiarity clearinghouse), **and** the
blockade/divert mechanic — *not* a narrowed reduce-up-only slice. Spec:
`docs/scenarios/scenario_0080_2_r2_recursive_reduce_opening_spec.md`.

## 2. Files reviewed

- `docs/design_0_0_8_0.md` (§0 transient constitution: §0.2 recursive allocation, §0.3 all-conflict-is-resource-flow)
- `docs/design_0_0_8_0_consumer_pulled_production_track.md` (§12.5 R2 row + retirement map)
- `docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md` (§4 arenas, §6 blockade/divert, §7 clearinghouse, §9 numbers)
- `docs/scenarios/scenario_0080_2_r1_disruption_heatmap_opening_spec.md`,
  `docs/tests/scenario_0080_2_r1_disruption_heatmap_report.md`,
  `docs/tests/scenario_0080_2_r1_acceptance_review.md`
- `crates/simthing-driver/src/dress_rehearsal_r1_disruption_heatmap.rs` (R1 output shape = R2 input contract)
- `crates/simthing-driver/src/econ_scale_0080_0.rs` (bounded faction index + subsidiarity clearinghouse — ECON-SCALE reuse)
- `crates/simthing-core/src/accumulator_op_builder.rs` (recipe machinery: ConjunctiveCrossing/CrossingFormula/SubtractFromAllInputs)
- `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_{gen,loc,store}.rs` (+ tests) — layout/owner-masked storage
- `docs/tests/scenario_0080_2_atlas_batch_0_close_report.md`, `docs/workshop/mapping_current_guidance.md`, `docs/worklog.md`
- `docs/invariants.md` — binding rules only; **not edited**.

## 3. Completed gates (context)

- `ATLAS-BATCH-0` — CLOSED / PASS (pre-rehearsal atlas prerequisite).
- `R1` — Disruption heatmap / EC1 — **ACCEPTED / CLOSED / IMPLEMENTED-PASS**.

## 4. Decision rationale

**Why Option A (open), and why full scope.** R1 is cleanly accepted and produces exactly R2's input
(channel/owner-partitioned cell rows + per-system disruption + owner per system). The next true vertical
step is the economy loop that consumes that field. On scope, two corrections to the proposed "narrowing"
were made under design authority:

- **Reduce-up and disburse-down are not split.** §0.2 recursive allocation defines them as **one**
  behavior (reduce-up + broadcast-down). Opening "reduce-up only, defer disburse" would fragment the very
  principle the rung exists to prove — that is drift, not scoping. R2 proves the whole loop.
- **The 21 negative-assertion tests were dropped.** Tests like `r2_no_invariant_edit`,
  `r2_no_clausething_dependency`, `r2_no_hard_currency`, `r2_no_m4a` assert absences of machinery R2 never
  approaches; they prove nothing about the capability and are exactly the self-feeding hygiene
  guardrailing removed from the constitution. Out-of-scope machinery is handled as **rung-identity
  boundaries** (§5 of the spec) plus a small set of **load-bearing identity tests** (the ones that
  distinguish R2 from R5/R6 and enforce the §0.0 column-flip-not-reparent conformance), not as a wall of
  negative tests.

**What scoping is real (kept out).** R3 techtree mask-down, R5 movement/REENROLL/`BoundaryRequest`, R6
combat HP/Damage, and the deeper system→planet tier recursion (needs interior 10×10 tiles R1 did not
materialize) are genuinely different machinery / a real build fork — correctly out of R2.

## 5. R2 scope (as opened)

- **M1** production economy: pop `IntrinsicFlow` labor → factory recipe (`10 labor → 1 production`,
  `SubtractFromAllInputs`) — existing op.
- **M2** recursive allocation: OWNER-masked reduce-up to per-faction stockpiles (Terran/Pirate never
  merged) **and** subsidiarity disburse-down to deficit systems (ECON-SCALE / FlatStarResourceFlow reuse;
  A-0 nested RF in scope here).
- **M3** blockade/divert: read R1 `final_disruption`; `Threshold{≥100}` gates a system's outflow; divert
  flips the production owner-column to the blockader — a **column flip, not reparenting, no occupant moved**.
- Consumes R1's accepted output; opt-in/default-off; deterministic; CPU oracle; no GPU required.

## 6. Implementation recipient

- **Cursor / Codex5.5max** — production implementation agent. Implementation PR only after this opening
  merges; scope = spec §4 within §5 boundaries.

## 7. Required future tests (capability + identity)

`r2_opening_status_matches_track` · `r2_consumes_accepted_r1_heatmap` ·
`r2_factory_recipe_converts_labor_to_production` ·
`r2_production_reduces_up_to_faction_stockpile_owner_masked` ·
`r2_faction_disburses_surplus_to_deficit_system` · `r2_blockade_threshold_gates_outflow_at_100` ·
`r2_divert_flips_production_owner_column_to_blockader` · `r2_divert_is_owner_column_flip_not_reparenting` ·
`r2_no_occupant_relocated_and_no_boundary_request` · `r2_no_combat_resolution` ·
`r2_single_tier_no_interior_subtile_materialization` · `r2_deterministic_replay_and_cpu_oracle_parity` ·
`r2_opt_in_default_off`.

## 8. Stop conditions / rung-identity boundaries (spec §5)

No occupant movement / no `BoundaryRequest` / no REENROLL (vs R5); divert = owner-column flip, **not**
reparenting (§0.0); no techtree mask-down (R3); no combat HP/Damage (R6); single galactic tier only (no
interior sub-tiles); reuse existing ops — no new op/shader/WGSL; opt-in/default-off; no CPU planner; no
hard currency/markets/trade/`ai_budget`, no ClauseThing/L3, no UI/realtime. Cross one → stop, return to Opus.

## 9. Confirmation

- **No code changed** (docs/design-only PR). Manual diff review sufficient; no Rust tests required.
- **No `docs/invariants.md` edit.**
- No default `SimSession` wiring · no SEAD movement · no `GradientXY` consumption · no R3/R4/R5/R6 · no
  REENROLL · no M-4A · no new shader/WGSL/GPU kernel · no f32 bit-exact GPU claim · no hard
  currency/markets/trade/`ai_budget` · no nested-RF *beyond* the disburse-down already named in R2 scope ·
  no ClauseThing — **all confirmed for this opening PR.**
- No scratch/tmp/log/`target/` outputs committed.
- R1 remains ACCEPTED / CLOSED. R3–R7 remain unopened.

## 10. §0.5 self-check

Opening-gate authoring only — no implementation, no code, no invariant edit, no scope beyond the named R2
rung, no shader/math/tolerance change, no default session wiring. Records the decision and authorizes one
bounded R2 implementation rung.
