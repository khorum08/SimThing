//! B-4 world-value summary runtime — summaries from `WorldGpuState.values`.

use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoder, CommandEncoderDescriptor, ComputePassDescriptor, ComputePipeline,
    ComputePipelineDescriptor, PipelineLayoutDescriptor, ShaderModuleDescriptor, ShaderSource,
};

use super::session::{AccumulatorOpSessionError, WORKGROUP_SIZE};
use super::types::{AccumulatorSummaryParams, SlotSummary, SlotSummaryGpu};
use crate::context::GpuContext;

/// Dedicated B-4 summary resources for integrated world execution (C-INF remedial).
pub struct WorldSummaryRuntime {
    summary_buffer: Buffer,
    summary_uniform: Buffer,
    summary_layout: BindGroupLayout,
    summary_pipeline: ComputePipeline,
    n_slots: u32,
    n_dims: u32,
}

impl WorldSummaryRuntime {
    pub fn new(ctx: &GpuContext, n_slots: u32, n_dims: u32) -> Self {
        assert!(n_slots > 0 && n_dims > 0, "n_slots and n_dims must be > 0");
        let device = &ctx.device;
        let summary_len = (n_slots as u64) * std::mem::size_of::<SlotSummaryGpu>() as u64;

        let summary_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("world_summary_buffer"),
            size: summary_len,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let summary_uniform = device.create_buffer(&BufferDescriptor {
            label: Some("world_summary_uniform"),
            size: std::mem::size_of::<AccumulatorSummaryParams>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("world_summary_shader"),
            source: ShaderSource::Wgsl(include_str!("../shaders/accumulator_op.wgsl").into()),
        });

        let summary_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("world_summary_layout"),
            entries: &[
                storage_entry(0, false),
                storage_entry(1, false),
                uniform_entry(2),
            ],
        });

        let summary_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("world_summary_pipeline"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("world_summary_pl"),
                bind_group_layouts: &[&summary_layout],
                push_constant_ranges: &[],
            })),
            module: &shader,
            entry_point: "write_summaries",
            compilation_options: Default::default(),
            cache: None,
        });

        Self {
            summary_buffer,
            summary_uniform,
            summary_layout,
            summary_pipeline,
            n_slots,
            n_dims,
        }
    }

    pub fn n_slots(&self) -> u32 {
        self.n_slots
    }

    pub fn n_dims(&self) -> u32 {
        self.n_dims
    }

    pub fn encode_into(&self, ctx: &GpuContext, encoder: &mut CommandEncoder, values: &Buffer) {
        self.encode_write_summaries_for_values(ctx, encoder, values);
    }

    pub fn dispatch(&self, ctx: &GpuContext, values: &Buffer) {
        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("world_summary_encoder"),
            });
        self.encode_into(ctx, &mut encoder, values);
        ctx.queue.submit(Some(encoder.finish()));
    }

    pub fn readback(
        &self,
        ctx: &GpuContext,
    ) -> Result<Vec<SlotSummary>, AccumulatorOpSessionError> {
        let bytes = read_buffer_bytes(ctx, &self.summary_buffer);
        let gpu: &[SlotSummaryGpu] = bytemuck::cast_slice(&bytes);
        Ok(gpu
            .iter()
            .map(|s| SlotSummary {
                slot: s.slot,
                flags: s.flags,
                checksum_all: s.checksum_all,
                group_checksums: s.group_checksums,
            })
            .collect())
    }

    fn encode_write_summaries_for_values(
        &self,
        ctx: &GpuContext,
        encoder: &mut CommandEncoder,
        values: &Buffer,
    ) {
        let summary_params = AccumulatorSummaryParams {
            n_slots: self.n_slots,
            n_dims: self.n_dims,
            _pad0: 0,
            _pad1: 0,
        };
        ctx.queue.write_buffer(
            &self.summary_uniform,
            0,
            bytemuck::bytes_of(&summary_params),
        );

        let summary_bind_group = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("world_summary_bg"),
            layout: &self.summary_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: values.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: self.summary_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: self.summary_uniform.as_entire_binding(),
                },
            ],
        });

        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("world_summary_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.summary_pipeline);
        pass.set_bind_group(0, &summary_bind_group, &[]);
        let groups = self.n_slots.div_ceil(WORKGROUP_SIZE);
        pass.dispatch_workgroups(groups, 1, 1);
    }
}

