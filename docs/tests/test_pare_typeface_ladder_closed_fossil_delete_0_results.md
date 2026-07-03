# TEST-PARE-TYPEFACE-LADDER-CLOSED-FOSSIL-DELETE-0 Results

## Status

PROBATION / DA REVIEW. Merge is not authorized for Codex.

This rung implements the DA-held deletion wave over the closed `simthing-tools` TYPEFACE-LADDER rows that #1115 left open.

## Mission

Classify and delete closed TYPEFACE-LADDER LR* fossil rows in `simthing-tools`, while preserving golden-byte glyph byte-exactness and selected admission representatives.

This rung deletes closed TYPEFACE-LADDER fossil rows without executing or compiling simthing-tools, simthing-mapeditor, Bevy, desktop, windowing, typeface, or GPU surfaces. AUDIT does not mean KEEP; closed-ladder capability-proof residue must be classified and deleted when it is not golden-byte, selected admission, or otherwise never-pare.

## Scope

In scope:

- 175 current `simthing-tools` TYPEFACE-LADDER LR* `AUDIT` rows in:
  - `crates/simthing-tools/tests/typeface_lr3.rs`
  - `crates/simthing-tools/tests/typeface_lr4.rs`
  - `crates/simthing-tools/tests/typeface_lr5.rs`
  - `crates/simthing-tools/tests/typeface_lr6.rs`
  - `crates/simthing-tools/tests/typeface_lr6a_icon_geometry.rs`
  - `crates/simthing-tools/tests/typeface_lr6b.rs`
  - `crates/simthing-tools/tests/typeface_lr6c.rs`
  - `crates/simthing-tools/tests/typeface_lr6d.rs`
  - `crates/simthing-tools/tests/typeface_lr7.rs`
  - `crates/simthing-tools/tests/typeface_lr9.rs`

Out of scope:

- `simthing-mapeditor` LR8 and live Studio behavior rows.
- Active 0.0.8.5 Studio consumer coverage.
- Studio/mapeditor admission singleton adjudication.
- `STUDIO-TYPEFACE-STAR-NAMEPLATES-0` owner visual sign-off.
- Admission Substrate, SimThing-Kernel, CI gate/profile/scanner/allowlist/workflow logic.

## DA Finding Being Remediated

#1115 was accepted as an interim review, but DA rejected the conclusion that the wave had no deletions. The corrected split is:

- 175 `simthing-tools` TYPEFACE-LADDER LR* `AUDIT` rows are closed-ladder residue requiring row-level classification.
- Behavior-regression/capability-proof rows in that closed ladder are fossil DELETE candidates.
- Golden-byte rows and selected admission representatives are never-pare.
- 618 `simthing-mapeditor` / live-Studio rows are out of scope for this rung.

## Method

1. Built the current target set from `scripts/ci/test_inventory.tsv`, not from rows already marked DELETE.
2. Joined current inventory rows to `scripts/ci/test_pare_boundary_rows.tsv` and `scripts/ci/test_pare_audit.tsv`.
3. Classified current `simthing-tools` typeface LR rows into DELETE or KEEP_NEVER_PARE.
4. Deleted behavior-regression closed-ladder rows at file or function granularity.
5. Removed corresponding live inventory rows.
6. Preserved historical boundary rows as DELETE rows with this rung named.
7. Updated stale selected-admission audit rows to KEEP representatives.

## 175-Row Target Set

The 175 `AUDIT` rows split as:

| Classification | Count | Action |
|---|---:|---|
| DELETE | 170 | deleted |
| KEEP_NEVER_PARE selected admission representatives | 5 | kept |
| ESCALATE | 0 | none |
| OUT_OF_SCOPE | 0 | none in target set |

The manifest also records 5 golden-byte rows that were already `KEEP` outside the 175-row `AUDIT` target.

## Classification Counts

Manifest rows:

| Classification | Count |
|---|---:|
| DELETE | 170 |
| KEEP_NEVER_PARE | 10 |
| ESCALATE | 0 |
| OUT_OF_SCOPE | 0 |

## Deleted Rows

Deleted 170 test rows:

- Whole-file deletions: 5 files / 97 test rows.
  - `crates/simthing-tools/tests/typeface_lr3.rs` (12)
  - `crates/simthing-tools/tests/typeface_lr6b.rs` (25)
  - `crates/simthing-tools/tests/typeface_lr6c.rs` (21)
  - `crates/simthing-tools/tests/typeface_lr6d.rs` (26)
  - `crates/simthing-tools/tests/typeface_lr9.rs` (13)
- Function-level deletions from mixed files: 73 test rows.
  - `typeface_lr4.rs`: 6 deleted, 1 golden kept.
  - `typeface_lr5.rs`: 30 deleted, 1 golden kept.
  - `typeface_lr6.rs`: 17 deleted, 1 golden + 1 selected admission kept.
  - `typeface_lr6a_icon_geometry.rs`: 11 deleted, 1 golden kept.
  - `typeface_lr7.rs`: 9 deleted, 1 golden + 4 selected admission kept.

## KEEP / Never-Pare Rows

Golden-byte glyph byte-exactness retained:

