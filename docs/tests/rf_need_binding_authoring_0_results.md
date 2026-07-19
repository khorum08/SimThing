# RF-NEED-BINDING-AUTHORING-0 (RF-5A) Results

## Status

**REMAND / proof-incomplete** — 2026-07-19 (coder=Grok-CLI).

Orchestration remander `5013877587` on PR #1416. No Studio, no canonical TP data amendment, no RF-5 resume, no OVL/freeze/merge/DA self-graduation.

## Identity

| Field | Value |
|---|---|
| Rung | `RF-NEED-BINDING-AUTHORING-0` (RF-5A) |
| PR | #1416 |
| Base | `cecde41f7ed73287f309020eec6db8f57776feb9` |
| HD-RECEIPT | `5844fdbd66f4` |
| DA ruling | #1414 comment `5013802819` Option A |
| Remander | issue comment `5013877587` |
| ORIENT-RECEIPT | `2c9fde39d1d6` (coding) |
| tested_code_sha | `80cdbfc268a6ea5ca25dc11d320a4ea837518e18` |
| implementation_sha | `80cdbfc268a6ea5ca25dc11d320a4ea837518e18` |
| evidence_sha | `b2cf718e1d07c2039afb32c799b1840bf270a917` |
| coverage_basis | workshop RF-5A 18/18 PASS + field_economy_grammar 7/7 + production open threshold arm |

## Remander disposition (5013877587)

| # | Finding | Disposition |
|---|---|---|
| 1 | `seed_need_binding_entity_properties` invent/re-home | **Removed.** Economy properties place on `{entity}_…` install_targets hosts via generic `ensure_resource_economy_properties`. Binding resolve only admits already-owned instances. |
| 2 | Live mutation via `seed_and_upload` / registration edit | **Deleted as exit proof.** `production_mid_session_refresh_path_absent_stop` records handoff STOP: no production mid-session authored-source refresh API on `SimSession`. |
| 3 | Unspanned admission errors | **Spans carried** on NeedBindingSpec + locus + InstallError::NeedBindingInvalid.span_token; hydrate requires thr/event/arena. |
| 4 | Multi-row silently narrowed | **Explicit STOP** when input/weight full-cells span multiple entity slots (multi-slot EvalEML not admitted). |
| 5 | First-arena / optional thr-event defaults | **Removed.** Explicit arena required (rejects empty/`default`); threshold + event_kind required. |
| 6 | Missing falsifiers + void rescan | **Falsifier matrix** in workshop (18 tests). `rescan_accumulator_thresholds_after_resource_flow` returns `Result` and fabric propagates. Ordinary thr kind 77 fires exactly once under post-RF rescan. |
| 7 | Reconstruction-heavy only proof | Unit substrate remains for admission; production open now arms thresholds via `install_spec_state` (Constant seed + emit_on_threshold upload + post-RF arm). Full clause→arena composition without unit RF inject is still a composition gap (field_economy lowers need_bindings onto RF without authored arenas). |
| 8 | Governance / orientation | Ledger set **REMAND / proof-incomplete**. Orientation regen + SHA binding follow this implementation commit. |

## What landed (this remander cycle)

- No binding invent/re-home of source properties.
- Generic entity-host placement for resource-economy stockpile properties (`{entity}_` prefix → unique install_targets).
- Required explicit arena, threshold, event_kind; profile-id join missing/mismatch/ambiguous fail closed.
- Spanned hydrate + admission diagnostics.
- Multi-row sources → STOP (no unowned mirrors).
- Production open path: Constant emission open-seed + `emit_on_threshold` GPU upload + post-RF need arm inside `SimSession::install_spec_state`.
- Post-RF need rescan failures propagate (no silent void).
- Workshop falsifier matrix + exact 0/1 need event counts + ordinary thr once.
- **Open gap:** mid-session production authored-source refresh.

## Full-cell source-of-authority (happy path)

For each authored input/weight locus:

1. `entity` → unique `Scenario.install_targets` host (`SimThingId`)
2. `property` → registered `SimPropertyId` (namespace::name)
3. `role` → `col_for_role` on that property layout
4. `slot` → `SlotAllocator::slot_of(simthing_id)`
5. All loci for one binding must share one `slot` (EvalEML slot-local)

Need write cell: admitted arena participant wrapper × flow property × `Named("weight")` AllocatorWeight.

## Production refresh entry point

| Path | Status |
|---|---|
| `SimSession::open_from_spec` → install_spec_state (Constant open-seed + thr upload + post-RF arm) | **Present** (open-time only) |
| `sync_resource_economy_if_enabled` (generation-keyed re-upload of same regs) | Present; does **not** rebind authored Constant values mid-session |
| Mid-session mutate authored source → live need change without reopen/reseed | **ABSENT** — handoff STOP reported |

## Event counts (GPU)

| Case | need event_kind 91 | ordinary event_kind 77 |
|---|---|---|
| Below need thr (weight 0.2) | **0** | n/a |
| Crossing need thr (weight 3.0) | **1** | n/a |
| High + ordinary ore thr | **1** | **1** |

## Fences held

- No raw slot/column Clause syntax
- No PropertyKey→one-row uniqueness
- No first-DFS row authority
- No binding invent/re-home of World-root stockpile
- No forbidden reseed claimed as live production refresh
- No TP tokens in clausething
- No Studio / canonical TP amendment / RF-5 close / merge / self-graduation
- RF-5/#1414 remains **BLOCKED-PAUSED @ `39b4b392`**
