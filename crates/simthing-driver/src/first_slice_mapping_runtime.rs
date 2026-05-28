//! Phase M-first-slice — opt-in mapping runtime for one bounded RegionField grid.
//!
//! Wires M-1 stencil execution, M-2 FieldScheduler, Layer 2 Sum reduction, and
//! Layer 3 field_urgency EvalEML behind explicit [`MappingExecutionProfile`] opt-in.
//! Not wired into the default production pass graph.

use simthing_core::{
    column_aware_reduction_op, eml_opcode, AccumulatorOp, CombineFn, ConsumeMode, EmlConsumerMask,
    EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec,
    ScaleSpec, SourceSpec,
};
use simthing_gpu::{
    accumulator_op::set_debug_readback_allowed, AccumulatorOpSession, EmlGpuProgramTable, GpuContext,
    StructuredFieldExecutionOptions, StructuredFieldExecutionReport, StructuredFieldStencilBoundaryMode,
    StructuredFieldStencilConfig, StructuredFieldStencilMaskMode, StructuredFieldStencilOp,
    StructuredFieldStencilOperator, StructuredFieldStencilSourcePolicy,
};
use simthing_spec::{
    compile_region_field_preview, estimate_region_field_budget, CompiledFieldCadence,
    CompiledRegionFieldOperator, CompiledRegionFieldPreview, CompiledRegionFieldStencilSpec,
    MappingExecutionProfile, RegionFieldBudgetEstimate, RegionFieldBudgetError, RegionFieldBudgetSpec,
    RegionFieldIsolationPolicyEstimate, RegionFieldSpec, SpecError,
};
use thiserror::Error;

use crate::field_scheduler::{
    DirtyRegionState, FieldCadence, FieldDispatchSchedule, FieldId, FieldRegionId,
    FieldRegionRegistration, FieldScheduleState, FieldScheduler, FieldSchedulerReport,
    ScheduledSingleStencilExecution,
};

/// Bridge compiled stencil spec → GPU config (promoted from M-3 admission tests).
pub fn compiled_stencil_to_gpu_config(
    compiled: &CompiledRegionFieldStencilSpec,
) -> StructuredFieldStencilConfig {
    StructuredFieldStencilConfig {
        width: compiled.width,
        height: compiled.height,
        n_dims: compiled.n_dims,
        source_col: compiled.source_col,
        target_col: compiled.target_col,
        horizon: compiled.horizon,
        alpha_self: compiled.alpha_self,
        gamma_neighbor: compiled.gamma_neighbor,
        source_cap: compiled.source_cap,
        operator: match compiled.operator {
            CompiledRegionFieldOperator::Normalized => StructuredFieldStencilOperator::Normalized,
            CompiledRegionFieldOperator::SourceCappedNormalized => {
                StructuredFieldStencilOperator::SourceCappedNormalized
            }
        },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: compiled.allow_extended_horizon,
    }
}

/// Bridge compiled cadence → scheduler cadence.
pub fn compiled_cadence_to_field_cadence(compiled: CompiledFieldCadence) -> FieldCadence {
    match compiled {
        CompiledFieldCadence::EveryTick => FieldCadence::EveryTick,
        CompiledFieldCadence::EveryN { n } => FieldCadence::EveryN { n },
        CompiledFieldCadence::OnEvent => FieldCadence::OnEvent,
    }
}

/// One-shot seed cell for caller-managed source protocol.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FirstSliceSeed {
    pub row: u32,
    pub col: u32,
    pub value: f32,
}

impl FirstSliceSeed {
    pub fn validate(&self, width: u32, height: u32) -> Result<(), FirstSliceMappingError> {
        if self.row >= height || self.col >= width {
            return Err(FirstSliceMappingError::InvalidSeed {
                row: self.row,
                col: self.col,
                width,
                height,
            });
        }
        if !self.value.is_finite() {
            return Err(FirstSliceMappingError::NonFiniteSeedValue {
                row: self.row,
                col: self.col,
            });
        }
        Ok(())
    }
}

/// Per-tick execution options.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FirstSliceTickOptions {
    pub readback_values: bool,
    pub collect_field_stats: bool,
}

