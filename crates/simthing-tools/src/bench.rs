use std::time::Instant;

use thiserror::Error;

use crate::{
    atlas::{AtlasTile, GlyphAtlasCore, GlyphAtlasStats},
    bevy::GlyphInstanceGpu,
    font::{load_font, ProbeFont, TypefaceError},
    icons::{IconError, IconSet, ICON_PUA_START},
    shaping::ShapingEngine,
};

const FIXTURE_FONT: &[u8] = include_bytes!("../../simthing-workshop/assets/typeface/test_font.ttf");

const STAR_SVG: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
  <path d="M 8 1 L 15 15 L 1 15 Z" fill="#ffffff"/>
</svg>
"##;

const CIRCLE_SVG: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
  <circle cx="8" cy="8" r="6" fill="#ffffff"/>
</svg>
"##;

const DEFAULT_PX: f32 = 24.0;
const DEFAULT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

/// CI-friendly default: hundreds of labels, completes under ~30s on dev machines.
pub const CI_BENCH_CONFIG: TypefaceBenchConfig = TypefaceBenchConfig {
    static_labels: 500,
    damage_labels: 100,
    frames: 12,
    icon_every_n_labels: 5,
    atlas_size: 2048,
};

/// Optional manual heavy bench (`#[ignore]` tests only).
pub const HEAVY_BENCH_CONFIG: TypefaceBenchConfig = TypefaceBenchConfig {
    static_labels: 5_000,
    damage_labels: 500,
    frames: 60,
    icon_every_n_labels: 5,
    atlas_size: 4096,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypefaceBenchConfig {
    pub static_labels: usize,
    pub damage_labels: usize,
    pub frames: usize,
    pub icon_every_n_labels: usize,
    pub atlas_size: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypefaceBenchResult {
    pub static_labels: usize,
    pub damage_labels: usize,
    pub frames: usize,
    pub initial_shape_rebuilds: u64,
    pub noop_shape_rebuilds: u64,
    pub damage_shape_rebuilds: u64,
    pub initial_rasterize_count: u64,
    pub noop_rasterize_count_delta: u64,
    pub damage_rasterize_count_delta: u64,
    pub instance_count: usize,
    pub atlas_tile_count: usize,
    pub icon_cache_entries: usize,
    pub elapsed_initial_ms: f64,
    pub elapsed_noop_ms: f64,
    pub elapsed_damage_ms: f64,
}

#[derive(Debug, Error)]
pub enum TypefaceBenchError {
    #[error("typeface font: {0}")]
    Font(#[from] TypefaceError),
    #[error("icon: {0}")]
    Icon(#[from] IconError),
    #[error("bench config invalid: {0}")]
    Config(&'static str),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TypefaceBenchDiagnostics {
    pub shape_rebuild_count: u64,
    pub instance_rebuild_count: u64,
}

#[derive(Debug, Clone)]
struct CachedLabel {
    text: String,
    px: f32,
    color: [f32; 4],
    built: bool,
    instances: Vec<GlyphInstanceGpu>,
}

/// CPU-side label pool mirroring LR3 changed-detection semantics for high-volume proof.
pub struct TypefaceBenchHarness {
    font: ProbeFont,
    shaper: ShapingEngine,
    atlas: GlyphAtlasCore,
    icons: IconSet,
    static_labels: Vec<CachedLabel>,
    damage_labels: Vec<CachedLabel>,
    diagnostics: TypefaceBenchDiagnostics,
    _atlas_size: u32,
}

impl TypefaceBenchHarness {
    pub fn new(config: TypefaceBenchConfig) -> Result<Self, TypefaceBenchError> {
        validate_config(config)?;
        let font = load_font(FIXTURE_FONT)?;
        let shaper = ShapingEngine::new_with_font(FIXTURE_FONT.to_vec())?;
        let atlas = GlyphAtlasCore::new(config.atlas_size);
        let icons = IconSet::new();
        Ok(Self {
            font,
            shaper,
            atlas,
            icons,
            static_labels: Vec::with_capacity(config.static_labels),
            damage_labels: Vec::with_capacity(config.damage_labels),
            diagnostics: TypefaceBenchDiagnostics::default(),
            _atlas_size: config.atlas_size,
        })
    }

    pub fn diagnostics(&self) -> TypefaceBenchDiagnostics {
        self.diagnostics
    }

    pub fn atlas_stats(&self) -> GlyphAtlasStats {
        self.atlas.stats()
    }

    pub fn atlas_core(&self) -> &GlyphAtlasCore {
        &self.atlas
    }

    pub fn icon_cache_entries(&self) -> usize {
        self.icons.cache_entry_count()
    }

    pub fn icons(&self) -> &IconSet {
        &self.icons
    }

    pub fn instance_count(&self) -> usize {
        self.static_labels
            .iter()
            .chain(self.damage_labels.iter())
            .map(|label| label.instances.len())
            .sum()
    }

    pub fn register_fixture_icons(&mut self) -> Result<(), TypefaceBenchError> {
        self.icons
            .register_svg(ICON_PUA_START + 1, STAR_SVG, DEFAULT_PX, &mut self.atlas)?;
        self.icons
            .register_svg(ICON_PUA_START + 2, CIRCLE_SVG, DEFAULT_PX, &mut self.atlas)?;
        Ok(())
    }

    pub fn build_static_labels(
        &mut self,
        count: usize,
        icon_every_n: usize,
    ) -> Result<(), TypefaceBenchError> {
        self.static_labels.clear();
        for index in 0..count {
            let text = static_label_text(index, icon_every_n);
            let mut label = CachedLabel {
                text,
                px: DEFAULT_PX,
                color: DEFAULT_COLOR,
                built: false,
                instances: Vec::new(),
            };
            self.rebuild_label(&mut label, true)?;
            self.static_labels.push(label);
        }
        Ok(())
    }

    pub fn build_damage_labels(&mut self, count: usize) -> Result<(), TypefaceBenchError> {
        self.damage_labels.clear();
        for index in 0..count {
            let text = damage_label_text(index, 0);
            let mut label = CachedLabel {
                text,
                px: DEFAULT_PX,
                color: [1.0, 0.35, 0.2, 1.0],
                built: false,
                instances: Vec::new(),
            };
            self.rebuild_label(&mut label, true)?;
            self.damage_labels.push(label);
        }
        Ok(())
    }

    pub fn run_noop_frames(&mut self, frames: usize) -> Result<(), TypefaceBenchError> {
        for _ in 0..frames {
            self.tick_unchanged()?;
        }
        Ok(())
    }

    pub fn run_damage_frames(&mut self, frames: usize) -> Result<(), TypefaceBenchError> {
        for frame in 0..frames {
            let count = self.damage_labels.len();
            for index in 0..count {
                let text = damage_label_text(index, frame + 1);
                self.damage_labels[index].text = text;
                self.rebuild_damage_label(index)?;
            }
        }
        Ok(())
    }

    fn tick_unchanged(&mut self) -> Result<(), TypefaceBenchError> {
        let static_count = self.static_labels.len();
        for index in 0..static_count {
            self.rebuild_static_label(index, false)?;
        }
        let damage_count = self.damage_labels.len();
        for index in 0..damage_count {
            self.rebuild_damage_label_unchanged(index)?;
        }
        Ok(())
    }

    fn rebuild_static_label(
        &mut self,
        index: usize,
        changed: bool,
    ) -> Result<(), TypefaceBenchError> {
        if !changed && self.static_labels[index].built {
            return Ok(());
        }
        self.diagnostics.shape_rebuild_count += 1;
        let label = &self.static_labels[index];
        let instances = self.icons.build_mixed_instances(
            &self.font,
            &mut self.shaper,
            &mut self.atlas,
            &label.text,
            label.px,
            label.color,
        )?;
        let label = &mut self.static_labels[index];
        label.instances = instances;
        label.built = true;
        self.diagnostics.instance_rebuild_count += 1;
        Ok(())
    }

    fn rebuild_damage_label(&mut self, index: usize) -> Result<(), TypefaceBenchError> {
        self.diagnostics.shape_rebuild_count += 1;
        let label = &self.damage_labels[index];
        let instances = self.icons.build_mixed_instances(
            &self.font,
            &mut self.shaper,
            &mut self.atlas,
            &label.text,
            label.px,
            label.color,
        )?;
        let label = &mut self.damage_labels[index];
        label.instances = instances;
        label.built = true;
        self.diagnostics.instance_rebuild_count += 1;
        Ok(())
    }

    fn rebuild_damage_label_unchanged(&mut self, index: usize) -> Result<(), TypefaceBenchError> {
        if self.damage_labels[index].built {
            return Ok(());
        }
        self.rebuild_damage_label(index)
    }

    fn rebuild_label(
        &mut self,
        label: &mut CachedLabel,
        changed: bool,
    ) -> Result<(), TypefaceBenchError> {
        if !changed && label.built {
            return Ok(());
        }
        self.diagnostics.shape_rebuild_count += 1;
        let instances = self.icons.build_mixed_instances(
            &self.font,
            &mut self.shaper,
            &mut self.atlas,
            &label.text,
            label.px,
            label.color,
        )?;
        label.instances = instances;
        label.built = true;
        self.diagnostics.instance_rebuild_count += 1;
        Ok(())
    }
}

pub fn run_typeface_bench(
    config: TypefaceBenchConfig,
) -> Result<TypefaceBenchResult, TypefaceBenchError> {
    validate_config(config)?;

    let mut harness = TypefaceBenchHarness::new(config)?;
    let icon_every_n = config.icon_every_n_labels.max(1);

    let initial_start = Instant::now();
    harness.register_fixture_icons()?;
    harness.build_static_labels(config.static_labels, icon_every_n)?;
    harness.build_damage_labels(config.damage_labels)?;
    let elapsed_initial_ms = initial_start.elapsed().as_secs_f64() * 1000.0;

    let initial_shape_rebuilds = harness.diagnostics().shape_rebuild_count;
    let initial_rasterize_count = harness.atlas_stats().rasterize_count;

    let noop_frames = config.frames.max(1) / 2;
    let noop_start = Instant::now();
    let raster_before_noop = harness.atlas_stats().rasterize_count;
    let shape_before_noop = harness.diagnostics().shape_rebuild_count;
    harness.run_noop_frames(noop_frames)?;
    let elapsed_noop_ms = noop_start.elapsed().as_secs_f64() * 1000.0;
    let noop_shape_rebuilds = harness.diagnostics().shape_rebuild_count - shape_before_noop;
    let noop_rasterize_count_delta = harness.atlas_stats().rasterize_count - raster_before_noop;

    let damage_frames = config.frames.saturating_sub(noop_frames).max(1);
    let damage_start = Instant::now();
    let raster_before_damage = harness.atlas_stats().rasterize_count;
    let shape_before_damage = harness.diagnostics().shape_rebuild_count;
    harness.run_damage_frames(damage_frames)?;
    let elapsed_damage_ms = damage_start.elapsed().as_secs_f64() * 1000.0;
    let damage_shape_rebuilds = harness.diagnostics().shape_rebuild_count - shape_before_damage;
    let damage_rasterize_count_delta = harness.atlas_stats().rasterize_count - raster_before_damage;

    Ok(TypefaceBenchResult {
        static_labels: config.static_labels,
        damage_labels: config.damage_labels,
        frames: config.frames,
        initial_shape_rebuilds,
        noop_shape_rebuilds,
        damage_shape_rebuilds,
        initial_rasterize_count,
        noop_rasterize_count_delta,
        damage_rasterize_count_delta,
        instance_count: harness.instance_count(),
        atlas_tile_count: harness.atlas_stats().rasterize_count as usize,
        icon_cache_entries: harness.icon_cache_entries(),
        elapsed_initial_ms,
        elapsed_noop_ms,
        elapsed_damage_ms,
    })
}

fn validate_config(config: TypefaceBenchConfig) -> Result<(), TypefaceBenchError> {
    if config.atlas_size < 256 {
        return Err(TypefaceBenchError::Config("atlas_size must be >= 256"));
    }
    if config.icon_every_n_labels == 0 {
        return Err(TypefaceBenchError::Config(
            "icon_every_n_labels must be >= 1",
        ));
    }
    Ok(())
}

fn static_label_text(index: usize, icon_every_n: usize) -> String {
    let names = [
        "Sol Prime",
        "Altair",
        "Deneb",
        "Vega",
        "Rigel",
        "Sirius",
        "Proxima",
        "Tau Ceti",
        "Epsilon Eridani",
        "Betelgeuse",
    ];
    let suffix = index % 100;
    let base = names[index % names.len()];
    if icon_every_n > 0 && index % icon_every_n == 0 {
        let icon = if index % 2 == 0 {
            char::from_u32(ICON_PUA_START + 1).expect("pua")
        } else {
            char::from_u32(ICON_PUA_START + 2).expect("pua")
        };
        format!("{base} {icon} {suffix}")
    } else {
        format!("{base} {suffix}")
    }
}

fn damage_label_text(index: usize, frame: usize) -> String {
    let value = (index.wrapping_mul(17).wrapping_add(frame.wrapping_mul(13))) % 9999;
    format!("-{value}")
}

/// Returns the atlas tile for a registered icon codepoint at the bench px size, if present.
pub fn icon_tile_in_atlas(
    atlas: &GlyphAtlasCore,
    icons: &IconSet,
    codepoint: u32,
) -> Option<AtlasTile> {
    let tile = icons.tile_for(codepoint)?;
    let _ = atlas.tile_pixels(tile);
    Some(tile)
}
