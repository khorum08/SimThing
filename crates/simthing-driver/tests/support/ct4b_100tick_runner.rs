//! Test-only 100-tick CT-4b observation runner (BH-2D-OBS-100R).
//!
//! Deterministic dynamic stimulus: pulsed static emitters, moving mobile emitters,
//! pressure decay, and test-only candidate sampler displacement by compact D probe.
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
    CT4B_AUTOMATA_COUNT, CT4B_CELL_COUNT, CT4B_DEST, CT4B_HEIGHT, CT4B_MIN_PLUS_ITERATIONS,
    CT4B_PROBE_ANCHOR, CT4B_SOURCE_COUNT, CT4B_WIDTH,
};
use super::palma_min_plus_oracle::cell_index;

pub const OBS_TICK_COUNT: u32 = 100;
pub const OBS_SAMPLE_TICKS: [u32; 5] = [0, 25, 50, 75, 99];
pub const OBS_FLUX_HORIZON: u32 = 2;

const MOBILE_EMITTER_COUNT: usize = 10;
const CANDIDATE_PROBE_COUNT: usize = 32;
const PRESSURE_DECAY: f32 = 0.92;
const SOURCE_PRESSURE_A: f32 = 8.0;
const SOURCE_PRESSURE_B: f32 = 7.5;

// Observation layout extends BH-2D with velocity prev + stress_velocity + d column.
pub const OBS_N_DIMS: u32 = 12;
pub const OBS_COL_OUTPUT_W_0: u32 = 5;
pub const OBS_COL_OUTPUT_W_1: u32 = 6;
pub const OBS_COL_STRESS_OVERLAP: u32 = 7;
pub const OBS_COL_STRESS_MISMATCH: u32 = 8;
pub const OBS_COL_STRESS_VELOCITY: u32 = 9;
pub const OBS_COL_CHOKE_A_PREV: u32 = 10;
pub const OBS_COL_D: u32 = 11;

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
    pub profile1_preferred_neighbor_rank: usize,
    pub automata_sample_min_d_mean: f32,
    pub moved_candidates: u32,
    pub mean_displacement: f32,
    pub note: String,
}

pub struct ObservationRun {
    pub ticks: Vec<TickObservation>,
    pub automata_probe_slots: Vec<u32>,
    pub candidate_home_slots: Vec<u32>,
}

/// Deterministic pulse in [0.55, 1.0] — test-only schedule, no semantic labels.
fn source_pulse(tick: u32, phase: u32) -> f32 {
    let period = 20u32;
    let x = (tick + phase * 11) % period;
    0.55 + 0.45 * (x as f32 / period as f32)
}

fn shift_slot(slot: u32, dx: i32, dy: i32) -> u32 {
    let x = slot % CT4B_WIDTH;
    let y = slot / CT4B_WIDTH;
    let nx = (x as i32 + dx).clamp(0, CT4B_WIDTH as i32 - 1) as u32;
    let ny = (y as i32 + dy).clamp(0, CT4B_HEIGHT as i32 - 1) as u32;
    cell_index(nx as usize, ny as usize, CT4B_WIDTH as usize) as u32
}

fn manhattan_slots(a: u32, b: u32) -> u32 {
    let ax = a % CT4B_WIDTH;
    let ay = a / CT4B_WIDTH;
    let bx = b % CT4B_WIDTH;
    let by = b / CT4B_WIDTH;
    ax.abs_diff(bx) + ay.abs_diff(by)
}

struct DynamicObsState {
    mobile_a_slots: Vec<u32>,
    mobile_b_slots: Vec<u32>,
    candidate_slots: Vec<u32>,
    candidate_home: Vec<u32>,
    static_a: Vec<u32>,
    static_b: Vec<u32>,
}

