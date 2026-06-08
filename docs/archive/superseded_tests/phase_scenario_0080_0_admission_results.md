# SCENARIO-0080-0 — Local Patrol Economy Admission Visibility Report

**Date:** 2026-06-02  
**Verdict:** **PROPOSED / READY-FOR-DESIGN-AUTHORITY**  
**Gate:** `SCENARIO-0080-0` (Tier-2 scenario/admission only)

## Summary

First 0.0.8.0 consumer-pulled scenario admission packet authored. Names **Local Patrol Economy**
as the product scenario and pulls exactly one parked substrate: **0.0.7.9 mobility/transfer**.
Defines a bounded basic local economy around patrol relocation, upkeep/supply, and owner/economy
coherence. Requests `PRODUCTION-PATH-0080-0` as the next gate **only after** design-authority/product
acceptance — not implemented in this PR.

## Files touched

| File | Action |
|---|---|
| `docs/scenarios/scenario_0080_0_admission_packet.md` | Created — admission packet |
| `docs/tests/phase_scenario_0080_0_admission_results.md` | Created — this report |
| `docs/design_0_0_8_0_consumer_pulled_production_track.md` | Updated — pointer to packet (not accepted) |
| `docs/workshop/mapping_current_guidance.md` | Updated — SCENARIO-0080-0 status line |
| `docs/worklog.md` | Updated — top entry |

## Scenario

| Field | Value |
|---|---|
| **Gate ID** | `SCENARIO-0080-0` |
| **Scenario name** | Local Patrol Economy |
| **Parked substrate consumed** | 0.0.7.9 mobility/transfer substrate (exactly one) |

## Basic economy scope

- Local values only: `supply`, `maintenance`, `local_output`, `local_security`, `disruption`
- Per-location supply balance, maintenance burden, patrol-affected output/security modifiers
- Patrol consumes upkeep/supply; contributes local security/output; shifts participation on move
- Excluded: hard currency, markets, trade, nested RF, multi-faction economy, `ai_budget`, gameplay UI

## Bounds summary

- Two or a few locations; one owner; one or very small fixed patrol count
- Spatial movement only; no nested movement, capture-as-reparenting, semantic WGSL, AI planner
- Small deterministic scale for admission — no new soak

## Requested next gate

| Gate | Status in this PR |
|---|---|
| `PRODUCTION-PATH-0080-0` | **CLOSED** — requested as next gate after acceptance only |
| Default `SimSession` / default schedule / gameplay / semantic WGSL | **Not requested now** |

## Scope confirmations

| Item | Present in this PR? |
|---|---|
| Runtime implementation | **No** |
| Production `SimSession` wiring | **No** |
| Default schedule | **No** |
| Gameplay surface | **No** |
| Semantic WGSL | **No** |
| ClauseThing implementation | **No** |
| `simthing-spec` alteration | **No** |
| Invariant edits | **No** |
| Passive proof wrapper | **No** |
| Rust / code changes | **No** |

## Manual diff review

- Docs-only diff reviewed: admission packet + visibility report + three doc pointer/status updates.
- No code files changed.
- `PRODUCTION-PATH-0080-0` references state closed/requested-next-gate only — not implemented.
- Stop conditions and ClauseThing horizon-only posture preserved.

## Next step

Design authority + product review of this admission packet. On acceptance, `PRODUCTION-PATH-0080-0`
may open for Local Patrol Economy scoped to the 0.0.7.9 mobility/transfer substrate — in a separate
authorized PR.
