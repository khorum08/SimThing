# TEST-PARE-STUDIO-TYPEFACE-OWNER-DEEP-0 Results

## Status

HOLD / DA REVIEW. Merge is not authorized for Codex.

This is a review-only rung with zero deletions. The current boundary ledger did not expose a live Studio/typeface DELETE row that satisfied the handoff deletion standard.

## Mission

Review lingering Studio/typeface owner-deep rows and delete only tests that are clearly dead, fossil, or redundant under `docs/ci_screening_surface.md` Section 10.

This rung does not execute or compile Linux-side Studio/typeface/Bevy/windowing/GPU binaries to justify deletion. Under ci_screening_surface.md §10, deletion is proven by coverage map, survivor compile floor, and owner/local deletion findings. Platform-unavailable or non-compiling deleted binaries are stronger delete signals, not preservation reasons.

## Scope

- Reviewed current `simthing-tools` typeface LR3/LR4/LR5/LR6/LR6A/LR6B/LR6C/LR6D/LR7/LR9 inventory rows.
- Reviewed current `simthing-mapeditor` typeface LR8 rows.
- Reviewed Studio rows surfaced by the prior audit, including active Terran-Pirate and Studio admission-adjacent rows.
- Did not touch Admission Substrate (`crates/simthing-spec/tests/**`, `crates/simthing-clausething/tests/**`), SimThing-Kernel, CI profiles, scanners, allowlists, workflows, `.github/**`, or `docs/sanctioned_surface.md`.
- Did not delete source tests because no live row met all DELETE predicates.

## Owner §10 Deletion Doctrine

Deletion proof for this track requires:

1. Coverage map: the deleted test's claimed surface is owned by a surviving representative.
2. Compile floor: survivors still compile; deleted binaries do not need to compile or run.
3. Owner/local deletion model: platform-unavailable or non-compiling owner-deep binaries are delete signals, not reasons to probe Linux desktop/GPU dependencies.

This rung found KEEP or ESCALATE rows only. No forbidden Linux Studio/typeface proof was used to promote an ambiguous row into DELETE.

## Candidate Discovery

Read and cross-checked:

- `docs/ci_screening_surface.md`
- `docs/design_0_0_8_4_6_ci_scaffolding.md`
- `docs/tests/test_pare_lingering_owner_deep_dead_tests_0_results.md`
- `docs/tests/test_pare_lingering_owner_deep_dead_tests_0_review.tsv`
- `docs/tests/test_pare_clausething_studio_dead_binaries_0_results.md`
- `docs/tests/test_pare_clausething_studio_dead_binaries_0_review.tsv`
- `docs/tests/tests_compile_floor_non_bevy_0r2_results.md`
- `docs/tests/current_evidence_index.md`
- `scripts/ci/test_inventory.tsv`
- `scripts/ci/test_pare_boundary_rows.tsv`
- `scripts/ci/test_pare_audit.tsv`

Current inventory has 4,240 rows. The owner-deep Studio/typeface review set included:

- `simthing-tools` typeface LR rows: 180 current rows.
- `simthing-mapeditor/tests/typeface_lr8.rs`: 27 current rows.
- Active Studio/Terran-Pirate and Studio admission-adjacent rows surfaced by the previous audit.

The important ledger finding is that the old `test_pare_audit.tsv` collapse plan for four live `typeface_lr7.rs` admission tests is stale: it points at `codepoint_outside_reserved_range_rejected`, which was already deleted by `TEST-PARE-GPU-BEVY-RESIDUE-0`. The current boundary ledger supersedes that historical audit and marks the remaining LR7 admission rows as KEEP representatives.

## DELETE Rows

None.

## KEEP Rows

- `crates/simthing-tools/tests/typeface_lr3.rs`: 12 behavior-regression rows retained.
- `crates/simthing-tools/tests/typeface_lr4.rs`: 6 behavior-regression rows and 1 golden-byte row retained; prior admission collapse rows are historical PARED rows, not live candidates.
- `crates/simthing-tools/tests/typeface_lr5.rs`: 30 behavior-regression rows and 1 golden-byte row retained.
- `crates/simthing-tools/tests/typeface_lr6.rs`: 17 behavior-regression rows, 1 golden-byte row, and 1 selected unsupported-vocabulary admission representative retained.
- `crates/simthing-tools/tests/typeface_lr6a_icon_geometry.rs`: 11 behavior-regression rows and 1 golden-byte row retained; prior dynamic-SVG row is historical PARED residue.
- `crates/simthing-tools/tests/typeface_lr6b.rs`: 25 behavior-regression rows retained.
- `crates/simthing-tools/tests/typeface_lr6c.rs`: 21 behavior-regression rows retained.
- `crates/simthing-tools/tests/typeface_lr6d.rs`: 26 behavior-regression rows retained.
- `crates/simthing-tools/tests/typeface_lr7.rs`: 9 behavior-regression rows, 1 golden-byte row, and 4 selected admission representatives retained.
- `crates/simthing-tools/tests/typeface_lr9.rs`: 13 behavior-regression rows retained.
- `crates/simthing-mapeditor/tests/typeface_lr8.rs`: 27 behavior-regression rows retained.
- `crates/simthing-mapeditor/tests/canonical_scenario_load_save_display.rs` / `studio_legacy_terran_pirate_still_loads_as_legacy_fixture`: retained as `B-T7-ACTIVE-TP-LIVE-RUNG / NEVER_PARE`.

