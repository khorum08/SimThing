# BH-2S overlap stress — test results

**Rung:** BH-2S multi-field overlap stress (scenario-track addendum)  
**Date:** 2026-06-11  
**PR:** BH-2S: multi-field overlap stress field algebra

## Purpose

Generic GPU stress field algebra over resident choke columns for FIELD_POLICY motivation
feedstock. No semantic production code, border objects, CPU planner, or full-field readback.

## Implementation summary

| Layer | Artifact |
|---|---|
| `simthing-spec` | `StressComposeSpec`, `compile_stress_compose_preview` |
| `simthing-gpu` | `StressComposeOp`, `stress_compose.wgsl`, `cpu_stress_compose_oracle` |
| `simthing-driver` | `compiled_stress_compose_to_gpu_config` (bridge only) |

Operators: overlap, mismatch, weighted, velocity. Admission caps: 8 profiles, 4 input field columns.

## Candidate-F sqrt audit

No forbidden sqrt/magnitude/norm tokens in BH-2S hot paths.

## Targeted gates

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo test -p simthing-gpu --test bh0_saturating_flux` | PASS |
| `cargo test -p simthing-gpu --test bh1_choke_readout` | PASS |
| `cargo test -p simthing-gpu --test bh1r_choke_threshold` | PASS |
| `cargo test -p simthing-gpu --test bh1r_scale_parallel_reduction` | PASS |
| `cargo test -p simthing-gpu --test bh2_w_composition` | PASS |
| `cargo test -p simthing-gpu --test bh2s_overlap_stress` | PASS |
| `cargo test -p simthing-spec --test bh2s_stress_compose_admission` | PASS |

## Deferred

- **BH-2C** — PALMA feedstock proof
- **BH-2D** — CT-4b 200×200 fixture proof
