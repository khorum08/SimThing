# RF-NEED-WEIGHT-PROFILE-TRANSPORT-0 (RF-5) Results

## Status

**PROBATION / proof-present / DA-review-pending** — 2026-07-18 (coder=Grok-CLI).

Remand `5013106236` + ontology addendum `5013140700` landed. Owner alone closes screenshot-C [OVL]. Coding agent does not graduate, mark ready, or merge. No executable freeze.

## Identity

| Field | Value |
|---|---|
| Rung | `RF-NEED-WEIGHT-PROFILE-TRANSPORT-0` (RF-5) |
| PR | #1414 |
| Base / master | `99fa9d06898a1b03b490b7f876a17dfd4568e500` |
| RF-4 graduation | **DA-GRADUATED / merged #1413 @ `99fa9d06`** |
| HD-RECEIPT | `9132665903e6` |
| ORIENT-RECEIPT | `2c9fde39d1d6` (role=coding) |
| Expected route | `DA-RESERVE(gate-wiring)` |
| tested_code_sha | *(bound after implementation commit)* |
| implementation_sha | *(bound after implementation commit)* |
| evidence_sha | *(bound after results commit)* |
| coverage_basis | PASS (workshop RF-5 8/8 + mapeditor RF-4 elevate 8/8 + AGENT-SCAN INSPECT justified) |

## Ontology addendum (`5013140700`)

GPU state is `slots × columns`. Live locus is always **`(slot, column)`**.

| Violation (remanded) | Fix |
|---|---|
| CPU copy of host PropertyValue + CPU overlay Add/Multiply into participant | **Removed.** No install-time mirror. |
| Missing source row identity | `NeedWeightSourceCell { source_slot, source_id, col, property }` on every input/weight |
| Cross-row transport | On-device `AccumulatorOp` Identity projection `(source_slot, col) → (participant_slot, col)` band 0, then EvalEML band 1 |
| Live mutation | Open once, mutate host input Amount, step — need rises without reopen/reseed/CPU copy |

## Remand-2 corrections (`5013106236`)

| Defect | Fix |
|---|---|
| Production binding absent | `compose_need_weight_bindings` — id-matched stacks only; Studio consumes production compose |
| Positional zip invents joins | Removed; explicit complete companion bindings or AdmissionGap |
| Live sealed event | Exit proof is post-`step_once` GPU `readback_threshold_events` |
| Full post-RF rescan | Need-only **append** rescan (no prepare/wipe); dual-threshold bite |
| Canonical + neutral path | Both compose/open/step same production path; bare TP → AdmissionGap |

## Admission gap (honest handoff)

Clause `weight_profile` authors EML stack + profile kind only. Without complete `NeedWeightProfileBindingSpec` companion rows (install / input / weight / threshold), compose returns `AdmissionGap`. No positional zip, name-stem, or first-stockpile authority.

## Load-bearing proofs

| Claim | Result |
|---|---|
| Bare weight_profiles → AdmissionGap | PASS |
| Host source_slot ≠ participant wrapper | PASS |
| Paired overlays diverge live need | PASS |
| Below thr: post-step_once sealed need events = 0 | PASS |
| Crossing: post-step_once sealed need events > 0 | PASS |
| Live host mutation raises need without reopen | PASS |
| Empty weight_properties fail closed | PASS |
| Misbound install fail closed | PASS |
| Post-RF need-only rescan; ordinary not duplicated | PASS |
| Canonical TP + neutral same production path | PASS |
| RF-4 mapeditor elevate 8/8 | PASS |
| agent_scan | INSPECT (TEST-BUDGET justified) |

## Fences held

- No new ClauseScript syntax / kernel WGSL primitive / Studio feeder.
- No synthetic need host property.
- No CPU PropertyValue mirror or CPU overlay arithmetic.
- No `value >= thr` as event proof.
- No 12.10 macro-emergence claim.

## Owner [OVL]

**Blocked until orchestration accepts.** Do not freeze executable or request screenshot C.

## Graduation routing

- Risk class: gate-wiring / DA-reserve (expected)
- Falsification: workshop RF-5 8/8 + RF-4 elevate nonregression
