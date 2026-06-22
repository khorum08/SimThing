# TYPEFACE-LR8-STUDIO-LABEL-SEAM-0 Results

## Status

PASS — Studio/game label seam wires `StudioTypefaceLabel` into existing typeface runtime components (`TextLabel`, `NumericDamageLabel`) with fixture manifest icon resolution at import/staging time. **PROBATION / integration seam** — not DA-approved.

## PR / branch / merge

- Branch: `typeface-lr8-studio-label-seam-0`
- PR: #893
- Merge SHA: `ec01da43c9`

## LR7 closeout

- `TYPEFACE-LR7-ICON-FONT-MANIFEST-0` — **DA APPROVED for manifest machinery**
- PR #892, merge `ac320204eb`, post-merge evidence `be8dde2388`
- Production icon source set remains input debt (fixture icons only)

## Studio/game label seam

- `crates/simthing-mapeditor/src/app/labels.rs` re-exports the Studio adapter
- `crates/simthing-tools/src/studio_labels.rs` owns data model, diagnostics, manifest bake
- `SimthingToolsTextPlugin` syncs `StudioTypefaceLabel` → typeface components before label rebuild
- No bespoke text renderer; no `bevy_text` fallback on the proved path

## Label data model

- `StudioLabelKind`: EntityName, RegionName, DamageText, DebugProbe
- `StudioTypefaceLabel`: text, kind, px, color, style_slot, render_mode, icon_name, deform/path/warp slots
- `StudioDamageTextEmitter`: queues transient numeric damage values for spawn into damage labels

## Manifest/icon integration

- `StudioTypefaceLabelPlugin` bakes fixture manifest once at PostStartup into `TypefaceIconSet`
- `icon_name` resolves to PUA via `IconManifestBake` at label sync (on change only)
- Mixed text+icon labels use `IconSet::build_mixed_instances` in rebuild when manifest codepoints present
- `manifest_reload_count` increments once at bake; no per-frame manifest/SVG IO

## Bevy/system behavior

- `sync_studio_typeface_labels` — Added/Changed `StudioTypefaceLabel` → `TextLabel` or `NumericDamageLabel`
- `emit_studio_damage_text_labels` — drains `StudioDamageTextEmitter` into damage labels
- `rebuild_changed_labels` — icon-aware mixed run when `TypefaceIconSet` present
- Style/render/deform/path/warp remain data slots on `TextLabel`

## No-op / update behavior

- Unchanged `StudioTypefaceLabel` does not re-sync; unchanged `TextLabel` does not reshape/rerasterize
- Text change triggers one bounded sync + one rebuild (proved via `TextPerfDiagnostics`)

## Damage/transient label path

- `StudioTypefaceLabel::damage_value` / `StudioDamageTextEmitter` → `NumericDamageLabel` fixed-width lane
- Uses existing LR5 numeric damage path (shaping bypass after init)

## GPU residency / CPU surfacing audit

Import/staging only — manifest bake and icon name resolution happen on PostStartup or label change, not in draw loops.

- **Allowed CPU:** Studio label spawn/update orchestration; manifest bake once; icon name → PUA resolve on label change; diagnostics; tests
- **Forbidden:** per-frame manifest reload; per-frame SVG parse; per-frame reshaping/rasterization of unchanged labels; bespoke CPU text renderer fallback
- **GPU owns:** atlas/MSDF sampling, style/effect composition, deformation, path/warp evaluation, instanced draw
- Deviations: none

## Tests

`crates/simthing-mapeditor/tests/typeface_lr8.rs` — 18 tests including LR7 closeout doc check, spawn/sync, style/render mode, noop/update, damage emitter, manifest icon resolve/mixed label, manifest reload guard, LR7/LR6D regressions, semantic-free guard, GPU residency doc check.

## Validation

```text
cargo fmt -p simthing-tools -p simthing-workshop -p simthing-mapeditor -- --check
cargo check -p simthing-tools
cargo check -p simthing-workshop
cargo check -p simthing-mapeditor
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
cargo test -p simthing-mapeditor --test typeface_lr8
git diff --check
```

## Files changed

- `crates/simthing-tools/src/studio_labels.rs` (new)
- `crates/simthing-tools/src/bevy.rs` — studio sync + icon-aware rebuild
- `crates/simthing-tools/src/lib.rs`
- `crates/simthing-mapeditor/src/app/labels.rs` (new)
- `crates/simthing-mapeditor/src/app/mod.rs`, `Cargo.toml`
- `crates/simthing-mapeditor/tests/typeface_lr8.rs` (new)
- Docs: ladder, evidence index, production log, LR7 closeout, this file

## Boundary / non-goals

- No LR9 final perf gate or track closure
- No TTF/OTF export, COLRv1, variable fonts
- No production icon source set invention
- No ScenarioSpec/RF/STEAD/sim changes
- Studio full UI wiring deferred — seam API + headless proofs only

## Known gaps

- Studio `run_studio()` does not yet mount `StudioTypefaceLabelPlugin` in production shell (compile-time seam ready)
- Production icon source set still input debt
- World-space camera-distance scaling for entity labels deferred

## DA recommendation

Recommend **PROBATION** retention for integration seam. Do **not** self-approve LR8 or the whole typeface track.

## Next recommended action

Codex review of Studio seam proofs; mount plugin in Studio when presentation wiring is scheduled; LR9 perf gate when track owner selects it.
