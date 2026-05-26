//! AccumulatorOp v2 world runtime envelope (C-INF-1).
//!
//! One logical runtime with named operation sets. C-1/C-2/C-3 still use
//! per-family GPU sessions (adapter shim matching pre-consolidation behavior)
//! until a single shared op buffer is validated for all families together.

use crate::{OverlayDelta, SlotDeltaRange, ThresholdRegistration};

use simthing_core::EmlExpressionRegistry;

use simthing_core::{AccumulatorOp, EmlTreeId};

use super::eml_program_table::{
    EmlGpuProgramTable, EmlUploadError, DEFAULT_EML_NODE_CAPACITY, DEFAULT_EML_TREE_CAPACITY,
};
use super::session::{AccumulatorOpSession, AccumulatorOpSessionError};
use super::types::AccumulatorOpGpu;
use super::types::DEFAULT_THRESHOLD_EMISSION_CAPACITY;
use super::world_summary::WorldSummaryRuntime;
use crate::world_state::IntentDelta;
use crate::GpuContext;

/// Cache key for uploaded C-8b intensity EvalEML ops (world shape + EML plan).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntensityEmlOpPlanSignature {
    pub eml_registry_generation: u64,
    pub n_slots: u32,
    pub n_dims: u32,
    pub n_entries: u32,
    pub n_ops: u32,
    pub tree_ids: Vec<u32>,
    pub intensity_cols: Vec<u32>,
    pub velocity_cols: Vec<u32>,
}

/// Cache key for uploaded C-8c transfer Eval ops (world shape + input-list plan).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransferOpPlanSignature {
    pub n_slots: u32,
    pub n_dims: u32,
    pub n_ops: u32,
    pub n_registrations: u32,
    pub input_list_generation: u64,
    pub input_slots: Vec<u32>,
    pub input_cols: Vec<u32>,
    pub unit_cost_bits: Vec<u32>,
}

/// Cache key for uploaded C-8d emission ops (world shape + EML + registration plan).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmissionOpPlanSignature {
    pub eml_registry_generation: u64,
    pub n_slots: u32,
    pub n_dims: u32,
    pub n_registrations: u32,
    pub n_ops: u32,
    pub source_slots: Vec<u32>,
    pub source_cols: Vec<u32>,
    pub tree_ids: Vec<u32>,
    pub formula_kinds: Vec<u32>,
    pub reg_indices: Vec<u32>,
    pub constant_value_bits: Vec<u32>,
    pub max_emit: Vec<u32>,
}

/// Operation family registered into the world AccumulatorOp runtime.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OperationFamily {
    Intent,
    Threshold,
    OverlayAdd,
    OverlayOrderBand,
    ReductionSoft,
    ReductionExact,
    Velocity,
    EvalEml,
    EconomicTransfer,
    EconomicEmission,
    EconomicConjunctiveProduction,
}

/// Exactness class for registration validation and oracle comparison policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExactnessClass {
    Exact,
    SoftAggregate,
    DebugOnly,
}

/// Legacy pass families invoked only as oracle/fallback during migration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegacyOracleFamily {
    IntentPass,
    ThresholdPass,
    OverlayPass,
    ReductionPass,
    VelocityPass,
    IntensityPass,
}

/// Slice into the shared op buffer for one migrated operation family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpSetHandle {
    pub family: OperationFamily,
    pub offset: u32,
    pub count: u32,
    pub active: bool,
    pub n_bands: u32,
    pub exactness: ExactnessClass,
}

impl OpSetHandle {
    pub const INACTIVE: Self = Self {
        family: OperationFamily::Intent,
        offset: 0,
        count: 0,
        active: false,
        n_bands: 0,
        exactness: ExactnessClass::Exact,
    };
}

#[derive(Clone, Debug, PartialEq)]
pub struct OverlayCompileCache {
    pub compiled_at_revision: u64,
    pub cached_deltas: Vec<OverlayDelta>,
    pub cached_ranges: Vec<SlotDeltaRange>,
    pub cached_n_bands: u32,
    pub cached_op_buffer_uploaded_n_ops: u32,
    pub compile_count: u64,
    pub upload_count: u64,
}

/// Single AccumulatorOp runtime envelope for `WorldGpuState`.
pub struct WorldAccumulatorRuntime {
    pub intent_ops: OpSetHandle,
    pub threshold_ops: OpSetHandle,
    pub overlay_add_ops: OpSetHandle,
    pub overlay_order_ops: OpSetHandle,
    pub reduction_soft_ops: OpSetHandle,
    pub reduction_exact_ops: OpSetHandle,
    pub velocity_ops: OpSetHandle,
    pub intensity_eml_ops: OpSetHandle,
    pub transfer_ops: OpSetHandle,
    pub emission_ops: OpSetHandle,

    intent_session: Option<AccumulatorOpSession>,
    threshold_session: Option<AccumulatorOpSession>,
    overlay_session: Option<AccumulatorOpSession>,
    reduction_soft_session: Option<AccumulatorOpSession>,
    velocity_session: Option<AccumulatorOpSession>,
    intensity_eml_session: Option<AccumulatorOpSession>,
    transfer_session: Option<AccumulatorOpSession>,
    emission_session: Option<AccumulatorOpSession>,
    pub summary: Option<WorldSummaryRuntime>,
    pub overlay_compile_cache: Option<OverlayCompileCache>,

