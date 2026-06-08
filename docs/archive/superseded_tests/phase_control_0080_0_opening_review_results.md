# CONTROL-0080-0 — Opening Review Visibility Report

**Date:** 2026-06-02
**Verdict:** **OPEN-WITH-NARROWING** (Option A). Docs/design only; no code.

## Decision rationale

Local Patrol Economy now runs deterministically (`DEFAULT-SCHEDULE-0080-0` 1A+1B) and is legible in
product-readable form (`GAMEPLAY-0080-0` read-only export, `observe_gameplay_0080_0`). The proposed
command vocabulary maps **1:1 onto existing `DefaultSchedule0080Input` bounded fields** (source/destination
`disruption`/`supply`/`local_security`, `step_count`, patrol disruption reduction) plus run/export. So a
bounded command gate **admits scenario parameters**, then the **existing** GPU-resident
`Threshold`+`EmitEvent`→`BoundaryRequest` path produces movement — commands never move a mover, never
emit a `BoundaryRequest`, and never bypass FIELD_POLICY. This keeps the decision posture intact and adds no
planner/urgency/commitment.

This is a genuine, sharply-bounded product consumer (designer-facing scenario setup → observe), at the
designer-facing barrier where guardrails belong — not a free-form control bus and not a passive proof
wrapper. **No issues detected.**

**Option B (park)** rejected: there is a clear, narrow consumer pull (parameter admission + observe) with
no scope risk once narrowed to bounded values. **Option C (remediation)** rejected: production
path/schedule/observation are all implemented/pass and docs are consistent — no blockers.

## Files touched

| File | Action |
|---|---|
| `docs/gameplay/control_0080_0_opening_spec.md` | Created — opening spec |
| `docs/tests/phase_control_0080_0_opening_review_results.md` | Created — this report |
| `docs/design_0_0_8_0_consumer_pulled_production_track.md` | Updated — ladder row + closed/parked paragraph |
| `docs/workshop/mapping_current_guidance.md` | Updated — status line |
| `docs/worklog.md` | Updated — top entry |

## Current completed gates

- `SCENARIO-0080-0` — ACCEPTED.
- `PRODUCTION-PATH-0080-0` — IMPLEMENTED / PASS.
- `DEFAULT-SCHEDULE-0080-0` — IMPLEMENTED / PASS (1A + 1B, deterministic cat-and-mouse).
- `GAMEPLAY-0080-0` — IMPLEMENTED / PASS (read-only observation export).

## Opened / closed status after this decision

- `CONTROL-0080-0` — **OPEN WITH NARROWING** (bounded Local Patrol Economy command admission; no
  implementation).
- Direct movement control, externally-scripted move requests, player command loop, UI framework,
  real-time loop, global default schedule, semantic WGSL, new shader/GPU kernel, CPU planner,
  ClauseThing/L3, Hybrid-Strata/faction-index scaling, atlas runtime, E-11B-5, B-1, FrontierV2-5,
  ACT/EVENT/OBS/PIPE — remain CLOSED / PARKED.

## Authorized next implementation slice

A future PR may add a bounded command/admission module that accepts the tiny validated command list,
mutates only allowed `DefaultSchedule0080Input` values/config before invoking the existing
schedule→observation path, returns the existing read-only export, keeps all existing tests green, and adds
rejection tests for forbidden direct-control paths. It must not implement direct movement control, a
gameplay UI, a real-time loop, a general command system, a general scenario editor, or ClauseThing.

## Future test list summary

18 command-admission tests named (opt-in only; accepts bounded scenario-value commands; runs/export after
admitted command; deterministic replay after command; rejects direct patrol/pirate move, external
boundary request, CPU planner/commitment, player-command loop, UI framework, real-time loop, global
default schedule, semantic/raw WGSL, hard currency/markets/trade/`ai_budget`, nested Resource Flow,
ClauseThing dependency; docs-status match). **None implemented.**

## Confirmations

No control implementation, no direct movement command, no externally-scripted move request, no player
command loop, no UI framework, no real-time loop, no global default schedule, no semantic/raw WGSL, no new
shader/GPU kernel, no CPU planner/urgency/commitment, no hard currency/markets/trade/`ai_budget`, no
nested Resource Flow, no multi-faction economy, no ClauseThing implementation, no `simthing-spec`
alteration, no invariant edit, no passive proof wrapper, **no code change**. All prior gates remain
IMPLEMENTED / PASS.

## Manual diff review

Docs-only diff: opening spec + this report + production-track ladder/paragraph + mapping-guidance status +
worklog entry. No code files changed. Every mention of the gate is bounded, opt-in, parameter-admission
only, and future-implementation-only. Direct movement control and global default schedule remain CLOSED.
