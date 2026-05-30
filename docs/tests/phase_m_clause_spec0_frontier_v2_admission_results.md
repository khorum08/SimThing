# CLAUSE-SPEC-0 — FrontierV2 Designer Scenario Admission Results

## Base HEAD

`d19d3e4541f699a1b8afadea86a7ce7c2e0c2b22`

Final commit SHA: available from the landing commit containing this report; not embedded to avoid self-referential SHA churn.

## Files changed

- `crates/simthing-spec/src/designer_admission/clause_spec.rs`
- `crates/simthing-spec/src/designer_admission/diagnostic.rs`
- `crates/simthing-spec/src/designer_admission/mod.rs`
- `crates/simthing-spec/src/designer_admission/preview.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/src/ron.rs`
- `crates/simthing-spec/tests/clause_spec0_frontier_v2_admission.rs`
- `crates/simthing-spec/tests/l1_0_designer_admission_substrate.rs`
- `crates/simthing-spec/tests/l1_1_designer_preflight_manifest.rs`
- `crates/simthing-driver/tests/phase_m_clause_spec0_frontier_v2_compile.rs`
- `docs/design_v7_8_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/sead_self_ai_track.md`
- `docs/worklog.md`
- `docs/tests/phase_m_clause_spec0_frontier_v2_admission_results.md`

## Required pre-edit evaluation

1. **What did L1-ACCEPT-0 open?** It accepted L1-0 diagnostics, L1-1 RON preflight manifests, and accepted FrontierV2 artifact target vocabulary as sufficient to open L2 / `CLAUSE-SPEC-0`.
2. **What exactly is in scope for `CLAUSE-SPEC-0`?** RON-first designer-authored FrontierV2 scenario admission through `simthing-spec`, plus lowering to accepted FrontierV2 fixture artifact metadata. No ClauseScript, ClauseThing runtime, production `SimSession`, scheduler/cache, or new GPU work.
3. **Which L1 diagnostics and preflight fields are reused?** The implementation reuses `DesignerAdmissionPreflightManifest`, `preview_designer_admission_preflight`, stable L1 diagnostic codes, feature-token guardrail routing, and accepted artifact target identifiers.
4. **Minimum designer-authored FrontierV2 scenario spec shape:** `scenario_id`, `profile_name`, `enabled_by_default`, `grid`, `closed_loop_ticks`, `factions`, `resource_flow`, `mapping`, `movement_feedback`, `structural_feedback`, and `artifact_targets`, with optional guardrail request booleans for negative admission tests.
5. **Accepted FrontierV2 artifacts lowered to:** `AcceptedFrontierV2FixtureArtifacts`, `FrontierV2CombinedFeedbackFixture`, `FrontierV2OwnColumnShadow`, `FrontierV2BoundaryRequestShadow`, and `ResourceFlowAllocatorRoute`.
6. **How default-off is enforced:** `enabled_by_default` and `default_sim_session_wiring` route through the L1 `DefaultOnRejected` diagnostic; defaults for mapping and Resource Flow remain disabled globally.
7. **Movement and structural guardrails:** movement admits only `OwnColumnShadowOnly` with no cross-entity or production writes; structural admits only `BoundaryRequestShadowOnly` with no production commitment emission.
8. **Resource Flow allocator routing:** resource dispatch must use `ResourceFlowAllocator`; bypass, parallel fixture economy, shared-pool tick writes, nested depth, E-11B-5, and D-2/D-2a requests reject at admission.
9. **Driver compile smoke:** `phase_m_clause_spec0_frontier_v2_compile` parses the RON scenario, admits it, checks the lowering summary against existing FrontierV2-4 fixture support constants, and validates the existing fixture skeleton without opening production runtime.
10. **Features parked:** ClauseThing, ClauseScript, production runtime wiring, FrontierV2-5, ACT/EVENT/OBS/PIPE expansion, semantic WGSL, CPU planner/urgency/commitment emission, simthing-sim semantic awareness, atlas, active mask, perception/fog, source identity, nested Resource Flow, D-2/D-2a, scheduler/cache.

## Designer scenario shape

The happy path is a RON-first `ClauseSpecFrontierV2Scenario`:

