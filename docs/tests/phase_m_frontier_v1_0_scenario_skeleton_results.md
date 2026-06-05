# FrontierV1-0 — Opt-In Scenario Skeleton and Admission Contract Results

## Base HEAD

`71a0e73` (post-FIELD_POLICY-V1-CONSOLIDATE-0 merge, pre-FrontierV1-0)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/phase_m_frontier_v1_0_scenario_skeleton.rs` | **New** — scenario skeleton, admission validator, 8 tests |
| `docs/tests/phase_m_frontier_v1_0_scenario_skeleton_results.md` | **New** — this report |
| `docs/accumulator_op_v2_production_plan.md` | FrontierV1-0 section |
| `docs/workshop/mapping_current_guidance.md` | FrontierV1-0 row update |
| `docs/worklog.md` | Append-only milestone |

**No new WGSL, descriptor, AccumulatorRole, runtime wiring, scheduler/cache, default SimSession behavior, or simthing-sim semantic awareness.**

## Pre-edit evaluation summary

| Question | Answer |
|---|---|
| **1. Phase M closure needs** | One bounded ≤32×32 first-slice RegionCell theater (`source_capped_normalized`, H≤8, EveryTick/dirty-skip); opt-in `MappingExecutionProfile`; no atlas/active mask/perception/source identity |
| **2. Phase E closure needs** | Two-faction flat-star depth-2 Resource Flow economy; OrderBand sweeps via Resource Flow allocator; bounded economy↔field coupling scoped to FrontierV1 only |
| **3. Reused substrates** | First-slice RegionCell mapping; FlatStarResourceFlow; FIELD_POLICY Field agent Proposal Pipeline V1 (OBS-0..4 + EVENT-0..2 + PIPE-0 + ACT-0..2); exact F-backed magnitude; Threshold+EmitEvent→BoundaryRequest |
| **4. Deferred (out of scope)** | Atlas/M-4A; active mask/M-6A; perception/fog; source identity; nested E-11B/E-11B-5; D-2a; ClauseThing; dual-output GradientXY; L1 coupling |
| **5. Admission before runtime wiring** | Default-off profile; explicit opt-in mapping + Resource Flow; mapping bounds; flat-star caps; FIELD_POLICY V1 routing guardrails; coupling scoped to `FrontierV1` only; no CPU planner/semantic WGSL |
| **6. Smallest safe skeleton** | `FrontierV1ScenarioSkeleton` + `validate_frontier_v1_admission` — one theater, two factions, flat-star depth 2, FIELD_POLICY V1 routing contract, optional economy↔field coupling flags |

## FrontierV1 skeleton layout

```text
FrontierV1ScenarioSkeleton
├── profile_name = "FrontierV1"
├── enabled_by_default = false
├── mapping_execution_profile (Disabled unless explicitly SparseRegionFieldV1)
├── resource_flow_opt_in (Disabled unless explicitly FlatStarOptIn)
├── resource_flow_execution_profile
├── theater: FrontierTheaterSpec (1 theater, grid ≤32×32)
├── factions: [FrontierFactionSpec; 2]
├── resource_flow: FrontierFlatStarResourceFlowSpec
├── field_policy: FrontierFieldPolicyFieldAgentSpec
└── coupling: FrontierEconomyFieldCouplingSpec
```

Skeleton ID: `frontier_v1_0_scenario_skeleton_v1`

## Admission report fields

| Field | Meaning |
|---|---|
| `accepted` | All constraint groups pass |
| `mapping_ok` | Theater bounds and first-slice mapping constraints |
| `flat_star_ok` | Two factions, flat-star depth/caps, allocator routing |
| `field_policy_v1_ok` | Proposal Pipeline V1 + exact F + accepted routing substrates |
| `coupling_ok` | Economy↔field coupling scoped to FrontierV1, default-off |
| `default_off_ok` | Profile not enabled by default; no implicit default-on |
| `rejected_reasons` | Explicit static rejection strings |

## Mapping constraints

- One theater only; grid width/height ≤ 32
- `source_capped_normalized` operator; horizon 1..=8
- Cadence: `EveryTick` or bounded `EveryN`
- Dirty-skip allowed when already accepted
- Rejects: atlas, active mask, perception/fog, source identity

## Resource Flow constraints

- Two factions; flat-star depth ≤ 2; children per allocator ≤ 100
- Resource Flow allocator + OrderBand sweeps only
- Explicit `FlatStarOptIn` when selected
- Rejects: nested E-11B, E-11B-5, D-2a, shared-pool tick writes, parallel fixture economy

## FIELD_POLICY routing constraints

- FIELD_POLICY Field agent Proposal Pipeline V1 only
- Exact F-backed magnitude path only
- Resource dispatch via Resource Flow allocator
- Structural commitments via Threshold + EmitEvent → BoundaryRequest
- Movement writes to unit own columns only
- Rejects: CPU planner, CPU urgency, CPU commitment emission, semantic WGSL

## Economy↔field coupling scope

- District output may seed supply field (FrontierV1 only)
- Field-derived proposals dispatch via Resource Flow allocator (FrontierV1 only)
- Coupling rejected for any other profile name
- Coupling remains default-off (`enabled_by_default = false`)

## Deferred features preserved

Atlas, active mask, perception/fog, source identity, nested E-11B/E-11B-5, D-2a hard-currency ordering, ClauseThing implementation — all rejected at admission or explicitly absent from skeleton.

## Test results

```text
cargo test -p simthing-driver --test phase_m_frontier_v1_0_scenario_skeleton -- --nocapture
  → 8/8 PASS

