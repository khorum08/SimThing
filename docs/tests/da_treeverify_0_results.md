# DA-TREEVERIFY-0 Results

## Status

**PROOF-PRESENT / harness advisor landed.** Advisory DA review-depth profiler for weighted verify-the-tree.
Not a clearance verdict; not merge authority.

## Identity

| Field | Value |
|---|---|
| Surfaces | `scripts/ci/da_treeverify.sh`, `scripts/ci/da_treeverify_lib.py`, `scripts/ci/da_review_profile.tsv` |
| Kind | Gate-wiring / harness adjacency (advisory) |
| Authority | Advisory `DA-TREEVERIFY-PROFILE` only |

## What it does

- Profiles changed paths → `RELAX` / `LIGHT-TREE` / `DEEP-TREE` + focus pack
- Core TSV rows permanent; non-core require `expires_on` and fail lifecycle if still `active` after expiry
- Expeditionary escape: charter + until; cannot RELAX production/engine/long-lifecycle
- Unmatched surfaces default DEEP (never silent RELAX)

## Commands

```bash
bash scripts/ci/da_treeverify.sh --selftest
bash scripts/ci/da_treeverify.sh --check-lifecycle
bash scripts/ci/da_treeverify.sh --files-from <list> [--body-file <body>]
bash scripts/ci/da_treeverify.sh --pr <n>
bash scripts/ci/clearance_check.sh --selftest
bash scripts/ci/gen_orientation.sh --check
bash scripts/ci/doc_budget_check.sh --check
```

## Selftest

`DA-TREEVERIFY-SELFTEST: PASS` (12 checks): docs RELAX, production/engine/CI DEEP, workshop LIGHT,
unclassified DEEP, expeditionary missing-until FAIL, expeditionary docs LIGHT, expeditionary production DEEP,
lifecycle expired non-core FAIL, lifecycle core PASS, live lifecycle PASS.

## Integration

- Clearance `GATE_WIRING_PATHS` includes da_treeverify surfaces
- Doctrine-scan stock gates run `--check-lifecycle`
- Orientation / onboarding / agents.md / handoff / closeout protocol point here
