---
rung: RF-LEGACY-RETIRE-REANCHOR-0
kind: rung
track: 0.0.8.6
base_sha: c206b0ef6b6ef99cfdaac6361c32db529b115b1f
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(kernel-contract)
owner_notes: "RF-2A and RF-2 were DA-graduated and merged in #1411. RF-3 retires the legacy RF execution frame, re-anchors doctrine/tests on recursive-default plus the independent RF-1 oracle, and owns the DA-discovered ct_2a/ct_2c participant-enrollment and fail-closed repair. Orchestrator owns review, clearance, remands, and delegated merge; relay to DA only for genuine contract residue."
surfaces: ["crates/simthing-driver/src", "crates/simthing-driver/tests", "crates/simthing-spec/src", "crates/simthing-spec/tests", "crates/simthing-workshop/tests", "docs/adr/resource_flow_substrate.md", "docs/design_0_0_8_6_studio_live_ops.md", "docs/tests", "docs/orchestrator_orientation.md", "scripts/ci/test_inventory.tsv"]
forbidden: ["new kernel/WGSL primitive, entry point, accumulator role/combine/gate, grammar, planner, or scenario API", "weaken, copy, or couple RF-1 rf_conservation_oracle to the executed source", "transplant RUNTIME-0080 RR-3/RR-4 into production or keep a rehearsal loop as tick authority", "NoAdapter/GPU-less skip or other fail-open behavior in load-bearing RF tests", "Studio-side RF arithmetic, 12.9 presentation/need work, or TP-specific code/tests in sealed crates"]
required_checks: ["cargo-check", "focused-tests", "gpu-proof", "agent-scan", "doctrine-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "retirement-requires-new-primitive-or-authority", "ct2a-ct2c-participant-enrollment-cannot-use-admitted-surfaces", "RF-1-oracle-fails-on-recursive-default", "ordinary-step-once-still-dispatches-legacy-rf"]
---
## BUILD
- First reconcile governance: add/stamp RF-2A and RF-2 as DA-GRADUATED / merged `#1411` @ `c206b0ef`; make RF-3 the active dispatched rung and regenerate orientation. Carry the DA-discovered ct_2a/ct_2c repair as a named RF-3 exit item.
- Repoint existing RF tests, `runtime_0080_*` rehearsal framing, and recursive-source/oracle prose to the landed contract: recursive Arena RF is the ordinary default execution source; the independent RF-1 oracle is the judge; RR-3/RR-4 remain falsification rehearsal only.
- Repair `gpu_micro_economy_matches_arena_allocation_oracle` (ct_2a) and its ct_2c sibling so their flat-star arenas enroll the required participants on admitted production surfaces. Remove `open_from_spec_or_skip`/NoAdapter fail-open behavior from these load-bearing tests; live GPU execution must run or fail.
- Retire the dead legacy default execution path and stale report-only/deferred assumptions. Preserve serialized-name compatibility only where required, with an explicit compatibility alias that cannot reactivate legacy dispatch.
- Re-anchor `resource_flow_substrate.md`, the active RF ladder, relevant anchors/results, and test inventory so future agents ingest recursive-executed-by-default plus RF-1 independence rather than the superseded legacy frame.
## FENCES
- This is retirement/repointing of landed machinery, not invention. No new RF primitive, shader entry point, accumulator role/combine/gate, grammar, planner, serialization authority, or scenario API.
- Do not modify RF-1 semantics to make tests pass. A broken recursive execution path must fail the unchanged oracle.
- Do not import RUNTIME-0080 rehearsal orchestration into `SimSession::step_once`; do not implement Studio telemetry, need-profile install, or 12.9 UI work here.
- Scenario-flavored proofs remain workshop-homed. Load-bearing GPU tests are fail-closed; no skip-on-NoAdapter escape.
## EXIT-PROOF
- Source/diff proof shows ordinary `SimSession::step_once` has no live legacy RF dispatch or report-only fallback; recursive Arena RF remains default and `DefaultDisabled` remains the explicit opt-out.
- ct_2a and ct_2c run on a live adapter, enroll non-empty participants, and pass against recursive-default execution plus RF-1. Removing participant enrollment must fail at plan-build; removing/redirecting governed Balance integration must still yield `ResidualNotIntegrated`.
- The same ct_2a/ct_2c commands fail closed on `NoAdapter`/Unsupported instead of reporting a passing skip. Record fail-then-pass evidence for both the enrollment defect and the fail-open mask.
- Existing RF-2 D=3 sibling marginal, zero-seeded residual, matching/nonmatching OrderBand, deterministic replay, and `DefaultDisabled` regressions remain green; no RR-3/RR-4 production import exists.
- ADR/design/orientation state names RF-2A + RF-2 graduated at `c206b0ef`, RF-3 active, and recursive-default + independent RF-1 as the canonical doctrine. Exact-head touched-crate checks, focused live-GPU tests, `agent_scan`, doctrine scan, orientation check, and doc budget are green; results/PR carry `tested_code_sha`, `coverage_basis`, HD receipt, and current ORIENT receipt.
