# MOBILITY-OWNER-0 Results

Date: 2026-06-01

Verdict: PASS / substrate only.

## Scope

Implemented the authorized owner-relations + latched modifier overlay substrate in `simthing-spec`.
The slice models owner relations as explicit owner columns/overlays, applies latched modifiers by
canonical owner-column matching, treats capture as an owner-column flip, reports deterministic
generation/resync for owner-column changes, and fissions partial owner changes into homogeneous
cohort records.

This PR does not authorize production runtime integration, production gameplay integration,
production `SimSession` wiring, Resource Flow runtime, hard-currency-through-Resource-Flow,
Hybrid-Strata channel binding, generational faction-index slab, faction-index scaling, semantic/raw
WGSL, designer-authored shader code, default-on behavior, CPU planner, CPU urgency, CPU commitment
emission, owner-as-spatial-parent, capture-as-reparenting, or nested arena reparenting.

## Files

- `crates/simthing-spec/src/designer_admission/mobility_owner0.rs`
- `crates/simthing-spec/tests/mobility_owner0_substrate.rs`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mapping_current_guidance.md`

## Required Battery

Substrate floor:

- `owner_column_overlay_applies_deterministically` PASS
- `owner_capture_is_column_flip_not_reparenting` PASS
- `owner_latched_modifier_overlay_persists` PASS
- `owner_blockade_immune_modifier_stays_latched` PASS
- `owner_down_broadcast_does_not_spawn_arena_columns` PASS
- `owner_generation_resync_on_owner_column_change` PASS
- `owner_cpu_gpu_parity_layout` PASS
- `owner_cohort_homogeneity_via_fission` PASS

Guardrails:

- `owner_rejects_owner_as_spatial_parent` PASS
- `owner_rejects_capture_as_reparenting` PASS
- `owner_rejects_nested_arena_reparenting` PASS
- `owner_rejects_default_on_resource_flow` PASS
- `owner_rejects_hard_currency_through_resource_flow` PASS
- `owner_rejects_production_simsession_wiring` PASS
- `owner_rejects_semantic_or_raw_wgsl` PASS
- `owner_rejects_cpu_planner_urgency_commitment` PASS
- `owner_rejects_hybrid_strata_or_faction_index_scaling_layer` PASS
- `owner_keeps_production_runtime_integration_parked` PASS

Performance bars:

- `owner_overlay_multi_cell_scale` PASS
- `owner_concentration_one_owner` PASS
- `owner_dirtyonly_amortized` PASS
- `owner_band_budget_audit` PASS
- `owner_scale_soak_34k` PASS

## Commands

```bash
cargo test -p simthing-spec --test mobility_scenario0_admission
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget
cargo test -p simthing-spec --test mobility_alloc0_substrate
cargo test -p simthing-spec --test mobility_reenroll0_substrate
cargo test -p simthing-spec --test mobility_idroute0_substrate
cargo test -p simthing-spec --test mobility_econ0_substrate
cargo test -p simthing-spec --test mobility_owner0_substrate
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation
cargo check --workspace
```

## Posture

MOBILITY-OWNER-0 is green as a metadata/testable owner-overlay substrate only. With OWNER-0 green,
the v7.9 mobility/transfer substrate ladder is complete at substrate level. Production runtime
integration remains a separate, currently closed gate. Hybrid-Strata/faction-index scaling remains a
later ECON slice. No invariant edits were made.