```ron
(
    scenario_id: "clause_spec0_frontier_v2_happy",
    profile_name: "FrontierV2",
    enabled_by_default: false,
    grid: (rows: 8, cols: 8),
    closed_loop_ticks: 4,
    factions: [
        (faction_id: "faction_a", initial_seed: 32),
        (faction_id: "faction_b", initial_seed: 24),
    ],
    resource_flow: (
        opt_in: FlatStarOptIn,
        execution_profile: FlatStarResourceFlow,
        route: ResourceFlowAllocator,
        depth_cap: 2,
    ),
    mapping: (
        execution_profile: SparseRegionFieldV1,
        atlas: false,
        active_mask: false,
        perception: false,
        source_identity: false,
    ),
    movement_feedback: (
        mode: OwnColumnShadowOnly,
        allow_cross_entity_write: false,
        allow_production_write: false,
    ),
    structural_feedback: (
        mode: BoundaryRequestShadowOnly,
        allow_production_commitment: false,
    ),
    artifact_targets: (
        accepted_frontier_v2_fixture_artifacts: true,
        combined_feedback_fixture: true,
        own_column_shadow: true,
        boundary_request_shadow: true,
        resource_flow_allocator_route: true,
    ),
)
```

## Admission and lowering

Pipeline:

`RON -> ClauseSpecFrontierV2Scenario -> L1 preflight preview -> CLAUSE-SPEC field validation -> ClauseSpecFrontierV2Admission -> accepted artifact target metadata`.

Lowering summary is metadata only:

- `maps_to_frontier_v2_combined_feedback_fixture = true`
- `resource_route = ResourceFlowAllocatorRoute`
- `movement = FrontierV2OwnColumnShadow`
- `structural = FrontierV2BoundaryRequestShadow`
- `metadata_only = true`

No production runtime object is opened by simthing-spec.

## Guardrail rejection matrix

| Guardrail | Diagnostic |
|---|---|
| default-on / default SimSession wiring | `L1-0-DEFAULT-ON-REJECTED` |
| non-FrontierV2 profile, bad grid/tick shape, missing target | `L1-0-MALFORMED-MANIFEST-REJECTED` |
| unknown artifact target | `L1-0-UNKNOWN-ARTIFACT-TARGET-REJECTED` |
| Resource Flow bypass | `L1-0-RESOURCE-FLOW-BYPASS-REJECTED` |
| nested RF depth > 2 / E-11B | `L1-0-NESTED-E11B-REQUESTED-WITHOUT-NAMED-SCENARIO` |
| E-11B-5 | `L1-0-E11B5-REQUESTED-WITHOUT-NAMED-SCENARIO` |
| D-2/D-2a | `L1-0-D2A-REQUESTED-WITHOUT-NAMED-SCENARIO` |
| cross-entity movement | `L1-0-CROSS-ENTITY-MOVEMENT-WRITE-REJECTED` |
| production movement | `L1-0-PRODUCTION-MOVEMENT-WRITE-REJECTED` |
| production commitment | `L1-0-PRODUCTION-COMMITMENT-EMISSION-REJECTED` |
| shared-pool tick write | `L1-0-SHARED-POOL-TICK-WRITE-REJECTED` |
| parallel fixture economy | `L1-0-PARALLEL-FIXTURE-ECONOMY-REJECTED` |
| CPU planner / urgency / commitment | `L1-0-CPU-PLANNER-REJECTED`, `L1-0-CPU-URGENCY-REJECTED`, `L1-0-CPU-COMMITMENT-EMISSION-REJECTED` |
| semantic WGSL | `L1-0-SEMANTIC-WGSL-REQUEST-REJECTED` |
| scheduler/cache | `L1-0-SCHEDULER-CACHE-REQUEST-REJECTED` |
| simthing-sim semantic leakage | `L1-0-SIMTHING-SIM-SEMANTIC-STATE-REQUEST-REJECTED` |
| atlas / active mask / perception / source identity | corresponding L1 parked mapping diagnostics |
| FrontierV2-5 | `L1-0-FRONTIERV2-5-REQUEST-REJECTED` |
| ACT/EVENT/OBS/PIPE reopen | `L1-0-ACT-EVENT-OBS-PIPE-LADDER-REOPEN-REJECTED` |
| ClauseScript / ClauseThing | `L1-0-CLAUSESCRIPT-PARSER-REQUEST-PARKED`, `L1-0-CLAUSETHING-RUNTIME-REQUEST-PARKED` |

## Diagnostic-code nit resolution

Resolved inline. L1 preview no longer reports empty `manifest_id`/`profile_name` or unknown artifact targets as `SimthingSimSemanticStateRequestRejected`.

New specific codes:

- `L1-0-MALFORMED-MANIFEST-REJECTED`
- `L1-0-UNKNOWN-ARTIFACT-TARGET-REJECTED`

## Test results

