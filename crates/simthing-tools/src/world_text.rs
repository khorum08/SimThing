use bevy::prelude::*;

use crate::bevy::{GlyphInstanceGpu, TextGlyphInstances};

pub const DEFAULT_WORLD_TEXT_HORIZON_TAPER: f32 = 0.75;

/// Per-frame GPU globals patch for screen-companion nameplate LOD (no glyph rebuild).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct WorldTextNameplateLodPatch {
    pub min_focused_px: f32,
    pub unselected_global_alpha: f32,
    pub min_unselected_px: f32,
}

impl Default for WorldTextNameplateLodPatch {
    fn default() -> Self {
        Self {
            min_focused_px: 12.0,
            unselected_global_alpha: 1.0,
            min_unselected_px: 24.0,
        }
    }
}

/// GPU sentinel written to `size_params.w` for [`WorldTextPlacementMode::ScreenCompanion`].
pub const WORLD_TEXT_SCREEN_COMPANION_MODE: f32 = -1.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum WorldTextPlacementMode {
    #[default]
    WorldPerspective,
    ScreenCompanion,
}

/// Generic world-space placement for an instanced, camera-facing text label.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct WorldTextBillboard {
    pub anchor: Vec3,
    /// World-perspective mode: near-camera label height in world units.
    pub near_height: f32,
    /// Screen-companion mode: rendered star visual diameter in world units at near camera.
    pub visual_envelope_world_height: f32,
    pub width_ratio: f32,
    pub vertical_gap_ratio: f32,
    pub near_distance: f32,
    pub far_distance: f32,
    pub target_height_ratio: f32,
    pub ceiling_falloff_percent: f32,
    pub ceiling_target_alpha: f32,
    pub base_alpha_ratio: f32,
    pub relative_falloff_percent: f32,
    pub relative_target_alpha: f32,
    pub horizon_taper: f32,
    pub placement_mode: WorldTextPlacementMode,
    /// Screen-companion only: selected or hovered labels use the focused readability threshold.
    pub screen_companion_focused: bool,
}

impl WorldTextBillboard {
    pub fn clamped(self) -> Self {
        let near_distance = self.near_distance.max(0.0);
        Self {
            anchor: self.anchor,
            near_height: self.near_height.max(0.0),
            visual_envelope_world_height: self.visual_envelope_world_height.max(0.0),
            width_ratio: self.width_ratio.clamp(0.01, 8.0),
            vertical_gap_ratio: self.vertical_gap_ratio.clamp(0.0, 4.0),
            near_distance,
            far_distance: self.far_distance.max(near_distance + f32::EPSILON),
            target_height_ratio: self.target_height_ratio.clamp(0.0, 1.0),
            ceiling_falloff_percent: self.ceiling_falloff_percent.clamp(0.01, 100.0),
            ceiling_target_alpha: self.ceiling_target_alpha.clamp(0.0, 1.0),
            base_alpha_ratio: self.base_alpha_ratio.clamp(0.0, 1.0),
            relative_falloff_percent: self.relative_falloff_percent.clamp(0.01, 100.0),
            relative_target_alpha: self.relative_target_alpha.clamp(0.0, 1.0),
            horizon_taper: self.horizon_taper.clamp(0.0, 1.0),
            placement_mode: self.placement_mode,
            screen_companion_focused: self.screen_companion_focused,
        }
    }
}

