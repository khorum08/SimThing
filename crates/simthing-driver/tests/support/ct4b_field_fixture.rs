//! CT-4b test-only fixture (BH-2D): 200×200 generic source-family field columns.
//!
//! Scenario docs may describe 100 source points and 150 local automata; production identifiers
//! stay generic (`field_a`, `field_b`, `choke_a`, `choke_b`). Not a production API.

use simthing_gpu::{
    GpuContext, StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOp, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy,
};

use super::palma_min_plus_oracle::cell_index;

/// CT-4b grid width (pinned BH-2D fixture shape).
pub const CT4B_WIDTH: u32 = 200;
/// CT-4b grid height.
pub const CT4B_HEIGHT: u32 = 200;
/// Total grid cells.
pub const CT4B_CELL_COUNT: u32 = CT4B_WIDTH * CT4B_HEIGHT;
/// Total static source points (50 per source family).
pub const CT4B_SOURCE_COUNT: usize = 100;
/// Source points for generic field A.
pub const CT4B_FIELD_A_SOURCES: usize = 50;
/// Source points for generic field B.
pub const CT4B_FIELD_B_SOURCES: usize = 50;
/// Local automata count (fixture metadata only — no movement policy in BH-2D).
pub const CT4B_AUTOMATA_COUNT: usize = 150;

pub const COL_BASE_W: u32 = 0;
pub const COL_PRESSURE_A: u32 = 1;
pub const COL_CHOKE_A: u32 = 2;
pub const COL_PRESSURE_B: u32 = 3;
pub const COL_CHOKE_B: u32 = 4;
pub const COL_OUTPUT_W_PROFILE_0: u32 = 5;
pub const COL_OUTPUT_W_PROFILE_1: u32 = 6;
pub const COL_STRESS_OVERLAP: u32 = 7;
pub const COL_STRESS_MISMATCH: u32 = 8;
pub const COL_D: u32 = 9;
pub const CT4B_N_DIMS: u32 = 10;

pub const CT4B_DEST: (u32, u32) = (0, 0);
/// Probe anchor within the 64-iteration min-plus reach cone from `CT4B_DEST` (Manhattan 32).
pub const CT4B_PROBE_ANCHOR: (u32, u32) = (16, 16);
pub const CT4B_MIN_PLUS_ITERATIONS: u32 = 64;

const FLUX_U_SAT: f32 = 6.0;
const FLUX_CHI: f32 = 0.2;
const FLUX_HORIZON: u32 = 4;
const SOURCE_PRESSURE_A: f32 = 8.0;
const SOURCE_PRESSURE_B: f32 = 7.5;
const BASE_W: f32 = 1.0;

fn idx(slot: u32, col: u32) -> usize {
    (slot * CT4B_N_DIMS + col) as usize
}

fn hash_slot(salt: u64, attempt: u64, cells: u32) -> u32 {
    let mut x = salt.wrapping_add(attempt.wrapping_mul(0x9E37_79B9_7F4A_7C15));
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51_afd7_ed55_8ccd);
    x ^= x >> 33;
    (x % cells as u64) as u32
}

fn deterministic_source_slots(count: usize, salt: u64) -> Vec<u32> {
    let cells = CT4B_CELL_COUNT;
    let dest_slot = cell_index(
        CT4B_DEST.0 as usize,
        CT4B_DEST.1 as usize,
        CT4B_WIDTH as usize,
    ) as u32;
    let mut slots = Vec::with_capacity(count);
    let mut attempt = 0u64;
    while slots.len() < count {
        let slot = hash_slot(salt, attempt, cells);
        attempt += 1;
        if slot == dest_slot {
            continue;
        }
        if slots.contains(&slot) {
            continue;
        }
        slots.push(slot);
    }
    slots.sort_unstable();
    slots
}

/// Test-only CT-4b fixture: interleaved columns + source-family metadata.
pub struct Ct4bFixture {
    pub values: Vec<f32>,
    pub field_a_sources: Vec<u32>,
    pub field_b_sources: Vec<u32>,
}