impl FirstSliceTickOptions {
    pub fn hot_path() -> Self {
        Self {
            readback_values: false,
            collect_field_stats: false,
        }
    }

    pub fn debug_readback() -> Self {
        Self {
            readback_values: true,
            collect_field_stats: false,
        }
    }
}

/// Per-tick mapping runtime report.
#[derive(Clone, Debug, PartialEq)]
pub struct FirstSliceMappingReport {
    pub enabled: bool,
    pub tick: u32,
    pub scheduled: bool,
    pub scheduler_report: Option<FieldSchedulerReport>,
    pub stencil_execution: Option<ScheduledSingleStencilExecution>,
    pub reduction_parent_value: Option<f32>,
    pub eml_output: Option<f32>,
    pub field_values: Option<Vec<f32>>,
    pub source_setup_dispatches: u32,
    pub propagation_dispatches: u32,
    pub total_dispatches: u32,
    pub reduction_executed: bool,
    pub eml_executed: bool,
}

impl FirstSliceMappingReport {
    fn disabled(tick: u32) -> Self {
        Self {
            enabled: false,
            tick,
            scheduled: false,
            scheduler_report: None,
            stencil_execution: None,
            reduction_parent_value: None,
            eml_output: None,
            field_values: None,
            source_setup_dispatches: 0,
            propagation_dispatches: 0,
            total_dispatches: 0,
            reduction_executed: false,
            eml_executed: false,
        }
    }
}

