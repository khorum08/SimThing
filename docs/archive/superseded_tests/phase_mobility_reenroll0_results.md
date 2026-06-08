# MOBILITY-REENROLL-0 — bilateral arena re-enrollment substrate results

Date: 2026-06-01

## Verdict

**PASS**

MOBILITY-REENROLL-0 implements the authorized bilateral arena re-enrollment substrate on top of
MOBILITY-ALLOC-0. The implementation is confined to `simthing-spec` designer-admission/substrate
modeling and tests; it does not wire production runtime behavior.

## Files Touched

- `crates/simthing-spec/src/designer_admission/mobility_reenroll0.rs`
- `crates/simthing-spec/src/designer_admission/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/mobility_reenroll0_substrate.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_reenroll0_results.md`

## Implemented Substrate Scope

- Spatial movement as a bilateral arena operation: `Departure(origin_key, entity_id)` +
  `Arrival(destination_key, same entity_id)` in one MOBILITY-ALLOC-0 boundary accounting pass.
- Bulk per-cell validation matching ALLOC semantics (departures before arrivals per parent/key block).
- Atomic commit-or-reject: pre-validate entire batch; on failure return snapshot unchanged with no
  generation bumps; on success commit via ALLOC and bump origin/destination registry generations.
- Canonical move ordering by `(entity_id, origin, destination)`; arrival order not replay-significant.
- Stable `entity_id` preserved across origin → destination transfer.
- Destination slot assignment delegated to MOBILITY-ALLOC-0 deterministic lowest-free slab behavior.
- Flat-star cell arenas only (`origin.parent_id == destination.parent_id`).
- CPU/driver-side accounting with deterministic CPU/GPU-proxy layout checksum (delegates to ALLOC-0).

## Explicit Non-Goals Preserved

No nested arena reparenting, capture-as-reparenting, owner-entity as spatial parent, IDROUTE, ECON,
OWNER, route/economy/owner-overlay runtime, production `SimSession` wiring, default-on behavior,
semantic/raw WGSL, GPU semaphore/atomics, nondeterministic allocator, live compaction, default-on
Resource Flow, hard-currency through Resource Flow, CPU planner, CPU urgency computation, CPU
commitment emission, or invariant edits.

## Test Battery

| Test | Result |
| --- | --- |
| `reenroll_bilateral_origin_destination_accounting` | PASS |
| `reenroll_atomic_or_reject_no_partial_mutation` | PASS |
| `reenroll_preserves_entity_identity` | PASS |
| `reenroll_uses_alloc0_destination_assignment` | PASS |
| `reenroll_no_live_slice_compaction` | PASS |
| `reenroll_arrival_order_independent` | PASS |
| `reenroll_cpu_gpu_parity_layout` | PASS |
| `reenroll_rejects_capture_as_reparenting` | PASS |
| `reenroll_rejects_owner_as_spatial_parent` | PASS |
| `reenroll_rejects_nested_arena_reparenting_without_gate` | PASS |
| `reenroll_keeps_idroute_econ_owner_parked` | PASS |
| `reenroll_does_not_authorize_production_simsession_wiring` | PASS |
| `reenroll_does_not_enable_default_on_behavior` | PASS |
| `reenroll_burst_transfer_O_blocks` | PASS — 250 moves, 2 touched blocks, 500 boundary events |
| `reenroll_origin_destination_high_water_bound` | PASS |
| `reenroll_scale_soak_34k_movement_churn` | PASS — 34k ring-rotation moves across 48 cells |

## Performance Bars

| Bar | Result |
| --- | --- |
| `reenroll_burst_transfer_O_blocks` | PASS — O(affected blocks) via ALLOC bulk grouping (2 blocks for 250 moves) |
| `reenroll_origin_destination_high_water_bound` | PASS — pending buffer bounded; capacity rejection is atomic |
| `reenroll_scale_soak_34k_movement_churn` | PASS — 68k boundary events, 48 bulk groups, 34k committed moves |

## Commands

```bash
cargo test -p simthing-spec --test mobility_scenario0_admission                # 13 passed
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget           # 8 passed
cargo test -p simthing-spec --test mobility_alloc0_substrate                   # 15 passed
cargo test -p simthing-spec --test mobility_reenroll0_substrate                # 16 passed
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation  # 25 + 10 + 15 passed
cargo check --workspace                                                        # Finished — ok
```

## Posture Attestation

MOBILITY-REENROLL-0 is green only for bilateral arena re-enrollment substrate. IDROUTE/ECON/OWNER
remain proposed/parked and require separate opening. No production runtime integration, no GPU
kernels, no default-on flags, no semantic/raw WGSL, no `simthing-sim` semantic awareness, no CPU
planner/urgency/commitment emission, no Resource Flow default-on, no hard-currency through Resource
Flow, no invariant changes. v7.8 M/E/T closure (A-0/B-0/C-2), AO-WGSL-0 default-off, ClauseThing/L3
parked, FrontierV2-5 rejected, ACT/EVENT/OBS/PIPE no reopen — all unchanged.
