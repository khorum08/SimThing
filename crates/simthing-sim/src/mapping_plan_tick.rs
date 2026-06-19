//! Resident tick seam for driver-compiled generic mapping plans.
//!
//! Plans contain semantic-free GPU operator descriptors only. Scenario/spec/driver
//! meaning stays outside sim; tick lifecycle and resident operator state live here.

use simthing_gpu::wgpu::Buffer;
use simthing_gpu::{
    cpu_min_plus_d_from_w, cpu_stencil_step, extract_d_flat, params_from_config,
    scoped_debug_readback_allowed, IndexedScatterOp, MinPlusStencilConfig, MinPlusStencilOp,
    MinPlusTraversalExecutionMode, MinPlusTraversalExecutionOptions, MinPlusTraversalInput,
    ScatterEntry, StructuredFieldStencilConfig, StructuredFieldStencilOp, WImpedanceComposeConfig,
    WImpedanceComposeOp,
};

use crate::accumulator_plan_tick::SimTickError;

/// Generic compiled mapping step — operator descriptors only.
#[derive(Clone, Debug)]
pub enum CompiledMappingStep {
    StructuredFieldStencil {
        config: StructuredFieldStencilConfig,
        hops: u32,
        /// Optional generic column scatter from field output into the shared interleaved buffer.
        interleaved_column_writes: Vec<(u32, u32)>,
    },
    WImpedanceCompose {
        config: WImpedanceComposeConfig,
    },
    MinPlusStencil {
        config: MinPlusStencilConfig,
        iterations: u32,
    },
}

/// Semantic-free compiled mapping plan for resident sim tick execution.
#[derive(Clone, Debug)]
pub struct CompiledMappingPlan {
    pub steps: Vec<CompiledMappingStep>,
    pub interleaved_width: u32,
    pub interleaved_height: u32,
    pub interleaved_n_dims: u32,
}

impl CompiledMappingPlan {
    pub fn interleaved_values_len(&self) -> usize {
        (self.interleaved_width * self.interleaved_height * self.interleaved_n_dims) as usize
    }

    pub fn structured_field_step_count(&self) -> usize {
        self.steps
            .iter()
            .filter(|step| matches!(step, CompiledMappingStep::StructuredFieldStencil { .. }))
            .count()
    }

    pub fn needs_interleaved_buffer(&self) -> bool {
        self.steps.iter().any(|step| match step {
            CompiledMappingStep::WImpedanceCompose { .. }
            | CompiledMappingStep::MinPlusStencil { .. } => true,
            CompiledMappingStep::StructuredFieldStencil {
                interleaved_column_writes,
                ..
            } => !interleaved_column_writes.is_empty(),
        })
    }
}

/// Per-tick inputs for mapping plan execution (generic buffers only).
pub struct MappingTickInputs<'a> {
    pub structured_field_values: &'a [Vec<f32>],
    pub interleaved_values: Option<&'a [f32]>,
}

/// Explicit readback policy for sim-owned mapping ticks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimGpuMappingReadbackPolicy {
    /// Production/resident tick: dispatch without proof readback.
    None,
    /// Proof/presentation: read back output values after tick.
    ProofReadback,
}

/// Mapping tick output — proof values are projection/cache only.
#[derive(Debug, Clone, PartialEq)]
pub struct SimGpuMappingTickOutput {
    pub proof_values: Option<Vec<f32>>,
}

enum ResidentMappingStep {
    StructuredField {
        op: StructuredFieldStencilOp,
        hops: u32,
        scatter_entries: Vec<ScatterEntry>,
    },
    WImpedanceCompose {
        config: WImpedanceComposeConfig,
    },
    MinPlus {
        op: MinPlusStencilOp,
        iterations: u32,
    },
}

/// Resident sim-owned GPU tick state for a compiled mapping plan.
pub struct SimGpuMappingTickState {
    plan: CompiledMappingPlan,
    steps: Vec<ResidentMappingStep>,
    w_compose_op: WImpedanceComposeOp,
    scatter_op: IndexedScatterOp,
    interleaved_buffer: Option<Buffer>,
    tick_count: u32,
}

