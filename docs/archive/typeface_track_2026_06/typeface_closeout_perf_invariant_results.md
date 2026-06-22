# TYPEFACE-CLOSEOUT-PERF-INVARIANT-0 Results

## Status

PASS — `cargo test -p simthing-tools` green on validation host after fixing changed-label icon rebuild allocation path. **PROBATION / closure blocker remediation** — typeface track not closed.

## PR / branch / merge

- Branch: `typeface-closeout-perf-invariant-0`
- PR: #897
- Merge SHA: `974ffcc7fc`

## Blocking issue

Clean master `cargo test -p simthing-tools` failed on:

```text
changed_label_rebuild_does_not_clone_old_instance_vec (typeface_lr5.rs)
```

Guard asserted `rebuild_changed_labels` must not contain `extend_from_slice(&instances)` — a copy from a temporary instance `Vec` during changed-label rebuild.

## Root cause

**Stale invariant violation after legitimate LR7/LR8 icon-path addition** — not a runtime perf regression or flakiness.

When `rebuild_changed_labels` handles manifest-icon mixed runs (`build_mixed_instances`), the existing-instances branch used:

```rust
existing.0.clear();
existing.0.extend_from_slice(&instances);
```

This copies from a freshly built temporary `Vec` into retained entity storage. The LR5S guard correctly flags it: changed-label rebuild should move or write in-place, not copy from a throwaway buffer. The non-icon text path already used clear + push in-place (correct).

## Fix

Replace copy with move assignment in `rebuild_changed_labels` icon branch:

```rust
existing.0 = instances;
```

Moves the new instance buffer into entity storage; drops the old vec without cloning contents. Non-icon paths unchanged (clear + push in-place; numeric lane uses `patch_numeric_instances`).

## Test disposition

`changed_label_rebuild_does_not_clone_old_instance_vec` — **green, retained unchanged**. Source-grep guard remains the authoritative allocation-path invariant for changed-label rebuild.

## Validation

```text
cargo fmt -p simthing-tools -p simthing-workshop -p simthing-mapeditor -- --check
cargo check -p simthing-tools
cargo check -p simthing-workshop
cargo check -p simthing-mapeditor
cargo test -p simthing-tools
cargo test -p simthing-tools --test typeface_lr9
cargo test -p simthing-tools --test semantic_free_guard
git diff --check
```

## Perf interpretation

Fix removes an unnecessary memcpy on manifest-icon label text changes. LR9 binding evidence (#896) interpretation preserved:

- Flat 5k avg noop **0.5037 ms** (max spike **1.0086 ms** watch item unchanged)
- Numeric 5k avg noop **0.3260 ms**; damage churn **~2.5 ms/frame** is changed-value cost, not noop regression
- Warped 256 avg noop **0.0683 ms**

Icon-mixed label rebuilds were not on the 5k binding hot path; this fix closes the guard without re-measuring binding profiles.

## GPU residency / CPU surfacing audit

Import/staging only — allocation-path fix in Update rebuild; no per-frame shaping/raster of unchanged labels; no manifest/SVG reload; aggregate patch and numeric lane paths unchanged.

## Files changed

- `crates/simthing-tools/src/bevy.rs` — move-assign icon instances on changed-label rebuild
- `docs/tests/typeface_closeout_perf_invariant_results.md` (this file)
- Docs: ladder, evidence index, production log, LR9 results cross-refs

## Remaining non-blocking debts

- Flat 5k max no-op spike watch item (**1.0086 ms** on validation host)
- 5k damage churn as changed-value cost (~2.5 ms/frame), not settled noop budget
- Interactive Studio window smoke
- Production icon source set beyond fixture manifest
- Future dirty-list / event-driven scan optimization

## DA recommendation

Recommend **PROBATION** — closure blocker cleared; full `simthing-tools` suite green. LR9 architecturally DA-approved after #896 binding evidence; **do not** self-close the typeface track in this PR. Codex may proceed to track-closure review with clean perf-invariant guard.
