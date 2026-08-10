# ACTIONBAND-SEMANTIC-SHADOW-0 results

- Track: 0.0.8.7 RF arena modernization (rung 7.5)
- Status: **PROBATION / proof-present / DA-review-pending**
- Branch: `coding/actionband-semantic-shadow-0`
- PR: **#1719**
- Base: `80e61a7062598486be1caf929617ac2037af2c34`
- HD-RECEIPT: `edeb59f58239`
- ORIENT-RECEIPT: `98a916672d1a`
- orientation_rule_stamp: `3e2afa381d2aea10`
- Dispatch: `5246978127` · Remand: `5247122299` (R1–R4)
- DA pre-dispatch: `5246937280`

## Field-neutrality gate

**Outcome: `FIELD-NEUTRAL`**

A1 positive proof: synthetic bound-observable `synthetic-rf-grant-axis-v1`
round-trips through post-authority readback without PALMA special-casing.

## Remand R1–R4 discharge

| Item | Discharge |
|---|---|
| **R1** | `SealedActionBandAuthority` private fields; `seal_actionband_authority` binds template via `FrozenActionBandTemplates::binding_for_event_kind` and generation from the production sealed-path authority generation. `project_semantic_readback` no longer accepts free template/generation/owner_subject. Wrong association + production-stamp proofs plant. |
| **R2** | `ActionBandTransitProjection.owner` is `Result<OwnerRef, OwnerResolutionError>`; foreign owner errors propagate through transit and refuse `to_fleet_presence_record` (no `None` alias). |
| **R3** | Transit loci from admitted structural table (source≠dest); `to_fleet_presence_record` → existing `fleet_icon_descriptors_from_records` yields `FleetIconPlacement::InTransit`. Zero icon-layer source change. |
| **R4** | PR body normalized with Rung/Handoff/HD-RECEIPT/tested_code_sha (orchestration re-clears). |

## Product

- `simthing-spec`: `binding_for_event_kind` + `UnboundEventKind` / `AmbiguousEventKind`
- `simthing-driver::action_band_semantic_shadow`: seal + project + fleet presence bridge

## Tests (9 integration + 2 unit)

Load-bearing GPU proofs do not skip.

## Batteries

Focused 7.5 + inherited 7.1/7.2/7.3/7.4 bound to final head in PR/board relay.

## Scope

Only 7.5 PROBATION. No 7.5a+, no merge, no pointer advance, no `/clearance` by coding.
