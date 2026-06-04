# SCENARIO-0080-2-R2 — Recursive Allocation + Faction Economy + Blockade/Divert Opening Spec

> **Status: OPENING SPEC / NO IMPLEMENTATION.** Design-authority gate (Opus). R1 is accepted/closed
> implemented-pass; R2 is opened here as a docs/design gate only. No code in this PR. One Cursor/Codex
> implementation rung follows, bounded to this spec.

> **Scope note (design authority, 2026-06-04).** R2 opens at the **full §12.5 R2 unit** — the recursive
> allocation loop **and** the faction economy **and** the blockade/divert mechanic — *not* a narrowed
> "reduce-up only" slice. Splitting reduce-up from disburse-down would bisect §0.2 recursive allocation
> (reduce-up + broadcast-down are **one** behavior); that fragmentation is drift, not scoping. The real
> boundaries — what makes this R2 and not R3/R5/R6 — are in §5 and are stated as **rung-identity lines**,
> not negative-assertion hygiene. R3 (techtree mask-down), R5 (movement), R6 (combat), and the deeper
> system→planet tier recursion are genuinely different machinery and stay out.

## 1. Gate and status

- **Gate:** `R2` — Recursive allocation + faction economy + blockade/divert (single galactic tier).
- **Predecessor:** `R1` — Disruption heatmap / EC1 — **ACCEPTED / CLOSED / IMPLEMENTED-PASS**
  (`docs/tests/scenario_0080_2_r1_acceptance_review.md`).
- **Verdict:** **OPEN / AUTHORED.** Implementation authorized within this spec's scope only. R3–R7 stay
  unopened. No implementation in this PR.
- **Authoring authority:** Opus. **Implementation recipient:** Cursor / Codex5.5max (§7).

> **Implementation result (2026-06-04):** `SCENARIO-0080-2-R2-IMPL-0` is **IMPLEMENTED / PASS** within
> this opening spec's scope. Evidence:
> [`docs/tests/scenario_0080_2_r2_recursive_allocation_report.md`](../tests/scenario_0080_2_r2_recursive_allocation_report.md).
> This note records the implementation result only; R3–R7 remain unopened/deferred as specified below.

## 2. Product purpose — why R2 matters

R1 proved a *local* galactic-tier field: occupant presence → a disruption heatmap over real gridcell
SimThings. R2 proves the rest of the constitution's economic core on top of that field:

- **Recursive allocation (§0.2):** a child's resource (production) **reduces up** to a parent ledger
  (faction stockpile) and the parent **disburses down** to deficit children — one behavior, two sweeps.
  This is the subsidiarity clearinghouse made concrete over real SimThings.
- **All conflict is resource flow (§0.3):** the **blockade/divert** mechanic turns the R1 disruption field
  into an economic weapon — a system pushed past `disruption ≥ 100` has its production **diverted to the
  blockading faction** by an owner-column re-route. No special-case combat code; conflict *is* the flow.
- **Owner/channel integrity at scale:** Terran and Pirate production stay distinct through the reduce-up
  (the EC-A3 OWNER masked reduction, already proven in ATLAS-BATCH-0 STORE), never blind-summed.

R2 is the moment the heatmap stops being an inert artifact and starts **moving production between
factions** — the economic loop the whole scenario turns on.

## 3. The precise question R2 must answer

> Reading the accepted R1 disruption field as input, can real SimThings run the full economic allocation
> loop — production reduces up to per-faction stockpiles (owner-masked, never merged), stockpiles disburse
> down to deficit systems, and a system crossing `disruption ≥ 100` has its production **diverted** to the
> blockading faction by an **owner-column flip (not reparenting, not movement)** — **deterministically,
> against a CPU oracle, reusing existing AccumulatorOp machinery, with no occupant ever moving and no new
> shader, op, or default-session wiring?**

## 4. Authorized scope — three coupled mechanisms, all existing machinery

### M1 — Production economy (the resource being allocated)
Per scenario §4/§9, reusing existing `AccumulatorOp`:
- **Pop cohort** emits labor via `IntrinsicFlow` (**+10 labor/tick**).
- **Factory** runs the recipe `ConjunctiveCrossing`(labor) → `CrossingFormula{unit_cost:10}` → emits **1
  production**, with `SubtractFromAllInputs` consuming 10 labor. *No new op.*
- Labor **nets locally**; production **climbs** (reduce-up, M2).

