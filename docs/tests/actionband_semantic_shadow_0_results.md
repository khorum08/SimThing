# ACTIONBAND-SEMANTIC-SHADOW-0 results

- Track: 0.0.8.7 RF arena modernization (rung 7.5)
- Status: **PROBATION / proof-present / DA-review-pending**
- Branch: `coding/actionband-semantic-shadow-0`
- PR: **#1719**
- Base: `80e61a7062598486be1caf929617ac2037af2c34`
- HD-RECEIPT: `edeb59f58239`
- ORIENT-RECEIPT: `98a916672d1a`
- Dispatch: `5246978127` · Remands: `5247122299`, `5247237560`, `5247464771`
- DA pre-dispatch: `5246937280`

## Field-neutrality

**FIELD-NEUTRAL** + A1 `synthetic-rf-grant-axis-v1` (preserved).

## Authority binding (remand 3 / R1a + R1b + R5)

| Mechanism | Property |
|---|---|
| `dispatch_and_seal` | **Only** public seal door: GPU dispatch and generation stamp minted in one call; production cannot be restamped by a foreign session |
| `SemanticallySealedProduction` | Opaque carrier; no independent `seal_production(production, execution)` pairing |
| `ActionBandSemanticSession` | Owns frozen product + admitted `FrozenActionBandStructuralRequests` (not a free loci table) |
| Structural loci | Actor/dest from admission-sealed `BoundaryRequest::Reparent`; source from authority-tree parent of actor |
| Free `seal_actionband_authority` / `seal_production` | **Deleted** |
| Detachability | No `simthing-driver` → `simthing-mapeditor` dep; R3 via engine `FleetPresenceRecord` |

### Biting falsifiers

- cross-dispatch restamp: unconstructible (`cross_dispatch_restamp_api_is_absent`)
- foreign compile session: `PlanFingerprintMismatch` when fingerprints differ
- caller loci table: unconstructible (`structural_loci_come_from_admitted_reparent_not_caller_table`)
- mapeditor proof coupling: absent (`fleet_presence_in_transit_from_admitted_reparent_without_mapeditor_coupling`)

## R2 / R3 / R4

Preserved: owner `Result` through transit; engine-side `FleetPresenceRecord::InTransit` product (icon peripheral remains mapeditor-side on existing types); PR routing metadata.

## Batteries

Focused 11 integration + 2 unit (no GPU skips). Inherited 7.1–7.4 green at final head (relay).

Coding does not `/clearance`, merge, move pointer, or start 7.5a.
