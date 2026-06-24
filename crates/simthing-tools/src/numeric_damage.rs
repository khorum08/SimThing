//! Fixed-width numeric damage glyph lane — import-time table, runtime integer updates only.

use bevy::prelude::*;

use crate::{
    atlas::{quantize_px, tile_uv_rect, AtlasTile},
    bevy::GlyphInstanceGpu,
    font::ProbeFont,
    shaping::{ShapedGlyph, ShapingEngine},
    TypefaceAtlas,
};

/// Default fixed digit count after the minus sign (`-####`).
pub const NUMERIC_DAMAGE_DEFAULT_WIDTH: usize = 4;

/// Fixed-width numeric damage label — no per-frame string formatting or cosmic-text shaping.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct NumericDamageLabel {
    pub value: i32,
    pub width: usize,
    pub px: f32,
    pub color: [f32; 4],
}

impl NumericDamageLabel {
    pub fn new(value: i32, px: f32, color: [f32; 4]) -> Self {
        Self {
            value,
            width: NUMERIC_DAMAGE_DEFAULT_WIDTH,
            px,
            color,
        }
    }
}

/// Per-glyph template: tile layout with slot x from reference shaping.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GlyphSlotTemplate {
    pos_y: f32,
    size_w: f32,
    size_h: f32,
    uv_rect: [f32; 4],
}

impl GlyphSlotTemplate {
    fn instance(&self, slot_x: f32, color: [f32; 4]) -> GlyphInstanceGpu {
        GlyphInstanceGpu {
            pos_size: [slot_x + 0.0, self.pos_y, self.size_w, self.size_h],
            uv_rect: self.uv_rect,
            color,
            sdf_params: [0.0; 4],
            style_params: [0.0; 4],
            deform_params: [0.0; 4],
            path_params: [0.0; 4],
            warp_params: [0.0; 4],
        }
    }
}

/// Import-time glyph table for a fixed-width numeric lane at one px bucket.
#[derive(Resource, Debug, Clone)]
pub struct NumericGlyphRunTable {
    pub px_bucket: u16,
    pub width: usize,
    pub glyph_count: usize,
    slot_x: Vec<f32>,
    minus: GlyphSlotTemplate,
    digits: [GlyphSlotTemplate; 10],
}

impl NumericGlyphRunTable {
    pub fn glyph_count(&self) -> usize {
        self.glyph_count
    }

    /// Compose a fixed-width run into `out` without allocation when `out.len() == glyph_count`.
    pub fn write_run(&self, value: i32, color: [f32; 4], out: &mut [GlyphInstanceGpu]) {
        debug_assert_eq!(out.len(), self.glyph_count);
        let abs = value.unsigned_abs().min(10_u32.pow(self.width as u32) - 1) as u32;
        let mut divisor = 10_u32.pow(self.width as u32 - 1);
        out[0] = self.minus.instance(self.slot_x[0], color);
        for (slot, dst) in out.iter_mut().skip(1).enumerate() {
            let digit = ((abs / divisor) % 10) as usize;
            divisor /= 10;
            *dst = self.digits[digit].instance(self.slot_x[slot + 1], color);
        }
    }

    pub fn compose_run(&self, value: i32, color: [f32; 4]) -> Vec<GlyphInstanceGpu> {
        let mut out = vec![GlyphInstanceGpu::default(); self.glyph_count];
        self.write_run(value, color, &mut out);
        out
    }
}

/// Diagnostics for the numeric damage lane.
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumericDamageDiagnostics {
    pub numeric_table_build_count: u64,
    pub numeric_label_update_count: u64,
    pub numeric_glyph_instance_patch_count: u64,
    pub numeric_shape_bypass_count: u64,
    pub numeric_string_format_count: u64,
}

pub fn numeric_damage_diagnostics(app: &App) -> NumericDamageDiagnostics {
    app.world()
        .get_resource::<NumericDamageDiagnostics>()
        .copied()
        .unwrap_or_default()
}