#[derive(Debug, Error)]
pub enum FirstSliceMappingError {
    #[error(transparent)]
    Spec(#[from] SpecError),
    #[error("mapping profile Disabled — runtime not enabled")]
    ProfileDisabled,
    #[error("first slice requires reduction binding")]
    MissingReduction,
    #[error("first slice requires parent formula binding")]
    MissingParentFormula,
    #[error("first slice requires field_urgency formula class")]
    MissingFieldUrgency,
    #[error(transparent)]
    Stencil(#[from] simthing_gpu::StructuredFieldStencilError),
    #[error(transparent)]
    Scheduler(#[from] crate::field_scheduler::FieldSchedulerError),
    #[error("EML setup failed: {0}")]
    EmlSetup(String),
    #[error("accumulator session error: {0}")]
    Accumulator(String),
    #[error("invalid seed at row={row} col={col} for grid {width}x{height}")]
    InvalidSeed {
        row: u32,
        col: u32,
        width: u32,
        height: u32,
    },
    #[error("non-finite seed value at row={row} col={col}")]
    NonFiniteSeedValue { row: u32, col: u32 },
}

/// Opt-in first-slice mapping runtime session.
pub struct FirstSliceMappingSession {
    enabled: bool,
    preview: CompiledRegionFieldPreview,
    scheduler: FieldScheduler,
    stencil: StructuredFieldStencilOp,
    values: Vec<f32>,
    tick: u32,
    field_id: FieldId,
    region_id: FieldRegionId,
    width: u32,
    n_dims: u32,
    source_col: u32,
    eml_registry: EmlExpressionRegistry,
    eml_table: EmlGpuProgramTable,
    acc_session: AccumulatorOpSession,
    tree_id: u32,
    pending_seeds: Vec<FirstSliceSeed>,
    eml_weight_a_col: u32,
    eml_weight_b_col: u32,
    eml_output_col: u32,
    eml_resource_col: u32,
    seeds_applied_this_tick: bool,
    gpu_state_canonical: bool,
    host_values_valid: bool,
}

struct StencilTickResult {
    report: StructuredFieldExecutionReport,
    source_setup_dispatches: u32,
    propagation_dispatches: u32,
}

impl FirstSliceMappingSession {
    /// Open a first-slice session. Execution is enabled only when profile enables mapping.
    pub fn open(
        ctx: &GpuContext,
        profile: MappingExecutionProfile,
        spec: &RegionFieldSpec,
    ) -> Result<Self, FirstSliceMappingError> {
        let preview = compile_region_field_preview(spec)?;
        Self::open_preview(
            ctx,
            profile,
            preview,
            spec.parent_formula.as_ref().and_then(|f| f.tree_id),
        )
    }

    /// Open from an already-admitted compile preview.
    pub fn open_preview(
        ctx: &GpuContext,
        profile: MappingExecutionProfile,
        preview: CompiledRegionFieldPreview,
        tree_id_override: Option<u32>,
    ) -> Result<Self, FirstSliceMappingError> {
        let enabled = profile.enables_execution();
        let gpu_config = compiled_stencil_to_gpu_config(&preview.stencil);
        gpu_config.validate().map_err(FirstSliceMappingError::Stencil)?;

        let reduction = preview
            .reduction
            .as_ref()
            .ok_or(FirstSliceMappingError::MissingReduction)?;
        let formula_class = preview
            .parent_formula_class
            .as_deref()
            .ok_or(FirstSliceMappingError::MissingParentFormula)?;
        if formula_class != "field_urgency" {
            return Err(FirstSliceMappingError::MissingFieldUrgency);
        }

        let stencil = StructuredFieldStencilOp::new(ctx, gpu_config)?;
        let values = vec![0.0f32; stencil.config().values_len()];

        let mut scheduler = FieldScheduler::new();
        let field_id = FieldId(0);
        let region_id = FieldRegionId(0);
        scheduler.register_field(FieldScheduleState {
            field_id,
            cadence: compiled_cadence_to_field_cadence(preview.cadence),
            event_pending: false,
        });
        scheduler.register_region(FieldRegionRegistration {
            region_id,
            field_id,
            dirty: DirtyRegionState::default(),
        });

        let tree_id = tree_id_override.unwrap_or(1);
        let n_dims = preview.stencil.n_dims;
        let parent_slot = reduction.parent_slot;
        let n_slots = parent_slot + 1;
        let eml_resource_col = 1;
        let eml_weight_a_col = 2;
        let eml_weight_b_col = 3;
        let eml_output_col = 4;

        let (eml_registry, eml_table) = build_field_urgency_eml(ctx, tree_id)?;

        let acc_session = AccumulatorOpSession::new(ctx, n_slots, n_dims);

        stencil.upload_values(ctx, &values)?;

        Ok(Self {
            enabled,
            width: preview.grid_size,
            n_dims,
            source_col: preview.stencil.source_col,
            preview,
            scheduler,
            stencil,
            values,
            tick: 0,
            field_id,
            region_id,
            tree_id,
            pending_seeds: Vec::new(),
            eml_registry,
            eml_table,
            acc_session,
            eml_weight_a_col,
            eml_weight_b_col,
            eml_output_col,
            eml_resource_col,
            seeds_applied_this_tick: false,
            gpu_state_canonical: true,
            host_values_valid: true,
        })
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn preview(&self) -> &CompiledRegionFieldPreview {
        &self.preview
    }

    pub fn scheduler(&self) -> &FieldScheduler {
        &self.scheduler
    }

    pub fn scheduler_mut(&mut self) -> &mut FieldScheduler {
        &mut self.scheduler
    }

    pub fn values(&self) -> &[f32] {
        &self.values
    }

    pub fn tick_index(&self) -> u32 {
        self.tick
    }

    /// Diagnostic readback of canonical GPU field state (input buffer).
    pub fn readback_canonical_field(&self, ctx: &GpuContext) -> Vec<f32> {
        self.stencil.readback_input_buffer(ctx)
    }

    /// Diagnostic readback of Layer 2/3 results from current GPU field without re-running stencil.
    pub fn diagnostic_readback_reduction_eml(
        &mut self,
        ctx: &GpuContext,
        weights: (f32, f32),
    ) -> Result<(f32, f32), FirstSliceMappingError> {
        let (threat, urgency) = self.run_reduction_and_eml(ctx, true, weights.0, weights.1)?;
        Ok((threat.unwrap(), urgency.unwrap()))
    }

    /// Queue seeds for the next scheduled tick (caller-managed protocol).
    pub fn queue_seeds(&mut self, seeds: &[FirstSliceSeed]) -> Result<(), FirstSliceMappingError> {
        let height = self.preview.stencil.height;
        for seed in seeds {
            seed.validate(self.width, height)?;
        }
        self.pending_seeds.extend_from_slice(seeds);
        self.apply_pending_seeds_to_host();
        self.mark_dirty_source();
        Ok(())
    }

    fn mark_dirty_source(&mut self) {
        if let Some(region) = self
            .scheduler
            .regions_mut()
            .iter_mut()
            .find(|r| r.field_id == self.field_id && r.region_id == self.region_id)
        {
            region.dirty.dirty_source_present = true;
        }
    }

    fn slot_idx(&self, slot: u32, col: u32) -> usize {
        (slot * self.n_dims + col) as usize
    }

    fn seed_slot_col_writes(&self) -> (Vec<(u32, u32, f32)>, Vec<(u32, u32)>) {
        let source_col = self.source_col;
        let width = self.width;
        let mut writes = Vec::with_capacity(self.pending_seeds.len());
        let mut zeros = Vec::with_capacity(self.pending_seeds.len());
        for seed in &self.pending_seeds {
            let slot = seed.row * width + seed.col;
            writes.push((slot, source_col, seed.value));
            zeros.push((slot, source_col));
        }
        (writes, zeros)
    }

    fn apply_pending_seeds_to_host(&mut self) {
        let source_col = self.source_col;
        let n_dims = self.n_dims;
        let width = self.width;
        for seed in &self.pending_seeds {
            let slot = seed.row * width + seed.col;
            let i = (slot * n_dims + source_col) as usize;
            if i < self.values.len() {
                self.values[i] = seed.value;
            }
        }
        self.seeds_applied_this_tick = !self.pending_seeds.is_empty();
    }

    fn run_caller_managed_stencil(
        &mut self,
        ctx: &GpuContext,
        options: FirstSliceTickOptions,
    ) -> Result<StencilTickResult, FirstSliceMappingError> {
        let horizon = self.stencil.config().horizon;
        let mut source_setup_dispatches = 0u32;

        if self.seeds_applied_this_tick {
            let (writes, zeros) = self.seed_slot_col_writes();
            self.stencil.write_cell_values(ctx, &self.stencil.input_buffer, &writes)?;
            source_setup_dispatches += self.stencil.dispatch_once(
                ctx,
                &self.stencil.input_buffer,
                &self.stencil.output_buffer,
            );
            self.stencil
                .zero_cell_values(ctx, &self.stencil.output_buffer, &zeros)?;
            self.stencil.copy_output_to_input(ctx);
            self.pending_seeds.clear();
            self.seeds_applied_this_tick = false;
        }

        let report = self.stencil.execute_configured(
            ctx,
            StructuredFieldExecutionOptions {
                readback_values: options.readback_values,
                collect_field_stats: options.collect_field_stats,
                steps: None,
            },
        )?;
        let propagation_dispatches = report.debug.dispatch_count;
        self.stencil
            .canonicalize_input_after_ping_pong(ctx, horizon);
        self.gpu_state_canonical = true;

        if options.readback_values {
            if let Some(ref vals) = report.values {
                self.values.clone_from(vals);
            } else {
                self.values = self.stencil.readback_input_buffer(ctx);
            }
            self.host_values_valid = true;
        } else {
            self.host_values_valid = false;
        }

        Ok(StencilTickResult {
            report,
            source_setup_dispatches,
            propagation_dispatches,
        })
    }

    fn field_values_for_reduction(&self, ctx: &GpuContext) -> Vec<f32> {
        if self.gpu_state_canonical {
            self.stencil.readback_input_buffer(ctx)
        } else if self.host_values_valid {
            self.values.clone()
        } else {
            self.stencil.readback_input_buffer(ctx)
        }
    }

    fn run_reduction_and_eml(
        &mut self,
        ctx: &GpuContext,
        readback_report: bool,
        weight_a: f32,
        weight_b: f32,
    ) -> Result<(Option<f32>, Option<f32>), FirstSliceMappingError> {
        let reduction = self.preview.reduction.as_ref().expect("validated at open");
        let reduction_op = column_aware_reduction_op(reduction.clone())
            .map_err(|e| FirstSliceMappingError::Accumulator(format!("{e}")))?;

        let parent_slot = reduction.parent_slot;
        let parent_col = reduction.parent_col;

        let field = self.field_values_for_reduction(ctx);
        let mut pv = field;
        let cell_count = self.preview.cell_count;
        let needed = ((parent_slot + 1) * self.n_dims) as usize;
        if pv.len() < needed {
            pv.resize(needed, 0.0);
        }
        for s in 0..cell_count {
            pv[self.slot_idx(s, self.eml_resource_col)] = 1.0;
        }
        pv[self.slot_idx(parent_slot, self.eml_weight_a_col)] = weight_a;
        pv[self.slot_idx(parent_slot, self.eml_weight_b_col)] = weight_b;

        let sum_resource_op = AccumulatorOp {
            source: SourceSpec::SlotRange {
                start: 0,
                count: cell_count,
                col: self.eml_resource_col,
            },
            combine: CombineFn::Sum,
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(parent_slot, self.eml_resource_col)],
        };

        let eml_op = AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: parent_slot,
                col: parent_col,
            },
            combine: CombineFn::EvalEML {
                tree_id: self.tree_id,
            },
            gate: GateSpec::OrderBand(1),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(parent_slot, self.eml_output_col)],
        };

        if readback_report {
            set_debug_readback_allowed(true);
        }
        self.acc_session.upload_values(ctx, &pv);
        self.acc_session
            .upload_ops_with_eml(
                ctx,
                &[reduction_op, sum_resource_op, eml_op],
                Some(&self.eml_registry),
            )
            .map_err(|e| FirstSliceMappingError::Accumulator(format!("{e}")))?;
        let eml = Some((&self.eml_table.node_buffer, &self.eml_table.range_buffer));
        self.acc_session
            .tick_with_eml(ctx, 0, eml)
            .map_err(|e| FirstSliceMappingError::Accumulator(format!("{e}")))?;
        self.acc_session
            .tick_with_eml(ctx, 1, eml)
            .map_err(|e| FirstSliceMappingError::Accumulator(format!("{e}")))?;

        if !readback_report {
            return Ok((None, None));
        }

        let out = self
            .acc_session
            .readback_full(ctx)
            .map_err(|e| FirstSliceMappingError::Accumulator(format!("{e}")))?;
        let threat = out[self.slot_idx(parent_slot, parent_col)];
        let urgency = out[self.slot_idx(parent_slot, self.eml_output_col)];
        Ok((Some(threat), Some(urgency)))
    }

