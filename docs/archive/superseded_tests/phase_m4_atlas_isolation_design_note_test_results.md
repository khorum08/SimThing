# Phase M-4 — Atlas Batching Isolation + VRAM Accounting Design Note — Test Results

**Date/time:** 2026-05-28  
**Base HEAD:** `f5f1dc6b28c72f185a3a111414a0f6858f63beab` (Phase M-3 merge + perception deferral note)  
**Branch:** `phase-m4-atlas-isolation-design-note`  
**Final commit SHA:** `c6917d5844d0d88ea7847d4475bc4a5e974be52a` (phase-m4-atlas-isolation-design-note)  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell

---

## Goal

Produce Opus-gated Phase M-4 design note for atlas batching isolation and VRAM accounting. Docs-only — no implementation.

---

## Docs-only confirmation

| Check | Result |
|-------|--------|
| Production code changed | **NO** |
| Atlas packer implemented | **NO** |
| WGSL added/changed | **NO** |
| Session mapping execution wired | **NO** |
| Mapping ADR classifications changed | **NO** (supplement only) |

---

## Files changed

| File | Change |
|------|--------|
| `docs/workshop/mapping_atlas_batching_isolation_design_note.md` | **Created** — M-4 design note |
| `docs/accumulator_op_v2_production_plan.md` | M-4 marked design note Done |
| `docs/todo.md` | M-4 design note Done; next step sign-off |
| `docs/worklog.md` | M-4 entry |
| `docs/workshop/mapping_current_guidance.md` | M-4 landed; read order updated |
| `docs/workshop/workshop_current_state.md` | M-4 status |

---

## Design note path

[`docs/workshop/mapping_atlas_batching_isolation_design_note.md`](../workshop/mapping_atlas_batching_isolation_design_note.md)

---

## Atlas contract summary

| Topic | Contract |
|-------|----------|
| Status | Provisional; unimplemented; blocked until human + Opus sign-off |
| Short-term isolation | `gutter >= effective_horizon` |
| Seed protocol | Per-tile identity seed clear; **column-wide source_col zeroing banned** |
| VRAM accounting | Mandatory multiplier/overhead/bytes reporting; refuse pack without accounting |
| Acceptance gate | Full-tile parity vs protocol-faithful per-tile CPU oracle |
| t44 alone | **Insufficient** for production acceptance |
| Local-bounds metadata | Deferred; requires future implementation ADR/PR |
| Active masks | Provisional; `ActiveOnlyExperimentalNoHalo` not authorized with atlas |
| v1 batching | Homogeneous square tiles; same grid_size/horizon/gutter/operator family |

---

## Commands run

| Command | Result |
|---------|--------|
| `git status --short` | Docs-only changes (workshop test report noise excluded from commit) |
| `git rev-parse HEAD` | `f5f1dc6` (base) |
| Keyword scan (`atlas`, `gutter`, `VRAM`, `t44`, etc.) | **PASS** — design note + updated active docs contain required terms |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` | **PASS** |
| `cargo test -p simthing-spec --test region_field_spec_admission` | **PASS** — 10/10 |
| `cargo test -p simthing-driver --test phase_m2_field_scheduler` | **PASS** — 12/12 |
| `cargo test -p simthing-gpu --test structured_field_stencil` | **PASS** — 16/16 |

---

## Pass/fail table

| Criterion | Status |
|-----------|--------|
| M-4 design note exists | **PASS** |
| Preserves atlas provisional status | **PASS** |
| Defines gutter >= effective horizon | **PASS** |
| Defines VRAM accounting formulas/fields | **PASS** |
| Defines per-tile seed-clearing protocol | **PASS** |
| Bans column-wide source_col zeroing | **PASS** |
| Requires full-tile protocol-oracle parity | **PASS** |
| Rejects t44-only production acceptance | **PASS** |
| Defers local-bounds metadata | **PASS** |
| Active masks provisional / halo-required | **PASS** |
| Active docs updated | **PASS** |
| No production code changes | **PASS** |
| No mapping runtime | **PASS** |
| Full workspace check/test | **PASS** |

---

## Final verdict

**PASS** — Phase M-4 atlas batching isolation + VRAM accounting design note landed; atlas remains provisional and unimplemented; future production atlas implementation is gated on gutter/local-bounds isolation, VRAM multiplier reporting, and full-tile protocol-oracle parity.
