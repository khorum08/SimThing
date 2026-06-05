# RUNTIME-0080-0-R1 — GPU-resident next-tick authority (`GPU-NEXTTICK-0`)

**Authoring authority:** Opus / design authority (`RUNTIME-0080-0-R1-DESIGN-0`, 2026-06-05).
**Track status:** OPEN (design opening; first implementation sub-rung scoped below).
**Implementation recipient (after this opening lands):** Cursor / Codex5.5max.
**Crosses a Tier-2 runtime boundary** — this opening defines the substrate primitive, the resident
state contract, and the stop-lines so implementation stays bounded.

This rung realizes the substrate gap that `RUNTIME-0080-0-R0A-REMEDIAL-0` honestly surfaced. It does
**not** reopen `SCENARIO-0080-2`, does **not** edit `docs/invariants.md`, and does **not** add a new
`AccumulatorOp` or semantic WGSL.

---

## 0. Claim check (confirmed) and R0A disposition

**Claim under review:**

> RUNTIME-0080-0 R0A proves a persistent-GPU-session mirror-dispatch scheduler, not GPU-resident tick
> authority. The CPU remains the tick driver. The next real runtime rung is the substrate that makes
> GPU-resident world state the input authority for tick N+1.

**Ruling: CONFIRMED, verbatim.** R0A holds an `AccumulatorOpSession` resident across 100 ticks and
dispatches the measured per-tick shapes (`inter_tick_world_readbacks=0`, checksum `1bba891c779190a4`),
but the CPU R6C model mutates the world and the GPU receives the already-mutated world via
`mirror_dispatch_after_cpu_tick`. `gpu_state_feeds_next_tick = false`. That is mirror-dispatch, not
next-tick authority.

**R0A disposition: CLOSED as PARTIAL / informative.** R0A did the correct thing by refusing to fake GPU
authority; it discovered the real substrate gap. No change to PR #531 is required (Outcome C is **not**
chosen — the PARTIAL posture and wording are correct as merged).

**Opus output: a hybrid of A and B.** R1 is opened as the umbrella substrate primitive (§6), but the
*first* implementation sub-rung is **deliberately narrowed** (§1, `R1a`) because the full R6C structural
write-back (REENROLL membership scatter, cohort birth/removal, fusion compaction/lineage) sits squarely
on the parked M-4A / free-list-scatter stop-lines. The field-column transition does not, and it is
already GPU-measured. We promote the safe surface first and gate the structural surface behind its own
rungs (§1.4).

---

## 1. The substrate primitive — `GPU-NEXTTICK-0`

**Canonical primitive name:** `GPU-NEXTTICK-0`.
**Canonical rung id:** `RUNTIME-0080-0-R1` (umbrella). **First IMPL sub-rung:** `RUNTIME-0080-0-R1a`.

`GPU-NEXTTICK-0` is the contract under which **GPU-resident world state is the authoritative input to
tick N+1** for the columns it covers:

```text
state_N lives on GPU (resident buffer A)
→ GPU dispatches the tick transforms (already-measured row/mask/reduce/disburse/threshold/emission-band)
→ state_N+1 is produced on GPU into resident buffer B
→ buffer B becomes the resident "current" at the tick boundary (double-buffer swap)
→ next tick reads state_N+1 directly from GPU residency (no CPU upload, no GPU wait on CPU)
→ CPU oracle observes/compares at the boundary, but does NOT drive state_N+1
```

The discriminator between this and R0A mirror-dispatch is exact and testable:
**`gpu_state_feeds_next_tick == true` for the covered columns** — the GPU tick reads its own prior-tick
output, never a CPU re-upload of CPU-computed state.

### 1.1 SimThing Maximality directive (binding for this track)

> SimThing Maximality requires that any state transition already expressible as
> row/mask/reduce/disburse/threshold/emission-band should be promoted toward **resident** execution
> rather than left as a CPU-managed orchestration. The CPU may remain the oracle, inspector, and report
> writer; it may **not** remain the hidden authority for state_N+1 when the runtime track claims
> GPU-resident execution.

This is **not** "GPU at all costs." The correct fallback when a transition cannot yet be made resident
within the stop-lines is an honest **PARTIAL/BLOCKED plus a named substrate gap** — never CPU
orchestration wearing GPU language. R0A is the reference example of the correct fallback.

