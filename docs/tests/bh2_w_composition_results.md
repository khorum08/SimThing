# BH-2 W composition — test results

**Rungs:** BH-2A named consumer contract + BH-2B W composition kernel  
**Date:** 2026-06-11  
**PR:** BH-2A/B: add CT-4b consumer contract and GPU W composition feedstock

## Named consumer

`CT-4b_Local_Automata_W_Feedstock` opens BH-2. Scope: numeric W feedstock only — no movement
policy, pathfinding engine, route/predecessor objects, or semantic WGSL.

## Implementation summary

| Layer | Artifact |
|---|---|
| `simthing-spec` | `WImpedanceComposeSpec`, `compile_w_impedance_compose_preview` |
| `simthing-gpu` | `WImpedanceComposeOp`, `w_impedance_compose.wgsl`, `cpu_w_impedance_compose_oracle` |
| `simthing-driver` | `compiled_w_impedance_compose_to_gpu_config` (bridge only) |

Per-cell per profile: `output_w = base_w + weight_a * choke_a + weight_b * choke_b`.

## Candidate-F sqrt audit

No `sqrt`, `length`, `distance`, `normalize`, `hypot`, `magnitude`, or `norm(` in BH-2B hot paths.

## Targeted gates

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo test -p simthing-gpu --test bh0_saturating_flux` | PASS |
| `cargo test -p simthing-gpu --test bh1_choke_readout` | PASS |
| `cargo test -p simthing-gpu --test bh1r_choke_threshold` | PASS |
| `cargo test -p simthing-gpu --test bh1r_scale_parallel_reduction` | PASS |
| `cargo test -p simthing-gpu --test bh2_w_composition` | PASS |
| `cargo test -p simthing-spec --test bh0_saturating_flux_admission` | PASS |
| `cargo test -p simthing-spec --test bh1_choke_readout_admission` | PASS |
| `cargo test -p simthing-spec --test bh2_w_composition_admission` | PASS |

## Deferred

- **BH-2C** — PALMA feedstock proof (composed W → min-plus traversal)
- **BH-2D** — CT-4b 200×200 two-field fixture proof