    /// Intensity EML tree IDs registered at last boundary sync (C-8b).
    intensity_tree_ids: Vec<EmlTreeId>,
    /// Registry generation reflected in uploaded intensity EvalEML ops.
    intensity_ops_registry_generation: Option<u64>,
    /// Authoritative cache key for uploaded intensity EvalEML ops (C-8b remedial).
    intensity_op_plan_signature: Option<IntensityEmlOpPlanSignature>,
    intensity_op_upload_count: u64,

    transfer_op_plan_signature: Option<TransferOpPlanSignature>,
    transfer_op_upload_count: u64,
    emission_op_plan_signature: Option<EmissionOpPlanSignature>,
    emission_op_upload_count: u64,
    pub input_lists: Option<super::input_list_table::AccumulatorInputListTable>,

    /// C-8a persistent GPU EML program table (shared across sessions).
    pub eml: Option<EmlGpuProgramTable>,
    /// Authoritative EML formula registry; uploaded at boundary sync when enabled.
    pub eml_registry: EmlExpressionRegistry,
}

impl WorldAccumulatorRuntime {
    pub fn new() -> Self {
        Self {
            intent_ops: OpSetHandle::INACTIVE,
            threshold_ops: OpSetHandle {
                family: OperationFamily::Threshold,
                exactness: ExactnessClass::Exact,
                ..OpSetHandle::INACTIVE
            },
            overlay_add_ops: OpSetHandle {
                family: OperationFamily::OverlayAdd,
                exactness: ExactnessClass::Exact,
                ..OpSetHandle::INACTIVE
            },
            overlay_order_ops: OpSetHandle {
                family: OperationFamily::OverlayOrderBand,
                exactness: ExactnessClass::Exact,
                ..OpSetHandle::INACTIVE
            },
            reduction_soft_ops: OpSetHandle {
                family: OperationFamily::ReductionSoft,
                exactness: ExactnessClass::SoftAggregate,
                ..OpSetHandle::INACTIVE
            },
            reduction_exact_ops: OpSetHandle {
                family: OperationFamily::ReductionExact,
                exactness: ExactnessClass::Exact,
                ..OpSetHandle::INACTIVE
            },
            velocity_ops: OpSetHandle {
                family: OperationFamily::Velocity,
                exactness: ExactnessClass::Exact,
                ..OpSetHandle::INACTIVE
            },
            intensity_eml_ops: OpSetHandle {
                family: OperationFamily::EvalEml,
                exactness: ExactnessClass::Exact,
                ..OpSetHandle::INACTIVE
            },
            transfer_ops: OpSetHandle {
                family: OperationFamily::EconomicTransfer,
                exactness: ExactnessClass::Exact,
                ..OpSetHandle::INACTIVE
            },
            emission_ops: OpSetHandle {
                family: OperationFamily::EconomicEmission,
                exactness: ExactnessClass::Exact,
                ..OpSetHandle::INACTIVE
            },
            intent_session: None,
            threshold_session: None,
            overlay_session: None,
            reduction_soft_session: None,
            velocity_session: None,
            intensity_eml_session: None,
            transfer_session: None,
            emission_session: None,
            summary: None,
            overlay_compile_cache: None,
            intensity_tree_ids: Vec::new(),
            intensity_ops_registry_generation: None,
            intensity_op_plan_signature: None,
            intensity_op_upload_count: 0,
            transfer_op_plan_signature: None,
            transfer_op_upload_count: 0,
            emission_op_plan_signature: None,
            emission_op_upload_count: 0,
            input_lists: None,
            eml: None,
            eml_registry: EmlExpressionRegistry::new(),
        }
    }

    fn bind_eml_if_present(&self, _session: &mut AccumulatorOpSession) {
        // Sessions bind dummy EML buffers by default; production dispatches pass
        // `WorldAccumulatorRuntime::eml_bind_buffers()` into encode/tick helpers.
    }

    pub fn eml_bind_buffers(&self) -> Option<(&wgpu::Buffer, &wgpu::Buffer)> {
        self.eml
            .as_ref()
            .map(|t| (&t.node_buffer, &t.range_buffer))
    }

    pub fn apply_eml_bindings_to_sessions(&mut self) {
        let _ = self.eml_bind_buffers();
    }

    pub fn ensure_eml_program_table(&mut self, ctx: &GpuContext) {
        if self.eml.is_none() {
            self.eml = Some(EmlGpuProgramTable::new(
                ctx,
                DEFAULT_EML_NODE_CAPACITY,
                DEFAULT_EML_TREE_CAPACITY,
            ));
        }
    }

    pub fn clear_eml_program_table(&mut self) {
        self.eml = None;
    }

    pub fn eml_generation(&self) -> u64 {
        self.eml.as_ref().map(|t| t.generation).unwrap_or(0)
    }