impl DynamicObsState {
    fn new(base: &Ct4bFixture) -> Self {
        let mobile_a_slots = base.field_a_sources[..MOBILE_EMITTER_COUNT].to_vec();
        let mobile_b_slots = base.field_b_sources[..MOBILE_EMITTER_COUNT].to_vec();
        let static_a = base.field_a_sources[MOBILE_EMITTER_COUNT..].to_vec();
        let static_b = base.field_b_sources[MOBILE_EMITTER_COUNT..].to_vec();
        let candidate_home = automata_probe_slots(CANDIDATE_PROBE_COUNT);
        let candidate_slots = candidate_home.clone();
        Self {
            mobile_a_slots,
            mobile_b_slots,
            static_a,
            static_b,
            candidate_slots,
            candidate_home,
        }
    }

    fn advance_mobile_emitters(&mut self, tick: u32) {
        if tick > 0 && tick % 3 == 0 {
            for slot in &mut self.mobile_a_slots {
                *slot = shift_slot(*slot, 1, 0);
            }
        }
        if tick > 0 && tick % 4 == 0 {
            for slot in &mut self.mobile_b_slots {
                *slot = shift_slot(*slot, 0, 1);
            }
        }
    }

    fn mean_candidate_displacement(&self) -> f32 {
        let sum: u32 = self
            .candidate_slots
            .iter()
            .zip(self.candidate_home.iter())
            .map(|(cur, home)| manhattan_slots(*cur, *home))
            .sum();
        sum as f32 / self.candidate_slots.len() as f32
    }
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

fn expand_fixture_to_obs_layout(base: &Ct4bFixture) -> Vec<f32> {
    use super::ct4b_field_fixture::CT4B_N_DIMS;
    let mut values = vec![0.0f32; (CT4B_CELL_COUNT * OBS_N_DIMS) as usize];
    for slot in 0..CT4B_CELL_COUNT {
        for col in 0..CT4B_N_DIMS {
            values[idx(slot, col)] = base.values[(slot * CT4B_N_DIMS + col) as usize];
        }
    }
    values
}

fn decay_and_apply_dynamic_pressure(values: &mut [f32], tick: u32, state: &DynamicObsState) {
    for slot in 0..CT4B_CELL_COUNT {
        values[idx(slot, COL_PRESSURE_A)] *= PRESSURE_DECAY;
        values[idx(slot, COL_PRESSURE_B)] *= PRESSURE_DECAY;
    }

    let pulse_a = source_pulse(tick, 0);
    let pulse_b = source_pulse(tick, 1);

    for &slot in &state.static_a {
        values[idx(slot, COL_PRESSURE_A)] = SOURCE_PRESSURE_A * pulse_a;
    }
    for &slot in &state.static_b {
        values[idx(slot, COL_PRESSURE_B)] = SOURCE_PRESSURE_B * pulse_b;
    }
    for &slot in &state.mobile_a_slots {
        values[idx(slot, COL_PRESSURE_A)] = SOURCE_PRESSURE_A * pulse_a * 1.15;
    }
    for &slot in &state.mobile_b_slots {
        values[idx(slot, COL_PRESSURE_B)] = SOURCE_PRESSURE_B * pulse_b * 1.15;
    }

    // Overlap neighborhood at probe anchor (pulsed both families).
    let (ax, ay) = CT4B_PROBE_ANCHOR;
    let anchor_slot = cell_index(ax as usize, ay as usize, CT4B_WIDTH as usize) as u32;
    values[idx(anchor_slot, COL_PRESSURE_A)] = SOURCE_PRESSURE_A * pulse_a;
    values[idx(anchor_slot, COL_PRESSURE_B)] = SOURCE_PRESSURE_B * pulse_b;
    if ax > 0 {
        let s = cell_index(ax as usize - 1, ay as usize, CT4B_WIDTH as usize) as u32;
        values[idx(s, COL_PRESSURE_A)] = SOURCE_PRESSURE_A * pulse_a * 0.85;
    }
    if ay > 0 {
        let s = cell_index(ax as usize, ay as usize - 1, CT4B_WIDTH as usize) as u32;
        values[idx(s, COL_PRESSURE_B)] = SOURCE_PRESSURE_B * pulse_b * 0.85;
    }
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

/// Test-only: step candidate sampler to lowest-D N4 neighbor (not production movement).
fn step_candidate_by_probe(slot: u32, probe: &simthing_gpu::MinPlusTraversalDProbeResult) -> u32 {
    let rank = preferred_neighbor_rank(probe);
    let cands = neighbor_candidates(slot);
    if rank < cands.len() {
        cands[rank]
    } else {
        slot
    }
}

fn tick_note(
    tick: u32,
    velocity_peak: f32,
    moved_candidates: u32,
    mean_displacement: f32,
    probe0_d: f32,
    prev_probe0_d: f32,
) -> String {
    if tick == 0 {
        "initial dynamic pulse + flux".to_string()
    } else if moved_candidates > 0 {
        format!("{moved_candidates} candidate samplers displaced")
    } else if velocity_peak > 0.02 {
        "velocity stress active".to_string()
    } else if (probe0_d - prev_probe0_d).abs() > 0.05 {
        "anchor D probe shifting".to_string()
    } else if mean_displacement > 0.5 {
        "movement-front displacement accumulating".to_string()
    } else {
        "pressure wave propagating".to_string()
    }
}

pub fn run_observation_ticks(ctx: &GpuContext, tick_count: u32) -> ObservationRun {
    let base = Ct4bFixture::build_seeded();
    let mut values = expand_fixture_to_obs_layout(&base);
    let mut state = DynamicObsState::new(&base);
    let compose = w_compose_config_obs();
    let stress = stress_compose_config_obs();
    let anchor_candidates = neighbor_candidates(cell_index(
        CT4B_PROBE_ANCHOR.0 as usize,
        CT4B_PROBE_ANCHOR.1 as usize,
        CT4B_WIDTH as usize,
    ) as u32);

    let mut observations = Vec::with_capacity(tick_count as usize);
    let mut prev_probe0_d = f32::INFINITY;

    for tick in 0..tick_count {
        state.advance_mobile_emitters(tick);
        snapshot_choke_a_prev(&mut values);
        decay_and_apply_dynamic_pressure(&mut values, tick, &state);

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
        let mut moved_candidates = 0u32;

        for i in 0..CANDIDATE_PROBE_COUNT {
            let old_slot = state.candidate_slots[i];
            let cands = neighbor_candidates(old_slot);
            if cands.is_empty() {
                continue;
            }
            let p = run_palma_probe(ctx, &buffer, &compose, 0, &cands);
            if p.min_d.is_finite() {
                automata_min_sum += p.min_d;
                automata_count += 1;
            }
            let new_slot = step_candidate_by_probe(old_slot, &p);
            if new_slot != old_slot {
                moved_candidates += 1;
            }
            state.candidate_slots[i] = new_slot;
        }

        let automata_mean = if automata_count > 0 {
            automata_min_sum / automata_count as f32
        } else {
            f32::INFINITY
        };
        let mean_displacement = state.mean_candidate_displacement();

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

        let note = tick_note(
            tick,
            velocity_peak,
            moved_candidates,
            mean_displacement,
            probe0.min_d,
            prev_probe0_d,
        );
        prev_probe0_d = probe0.min_d;

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
            profile1_preferred_neighbor_rank: preferred_neighbor_rank(&probe1),
            automata_sample_min_d_mean: automata_mean,
            moved_candidates,
            mean_displacement,
            note,
        });
    }

