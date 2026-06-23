# ADR — simthing-tools TypeFace Runtime

> **Status: ACCEPTED / CLOSED / DA-APPROVED.** Consolidates the closed TYPEFACE-LADDER production
> track into one durable root reference. The old ladder and proposal are archived under
> `docs/archive/typeface_track_2026_06/`.

---

## Decision

`crates/simthing-tools` owns the presentation-only typeface runtime for SimThing: TTF/OTF font loading,
shaping, raster atlas, MSDF/SDF atlas, SVG icon ingestion, icon manifest baking, GPU style rows,
deformation/path/warp tables, Studio label seam, and LR9 performance harnesses.

It is a support crate, not simulation authority.

---

## Why this exists

Studio and future presentation layers need labels, icons, warped nameplates, status/progress plates,
damage text, and map text that are GPU-resident, deterministic, semantic-free at shader level, and not
part of simulation authority.

The closed track proved this through LR0–LR9 plus remediations, ending with LR9 binding evidence and the
closeout perf-invariant remediation (TYPEFACE-CLOSEOUT-PERF-INVARIANT-0, PR #897; TYPEFACE-TRACK-CLOSEOUT-0,
PR #898).

---

## Authority boundary

- CPU may import, normalize, shape, pack, bake, cache, and stage presentation artifacts at import/change time.
- GPU owns sampling, fill/outline/glow/gradient effects, deformation, path/warp, and instanced draw composition.
- SVG is import-time source only; runtime consumes baked icon vector/raster/MSDF data.
- TTF/OTF is import/change-time source only; runtime consumes shaped glyph IDs, atlas tiles, instance rows, and table slots.
- `simthing-tools` must not mutate Scenario/RF/spatial authority.
- `simthing-tools` must not introduce gameplay semantics into WGSL.
- Dynamic label effects use style/deform/path/warp table slots, not CPU reshape or outline regeneration.

---

## Public API seams

Export surface from `crates/simthing-tools/src/lib.rs`. See the source for the complete list.

### Font / shaping
- `load_font`, `ProbeFont`, `GlyphMetrics`
- `ShapingEngine`, `ShapedRun`, `ShapedGlyph`

### SVG icons and manifest
- `IconSet`, `IconRegistration`, `IconVector`, `IconVectorLayer`
- `IconManifest`, `IconManifestEntry`, `IconManifestBake`
- `load_icon_manifest`, `bake_icon_manifest`, `fixture_manifest_path`
- `ICON_PUA_START`

### Atlas / MSDF / GPU instances
- `GlyphAtlasCore`, `DistanceFieldAtlasCore`
- `GlyphInstanceGpu`, `TextLabel`, `TextLabelRenderMode`
- `SimthingToolsTextPlugin`, `TonemappingLutFixPlugin`

### Style / deformation / path / warp
- `TextStyleTable`, `TextStyleRow` — `style_params_for_slot`
- `TextDeformTable`, `TextDeformParams` — `deform_params_for_slot`
- `TextPathTable` — `path_params_for_slot`
- `TextWarpTable` — `warp_params_for_slot`

### Studio seam
- `StudioTypefaceLabel`, `StudioDamageTextEmitter`
- `StudioTypefaceLabelPlugin`, `TypefaceIconSet`

### LR9 perf harnesses
- `Lr9Config`, `LR9_CI_CONFIG`, `LR9_BINDING_CONFIG`
- `profile_flat_animated_labels`, `profile_numeric_damage_lane`
- `profile_dynamic_style_labels`, `profile_warped_nameplates`
- `profile_studio_seam_labels`
- `collect_lr9_metrics`, `format_lr9_scenario_report`

### Numeric damage lane
- `NumericDamageLabel`, `NumericGlyphRunTable`
- `NUMERIC_DAMAGE_DEFAULT_WIDTH`

---

## Closed-track outcomes

| Rung | Description | PR | Status |
|---|---|---|---|
| LR0 | Font loading/metrics | #872 | ACCEPTED as part of TYPEFACE-TRACK-CLOSEOUT-0 |
| LR1 | Shaping engine | #873 | ACCEPTED as part of TYPEFACE-TRACK-CLOSEOUT-0 |
| LR2 | Raster atlas | #874 | DA APPROVED after LR2R |
| LR2R | GlyphAtlasCore CPU-only path | #875 | DA remediation accepted |
| LR3 | `simthing-tools` crate + instanced draw | #876 | DA APPROVED after LR3R |
| LR3R | Route B raw-wgpu shader smoke | #877 | DA remediation accepted |
| LR4 | SVG/PUA icon ingestion + IconVector IR | #878 | ACCEPTED / closed |
| LR5 | High-volume bench + numeric damage lane | #879–#882 | DA APPROVED after LR5R/LR5S/LR5T |
| LR6 | MSDF/SDF atlas + shader path | #883 | DA APPROVED for glyph MSDF after LR6A |
| LR6A | Production MSDF opt-in | #884 | ACCEPTED |
| LR6A-ICON | IconVector geometry bridge | #885 | ACCEPTED / closed |
| LR6B | GPU style table + effects + residency | #886–#887 | DA APPROVED |
| LR6C | Persistent atlas bind group + deformation | #888–#889 | DA APPROVED |
| LR6D | Path/warp tables + combined MSDF/deform proof | #890–#891 | DA APPROVED |
| LR7 | Icon manifest machinery | #892 | DA APPROVED |
| LR8 | Studio label seam + shell mount | #893–#894 | DA APPROVED |
| LR9 | Final perf gate + binding evidence | #895–#896 | DA APPROVED |
| TYPEFACE-CLOSEOUT-PERF-INVARIANT-0 | Perf invariant green | #897 | ACCEPTED / closed |
| TYPEFACE-TRACK-CLOSEOUT-0 | Executive-DA track closure | #898 | CLOSED / DA-APPROVED 2026-06-22 |
| TYPEFACE-CLEANUP-DOCS-ARCHIVE-0 | Docs consolidation | #900 | ACCEPTED / closed |

---

## GPU-residency guarantees

- Style table buffers persist across frames (bind-group reuse, rows write on `rows_generation` only).
- Atlas bind groups persist; `prepare_text_atlas_bind_group` skips on `atlas_id` match.
- MSDF/SDF sampling is shader-side (`sdf_params_for_distance_field_tile`).
- Deformation, path, and warp are vertex-shader side over static glyph geometry.
- Numeric damage no-op lane does not reshape settled labels (`changed_label_rebuild_does_not_clone_old_instance_vec`).
- Manifest reload count is one at startup; no runtime SVG/manifest reload.
- Semantic-free shader guard (`semantic_free_guard` test) passes.

---

## Supported use-cases

- Map/Studio star nameplates and entity labels
- Warped/bent/twisted nameplate styles (path + warp tables)
- Icons as glyphs in mixed text runs (PUA codepoints)
- Progress/status plates via animatable style slots
- Numeric damage text with fixed-width no-op budget
- Studio label queue/seam (`StudioTypefaceLabelPlugin`)

---

## Non-goals / deferred

- Production icon art/source set
- Bundled default game font decision
- Interactive Studio window FPS smoke (windowed mode)
- COLRv1 / TTF/OTF exporter
- World-space camera-distance label scaling
- Dirty-list/event-driven scan optimization beyond 5k

Non-blocking debts at closure: 5k noop O(N) scan spike ~1.0086 ms, damage churn ~2.5 ms, windowed Studio smoke.

---

## Evidence

- `docs/tests/current_evidence_index.md` — TYPEFACE rows (LR0–LR9, closeout)
- `docs/archive/typeface_track_2026_06/` — per-rung process reports

Closure commits:

```
TYPEFACE-LR9-FINAL-PERF-GATE-0          — PR #895 `c5b5faeab2`
TYPEFACE-LR9-BINDING-PERF-EVIDENCE-0R   — PR #896 `bda6147c95`
TYPEFACE-CLOSEOUT-PERF-INVARIANT-0      — PR #897 `974ffcc7fc`
TYPEFACE-TRACK-CLOSEOUT-0               — PR #898 `82416b9d27`
TYPEFACE-TRACK-CLOSEOUT-LEDGER-FIX-0   — PR #899 (ledger repair)
TYPEFACE-CLEANUP-DOCS-ARCHIVE-0         — PR #900 `eafa522856`
LOCAL-WORKTREE-SPACE-RECOVERY-0        — PR #901 `a9e6563af6`
```

---

## Decision rule for future work

Future typeface work must modify this ADR or create a successor ADR. Do not reopen the archived ladder as an
active design surface. If a feature changes runtime authority, GPU residency, style/deform/path semantics,
icon manifest policy, or Studio label seam behavior, it is a new DA-reviewed track.
