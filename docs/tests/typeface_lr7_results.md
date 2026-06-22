# TYPEFACE-LR7-ICON-FONT-MANIFEST-0 Results

## Status

PASS — declarative RON icon manifest loads, validates, and bakes fixture icons into `IconSet` + shared raster atlas with stable name ↔ PUA codepoint tables. **PROBATION / mechanical** — not DA-approved. LR7 manifest machinery complete with fixture icon set; production icon set remains input debt.

## PR / branch / merge

- Branch: `typeface-lr7-icon-font-manifest-0`
- PR: #892
- Merge SHA: `ac320204eb`

## LR6D closeout

- `TYPEFACE-LR6D-TEXT-ON-PATH-WARP-FIELD-0` — **DA APPROVED** after #891
- `TYPEFACE-LR6D-COMBINED-MSDF-DEFORM-PROOF-0R` — **ACCEPTED / closed**
- PR #891, merge `ffc4bb6891`, post-merge evidence `6a32763bdd`

## Source set / reserved range

Fixture set only (LR7 mechanical scope):

- `0xF0001` — `test.background-accent` → `background_accent.svg`
- `0xF0002` — `test.outline-accent` → `outline_accent.svg`
- Reserved range: `0xF0000..=0xF00FF` (256 PUA slots)

Production game icon source set not supplied; manifest format and bake path are ready for drop-in replacement of fixture entries.

## Manifest format

RON file beside static SVG assets:

```ron
(
    reserved_range_start: 0xF0000,
    reserved_range_end: 0xF00FF,
    icons: [
        (name: "test.background-accent", codepoint: 0xF0001, svg_path: "background_accent.svg"),
        (name: "test.outline-accent", codepoint: 0xF0002, svg_path: "outline_accent.svg"),
    ],
)
```

Validation: unique names/codepoints, codepoints inside reserved range, relative paths only, no directory escape, static SVG only, deterministic sort by codepoint at bake.

## Public API

- `IconManifest`, `IconManifestEntry`, `IconManifestBake`
- `load_icon_manifest(path) -> Result<IconManifest, TypefaceError>`
- `bake_icon_manifest(manifest_path, icons, raster_atlas, px) -> Result<IconManifestBake, TypefaceError>`
- `fixture_manifest_path()` for hermetic tests

## Bake behavior

Import/staging only:

1. Parse RON manifest
2. Validate reserved range, uniqueness, path safety, SVG file presence
3. Read SVG bytes from disk once
4. `IconSet::register_svg` → `IconVector` geometry + role layers + raster tile in shared atlas
5. Return stable `BTreeMap` name/codepoint tables and ordered `baked_codepoints`

No per-frame SVG parsing. No TTF/OTF export (deferred LR7A).

## Stable name/codepoint table

Golden (deterministic):

```text
F0001 test.background-accent
F0002 test.outline-accent
```

## Static SVG security

Unchanged LR4/LR6A-ICON policy: manifest bake calls `IconVector::from_svg` / `register_svg`, rejecting dynamic scripts, external refs, and non-static content. Path resolution rejects `..`, absolute paths, and paths outside the manifest directory.

## IconVector / role-layer preservation

Fixture icons preserve LR6A-ICON role tags:

- `test.background-accent`: Background + Accent layers
- `test.outline-accent`: Outline + Accent layers

Per-role style-slot refs remain available via `IconSet::style_layers_for`.

## Raster fallback / MSDF disposition

Manifest loads/bakes raster tiles and `IconVector` geometry now. MSDF generation remains available through existing `DistanceFieldAtlasCore::get_or_generate_icon_msdf` API; dedicated `bake_icon_manifest_msdf` deferred — not required for LR7 manifest stability proof.

## GPU residency / CPU surfacing audit

Import/staging only — manifest load and SVG read happen at bake time, not in the draw loop. Scope: import/staging.

- **Allowed CPU:** manifest load/validate at import or staging; one-time SVG file read; `IconSet::register_svg` rasterization; stable table construction; diagnostics/tests
- **Forbidden:** per-frame manifest reload; per-frame SVG parse; runtime SVG path resolution during draw/update loops
- **GPU owns:** instanced draw of baked icon/glyph tiles; MSDF/style/deform/path/warp when opted in on instances
- Manifest tables live on CPU as lookup metadata only; atlas tiles uploaded through existing atlas residency path
- Deviations: none

## Tests

`crates/simthing-tools/tests/typeface_lr7.rs`:

- `lr6d_closeout_records_da_approval`
- `manifest_loads_fixture_icons`
- `manifest_bakes_all_icons`
- `codepoint_table_is_stable_golden`
- `duplicate_codepoint_rejected`
- `duplicate_name_rejected`
- `codepoint_outside_reserved_range_rejected`
- `missing_svg_path_errors`
- `path_escape_rejected`
- `invalid_or_dynamic_svg_rejected`
- `manifest_icons_preserve_iconvector_geometry`
- `manifest_icons_preserve_role_layers`
- `manifest_icons_can_render_mixed_text_icon_run`
- `manifest_bake_has_no_runtime_svg_dependency`
- `semantic_free_guard_still_passes`
- `gpu_residency_audit_documented_for_lr7`

## Validation

```text
cargo fmt -p simthing-tools -p simthing-workshop -- --check
cargo check -p simthing-tools
cargo check -p simthing-workshop
cargo test -p simthing-workshop --test typeface_lr0
cargo test -p simthing-workshop --test typeface_lr1
cargo test -p simthing-workshop --test typeface_lr2
cargo test -p simthing-tools --test typeface_lr3
cargo test -p simthing-tools --test semantic_free_guard
cargo test -p simthing-tools --test typeface_lr4
cargo test -p simthing-tools --test typeface_lr5
cargo test -p simthing-tools --test typeface_lr6
cargo test -p simthing-tools --test typeface_lr6a_icon_geometry
cargo test -p simthing-tools --test typeface_lr6b
cargo test -p simthing-tools --test typeface_lr6c
cargo test -p simthing-tools --test typeface_lr6d
cargo test -p simthing-tools --test typeface_lr7
git diff --check
```

## Files changed

- `crates/simthing-tools/src/manifest.rs` (new)
- `crates/simthing-tools/assets/typeface/icons/manifest.ron` + fixture SVGs (new)
- `crates/simthing-tools/src/lib.rs`, `font.rs`, `Cargo.toml`
- `crates/simthing-tools/tests/typeface_lr7.rs` (new)
- Docs: ladder, evidence index, production log, LR6D closeout, this file
- `Cargo.lock`, `THIRD_PARTY_LICENSES.md`

## Boundary / non-goals

- No `.ttf`/`.otf` exporter, COLRv1, variable font export (LR7A deferred)
- No Studio/game label seam (LR8)
- No ScenarioSpec/RF/STEAD changes
- No production game icon lore/invention

## Known gaps

- Production icon source set not supplied — fixture-only manifest
- `bake_icon_manifest_msdf` not added; use existing icon MSDF API when needed
- TTF/OTF export remains optional LR7A

## DA recommendation

Recommend **PROBATION** retention for mechanical manifest rung. Do **not** self-approve LR7 or the whole typeface track.

## Next recommended action

Codex review of LR7 manifest stability and fixture bake proofs; supply production PUA icon source set when ready; LR8 Studio label seam when typeface track owner selects it.