    pub fn upload_eml_trees(&mut self, ctx: &GpuContext) -> Result<(), EmlUploadError> {
        self.ensure_eml_program_table(ctx);
        let registry_generation = self.eml_registry.generation();
        if self
            .eml
            .as_ref()
            .and_then(|t| t.uploaded_registry_generation)
            == Some(registry_generation)
        {
            return Ok(());
        }
        let trees: Vec<_> = self
            .eml_registry
            .formulas_for_gpu_upload()
            .map(|(id, meta, nodes)| (id, meta.clone(), nodes.to_vec()))
            .collect();
        let mut trees = trees;
        trees.sort_by_key(|(id, _, _)| id.0);
        let table = self.eml.as_mut().expect("eml table");
        let mapping = table.upload_trees(ctx, &trees)?;
        for (tree_id, range_index) in mapping {
            self.eml_registry
                .mark_tree_uploaded(tree_id, range_index, table.generation)
                .expect("mark_tree_uploaded after GPU upload");
        }
        table.uploaded_registry_generation = Some(registry_generation);
        self.apply_eml_bindings_to_sessions();
        Ok(())
    }

    pub fn take_intent_session(&mut self) -> Option<AccumulatorOpSession> {
        self.intent_session.take()
    }

    pub fn take_threshold_session(&mut self) -> Option<AccumulatorOpSession> {
        self.threshold_session.take()
    }

    pub fn take_overlay_session(&mut self) -> Option<AccumulatorOpSession> {
        self.overlay_session.take()
    }

    pub fn take_reduction_soft_session(&mut self) -> Option<AccumulatorOpSession> {
        self.reduction_soft_session.take()
    }

    pub fn take_velocity_session(&mut self) -> Option<AccumulatorOpSession> {
        self.velocity_session.take()
    }

    pub fn take_intensity_eml_session(&mut self) -> Option<AccumulatorOpSession> {
        self.intensity_eml_session.take()
    }

    pub fn restore_intent_session(&mut self, session: Option<AccumulatorOpSession>) {
        self.intent_session = session;
    }

    pub fn restore_threshold_session(&mut self, session: Option<AccumulatorOpSession>) {
        self.threshold_session = session;
    }

    pub fn restore_overlay_session(&mut self, session: Option<AccumulatorOpSession>) {
        self.overlay_session = session;
    }

    pub fn restore_reduction_soft_session(&mut self, session: Option<AccumulatorOpSession>) {
        self.reduction_soft_session = session;
    }

    pub fn restore_velocity_session(&mut self, session: Option<AccumulatorOpSession>) {
        self.velocity_session = session;
    }

    pub fn restore_intensity_eml_session(&mut self, session: Option<AccumulatorOpSession>) {
        self.intensity_eml_session = session;
    }

    pub fn intent_session(&mut self) -> Option<&mut AccumulatorOpSession> {
        self.intent_session.as_mut()
    }

    pub fn threshold_session(&mut self) -> Option<&mut AccumulatorOpSession> {
        self.threshold_session.as_mut()
    }

    pub fn overlay_session(&mut self) -> Option<&mut AccumulatorOpSession> {
        self.overlay_session.as_mut()
    }

    pub fn reduction_soft_session(&mut self) -> Option<&mut AccumulatorOpSession> {
        self.reduction_soft_session.as_mut()
    }

    pub fn velocity_session(&mut self) -> Option<&mut AccumulatorOpSession> {
        self.velocity_session.as_mut()
    }

    pub fn intensity_eml_session(&mut self) -> Option<&mut AccumulatorOpSession> {
        self.intensity_eml_session.as_mut()
    }

    pub fn intent_active(&self) -> bool {
        self.intent_session.is_some()
    }

    pub fn threshold_active(&self) -> bool {
        self.threshold_session.is_some()
    }

    pub fn overlay_add_active(&self) -> bool {
        self.overlay_order_ops.active
    }

    pub fn overlay_add_bands(&self) -> u32 {
        self.overlay_order_ops.n_bands
    }

    pub fn overlay_active(&self) -> bool {
        self.overlay_order_ops.active
    }

    pub fn overlay_n_bands(&self) -> u32 {
        self.overlay_order_ops.n_bands
    }

    pub fn reduction_soft_active(&self) -> bool {
        self.reduction_soft_ops.active
    }

    pub fn reduction_soft_bands(&self) -> u32 {
        self.reduction_soft_ops.n_bands
    }

    pub fn velocity_active(&self) -> bool {
        self.velocity_ops.active
    }

    pub fn velocity_bands(&self) -> u32 {
        self.velocity_ops.n_bands
    }

    pub fn intensity_eml_active(&self) -> bool {
        self.intensity_eml_ops.active
    }

    pub fn intensity_eml_bands(&self) -> u32 {
        self.intensity_eml_ops.n_bands
    }

    pub fn intensity_ops_registry_generation(&self) -> Option<u64> {
        self.intensity_ops_registry_generation
    }

    pub fn intensity_op_plan_signature(&self) -> Option<&IntensityEmlOpPlanSignature> {
        self.intensity_op_plan_signature.as_ref()
    }

