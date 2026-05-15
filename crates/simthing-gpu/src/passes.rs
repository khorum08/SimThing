//! Compute pipelines and dispatch wrappers for Passes 0/1/2.
//!
//! Each pass owns its shader module, bind group layout, pipeline layout, and
//! pipeline. Bind groups are created per-dispatch from the supplied
//! `WorldGpuState` (cheap; lets us reuse one `Pipelines` instance across
//! multiple `WorldGpuState`s if needed).
//!
//! Uniform buffer (`Params { delta_time, n_dims }`) is shared across passes
//! and rewritten on each dispatch with the current dt.

use bytemuck::{Pod, Zeroable};
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor,
    BufferUsages, CommandEncoderDescriptor, ComputePassDescriptor, ComputePipeline,
    ComputePipelineDescriptor, PipelineLayoutDescriptor, ShaderModuleDescriptor,
    ShaderSource, ShaderStages,
};

use crate::context::GpuContext;
use crate::world_state::WorldGpuState;

const WORKGROUP_SIZE: u32 = 64;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct PassParams {
    delta_time: f32,
    n_dims:     u32,
    _pad0:      u32,
    _pad1:      u32,
}

pub struct Pipelines {
    uniform_buffer: Buffer,

    snapshot_layout:   BindGroupLayout,
    snapshot_pipeline: ComputePipeline,

    velocity_layout:   BindGroupLayout,
    velocity_pipeline: ComputePipeline,

    intensity_layout:   BindGroupLayout,
    intensity_pipeline: ComputePipeline,
}

