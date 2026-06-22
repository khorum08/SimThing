use std::{collections::HashSet, mem::size_of};

use bevy::{
    core_pipeline::core_2d::{graph::Core2d, AlphaMask2d, Opaque2d, Transparent2d},
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    ecs::system::{lifetimeless::*, SystemParamItem},
    math::FloatOrd,
    prelude::*,
    render::{
        camera::{
            Camera, CameraRenderGraph, OrthographicProjection, RenderTarget, ScalingMode, Viewport,
        },
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        mesh::{
            allocator::MeshAllocator, MeshVertexBufferLayoutRef, RenderMesh, RenderMeshBufferInfo,
        },
        render_asset::RenderAssets,
        render_phase::{
            AddRenderCommand, DrawFunctions, PhaseItem, PhaseItemExtraIndex, RenderCommand,
            RenderCommandResult, SetItemPipeline, TrackedRenderPass, ViewBinnedRenderPhases,
            ViewSortedRenderPhases,
        },
        render_resource::{
            binding_types::{sampler, texture_2d},
            BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, Buffer,
            BufferInitDescriptor, BufferUsages, PipelineCache, RenderPipelineDescriptor,
            SamplerBindingType, ShaderStages, SpecializedMeshPipeline,
            SpecializedMeshPipelineError, SpecializedMeshPipelines, TextureSampleType,
            VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode,
        },
        renderer::RenderDevice,
        sync_world::MainEntity,
        texture::{FallbackImage, GpuImage},
        view::{Msaa, NoIndirectDrawing, RetainedViewEntity, ViewUniforms},
        Extract, ExtractSchedule, Render, RenderApp, RenderSet,
    },
    sprite::{
        Mesh2dPipeline, Mesh2dPipelineKey, RenderMesh2dInstances, SetMesh2dBindGroup,
        SetMesh2dViewBindGroup,
    },
};

use crate::bevy::{GlyphInstanceGpu, TextAggregateVersion, TextDrawExtract, TEXT_SHADER_HANDLE};

/// Marker for entities drawn by the text instanced pipeline.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TextInstancedDraw;

/// Offscreen Core2d views for queue-wiring tests. Bypasses `Camera2d` until Route A
/// (`Camera2d` + `Tonemapping::None` + standard image readback) is wired; see render notes.
#[derive(Component, Clone, Copy, Debug, Default, ExtractComponent)]
#[extract_component_filter(With<Camera>)]
pub struct TextOffscreenCamera;

/// GPU atlas image handle mirrored into the render world.
#[derive(Resource, Clone, ExtractResource, Debug)]
pub struct TextAtlasImageHandle(pub Handle<Image>);

/// Render-world perf counters merged into [`crate::bevy::TextPerfDiagnostics`].
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextRenderPerfDiagnostics {
    pub extract_clone_count: u64,
    pub extracted_instance_count: u64,
    pub instance_buffer_create_count: u64,
    pub instance_buffer_reuse_count: u64,
    pub instance_buffer_upload_count: u64,
    pub queued_draw_count: u64,
    pub queued_instance_count: u64,
}

/// Diagnostics for render-queue tests (render world).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextRenderQueueState {
    pub queued_draw_count: u32,
    pub queued_instance_count: u32,
    pub draw_entities_seen: u32,
    pub views_seen: u32,
    pub skipped_empty_instances: u32,
    pub skipped_missing_mesh_instance: u32,
    pub skipped_missing_mesh: u32,
    pub skipped_missing_phase: u32,
    pub skipped_pipeline_error: u32,
}

/// Render-world instance buffer for one draw entity.
#[derive(Component, Clone)]
pub struct TextInstanceBuffer {
    pub buffer: Buffer,
    pub capacity_instances: usize,
    pub data_version: u64,
}

/// Shared atlas bind group for the text shader (@group(2)).
#[derive(Resource, Clone)]
pub struct TextAtlasBindGroupResource {
    pub bind_group: BindGroup,
}

pub(crate) struct TextInstancedRenderPlugin;