    /// Execute one mapping tick. No-op when profile Disabled.
    pub fn tick(
        &mut self,
        ctx: &GpuContext,
        options: FirstSliceTickOptions,
        eml_weights: (f32, f32),
    ) -> Result<FirstSliceMappingReport, FirstSliceMappingError> {
        let tick = self.tick;
        if !self.enabled {
            self.tick += 1;
            return Ok(FirstSliceMappingReport::disabled(tick));
        }

        self.apply_pending_seeds_to_host();

        let (decisions, scheduler_report) = self.scheduler.decide_tick(tick)?;
        let scheduled = decisions.iter().any(|d| {
            d.field_id == self.field_id
                && d.region_id == self.region_id
                && matches!(d.schedule, FieldDispatchSchedule::Dispatch)
        });

        let mut stencil_execution = None;
        let mut field_values = None;
        let mut source_setup_dispatches = 0u32;
        let mut propagation_dispatches = 0u32;

        if scheduled {
            let stencil_result = self.run_caller_managed_stencil(ctx, options)?;
            source_setup_dispatches = stencil_result.source_setup_dispatches;
            propagation_dispatches = stencil_result.propagation_dispatches;
            stencil_execution = Some(ScheduledSingleStencilExecution {
                field_id: self.field_id,
                region_id: self.region_id,
                report: stencil_result.report.clone(),
            });
            field_values = stencil_result.report.values.clone();
        }

        let (reduction_parent_value, eml_output, reduction_executed, eml_executed) =
            if scheduled {
                let (threat, urgency) = self.run_reduction_and_eml(
                    ctx,
                    options.readback_values,
                    eml_weights.0,
                    eml_weights.1,
                )?;
                (threat, urgency, true, true)
            } else {
                (None, None, false, false)
            };

        if scheduled {
            if let Some(region) = self
                .scheduler
                .regions_mut()
                .iter_mut()
                .find(|r| r.field_id == self.field_id && r.region_id == self.region_id)
            {
                region.dirty.dirty_source_present = false;
                region.dirty.last_topology_generation = region.dirty.topology_generation;
                region.dirty.last_operator_generation = region.dirty.operator_generation;
            }
        }

        self.tick += 1;

        Ok(FirstSliceMappingReport {
            enabled: true,
            tick,
            scheduled,
            scheduler_report: Some(scheduler_report),
            stencil_execution,
            reduction_parent_value,
            eml_output,
            field_values,
            source_setup_dispatches,
            propagation_dispatches,
            total_dispatches: source_setup_dispatches + propagation_dispatches,
            reduction_executed,
            eml_executed,
        })
    }
}