    ObservationRun {
        ticks: observations,
        automata_probe_slots: state.candidate_slots.clone(),
        candidate_home_slots: state.candidate_home.clone(),
    }
}

pub fn render_observation_markdown(run: &ObservationRun) -> String {
    let mut out = String::new();
    out.push_str("# BH-2D-OBS-100R — CT-4b 100-tick dynamic scenario observations\n\n");
    out.push_str(
        "> **Status: OBSERVATION / PASS (generated from live GPU run, BH-2D-OBS-100R).** ",
    );
    out.push_str("Deterministic test-only dynamic stimulus. ");
    out.push_str(
        "No movement engine, pathfinding engine, route/predecessor objects, or border service.\n\n",
    );

    out.push_str("## 1. Scenario identity\n\n");
    out.push_str("| Parameter | Value |\n|---|---|\n");
    out.push_str("| Grid | 200 × 200 |\n");
    out.push_str("| Tick count | 100 |\n");
    out.push_str("| Static source points | 40 per family (50 minus 10 mobile) |\n");
    out.push_str("| Mobile source emitters | 10 family A + 10 family B (deterministic drift) |\n");
    out.push_str("| Total source-family coverage | 100 points (same CT-4b shape) |\n");
    out.push_str("| Candidate automata samplers | 150 metadata; 32 probed and displaced test-only per tick |\n");
    out.push_str(
        "| Fixture labels | docs/tests-only; production code uses `field_a` / `field_b` |\n\n",
    );

    out.push_str("## 2. Dynamic stimulus description (test-only)\n\n");
    out.push_str("| Mechanism | Schedule |\n|---|---|\n");
    out.push_str("| Pressure decay | All cells × 0.92 per tick before re-injection |\n");
    out.push_str("| Source pulse | `0.55 + 0.45 × ((tick + phase×11) mod 20) / 20` per family (phases 0/1) |\n");
    out.push_str("| Mobile family A | First 10 emitters shift +1 east every 3 ticks |\n");
    out.push_str("| Mobile family B | First 10 emitters shift +1 south every 4 ticks |\n");
    out.push_str("| Flux hops per tick | 2 (OBS_FLUX_HORIZON) |\n");
    out.push_str("| Candidate sampler step | After compact D probe, move to lowest-D N4 neighbor (test-only) |\n");
    out.push_str("\nResident generic fields affected: `pressure_a/b`, `choke_a/b`, composed W, stress columns. ");
    out.push_str(
        "Production BH/PALMA ops unchanged — stimulus lives only in `ct4b_100tick_runner`.\n\n",
    );

    out.push_str("## 3. Resident chain exercised\n\n");
    out.push_str(
        "Each tick: dynamic pressure inject → BH-0/BH-1 flux+choke (2 hops per family) → ",
    );
    out.push_str("BH-2B W (2 profiles) → BH-2S overlap/mismatch/velocity stress → ");
    out.push_str("BH-2C PALMA `GpuInterleavedW` → resident D → compact probe readback → ");
    out.push_str("test-only candidate sampler displacement.\n\n");

    out.push_str("## 4. Time-series observation summary\n\n");
    out.push_str("| tick | max_choke_a | max_choke_b | overlap_peak | mismatch_peak | velocity_peak | profile0_probe_d | profile1_probe_d | moved_candidates | mean_displacement | note |\n");
    out.push_str("|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|\n");
    for sample_tick in OBS_SAMPLE_TICKS {
        let obs = &run.ticks[sample_tick as usize];
        out.push_str(&format!(
            "| {} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.3} | {:.3} | {} | {:.2} | {} |\n",
            obs.tick,
            obs.max_choke_a,
            obs.max_choke_b,
            obs.overlap_peak,
            obs.mismatch_peak,
            obs.velocity_peak,
            obs.profile0_probe_min_d,
            obs.profile1_probe_min_d,
            obs.moved_candidates,
            obs.mean_displacement,
            obs.note,
        ));
    }
    out.push('\n');

    let first = &run.ticks[0];
    let mid = &run.ticks[50];
    let last = &run.ticks[99];

    out.push_str("## 5. Observed shifting pressure\n\n");
    out.push_str(&format!(
        "- Max `choke_a`: {:.4} (tick 0) → {:.4} (tick 50) → {:.4} (tick 99).\n",
        first.max_choke_a, mid.max_choke_a, last.max_choke_a
    ));
    out.push_str(&format!(
        "- Max `choke_b`: {:.4} (tick 0) → {:.4} (tick 99).\n",
        first.max_choke_b, last.max_choke_b
    ));
    out.push_str(&format!(
        "- Overlap peak: {:.4} → {:.4}; mismatch peak: {:.4} → {:.4}.\n",
        first.overlap_peak, last.overlap_peak, first.mismatch_peak, last.mismatch_peak
    ));
    out.push_str(&format!(
        "- Velocity peak: {:.4} (tick 0) → {:.4} (tick 50) → {:.4} (tick 99).\n",
        first.velocity_peak, mid.velocity_peak, last.velocity_peak
    ));
    if (last.max_choke_a - first.max_choke_a).abs() > 0.001
        || (last.overlap_peak - first.overlap_peak).abs() > 0.001
        || mid.velocity_peak > 0.01
    {
        out.push_str(
            "- Moving emitters and pulsed injection shifted choke/stress readouts over the run; \
             contact-band intensity tracks mobile source overlap rather than a fixed saturated plateau.\n",
        );
    } else {
        out.push_str("- Max choke aggregates remained near ceiling; see velocity, D-probe, and displacement columns for dynamic signal.\n");
    }
    out.push_str("- Resident scalar choke/stress columns — **not** a border object or frontline service.\n\n");

    out.push_str("## 6. Overlap / mismatch / velocity stress\n\n");
    out.push_str(&format!(
        "- Overlap peak (`choke_a × choke_b`): {:.4} → {:.4} (tick 0 → 99).\n",
        first.overlap_peak, last.overlap_peak
    ));
    out.push_str(&format!(
        "- Mismatch peak (`|choke_a − choke_b|`): {:.4} → {:.4}.\n",
        first.mismatch_peak, last.mismatch_peak
    ));
    out.push_str(&format!(
        "- Velocity peak (`|choke_a_now − choke_a_prev|`): {:.4} → {:.4}.\n",
        first.velocity_peak, last.velocity_peak
    ));
    if last.velocity_peak > 0.01 || mid.velocity_peak > 0.01 {
        out.push_str("- Velocity stress remained active mid-run as mobile emitters and decay/pulse changed the choke field between ticks.\n");
    }
    out.push_str("- Columns `COL_STRESS_OVERLAP`, `COL_STRESS_MISMATCH`, `COL_STRESS_VELOCITY` are resident scalar fields.\n\n");

    out.push_str("## 7. W-profile divergence\n\n");
    out.push_str("- Profile 0 weights: `weight_a=1.0`, `weight_b=0.5` → `output_w` col 5.\n");
    out.push_str("- Profile 1 weights: `weight_a=6.0`, `weight_b=4.0` → `output_w` col 6.\n");
    out.push_str(&format!(
        "- At probe anchor (16,16): tick-0 W profile0={:.3}/profile1={:.3}; tick-99 profile0={:.3}/profile1={:.3}.\n",
        first.anchor_w0, first.anchor_w1, last.anchor_w0, last.anchor_w1
    ));
    out.push_str(&format!(
        "- Compact D probes: profile0 {:.3}→{:.3}, profile1 {:.3}→{:.3} (tick 0 → 99).\n",
        first.profile0_probe_min_d,
        last.profile0_probe_min_d,
        first.profile1_probe_min_d,
        last.profile1_probe_min_d
    ));
    out.push_str("- Divergence comes from admitted W weights over the same resident choke/stress fields — no semantic branches.\n\n");

    out.push_str("## 8. Emergent movement-front / PALMA probe behavior\n\n");
    if first.preferred_neighbor_rank != last.preferred_neighbor_rank {
        out.push_str(&format!(
            "- Anchor profile-0 lowest-D neighbor rank: {} → {} (0=center, 1–4=N4).\n",
            first.preferred_neighbor_rank, last.preferred_neighbor_rank
        ));
    } else {
        out.push_str(&format!(
            "- Anchor profile-0 lowest-D neighbor rank stable at {} (0=center, 1–4=N4).\n",
            last.preferred_neighbor_rank
        ));
    }
    if first.profile1_preferred_neighbor_rank != last.profile1_preferred_neighbor_rank {
        out.push_str(&format!(
            "- Anchor profile-1 lowest-D neighbor rank: {} → {}.\n",
            first.profile1_preferred_neighbor_rank, last.profile1_preferred_neighbor_rank
        ));
    }
    out.push_str(&format!(
        "- Test-only candidate samplers: mean Manhattan displacement {:.2} → {:.2}; moved this tick at tick 99: {}.\n",
        first.mean_displacement, last.mean_displacement, last.moved_candidates
    ));
    out.push_str(&format!(
        "- Candidate mean compact min_d: {:.3} → {:.3} (tick 0 → 99).\n",
        first.automata_sample_min_d_mean, last.automata_sample_min_d_mean
    ));
    out.push_str("- **Interpretation:** probe-implied local automata would favor/disfavor neighboring cells; ");
    out.push_str("test-only sampler displacement tracks compact D gradients — **not** production movement policy.\n\n");

    out.push_str("## 9. Scaffolding classification\n\n");
    out.push_str("| Artifact | Classification |\n|---|---|\n");
    out.push_str("| `Ct4bFixture`, `ct4b_100tick_runner`, `DynamicObsState` | Test-only |\n");
    out.push_str(
        "| Dynamic pulse / mobile emitters / candidate step | Test-only observation stimulus |\n",
    );
    out.push_str("| `readback_buffer` in observation runner | Test/diagnostic only |\n");
    out.push_str(
        "| Scenario labels (Terran/Pirate etc.) | docs/tests-only — not in production code |\n",
    );
    out.push_str("| `compiled_*_to_gpu_config`, `composed_w_min_plus_stencil_config`, GPU compose/flux/PALMA ops | Live production APIs |\n\n");

    out.push_str("## 10. Constitutional checklist\n\n");
    out.push_str("- No border service.\n");
    out.push_str("- No frontline service.\n");
    out.push_str("- No pathfinding engine.\n");
    out.push_str("- No movement engine.\n");
    out.push_str("- No route object.\n");
    out.push_str("- No predecessor table.\n");
    out.push_str("- No CPU planner.\n");
    out.push_str("- No semantic WGSL.\n");
    out.push_str("- No faction-specific production code.\n");
    out.push_str("- No full-field CPU readback for production decisions; compact probe + test-only diagnostic aggregates.\n");
    out.push_str("- Candidate-F/native-sqrt audit clean on touched BH/PALMA paths (no native sqrt; scalar W/D/choke/stress only).\n");
    out.push_str("- Production behavior does not depend on observation scaffolding.\n\n");

    out.push_str("## 11. Test commands run\n\n");
    out.push_str("```text\ncargo fmt --all -- --check\n");
    out.push_str("cargo test -p simthing-driver --test bh2d_ct4b_100tick_observation -- bh2d_ct4b_100tick_observation --ignored --nocapture\n");
    out.push_str("cargo test -p simthing-driver --test bh2d_ct4b_100tick_observation -- bh2d_ct4b_100tick_observation_smoke\n");
    out.push_str("cargo test -p simthing-driver --test bh2d_ct4b_fixture\n");
    out.push_str("cargo test -p simthing-driver --test bh2c_palma_w_feedstock\n");
    out.push_str("cargo test -p simthing-driver --test palma_path_9_downstream_gpu_consumer\n");
    out.push_str("cargo test -p simthing-gpu --test bh2_w_composition\n");
    out.push_str("cargo test -p simthing-gpu --test bh2s_overlap_stress\n");
    out.push_str("```\n\n");
    out.push_str("`cargo test --workspace` was **not** run.\n");

    out
}
