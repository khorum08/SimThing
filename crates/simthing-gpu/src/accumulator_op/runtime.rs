//! AccumulatorOp v2 world runtime envelope (C-INF-1 scaffold).
//!
//! Target: one GPU-resident runtime with named operation sets instead of
//! per-family `Option<AccumulatorOpSession>` sidecars. New migrations register
//! into this structure; existing C-1/C-2/C-3 sessions may remain adapter-
//! shimmed until consolidation PR completes.

use super::session::AccumulatorOpSession;

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
    pub family:       OperationFamily,
    pub offset:       u32,
    pub count:        u32,
    pub active:       bool,
    pub n_bands:      u32,
    pub exactness:    ExactnessClass,
}

impl OpSetHandle {
    pub const INACTIVE: Self = Self {
        family:    OperationFamily::Intent,
        offset:    0,
        count:     0,
        active:    false,
        n_bands:   0,
        exactness: ExactnessClass::Exact,
    };
}

/// Single AccumulatorOp runtime envelope for `WorldGpuState` (target shape).
///
/// C-INF-1 consolidation PR will replace `threshold_accumulator`,
/// `intent_accumulator`, and `overlay_add_accumulator` sidecars with this
/// struct. Until then, sidecars remain authoritative at runtime.
pub struct WorldAccumulatorRuntime {
    pub session:              AccumulatorOpSession,
    pub intent_ops:           OpSetHandle,
    pub threshold_ops:        OpSetHandle,
    pub overlay_add_ops:      OpSetHandle,
    pub overlay_order_ops:    OpSetHandle,
    pub reduction_soft_ops:   OpSetHandle,
    pub reduction_exact_ops:  OpSetHandle,
    pub velocity_ops:         OpSetHandle,
    pub intensity_eml_ops:    OpSetHandle,
}

impl WorldAccumulatorRuntime {
    pub fn new(session: AccumulatorOpSession) -> Self {
        Self {
            session,
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
        }
    }
}