### M2 — Recursive allocation: reduce-up + disburse-down (§0.2 subsidiarity)
- **Reduce-up:** each system's production reduces up to its **faction stockpile** (Balance ledger),
  **OWNER-masked** (EC-A3 form: `EvalEML CMP_EQ`/`SELECT` owner-column + `Sum`) so Terran and Pirate
  totals never blind-sum. Reuses the `econ_scale_0080_0` bounded faction index + subsidiarity
  clearinghouse (ECON-SCALE reuse) and the `FlatStarResourceFlow` posture (A-0 nested RF is *in scope*
  here — disburse-down is the nested half).
- **Disburse-down:** the faction stockpile disburses surplus to **deficit systems** (e.g. a starport with
  unmet ship-need) — a deterministic subsidiarity sweep, **not** a planner/optimizer. One up sweep, one
  down sweep per tick.

### M3 — Blockade + divert (the §6 headline; couples R1 → economy)
- Read the **accepted R1 `final_disruption`** at each system cell.
- **Blockade:** `Threshold{≥100}` — a system at `disruption ≥ 100` is blockaded; its outbound production is
  **gated off** from its own faction's reduce-up.
- **Divert:** the blockaded system's production-outflow **owner-column flips** owner → **blockader** (the
  faction whose channel drove the disruption — pirate channel ⇒ `Pirate`), so the production sums under the
  **blockader's** stockpile. Mechanically a `Threshold`-gated owner-masked re-route of an existing flow.
- **Conformance (load-bearing):** this is the **D=2 decaying-owner posture** — an owner-**column** change,
  **never** reparenting and **never** moving a SimThing (the canonical §0.0 violation R2 must avoid).

### Inputs from R1 (consumed, not rebuilt)
R2 consumes R1's **accepted** output: `final_disruption[]` (per cell), `system_cells[]` (owner per system),
and the `cell_inputs[]` channel partition (to attribute the blockader). R2 may rerun the R1 fixture or
consume its report structures. R1's columns and artifact posture are the input contract.

### Output (deterministic artifact)
Per-system production rows; per-faction stockpile ledger (Terran/Pirate) before reduce-up, after
reduce-up, after disburse-down; blockade flags; **diverted-production rows** (which system's production
flipped to which blockader); top-N; stable checksum; CPU-oracle parity flag.

### Posture
Opt-in / default-off; deterministic; CPU oracle is the authority; **no GPU required** (the reduce-up and
divert are integer/bounded masked sums — if a GPU cross-check is ever added it is `GpuVerified`, never f32
bit-exact, and none is authorized here).

## 5. Rung-identity boundaries (what makes this R2, not R3/R5/R6)

These are the lines that **define** R2. Crossing one means a different rung was built — the implementer
must stop and return to Opus. (Stated as identity, not as negative-test hygiene.)

1. **No occupant ever moves.** Fleet/occupant positions are R1 fixtures, **read-only** in R2. No
   relocation, no REENROLL, no `BoundaryRequest`. *Divert moves a flow's owner-column, never a SimThing.*
2. **Divert is an owner-column flip, never reparenting.** The system stays the child it was; only the
   production-outflow's owner-column changes (D=2). No tree mutation. *(The §0.0 conformance point.)*
3. **No techtree / modifier overlays (R3).** No decay-resistance / suppression / combat bonuses; flows use
   base rates.
4. **No combat HP/Damage arena (R6).** Co-located hostiles do **not** fight in R2; the only contention is
   economic divert.
5. **Single galactic tier only.** R2 does **not** materialize the system-interior 10×10 sub-tiles; the
   recursive *behavior* is proven at one level (system cells → faction stockpile + galactic summary).
   Deeper system→planet recursion is a **named future build fork**, not part of R2.
6. **Reuse existing machinery — no new op, no shader, no WGSL.** `AccumulatorOp`
   (`ConjunctiveCrossing`/`CrossingFormula`/`SubtractFromAllInputs`/whitelisted `EvalEML`
   `CMP_EQ`/`SELECT`/`Threshold`) + OWNER masked reduction + `econ_scale_0080_0` clearinghouse.
7. **Opt-in / default-off.** No default `SimSession` pass-graph change; no global schedule.
8. **No CPU planner.** Disburse-down is a deterministic subsidiarity sweep, not search/optimization.
9. **No hard currency / markets / trade / `ai_budget`.** Production is the resource; no price/market layer
   (stays gated — no consumer in this scenario). No ClauseThing/L3, no UI/realtime loop.