## ESCALATE / Follow-On Rows

- Studio/mapeditor admission singleton rows remain owner/DA review material; coverage ownership is too ambiguous for deletion without a focused Studio admission ruling.
- `STUDIO-TYPEFACE-STAR-NAMEPLATES-0` remains in PROBATION with owner visual sign-off pending; that is a product/visual legitimacy question, not deletion authority for this rung.
- Any future deletion of typeface golden-byte, oracle, visual legitimacy, or selected admission representatives requires DA judgment or a new boundary-specific handoff.

## Coverage Map

| Candidate surface | Current owner |
|---|---|
| `typeface_lr4` historical dynamic/admission rows | Already deleted; LR7 `invalid_or_dynamic_svg_rejected` and `path_escape_rejected` remain live representatives. |
| `typeface_lr6a_icon_geometry` historical dynamic-SVG row | Already deleted; LR7 `path_escape_rejected` remains live representative. |
| `typeface_lr7` duplicate-name/parser/missing/path admission rows | Current boundary ledger marks each remaining row as KEEP representative. |
| `typeface_lr3` through `typeface_lr9` behavior rows | `TYPEFACE-TRACK-CLOSEOUT-0` plus row-level `B-T5-BEHAVIOR-REGRESSION` ownership. |
| Typefaces with golden-byte rows | `B-T7-GOLDEN-BYTE-DETERMINISM / NEVER_PARE`. |
| `typeface_lr8.rs` Studio/mapeditor behavior rows | `B-T5-BEHAVIOR-REGRESSION`; runtime proof is owner-deep only. |
| Active Terran-Pirate Studio legacy row | `B-T7-ACTIVE-TP-LIVE-RUNG / NEVER_PARE`. |

## Proof

- Doctrine Scan: PASS (`DOCTRINE-SCAN-VERDICT: PASS failures=0 inspect=0 selftest=SKIPPED`).
- Digest: PASS (`gen_digest --check: PASS`).
- Inventory check: PASS (`TEST-INVENTORY-CHECK-VERDICT: PASS`; rows/discovered 4,240).
- Boundary check: PASS (`TEST-PARE-BOUNDARY-CHECK-VERDICT: PASS`; live rows with owning boundary 4,240; historical PARED rows mapped 1,505).
- Drift check: PASS (`TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS`; unledgered 0, stale 0).
- Survivor compile floor: PASS.
  - `cargo check -p simthing-core --tests`
  - `cargo check -p simthing-kernel --tests`
  - `cargo check -p simthing-sim --tests`
  - `cargo check -p simthing-workshop --tests`
  - `cargo check -p simthing-mapgenerator --tests`
- `git diff --check origin/master...HEAD`: PASS.

## Forbidden Proof Avoided

- No `cargo check -p simthing-tools --tests`.
- No `cargo check -p simthing-mapeditor --tests`.
- No `cargo test -p simthing-tools`.
- No `cargo test -p simthing-mapeditor`.
- No typeface runtime, mapeditor runtime, Studio runtime, Bevy, winit, wgpu, ALSA/libasound, X/X11/Xvfb, Wayland, Mesa/Vulkan, libudev/udev, xkbcommon/xcb/EGL/GLX, or `apt-get`.
- No smuggling through `-p`, `--package`, `crate_checks`, `tests`, `doc_tests`, or `workflow_dispatch`.

## Inventory Delta

- Before: 4,240 rows.
- After: 4,240 rows.
- Delta: 0 rows.

## Graduation Routing

```text
Graduation routing:
  Status: HOLD / DA REVIEW
  Risk class: owner-deep deletion wave / Studio-typeface residue
  Protected corpus touched: no
  Active TP / NEVER_PARE touched: no
  CI profile/gate/scanner/allowlist/workflow touched: no
  Source tests deleted: no
  DA question: yes - current boundary rows leave no live DELETE candidates; stale audit-vs-boundary mismatch and Studio/typeface legitimacy require DA/orchestrator triage before merge.
```