impl Ct4bFixture {
    /// Build seeded pressure columns from 100 deterministic source points.
    pub fn build_seeded() -> Self {
        let field_a_sources = deterministic_source_slots(CT4B_FIELD_A_SOURCES, 0xA001);
        let field_b_sources = deterministic_source_slots(CT4B_FIELD_B_SOURCES, 0xB002);
        let mut values = vec![0.0f32; (CT4B_CELL_COUNT * CT4B_N_DIMS) as usize];
        for slot in 0..CT4B_CELL_COUNT {
            values[idx(slot, COL_BASE_W)] = BASE_W;
        }
        for &slot in &field_a_sources {
            values[idx(slot, COL_PRESSURE_A)] = SOURCE_PRESSURE_A;
        }
        for &slot in &field_b_sources {
            values[idx(slot, COL_PRESSURE_B)] = SOURCE_PRESSURE_B;
        }
        // Overlap neighborhood around probe anchor (both families) for local W/stress evidence.
        let (ax, ay) = CT4B_PROBE_ANCHOR;
        let anchor_slot = cell_index(ax as usize, ay as usize, CT4B_WIDTH as usize) as u32;
        values[idx(anchor_slot, COL_PRESSURE_A)] = SOURCE_PRESSURE_A;
        values[idx(anchor_slot, COL_PRESSURE_B)] = SOURCE_PRESSURE_B;
        if ax > 0 {
            let s = cell_index(ax as usize - 1, ay as usize, CT4B_WIDTH as usize) as u32;
            values[idx(s, COL_PRESSURE_A)] = SOURCE_PRESSURE_A * 0.9;
        }
        if ay > 0 {
            let s = cell_index(ax as usize, ay as usize - 1, CT4B_WIDTH as usize) as u32;
            values[idx(s, COL_PRESSURE_B)] = SOURCE_PRESSURE_B * 0.9;
        }
        Self {
            values,
            field_a_sources,
            field_b_sources,
        }
    }

    pub fn values_len(&self) -> usize {
        self.values.len()
    }

    /// Test-only: run one BH-0/BH-1 SaturatingFlux hop with choke readout for one pressure column.
    pub fn apply_gpu_flux_choke(&mut self, ctx: &GpuContext, pressure_col: u32, choke_col: u32) {
        let config = StructuredFieldStencilConfig {
            width: CT4B_WIDTH,
            height: CT4B_HEIGHT,
            n_dims: CT4B_N_DIMS,
            source_col: pressure_col,
            target_col: pressure_col,
            horizon: FLUX_HORIZON,
            alpha_self: 1.0,
            gamma_neighbor: 0.0,
            weight_north: 0.0,
            weight_south: 0.0,
            weight_east: 0.0,
            weight_west: 0.0,
            source_cap: None,
            operator: StructuredFieldStencilOperator::SaturatingFlux {
                u_sat: FLUX_U_SAT,
                chi: FLUX_CHI,
                choke_output_col: Some(choke_col),
            },
            source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
            boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
            mask_mode: StructuredFieldStencilMaskMode::All,
            allow_extended_horizon: false,
        };
        let op = StructuredFieldStencilOp::new(ctx, config).expect("flux op");
        op.upload_values(ctx, &self.values).expect("flux upload");
        let (out, _) = op.run_ping_pong(ctx, FLUX_HORIZON).expect("flux dispatch");
        self.values = out;
    }

    /// Test-only: apply BH-0/BH-1 for both source-family pressure columns.
    pub fn apply_gpu_flux_choke_both_fields(&mut self, ctx: &GpuContext) {
        self.apply_gpu_flux_choke(ctx, COL_PRESSURE_A, COL_CHOKE_A);
        self.apply_gpu_flux_choke(ctx, COL_PRESSURE_B, COL_CHOKE_B);
    }

    pub fn choke_a_at(&self, x: u32, y: u32) -> f32 {
        let slot = cell_index(x as usize, y as usize, CT4B_WIDTH as usize) as u32;
        self.values[idx(slot, COL_CHOKE_A)]
    }

    pub fn choke_b_at(&self, x: u32, y: u32) -> f32 {
        let slot = cell_index(x as usize, y as usize, CT4B_WIDTH as usize) as u32;
        self.values[idx(slot, COL_CHOKE_B)]
    }

    pub fn probe_anchor_candidates() -> Vec<u32> {
        let (x, y) = CT4B_PROBE_ANCHOR;
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
        .filter(|(nx, ny)| {
            *nx >= 0 && *ny >= 0 && *nx < CT4B_WIDTH as i32 && *ny < CT4B_HEIGHT as i32
        })
        .map(|(nx, ny)| cell_index(nx as usize, ny as usize, CT4B_WIDTH as usize) as u32)
        .collect()
    }
}
