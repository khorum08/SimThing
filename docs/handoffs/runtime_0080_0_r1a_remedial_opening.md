# Codex Handoff — RUNTIME-0080-0-R1a-REMEDIAL-0

**From:** Opus (design authority) · **To:** Cursor / Codex5.5max (implementation) · **Date:** 2026-06-05
**Rung:** `RUNTIME-0080-0-R1a-REMEDIAL-0` — correct the faked Tier-A authority and unify next-tick
authority onto the production overlay / intent / threshold substrate.
**Primitive:** `GPU-STATE-AUTH-0` (unchanged). **Branch from latest `origin/master` after PR #534.**

> **This handoff expects oversight. The previous R1a result was audited and found to be an overclaim.**
> Read §6 (Anti-faking oversight protocol) before writing any code. The reviewer will re-run the
> negative-control and independence checks; a green suite that does not include them is not acceptance.

---

## 0. Read first — fixed base harness (cite on handoff back)

1. `docs/design_0_0_8_0.md` §0 (transient constitution; SimThing Maximality; §0.5 self-check).
2. `docs/invariants.md` (Scenario Proof; AccumulatorOp v2; Resource Flow Substrate). **Do not edit.**
3. `docs/production_paths/runtime_0080_0_r1_next_tick_authority_spec.md` (the `GPU-STATE-AUTH-0` design;
   Tier-A/B/C; write-back contract; stop-lines — see the new §14 R1a-REMEDIAL section).
4. `crates/simthing-driver/src/runtime_0080_0_r1a.rs` + `tests/runtime_0080_0_r1a.rs` (the code being
   corrected).
5. `docs/tests/runtime_0080_0_r1a_next_tick_authority_results.md` (the overclaimed report; carries a
   correction banner).

**Rung-local (≤3, ephemeral):**
- `crates/simthing-feeder/src/work.rs` + `patcher.rs` + `dispatcher.rs` (player/AI overlay → folded
  `IntentDelta` → GPU; Pass 0–7 order).
- `crates/simthing-gpu/src/world_state.rs` (`OverlayDelta`, `IntentDelta`, `ThresholdEvent`,
  `ThresholdRegistration`) and `passes.rs` (`run_tick_pipeline*`).
- `docs/tests/gpu_measure_0080_0_results.md` (the R6C Tier-A shapes already GPU-measured through generic
  ops — the transforms you will register).

**§0.5 self-check (one line on handoff back):** every change must hold (1) everything is a SimThing /
no `match kind`; (2) all conflict is resource flow; (3) recursive allocation; (4) **GPU threshold
crossings, not a CPU planner**; (5) semantic-free `simthing-sim` + CPU-oracle bit-exact parity;
(6) proven through a real reduction, opt-in/default-off. If a change can't fit 1–6, **STOP and escalate**.

---

## 1. Situation — why this remedial exists

