# RF-NEED-BINDING-AUTHORING-0 (RF-5A) Results

## Status

**PROBATION / proof-present / DA-review-pending** — 2026-07-19 (coder=Grok-CLI).

DA Option A prerequisite for RF-5. No Studio, no canonical TP data amendment, no RF-5 resume, no OVL/freeze/merge.

## Identity

| Field | Value |
|---|---|
| Rung | `RF-NEED-BINDING-AUTHORING-0` (RF-5A) |
| PR | #1416 |
| Base | `cecde41f7ed73287f309020eec6db8f57776feb9` |
| HD-RECEIPT | `5844fdbd66f4` |
| DA ruling | #1414 comment `5013802819` Option A |
| ORIENT-RECEIPT | `2c9fde39d1d6` |
| tested_code_sha | `98777b8a2bad5518ff35cdc229172ed50a8b4dfb` |
| implementation_sha | `98777b8a2bad5518ff35cdc229172ed50a8b4dfb` |
| evidence_sha | `ea2ca3ce05e897126fecc55f797893cb3061b9dc` |
| coverage_basis | PASS workshop RF-5A 5/5 + field_economy_grammar nonregression |

## What landed

- ClauseScript **`need_binding`**: participant named entity; `input`/`weight` as `(entity, property, role)`; threshold/event; profile-id join to `weight_profile` stack.
- Admission resolves entity → exactly one `install_targets` host; property/role → `col_for_role`; full-cell `(slot, col)`.
- EvalEML on source entity row; need write only to admitted participant AllocatorWeight (already seeded by arena materialize; no invent).
- Entity-name uniqueness for row authority; **PropertyKey never row authority** (Option C rejected).
- RF-5/#1414 remains **BLOCKED-PAUSED @ `39b4b392`**.

## Proofs

| Claim | Result |
|---|---|
| Foundry + aqueduct hydrate same generic form | PASS |
| Absent entity fail closed | PASS |
| Paired weight currents → live need + sealed GPU events | PASS |
| Live Constant emission refresh raises need | PASS |
| Second scenario same path | PASS |

## Fences held

- No raw slot/column Clause syntax
- No PropertyKey→one-row uniqueness
- No first-DFS row authority
- No global GameMode overlay ADR overturn for RF-5A (overlays via domain pack in workshop fixture only)
- No TP tokens in clausething
- No Studio / canonical TP amendment / RF-5 close