/// Estimate first-slice budget from a RegionFieldSpec (designer preview).
pub fn estimate_first_slice_budget(
    spec: &RegionFieldSpec,
    isolation: RegionFieldIsolationPolicyEstimate,
) -> Result<RegionFieldBudgetEstimate, RegionFieldBudgetError> {
    let preview = compile_region_field_preview(spec).map_err(|_| RegionFieldBudgetError {
        requested_bytes: u64::MAX,
        allowed_bytes: spec.max_region_field_vram_bytes.unwrap_or(0),
        estimate: RegionFieldBudgetEstimate {
            useful_cells: 0,
            bytes_per_cell: 0,
            base_bytes: 0,
            isolation_multiplier: 1.0,
            buffer_multiplier: 2.0,
            copy_multiplier: 1.0,
            estimated_bytes: u64::MAX,
        },
        grid_size: spec.grid_size,
        column_count: spec.n_dims,
        isolation_policy: isolation,
    })?;
    let budget_spec = RegionFieldBudgetSpec {
        grid_size: preview.grid_size,
        column_count: preview.stencil.n_dims,
        buffer_multiplier: 2.0,
        copy_multiplier: 1.0,
        tile_count: 1,
        isolation_policy: isolation,
        max_region_field_vram_bytes: spec.max_region_field_vram_bytes,
    };
    estimate_region_field_budget(&budget_spec)
}

