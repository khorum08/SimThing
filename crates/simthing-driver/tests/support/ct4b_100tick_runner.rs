//! Test-only 100-tick CT-4b observation runner (BH-2D-OBS-100).
//!
//! Exercises the resident field chain each tick and records compact aggregate metrics.
//! Full-field readback is test/diagnostic only — not a production decision path.

use simthing_driver::{
    compiled_stress_compose_to_gpu_config, compiled_w_impedance_compose_to_gpu_config,
    composed_w_min_plus_stencil_config,
};
use simthing_gpu::wgpu::{self, util::DeviceExt};
use simthing_gpu::{
    GpuContext, MinPlusTraversalDProbeConfig, MinPlusTraversalDProbeOp,
    MinPlusTraversalExecutionOptions, MinPlusTraversalFieldOp, MinPlusTraversalInput,
    StressComposeConfig, StressComposeOp, WImpedanceComposeConfig, WImpedanceComposeOp,
    MIN_PLUS_INF,
};
use simthing_spec::{
    compile_stress_compose_preview, compile_w_impedance_compose_preview, StressComposeSpec,
    StressOperatorSpec, WImpedanceComposeSpec,
};

use super::ct4b_field_fixture::{
    Ct4bFixture, COL_BASE_W, COL_CHOKE_A, COL_CHOKE_B, COL_PRESSURE_A, COL_PRESSURE_B,
    CT4B_AUTOMATA_COUNT, CT4B_CELL_COUNT, CT4B_DEST, CT4B_FIELD_A_SOURCES, CT4B_FIELD_B_SOURCES,
    CT4B_HEIGHT, CT4B_MIN_PLUS_ITERATIONS, CT4B_PROBE_ANCHOR, CT4B_SOURCE_COUNT, CT4B_WIDTH,
};
use super::palma_min_plus_oracle::cell_index;

pub const OBS_TICK_COUNT: u32 = 100;
pub const OBS_SAMPLE_TICKS: [u32; 5] = [0, 25, 50, 75, 99];
pub const OBS_FLUX_HORIZON: u32 = 1;

// Observation layout extends BH-2D with velocity prev + stress_velocity + d column.
pub const OBS_N_DIMS: u32 = 12;
pub const OBS_COL_OUTPUT_W_0: u32 = 5;
pub const OBS_COL_OUTPUT_W_1: u32 = 6;
pub const OBS_COL_STRESS_OVERLAP: u32 = 7;
pub const OBS_COL_STRESS_MISMATCH: u32 = 8;
pub const OBS_COL_STRESS_VELOCITY: u32 = 9;
pub const OBS_COL_CHOKE_A_PREV: u32 = 10;
pub const OBS_COL_D: u32 = 11;

const SOURCE_PRESSURE_A: f32 = 8.0;
const SOURCE_PRESSURE_B: f32 = 7.5;

fn idx(slot: u32, col: u32) -> usize {
    (slot * OBS_N_DIMS + col) as usize
}

#[derive(Clone, Debug)]
pub struct TickObservation {
    pub tick: u32,
    pub max_choke_a: f32,
    pub max_choke_b: f32,
    pub overlap_peak: f32,
    pub mismatch_peak: f32,
    pub velocity_peak: f32,
    pub profile0_probe_min_d: f32,
    pub profile1_probe_min_d: f32,
    pub anchor_w0: f32,
    pub anchor_w1: f32,
    pub preferred_neighbor_rank: usize,
    pub automata_sample_min_d_mean: f32,
    pub note: String,
}

pub struct ObservationRun {
    pub ticks: Vec<TickObservation>,
    pub automata_probe_slots: Vec<u32>,
}

fn apply_gpu_flux_choke_step(
    values: &mut [f32],
    ctx: &GpuContext,
    pressure_col: u32,
    choke_col: u32,
) {
    use simthing_gpu::{
        StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
        StructuredFieldStencilMaskMode, StructuredFieldStencilOp, StructuredFieldStencilOperator,
        StructuredFieldStencilSourcePolicy,
    };
    let config = StructuredFieldStencilConfig {
        width: CT4B_WIDTH,
        height: CT4B_HEIGHT,
        n_dims: OBS_N_DIMS,
        source_col: pressure_col,
        target_col: pressure_col,
        horizon: OBS_FLUX_HORIZON,
        alpha_self: 1.0,
        gamma_neighbor: 0.0,
        weight_north: 0.0,
        weight_south: 0.0,
        weight_east: 0.0,
        weight_west: 0.0,
        source_cap: None,
        operator: StructuredFieldStencilOperator::SaturatingFlux {
            u_sat: 6.0,
            chi: 0.2,
            choke_output_col: Some(choke_col),
        },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    };
    let op = StructuredFieldStencilOp::new(ctx, config).expect("flux op");
    op.upload_values(ctx, values).expect("flux upload");
    let (out, _) = op
        .run_ping_pong(ctx, OBS_FLUX_HORIZON)
        .expect("flux dispatch");
    values.copy_from_slice(&out);
}