    pub fn intensity_op_upload_count(&self) -> u64 {
        self.intensity_op_upload_count
    }

    pub fn any_pipeline_active(&self) -> bool {
        self.intent_active()
            || self.threshold_active()
            || self.overlay_order_ops.active
            || self.reduction_soft_ops.active
            || self.velocity_ops.active
            || self.intensity_eml_ops.active
            || self.transfer_ops.active
            || self.emission_ops.active
    }

    pub fn ensure_intent_session(
        &mut self,
        ctx: &GpuContext,
        n_slots: u32,
        n_dims: u32,
        emission_capacity: u32,
    ) {
        if self.intent_session.is_none() {
            let mut session = AccumulatorOpSession::new_attached(
                ctx,
                n_slots,
                n_dims,
                emission_capacity,
            );
            self.bind_eml_if_present(&mut session);
            self.intent_session = Some(session);
        }
        self.intent_ops = OpSetHandle {
            family: OperationFamily::Intent,
            exactness: ExactnessClass::Exact,
            active: true,
            ..OpSetHandle::INACTIVE
        };
    }

    pub fn ensure_threshold_session(
        &mut self,
        ctx: &GpuContext,
        n_slots: u32,
        n_dims: u32,
        emission_capacity: u32,
    ) {
        if self.threshold_session.is_none() {
            let mut session = AccumulatorOpSession::new_attached(
                ctx,
                n_slots,
                n_dims,
                emission_capacity,
            );
            self.bind_eml_if_present(&mut session);
            self.threshold_session = Some(session);
        }
        self.threshold_ops = OpSetHandle {
            family: OperationFamily::Threshold,
            exactness: ExactnessClass::Exact,
            active: true,
            ..OpSetHandle::INACTIVE
        };
    }

    pub fn ensure_overlay_session(
        &mut self,
        ctx: &GpuContext,
        n_slots: u32,
        n_dims: u32,
        emission_capacity: u32,
    ) {
        if self.overlay_session.is_none() {
            let mut session = AccumulatorOpSession::new_attached(
                ctx,
                n_slots,
                n_dims,
                emission_capacity,
            );
            self.bind_eml_if_present(&mut session);
            self.overlay_session = Some(session);
        }
    }

    pub fn disable_intent(&mut self) {
        self.intent_session = None;
        self.intent_ops = OpSetHandle {
            family: OperationFamily::Intent,
            exactness: ExactnessClass::Exact,
            ..OpSetHandle::INACTIVE
        };
    }

    pub fn clear_intent(&mut self) {
        self.disable_intent();
    }

    pub fn disable_threshold(&mut self) {
        self.threshold_session = None;
        self.threshold_ops = OpSetHandle {
            family: OperationFamily::Threshold,
            exactness: ExactnessClass::Exact,
            ..OpSetHandle::INACTIVE
        };
    }

    pub fn clear_threshold(&mut self) {
        self.disable_threshold();
    }

    pub fn clear_overlay_add(&mut self) {
        self.clear_overlay_orderband();
    }

    pub fn clear_overlay_orderband(&mut self) {
        self.overlay_session = None;
        self.overlay_add_ops = OpSetHandle {
            family: OperationFamily::OverlayAdd,
            exactness: ExactnessClass::Exact,
            ..OpSetHandle::INACTIVE
        };
        self.overlay_order_ops = OpSetHandle {
            family: OperationFamily::OverlayOrderBand,
            exactness: ExactnessClass::Exact,
            ..OpSetHandle::INACTIVE
        };
        self.overlay_compile_cache = None;
    }

    pub fn ensure_reduction_soft_session(
        &mut self,
        ctx: &GpuContext,
        n_slots: u32,
        n_dims: u32,
        output_vectors: &wgpu::Buffer,
    ) {
        if self.reduction_soft_session.is_none() {
            let mut session = AccumulatorOpSession::new_attached_for_external_values(
                ctx,
                n_slots,
                n_dims,
                output_vectors,
            );
            self.bind_eml_if_present(&mut session);
            self.reduction_soft_session = Some(session);
        }
    }

    pub fn clear_reduction_soft(&mut self) {
        self.reduction_soft_session = None;
        self.reduction_soft_ops = OpSetHandle {
            family: OperationFamily::ReductionSoft,
            exactness: ExactnessClass::SoftAggregate,
            ..OpSetHandle::INACTIVE
        };
        self.reduction_exact_ops = OpSetHandle {
            family: OperationFamily::ReductionExact,
            exactness: ExactnessClass::Exact,
            ..OpSetHandle::INACTIVE
        };
    }

    pub fn ensure_velocity_session(
        &mut self,
        ctx: &GpuContext,
        n_slots: u32,
        n_dims: u32,
        emission_capacity: u32,
    ) {
        if self.velocity_session.is_none() {
            let mut session = AccumulatorOpSession::new_attached(
                ctx,
                n_slots,
                n_dims,
                emission_capacity,
            );
            self.bind_eml_if_present(&mut session);
            self.velocity_session = Some(session);
        }
    }

