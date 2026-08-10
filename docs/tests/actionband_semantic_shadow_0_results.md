# ACTIONBAND-SEMANTIC-SHADOW-0 results

- Track: 0.0.8.7 RF arena modernization (rung 7.5)
- Status: **PROBATION / proof-present / DA-review-pending**
- Branch: `coding/actionband-semantic-shadow-0`
- PR: **#1719**
- Base: `80e61a7062598486be1caf929617ac2037af2c34`
- HD-RECEIPT: `edeb59f58239`
- ORIENT-RECEIPT: `98a916672d1a`
- Dispatch: `5246978127` · Remands: `5247122299`, `5247237560`
- DA pre-dispatch: `5246937280`

## Field-neutrality

**FIELD-NEUTRAL** + A1 `synthetic-rf-grant-axis-v1` (preserved).

## Authority binding (remand 2 / R1)

| Mechanism | Property |
|---|---|
| `CompiledActionBandGpuExecution::seal_production` | Only public seal door; generation from `ActionBandGpuSession` after dispatch; template + plan fingerprint from compile product |
| `ActionBandSemanticSession` | Owns frozen product + private structural loci table; project requires matching plan fingerprint |
| Free `seal_actionband_authority(commitment, gen)` | **Deleted** |
| Public `AdmittedStructuralLoci` | **Deleted**; loci only at session open |

### Biting falsifiers

- substituted generation: unconstructible (no free seal API)
- foreign compile session: `PlanFingerprintMismatch` when fingerprints differ
- forged loci at project time: unconstructible (no loci arg on project)

## R2 / R3 / R4

Preserved: owner `Result` through transit; icon `InTransit` via existing consumer; PR routing metadata.

## Batteries

Focused 11 integration + 2 unit (no GPU skips). Inherited 7.1–7.4 green at final head (relay).

Coding does not `/clearance`, merge, move pointer, or start 7.5a.
