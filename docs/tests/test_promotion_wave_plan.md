# Test Promotion Wave Plan

This is the rustification backlog for KEEP rows whose survival is tied to a named promotion target. A follow-on `TEST-PROMOTE-<boundary>-0` rung must land the type/admission/scan boundary and retire redundant tests in the same PR.

total promotion-target rows: 122

| target_boundary | owning_crate | row_count | example_rows | recommended_promotion_rung | retirement_action |
|---|---|---:|---|---|---|
| `promotion-target:doc-named-invariant-boundary` | simthing-core (1) | 1 | `crates/simthing-core/src/property.rs::custom_layout_ethics_axis` | `TEST-PROMOTE-DOC_NAMED_INVARIANT_BOUNDARY-0` | land owner boundary; delete/collapse/consolidate rows that become redundant in the same PR |
| `promotion-target:stead-required-contract` | simthing-clausething (121) | 121 | `crates/simthing-clausething/src/mapgen_palma.rs::cfg_test_mod::tests`<br>`crates/simthing-clausething/src/mapgen_palma.rs::default_columns_match_pr6_slice_geometry`<br>`crates/simthing-clausething/tests/mapgen_constitution_guards.rs::allow_extended_horizon_is_rejected` | `TEST-PROMOTE-STEAD_REQUIRED_CONTRACT-0` | land owner boundary; delete/collapse/consolidate rows that become redundant in the same PR |

Rows without `promotion-target:` are either permanent residue or already marked for delete/collapse/consolidation/promotion-required by the boundary ledger.
