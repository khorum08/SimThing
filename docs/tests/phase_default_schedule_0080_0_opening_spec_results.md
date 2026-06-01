# DEFAULT-SCHEDULE-0080-0 — Opening Spec Visibility Report

**Date:** 2026-06-02
**Verdict:** **OPENING-SPEC-AUTHORED** (Option A — OPEN WITH NARROWING). Docs/design only; no code.

## Scenario / consumed path

| Field | Value |
|---|---|
| Scenario | **Local Patrol Economy** (`SCENARIO-0080-0`, ACCEPTED) |
| Implemented production path consumed | `PRODUCTION-PATH-0080-0` — IMPLEMENTED / PASS (`run_production_path_0080_0`, 21 driver tests + substrate regressions green; verified in `phase_production_path_0080_0_impl_results.md`) |
| Gate opened | `DEFAULT-SCHEDULE-0080-0` — scenario-scoped schedule, docs/design gate only |

## Files touched

| File | Action |
|---|---|
| `docs/production_paths/default_schedule_0080_0_opening_spec.md` | Created — opening spec |
| `docs/tests/phase_default_schedule_0080_0_opening_spec_results.md` | Created — this report |
| `docs/design_0_0_8_0_consumer_pulled_production_track.md` | Updated — ladder index + DEFAULT-SCHEDULE row |
| `docs/workshop/mapping_current_guidance.md` | Updated — status line |
| `docs/worklog.md` | Updated — top entry |

## Schedule contract summary

Opt-in registration only; **no global default schedule**; no workspace scheduler; no gameplay/real-time
loop; deterministic tick/step; deterministic replay; bounded steps; disabled path registers no schedule
and invokes no path. Each step evaluates GPU-resident SEAD thresholds → emits zero/one `BoundaryRequest`
per mover per step → routes into `run_production_path_0080_0` → records a deterministic per-step report.

## Authorized next implementation slice

A future PR may implement the opt-in scenario-scoped schedule that drives `run_production_path_0080_0`
per deterministic step. **Design-authority enrichment (bounded, sub-sliced for pace):** a pirate/patrol
predator loop — pirate (a *second IDROUTE identity, not a second economy owner*) raises `disruption` and
drains `local_supply ∝ disruption` per tick and relocates when `disruption ≥ 0.5 × local_supply`; patrol
reduces `disruption` per tick and relocates toward depleted supply — all GPU-resident threshold-driven,
through the same mobility/transfer substrate. **1A** (schedule + patrol loop) may ship first; **1B**
(pirate disruptor) is the immediate follow-on so pace is never blocked. No new substrate/shader/gate.

## Future test list summary

17 schedule-contract tests named (opt-in, no-global-default, threshold true/false → boundary request,
routing to production path, no CPU planner, identity/owner/economy preservation, bounded economy,
replay determinism, gameplay/WGSL/hard-currency/nested-RF/ClauseThing rejections, docs-status match) +
5 pirate-loop (1B) tests named. **None implemented.**

## Confirmations

No runtime schedule implementation, no global default schedule, no gameplay surface, no semantic/raw
WGSL, no GPU kernel, no CPU planner/urgency/commitment, no hard currency/markets/trade/`ai_budget`,
no nested Resource Flow, no multi-faction economy, no Hybrid-Strata/faction-index scaling, no
ClauseThing implementation, no `simthing-spec` alteration, no invariant edit, no passive proof wrapper,
**no code change**. `PRODUCTION-PATH-0080-0` remains IMPLEMENTED / PASS.

## Manual diff review

Docs-only diff: opening spec + this report + production-track ladder/row + mapping-guidance status +
worklog entry. No code files changed. All schedule mentions are scenario-scoped, opt-in, and
future-implementation-only. Global default schedule remains CLOSED.