    pub fn clear_velocity(&mut self) {
        self.velocity_session = None;
        self.velocity_ops = OpSetHandle {
            family: OperationFamily::Velocity,
            exactness: ExactnessClass::Exact,
            ..OpSetHandle::INACTIVE
        };
    }

    pub fn ensure_intensity_eml_session(
        &mut self,
        ctx: &GpuContext,
        n_slots: u32,
        n_dims: u32,
        emission_capacity: u32,
    ) {
        if self.intensity_eml_session.is_none() {
            let mut session = AccumulatorOpSession::new_attached(
                ctx,
                n_slots,
                n_dims,
                emission_capacity,
            );
            self.bind_eml_if_present(&mut session);
            self.intensity_eml_session = Some(session);
        }
    }

    pub fn clear_intensity_eml(&mut self) {
        self.intensity_eml_session = None;
        self.intensity_tree_ids.clear();
        self.intensity_ops_registry_generation = None;
        self.intensity_op_plan_signature = None;
        self.intensity_eml_ops = OpSetHandle {
            family: OperationFamily::EvalEml,
            exactness: ExactnessClass::Exact,
            ..OpSetHandle::INACTIVE
        };
    }

    pub fn upload_intensity_eml_ops(
        &mut self,
        ctx: &GpuContext,
        ops: &[AccumulatorOp],
        n_bands: u32,
        signature: IntensityEmlOpPlanSignature,
    ) -> Result<(), AccumulatorOpSessionError> {
        let registry_generation = signature.eml_registry_generation;
        let needs_upload = self.intensity_op_plan_signature.as_ref() != Some(&signature)
            || self
                .intensity_eml_session
                .as_ref()
                .map(|s| s.n_ops() == 0)
                .unwrap_or(true);

        if needs_upload {
            let shape_mismatch = self.intensity_eml_session.as_ref().is_some_and(|s| {
                s.n_slots() != signature.n_slots || s.n_dims() != signature.n_dims
            });
            if shape_mismatch {
                self.intensity_eml_session = None;
            }
        }

        if !needs_upload {
            return Ok(());
        }

        if self.intensity_eml_session.is_none() {
            self.ensure_intensity_eml_session(
                ctx,
                signature.n_slots,
                signature.n_dims,
                DEFAULT_THRESHOLD_EMISSION_CAPACITY,
            );
        }

        if let Some(session) = self.intensity_eml_session.as_mut() {
            session.upload_ops_with_eml(ctx, ops, Some(&self.eml_registry))?;
            self.intensity_eml_ops = OpSetHandle {
                family: OperationFamily::EvalEml,
                offset: 0,
                count: session.n_ops(),
                active: !ops.is_empty(),
                n_bands,
                exactness: ExactnessClass::Exact,
            };
            self.intensity_op_plan_signature = Some(signature);
            self.intensity_ops_registry_generation = Some(registry_generation);
            self.intensity_op_upload_count += 1;
        }
        Ok(())
    }

    pub fn ensure_input_list_table(&mut self, ctx: &GpuContext) {
        if self.input_lists.is_none() {
            self.input_lists = Some(super::input_list_table::AccumulatorInputListTable::new(
                ctx,
                super::input_list_table::DEFAULT_INPUT_LIST_CAPACITY,
            ));
        }
    }

    pub fn input_list_bind_buffer(&self) -> Option<&wgpu::Buffer> {
        self.input_lists.as_ref().map(|t| t.buffer())
    }

    pub fn ensure_transfer_session(
        &mut self,
        ctx: &GpuContext,
        n_slots: u32,
        n_dims: u32,
        emission_capacity: u32,
    ) {
        if self.transfer_session.is_none() {
            let mut session =
                AccumulatorOpSession::new_attached(ctx, n_slots, n_dims, emission_capacity);
            self.bind_eml_if_present(&mut session);
            self.transfer_session = Some(session);
        }
    }

    pub fn clear_transfer(&mut self) {
        self.transfer_session = None;
        self.transfer_op_plan_signature = None;
        self.transfer_ops = OpSetHandle {
            family: OperationFamily::EconomicTransfer,
            exactness: ExactnessClass::Exact,
            ..OpSetHandle::INACTIVE
        };
    }

    pub fn take_transfer_session(&mut self) -> Option<AccumulatorOpSession> {
        self.transfer_session.take()
    }

    pub fn restore_transfer_session(&mut self, session: Option<AccumulatorOpSession>) {
        self.transfer_session = session;
    }

    pub fn transfer_session(&mut self) -> Option<&mut AccumulatorOpSession> {
        self.transfer_session.as_mut()
    }

    pub fn transfer_active(&self) -> bool {
        self.transfer_ops.active
    }

    pub fn transfer_bands(&self) -> u32 {
        self.transfer_ops.n_bands
    }

    pub fn transfer_op_upload_count(&self) -> u64 {
        self.transfer_op_upload_count
    }

    pub fn transfer_op_plan_signature(&self) -> Option<&TransferOpPlanSignature> {
        self.transfer_op_plan_signature.as_ref()
    }