## 6. Candidate data surfaces (name only — do not implement here)

- child gridcell / system disruption rows consumed from R1 (`final_disruption`, owner per system, channel);
- per-system labor→production rows (M1);
- per-faction stockpile ledger rows: pre-reduce, post-reduce, post-disburse (M2);
- owner/channel-partitioned reduce-up totals (EC-A3 masked);
- blockade flags + diverted-production rows (M3: system → blockader, amount);
- deficit-disbursement rows (faction → deficit system, amount);
- top-N summary + deterministic checksum + CPU-oracle parity.

## 7. Implementation recipient (after this opening merges)

- **Recipient:** Cursor / Codex5.5max — production implementation agent.
- **Scope:** exactly §4 within the §5 boundaries; implementation PR only after this opening is merged.
- **Likely files:** `crates/simthing-driver/src/dress_rehearsal_r2_recursive_allocation.rs`,
  `crates/simthing-driver/tests/dress_rehearsal_r2_recursive_allocation.rs`,
  `docs/tests/scenario_0080_2_r2_recursive_allocation_report.md` (report only after tests pass).
- Wire into `crates/simthing-driver/src/lib.rs` opt-in/default-off.

## 8. Required tests for the implementation rung (capability + identity only)

Every test below proves a real capability or defines the rung's identity. **No negative-assertion
hygiene tests** (no `no_invariant_edit`, `no_clausething`, etc.) — absence of out-of-scope machinery is a
boundary (§5), not something to dignify with a test.

1. `r2_opening_status_matches_track` — status string ↔ production-track R2 row.
2. `r2_consumes_accepted_r1_heatmap` — reads R1 `final_disruption` + per-system owner as the input contract.
3. `r2_factory_recipe_converts_labor_to_production` — 10 labor → 1 production via `SubtractFromAllInputs`.
4. `r2_production_reduces_up_to_faction_stockpile_owner_masked` — Terran/Pirate stockpiles, never blind-summed.
5. `r2_faction_disburses_surplus_to_deficit_system` — subsidiarity down-sweep to an unmet starport need.
6. `r2_blockade_threshold_gates_outflow_at_100` — `disruption ≥ 100` gates a system's outbound production.
7. `r2_divert_flips_production_owner_column_to_blockader` — pirate-blockaded Terran system → Pirate stockpile.
8. `r2_divert_is_owner_column_flip_not_reparenting` — tree/identity unchanged; only the owner-column moves.
9. `r2_no_occupant_relocated_and_no_boundary_request` — identity vs R5 (positions read-only; no relocation event).
10. `r2_no_combat_resolution` — identity vs R6 (co-located hostiles lose no HP; only economic divert).
11. `r2_single_tier_no_interior_subtile_materialization` — build-fork boundary (no 10×10 interior tiles).
12. `r2_deterministic_replay_and_cpu_oracle_parity` — identical field/ledger/checksum on replay; oracle agreement.
13. `r2_opt_in_default_off` — disabled is a no-op; default-on rejected; no default-session wiring.

## 9. Acceptance criteria (for the eventual R2 implementation)

1. §7 files exist; opt-in/default-off; default `SimSession` pass graph unchanged.
2. M1 production economy runs on the existing recipe machinery (no new op).
3. M2 reduce-up is OWNER-masked (factions never merged); disburse-down reaches a real deficit system.
4. M3 blockade gates at `≥100` and divert flips the owner-column to the blockader — proven a **column flip,
   not reparenting, with no occupant moved**.
5. Consumes R1's accepted output as the input contract.
6. Deterministic artifact + stable checksum + CPU-oracle parity.
7. No §5 boundary crossed.
8. Report written **after** tests pass.

## 10. §0.5 self-check

Opening-spec authoring only — no implementation, no code, no shader/math/tolerance change, no
`simthing-sim` semantic expansion, no default session wiring, no movement/REENROLL, no combat, no techtree
mask-down, no R3/R4/R5/R6/R7 work, no invariant edit. Opens exactly one bounded rung (R2: recursive
allocation + faction economy + blockade/divert, single galactic tier) at full §12.5 scope with explicit
rung-identity boundaries, and names the next recipient (Cursor / Codex5.5max).
