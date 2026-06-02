# DEFAULT-SCHEDULE-0080-1 — Opening Review Visibility Report

**Date:** 2026-06-02
**Verdict:** **OPEN-WITH-NARROWING** (Option A). Docs/design only; no code.

## Decision rationale

The full green chain is in place: `SCENARIO-0080-1` accepted; `ATLAS-0080-0`, `ECON-SCALE-0080-0`, and
`PRODUCTION-PATH-0080-1` all IMPLEMENTED/PASS. The production path already composes the scenario and
reports **read-only SEAD composite-gap terms** (`run_production_path_0080_1` →
`sead_composite_gap_terms`). The next product step is to turn those terms into **deterministic
SEAD-sourced movement** — the schedule gate.

This is the most consequential gate of the track because it introduces **live movement**. The narrowing
keeps movement strictly on the proven posture: decisions are `Threshold + EmitEvent → BoundaryRequest`
derived from the already-reported composite-gap terms, routed through the **existing mobility/transfer
substrate** — exactly the posture proven at `DEFAULT-SCHEDULE-0080-0` (no CPU planner, no direct move, no
external `BoundaryRequest`, no new substrate). No observation/control/demo, no global default schedule, no
default session pass-graph wiring.

**Option B (remediation) rejected:** all three prerequisites pass with green regressions; the production
path already exposes the composite-gap terms the schedule needs; the docs are consistent — no blockers.

## Files touched

| File | Action |
|---|---|
| `docs/production_paths/default_schedule_0080_1_opening_spec.md` | Created — opening spec |
| `docs/tests/phase_default_schedule_0080_1_opening_review_results.md` | Created — this report |
| `docs/design_0_0_8_0_consumer_pulled_production_track.md` | Updated — ladder row |
| `docs/workshop/mapping_current_guidance.md` | Updated — status line |
| `docs/worklog.md` | Updated — top entry |

## Current completed gates

- `SCENARIO-0080-1` — ACCEPTED.
- `ATLAS-0080-0` — IMPLEMENTED / PASS.
- `ECON-SCALE-0080-0` — IMPLEMENTED / PASS.
- `PRODUCTION-PATH-0080-1` — IMPLEMENTED / PASS.

## Opened / closed status after this decision

- `DEFAULT-SCHEDULE-0080-1` — **OPEN WITH NARROWING** (scenario-scoped Nested Starmap schedule/movement
  gate; no implementation).
- Observation / control / demo for `0080-1`, direct movement commands, externally-scripted boundary
  requests, CPU planner, default session pass-graph wiring, global default schedule, real-time loop, UI,
  semantic/raw WGSL, new shader/GPU kernel, hard currency/markets/trade/`ai_budget`, nested Resource Flow,
  unbounded factions, ClauseThing/L3 — remain CLOSED / not opened.

## Future implementation slice

A future PR may add a narrow scenario-scoped schedule that consumes `run_production_path_0080_1`, runs a
bounded number of deterministic steps, derives movement from the read-only composite-gap terms via
`Threshold + EmitEvent → BoundaryRequest`, routes accepted requests through the existing mobility/transfer
substrate (Terran ships among Terran/contended starsystems; Pirate ships among neutral/weak starsystems),
preserves identity + owner overlays, updates membership without reparenting, and records deterministic
per-step + replay reports. No observation/control/demo, no new substrate.

## Future test list summary

27 schedule/movement tests named (opt-in only; requires production path admitted; rejects disabled
production path; bounded steps; threshold false/true → boundary request; routes through mobility substrate;
Terran/Pirate ship movement by SEAD gap; identity + owner-overlay preservation; membership update without
reparenting; consumes atlas + faction-index ECON reports; replay determinism; and the full
no-observation/control/demo, no-direct-move, no-external-boundary, no-CPU-planner, no-pass-graph,
no-global-schedule, no-realtime/UI, no-WGSL/kernel, no-hard-currency, no-nested-RF, no-ClauseThing
rejections; docs-status match). **None implemented.**

## Confirmations

No schedule implementation, no movement execution, no observation/control/demo, no direct movement command,
no external `BoundaryRequest`, no default pass-graph wiring, no global default schedule, no real-time loop,
no UI, no semantic/raw WGSL, no new shader/GPU kernel, no CPU planner/urgency/commitment, no hard
currency/markets/trade/`ai_budget`, no nested Resource Flow, no ClauseThing implementation, no
`simthing-spec` alteration, no invariant edit, no passive proof wrapper, **no code change**.
`PRODUCTION-PATH-0080-1`, `ATLAS-0080-0`, `ECON-SCALE-0080-0` remain IMPLEMENTED / PASS; `SCENARIO-0080-0`
remains COMPLETE/PARKED.

## Manual diff review

Docs-only diff: opening spec + this report + production-track ladder row + mapping-guidance status +
worklog entry. No code files changed. Every gate mention is opt-in, scenario-scoped, SEAD-sourced, and
future-implementation-only. Observation/control/demo and all parked ladders remain closed.
