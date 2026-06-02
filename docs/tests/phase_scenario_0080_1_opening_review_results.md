# SCENARIO-0080-1 — Opening Review Visibility Report

**Date:** 2026-06-02
**Verdict:** **OPENED — scenario accepted; two parked-substrate gates opened (docs/design).** No code.

## Decision rationale

`SCENARIO-0080-1` (Nested Starmap, Terran/Pirate multi-theater) is the second 0.0.8.0 consumer-pulled
scenario, after Local Patrol Economy (COMPLETE/PARKED). It is a genuine new pull: nested multi-theater
structure, atlas residency, decision weights **sourced from broadcast faction overlays** (not local
constants), ownership **up-aggregation**, and **multi-faction adversarial resource flow** — none of which
the closed slice exercised. It is a new track, not accretion on `SCENARIO-0080-0`.

Two product decisions (2026-06-02, via design-authority question) set scope:
1. **Atlas:** engage it properly → **`ATLAS-0080-0`** opened. This scenario is exactly the *named
   multi-theater* consumer the atlas park-condition required, and the *named first slice* the invariant
   *"No production mapping runtime without first-slice gating"* contemplates. Opening it is legitimate and
   needs no invariant edit; wiring stays opt-in/default-off (no default session pass graph).
2. **Pirate = full economy faction** → **`ECON-SCALE-0080-0`** opened (Hybrid-Strata/faction-index ECON
   scaling), previously parked, now consumer-named by adversarial contended resource flow.

The native/proven parts (nested `{children}` structure, OWNER overlay down-broadcast of personality/policy
weights, SEAD-sourced composite-gap decision, read-only observability) require no new substrate. The two
gates above are the only parked substrates pulled; both are opened as **docs/design only**.

## Files touched

| File | Action |
|---|---|
| `docs/scenarios/scenario_0080_1_admission_packet.md` | Created — scenario admission packet |
| `docs/production_paths/atlas_0080_0_opening_spec.md` | Created — ATLAS-0080-0 opening spec |
| `docs/production_paths/econ_scale_0080_0_opening_spec.md` | Created — ECON-SCALE-0080-0 opening spec |
| `docs/tests/phase_scenario_0080_1_opening_review_results.md` | Created — this report |
| `docs/design_0_0_8_0_consumer_pulled_production_track.md` | Updated — ladder rows + status note |
| `docs/workshop/mapping_current_guidance.md` | Updated — status line |
| `docs/worklog.md` | Updated — top entry |

## Opened / closed status after this decision

- `SCENARIO-0080-1` — **ACCEPTED**.
- `ATLAS-0080-0` — **OPEN** (docs/design gate; no implementation).
- `ECON-SCALE-0080-0` — **OPEN** (docs/design gate; no implementation).
- `PRODUCTION-PATH-0080-1` — **NOT YET OPENED** (opens after the two substrate gates have accepted opening specs).
- `SCENARIO-0080-0` (Local Patrol Economy) — remains **COMPLETE/PARKED**.
- Default-on session wiring, real-time loop, player command loop, UI framework, direct movement control,
  hard currency/markets/trade/`ai_budget`, nested Resource Flow depth, semantic/raw WGSL, ClauseThing/L3,
  global default schedule — remain CLOSED/PARKED.

## Bounds preserved

Opt-in/default-off; bounded 10×10 grids and fixed small faction set; deterministic seed + replay;
I8 CPU-oracle parity required of any GPU-resident field; owner-relation broadcast (not spatial
reparenting / not owner-as-spatial-parent); player orders (if later) a weighted overlay term, not a
direct-move and not the currency mechanism; SEAD-sourced decisions (no CPU planner); residency is a
strict value no-op; adversarial economy modeled within Resource Flow / subsidiarity (no hard currency,
no nested RF). Reversible. Any crossing is a stop-and-escalate.

## Future test summary

ATLAS-0080-0: 13 tests named. ECON-SCALE-0080-0: 14 tests named. **None implemented.** Production-path /
schedule / observation / control / demo tests for the scenario will be named at their own opening gates.

## Confirmations

No implementation of any kind, no code change, no atlas runtime, no ECON scaling, no nested structure
built, no default-on session wiring, no real-time loop, no UI, no direct movement control, no hard
currency/markets/trade/`ai_budget`, no nested Resource Flow, no semantic/raw WGSL, no new shader/GPU
kernel, no CPU planner, no ClauseThing, no `simthing-spec` alteration, no invariant edit, no passive proof
wrapper. `SCENARIO-0080-0` remains COMPLETE/PARKED; all `0080-0` gates remain IMPLEMENTED/PASS.

## Manual diff review

Docs-only diff: scenario packet + two gate opening specs + this report + track ladder/status note +
mapping guidance + worklog. No code files changed. Every gate mention is opt-in, bounded, scenario-scoped,
and future-implementation-only. Default-on, hard currency, nested RF, semantic WGSL, and ClauseThing
remain closed.
