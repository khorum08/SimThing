# L1-0 — simthing-spec Designer Admission Substrate Preflight Results

## Base HEAD

`9e0d597377f2a3784df8ec736bf4ffc2bf2db01b` (post v7.8 constitution/production-track split, pre-L1-0)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-spec/src/designer_admission/mod.rs` | **New** — L1 designer admission substrate module |
| `crates/simthing-spec/src/designer_admission/diagnostic.rs` | **New** — stable diagnostic codes + guardrail classes |
| `crates/simthing-spec/src/designer_admission/artifact_target.rs` | **New** — accepted FrontierV2 artifact target vocabulary |
| `crates/simthing-spec/src/designer_admission/preflight.rs` | **New** — preflight request evaluation |
| `crates/simthing-spec/src/lib.rs` | Export designer admission types |
| `crates/simthing-spec/tests/l1_0_designer_admission_substrate.rs` | **New** — 12 L1-0 tests |
| `docs/design_v7_8_production_track.md` | L1-0 row marked Done |
| `docs/workshop/mapping_current_guidance.md` | L1-0 status row |
| `docs/workshop/sead_self_ai_track.md` | L1-0 clarification under §11 |
| `docs/accumulator_op_v2_production_plan.md` | Pointer to v7.8 production track |
| `docs/worklog.md` | Append-only L1-0 milestone |
| `docs/tests/phase_m_l1_0_designer_admission_substrate_results.md` | **New** — this report |

**No ClauseThing, no ClauseScript parser, no FrontierV2-5, no production runtime, no default SimSession wiring, no phase closure declaration.**

Standing posture: no semantic WGSL / no default wiring / `simthing-sim` map-free / defaults unchanged.

## Pre-edit evaluation summary

| Question | Answer |
|---|---|
| **1. What changed in the v7.8 constitution / production-track split?** | `design_v7_8.md` became the **constitution** (operating doctrine §2 + parked line state A/B/C). `design_v7_8_production_track.md` became the **PR-ladder home** (L0 landed, L1 active, L2–L3 and Lines A–C parked). |
| **2. Why is L1 active before L2 / CLAUSE-SPEC-0?** | L0 proved the consumer at fixture level; guardrails must **relocate to spec admission** before a designer-authored scenario can be admitted. L1 builds the shared substrate; L2 consumes it for full FrontierV2 scenario admission. |
| **3. What does L1 need to provide for L2?** | Shared `DesignerAdmissionDiagnostic` vocabulary, stable rejection codes, guardrail classes, and `AcceptedFrontierArtifactTarget` lowering identifiers — without admitting a full scenario or invoking runtime. |
| **4. Which guardrails must become designer/spec admission diagnostics?** | Default-on, RF bypass, cross-entity/production movement writes, production commitment emission, shared-pool tick writes, CPU planner/urgency/commitment, semantic WGSL, scheduler/cache, simthing-sim semantic leakage, atlas/mask/perception/source without gates, nested E-11B/E-11B-5/D-2a without named scenarios, ClauseScript/ClauseThing parked, FrontierV2-5 rejected, ACT-5/EVENT-3/OBS-5/PIPE-1 rejected. |
| **5. Which docs still risk implying CLAUSE-SPEC-0 starts immediately?** | `sead_self_ai_track.md` §11 names CLAUSE-SPEC-0 as the *design* next gate; clarified by new L1-0 subsection and production-track sequencing (L1 before L2). `mapping_current_guidance.md` forward horizon updated to L1-first. |
| **6. What must remain parked?** | L2 CLAUSE-SPEC-0, L3 ClauseThing/ClauseScript, Lines A/B/C, FrontierV2-5, ACT-5/EVENT-3/OBS-5/PIPE-1, atlas/active mask/perception/source identity/nested E-11B/D-2a, production SimSession wiring, scheduler/cache. |
| **7. Why this is not another hygiene loop?** | L1-0 ships **designer-facing admission substrate code** that L2 will consume — not a third meta-doc or consumer-less prooflet. It is the first rung of the simthing-spec buildout named in the v7.8 production track. |

## v7.8 constitution summary

v7.8 is the bounded-posture **expansion constitution**: operating doctrine at §2 (guardrails at designer/spec admission; semantic-WGSL ban is semantic-only; EML gadgets at designer layer; Tier-1/Tier-2 gating; §2.5 non-negotiables). Lines A/B/C are parked behind named scenarios. The constitution authorizes; it does not sequence PRs.

## v7.8 production-track summary

| Ladder | Status |
|---|---|
| L0 Frontier consumer | landed + ACCEPTED (V1-5 → V2-0..4) |
| **L1 simthing-spec buildout** | **active — L1-0 landed** |
| L2 CLAUSE-SPEC | parked downstream of L1 |
| L3 ClauseThing | parked downstream of L2 |
| Lines A/B/C | parked |

## Admission diagnostic vocabulary

- `DesignerFacingGuardrailClass` — 14 guardrail buckets (DefaultOff, ResourceFlowRouting, MovementWriteBoundary, …)
- `DesignerAdmissionRejectionKind` — 24 rejection kinds
- `DesignerAdmissionDiagnosticCode` — 24 stable `L1-0-*` string codes
- `DesignerAdmissionDiagnostic` — designer-facing message + optional hint
- `DesignerAdmissionRequest` + `evaluate_designer_admission_request` — preflight evaluation surface

## Guardrail rejection matrix

| Diagnostic code | Guardrail class | Rejection kind |
|---|---|---|
| L1-0-DEFAULT-ON-REJECTED | DefaultOff | DefaultOnRequest |
| L1-0-RESOURCE-FLOW-BYPASS-REJECTED | ResourceFlowRouting | ResourceFlowBypass |
| L1-0-CROSS-ENTITY-MOVEMENT-WRITE-REJECTED | MovementWriteBoundary | CrossEntityMovementWrite |
| L1-0-PRODUCTION-MOVEMENT-WRITE-REJECTED | MovementWriteBoundary | ProductionMovementWrite |
| L1-0-PRODUCTION-COMMITMENT-EMISSION-REJECTED | CommitmentEmission | ProductionCommitmentEmission |
| L1-0-SHARED-POOL-TICK-WRITE-REJECTED | TickTimeContention | SharedPoolTickWrite |
| L1-0-PARALLEL-FIXTURE-ECONOMY-REJECTED | EconomySubstrate | ParallelFixtureEconomy |
| L1-0-CPU-PLANNER-REJECTED | CpuDecisionPath | CpuPlanner |
| L1-0-CPU-URGENCY-REJECTED | CpuDecisionPath | CpuUrgency |
| L1-0-CPU-COMMITMENT-EMISSION-REJECTED | CpuDecisionPath | CpuCommitmentEmission |
| L1-0-SEMANTIC-WGSL-REQUEST-REJECTED | ShaderSemantics | SemanticWgslRequest |
| L1-0-SCHEDULER-CACHE-REQUEST-REJECTED | RuntimeWiring | SchedulerCacheRequest |
| L1-0-SIMTHING-SIM-SEMANTIC-STATE-REQUEST-REJECTED | SimSemanticLeakage | SimthingSimSemanticStateRequest |
| L1-0-ATLAS-REQUESTED-WITHOUT-GATE | MappingExpansion | AtlasWithoutGate |
| L1-0-ACTIVE-MASK-REQUESTED-WITHOUT-GATE | MappingExpansion | ActiveMaskWithoutGate |
| L1-0-PERCEPTION-FOG-REQUESTED-WITHOUT-GATE | MappingExpansion | PerceptionFogWithoutGate |
| L1-0-SOURCE-IDENTITY-REQUESTED-WITHOUT-GATE | MappingExpansion | SourceIdentityWithoutGate |
| L1-0-NESTED-E11B-REQUESTED-WITHOUT-NAMED-SCENARIO | ResourceFlowExpansion | NestedE11BWithoutNamedScenario |
| L1-0-E11B5-REQUESTED-WITHOUT-NAMED-SCENARIO | ResourceFlowExpansion | E11B5WithoutNamedScenario |
| L1-0-D2A-REQUESTED-WITHOUT-NAMED-SCENARIO | DiscreteOrderingExpansion | D2aWithoutNamedScenario |
| L1-0-CLAUSESCRIPT-PARSER-REQUEST-PARKED | AuthoringFrontEnd | ClauseScriptParserParked |
| L1-0-CLAUSETHING-RUNTIME-REQUEST-PARKED | AuthoringFrontEnd | ClauseThingRuntimeParked |
| L1-0-FRONTIERV2-5-REQUEST-REJECTED | ConsumerProofLadder | FrontierV2FiveRejected |
| L1-0-ACT-EVENT-OBS-PIPE-LADDER-REOPEN-REJECTED | ConsumerProofLadder | SeadLadderReopenRejected |

## Lowering target vocabulary

| Target ID | Description |
|---|---|
| `AcceptedFrontierV2FixtureArtifacts` | Accepted FrontierV2 fixture artifact set (L0 V1-5 → V2-0..4) |
| `FrontierV2CombinedFeedbackFixture` | FrontierV2-4 combined feedback loop (fingerprint `dbb54b952f9face8`) |
| `FrontierV2OwnColumnShadow` | Fixture-only own-column movement shadow |
| `FrontierV2BoundaryRequestShadow` | Fixture-only BoundaryRequest shadow |
| `ResourceFlowAllocatorRoute` | Resource dispatch via accepted Resource Flow allocator |

Metadata only — no runtime invocation.

## Test results

```text
cargo test -p simthing-spec --test l1_0_designer_admission_substrate -- --nocapture
  test result: ok. 12 passed; 0 failed

