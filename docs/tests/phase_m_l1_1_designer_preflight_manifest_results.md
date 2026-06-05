# L1-1 — Designer Admission RON Preflight Manifest + Diagnostic Preview Results

## Base HEAD

`cecf40c6ddb4810085dbcf80dabf440f983e4de5` (post-L1-0 merge, pre-L1-1)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-spec/src/designer_admission/manifest.rs` | **New** — `DesignerAdmissionPreflightManifest` RON spec |
| `crates/simthing-spec/src/designer_admission/preview.rs` | **New** — `DesignerAdmissionPreviewReport` + preflight preview |
| `crates/simthing-spec/src/designer_admission/mod.rs` | Export manifest + preview |
| `crates/simthing-spec/src/ron.rs` | Deserialize/serialize preflight manifest RON |
| `crates/simthing-spec/src/lib.rs` | Public exports |
| `crates/simthing-spec/tests/l1_1_designer_preflight_manifest.rs` | **New** — 13 L1-1 tests |
| `crates/simthing-spec/tests/l1_0_designer_admission_substrate.rs` | Fix import-only simthing-sim awareness check |
| `docs/design_v7_8_production_track.md` | L1-1 row marked Done |
| `docs/workshop/mapping_current_guidance.md` | L1-1 status row |
| `docs/workshop/field_policy_track.md` | L1-1 clarification |
| `docs/worklog.md` | Append-only milestone |
| `docs/tests/phase_m_l1_1_designer_preflight_manifest_results.md` | **New** — this report |

**No full scenario admission, no ClauseThing, no ClauseScript, no FrontierV2-5, no production runtime, no default SimSession wiring, no phase closure declaration.**

Standing posture: no semantic WGSL / no default wiring / `simthing-sim` map-free / defaults unchanged.

## Pre-edit evaluation summary

| Question | Answer |
|---|---|
| **1. What did L1-0 provide?** | 24 stable `L1-0-*` diagnostic codes, `DesignerAdmissionRequest` preflight evaluation, `AcceptedFrontierArtifactTarget` lowering vocabulary, and 12 substrate tests. |
| **2. What does L1 still need before L2 / CLAUSE-SPEC?** | RON-first designer input surface, preview reports that compose L1-0 diagnostics from shallow manifests, and eventual full FrontierV2 scenario spec + compile-to-artifacts path (L2). |
| **3. What should a RON preflight manifest contain?** | Shallow posture fields: `manifest_id`, `profile_name`, `enabled_by_default`, requested artifact targets, guardrail overrides, runtime/mapping/resource-flow/authoring feature tokens — not a full scenario graph. |
| **4. What diagnostics should the preview surface emit?** | Stable L1-0 `DesignerAdmissionDiagnostic` entries with codes, guardrail classes, messages, and hints; aggregated in `DesignerAdmissionPreviewReport` with `rejected` flag and `summary_lines`. |
| **5. What accepted FrontierV2 artifact targets can be named without runtime wiring?** | All five L1-0 targets as metadata-only strings in `requested_artifact_targets`; resolved and listed in `accepted_artifact_targets` without GPU/driver invocation. |
| **6. What must remain parked?** | L2 CLAUSE-SPEC-0, L3 ClauseThing/ClauseScript, FrontierV2-5, ACT-5/EVENT-3/OBS-5/PIPE-1, Lines A/B/C, production SimSession wiring, scheduler/cache. |
| **7. Why this is not L2 and not a hygiene loop?** | L1-1 exercises guardrail vocabulary from designer-authored RON input but does **not** admit or compile a full FrontierV2 scenario — that is L2. It ships concrete spec-layer code toward the ClauseThing horizon. |

## L1-0 summary

Shared designer admission substrate: diagnostic codes, rejection kinds, preflight request evaluation, artifact target identifiers. No runtime wiring.

## v7.8 production-track status

| Ladder | Status |
|---|---|
| L0 Frontier consumer | landed + ACCEPTED |
| L1 simthing-spec buildout | **active — L1-0 + L1-1 landed** |
| L2 CLAUSE-SPEC | parked downstream of L1 |
| L3 ClauseThing | parked |

## Manifest shape

```text
DesignerAdmissionPreflightManifest {
  manifest_id: String
  profile_name: String
  enabled_by_default: bool (default false)
  requested_artifact_targets: [String]
  requested_guardrail_overrides: [String]
  requested_runtime_features: [String]
  requested_mapping_features: [String]
  requested_resource_flow_features: [String]
  requested_authoring_frontend: [String]
  cross_entity_movement_source_unit: Option<u32>
  cross_entity_movement_target_unit: Option<u32>
}
```

## Preview report shape

```text
DesignerAdmissionPreviewReport {
  manifest_id: String
  accepted_artifact_targets: [String]
  diagnostics: [DesignerAdmissionDiagnostic]
  rejected: bool
  summary_lines: [String]
}
```

## RON example summary

Happy-path manifest (roundtrips, non-rejected preview):

```ron
(
    manifest_id: "frontier_v2_preflight_happy",
    profile_name: "FrontierV2",
    enabled_by_default: false,
    requested_artifact_targets: [
        "AcceptedFrontierV2FixtureArtifacts",
        "FrontierV2CombinedFeedbackFixture",
        "ResourceFlowAllocatorRoute",
    ],
)
```

## Diagnostic preview matrix

| Feature token / condition | L1-0 diagnostic code |
|---|---|
| `enabled_by_default: true` | L1-0-DEFAULT-ON-REJECTED |
| `resource_flow_bypass` | L1-0-RESOURCE-FLOW-BYPASS-REJECTED |
| `cross_entity_movement_write` | L1-0-CROSS-ENTITY-MOVEMENT-WRITE-REJECTED |
| `production_commitment_emission` | L1-0-PRODUCTION-COMMITMENT-EMISSION-REJECTED |
| `shared_pool_tick_write` | L1-0-SHARED-POOL-TICK-WRITE-REJECTED |
| `clause_script_parser` | L1-0-CLAUSESCRIPT-PARSER-REQUEST-PARKED |
| `clausething_runtime` | L1-0-CLAUSETHING-RUNTIME-REQUEST-PARKED |
| `frontier_v2_5` | L1-0-FRONTIERV2-5-REQUEST-REJECTED |
| `act_5` / `event_3` / `obs_5` / `pipe_1` | L1-0-ACT-EVENT-OBS-PIPE-LADDER-REOPEN-REJECTED |
| `atlas_batching` | L1-0-ATLAS-REQUESTED-WITHOUT-GATE |
| `active_mask` | L1-0-ACTIVE-MASK-REQUESTED-WITHOUT-GATE |
| `perception_fog` | L1-0-PERCEPTION-FOG-REQUESTED-WITHOUT-GATE |
| `source_identity` | L1-0-SOURCE-IDENTITY-REQUESTED-WITHOUT-GATE |
| `nested_e11b` | L1-0-NESTED-E11B-REQUESTED-WITHOUT-NAMED-SCENARIO |
| `e11b5_dynamic_enrollment` | L1-0-E11B5-REQUESTED-WITHOUT-NAMED-SCENARIO |
| `d2a_boundary_scheduling` | L1-0-D2A-REQUESTED-WITHOUT-NAMED-SCENARIO |

## Test results

```text
cargo test -p simthing-spec --test l1_1_designer_preflight_manifest -- --nocapture
  test result: ok. 13 passed; 0 failed