impl SimGpuMappingTickState {
    /// Initialize resident mapping tick state from a compiled plan.
    pub fn new(
        ctx: &simthing_gpu::GpuContext,
        plan: CompiledMappingPlan,
    ) -> Result<Self, SimTickError> {
        if plan.steps.is_empty() {
            return Err(SimTickError::Readback("empty mapping plan".into()));
        }

        let mut steps = Vec::with_capacity(plan.steps.len());
        for step in &plan.steps {
            match step {
                CompiledMappingStep::StructuredFieldStencil {
                    config,
                    hops,
                    interleaved_column_writes,
                } => {
                    config.validate().map_err(map_structured_field_err)?;
                    let scatter_entries = if interleaved_column_writes.is_empty() {
                        Vec::new()
                    } else {
                        build_column_scatter_entries(
                            config.cells(),
                            config.n_dims,
                            plan.interleaved_n_dims,
                            interleaved_column_writes,
                        )
                    };
                    let op = StructuredFieldStencilOp::new(ctx, config.clone())
                        .map_err(map_structured_field_err)?;
                    steps.push(ResidentMappingStep::StructuredField {
                        op,
                        hops: *hops,
                        scatter_entries,
                    });
                }
                CompiledMappingStep::WImpedanceCompose { config } => {
                    config.validate().map_err(map_w_compose_err)?;
                    steps.push(ResidentMappingStep::WImpedanceCompose {
                        config: config.clone(),
                    });
                }
                CompiledMappingStep::MinPlusStencil { config, iterations } => {
                    config.validate().map_err(map_min_plus_err)?;
                    let op =
                        MinPlusStencilOp::new(ctx, config.clone()).map_err(map_min_plus_err)?;
                    steps.push(ResidentMappingStep::MinPlus {
                        op,
                        iterations: *iterations,
                    });
                }
            }
        }

        let interleaved_buffer = if plan.needs_interleaved_buffer() {
            let bytes = (plan.interleaved_values_len() * std::mem::size_of::<f32>()) as u64;
            Some(
                ctx.device
                    .create_buffer(&simthing_gpu::wgpu::BufferDescriptor {
                        label: Some("sim_mapping_plan_interleaved"),
                        size: bytes,
                        usage: simthing_gpu::wgpu::BufferUsages::STORAGE
                            | simthing_gpu::wgpu::BufferUsages::COPY_DST,
                        mapped_at_creation: false,
                    }),
            )
        } else {
            None
        };

        Ok(Self {
            plan,
            steps,
            w_compose_op: WImpedanceComposeOp::new(ctx),
            scatter_op: IndexedScatterOp::new(ctx),
            interleaved_buffer,
            tick_count: 0,
        })
    }

    pub fn plan(&self) -> &CompiledMappingPlan {
        &self.plan
    }

    pub fn resident_tick_count(&self) -> u32 {
        self.tick_count
    }

    /// Execute one mapping tick with explicit readback policy.
    pub fn tick(
        &mut self,
        ctx: &simthing_gpu::GpuContext,
        inputs: MappingTickInputs<'_>,
        readback: SimGpuMappingReadbackPolicy,
    ) -> Result<SimGpuMappingTickOutput, SimTickError> {
        validate_mapping_tick_inputs(&self.plan, &inputs)?;

        if let (Some(buffer), Some(values)) = (&self.interleaved_buffer, inputs.interleaved_values)
        {
            ctx.queue
                .write_buffer(buffer, 0, bytemuck::cast_slice(values));
        }

        let mut structured_field_index = 0usize;
        let mut last_structured_values: Option<Vec<f32>> = None;
        let mut last_min_plus_iterations: Option<u32> = None;

        for step in &mut self.steps {
            match step {
                ResidentMappingStep::StructuredField {
                    op,
                    hops,
                    scatter_entries,
                } => {
                    let values = &inputs.structured_field_values[structured_field_index];
                    structured_field_index += 1;
                    op.upload_values(ctx, values)
                        .map_err(map_structured_field_err)?;
                    op.dispatch_ping_pong(ctx, *hops)
                        .map_err(map_structured_field_err)?;

                    if let (Some(interleaved), false) =
                        (&self.interleaved_buffer, scatter_entries.is_empty())
                    {
                        let src = structured_field_values_buffer(op, *hops);
                        self.scatter_op
                            .dispatch(ctx, src, interleaved, scatter_entries)
                            .map_err(map_scatter_err)?;
                    }

                    if readback == SimGpuMappingReadbackPolicy::ProofReadback {
                        last_structured_values = Some(run_with_proof_readback_enabled(|| {
                            Ok(op.readback_after_ping_pong(ctx, *hops))
                        })?);
                    }
                }
                ResidentMappingStep::WImpedanceCompose { config } => {
                    let interleaved = self.interleaved_buffer.as_ref().ok_or_else(|| {
                        SimTickError::Readback("missing interleaved buffer".into())
                    })?;
                    self.w_compose_op
                        .compose_resident_field(ctx, interleaved, config)
                        .map_err(map_w_compose_err)?;
                }
                ResidentMappingStep::MinPlus { op, iterations } => {
                    let interleaved = self.interleaved_buffer.as_ref().ok_or_else(|| {
                        SimTickError::Readback("missing interleaved buffer".into())
                    })?;
                    let mode = match readback {
                        SimGpuMappingReadbackPolicy::None => {
                            MinPlusTraversalExecutionMode::GpuResident
                        }
                        SimGpuMappingReadbackPolicy::ProofReadback => {
                            MinPlusTraversalExecutionMode::DiagnosticReadback
                        }
                    };
                    let report = op
                        .dispatch_traversal_from_input(
                            ctx,
                            MinPlusTraversalInput::GpuInterleavedW(interleaved),
                            None,
                            MinPlusTraversalExecutionOptions {
                                mode,
                                iterations: *iterations,
                            },
                        )
                        .map_err(map_min_plus_err)?;
                    if readback == SimGpuMappingReadbackPolicy::ProofReadback {
                        last_structured_values = report.values;
                    }
                    last_min_plus_iterations = Some(*iterations);
                }
            }
        }

        self.tick_count += 1;

        let proof_values = match readback {
            SimGpuMappingReadbackPolicy::None => None,
            SimGpuMappingReadbackPolicy::ProofReadback => {
                if last_min_plus_iterations.is_some() {
                    let values = last_structured_values.ok_or_else(|| {
                        SimTickError::Readback("min-plus proof readback missing values".into())
                    })?;
                    let config = match self.plan.steps.last() {
                        Some(CompiledMappingStep::MinPlusStencil { config, .. }) => config,
                        _ => {
                            return Err(SimTickError::Readback(
                                "expected min-plus terminal step".into(),
                            ));
                        }
                    };
                    Some(extract_d_flat(&values, config).map_err(map_min_plus_err)?)
                } else {
                    last_structured_values
                }
            }
        };

        Ok(SimGpuMappingTickOutput { proof_values })
    }
}

