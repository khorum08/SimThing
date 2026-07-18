---
rung: RF-GOVERNED-INTEGRATION-ORDERBAND-0
kind: remedial
track: 0.0.8.6
base_sha: f313f5406efb8bdbfac09b7a3ca7dcfc1d36cd58
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(kernel-contract)
owner_notes: "DA-authorized prerequisite repair for RF-2 on PR #1411. Implement on the existing branch and preserve the accepted RF-2 work. Orchestrator owns clearance, tree review, and remands; no merge without DA graduation."
surfaces: ["crates/simthing-driver/src/arena_allocation_plan.rs", "crates/simthing-driver/tests", "crates/simthing-kernel/src/shaders/accumulator_op.wgsl", "crates/simthing-kernel/tests", "crates/simthing-gpu/tests", "crates/simthing-workshop/tests/rf_execute_recursive_default_0.rs", "docs/tests/rf_execute_recursive_default_0_results.md"]
forbidden: ["new kernel primitive, shader entry point, accumulator role/combine/gate, grammar, or scenario API", "weaken or couple RF-1 rf_conservation_oracle to the executed source", "transplant RUNTIME-0080 RR-3/RR-4 or retain a CPU-seeded residual as final proof", "change unrelated RF-2 defaults, add Studio arithmetic, perform RF-3 cleanup, or resume 12.9 presentation"]
required_checks: ["cargo-check", "focused-tests", "gpu-proof", "agent-scan", "doctrine-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "repair-requires-new-primitive-or-entrypoint", "matching-band-integration-still-bypasses-target1", "RF-1-oracle-fails-after-zero-initialized-residual"]
---
## BUILD
- In `cpu_op_from_integration_gpu`, reconstruct governed integration with both authored targets:
  `target0` amount and `target1` governed velocity/rate. Preserve its `OrderBand`; respect
  `n_targets`; do not synthesize new semantics.
- In `dispatch_one_op_for_band`, make `COMBINE_INTEGRATE_CLAMP` execute only when its authored
  gate matches `current_band`. Preserve threshold and affine dispatch semantics.
- Add biting tests: adapter round-trip requires two exact targets plus the exact gate; matching
  band mutates amount/eligible velocity once; nonmatching band mutates neither.
- Then resume only the blocked RF-2 proof: generate the residual from zero through Arena operations
  in ordinary `SimSession::step_once`; RF-1 must close the governed-Balance integration invariant.
  Remove the CPU-seeded residual from load-bearing evidence.
## FENCES
- This is a minimal existing-contract repair using the present `AccumulatorOpGpu` layout,
  `IntegrateWithClamp`, and `GateSpec::OrderBand`. No new primitive, role, gate, combine, entry point,
  serialization shape, planner, grammar, or scenario API.
- Keep the accepted RF-2 default-execution implementation, explicit `DefaultDisabled` control,
  D=3 sibling marginal proof, and RF-1 independence intact.
- Scenario-specific proof remains in `simthing-workshop`; no Studio-side RF arithmetic and no
  RUNTIME-0080 rehearsal transplant.
## EXIT-PROOF
- Adapter falsifier fails if `target1` is omitted/swapped, `n_targets` is wrong, or the band is lost.
- GPU falsifiers prove matching-band governed integration executes exactly once and nonmatching-band
  dispatch leaves both amount and velocity unchanged.
- The RF-2 production proof starts governed rate at zero, derives the residual through admitted Arena
  operations during ordinary `step_once`, and passes all RF-1 invariants; the `DefaultDisabled` and
  D=3 selected-child marginal controls still bite.
- At exact head: touched-crate checks, focused adapter/kernel/GPU/workshop tests, `agent_scan`,
  doctrine scan, orientation check, and doc budget are green. Stamp results/PR as PROBATION with
  `tested_code_sha`, `coverage_basis`, this HD receipt, and the carried ORIENT receipt.
