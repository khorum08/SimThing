# WORKSPACE-CLEANUP-INVENTORY-0 Results

## Status

PASS — local disk inventory recorded; **133.1 GB** reclaimed from safe delete-now artifacts on `C:\Users\mvorm\SimThing`. Tracked load-bearing evidence untouched. Conservative maybe-archive and potentially-synthesize manifests produced with reference checks.

## PR / branch / merge

- Branch: `workspace-cleanup-inventory-0`
- PR: #870
- Merge SHA: `33200060e3fdab94997ffde94bcc1bca8ee3e2b3`

## Mission

Reclaim local disk from regenerable/ignored workspace artifacts; classify cleanup candidates into Delete now / Maybe archive / Potentially synthesize with measured projected and actual savings; harden `.gitignore` for generated local artifacts.

## Local workspace inspected

- Path: `C:\Users\mvorm\SimThing`
- Source SHA at inventory: `fbf92ebd6a1a97db034d0005ce2c4b447029c34a`
- Ignored paths present: `target/`, `diagnostics/`, `.claude/settings.local.json`
- Untracked local harness (not deleted): `agent-tools/` (~0.011 GB), `mcps/` (~0.0001 GB), `terminals/` (~0.001 GB), `crates/simthing-mapeditor/src/app/gpu_render.rs` (local WIP)

## Before cleanup disk usage

| Path / area | Size (GB) | Git status |
|---|---|---|
| `target/` | **133.100** | ignored |
| `diagnostics/` | **0.000** (0.14 MB) | ignored |
| `docs/` (tracked tree) | 0.013 | tracked |
| `crates/` (source) | 0.012 | tracked |
| `agent-tools/` | 0.011 | untracked |
| `screenshot_*.png` | 0 | none found |
| Largest single file | `target/debug/deps/simthing_studio.pdb` (~0.879 GB) | ignored |

`git clean -ndX`: would remove `.claude/settings.local.json`, `diagnostics/`, `target/`.

## Projected storage savings by bracket

| Bracket | Projected savings GB | Actually reclaimed GB | Confidence | Basis |
|---|---|---|---|---|
| **Delete now** | **133.101** | **133.101** | High | Measured `target/` + `diagnostics/` + screenshots (none) |
| **Maybe archive** | **0.009** | 0 | Medium | Measured tracked PNG cluster under `docs/tests/` (~7.6 MB); not deleted |
| **Potentially synthesize** | **0.001** | 0 | Low/Medium (disk); High (context) | Measured `bevy_mapgen_editor_pr2r*_results.md` cluster (~0.08 MB); not deleted |

## Delete-now actions taken

| Path | Size before GB | Projected GB | Action | Reclaimed GB | Reason |
|---|---|---|---|---|---|
| `target/` | 133.100 | 133.100 | `cargo clean` | 133.100 | Rust build products/PDBs/exes; fully regenerable |
| `diagnostics/` | 0.000 | 0.000 | `Remove-Item -Recurse -Force` | 0.000 | Gitignored local diagnostic output; summaries in docs |
| `screenshot_*.png` | 0 | 0 | none (absent) | 0 | Gitignored Studio screenshots |
| `*.log` / scratch at repo root | 0 | 0 | none (absent) | 0 | N/A |

**Not deleted (owner-visible):** `agent-tools/`, `mcps/`, `terminals/`, untracked `gpu_render.rs` — small or possible local WIP; `git clean -fdx` not run.

## Space reclaimed

**Total reclaimed: 133.101 GB** (`cargo clean` removed 107,487 files, 133.1 GiB per Cargo).

Post-cleanup `target/` size: **0 GB** (directory absent/empty).

## Maybe-archive candidates