### 1.2 Parity-preservation directive (binding — addresses the shadow-table risk)

Disconnecting the CPU game-state mirror outright would break semantic-GPU state parity (the CPU oracle
is the determinism reference the whole track is checked against). Therefore `GPU-NEXTTICK-0` does **not**
delete the CPU shadow. It changes **who is authoritative**, not **who exists**:

- The GPU resident buffer is the **authority** for covered columns; the GPU tick reads it directly.
- The CPU shadow is retained as **oracle / parity witness / save-state materializer**, reconciled at the
  **tick boundary only** (§2). The CPU shadow is never the mid-tick input to the next GPU tick.
- Parity is asserted at each tick boundary: covered resident columns must reproduce the R6C CPU oracle
  bit-exact for integer state and within the accepted f32 bound for R4-derived quantities, including the
  trajectory checksum `1bba891c779190a4`. If parity cannot hold resident, **STOP and report the delta**
  (§7) — do not loosen any bound to force a pass.

### 1.3 Tick boundary = save / pause / stable-state point (binding)

The double-buffer swap at the tick boundary is the canonical **save / pause / stable-state** point:
after the swap, the resident "current" buffer is a complete, self-consistent snapshot of all covered
columns for tick N+1. Pausing at a boundary yields a clean save; resuming reads the resident snapshot
directly. No partial-tick state is ever a save point.

### 1.4 GPU-must-not-wait-on-CPU directive (optimization target)

Wherever the design can eliminate a state in which the **GPU waits on the CPU**, it must:

- R1a removes the per-tick CPU→GPU upload for covered field columns: the GPU tick reads its own
  prior-tick resident output, so there is no inter-tick upload wait for those columns
  (`inter_tick_world_readbacks` stays 0 **and** `inter_tick_world_uploads` for covered columns → 0).
- CPU engagement for structural maintenance is **moved to the boundary** and **driven by GPU-written
  events** (the boundary-event dispatch, §3), so the GPU does not block mid-tick on a CPU planner.
- Where boundary maintenance can be overlapped with the next field-tick without violating the
  save-state guarantee, the design permits it; where it cannot, the boundary is the synchronization
  point and that is acceptable (pause/save requires a boundary barrier anyway).

---

## 2. Authoritative resident state — column classification

The R6C world (`DressRehearsalR6cWorld`) is classified into three tiers. **Only Tier-A is made
GPU-authoritative in the first IMPL sub-rung (`R1a`).**

### Tier-A — GPU-authoritative in `R1a` (field columns; already GPU-measured)

These are pure row/mask/reduce/disburse/threshold/emission-band columns with bit-exact (or within-bound)
GPU-measured shapes from `GPU-MEASURE-0080-0`. They become resident, double-buffered, GPU-authoritative
inputs to tick N+1:

| Column | R6C field | Transition | Measured shape |
| --- | --- | --- | --- |
| Disruption per cell | `disruption: Vec<f32>` | R1 bounded-feedback recurrence | integer bit-exact |
| Location status (sink) | `location_status: Vec<f32>` | R1 diffusion sink | integer bit-exact |
| Faction stockpiles | `stockpiles` (Terran/Pirate) | R2 reduce-up / disburse-down | integer bit-exact |
| Construction progress per system | `construction_progress` | R6B threshold accumulator | integer bit-exact |
| Per-cohort ship-count column | `fleets[*].num_ships` (value only) | R6 attrition decrement + R6B reinforcement increment **into existing slots** | integer bit-exact |
| Blockade/divert owner code per system | `blockade_divert_owner` | R2 owner-column flip (value, not reparent) | integer bit-exact |
| R4 composite/gradient scratch | derived | R4 GradientXY + Candidate-F magnitude | within `1.0e-4`; Candidate-F bits match |

Note the deliberate restriction on `num_ships`: in `R1a` the GPU is authoritative for the **value
update of an existing cohort slot** (decrement on attrition, increment on reinforcement). It is **not**
yet authoritative for slot **creation** (birth), slot **removal** (departure at zero), or slot **fusion**
(compaction) — those are structural (Tier-B).

