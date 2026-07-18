---
rung: RF-EXECUTE-RECURSIVE-DEFAULT-0
kind: rung
track: 0.0.8.6
base_sha: 14c714f2371e4b432415c6903fcbda9b6f76dbb6
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "MANUAL PROGRESSION. Owner drives coder + orchestrator pokes; orchestrator owns CI/clearance/tree-review/remands + delegated merge; escalations->orchestrator not DA. Load-bearing RF execution flip; RF-1's oracle is the judge."
surfaces: ["crates/simthing-spec", "crates/simthing-driver", "crates/simthing-core", "crates/simthing-workshop", "docs/design_0_0_8_6_studio_live_ops.md"]
forbidden: ["new kernel/WGSL/GPU/spec/accumulator primitive; Studio-side RF arithmetic", "transplant RUNTIME-0080 RR-3/RR-4 rehearsal into the production tick", "weaken RF-1 rf_conservation_oracle or break its independence", "CPU planner / decisions outside threshold crossings; scenario-specific code/tests in a sealed crate"]
required_checks: ["cargo-check", "focused-tests", "doctrine-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "executed-reduce-up-needs-nonexistent-admitted-primitive", "RF-1-oracle-fails-on-executed-path"]
---
## BUILD
- Make recursive Arena RF the DEFAULT EXECUTED tick source: flip `resource_flow_execution_profile`
  default (spec/game_mode.rs; admission resource_economy_admission.rs) so ordinary `SimSession::step_once`
  EXECUTES child->ancestor reduce-up (OrderBand up) + root->leaf disburse-down + `runtime_local_allocation`
  writeback; un-defer `economy_execution_deferred` (default true->false) so RF MUTATES state.
- Retire the legacy planet-child/owner-silo path AS DEFAULT + its report-only gating. Full sweep / repoint /
  ADR+anchor re-indoctrination is RF-3; here keep the tree GREEN. RF-1's `rf_conservation_oracle`
  (simthing-driver) is the JUDGE — the executed path must satisfy its three ADR invariants.
## FENCES
- Existing AccumulatorOp / `governed_by` / OrderBand ONLY — NO new kernel/WGSL/GPU/spec/accumulator
  primitive. If the executed reduce-up/writeback needs a nonexistent primitive, STOP and report (DA-route).
- No RUNTIME-0080 RR-3/RR-4 transplant (falsification oracle only). Do NOT weaken RF-1's oracle to pass —
  if it fails, the EXECUTION is wrong; keep it independent. ScenarioSpec authority; decisions = threshold
  crossings. §12: scenario tests -> `simthing-workshop`; WORKSHOP-HOMING-DETECTION PASS 0.
## EXIT-PROOF
- Ordinary admitted `step_once` executes recursive reduce-up + disburse-down (economy_execution_deferred=
  false); RF-1's oracle PASSES all three invariants on the executed path.
- BITING reduce-up (the REMAND-7 / 12.9 proof, on the default executed path): a named child's admitted
  marginal contribution reaches its named ancestor/Owner aggregate; remove/disable ONLY that child ->
  ancestor differential = its marginal contribution, zero without it. Tests BITE.
- Legacy retired AS DEFAULT (grep-prove recursive is now the default); determinism bit-exact; ct_2a/ct_2c
  + build + doctrine-scan(PR delta)/orient/doc-budget green; tests ledgered. PROBATION LEADS the RF-2 cell
  + Active-open-rung row; orientation regenerated.
