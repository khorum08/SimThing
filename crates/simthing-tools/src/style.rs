//! GPU-resident text/icon style table (LR6B).

use bevy::prelude::Resource;
use bevy::render::extract_resource::ExtractResource;

pub const MAX_STYLE_SLOTS: usize = 32;

/// Style slot index assigned to glyph/icon instances.
pub type TextStyleSlot = u16;

/// Gradient modes encoded in `TextStyleRowGpu.params0.y`.
pub const GRADIENT_MODE_NONE: f32 = 0.0;
pub const GRADIENT_MODE_LINEAR_U: f32 = 1.0;
pub const GRADIENT_MODE_LINEAR_V: f32 = 2.0;

/// GPU style row consumed by `text_instanced.wgsl`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct TextStyleRowGpu {
    pub fill_rgba: [f32; 4],
    pub accent_rgba: [f32; 4],
    pub outline_rgba: [f32; 4],
    pub glow_rgba: [f32; 4],
    /// x = opacity, y = gradient_mode, z = outline_width, w = glow_radius
    pub params0: [f32; 4],
    /// x = pulse_amplitude, y = pulse_frequency, z = pulse_phase, w = reserved
    pub params1: [f32; 4],
}

/// CPU-side authoring row for the style table.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextStyleRow {
    pub fill_rgba: [f32; 4],
    pub accent_rgba: [f32; 4],
    pub outline_rgba: [f32; 4],
    pub glow_rgba: [f32; 4],
    pub opacity: f32,
    pub gradient_mode: f32,
    pub outline_width: f32,
    pub glow_radius: f32,
    pub pulse_amplitude: f32,
    pub pulse_frequency: f32,
    pub pulse_phase: f32,
}

impl Default for TextStyleRow {
    fn default() -> Self {
        Self {
            fill_rgba: [1.0, 1.0, 1.0, 1.0],
            accent_rgba: [1.0, 1.0, 1.0, 1.0],
            outline_rgba: [0.0, 0.0, 0.0, 1.0],
            glow_rgba: [1.0, 1.0, 1.0, 0.5],
            opacity: 1.0,
            gradient_mode: GRADIENT_MODE_NONE,
            outline_width: 0.0,
            glow_radius: 0.0,
            pulse_amplitude: 0.0,
            pulse_frequency: 0.0,
            pulse_phase: 0.0,
        }
    }
}

impl TextStyleRow {
    pub fn solid_fill(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            fill_rgba: [r, g, b, a],
            ..Default::default()
        }
    }

    pub fn linear_gradient_u(fill: [f32; 4], accent: [f32; 4]) -> Self {
        Self {
            fill_rgba: fill,
            accent_rgba: accent,
            gradient_mode: GRADIENT_MODE_LINEAR_U,
            ..Default::default()
        }
    }

    pub fn with_outline(mut self, rgba: [f32; 4], width: f32) -> Self {
        self.outline_rgba = rgba;
        self.outline_width = width;
        self
    }

    pub fn with_glow(mut self, rgba: [f32; 4], radius: f32) -> Self {
        self.glow_rgba = rgba;
        self.glow_radius = radius;
        self
    }

    pub fn with_pulse(mut self, amplitude: f32, frequency: f32, phase: f32) -> Self {
        self.pulse_amplitude = amplitude;
        self.pulse_frequency = frequency;
        self.pulse_phase = phase;
        self
    }

    pub fn to_gpu(&self) -> TextStyleRowGpu {
        TextStyleRowGpu {
            fill_rgba: self.fill_rgba,
            accent_rgba: self.accent_rgba,
            outline_rgba: self.outline_rgba,
            glow_rgba: self.glow_rgba,
            params0: [
                self.opacity,
                self.gradient_mode,
                self.outline_width,
                self.glow_radius,
            ],
            params1: [
                self.pulse_amplitude,
                self.pulse_frequency,
                self.pulse_phase,
                0.0,
            ],
        }
    }
}

/// Fixed-capacity style table uploaded to GPU on change only.
#[derive(Clone, Debug, PartialEq)]
pub struct TextStyleTable {
    rows: Vec<TextStyleRowGpu>,
    slot_count: u16,
}

impl Default for TextStyleTable {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl TextStyleTable {
    pub fn with_defaults() -> Self {
        let mut rows = vec![TextStyleRowGpu::default(); MAX_STYLE_SLOTS];
        rows[0] = TextStyleRow::default().to_gpu();
        Self {
            rows,
            slot_count: 1,
        }
    }

    pub fn slot_count(&self) -> u16 {
        self.slot_count
    }

    pub fn row(&self, slot: TextStyleSlot) -> Option<&TextStyleRowGpu> {
        self.rows.get(slot as usize)
    }

    pub fn set_row(&mut self, slot: TextStyleSlot, row: TextStyleRow) -> Result<(), StyleError> {
        let idx = slot as usize;
        if idx >= MAX_STYLE_SLOTS {
            return Err(StyleError::SlotOutOfRange(slot));
        }
        if idx >= self.rows.len() {
            self.rows.resize(idx + 1, TextStyleRowGpu::default());
        }
        self.rows[idx] = row.to_gpu();
        if slot >= self.slot_count {
            self.slot_count = slot + 1;
        }
        Ok(())
    }

