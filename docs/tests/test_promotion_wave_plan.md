# Test Promotion Wave Plan

This is the rustification backlog for KEEP rows whose survival is tied to a named promotion target. A follow-on `TEST-PROMOTE-<boundary>-0` rung must land the type/admission/scan boundary and retire redundant tests in the same PR.

total promotion-target rows: 0

| target_boundary | owning_crate | row_count | example_rows | recommended_promotion_rung | retirement_action |
|---|---|---:|---|---|---|
| *(none at this stage)* | — | 0 | — | — | — |

**DA correction (executive DA, 2026-07-02, 0R on PR #1088).** The initial plan listed 122 promotion-target
rows — all of them never-pare-set members: the nine STEAD `stead_spatial_contract.md` §8 required suites
(121 rows) and `custom_layout_ethics_axis` (the invariant proof test). **Never-pare takes categorical
precedence over promotion-targeting**: a doc-named/STEAD-required test may only be retired via a DA/Tier-2
amendment to the naming contract itself, never queued by ledger data. Root cause: the original handoff's
`permanent-residue` enum omitted tokens for doc-named/STEAD-required classes, leaving those rows no legal
permanent home; the enum now carries `permanent-residue:doc-named-invariant` and
`permanent-residue:stead-required`, and the 122 rows are reclassified accordingly. The kernel/sim strict
tier is deliberately **not** widened by these tokens.

The genuine promotion backlog populates as the 5,472 AUDIT rows classify through the paring waves: any
AUDIT row that survives as KEEP must then name a permanent-residue class or a legal (non-never-pare)
promotion target, and this plan regenerates from those rows.

Rows without `promotion-target:` are either permanent residue or already marked for delete/collapse/consolidation/promotion-required by the boundary ledger.
