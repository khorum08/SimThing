# BH-2D-OBS-100R — CT-4b 100-tick dynamic scenario observations

> **Status: OBSERVATION / PASS (generated from live GPU run, BH-2D-OBS-100R).** Deterministic test-only dynamic stimulus. No movement engine, pathfinding engine, route/predecessor objects, or border service.

## 1. Scenario identity

| Parameter | Value |
|---|---|
| Grid | 200 × 200 |
| Tick count | 100 |
| Static source points | 40 per family (50 minus 10 mobile) |
| Mobile source emitters | 10 family A + 10 family B (deterministic drift) |
| Total source-family coverage | 100 points (same CT-4b shape) |
| Candidate automata samplers | 150 metadata; 32 probed and displaced test-only per tick |
| Fixture labels | docs/tests-only; production code uses `field_a` / `field_b` |

## 2. Dynamic stimulus description (test-only)

| Mechanism | Schedule |
|---|---|
| Pressure decay | All cells × 0.92 per tick before re-injection |
| Source pulse | `0.55 + 0.45 × ((tick + phase×11) mod 20) / 20` per family (phases 0/1) |
| Mobile family A | First 10 emitters shift +1 east every 3 ticks |
| Mobile family B | First 10 emitters shift +1 south every 4 ticks |
| Flux hops per tick | 2 (OBS_FLUX_HORIZON) |
| Candidate sampler step | After compact D probe, move to lowest-D N4 neighbor (test-only) |

Resident generic fields affected: `pressure_a/b`, `choke_a/b`, composed W, stress columns. Production BH/PALMA ops unchanged — stimulus lives only in `ct4b_100tick_runner`.

## 3. Resident chain exercised

Each tick: dynamic pressure inject → BH-0/BH-1 flux+choke (2 hops per family) → BH-2B W (2 profiles) → BH-2S overlap/mismatch/velocity stress → BH-2C PALMA `GpuInterleavedW` → resident D → compact probe readback → test-only candidate sampler displacement.

## 4. Time-series observation summary

| tick | max_choke_a | max_choke_b | overlap_peak | mismatch_peak | velocity_peak | profile0_probe_d | profile1_probe_d | moved_candidates | mean_displacement | note |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 0 | 0.6327 | 0.9500 | 0.6002 | 0.8025 | 0.6327 | 32.535 | 41.954 | 1 | 0.03 | initial dynamic pulse + flux |
| 25 | 0.9450 | 1.0000 | 0.8748 | 1.0000 | 0.0911 | 33.379 | 47.502 | 1 | 0.81 | 1 candidate samplers displaced |
| 50 | 1.0000 | 0.9386 | 0.7066 | 1.0000 | 0.0776 | 33.144 | 45.399 | 0 | 1.12 | velocity stress active |
| 75 | 1.0000 | 0.9406 | 0.8576 | 1.0000 | 0.9175 | 33.223 | 46.241 | 0 | 1.12 | velocity stress active |
| 99 | 1.0000 | 1.0000 | 0.9500 | 1.0000 | 0.9764 | 33.229 | 46.467 | 0 | 1.12 | velocity stress active |

## 5. Observed shifting pressure

- Max `choke_a`: 0.6327 (tick 0) → 1.0000 (tick 50) → 1.0000 (tick 99).
- Max `choke_b`: 0.9500 (tick 0) → 1.0000 (tick 99).
- Overlap peak: 0.6002 → 0.9500; mismatch peak: 0.8025 → 1.0000.
- Velocity peak: 0.6327 (tick 0) → 0.0776 (tick 50) → 0.9764 (tick 99).
- Moving emitters and pulsed injection shifted choke/stress readouts over the run; contact-band intensity tracks mobile source overlap rather than a fixed saturated plateau.
- Resident scalar choke/stress columns — **not** a border object or frontline service.

## 6. Overlap / mismatch / velocity stress

- Overlap peak (`choke_a × choke_b`): 0.6002 → 0.9500 (tick 0 → 99).
- Mismatch peak (`|choke_a − choke_b|`): 0.8025 → 1.0000.
- Velocity peak (`|choke_a_now − choke_a_prev|`): 0.6327 → 0.9764.
- Velocity stress remained active mid-run as mobile emitters and decay/pulse changed the choke field between ticks.
- Columns `COL_STRESS_OVERLAP`, `COL_STRESS_MISMATCH`, `COL_STRESS_VELOCITY` are resident scalar fields.

## 7. W-profile divergence

- Profile 0 weights: `weight_a=1.0`, `weight_b=0.5` → `output_w` col 5.
- Profile 1 weights: `weight_a=6.0`, `weight_b=4.0` → `output_w` col 6.
- At probe anchor (16,16): tick-0 W profile0=1.986/profile1=7.732; tick-99 profile0=2.429/profile1=10.428.
- Compact D probes: profile0 32.535→33.229, profile1 41.954→46.467 (tick 0 → 99).
- Divergence comes from admitted W weights over the same resident choke/stress fields — no semantic branches.

## 8. Emergent movement-front / PALMA probe behavior

- Anchor profile-0 lowest-D neighbor rank stable at 3 (0=center, 1–4=N4).
- Anchor profile-1 lowest-D neighbor rank: 1 → 3.
- Test-only candidate samplers: mean Manhattan displacement 0.03 → 1.12; moved this tick at tick 99: 0.
- Candidate mean compact min_d: 35.000 → 0.000 (tick 0 → 99).
- **Interpretation:** probe-implied local automata would favor/disfavor neighboring cells; test-only sampler displacement tracks compact D gradients — **not** production movement policy.

## 9. Scaffolding classification

| Artifact | Classification |
|---|---|
| `Ct4bFixture`, `ct4b_100tick_runner`, `DynamicObsState` | Test-only |
| Dynamic pulse / mobile emitters / candidate step | Test-only observation stimulus |
| `readback_buffer` in observation runner | Test/diagnostic only |
| Scenario labels (Terran/Pirate etc.) | docs/tests-only — not in production code |
| `compiled_*_to_gpu_config`, `composed_w_min_plus_stencil_config`, GPU compose/flux/PALMA ops | Live production APIs |

## 10. Constitutional checklist

- No border service.
- No frontline service.
- No pathfinding engine.
- No movement engine.
- No route object.
- No predecessor table.
- No CPU planner.
- No semantic WGSL.
- No faction-specific production code.
- No full-field CPU readback for production decisions; compact probe + test-only diagnostic aggregates.
- Candidate-F/native-sqrt audit clean on touched BH/PALMA paths (no native sqrt; scalar W/D/choke/stress only).
- Production behavior does not depend on observation scaffolding.

## 11. Test commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-driver --test bh2d_ct4b_100tick_observation -- bh2d_ct4b_100tick_observation --ignored --nocapture
cargo test -p simthing-driver --test bh2d_ct4b_100tick_observation -- bh2d_ct4b_100tick_observation_smoke
cargo test -p simthing-driver --test bh2d_ct4b_fixture
cargo test -p simthing-driver --test bh2c_palma_w_feedstock
cargo test -p simthing-driver --test palma_path_9_downstream_gpu_consumer
cargo test -p simthing-gpu --test bh2_w_composition
cargo test -p simthing-gpu --test bh2s_overlap_stress
```

`cargo test --workspace` was **not** run.
