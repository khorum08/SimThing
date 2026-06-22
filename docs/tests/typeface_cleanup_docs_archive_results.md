# TYPEFACE-CLEANUP-DOCS-ARCHIVE-0 Results

## Status

PASS — typeface design/API material consolidated; process reports archived; live test doc paths updated; no
runtime behavior changes.

## PR / branch / merge

- Branch: `typeface-cleanup-docs-archive-0`
- PR: #900
- Merge SHA: `eafa522856`

## Core design update

Added `## SimThing tools crate — presentation/support services` to `docs/simthing_core_design.md` with
authority boundaries and compact API seam block.

## Constitution update

Added `## Typeface / simthing-tools reference` pointer section to `docs/design_0_0_8_3.md`.

## Typeface docs consolidation

- `docs/design_typeface_ladder.md` — added Final architecture summary, Public API seams, Module map, Runtime
  authority boundaries, GPU-residency guarantees, SVG/TTF asset policy, Studio label seam, Final validation
  and closure, Carried non-blocking debts; report links point to archive.
- `docs/design_simthing_typeface_track_proposal.md` — status CLOSED; ladder table collapsed/superseded; links
  to ladder doc.

## Archived reports

Moved 27 process reports to `docs/archive/typeface_track_2026_06/` (see table below). Added archive README.

Smoke PNG artifacts **kept** in `docs/tests/` (`typeface_lr3r_smoke.png`, `typeface_lr6_sdf_smoke.png`,
`typeface_lr3_smoke.png`) — live tests reference these paths.

## Deleted files

No tracked files deleted. No `git clean` executed — untracked local scratch documented as REVIEW only.

## Files reviewed and kept

| Path | Action | Reason | Reference check |
|---|---|---|---|
| `docs/tests/typeface_lr3r_smoke.png` | KEEP | LR3 test fixture path | `typeface_lr3.rs` |
| `docs/tests/typeface_lr6_sdf_smoke.png` | KEEP | LR6 smoke artifact | `typeface_lr6.rs` |
| `docs/tests/typeface_lr3_smoke.png` | KEEP | LR3 legacy smoke | production log reference |
| `docs/tests/current_evidence_index.md` | KEEP | live ledger | required |
| `docs/tests/typeface_cleanup_docs_archive_results.md` | KEEP | this report | new live doc |
| `agent-tools/` (untracked) | REVIEW | local agent scratch | not in repo; do not delete |
| `mcps/`, `terminals/` (untracked) | REVIEW | local IDE tooling | not in repo |
| `target/` (ignored) | KEEP | build output | standard cargo ignore |

## Disk-space recovered

Not measured — no deletions performed in this PR.

## Reference/link checks

- Updated live test `read_doc` / `include_str!` paths in `typeface_lr5`, `typeface_lr6*`, `typeface_lr7`,
  `typeface_lr9`, `typeface_lr8` (mapeditor) to `docs/archive/typeface_track_2026_06/`.
- Evidence index TYPEFACE rows updated to archive paths.
- Ladder report links updated to archive paths.

## Validation

```text
cargo fmt -p simthing-tools -p simthing-workshop -p simthing-mapeditor -- --check
cargo check -p simthing-tools
cargo check -p simthing-workshop
cargo check -p simthing-mapeditor
cargo test -p simthing-tools
cargo test -p simthing-tools --test semantic_free_guard
cargo test -p simthing-tools --test typeface_lr9
cargo test -p simthing-mapeditor --test typeface_lr8
git diff --check
```

## Remaining debts

Unchanged from track closeout: flat 5k O(N) scan spike; 5k damage churn changed-value cost; interactive Studio
window smoke; production icon art/source set; future dirty-list optimization.

## DA recommendation

**ACCEPTED / closed** — cleanup complete; typeface track remains **CLOSED / DA-APPROVED**; no ladder reopen.

## Archived report inventory

| Path | Action | Reason | Reference check |
|---|---|---|---|
| `docs/archive/typeface_track_2026_06/typeface_lr0_results.md` | ARCHIVE | process report | index + ladder |
| `docs/archive/typeface_track_2026_06/typeface_lr1_results.md` | ARCHIVE | process report | index + ladder |
| `docs/archive/typeface_track_2026_06/typeface_lr2_results.md` | ARCHIVE | process report | index + ladder |
| `docs/archive/typeface_track_2026_06/typeface_lr2r_results.md` | ARCHIVE | process report | index + ladder |
| `docs/archive/typeface_track_2026_06/typeface_lr3_results.md` | ARCHIVE | process report | index + ladder |
| `docs/archive/typeface_track_2026_06/typeface_lr4_results.md` | ARCHIVE | process report | index + ladder |
| `docs/archive/typeface_track_2026_06/typeface_lr5_results.md` | ARCHIVE | process report | index + ladder |
| `docs/archive/typeface_track_2026_06/typeface_lr5r_results.md` | ARCHIVE | process report | index + ladder |
| `docs/archive/typeface_track_2026_06/typeface_lr5s_results.md` | ARCHIVE | process report | index + ladder |
| `docs/archive/typeface_track_2026_06/typeface_lr5t_results.md` | ARCHIVE | process report | index + lr5 test |
| `docs/archive/typeface_track_2026_06/typeface_lr6_results.md` | ARCHIVE | process report | index + lr6 test |
| `docs/archive/typeface_track_2026_06/typeface_lr6a_results.md` | ARCHIVE | process report | lr6 test |
| `docs/archive/typeface_track_2026_06/typeface_lr6a_icon_geometry_results.md` | ARCHIVE | process report | lr6a test |
| `docs/archive/typeface_track_2026_06/typeface_lr6a_icon_msdf_deferred.md` | ARCHIVE | deferral note | lr6 test |
| `docs/archive/typeface_track_2026_06/typeface_lr6b_results.md` | ARCHIVE | process report | lr6b test |
| `docs/archive/typeface_track_2026_06/typeface_lr6b_style_buffer_residency_results.md` | ARCHIVE | process report | lr6b test |
| `docs/archive/typeface_track_2026_06/typeface_lr6c_results.md` | ARCHIVE | process report | lr6c/d test |
| `docs/archive/typeface_track_2026_06/typeface_lr6c_deform_uv_sampling_results.md` | ARCHIVE | process report | lr6c/d test |
| `docs/archive/typeface_track_2026_06/typeface_lr6d_results.md` | ARCHIVE | process report | lr6d/7/9 test |
| `docs/archive/typeface_track_2026_06/typeface_lr6d_combined_msdf_deform_results.md` | ARCHIVE | process report | lr6d/7 test |
| `docs/archive/typeface_track_2026_06/typeface_lr7_results.md` | ARCHIVE | process report | lr7/8 test |
| `docs/archive/typeface_track_2026_06/typeface_lr8_results.md` | ARCHIVE | process report | lr8/9 test |
| `docs/archive/typeface_track_2026_06/typeface_lr8_studio_plugin_mount_results.md` | ARCHIVE | process report | lr8/9 test |
| `docs/archive/typeface_track_2026_06/typeface_lr9_results.md` | ARCHIVE | process report | lr9 test |
| `docs/archive/typeface_track_2026_06/typeface_lr9_binding_perf_results.md` | ARCHIVE | binding evidence | lr9 test |
| `docs/archive/typeface_track_2026_06/typeface_closeout_perf_invariant_results.md` | ARCHIVE | closeout blocker | index |
| `docs/archive/typeface_track_2026_06/README.md` | KEEP | archive index | new |