impl Plugin for TextInstancedRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<TextAtlasImageHandle>::default())
            .add_plugins(ExtractComponentPlugin::<TextOffscreenCamera>::default());

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        use bevy::render::batching::gpu_preprocessing::{
            GpuPreprocessingMode, GpuPreprocessingSupport,
        };
        if !render_app
            .world()
            .contains_resource::<GpuPreprocessingSupport>()
        {
            render_app.insert_resource(GpuPreprocessingSupport {
                max_supported_mode: GpuPreprocessingMode::PreprocessingOnly,
            });
        }

        render_app
            .init_resource::<TextRenderQueueState>()
            .init_resource::<TextRenderPerfDiagnostics>()
            .init_resource::<SpecializedMeshPipelines<TextInstancedPipeline>>()
            .add_render_command::<Transparent2d, DrawTextInstanced>()
            .add_systems(ExtractSchedule, stamp_text_draw_extract_version)
            .add_systems(ExtractSchedule, extract_text_offscreen_phases)
            .add_systems(
                Render,
                (
                    queue_text_instanced.in_set(RenderSet::QueueMeshes),
                    prepare_text_instance_buffers.in_set(RenderSet::PrepareResources),
                    (
                        prepare_text_atlas_bind_group,
                        prepare_text_offscreen_mesh2d_view_bind_groups,
                    )
                        .in_set(RenderSet::PrepareBindGroups),
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.init_resource::<TextInstancedPipeline>();
    }
}

#[derive(Resource, Clone)]
pub struct TextInstancedPipeline {
    pub shader: Handle<Shader>,
    pub mesh2d_pipeline: Mesh2dPipeline,
    pub atlas_layout: BindGroupLayout,
}

impl FromWorld for TextInstancedPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let atlas_layout = render_device.create_bind_group_layout(
            "text_atlas_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                ),
            ),
        );

        Self {
            shader: TEXT_SHADER_HANDLE.clone(),
            mesh2d_pipeline: world.resource::<Mesh2dPipeline>().clone(),
            atlas_layout,
        }
    }
}

impl SpecializedMeshPipeline for TextInstancedPipeline {
    type Key = Mesh2dPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let blend_key = key | Mesh2dPipelineKey::BLEND_ALPHA;
        let mut descriptor = self.mesh2d_pipeline.specialize(blend_key, layout)?;
        descriptor.vertex.shader = self.shader.clone();
        if let Some(fragment) = descriptor.fragment.as_mut() {
            fragment.shader = self.shader.clone();
        }
        descriptor.layout.push(self.atlas_layout.clone());
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: size_of::<GlyphInstanceGpu>() as u64,
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
            ],
        });
        Ok(descriptor)
    }
}

fn stamp_text_draw_extract_version(
    version: Res<TextAggregateVersion>,
    mut draw_extracts: Query<&mut TextDrawExtract>,
    mut last_version: Local<Option<u64>>,
    mut render_diag: ResMut<TextRenderPerfDiagnostics>,
) {
    if draw_extracts.is_empty() {
        return;
    }
    if *last_version != Some(version.current) {
        render_diag.extract_clone_count += 1;
        *last_version = Some(version.current);
    }
    render_diag.extracted_instance_count = draw_extracts
        .iter()
        .next()
        .map(|extract| extract.instances.len() as u64)
        .unwrap_or(0);
    for mut extract in &mut draw_extracts {
        extract.data_version = version.current;
    }
}