/// Build the numeric glyph table at plugin init (cosmic-text runs once per slot/digit).
pub fn build_numeric_glyph_run_table(
    font: &ProbeFont,
    shaper: &mut ShapingEngine,
    atlas: &mut TypefaceAtlas,
    px: f32,
    width: usize,
    diagnostics: &mut NumericDamageDiagnostics,
) -> NumericGlyphRunTable {
    diagnostics.numeric_table_build_count += 1;
    let px_bucket = quantize_px(px);
    let glyph_count = width + 1;

    let reference = reference_damage_text(width);
    let shaped = shaper.shape(&reference, px);
    let slot_x: Vec<f32> = shaped.glyphs.iter().map(|g| g.x).collect();
    debug_assert_eq!(slot_x.len(), glyph_count);

    let minus = template_for_char(
        font,
        shaper,
        atlas,
        "-",
        px,
        atlas.atlas_size,
        &shaped.glyphs[0],
    );
    let mut digits = [GlyphSlotTemplate {
        pos_y: 0.0,
        size_w: 0.0,
        size_h: 0.0,
        uv_rect: [0.0; 4],
    }; 10];
    for (digit, slot) in (0..10_u8).zip(shaped.glyphs.iter().skip(1)) {
        let ch = (b'0' + digit) as char;
        digits[digit as usize] = template_for_char(
            font,
            shaper,
            atlas,
            &ch.to_string(),
            px,
            atlas.atlas_size,
            slot,
        );
    }
    atlas.cpu.clear_dirty_regions();

    NumericGlyphRunTable {
        px_bucket,
        width,
        glyph_count,
        slot_x,
        minus,
        digits,
    }
}

fn reference_damage_text(width: usize) -> String {
    let mut s = String::with_capacity(width + 1);
    s.push('-');
    for _ in 0..width {
        s.push('0');
    }
    s
}

fn template_for_char(
    font: &ProbeFont,
    shaper: &mut ShapingEngine,
    atlas: &mut TypefaceAtlas,
    text: &str,
    px: f32,
    atlas_size: u32,
    layout_glyph: &ShapedGlyph,
) -> GlyphSlotTemplate {
    let shaped = shaper.shape(text, px);
    let glyph = shaped
        .glyphs
        .first()
        .expect("numeric lane char must shape to one glyph");
    let tile = atlas
        .cpu
        .get_or_rasterize(font, glyph.glyph_id, px)
        .expect("numeric lane glyph must rasterize");
    let instance = instance_from_tile(layout_glyph, tile, [1.0; 4], atlas_size);
    GlyphSlotTemplate {
        pos_y: instance.pos_size[1],
        size_w: instance.pos_size[2],
        size_h: instance.pos_size[3],
        uv_rect: instance.uv_rect,
    }
}

/// Patch existing instances in place when capacity matches.
pub fn patch_numeric_instances(
    table: &NumericGlyphRunTable,
    value: i32,
    color: [f32; 4],
    instances: &mut [GlyphInstanceGpu],
    diagnostics: &mut NumericDamageDiagnostics,
) {
    table.write_run(value, color, instances);
    diagnostics.numeric_glyph_instance_patch_count += instances.len() as u64;
}

fn instance_from_tile(
    glyph: &ShapedGlyph,
    tile: AtlasTile,
    color: [f32; 4],
    atlas_size: u32,
) -> GlyphInstanceGpu {
    GlyphInstanceGpu {
        pos_size: [
            glyph.x + tile.left as f32,
            glyph.y + tile.top as f32,
            tile.w as f32,
            tile.h as f32,
        ],
        uv_rect: tile_uv_rect(tile, atlas_size),
        color,
        sdf_params: [0.0; 4],
        style_params: [0.0; 4],
        deform_params: [0.0; 4],
        path_params: [0.0; 4],
        warp_params: [0.0; 4],
    }
}
