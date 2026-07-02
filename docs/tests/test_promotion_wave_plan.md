# Test Promotion Wave Plan

This is the rustification backlog for KEEP rows whose survival is tied to a named promotion target. A follow-on `TEST-PROMOTE-<boundary>-0` rung must land the type/admission/scan boundary and retire redundant tests in the same PR.

total promotion-target rows: 9

| target_boundary | owning_crate | row_count | example_rows | recommended_promotion_rung | retirement_action |
|---|---|---:|---|---|---|
| `promotion-target:test-pare-spec-b-t2-simthing-spec-admission-hard-error` | simthing-spec | 1 | `crates/simthing-spec/tests/jit_kernel_graph_admission.rs::jit_desc2_rejects_cycles` | `TEST-PROMOTE-SPEC-ADMISSION-HARD-ERROR-0` | Retire when the generic admission hard-error boundary has a stronger non-runtime proof. |
| `promotion-target:test-pare-spec-b-t2-simthing-spec-duplicate-id-admission` | simthing-spec | 1 | `crates/simthing-spec/tests/scenario_ingestion_admission.rs::rejects_duplicate_owner_ids` | `TEST-PROMOTE-SPEC-DUPLICATE-ID-0` | Retire when duplicate-id admission is owned below runtime tests. |
| `promotion-target:test-pare-spec-b-t2-simthing-spec-field-payload-admission` | simthing-spec | 1 | `crates/simthing-spec/src/compile/region_field_budget.rs::over_budget_rejects` | `TEST-PROMOTE-SPEC-FIELD-PAYLOAD-0` | Retire when field-payload budget admission is owned below runtime tests. |
| `promotion-target:test-pare-spec-b-t2-simthing-spec-finite-number-admission` | simthing-spec | 1 | `crates/simthing-spec/tests/eml_gadget_tier2_acceleration.rs::rejects_non_finite_dt` | `TEST-PROMOTE-SPEC-FINITE-NUMBER-0` | Retire when finite-number admission is owned below runtime tests. |
| `promotion-target:test-pare-spec-b-t2-simthing-spec-missing-or-unknown-reference-admission` | simthing-spec | 1 | `crates/simthing-spec/tests/scenario_ingestion_admission.rs::rejects_missing_owner` | `TEST-PROMOTE-SPEC-REFERENCE-ADMISSION-0` | Retire when missing/unknown-reference admission is owned below runtime tests. |
| `promotion-target:test-pare-spec-b-t2-simthing-spec-parser-span-admission` | simthing-spec | 1 | `crates/simthing-spec/tests/eml_gadget_tier1.rs::invalid_params_reject` | `TEST-PROMOTE-SPEC-PARSER-SPAN-0` | Retire when parser-span admission is owned below runtime tests. |
| `promotion-target:test-pare-spec-b-t2-simthing-spec-topology-admission` | simthing-spec | 1 | `crates/simthing-spec/tests/planet_child_location_admission.rs::duplicate_planet_id_rejected` | `TEST-PROMOTE-SPEC-TOPOLOGY-0` | Retire when topology admission is owned below runtime tests. |
| `promotion-target:test-pare-spec-field-payload-admission-integration-representative` | simthing-spec | 1 | `crates/simthing-spec/tests/bh2s_stress_compose_admission.rs::bh2s_admission_rejects_input_field_budget_exceeded` | `TEST-PROMOTE-SPEC-FIELD-PAYLOAD-0` | Retire with the field-payload admission representative above. |
| `promotion-target:test-pare-spec-hygiene-theater-table` | simthing-spec | 1 | `crates/simthing-spec/tests/test_pare_spec_0_hygiene_consolidation.rs::hygiene_theater_cases_table_preserves_inputs` | `TEST-PROMOTE-SPEC-HYGIENE-THEATER-0` | Retire if the classifier-input family gets a stronger generated table or non-runtime proof. |

**DA correction (executive DA, 2026-07-02, 0R on PR #1088).** The initial plan listed 122 promotion-target
rows — all of them never-pare-set members: the nine STEAD `stead_spatial_contract.md` §8 required suites
(121 rows) and `custom_layout_ethics_axis` (the invariant proof test). **Never-pare takes categorical
precedence over promotion-targeting**: a doc-named/STEAD-required test may only be retired via a DA/Tier-2
amendment to the naming contract itself, never queued by ledger data. Root cause: the original handoff's
`permanent-residue` enum omitted tokens for doc-named/STEAD-required classes, leaving those rows no legal
permanent home; the enum now carries `permanent-residue:doc-named-invariant` and
`permanent-residue:stead-required`, and the 122 rows are reclassified accordingly. The kernel/sim strict
tier is deliberately **not** widened by these tokens.

The genuine promotion backlog now starts with `TEST-PARE-SPEC-0`: retained representatives that survive
the first material spec paring wave carry promotion targets so they can be retired by later stronger
boundaries rather than becoming permanent residue by accident.

The remaining promotion backlog populates as the 5,032 AUDIT rows classify through the paring waves: any
AUDIT row that survives as KEEP must then name a permanent-residue class or a legal (non-never-pare)
promotion target, and this plan regenerates from those rows.

Rows without `promotion-target:` are either permanent residue or already marked for delete/collapse/consolidation/promotion-required by the boundary ledger.
