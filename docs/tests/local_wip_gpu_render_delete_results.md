# LOCAL-WIP-GPU-RENDER-DELETE-0 Results

## Status

PASS — orphaned untracked local spike removed; no committed code references; no runtime Studio behavior change.

## PR / branch / merge

- Branch: `local-wip-gpu-render-delete-0`
- PR: #902
- Merge SHA: `6d1edf8257`

## Deleted file

| Path | Git status before | Action |
|---|---|---|
| `crates/simthing-mapeditor/src/app/gpu_render.rs` | untracked (`??`) | deleted from working tree |

File was ~6 KB; created ~2026-06-20 during the Studio GPU/performance investigation. It had **no commit history** and was never tracked.

## Why it was safe to delete

- **Not wired:** `crates/simthing-mapeditor/src/app/mod.rs` does not declare `mod gpu_render`; shipped Studio uses `DefaultPlugins`.
- **Would not compile if wired:** the spike imported `pollster` and `wgpu` directly, but `simthing-mapeditor/Cargo.toml` does not declare those dependencies.
- **No evidence track / PR:** no row in the formal map-editor ladder; no `STUDIO-GPU-ADAPTER-*` handoff.
- **No runtime effect:** the committed Studio binary never included this file.
- **Policy mismatch:** the spike hard-panicked on integrated-only adapters; committed `simthing-gpu` prefers discrete but allows fallback. The formal FPS audit (#860) classified the Studio regression as CPU/egui-bound, not GPU-adapter-bound.
- **Not a feature conversion:** this task deletes local WIP only; it does not open a discrete-GPU adapter track.

## Reference checks

```text
grep -R -n "gpu_render" crates docs Cargo.toml
```

Results:

- **crates/** — no matches (file removed; never referenced from committed Rust).
- **Cargo.toml** — no matches.
- **docs/** — historical mentions only in prior cleanup reports (`workspace_cleanup_inventory_0_results.md`, `local_space_recovery_results.md`) documenting that the file existed as untracked WIP. No live doc links to the file path as a required artifact.

Guard (no runtime wiring):

```text
grep -R -n "mod gpu_render|studio_render_plugin|studio_discrete_render_creation|select_studio_discrete_adapter" crates docs Cargo.toml
```

No matches.

## Validation

```text
git status --short          — clean except new results doc + evidence index (after commit)
git diff --check            — PASS (docs-only PR)
cargo check -p simthing-mapeditor   — PASS
cargo test -p simthing-tools --test semantic_free_guard   — PASS
```

No committed Rust diff from the deletion itself (file was never tracked). PR contains documentation only.

## Remaining GPU-adapter policy

- **Studio (Bevy):** `DefaultPlugins` + `StudioGpuIdentityInitPlugin` reports adapter identity in Performance Telemetry; does not force discrete selection.
- **simthing-gpu tests/runtime:** `GpuContext::new()` enumerates adapters and prefers the first `DiscreteGpu`, with high-performance fallback when none exists (`crates/simthing-gpu/src/context.rs`).
- **Optional local override:** diagnostic notes in `studio_windows_diagnostic_debug_exe_0_results.md` mention `WGPU_BACKEND` / `WGPU_POWER_PREF` env vars for manual adapter experiments — not shipped policy.

## Next action

None required for this cleanup track. If discrete-first Bevy render init is ever desired, open a formal `STUDIO-DISCRETE-GPU-ADAPTER-*` track with design review, fallback policy, dependencies, tests, and evidence — do not revive the deleted spike verbatim.
