# LOCAL-WORKTREE-SPACE-RECOVERY-0 Results

## Status

PASS — local disk usage measured; `target/` and confirmed scratch directories removed; no source, tests,
fixtures, or evidence deleted.

## PR / branch / merge

- Branch: `local-worktree-space-recovery-0`
- PR: #901
- Merge SHA: `a9e6563af6`

## Starting disk usage

| Metric | Value |
|---|---:|
| Total worktree (all files) | **66.172 GB** (67,760 MB) |
| `target/` | **66.048 GB** (67,634 MB) |
| `.git/` | 0.084 GB (86 MB) |
| `crates/` | 0.013 GB (14 MB) |
| `docs/` | 0.013 GB (14 MB) |
| `agent-tools/` (untracked) | 0.011 GB (12 MB) |
| `terminals/` (untracked) | 0.001 GB (0.75 MB) |
| `mcps/` (untracked) | 0.000 GB (0.11 MB) |
| `vendor/` | 0.001 GB (0.86 MB) |

Host: Windows 10 validation workstation (`C:\Users\mvorm\SimThing`).

## Largest directories

Before cleanup (top-level):

| Path | Size |
|---|---:|
| `target/debug/` | 66.046 GB |
| `.git/` | 86 MB |
| `crates/` | 14 MB |
| `docs/` | 14 MB |
| `agent-tools/` | 12 MB |

After cleanup (pre-rebuild):

| Path | Size |
|---|---:|
| `.git/` | 86 MB |
| `crates/` | 14 MB |
| `docs/` | 14 MB |
| `vendor/` | 0.39 MB |
| *(no `target/`, no scratch dirs)* | — |

## Ignored cleanup preview

`git clean -ndX` (before):

```text
Would remove .claude/settings.local.json
Would remove target/
Would remove vendor/msdf_font/Cargo.lock
Would remove vendor/msdf_font/Cargo.toml.orig
Would remove vendor/msdf_font/assets/msdf_atlas_fix.png
```

Executed `cargo clean` first (removed `target/`), then `git clean -fdX` for remaining ignored artifacts.

## Untracked cleanup preview

`git clean -nd` (before):

```text
Would remove agent-tools/
Would remove crates/simthing-driver/examples/
Would remove crates/simthing-gpu/examples/
Would remove crates/simthing-mapeditor/src/app/gpu_render.rs
Would remove crates/simthing-spec/examples/
Would remove mcps/
Would remove terminals/
Would remove tools/
```

**Did not run `git clean -fd`** — preview included paths that may be wanted local work (`gpu_render.rs`,
crate `examples/`, `tools/`). Only the three confirmed agent scratch directories were deleted explicitly.

## Deleted paths

| Path | Size before | Action | Reason | Space recovered |
|---|---:|---|---|---:|
| `target/` | 66.048 GB | DELETE (`cargo clean`) | Ignored Rust build output; 35,616 files | **66.0 GiB** (cargo reported) |
| `agent-tools/` | 11.59 MB | DELETE (`Remove-Item -Recurse -Force`) | Abandoned agent scratch logs (88 UUID `.txt` files) | **~12 MB** |
| `terminals/` | 0.75 MB | DELETE | Cursor terminal session snapshots | **~0.75 MB** |
| `mcps/` | 0.11 MB | DELETE | Local MCP tool descriptor cache | **~0.11 MB** |
| `.claude/settings.local.json` | negligible | DELETE (`git clean -fdX`) | Ignored local settings | negligible |
| `vendor/msdf_font/Cargo.lock` | negligible | DELETE (`git clean -fdX`) | Ignored vendor build artifact | negligible |
| `vendor/msdf_font/Cargo.toml.orig` | negligible | DELETE (`git clean -fdX`) | Ignored vendor backup | negligible |
| `vendor/msdf_font/assets/msdf_atlas_fix.png` | negligible | DELETE (`git clean -fdX`) | Ignored generated PNG | negligible |

## Space recovered

| Metric | Value |
|---|---:|
| **Total recovered (measured)** | **~66.06 GB** |
| `cargo clean` reported | 66.0 GiB (35,616 files) |
| Scratch dirs (`agent-tools` + `terminals` + `mcps`) | ~12.5 MB |
| Worktree after cleanup (pre-rebuild) | 0.111 GB (114 MB) |

Note: post-validation `cargo check` / `cargo test` partially repopulated `target/` (~2–3 GB dev profile).
That is expected; the recovered space is the removed stale build tree.

## Kept paths

| Path | Reason |
|---|---|
| `crates/**` | Source |
| `docs/design_*.md`, `docs/simthing_core_design.md` | Live design docs |
| `docs/tests/current_evidence_index.md` | Live evidence ledger |
| `docs/archive/typeface_track_2026_06/**` | Archived typeface reports |
| `Cargo.lock`, `Cargo.toml`, `.git` | Repo authority |
| Test fixtures (`*.ttf`, smoke PNGs, manifests) | Referenced by live tests |
| `crates/simthing-mapeditor/src/app/gpu_render.rs` | Untracked local WIP — not deleted |
| `crates/*/examples/`, `tools/` | Untracked — not in preview review scope for deletion |

## Review-only paths

| Path | Status | Reason kept |
|---|---|---|
| `crates/simthing-driver/examples/` | REVIEW | Untracked; not confirmed disposable |
| `crates/simthing-gpu/examples/` | REVIEW | Untracked; not confirmed disposable |
| `crates/simthing-spec/examples/` | REVIEW | Untracked; not confirmed disposable |
| `tools/` | REVIEW | Untracked; unknown active use |
| `crates/simthing-mapeditor/src/app/gpu_render.rs` | REVIEW | Possible in-progress local work |

## Validation

After cleanup:

```text
git status --short
?? crates/simthing-mapeditor/src/app/gpu_render.rs

cargo check -p simthing-tools          PASS (rebuild from clean)
cargo test -p simthing-tools --test semantic_free_guard   PASS (1 passed)
```

No source, tests, fixtures, or evidence files deleted.

## Remaining local cleanup options

1. **`target/`** — will grow again with normal development; run `cargo clean` periodically or when disk is tight.
2. **Untracked `examples/` dirs** under `simthing-driver`, `simthing-gpu`, `simthing-spec` — review owner intent before removal.
3. **`tools/`** — review before deletion.
4. **`gpu_render.rs`** — local WIP; keep or commit separately.
