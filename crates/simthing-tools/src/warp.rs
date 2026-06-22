//! GPU-resident warp field / control lattice table (LR6D).

use bevy::{prelude::*, render::extract_resource::ExtractResource};

pub const MAX_WARP_SLOTS: usize = 16;

/// Warp slot index on glyph instances.
pub type TextWarpSlot = u16;

/// Semantic-free warp kinds encoded in GPU rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextWarpKind {
    #[default]
    None = 0,
    Affine = 1,
    Lattice2x2 = 2,
    Lattice3x3 = 3,
    RadialBend = 4,
}

impl TextWarpKind {
    pub fn to_gpu(self) -> f32 {
        self as u8 as f32
    }
}

/// CPU-side warp row authoring.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextWarpParams {
    pub kind: TextWarpKind,
    pub strength: f32,
    pub phase: f32,
    pub control_points: [[f32; 2]; 4],
}

impl Default for TextWarpParams {
    fn default() -> Self {
        Self {
            kind: TextWarpKind::None,
            strength: 0.0,
            phase: 0.0,
            control_points: [[0.0; 2]; 4],
        }
    }
}

impl TextWarpParams {
    pub fn lattice2x2(strength: f32, offsets: [[f32; 2]; 4]) -> Self {
        Self {
            kind: TextWarpKind::Lattice2x2,
            strength,
            control_points: offsets,
            ..Default::default()
        }
    }

    pub fn radial_bend(strength: f32, phase: f32) -> Self {
        Self {
            kind: TextWarpKind::RadialBend,
            strength,
            phase,
            ..Default::default()
        }
    }
}

/// GPU warp control point.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct TextWarpControlPointGpu {
    pub xy_weight: [f32; 4],
}

/// GPU warp row consumed by `text_instanced.wgsl`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct TextWarpRowGpu {
    /// x = kind, y = strength, z = phase, w reserved
    pub params0: [f32; 4],
    pub points0: [f32; 4],
    pub points1: [f32; 4],
    pub points2: [f32; 4],
    pub points3: [f32; 4],
}

impl TextWarpParams {
    pub fn to_gpu(&self) -> TextWarpRowGpu {
        TextWarpRowGpu {
            params0: [self.kind.to_gpu(), self.strength, self.phase, 0.0],
            points0: [
                self.control_points[0][0],
                self.control_points[0][1],
                0.0,
                0.0,
            ],
            points1: [
                self.control_points[1][0],
                self.control_points[1][1],
                0.0,
                0.0,
            ],
            points2: [
                self.control_points[2][0],
                self.control_points[2][1],
                0.0,
                0.0,
            ],
            points3: [
                self.control_points[3][0],
                self.control_points[3][1],
                0.0,
                0.0,
            ],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextWarpTable {
    rows: Vec<TextWarpRowGpu>,
    slot_count: u16,
}

impl Default for TextWarpTable {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl TextWarpTable {
    pub fn with_defaults() -> Self {
        let mut rows = vec![TextWarpRowGpu::default(); MAX_WARP_SLOTS];
        rows[0] = TextWarpParams::default().to_gpu();
        Self {
            rows,
            slot_count: 1,
        }
    }

    pub fn set_row(&mut self, slot: TextWarpSlot, row: TextWarpParams) -> Result<(), WarpError> {
        let idx = slot as usize;
        if idx >= MAX_WARP_SLOTS {
            return Err(WarpError::SlotOutOfRange(slot));
        }
        if idx >= self.rows.len() {
            self.rows.resize(idx + 1, TextWarpRowGpu::default());
        }
        self.rows[idx] = row.to_gpu();
        if slot >= self.slot_count {
            self.slot_count = slot + 1;
        }
        Ok(())
    }

    pub fn to_rows_uniform(&self) -> TextWarpRowsUniform {
        let mut rows = [TextWarpRowGpu::default(); MAX_WARP_SLOTS];
        for (dst, src) in rows.iter_mut().zip(self.rows.iter()) {
            *dst = *src;
        }
        TextWarpRowsUniform { rows }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextWarpRowsUniform {
    pub rows: [TextWarpRowGpu; MAX_WARP_SLOTS],
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum WarpError {
    #[error("warp slot {0} out of range (max {MAX_WARP_SLOTS})")]
    SlotOutOfRange(TextWarpSlot),
}

#[inline]
pub fn warp_params_for_slot(warp_slot: TextWarpSlot, strength_mul: f32) -> [f32; 4] {
    [f32::from(warp_slot), strength_mul, 0.0, 0.0]
}

#[derive(Resource, Clone, Debug, PartialEq)]
pub struct TextWarpTableResource {
    pub table: TextWarpTable,
    pub rows_dirty: bool,
    pub rows_generation: u64,
}

impl Default for TextWarpTableResource {
    fn default() -> Self {
        Self {
            table: TextWarpTable::with_defaults(),
            rows_dirty: true,
            rows_generation: 0,
        }
    }
}

impl TextWarpTableResource {
    pub fn set_row(&mut self, slot: TextWarpSlot, row: TextWarpParams) -> Result<(), WarpError> {
        self.table.set_row(slot, row)?;
        self.rows_dirty = true;
        Ok(())
    }

    pub fn mark_rows_clean(&mut self) {
        self.rows_dirty = false;
    }
}

#[derive(Resource, Clone, Debug, PartialEq, ExtractResource)]
pub struct ExtractedTextWarpTable {
    pub rows: TextWarpRowsUniform,
    pub rows_generation: u64,
}

impl Default for ExtractedTextWarpTable {
    fn default() -> Self {
        let table = TextWarpTable::with_defaults();
        Self {
            rows: table.to_rows_uniform(),
            rows_generation: 0,
        }
    }
}

pub fn test_warp_table_lattice2x2() -> TextWarpTable {
    let mut table = TextWarpTable::with_defaults();
    table
        .set_row(
            1,
            TextWarpParams::lattice2x2(1.0, [[0.0, 0.0], [24.0, 0.0], [0.0, 18.0], [24.0, 18.0]]),
        )
        .expect("slot 1");
    table
}