fn extract_text_offscreen_phases(
    mut transparent_2d_phases: ResMut<ViewSortedRenderPhases<Transparent2d>>,
    mut opaque_2d_phases: ResMut<ViewBinnedRenderPhases<Opaque2d>>,
    mut alpha_mask_2d_phases: ResMut<ViewBinnedRenderPhases<AlphaMask2d>>,
    cameras: Extract<Query<(Entity, &Camera), With<TextOffscreenCamera>>>,
    mut live_entities: Local<HashSet<RetainedViewEntity>>,
) {
    live_entities.clear();

    for (main_entity, camera) in &cameras {
        if !camera.is_active {
            continue;
        }

        let retained_view_entity = RetainedViewEntity::new(main_entity.into(), None, 0);
        transparent_2d_phases.insert_or_clear(retained_view_entity);
        opaque_2d_phases.prepare_for_new_frame(
            retained_view_entity,
            bevy::render::batching::gpu_preprocessing::GpuPreprocessingMode::None,
        );
        alpha_mask_2d_phases.prepare_for_new_frame(
            retained_view_entity,
            bevy::render::batching::gpu_preprocessing::GpuPreprocessingMode::None,
        );
        live_entities.insert(retained_view_entity);
    }

    transparent_2d_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
    opaque_2d_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
    alpha_mask_2d_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
}

fn prepare_text_offscreen_mesh2d_view_bind_groups(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mesh2d_pipeline: Res<Mesh2dPipeline>,
    view_uniforms: Res<ViewUniforms>,
    views: Query<
        Entity,
        (
            With<bevy::render::view::ExtractedView>,
            With<TextOffscreenCamera>,
        ),
    >,
    globals_buffer: Res<bevy::render::globals::GlobalsBuffer>,
    fallback_image: Res<FallbackImage>,
) {
    let (Some(view_binding), Some(globals)) = (
        view_uniforms.uniforms.binding(),
        globals_buffer.buffer.binding(),
    ) else {
        return;
    };

    let lut_texture = &fallback_image.d3.texture_view;
    let lut_sampler = &fallback_image.d3.sampler;

    for entity in &views {
        let view_bind_group = render_device.create_bind_group(
            "text_offscreen_mesh2d_view_bind_group",
            &mesh2d_pipeline.view_layout,
            &BindGroupEntries::with_indices((
                (0, view_binding.clone()),
                (1, globals.clone()),
                (2, lut_texture),
                (3, lut_sampler),
            )),
        );
        commands
            .entity(entity)
            .insert(bevy::sprite::Mesh2dViewBindGroup {
                value: view_bind_group,
            });
    }
}

fn queue_text_instanced(
    draw_functions: Res<DrawFunctions<Transparent2d>>,
    pipeline: Res<TextInstancedPipeline>,
    mut pipelines: ResMut<SpecializedMeshPipelines<TextInstancedPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    meshes: Res<RenderAssets<RenderMesh>>,
    render_mesh_instances: Res<RenderMesh2dInstances>,
    draw_entities: Query<(Entity, &MainEntity, &TextDrawExtract)>,
    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent2d>>,
    mut queue_state: ResMut<TextRenderQueueState>,
    mut render_diag: ResMut<TextRenderPerfDiagnostics>,
    views: Query<(&bevy::render::view::ExtractedView, &Msaa)>,
) {
    queue_state.queued_draw_count = 0;
    queue_state.queued_instance_count = 0;
    queue_state.draw_entities_seen = 0;
    queue_state.views_seen = 0;
    queue_state.skipped_empty_instances = 0;
    queue_state.skipped_missing_mesh_instance = 0;
    queue_state.skipped_missing_mesh = 0;
    queue_state.skipped_missing_phase = 0;
    queue_state.skipped_pipeline_error = 0;

    let draw_custom = draw_functions.read().id::<DrawTextInstanced>();

    for (view, msaa) in &views {
        queue_state.views_seen += 1;
        let Some(transparent_phase) = transparent_render_phases.get_mut(&view.retained_view_entity)
        else {
            queue_state.skipped_missing_phase += 1;
            continue;
        };

        let view_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples())
            | Mesh2dPipelineKey::from_hdr(view.hdr);
        let rangefinder = view.rangefinder3d();

        for (entity, main_entity, extract) in &draw_entities {
            queue_state.draw_entities_seen += 1;
            if extract.instances.is_empty() {
                queue_state.skipped_empty_instances += 1;
                continue;
            }
            let Some(mesh_instance) = render_mesh_instances.get(main_entity) else {
                queue_state.skipped_missing_mesh_instance += 1;
                continue;
            };
            let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id) else {
                queue_state.skipped_missing_mesh += 1;
                continue;
            };

            let key =
                view_key | Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology());
            let pipeline_id =
                match pipelines.specialize(&pipeline_cache, &pipeline, key, &mesh.layout) {
                    Ok(id) => id,
                    Err(_) => {
                        queue_state.skipped_pipeline_error += 1;
                        continue;
                    }
                };

            let instance_count = extract.instances.len() as u32;
            transparent_phase.add(Transparent2d {
                sort_key: FloatOrd(rangefinder.distance_translation(&Vec3::ZERO)),
                entity: (entity, *main_entity),
                pipeline: pipeline_id,
                draw_function: draw_custom,
                batch_range: 0..instance_count,
                extracted_index: 0,
                extra_index: PhaseItemExtraIndex::None,
                indexed: mesh.indexed(),
            });

            queue_state.queued_draw_count += 1;
            queue_state.queued_instance_count += instance_count;
        }
    }

    render_diag.queued_draw_count = queue_state.queued_draw_count as u64;
    render_diag.queued_instance_count = queue_state.queued_instance_count as u64;
}