### Tier-B — CPU-observed/maintenance in `R1a`; candidate resident in `R1b`/`R1c`

Structural columns requiring scatter/compact/allocation. In `R1a` these remain CPU boundary-maintained,
driven by GPU-written boundary events (§3). They are **not** CPU-planner decisions — the *decision* is a
GPU threshold/emission-band crossing; only the *structural application* is a bounded CPU boundary pass.

| Column | R6C field | Why structural |
| --- | --- | --- |
| Arena membership / occupancy | `arena_membership: BTreeMap<u32, Vec<u64>>` | REENROLL scatter between source/destination cells |
| Fleet cohort table membership | `fleets: Vec<…>` add/remove | birth (ALLOC arrival) and removal (departure at zero ships) |
| Fleet cell index (position) | `fleets[*].cell_index` | movement reassignment = REENROLL |
| Fusion lineage / identity | `fleets[*].lineage`, `identity_lane`, `owner_faction_id` | cohort compaction records IDROUTE identity + lineage |
| Movement event rows | boundary-request / movement rows | emitted by threshold crossing; applied at boundary |

### Tier-C — deferred (requires M-4A or multi-atlas; out of scope for all of R1)

| Concern | Reason deferred |
| --- | --- |
| Multi-atlas batching of membership | §11 atlas-batching admission gate (M-4) |
| M-4A algebraic tile-local masking at scale | gated; distinct rung |
| System→planet recursive tiering | candidate E |
| Multi-faction ECON beyond Terran/Pirate | candidate D |

---

## 3. Write-back contract

The covered (Tier-A) transition uses **double buffering** for resident field state and a **GPU-written
event journal** for the structural (Tier-B) hand-off to the CPU boundary-maintenance pass.

### 3.1 Field columns (Tier-A) — double-buffered resident swap

```text
buffer_cur (state_N)  ── GPU tick transforms ──▶  buffer_next (state_N+1)
                                                   │
                              tick boundary swap ──┘   (buffer_next becomes buffer_cur)
CPU oracle reads buffer_cur after the boundary for parity/report/save — never mutates it.
The next GPU tick reads buffer_cur (= state_N+1) directly; no CPU upload of Tier-A columns.
```

- **GPU transform writes `state_N+1` into the resident next buffer.** No CPU write to Tier-A between
  ticks.
- **CPU may read** the resident buffer **after the tick boundary** for oracle/report/save.
- **CPU may not mutate** authoritative Tier-A state between ticks.
- **Next GPU tick reads resident `state_N+1`** directly.

