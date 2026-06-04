# SCENARIO-0080-2 — R1-OPEN Disruption Heatmap / EC1 Opening Spec

> **This is a design-authority *opening spec*, not an implementation.** It authorizes — and bounds — a
> single Cursor/Codex implementation rung. The implementation agent must build only what §4/§9 authorize,
> stop at the §11 boundaries, and update docs only after the §10 tests pass.

## 1. Gate and verdict

- **Gate:** `R1-OPEN` — Disruption heatmap / EC1.
- **Predecessor:** `ATLAS-BATCH-0-CLOSE` — CLOSED / PASS (`docs/tests/scenario_0080_2_atlas_batch_0_close_report.md`).
- **Verdict:** **OPEN / AUTHORED.** R1 implementation is authorized within this spec's scope. R2–R7 stay
  unopened; no parked boundary is opened beyond what §4 names.
- **Authoring authority:** Opus (design authority). **Implementation recipient:** Cursor / Codex5.5max (§13).

> **Implementation result (2026-06-04):** `SCENARIO-0080-2-R1-IMPL-0` is **IMPLEMENTED / PASS** within
> this opening spec's scope. Evidence:
> [`docs/tests/scenario_0080_2_r1_disruption_heatmap_report.md`](../tests/scenario_0080_2_r1_disruption_heatmap_report.md).
> This note records the implementation result only; R2–R7 remain unopened/deferred as specified below.

## 2. Canonical citations (§12.0 harness — cite on the implementation handoff)

1. `docs/design_0_0_8_0.md` — §0 transient constitution: all conflict is resource flow (§0.3); recursive
   allocation (§0.2); SEAD = GPU-resident threshold crossings, **no CPU planner**; §0.1 `kind` is the
   install-time selector only (never a runtime branch); §0.5 harness discipline.
2. `docs/invariants.md` — **Scenario Proof** (a rung is proved only through a real reduction over real
   SimThings, not math in a vacuum); AccumulatorOp v2; Resource Flow Substrate; parity classes
   (`ExactDeterministic` vs `GpuVerified`).
3. `docs/design_0_0_8_0_consumer_pulled_production_track.md` — §12–§12.5: rehearsal design, EC1/EC2
   (§12.1), the recursive nested-grid field hierarchy (§12.2), ATLAS-BATCH-0 (§12.3), OWNER masked
   reduction (§12.4), rung ladder + retirement map (§12.5).
4. `docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md` — concrete scenario: 20×20 galaxy, 13 systems,
   disruption-as-blockade (§6), galactic-tier heatmaps (§4.1), anticipated emergent behaviors (§8.1).
5. `crates/simthing-core/src/accumulator_op.rs` — `SourceSpec`, `CombineFn` (incl. whitelisted
   `EvalEML`), `GateSpec`, `ScaleSpec`, `ConsumeMode` (`SubtractFromSource`/`SubtractFromAllInputs` are
   THE transfer mechanisms).
6. `docs/workshop/sead_self_ai_track.md` — SEAD charter: field-as-policy, GPU-resident threshold crossings
   → `BoundaryRequest`, no CPU planner.
7. `crates/simthing-spec/src/spec/eml_gadget.rs` — `EmlGadgetInstanceSpec::BoundedFeedback`
   (`next = clamp(previous*decay + input*gain, min, max)`; strict `min < max`) and `Decay`; the Tier-2
   temporal substrate R1 is the first real consumer of.
8. `crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_gen.rs` — the canonical R1 input: `Owner`,
   `FleetKind::{Patrol, Pirate}`, `FleetPlacement { kind, owner, galactic_cell, system_cell }`,
   `GridCell`, `GridDims`, `SystemDescriptor`, `GALAXY_SIDE = 20`, `SYSTEM_COUNT = 13`.

## 3. Purpose

R1 is the **first true full-vertical scenario-proof rung** after the atlas prerequisite. It proves **EC1**:

> A **non-trivial disruption heatmap** over **real gridcell SimThings**, **produced by pirate/patrol
> presence** (not hand-seeded), **verified against a CPU oracle**, and **emitted as an inspectable
> heatmap artifact**.

R1 closes the connecting tissue that every prior 0080-x "pass" left un-wired (§12.1 F5):

