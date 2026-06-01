# MOBILITY-ALLOC-0-OPEN-0 - ALLOC opening review

Date: 2026-06-01

## Verdict

**OPEN**

MOBILITY-ALLOC-0 may open as the next v7.9 implementation ladder, limited to deterministic slab + bulk-accounting allocator substrate work. This review authorizes ALLOC only; it does not implement the allocator.

## Reviewed Files

- `docs/workshop/phase_m_gating_and_doc_policy.md`
- `docs/invariants.md`
- `docs/design_v7_8.md`
- `docs/design_v7_8_production_track.md`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mobility_and_transfer_allocation.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/sead_self_ai_track.md`
- `docs/tests/phase_mobility_scenario0_results.md`
- `docs/tests/phase_mobility_scenario0_acceptance_review_results.md`
- `docs/tests/phase_mobility_owner_band_budget_audit_results.md`
- `crates/simthing-spec/src/designer_admission/mobility_scenario0.rs`
- `crates/simthing-spec/src/designer_admission/mobility_audit0.rs`
- `crates/simthing-spec/tests/mobility_scenario0_admission.rs`
- `crates/simthing-spec/tests/mobility_audit0_owner_band_budget.rs`
- `docs/worklog.md`

## Accepted Prerequisites

| Prerequisite | Result |
| --- | --- |
| MOBILITY-SCENARIO-0 accepted | PASS |
| MOBILITY-AUDIT-0 passed | PASS - 13 required OrderBands under ceiling 16 |
| A-0 static nested Resource Flow baseline compatible | PASS - static nested first slice remains accepted and closed |
| v7.8 M/E/T closeout preserved | PASS |
| ALLOC isolated from REENROLL/IDROUTE/ECON/OWNER | PASS |
| Substrate-level testing possible without production runtime wiring | PASS |

## Authorized ALLOC Scope

- Deterministic per-parent/key slab allocation.
- Preformatted contiguous block per parent/key.
- Arrivals claim slices inside reserved headroom.
- Whole-block reclaim only on parent/key removal.
- No live-slice compaction.
- Lowest-free-first deterministic allocation.
- Net births/deaths handled in one boundary accounting pass.
- CPU/driver accounting only.
- Post-allocation layouts remain GPU-consumable and parity-testable.

## Explicit Non-Goals

- No GPU-side semaphore or CUDA-style atomics.
- No nondeterministic allocator.
- No live compaction.
- No indirection-list SlotRange.
- No semantic/raw WGSL.
- No owner/economy semantics.
- No reparenting implementation.
- No route/economy/owner-overlay runtime.
- No production `SimSession` wiring.
- No default-on behavior.
- No default-on Resource Flow or hard-currency through Resource Flow.
- No CPU planner, CPU urgency computation, or CPU commitment emission.

## Authorized Test Battery

Substrate floor:

- `alloc_no_live_slice_moves`
- `alloc_bulk_accounting_determinism`
- `alloc_cpu_gpu_parity`

Performance bars:

- `alloc_burst_absorption_O_blocks`
- `alloc_high_water_bound`
- `alloc_collapse_fragmentation_ratio`
- `alloc_scale_soak_34k`

## Posture Attestation

Owner-entities remain non-spatial; capture remains an owner-column flip, not reparenting; arrival order remains non-authoritative for replay ordering; live-slot compaction stays forbidden; GPU semaphore/nondeterministic atomics stay rejected; semantic/raw WGSL stays rejected; `simthing-sim` remains semantic-free; SEAD decisions stay GPU-resident threshold/event outputs rather than CPU planning; REENROLL/IDROUTE/ECON/OWNER remain proposed/parked. No invariant changes.

## Commands

```bash
cargo test -p simthing-spec --test mobility_scenario0_admission
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation
cargo check --workspace
```