    pub fn gpu_rows(&self) -> &[TextStyleRowGpu] {
        &self.rows
    }
}

/// Globals uploaded each frame for pulse modulation (small buffer, not a full table reupload).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct TextStyleGlobalsGpu {
    pub time: f32,
    pub nameplate_min_focused_px: f32,
    pub nameplate_unselected_global_alpha: f32,
    pub nameplate_min_unselected_px: f32,
}

/// GPU row array uploaded only when style rows change.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextStyleRowsUniform {
    pub rows: [TextStyleRowGpu; MAX_STYLE_SLOTS],
}

impl TextStyleTable {
    pub fn to_rows_uniform(&self) -> TextStyleRowsUniform {
        let mut rows = [TextStyleRowGpu::default(); MAX_STYLE_SLOTS];
        for (dst, src) in rows.iter_mut().zip(self.rows.iter()) {
            *dst = *src;
        }
        TextStyleRowsUniform { rows }
    }

    pub fn to_globals(&self, time: f32) -> TextStyleGlobalsGpu {
        TextStyleGlobalsGpu {
            time,
            nameplate_min_focused_px: 16.0,
            nameplate_unselected_global_alpha: 1.0,
            nameplate_min_unselected_px: 0.0,
        }
    }
}

/// Legacy combined uniform used by raw-wgpu smoke tests.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextStyleTableUniform {
    pub globals: TextStyleGlobalsGpu,
    pub rows: [TextStyleRowGpu; MAX_STYLE_SLOTS],
}

impl TextStyleTable {
    pub fn to_uniform(&self, time: f32) -> TextStyleTableUniform {
        TextStyleTableUniform {
            globals: self.to_globals(time),
            rows: self.to_rows_uniform().rows,
        }
    }
}

/// Instance style metadata encoded in `GlyphInstanceGpu.style_params`.
#[inline]
pub fn style_params_for_slot(style_slot: TextStyleSlot, role_slot: u16) -> [f32; 4] {
    [f32::from(style_slot), f32::from(role_slot), 0.0, 0.0]
}

/// Map icon layer role to a stable role-slot index for shader/tests.
pub fn role_slot_for_icon_layer(role: crate::icons::IconLayerRole) -> u16 {
    use crate::icons::IconLayerRole;
    match role {
        IconLayerRole::Primary => 0,
        IconLayerRole::Secondary => 1,
        IconLayerRole::Accent => 2,
        IconLayerRole::Outline => 3,
        IconLayerRole::Background => 4,
        IconLayerRole::Mask => 5,
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum StyleError {
    #[error("style slot {0} out of range (max {MAX_STYLE_SLOTS})")]
    SlotOutOfRange(TextStyleSlot),
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TextStyleDiagnostics {
    pub style_table_upload_count: u64,
    pub style_table_cache_hit_count: u64,
    pub styled_instance_count: u64,
    pub gradient_instance_count: u64,
    pub icon_role_style_instance_count: u64,
}

/// Main-world style table resource; row uploads occur only when `rows_dirty`.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct TextStyleTableResource {
    pub table: TextStyleTable,
    pub time: f32,
    pub rows_dirty: bool,
    pub rows_generation: u64,
}

impl Default for TextStyleTableResource {
    fn default() -> Self {
        Self {
            table: TextStyleTable::with_defaults(),
            time: 0.0,
            rows_dirty: true,
            rows_generation: 0,
        }
    }
}

impl TextStyleTableResource {
    pub fn set_row(&mut self, slot: TextStyleSlot, row: TextStyleRow) -> Result<(), StyleError> {
        self.table.set_row(slot, row)?;
        self.rows_dirty = true;
        Ok(())
    }

    pub fn mark_rows_clean(&mut self) {
        self.rows_dirty = false;
    }
}

/// Extracted style table rows for the render world.
#[derive(Resource, Clone, Debug, PartialEq, ExtractResource)]
pub struct ExtractedTextStyleTable {
    pub rows: TextStyleRowsUniform,
    pub globals: TextStyleGlobalsGpu,
    pub rows_generation: u64,
}

impl Default for ExtractedTextStyleTable {
    fn default() -> Self {
        let table = TextStyleTable::with_defaults();
        Self {
            rows: table.to_rows_uniform(),
            globals: table.to_globals(0.0),
            rows_generation: 0,
        }
    }
}

pub fn test_style_table_solid_red() -> TextStyleTable {
    let mut table = TextStyleTable::with_defaults();
    table
        .set_row(1, TextStyleRow::solid_fill(1.0, 0.0, 0.0, 1.0))
        .expect("slot 1");
    table
}

pub fn test_style_table_gradient() -> TextStyleTable {
    let mut table = TextStyleTable::with_defaults();
    table
        .set_row(
            2,
            TextStyleRow::linear_gradient_u([0.0, 0.0, 1.0, 1.0], [1.0, 1.0, 0.0, 1.0]),
        )
        .expect("slot 2");
    table
}
