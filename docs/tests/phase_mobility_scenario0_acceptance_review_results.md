# MOBILITY-SCENARIO-0-ACCEPT-0 — Design-Authority / Product Acceptance Review Results

## Verdict

**ACCEPT MOBILITY-SCENARIO-0 (Option A).** The landed scenario/admission packet is accepted as the
named product scenario for the v7.9 mobility/transfer track. Acceptance opens **only** the next narrow
gate, `MOBILITY-AUDIT-0 / owner_band_budget_audit`. No ALLOC/REENROLL/IDROUTE/ECON/OWNER
implementation gate is opened. No runtime implementation, no invariant change, no default-on behavior.

The packet is intrinsically first-slice-narrowed by construction (routing
`NarrowedAdversarialFirstSlice`, spatial depth 4, `max_factions_per_cell` 4, 48 cells, 34k soak), so
acceptance requires no additional narrowing (Option B not needed).

## Base HEAD

`6c57aa7` (master — MOBILITY-SCENARIO-0 admission merge, PR #368) + this acceptance commit.

## Reviewer

Design authority (Opus 4.8 lane) + product. Acceptance is design-authority/product scoped, not
implementer self-acceptance — the packet itself forces `status = ScenarioAdmissionProposed` and rejects
self-authorization.

## Reviewed files

- `crates/simthing-spec/src/designer_admission/mobility_scenario0.rs` (packet, bounds + guardrail validation)
- `crates/simthing-spec/src/designer_admission/diagnostic.rs` (12 MOBILITY-SCENARIO diagnostic codes wired: enum, `as_str`, guardrail class, rejection kind, registry list)
- `crates/simthing-spec/tests/mobility_scenario0_admission.rs` (13 admission/rejection tests, RON roundtrip)
- `docs/tests/phase_mobility_scenario0_results.md` (landing report)
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`, `docs/workshop/mobility_and_transfer_allocation.md`, `docs/workshop/mapping_current_guidance.md`, `docs/design_v7_8.md`, `docs/invariants.md`, `docs/worklog.md`

## Acceptance rationale

The packet is metadata/admission only — no allocator, reparenting, routing, economy, owner-overlay,
GPU kernel, or `SimSession` wiring is created. `admit_mobility_scenario0_packet` rejects any packet
that sets `implementation_authorized`, `enabled_by_default`, or `status != ScenarioAdmissionProposed`,
so the landed artifact cannot self-promote. It declares every parameter the v7.9 SCENARIO gate
requires and self-narrows routing to a single adversarial first slice.

## Acceptance checklist audit

| Required declaration / rejection | Packet field / guard | Result |
|---|---|---|
| Theater shape / spatial depth | `theater` (1/3/48, depth 4); depth ≥ 2 enforced | PASS |
| `max_factions_per_cell` | `identity_channels.max_factions_per_cell` = 4 | PASS |
| Routing EML node budget | `routing_eml_node_budget` = 16 (≥ 4×4 enforced) | PASS |
| Fleet density / block size | density 64, block 96, headroom 32; block ≥ density, slab-first | PASS |
| Entity identity boundary | `identity_boundary` SimThing slots vs count columns | PASS |
| SimThing slots vs aggregate count columns | slots {cell,fleet,ship_class_cohort,pop_cohort}; counts {fighter_count,…} | PASS |
| Owner columns + disciplines | faction (flow-pooling) + species/blueprint/tech (down-broadcast); both required | PASS |
| Flow-pooling vs down-broadcast overlay distinction | `MobilityOwnerRelationDiscipline`; both-present check | PASS |
| Hard fixed-point + soft float classes | Band Alpha / Band Beta lists | PASS |
| Prevent hard/soft silent mixing | `hard_and_soft_never_silently_mix` required true | PASS |
| Prevent float structural gates | `float_values_gate_structural_transitions` must be false | PASS |
| Supply/economy scope | `supply_scope` (spatial structure, subsidiarity depth) | PASS |
| Blockade semantics | cut flows + blockade-immune overlays | PASS |
| Routing mode | `NarrowedAdversarialFirstSlice`, identity-is-column | PASS |
| 34k soak profile | `soak.entity_count` == 34_000 enforced; stress mix | PASS |
| Reject owner-entity spatial parent | `MobilityOwnerSpatialParentRejected` | PASS (tested) |
| Reject capture-as-reparenting | `MobilityCaptureAsReparentingRejected` | PASS (tested) |
| Reject semantic/raw WGSL | `SemanticWgslRequestRejected` | PASS (tested) |
| Reject GPU allocator semaphore / nondeterministic atomics | `MobilityGpuAllocatorSemaphoreRejected` | PASS (tested) |
| Reject indirection-before-slab | `MobilityIndirectionBeforeSlabRejected` | PASS (tested) |
| Reject arrival-order replay ordering | `MobilityArrivalOrderReplayOrderingRejected` | PASS (tested) |
| Reject Hybrid Strata silent rebind | `MobilityHybridStrataSilentRebindRejected` | PASS (tested) |
| Reject default-on Resource Flow | `DefaultOnRejected` | PASS (tested) |
| Reject hard-currency through Resource Flow | `MobilityHardCurrencyThroughResourceFlowRejected` | PASS (tested) |
| Reject closed-ladder reopen | `MobilityClosedLadderReopenRejected` | PASS (tested) |
| Keep ClauseThing/L3 parked | `ClauseThingRuntimeRequestParked` | PASS (tested) |
| Keep A/B/C, FrontierV2-5, ACT/EVENT/OBS/PIPE closed/parked | no reopen path; closed-ladder guard | PASS |

## Constitution / workshop / SEAD alignment

- **v7.8 constitution:** guardrails at designer/spec admission; `simthing-sim` untouched; opt-in/default-off; no CPU planner/urgency/commitment emission (rejected); exact authority artifact-backed; no `SimSession` wiring. PASS.
- **mobility workshop:** owner-entities session descendants not spatial parents; capture = column flip not reparenting; identity = D=2 masked reduction (column not tree); Hybrid Strata local channels (`local_identity_channels == max_factions_per_cell`); hard Band Alpha precedes soft Band Beta; cohorts are SimThings with count columns; down-broadcast overlays do not become flow-pooling arena columns. PASS.
- **SEAD:** AI stays SimThing/GPU-resident; decisions are threshold crossings; no CPU planner/urgency/commitment emission; structural path stays Threshold+EmitEvent → BoundaryRequest; movement writes only the mover's own columns. No contradiction introduced (scenario/admission only). PASS.
- **tracked goals:** no runtime in scenario PR; no code-path widening beyond simthing-spec admission; no invariant change; no default-on; no atlas/runtime/nested/hard-currency/ClauseThing reopen. PASS.

## Commands

```bash
cargo test -p simthing-spec --test mobility_scenario0_admission                                   # 13 passed
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation  # 15 + 10 + 25 passed
cargo check --workspace                                                                           # Finished — ok
```

## Next gate

**`MOBILITY-AUDIT-0 / owner_band_budget_audit`** — proposed early audit, no runtime implementation:
determine whether the interleaved circulation families (modifier-down, economy-up/down, research-up,
thresholds, Band Alpha, Band Beta) fit `max_orderband_depth` at spatial depth 4, or whether the
scenario depth must narrow / a separate OrderBand-depth scenario is required. **Not started in this
PR.**

## Posture attestation

No runtime implementation; no GPU kernels; no allocator/reparenting/routing/economy/owner-overlay
code; no production `SimSession` wiring; no default-on flags; no semantic/raw WGSL; no `simthing-sim`
semantic awareness; no CPU planner/urgency/commitment emission; no Resource Flow default-on; no
hard-currency through Resource Flow; no invariant changes. v7.8 M/E/T closure (A-0/B-0/C-2),
AO-WGSL-0 default-off posture, ClauseThing/L3 parked, FrontierV2-5 rejected, ACT/EVENT/OBS/PIPE no
reopen — all unchanged. ALLOC/REENROLL/IDROUTE/ECON/OWNER remain proposed/parked.

## Final verdict

**ACCEPT MOBILITY-SCENARIO-0 — accepted as the named v7.9 mobility/transfer product scenario. Opens
only MOBILITY-AUDIT-0 / owner_band_budget_audit. No implementation gate opened by implication.**