cargo test -p simthing-driver --test phase_m_field_policy_act4_economic_fixture_validation_corpus -- --nocapture
  → 6/6 PASS

cargo test -p simthing-spec --test field_policy_obs0_overlay_score_admission -- --nocapture
  → 29/29 PASS

cargo check --workspace
  → PASS
```

| Test | Result |
|---|---|
| `frontier_v1_0_happy_path_skeleton_admits` | PASS |
| `frontier_v1_0_rejects_default_on` | PASS |
| `frontier_v1_0_rejects_out_of_bounds_mapping` | PASS |
| `frontier_v1_0_rejects_non_flat_star_resource_flow` | PASS |
| `frontier_v1_0_rejects_field_policy_routing_bypass` | PASS |
| `frontier_v1_0_coupling_scoped_to_frontier_only` | PASS |
| `frontier_v1_0_no_simthing_sim_semantic_awareness` | PASS |
| `frontier_v1_0_no_new_gpu_primitive` | PASS |

## Scans run

| Scan | Result |
|---|---|
| `FrontierV1\|frontier_v1_0\|FrontierV1-0` in crates/docs | skeleton test + report + active docs present |
| `ACT-5\|EVENT-3\|OBS-5\|PIPE-1` in crates/docs | no next-number authorization (negative/stop refs only) |
| guardrail terms in report + active docs | guardrail-only; no unauthorized widening |
| `FrontierV1\|FIELD_POLICY\|RegionCell\|ArenaRegistry\|proposal` in simthing-sim | no matches |
| `F64RoundDown\|SHADER_F64\|f64(\|sqrt_cr_c\|Candidate C\|native sqrt.*Exact` | no Candidate C/f64/native exact sqrt regression |
| `find docs/tests … scratch/tmp` | E-phase evidence logs retained; no scratch deleted |

## Transient cleanup result

No scratch/tmp artifacts removed under `docs/tests` (E-phase/E11 evidence logs retained).

## M/E Closure Relevance

1. **Phase M:** FrontierV1-0 defines the admission envelope for the single bounded first-slice RegionCell theater that closes mapping on accepted substrates — without atlas, active mask, or deferred M features.
2. **Phase E:** FrontierV1-0 validates flat-star depth-2 Resource Flow constraints and FIELD_POLICY V1 proposal routing through the Resource Flow allocator, establishing the scenario contract for economy closure at flat-star (not nested E-11B).
3. **Still pending before M/E close:** FrontierV1-1 end-to-end opt-in fixture wiring; GPU-resident integration; CPU-oracle parity; replay reproducibility behind explicit opt-in profile.
4. **Non-blocking deferred:** Atlas, active mask, perception, source identity, nested E-11B/E-11B-5, D-2a, ClauseThing — explicitly rejected or absent from skeleton.
5. **Not a new FIELD_POLICY ladder:** This is a named scenario integration slice per charter; no ACT-5/EVENT-3/OBS-5/PIPE-1; FIELD_POLICY V1 is consumed as consolidated vertical, not extended.

FrontierV1-0 creates the bounded default-off scenario skeleton and admission contract needed to turn accepted Mapping, Resource Flow, and FIELD_POLICY Field agent substrates into the named M/E closing vertical. It does not add runtime wiring yet; it defines and validates the minimal scenario envelope for the next end-to-end integration slice.

## Final verdict

**PASS** — FrontierV1-0 defined the default-off FrontierV1 scenario skeleton and admission contract for the named M/E closing vertical; validated one bounded first-slice RegionCell theater, flat-star Resource Flow constraints, and FIELD_POLICY Field agent Proposal Pipeline V1 routing through accepted substrates; preserved all deferred features and hard guardrails; added no runtime wiring, semantic WGSL, scheduler/cache/default SimSession behavior, CPU planner, or simthing-sim semantic awareness; updated active docs and production plan; saved test results in `docs/tests`; and kept V7.7 / Mapping ADR / Resource Flow ADR / FIELD_POLICY charter posture intact.
