# MOBILITY-ECON-0 Results

Date: 2026-06-01

Verdict: PASS / substrate only.

## Scope

Implemented the authorized session-clearinghouse + subsidiarity economy substrate in `simthing-spec`.
The slice accepts local cell records from the mobility ladder, aggregates by `(session_id, resource_id)`,
balances hard fixed-point Band Alpha exactly at the clearinghouse boundary, then emits deterministic
down-disburse records where soft Band Beta reads finalized Alpha.

This PR does not authorize OWNER, Hybrid-Strata channel partitioning, generational faction-index slab
scaling, Resource Flow runtime routing, production `SimSession` wiring, semantic/raw WGSL, default-on
behavior, CPU planner urgency/commitment emission, owner-as-spatial-parent, or capture-as-reparenting.

## Files

- `crates/simthing-spec/src/designer_admission/mobility_econ0.rs`
- `crates/simthing-spec/tests/mobility_econ0_substrate.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`

## Required Battery

Substrate floor:

- `econ_session_clearinghouse_aggregates_local_cells` PASS
- `econ_subsidiarity_balance_conservation` PASS
- `econ_hard_band_alpha_before_soft_band_beta` PASS
- `econ_rejects_hard_soft_silent_mix` PASS
- `econ_deterministic_up_down_disburse` PASS
- `econ_cpu_gpu_parity_layout` PASS

Guardrails:

- `econ_rejects_owner_overlay_runtime` PASS
- `econ_keeps_owner_parked` PASS
- `econ_rejects_default_on_resource_flow` PASS
- `econ_rejects_hard_currency_through_resource_flow` PASS
- `econ_rejects_float_structural_gate` PASS
- `econ_rejects_production_simsession_wiring` PASS
- `econ_rejects_semantic_or_raw_wgsl` PASS
- `econ_rejects_cpu_planner_urgency_commitment` PASS
- `econ_rejects_owner_as_spatial_parent` PASS
- `econ_rejects_capture_as_reparenting` PASS
- `econ_rejects_hybrid_strata_or_faction_index_scaling_layer` PASS

Performance bars:

- `econ_multi_cell_clearinghouse_scale` PASS
- `econ_concentration_one_session` PASS
- `econ_scale_soak_34k` PASS

## Commands

```bash
cargo test -p simthing-spec --test mobility_scenario0_admission
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget
cargo test -p simthing-spec --test mobility_alloc0_substrate
cargo test -p simthing-spec --test mobility_reenroll0_substrate
cargo test -p simthing-spec --test mobility_idroute0_substrate
cargo test -p simthing-spec --test mobility_econ0_substrate
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation
cargo check --workspace
```

## Posture

MOBILITY-ECON-0 is green as a metadata/testable clearinghouse substrate only. OWNER remains parked.
Hybrid-Strata/faction-index scaling remains a later ECON slice. No production runtime integration gate
is open.