```
occupants in galactic grid cells (fleets + systems)
  → per-channel/per-owner cell contributions          (OWNER masked routing, EC-A3 form)
  → disruption SimProperty column on gridcell SimThings
  → BoundedFeedback recurrence (accumulate + decay)    (EML Tier-2, first real consumer)
  → diffusion into location_status (sink column)        (stencil, strict source≠target)
  → reduce a summary up to the starmap Location         (the heatmap over its child gridcells)
  → CPU oracle parity
  → inspectable heatmap artifact
```

**R1 does NOT include SEAD movement.** A mover *consuming* this heatmap gradient to choose sit-vs-step is
**EC2 / R4**, explicitly out of scope (§11).

## 4. Authorized scope

R1 is bounded to a **single galactic-tier (20×20) field**. Disruption lives directly on the galactic
gridcell SimThings; fleets and systems are **occupants/contributors** into their galactic cell. This
deliberately defers the **recursive multi-tier reduce-up** (system 10×10 → galactic, §12.2) to **R2** so
R1 does not swallow R2's nested reduction. "Reduce up to the starmap" in R1 means the starmap `Location`
holds the **field over its 400 child galactic gridcells plus a summary column** (total / top-N / checksum)
— not a system→galactic recursion.

Authorized to build:

### 4.1 Real SimThing gridcell disruption column
Build on the `ATLAS-BATCH-0` Location/gridcell layout (`dress_rehearsal_atlas_batch_0_gen.rs` +
`..._loc.rs`). Express disruption as **real SimThing data**:

```
worldstate → starmap Location (20×20)
  → galactic gridcell SimThings (400, dense row-major; cell_index = y*20 + x is the only cell home)
    → disruption       SimProperty column   (source/state)
    → location_status  SimProperty column   (sink/diffused)
```

The cell is the dense map slot; occupants are **contributors into the cell**, never merged into it
(per LOC/STORE EC-A3 — co-located occupants stay distinct by channel/owner before reduction).

### 4.2 Occupant-produced sources (no hand-seeded heatmap)
Sources arise **only** from scenario occupants:

```
Pirate fleet  (FleetKind::Pirate, Owner::Pirate)  present in a galactic cell → +pirate_emit disruption
Patrol fleet  (FleetKind::Patrol, Owner::Terran)  present in a galactic cell → −patrol_suppress disruption
root/session overlay                                                          → decay / gravity-to-zero
```

**Hand-seeding the heatmap field as the primary proof is forbidden.** Test fixtures may set **fleet
positions** (occupant placement is the input); the **field values must arise from those occupants'
contributions through the recurrence**, not be written directly. Planet/factory/pop/starport occupants
**must not** write the pirate/patrol disruption channels.

### 4.3 BoundedFeedback disruption recurrence (the one new substrate consumer)
Use the existing whitelisted `EmlGadgetInstanceSpec::BoundedFeedback`
(`next = clamp(previous*decay + input*gain, min, max)`). The per-cell **input** is the net occupant
contribution for that cell:

```
input_cell      = (pirate_ships_in_cell * PIRATE_EMIT) − (patrol_ships_in_cell * PATROL_SUPPRESS)
disruption_next = clamp(disruption_prev * DECAY + input_cell * GAIN, FLOOR, CEILING)
```

This is R1's first **real** consumer of the EML Tier-2 temporal substrate.

### 4.4 Diffusion into `location_status` (two-column, strict sink)
A **second** column receives the stencil-diffused falloff. Single dense pass over the 400 galactic cells
using the proven `StructuredFieldStencilOp` diffusion (`GpuVerified` f32) **or its CPU oracle**:

```
source_col (disruption) ≠ target_col (location_status)     — strict sink; diffusion never writes back into disruption
neighbors are NOT arena-enrolled                            — sparse arenas, dense diffusion meet at the column
tile-local G=0 isolation preserved                          — no inter-tile bleed across unrelated atlas tile boundaries
```

### 4.5 Reduce up to the starmap heatmap + artifact
Reduce a **summary** (`SlotRange Sum` total + top-N hotspot cells + a stable checksum) up into the starmap
`Location`'s summary column, and **emit an inspectable artifact** (§8).

