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
    accumulator_op::set_debug_readback_allowed, AccumulatorOpSession, EmlGpuProgramTable,
    GpuContext, StructuredFieldExecutionOptions, StructuredFieldExecutionReport,
    StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOp, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy, ThresholdEvent, ThresholdRegistration, DIR_UPWARD,
    THRESH_BUF_VALUES,
};
use simthing_spec::{
    compile_region_field_preview, estimate_region_field_budget, CompiledFieldCadence,
    CompiledFirstSliceCommitmentThreshold, CompiledFirstSliceScenarioPreview,
    CompiledRegionFieldOperator, CompiledRegionFieldPreview, CompiledRegionFieldStencilSpec,
    CompiledRegionFieldSummaryPolicy, MappingExecutionProfile, RegionFieldBudgetError,
    RegionFieldBudgetEstimate, RegionFieldBudgetSpec, RegionFieldIsolationPolicyEstimate,
    RegionFieldSpec, SpecError,
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
        weight_north: compiled.weight_north,
        weight_south: compiled.weight_south,
        weight_east: compiled.weight_east,
        weight_west: compiled.weight_west,
        source_cap: compiled.source_cap,
        operator: match compiled.operator {
            CompiledRegionFieldOperator::Normalized => StructuredFieldStencilOperator::Normalized,
            CompiledRegionFieldOperator::SourceCappedNormalized => {
                StructuredFieldStencilOperator::SourceCappedNormalized
            }
            CompiledRegionFieldOperator::Gradient { axis } => match axis {
                simthing_spec::CompiledGradientAxis::X => StructuredFieldStencilOperator::GradientX,
                simthing_spec::CompiledGradientAxis::Y => StructuredFieldStencilOperator::GradientY,
            },
            CompiledRegionFieldOperator::SaturatingFlux { u_sat, chi } => {
                StructuredFieldStencilOperator::SaturatingFlux { u_sat, chi }
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

/// Runtime summary validity status for one first-slice tick (metadata only — not gameplay recomputation on CPU).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FirstSliceSummaryStatus {
    FreshThisTick,
    Cached { age_ticks: u32 },
    ZeroInitial,
    InvalidOrUnavailable,
}

/// Summary validity report for one first-slice tick (metadata only).
///
/// - `policy` = admitted/configured summary policy
/// - `status` = actual validity state for this tick
/// - `status` is authoritative
/// - `policy` alone does not imply a valid summary
///
/// Cached ticks emit no threshold event. `summary_used_for_commitment_scan` stays false.
/// GPU-substrate cached commitment scan is deferred in V1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FirstSliceSummaryReport {
    pub policy: CompiledRegionFieldSummaryPolicy,
    pub status: FirstSliceSummaryStatus,
    pub age_ticks: u32,
    pub has_gpu_parent_summary: bool,
    pub last_fresh_tick: Option<u32>,
    pub summary_used_for_commitment_scan: bool,
}

/// First-slice residency status for one tick (metadata only — not gameplay recomputation on CPU).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FirstSliceResidencyStatus {
    HotExecutedThisTick,
    ResidentCached,
    ColdSkipped,
    DisabledUnavailable,
}

/// Residency report for one first-slice tick (metadata only).
///
/// Residency status describes whether the opted-in field executed densely this tick, retained a
/// prior GPU parent summary while skipped, was cold before first execution, or was unavailable
/// under a Disabled profile. It does not rederive threat/urgency on CPU.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FirstSliceResidencyReport {
    pub status: FirstSliceResidencyStatus,
    pub summary_visible_to_parent: bool,
    pub dense_field_executed: bool,
    pub parent_summary_retained_on_gpu: bool,
    pub cached_commitment_scan_supported: bool,
}