    pub fn upload_transfer_ops(
        &mut self,
        ctx: &GpuContext,
        gpu_ops: &[AccumulatorOpGpu],
        n_bands: u32,
        signature: TransferOpPlanSignature,
    ) -> Result<(), AccumulatorOpSessionError> {
        let needs_upload = self.transfer_op_plan_signature.as_ref() != Some(&signature)
            || self
                .transfer_session
                .as_ref()
                .map(|s| s.n_ops() == 0)
                .unwrap_or(true);

        if needs_upload {
            let shape_mismatch = self.transfer_session.as_ref().is_some_and(|s| {
                s.n_slots() != signature.n_slots || s.n_dims() != signature.n_dims
            });
            if shape_mismatch {
                self.transfer_session = None;
            }
        }

        if !needs_upload {
            return Ok(());
        }

        if self.transfer_session.is_none() {
            self.ensure_transfer_session(
                ctx,
                signature.n_slots,
                signature.n_dims,
                DEFAULT_THRESHOLD_EMISSION_CAPACITY,
            );
        }

        if let Some(session) = self.transfer_session.as_mut() {
            session.upload_gpu_ops(ctx, gpu_ops)?;
            self.transfer_ops = OpSetHandle {
                family: OperationFamily::EconomicTransfer,
                offset: 0,
                count: session.n_ops(),
                active: !gpu_ops.is_empty(),
                n_bands,
                exactness: ExactnessClass::Exact,
            };
            self.transfer_op_plan_signature = Some(signature);
            self.transfer_op_upload_count += 1;
        }
        Ok(())
    }

    pub fn ensure_emission_session(
        &mut self,
        ctx: &GpuContext,
        n_slots: u32,
        n_dims: u32,
        emission_capacity: u32,
    ) {
        if self.emission_session.is_none() {
            let mut session =
                AccumulatorOpSession::new_attached(ctx, n_slots, n_dims, emission_capacity);
            self.bind_eml_if_present(&mut session);
            self.emission_session = Some(session);
        }
    }

    pub fn clear_emission(&mut self) {
        self.emission_session = None;
        self.emission_op_plan_signature = None;
        self.emission_ops = OpSetHandle {
            family: OperationFamily::EconomicEmission,
            exactness: ExactnessClass::Exact,
            ..OpSetHandle::INACTIVE
        };
    }

    pub fn take_emission_session(&mut self) -> Option<AccumulatorOpSession> {
        self.emission_session.take()
    }

    pub fn restore_emission_session(&mut self, session: Option<AccumulatorOpSession>) {
        self.emission_session = session;
    }

    pub fn emission_session(&mut self) -> Option<&mut AccumulatorOpSession> {
        self.emission_session.as_mut()
    }

    pub fn emission_active(&self) -> bool {
        self.emission_ops.active
    }

    pub fn emission_bands(&self) -> u32 {
        self.emission_ops.n_bands
    }

    pub fn emission_op_upload_count(&self) -> u64 {
        self.emission_op_upload_count
    }

    pub fn emission_op_plan_signature(&self) -> Option<&EmissionOpPlanSignature> {
        self.emission_op_plan_signature.as_ref()
    }

    pub fn upload_emission_ops(
        &mut self,
        ctx: &GpuContext,
        gpu_ops: &[AccumulatorOpGpu],
        n_bands: u32,
        signature: EmissionOpPlanSignature,
    ) -> Result<(), AccumulatorOpSessionError> {
        let needs_upload = self.emission_op_plan_signature.as_ref() != Some(&signature)
            || self
                .emission_session
                .as_ref()
                .map(|s| s.n_ops() == 0)
                .unwrap_or(true);

        if needs_upload {
            let shape_mismatch = self.emission_session.as_ref().is_some_and(|s| {
                s.n_slots() != signature.n_slots || s.n_dims() != signature.n_dims
            });
            if shape_mismatch {
                self.emission_session = None;
            }
        }

        if !needs_upload {
            return Ok(());
        }

        if self.emission_session.is_none() {
            self.ensure_emission_session(
                ctx,
                signature.n_slots,
                signature.n_dims,
                DEFAULT_THRESHOLD_EMISSION_CAPACITY,
            );
        }

        if let Some(session) = self.emission_session.as_mut() {
            session.upload_gpu_ops(ctx, gpu_ops)?;
            self.emission_ops = OpSetHandle {
                family: OperationFamily::EconomicEmission,
                offset: 0,
                count: session.n_ops(),
                active: !gpu_ops.is_empty(),
                n_bands,
                exactness: ExactnessClass::Exact,
            };
            self.emission_op_plan_signature = Some(signature);
            self.emission_op_upload_count += 1;
        }
        Ok(())
    }

    pub fn register_intensity_eml_at_boundary(
        &mut self,
        dimension_registry: &simthing_core::DimensionRegistry,
    ) -> Result<Vec<crate::IntensityEmlEntry>, simthing_core::EmlRegistryError> {
        use crate::intensity_accumulator::register_intensity_eml_formulas;
        let previous = self.intensity_tree_ids.clone();
        let entries = register_intensity_eml_formulas(
            &mut self.eml_registry,
            dimension_registry,
            &previous,
        )?;
        self.intensity_tree_ids = entries.iter().map(|e| e.tree_id).collect();
        Ok(entries)
    }

