# DEMO-0080-1 — Opening Review Visibility Report

**Date:** 2026-06-02
**Verdict:** **OPEN-WITH-NARROWING** (Option A). Docs/design only; no code.

## Decision rationale

The Nested Starmap vertical slice is now complete and implemented/pass end-to-end: instantiate
(`PRODUCTION-PATH-0080-1`) → schedule FIELD_POLICY-sourced movement (`DEFAULT-SCHEDULE-0080-1`) → observe/export
(`GAMEPLAY-0080-1`) → bounded command admission (`CONTROL-0080-1`), over the green `ATLAS-0080-0` +
`ECON-SCALE-0080-0` substrates. A demo gate is **pure packaging** over these existing seams: apply a
canonical bounded `CONTROL-0080-1` command batch, run the existing path, emit the existing deterministic
transcript/export + a compact demo report. It adds **no new simulation behavior, no new substrate, and no
decision logic** — FIELD_POLICY remains the sole mover-decision source. This is the natural "make the finished
vertical slice reproducible/usable" step at the designer-facing barrier — the same posture proven at
`DEMO-0080-0`.

**CLI decision: `No CLI binary`** (§4 default when unsure; keeps the surface minimal and non-interactive)
— a library helper + tests, with an optional golden transcript.

**Option B (park) rejected:** demo packaging is the natural completion of the vertical *before* closeout,
and it is a genuine usability consumer, not per-slice accretion (it adds no behavior). The close/park
review is `SCENARIO-0080-1-CLOSE-0`, which comes after the demo lands. **Option C (remediation) rejected:**
all prior gates pass with green regressions and the docs are consistent — no blockers.

## Files touched

| File | Action |
|---|---|
| `docs/gameplay/demo_0080_1_opening_spec.md` | Created — opening spec |
| `docs/tests/phase_demo_0080_1_opening_review_results.md` | Created — this report |
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
- `CONTROL-0080-1` — IMPLEMENTED / PASS.

## Opened / closed status after this decision

- `DEMO-0080-1` — **OPEN WITH NARROWING** (headless Nested Starmap demo/export packaging; no
  implementation). **CLI decision: `No CLI binary`.**
- Direct movement control, externally-scripted boundary requests, FIELD_POLICY bypass, CPU planner, player command
  loop, UI framework, real-time loop, global default schedule, CLI binary, semantic/raw WGSL, new
  shader/GPU kernel, hard currency/markets/trade/`ai_budget`, nested Resource Flow, unbounded factions,
  ClauseThing/L3 — remain CLOSED / not opened.

## CLI / binary decision

**`No CLI binary`.** Library helper + tests only (with an optional golden transcript). No binary target is
authorized under this gate; adding one is a separate, explicitly-authorized future decision.

## Future implementation slice

A future PR may add a narrow `demo_0080_1` library helper that creates a canonical bounded `CONTROL-0080-1`
command batch, runs the existing `control → schedule → observation/export` path, emits the existing
deterministic transcript/export plus a compact demo report (scenario id/name, starmap shape, atlas
residency summary, faction-index ECON summary, owner-overlay/up-aggregation summary, FIELD_POLICY movement trace,
Terran/Pirate movement rows, command transcript rows, replay checksum), optionally a golden transcript,
and keeps all regressions green. No CLI binary, no new behavior/substrate.

## Future test list summary

24 demo/export tests named (opt-in only; runs canonical control batch; uses existing
control→schedule→observation path; emits export + command transcript + Terran/Pirate movement rows + atlas
residency + faction-index ECON + owner-overlay/up-aggregation summaries; deterministic replay; and the full
no-CLI-binary, no-direct-move, no-external-boundary, no-FIELD_POLICY-bypass, no-player-loop, no-UI/realtime,
no-global-schedule, no-WGSL/kernel, no-hard-currency, no-nested-RF, no-ClauseThing rejections; docs-status
match). **None implemented.**

## Confirmations

No demo implementation, no CLI/binary (explicitly `No CLI binary`), no direct movement command, no external
`BoundaryRequest`, no FIELD_POLICY bypass, no CPU planner/urgency/commitment, no player command loop, no UI, no
real-time loop, no global default schedule, no semantic/raw WGSL, no new shader/GPU kernel, no hard
currency/markets/trade/`ai_budget`, no nested Resource Flow, no ClauseThing implementation, no
`simthing-spec` alteration, no invariant edit, no passive proof wrapper, **no code change**. All `0080-1`
gates remain IMPLEMENTED / PASS; `SCENARIO-0080-0` remains COMPLETE/PARKED.

## Manual diff review

Docs-only diff: opening spec + this report + production-track ladder row + mapping-guidance status +
worklog entry. No code files changed. Every gate mention is deterministic, opt-in, headless,
non-interactive, packaging-only, and future-implementation-only. UI, real-time loop, direct movement
control, CLI binary, and global default schedule remain CLOSED.
