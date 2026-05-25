//! AccumulatorOp v2 world runtime envelope (C-INF-1).
//!
//! One logical runtime with named operation sets. C-1/C-2/C-3 still use
//! per-family GPU sessions (adapter shim matching pre-consolidation behavior)
//! until a single shared op buffer is validated for all families together.

use crate::{OverlayDelta, SlotDeltaRange, ThresholdRegistration};

use super::session::{AccumulatorOpSession, AccumulatorOpSessionError};
use super::types::AccumulatorOpGpu;
use super::world_summary::WorldSummaryRuntime;
use crate::world_state::IntentDelta;
use crate::GpuContext;

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

    intent_session: Option<AccumulatorOpSession>,
    threshold_session: Option<AccumulatorOpSession>,
    overlay_session: Option<AccumulatorOpSession>,
    reduction_soft_session: Option<AccumulatorOpSession>,
    pub summary: Option<WorldSummaryRuntime>,
    pub overlay_compile_cache: Option<OverlayCompileCache>,
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
            intent_session: None,
            threshold_session: None,
            overlay_session: None,
            reduction_soft_session: None,
            summary: None,
            overlay_compile_cache: None,
        }
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

    pub fn any_pipeline_active(&self) -> bool {
        self.intent_active()
            || self.threshold_active()
            || self.overlay_order_ops.active
            || self.reduction_soft_ops.active
    }

    pub fn ensure_intent_session(
        &mut self,
        ctx: &GpuContext,
        n_slots: u32,
        n_dims: u32,
        emission_capacity: u32,
    ) {
        if self.intent_session.is_none() {
            self.intent_session = Some(AccumulatorOpSession::new_attached(
                ctx,
                n_slots,
                n_dims,
                emission_capacity,
            ));
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
            self.threshold_session = Some(AccumulatorOpSession::new_attached(
                ctx,
                n_slots,
                n_dims,
                emission_capacity,
            ));
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
            self.overlay_session = Some(AccumulatorOpSession::new_attached(
                ctx,
                n_slots,
                n_dims,
                emission_capacity,
            ));
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
            self.reduction_soft_session = Some(AccumulatorOpSession::new_attached_to_buffer(
                ctx,
                n_slots,
                n_dims,
                output_vectors,
            ));
        }
    }

    pub fn clear_reduction_soft(&mut self) {
        self.reduction_soft_session = None;
        self.reduction_soft_ops = OpSetHandle {
            family: OperationFamily::ReductionSoft,
            exactness: ExactnessClass::SoftAggregate,
            ..OpSetHandle::INACTIVE
        };
    }

    pub fn upload_reduction_soft_ops(
        &mut self,
        ctx: &GpuContext,
        ops: &[AccumulatorOpGpu],
        n_bands: u32,
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
}