fn structured_field_values_buffer<'a>(op: &'a StructuredFieldStencilOp, hops: u32) -> &'a Buffer {
    if hops % 2 == 1 {
        &op.output_buffer
    } else {
        &op.input_buffer
    }
}

fn build_column_scatter_entries(
    cells: u32,
    field_n_dims: u32,
    interleaved_n_dims: u32,
    writes: &[(u32, u32)],
) -> Vec<ScatterEntry> {
    let mut entries = Vec::new();
    for &(field_col, interleaved_col) in writes {
        for slot in 0..cells {
            entries.push(ScatterEntry {
                src_index: slot * field_n_dims + field_col,
                dst_index: slot * interleaved_n_dims + interleaved_col,
            });
        }
    }
    entries
}

fn validate_mapping_tick_inputs(
    plan: &CompiledMappingPlan,
    inputs: &MappingTickInputs<'_>,
) -> Result<(), SimTickError> {
    let expected = plan.structured_field_step_count();
    if inputs.structured_field_values.len() != expected {
        return Err(SimTickError::InvalidInputLength {
            expected,
            actual: inputs.structured_field_values.len(),
        });
    }
    for (index, values) in inputs.structured_field_values.iter().enumerate() {
        let CompiledMappingStep::StructuredFieldStencil { config, .. } = plan
            .steps
            .iter()
            .filter(|step| matches!(step, CompiledMappingStep::StructuredFieldStencil { .. }))
            .nth(index)
            .expect("structured field step")
        else {
            unreachable!("filtered structured field step");
        };
        let expected_len = config.values_len();
        if values.len() != expected_len {
            return Err(SimTickError::InvalidInputLength {
                expected: expected_len,
                actual: values.len(),
            });
        }
    }
    if plan.needs_interleaved_buffer() {
        let interleaved = inputs.interleaved_values.ok_or_else(|| {
            SimTickError::Readback("interleaved values required for mapping plan".into())
        })?;
        let expected_len = plan.interleaved_values_len();
        if interleaved.len() != expected_len {
            return Err(SimTickError::InvalidInputLength {
                expected: expected_len,
                actual: interleaved.len(),
            });
        }
    }
    Ok(())
}

fn run_with_proof_readback_enabled<T>(
    f: impl FnOnce() -> Result<T, SimTickError>,
) -> Result<T, SimTickError> {
    let _guard = scoped_debug_readback_allowed(true);
    f()
}

fn map_structured_field_err(err: simthing_gpu::StructuredFieldStencilError) -> SimTickError {
    SimTickError::GpuAccumulator(err.to_string())
}

fn map_w_compose_err(err: simthing_gpu::WImpedanceComposeError) -> SimTickError {
    SimTickError::GpuAccumulator(err.to_string())
}

fn map_min_plus_err(err: simthing_gpu::MinPlusStencilError) -> SimTickError {
    SimTickError::GpuAccumulator(err.to_string())
}

fn map_scatter_err(err: simthing_gpu::IndexedScatterError) -> SimTickError {
    SimTickError::GpuAccumulator(err.to_string())
}

/// CPU oracle for a single structured-field stencil step.
pub fn cpu_structured_field_horizon(
    values: &[f32],
    config: &StructuredFieldStencilConfig,
    hops: u32,
) -> Vec<f32> {
    let params = params_from_config(config);
    let mut cur = values.to_vec();
    for _ in 0..hops {
        cur = cpu_stencil_step(&cur, &params);
    }
    cur
}

/// CPU oracle for min-plus D field using composed W column in interleaved layout.
pub fn cpu_min_plus_d_from_composed_interleaved(
    interleaved: &[f32],
    stencil: &MinPlusStencilConfig,
    iterations: u32,
) -> Result<Vec<f32>, SimTickError> {
    let cells = stencil.cells() as usize;
    let mut w_flat = vec![0.0f32; cells];
    for slot in 0..cells as u32 {
        w_flat[slot as usize] = interleaved[idx(slot, stencil.w_col, stencil.n_dims)];
    }
    cpu_min_plus_d_from_w(&w_flat, stencil, iterations).map_err(map_min_plus_err)
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}