    pub fn upload_velocity_ops(
        &mut self,
        ctx: &GpuContext,
        ops: &[AccumulatorOpGpu],
        n_bands: u32,
    ) -> Result<(), AccumulatorOpSessionError> {
        if let Some(session) = self.velocity_session.as_mut() {
            session.upload_gpu_ops(ctx, ops)?;
            self.velocity_ops = OpSetHandle {
                family: OperationFamily::Velocity,
                offset: 0,
                count: session.n_ops(),
                active: !ops.is_empty(),
                n_bands,
                exactness: ExactnessClass::Exact,
            };
        }
        Ok(())
    }

    pub fn upload_reduction_soft_ops(
        &mut self,
        ctx: &GpuContext,
        ops: &[AccumulatorOpGpu],
        n_bands: u32,
        exact_active: bool,
    ) -> Result<(), AccumulatorOpSessionError> {
        if let Some(session) = self.reduction_soft_session.as_mut() {
            session.upload_gpu_ops(ctx, ops)?;
            self.reduction_soft_ops = OpSetHandle {
                family: OperationFamily::ReductionSoft,
                offset: 0,
                count: session.n_ops(),
                active: !ops.is_empty(),
                n_bands,
                exactness: ExactnessClass::SoftAggregate,
            };
            self.reduction_exact_ops = OpSetHandle {
                family: OperationFamily::ReductionExact,
                offset: 0,
                count: if exact_active { session.n_ops() } else { 0 },
                active: exact_active && !ops.is_empty(),
                n_bands,
                exactness: ExactnessClass::Exact,
            };
        }
        Ok(())
    }

    pub fn upload_intent_ops(
        &mut self,
        ctx: &GpuContext,
        deltas: &[IntentDelta],
    ) -> Result<(), AccumulatorOpSessionError> {
        if let Some(session) = self.intent_session.as_mut() {
            session.upload_intent_ops(ctx, deltas)?;
            self.intent_ops.count = session.n_ops();
        }
        Ok(())
    }

    pub fn upload_threshold_ops(
        &mut self,
        ctx: &GpuContext,
        regs: &[ThresholdRegistration],
    ) -> Result<(), AccumulatorOpSessionError> {
        if let Some(session) = self.threshold_session.as_mut() {
            session.upload_threshold_ops(ctx, regs)?;
            self.threshold_ops.count = session.n_ops();
        }
        Ok(())
    }

    pub fn upload_overlay_add_ops(
        &mut self,
        ctx: &GpuContext,
        ops: &[AccumulatorOpGpu],
        n_bands: u32,
    ) -> Result<(), AccumulatorOpSessionError> {
        self.upload_overlay_ops(ctx, ops, n_bands)
    }

    pub fn upload_overlay_ops(
        &mut self,
        ctx: &GpuContext,
        ops: &[AccumulatorOpGpu],
        n_bands: u32,
    ) -> Result<(), AccumulatorOpSessionError> {
        if let Some(session) = self.overlay_session.as_mut() {
            session.upload_gpu_ops(ctx, ops)?;
            self.overlay_order_ops = OpSetHandle {
                family: OperationFamily::OverlayOrderBand,
                offset: 0,
                count: session.n_ops(),
                active: !ops.is_empty(),
                n_bands,
                exactness: ExactnessClass::Exact,
            };
            self.overlay_add_ops = OpSetHandle {
                family: OperationFamily::OverlayAdd,
                offset: 0,
                count: session.n_ops(),
                active: !ops.is_empty(),
                n_bands,
                exactness: ExactnessClass::Exact,
            };
        }
        Ok(())
    }

    pub fn readback_threshold_events(
        &mut self,
        ctx: &GpuContext,
    ) -> Result<Vec<crate::ThresholdEvent>, AccumulatorOpSessionError> {
        if let Some(session) = self.threshold_session.as_mut() {
            session.readback_threshold_events(ctx)
        } else {
            Ok(Vec::new())
        }
    }

    pub fn ensure_summary(&mut self, ctx: &GpuContext, n_slots: u32, n_dims: u32) {
        let needs_rebuild = self
            .summary
            .as_ref()
            .map_or(true, |s| s.n_slots() != n_slots || s.n_dims() != n_dims);
        if needs_rebuild {
            self.summary = Some(WorldSummaryRuntime::new(ctx, n_slots, n_dims));
        }
    }

    pub fn encode_world_summary_into(
        &mut self,
        ctx: &GpuContext,
        encoder: &mut wgpu::CommandEncoder,
        values: &wgpu::Buffer,
    ) -> bool {
        if let Some(summary) = self.summary.as_ref() {
            summary.encode_into(ctx, encoder, values);
            true
        } else {
            false
        }
    }

    pub fn dispatch_world_summary(&mut self, ctx: &GpuContext, values: &wgpu::Buffer) -> bool {
        if let Some(summary) = self.summary.as_ref() {
            summary.dispatch(ctx, values);
            true
        } else {
            false
        }
    }

