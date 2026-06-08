# PRODUCTION-PATH-0080-1 — Opening Review Visibility Report

**Date:** 2026-06-02
**Verdict:** **OPEN-WITH-NARROWING** (Option A). Docs/design only; no code.

## Decision rationale

Both prerequisite substrate gates are implemented/pass: `ATLAS-0080-0` (sparse-residency nested mapping,
`run_atlas_0080_0`) and `ECON-SCALE-0080-0` (bounded Terran/Pirate faction-indexed contended ECON,
`run_econ_scale_0080_0`). `SCENARIO-0080-1` is accepted. `PRODUCTION-PATH-0080-1` is the pure
**composition** of those two green capabilities into one opt-in scenario surface — it instantiates the
accepted Nested Starmap, drives both substrates with explicit opt-in, validates both reports are admitted,
and composes one inspectable scenario-level report (no schedule, no movement, no new substrate).

This is rung 3 of the §11 PR ladder, the design-authority OPEN gate. **Option B (remediation) rejected:**
both substrates pass with full regression suites green, their reports expose exactly the fields the
composition needs, and the docs are consistent — no blockers exist.

The composition is narrowed to read-side/structural composition only: owner-overlay inheritance and
ownership up-aggregation as **numeric overlay summaries** (no new owner substrate), FIELD_POLICY composite-gap
terms as **read-only** reports (no CPU planner, no movement). Movement/schedule is explicitly the next
slice, not this gate.

## Files touched

| File | Action |
|---|---|
| `docs/production_paths/production_path_0080_1_opening_spec.md` | Created — opening spec |
| `docs/tests/phase_production_path_0080_1_opening_review_results.md` | Created — this report |
| `docs/design_0_0_8_0_consumer_pulled_production_track.md` | Updated — ladder row |
| `docs/workshop/mapping_current_guidance.md` | Updated — status line |
| `docs/worklog.md` | Updated — top entry |

## Current completed gates

- `SCENARIO-0080-1` — ACCEPTED.
- `ATLAS-0080-0` — IMPLEMENTED / PASS.
- `ECON-SCALE-0080-0` — IMPLEMENTED / PASS.

## Opened / closed status after this decision

- `PRODUCTION-PATH-0080-1` — **OPEN WITH NARROWING** (opt-in Nested Starmap production path; no
  implementation).
- Schedule / observation / control / demo for `0080-1`, default session pass-graph wiring, global default
  schedule, real-time loop, UI framework, semantic/raw WGSL, new shader/GPU kernel, hard
  currency/markets/trade/`ai_budget`, nested Resource Flow, unbounded factions, direct movement control,
  externally-scripted boundary request, CPU planner, ClauseThing/L3 — remain CLOSED / not opened.

## Future implementation slice

A future PR may add a narrow `production_path_0080_1` module that instantiates the accepted Nested Starmap
scenario, calls `run_atlas_0080_0` + `run_econ_scale_0080_0` with explicit opt-in, validates both are
admitted (rejecting if either is disabled/rejected/not admitted), and composes a deterministic
scenario-level report (starmap/starsystem/planet structure, resident theaters, fixed faction set, pirate
full-economy participation, contended clearing, derived ownership up-aggregation summary, inherited overlay
weight summary, read-only FIELD_POLICY composite-gap terms, replay checksum). No schedule/movement, no new
substrate.

## Future test list summary

22 composition tests named (opt-in only; requires both substrates admitted; rejects disabled atlas/econ;
instantiates nested shape; composes residency + faction-index ECON reports; reports owner-overlay
inheritance + ownership up-aggregation; read-only FIELD_POLICY composite-gap terms; replay determinism; and the
full no-schedule/no-movement/no-pass-graph/no-WGSL/no-hard-currency/no-nested-RF/no-ClauseThing
rejections; docs-status match). **None implemented.**

## Confirmations

No `PRODUCTION-PATH-0080-1` implementation, no schedule/observation/control/demo, no direct movement
command, no external boundary request, no default pass-graph wiring, no global default schedule, no
real-time loop, no UI, no semantic/raw WGSL, no new shader/GPU kernel, no hard currency/markets/trade/
`ai_budget`, no nested Resource Flow, no CPU planner/urgency/commitment, no ClauseThing implementation, no
`simthing-spec` alteration, no invariant edit, no passive proof wrapper, **no code change**. `ATLAS-0080-0`
and `ECON-SCALE-0080-0` remain IMPLEMENTED / PASS; `SCENARIO-0080-0` remains COMPLETE/PARKED.

## Manual diff review

Docs-only diff: opening spec + this report + production-track ladder row + mapping-guidance status +
worklog entry. No code files changed. Every gate mention is opt-in, scenario-scoped, composition-only, and
future-implementation-only. Schedule/movement and all parked ladders remain closed.