fn prepare_text_instance_buffers(
    mut commands: Commands,
    query: Query<(Entity, &TextDrawExtract)>,
    mut buffers: Query<&mut TextInstanceBuffer>,
    render_device: Res<RenderDevice>,
    render_queue: Res<bevy::render::renderer::RenderQueue>,
    mut render_diag: ResMut<TextRenderPerfDiagnostics>,
) {
    for (entity, extract) in &query {
        if extract.instances.is_empty() {
            commands.entity(entity).remove::<TextInstanceBuffer>();
            continue;
        }

        let needed = extract.instances.len();
        if let Ok(mut existing) = buffers.get_mut(entity) {
            if existing.data_version == extract.data_version {
                render_diag.instance_buffer_reuse_count += 1;
                continue;
            }
            if existing.capacity_instances >= needed {
                render_queue.write_buffer(
                    &existing.buffer,
                    0,
                    bytemuck::cast_slice(&extract.instances),
                );
                existing.data_version = extract.data_version;
                render_diag.instance_buffer_upload_count += 1;
                continue;
            }
        }

        render_diag.instance_buffer_create_count += 1;
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("text_glyph_instance_buffer"),
            contents: bytemuck::cast_slice(&extract.instances),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        commands.entity(entity).insert(TextInstanceBuffer {
            buffer,
            capacity_instances: needed,
            data_version: extract.data_version,
        });
    }
}

fn prepare_text_atlas_bind_group(
    mut commands: Commands,
    pipeline: Res<TextInstancedPipeline>,
    atlas_handle: Option<Res<TextAtlasImageHandle>>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    render_device: Res<RenderDevice>,
) {
    let Some(atlas_handle) = atlas_handle else {
        commands.remove_resource::<TextAtlasBindGroupResource>();
        return;
    };
    let Some(gpu_image) = gpu_images.get(&atlas_handle.0) else {
        return;
    };
    let bind_group = render_device.create_bind_group(
        "text_atlas_bind_group",
        &pipeline.atlas_layout,
        &BindGroupEntries::sequential((&gpu_image.texture_view, &gpu_image.sampler)),
    );
    commands.insert_resource(TextAtlasBindGroupResource { bind_group });
}

pub type DrawTextInstanced = (
    SetItemPipeline,
    SetMesh2dViewBindGroup<0>,
    SetMesh2dBindGroup<1>,
    SetTextAtlasBindGroup<2>,
    DrawTextInstancedMesh,
);

pub struct SetTextAtlasBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetTextAtlasBindGroup<I> {
    type Param = SRes<TextAtlasBindGroupResource>;
    type ViewQuery = ();
    type ItemQuery = ();

    fn render<'w>(
        _item: &P,
        _view: (),
        _item_query: Option<()>,
        atlas_bind_group: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let atlas_bind_group = atlas_bind_group.into_inner();
        pass.set_bind_group(I, &atlas_bind_group.bind_group, &[]);
        RenderCommandResult::Success
    }
}

