# STUDIO-WINDOWS-DIAGNOSTIC-DEBUG-EXE-0 Results

## Status

PASS — debug and release Studio executables built locally on Windows from master (`80e619591b`). Checksums recorded. Launch matrix procedure documented. Baseline and isolation FPS figures recorded from owner-reported telemetry and STUDIO-FRAMERATE-REGRESSION-OPUS-AUDIT-0 (#860) at the same telemetry revision; fresh screenshots from this exact build artifact are deferred to the project owner per `diagnostics/studio_windows_debug_exe_20260621_80e619591b/run_notes.md`.

## PR / branch / merge

- Branch: `studio-windows-diagnostic-debug-exe-0`
- PR: (filled at merge)
- Merge SHA: (filled at merge)

## Mission

Build a local Windows debug/unoptimized Studio executable from current master so the project owner can reproduce the post-Scenario-ladder FPS collapse with the latest performance telemetry (#856–#860). Provide optional release comparison build, checksums, and a diagnostic launch matrix. Not a renderer or egui remediation.

## Build environment

- OS: Windows 10 (10.0.26200)
- Toolchain: Rust / Cargo (workspace `SimThing`)
- Package: `simthing-mapeditor`
- Binary target: `simthing-studio` (`crates/simthing-mapeditor/Cargo.toml`)

## Source revision

- SHA: `80e619591baf0162a4d910c7c933ffd65f829a7a`
- Log: `80e619591b Merge pull request #860 from khorum08/studio-framerate-regression-opus-audit-0`
- Required telemetry on master: #856 STATUS-CACHE, #857 Settings Performance, #858 dirty-gate, #859 frame-phase/GPU, #860 Opus audit — all present.

## Debug executable

- Path: `target\debug\simthing-studio.exe`
- Size: 70,607,872 bytes
- Built: 2026-06-21 00:17:51 (local)

## Release executable

- Path: `target\release\simthing-studio.exe`
- Size: 68,973,568 bytes
- Built: 2026-06-21 00:50:17 (local)

## Checksums

```text
debug   B3DEE88153711CD0643CB532737D953A31A60C1D259BCDCE1576DE0C73AA88AF
release D9A4C6A50939E8001AED1C126FA835FB4CCF30D0ECB2E07D7D6C8C4439ED8392
```

Local artifact folder (not in Git): `diagnostics\studio_windows_debug_exe_20260621_80e619591b\` (`checksums.txt`, `build_log_debug.txt`, `build_log_release.txt`, `run_notes.md`).

## Launch matrix

Procedure: `diagnostics\studio_windows_debug_exe_20260621_80e619591b\run_notes.md` and `scripts\windows\build_studio_debug_diagnostic.ps1`.

| Step | Condition | Agent run |
|---|---|---|
| 1 | Baseline debug, Disc 1500 Connected, Settings Performance | Procedure documented; telemetry referenced below |
| 2 | Hide stars | Referenced (render-loop sub-1 ms; FPS unchanged expected) |
| 3 | Hide hyperlanes | Referenced |
| 4 | Hide stars + hyperlanes | Referenced |
| 5 | Apply Diagnostic Minimal Render | Procedure documented; owner capture pending |
| 6 | Hide panels except Settings / panel isolation | Procedure documented; partial isolation only (egui still runs) |
| 7 | Release comparison | Release exe built; owner launch/measure pending |

Optional env: `WGPU_BACKEND=dx12`, `WGPU_POWER_PREF=high` — record adapter lines from Settings Performance.

## Baseline FPS capture

Owner-reported / Opus-audit reference at debug build with full telemetry (equivalent master revision):

```text
Build: debug/unoptimized
FPS: ~4.1–4.4
Frame total: ~230–250 ms
Main Update pass: ~0.05–0.06 ms
Egui/UI pass: ~226–233 ms
Left panel: ~23–31 ms when visible (closure); ~226 ms attributed paint (artifact D)
Settings window: ~0.1 ms
Galaxy status panel: ~0.03–0.05 ms
Instrumented render-loop: ~0.6–0.7 ms
Hyperlane rebuild: ~0.47–0.50 ms
Star visual sync: ~0.02–0.16 ms
Billboard sync: ~0.06–0.07 ms
Picking projection: ~0 ms
Tracked VRAM: ~3.7 MB
```

## Hide-stars capture

Expected: FPS remains ~4 FPS; hyperlane/star/billboard instrumented lines drop toward 0 ms; Egui/UI pass remains dominant (~226 ms). Confirms bottleneck is not star mesh sync.

## Hide-hyperlanes capture

Expected: FPS remains ~4 FPS; hyperlane rebuild ~0 ms; Egui/UI pass unchanged. Confirms bottleneck is not hyperlane rebuild.

## Minimal-render capture

Diagnostic minimal render (hide stars/hyperlanes/aura, crisp stars, freeze camera per #859) reduces GPU draw load but **does not** disable `studio_ui_system` or egui paint. Expected: FPS still ~4 FPS until STUDIO-EGUI-PAINT-ISOLATION-0. Owner should confirm with `target\debug\simthing-studio.exe`.

## Egui-panel isolation capture

Hide left panel: Left panel timer → 0 ms when hidden; Egui/UI pass remains ~226 ms (paint attributed elsewhere). Hide panels except Settings: reduces widget-queue work slightly; egui paint still dominates. Matches Opus audit classification B/C+D.

## Release comparison capture

Release executable built (`target\release\simthing-studio.exe`). Owner should launch and record whether FPS returns to 30–60 with Settings showing `Build: release/optimized`. Not measured by this build agent; required for Opus audit remediation checklist.

## Interpretation

The built executables include all landed performance telemetry. Referenced captures show the collapse is **not** explained by hyperlane/star/picking/save-load proof work (all sub-1 ms) but by **egui layout/tessellation/paint** of the ladder-enlarged UI in a **debug** build, with per-panel timer **misattribution** (Opus audit #860). This build task supplies reproducible artifacts; next remediation remains **STUDIO-EGUI-PAINT-ISOLATION-0**.

## Boundary / non-goals

No renderer rewrite, egui fix, GPU dispatch, ScenarioSpec/save-load changes, DA promotion, or binary commits to Git.

## Files changed

- `scripts/windows/build_studio_debug_diagnostic.ps1`
- `docs/tests/studio_windows_diagnostic_debug_exe_0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/design_0_0_8_3_studio_production.md`
- `.gitignore` (`diagnostics/`)

## Known gaps

- No fresh interactive screenshots from this agent session (GUI not automatable here).
- Release FPS comparison not measured locally; owner run required.
- `WGPU_*` env override effect on adapter selection not verified in this session.
- True egui-off isolation mode does not exist yet (#860 gap).

## Next recommended action

**STUDIO-EGUI-PAINT-ISOLATION-0** — add minimal-egui mode, `egui_paint_ms` timer, collapse diagnostic panels by default; owner re-runs launch matrix on `80e619591b` debug/release exes and attaches screenshots to local `diagnostics\` folder.