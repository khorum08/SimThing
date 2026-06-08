# MOBILITY-SCENARIO-0 — Scenario Admission Packet Results

## Verdict

**PASS — MOBILITY-SCENARIO-0 scenario/admission metadata landed with guardrail rejection coverage. Awaiting design-authority/product acceptance; no implementation gate opened.**

## Files touched

| File | Change |
|---|---|
| `crates/simthing-spec/src/designer_admission/mobility_scenario0.rs` | Added typed v7.9 mobility scenario packet, default bounded packet, admission summary, and guardrail validation. |
| `crates/simthing-spec/src/designer_admission/diagnostic.rs` | Added MOBILITY-SCENARIO-0 diagnostic vocabulary for scenario-layer rejections. |
| `crates/simthing-spec/src/designer_admission/mod.rs`, `crates/simthing-spec/src/lib.rs`, `crates/simthing-spec/src/ron.rs` | Exported packet/admission APIs and RON roundtrip helpers. |
| `crates/simthing-spec/tests/mobility_scenario0_admission.rs` | Added scenario/admission tests. |
| `docs/design_v7_9_mobility_transfer_allocation_production_track.md` | Updated SCENARIO row and compact track row. |
| `docs/workshop/mapping_current_guidance.md` | Added compact status row. |
| `docs/worklog.md` | Added one top entry. |
| `docs/tests/phase_mobility_scenario0_results.md` | This report. |

## Scenario Parameters Admitted

| Parameter | MOBILITY-SCENARIO-0 bound |
|---|---|
| Theater shape | 1 sector, 3 systems, 48 cells, spatial depth 4; single-theater multi-cell first slice. |
| `max_factions_per_cell` | 4 local identity channels; routing EML node budget 16; expected first-slice peak 3. |
| Fleet/block density | 64 moving entities per cell; block size 96; 32 reserved headroom; overflow visibly rejects/narrows. |
| Identity boundary | Cells, fleets, ship-class cohorts, and pop cohorts are SimThing slots; fighters/HP pools/population are count columns. |
| Owner columns | `faction_owner` flow-pooling; `species_owner`, `blueprint_owner`, `tech_owner` down-broadcast overlays. |
| Quantity classes | Band Alpha hard fixed-point: hard currency, munitions, supply balance tests. Band Beta soft float: damage, repair, morale. No silent mixed pass. |
| Supply/economy scope | First slice keeps sector/cell edges as spatial structure; subsidiarity balances at the lowest node with residual escalation. |
| Blockade semantics | Cuts per-tick supply/munitions resupply; species/tech/policy modifiers are latched and blockade-immune. |
| Routing mode | Narrowed adversarial first slice; identity is a column, never tree structure. |
| 34k soak | 34,000 entities with churn, movement, capture/unlock cadence, and ALLOC/REENROLL/IDROUTE/ECON/OWNER stress mix recorded. |

## Rejection Cases Covered

- Owner-entities as spatial parents.
- Capture modeled as reparenting.
- Semantic/raw WGSL.
- GPU allocator semaphore / nondeterministic allocator.
- Indirection buffer before slab/block path.
- Arrival-order replay-significant ordering.
- Silent Hybrid Strata rebind.
- Hard/soft mixed pass.
- Float structural gate.
- `max_factions_per_cell` and routing EML node budget overrun.
- Default-on Resource Flow.
- Hard currency routed through Resource Flow.
- ClauseThing/L3 or closed ladder reopen.

## Commands

```bash
cargo test -p simthing-spec --test mobility_scenario0_admission
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation
cargo check --workspace
```

All commands passed. `cargo check` reports pre-existing warnings in `simthing-core` and `simthing-driver`.

## Posture Attestation

No runtime implementation, no allocator/reparenting/routing/economy/owner-overlay implementation, no GPU kernels, no default-on behavior, no production `SimSession` wiring, no invariant changes, no ClauseThing/L3, no A/B/C reopen, no FrontierV2-5 or ACT/EVENT/OBS/PIPE reopen. v7.9 remains parked until separate design-authority/product acceptance opens a later gate.