impl Default for WorldTextBillboard {
    fn default() -> Self {
        Self {
            anchor: Vec3::ZERO,
            near_height: 1.0,
            visual_envelope_world_height: 1.0,
            width_ratio: 1.0,
            vertical_gap_ratio: 0.1,
            near_distance: 0.0,
            far_distance: 100.0,
            target_height_ratio: 0.25,
            ceiling_falloff_percent: 100.0,
            ceiling_target_alpha: 0.1,
            base_alpha_ratio: 1.0,
            relative_falloff_percent: 50.0,
            relative_target_alpha: 0.5,
            horizon_taper: DEFAULT_WORLD_TEXT_HORIZON_TAPER,
            placement_mode: WorldTextPlacementMode::WorldPerspective,
            screen_companion_focused: false,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct WorldGlyphInstanceGpu {
    pub glyph: GlyphInstanceGpu,
    /// xyz = world anchor, w = near label height (world) or visual envelope height (screen).
    pub anchor_height: [f32; 4],
    /// x = width ratio, y = vertical gap ratio,
    /// z = target height ratio (world) or unused (screen companion; glyph x already spans run aspect),
    /// w = horizon taper (world) or screen-companion mode sentinel.
    pub size_params: [f32; 4],
    /// x/y = near/far distance, z = ceiling falloff percent, w = ceiling target alpha.
    pub distance_params: [f32; 4],
}

#[derive(Component, Clone, Default, Debug)]
pub struct WorldTextGlyphInstances(pub Vec<WorldGlyphInstanceGpu>);

#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WorldTextDiagnostics {
    pub instance_rebuild_count: u64,
    pub aggregate_rebuild_count: u64,
    pub draw_sync_count: u64,
    pub queued_draw_count: u64,
    pub queued_instance_count: u64,
    pub buffer_create_count: u64,
    pub buffer_reuse_count: u64,
    pub buffer_upload_count: u64,
}

pub fn world_text_diagnostics(app: &App) -> WorldTextDiagnostics {
    #[allow(unused_mut)]
    let mut diagnostics = app
        .world()
        .get_resource::<WorldTextDiagnostics>()
        .copied()
        .unwrap_or_default();
    #[cfg(feature = "world-text-3d")]
    if let Some(render_app) = app.get_sub_app(bevy::render::RenderApp) {
        if let Some(render) = render_app
            .world()
            .get_resource::<WorldTextRenderDiagnostics>()
        {
            diagnostics.queued_draw_count = render.queued_draw_count;
            diagnostics.queued_instance_count = render.queued_instance_count;
            diagnostics.buffer_create_count = render.buffer_create_count;
            diagnostics.buffer_reuse_count = render.buffer_reuse_count;
            diagnostics.buffer_upload_count = render.buffer_upload_count;
        }
    }
    diagnostics
}

pub fn natural_run_aspect_from_glyphs(glyphs: &[GlyphInstanceGpu]) -> f32 {
    if glyphs.is_empty() {
        return 1.0;
    }
    let min_x = glyphs
        .iter()
        .map(|glyph| glyph.pos_size[0])
        .fold(f32::INFINITY, f32::min);
    let min_y = glyphs
        .iter()
        .map(|glyph| glyph.pos_size[1])
        .fold(f32::INFINITY, f32::min);
    let max_x = glyphs
        .iter()
        .map(|glyph| glyph.pos_size[0] + glyph.pos_size[2])
        .fold(f32::NEG_INFINITY, f32::max);
    let max_y = glyphs
        .iter()
        .map(|glyph| glyph.pos_size[1] + glyph.pos_size[3])
        .fold(f32::NEG_INFINITY, f32::max);
    let run_height = (max_y - min_y).max(1.0);
    ((max_x - min_x) / run_height).clamp(0.01, 32.0)
}

pub fn build_world_glyph_instances(
    glyphs: &[GlyphInstanceGpu],
    placement: WorldTextBillboard,
) -> Vec<WorldGlyphInstanceGpu> {
    if glyphs.is_empty() {
        return Vec::new();
    }
    let placement = placement.clamped();
    let min_x = glyphs
        .iter()
        .map(|glyph| glyph.pos_size[0])
        .fold(f32::INFINITY, f32::min);
    let min_y = glyphs
        .iter()
        .map(|glyph| glyph.pos_size[1])
        .fold(f32::INFINITY, f32::min);
    let max_x = glyphs
        .iter()
        .map(|glyph| glyph.pos_size[0] + glyph.pos_size[2])
        .fold(f32::NEG_INFINITY, f32::max);
    let max_y = glyphs
        .iter()
        .map(|glyph| glyph.pos_size[1] + glyph.pos_size[3])
        .fold(f32::NEG_INFINITY, f32::max);
    let run_height = (max_y - min_y).max(1.0);
    let run_center_x = (min_x + max_x) * 0.5;
    let (anchor_height_w, size_z, size_w) = match placement.placement_mode {
        WorldTextPlacementMode::ScreenCompanion => (
            placement.visual_envelope_world_height,
            if placement.screen_companion_focused {
                1.0
            } else {
                0.0
            },
            WORLD_TEXT_SCREEN_COMPANION_MODE,
        ),
        WorldTextPlacementMode::WorldPerspective => (
            placement.near_height,
            placement.target_height_ratio,
            placement.horizon_taper,
        ),
    };

    glyphs
        .iter()
        .map(|source| {
            let mut glyph = *source;
            // Contract A: x normalized by run height so local_xy.x already spans natural run aspect.
            glyph.pos_size = [
                (source.pos_size[0] - run_center_x) / run_height,
                0.5 - (source.pos_size[1] - min_y) / run_height,
                source.pos_size[2] / run_height,
                -source.pos_size[3] / run_height,
            ];
            glyph.color[3] *= placement.base_alpha_ratio;
            glyph.style_params[1] = placement.target_height_ratio;
            glyph.style_params[2] = placement.relative_falloff_percent;
            glyph.style_params[3] = placement.relative_target_alpha;
            WorldGlyphInstanceGpu {
                glyph,
                anchor_height: [
                    placement.anchor.x,
                    placement.anchor.y,
                    placement.anchor.z,
                    anchor_height_w,
                ],
                size_params: [
                    placement.width_ratio,
                    placement.vertical_gap_ratio,
                    size_z,
                    size_w,
                ],
                distance_params: [
                    placement.near_distance,
                    placement.far_distance,
                    placement.ceiling_falloff_percent,
                    placement.ceiling_target_alpha,
                ],
            }
        })
        .collect()
}

#[derive(Resource, Default)]
pub(crate) struct WorldTextAggregate {
    instances: Vec<WorldGlyphInstanceGpu>,
    version: u64,
    dirty: bool,
}

#[derive(Component, Clone, Default)]
pub(crate) struct WorldTextDrawInstances {
    instances: Vec<WorldGlyphInstanceGpu>,
    version: u64,
}

#[derive(Resource)]
pub(crate) struct WorldTextDrawEntity(pub Entity);

pub(crate) fn rebuild_world_text_instances(
    mut commands: Commands,
    mut diagnostics: ResMut<WorldTextDiagnostics>,
    mut aggregate: ResMut<WorldTextAggregate>,
    mut labels: Query<
        (
            Entity,
            &TextGlyphInstances,
            &WorldTextBillboard,
            Option<&mut WorldTextGlyphInstances>,
        ),
        Or<(
            Added<WorldTextBillboard>,
            Changed<WorldTextBillboard>,
            Changed<TextGlyphInstances>,
        )>,
    >,
) {
    for (entity, glyphs, placement, existing) in &mut labels {
        let instances = build_world_glyph_instances(&glyphs.0, *placement);
        if let Some(mut existing) = existing {
            existing.0 = instances;
        } else {
            commands
                .entity(entity)
                .insert(WorldTextGlyphInstances(instances));
        }
        diagnostics.instance_rebuild_count += 1;
        aggregate.dirty = true;
    }
}

pub(crate) fn mark_world_text_aggregate_dirty(
    mut aggregate: ResMut<WorldTextAggregate>,
    added: Query<(), Added<WorldTextGlyphInstances>>,
    changed_visibility: Query<(), (With<WorldTextGlyphInstances>, Changed<Visibility>)>,
    mut removed: RemovedComponents<WorldTextGlyphInstances>,
    mut removed_billboards: RemovedComponents<WorldTextBillboard>,
) {
    if added.iter().next().is_some()
        || changed_visibility.iter().next().is_some()
        || removed.read().next().is_some()
        || removed_billboards.read().next().is_some()
    {
        aggregate.dirty = true;
    }
}

pub(crate) fn aggregate_world_text_instances(
    labels: Query<(Entity, &WorldTextGlyphInstances, Option<&Visibility>)>,
    mut aggregate: ResMut<WorldTextAggregate>,
    mut diagnostics: ResMut<WorldTextDiagnostics>,
) {
    if !aggregate.dirty {
        return;
    }
    aggregate.dirty = false;
    aggregate.version = aggregate.version.wrapping_add(1);
    let mut ordered: Vec<_> = labels
        .iter()
        .filter(|(_, _, visibility)| !matches!(visibility, Some(Visibility::Hidden)))
        .collect();
    ordered.sort_by_key(|(entity, _, _)| entity.index());
    let required = ordered
        .iter()
        .map(|(_, instances, _)| instances.0.len())
        .sum();
    aggregate.instances.clear();
    aggregate.instances.reserve(required);
    for (_, instances, _) in ordered {
        aggregate.instances.extend_from_slice(&instances.0);
    }
    diagnostics.aggregate_rebuild_count += 1;
}

pub(crate) fn sync_world_text_draw_instances(
    aggregate: Res<WorldTextAggregate>,
    draw_entity: Option<Res<WorldTextDrawEntity>>,
    mut draw: Query<&mut WorldTextDrawInstances>,
    mut diagnostics: ResMut<WorldTextDiagnostics>,
) {
    let Some(draw_entity) = draw_entity else {
        return;
    };
    let Ok(mut draw) = draw.get_mut(draw_entity.0) else {
        return;
    };
    if draw.version == aggregate.version {
        return;
    }
    draw.instances.clone_from(&aggregate.instances);
    draw.version = aggregate.version;
    diagnostics.draw_sync_count += 1;
}

pub(crate) fn force_world_text_draw_visible(
    draw_entity: Option<Res<WorldTextDrawEntity>>,
    mut visibility: Query<&mut ViewVisibility>,
) {
    let Some(draw_entity) = draw_entity else {
        return;
    };
    if let Ok(mut visibility) = visibility.get_mut(draw_entity.0) {
        visibility.set();
    }
}

#[cfg(feature = "world-text-3d")]
mod render_3d {
    use std::mem::size_of;

    use bevy::{
        core_pipeline::core_3d::Transparent3d,
        ecs::system::{lifetimeless::*, SystemParamItem},
        pbr::{
            MeshPipeline, MeshPipelineKey, RenderMeshInstances, SetMeshBindGroup,
            SetMeshViewBindGroup,
        },
        prelude::*,
        render::{
            extract_component::{ExtractComponent, ExtractComponentPlugin},
            mesh::{
                allocator::MeshAllocator, MeshVertexBufferLayoutRef, RenderMesh,
                RenderMeshBufferInfo,
            },
            render_asset::RenderAssets,
            render_phase::{
                AddRenderCommand, DrawFunctions, PhaseItem, PhaseItemExtraIndex, RenderCommand,
                RenderCommandResult, SetItemPipeline, TrackedRenderPass, ViewSortedRenderPhases,
            },
            render_resource::{
                Buffer, BufferInitDescriptor, BufferUsages, FragmentState, PipelineCache,
                RenderPipelineDescriptor, ShaderDefVal, SpecializedMeshPipeline,
                SpecializedMeshPipelineError, SpecializedMeshPipelines, VertexAttribute,
                VertexBufferLayout, VertexFormat, VertexStepMode,
            },
            renderer::RenderDevice,
            sync_world::{MainEntity, SyncToRenderWorld},
            view::{ExtractedView, Msaa, NoFrustumCulling, NoIndirectDrawing},
            Render, RenderApp, RenderSet,
        },
    };

    use crate::{
        bevy::{GlyphInstanceGpu, TEXT_SHADER_HANDLE},
        text_render::{
            SetTextAtlasBindGroup, SetTextDeformBindGroup, SetTextPathBindGroup,
            SetTextStyleBindGroup, SetTextWarpBindGroup, TextInstancedPipeline,
        },
    };

    use super::{WorldGlyphInstanceGpu, WorldTextDrawEntity, WorldTextDrawInstances};

    #[derive(Component, Clone, Copy, Debug, Default)]
    pub(crate) struct WorldTextInstancedDraw;

    #[derive(Component, Clone)]
    struct WorldTextDrawExtract {
        instances: Vec<WorldGlyphInstanceGpu>,
        version: u64,
    }

    impl ExtractComponent for WorldTextDrawExtract {
        type QueryData = &'static WorldTextDrawInstances;
        type QueryFilter = With<WorldTextInstancedDraw>;
        type Out = Self;

        fn extract_component(item: bevy::ecs::query::QueryItem<Self::QueryData>) -> Option<Self> {
            if item.instances.is_empty() {
                return None;
            }
            Some(Self {
                instances: item.instances.clone(),
                version: item.version,
            })
        }
    }

    #[derive(Component)]
    struct WorldTextInstanceBuffer {
        buffer: Buffer,
        capacity: usize,
        count: usize,
        version: u64,
    }

    #[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub(crate) struct WorldTextRenderDiagnostics {
        pub queued_draw_count: u64,
        pub queued_instance_count: u64,
        pub buffer_create_count: u64,
        pub buffer_reuse_count: u64,
        pub buffer_upload_count: u64,
    }

    pub(crate) struct WorldTextRenderPlugin;

    impl Plugin for WorldTextRenderPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins(ExtractComponentPlugin::<WorldTextDrawExtract>::default());
            let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
                return;
            };
            render_app
                .init_resource::<WorldTextRenderDiagnostics>()
                .init_resource::<SpecializedMeshPipelines<WorldTextPipeline>>()
                .add_render_command::<Transparent3d, DrawWorldText>()
                .add_systems(
                    Render,
                    (
                        queue_world_text.in_set(RenderSet::QueueMeshes),
                        prepare_world_text_buffers.in_set(RenderSet::PrepareResources),
                    ),
                );
        }

        fn finish(&self, app: &mut App) {
            let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
                return;
            };
            render_app.init_resource::<WorldTextPipeline>();
        }
    }

    #[derive(Resource)]
    struct WorldTextPipeline {
        shader: Handle<Shader>,
        mesh_pipeline: MeshPipeline,
        text_pipeline: TextInstancedPipeline,
    }

    impl FromWorld for WorldTextPipeline {
        fn from_world(world: &mut World) -> Self {
            Self {
                shader: TEXT_SHADER_HANDLE.clone(),
                mesh_pipeline: world.resource::<MeshPipeline>().clone(),
                text_pipeline: world.resource::<TextInstancedPipeline>().clone(),
            }
        }
    }

    impl SpecializedMeshPipeline for WorldTextPipeline {
        type Key = MeshPipelineKey;

        fn specialize(
            &self,
            key: Self::Key,
            layout: &MeshVertexBufferLayoutRef,
        ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
            let mut descriptor = self
                .mesh_pipeline
                .specialize(key | MeshPipelineKey::BLEND_ALPHA, layout)?;
            descriptor.label = Some("world_text_pipeline".into());
            descriptor.vertex.shader = self.shader.clone();
            descriptor
                .vertex
                .shader_defs
                .push(ShaderDefVal::Bool("WORLD_TEXT".into(), true));
            if let Some(fragment) = descriptor.fragment.take() {
                descriptor.fragment = Some(FragmentState {
                    shader: self.shader.clone(),
                    shader_defs: vec![ShaderDefVal::Bool("WORLD_TEXT".into(), true)],
                    entry_point: "fragment".into(),
                    targets: fragment.targets,
                });
            }
            descriptor.primitive.cull_mode = None;
            if let Some(depth) = descriptor.depth_stencil.as_mut() {
                depth.depth_write_enabled = false;
            }
            descriptor
                .layout
                .push(self.text_pipeline.atlas_layout.clone());
            descriptor
                .layout
                .push(self.text_pipeline.style_layout.clone());
            descriptor
                .layout
                .push(self.text_pipeline.deform_layout.clone());
            descriptor
                .layout
                .push(self.text_pipeline.path_layout.clone());
            descriptor
                .layout
                .push(self.text_pipeline.warp_layout.clone());
            let base_stride = size_of::<GlyphInstanceGpu>() as u64;
            descriptor.vertex.buffers.push(VertexBufferLayout {
                array_stride: size_of::<WorldGlyphInstanceGpu>() as u64,
                step_mode: VertexStepMode::Instance,
                attributes: vec![
                    VertexAttribute {
                        format: VertexFormat::Float32x4,
                        offset: 0,
                        shader_location: 5,
                    },
                    VertexAttribute {
                        format: VertexFormat::Float32x4,
                        offset: VertexFormat::Float32x4.size(),
                        shader_location: 6,
                    },
                    VertexAttribute {
                        format: VertexFormat::Float32x4,
                        offset: VertexFormat::Float32x4.size() * 2,
                        shader_location: 7,
                    },
                    VertexAttribute {
                        format: VertexFormat::Float32x4,
                        offset: VertexFormat::Float32x4.size() * 3,
                        shader_location: 8,
                    },
                    VertexAttribute {
                        format: VertexFormat::Float32x4,
                        offset: VertexFormat::Float32x4.size() * 4,
                        shader_location: 9,
                    },
                    VertexAttribute {
                        format: VertexFormat::Float32x4,
                        offset: VertexFormat::Float32x4.size() * 5,
                        shader_location: 10,
                    },
                    VertexAttribute {
                        format: VertexFormat::Float32x4,
                        offset: VertexFormat::Float32x4.size() * 6,
                        shader_location: 11,
                    },
                    VertexAttribute {
                        format: VertexFormat::Float32x4,
                        offset: VertexFormat::Float32x4.size() * 7,
                        shader_location: 12,
                    },
                    VertexAttribute {
                        format: VertexFormat::Float32x4,
                        offset: base_stride,
                        shader_location: 13,
                    },
                    VertexAttribute {
                        format: VertexFormat::Float32x4,
                        offset: base_stride + VertexFormat::Float32x4.size(),
                        shader_location: 14,
                    },
                    VertexAttribute {
                        format: VertexFormat::Float32x4,
                        offset: base_stride + VertexFormat::Float32x4.size() * 2,
                        shader_location: 15,
                    },
                ],
            });
            Ok(descriptor)
        }
    }

    fn queue_world_text(
        draw_functions: Res<DrawFunctions<Transparent3d>>,
        pipeline: Res<WorldTextPipeline>,
        mut pipelines: ResMut<SpecializedMeshPipelines<WorldTextPipeline>>,
        pipeline_cache: Res<PipelineCache>,
        meshes: Res<RenderAssets<RenderMesh>>,
        render_mesh_instances: Res<RenderMeshInstances>,
        draws: Query<(Entity, &MainEntity, &WorldTextDrawExtract)>,
        mut phases: ResMut<ViewSortedRenderPhases<Transparent3d>>,
        views: Query<(&ExtractedView, &Msaa)>,
        mut diagnostics: ResMut<WorldTextRenderDiagnostics>,
    ) {
        diagnostics.queued_draw_count = 0;
        diagnostics.queued_instance_count = 0;
        let draw_function = draw_functions.read().id::<DrawWorldText>();
        for (view, msaa) in &views {
            let Some(phase) = phases.get_mut(&view.retained_view_entity) else {
                continue;
            };
            let view_key = MeshPipelineKey::from_msaa_samples(msaa.samples())
                | MeshPipelineKey::from_hdr(view.hdr);
            for (entity, main_entity, extract) in &draws {
                let Some(mesh_instance) =
                    render_mesh_instances.render_mesh_queue_data(*main_entity)
                else {
                    continue;
                };
                let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id) else {
                    continue;
                };
                let key = view_key
                    | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology())
                    | MeshPipelineKey::BLEND_ALPHA;
                let Ok(pipeline_id) =
                    pipelines.specialize(&pipeline_cache, &pipeline, key, &mesh.layout)
                else {
                    continue;
                };
                let instance_count = extract.instances.len() as u32;
                phase.add(Transparent3d {
                    entity: (entity, *main_entity),
                    pipeline: pipeline_id,
                    draw_function,
                    distance: f32::NEG_INFINITY,
                    batch_range: 0..instance_count,
                    extra_index: PhaseItemExtraIndex::None,
                    indexed: mesh.indexed(),
                });
                diagnostics.queued_draw_count += 1;
                diagnostics.queued_instance_count += instance_count as u64;
            }
        }
    }

    fn prepare_world_text_buffers(
        mut commands: Commands,
        draws: Query<(Entity, &WorldTextDrawExtract)>,
        mut buffers: Query<&mut WorldTextInstanceBuffer>,
        render_device: Res<RenderDevice>,
        render_queue: Res<bevy::render::renderer::RenderQueue>,
        mut diagnostics: ResMut<WorldTextRenderDiagnostics>,
    ) {
        for (entity, extract) in &draws {
            let needed = extract.instances.len();
            if let Ok(mut existing) = buffers.get_mut(entity) {
                if existing.version == extract.version {
                    diagnostics.buffer_reuse_count += 1;
                    continue;
                }
                if existing.capacity >= needed {
                    render_queue.write_buffer(
                        &existing.buffer,
                        0,
                        bytemuck::cast_slice(&extract.instances),
                    );
                    existing.version = extract.version;
                    existing.count = needed;
                    diagnostics.buffer_upload_count += 1;
                    continue;
                }
            }
            let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                label: Some("world_text_instance_buffer"),
                contents: bytemuck::cast_slice(&extract.instances),
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            });
            commands.entity(entity).insert(WorldTextInstanceBuffer {
                buffer,
                capacity: needed,
                count: needed,
                version: extract.version,
            });
            diagnostics.buffer_create_count += 1;
        }
    }

    type DrawWorldText = (
        SetItemPipeline,
        SetMeshViewBindGroup<0>,
        SetMeshBindGroup<1>,
        SetTextAtlasBindGroup<2>,
        SetTextStyleBindGroup<3>,
        SetTextDeformBindGroup<4>,
        SetTextPathBindGroup<5>,
        SetTextWarpBindGroup<6>,
        DrawWorldTextMesh,
    );

    struct DrawWorldTextMesh;

    impl<P: PhaseItem> RenderCommand<P> for DrawWorldTextMesh {
        type Param = (
            SRes<RenderAssets<RenderMesh>>,
            SRes<RenderMeshInstances>,
            SRes<MeshAllocator>,
        );
        type ViewQuery = ();
        type ItemQuery = Read<WorldTextInstanceBuffer>;

        fn render<'w>(
            item: &P,
            _view: (),
            instance_buffer: Option<&'w WorldTextInstanceBuffer>,
            (meshes, render_mesh_instances, mesh_allocator): SystemParamItem<'w, '_, Self::Param>,
            pass: &mut TrackedRenderPass<'w>,
        ) -> RenderCommandResult {
            let mesh_allocator = mesh_allocator.into_inner();
            let Some(mesh_instance) = render_mesh_instances
                .into_inner()
                .render_mesh_queue_data(item.main_entity())
            else {
                return RenderCommandResult::Skip;
            };
            let Some(mesh) = meshes.into_inner().get(mesh_instance.mesh_asset_id) else {
                return RenderCommandResult::Skip;
            };
            let Some(instance_buffer) = instance_buffer else {
                return RenderCommandResult::Skip;
            };
            let Some(vertex_slice) = mesh_allocator.mesh_vertex_slice(&mesh_instance.mesh_asset_id)
            else {
                return RenderCommandResult::Skip;
            };
            pass.set_vertex_buffer(0, vertex_slice.buffer.slice(..));
            pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));
            let instance_range = 0..instance_buffer.count as u32;
            match &mesh.buffer_info {
                RenderMeshBufferInfo::Indexed {
                    index_format,
                    count,
                } => {
                    let Some(index_slice) =
                        mesh_allocator.mesh_index_slice(&mesh_instance.mesh_asset_id)
                    else {
                        return RenderCommandResult::Skip;
                    };
                    pass.set_index_buffer(index_slice.buffer.slice(..), 0, *index_format);
                    pass.draw_indexed(
                        index_slice.range.start..(index_slice.range.start + count),
                        vertex_slice.range.start as i32,
                        instance_range,
                    );
                }
                RenderMeshBufferInfo::NonIndexed => {
                    pass.draw(vertex_slice.range, instance_range);
                }
            }
            RenderCommandResult::Success
        }
    }

    pub(crate) fn spawn_world_text_draw_entity(commands: &mut Commands, quad: Handle<Mesh>) {
        let entity = commands
            .spawn((
                Mesh3d(quad),
                WorldTextDrawInstances::default(),
                WorldTextInstancedDraw,
                bevy::render::batching::NoAutomaticBatching,
                SyncToRenderWorld,
                NoFrustumCulling,
                NoIndirectDrawing,
            ))
            .id();
        commands.insert_resource(WorldTextDrawEntity(entity));
    }
}