impl Pipelines {
    pub fn new(ctx: &GpuContext) -> Self {
        let device = &ctx.device;

        // Uniform buffer — small, frequently overwritten.
        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("pass_params_uniform"),
            size:  std::mem::size_of::<PassParams>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // ── Pass 0: snapshot ────────────────────────────────────────────────
        let snapshot_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("snapshot_bgl"),
            entries: &[
                storage_entry(0, /*read_only*/ true),
                storage_entry(1, /*read_only*/ false),
            ],
        });
        let snapshot_module = device.create_shader_module(ShaderModuleDescriptor {
            label:  Some("snapshot_shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/snapshot.wgsl").into()),
        });
        let snapshot_pl_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("snapshot_pl_layout"),
            bind_group_layouts: &[&snapshot_layout],
            push_constant_ranges: &[],
        });
        let snapshot_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("snapshot_pipeline"),
            layout: Some(&snapshot_pl_layout),
            module: &snapshot_module,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });

        // ── Pass 1: velocity integration ────────────────────────────────────
        let velocity_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("velocity_bgl"),
            entries: &[
                storage_entry(0, /*read_only*/ false), // values (rw)
                storage_entry(1, /*read_only*/ true),  // pairs
                uniform_entry(2),                       // params
            ],
        });
        let velocity_module = device.create_shader_module(ShaderModuleDescriptor {
            label:  Some("velocity_shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/velocity_integration.wgsl").into()),
        });
        let velocity_pl_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("velocity_pl_layout"),
            bind_group_layouts: &[&velocity_layout],
            push_constant_ranges: &[],
        });
        let velocity_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("velocity_pipeline"),
            layout: Some(&velocity_pl_layout),
            module: &velocity_module,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });

        // ── Pass 2: intensity update ────────────────────────────────────────
        let intensity_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("intensity_bgl"),
            entries: &[
                storage_entry(0, /*read_only*/ false), // values (rw)
                storage_entry(1, /*read_only*/ true),  // intensity_params
                uniform_entry(2),                       // pass_params
            ],
        });
        let intensity_module = device.create_shader_module(ShaderModuleDescriptor {
            label:  Some("intensity_shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/intensity_update.wgsl").into()),
        });
        let intensity_pl_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("intensity_pl_layout"),
            bind_group_layouts: &[&intensity_layout],
            push_constant_ranges: &[],
        });
        let intensity_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("intensity_pipeline"),
            layout: Some(&intensity_pl_layout),
            module: &intensity_module,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });

        Self {
            uniform_buffer,
            snapshot_layout, snapshot_pipeline,
            velocity_layout, velocity_pipeline,
            intensity_layout, intensity_pipeline,
        }
    }

    fn write_params(&self, ctx: &GpuContext, state: &WorldGpuState, dt: f32) {
        let p = PassParams {
            delta_time: dt,
            n_dims:     state.n_dims,
            _pad0: 0, _pad1: 0,
        };
        ctx.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&p));
    }

    pub fn run_snapshot(&self, state: &WorldGpuState) {
        let ctx = &state.ctx;
        let bg = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("snapshot_bg"),
            layout: &self.snapshot_layout,
            entries: &[
                BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                BindGroupEntry { binding: 1, resource: state.previous_values.as_entire_binding() },
            ],
        });

        let total = state.n_slots * state.n_dims;
        let groups = total.div_ceil(WORKGROUP_SIZE);

        let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("snapshot_encoder"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("snapshot_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.snapshot_pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups(groups, 1, 1);
        }
        ctx.queue.submit(Some(encoder.finish()));
    }

    pub fn run_velocity_integration(&self, state: &WorldGpuState, dt: f32) {
        if state.n_governed_pairs == 0 { return; }
        let ctx = &state.ctx;
        self.write_params(ctx, state, dt);

        let bg = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("velocity_bg"),
            layout: &self.velocity_layout,
            entries: &[
                BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                BindGroupEntry { binding: 1, resource: state.governed_pairs.as_entire_binding() },
                BindGroupEntry { binding: 2, resource: self.uniform_buffer.as_entire_binding() },
            ],
        });

        let total = state.n_slots * state.n_governed_pairs;
        let groups = total.div_ceil(WORKGROUP_SIZE);

        let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("velocity_encoder"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("velocity_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.velocity_pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups(groups, 1, 1);
        }
        ctx.queue.submit(Some(encoder.finish()));
    }

    pub fn run_intensity_update(&self, state: &WorldGpuState, dt: f32) {
        if state.n_intensity_params == 0 { return; }
        let ctx = &state.ctx;
        self.write_params(ctx, state, dt);

        let bg = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("intensity_bg"),
            layout: &self.intensity_layout,
            entries: &[
                BindGroupEntry { binding: 0, resource: state.values.as_entire_binding() },
                BindGroupEntry { binding: 1, resource: state.intensity_params.as_entire_binding() },
                BindGroupEntry { binding: 2, resource: self.uniform_buffer.as_entire_binding() },
            ],
        });

        let total = state.n_slots * state.n_intensity_params;
        let groups = total.div_ceil(WORKGROUP_SIZE);

        let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("intensity_encoder"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("intensity_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.intensity_pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups(groups, 1, 1);
        }
        ctx.queue.submit(Some(encoder.finish()));
    }
}

