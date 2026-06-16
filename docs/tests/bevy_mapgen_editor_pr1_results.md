# BEVY-MAPGEN-EDITOR-PR1 — Windows SimThing Studio shell + 3D galaxy viewer

**Classification: PROBATION until DA approval**

## Artifact lifecycle audit

| Artifact | Classification | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Guardrail row + PROBATION amendment |
| `docs/tests/mapgenerator_cli_editor_prep_0r_results.md` | PROBATION | Producer contract dependency |
| `crates/simthing-mapeditor/` | PROBATION | New Bevy studio shell |
| `docs/tests/bevy_mapgen_editor_pr1_results.md` | PROBATION | This report |

## Old vs new

No prior Bevy editor existed. This PR introduces the first SimThing Studio shell.

## Window / UI behavior

- **Windows-only** binary `simthing-studio` (`crates/simthing-mapeditor`).
- Borderless black window (no OS title bar); custom top-right controls: minimize, borderless fullscreen, exclusive fullscreen (experimental `WindowMode::Fullscreen`), close.
- Left panel: 20–50% width, `<<`/`>>` collapse, opacity 50% idle / 80% hovered (eased).
- Panel title empty before generation; after generation shows e.g. `Unnamed 2-Armed Spiral`.
- New/Load/Save visible but disabled; click opens warning dialog.
- Generate runs typed `run_generation()` → `simthing-mapgenerator` library (no stdout scrape).
- Right status panel appears after generation with JSON report stats.
- Default preset: Spiral 2 Dense 3000 (matches healthy editor-prep params).

## Generator integration path

```text
GenerationProfile → MapGeneratorParams → generate_galaxy_with_structure
  → GalaxyGenerationResult + build_generation_report
  → StudioSession + StudioGalaxyViewModel
  → Bevy 3D render (presentation only)
```

Structural `(col,row)` from `PlacedSystemSeed.coord` remain authoritative. World positions and Y-height are render-only (`StudioGalaxyRenderMeta`).

## Trusted data path

- Internal: typed Rust structs (`StudioSession`, `GenerationReport`, `GalaxyGenerationResult`).
- Settings: RON at `%APPDATA%/SimThing/Studio/settings.ron` (local `settings.ron` fallback if APPDATA missing).
- JSON report consumed via `build_generation_report` in-process (not scraped from CLI stdout).

## Render metatable

`StudioGalaxyRenderMeta` controls galactic thickness, star scale, hyperlane alpha/depth fade. Height uses deterministic hash × amplitude lerped from core to edge — **never written back to scenario**.

Stars: emissive unlit spheres (interim; starburst sprites deferred). Hyperlanes: independent `LineList` segments with camera-distance alpha fade.

## UI substrate note

PR1 uses Bevy-native/egui panels. HTML/CSS/WebView is deferred. Clausewitz UI import remains a horizon goal and must not compromise the SimThing/STEAD authority boundary.

## Tests run

```text
cargo test -p simthing-mapeditor
```

9 unit tests (settings roundtrip, default profile, view model structural/render separation, quality panel, warning dialog).

## Commands run

```text
cargo fmt --all
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
git diff --check
```

Launch (Windows):

```text
cargo run -p simthing-mapeditor --bin simthing-studio
```

## Files changed

- `Cargo.toml` — workspace member
- `crates/simthing-mapeditor/` — studio crate (lib + binary)
- `docs/clausething/MapGeneratorCLI.md` — Studio consumer section
- `docs/design_0_0_8_0_consumer_pulled_production_track.md` — Studio horizon row
- `docs/tests/current_evidence_index.md` — guardrail + PROBATION row
- `docs/tests/bevy_mapgen_editor_pr1_results.md` — this report

## Known deferred features

- New/Load/Save session (RON/JSON)
- Star/hyperlane selection and inspection (PR2)
- Live SimThing simulation in Studio
- Clausewitz UI import experiment
- Starburst sprite atlas (using emissive spheres interim)
- Non-Windows platforms

## Merge

- PR [#725](https://github.com/khorum08/SimThing/pull/725) — commit `014fbeec`, merge `dd2d3af3`

## DA status

**PROBATION** — pending owner/design-authority approval. Do not pre-file DA approval.