fn read_buffer_bytes(ctx: &GpuContext, buf: &Buffer) -> Vec<u8> {
    let size = buf.size();
    let staging = ctx.device.create_buffer(&BufferDescriptor {
        label: Some("world_summary_staging_read"),
        size,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut encoder = ctx
        .device
        .create_command_encoder(&CommandEncoderDescriptor {
            label: Some("world_summary_readback_encoder"),
        });
    encoder.copy_buffer_to_buffer(buf, 0, &staging, 0, size);
    ctx.queue.submit(Some(encoder.finish()));

    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    ctx.device.poll(wgpu::Maintain::Wait);
    let mapped = slice.get_mapped_range();
    mapped.to_vec()
}

fn storage_entry(binding: u32, read_only: bool) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn uniform_entry(binding: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accumulator_op::{set_debug_readback_allowed, summaries_from_values};
    use simthing_core::SimProperty;

    fn write_slot_values(state: &mut crate::WorldGpuState, slot: u32, cols: &[f32]) {
        let mut values = state.read_values();
        let base = slot as usize * state.n_dims as usize;
        for (i, v) in cols.iter().enumerate() {
            values[base + i] = *v;
        }
        state.install_resolved_values_at_boundary(&values);
    }

    #[test]
    fn b4_world_summary_matches_values_direct_dispatch() {
        set_debug_readback_allowed(true);
        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut reg = simthing_core::DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "a", 0));
        let n_slots = 3u32;
        let n_dims = reg.total_columns as u32;
        let mut state = crate::WorldGpuState::new(ctx, &reg, n_slots);
        write_slot_values(&mut state, 0, &[1.0, 2.0, 3.0]);
        write_slot_values(&mut state, 1, &[0.5, 0.25, 0.125]);

        let summary = WorldSummaryRuntime::new(&state.ctx, n_slots, n_dims);
        summary.dispatch(&state.ctx, &state.resolved.values());
        let gpu = summary.readback(&state.ctx).unwrap();
        let cpu = summaries_from_values(&state.read_values(), n_slots, n_dims);
        assert_eq!(gpu, cpu);
    }

    #[test]
    fn b4_world_summary_n_dims_less_than_four() {
        set_debug_readback_allowed(true);
        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut reg = simthing_core::DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "a", 0));
        let n_slots = 2u32;
        let n_dims = 2u32;
        let mut state = crate::WorldGpuState::new(ctx, &reg, n_slots);
        write_slot_values(&mut state, 0, &[10.0, 20.0]);

        let summary = WorldSummaryRuntime::new(&state.ctx, n_slots, n_dims);
        summary.dispatch(&state.ctx, &state.resolved.values());
        assert_eq!(
            summary.readback(&state.ctx).unwrap(),
            summaries_from_values(&state.read_values(), n_slots, n_dims)
        );
    }

    #[test]
    fn b4_world_summary_n_dims_sixty_four() {
        set_debug_readback_allowed(true);
        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut reg = simthing_core::DimensionRegistry::new();
        for i in 0..64 {
            reg.register(SimProperty::simple("core", &format!("c{i}"), 0));
        }
        let n_slots = 1u32;
        let n_dims = 64u32;
        let mut state = crate::WorldGpuState::new(ctx, &reg, n_slots);
        let mut cols = vec![0.0f32; 64];
        cols[0] = 1.0;
        cols[63] = 2.0;
        write_slot_values(&mut state, 0, &cols);

        let summary = WorldSummaryRuntime::new(&state.ctx, n_slots, n_dims);
        summary.dispatch(&state.ctx, &state.resolved.values());
        assert_eq!(
            summary.readback(&state.ctx).unwrap(),
            summaries_from_values(&state.read_values(), n_slots, n_dims)
        );
    }

    #[test]
    fn b4_world_summary_unchanged_when_values_unchanged() {
        set_debug_readback_allowed(true);
        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut reg = simthing_core::DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "a", 0));
        let n_slots = 1u32;
        let n_dims = reg.total_columns as u32;
        let mut state = crate::WorldGpuState::new(ctx, &reg, n_slots);
        write_slot_values(&mut state, 0, &[1.0, 2.0, 3.0]);

        let summary = WorldSummaryRuntime::new(&state.ctx, n_slots, n_dims);
        summary.dispatch(&state.ctx, &state.resolved.values());
        let first = summary.readback(&state.ctx).unwrap();
        summary.dispatch(&state.ctx, &state.resolved.values());
        let second = summary.readback(&state.ctx).unwrap();
        assert_eq!(first, second);
    }
}