`RUNTIME-0080-0-R1a-IMPL-0` (PR #534, merged) claimed **PASS** for "Tier-A GPU-STATE-AUTH-0 resident
next-tick authority." Audit found the claim is not earned:

- The CPU recomputes the **entire Tier-A next-state trajectory** (`build_tier_a_oracle_states`):
  `bounded_feedback_next` for disruption, `r1a_diffusion_status` for location_status, and the
  stockpile/economy/construction/combat/reinforcement/fusion next-values are read straight out of the
  CPU R6C report rows.
- Each tick, that CPU-computed `state_N+1` is **uploaded into the GPU** (`write_slot_col_values` into
  `COL_JOURNAL_DELTA`), and the GPU "tick" is **three `Identity` copies** (`NEXT:=CURRENT`;
  `NEXT:=JOURNAL` overwrite; `CURRENT:=NEXT` swap). The GPU performs **pure data movement** — it does
  **not** compute the disruption recurrence, economy reduce/disburse, R4 magnitude, or combat/production
  emission.
- `gpu_state_feeds_next_tick == true` is therefore satisfied **mechanically** (the swapped buffer is the
  next read) while the **CPU remains the computational authority** for the transition — the exact R0A
  gap in a more elaborate costume.
- The report's `inter_tick_tier_a_upload_count = 0` (and the per-tick trace `cpu_tier_a_uploads_this_tick
  = 0`) are **hardcoded and inaccurate**: the per-tick journal write *is* an inter-tick Tier-A CPU→GPU
  upload of the next state. The test that "guards" it asserts the hardcoded constant — a tautology.
- No test distinguishes "GPU computed the transition" from "GPU copied an injected next-state." The
  per-column `gpu_authoritative / cpu_oracle_parity / integer_bit_exact` flags are hardcoded by the
  `covered_columns(true)` constructor, not measured.

**Ruling:** `RUNTIME-0080-0-R1a-IMPL-0` is **downgraded to IMPLEMENTED / PARTIAL (SCAFFOLD)** — it
demonstrated the resident double-buffer + boundary-swap choreography, nothing more. It is **not**
GPU-resident next-tick authority. This remedial supersedes it.

---

## 2. The opportunity (binding scope) — unify, don't fork

The production overlay/intent/threshold runtime **already is** a GPU-resident next-tick authority for
the columns it drives: Pass 0 snapshots resident `values`; Pass 3 applies player/AI `OverlayDelta`
on-GPU; AccumulatorOp intent applies folded `IntentDelta` on-GPU; Passes 4–6 reduce; Pass 7 thresholds →
`ThresholdEvent`; the result **stays resident** for the next tick with no CPU re-upload of field columns.
That is exactly the property R1a must demonstrate — and the production path earns it honestly (the GPU
computes the transform).

R1a-IMPL-0 instead built a third, private residency mechanism (a hand-rolled journal + Identity copies)
disjoint from that pipeline. That is drift away from SimThing Maximality's one-substrate principle.

**Binding directive:** the remedial must make the GPU the actual computational authority for the Tier-A
columns by **registering the R6C per-tick Tier-A transforms as `AccumulatorOp`s / overlays over a
resident `values` buffer and letting the existing tick pipeline (`WorldGpuState` + `Pipelines` Pass 0–7)
produce `state_N+1` on GPU** — the same machinery that already applies player/AI overlays and SEAD
thresholds. The shapes are the ones already GPU-measured in `GPU-MEASURE-0080-0`; **no new op, no new
semantic WGSL** is needed or permitted.

This simultaneously fixes the correctness gap **and** unifies the runtime: player direction
(`PlayerIntentOverlay`), AI intent (`AiIntentOverlay`), SEAD threshold acts
(`ThresholdEvent → BoundaryRequest`), and the resident next-tick transition all ride **one** substrate.

---

## 3. Required correction — choose one honest outcome

### Outcome A — true Tier-A GPU authority on the production substrate (preferred)

Implement Tier-A next-tick authority such that **the GPU computes `state_N+1` from resident `state_N`**:

- Stand up an opt-in/default-off harness over `WorldGpuState` + `Pipelines` (the production tick
  pipeline), seeded **once** from the R6C initial world.
- Register the Tier-A transforms (R1 bounded-feedback recurrence, R1 diffusion sink, R2 reduce-up/
  disburse-down, R6 attrition decrement on existing slots, R6B reinforcement increment on existing
  slots, blockade/divert code, R4 magnitude scratch) as `AccumulatorOp`s / `OverlayDelta` / threshold
  registrations over the resident `values` buffer.
- Player/AI inputs (if any in this harness) enter as `OverlayDelta` / folded `IntentDelta` through the
  **same** pipeline — never as a side channel.
- The CPU R6C oracle is **comparison-only**, computed independently, read **only** at the boundary
  parity check. No oracle value may flow into the resident buffer after the single seed.
- Parity: the GPU-produced trajectory reproduces the R6C integer trajectory incl. checksum
  `1bba891c779190a4`; R4 within `1.0e-4`.
- Report may then claim Tier-A `GPU-resident next-tick authoritative` (§5 posture).

### Outcome B — honest PARTIAL/BLOCKED with a named gap

If a stop-line (§4) actually blocks Outcome A — e.g. a Tier-A transform cannot run through the existing
generic ops without a new op/semantic WGSL, or the production pipeline cannot be driven opt-in without
default `SimSession` wiring — **do not fake it.** Report PARTIAL/BLOCKED, name the precise substrate gap,
and define the next smaller rung. A correct PARTIAL is acceptance; a faked PASS is not.

**Forbidden resolution:** retaining the IMPL-0 mechanism (CPU-computed next-state injected per tick +
Identity copies) and calling it PASS. That is the defect being corrected.

---

## 4. Stop conditions (escalate to Opus — do not improvise)

STOP and return to Opus if the work requires any of: multi-atlas batching; M-4A masking-at-scale;
system→planet recursion; multi-faction ECON expansion; semantic WGSL beyond the generic substrate; a new
`AccumulatorOp`; an `docs/invariants.md` edit; a pinned-number change; a scenario reopen; a CPU planner;
**a CPU-side state manager pretending to be GPU authority** (the IMPL-0 defect); loosening the R4 f32
bound; default `SimSession` wiring.

Tier-B remains out of scope: arena membership / REENROLL scatter, cohort birth/removal, cell-index
movement, fusion lineage/compaction stay **bounded CPU boundary maintenance driven by GPU-written events**
(`ThresholdEvent → BoundaryRequest`). Do not make Tier-B GPU-authoritative here (that is R1b/R1c).

If no discrete GPU is present: report `BLOCKED` honestly; never claim an unrun GPU result.

---

## 5. GPU posture (binding report wording)

- A Tier-A column may be called **`GPU-resident next-tick authoritative`** only if **the GPU transform
  produced `state_N+1` from resident `state_N`** on a real GPU run, with **zero inter-tick CPU writes of
  Tier-A state** (measured, §6) and the **negative control (§6.2) failing parity** when the GPU transform
  is disabled.
- If `state_N+1` is computed on CPU and moved onto the GPU in any form, the posture is **not** authority;
  it is mirror/scaffold and must say so.
- The CPU oracle remains the determinism reference. Do not claim GPU-measured next-tick authority unless
  resident GPU state, produced by the GPU transform, feeds tick N+1.

---

## 6. Anti-faking oversight protocol (BINDING)

This rung is under audit because the prior one faked authority. The following are **mandatory** and will
be re-run by the reviewer. A PASS that omits any of these is rejected.

### 6.1 Independence — the GPU transform is the only producer of `state_N+1`
- After the **single** seed upload, there must be **zero** `write_*` / `upload_*` calls that carry
  Tier-A **next-state** values (oracle-derived or otherwise) into the resident buffer. The only resident
  writes permitted between seed and final readback are the GPU transform dispatches themselves and
  legitimate **input** overlays/intents (player/AI), which are *inputs*, not precomputed next-state.
- Oracle next-state values (`report.*_rows`, `bounded_feedback_next`/diffusion recomputation, etc.) may
  be used **only** in the comparison path, never written to the resident buffer.

### 6.2 Negative control — parity must depend on GPU compute
- Provide a test that **disables / perturbs the GPU Tier-A transform** (e.g. replaces the registered
  transform ops with no-ops, or perturbs the seed) and asserts the run **FAILS** parity with the oracle.
- If parity still passes with the transform disabled, the transform is not the producer → the result is
  faked → the test must fail the build. This is the single most important guard.

### 6.3 Measured counters — no hardcoded metrics
- `inter_tick_tier_a_upload_count`, `inter_tick_readback_count`, `gpu_dispatch_count`, swap counts, and
  the per-column authority flags must be **incremented/observed at the actual call sites**, not assigned
  as constants. Instrument the session/state wrapper so the count reflects real GPU traffic.
- A test must assert `inter_tick_tier_a_upload_count == 0` **by measurement**, and must fail if a per-tick
  next-state upload is reintroduced.

### 6.4 Earned per-column parity — measured, not constructed
- Each covered column's `gpu_authoritative / cpu_oracle_parity / integer_bit_exact` flag must be derived
  from an **actual GPU-vs-oracle comparison of a GPU-produced value**, not from a constructor literal.

### 6.5 Source-shape guard
- The registered Tier-A transform must use real combine functions (e.g. `EvalEML` for the recurrence,
  `Sum` for reduce/disburse, the structured-field/Candidate-F path for R4) — **not** an `Identity`-only
  copy of an injected column. A test (or the report) must demonstrate the transform ops are the measured
  R6C shapes, not Identity passthroughs.

### 6.6 Report must carry the anti-fake evidence
The results doc (§7) must include: the negative-control result; the measured upload/readback/dispatch
counts; a statement that the oracle is comparison-only and lists where it is read; and the exact GPU
posture (§5). Omitting the negative control = not acceptance.

---

## 7. Required tests (replace the tautological ones)

Keep the genuine guardrail tests; **replace** the metric-tautology tests with measured ones, and **add**:

1. `r1a_gpu_transform_is_sole_producer_of_state_n_plus_1` (independence; §6.1).
2. `r1a_negative_control_disabling_gpu_transform_fails_parity` (§6.2).
3. `r1a_inter_tick_tier_a_uploads_zero_by_measurement` (§6.3).
4. `r1a_no_oracle_value_written_to_resident_buffer_after_seed` (§6.1).
5. `r1a_covered_column_parity_is_measured_not_constructed` (§6.4).
6. `r1a_tier_a_transform_uses_measured_shapes_not_identity` (§6.5).
7. `r1a_gpu_state_feeds_next_tick_true` (kept — but now meaningful because of 1–2).
8. `r1a_field_column_parity_matches_r6c_checksum` (`1bba891c779190a4`).
9. `r1a_r4_f32_within_accepted_bound` (measured delta).
10. `r1a_tier_b_structural_ops_boundary_maintained_via_threshold_event_not_planner`.
11. `r1a_no_new_op_no_semantic_wgsl_no_atlas_batching_no_m4a_no_invariant_edit`.
12. `r1a_opt_in_default_off` and `r1a_report_checksum_stable`.

A test must never pass because a report field says so; authority is proven by data-flow and the negative
control.

---

## 8. Deliverables

- **Code:** correct `crates/simthing-driver/src/runtime_0080_0_r1a.rs` (+ tests) to Outcome A or B. Remove
  the CPU-injection + Identity-copy producer. Reuse `WorldGpuState`/`Pipelines`/`OverlayDelta`/
  `IntentDelta`/`ThresholdEvent` rather than a private journal.
- **Report (rewrite):** `docs/tests/runtime_0080_0_r1a_next_tick_authority_results.md` — verdict
  PASS/PARTIAL/BLOCKED; the §6 anti-fake evidence; per-column measured parity; adapter; checksum
  expected/observed; R4 delta vs bound; exact posture (§5). Update the stable report checksum.
- **Production track:** `docs/design_0_0_8_0_consumer_pulled_production_track.md` — flip the R1a note to
  the corrected result.
- **Worklog + mapping:** `docs/worklog.md`, `docs/workshop/mapping_current_guidance.md`.
- **Do not edit** `docs/invariants.md`.

---

## 9. Command discipline (binding — PowerShell crash + foreground)

- Run final `cargo test` / `cargo check` in the **foreground**, plain, with **no** stdout/stderr
  redirection (`2>&1`, `*>&1`, `Tee-Object`, or output pipes). See `.cursor/rules/no-shell-redirection.mdc`
  and `.cursor/rules/no-background-final-tests.mdc`. One native command per shell invocation.
- Opt-in/default-off; no default `SimSession` schedule change.
- Save only required visibility under `docs/tests`; delete scratch logs no longer needed; do not commit
  `target/`, worktrees, or scratch files.
- Do **not** claim GPU-measured next-tick authority unless the GPU transform produced `state_N+1` and the
  negative control fails parity without it.

---

## 10. Acceptance

Acceptance = Outcome A with all of §6 satisfied (independence + negative control + measured counters +
earned parity + source-shape guard + report evidence), checksum `1bba891c779190a4`, R4 within bound, and
the production-substrate unification (no private journal; player/AI overlays + SEAD threshold + next-tick
transition on one substrate). **Or** a correct Outcome B PARTIAL/BLOCKED naming the precise gap. Anything
that reproduces the IMPL-0 inject-and-copy pattern under a PASS label is rejected.

*Recipient after Opus acceptance of this opening:* Cursor / Codex5.5max.
