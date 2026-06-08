# GPU-MEASURE-0080-0 — opening handoff (next track after SCENARIO-0080-2)

**Authoring authority:** Opus / design authority (`SCENARIO-0080-2-COMPLETE-0` ruling).
**Intended recipient:** Cursor / Codex5.5max — production implementation agent.
**Branch from:** latest `origin/master` after PR #526.
**Opens one focused PR.** This is a *measurement* track over already-accepted shapes, not a new
mechanism, not a new scenario, and not a reopening of `SCENARIO-0080-2`.

---

## 0. Why this is the next track

`SCENARIO-0080-2` is **closed**: the R1→R6B mechanism chain is proven and R6C ran the assembled stack
over one mutable 100-tick session with write-back (PR #526). Every rehearsal rung is described as
**`GPU-conformant; GPU execution not yet measured`**.

That phrase names the single most material standing gap in the vertical. SimThing's whole point
(design §0/§0.1) is **GPU-resident** computation; "GPU-conformant" is positive conformance to that
target, but it is still *unmeasured* for R4/R6/R6B/R6C. The next highest-value step is therefore not
more emergence (the rehearsal is done) and not more proof-wrappers — it is to **measure the existing
GPU-conformant row/mask/threshold/emission-band shapes actually executing on the GPU**, converting
"conformant" into "measured" against the CPU oracle.

This track was selected over richer-emergence and runtime-scheduler tracks because:

- The rehearsal mechanisms are complete; emergence robustness is a **recorded finding**, not a defect
  (see §3), and chasing it now would be the rehearsal-extension the anti-ceremony doctrine forbids.
- Closing the measurement gap directly strengthens the project's central claim with no redesign.
- The GPU substrate it needs **already exists and is accepted** (§2), so this is a consumer/measurement
  pass in the same shape as ATLAS-BATCH-0 STORE-GPU — not new substrate.

---

## 1. One-rung purpose

Measure the dress-rehearsal reduction/threshold/emission-band shapes **executing on the discrete GPU**
through the **existing accepted generic GPU path**, and assert parity against the CPU oracle that is
the determinism reference. Produce one report that states, per shape, exactly one of:

- **`GPU-measured (integer bit-exact)`** — integer masked-reduction shapes (R1 bounded recurrence
  accumulation, R2 reduce-up/disburse-down, R6/R6A emission-band attrition input sums, R6B construction
  threshold sums) matched bit-exact CPU↔GPU on the RTX 4080 ladder; **or**
- **`GPU-measured (verified-approximate, within accepted f32 bound)`** — the R4 GradientXY → exact-mag2
  → Candidate-F sqrt f32 chain, measured against the accepted NVIDIA FP-determinism bound; **or**
- **`GPU-conformant; GPU execution not yet measured`** — unchanged, for any shape that genuinely cannot
  be measured this pass (must be justified in the report, not left silent).

No behavior changes. No new gameplay. The CPU result remains the ground truth; the GPU number is
checked *against* it.

---

## 2. Use only the already-accepted GPU substrate

Map each rehearsal shape onto an existing accepted path. Do **not** author new semantic shaders.

- **Generic semantic-free AccumulatorOp GPU path** (AO-WGSL-0 fast path / GPU-EXEC + KERNEL-0..6
  substrate) — the masked `CMP_EQ`/`Sum`/reduce primitives.
- **ATLAS-BATCH-0 STORE-GPU** masked-reduction harness (integer bit-exact, validated cross-adapter on
  the RTX 4080 ladder) — the precedent and template for the integer-exact assertions.
- **Candidate-F exact-sqrt** (`m_jit_mag_f_from_exact_mag2`, artifact hash `e2e9e27601ee2e13`) and the
  NVIDIA FP-determinism ladder — the reference for the R4 f32 magnitude shape.

Express each rung's core reduction as an AccumulatorOp plan over the rehearsal's own pinned numbers and
run it through that path. The rehearsal already emits these as row/mask/reduce/disburse/threshold/
emission-band traces (R6C detector: "Modder-facing expressibility"), so the plans should be derivable
from existing rung outputs, not re-derived by hand.

---

## 3. Inherited findings from R6C (record, do not "fix" here)

These are **findings**, not defects, and are **out of scope** for this track. Do not tune the field,
extend ticks, or add mechanisms to make them emerge:

- **Terran patrols never crossed the movement threshold** in the canonical run (detector "Patrol
  response to disruption" = *Not observed*); the contest is one-sided (pirate-driven).
- **Race not resolved in 100 ticks** (detector *Partially emerged*): Terran 3→7 ships, Pirate 10→12;
  Terran stockpile grew to 356 but converted to few ships. Equilibrium is unanswered, not absent.
- **Front/standoff and self-sustaining pirate loop** are *Partially emerged* (legs present, not yet
  recurrent).
- **CPU-oracle parity is structural** (the R6C oracle re-runs the same deterministic model), consistent
  with the other rungs — adequate as a determinism reference, not an independent re-derivation.

These feed a **future** `SCENARIO-0080-3 — richer multi-hotspot emergence run`, which is *not* opened
now. They are not inputs to this measurement track.

---

## 4. Stop conditions (escalate to Opus, do not improvise)

- A rung shape **cannot** be expressed on an existing accepted GPU path **without new semantic WGSL or a
  new AccumulatorOp** → STOP. That is a substrate gap and a Tier-2 design-authority call.
- f32 (R4) parity **cannot reach the accepted NVIDIA FP-determinism bound** on the RTX ladder → STOP and
  report the measured delta; do not loosen any invariant or determinism bound to force a pass.
- The work would require editing `docs/invariants.md`, adding a new op, adding a default `SimSession`
  wiring, or changing any pinned rehearsal number → STOP.
- If no discrete GPU is available in the run environment, STOP and report "not measurable here" rather
  than claiming a GPU result. **Never claim GPU execution that was not actually run.**

---

## 5. Required deliverables

- **Report:** `docs/tests/gpu_measure_0080_0_results.md` — per-shape table (shape → AO plan → CPU value →
  GPU value → verdict `GPU-measured (integer bit-exact)` / `GPU-measured (verified-approximate)` /
  `GPU-conformant; not yet measured`), adapter/driver identity for any GPU run, and the parity method.
- **Production track:** `docs/design_0_0_8_0_consumer_pulled_production_track.md` — flip the
  GPU-MEASURE-0080-0 forward note (§12.5 trailer) to its result; update each affected rung's GPU posture
  line **only** to the measured wording, leaving mechanism claims untouched.
- **Worklog + mapping:** `docs/worklog.md` and `docs/workshop/mapping_current_guidance.md` — record the
  measurement result and what remains conformant-only.

## 6. GPU posture (binding for the report wording)

- GPU residency is the target; GPU-shaped is positive conformance, not a hedge.
- The CPU oracle remains the determinism reference the GPU path is checked against.
- A shape may be called **GPU-measured** only if it actually ran on a GPU this pass.
- Any unmeasured shape stays exactly `GPU-conformant; GPU execution not yet measured`.
- Do not suppress GPU posture language as overclaim; do not claim GPU execution unless measured.

## 7. Operating rules

- Windows PowerShell: run final `cargo test` in the **foreground** (no `block_until_ms: 0`); avoid bare
  `2>&1` on `cargo` (use plain `cargo test …` or `*>&1 | Tee-Object`).
- Default-off / opt-in for any new measurement harness; no change to default scheduling.
- No new semantic WGSL, no new op, no new invariant, no pinned-number changes (see §4).
- Save only required visibility under `docs/tests`; delete scratch/tmp/log outputs no longer needed.
- Do **not** commit `target/`, `.claude/worktrees/`, local logs, or replay LDJSON unless it is an
  explicit named report artifact.
- Do **not** edit `docs/invariants.md` unless Opus explicitly opens a constitutional change.

## 8. Acceptance

- At least the integer masked-reduction shapes (R1/R2/R6/R6B sums) are **GPU-measured integer
  bit-exact** against the CPU oracle on the RTX ladder, or a stop condition (§4) is hit and escalated.
- R4 f32 magnitude is either GPU-measured verified-approximate within the accepted bound, or escalated.
- The report states one explicit verdict per shape with no silent omissions.
- Upstream rehearsal suites (R1–R6C, ATLAS STORE CPU+GPU, mobility, FrontierV2) and
  `cargo check --workspace` remain green.
