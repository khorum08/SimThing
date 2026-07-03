# Test Promotion Wave Plan

This is the rustification backlog for KEEP rows whose survival is tied to a named promotion target. A follow-on `TEST-PROMOTE-<boundary>-0` rung must land the type/admission/scan boundary and retire redundant tests in the same PR.

total promotion-target rows: 18

| target_boundary | owning_crate | row_count | example_rows | recommended_promotion_rung | retirement_action |
|---|---|---:|---|---|---|
| `promotion-target:test-pare-tier2-cpu-admission-collapse-clausething-missing-or-unknown-reference-admission` | simthing-clausething | 1 | `crates/simthing-clausething/tests/bh3_authoring_parse.rs::bh3_authoring_rejects_missing_u_sat` | `TEST-PROMOTE-TEST-PARE-TIER2-CPU-ADMISSION-COLLAPSE-CLAUSETHING-MISSING-OR-UNKNOWN-REFERENCE-ADMISSION-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-pare-tier2-cpu-admission-collapse-clausething-admission-hard-error` | simthing-clausething | 1 | `crates/simthing-clausething/tests/ct_0c_expansion.rs::recursive_inline_script_is_rejected` | `TEST-PROMOTE-TEST-PARE-TIER2-CPU-ADMISSION-COLLAPSE-CLAUSETHING-ADMISSION-HARD-ERROR-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-pare-tier2-cpu-admission-collapse-clausething-unsupported-vocabulary-admission` | simthing-clausething | 1 | `crates/simthing-clausething/tests/ct_1a_entity.rs::unsupported_entity_field_is_hard_error` | `TEST-PROMOTE-TEST-PARE-TIER2-CPU-ADMISSION-COLLAPSE-CLAUSETHING-UNSUPPORTED-VOCABULARY-ADMISSION-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-pare-tier2-cpu-admission-collapse-clausething-parser-span-admission` | simthing-clausething | 1 | `crates/simthing-clausething/tests/bh3_authoring_parse.rs::bh3_authoring_rejects_invalid_chi_literal` | `TEST-PROMOTE-TEST-PARE-TIER2-CPU-ADMISSION-COLLAPSE-CLAUSETHING-PARSER-SPAN-ADMISSION-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-pare-tier2-cpu-admission-collapse-clausething-duplicate-id-admission` | simthing-clausething | 1 | `crates/simthing-clausething/tests/ct_scenario_container.rs::duplicate_location_ids_are_rejected` | `TEST-PROMOTE-TEST-PARE-TIER2-CPU-ADMISSION-COLLAPSE-CLAUSETHING-DUPLICATE-ID-ADMISSION-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-pare-tier2-cpu-admission-collapse-clausething-finite-number-admission` | simthing-clausething | 1 | `crates/simthing-clausething/tests/ct_scenario_container.rs::scenario_commitment_non_finite_threshold_is_rejected` | `TEST-PROMOTE-TEST-PARE-TIER2-CPU-ADMISSION-COLLAPSE-CLAUSETHING-FINITE-NUMBER-ADMISSION-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-pare-tier2-cpu-admission-collapse-clausething-field-payload-admission` | simthing-clausething | 1 | `crates/simthing-clausething/tests/ct_scenario_container.rs::scenario_second_field_operator_is_rejected` | `TEST-PROMOTE-TEST-PARE-TIER2-CPU-ADMISSION-COLLAPSE-CLAUSETHING-FIELD-PAYLOAD-ADMISSION-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-pare-tier2-cpu-admission-collapse-clausething-topology-admission` | simthing-clausething | 1 | `crates/simthing-clausething/tests/mapgen_links.rs::self_link_is_rejected` | `TEST-PROMOTE-TEST-PARE-TIER2-CPU-ADMISSION-COLLAPSE-CLAUSETHING-TOPOLOGY-ADMISSION-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-pare-spec-b-t2-simthing-spec-field-payload-admission` | simthing-spec | 1 | `crates/simthing-spec/src/compile/region_field_budget.rs::over_budget_rejects` | `TEST-PROMOTE-TEST-PARE-SPEC-B-T2-SIMTHING-SPEC-FIELD-PAYLOAD-ADMISSION-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-pare-spec-field-payload-admission-integration-representative` | simthing-spec | 1 | `crates/simthing-spec/tests/bh2s_stress_compose_admission.rs::bh2s_admission_rejects_input_field_budget_exceeded` | `TEST-PROMOTE-TEST-PARE-SPEC-FIELD-PAYLOAD-ADMISSION-INTEGRATION-REPRESENTATIVE-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-pare-spec-b-t2-simthing-spec-parser-span-admission` | simthing-spec | 1 | `crates/simthing-spec/tests/eml_gadget_tier1.rs::invalid_params_reject` | `TEST-PROMOTE-TEST-PARE-SPEC-B-T2-SIMTHING-SPEC-PARSER-SPAN-ADMISSION-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-pare-spec-b-t2-simthing-spec-finite-number-admission` | simthing-spec | 1 | `crates/simthing-spec/tests/eml_gadget_tier2_acceleration.rs::rejects_non_finite_dt` | `TEST-PROMOTE-TEST-PARE-SPEC-B-T2-SIMTHING-SPEC-FINITE-NUMBER-ADMISSION-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-pare-spec-b-t2-simthing-spec-admission-hard-error` | simthing-spec | 1 | `crates/simthing-spec/tests/jit_kernel_graph_admission.rs::jit_desc2_rejects_cycles` | `TEST-PROMOTE-TEST-PARE-SPEC-B-T2-SIMTHING-SPEC-ADMISSION-HARD-ERROR-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-pare-spec-b-t2-simthing-spec-topology-admission` | simthing-spec | 1 | `crates/simthing-spec/tests/planet_child_location_admission.rs::duplicate_planet_id_rejected` | `TEST-PROMOTE-TEST-PARE-SPEC-B-T2-SIMTHING-SPEC-TOPOLOGY-ADMISSION-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-pare-spec-b-t2-simthing-spec-duplicate-id-admission` | simthing-spec | 1 | `crates/simthing-spec/tests/scenario_ingestion_admission.rs::rejects_duplicate_owner_ids` | `TEST-PROMOTE-TEST-PARE-SPEC-B-T2-SIMTHING-SPEC-DUPLICATE-ID-ADMISSION-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-pare-spec-b-t2-simthing-spec-missing-or-unknown-reference-admission` | simthing-spec | 1 | `crates/simthing-spec/tests/scenario_ingestion_admission.rs::rejects_missing_owner` | `TEST-PROMOTE-TEST-PARE-SPEC-B-T2-SIMTHING-SPEC-MISSING-OR-UNKNOWN-REFERENCE-ADMISSION-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-pare-spec-hygiene-theater-table` | simthing-spec | 1 | `crates/simthing-spec/tests/test_pare_spec_0_hygiene_consolidation.rs::hygiene_theater_cases_table_preserves_inputs` | `TEST-PROMOTE-TEST-PARE-SPEC-HYGIENE-THEATER-TABLE-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |
| `promotion-target:test-consolidate-classifier-families-track-d-hygiene-table` | simthing-spec | 1 | `crates/simthing-spec/tests/test_consolidate_classifier_families_0_hygiene_consolidation.rs::hygiene_theater_cases_table_preserves_inputs` | `TEST-PROMOTE-TEST-CONSOLIDATE-CLASSIFIER-FAMILIES-TRACK-D-HYGIENE-TABLE-0` | Retire when this promotion target has a stronger non-runtime or lower-boundary proof. |

**DA correction (executive DA, 2026-07-02, 0R on PR #1088).** The initial plan listed 122 promotion-target
rows — all of them never-pare-set members: the nine STEAD `stead_spatial_contract.md` §8 required suites
(121 rows) and `custom_layout_ethics_axis` (the invariant proof test). **Never-pare takes categorical
precedence over promotion-targeting**: a doc-named/STEAD-required test may only be retired via a DA/Tier-2
amendment to the naming contract itself, never queued by ledger data. Root cause: the original handoff's
`permanent-residue` enum omitted tokens for doc-named/STEAD-required classes, leaving those rows no legal
permanent home; the enum now carries `permanent-residue:doc-named-invariant` and
`permanent-residue:stead-required`, and the 122 rows are reclassified accordingly. The kernel/sim strict
tier is deliberately **not** widened by these tokens.

**0R on PR #1092 (TEST-CONSOLIDATE-CLASSIFIER-FAMILIES-0R).** Six per-crate Bevy/GPU-linked hygiene-table
promotion targets were collapsed to one CPU-side `simthing-spec` representative
(`promotion-target:test-consolidate-classifier-families-track-d-hygiene-table`). Kernel/sim strict tier:
no `simthing-sim` hygiene-table row.

The genuine promotion backlog now includes `TEST-PARE-SPEC-0` and `TEST-CONSOLIDATE-CLASSIFIER-FAMILIES-0`
retained representatives that survive material paring waves.

The remaining promotion backlog populates as AUDIT rows classify through the paring waves: any
AUDIT row that survives as KEEP must then name a permanent-residue class or a legal (non-never-pare)
promotion target, and this plan regenerates from those rows.

Rows without `promotion-target:` are either permanent residue or already marked for delete/collapse/consolidation/promotion-required by the boundary ledger.