    pub fn readback_world_summary(
        &self,
        ctx: &GpuContext,
    ) -> Result<Vec<super::types::SlotSummary>, AccumulatorOpSessionError> {
        if let Some(summary) = self.summary.as_ref() {
            summary.readback(ctx)
        } else {
            Ok(Vec::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world_state::IntentDelta;

    #[test]
    fn runtime_tracks_per_family_sessions() {
        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut runtime = WorldAccumulatorRuntime::new();
        runtime.ensure_intent_session(&ctx, 2, 1, 16);
        runtime
            .upload_intent_ops(
                &ctx,
                &[IntentDelta {
                    slot: 0,
                    col: 0,
                    mul: 1.0,
                    add: 1.0,
                }],
            )
            .unwrap();
        assert!(runtime.intent_active());
        assert_eq!(runtime.intent_ops.count, 1);
    }

    #[test]
    fn c8a_boundary_sync_skips_unchanged_eml_table_upload() {
        use simthing_core::{eml_opcode, EmlExecutionClass, EmlFormulaMeta, EmlNodeGpu, EmlTreeId};

        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut runtime = WorldAccumulatorRuntime::new();
        runtime
            .eml_registry
            .register_formula(
                EmlTreeId(1),
                EmlFormulaMeta {
                    tree_id: EmlTreeId(1),
                    execution_class: EmlExecutionClass::ExactDeterministic,
                    allowed_consumers: Default::default(),
                    max_abs_error: None,
                    deterministic_gpu: true,
                    requires_guard_for_hard_threshold: false,
                    node_count: 2,
                    max_stack_depth: 1,
                    has_loops: false,
                    has_recursion: false,
                    display_name: "lit".into(),
                },
                vec![
                    EmlNodeGpu {
                        opcode: eml_opcode::LITERAL_F32,
                        flags: 0,
                        a: 1.0f32.to_bits(),
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
                ],
            )
            .unwrap();
        runtime.upload_eml_trees(&ctx).unwrap();
        let table = runtime.eml.as_ref().unwrap();
        let node_uploads = table.node_upload_count;
        let range_uploads = table.range_upload_count;
        let generation = table.generation;
        runtime.upload_eml_trees(&ctx).unwrap();
        let table = runtime.eml.as_ref().unwrap();
        assert_eq!(table.node_upload_count, node_uploads);
        assert_eq!(table.range_upload_count, range_uploads);
        assert_eq!(table.generation, generation);
    }

    #[test]
    fn c8b_intensity_op_plan_signature_controls_upload_cache() {
        use simthing_core::{
            compile_intensity_behavior_to_eml, intensity_tree_id, AccumulatorOp, CombineFn,
            ConsumeMode, GateSpec, IntensityBehavior, ScaleSpec, SourceSpec,
        };

        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut runtime = WorldAccumulatorRuntime::new();
        let behavior = IntensityBehavior::default();
        let (meta, nodes) = compile_intensity_behavior_to_eml(
            &behavior,
            intensity_tree_id(0),
            1,
            2,
        );
        runtime
            .eml_registry
            .replace_formula(intensity_tree_id(0), meta, nodes)
            .unwrap();
        runtime.upload_eml_trees(&ctx).unwrap();
        let gen = runtime.eml_registry.generation();

        let op = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 0, col: 2 },
            combine: CombineFn::EvalEML {
                tree_id: intensity_tree_id(0).0,
            },
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(0, 2)],
        };
        let signature = IntensityEmlOpPlanSignature {
            eml_registry_generation: gen,
            n_slots: 1,
            n_dims: 4,
            n_entries: 1,
            n_ops: 1,
            tree_ids: vec![intensity_tree_id(0).0],
            intensity_cols: vec![2],
            velocity_cols: vec![1],
        };
        runtime
            .upload_intensity_eml_ops(&ctx, &[op.clone()], 1, signature.clone())
            .unwrap();
        assert_eq!(runtime.intensity_op_upload_count(), 1);

        runtime
            .upload_intensity_eml_ops(&ctx, &[op.clone()], 1, signature)
            .unwrap();
        assert_eq!(runtime.intensity_op_upload_count(), 1);

        let op_slot1 = AccumulatorOp {
            source: SourceSpec::SlotValue { slot: 1, col: 2 },
            combine: CombineFn::EvalEML {
                tree_id: intensity_tree_id(0).0,
            },
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(1, 2)],
        };
        let signature_grown = IntensityEmlOpPlanSignature {
            eml_registry_generation: gen,
            n_slots: 2,
            n_dims: 4,
            n_entries: 1,
            n_ops: 2,
            tree_ids: vec![intensity_tree_id(0).0],
            intensity_cols: vec![2],
            velocity_cols: vec![1],
        };
        runtime
            .upload_intensity_eml_ops(&ctx, &[op, op_slot1], 1, signature_grown)
            .unwrap();
        assert_eq!(runtime.intensity_op_upload_count(), 2);
        assert_eq!(
            runtime.intensity_eml_session.as_ref().unwrap().n_ops(),
            2
        );
    }
}
