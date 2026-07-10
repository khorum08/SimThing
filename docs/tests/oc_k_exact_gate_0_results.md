# OC-K-EXACT-GATE-0 Results

## Status
**DONE / PROBATION** — exact-magnitude proof token door; DA deep audit/merge.

## PR / branch / head
See PR body `tested_code_sha` after push.

## What changed
- `crates/simthing-kernel/src/exact_magnitude_gate.rs` — `ExactMagnitudeProof`, `ApproximateDiagnostic`, `CommitmentRegistration`, `ThresholdRegistration::register_exact_*`, Candidate F CPU mint/parity
- lib exports; constitution §0.7 token API + 3-4-5 worked example; design K3 stamp; inventory

## Forgeability
`oc_k_exact_gate_0_approximate_value_can_reach_gate_today`: bare `ThresholdRegistration { threshold: ApproximateDiagnostic::from_native_sqrt(...).value(), ... }` still constructs (POD residual) — illegal approximate path representable today outside `register_exact_*`.

## Door
`ExactMagnitudeProof` (private bits) ← Candidate F only; `register_exact_magnitude_sensitive` / `CommitmentRegistration::register_exact` require the token. compile_fail: no forge from bits / ApproximateDiagnostic into proof or register_exact.

## Pathway
Candidate F allowlisted end-to-end: mint → proof → register_exact (CPU twin bit-exact with WGSL CR-F).

## Worked bit-exact sqrt
§0.7: (3,4) → mag2=25.0 bits → CR-F → proof bits = threshold bits. Anchor `exact-numeric-candidate-f`.

## CPU-oracle parity
`oc_k_exact_gate_0_candidate_f_token_threshold_parity_bits` — `f32::to_bits()` identity.

## seal_residue_risk
**B4 residual:** `ThresholdRegistration` POD still public-constructible with bare f32 (legacy non-magnitude + unsealed residual; register_exact is the new door). Candidate F WGSL shader text permanent residue. B1–B3, B5–B8 clean for this rung. Not aspirational baseline-zero.

## Scan ledger
No scan row deleted; no new scan required (type door). Net 0.

## Scope ledger
Specified = implemented: proof token, Candidate F mint, ApproximateDiagnostic block on register_exact, parity, anchor example. Proxied: full POD seal of ThresholdRegistration → OC-K3.1 backlog. Deferred: K1/K4/closeout.

## Sticky / ACKs
See PR after push.
