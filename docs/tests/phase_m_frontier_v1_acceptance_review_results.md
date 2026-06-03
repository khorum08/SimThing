# FrontierV1-ACCEPT-0 — Formal FrontierV1 Acceptance Review and M/E Closure Decision

> **Closure status revised — 2026-06-03 (design authority).** The 2026-06-03 audit found the FrontierV1
> "SEAD route" (`validate_sead_v1_consumed`) only asserts that two kernel descriptors are **registered** —
> the diffused field is computed in the same fixture but **never consumed by SEAD to derive an action**.
> Under `invariants.md` "Scenario Proof" (2026-06-02) this acceptance is reclassified
> **numeric/registration-proven; consumption-proof pending** the dress-rehearsal **R4/R7** (production
> track `design_0_0_8_0_consumer_pulled_production_track.md` §12.5). The landed substrate **stands and is
> reused**; the field → SEAD → action loop is what R4 proves. **Dated evidence below is unaltered.**

## Base HEAD

`eaf78b7` (post-FrontierV1-4 merge, pre-FrontierV1-ACCEPT-0)

## Files changed

| File | Change |
|---|---|
| `docs/tests/phase_m_frontier_v1_acceptance_review_results.md` | **New** — this acceptance review |
| `docs/accumulator_op_v2_production_plan.md` | FrontierV1-ACCEPT-0 M/E closure section |
| `docs/workshop/mapping_current_guidance.md` | FrontierV1-ACCEPT-0 row update |
| `docs/worklog.md` | Append-only milestone |

**No implementation code, tests, WGSL, kernel descriptors, AccumulatorRole, default SimSession wiring, scheduler/cache, or simthing-sim changes.**

## Pre-edit evaluation summary

| Question | Answer |
|---|---|
| **1. Current FrontierV1 coverage** | V1-0 admission skeleton; V1-1 CPU-oracle fixture; V1-2 GPU mapping/replay; V1-3 GPU flat-star RF; V1-4 SEAD V1 route replay integration |
| **2. GPU-verified** | First-slice 8×8 RegionCell mapping; reduction + EML (field_urgency); flat-star Resource Flow allocation (exact CPU oracle parity) |
| **3. Replay-accepted** | SEAD PIPE-0 full chain (consumed via landed descriptors); route classification execution; resource/structural/movement route codes |
| **4. Is ReplayAccepted SEAD route coverage sufficient for M/E closure?** | **Yes** — route contracts are exact against CPU oracle; SEAD V1 consolidated evidence is consumed, not extended; observer→EML subset is GPU-resident |
| **5. Full PIPE-0 GPU re-run inside FrontierV1 required?** | **No — optional only.** Not a blocker; would be cosmetic re-execution of already-accepted SEAD V1 GPU evidence |
| **6. Deferred and non-blocking** | Atlas/M-4A; active mask/M-6A; perception/fog; source identity; nested E-11B/E-11B-5; D-2a; ClauseThing; broader production integration outside FrontierV1 |
| **7. Constitutional guardrails preserved?** | **Yes** — opt-in/default-off; no default SimSession; no scheduler/cache; no semantic WGSL; no CPU planner/urgency/commitment; no simthing-sim awareness; no parallel fixture economy; no shared-pool tick writes; SEAD ladder closed |
| **8. Formal M/E closure decision** | **ACCEPT — Phase M and Phase E closed for FrontierV1 scope** |

## Evidence chain reviewed

| Slice | Report | Role |
|---|---|---|
| FrontierV1-0 | [`phase_m_frontier_v1_0_scenario_skeleton_results.md`](phase_m_frontier_v1_0_scenario_skeleton_results.md) | Admission contract, default-off skeleton |
| FrontierV1-1 | [`phase_m_frontier_v1_1_opt_in_fixture_results.md`](phase_m_frontier_v1_1_opt_in_fixture_results.md) | CPU-oracle end-to-end fixture; fingerprint `49d4c94ce1f52be5` |
| FrontierV1-2 | [`phase_m_frontier_v1_2_gpu_replay_acceptance_results.md`](phase_m_frontier_v1_2_gpu_replay_acceptance_results.md) | GPU mapping + reduction/EML; fingerprint `42b0455e4d0b59ac` |
| FrontierV1-3 | [`phase_m_frontier_v1_3_gpu_resource_flow_results.md`](phase_m_frontier_v1_3_gpu_resource_flow_results.md) | GPU flat-star RF; fingerprint `7bacf7921b807bee` |
| FrontierV1-4 | [`phase_m_frontier_v1_4_sead_route_replay_results.md`](phase_m_frontier_v1_4_sead_route_replay_results.md) | SEAD V1 route replay; fingerprint `4382ec7ef93c9174` |
| SEAD-V1 | [`phase_m_sead_v1_consolidation_results.md`](phase_m_sead_v1_consolidation_results.md) | Consolidated Proposal Pipeline V1; ladder closed |

## Acceptance matrix

