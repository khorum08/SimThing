# TYPEFACE-LR8-STUDIO-PLUGIN-MOUNT-0R Results

## Status

PASS — Studio/mapeditor app shell mounts `StudioTypefaceShellPlugin` (fixture font + `StudioTypefaceLabelPlugin` + probe staging) via the same path as `run_studio()`. **PROBATION / DA remediation** — closes LR8 shell-mount gap; not DA-approved.

## PR / branch / merge

- Branch: `typeface-lr8-studio-plugin-mount-0r`
- PR: (pending)
- Merge SHA: (pending)

## Remediation target

- `TYPEFACE-LR8-STUDIO-LABEL-SEAM-0` (#893, merge `ec01da43c9`, post-merge evidence `22e7bfb9a4`) — **PROBATION / DA HOLD** on Studio shell mount gap; remediated by this rung.

## Studio shell mount

- `crates/simthing-mapeditor/src/studio_typeface_shell.rs` — `StudioTypefaceShellPlugin`, `mount_studio_typeface_plugins`
- `crates/simthing-mapeditor/src/app/mod.rs` — `run_studio()` calls `mount_studio_typeface_plugins` after core plugins
- Bundles `SimthingToolsTextPlugin` (fixture `test_font.ttf`) + `StudioTypefaceLabelPlugin`
- Startup stages probe `StudioTypefaceLabel::entity_name("Studio", …)` and `StudioDamageTextEmitter`
- `StudioTypefaceShellMounted` resource set once fixture manifest bake is visible

## Manifest bake in app path

- Fixture manifest bakes once at PostStartup via existing `StudioTypefaceLabelPlugin` (`manifest_reload_count == 1`)
- No per-frame manifest reload or runtime SVG parse on shell path

## Entity/world label app proof

- Shell probe spawns at least one `StudioTypefaceLabel` with synced `TextLabel`
- Shell tests spawn additional labels and resolve manifest icon names to PUA codepoints

## Damage/transient app proof

- Shell probe spawns `StudioDamageTextEmitter`; tests emit numeric values → `NumericDamageLabel` via existing typeface path

## No-op / update behavior

- Unchanged shell-probe labels do not trigger extra shape/instance rebuilds across idle frames (`TextPerfDiagnostics` stable)

## GPU residency / CPU surfacing audit

Import/staging only — manifest bake and icon name resolution happen on PostStartup or label change, not in draw loops. Same LR8 seam doctrine applies to shell mount.

- **Allowed CPU:** plugin mount; one-time fixture manifest bake (import/staging); label spawn/sync on change; damage emitter drain; diagnostics; headless/shell tests
- **Forbidden:** per-frame manifest reload; per-frame SVG parse; per-frame reshaping of unchanged labels; bespoke CPU text renderer fallback
- **GPU owns:** atlas/MSDF sampling, style/effect composition, deformation, path/warp evaluation, instanced draw
- Deviations: none

## Tests

`crates/simthing-mapeditor/tests/typeface_lr8.rs` — extended with shell-mount subset:

- `studio_app_shell_mounts_typeface_label_plugin`
- `studio_app_shell_bakes_fixture_manifest_once`
- `studio_app_shell_can_spawn_typeface_label`
- `studio_app_shell_can_resolve_manifest_icon_label`
- `studio_app_shell_can_emit_damage_text`
- `studio_app_shell_noop_does_not_rebuild_or_reshape`
- `studio_app_shell_no_runtime_svg_or_manifest_reload`
- `lr8_headless_tests_still_pass`
- `gpu_residency_audit_documented_for_lr8_plugin_mount`

Headless LR8 seam tests use `lr8_headless_app()` (plugin stack only). Shell tests use `studio_app_shell()` — same registration path as `run_studio()` without `DefaultPlugins`/window.

**Untested in CI:** full interactive Studio window smoke (DefaultPlugins + egui + galaxy render). Shell mount is proved via shared plugin registration + headless frame updates.

## Validation

```text
cargo fmt -p simthing-tools -p simthing-workshop -p simthing-mapeditor -- --check
cargo check -p simthing-tools
cargo check -p simthing-workshop
cargo check -p simthing-mapeditor
cargo test -p simthing-tools --test semantic_free_guard
cargo test -p simthing-tools --test typeface_lr7
cargo test -p simthing-tools --test typeface_lr6d
cargo test -p simthing-mapeditor --test typeface_lr8
git diff --check
```

## Files changed

- `crates/simthing-mapeditor/src/studio_typeface_shell.rs` (new)
- `crates/simthing-mapeditor/src/app/mod.rs` — mount in `run_studio()`
- `crates/simthing-mapeditor/src/app/labels.rs` — re-exports shell mount
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/tests/typeface_lr8.rs` — shell mount tests
- Docs: this file, ladder, evidence index, production log, LR8 results cross-link

## Boundary / non-goals

- No LR9 final perf gate or track closure
- No TTF/OTF export, COLRv1, production icon source set
- No ScenarioSpec/RF/STEAD/sim changes
- No full interactive Studio smoke in CI

## Known gaps

- Interactive Studio window smoke not automated in CI
- Production icon source set still input debt
- World-space camera-distance scaling for entity labels deferred
- LR9 perf gate blocked until DA review

## DA recommendation

Recommend **PROBATION** retention for shell mount remediation. LR8 seam + shell mount proved headless; **do not** self-approve LR8 or close the typeface track. LR9 remains blocked.

## Next recommended action

Codex review of shell mount proofs; schedule interactive Studio smoke when presentation wiring expands; LR9 when track owner selects perf gate.
