# RUNTIME-0080-M4A-OPEN-0 — multi-theater sparse-residency dispatch of the closed runtime

> ## ⛔ SUPERSEDED 2026-06-07 by design authority on product mandate.
> This opening was **mis-scoped**. It defined M-4A as running **N independent parallel** copies of the
> flat R2 loop — which is *not* what the product specified. The governing dress-rehearsal spec calls for a
> **nested containment hierarchy** (galaxy → star-system 10×10 subgrid → planet → planet-surface 10×10 →
> pop-cohort + factory building children), which the closed rehearsal silently flattened (see the
> Deviation Record in [`runtime_0080_0_r2_stable_100_tick_rehearsal_results.md`](../tests/runtime_0080_0_r2_stable_100_tick_rehearsal_results.md)).
> The correct next track is **nested residency + planet-surface economy**, defined in
> [`runtime_0080_recursive_rehearsal_opening.md`](runtime_0080_recursive_rehearsal_opening.md). Multi-theater
> *sparse residency* is consumed there as the residency mechanism for the nested hierarchy — not as
> standalone parallel theaters. Do not implement against this superseded opening.

> **Status: SUPERSEDED (was: OPEN capability track, 2026-06-06).**
> Selected as the next consumer after `RUNTIME-0080-0-R2-REVIEW-0` (RUNTIME-0080-0 CLOSED).
> **This is a capability/infrastructure track, not a gameplay consumer.** No gameplay
> overclaim until a named gameplay consumer uses it.

**Intended recipient:** Cursor / Codex production implementation agent
**Authority:** design authority, after PR #552
**Role:** capability implementation agent
**Objective:** prove the closed single-theater 100-tick GPU-forward runtime can run across
more than one resident theater under one sparse-residency dispatch, without adding a CPU
planner, without scenario reopen, and without gameplay overclaim.

## Why this track exists (and its honest framing)

RUNTIME-0080-0 closed at the stable single-theater 100-tick GPU-forward rehearsal. The design
authority's recommended next consumer was richer emergence (`SCENARIO-0080-3`); the project
instead selected **M-4A / multi-atlas sparse residency**. M-4A is **substrate/capability**, so
this track is opened explicitly as a **bounded capability-readiness proof**, not as evidence of
any gameplay phenomenon. It earns the right to be built by proving multi-theater residency works
and scales sparsely — it does **not** claim emergence, balance, or product behavior.

## Prior art this track must consume, not redo

- **`ATLAS-BATCH-0` (atlas batch allocation, IMPLEMENTED/PASS):** packs homogeneous tiles into a
  static multi-theater map. Batch allocation only; isolated from the residency scheduler.
- **`atlas_0080_0` (scenario-scoped sparse-residency nested mapping, IMPLEMENTED/PASS for
  `SCENARIO-0080-1`):** materialize/reside only active cells per theater, scenario-scoped.
- **`RUNTIME-0080-0` R1a–R1c-f + R2 (CLOSED):** the resident single-theater 100-tick GPU-forward
  loop with GPU-decided ZeroCohort and R6C checksum-equivalence.

What remains genuinely parked, and what this track opens: a **generic resident multi-theater
scheduler** that runs the closed RUNTIME-0080-0 stack across N theaters in one sparse-residency
dispatch pass — independent of any single scenario fixture.

## First slice (smallest honest unit)

`RUNTIME-0080-M4A-0 — multi-theater resident dispatch of the R2 loop under sparse residency`

Implement a runner that drives **N ≥ 2 independent resident theaters** of the existing
RUNTIME-0080-0 100-tick GPU-forward loop under one scheduler, where:

1. each theater runs the full R1a Tier-A + R1b journal + R1c-a→e substrate + R1c-f GPU ZeroCohort
   per-tick loop the closed R2 runner already proves for one theater;
2. **sparse residency**: inactive theaters incur ~0 per-tick dispatch and ~0 steady-state
   resident buffer growth (only active theaters materialize);
3. **no cross-theater leakage**: theater A's resident state never reads or writes theater B's;
   prove with a disabled-isolation negative control (deliberately crossing isolation must break
   parity);
