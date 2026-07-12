//! Bounded backdrop blur for Studio's translucent egui surfaces.

use bevy::asset::{load_internal_asset, weak_handle};
use bevy::core_pipeline::{
    core_3d::graph::{Core3d, Node3d},
    fullscreen_vertex_shader::fullscreen_shader_vertex_state,
};
use bevy::ecs::query::QueryItem;
use bevy::image::BevyDefault;
use bevy::prelude::*;
use bevy::render::extract_component::{
    ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin,
    UniformComponentPlugin,
};
use bevy::render::render_graph::{RenderGraphApp, RenderLabel, ViewNode, ViewNodeRunner};
use bevy::render::render_resource::{
    binding_types::{sampler, texture_2d, uniform_buffer},
    *,
};
use bevy::render::renderer::{RenderContext, RenderDevice};
use bevy::render::view::ViewTarget;
use bevy::render::{Render, RenderApp, RenderSet};

pub const FROSTED_GLASS_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("a1f9f726-5889-44ba-b923-52233374ef52");
pub const FROSTED_GLASS_DOWNSAMPLE_FACTOR: u32 = 8;
pub const FROSTED_GLASS_BLUR_PASS_COUNT: u32 = 2;
pub const FROSTED_GLASS_SHARED_TARGET_COUNT: u32 = 1;
pub const FROSTED_GLASS_MAX_PANELS: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrostedGlassRenderPlan {
    pub downsample_factor: u32,
    pub blur_pass_count: u32,
    pub shared_target_count: u32,
    pub blur_radius_target_px: f32,
}

