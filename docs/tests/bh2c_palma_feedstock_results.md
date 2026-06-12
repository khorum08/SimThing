# BH-2C PALMA feedstock proof — test results

> **Status: IMPLEMENTED / PASS (2026-06-11).** BH-2B composed W feeds PALMA/min-plus traversal
> as GPU-resident impedance via `MinPlusTraversalInput::GpuInterleavedW`. D stays resident;
> compact `MinPlusTraversalDProbeOp` readback only. No movement engine, pathfinding, route, or
> predecessor objects.

## Production chain

```text
WImpedanceComposeOp → GpuInterleavedW → MinPlusTraversalFieldOp (GpuResident) → D probe
```

Live API: `composed_w_min_plus_stencil_config` in `w_impedance_compose_bridge.rs`.

## Gates run

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo test -p simthing-driver --test bh2c_palma_w_feedstock` | 8/8 PASS |
| `cargo test -p simthing-gpu --test bh2_w_composition` | PASS (regression) |
| `cargo test -p simthing-driver --test palma_path_9_downstream_gpu_consumer` | PASS (regression) |
| `cargo test --workspace` | not run (gate discipline) |

## Tests

| Test | Proves |
|---|---|
| `bh2c_composed_w_feeds_palma_gpu_traversal` | Compose → PALMA → probe matches CPU oracle |
| `bh2c_choke_weight_changes_traversal_cost` | Higher choke weights raise min-plus D at candidates |
| `bh2c_resident_d_no_full_field_readback` | GpuResident dispatch omits full D readback |
| `bh2c_cpu_oracle_test_only` | Oracle/fixture helpers confined to tests |
| `bh2c_no_route_or_predecessor_objects` | No forbidden pathfinding/movement constructs |
| `bh2c_no_native_sqrt_in_hot_path` | Candidate-F audit clean on touched paths |
| `bh2c_scaffolding_not_required_for_production_pass` | Live API promoted; test helpers quarantined |
| `bh2c_forbidden_production_vocabulary` | No semantic production vocab in bridge/GPU hot paths |

## Scaffolding classification

| Artifact | Classification |
|---|---|
| `composed_w_min_plus_stencil_config` | **Live production API** |
| `build_interleaved_fixture` | Test-only (quarantined in test file) |
| `cpu_oracle_probe` | Test-only CPU oracle |
| `run_compose_then_traversal_probe` | Test-only GPU chain helper |
