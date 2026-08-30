# CLEARING-WEIGHT-SEMANTIC-PARTITION-0 Results

- Status: **PROBATION / proof-present / DA-review-pending**
- Rung: `CLEARING-WEIGHT-SEMANTIC-PARTITION-0`
- Handoff-authored provenance base: `861e098419328f362dee9a09e0536d4833ffb6cc`
- Exact dispatch base: `7858f018a8a2d0bf15e0f6eed709f8e4afbfdfd2`
- tested_code_sha / final_head_sha: PR- and Board-relay-bound after this evidence commit; this file does not self-hash
- ORIENT-RECEIPT: `66843f36b567`
- role: `coding`
- orientation_rule_stamp: `3e76a60f12435301`
- orientation_digest_sha: `8078029be9b3d8d6bb9838732746ed89eb6388467ca435f8846a0509d081a6fd`
- HD-RECEIPT: `a5dd6508d523`
- expected route: `DA-RESERVE(binding)`

## Mandatory RED-before

Before any production edit, the exact DA S1 was added as the first standing
witness: four preorder rows `root -> child -> grandchild`, then the root's
sibling; default `1.0`; child `Set(1.0)`. Admission asserted exactly one stored
profile/span. Changing the default from `1.0` to `2.0` then reproduced the
dispatch-base loss of the compressed child derivation boundary:

```text
assertion `left == right` failed
  left: Some(2.0)
 right: Some(1.0)
```

Command: `cargo test -p simthing-driver --test clearing_weight_semantic_partition_0 exact_s1_reconstitutes_equal_valued_child_boundary_after_default_change -- --nocapture`

The failure occurred only after the `(profile_count, span_count) == (1, 1)`
admission assertion passed. The handoff stop condition therefore did not fire.

## Semantic-window repair

`ClearingWeightSpanProjection::refresh` now treats the frozen override subtree
ranges as derivation authority and the stored effective spans as compressed
representation. For each invalidated range independently it:

1. collects the affected range start/end and the clipped start/end of every
   intersecting admitted override subtree;
2. sorts and deduplicates those boundaries;
3. resolves one effective weight/profile at each resulting window start; and
4. remaps that window through the existing
   `DerivedSpanProjection::remap_range` authority.

The graduated adjacent-equal coalescer remains the only physical merge
authority. No generic derived-span surface, registry, cache, dense row map, or
secondary clearing path was added.

## GREEN acceptance matrix

| Acceptance item | Semantic bounds | Result |
|---|---|---|
| Exact S1 default refresh | affected `[0,4)`; child override `[1,3)`; windows `[0,1)`, `[1,3)`, `[3,4)` | PASS — root/sibling `2.0`; child/grandchild `1.0`; physical counts change from `1/1` to `2/3` |
| S1 re-coalescence | the same three independently evaluated windows | PASS — default restored to `1.0`; all four rows `1.0`; physical counts return to maximal `1/1` |
| Nested preservation | affected target `[3,5)`; leaf override `[4,5)`; windows `[3,4)`, `[4,5)` | PASS — initially compressed target/leaf `2.0`; ancestor changes to `Multiply(3.0)` so target becomes `3.0` while leaf remains `2.0` |
| Nested re-coalescence | the same two semantic windows | PASS — ancestor returns to `Multiply(2.0)`; target/leaf return to one adjacent-equal physical region |

The exact S1 and nested witness pass together: 2 passed, 0 failed.

## Affected-only locality

| Refresh | Affected ranges | Affected rows | Dirty spans | Semantic windows rebuilt | Spans examined | Member rows scanned | Bounded unaffected identities checked / changed |
|---|---:|---:|---:|---:|---:|---:|---:|
| S1 default `1 -> 2` | 1 | 4 | 1 | 3 | 1 | 0 | 0 / 0 |
| Nested target `Multiply(2) -> Multiply(3)` | 1 | 2 | 1 | 2 | 1 | 0 | 2 / 0 |
| Nested target `Multiply(3) -> Multiply(2)` | 1 | 2 | 1 | 2 | 1 | 0 | 2 / 0 |

The nested fixture has six unrelated stored physical regions. Their effective
values remain exactly `10.0`, `20.0`, `30.0`, and `40.0`, while the refresh
examines one dirty span and evaluates only the two affected semantic windows.
Immediate rows before/after an affected range are the only production
unaffected-profile probes.

Production search finds `semantic_windows`, bounded
`unaffected_profile_samples`, and per-window `remap_range` calls, with zero
`iter_spans` or `spans_in_range` calls in
`clearing_weight_projection.rs`. Thus refresh performs no logical-member scan
and no whole-projection span walk.

## Predecessor seal equality

The required dispatch-base census was captured before production edits and
repeated after the repair. All three files are byte-identical:

| Seal file | Base/head blob | `assert_eq!` | `assert!` | `assert_ne!` | Total assertions |
|---|---|---:|---:|---:|---:|
| `clearing_weight_span_unification_0.rs` | `ebb911f16d429f7388a9ed3e624973418a2b2d25` | 13 | 4 | 1 | 18 |
| `stemthing_b_flow_market_germ_0.rs` | `eea2784d15919f30097f34b394cc9c007c0b3e70` | 30 | 15 | 1 | 46 |
| `clearing_weight_deformation_lifecycle_0.rs` | `1049ba7c5fd99626f967b0c2899552cdabd6edbb` | 26 | 0 | 0 | 26 |

## Verification

| Command / proof | Result |
|---|---|
| Focused 13.9 semantic-partition witness | PASS — 2 passed, 0 failed |
| 13.8 deformation lifecycle | PASS — 1 passed, 0 failed |
| 13.6 matrix, including integer apportionment, tie rotation, and replay | PASS — 2 passed, 0 failed |
| Flow-market germ battery | PASS — 4 passed, 0 failed |
| Generic derived-span tests | PASS — 4 passed, 0 failed |
| Constrained-clearing execution seal | PASS — 1 passed, 0 failed |
| `cargo check -p simthing-kernel` / `cargo check -p simthing-driver` | PASS / PASS |
| Test inventory / drift | PASS — 1,360 discovered and ledgered; zero missing, extra, unledgered, parked, or stale |
| `cargo fmt --all -- --check` / `git diff --check` | PASS / PASS |
| `cargo test --workspace --all-targets --no-fail-fast -j 1 --quiet` | `STRUCTURAL-CERTIFICATE-SUMMARY suites=128 passed=481 failed=0 ignored=14 exit=0` |

Exact-head agent/doctrine/orientation/handoff/anchor/diff checks, hosted
Doctrine Scan/Exec, fresh `/clearance`, and `/relay-lint` are recorded in the
PR/Board relay after publication of the final head.

## Changed-file and scope census

Exactly the six handoff-authorized surfaces changed:

- the one clearing-weight production module;
- the new two-test semantic-partition witness;
- two appended test-inventory rows;
- required append-only anchor reach;
- this new results packet; and
- one current-evidence-index row.

No predecessor seal, generic derived-span implementation, constrained-clearing
arithmetic, eligibility, epsilon, tie/remainder law, DecimalField exactness,
generation authority, replay, dependency direction, manifest, lockfile, gate
code, performance ledger, closeout artifact, pointer, or next-track surface
changed.

Return posture is **PROBATION / proof-present / DA-review-pending** under
`DA-RESERVE(binding)`. Coding does not merge or self-graduate.

Owner hold: `5466933355`.
