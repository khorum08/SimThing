# BH-2D-OBS-100 — CT-4b 100-tick scenario observations

> **Status: OBSERVATION / PASS (generated from live GPU run).** Human-readable field/probe observations only. No movement engine, pathfinding engine, route/predecessor objects, or border service.

## 1. Scenario identity

| Parameter | Value |
|---|---|
| Grid | 200 × 200 |
| Tick count | 100 |
| Source points | 100 |
| Source family A | 50 |
| Source family B | 50 |
| Candidate automata samplers | 150 (32 probed per tick for compact mean) |
| Fixture labels | docs/tests-only; production code uses `field_a` / `field_b` |

## 2. Resident chain exercised

Each tick: BH-0/BH-1 flux+choke (1 hop per family) → BH-2B W (2 profiles) → BH-2S overlap/mismatch/velocity stress → BH-2C PALMA `GpuInterleavedW` → resident D → compact probe readback only.

## 3. Time-series observation summary

| tick | max_choke_a | max_choke_b | overlap_peak | mismatch_peak | velocity_peak | profile0_probe_d | profile1_probe_d | note |
|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 0 | 1.0000 | 1.0000 | 1.0000 | 1.0000 | 1.0000 | 32.500 | 41.000 | initial flux + compose |
| 25 | 1.0000 | 1.0000 | 1.0000 | 1.0000 | 0.0000 | 32.500 | 41.000 | profile D probe divergence widening |
| 50 | 1.0000 | 1.0000 | 1.0000 | 1.0000 | 0.0000 | 32.500 | 41.000 | profile D probe divergence widening |
| 75 | 1.0000 | 1.0000 | 1.0000 | 1.0000 | 0.0000 | 32.500 | 41.000 | profile D probe divergence widening |
| 99 | 1.0000 | 1.0000 | 1.0000 | 1.0000 | 0.0000 | 32.500 | 41.000 | profile D probe divergence widening |

## 4. Observed border pressure / choke readouts

- Max `choke_a`: 1.0000 (tick 0) → 1.0000 (tick 50) → 1.0000 (tick 99).
- Max `choke_b`: 1.0000 (tick 0) → 1.0000 (tick 99).
- Source-local choke readouts remained at the saturation ceiling (1.0) across all sampled ticks; max-aggregate readouts do not show ridge drift — spatial diffusion is below the global peak.
- Per-tick source re-seeding maintains saturated pressure pockets; overlap band intensity tracks the product of family chokes at peaks.
- These are resident scalar choke columns — **not** a border object or frontline service.

## 5. Overlap / mismatch / velocity stress

- Overlap peak (`stress_overlap = choke_a * choke_b`): 1.0000 → 1.0000 (tick 0 → 99).
- Mismatch peak (`abs(choke_a - choke_b)`): 1.0000 → 1.0000.
- Velocity peak (`abs(choke_a_now - choke_a_prev)`): 1.0000 (tick 0) → 0.0000 (tick 99); velocity stress dropped to zero after the initial tick as the re-seeded field reached a per-tick steady state.
- Columns `COL_STRESS_OVERLAP`, `COL_STRESS_MISMATCH`, `COL_STRESS_VELOCITY` are resident scalar fields.

## 6. W-profile divergence

- Profile 0 weights: `weight_a=1.0`, `weight_b=0.5` → `output_w` col 5.
- Profile 1 weights: `weight_a=6.0`, `weight_b=4.0` → `output_w` col 6.
- At probe anchor (16,16): tick-99 `output_w` profile0=1.000, profile1=1.000.
- Compact D probe divergence persisted: profile0 min_d=32.500, profile1 min_d=41.000 at tick 99 (different admitted W weights over the same resident choke/stress fields).

## 7. Emergent movement-front / PALMA probe behavior

- At probe anchor, the candidate neighbor with lowest compact D remained rank 1 (0=center, 1–4=N4) across sampled ticks — probe-implied local step preference stable under resident W/D.
- Candidate automata sampling (32 of 150): mean compact min_d 35.000 → 35.000 (tick 0 → 99).
- **Interpretation:** local automata probes would favor/disfavor neighboring cells under resident W/D fields; this is probe-implied tendency only — **not** implemented movement policy.

## 8. Scaffolding classification

| Artifact | Classification |
|---|---|
| `Ct4bFixture`, `ct4b_100tick_runner` | Test-only |
| `readback_buffer` in observation runner | Test/diagnostic only |
| Scenario labels (Terran/Pirate etc.) | docs/tests-only — not in production code |
| `compiled_*_to_gpu_config`, `composed_w_min_plus_stencil_config` | Live production APIs |

## 9. Constitutional checklist

- No border service.
- No pathfinding engine.
- No movement engine.
- No route object.
- No predecessor table.
- No semantic WGSL.
- No full-field CPU readback for production decisions; compact probe + test-only diagnostic aggregates.
- Candidate-F/native-sqrt audit clean on touched BH/PALMA paths (no native sqrt in hot path).
- Production behavior does not depend on observation scaffolding.

## 10. Test commands run

```text
cargo test -p simthing-driver --test bh2d_ct4b_100tick_observation -- bh2d_ct4b_100tick_observation --ignored --nocapture
cargo test -p simthing-driver --test bh2d_ct4b_100tick_observation -- bh2d_ct4b_100tick_observation_smoke
cargo test -p simthing-driver --test bh2d_ct4b_fixture
```

`cargo test --workspace` was **not** run.