#[cfg(feature = "world-text-3d")]
pub(crate) use render_3d::{
    spawn_world_text_draw_entity, WorldTextRenderDiagnostics, WorldTextRenderPlugin,
};

#[cfg(not(feature = "world-text-3d"))]
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct WorldTextRenderDiagnostics {
    pub queued_draw_count: u64,
    pub queued_instance_count: u64,
    pub buffer_create_count: u64,
    pub buffer_reuse_count: u64,
    pub buffer_upload_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_instances_normalize_run_and_pack_placement() {
        let glyphs = vec![
            GlyphInstanceGpu {
                pos_size: [10.0, 20.0, 8.0, 12.0],
                color: [1.0; 4],
                ..Default::default()
            },
            GlyphInstanceGpu {
                pos_size: [20.0, 20.0, 10.0, 12.0],
                color: [1.0; 4],
                ..Default::default()
            },
        ];
        let placement = WorldTextBillboard {
            anchor: Vec3::new(1.0, 2.0, 3.0),
            near_height: 4.0,
            base_alpha_ratio: 0.8,
            ..Default::default()
        };
        let world = build_world_glyph_instances(&glyphs, placement);
        assert_eq!(world.len(), 2);
        assert_eq!(world[0].anchor_height, [1.0, 2.0, 3.0, 4.0]);
        assert!((world[0].glyph.pos_size[0] + 10.0 / 12.0).abs() < f32::EPSILON);
        assert!((world[1].glyph.pos_size[0] - 0.0).abs() < f32::EPSILON);
        assert!((world[0].glyph.color[3] - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn natural_run_aspect_matches_shaped_run_width_over_height() {
        let glyphs = vec![
            GlyphInstanceGpu {
                pos_size: [0.0, 0.0, 10.0, 20.0],
                ..Default::default()
            },
            GlyphInstanceGpu {
                pos_size: [30.0, 0.0, 10.0, 20.0],
                ..Default::default()
            },
        ];
        assert!((natural_run_aspect_from_glyphs(&glyphs) - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn screen_companion_mode_packs_visual_envelope_and_mode_sentinel() {
        let glyphs = vec![GlyphInstanceGpu {
            pos_size: [0.0, 0.0, 10.0, 20.0],
            ..Default::default()
        }];
        let placement = WorldTextBillboard {
            placement_mode: WorldTextPlacementMode::ScreenCompanion,
            visual_envelope_world_height: 3.5,
            ..Default::default()
        };
        let world = build_world_glyph_instances(&glyphs, placement);
        assert_eq!(world[0].size_params[3], WORLD_TEXT_SCREEN_COMPANION_MODE);
        assert_eq!(world[0].size_params[2], 0.0);
        assert!((world[0].anchor_height[3] - 3.5).abs() < f32::EPSILON);

        let focused = WorldTextBillboard {
            screen_companion_focused: true,
            ..placement
        };
        let world_focused = build_world_glyph_instances(&glyphs, focused);
        assert_eq!(world_focused[0].size_params[2], 1.0);
    }
}
