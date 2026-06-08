# DEMO-0080-0 — Opening Review Visibility Report

**Date:** 2026-06-02
**Verdict:** **OPEN-WITH-NARROWING** (Option A). Docs/design only; no code.

## Decision rationale

The Local Patrol Economy vertical slice is now complete and implemented/pass end-to-end: bounded command
admission (`admit_control_0080_0`, `Control0080CommandBatch::canonical_run()`, `replay_admit_control_0080_0`)
→ schedule (`run_default_schedule_0080_0`) → read-only observation export (`observe_gameplay_0080_0`). A
demo gate is **pure packaging** over these existing seams: apply a canonical bounded command batch, run
the existing path, emit the existing deterministic transcript/export. It adds **no new simulation
behavior, no new substrate, and no decision logic** — FIELD_POLICY remains the sole mover-decision source.

This is the natural "make the finished vertical slice reproducible/usable" step at the designer-facing
barrier — not substrate-ahead-of-need and not a passive proof wrapper. **No issues detected.**

**Option B (park)** rejected: there is a clear, narrow usability pull (a reusable headless demo over the
completed slice) with zero new behavior. **Option C (remediation)** rejected: all prior gates are
implemented/pass and docs are consistent — no blockers.

## Files touched

| File | Action |
|---|---|
| `docs/gameplay/demo_0080_0_opening_spec.md` | Created — opening spec |
| `docs/tests/phase_demo_0080_0_opening_review_results.md` | Created — this report |
| `docs/design_0_0_8_0_consumer_pulled_production_track.md` | Updated — ladder row + closed/parked paragraph |
| `docs/workshop/mapping_current_guidance.md` | Updated — status line |
| `docs/worklog.md` | Updated — top entry |

## Current completed gates

- `SCENARIO-0080-0` — ACCEPTED.
- `PRODUCTION-PATH-0080-0` — IMPLEMENTED / PASS.
- `DEFAULT-SCHEDULE-0080-0` — IMPLEMENTED / PASS (1A + 1B, deterministic cat-and-mouse).
- `GAMEPLAY-0080-0` — IMPLEMENTED / PASS (read-only observation export).
- `CONTROL-0080-0` — IMPLEMENTED / PASS (bounded command admission).

## Opened / closed status after this decision

- `DEMO-0080-0` — **OPEN WITH NARROWING** (headless Local Patrol Economy demo/export packaging; no
  implementation).
- Direct movement control, externally-scripted boundary requests, player command loop, UI framework,
  real-time loop, global default schedule, new schedule implementation, semantic WGSL, new shader/GPU
  kernel, CPU planner, ClauseThing/L3, Hybrid-Strata/faction-index scaling, atlas runtime, E-11B-5, B-1,
  FrontierV2-5, ACT/EVENT/OBS/PIPE — remain CLOSED / PARKED.

## Future implementation slice

A future PR may add a headless demo/export **helper** for Local Patrol Economy that uses existing
`CONTROL-0080-0` admission, applies a canonical bounded command batch, runs the existing
`control → schedule → observation/export` path, emits the existing deterministic transcript (optionally a
golden transcript), keeps all regression tests green, and updates docs. It must not add a CLI binary, UI
framework, interactive commands, real-time loop, global default schedule, direct movement control, new
WGSL/shader/kernel, new simulation behavior, or ClauseThing.

## CLI / binary decision

**`No CLI binary`.** Library helper + tests only (with an optional golden transcript). No binary target is
authorized under this gate; adding one is a separate, explicitly-authorized future decision.

## Future test list summary

15 demo/export tests named (opt-in only; runs canonical control batch; emits observation export;
deterministic replay export; uses existing control→schedule→observation path; rejects/omits direct
movement command, external boundary request, player command loop, UI framework, real-time loop, global
default schedule, semantic/raw WGSL, hard currency/markets/trade/`ai_budget`, ClauseThing dependency;
docs-status match). **None implemented.**

## Confirmations

No demo implementation, no CLI/binary (explicitly `No CLI binary`), no direct movement command, no
external boundary request, no player command loop, no UI framework, no real-time loop, no global default
schedule, no new schedule implementation, no semantic/raw WGSL, no new shader/GPU kernel, no CPU
planner/urgency/commitment, no hard currency/markets/trade/`ai_budget`, no nested Resource Flow, no
multi-faction economy, no ClauseThing implementation, no `simthing-spec` alteration, no invariant edit, no
passive proof wrapper, **no code change**. All prior gates remain IMPLEMENTED / PASS.

## Manual diff review

Docs-only diff: opening spec + this report + production-track ladder/paragraph + mapping-guidance status +
worklog entry. No code files changed. Every mention of the gate is deterministic, opt-in, headless,
non-interactive, packaging-only, and future-implementation-only. UI framework, real-time loop, direct
movement control, and global default schedule remain CLOSED.
