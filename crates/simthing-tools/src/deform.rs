//! GPU-resident parametric glyph deformation table (LR6C).

use bevy::{prelude::*, render::extract_resource::ExtractResource};

pub const MAX_DEFORM_SLOTS: usize = 32;
pub const DEFORM_TESS_LEVEL_FLAT: u16 = 0;
pub const DEFORM_TESS_LEVEL_DEFORM: u16 = 4;

/// Deformation slot index on glyph instances.
pub type TextDeformSlot = u16;

/// Semantic-free deformation kinds encoded in GPU rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextDeformKind {
    #[default]
    None = 0,
    Scale = 1,
    Stretch = 2,
    Skew = 3,
    Fold = 4,
    PulseScale = 5,
}

impl TextDeformKind {
    pub fn to_gpu(self) -> f32 {
        self as u8 as f32
    }

    pub fn needs_tessellation(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// CPU-side deformation row authoring.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextDeformParams {
    pub kind: TextDeformKind,
    pub amount_x: f32,
    pub amount_y: f32,
    pub shear_x: f32,
    pub shear_y: f32,
    pub fold_axis_x: f32,
    pub fold_axis_y: f32,
    pub fold_amount: f32,
    pub phase: f32,
}

impl Default for TextDeformParams {
    fn default() -> Self {
        Self {
            kind: TextDeformKind::None,
            amount_x: 0.0,
            amount_y: 0.0,
            shear_x: 0.0,
            shear_y: 0.0,
            fold_axis_x: 1.0,
            fold_axis_y: 0.0,
            fold_amount: 0.0,
            phase: 0.0,
        }
    }
}

impl TextDeformParams {
    pub fn stretch(amount_x: f32, amount_y: f32) -> Self {
        Self {
            kind: TextDeformKind::Stretch,
            amount_x,
            amount_y,
            ..Default::default()
        }
    }

    pub fn skew(shear_x: f32, shear_y: f32) -> Self {
        Self {
            kind: TextDeformKind::Skew,
            shear_x,
            shear_y,
            ..Default::default()
        }
    }

    pub fn fold(axis_x: f32, axis_y: f32, amount: f32) -> Self {
        Self {
            kind: TextDeformKind::Fold,
            fold_axis_x: axis_x,
            fold_axis_y: axis_y,
            fold_amount: amount,
            ..Default::default()
        }
    }

    pub fn pulse_scale(amplitude: f32, phase: f32) -> Self {
        Self {
            kind: TextDeformKind::PulseScale,
            amount_x: amplitude,
            phase,
            ..Default::default()
        }
    }
}

/// GPU deformation row consumed by `text_instanced.wgsl`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct TextDeformRowGpu {
    /// x = kind, y = amount_x, z = amount_y, w = phase
    pub params0: [f32; 4],
    /// x = shear_x, y = shear_y, z = fold_axis_x, w = fold_axis_y
    pub params1: [f32; 4],
    /// x = fold_amount, yzw reserved
    pub params2: [f32; 4],
}