impl Default for FrostedGlassRenderPlan {
    fn default() -> Self {
        Self {
            downsample_factor: FROSTED_GLASS_DOWNSAMPLE_FACTOR,
            blur_pass_count: FROSTED_GLASS_BLUR_PASS_COUNT,
            shared_target_count: FROSTED_GLASS_SHARED_TARGET_COUNT,
            blur_radius_target_px: 1.5,
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct FrostedGlassPanelRegistry {
    rects: Vec<Vec4>,
}

impl Default for FrostedGlassPanelRegistry {
    fn default() -> Self {
        Self {
            rects: Vec::with_capacity(FROSTED_GLASS_MAX_PANELS),
        }
    }
}

impl FrostedGlassPanelRegistry {
    pub fn begin_frame(&mut self) {
        self.rects.clear();
    }

    pub fn register_logical_rect(&mut self, min: [f32; 2], max: [f32; 2], screen: [f32; 2]) {
        if self.rects.len() >= FROSTED_GLASS_MAX_PANELS || screen[0] <= 0.0 || screen[1] <= 0.0 {
            return;
        }
        let rect = Vec4::new(
            (min[0] / screen[0]).clamp(0.0, 1.0),
            (min[1] / screen[1]).clamp(0.0, 1.0),
            (max[0] / screen[0]).clamp(0.0, 1.0),
            (max[1] / screen[1]).clamp(0.0, 1.0),
        );
        if rect.z > rect.x && rect.w > rect.y {
            self.rects.push(rect);
        }
    }

    pub fn panel_count(&self) -> usize {
        self.rects.len()
    }
}

#[allow(dead_code)] // ShaderType emits compile-time layout checks used only by the derive.
#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct FrostedGlassSettings {
    source_texel_size: Vec2,
    blur_texel_size: Vec2,
    panel_rects: [Vec4; FROSTED_GLASS_MAX_PANELS],
    panel_count: u32,
    enabled: u32,
    _padding: Vec2,
}

impl Default for FrostedGlassSettings {
    fn default() -> Self {
        Self {
            source_texel_size: Vec2::ONE,
            blur_texel_size: Vec2::ONE,
            panel_rects: [Vec4::ZERO; FROSTED_GLASS_MAX_PANELS],
            panel_count: 0,
            enabled: 0,
            _padding: Vec2::ZERO,
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct FrostedGlassFrameTelemetry {
    phase: FrostedGlassPerfPhase,
    samples: Vec<f64>,
    pub baseline_frame_ms: Option<f64>,
    pub frosted_frame_ms: Option<f64>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum FrostedGlassPerfPhase {
    #[default]
    Warmup,
    Baseline,
    Frosted,
    Complete,
}

impl FrostedGlassFrameTelemetry {
    const WARMUP_FRAMES: usize = 30;
    const SAMPLE_FRAMES: usize = 60;

    pub fn effect_enabled(&self) -> bool {
        matches!(
            self.phase,
            FrostedGlassPerfPhase::Frosted | FrostedGlassPerfPhase::Complete
        )
    }

    pub fn record_frame_ms(&mut self, frame_ms: f64) -> bool {
        if !frame_ms.is_finite() || frame_ms <= 0.0 || self.phase == FrostedGlassPerfPhase::Complete
        {
            return false;
        }
        self.samples.push(frame_ms);
        let target = if self.phase == FrostedGlassPerfPhase::Warmup {
            Self::WARMUP_FRAMES
        } else {
            Self::SAMPLE_FRAMES
        };
        if self.samples.len() < target {
            return false;
        }
        let mean = self.samples.iter().sum::<f64>() / self.samples.len() as f64;
        self.samples.clear();
        match self.phase {
            FrostedGlassPerfPhase::Warmup => self.phase = FrostedGlassPerfPhase::Baseline,
            FrostedGlassPerfPhase::Baseline => {
                self.baseline_frame_ms = Some(mean);
                self.phase = FrostedGlassPerfPhase::Frosted;
            }
            FrostedGlassPerfPhase::Frosted => {
                self.frosted_frame_ms = Some(mean);
                self.phase = FrostedGlassPerfPhase::Complete;
                return true;
            }
            FrostedGlassPerfPhase::Complete => {}
        }
        false
    }
}

pub struct StudioFrostedGlassPlugin;

impl Plugin for StudioFrostedGlassPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            FROSTED_GLASS_SHADER_HANDLE,
            "shaders/studio_frosted_glass.wgsl",
            Shader::from_wgsl
        );
        app.init_resource::<FrostedGlassPanelRegistry>()
            .init_resource::<FrostedGlassFrameTelemetry>()
            .add_plugins((
                ExtractComponentPlugin::<FrostedGlassSettings>::default(),
                UniformComponentPlugin::<FrostedGlassSettings>::default(),
            ))
            .add_systems(
                Update,
                (sync_frosted_glass_settings, record_frosted_glass_frame_time),
            );

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .add_render_graph_node::<ViewNodeRunner<FrostedGlassNode>>(Core3d, FrostedGlassLabel)
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::EndMainPassPostProcessing,
                    FrostedGlassLabel,
                    bevy_egui::render::graph::NodeEgui::EguiPass,
                ),
            )
            .add_systems(
                Render,
                prepare_frosted_glass_textures.in_set(RenderSet::PrepareResources),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.init_resource::<FrostedGlassPipeline>();
    }
}

fn sync_frosted_glass_settings(
    registry: Res<FrostedGlassPanelRegistry>,
    telemetry: Res<FrostedGlassFrameTelemetry>,
    mut cameras: Query<(&Camera, &mut FrostedGlassSettings)>,
) {
    let Ok((camera, mut settings)) = cameras.single_mut() else {
        return;
    };
    let Some(size) = camera.physical_viewport_size() else {
        return;
    };
    let blur_size = UVec2::new(
        size.x.div_ceil(FROSTED_GLASS_DOWNSAMPLE_FACTOR),
        size.y.div_ceil(FROSTED_GLASS_DOWNSAMPLE_FACTOR),
    );
    settings.source_texel_size = 1.0 / size.as_vec2();
    settings.blur_texel_size = 1.0 / blur_size.as_vec2();
    settings.panel_rects.fill(Vec4::ZERO);
    for (target, source) in settings.panel_rects.iter_mut().zip(&registry.rects) {
        *target = *source;
    }
    settings.panel_count = registry.panel_count() as u32;
    settings.enabled = u32::from(telemetry.effect_enabled());
}

fn record_frosted_glass_frame_time(
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
    mut telemetry: ResMut<FrostedGlassFrameTelemetry>,
) {
    let Some(frame_ms) = diagnostics
        .get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|diagnostic| diagnostic.smoothed())
    else {
        return;
    };
    if telemetry.record_frame_ms(frame_ms) {
        let plan = FrostedGlassRenderPlan::default();
        let summary = format!(
            "FROSTED-GLASS-PERF baseline_frame_ms={:.3} frosted_frame_ms={:.3} downsample={} blur_radius_target_px={:.2} blur_passes={} shared_targets={}",
            telemetry.baseline_frame_ms.unwrap_or_default(),
            telemetry.frosted_frame_ms.unwrap_or_default(),
            plan.downsample_factor,
            plan.blur_radius_target_px,
            plan.blur_pass_count,
            plan.shared_target_count,
        );
        info!("{summary}");
        eprintln!("{summary}");
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct FrostedGlassLabel;

#[derive(Component)]
struct FrostedGlassTextures {
    size: UVec2,
    horizontal: Texture,
    horizontal_view: TextureView,
    vertical: Texture,
    vertical_view: TextureView,
}

fn prepare_frosted_glass_textures(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    views: Query<(Entity, &ViewTarget, Option<&FrostedGlassTextures>)>,
) {
    for (entity, target, existing) in &views {
        let source_size = target.main_texture().size();
        let size = UVec2::new(
            source_size.width.div_ceil(FROSTED_GLASS_DOWNSAMPLE_FACTOR),
            source_size.height.div_ceil(FROSTED_GLASS_DOWNSAMPLE_FACTOR),
        );
        if existing.is_some_and(|textures| textures.size == size) {
            continue;
        }
        let descriptor = TextureDescriptor {
            label: Some("studio_frosted_glass_shared_blur"),
            size: Extent3d {
                width: size.x.max(1),
                height: size.y.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let horizontal = render_device.create_texture(&descriptor);
        let vertical = render_device.create_texture(&TextureDescriptor {
            label: Some("studio_frosted_glass_shared_blur_ping"),
            ..descriptor
        });
        let horizontal_view = horizontal.create_view(&TextureViewDescriptor::default());
        let vertical_view = vertical.create_view(&TextureViewDescriptor::default());
        commands.entity(entity).insert(FrostedGlassTextures {
            size,
            horizontal,
            horizontal_view,
            vertical,
            vertical_view,
        });
    }
}

#[derive(Resource)]
struct FrostedGlassPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    downsample: CachedRenderPipelineId,
    horizontal: CachedRenderPipelineId,
    vertical: CachedRenderPipelineId,
    composite: CachedRenderPipelineId,
}

impl FromWorld for FrostedGlassPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let layout = render_device.create_bind_group_layout(
            "studio_frosted_glass_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<FrostedGlassSettings>(true),
                ),
            ),
        );
        let sampler = render_device.create_sampler(&SamplerDescriptor {
            label: Some("studio_frosted_glass_sampler"),
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            ..default()
        });
        let cache = world.resource_mut::<PipelineCache>();
        let make = |label: &'static str, entry: &'static str| RenderPipelineDescriptor {
            label: Some(label.into()),
            layout: vec![layout.clone()],
            vertex: fullscreen_shader_vertex_state(),
            fragment: Some(FragmentState {
                shader: FROSTED_GLASS_SHADER_HANDLE,
                shader_defs: vec![],
                entry_point: entry.into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
            zero_initialize_workgroup_memory: false,
        };
        let downsample =
            cache.queue_render_pipeline(make("studio_frosted_downsample", "downsample"));
        let horizontal =
            cache.queue_render_pipeline(make("studio_frosted_horizontal", "blur_horizontal"));
        let vertical =
            cache.queue_render_pipeline(make("studio_frosted_vertical", "blur_vertical"));
        let composite = cache.queue_render_pipeline(make("studio_frosted_composite", "composite"));
        Self {
            layout,
            sampler,
            downsample,
            horizontal,
            vertical,
            composite,
        }
    }
}

#[derive(Default)]
struct FrostedGlassNode;

impl ViewNode for FrostedGlassNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static FrostedGlassTextures,
        &'static FrostedGlassSettings,
        &'static DynamicUniformIndex<FrostedGlassSettings>,
    );