fn storage_entry(binding: u32, read_only: bool) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::COMPUTE,
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
        visibility: ShaderStages::COMPUTE,
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
    use simthing_core::evaluate::Evaluator;
    use simthing_core::{
        DimensionRegistry, IntensityBehavior, PropertyValue, SimProperty, SimThing,
        SimThingKind, SubFieldRole,
    };

    fn try_gpu() -> Option<GpuContext> {
        GpuContext::new_blocking().ok()
    }

    fn loyalty_property() -> SimProperty {
        let mut p = SimProperty::simple("core", "loyalty", 0);
        p.intensity_behavior = Some(IntensityBehavior::default());
        p
    }

    fn assert_bits_eq(label: &str, cpu: &[f32], gpu: &[f32]) {
        assert_eq!(cpu.len(), gpu.len(), "{label}: length mismatch");
        for (i, (a, b)) in cpu.iter().zip(gpu.iter()).enumerate() {
            assert_eq!(
                a.to_bits(), b.to_bits(),
                "{label}: index {i} diverges — cpu={a} ({:08x}), gpu={b} ({:08x})",
                a.to_bits(), b.to_bits(),
            );
        }
    }

    #[test]
    fn snapshot_copies_values_to_previous() {
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };
        let mut reg = DimensionRegistry::new();
        reg.register(loyalty_property());
        let state = WorldGpuState::new(ctx, &reg, 3);

        let input: Vec<f32> = (0..state.values_len()).map(|i| (i as f32) * 0.13 + 0.5).collect();
        state.write_values(&input);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_snapshot(&state);

        let prev = state.read_previous_values();
        assert_bits_eq("snapshot", &input, &prev);
    }

    /// Pass 1 alone, dt = 1.0. Two slots: one mid-range, one at floor with
    /// negative velocity (exercises velocity pinning).
    #[test]
    fn velocity_integration_matches_cpu_oracle_dt_one() {
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };
        let mut reg = DimensionRegistry::new();
        let id = reg.register(loyalty_property());
        let layout = reg.property(id).layout.clone();
        let stride = layout.stride();

        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        let mut slots = vec![vec![0.0f32; stride], vec![0.0f32; stride]];
        slots[0][a_off] = 0.4;  slots[0][v_off] =  0.07; slots[0][i_off] = 0.2;
        slots[1][a_off] = 0.0;  slots[1][v_off] = -0.05; slots[1][i_off] = 0.3;

        let dt = 1.0;

        // CPU oracle.
        let mut cpu: Vec<Vec<f32>> = slots.iter().cloned().collect();
        for d in &mut cpu {
            let mut pv = PropertyValue { data: d.clone() };
            pv.integrate(&layout, dt);
            *d = pv.data;
        }

        // GPU.
        let state = WorldGpuState::new(ctx, &reg, 2);
        let n_dims = state.n_dims as usize;
        let mut flat = vec![0.0f32; state.values_len()];
        for (s, d) in slots.iter().enumerate() {
            flat[s * n_dims .. s * n_dims + stride].copy_from_slice(d);
        }
        state.write_values(&flat);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_velocity_integration(&state, dt);

        let gpu_flat = state.read_values();
        for s in 0..2 {
            let gpu_slice = &gpu_flat[s * n_dims .. s * n_dims + stride];
            assert_bits_eq(&format!("slot {s}"), &cpu[s], gpu_slice);
        }
    }

    /// Pass 1 alone, dt = 0.5. Non-power-of-2 multiplier exercises potential
    /// FMA fusion between (vel * dt) and (value + ...). If this fails by 1 ULP,
    /// the FMA assumption in agents.md Option B is wrong and we need to switch
    /// to explicit fma() on both sides.
    #[test]
    fn velocity_integration_matches_cpu_oracle_fractional_dt() {
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };
        let mut reg = DimensionRegistry::new();
        let id = reg.register(loyalty_property());
        let layout = reg.property(id).layout.clone();
        let stride = layout.stride();

        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();

        let mut d = vec![0.0f32; stride];
        d[a_off] = 0.4_f32 + 1e-7;   // not a clean fraction
        d[v_off] = 0.07_f32 - 3e-8;  // not a clean fraction

        let dt = 0.5;
        let mut pv = PropertyValue { data: d.clone() };
        pv.integrate(&layout, dt);

        let state = WorldGpuState::new(ctx, &reg, 1);
        let mut flat = vec![0.0f32; state.values_len()];
        flat[..stride].copy_from_slice(&d);
        state.write_values(&flat);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_velocity_integration(&state, dt);

        let gpu_flat = state.read_values();
        assert_bits_eq("fractional-dt", &pv.data, &gpu_flat[..stride]);
    }

    /// Pass 2 alone. Uses raw initial velocity (no Pass 1 first), so CPU
    /// `update_intensity` is called directly on initial data.
    #[test]
    fn intensity_update_matches_cpu_oracle() {
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };
        let mut reg = DimensionRegistry::new();
        let id = reg.register(loyalty_property());
        let layout = reg.property(id).layout.clone();
        let behavior = reg.property(id).intensity_behavior.clone().unwrap();
        let stride = layout.stride();

        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        // Slot 0: high velocity → build branch. Slot 1: low velocity → decay branch.
        let mut slots = vec![vec![0.0f32; stride], vec![0.0f32; stride]];
        slots[0][v_off] = 0.09;  slots[0][i_off] = 0.2;
        slots[1][v_off] = 0.001; slots[1][i_off] = 0.7;

        let dt = 0.5;

        let mut cpu: Vec<Vec<f32>> = slots.iter().cloned().collect();
        for d in &mut cpu {
            let mut pv = PropertyValue { data: d.clone() };
            pv.update_intensity(&behavior, &layout, dt);
            *d = pv.data;
        }

        let state = WorldGpuState::new(ctx, &reg, 2);
        let n_dims = state.n_dims as usize;
        let mut flat = vec![0.0f32; state.values_len()];
        for (s, d) in slots.iter().enumerate() {
            flat[s * n_dims .. s * n_dims + stride].copy_from_slice(d);
        }
        state.write_values(&flat);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_intensity_update(&state, dt);

        let gpu_flat = state.read_values();
        for s in 0..2 {
            let gpu_slice = &gpu_flat[s * n_dims .. s * n_dims + stride];
            assert_bits_eq(&format!("slot {s}"), &cpu[s], gpu_slice);
        }
    }

    /// End-to-end parity: SlotAllocator + tree projection + Pass 0/1/2
    /// against simthing-core's `Evaluator` on a multi-node tree with multiple
    /// properties (one with intensity_behavior, one without). Verifies that
    /// the GPU pipeline driven from a real SimThing tree matches the CPU
    /// oracle bit-exactly across every (slot, property, column).
    #[test]
    fn tree_driven_pipeline_matches_evaluator() {
        use crate::projection::project_tree_to_values;
        use crate::slot::SlotAllocator;
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

        let mut reg = DimensionRegistry::new();
        let loyalty_id = reg.register(loyalty_property());
        let food_id    = reg.register(SimProperty::simple("core", "food_security", 0));

        let l_layout = reg.property(loyalty_id).layout.clone();
        let f_layout = reg.property(food_id).layout.clone();

        let la = l_layout.offset_of(&SubFieldRole::Amount).unwrap();
        let lv = l_layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let li = l_layout.offset_of(&SubFieldRole::Intensity).unwrap();
        let fa = f_layout.offset_of(&SubFieldRole::Amount).unwrap();
        let fv = f_layout.offset_of(&SubFieldRole::Velocity).unwrap();

        // Build tree: World → 2 Locations → 2 Cohorts each. 7 nodes total.
        // Each cohort carries loyalty; only first cohort of each location
        // carries food_security too. Velocities span build/decay branches
        // and one cohort starts at the loyalty floor (pinning case).
        let cohort_specs: [(f32, f32, f32); 4] = [
            // (loyalty_amount, loyalty_velocity, loyalty_intensity)
            (0.40,  0.07,  0.20),  // mid-range, building intensity
            (0.85, -0.001, 0.60),  // near ceiling, decay branch
            (0.00, -0.05,  0.30),  // at floor, negative vel → pinning
            (0.50,  0.09,  0.10),  // mid-range, building
        ];

        let mut world = SimThing::new(SimThingKind::World, 0);
        let mut cohort_ids = Vec::new();
        let mut loc_food_owners = Vec::new();
        for loc_i in 0..2 {
            let mut loc = SimThing::new(SimThingKind::Location, 0);
            for cj in 0..2 {
                let (la_v, lv_v, li_v) = cohort_specs[loc_i * 2 + cj];
                let mut cohort = SimThing::new(SimThingKind::Cohort, 0);

                let mut pv_l = PropertyValue::from_layout(&l_layout);
                pv_l.data[la] = la_v;
                pv_l.data[lv] = lv_v;
                pv_l.data[li] = li_v;
                cohort.add_property(loyalty_id, pv_l);

                if cj == 0 {
                    let mut pv_f = PropertyValue::from_layout(&f_layout);
                    pv_f.data[fa] = 0.7 + 0.05 * (loc_i as f32);
                    pv_f.data[fv] = 0.02;
                    cohort.add_property(food_id, pv_f);
                    loc_food_owners.push(cohort.id);
                }
                cohort_ids.push(cohort.id);
                loc.add_child(cohort);
            }
            world.add_child(loc);
        }

        let dt = 0.5;

        // CPU oracle.
        let cpu_snap = Evaluator::new(&reg, dt).evaluate(&world, 1);

        // Allocate slots for every node in the tree, then project.
        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);

        let state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
        let n_dims = state.n_dims as usize;
        let mut flat = vec![0.0f32; state.values_len()];
        project_tree_to_values(&world, &reg, &alloc, n_dims, &mut flat);
        state.write_values(&flat);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_snapshot(&state);
        pipelines.run_velocity_integration(&state, dt);
        pipelines.run_intensity_update(&state, dt);

        let gpu_flat = state.read_values();

        // Compare every CPU-snapshot entity's properties against the
        // corresponding slot row in the GPU buffer.
        for entity in &cpu_snap.entities {
            let slot = alloc.slot_of(entity.id)
                .unwrap_or_else(|| panic!("entity {:?} not allocated", entity.id));
            let slot_base = slot as usize * n_dims;

            for (prop_id, cpu_pv) in &entity.properties {
                let range = reg.column_range(*prop_id);
                let start = slot_base + range.start;
                let end   = start + cpu_pv.data.len();
                let gpu_data = &gpu_flat[start..end];
                let label = format!(
                    "entity {:?} slot {} prop {:?}",
                    entity.id, slot, prop_id,
                );
                assert_bits_eq(&label, &cpu_pv.data, gpu_data);
            }
        }

        // Pass 0 invariant: previous_values still matches the initial flat
        // buffer (snapshot ran before integration mutated values).
        let prev = state.read_previous_values();
        assert_bits_eq("Pass 0 (tree) snapshot", &flat, &prev);
    }

    /// Full Pass 0+1+2 pipeline matches simthing-core's `Evaluator` (the
    /// authoritative CPU oracle) on a single SimThing with one property
    /// and no overlays. Pass 0 result is verified via previous_values readback.
    #[test]
    fn full_pipeline_matches_evaluator() {
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

        let mut reg = DimensionRegistry::new();
        let id = reg.register(loyalty_property());
        let layout = reg.property(id).layout.clone();
        let stride = layout.stride();

        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        let mut pv = PropertyValue::from_layout(&layout);
        pv.data[a_off] = 0.4;
        pv.data[v_off] = 0.07;
        pv.data[i_off] = 0.2;
        cohort.add_property(id, pv);
        let cohort_id = cohort.id;
        let initial_data = cohort.properties[&id].data.clone();

        let dt = 0.5;

        let evaluator = Evaluator::new(&reg, dt);
        let snap = evaluator.evaluate(&cohort, 1);
        let cpu_data = &snap.get(cohort_id).unwrap().properties[&id].data;

        let state = WorldGpuState::new(ctx, &reg, 1);
        let range = reg.column_range(id).clone();
        let mut flat = vec![0.0f32; state.values_len()];
        flat[range.start..range.start + stride].copy_from_slice(&initial_data);
        state.write_values(&flat);

        let pipelines = Pipelines::new(&state.ctx);
        pipelines.run_snapshot(&state);
        pipelines.run_velocity_integration(&state, dt);
        pipelines.run_intensity_update(&state, dt);

        // Pass 0 invariant: previous_values must equal the pre-pass values.
        let prev = state.read_previous_values();
        assert_bits_eq(
            "Pass 0 snapshot",
            &flat,
            &prev,
        );

        let gpu_flat = state.read_values();
        let gpu_data = &gpu_flat[range.start..range.start + stride];
        assert_bits_eq("Evaluator vs GPU pipeline", cpu_data, gpu_data);
    }
}

