# OC-K-DECISION-INGRESS-0 Results

## Status
**DONE / PROBATION** — sealed decision ingress door; DA deep audit/merge.

## PR / branch / head
See PR body `tested_code_sha` after push.

## STOP-gate forgeability
**PASS** — `oc_k_decision_ingress_0_cpu_commitment_can_bypass_today`: `CpuDiagnosticDecision::observe` and `ApproximateDecisionDiagnostic::from_cpu_urgency` still freely construct outside sealed path (diagnostic residual). Free feeder `BoundaryRequest` construction remains B4 residual for non-decision structural work.

## What changed
- `crates/simthing-kernel/src/decision_ingress.rs` — token chain + `StructuralCommitment`
- lib exports + kernel_surface allowlist
- core §8 SANCTIONED minting walkthrough (`field-policy-time-decisions`)
- design K1 stamp; inventory; this doc

## Door
`ThresholdEvent` → `ThresholdCrossingToken` → `EmissionToken` → `BoundaryEmissionToken` → `StructuralCommitment`.

## Pathway
Core §8 walkthrough under `field-policy-time-decisions` anchor.

## Blocked proofs
compile_fail + unit tests: diagnostic/approximate cannot convert into commitment; private-field constructor blocked; boundary bind required (locus match).

## seal_residue_risk
**B4 residual:** free `BoundaryRequest` POD construction in feeder (non-decision structural). Diagnostic decision types freely constructible (observation only). Sealed records remain sealed. B1–B3, B5–B8 clean for this rung. Not baseline-zero.

## Scan ledger
No scan row change. Net 0. kernel_surface allowlist +exports.

## TEST-BUDGET (6 tests)
| test | catches | why type alone insufficient | residue |
|---|---|---|---|
| cpu_bypass_today | forgeability | docs residual | permanent:seal-proof |
| sealed_path_mints | sanctioned path | runtime bind | permanent:seal-proof |
| cpu_diagnostic_cannot | diagnostic leakage | compile_fail + positive | permanent:seal-proof |
| approximate_cannot | approx leakage | compile_fail + positive | permanent:seal-proof |
| raw_constructor_blocked | POD bypass | private fields | permanent:seal-proof |
| boundary_token_required | skip boundary | locus fail-closed | permanent:seal-proof |
Delete-at-closeout: no (TIER7 seal-proof).

## Scope ledger
Implemented: sealed token chain + mint + walkthrough. Proxied: full BoundaryRequest seal → backlog. Deferred: K4/closeout.

## Sticky / ACKs
See PR after push.
