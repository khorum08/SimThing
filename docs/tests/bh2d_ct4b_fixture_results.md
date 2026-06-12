# BH-2D CT-4b 200×200 fixture proof — test results

> **Status: IMPLEMENTED / PASS (2026-06-11).** Full resident feedstock chain at 200×200 over
> generic `field_a` / `field_b` choke columns. No movement engine, pathfinding, route, or
> predecessor objects.

## Fixture (test-only)

| Parameter | Value |
|---|---|
| Grid | 200 × 200 |
| Source points | 100 (50 per family) |
| Automata (metadata) | 150 |
| Module | `support/ct4b_field_fixture.rs` |

## Production chain proved

```text
BH-0/BH-1 choke → BH-2B W (2 profiles) → BH-2S stress → PALMA GpuInterleavedW → D probe
```

Live APIs: BH-2B/BH-2C bridges unchanged. Fixture/oracle quarantined in test modules.

## Gates run

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo test -p simthing-driver --test bh2d_ct4b_fixture` | 11/11 PASS |
| `cargo test -p simthing-driver --test bh2c_palma_w_feedstock` | PASS (regression) |
| `cargo test -p simthing-driver --test palma_path_9_downstream_gpu_consumer` | PASS (regression) |
| `cargo test -p simthing-gpu --test bh2_w_composition` | PASS (regression) |
| `cargo test -p simthing-gpu --test bh2s_overlap_stress` | PASS (regression) |
| `cargo test --workspace` | not run (gate discipline) |

## Scaffolding classification

| Artifact | Classification |
|---|---|
| `compiled_*_to_gpu_config`, `composed_w_min_plus_stencil_config` | Live production APIs |
| `Ct4bFixture`, `apply_gpu_flux_choke_*` | Test-only fixture (`ct4b_field_fixture.rs`) |
| `readback_buffer`, `cpu_oracle_probe` | Test-only (`bh2d_ct4b_fixture.rs`) |