| Path | Size GB | Projected GB | Referenced by | Load-bearing risk | Proposed archive action |
|---|---|---|---|---|---|
| `docs/tests/mapgenerator_cli_spiral2_dense_3000_editor_prep.png` | 0.0025 | 0.0025 | editor-prep result reports | Medium — visual sample | External zip or keep; regenerate via MapGeneratorCLI |
| `docs/tests/mapgenerator_cli_spiral2_dense_3000.png` | 0.0022 | 0.0022 | `mapgenerator_cli_spiral2_dense_3000_results.md` | Medium | Keep until owner approves archive |
| `docs/tests/mapgenerator_cli_disc_1500_connected_3000.png` | 0.0021 | 0.0021 | `mapgenerator_cli_disc_1500_connected_3000_results.md` | Medium | Keep until owner approves archive |
| `docs/tests/mapgenerator_cli_spiral_1500_starlanes_3000.png` | 0.0006 | 0.0006 | mapgen artifact reports | Medium | Keep |
| `docs/tests/mapgenerator_cli_visual_spiral_1500.png` | 0.0001 | 0.0001 | visual reports | Low | Keep |
| `docs/tests/mapgenerator_cli_spiral2_dense_3000_editor_prep.report.json` | ~0 | ~0 | **current_evidence_index.md** (#724 sample) | **High** — live evidence pointer | **Do not delete**; archive only with index update |
| `docs/archive/` subtree | 0.002 | 0.002 | ADR/synthesis pointers | Medium — provenance | Keep in repo; recoverable from Git history if ever externalized |

## Potentially-synthesize candidates

| Path / cluster | Size / count | Projected GB | Current evidence replacement | Risk | Proposed synthesis |
|---|---|---|---|---|---|
| `docs/tests/bevy_mapgen_editor_pr2r*_results.md` (12 files) | ~0.08 MB | 0.0001 | `current_evidence_index.md` PROBATION rows + `design_0_0_8_3_studio_production.md` | Medium | Future doc PR: digest visual-tuning findings into production synthesis; do not mass-delete |
| `docs/workshop/studio_production_log.md` | 0.11 MB | 0.0001 | Rolling operational log | Low disk / high context | Periodic trim/summary sections, not deletion in this pass |
| `docs/worklog.md` | 0.68 MB | 0.0007 | Workshop history | Low | Archive externally or summarize; not deleted here |
| `docs/archive/superseded_design/` | ~0.5 MB total | 0.0005 | `current_evidence_index.md` notes superseded habits | Low | Already archived; synthesis pointers sufficient |

## Load-bearing checks

No tracked files deleted. Reference checks performed on maybe-archive PNGs and `editor_prep.report.json`:

- `mapgenerator_cli_spiral2_dense_3000_editor_prep.report.json` — cited in `current_evidence_index.md` (MAPGENCLI-EDITOR-PREP-0R). **Retained.**
- Mapgen PNGs — cited in per-rung `docs/tests/*_results.md`. **Retained.**
- No changes to DA-approved Scenario Runtime + Save/Load evidence rows.

## Files changed

- `.gitignore` — log/tmp/cache/build artifact patterns
- `scripts/windows/workspace_cleanup_inventory.ps1` — safe reclaim helper
- `crates/simthing-mapeditor/src/dialog.rs` — #869 telemetry close test split (icon vs button dialogs)
- `docs/tests/workspace_cleanup_inventory_0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/workshop/studio_production_log.md`

## Gitignore updates

Added: `*.log`, `*.tmp`, `*.bak`, `*.old`, `*.orig`, `*.stdout`, `*.stderr`, `.cache/`, `.pytest_cache/`, `.mypy_cache/`, `.ruff_cache/`, `node_modules/`, `dist/`, `build/`, `tmp/`, `temp/` (in addition to existing `target/`, `diagnostics/`, `screenshot_*.png`).

## Optional #869 telemetry close test

Updated `telemetry_dialog_close_icon_and_button_hide_window` to use separate `icon_dialog` and `button_dialog` instances per #869 compliance note.

## Validation

```text
cargo test -p simthing-mapeditor --lib telemetry_dialog_close
cargo check -p simthing-mapeditor
git diff --check
git check-ignore -v screenshot_00000.png
git check-ignore -v diagnostics/example.log
```

No tracked deletions. No screenshots committed.

## Known gaps

- `agent-tools/`, `mcps/`, `terminals/` left in place (~0.012 GB total); owner may delete manually or add harness-specific ignores.
- Untracked `gpu_render.rs` not touched (possible local WIP).
- Maybe-archive / synthesize tiers not executed — manifest only.
- Rebuild `target/debug/simthing-studio.exe` required after cleanup (`cargo build -p simthing-mapeditor --bin simthing-studio`).

## Next recommended action

Owner: run `scripts/windows/workspace_cleanup_inventory.ps1` after heavy local build sessions; rebuild Studio when needed. Future optional PR: synthesize repetitive `bevy_mapgen_editor_pr2r*` reports into a single performance/visual tuning digest (do not delete until synthesis merged).