cargo test -p simthing-driver --test phase_m_frontier_v2_4_combined_feedback_loop -- --nocapture
  test result: ok. 12 passed; 0 failed

cargo test -p simthing-spec --test sead_obs0_overlay_score_admission -- --nocapture
  test result: ok. 29 passed; 0 failed

cargo check --workspace
  Finished dev profile (warnings only, pre-existing)
```

## Scans run

| Scan | Expected | Result |
|---|---|---|
| `L1-0\|DesignerAdmissionDiagnostic\|AcceptedFrontierV2FixtureArtifacts` in crates/docs | new L1-0 artifacts present | PASS — simthing-spec module + tests + docs |
| `FrontierV2-5\|ACT-5\|EVENT-3\|OBS-5\|PIPE-1` in crates/docs | negative/guardrail only | PASS — rejection codes in designer_admission; historical stop refs elsewhere |
| `CLAUSE-SPEC-0\|ClauseThing\|ClauseScript` in production track / sead / guidance | L2/downstream parked | PASS — L2 parked; L1-0 clarification added |
| Guardrail terms in active docs | guardrail-only | PASS — no unauthorized widening |
| Semantic terms in `crates/simthing-sim` | no awareness | PASS — no FrontierV2/ClauseThing/SEAD matches |
| Phase closure / self-acceptance in L1-0 report/docs | design-authority refs only | PASS — no implementer closure in L1-0 code |
| `find docs/tests … tmp/scratch/log` | none or delete | PASS — no scratch/tmp artifacts found |

## Transient cleanup result

No scratch/tmp/log artifacts under `docs/tests/` required deletion.

## Next gate status

| Gate | Status |
|---|---|
| L1 simthing-spec buildout | **in progress** — L1-0 landed; further L1 steps TBD |
| L2 CLAUSE-SPEC-0 | **parked** — starts after L1 |
| L3 ClauseThing / ClauseScript | **parked** |
| FrontierV2-5 | **rejected** |
| Lines A/B/C | **parked** |

## Final verdict

**PASS** — L1-0 began the v7.8 simthing-spec designer admission substrate buildout by adding shared guardrail diagnostics and accepted FrontierV2 artifact-target vocabulary; kept CLAUSE-SPEC-0 parked downstream; updated the v7.8 production track and active guidance; saved test results in docs/tests; added no ClauseThing, ClauseScript, FrontierV2-5, production runtime wiring, default SimSession behavior, scheduler/cache, semantic WGSL, CPU planner, simthing-sim semantic awareness, or ACT/EVENT/OBS/PIPE expansion; and preserved the v7.8 constitution / production-track split.