Double-buffer is chosen over a pure journal for Tier-A because the field transition is a dense
whole-grid update (every cell's disruption/status), where a ping-pong resident pair is the natural,
already-proven shape (`StructuredFieldStencil` ping-pong, `AccumulatorOpSession`).

### 3.2 Structural columns (Tier-B) — GPU-written boundary event journal + CPU maintenance dispatch

This is where the **boundaryEvent dispatch system engages the CPU for maintenance** (the user
directive). The model is a **resident event journal**, not a CPU planner:

```text
GPU threshold/emission-band crossings WRITE event rows into a resident journal:
   - MovementRequest{mover_id, source_cell, dest_cell, mag_bits}      (R4/R5 step opportunity)
   - ShipLoss{cohort_slot, ships_destroyed}                           (R6 emission-band)
   - ShipGain{cohort_slot or birth_request, ships_added}              (R6B reinforcement / birth)
   - FusionRequest{left_slot, right_slot}                             (R6B friendly fusion)
At the tick boundary, the CPU drains the journal as a BOUNDED MAINTENANCE PASS:
   - applies REENROLL membership scatter, cohort birth/removal, fusion compaction + lineage
   - this is deterministic structural bookkeeping, NOT decision-making
   - the decisions were already made by GPU threshold crossings on the resident field
```

- The CPU boundary pass is **bounded** (≤ the existing per-tick row counts) and **deterministic** — it
  is the same structural application R6C already performs, relocated to a boundary-dispatch consumer.
- No CPU planner, route search, or policy AI is introduced or permitted (§7).
- The journal is the seam that lets `R1a` ship honestly: Tier-A is GPU-authoritative now; Tier-B is a
  GPU-decided / CPU-applied maintenance step now, and a candidate for full residency in `R1b`/`R1c`.

### 3.3 Boundary barrier and save-state

The boundary performs, in order: (1) GPU Tier-A swap; (2) GPU journal flush to resident readback;
(3) CPU drains journal → applies Tier-B maintenance → re-seeds any Tier-A slot bookkeeping that
structural changes imply (e.g. a birthed cohort's initial `num_ships`); (4) parity assertion + optional
save snapshot. After (4) the world is a clean stable state. Pause = stop at step (4).

---

## 4. Movement / REENROLL residency (R1a posture + R1b/R1c target)

R6C movement is `R4 StepOpportunity → BoundaryRequest → REENROLL source/dest membership → updated
position/occupancy`.

- **R1a posture:** the **decision** is resident — the R4 composite read + exact-magnitude threshold
  crossing runs on GPU and writes a `MovementRequest` event row into the resident journal (§3.2). The
  **application** (membership scatter, `cell_index` update, IDROUTE identity preservation) is the CPU
  boundary-maintenance pass. `BoundaryRequest` is therefore a **GPU-written event row**, consumed by CPU
  at the boundary. CPU involvement is limited to deterministic structural bookkeeping — **no planner**.
- **R1b target (`RESIDENT-EVENTLOG-0`):** make the journal itself fully resident and have the boundary
  consumer read it without CPU re-derivation of which rows exist (CPU still applies scatter).
- **R1c target (`RESIDENT-REENROLL-0`):** REENROLL becomes a GPU **scatter/compact** pass over a resident
  membership/free-list table (the §0.4 REENROLL free-list, ALLOC/IDROUTE substrate). This is the rung
  most likely to hit the M-4A / free-list-scatter stop-line; if so, **STOP** and define the next smaller
  rung. IDROUTE identity is preserved by carrying the identity lane as a resident column keyed to the
  cohort slot, never by CPU re-assignment.

---

## 5. Combat / production write-back (R1a posture + targets)

**Combat:** `hostile co-location → damage reduce-up → hostile disburse-down → emission-band ship loss →
num_ships decrement → removal only at zero`.

- **R1a:** reduce-up / disburse-down / emission-band run on GPU (already measured, integer bit-exact);
  the `num_ships` **decrement into the existing cohort slot is GPU-authoritative (Tier-A)**. **Removal**
  at zero ships is a `ShipLoss`→departure event applied in the CPU boundary pass (Tier-B). Next tick sees
  the decremented `num_ships` resident; removed slots are reconciled at the boundary.

**Production:** `construction threshold → ship_count_delta → masked compatible cohort selection →
num_ships increment OR local birth → friendly fusion`.

- **R1a:** construction threshold + ship_count_delta + masked compatible-cohort selection run on GPU; an
  increment into an **existing** compatible cohort slot is GPU-authoritative (Tier-A). **Birth** (new
  slot / ALLOC arrival) and **fusion** (compaction + lineage) are `ShipGain`/`FusionRequest` events
  applied in the CPU boundary pass (Tier-B).

**Residency requirements to graduate Tier-B → Tier-A (R1c):** `num_ships` updates already resident;
removals/departures need a resident free-list mark (no compaction); births need a resident ALLOC arrival
into a free slot; fusion needs a resident masked-reduction write + a resident lineage column. Each is a
candidate `R1c` deliverable, gated by the stop-lines.

---

## 6. Existing-substrate reuse classification

| Dependency | Verdict for `R1a` | Notes |
| --- | --- | --- |
| ATLAS-0080-0 sparse residency (single theater) | **sufficient as-is** | one resident theater; residency trace already used by R0A |
| Generic `AccumulatorOp` GPU path | **sufficient as-is** | R1/R2/R6/R6B reduce/disburse/threshold/emission-band already measured through it |
| `StructuredFieldStencil` (GradientXY) | **sufficient as-is** | R4 magnitude; ping-pong is the Tier-A double-buffer shape |
| GPU-EXEC / KERNEL substrate | **sufficient as-is** | dispatch mechanism |
| STORE-GPU masked reduction (ExactDeterministic) | **sufficient as-is** | owner-masked integer sums (stockpiles, combat) |
| Candidate-F / R4 f32 path | **sufficient as-is** | exact-sqrt magnitude authority; f32 bound unchanged |
| Resident **double-buffer field swap** | **sufficient with glue** | pair existing resident sessions; add boundary swap + parity assert. No new primitive. |
| Resident **GPU-written event journal** (Tier-B hand-off) | **sufficient with glue** | emission/threshold already emit rows; glue = a resident emission buffer the CPU drains at the boundary. No new op. |
| Mobility ALLOC/IDROUTE/OWNER/REENROLL substrate | **sufficient with glue for R1a (CPU boundary apply); insufficient-without-new-primitive for R1c resident scatter** | R1a applies structural ops on CPU at the boundary; resident scatter/compact (R1c) is the gated graduation and may hit the stop-line |

**Net:** `R1a` requires **no new runtime primitive, no new `AccumulatorOp`, no semantic WGSL** — only
glue: a resident double-buffer swap for Tier-A and a resident emission journal drained at the boundary
for Tier-B. That is exactly why `R1a` is the correct first rung.

---

## 7. Stop conditions (escalate to Opus — do not improvise)

`RUNTIME-0080-0-R1` (any sub-rung) must **STOP and return to Opus** if it requires any of:

- multi-atlas batching; M-4A masking-at-scale; system→planet recursion; multi-faction ECON expansion;
- semantic WGSL beyond the generic substrate; a new `AccumulatorOp`;
- an `docs/invariants.md` edit; a pinned-number change; a scenario reopen;
- a **CPU planner** or **CPU-side state manager pretending to be GPU authority**;
- loosening the R4 f32 bound (`1.0e-4`) or the Candidate-F exact-sqrt authority;
- default `SimSession` wiring.

If `R1a` field-column residency itself cannot reproduce the R6C integer trajectory / checksum
`1bba891c779190a4`, or R4 exceeds the bound: **STOP and report the delta** — do not loosen a bound.

If `R1c` resident REENROLL scatter/compact requires the parked free-list-scatter / M-4A machinery:
**STOP** and define the next smaller substrate rung (e.g. resident free-list mark-only, no compaction)
rather than telling Cursor to improvise.

No discrete GPU in the run environment → report "not measurable here"; never claim an unrun GPU result.

---

## 8. GPU posture (binding for report wording)

- A column/run may be called **`GPU-resident next-tick authoritative`** only if the GPU tick reads its
  own prior-tick resident output for that column (`gpu_state_feeds_next_tick == true`) on a real GPU run.
- For columns still applied by the CPU boundary pass (Tier-B in `R1a`), the posture stays exactly:
  `next-tick authority GPU-resident for field columns; structural maintenance applied at tick boundary
  via GPU-written event journal (CPU maintenance, not CPU authority)`.
- The CPU oracle remains the determinism reference. Do not claim GPU-measured next-tick authority unless
  resident GPU state actually feeds tick N+1.
- Do not suppress GPU posture as overclaim; do not claim residency that was not run.

---

## 9. First IMPL sub-rung — `RUNTIME-0080-0-R1a` (scope for Cursor)

**One-rung purpose.** Make the Tier-A field columns (§2) GPU-resident next-tick authoritative across the
R6C 100-tick run via a resident double-buffer swap, with Tier-B structural changes applied by a CPU
boundary-maintenance pass driven by a GPU-written event journal. Measure it; assert parity.

`R1a` must:

1. Stand up an opt-in/default-off resident double-buffer for the Tier-A columns; the GPU tick reads
   `buffer_cur` (state_N) and writes `buffer_next` (state_N+1); swap at the boundary.
2. Set `gpu_state_feeds_next_tick = true` for Tier-A and prove it (a test that fails if a CPU re-upload
   of Tier-A state is what feeds the next tick — see §11).
3. Route Tier-B structural changes through a resident GPU-written event journal drained by a bounded,
   deterministic CPU boundary-maintenance pass (no planner).
4. Assert CPU-oracle parity for the covered trajectory incl. checksum `1bba891c779190a4`; R4 within
   `1.0e-4`.
5. Keep the tick boundary as the save/pause/stable-state point (§1.3).
6. Emit a residency/authority trace + stable report checksum.

Anything beyond Tier-A authority (resident REENROLL scatter, resident birth/removal/fusion) is **not**
`R1a` — it is `R1b`/`R1c`, each its own rung, each behind the §7 stop-lines.

---

## 10. Required deliverables (for the `R1a` implementation handoff)

- **Report:** `docs/tests/runtime_0080_0_r1_next_tick_authority_results.md` — verdict PASS/PARTIAL/BLOCKED;
  whether Tier-A is true resident next-tick authority; per-column authority classification (Tier-A vs
  Tier-B-at-boundary); adapter identity; CPU-oracle parity verdict + checksum expected/observed; R4 delta
  vs bound; double-buffer swap + journal-drain description; readback boundaries; stop-line status; exact
  GPU posture wording (§8).
- **Production track:** `docs/design_0_0_8_0_consumer_pulled_production_track.md` — flip the
  RUNTIME-0080-0 forward note to the R1/R1a result; update R6C whole-run posture only to what was
  measured.
- **Worklog + mapping:** `docs/worklog.md`, `docs/workshop/mapping_current_guidance.md` — record the R1a
  result and what remains (R1b journal residency; R1c resident REENROLL/birth/removal/fusion behind
  stop-lines).
- **Do not edit** `docs/invariants.md`.

## 11. Required tests for `R1a` (must distinguish authority from mirror)

1. `runtime_0080_r1_gpu_state_feeds_next_tick_for_field_columns` — fails unless the GPU tick reads its
   own prior-tick resident Tier-A output.
2. `runtime_0080_r1_cpu_does_not_mutate_tier_a_between_ticks`.
3. `runtime_0080_r1_double_buffer_swap_is_tick_boundary_save_state`.
4. `runtime_0080_r1_structural_changes_routed_through_resident_event_journal`.
5. `runtime_0080_r1_cpu_boundary_pass_is_maintenance_not_planner` (asserts bounded, decision-free).
6. `runtime_0080_r1_field_column_parity_matches_r6c_checksum` (`1bba891c779190a4`).
7. `runtime_0080_r1_r4_f32_within_accepted_bound`.
8. `runtime_0080_r1_no_new_op_no_semantic_wgsl_no_atlas_batching_no_m4a`.
9. `runtime_0080_r1_no_scenario_reopen_or_invariant_edit`.
10. `runtime_0080_r1_report_checksum_stable`.
11. `runtime_0080_r1_reports_partial_if_any_tier_a_column_remains_cpu_authoritative`.

A test must never pass merely because a report field says so; authority is proven by the data-flow, not
by a string.

## 12. Operating rules / command discipline (binding for the implementation handoff)

- Windows PowerShell: run final `cargo test` / `cargo check` in the **foreground**, plain, with **no**
  stdout/stderr redirection (`2>&1`, `*>&1`, `Tee-Object`, or output pipes). See
  `.cursor/rules/no-shell-redirection.mdc` and `.cursor/rules/no-background-final-tests.mdc`.
- Opt-in/default-off; no default `SimSession` schedule change.
- No new semantic WGSL, no new op, no new invariant, no pinned-number change, no scenario reopen (§7).
- Save only required visibility under `docs/tests`; delete scratch/tmp/log outputs no longer needed.
- Do **not** commit `target/`, worktrees, local logs, or scratch files.
- Do **not** edit `docs/invariants.md` unless Opus explicitly opens a constitutional change.
- Do **not** claim GPU-measured next-tick authority unless resident GPU state actually feeds tick N+1.

## 13. Forward map (non-binding horizon)

- **`R1b` — `RESIDENT-EVENTLOG-0`:** fully resident event journal; boundary consumer reads it without CPU
  re-derivation (CPU still applies scatter).
- **`R1c` — `RESIDENT-REENROLL-0`:** resident scatter/compact for membership + cohort table (free-list
  mark; birth/removal/fusion) — behind the §7 stop-lines; STOP if M-4A/free-list-scatter is required.
- Then: multi-atlas batching + M-4A (§11 gate); richer emergence (`SCENARIO-0080-3`); multi-faction ECON;
  system→planet recursion.

These remain parked until `R1a` establishes resident field-column next-tick authority.