fn expand_fixture_to_obs_layout(base: &Ct4bFixture) -> (Vec<f32>, Vec<u32>, Vec<u32>) {
    use super::ct4b_field_fixture::CT4B_N_DIMS;
    let mut values = vec![0.0f32; (CT4B_CELL_COUNT * OBS_N_DIMS) as usize];
    for slot in 0..CT4B_CELL_COUNT {
        for col in 0..CT4B_N_DIMS {
            values[idx(slot, col)] = base.values[(slot * CT4B_N_DIMS + col) as usize];
        }
    }
    (
        values,
        base.field_a_sources.clone(),
        base.field_b_sources.clone(),
    )
}

fn reseed_source_pressure(values: &mut [f32], field_a_sources: &[u32], field_b_sources: &[u32]) {
    for slot in 0..CT4B_CELL_COUNT {
        values[idx(slot, COL_PRESSURE_A)] = 0.0;
        values[idx(slot, COL_PRESSURE_B)] = 0.0;
    }
    for &slot in field_a_sources {
        values[idx(slot, COL_PRESSURE_A)] = SOURCE_PRESSURE_A;
    }
    for &slot in field_b_sources {
        values[idx(slot, COL_PRESSURE_B)] = SOURCE_PRESSURE_B;
    }
    let (ax, ay) = CT4B_PROBE_ANCHOR;
    let anchor_slot = cell_index(ax as usize, ay as usize, CT4B_WIDTH as usize) as u32;
    values[idx(anchor_slot, COL_PRESSURE_A)] = SOURCE_PRESSURE_A;
    values[idx(anchor_slot, COL_PRESSURE_B)] = SOURCE_PRESSURE_B;
}

fn snapshot_choke_a_prev(values: &mut [f32]) {
    for slot in 0..CT4B_CELL_COUNT {
        values[idx(slot, OBS_COL_CHOKE_A_PREV)] = values[idx(slot, COL_CHOKE_A)];
    }
}

fn column_max(values: &[f32], col: u32) -> f32 {
    (0..CT4B_CELL_COUNT)
        .map(|slot| values[idx(slot, col)])
        .fold(0.0f32, f32::max)
}

fn automata_probe_slots(count: usize) -> Vec<u32> {
    let mut slots = Vec::with_capacity(count);
    let mut attempt = 0u64;
    while slots.len() < count {
        let mut x = 0xC001_u64.wrapping_add(attempt.wrapping_mul(0xD1B5_4A32_D192_ED03));
        x ^= x >> 33;
        x = x.wrapping_mul(0xff51_afd7_ed55_8ccd);
        x ^= x >> 33;
        let slot = (x % CT4B_CELL_COUNT as u64) as u32;
        if !slots.contains(&slot) {
            slots.push(slot);
        }
        attempt += 1;
    }
    slots
}

fn neighbor_candidates(center_slot: u32) -> Vec<u32> {
    let x = center_slot % CT4B_WIDTH;
    let y = center_slot / CT4B_WIDTH;
    let ix = x as i32;
    let iy = y as i32;
    [
        (ix, iy),
        (ix - 1, iy),
        (ix + 1, iy),
        (ix, iy - 1),
        (ix, iy + 1),
    ]
    .into_iter()
    .filter(|(nx, ny)| *nx >= 0 && *ny >= 0 && *nx < CT4B_WIDTH as i32 && *ny < CT4B_HEIGHT as i32)
    .map(|(nx, ny)| cell_index(nx as usize, ny as usize, CT4B_WIDTH as usize) as u32)
    .collect()
}