/// Opus/product readiness summary for one first-slice tick (generic observability).
#[derive(Clone, Debug, PartialEq)]
pub struct FirstSliceReadinessReport {
    pub mapping_enabled: bool,
    pub scheduled: bool,
    pub source_setup_dispatches: u32,
    pub propagation_dispatches: u32,
    pub total_dispatches: u32,
    pub reduction_executed: bool,
    pub eml_executed: bool,
    pub reduction_stencil_readbacks: u32,
    pub field_values_present: bool,
    pub parent_reduction_present: bool,
    pub eml_output_present: bool,
    pub grid_size: u32,
    pub cell_count: u32,
    pub n_dims: u32,
    pub horizon: u32,
    pub operator: &'static str,
    pub source_policy: &'static str,
    pub boundary_mode: &'static str,
    pub cadence: String,
    pub budget_estimate_bytes: Option<u64>,
    pub budget_limit_bytes: Option<u64>,
    pub gpu_bridge_bytes_copied: u64,
    /// Per-slot scalar queue writes (parent personality/weight columns only on V1 hot path).
    pub gpu_bridge_slot_col_writes: u32,
    /// Bulk column fill operations (one per child resource column fill).
    pub gpu_bridge_bulk_col_fills: u32,
    /// Slot values written via bulk column fill helpers.
    pub gpu_bridge_bulk_fill_values: u32,
    /// Constant-size parent scalar queue writes (personality/weight columns).
    pub gpu_bridge_parent_scalar_writes: u32,
    /// Informational only; not a CI stability gate.
    pub hot_path_wall_ms_observed: Option<f64>,
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
    /// Stencil field readbacks performed while preparing reduction/EML (0 on GPU-resident hot path).
    pub reduction_stencil_readbacks: u32,
    pub readiness: FirstSliceReadinessReport,
    pub summary: FirstSliceSummaryReport,
    pub residency: FirstSliceResidencyReport,
}

/// Narrow fixture report for Phase M commitment tests.
///
/// This is intentionally opt-in and remains outside the default SimSession pass graph.
#[derive(Clone, Debug, PartialEq)]
pub struct FirstSliceCommitmentReport {
    pub mapping: FirstSliceMappingReport,
    pub threshold: f32,
    pub event_kind: u32,
    pub threshold_events: Vec<ThresholdEvent>,
}

impl FirstSliceMappingReport {
    fn summary_invalid_or_unavailable() -> FirstSliceSummaryReport {
        FirstSliceSummaryReport {
            policy: CompiledRegionFieldSummaryPolicy::CachedUntilDirtyWithZeroInitial,
            status: FirstSliceSummaryStatus::InvalidOrUnavailable,
            age_ticks: 0,
            has_gpu_parent_summary: false,
            last_fresh_tick: None,
            summary_used_for_commitment_scan: false,
        }
    }

    fn residency_disabled_unavailable() -> FirstSliceResidencyReport {
        FirstSliceResidencyReport {
            status: FirstSliceResidencyStatus::DisabledUnavailable,
            summary_visible_to_parent: false,
            dense_field_executed: false,
            parent_summary_retained_on_gpu: false,
            cached_commitment_scan_supported: false,
        }
    }

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
            reduction_stencil_readbacks: 0,
            readiness: FirstSliceReadinessReport {
                mapping_enabled: false,
                scheduled: false,
                source_setup_dispatches: 0,
                propagation_dispatches: 0,
                total_dispatches: 0,
                reduction_executed: false,
                eml_executed: false,
                reduction_stencil_readbacks: 0,
                field_values_present: false,
                parent_reduction_present: false,
                eml_output_present: false,
                grid_size: 0,
                cell_count: 0,
                n_dims: 0,
                horizon: 0,
                operator: "none",
                source_policy: "none",
                boundary_mode: "none",
                cadence: "none".into(),
                budget_estimate_bytes: None,
                budget_limit_bytes: None,
                gpu_bridge_bytes_copied: 0,
                gpu_bridge_slot_col_writes: 0,
                gpu_bridge_bulk_col_fills: 0,
                gpu_bridge_bulk_fill_values: 0,
                gpu_bridge_parent_scalar_writes: 0,
                hot_path_wall_ms_observed: None,
            },
            summary: Self::summary_invalid_or_unavailable(),
            residency: Self::residency_disabled_unavailable(),
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
    #[error("scenario preview missing commitment binding")]
    MissingCommitmentBinding,
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
    /// Cells whose seed values were written GPU-side (indexed scatter) —
    /// no host mirror; the seed sequence runs without host writes.
    pending_gpu_seed_cells: Vec<(u32, u32)>,
    /// Line 3R: whether the commitment scan's GPU-resident previous-values
    /// baseline has been initialized (zeroed once at first scan, then
    /// snapshotted on-device after every scan — never reset per tick).
    commitment_scan_initialized: bool,
    eml_weight_a_col: u32,
    eml_weight_b_col: u32,
    eml_output_col: u32,
    eml_resource_col: u32,
    seeds_applied_this_tick: bool,
    gpu_state_canonical: bool,
    host_values_valid: bool,
    reduction_stencil_readbacks_this_tick: u32,
    gpu_bridge_bytes_copied_this_tick: u64,
    gpu_bridge_slot_col_writes_this_tick: u32,
    gpu_bridge_bulk_col_fills_this_tick: u32,
    gpu_bridge_bulk_fill_values_this_tick: u32,
    gpu_bridge_parent_scalar_writes_this_tick: u32,
    budget_estimate_bytes: Option<u64>,
    budget_limit_bytes: Option<u64>,
    hot_path_wall_ms_observed: Option<f64>,
    summary_policy: CompiledRegionFieldSummaryPolicy,
    summary_age_ticks: u32,
    has_gpu_parent_summary: bool,
    last_fresh_tick: Option<u32>,
}