pub struct DrawTextInstancedMesh;

impl<P: PhaseItem> RenderCommand<P> for DrawTextInstancedMesh {
    type Param = (
        SRes<RenderAssets<RenderMesh>>,
        SRes<RenderMesh2dInstances>,
        SRes<MeshAllocator>,
    );
    type ViewQuery = ();
    type ItemQuery = Read<TextInstanceBuffer>;

    fn render<'w>(
        item: &P,
        _view: (),
        instance_buffer: Option<&'w TextInstanceBuffer>,
        (meshes, render_mesh_instances, mesh_allocator): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let meshes = meshes.into_inner();
        let render_mesh_instances = render_mesh_instances.into_inner();
        let mesh_allocator = mesh_allocator.into_inner();

        let Some(mesh_instance) = render_mesh_instances.get(&item.main_entity()) else {
            return RenderCommandResult::Skip;
        };
        let Some(gpu_mesh) = meshes.get(mesh_instance.mesh_asset_id) else {
            return RenderCommandResult::Skip;
        };
        let Some(instance_buffer) = instance_buffer else {
            return RenderCommandResult::Skip;
        };
        let Some(vertex_buffer_slice) =
            mesh_allocator.mesh_vertex_slice(&mesh_instance.mesh_asset_id)
        else {
            return RenderCommandResult::Skip;
        };

        pass.set_vertex_buffer(0, vertex_buffer_slice.buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));

        let instance_range = item.batch_range().clone();
        match &gpu_mesh.buffer_info {
            RenderMeshBufferInfo::Indexed {
                index_format,
                count,
            } => {
                let Some(index_buffer_slice) =
                    mesh_allocator.mesh_index_slice(&mesh_instance.mesh_asset_id)
                else {
                    return RenderCommandResult::Skip;
                };
                pass.set_index_buffer(index_buffer_slice.buffer.slice(..), 0, *index_format);
                pass.draw_indexed(
                    index_buffer_slice.range.start..(index_buffer_slice.range.start + count),
                    vertex_buffer_slice.range.start as i32,
                    instance_range,
                );
            }
            RenderMeshBufferInfo::NonIndexed => {
                pass.draw(vertex_buffer_slice.range, instance_range);
            }
        }
        RenderCommandResult::Success
    }
}

pub fn text_render_queue_state(app: &App) -> TextRenderQueueState {
    app.get_sub_app(RenderApp)
        .and_then(|render_app| {
            render_app
                .world()
                .get_resource::<TextRenderQueueState>()
                .copied()
        })
        .unwrap_or_default()
}

pub fn text_instanced_pipeline_initialized(app: &App) -> bool {
    app.get_sub_app(RenderApp).is_some_and(|render_app| {
        render_app
            .world()
            .contains_resource::<TextInstancedPipeline>()
    })
}

/// Camera bundle for offscreen instanced text draws in pixel-space coordinates.
pub fn text_render_camera_bundle(target: Handle<Image>, width: u32, height: u32) -> impl Bundle {
    let projection = OrthographicProjection {
        near: -1000.0,
        scaling_mode: ScalingMode::Fixed {
            width: width as f32,
            height: height as f32,
        },
        // cosmic-text uses a top-left origin with +y down; match that in world space.
        area: Rect::new(0.0, -(height as f32), width as f32, 0.0),
        viewport_origin: Vec2::ZERO,
        ..OrthographicProjection::default_2d()
    };
    (
        Camera {
            target: RenderTarget::Image(target.into()),
            clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 1.0)),
            viewport: Some(Viewport {
                physical_position: UVec2::ZERO,
                physical_size: UVec2::new(width, height),
                ..default()
            }),
            ..default()
        },
        CameraRenderGraph::new(Core2d),
        TextOffscreenCamera,
        Tonemapping::None,
        DebandDither::Disabled,
        Projection::Orthographic(projection),
        GlobalTransform::IDENTITY,
        Transform::default(),
        Visibility::default(),
        Msaa::Off,
        NoIndirectDrawing,
    )
}