    fn run<'w>(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (target, textures, settings, settings_index): QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        if settings.enabled == 0 || settings.panel_count == 0 {
            return Ok(());
        }
        let pipeline = world.resource::<FrostedGlassPipeline>();
        let cache = world.resource::<PipelineCache>();
        let Some((downsample, horizontal, vertical, composite)) = cache
            .get_render_pipeline(pipeline.downsample)
            .zip(cache.get_render_pipeline(pipeline.horizontal))
            .zip(cache.get_render_pipeline(pipeline.vertical))
            .zip(cache.get_render_pipeline(pipeline.composite))
            .map(|(((a, b), c), d)| (a, b, c, d))
        else {
            return Ok(());
        };
        let Some(uniforms) = world
            .resource::<ComponentUniforms<FrostedGlassSettings>>()
            .uniforms()
            .binding()
        else {
            return Ok(());
        };
        let post = target.post_process_write();
        let device = world.resource::<RenderDevice>();
        let bind = |label, source: &TextureView, aux: &TextureView| {
            device.create_bind_group(
                label,
                &pipeline.layout,
                &BindGroupEntries::sequential((source, aux, &pipeline.sampler, uniforms.clone())),
            )
        };
        let downsample_bind = bind("studio_frosted_downsample_bind", post.source, post.source);
        let horizontal_bind = bind(
            "studio_frosted_horizontal_bind",
            &textures.horizontal_view,
            &textures.horizontal_view,
        );
        let vertical_bind = bind(
            "studio_frosted_vertical_bind",
            &textures.vertical_view,
            &textures.vertical_view,
        );
        let composite_bind = bind(
            "studio_frosted_composite_bind",
            post.source,
            &textures.horizontal_view,
        );
        let mut pass = |label: &'static str,
                        view: &TextureView,
                        render_pipeline: &RenderPipeline,
                        bind_group: &BindGroup| {
            let mut pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
                label: Some(label),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: Operations::default(),
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_render_pipeline(render_pipeline);
            pass.set_bind_group(0, bind_group, &[settings_index.index()]);
            pass.draw(0..3, 0..1);
        };
        pass(
            "studio_frosted_downsample_pass",
            &textures.horizontal_view,
            downsample,
            &downsample_bind,
        );
        pass(
            "studio_frosted_horizontal_pass",
            &textures.vertical_view,
            horizontal,
            &horizontal_bind,
        );
        pass(
            "studio_frosted_vertical_pass",
            &textures.horizontal_view,
            vertical,
            &vertical_bind,
        );
        pass(
            "studio_frosted_composite_pass",
            post.destination,
            composite,
            &composite_bind,
        );
        let _keep_textures_alive = (&textures.horizontal, &textures.vertical);
        Ok(())
    }
}
