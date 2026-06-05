# RUNTIME-0080-0 — production runtime tick scheduler over sparse-resident GPU shapes (design opening)

**Authoring authority:** Opus / design authority (`GPU-MEASURE-0080-0-COMPLETE-0` ruling).
**Track status:** OPEN (design opening; first rung scoped below).
**Implementation recipient (after this opening lands):** Cursor / Codex5.5max.
**Crosses a Tier-2 runtime boundary** — this opening defines scope and stop-lines so implementation
stays bounded.

This is the realization of the parked **M-4A sparse-residency scheduler / Atlas production runtime**
need. It does **not** reopen `SCENARIO-0080-2` and does **not** add a proof-wrapper rung.

---

## 0. Why this is the next track (and why it subsumes the R6C whole-run GPU gap)

After `GPU-MEASURE-0080-0` (PR #528), the standing inventory is:

- The R1→R6B **mechanisms are proven**; the R6C **integrated 100-tick run** executed with write-back;
  the **per-tick GPU shapes are now measured** on the RTX 4080 (R1/R2/R6/R6B integer bit-exact, R4
  verified-approximate within the f32 bound).
- The **only** remaining GPU posture caveat is R6C's *whole-run* execution:
  `GPU-conformant; GPU execution not yet measured`.

That whole-run gap is **not a measurement gap — it is a runtime-substrate gap.** Measuring the R6C run
"on GPU" honestly means holding world state **GPU-resident across ticks** and dispatching the
already-measured per-tick kernels from a scheduler — i.e. a production tick scheduler over sparse
residency. (This is exactly the stop-line the COMPLETE prompt set for candidate B: *"if whole-run GPU
execution requires new runtime substrate, stop and define a proper runtime track instead."*)

So this track is chosen over re-running the rehearsal for emergence (candidate A — a recorded finding,
not a bottleneck; better explored *on* the runtime later) and over a standalone R6C-whole-run
measurement (candidate B — which collapses into this substrate). It builds directly on accepted work:

- **`ATLAS-0080-0`** — accepted opt-in/default-off **sparse-residency nested mapping runtime**
  (starmap→starsystem→planet; descent/ascent residency reports; CPU oracle; residency changes *where*
  state lives, never values). This is the residency substrate.
- **`GPU-MEASURE-0080-0`** — the measured per-tick GPU shapes (the kernels to schedule).
- **`R6C`** — the integrated tick loop (the first consumer that needs cross-tick resident state).
- **GPU-EXEC / KERNEL substrate + the generic AccumulatorOp GPU path** — the dispatch mechanism.

---

## 1. First rung — `RUNTIME-0080-0-R0`: single-tier GPU-resident tick scheduler

**One-rung purpose.** Run the R6C 100-tick loop with world state held **GPU-resident across ticks** for
the **single galactic tier**, dispatching the already-measured per-tick shapes through the accepted GPU
path, and **measure it** — converting R6C's posture from `GPU-conformant; GPU execution not yet
measured` into a measured whole-run result for the single-tier scheduler.

Specifically R0 must:

1. Stand up an opt-in/default-off scheduler that owns one resident world buffer (positions, ship counts,
   stockpiles, disruption, construction/blockade state) and ticks it via the ATLAS-0080-0 residency
   model for **one** resident theater (no multi-atlas batching — see §3).
2. Per tick, dispatch the measured shapes (R1 recurrence, R2 reduce-up/disburse-down, R4 magnitude,
   R6 combat reduce/attrition, R6B threshold/fusion) on the GPU **without intermediate CPU readback of
   state between ticks** beyond what residency reporting already allows.
3. Assert **CPU-oracle parity**: the GPU-resident whole-run must reproduce the R6C CPU reference exactly
   for integer state (and within the accepted f32 bound for R4-derived quantities), including R6C's
   deterministic replay checksum `1bba891c779190a4` for the integer-determined trajectory.
4. Emit a residency/scheduler trace + a stable report checksum.

**Recipient:** Cursor / Codex5.5max, after this opening is accepted.

---

## 2. Non-goals for R0 (horizon, not this rung)

- Multi-atlas batching and **M-4A algebraic tile-local masking at scale** — gated; see §3.
- System→planet **recursive** tiering inside the scheduler (candidate E).
- **Multi-faction** ECON scaling beyond Terran/Pirate (candidate D).
- Default `SimSession` wiring, UI, real-time loop, scheduling/observation/control demos.
- Richer emergence topology (candidate A — future `SCENARIO-0080-3`).
- Any new gameplay behavior. R0 changes *where/when* the proven shapes execute, never *what* they compute.

---

## 3. Stop conditions (escalate to Opus — do not improvise)

- **Atlas-batching admission gate.** `request_atlas_batching` is rejected at admission until a
  §11-gate-passing M-4 PR. R0 must stay **single resident theater**; if R0 appears to require multi-atlas
  batching or M-4A masking, **STOP** — that is a distinct gated rung, not R0.
- **New substrate primitive.** If GPU-resident cross-tick state needs a runtime primitive **not** present
  in ATLAS-0080-0 / the GPU-EXEC·KERNEL substrate / the AccumulatorOp GPU path, **STOP and define that
  primitive as its own rung** (do not smuggle it into R0).
- **Semantic WGSL / new op.** Any need for new semantic WGSL or a new `AccumulatorOp` → **STOP**
  (Tier-2 design-authority call).
- **Determinism.** If GPU-resident whole-run parity cannot reproduce the R6C integer trajectory, or R4
  exceeds the accepted f32 bound, **STOP and report the delta** — do not loosen any invariant, pinned
  number, or determinism bound to force a pass.
- **No discrete GPU available** in the run environment → report "not measurable here"; never claim an
  unrun GPU result.
- Any pressure to edit `docs/invariants.md`, change pinned rehearsal numbers, or reopen a closed scenario
  → **STOP**.

---

## 4. GPU posture (binding for report wording)

- **GPU residency remains the target**; GPU-shaped is positive conformance, not a hedge.
- The **CPU oracle remains the determinism reference** the GPU-resident run is checked against.
- A whole-run result may be called **`GPU-measured`** only if the scheduler actually ran resident on a
  GPU this pass; otherwise the posture stays exactly `GPU-conformant; GPU execution not yet measured`.
- Do not suppress GPU posture as overclaim; do not claim GPU execution unless measured.

---

## 5. Required deliverables

- **Report:** `docs/tests/runtime_0080_0_r0_results.md` — scheduler/residency trace summary, per-tick
  dispatch confirmation, CPU-oracle parity verdict, R6C trajectory/checksum match, adapter/driver
  identity for the GPU run, and the resulting R6C whole-run GPU posture (`GPU-measured` or unchanged with
  justification).
- **Production track:** `docs/design_0_0_8_0_consumer_pulled_production_track.md` — flip the
  RUNTIME-0080-0 forward note to its R0 result; if R0 measures the whole run, update R6C's GPU posture
  line accordingly (mechanism claims untouched).
- **Worklog + mapping:** `docs/worklog.md` and `docs/workshop/mapping_current_guidance.md` — record the
  R0 result and what remains for later rungs (multi-atlas/M-4A behind §11, recursion, faction scaling).

## 6. Operating rules / scratch policy

- Windows PowerShell: run the final `cargo test` in the **foreground** (no `block_until_ms: 0`); use plain
  `cargo test …` with **no** stdout/stderr redirection (`2>&1`, `*>&1`, `Tee-Object`, or output pipes).
- Opt-in/default-off; no default `SimSession` schedule change.
- No new semantic WGSL, no new op, no new invariant, no pinned-number change, no scenario reopen (§3).
- Save only required visibility under `docs/tests`; delete scratch/tmp/log outputs no longer needed.
- Do **not** commit `target/`, `.claude/worktrees/`, local logs, or scratch files.
- Do **not** edit `docs/invariants.md` unless Opus explicitly opens a constitutional change.

## 7. Acceptance for R0

- The R6C 100-tick run executes with state held GPU-resident across ticks for one theater, dispatching
  the measured per-tick shapes, **or** a stop condition (§3) is hit and escalated with specifics.
- CPU-oracle parity holds (integer trajectory reproduces R6C incl. checksum `1bba891c779190a4`; R4 within
  the accepted f32 bound), or the delta is reported and escalated.
- R6C whole-run GPU posture is updated to a measured verdict **only** if actually measured.
- Upstream suites (R1–R6C, GPU-MEASURE-0080-0, ATLAS STORE CPU+GPU, ATLAS-0080-0, mobility, FrontierV2)
  and `cargo check --workspace` remain green.

---

## 8. Forward map (non-binding horizon)

> **R0 result (R0A remedial, 2026-06-05):** R0 landed as **IMPLEMENTED / PARTIAL** — a persistent
> GPU-session **mirror-dispatch** scheduler, not GPU-resident next-tick authority. CPU R6C remains tick
> authority; per-tick shapes GPU-dispatched (`inter_tick_world_readbacks=0`, checksum `1bba891c779190a4`).
> Report: [`../tests/runtime_0080_0_r0_results.md`](../tests/runtime_0080_0_r0_results.md).

> **R1 OPEN (`RUNTIME-0080-0-R1-DESIGN-0`, 2026-06-05, Opus):** the substrate that makes GPU-resident
> world state the **input authority for tick N+1** is now defined as the primitive **`GPU-STATE-AUTH-0`**.
> First IMPL sub-rung **`RUNTIME-0080-0-R1a`** promotes the already-measured **Tier-A field columns**
> (disruption, location_status, stockpiles, construction_progress, per-cohort `num_ships` value,
> blockade/divert code, R4 magnitude) to a resident double-buffered next-tick authority; **Tier-B**
> structural changes (REENROLL membership scatter, cohort birth/removal, fusion lineage) are applied by a
> bounded CPU **boundary-maintenance** pass driven by a **GPU-written event journal** (the boundaryEvent
> dispatch), not a CPU planner. Spec:
> [`runtime_0080_0_r1_next_tick_authority_spec.md`](runtime_0080_0_r1_next_tick_authority_spec.md).

- **R1a:** resident field-column next-tick authority (no new op / WGSL / atlas batching / M-4A).
- **R1b (`RESIDENT-EVENTLOG-0`):** fully resident event journal.
- **R1c (`RESIDENT-REENROLL-0`):** resident scatter/compact for membership + cohort table — behind the
  §11 / free-list-scatter stop-lines; STOP and define a smaller rung if M-4A is required.
- **Later:** multi-atlas batching + M-4A algebraic tile-local masking (§11 gate); recursive
  system→planet tiering (candidate E); multi-faction ECON scaling (candidate D); richer emergence run
  (candidate A / `SCENARIO-0080-3`).
