# SCENARIO-0080-0 — Local Patrol Economy Acceptance Review

**Date:** 2026-06-02
**Reviewer:** Design authority (Opus) + product.
**Gate:** `SCENARIO-0080-0` (Tier-2 scenario/admission). Docs-only review — no implementation.

## Verdict

**ACCEPTED — with a design-authority enrichment (SEAD decision source).**

Local Patrol Economy is accepted as the first 0.0.8.0 consumer-pulled scenario. `SCENARIO-0080-0` →
ACCEPTED; `PRODUCTION-PATH-0080-0` → OPEN, scoped only to Local Patrol Economy on the 0.0.7.9
mobility/transfer substrate. No production wiring in this PR.

## Scenario / substrate

| Field | Value |
|---|---|
| Scenario | **Local Patrol Economy** |
| Parked substrate consumed (exactly one) | **0.0.7.9 mobility/transfer** (ALLOC/REENROLL/IDROUTE/ECON/OWNER + RUNTIME-0/1A/1B) |

## Acceptance rationale — and the enrichment (per design-authority instinct)

The packet is a **real product consumer**, not another substrate proof: a patrol relocates between
local economies while preserving identity and owner relation. It exercises two of the three pillars
cleanly:

- **Ownership (OWNER):** `patrol_owner` + `owner_relation`/`owner_overlay` continuity across the move —
  latched owner overlays still apply post-relocation. ✓
- **Flow arenas (ECON):** bounded local economy (`supply`/`maintenance`/`local_output`/
  `local_security`/`disruption`), patrol upkeep consumption, and economy **reassociation** on move
  (source stops counting it; destination starts). ✓

**Gap found — SEAD was not exercised.** The patrol's relocate/patrol behavior was an undefined
`move_request` that could be externally scripted; the packet excludes the CPU planner (correct) but
did not include the accepted GPU-resident decision path. Per design authority, the first consumer
scenario should be **basic but rich enough to test SEAD, Ownership, and Flow together.**

**Design-authority enrichment (accepted into the scenario):** the patrol's relocate/patrol decision is
**sourced from the already-accepted GPU-resident SEAD posture — a `disruption`/`local_security`
`Threshold` crossing → `EmitEvent` → `BoundaryRequest`** — not an externally-scripted `move_request`
and not a CPU planner. The `move_request` becomes the *materialized form* of that GPU-resident
proposal. This makes Local Patrol Economy a genuine **SEAD → mobility → Ownership+Flow** loop:
SEAD decides (threshold→event), the mobility substrate moves the patrol (identity-preserving
re-enrollment), owner overlays carry over, and the local economy reassociates.

**This enrichment pulls no new substrate.** SEAD Self-AI Proposal Pipeline V1 is an *accepted mechanism*
(Threshold+EmitEvent→BoundaryRequest), not a parked substrate awaiting a production gate. It is the
**decision source** for the move; the **mobility/transfer substrate remains the single substrate the
production-path gate wires.** No separate SEAD production gate opens. The enrichment actively reinforces
the no-CPU-planner stop condition (the decision is GPU-resident by construction).

## Bounds confirmed

- Exactly one parked substrate pulled (mobility/transfer). ✓
- Bounded local economy (supply, maintenance, local output/security, disruption, patrol participation
  after movement); does not reopen economy architecture. ✓
- Two/few locations, one owner, one (or very small fixed) patrol; spatial movement only. ✓
- Excludes hard currency, markets, trade, nested Resource Flow, `ai_budget`, policy overlays,
  multi-faction economy, Hybrid-Strata/faction-index scaling. ✓
- Movement genuinely requires identity preservation, source/destination membership update, owner
  overlay continuity, and local economy reassociation. ✓
- ClauseThing horizon-only; no `simthing-spec` alteration required. ✓

## Stop conditions confirmed (all binding)

No owner-entity as spatial parent; no capture-as-reparenting; no nested transfer; no hard-currency
through Resource Flow; no market/trade/`ai_budget`; no semantic/raw WGSL; **no CPU planner / urgency /
commitment emission** (the SEAD decision is GPU-resident, the opposite of a CPU planner); no default-on
without a production gate; no passive proof wrappers; no closed-ladder reopen (atlas runtime, E-11B-5,
B-1, ClauseThing/L3, FrontierV2-5, ACT/EVENT/OBS/PIPE all stay closed/parked).

## Next gate

`PRODUCTION-PATH-0080-0` — **OPEN.** Scope: *first non-test-support default `SimSession` path for
Local Patrol Economy using the 0.0.7.9 mobility/transfer substrate*, with the patrol relocate decision
sourced from the accepted GPU-resident SEAD `Threshold`+`EmitEvent`→`BoundaryRequest` path. The
mobility/transfer substrate is the only substrate wired. A separate authorized PR may author the
`PRODUCTION-PATH-0080-0` opening spec; **this PR does not implement it.**

## This PR adds

- no runtime implementation
- no production `SimSession` wiring
- no default schedule
- no gameplay surface
- no semantic WGSL
- no ClauseThing implementation; no `simthing-spec` alteration
- no invariant edits
- no passive proof wrapper
- no code-file changes (docs-only)