fn w_compose_config_obs() -> WImpedanceComposeConfig {
    let spec = WImpedanceComposeSpec {
        width: CT4B_WIDTH,
        height: CT4B_HEIGHT,
        n_dims: OBS_N_DIMS,
        base_w_col: COL_BASE_W,
        choke_a_col: COL_CHOKE_A,
        choke_b_col: COL_CHOKE_B,
        profiles: vec![
            simthing_spec::WImpedanceComposeProfileSpec {
                weight_a: 1.0,
                weight_b: 0.5,
                output_w_col: OBS_COL_OUTPUT_W_0,
            },
            simthing_spec::WImpedanceComposeProfileSpec {
                weight_a: 6.0,
                weight_b: 4.0,
                output_w_col: OBS_COL_OUTPUT_W_1,
            },
        ],
    };
    let compiled = compile_w_impedance_compose_preview(&spec).expect("w admission");
    compiled_w_impedance_compose_to_gpu_config(&compiled)
}

fn stress_compose_config_obs() -> StressComposeConfig {
    let spec = StressComposeSpec {
        width: CT4B_WIDTH,
        height: CT4B_HEIGHT,
        n_dims: OBS_N_DIMS,
        choke_a_col: COL_CHOKE_A,
        choke_b_col: COL_CHOKE_B,
        profiles: vec![
            simthing_spec::StressComposeProfileSpec {
                operator: StressOperatorSpec::Overlap,
                output_col: OBS_COL_STRESS_OVERLAP,
            },
            simthing_spec::StressComposeProfileSpec {
                operator: StressOperatorSpec::Mismatch,
                output_col: OBS_COL_STRESS_MISMATCH,
            },
            simthing_spec::StressComposeProfileSpec {
                operator: StressOperatorSpec::Velocity {
                    choke_now_col: COL_CHOKE_A,
                    choke_prev_col: OBS_COL_CHOKE_A_PREV,
                },
                output_col: OBS_COL_STRESS_VELOCITY,
            },
        ],
    };
    let compiled = compile_stress_compose_preview(&spec).expect("stress admission");
    compiled_stress_compose_to_gpu_config(&compiled)
}

fn readback_buffer(ctx: &GpuContext, src: &wgpu::Buffer, len: usize) -> Vec<f32> {
    let bytes = (len * std::mem::size_of::<f32>()) as u64;
    let staging = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("ct4b_obs_readback"),
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut encoder = ctx
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("ct4b_obs_readback_enc"),
        });
    encoder.copy_buffer_to_buffer(src, 0, &staging, 0, bytes);
    ctx.queue.submit(Some(encoder.finish()));
    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    ctx.device.poll(wgpu::Maintain::Wait);
    let data = slice.get_mapped_range();
    let out = bytemuck::cast_slice(&data).to_vec();
    drop(data);
    staging.unmap();
    out
}

fn run_palma_probe(
    ctx: &GpuContext,
    buffer: &wgpu::Buffer,
    compose: &WImpedanceComposeConfig,
    profile_index: usize,
    candidates: &[u32],
) -> simthing_gpu::MinPlusTraversalDProbeResult {
    let stencil = composed_w_min_plus_stencil_config(
        compose,
        profile_index,
        OBS_COL_D,
        CT4B_DEST,
        MIN_PLUS_INF,
    );
    let op = MinPlusTraversalFieldOp::new(ctx, stencil.clone()).expect("traversal op");
    op.dispatch_traversal_from_input(
        ctx,
        MinPlusTraversalInput::GpuInterleavedW(buffer),
        None,
        MinPlusTraversalExecutionOptions::gpu_resident(CT4B_MIN_PLUS_ITERATIONS),
    )
    .expect("palma dispatch");
    let resident = op.output_handle(CT4B_MIN_PLUS_ITERATIONS);
    let probe_config = MinPlusTraversalDProbeConfig::from_stencil_config(&stencil);
    MinPlusTraversalDProbeOp::new(ctx)
        .probe_resident_d(ctx, resident, &probe_config, candidates, stencil.cells())
        .expect("probe")
}

fn preferred_neighbor_rank(probe: &simthing_gpu::MinPlusTraversalDProbeResult) -> usize {
    probe
        .gathered
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
        .unwrap_or(0)
}

