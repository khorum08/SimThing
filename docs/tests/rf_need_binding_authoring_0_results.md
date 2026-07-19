# RF-NEED-BINDING-AUTHORING-0 (RF-5A) Results

## Status

**PROBATION / proof-present / DA-review-pending** — 2026-07-19 (coder=Grok-CLI).

DA Modified Option A (`5014178941`) + coder remander `5014217843`.  
RF-5/#1414 remains **BLOCKED-PAUSED**. No Studio, TP amendment, OVL, freeze, merge, or self-graduation.

## Identity

| Field | Value |
|---|---|
| Rung | `RF-NEED-BINDING-AUTHORING-0` (RF-5A) |
| PR | #1416 (draft) |
| Base | `cecde41f7ed73287f309020eec6db8f57776feb9` |
| HD-RECEIPT | `5844fdbd66f4` |
| DA ruling | `5014178941` Modified Option A |
| Remander | `5014217843` (handoff stamp `5014219181` on #1414) |
| ORIENT-RECEIPT | `2c9fde39d1d6` |
| tested_code_sha | *(bind after green push)* |
| implementation_sha | *(bind after green push)* |
| evidence_sha | *(bind after green push)* |
| coverage_basis | workshop RF-5A 20/20 + field_economy_grammar nonregression |

## Contract landed (Modified Option A)

### Cross-row transport — staged GPU projection
1. Arena flow layout owns `need_stage_in_{i}` / `need_stage_w_{i}` roles (via `expand_arena_internal_columns`).
2. Per-tick `Identity` AccumulatorOps project each authored source `(slot,col)` → participant staged cell at **OrderBand 0**.
3. Slot-local **EvalEML** on participant reads staged cols only, writes AllocatorWeight `Named("weight")` at **OrderBand 1**.
4. Arena reduce/disburse shifted by `NEED_BINDING_PRE_BANDS` (2).

Path: `source (slot_S,col_S) → stage (slot_P,col_stage) → need (slot_P,col_need)`.

### Entity-hosted property authority
- Deleted `{entity}_` prefix inference.
- Explicit `host_entity` / `source_host_entity` / `target_host_entity` on economy emissions/transfers.
- Field economy lowers silo.owner into those fields.
- Missing/ambiguous host → spanned hard error.

### Dense open-time value writes banned
- Removed `install_resolved_values_at_boundary` Constant seed path from RF-5A open.
- Authored Constants seed tree `PropertyValue` at economy place-time; GPU sees them via `project_tree_to_values` / `initial_gpu_sync`.
- Threshold upload remains ordinary session-install machinery only.

### Three-control anti-mirror matrix
| Control | Result |
|---|---|
| LIVE-TRACKING | transfer drip moves weight; stage tracks source; need tracks |
| DISCONNECT | `need_stage_projections_disabled`; source moves, stage+need freeze |
| STATIC | no drip; source/stage/need static across ticks |

### Event counts
| Case | need (91) | ordinary (77) |
|---|---|---|
| Below thr | **0** | n/a |
| Crossing thr | **1** | n/a |
| High + ordinary ore thr | **1** | **1** |

## Workshop battery (20)

Hydrate spans, profile join missing/mismatch, thr/arena required, absent/ambiguous entity, missing property/role, non-admitted participant, prepare no-invent, **cross-row happy path**, paired 0/1 events, ordinary thr once, aqueduct second scenario, LIVE/DISCONNECT/STATIC, full-cell SOA.

## Deferred (not RF-5A blockers per DA)
- Mid-session re-authoring API (Studio candidate)
- Authored complete arena composition in Clause (RF-harness candidate)

## Fences held
- No EvalEML multi-slot widening
- No new kernel/WGSL primitive / COLUMN-INDEX-MINT exclusion
- No PropertyKey row authority
- No binding invent/re-home
- No #1414 resume / merge / OVL / Studio / TP amendment