4. **per-theater parity preserved**: each theater independently reaches the same per-tick journal
   parity and Tier-A endpoint parity it reaches single-theater; with the canonical seed each
   theater is checksum-equivalent to pinned R6C `1bba891c779190a4`;
5. **single-theater R2 unchanged** (regression: the existing `runtime_0080_0_r2` suite stays green).

Reuse the closed R2 runner as the per-theater unit. The new code is the **scheduler/orchestration
layer + sparse-residency accounting**, not a reimplementation of the tick loop.

## Acceptance measures

- `runs_multi_theater = true` with `theater_count ≥ 2`;
- every active theater: per-tick journal parity = true, Tier-A tick-100 parity = true;
- canonical-seed theaters: checksum-equivalent to `1bba891c779190a4` (claim boundary identical to
  R2 — assigned on parity, not a fresh hash; do not claim a fresh multi-theater hash unless one is
  actually computed and validated);
- `sparse_residency_inactive_theater_dispatch = 0` (inactive theaters not dispatched);
- resident GPU buffer bytes scale with **active** theater count, not total theater count
  (report the figure, in the style of the R2 profiling capture);
- `cross_theater_leakage = false` with a passing disabled-isolation negative control;
- `gpu_decided_zero_cohort_per_theater = true`; CPU witness still excludes ZeroCohort;
- `cpu_planner_added = false`; `new_semantic_gpu_op = false`.

## What must be true to PASS

A report-only aggregation of single-theater results is **not** this slice. PASS requires a real
multi-theater scheduler that actually drives ≥2 resident theaters in one run and proves sparse
residency + isolation + per-theater parity.

## What stays parked

- gameplay emergence / balance claims (that is `SCENARIO-0080-3`, not opened);
- default `SimSession` wiring;
- multi-faction economy generality (`ECON-0080-MULTIFACTION`, not opened);
- system→planet recursion (`RECURSION-0080-SYSTEM-PLANET`, not opened);
- class-by-class GPU conversion of remaining CPU-decided structural classes (DamageDelta,
  MoveRequest, LocalBirthRequest, FusionRequest, ShipCountDelta, OwnerCodeFlip) — still findings,
  convert only if a theater concretely requires it;
- `docs/invariants.md` edits;
- pinned-number changes;
- SCENARIO-0080-2 reopen.

## Stop conditions (return to design authority)

Stop if the slice requires:

- an invariant edit or pinned-number change;
- a scenario reopen or default session wiring;
- a new semantic GPU operation tied only to one fixture;
- a CPU planner masquerading as GPU authority;
- a new copy-substrate rung with no consumer;
- evidence that single-theater R2 must change to support multi-theater (that would reopen
  RUNTIME-0080-0 — return first).

## Suggested files (for the implementation handoff)

Create:
- `crates/simthing-driver/src/runtime_0080_m4a.rs` (multi-theater scheduler over the R2 unit)
- `crates/simthing-driver/tests/runtime_0080_m4a.rs`
- `docs/tests/runtime_0080_m4a_multi_theater_results.md`

Update:
- `crates/simthing-driver/src/lib.rs`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/worklog.md`
- `docs/workshop/mapping_current_guidance.md`

Do not edit:
- `docs/invariants.md`

## Required foreground commands (no shell redirection, no Tee-Object, no pipes)

```powershell
cargo test -p simthing-driver --test runtime_0080_m4a
cargo test -p simthing-driver --test runtime_0080_0_r2
cargo test -p simthing-driver --test runtime_0080_0_r1c_f
cargo test -p simthing-driver --test atlas_0080_0
cargo test -p simthing-gpu
cargo build --workspace
cargo fmt --all -- --check
cargo check --workspace
```

(If any predecessor target is absent, record the exact absence; do not invent a pass.)

## Report

`docs/tests/runtime_0080_m4a_multi_theater_results.md` must include: verdict; adapter; theater
count; per-theater parity + checksum-equivalence; sparse-residency dispatch accounting; resident
buffer bytes vs active-theater count; isolation negative-control result; confirmation that no
gameplay claim is made; confirmation that single-theater R2 is unchanged; foreground commands and
results; scratch/log cleanup confirmation.
