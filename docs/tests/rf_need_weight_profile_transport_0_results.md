# RF-NEED-WEIGHT-PROFILE-TRANSPORT-0 (RF-5) Results

## Status

**PROBATION / proof-present / partial BLOCKED** — 2026-07-19 (coder=Grok-CLI).

Remand `5013605086` (full-cell authority). Owner OVL / freeze / DA relay remain blocked.

## Identity

| Field | Value |
|---|---|
| Rung | `RF-NEED-WEIGHT-PROFILE-TRANSPORT-0` (RF-5) |
| PR | #1414 |
| Base reviewed head | `e0cf893ceae665495002dff822fa401c4f5c49ad` |
| ORIENT-RECEIPT | `2c9fde39d1d6` (role=coding) |
| tested_code_sha | `d0ca1d89ce0b582fdf540fff3de957b05b15e626` |
| implementation_sha | `d0ca1d89ce0b582fdf540fff3de957b05b15e626` |
| evidence_sha | *(bound after evidence)* |
| coverage_basis | PASS workshop RF-5 8/8 + RF-4 elevate 8/8 |

## Remand-3 corrections

| Defect | Fix |
|---|---|
| Source slot re-homed to install host | Each property resolved via `find_property_owner`; missing instance → InstallError (no invent) |
| Same-col projection to unowned participant cells | **Removed.** EvalEML runs on **authored source row**; writes need only to admitted participant `AllocatorWeight` |
| AdmissionGap → silent empty | Studio carries `rf5_admission_gap` telemetry; does not invent bindings |
| Canonical transport | Bare TP → typed **AdmissionGap BLOCKED** on Studio profile (no Rust companion as production close) |
| Global overlay ADR broaden | **Reverted** — GameMode overlays remain deferred per ADR |
| Live mutation dense poke | Mutate admitted Constant emission formula + re-seed |
| Non-biting ordinary thr | Ordinary thr actually crosses; assert ordinary_n=1 and need_n=1 |

## Honest stop condition (canonical)

Clause `weight_profile` does not author install/input/weight/threshold. Without complete production companion bindings on GameMode, `compose_need_weight_bindings` returns **AdmissionGap**. Studio surfaces it; does not invent. **Canonical production need transport remains BLOCKED** until Owner/DA admit companion binding authority or new ClauseScript fields.

## Proofs

| Claim | Result |
|---|---|
| Bare weight_profiles → AdmissionGap | PASS |
| Sources from property owners (not re-home) | PASS |
| Paired need + sealed GPU events | PASS |
| Live emission mutation without reopen | PASS |
| Empty weights / misbind fail closed | PASS |
| Exactly-once ordinary + need after rescan | PASS |
| Canonical TP Studio = gap BLOCKED | PASS |
| RF-4 elevate nonregression | PASS 8/8 |

## Fences

- No property invent on install host
- No same-property GPU shadow on participant
- No global GameMode overlay install (ADR held)
- No Rust companion as canonical close
- No OVL / freeze / DA relay