### 4.6 Implementation path — CPU oracle primary; GPU optional cross-check
**The primary R1 proof is the CPU oracle over existing whitelisted primitives. No new GPU/shader code is
required or authorized for R1.** Rationale: EC1's job is to prove the *vertical wiring is real* (occupants
→ column → recurrence → diffuse → reduce → artifact through real SimThings), not GPU throughput; the GPU
diffusion stencil and masked reduction are **already proven** by ATLAS-BATCH-0 (`PACK-GPU` GpuVerified,
`STORE-GPU` bit-exact). The implementation **may** additionally run the existing GPU diffusion as a
`GpuVerified` (f32, L∞ ≤ 1e-4) cross-check on the RTX 4080, but it is **optional** and must **not** make a
bit-exact f32 claim. Any *new* GPU/shader code requires a separate, separately-justified gate (§11).

## 5. Data model

| Element | SimThing kind | Columns / role | Notes |
|---|---|---|---|
| Galactic starmap | `Location` (20×20) | summary column (total / top-N / checksum) | parent of the 400 galactic gridcells |
| Galactic gridcell | `Location` | `disruption` (source/state), `location_status` (sink/diffused) | dense, row-major; `cell_index = y*20 + x` |
| Pirate fleet | `Fleet` (`FleetKind::Pirate`, `Owner::Pirate`) | contributes `+PIRATE_EMIT` into its `galactic_cell` | occupant, not a cell |
| Patrol fleet | `Fleet` (`FleetKind::Patrol`, `Owner::Terran`) | contributes `−PATROL_SUPPRESS` into its `galactic_cell` | occupant, not a cell |
| System / planet / factory / pop / starport | `Location` / `Custom(..)` | **do not** write the disruption channels in R1 | present, inert to disruption |

Channel/owner separation is the **EC-A3 OWNER masked-routing** form already proven in STORE: a cell's
pirate contribution and patrol contribution remain distinct (by channel/owner) until the recurrence's
`input_cell` net is formed. R1 uses **only** as much OWNER routing as cell-contribution routing requires —
no R3 mask-down, no session-pass OWNER runtime (§11).

## 6. Numeric recurrence (PINNED for R1; tunable in later rungs)

```
DECAY          = 0.80     // 20% bleed/tick → monotone decay toward FLOOR when no source persists
GAIN           = 1.00
FLOOR          = 0.0      // patrol suppression cannot drive disruption below 0 (clamp floor)
CEILING        = 100.0    // pirate emission cannot exceed 100 (clamp ceiling; == the §6 blockade line)
PIRATE_EMIT    = 20.0     // per pirate ship present in a galactic cell, per tick
PATROL_SUPPRESS= 15.0     // per patrol ship present in a galactic cell, per tick
```

Consequences (the CPU oracle must reproduce these exactly):
- **Steady state, one pirate, no patrol:** `d* = GAIN*PIRATE_EMIT/(1−DECAY) = 20/0.2 = 100.0` → saturates
  at `CEILING` (a lone uncontested pirate raids a cell up to the blockade line). Reaches it monotonically.
- **Decay with no source:** `d_{n+1} = 0.8·d_n` → strictly decreasing toward `FLOOR`.
- **Patrol dominates a lone pirate:** one pirate (+20) under two patrol (−30) → `input = −10` → clamped at
  `FLOOR = 0.0` (suppression floors the cell; cannot underflow).
- **Ceiling holds:** `input` can never push `disruption_next` above `CEILING`.

Required recurrence properties: **bounded**, **deterministic**, **CPU-oracle reproducible**, monotone
decay toward zero without new source, floor cannot underflow, ceiling cannot be exceeded.

> **R1 does NOT implement the `disruption ≥ 100` blockade gate or production diversion (§6 of the scenario
> spec).** `CEILING = 100.0` is only the field's natural saturation. The blockade/divert mechanic is
> R1/R2 coupling and is **out of scope** here (§11).

## 7. Diffusion / `location_status` model

- **Two columns, strict sink:** `source_col (disruption) ≠ target_col (location_status)`. Diffusion writes
  **only** into `location_status`; it never feeds back into `disruption`.
