# MOBILITY-OWNER-0-R1 - isolated owner-overlay down-broadcast hardening results

Date: 2026-06-01

## Verdict

**PASS / substrate-only hardening.**

R1 adds explicit coverage for sparse ownership completeness on the landed OWNER-0 substrate. The owner-overlay down-broadcast now has a named regression test proving that a latched owner modifier reaches every local record with the matching owner column, including a single isolated owned SimThing in an otherwise empty/non-dense cell.

## Evidence

- Added `owner_down_broadcast_reaches_every_owned_including_isolated`.
- Matching is by owner-column presence, not by cell membership, resource-flow enrollment, cell density, or owner-as-spatial-parent.
- An unrelated SimThing without the matching owner column receives no overlay.
- The overlay remains an OWNER latched modifier; this PR adds no ECON/resource-flow transfer.
- Down-broadcast still spawns no arena or aggregation columns.
- Cost decomposition is explicit in assertions:
  - dirty owner/modifier tick may touch all owned records: `modifier_dispersal_count == O(owned)`;
  - steady state with no owner-set/modifier-set change has zero redisperse cost: `modifier_dispersal_count == 0`, with deterministic `dirtyonly_noop_count`.

## Posture

No production runtime integration, production `SimSession` wiring, Resource Flow runtime, owner-entity spatial parent, capture-as-reparenting, nested arena reparenting, semantic/raw WGSL, default-on behavior, CPU planner/urgency/commitment, Hybrid-Strata/faction-index scaling, or invariant edit.

## Commands

```bash
cargo test -p simthing-spec --test mobility_owner0_substrate
cargo test -p simthing-spec --test mobility_scenario0_admission
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget
cargo test -p simthing-spec --test mobility_alloc0_substrate
cargo test -p simthing-spec --test mobility_reenroll0_substrate
cargo test -p simthing-spec --test mobility_idroute0_substrate
cargo test -p simthing-spec --test mobility_econ0_substrate
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation
cargo check --workspace
```
