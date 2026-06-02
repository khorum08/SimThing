# CONTROL-0080-1 — Opening Review Visibility Report

**Date:** 2026-06-02
**Verdict:** **OPEN-WITH-NARROWING** (Option A). Docs/design only; no code.

## Decision rationale

The full `0080-1` chain is green: `SCENARIO-0080-1` accepted; `ATLAS-0080-0`, `ECON-SCALE-0080-0`,
`PRODUCTION-PATH-0080-1`, `DEFAULT-SCHEDULE-0080-1`, and `GAMEPLAY-0080-1` all IMPLEMENTED/PASS. Nested
Starmap runs deterministically and is observable in product-readable form. The proposed command vocabulary
maps onto existing `DefaultSchedule0081Input` / Nested Starmap bounded fields (step count, Terran/Pirate
movement thresholds, source/candidate starsystem selectors, composite-gap terms) plus run/export — so a
bounded command gate **admits scenario parameters**, then the **existing** GPU-resident
`Threshold + EmitEvent → BoundaryRequest` schedule produces movement. Commands never move a ship, never
emit a `BoundaryRequest`, and never bypass SEAD. This keeps the decision posture intact and adds no
planner/urgency/commitment — the same proven posture as `CONTROL-0080-0`.

This sits at the designer-facing barrier where guardrails belong — a sharply-bounded product consumer
(designer scenario setup → observe), not a free-form control bus and not a passive proof wrapper.

**Option B (remediation) rejected:** the schedule + observation are implemented/pass with green
regressions, the schedule input exposes exactly the bounded fields the vocabulary targets, and the docs
are consistent — no blockers.

## Files touched

| File | Action |
|---|---|
| `docs/gameplay/control_0080_1_opening_spec.md` | Created — opening spec |
| `docs/tests/phase_control_0080_1_opening_review_results.md` | Created — this report |
| `docs/design_0_0_8_0_consumer_pulled_production_track.md` | Updated — ladder row |
| `docs/workshop/mapping_current_guidance.md` | Updated — status line |
| `docs/worklog.md` | Updated — top entry |

## Current completed gates

- `SCENARIO-0080-1` — ACCEPTED.
- `ATLAS-0080-0` — IMPLEMENTED / PASS.
- `ECON-SCALE-0080-0` — IMPLEMENTED / PASS.
- `PRODUCTION-PATH-0080-1` — IMPLEMENTED / PASS.
- `DEFAULT-SCHEDULE-0080-1` — IMPLEMENTED / PASS.
- `GAMEPLAY-0080-1` — IMPLEMENTED / PASS.

## Opened / closed status after this decision

- `CONTROL-0080-1` — **OPEN WITH NARROWING** (bounded Nested Starmap command admission; no implementation).
- Direct ship movement control, externally-scripted boundary requests, SEAD bypass, CPU planner, player
  command loop, UI framework, real-time loop, demo packaging for `0080-1`, global default schedule,
  semantic/raw WGSL, new shader/GPU kernel, hard currency/markets/trade/`ai_budget`, nested Resource Flow,
  unbounded factions, ClauseThing/L3 — remain CLOSED / not opened.

## Future implementation slice

A future PR may add a narrow `control_0080_1` module that accepts the tiny validated command list, mutates
only allowed `DefaultSchedule0081Input` / Nested Starmap bounded input/config values, invokes the existing
`DEFAULT-SCHEDULE-0080-1` → `GAMEPLAY-0080-1` path, returns the existing read-only observation export after
admitted commands, keeps all regressions green, and adds rejection tests for forbidden direct-control
paths. It must not implement direct movement control, external boundary requests, a gameplay UI, a
real-time loop, a general command system, a general scenario editor, or ClauseThing.

## Future test list summary

20 command-admission tests named (opt-in only; accepts bounded schedule-value commands; runs/export after
admitted command; deterministic replay after command; rejects direct Terran/Pirate move, external boundary
request, SEAD bypass, CPU planner/commitment, player-command loop, UI framework, real-time loop, global
default schedule, semantic/raw WGSL, new shader/GPU kernel, hard currency/markets/trade/`ai_budget`, nested
Resource Flow, ClauseThing dependency; docs-status match). **None implemented.**

## Confirmations

No control implementation, no command input, no direct movement command, no external `BoundaryRequest`, no
SEAD bypass, no CPU planner/urgency/commitment, no player command loop, no UI, no real-time loop, no global
default schedule, no semantic/raw WGSL, no new shader/GPU kernel, no hard currency/markets/trade/`ai_budget`,
no nested Resource Flow, no ClauseThing implementation, no `simthing-spec` alteration, no invariant edit, no
passive proof wrapper, **no code change**. All `0080-1` gates remain IMPLEMENTED / PASS; `SCENARIO-0080-0`
remains COMPLETE/PARKED.

## Manual diff review

Docs-only diff: opening spec + this report + production-track ladder row + mapping-guidance status +
worklog entry. No code files changed. Every gate mention is bounded, opt-in, parameter-admission only, and
future-implementation-only. Direct movement control and SEAD bypass remain CLOSED.