- **Dense sweep:** one stencil pass over all 400 galactic cells (the materialized galactic tier).
- **Neighbors not enrolled:** falloff is a **property-field stencil**, not arena enrollment.
- **Horizon H:** the stencil falloff radius is the strategic sight radius (§4.1). **PINNED for R1:**
  a single normalized 4-neighbor (von Neumann) diffusion step with weight `H_WEIGHT = 0.25` per neighbor
  (i.e. `location_status[c] = disruption[c]` blended with `0.25·Σ neighbor disruption`, normalized,
  clamped to `[FLOOR, CEILING]`). Falloff must reach neighbors and decay with distance; a hotspot under a
  pirate must show a non-zero gradient to adjacent cells.
- **Isolation:** tile-local `G=0` isolation from ATLAS-BATCH-0 holds where relevant — **no inter-tile
  bleed** across unrelated atlas tile boundaries.

## 8. Heatmap artifact

Emit a compact, deterministic, **non-UI** artifact (markdown or JSON; no rendering/realtime loop):

```
- 20×20 galactic grid, one row per cell:  (x, y), disruption, location_status
- top-N hotspot cells (N = 8), descending by disruption
- summary: total disruption, max cell, occupied-cell count
- stable checksum over the field (deterministic across runs)
- CPU-oracle parity column (and GPU cross-check values iff the optional §4.6 GPU path is run)
```

Artifact requirements: emitted, **deterministic** (identical across runs / re-checksums stable), contains
a **non-trivial nonzero field**, a **hotspot near pirate presence**, a **suppressed region near patrol
presence**.

## 9. Required implementation deliverables

| File | Role |
|---|---|
| `crates/simthing-driver/src/dress_rehearsal_r1_disruption_heatmap.rs` | R1 builder + CPU oracle: occupant→column routing, BoundedFeedback recurrence, diffusion, reduce-up, artifact emit |
| `crates/simthing-driver/tests/dress_rehearsal_r1_disruption_heatmap.rs` | the §10 test battery |
| `docs/tests/scenario_0080_2_r1_disruption_heatmap_report.md` | implementation report (created only **after** tests pass) |

Wire `dress_rehearsal_r1_disruption_heatmap` into `crates/simthing-driver/src/lib.rs`. The R1 path is
**opt-in / default-off** — it must not alter the default `SimSession` pass graph (§11).

## 10. Required tests

### 10.1 Shape / layout
- Uses the `ATLAS-BATCH-0` GEN/LOC descriptor + layout (`GALAXY_SIDE = 20`, `SYSTEM_COUNT = 13`).
- 20×20 galactic grid present; the 13 systems from the descriptor preserved.
- `disruption` and `location_status` columns allocated; `cell_index = y*20 + x` the only dense cell home.
- Occupants remain contributors into cells; **no blind sum by position** across channels/owners.

### 10.2 Source production
- A pirate occupant contributes **positive** disruption to its `galactic_cell`.
- A patrol occupant contributes **suppression** to its `galactic_cell`.
- Planet/factory/pop/starport occupants **do not** write the pirate/patrol disruption channels.
- Co-located occupants (e.g. a pirate and a patrol in one cell) stay separated by channel/owner **before**
  the `input_cell` net is formed.

### 10.3 Recurrence / bounded-feedback CPU oracle (several deterministic ticks)
- Same inputs → same `disruption_next` (deterministic).
- Decay occurs when no source persists (`0.8×` per tick toward `FLOOR`).
- Ceiling holds (`≤ CEILING`); floor holds (`≥ FLOOR`, suppression cannot underflow).
- Pirate presence raises local disruption; patrol presence reduces it.
- The lone-pirate steady state converges to `100.0` (§6); two-patrol-vs-one-pirate floors at `0.0`.
- The implementation values match the CPU oracle.

### 10.4 Diffusion / no-bleed
- `location_status` receives diffused falloff; the `disruption` source column is **not** overwritten.
- Falloff reaches neighbors per the stencil and decays with distance.
- `G=0` / tile-local isolation holds where relevant; **no inter-tile bleed** across unrelated tiles.

### 10.5 Heatmap artifact
- Artifact emitted; deterministic; contains a non-trivial nonzero field.
- Hotspot near pirate presence; suppressed region near patrol presence; checksum stable across runs.