impl TextDeformParams {
    pub fn to_gpu(&self) -> TextDeformRowGpu {
        TextDeformRowGpu {
            params0: [self.kind.to_gpu(), self.amount_x, self.amount_y, self.phase],
            params1: [
                self.shear_x,
                self.shear_y,
                self.fold_axis_x,
                self.fold_axis_y,
            ],
            params2: [self.fold_amount, 0.0, 0.0, 0.0],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextDeformTable {
    rows: Vec<TextDeformRowGpu>,
    slot_count: u16,
}

impl Default for TextDeformTable {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl TextDeformTable {
    pub fn with_defaults() -> Self {
        let mut rows = vec![TextDeformRowGpu::default(); MAX_DEFORM_SLOTS];
        rows[0] = TextDeformParams::default().to_gpu();
        Self {
            rows,
            slot_count: 1,
        }
    }

    pub fn set_row(
        &mut self,
        slot: TextDeformSlot,
        row: TextDeformParams,
    ) -> Result<(), DeformError> {
        let idx = slot as usize;
        if idx >= MAX_DEFORM_SLOTS {
            return Err(DeformError::SlotOutOfRange(slot));
        }
        if idx >= self.rows.len() {
            self.rows.resize(idx + 1, TextDeformRowGpu::default());
        }
        self.rows[idx] = row.to_gpu();
        if slot >= self.slot_count {
            self.slot_count = slot + 1;
        }
        Ok(())
    }

    pub fn to_rows_uniform(&self) -> TextDeformRowsUniform {
        let mut rows = [TextDeformRowGpu::default(); MAX_DEFORM_SLOTS];
        for (dst, src) in rows.iter_mut().zip(self.rows.iter()) {
            *dst = *src;
        }
        TextDeformRowsUniform { rows }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextDeformRowsUniform {
    pub rows: [TextDeformRowGpu; MAX_DEFORM_SLOTS],
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum DeformError {
    #[error("deform slot {0} out of range (max {MAX_DEFORM_SLOTS})")]
    SlotOutOfRange(TextDeformSlot),
}

#[inline]
pub fn deform_params_for_slot(deform_slot: TextDeformSlot, tess_level: u16) -> [f32; 4] {
    [f32::from(deform_slot), f32::from(tess_level), 0.0, 0.0]
}

pub fn tess_level_for_deform_slot(deform_slot: TextDeformSlot) -> u16 {
    if deform_slot == 0 {
        DEFORM_TESS_LEVEL_FLAT
    } else {
        DEFORM_TESS_LEVEL_DEFORM
    }
}

/// Vertex count for a tessellated mesh at the given subdivision level.
pub fn tessellated_vertex_count(subdivisions: u32) -> u32 {
    let n = subdivisions + 1;
    n * n
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TextDeformDiagnostics {
    pub deform_table_upload_count: u64,
    pub deform_table_cache_hit_count: u64,
    pub deform_instance_count: u64,
    pub tessellated_label_count: u64,
    pub tessellated_vertex_count: u64,
    pub deformation_rebuild_count: u64,
    pub deformation_noop_reuse_count: u64,
}

#[derive(Resource, Clone, Debug, PartialEq)]
pub struct TextDeformTableResource {
    pub table: TextDeformTable,
    pub rows_dirty: bool,
    pub rows_generation: u64,
}

impl Default for TextDeformTableResource {
    fn default() -> Self {
        Self {
            table: TextDeformTable::with_defaults(),
            rows_dirty: true,
            rows_generation: 0,
        }
    }
}

impl TextDeformTableResource {
    pub fn set_row(
        &mut self,
        slot: TextDeformSlot,
        row: TextDeformParams,
    ) -> Result<(), DeformError> {
        self.table.set_row(slot, row)?;
        self.rows_dirty = true;
        Ok(())
    }

    pub fn mark_rows_clean(&mut self) {
        self.rows_dirty = false;
    }
}

#[derive(Resource, Clone, Debug, PartialEq, ExtractResource)]
pub struct ExtractedTextDeformTable {
    pub rows: TextDeformRowsUniform,
    pub rows_generation: u64,
}

impl Default for ExtractedTextDeformTable {
    fn default() -> Self {
        let table = TextDeformTable::with_defaults();
        Self {
            rows: table.to_rows_uniform(),
            rows_generation: 0,
        }
    }
}

#[derive(Resource, Clone)]
pub struct TextDeformTessMesh(pub Handle<Mesh>);

pub fn test_deform_table_stretch() -> TextDeformTable {
    let mut table = TextDeformTable::with_defaults();
    table
        .set_row(1, TextDeformParams::stretch(0.35, 0.0))
        .expect("slot 1");
    table
}

pub fn test_deform_table_skew() -> TextDeformTable {
    let mut table = TextDeformTable::with_defaults();
    table
        .set_row(2, TextDeformParams::skew(0.75, 0.0))
        .expect("slot 2");
    table
}

pub fn test_deform_table_fold() -> TextDeformTable {
    let mut table = TextDeformTable::with_defaults();
    table
        .set_row(3, TextDeformParams::fold(1.0, 0.0, 0.5))
        .expect("slot 3");
    table
}
