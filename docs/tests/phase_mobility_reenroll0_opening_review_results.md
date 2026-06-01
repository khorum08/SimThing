# MOBILITY-REENROLL-0-OPEN-0 — REENROLL opening review

Date: 2026-06-01
Reviewer: Design authority (Opus 4.8 lane) + product. Gate-opening / production-track authorization
only — not implementer self-acceptance.

## Verdict

**OPEN** (Option A).

`MOBILITY-REENROLL-0` may open as the next v7.9 implementation ladder, limited to the **bilateral
arena re-enrollment substrate floor + performance bars** built on the MOBILITY-ALLOC-0 deterministic
slab + bulk-accounting substrate. The authorized scope is inherently first-slice-narrowed by the
track §7 definition (flat-star cell arenas only, spatial movement only, no nested arena reparenting,
no capture-as-reparenting, allocator-substrate-level, no runtime). This review **authorizes** REENROLL
only; it does **not** implement it, and it opens **no** other ladder.

## Reviewed files

- `docs/workshop/phase_m_gating_and_doc_policy.md`
- `docs/invariants.md` (targeted scan — no reparenting/spatial-parent term; no conflict)
- `docs/design_v7_8.md` (§6 forward territory)
- `docs/design_v7_8_production_track.md`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mobility_and_transfer_allocation.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/sead_self_ai_track.md`
- `docs/tests/phase_mobility_scenario0_results.md`, `phase_mobility_scenario0_acceptance_review_results.md`
- `docs/tests/phase_mobility_owner_band_budget_audit_results.md`
- `docs/tests/phase_mobility_alloc0_opening_review_results.md`, `phase_mobility_alloc0_results.md`
- `crates/simthing-spec/src/designer_admission/mobility_scenario0.rs`, `mobility_audit0.rs`, `mobility_alloc0.rs`
- `crates/simthing-spec/tests/mobility_scenario0_admission.rs`, `mobility_audit0_owner_band_budget.rs`, `mobility_alloc0_substrate.rs`
- `docs/worklog.md`

## Accepted prerequisites (verified against the tree, not only the reports)

| Prerequisite | Result |
| --- | --- |
| MOBILITY-SCENARIO-0 accepted (MOBILITY-SCENARIO-0-ACCEPT-0) | PASS — packet forces `ScenarioAdmissionProposed`, cannot self-promote; `mobility_scenario0_admission` **13 passed** |
| MOBILITY-AUDIT-0 passed | PASS — 13 required OrderBands under ceiling 16 (slack 3); `mobility_audit0_owner_band_budget` **8 passed** |
| MOBILITY-ALLOC-0 passed; usable as REENROLL allocator substrate | PASS — `mobility_alloc0_substrate` **15 passed**; substrate exposes `Arrival`/`Departure`/`ParentRemoved` boundary events keyed by `(parent_id,key_id)` with preserved `entity_id`, deterministic lowest-free assignment, whole-block reclaim, no compaction |
| v7.8 M/E/T closeout preserved | PASS — `c2_atlas_admission_relaxation` 15, `clause_spec0_frontier_v2_admission` 25, `v7_8_met_consumer_scenarios` 10 all green; `cargo check --workspace` clean (pre-existing `simthing-driver` unused-import warning only) |
| REENROLL isolable from IDROUTE/ECON/OWNER | PASS — REENROLL is bilateral re-enrollment bookkeeping (deregister origin + register destination) on the ALLOC substrate; it adds no routing, economy, or owner-overlay semantics |
| Testable at substrate level without production runtime wiring | PASS — ALLOC-0 set the precedent (simthing-spec designer-admission/substrate module + tests, no `SimSession`) |

## Authorized REENROLL scope (substrate only)

- Spatial reparenting as a **bilateral** arena operation: deregister from origin cell arena +
  register into destination cell arena, in one boundary accounting pass.
- Destination assignment uses the MOBILITY-ALLOC-0 deterministic slab/bulk substrate
  (`Departure(origin_key)` + `Arrival(dest_key)` with the same `entity_id`).
- Stable entity identity preserved across origin→destination transfer.
- No live-slice compaction (inherited ALLOC floor).
- Boundary event ordering canonicalized — arrival order is not replay-significant.
- **Commit atomically or reject with no partial mutation**; origin/destination registry generation
  bumps only on successful commit.
- Flat-star cell arenas first.
- Movement kept distinct from ownership/capture semantics.
- Produces GPU-consumable, parity-testable arena layouts after re-enrollment (CPU/driver accounting
  + GPU-parity proxy, as in ALLOC-0). No semantic GPU kernel added.

## Explicit non-goals (unchanged from the handoff; enforced at designer/scenario admission)

No nested arena reparenting; no capture-as-reparenting; no owner-entity as spatial parent; no IDROUTE;
no ECON; no OWNER; no route/economy/owner-overlay runtime; no production `SimSession` wiring; no
semantic/raw WGSL; no GPU semaphore/atomics; no nondeterministic allocator; no live compaction; no
default-on behavior; no default-on Resource Flow; no hard-currency through Resource Flow; no CPU
planner; no CPU urgency computation; no CPU commitment emission.

## Opening checks

| Check | Result |
| --- | --- |
| MOBILITY-SCENARIO-0 accepted | PASS |
| MOBILITY-AUDIT-0 passed | PASS |
| MOBILITY-ALLOC-0 passed and usable as substrate | PASS |
| REENROLL isolable from IDROUTE/ECON/OWNER | PASS |
| Testable at substrate level without runtime wiring | PASS |
| Owner-entity never spatial parent | PASS (preserved; REENROLL is spatial movement only) |
| Capture remains owner-column flip, not reparenting | PASS (explicit non-goal; scenario0 rejects capture-as-reparenting) |
| Movement writes only the mover's own authoritative columns | PASS (SEAD principle preserved) |
| Arrival order not replay-significant | PASS (ALLOC substrate floor inherited) |
| No live-slot compaction | PASS (ALLOC substrate floor inherited) |
| No GPU semaphore / nondeterministic atomics | PASS (rejected) |
| No semantic/raw WGSL | PASS (rejected at designer admission) |
| `simthing-sim` remains semantic-free | PASS |
| No CPU planner / urgency / commitment emission | PASS |
| Expected REENROLL test battery sufficient before implementation | PASS — see below (authorized battery is the complete substrate-correctness + guardrail + perf set; one design-authority addition for atomicity) |

## Authorized REENROLL test battery (to implement in a later PR — none green yet)

**Substrate floor**

- `reenroll_bilateral_origin_destination_accounting`
- `reenroll_atomic_or_reject_no_partial_mutation` *(design-authority addition — the load-bearing
  bilateral property per track §7; commit both sides or neither, generation bump only on commit)*
- `reenroll_preserves_entity_identity`
- `reenroll_uses_alloc0_destination_assignment`
- `reenroll_no_live_slice_compaction`
- `reenroll_arrival_order_independent`
- `reenroll_cpu_gpu_parity_layout`

**Guardrails**

- `reenroll_rejects_capture_as_reparenting`
- `reenroll_rejects_owner_as_spatial_parent`
- `reenroll_rejects_nested_arena_reparenting_without_gate`
- `reenroll_keeps_idroute_econ_owner_parked`
- `reenroll_does_not_authorize_production_simsession_wiring`
- `reenroll_does_not_enable_default_on_behavior`

**Performance bars**

- `reenroll_burst_transfer_O_blocks`
- `reenroll_origin_destination_high_water_bound`
- `reenroll_scale_soak_34k_movement_churn`

This battery is **authorized, not implemented**; no REENROLL test is marked green in this PR.

## Commands

```bash
cargo test -p simthing-spec --test mobility_scenario0_admission                # 13 passed
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget           # 8 passed
cargo test -p simthing-spec --test mobility_alloc0_substrate                   # 15 passed
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation  # 25 + 10 + 15 passed
cargo check --workspace                                                        # Finished (pre-existing simthing-driver unused-import warning only)
```

## Posture attestation

Opening review only — no REENROLL implementation, no allocator/reparenting/routing/economy/owner
code, no GPU kernels, no production `SimSession` wiring, no default-on flags, no semantic/raw WGSL,
no `simthing-sim` semantic awareness, no CPU planner/urgency/commitment emission, no Resource Flow
default-on, no hard-currency through Resource Flow, no invariant changes. Owner-entities remain
non-spatial; capture remains an owner-column flip; arrival order remains non-authoritative; live-slot
compaction stays forbidden; GPU semaphore/nondeterministic atomics stay rejected; SEAD decisions stay
GPU-resident threshold/event outputs. v7.8 M/E/T closure (A-0/B-0/C-2), AO-WGSL-0 default-off,
ClauseThing/L3 parked, FrontierV2-5 rejected, ACT/EVENT/OBS/PIPE no reopen — all unchanged.
IDROUTE/ECON/OWNER remain proposed/parked.

## Next gate

**`MOBILITY-REENROLL-0`** — implement the authorized substrate floor + performance bars above (later
PR, substrate-only). IDROUTE remains the subsequent candidate (entry gate: ALLOC + REENROLL green),
still proposed/parked.