### 10.6 Regression (cheap, directly relevant prerequisites)
```
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_gen
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_loc
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store
```
**Only if the optional §4.6 GPU cross-check is implemented**, also run on the discrete RTX:
```
$env:SIMTHING_RUN_GPU_TESTS=1
$env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"
$env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1
cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu
```
GPU results are `GpuVerified` (f32 tolerance) — **no bit-exact f32 claim**.

## 11. Non-goals and stop conditions

R1 **must not** implement (if any is required, **stop and return to Opus**):

```
SEAD movement / action selection            GradientXY consumption by a mover (EC2 / R4)
exact sqrt Candidate F                       REENROLL
M-4A sparse-residency scheduler              R2 recursive nested reduction / faction economy
R3 capability-tree mask-down                 R4 SEAD field-consumption
R5 movement / ship fission                   R6 combat
the disruption ≥ 100 blockade gate / production diversion (scenario §6 — R1/R2 coupling)
hard currency / markets / trade / ai_budget  OWNER masked-reduction runtime beyond R1 cell-contribution routing
default SimSession pass-graph change (R1 is opt-in fixture only)
global default schedule                      UI / realtime loop / CLI binary
ClauseThing / L3                             semantic / raw WGSL
new shader code (unless separately justified + bounded)
f32 bit-exact claims for adapter-sensitive paths
```

## 12. Acceptance criteria

R1 implementation is accepted (by Opus) when **all** hold:
1. §9 files exist; R1 is opt-in/default-off; the default `SimSession` pass graph is unchanged.
2. The heatmap is **produced by occupant presence through the recurrence**, not hand-seeded (§4.2).
3. The BoundedFeedback recurrence matches the §6 pinned constants and the CPU oracle, with the bounded /
   monotone-decay / floor / ceiling properties proven (§10.3).
4. Diffusion is a strict sink into `location_status`, dense, no inter-tile bleed (§10.4).
5. A deterministic, non-trivial heatmap artifact is emitted with a hotspot near pirates and suppression
   near patrols; checksum stable (§10.5).
6. §10.6 regressions green; if the optional GPU cross-check ran, it is `GpuVerified` on the RTX with no
   bit-exact f32 claim.
7. No §11 boundary was crossed.
8. The report (`docs/tests/scenario_0080_2_r1_disruption_heatmap_report.md`) records each test's result,
   the CPU-oracle parity, the artifact, and a §0.5 self-check — **written only after tests pass**.

## 13. Handoff to implementation agent

```
Recipient: Cursor / Codex5.5max
Role: production implementation agent
Task: Implement SCENARIO-0080-2 R1 — Disruption heatmap / EC1 — per this opening spec.

Authorized scope:        §4 (galactic-tier single field; CPU oracle primary; optional GPU GpuVerified cross-check)
Data model:              §5     Recurrence (pinned): §6     Diffusion: §7     Artifact: §8
Create:
  - crates/simthing-driver/src/dress_rehearsal_r1_disruption_heatmap.rs
  - crates/simthing-driver/tests/dress_rehearsal_r1_disruption_heatmap.rs
  - docs/tests/scenario_0080_2_r1_disruption_heatmap_report.md   (only after tests pass)
Wire into:               crates/simthing-driver/src/lib.rs   (opt-in / default-off)
Required tests:          §10.1–§10.6
Stop conditions:         §11 — if implementation requires ANY listed item, STOP and return to Opus.
Cite on handback:        this spec + §2 harness.
Update docs only after tests pass; do not change R2–R7 status; do not delete temporary NVIDIA files.
```

## 14. §0.5 self-check

Opening-spec authoring only — **no implementation, no code, no shader/math/tolerance change, no
`simthing-sim` semantic expansion, no recursive-allocation change, no CPU planner, no default session
wiring, no SEAD movement, no REENROLL, no R2/R3/R4/R5/R6 work.** Authorizes and bounds exactly one R1
implementation rung (EC1: occupant-produced disruption heatmap over real gridcell SimThings, CPU-oracle
verified, inspectable artifact), and names the next recipient (Cursor / Codex5.5max). All numeric constants
are pinned for R1 and explicitly tunable in later rungs.