| Area | Evidence | Status | Acceptance decision |
|---|---|---|---|
| Mapping first-slice | FrontierV1-2/3/4 | GpuVerified | **Accept** |
| Reduction + EML | FrontierV1-2/3/4 | GpuVerified | **Accept** |
| Flat-star Resource Flow | FrontierV1-3/4 | GpuVerified | **Accept** |
| Resource dispatch route | FrontierV1-4 | ReplayAccepted | **Accept** |
| Structural route | FrontierV1-4 | ReplayAccepted | **Accept** |
| Movement route | FrontierV1-4 | ReplayAccepted | **Accept** |
| SEAD PIPE-0 V1 evidence | SEAD V1 consolidation + FrontierV1-4 | ReplayAccepted | **Accept** |
| Replay reproducibility | FrontierV1-1..4 | Stable fingerprints | **Accept** |
| Defaults/guardrails | FrontierV1-0..4 scans | Preserved | **Accept** |

## Stable replay fingerprints

| Fixture | Fingerprint |
|---|---|
| FrontierV1-1 CPU oracle | `49d4c94ce1f52be5` |
| FrontierV1-2 GPU mapping/replay | `42b0455e4d0b59ac` |
| FrontierV1-3 GPU Resource Flow | `7bacf7921b807bee` |
| FrontierV1-4 SEAD route replay | `4382ec7ef93c9174` |

## Guardrail verification

| Guardrail | Status |
|---|---|
| FrontierV1 opt-in/default-off | Preserved |
| No default SimSession behavior | Preserved |
| No scheduler/cache | Preserved |
| No semantic WGSL | Preserved |
| No CPU planner/urgency/commitment emission | Preserved |
| No simthing-sim semantic awareness | Preserved |
| No parallel fixture economy | Preserved |
| No shared-pool tick writes | Preserved |
| Resource dispatch via Resource Flow allocator | Verified |
| Structural via Threshold+EmitEvent→BoundaryRequest | Verified |
| Movement own-column-only | Verified |
| No ACT-5/EVENT-3/OBS-5/PIPE-1 | Preserved |
| SEAD ladder closed | Preserved |

## Optional vs required follow-up

| Item | Required for M/E closure? | Decision |
|---|---|---|
| Full PIPE-0 GPU re-run inside FrontierV1 | No | **Optional** — not authorized as FrontierV1-5 unless a future product need arises |
| FrontierV1-5 | No | **Not created** — no acceptance gap identified |
| Broader production SimSession wiring | No | **Out of scope** — separate gated decision |

## Smoke confirmation (optional)

```text
cargo check --workspace
  → PASS

cargo test -p simthing-driver --test phase_m_frontier_v1_4_sead_route_replay -- --nocapture
  → 9/9 PASS

cargo test -p simthing-driver --test phase_m_frontier_v1_3_gpu_resource_flow -- --nocapture
  → 9/9 PASS
```

## Scans run

| Scan | Result |
|---|---|
| `FrontierV1-ACCEPT-0\|frontier_v1_acceptance\|M/E closure\|ReplayAccepted\|GpuVerified` in docs | acceptance report + active docs present |
| `ACT-5\|EVENT-3\|OBS-5\|PIPE-1` in crates/docs | no next-number authorization (stop/negative refs only) |
| guardrail terms in report + active docs | guardrail-only |
| simthing-sim semantic markers | no matches |
| Candidate C/f64 regression | forbidden-term scan refs only in existing SEAD tests |
| scratch/tmp cleanup | E-phase `.log` evidence retained |

## Transient cleanup result

No scratch/tmp artifacts removed.

## M/E Closure Relevance

### Phase M closure (FrontierV1 scope)

FrontierV1 closes Phase M for the named vertical by GPU-verifying:

- bounded 8×8 first-slice RegionCell mapping (`SparseRegionFieldV1`, explicit opt-in);
- reduction + EML field_urgency execution on GPU;
- replay-stable mapping fingerprints.

Deferred M items (atlas, active mask, perception, source identity, dual-output GradientXY, L1 coupling) remain separately gated and are non-blocking for this vertical.

### Phase E closure (FrontierV1 scope)

FrontierV1 closes Phase E for the named vertical by GPU-verifying:

- flat-star depth-2 Resource Flow allocation through the accepted allocator path;
- exact CPU oracle parity on faction allocations.

Route and SEAD contracts are **ReplayAccepted** against consolidated SEAD V1 evidence and FrontierV1 admission rules — sufficient because route codes match CPU oracle exactly, invalid routes are zero, and overflow flags are zero.

### Formal decision

**ACCEPT — FrontierV1 is the accepted M/E closing vertical for its bounded scope.**

Phase M and Phase E are **closed for FrontierV1**. No FrontierV1-5 is required. Full PIPE-0 GPU re-run inside the FrontierV1 fixture is optional and not authorized unless a separate product need arises.

## Final verdict

**PASS** — FrontierV1-ACCEPT-0 reviewed FrontierV1-0..4 evidence; accepted GPU-verified Mapping and Resource Flow plus ReplayAccepted SEAD route contracts as sufficient for FrontierV1 M/E closure; recorded Phase M and Phase E closure for FrontierV1 scope; updated active docs and production plan; saved review results in `docs/tests`; added no code/runtime wiring; authorized no default SimSession behavior, scheduler/cache, semantic WGSL, CPU planner, parallel fixture economy, shared-pool tick writes, simthing-sim semantic awareness, or ACT/EVENT/PIPE ladder expansion; and kept V7.7 / Mapping ADR / Resource Flow ADR / SEAD charter posture intact.
