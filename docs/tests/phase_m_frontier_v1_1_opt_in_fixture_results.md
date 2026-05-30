# FrontierV1-1 — Opt-In End-to-End Fixture Wiring Results

## Base HEAD

`4af476c` (post-FrontierV1-0 merge, pre-FrontierV1-1)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/tests/support/frontier_v1.rs` | **New** — shared skeleton, admission, CPU oracle, fixture runner |
| `crates/simthing-driver/tests/phase_m_frontier_v1_0_scenario_skeleton.rs` | Refactored to use shared `support/frontier_v1.rs` |
| `crates/simthing-driver/tests/phase_m_frontier_v1_1_opt_in_fixture.rs` | **New** — end-to-end opt-in fixture, 9 tests |
| `crates/simthing-driver/tests/support/mod.rs` | Export `frontier_v1` module |
| `docs/tests/phase_m_frontier_v1_1_opt_in_fixture_results.md` | **New** — this report |
| `docs/accumulator_op_v2_production_plan.md` | FrontierV1-1 section |
| `docs/workshop/mapping_current_guidance.md` | FrontierV1-1 row update |
| `docs/worklog.md` | Append-only milestone |

**No new WGSL, descriptor, AccumulatorRole, default SimSession wiring, scheduler/cache, or simthing-sim semantic awareness.**

## Pre-edit evaluation summary

| Question | Answer |
|---|---|
| **1. FrontierV1-0 validated** | Default-off `FrontierV1` profile; mapping ≤32×32; flat-star depth-2; SEAD V1 routing; coupling scoped to FrontierV1; admission validator |
| **2. Allowed substrates for V1-1** | First-slice RegionCell mapping (`SparseRegionFieldV1`); flat-star Resource Flow (`FlatStarOptIn` + `FlatStarResourceFlow`); SEAD Proposal Pipeline V1 (consumed, not extended); exact F magnitude path; Threshold+EmitEvent; own-column movement |
| **3. Minimal fixture wiring** | Admit skeleton → 8×8 mapping oracle → flat-star allocation oracle → SEAD route classification → output summary/fingerprint |
| **4. CPU oracle parity** | Mapping cell sum/overflow; resource allocation A/B; proposal/route counts; event count; fingerprint match |
| **5. Replay reproducibility** | Two identical runs → same fingerprint `49d4c94ce1f52be5` |
| **6. Remains deferred/rejected** | Atlas, active mask, perception, source identity, nested E-11B/E-11B-5, D-2a, ClauseThing, ACT-5/EVENT-3/OBS-5/PIPE-1, CPU planner |
| **7. M/E closure movement** | First opt-in end-to-end fixture wiring proves accepted substrates compose coherently; GPU-resident execution remains for FrontierV1-2 |

## Fixture layout

```text
FrontierV1ScenarioSkeleton (8×8 smoke)
  → validate_frontier_v1_admission()
  → cpu_mapping_oracle()        // source_capped seed + district coupling
  → cpu_resource_flow_oracle()  // flat-star OrderBand-style 3:2 split
  → cpu_route_oracle()          // ResourceFlowAllocator / ThresholdEmit / OwnColumns
  → FrontierV1FixtureOutput + FrontierV1FixtureFingerprint
```

Fixture ID: `frontier_v1_1_opt_in_fixture_v1`

## Explicit opt-in profile settings

| Setting | Value |
|---|---|
| `profile_name` | `FrontierV1` |
| `enabled_by_default` | `false` |
| `mapping_execution_profile` | `SparseRegionFieldV1` (explicit) |
| `resource_flow_opt_in` | `FlatStarOptIn` (explicit) |
| `resource_flow_execution_profile` | `FlatStarResourceFlow` (explicit) |
| Global defaults | `MappingExecutionProfile::Disabled`, `ResourceFlowOptInMode::Disabled` |

## Mapping fixture summary

- One theater, 8×8 smoke grid (32×32 cap validated by FrontierV1-0 admission)
- `source_capped_normalized`, horizon 8, `EveryTick`
- District outputs seed corner cells (economy↔field coupling)
- No atlas, active mask, perception, source identity

## Resource Flow fixture summary

- Two factions, flat-star depth 2
- OrderBand-style 3:2 allocation split via CPU oracle
- Resource Flow allocator routing only
- No nested E-11B, E-11B-5, D-2a, shared-pool writes, parallel fixture economy

## SEAD proposal routing summary

| Proposal kind | Route | Status |
|---|---|---|
| Resource dispatch | `ResourceFlowAllocator` | **Executed in fixture** |
| Structural commit | `ThresholdEmitBoundaryRequest` | Classified (CPU oracle) |
| Movement | `OwnColumnsOnly` | Classified (CPU oracle) |

SEAD Proposal Pipeline V1 consumed as consolidated vertical — not extended.

## CPU oracle design

Integer-only deterministic oracles:

- **Mapping:** seeded cells + district coupling + source cap over horizon ticks
- **Resource Flow:** district totals + coupling bonus → 3:2 faction split
- **Routing:** `classify_proposal_route()` per SEAD guardrails
- **Overflow flags:** tracked but false on smoke config

## Replay fingerprint design

```text
FrontierV1FixtureFingerprint {
    mapping_summary_hash,
    resource_flow_summary_hash,
    proposal_summary_hash,
    route_summary_hash,
}
```

Stable FNV-1a64-style hashing over integer summaries. Combined fingerprint: **`49d4c94ce1f52be5`**

## Test results

```text
cargo test -p simthing-driver --test phase_m_frontier_v1_1_opt_in_fixture -- --nocapture
  → 9/9 PASS

