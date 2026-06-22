//! GPU-resident text-on-path table (LR6D).

use bevy::{prelude::*, render::extract_resource::ExtractResource};

pub const MAX_PATH_SLOTS: usize = 16;

/// Path slot index on glyph instances.
pub type TextPathSlot = u16;

/// Semantic-free path kinds encoded in GPU rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextPathKind {
    #[default]
    None = 0,
    Arc = 1,
    QuadraticBezier = 2,
    CubicBezier = 3,
    SampledPolyline = 4,
}

impl TextPathKind {
    pub fn to_gpu(self) -> f32 {
        self as u8 as f32
    }
}

/// CPU-side path row authoring.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextPathParams {
    pub kind: TextPathKind,
    pub start: [f32; 2],
    pub control0: [f32; 2],
    pub control1: [f32; 2],
    pub end: [f32; 2],
    pub length: f32,
}

impl Default for TextPathParams {
    fn default() -> Self {
        Self {
            kind: TextPathKind::None,
            start: [0.0, 0.0],
            control0: [0.0, 0.0],
            control1: [0.0, 0.0],
            end: [0.0, 0.0],
            length: 0.0,
        }
    }
}

impl TextPathParams {
    pub fn arc(start: [f32; 2], end: [f32; 2], radius: f32) -> Self {
        let center = [
            (start[0] + end[0]) * 0.5,
            (start[1] + end[1]) * 0.5 - radius,
        ];
        Self {
            kind: TextPathKind::Arc,
            start,
            end,
            control0: center,
            length: radius,
            ..Default::default()
        }
    }

    pub fn quadratic_bezier(start: [f32; 2], control: [f32; 2], end: [f32; 2]) -> Self {
        Self {
            kind: TextPathKind::QuadraticBezier,
            start,
            control0: control,
            control1: [0.0, 0.0],
            end,
            length: 1.0,
        }
    }
}

/// GPU path sample for polyline paths.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct TextPathSampleGpu {
    /// x, y, tangent_x, tangent_y
    pub pos_tangent: [f32; 4],
}

/// GPU path row consumed by `text_instanced.wgsl`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct TextPathRowGpu {
    /// x = kind, y = length/radius, z = sample_count, w reserved
    pub params0: [f32; 4],
    pub start: [f32; 4],
    pub control0: [f32; 4],
    pub control1: [f32; 4],
    pub end: [f32; 4],
}

impl TextPathParams {
    pub fn to_gpu(&self) -> TextPathRowGpu {
        TextPathRowGpu {
            params0: [self.kind.to_gpu(), self.length, 0.0, 0.0],
            start: [self.start[0], self.start[1], 0.0, 0.0],
            control0: [self.control0[0], self.control0[1], 0.0, 0.0],
            control1: [self.control1[0], self.control1[1], 0.0, 0.0],
            end: [self.end[0], self.end[1], 0.0, 0.0],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextPathTable {
    rows: Vec<TextPathRowGpu>,
    slot_count: u16,
}

impl Default for TextPathTable {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl TextPathTable {
    pub fn with_defaults() -> Self {
        let mut rows = vec![TextPathRowGpu::default(); MAX_PATH_SLOTS];
        rows[0] = TextPathParams::default().to_gpu();
        Self {
            rows,
            slot_count: 1,
        }
    }

    pub fn set_row(&mut self, slot: TextPathSlot, row: TextPathParams) -> Result<(), PathError> {
        let idx = slot as usize;
        if idx >= MAX_PATH_SLOTS {
            return Err(PathError::SlotOutOfRange(slot));
        }
        if idx >= self.rows.len() {
            self.rows.resize(idx + 1, TextPathRowGpu::default());
        }
        self.rows[idx] = row.to_gpu();
        if slot >= self.slot_count {
            self.slot_count = slot + 1;
        }
        Ok(())
    }

    pub fn to_rows_uniform(&self) -> TextPathRowsUniform {
        let mut rows = [TextPathRowGpu::default(); MAX_PATH_SLOTS];
        for (dst, src) in rows.iter_mut().zip(self.rows.iter()) {
            *dst = *src;
        }
        TextPathRowsUniform { rows }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextPathRowsUniform {
    pub rows: [TextPathRowGpu; MAX_PATH_SLOTS],
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PathError {
    #[error("path slot {0} out of range (max {MAX_PATH_SLOTS})")]
    SlotOutOfRange(TextPathSlot),
}

#[inline]
pub fn path_params_for_slot(
    path_slot: TextPathSlot,
    path_u_offset: f32,
    path_u_scale: f32,
) -> [f32; 4] {
    [f32::from(path_slot), path_u_offset, path_u_scale, 0.0]
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TextPathWarpDiagnostics {
    pub path_table_upload_count: u64,
    pub warp_table_upload_count: u64,
    pub path_table_cache_hit_count: u64,
    pub warp_table_cache_hit_count: u64,
    pub path_instance_count: u64,
    pub warp_instance_count: u64,
    pub path_warp_rebuild_count: u64,
    pub path_warp_noop_reuse_count: u64,
}

#[derive(Resource, Clone, Debug, PartialEq)]
pub struct TextPathTableResource {
    pub table: TextPathTable,
    pub rows_dirty: bool,
    pub rows_generation: u64,
}

impl Default for TextPathTableResource {
    fn default() -> Self {
        Self {
            table: TextPathTable::with_defaults(),
            rows_dirty: true,
            rows_generation: 0,
        }
    }
}

impl TextPathTableResource {
    pub fn set_row(&mut self, slot: TextPathSlot, row: TextPathParams) -> Result<(), PathError> {
        self.table.set_row(slot, row)?;
        self.rows_dirty = true;
        Ok(())
    }

    pub fn mark_rows_clean(&mut self) {
        self.rows_dirty = false;
    }
}

#[derive(Resource, Clone, Debug, PartialEq, ExtractResource)]
pub struct ExtractedTextPathTable {
    pub rows: TextPathRowsUniform,
    pub rows_generation: u64,
}

impl Default for ExtractedTextPathTable {
    fn default() -> Self {
        let table = TextPathTable::with_defaults();
        Self {
            rows: table.to_rows_uniform(),
            rows_generation: 0,
        }
    }
}

pub fn test_path_table_arc() -> TextPathTable {
    let mut table = TextPathTable::with_defaults();
    table
        .set_row(1, TextPathParams::arc([80.0, 90.0], [180.0, 90.0], 50.0))
        .expect("slot 1");
    table
}

pub fn test_path_table_quadratic_bezier() -> TextPathTable {
    let mut table = TextPathTable::with_defaults();
    table
        .set_row(
            2,
            TextPathParams::quadratic_bezier([60.0, 100.0], [128.0, 20.0], [196.0, 100.0]),
        )
        .expect("slot 2");
    table
}
