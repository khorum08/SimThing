# MOBILITY-ALLOC-0 - deterministic slab allocator substrate results

Date: 2026-06-01

## Verdict

**PASS**

MOBILITY-ALLOC-0 implements the authorized deterministic slab + bulk-accounting allocator substrate. The implementation is confined to `simthing-spec` designer-admission/substrate modeling and tests; it does not wire production runtime behavior.

## Files Touched

- `crates/simthing-spec/src/designer_admission/mobility_alloc0.rs`
- `crates/simthing-spec/src/designer_admission/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/mobility_alloc0_substrate.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`
- `docs/tests/phase_mobility_alloc0_results.md`

## Implemented Substrate Scope

- Deterministic per-parent/key slab allocation.
- Preformatted contiguous block per parent/key.
- Reserved-headroom/lowest-free slice claims for arrivals.
- Whole-block reclaim only on parent/key removal after the block is empty.
- No live-slice compaction.
- Canonical parent/key and entity ordering; arrival order is ignored for replay-significant assignment.
- Two-stage boundary accounting: group events by parent/key, then apply deterministic departures/reclaims/arrivals.
- CPU/driver-side accounting model only.
- GPU-consumable parity proxy via deterministic flat layout checksum.

## Test Battery

| Test | Result |
| --- | --- |
| `alloc_no_live_slice_moves` | PASS |
| `alloc_bulk_accounting_determinism` | PASS |
| `alloc_cpu_gpu_parity` | PASS |
| `alloc_burst_absorption_O_blocks` | PASS - deterministic proxy records one bulk group/touched block for 500 same-block arrivals |
| `alloc_high_water_bound` | PASS |
| `alloc_collapse_fragmentation_ratio` | PASS |
| `alloc_scale_soak_34k` | PASS - 34,000 arrivals across 48 preformatted blocks |
| `alloc_rejects_live_compaction` | PASS |
| `alloc_rejects_arrival_order_replay_significance` | PASS |
| `alloc_rejects_gpu_semaphore_or_atomic_path` | PASS |
| `alloc_rejects_indirection_list_slotrange` | PASS |
| `alloc_keeps_reenroll_idroute_econ_owner_parked` | PASS |
| `alloc_does_not_authorize_production_simsession_wiring` | PASS |
| `alloc_does_not_enable_default_on_behavior` | PASS |
| `alloc_whole_block_reclaim_requires_parent_removal_and_empty_block` | PASS |

## Explicit Non-Goals Preserved

No REENROLL, IDROUTE, ECON, OWNER, capture/reparenting implementation, route/economy/owner-overlay runtime, production `SimSession` wiring, default-on behavior, semantic/raw WGSL, GPU semaphore/atomics, default-on Resource Flow, hard-currency through Resource Flow, CPU planner, CPU urgency computation, CPU commitment emission, or invariant edits.

## Commands

```bash
cargo test -p simthing-spec --test mobility_scenario0_admission
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget
cargo test -p simthing-spec --test mobility_alloc0_substrate
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation
cargo check --workspace
```

## Posture Attestation

MOBILITY-ALLOC-0 is green only for deterministic slab + bulk-accounting allocator substrate. REENROLL/IDROUTE/ECON/OWNER remain proposed/parked and require separate opening. The implementation is semantic-free, admission/substrate-scoped, default-off by construction, and production-runtime-free.