cargo test -p simthing-driver --test phase_m_frontier_v1_0_scenario_skeleton -- --nocapture
  → 8/8 PASS

cargo test -p simthing-driver --test phase_m_sead_act4_economic_fixture_validation_corpus -- --nocapture
  → 6/6 PASS

cargo test -p simthing-spec --test sead_obs0_overlay_score_admission -- --nocapture
  → 29/29 PASS

cargo check --workspace
  → PASS
```

| Test | Result |
|---|---|
| `frontier_v1_1_happy_path_opt_in_fixture_runs` | PASS |
| `frontier_v1_1_defaults_remain_disabled` | PASS |
| `frontier_v1_1_resource_dispatch_routes_through_allocator` | PASS |
| `frontier_v1_1_coupling_rejects_non_frontier_profile` | PASS |
| `frontier_v1_1_deferred_features_reject` | PASS |
| `frontier_v1_1_cpu_oracle_parity` | PASS |
| `frontier_v1_1_replay_reproducibility` | PASS |
| `frontier_v1_1_no_simthing_sim_semantic_awareness` | PASS |
| `frontier_v1_1_no_new_gpu_primitive` | PASS |

## Scans run

| Scan | Result |
|---|---|
| `FrontierV1-1\|frontier_v1_1\|phase_m_frontier_v1_1` in crates/docs | fixture + report + active docs present |
| `ACT-5\|EVENT-3\|OBS-5\|PIPE-1` in crates/docs | no next-number authorization |
| guardrail terms in report + active docs | guardrail-only; no unauthorized widening |
| `FrontierV1\|SEAD\|RegionCell\|ArenaRegistry\|proposal\|ResourceFlow` in simthing-sim | no matches |
| Candidate C/f64/native exact sqrt regression scan | no regression |
| `find docs/tests … scratch/tmp` | E-phase evidence logs retained |

## Transient cleanup result

No scratch/tmp artifacts removed under `docs/tests`.

## M/E Closure Relevance

1. **Builds on FrontierV1-0:** Reuses admission validator and extends skeleton into runnable fixture with CPU oracle outputs.
2. **Phase M proof:** First-slice 8×8 mapping fixture wired with explicit opt-in profile; district→field coupling exercised in oracle.
3. **Phase E proof:** Flat-star Resource Flow allocation oracle with allocator routing; resource dispatch proposal routes through Resource Flow allocator.
4. **Still pending:** FrontierV1-2 GPU-resident execution + replay acceptance; production-doc M/E closure sign-off.
5. **Non-blocking deferred:** Atlas, active mask, perception, source identity, nested E-11B/E-11B-5, D-2a, ClauseThing.
6. **Not a SEAD ladder:** Consumes SEAD V1 vertical; no ACT-5/EVENT-3/OBS-5/PIPE-1; named scenario integration only.

FrontierV1-1 proves the first opt-in end-to-end fixture wiring for the named M/E closing vertical. It consumes accepted Mapping, Resource Flow, and SEAD Self-AI substrates without extending the SEAD ladder or adding default runtime behavior. M/E closure still requires GPU-resident execution evidence, CPU oracle parity, replay reproducibility, and production-doc acceptance, but deferred features remain non-blocking.

## Final verdict

**PASS** — FrontierV1-1 wired the default-off FrontierV1 fixture through accepted first-slice Mapping, flat-star Resource Flow, and SEAD Self-AI Proposal Pipeline V1 substrates; resource dispatch routes through Resource Flow allocator; coupling remains FrontierV1-only/default-off; CPU oracle parity and replay reproducibility were recorded; deferred features stayed out of scope; no default SimSession behavior, scheduler/cache, semantic WGSL, CPU planner, or simthing-sim semantic awareness was added; docs and production plan were updated; test results saved in `docs/tests`; and V7.7 / Mapping ADR / Resource Flow ADR / SEAD charter posture remained intact.