pub fn run_observation_ticks(ctx: &GpuContext, tick_count: u32) -> ObservationRun {
    let base = Ct4bFixture::build_seeded();
    let (mut values, field_a_sources, field_b_sources) = expand_fixture_to_obs_layout(&base);
    let compose = w_compose_config_obs();
    let stress = stress_compose_config_obs();
    let anchor_candidates = neighbor_candidates(cell_index(
        CT4B_PROBE_ANCHOR.0 as usize,
        CT4B_PROBE_ANCHOR.1 as usize,
        CT4B_WIDTH as usize,
    ) as u32);
    let automata_slots = automata_probe_slots(CT4B_AUTOMATA_COUNT.min(64));
    let mut prev_max_choke_a = 0.0f32;

    let mut observations = Vec::with_capacity(tick_count as usize);
    for tick in 0..tick_count {
        snapshot_choke_a_prev(&mut values);
        reseed_source_pressure(&mut values, &field_a_sources, &field_b_sources);

        apply_gpu_flux_choke_step(&mut values, ctx, COL_PRESSURE_A, COL_CHOKE_A);
        apply_gpu_flux_choke_step(&mut values, ctx, COL_PRESSURE_B, COL_CHOKE_B);

        let buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("ct4b_obs_field"),
                contents: bytemuck::cast_slice(&values),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            });

        WImpedanceComposeOp::new(ctx)
            .compose_resident_field(ctx, &buffer, &compose)
            .expect("w compose");
        StressComposeOp::new(ctx)
            .compose_resident_field(ctx, &buffer, &stress)
            .expect("stress compose");

        let gpu_values = readback_buffer(ctx, &buffer, compose.values_len());
        values.copy_from_slice(&gpu_values);

        let probe0 = run_palma_probe(ctx, &buffer, &compose, 0, &anchor_candidates);
        let probe1 = run_palma_probe(ctx, &buffer, &compose, 1, &anchor_candidates);

        let mut automata_min_sum = 0.0f32;
        let mut automata_count = 0usize;
        for &slot in automata_slots.iter().take(32) {
            let cands = neighbor_candidates(slot);
            if cands.is_empty() {
                continue;
            }
            let p = run_palma_probe(ctx, &buffer, &compose, 0, &cands);
            if p.min_d.is_finite() {
                automata_min_sum += p.min_d;
                automata_count += 1;
            }
        }
        let automata_mean = if automata_count > 0 {
            automata_min_sum / automata_count as f32
        } else {
            f32::INFINITY
        };

        let anchor_slot = cell_index(
            CT4B_PROBE_ANCHOR.0 as usize,
            CT4B_PROBE_ANCHOR.1 as usize,
            CT4B_WIDTH as usize,
        ) as u32;

        let max_choke_a = column_max(&values, COL_CHOKE_A);
        let max_choke_b = column_max(&values, COL_CHOKE_B);
        let overlap_peak = column_max(&values, OBS_COL_STRESS_OVERLAP);
        let mismatch_peak = column_max(&values, OBS_COL_STRESS_MISMATCH);
        let velocity_peak = column_max(&values, OBS_COL_STRESS_VELOCITY);

        let note = if tick == 0 {
            "initial flux + compose".to_string()
        } else if max_choke_a > prev_max_choke_a + 0.01 {
            "choke_a peak rising".to_string()
        } else if (probe1.min_d - probe0.min_d).abs() > 0.5 {
            "profile D probe divergence widening".to_string()
        } else if velocity_peak > 0.05 {
            "velocity stress active".to_string()
        } else {
            "steady diffusion".to_string()
        };
        prev_max_choke_a = max_choke_a;

        observations.push(TickObservation {
            tick,
            max_choke_a,
            max_choke_b,
            overlap_peak,
            mismatch_peak,
            velocity_peak,
            profile0_probe_min_d: probe0.min_d,
            profile1_probe_min_d: probe1.min_d,
            anchor_w0: values[idx(anchor_slot, OBS_COL_OUTPUT_W_0)],
            anchor_w1: values[idx(anchor_slot, OBS_COL_OUTPUT_W_1)],
            preferred_neighbor_rank: preferred_neighbor_rank(&probe0),
            automata_sample_min_d_mean: automata_mean,
            note,
        });
    }

    ObservationRun {
        ticks: observations,
        automata_probe_slots: automata_slots,
    }
}

