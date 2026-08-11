# ACTIONBAND-SEMANTIC-SHADOW-0 results

- Track: 0.0.8.7 RF arena modernization (rung 7.5)
- Status: **PROBATION / proof-present / DA-review-pending**
- Branch: `coding/actionband-semantic-shadow-0`
- PR: **#1719**
- Base: `80e61a7062598486be1caf929617ac2037af2c34`
- HD-RECEIPT: `edeb59f58239`
- ORIENT-RECEIPT: `98a916672d1a`
- Dispatch: `5246978127` · Remands: `5247122299`, `5247237560`, `5247464771`, `5247550364`
- DA pre-dispatch: `5246937280`

## Field-neutrality

**FIELD-NEUTRAL** + A1 `synthetic-rf-grant-axis-v1` (preserved).

## Authority binding (remand 4 / R1a.1 + R1b.1)

| Mechanism | Property |
|---|---|
| `ActionBandSessionOrigin` | Opaque per-compile association id; distinct under identity-blind same numeric fingerprints |
| `frozen_admission_binding_id` | Binds compile product to the frozen admission it was lowered from (authored semantic identity included) |
| `ActionBandSemanticSession::open` | Requires structural.origin == compiled.origin and frozen binding match |
| `ActionBandBoundDispatch` | Sole seal door: `bind_dispatch` + `dispatch_and_seal`; foreign `compiled` not independently selectable |
| Sealed authority | Carries session origin; project rejects foreign origin |
| Structural loci | Admitted `Reparent` + tree parent (R1b preserved) |
| Detachability | R5 held: no driver→mapeditor dep |

### Biting falsifiers

- same-shape foreign compile at open/seal: `same_shape_foreign_compile_cannot_be_selected_at_seal_or_open`
- same-shape cross-session structural projection: `same_shape_cross_session_structural_projection_is_red` → `SessionOriginMismatch`
- free cross-dispatch / foreign-compile seal API: `cross_dispatch_restamp_and_foreign_compile_api_is_absent`

## R2 / R3 / R4 / R5

Preserved: owner `Result` through transit; engine `FleetPresenceRecord::InTransit`; PR routing; detachability 0/0/0.

## Batteries

Focused 13 integration + 2 unit (no GPU skips). Inherited 7.1–7.4 green at final head (relay).

Coding does not `/clearance`, merge, move pointer, or start 7.5a.