struct GpuBridgeShape {
    bytes_copied: u64,
    bulk_col_fills: u32,
    bulk_fill_values: u32,
    parent_scalar_writes: u32,
    slot_col_writes: u32,
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
        let budget = estimate_first_slice_budget(
            spec,
            RegionFieldIsolationPolicyEstimate::SingleGridNoAtlas,
        )
        .ok();
        Self::open_preview_with_budget(
            ctx,
            profile,
            preview,
            spec.parent_formula.as_ref().and_then(|f| f.tree_id),
            budget.map(|b| b.estimated_bytes),
            spec.max_region_field_vram_bytes,
        )
    }

    /// Open from an already-admitted compile preview.
    pub fn open_preview(
        ctx: &GpuContext,
        profile: MappingExecutionProfile,
        preview: CompiledRegionFieldPreview,
        tree_id_override: Option<u32>,
    ) -> Result<Self, FirstSliceMappingError> {
        Self::open_preview_with_budget(ctx, profile, preview, tree_id_override, None, None)
    }

    /// Open from an admitted first-slice scenario compile preview.
    pub fn open_from_scenario_preview(
        ctx: &GpuContext,
        preview: &CompiledFirstSliceScenarioPreview,
    ) -> Result<Self, FirstSliceMappingError> {
        Self::open_preview_with_budget(
            ctx,
            preview.mapping_execution_profile,
            preview.region_field.clone(),
            preview.parent_formula_tree_id,
            preview.budget_estimate_bytes,
            preview.budget_limit_bytes,
        )
    }

    fn open_preview_with_budget(
        ctx: &GpuContext,
        profile: MappingExecutionProfile,
        preview: CompiledRegionFieldPreview,
        tree_id_override: Option<u32>,
        budget_estimate_bytes: Option<u64>,
        budget_limit_bytes: Option<u64>,
    ) -> Result<Self, FirstSliceMappingError> {
        let enabled = profile.enables_execution();
        let gpu_config = compiled_stencil_to_gpu_config(&preview.stencil);
        gpu_config
            .validate()
            .map_err(FirstSliceMappingError::Stencil)?;

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

        let summary_policy = preview.summary_policy;
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
            pending_gpu_seed_cells: Vec::new(),
            commitment_scan_initialized: false,
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
            reduction_stencil_readbacks_this_tick: 0,
            gpu_bridge_bytes_copied_this_tick: 0,
            gpu_bridge_slot_col_writes_this_tick: 0,
            gpu_bridge_bulk_col_fills_this_tick: 0,
            gpu_bridge_bulk_fill_values_this_tick: 0,
            gpu_bridge_parent_scalar_writes_this_tick: 0,
            budget_estimate_bytes,
            budget_limit_bytes,
            hot_path_wall_ms_observed: None,
            summary_policy,
            summary_age_ticks: 0,
            has_gpu_parent_summary: false,
            last_fresh_tick: None,
        })
    }

    fn update_summary_state(&mut self, tick: u32, scheduled: bool, reduction_executed: bool) {
        if scheduled && reduction_executed {
            self.has_gpu_parent_summary = true;
            self.summary_age_ticks = 0;
            self.last_fresh_tick = Some(tick);
        } else if self.has_gpu_parent_summary {
            self.summary_age_ticks = self.summary_age_ticks.saturating_add(1);
        }
    }

    fn build_summary_report(
        &self,
        scheduled: bool,
        reduction_executed: bool,
        summary_used_for_commitment_scan: bool,
    ) -> FirstSliceSummaryReport {
        let status = if !self.enabled {
            FirstSliceSummaryStatus::InvalidOrUnavailable
        } else if scheduled && reduction_executed {
            FirstSliceSummaryStatus::FreshThisTick
        } else if self.has_gpu_parent_summary {
            FirstSliceSummaryStatus::Cached {
                age_ticks: self.summary_age_ticks,
            }
        } else {
            FirstSliceSummaryStatus::ZeroInitial
        };

        FirstSliceSummaryReport {
            policy: self.summary_policy,
            status,
            age_ticks: self.summary_age_ticks,
            has_gpu_parent_summary: self.has_gpu_parent_summary,
            last_fresh_tick: self.last_fresh_tick,
            summary_used_for_commitment_scan,
        }
    }

    fn build_residency_report(
        &self,
        scheduled: bool,
        reduction_executed: bool,
    ) -> FirstSliceResidencyReport {
        let dense_field_executed = scheduled && reduction_executed;
        let parent_summary_retained_on_gpu = self.has_gpu_parent_summary;
        let summary_visible_to_parent = self.has_gpu_parent_summary;
        let status = if !self.enabled {
            FirstSliceResidencyStatus::DisabledUnavailable
        } else if dense_field_executed {
            FirstSliceResidencyStatus::HotExecutedThisTick
        } else if self.has_gpu_parent_summary {
            FirstSliceResidencyStatus::ResidentCached
        } else {
            FirstSliceResidencyStatus::ColdSkipped
        };

        FirstSliceResidencyReport {
            status,
            summary_visible_to_parent,
            dense_field_executed,
            parent_summary_retained_on_gpu,
            cached_commitment_scan_supported: false,
        }
    }

    fn cadence_label(cadence: CompiledFieldCadence) -> String {
        match cadence {
            CompiledFieldCadence::EveryTick => "EveryTick".into(),
            CompiledFieldCadence::EveryN { n } => format!("EveryN({n})"),
            CompiledFieldCadence::OnEvent => "OnEvent".into(),
        }
    }

    fn operator_label(operator: CompiledRegionFieldOperator) -> &'static str {
        match operator {
            CompiledRegionFieldOperator::Normalized => "normalized",
            CompiledRegionFieldOperator::SourceCappedNormalized => "source_capped_normalized",
            CompiledRegionFieldOperator::Gradient { axis } => match axis {
                simthing_spec::CompiledGradientAxis::X => "gradient_x",
                simthing_spec::CompiledGradientAxis::Y => "gradient_y",
            },
            CompiledRegionFieldOperator::SaturatingFlux { .. } => "saturating_flux",
        }
    }

    fn build_readiness_report(
        &self,
        scheduled: bool,
        source_setup_dispatches: u32,
        propagation_dispatches: u32,
        reduction_executed: bool,
        eml_executed: bool,
        field_values: &Option<Vec<f32>>,
        reduction_parent_value: &Option<f32>,
        eml_output: &Option<f32>,
    ) -> FirstSliceReadinessReport {
        FirstSliceReadinessReport {
            mapping_enabled: self.enabled,
            scheduled,
            source_setup_dispatches,
            propagation_dispatches,
            total_dispatches: source_setup_dispatches + propagation_dispatches,
            reduction_executed,
            eml_executed,
            reduction_stencil_readbacks: self.reduction_stencil_readbacks_this_tick,
            field_values_present: field_values.is_some(),
            parent_reduction_present: reduction_parent_value.is_some(),
            eml_output_present: eml_output.is_some(),
            grid_size: self.preview.grid_size,
            cell_count: self.preview.cell_count,
            n_dims: self.n_dims,
            horizon: self.preview.stencil.horizon,
            operator: Self::operator_label(self.preview.stencil.operator),
            source_policy: "caller_managed_one_shot_seed_then_zero",
            boundary_mode: "zero",
            cadence: Self::cadence_label(self.preview.cadence),
            budget_estimate_bytes: self.budget_estimate_bytes,
            budget_limit_bytes: self.budget_limit_bytes,
            gpu_bridge_bytes_copied: self.gpu_bridge_bytes_copied_this_tick,
            gpu_bridge_slot_col_writes: self.gpu_bridge_slot_col_writes_this_tick,
            gpu_bridge_bulk_col_fills: self.gpu_bridge_bulk_col_fills_this_tick,
            gpu_bridge_bulk_fill_values: self.gpu_bridge_bulk_fill_values_this_tick,
            gpu_bridge_parent_scalar_writes: self.gpu_bridge_parent_scalar_writes_this_tick,
            hot_path_wall_ms_observed: self.hot_path_wall_ms_observed,
        }
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

    /// CT-3b+4a GPU projection: the seed values are already in the stencil
    /// input buffer (written on-device by the indexed scatter); register the
    /// cells so the one-shot seed-then-zero sequence runs without any host
    /// value write. Host values become stale until the next readback.
    pub fn queue_gpu_seed_cells(
        &mut self,
        cells: &[(u32, u32)],
    ) -> Result<(), FirstSliceMappingError> {
        let height = self.preview.stencil.height;
        for (row, col) in cells {
            if *row >= height || *col >= self.width {
                return Err(FirstSliceMappingError::InvalidSeed {
                    row: *row,
                    col: *col,
                    width: self.width,
                    height,
                });
            }
        }
        self.pending_gpu_seed_cells.extend_from_slice(cells);
        self.seeds_applied_this_tick = true;
        self.host_values_valid = false;
        self.mark_dirty_source();
        Ok(())
    }

    /// Stencil input buffer — the destination of on-device pressure scatter.
    pub fn stencil_input_buffer(&self) -> &simthing_gpu::wgpu::Buffer {
        &self.stencil.input_buffer
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
        self.seeds_applied_this_tick =
            !self.pending_seeds.is_empty() || !self.pending_gpu_seed_cells.is_empty();
    }

    fn run_caller_managed_stencil(
        &mut self,
        ctx: &GpuContext,
        options: FirstSliceTickOptions,
    ) -> Result<StencilTickResult, FirstSliceMappingError> {
        let horizon = self.stencil.config().horizon;
        let mut source_setup_dispatches = 0u32;

        if self.seeds_applied_this_tick {
            let (writes, mut zeros) = self.seed_slot_col_writes();
            // GPU-scattered seeds are already in the input buffer; they only
            // join the one-shot zero list.
            let source_col = self.source_col;
            let width = self.width;
            zeros.extend(
                self.pending_gpu_seed_cells
                    .iter()
                    .map(|(row, col)| (row * width + col, source_col)),
            );
            if !writes.is_empty() {
                self.stencil
                    .write_cell_values(ctx, &self.stencil.input_buffer, &writes)?;
            }
            source_setup_dispatches += self.stencil.dispatch_once(
                ctx,
                &self.stencil.input_buffer,
                &self.stencil.output_buffer,
            );
            self.stencil
                .zero_cell_values(ctx, &self.stencil.output_buffer, &zeros)?;
            self.stencil.copy_output_to_input(ctx);
            self.pending_seeds.clear();
            self.pending_gpu_seed_cells.clear();
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

    fn bridge_stencil_field_to_accumulator(
        &mut self,
        ctx: &GpuContext,
        weight_a: f32,
        weight_b: f32,
    ) -> Result<GpuBridgeShape, FirstSliceMappingError> {
        let reduction = self.preview.reduction.as_ref().expect("validated at open");
        let parent_slot = reduction.parent_slot;
        let cell_count = self.preview.cell_count;
        let prefix_bytes = (cell_count * self.n_dims) as u64 * std::mem::size_of::<f32>() as u64;

        self.acc_session.zero_values_buffer(ctx);
        self.acc_session
            .copy_values_prefix_from_buffer(ctx, &self.stencil.input_buffer, 0, 0, prefix_bytes)
            .map_err(|e| FirstSliceMappingError::Accumulator(format!("{e}")))?;

        let parent_scalar_writes = 2u32;
        self.acc_session
            .fill_slot_range_col(ctx, 0, cell_count, self.eml_resource_col, 1.0)
            .map_err(|e| FirstSliceMappingError::Accumulator(format!("{e}")))?;
        self.acc_session
            .write_slot_col_values(
                ctx,
                &[
                    (parent_slot, self.eml_weight_a_col, weight_a),
                    (parent_slot, self.eml_weight_b_col, weight_b),
                ],
            )
            .map_err(|e| FirstSliceMappingError::Accumulator(format!("{e}")))?;

        let shape = GpuBridgeShape {
            bytes_copied: prefix_bytes,
            bulk_col_fills: 1,
            bulk_fill_values: cell_count,
            parent_scalar_writes,
            slot_col_writes: parent_scalar_writes,
        };
        self.gpu_bridge_bytes_copied_this_tick = shape.bytes_copied;
        self.gpu_bridge_slot_col_writes_this_tick = shape.slot_col_writes;
        self.gpu_bridge_bulk_col_fills_this_tick = shape.bulk_col_fills;
        self.gpu_bridge_bulk_fill_values_this_tick = shape.bulk_fill_values;
        self.gpu_bridge_parent_scalar_writes_this_tick = shape.parent_scalar_writes;
        Ok(shape)
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
        let cell_count = self.preview.cell_count;

        self.reduction_stencil_readbacks_this_tick = 0;
        let _bridge = self.bridge_stencil_field_to_accumulator(ctx, weight_a, weight_b)?;

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

    /// Line 3R production commitment scan: edge-detected on GPU. The
    /// previous-values baseline persists on-device across scans (zeroed once
    /// at first scan — the first above-threshold value is a genuine rising
    /// edge), so a held above-threshold urgency emits **no** repeated upward
    /// crossings, while falling below and rising again emits a fresh one.
    /// CPU reads back compact threshold events only; the decision is GPU-side.
    fn scan_commitment_threshold_edge(
        &mut self,
        ctx: &GpuContext,
        threshold: f32,
        event_kind: u32,
    ) -> Result<Vec<ThresholdEvent>, FirstSliceMappingError> {
        let reduction = self.preview.reduction.as_ref().expect("validated at open");
        let parent_slot = reduction.parent_slot;
        if !self.commitment_scan_initialized {
            let previous = vec![0.0f32; self.acc_session.values_len()];
            self.acc_session.upload_previous_values(ctx, &previous);
            self.commitment_scan_initialized = true;
        }
        self.acc_session.ensure_threshold_emission_capacity(ctx, 1);
        self.acc_session
            .upload_threshold_ops(
                ctx,
                &[ThresholdRegistration {
                    slot: parent_slot,
                    col: self.eml_output_col,
                    threshold,
                    direction: DIR_UPWARD,
                    event_kind,
                    buffer: THRESH_BUF_VALUES,
                }],
            )
            .map_err(|e| FirstSliceMappingError::Accumulator(format!("{e}")))?;
        self.acc_session
            .tick(ctx, 0)
            .map_err(|e| FirstSliceMappingError::Accumulator(format!("{e}")))?;
        let events = self
            .acc_session
            .readback_threshold_events(ctx)
            .map_err(|e| FirstSliceMappingError::Accumulator(format!("{e}")))?;
        self.acc_session.copy_values_to_previous(ctx);
        Ok(events)
    }

    /// Line 3R production tick: the mapping chain plus the edge-detected
    /// commitment scan. Production session code calls this — never the
    /// `*_fixture` variants (which reset the baseline every scan and are
    /// retained only for standing single-tick fixtures).
    pub fn tick_with_commitment_spec(
        &mut self,
        ctx: &GpuContext,
        options: FirstSliceTickOptions,
        eml_weights: (f32, f32),
        commitment: &CompiledFirstSliceCommitmentThreshold,
    ) -> Result<FirstSliceCommitmentReport, FirstSliceMappingError> {
        let mapping = self.tick(ctx, options, eml_weights)?;
        let mut threshold_events = Vec::new();
        let mut summary_used_for_commitment_scan = false;
        if mapping.enabled && mapping.scheduled && mapping.eml_executed {
            threshold_events = self.scan_commitment_threshold_edge(
                ctx,
                commitment.threshold,
                commitment.event_kind,
            )?;
            summary_used_for_commitment_scan = true;
        }
        let mut mapping = mapping;
        mapping.summary.summary_used_for_commitment_scan = summary_used_for_commitment_scan;
        Ok(FirstSliceCommitmentReport {
            mapping,
            threshold: commitment.threshold,
            event_kind: commitment.event_kind,
            threshold_events,
        })
    }

    fn scan_commitment_threshold(
        &mut self,
        ctx: &GpuContext,
        threshold: f32,
        event_kind: u32,
    ) -> Result<Vec<ThresholdEvent>, FirstSliceMappingError> {
        let reduction = self.preview.reduction.as_ref().expect("validated at open");
        let parent_slot = reduction.parent_slot;
        let previous = vec![0.0f32; self.acc_session.values_len()];
        self.acc_session.upload_previous_values(ctx, &previous);
        self.acc_session.ensure_threshold_emission_capacity(ctx, 1);
        self.acc_session
            .upload_threshold_ops(
                ctx,
                &[ThresholdRegistration {
                    slot: parent_slot,
                    col: self.eml_output_col,
                    threshold,
                    direction: DIR_UPWARD,
                    event_kind,
                    buffer: THRESH_BUF_VALUES,
                }],
            )
            .map_err(|e| FirstSliceMappingError::Accumulator(format!("{e}")))?;
        self.acc_session
            .tick(ctx, 0)
            .map_err(|e| FirstSliceMappingError::Accumulator(format!("{e}")))?;
        self.acc_session
            .readback_threshold_events(ctx)
            .map_err(|e| FirstSliceMappingError::Accumulator(format!("{e}")))
    }

    /// Execute the first-slice fixture and scan one GPU threshold over parent `field_urgency`.
    ///
    /// The threshold decision is produced by the existing AccumulatorOp Threshold + EmitEvent
    /// substrate. CPU readback is only used to inspect the emitted event after the GPU scan.
    pub fn tick_with_commitment_threshold_fixture(
        &mut self,
        ctx: &GpuContext,
        options: FirstSliceTickOptions,
        eml_weights: (f32, f32),
        threshold: f32,
        event_kind: u32,
    ) -> Result<FirstSliceCommitmentReport, FirstSliceMappingError> {
        let mapping = self.tick(ctx, options, eml_weights)?;
        let mut threshold_events = Vec::new();
        let mut summary_used_for_commitment_scan = false;
        if mapping.enabled && mapping.scheduled && mapping.eml_executed {
            threshold_events = self.scan_commitment_threshold(ctx, threshold, event_kind)?;
            summary_used_for_commitment_scan = true;
        }
        let mut mapping = mapping;
        mapping.summary.summary_used_for_commitment_scan = summary_used_for_commitment_scan;
        Ok(FirstSliceCommitmentReport {
            mapping,
            threshold,
            event_kind,
            threshold_events,
        })
    }

    pub fn tick_with_commitment_spec_fixture(
        &mut self,
        ctx: &GpuContext,
        options: FirstSliceTickOptions,
        eml_weights: (f32, f32),
        commitment: &CompiledFirstSliceCommitmentThreshold,
    ) -> Result<FirstSliceCommitmentReport, FirstSliceMappingError> {
        self.tick_with_commitment_threshold_fixture(
            ctx,
            options,
            eml_weights,
            commitment.threshold,
            commitment.event_kind,
        )
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
        self.reduction_stencil_readbacks_this_tick = 0;
        self.gpu_bridge_bytes_copied_this_tick = 0;
        self.gpu_bridge_slot_col_writes_this_tick = 0;
        self.gpu_bridge_bulk_col_fills_this_tick = 0;
        self.gpu_bridge_bulk_fill_values_this_tick = 0;
        self.gpu_bridge_parent_scalar_writes_this_tick = 0;
        self.hot_path_wall_ms_observed = None;

        let tick_started = (!options.readback_values).then(std::time::Instant::now);

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

        let (reduction_parent_value, eml_output, reduction_executed, eml_executed) = if scheduled {
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

        self.update_summary_state(tick, scheduled, reduction_executed);
        self.tick += 1;

        if let (Some(started), true) = (tick_started, scheduled && !options.readback_values) {
            self.hot_path_wall_ms_observed = Some(started.elapsed().as_secs_f64() * 1000.0);
        }

        let readiness = self.build_readiness_report(
            scheduled,
            source_setup_dispatches,
            propagation_dispatches,
            reduction_executed,
            eml_executed,
            &field_values,
            &reduction_parent_value,
            &eml_output,
        );
        let summary = self.build_summary_report(scheduled, reduction_executed, false);
        let residency = self.build_residency_report(scheduled, reduction_executed);

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
            reduction_stencil_readbacks: self.reduction_stencil_readbacks_this_tick,
            readiness,
            summary,
            residency,
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