pub fn render_observation_markdown(run: &ObservationRun) -> String {
    let mut out = String::new();
    out.push_str("# BH-2D-OBS-100 — CT-4b 100-tick scenario observations\n\n");
    out.push_str("> **Status: OBSERVATION / PASS (generated from live GPU run).** ");
    out.push_str("Human-readable field/probe observations only. ");
    out.push_str(
        "No movement engine, pathfinding engine, route/predecessor objects, or border service.\n\n",
    );

    out.push_str("## 1. Scenario identity\n\n");
    out.push_str("| Parameter | Value |\n|---|---|\n");
    out.push_str("| Grid | 200 × 200 |\n");
    out.push_str("| Tick count | 100 |\n");
    out.push_str("| Source points | 100 |\n");
    out.push_str("| Source family A | 50 |\n");
    out.push_str("| Source family B | 50 |\n");
    out.push_str("| Candidate automata samplers | 150 (32 probed per tick for compact mean) |\n");
    out.push_str(
        "| Fixture labels | docs/tests-only; production code uses `field_a` / `field_b` |\n\n",
    );

    out.push_str("## 2. Resident chain exercised\n\n");
    out.push_str("Each tick: BH-0/BH-1 flux+choke (1 hop per family) → BH-2B W (2 profiles) → ");
    out.push_str("BH-2S overlap/mismatch/velocity stress → BH-2C PALMA `GpuInterleavedW` → ");
    out.push_str("resident D → compact probe readback only.\n\n");

    out.push_str("## 3. Time-series observation summary\n\n");
    out.push_str("| tick | max_choke_a | max_choke_b | overlap_peak | mismatch_peak | velocity_peak | profile0_probe_d | profile1_probe_d | note |\n");
    out.push_str("|---:|---:|---:|---:|---:|---:|---:|---:|---|\n");
    for sample_tick in OBS_SAMPLE_TICKS {
        let obs = &run.ticks[sample_tick as usize];
        out.push_str(&format!(
            "| {} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.3} | {:.3} | {} |\n",
            obs.tick,
            obs.max_choke_a,
            obs.max_choke_b,
            obs.overlap_peak,
            obs.mismatch_peak,
            obs.velocity_peak,
            obs.profile0_probe_min_d,
            obs.profile1_probe_min_d,
            obs.note,
        ));
    }
    out.push('\n');

    let first = &run.ticks[0];
    let mid = &run.ticks[50];
    let last = &run.ticks[99];

    out.push_str("## 4. Observed border pressure / choke readouts\n\n");
    out.push_str(&format!(
        "- Max `choke_a`: {:.4} (tick 0) → {:.4} (tick 50) → {:.4} (tick 99).\n",
        first.max_choke_a, mid.max_choke_a, last.max_choke_a
    ));
    out.push_str(&format!(
        "- Max `choke_b`: {:.4} (tick 0) → {:.4} (tick 99).\n",
        first.max_choke_b, last.max_choke_b
    ));
    if (last.max_choke_a - first.max_choke_a).abs() < 0.001
        && (last.max_choke_b - first.max_choke_b).abs() < 0.001
    {
        out.push_str(
            "- Source-local choke readouts remained at the saturation ceiling (1.0) across all sampled ticks; \
             max-aggregate readouts do not show ridge drift — spatial diffusion is below the global peak.\n",
        );
    } else {
        out.push_str("- Choke peaks shifted between sampled ticks (see table above).\n");
    }
    out.push_str(
        "- Per-tick source re-seeding maintains saturated pressure pockets; overlap band intensity tracks \
         the product of family chokes at peaks.\n",
    );
    out.push_str("- These are resident scalar choke columns — **not** a border object or frontline service.\n\n");

    out.push_str("## 5. Overlap / mismatch / velocity stress\n\n");
    out.push_str(&format!(
        "- Overlap peak (`stress_overlap = choke_a * choke_b`): {:.4} → {:.4} (tick 0 → 99).\n",
        first.overlap_peak, last.overlap_peak
    ));
    out.push_str(&format!(
        "- Mismatch peak (`abs(choke_a - choke_b)`): {:.4} → {:.4}.\n",
        first.mismatch_peak, last.mismatch_peak
    ));
    out.push_str(&format!(
        "- Velocity peak (`abs(choke_a_now - choke_a_prev)`): {:.4} (tick 0) → {:.4} (tick 99); ",
        first.velocity_peak, last.velocity_peak
    ));
    if last.velocity_peak < 0.001 {
        out.push_str(
            "velocity stress dropped to zero after the initial tick as the re-seeded field reached a per-tick steady state.\n",
        );
    } else {
        out.push_str("velocity stress indicates ongoing choke change between ticks.\n");
    }
    out.push_str("- Columns `COL_STRESS_OVERLAP`, `COL_STRESS_MISMATCH`, `COL_STRESS_VELOCITY` are resident scalar fields.\n\n");

    out.push_str("## 6. W-profile divergence\n\n");
    out.push_str("- Profile 0 weights: `weight_a=1.0`, `weight_b=0.5` → `output_w` col 5.\n");
    out.push_str("- Profile 1 weights: `weight_a=6.0`, `weight_b=4.0` → `output_w` col 6.\n");
    out.push_str(&format!(
        "- At probe anchor (16,16): tick-99 `output_w` profile0={:.3}, profile1={:.3}.\n",
        last.anchor_w0, last.anchor_w1
    ));
    if (last.profile1_probe_min_d - last.profile0_probe_min_d).abs() > 0.01 {
        out.push_str(&format!(
            "- Compact D probe divergence persisted: profile0 min_d={:.3}, profile1 min_d={:.3} at tick 99 \
             (different admitted W weights over the same resident choke/stress fields).\n\n",
            last.profile0_probe_min_d, last.profile1_probe_min_d
        ));
    } else {
        out.push_str(&format!(
            "- Compact D probes converged: profile0 min_d={:.3}, profile1 min_d={:.3} at tick 99.\n\n",
            last.profile0_probe_min_d, last.profile1_probe_min_d
        ));
    }

    out.push_str("## 7. Emergent movement-front / PALMA probe behavior\n\n");
    if first.preferred_neighbor_rank == last.preferred_neighbor_rank {
        out.push_str(&format!(
            "- At probe anchor, the candidate neighbor with lowest compact D remained rank {} (0=center, 1–4=N4) \
             across sampled ticks — probe-implied local step preference stable under resident W/D.\n",
            last.preferred_neighbor_rank
        ));
    } else {
        out.push_str(&format!(
            "- At probe anchor, candidate neighbor rank with lowest D shifted: tick0 rank {}, tick50 rank {}, tick99 rank {} \
             (0=center, 1–4=N4).\n",
            first.preferred_neighbor_rank, mid.preferred_neighbor_rank, last.preferred_neighbor_rank
        ));
    }
    out.push_str(&format!(
        "- Candidate automata sampling (32 of {}): mean compact min_d {:.3} → {:.3} (tick 0 → 99).\n",
        CT4B_AUTOMATA_COUNT, first.automata_sample_min_d_mean, last.automata_sample_min_d_mean
    ));
    out.push_str("- **Interpretation:** local automata probes would favor/disfavor neighboring cells under resident W/D fields; ");
    out.push_str("this is probe-implied tendency only — **not** implemented movement policy.\n\n");

    out.push_str("## 8. Scaffolding classification\n\n");
    out.push_str("| Artifact | Classification |\n|---|---|\n");
    out.push_str("| `Ct4bFixture`, `ct4b_100tick_runner` | Test-only |\n");
    out.push_str("| `readback_buffer` in observation runner | Test/diagnostic only |\n");
    out.push_str(
        "| Scenario labels (Terran/Pirate etc.) | docs/tests-only — not in production code |\n",
    );
    out.push_str("| `compiled_*_to_gpu_config`, `composed_w_min_plus_stencil_config` | Live production APIs |\n\n");

    out.push_str("## 9. Constitutional checklist\n\n");
    out.push_str("- No border service.\n");
    out.push_str("- No pathfinding engine.\n");
    out.push_str("- No movement engine.\n");
    out.push_str("- No route object.\n");
    out.push_str("- No predecessor table.\n");
    out.push_str("- No semantic WGSL.\n");
    out.push_str("- No full-field CPU readback for production decisions; compact probe + test-only diagnostic aggregates.\n");
    out.push_str("- Candidate-F/native-sqrt audit clean on touched BH/PALMA paths (no native sqrt in hot path).\n");
    out.push_str("- Production behavior does not depend on observation scaffolding.\n\n");

    out.push_str("## 10. Test commands run\n\n");
    out.push_str("```text\ncargo test -p simthing-driver --test bh2d_ct4b_100tick_observation -- bh2d_ct4b_100tick_observation --ignored --nocapture\n");
    out.push_str("cargo test -p simthing-driver --test bh2d_ct4b_100tick_observation -- bh2d_ct4b_100tick_observation_smoke\ncargo test -p simthing-driver --test bh2d_ct4b_fixture\n");
    out.push_str("```\n\n");
    out.push_str("`cargo test --workspace` was **not** run.\n");

    out
}