cargo test -p simthing-spec --test l1_0_designer_admission_substrate -- --nocapture
  test result: ok. 12 passed; 0 failed

cargo test -p simthing-driver --test phase_m_frontier_v2_4_combined_feedback_loop -- --nocapture
  test result: ok. 12 passed; 0 failed

cargo test -p simthing-spec --test field_policy_obs0_overlay_score_admission -- --nocapture
  test result: ok. 29 passed; 0 failed

cargo check --workspace
  Finished dev profile (warnings only, pre-existing)
```

## Scans run

| Scan | Expected | Result |
|---|---|---|
| `L1-1\|DesignerAdmissionPreflightManifest\|DesignerAdmissionPreviewReport` | new L1-1 artifacts | PASS |
| `FrontierV2-5\|ACT-5\|EVENT-3\|OBS-5\|PIPE-1` | rejection/guardrail only | PASS |
| `CLAUSE-SPEC-0\|ClauseThing\|ClauseScript` in active docs | L2/downstream parked | PASS |
| Guardrail terms in active docs | guardrail-only | PASS |
| Semantic terms in `crates/simthing-sim` | generic BoundaryRequest only | PASS |
| Phase closure / self-acceptance | none in L1-1 code | PASS |
| scratch/tmp under docs/tests | none | PASS |

## Transient cleanup result

No scratch/tmp/log artifacts under `docs/tests/` required deletion.

## Next gate status

| Gate | Status |
|---|---|
| L1 simthing-spec buildout | **in progress** — L1-0 + L1-1 landed; further L1 steps TBD |
| L2 CLAUSE-SPEC-0 | **parked** |
| L3 ClauseThing / ClauseScript | **parked** |
| FrontierV2-5 | **rejected** |

## Final verdict

**PASS** — L1-1 added a RON-first designer admission preflight manifest and diagnostic preview surface; exercised L1-0 guardrail diagnostics from shallow designer-authored input; allowed accepted FrontierV2 artifact target requests as metadata only; kept CLAUSE-SPEC-0 parked downstream; updated the v7.8 production track and active guidance; saved test results in docs/tests; added no full scenario admission, ClauseThing, ClauseScript, FrontierV2-5, production runtime wiring, default SimSession behavior, scheduler/cache, semantic WGSL, CPU planner, simthing-sim semantic awareness, or ACT/EVENT/OBS/PIPE expansion; and preserved the v7.8 constitution / production-track split.