fn build_field_urgency_eml(
    ctx: &GpuContext,
    tree_id: u32,
) -> Result<(EmlExpressionRegistry, EmlGpuProgramTable), FirstSliceMappingError> {
    let nodes = vec![
        EmlNodeGpu {
            opcode: eml_opcode::SLOT_VALUE,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::SLOT_VALUE,
            flags: 0,
            a: 2,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::MUL,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::SLOT_VALUE,
            flags: 0,
            a: 1,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::SLOT_VALUE,
            flags: 0,
            a: 3,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::MUL,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::ADD,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::RETURN_TOP,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
    ];
    let mut registry = EmlExpressionRegistry::new();
    registry
        .register_formula(
            EmlTreeId(tree_id),
            EmlFormulaMeta {
                tree_id: EmlTreeId(tree_id),
                execution_class: EmlExecutionClass::ExactDeterministic,
                allowed_consumers: EmlConsumerMask(
                    EmlConsumerMask::ALL_PRODUCTION | EmlConsumerMask::DEBUG_ORACLE,
                ),
                max_abs_error: None,
                deterministic_gpu: true,
                requires_guard_for_hard_threshold: false,
                node_count: nodes.len() as u32,
                max_stack_depth: 0,
                has_loops: false,
                has_recursion: false,
                display_name: "field_urgency".into(),
            },
            nodes,
        )
        .map_err(|e| FirstSliceMappingError::EmlSetup(format!("{e}")))?;
    let mut table = EmlGpuProgramTable::new(ctx, 128, 16);
    let trees: Vec<_> = registry
        .formulas_for_gpu_upload()
        .map(|(t, m, n)| (t, m.clone(), n.to_vec()))
        .collect();
    for (t, ri) in table
        .upload_trees(ctx, &trees)
        .map_err(|e| FirstSliceMappingError::EmlSetup(format!("{e}")))?
    {
        registry
            .mark_tree_uploaded(t, ri, table.generation)
            .map_err(|e| FirstSliceMappingError::EmlSetup(format!("{e}")))?;
    }
    Ok((registry, table))
}
