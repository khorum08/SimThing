# AGENTS.md
Cold-start: run `bash scripts/ci/orient.sh --role=<coding|orchestrator|da>`; doctrine via `bash scripts/ci/anchor_query.sh` (not raw greps).
Full contract: read the generated `docs/orchestrator_orientation.md` and carry the emitted ORIENT-RECEIPT.
Human operators bringing up an agent: see `docs/agent_onboarding.md`.
This file only points; do not add guidance here.

## Cursor Cloud specific instructions
Product/run/test commands live in `docs/agents.md` (the build/run/test router) — use it, not this section, for standard cargo workflow. Notes below are only non-obvious cloud-VM caveats.

- **Toolchain:** `simthing-clausething` is edition 2024, so Rust must be >= 1.85. The VM defaults to `rustup` stable (currently 1.97); the repo has no `rust-toolchain.toml`. Cargo 1.83 fails to even parse the workspace manifest.
- **No physical GPU:** there is no `/dev/dri`. GPU work runs on Mesa **lavapipe** (software Vulkan, `llvmpipe`). For anything that opens a wgpu adapter — `simthing record`, `simthing bench`, and GPU-backed tests (e.g. `simthing-gpu`, most `simthing-driver` runtime tests) — export before running:
  - `export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json`
  - `unset DISPLAY` (a stale `DISPLAY=:1` is set in the VM and makes Vulkan/wgpu surface/WSI enumeration fail with an X `BadMatch`; unsetting it forces headless adapter enumeration). `simthing replay` and pure-CPU tests do not need this.
- **Runnable binaries on Linux:** only `mapgen` (`simthing-mapgenerator`, CPU) and `simthing` (`simthing-driver`: `record`/`replay`/`bench`). `simthing-studio` (`simthing-mapeditor`) is **Windows-only** — its `main.rs` exits immediately on non-Windows, so the authoring GUI cannot be launched here (its lib still builds/tests on Linux).
- **Lint drift:** `cargo fmt --all -- --check` currently reports diffs on committed files (rustfmt-version drift from the maintainer's Windows toolchain), unrelated to any change you make.
- `scripts/ci/` is the agent-governance/doctrine harness, **not** the product build.