- `typeface_lr4.rs` / `icon_tile_bytes_deterministic`
- `typeface_lr5.rs` / `bench_result_report_is_deterministic_enough`
- `typeface_lr6.rs` / `msdf_glyph_tile_is_deterministic`
- `typeface_lr6a_icon_geometry.rs` / `icon_vector_geometry_is_deterministic`
- `typeface_lr7.rs` / `codepoint_table_is_stable_golden`

Selected admission representatives retained:

- `typeface_lr6.rs` / `glyph_source_api_does_not_silently_claim_unsupported_glyph_ids`
- `typeface_lr7.rs` / `duplicate_name_rejected`
- `typeface_lr7.rs` / `invalid_or_dynamic_svg_rejected`
- `typeface_lr7.rs` / `missing_svg_path_errors`
- `typeface_lr7.rs` / `path_escape_rejected`

## ESCALATE Rows

None in the 175-row `simthing-tools` closed-ladder target set.

The Studio/mapeditor admission singletons escalated by #1115 remain outside this rung.

## Out-of-Scope Rows

Out-of-scope summary only:

- 618 `simthing-mapeditor` / live-Studio rows remain for live Studio behavior and active 0.0.8.5 consumer coverage.
- `crates/simthing-mapeditor/tests/typeface_lr8.rs` is not touched.
- Active Terran-Pirate rows are not touched.
- Admission Substrate and SimThing-Kernel are not touched.

## Coverage Map

| Deleted surface | Coverage owner |
|---|---|
| LR3 instanced-text capability proof rows | TYPEFACE-TRACK-CLOSEOUT-0; closed ladder evidence in `docs/archive/typeface_track_2026_06`; no live Studio consumer depends on the exact rows. |
| LR4 SVG/icon behavior rows | Golden `icon_tile_bytes_deterministic` plus selected LR7 admission representatives; closed LR4 evidence remains archived. |
| LR5 benchmark/perf behavior rows | Golden `bench_result_report_is_deterministic_enough`; closed LR5/LR5R/LR5S/LR5T evidence remains archived. |
| LR6 MSDF behavior rows | Golden `msdf_glyph_tile_is_deterministic` and selected unsupported-glyph admission representative. |
| LR6A icon geometry behavior rows | Golden `icon_vector_geometry_is_deterministic`; closed LR6A icon geometry evidence remains archived. |
| LR6B/LR6C/LR6D style/deform/path/warp behavior rows | TYPEFACE-TRACK-CLOSEOUT-0 and archived accepted rung evidence; no current row is golden/admission. |
| LR7 manifest behavior rows | Golden codepoint table and selected manifest admission representatives. |
| LR9 final perf behavior rows | TYPEFACE-TRACK-CLOSEOUT-0 plus archived LR9 binding evidence; no current row is golden/admission. |

## Forbidden Proof Avoided

- No `cargo check -p simthing-tools --tests`.
- No `cargo check -p simthing-mapeditor --tests`.
- No `cargo test -p simthing-tools`.
- No `cargo test -p simthing-mapeditor`.
- No typeface runtime, mapeditor runtime, Studio runtime, Bevy, winit, wgpu, ALSA/libasound, X/X11/Xvfb, Wayland, Mesa/Vulkan, libudev/udev, xkbcommon/xcb/EGL/GLX, or `apt-get`.
- No smuggling through `-p`, `--package`, `crate_checks`, `tests`, `doc_tests`, or `workflow_dispatch`.

## Inventory Delta

- Before: 4,240 rows.
- After: 4,070 rows.
- Delta: -170 rows.

## Proof

- Doctrine Scan: PASS (`DOCTRINE-SCAN-VERDICT: PASS failures=0 inspect=0 selftest=SKIPPED`).
- Digest: PASS (`gen_digest --check: PASS`).
- Inventory check: PASS (`rows: 4070`, `discovered: 4070`, `missing: 0`, `extra: 0`, edit-scope PASS).
- Boundary check: PASS (`live inventory rows: 4070`, `historical PARED rows mapped: 1675`).
- Drift check: PASS (`rows: 4070`, `discovered: 4070`, `unledgered: 0`, `stale: 0`, `promotion-target rows: 29`).
- Survivor compile floor: PASS for the allowed crates only:
  - `cargo check -p simthing-core --tests`
  - `cargo check -p simthing-kernel --tests`
  - `cargo check -p simthing-sim --tests`
  - `cargo check -p simthing-workshop --tests`
  - `cargo check -p simthing-mapgenerator --tests`
- `git diff --check origin/master...HEAD`: PASS.
- Live Doctrine Scan: pending after PR opens.
- Live Doctrine Exec smoke: pending after PR opens.

## Graduation Routing

```text
Graduation routing:
  Status: PROBATION / DA REVIEW
  Risk class: owner-deep deletion wave / closed-ladder fossil pare / DA-held
  Protected corpus touched: no
  Active TP / NEVER_PARE touched: no
  CI profile/gate/scanner/allowlist/workflow touched: no
  simthing-tools/simthing-mapeditor execution: avoided
  DA question: Does Opus accept the row-level split and deletion of closed TYPEFACE-LADDER fossil rows while preserving never-pare golden/admission representatives?
```