| Command | Result |
|---|---|
| `cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission -- --nocapture` | PASS — 25/25 |
| `cargo test -p simthing-spec --test l1_1_designer_preflight_manifest -- --nocapture` | PASS — 14/14 |
| `cargo test -p simthing-spec --test l1_0_designer_admission_substrate -- --nocapture` | PASS — 12/12 |
| `cargo test -p simthing-driver --test phase_m_clause_spec0_frontier_v2_compile -- --nocapture` | PASS — 1/1 |
| `cargo check --workspace` | PASS |

Pre-existing warnings observed: existing `simthing-core` deprecated `EmlTreeMeta` warnings and one existing `simthing-driver` unused import warning.

## Scans run

| Scan | Result |
|---|---|
| `rg "CLAUSE-SPEC-0|ClauseSpecFrontierV2|clause_spec0|designer-authored FrontierV2" crates docs` | PASS - implementation/report/docs references found; archive hits are historical/reference-only |
| `rg "FrontierV2-5|ACT-5|EVENT-3|OBS-5|PIPE-1" crates docs` | PASS - rejection/parked/historical references only; no new authorization |
| `rg "ClauseThing|ClauseScript" docs/design_v7_8_production_track.md docs/workshop/sead_self_ai_track.md docs/workshop/mapping_current_guidance.md docs/tests/phase_m_clause_spec0_frontier_v2_admission_results.md crates/simthing-spec` | PASS - parked/rejected references only; no parser/runtime |
| `rg "default SimSession|scheduler|kernel cache|semantic WGSL|CPU planner|CPU urgency|CPU commitment|production commitment|Resource Flow bypass|shared-pool tick|parallel fixture economy" crates docs/tests/phase_m_clause_spec0_frontier_v2_admission_results.md docs/design_v7_8_production_track.md docs/workshop/mapping_current_guidance.md docs/workshop/sead_self_ai_track.md docs/invariants.md` | PASS - guardrail/status references only; pre-existing scheduler/cache text is gated/historical |
| `rg "FrontierV2|ClauseThing|ClauseScript|SEAD|RegionCell|ArenaRegistry|proposal|ResourceFlow|BoundaryRequest" crates/simthing-sim` | PASS - only pre-existing generic `BoundaryRequest` plumbing appears; no FrontierV2, ClauseThing, ClauseScript, SEAD, RegionCell, ResourceFlow, or semantic awareness |
| `rg "Phase M.*closed|Phase E.*closed|M/E.*closed|FrontierV2.*accepted|ClauseThing.*unblocked|L2.*accepted" docs/tests/phase_m_clause_spec0_frontier_v2_admission_results.md docs/design_v7_8_production_track.md docs/workshop/mapping_current_guidance.md docs/workshop/sead_self_ai_track.md` | PASS - historical/design-authority status references only; CLAUSE-SPEC-0 remains implemented and pending review, not accepted |
| `Get-ChildItem -Path docs\tests -File | Where-Object { $_.Name -like "*.log" -or $_.Name -like "*tmp*" -or $_.Name -like "*scratch*" }` | PASS - no transient docs/test logs or scratch files found |

## Transient cleanup result

No authoritative evidence was deleted. Accidental broad `cargo fmt` churn was restored to keep the diff scoped. Pre-existing untracked local artifacts were left alone: `.claude/worktrees/`, `crates/simthing-workshop/target/`, and `demo.replay.ldjson`.

## Next gate status

| Gate | Status |
|---|---|
| L0 Frontier consumer | landed + ACCEPTED |
| L1 simthing-spec buildout | landed + ACCEPTED |
| L2 / `CLAUSE-SPEC-0` | implemented; pending design-authority review |
| L3 ClauseThing / ClauseScript | parked |
| FrontierV2-5 | rejected |
| ACT-5 / EVENT-3 / OBS-5 / PIPE-1 | unauthorized |
| Lines A/B/C | parked behind named scenarios and gates |

## Final verdict

PASS — CLAUSE-SPEC-0 admitted a RON-first designer-authored FrontierV2 scenario through simthing-spec; reused L1 preflight diagnostics; lowered the admitted scenario to accepted FrontierV2 fixture artifact targets; enforced guardrails at admission for Resource Flow bypass, cross-entity/production movement writes, production commitment emission, shared-pool tick writes, simthing-sim semantic leakage, FrontierV2-5, ACT/EVENT/OBS/PIPE reopen, ClauseScript, and ClauseThing requests; refined the L1 diagnostic-code labeling nit inline; updated the v7.8 production track and active guidance; saved test results in docs/tests; added no ClauseThing, ClauseScript parser, production SimSession wiring, scheduler/cache, semantic WGSL, CPU planner, simthing-sim semantic awareness, or line activation; and left L2 implemented but pending design-authority review.
