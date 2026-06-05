//! Stable designer-facing admission diagnostic vocabulary.

/// High-level guardrail bucket surfaced to designers at import/admission time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DesignerFacingGuardrailClass {
    DefaultOff,
    ResourceFlowRouting,
    MovementWriteBoundary,
    CommitmentEmission,
    TickTimeContention,
    EconomySubstrate,
    CpuDecisionPath,
    ShaderSemantics,
    RuntimeWiring,
    SimSemanticLeakage,
    MappingExpansion,
    ResourceFlowExpansion,
    DiscreteOrderingExpansion,
    MobilityScenario,
    AuthoringFrontEnd,
    ConsumerProofLadder,
}

/// Specific rejection reason within the designer admission substrate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DesignerAdmissionRejectionKind {
    MalformedManifest,
    UnknownArtifactTarget,
    DefaultOnRequest,
    ResourceFlowBypass,
    CrossEntityMovementWrite,
    ProductionMovementWrite,
    ProductionCommitmentEmission,
    SharedPoolTickWrite,
    ParallelFixtureEconomy,
    CpuPlanner,
    CpuUrgency,
    CpuCommitmentEmission,
    SemanticWgslRequest,
    SchedulerCacheRequest,
    SimthingSimSemanticStateRequest,
    AtlasWithoutGate,
    ActiveMaskWithoutGate,
    PerceptionFogWithoutGate,
    SourceIdentityWithoutGate,
    NestedE11BWithoutNamedScenario,
    E11B5WithoutNamedScenario,
    D2aWithoutNamedScenario,
    OwnerEntitySpatialParent,
    CaptureAsReparenting,
    GpuAllocatorSemaphore,
    IndirectionBeforeSlab,
    ArrivalOrderReplayOrdering,
    HybridStrataSilentRebind,
    HardSoftMixedPass,
    FloatStructuralGate,
    MaxFactionsPerCellExceeded,
    RoutingEmlNodeBudgetExceeded,
    HardCurrencyThroughResourceFlow,
    ClosedLadderReopen,
    ClauseScriptParserParked,
    ClauseThingRuntimeParked,
    FrontierV2FiveRejected,
    FieldPolicyLadderReopenRejected,
}

/// Stable diagnostic code string for designer admission rejections.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DesignerAdmissionDiagnosticCode {
    MalformedManifestRejected,
    UnknownArtifactTargetRejected,
    DefaultOnRejected,
    ResourceFlowBypassRejected,
    CrossEntityMovementWriteRejected,
    ProductionMovementWriteRejected,
    ProductionCommitmentEmissionRejected,
    SharedPoolTickWriteRejected,
    ParallelFixtureEconomyRejected,
    CpuPlannerRejected,
    CpuUrgencyRejected,
    CpuCommitmentEmissionRejected,
    SemanticWgslRequestRejected,
    SchedulerCacheRequestRejected,
    SimthingSimSemanticStateRequestRejected,
    AtlasRequestedWithoutGate,
    ActiveMaskRequestedWithoutGate,
    PerceptionFogRequestedWithoutGate,
    SourceIdentityRequestedWithoutGate,
    // C-2 specific
    AtlasSpecNotHomogeneousSquareRejected,
    AtlasSpecUnsupportedIsolationRejected,
    AtlasSpecMissingProtocolOracleRejected,
    AtlasSpecOverActiveBudgetRejected,
    AtlasSpecMissingMultiplierReportingRejected,
    AtlasSpecPhysicalGutterRequiresRaisedGateRejected,
    AtlasProductionRuntimeRejected,
    NestedE11BRequestedWithoutNamedScenario,
    E11B5RequestedWithoutNamedScenario,
    D2aRequestedWithoutNamedScenario,
    MobilityOwnerSpatialParentRejected,
    MobilityCaptureAsReparentingRejected,
    MobilityGpuAllocatorSemaphoreRejected,
    MobilityIndirectionBeforeSlabRejected,
    MobilityArrivalOrderReplayOrderingRejected,
    MobilityHybridStrataSilentRebindRejected,
    MobilityHardSoftMixedPassRejected,
    MobilityFloatStructuralGateRejected,
    MobilityMaxFactionsPerCellExceeded,
    MobilityRoutingEmlNodeBudgetExceeded,
    MobilityHardCurrencyThroughResourceFlowRejected,
    MobilityClosedLadderReopenRejected,
    ClauseScriptParserRequestParked,
    ClauseThingRuntimeRequestParked,
    FrontierV2FiveRequestRejected,
    ActEventObsPipeLadderReopenRejected,
}

impl DesignerAdmissionDiagnosticCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MalformedManifestRejected => "L1-0-MALFORMED-MANIFEST-REJECTED",
            Self::UnknownArtifactTargetRejected => "L1-0-UNKNOWN-ARTIFACT-TARGET-REJECTED",
            Self::DefaultOnRejected => "L1-0-DEFAULT-ON-REJECTED",
            Self::ResourceFlowBypassRejected => "L1-0-RESOURCE-FLOW-BYPASS-REJECTED",
            Self::CrossEntityMovementWriteRejected => "L1-0-CROSS-ENTITY-MOVEMENT-WRITE-REJECTED",
            Self::ProductionMovementWriteRejected => "L1-0-PRODUCTION-MOVEMENT-WRITE-REJECTED",
            Self::ProductionCommitmentEmissionRejected => {
                "L1-0-PRODUCTION-COMMITMENT-EMISSION-REJECTED"
            }
            Self::SharedPoolTickWriteRejected => "L1-0-SHARED-POOL-TICK-WRITE-REJECTED",
            Self::ParallelFixtureEconomyRejected => "L1-0-PARALLEL-FIXTURE-ECONOMY-REJECTED",
            Self::CpuPlannerRejected => "L1-0-CPU-PLANNER-REJECTED",
            Self::CpuUrgencyRejected => "L1-0-CPU-URGENCY-REJECTED",
            Self::CpuCommitmentEmissionRejected => "L1-0-CPU-COMMITMENT-EMISSION-REJECTED",
            Self::SemanticWgslRequestRejected => "L1-0-SEMANTIC-WGSL-REQUEST-REJECTED",
            Self::SchedulerCacheRequestRejected => "L1-0-SCHEDULER-CACHE-REQUEST-REJECTED",
            Self::SimthingSimSemanticStateRequestRejected => {
                "L1-0-SIMTHING-SIM-SEMANTIC-STATE-REQUEST-REJECTED"
            }
            Self::AtlasRequestedWithoutGate => "L1-0-ATLAS-REQUESTED-WITHOUT-GATE",
            Self::AtlasSpecNotHomogeneousSquareRejected => {
                "C-2-ATLAS-SPEC-NOT-HOMOGENEOUS-SQUARE-REJECTED"
            }
            Self::AtlasSpecUnsupportedIsolationRejected => {
                "C-2-ATLAS-SPEC-UNSUPPORTED-ISOLATION-REJECTED"
            }
            Self::AtlasSpecMissingProtocolOracleRejected => {
                "C-2-ATLAS-SPEC-MISSING-PROTOCOL-ORACLE-REJECTED"
            }
            Self::AtlasSpecOverActiveBudgetRejected => "C-2-ATLAS-SPEC-OVER-ACTIVE-BUDGET-REJECTED",
            Self::AtlasSpecMissingMultiplierReportingRejected => {
                "C-2-ATLAS-SPEC-MISSING-MULTIPLIER-REPORTING-REJECTED"
            }
            Self::AtlasSpecPhysicalGutterRequiresRaisedGateRejected => {
                "C-2-ATLAS-SPEC-PHYSICAL-GUTTER-REQUIRES-RAISED-GATE-REJECTED"
            }
            Self::AtlasProductionRuntimeRejected => "C-2-ATLAS-PRODUCTION-RUNTIME-REJECTED",
            Self::ActiveMaskRequestedWithoutGate => "L1-0-ACTIVE-MASK-REQUESTED-WITHOUT-GATE",
            Self::PerceptionFogRequestedWithoutGate => "L1-0-PERCEPTION-FOG-REQUESTED-WITHOUT-GATE",
            Self::SourceIdentityRequestedWithoutGate => {
                "L1-0-SOURCE-IDENTITY-REQUESTED-WITHOUT-GATE"
            }
            Self::NestedE11BRequestedWithoutNamedScenario => {
                "L1-0-NESTED-E11B-REQUESTED-WITHOUT-NAMED-SCENARIO"
            }
            Self::E11B5RequestedWithoutNamedScenario => {
                "L1-0-E11B5-REQUESTED-WITHOUT-NAMED-SCENARIO"
            }
            Self::D2aRequestedWithoutNamedScenario => "L1-0-D2A-REQUESTED-WITHOUT-NAMED-SCENARIO",
            Self::MobilityOwnerSpatialParentRejected => {
                "MOBILITY-SCENARIO-0-OWNER-SPATIAL-PARENT-REJECTED"
            }
            Self::MobilityCaptureAsReparentingRejected => {
                "MOBILITY-SCENARIO-0-CAPTURE-AS-REPARENTING-REJECTED"
            }
            Self::MobilityGpuAllocatorSemaphoreRejected => {
                "MOBILITY-SCENARIO-0-GPU-ALLOCATOR-SEMAPHORE-REJECTED"
            }
            Self::MobilityIndirectionBeforeSlabRejected => {
                "MOBILITY-SCENARIO-0-INDIRECTION-BEFORE-SLAB-REJECTED"
            }
            Self::MobilityArrivalOrderReplayOrderingRejected => {
                "MOBILITY-SCENARIO-0-ARRIVAL-ORDER-REPLAY-ORDERING-REJECTED"
            }
            Self::MobilityHybridStrataSilentRebindRejected => {
                "MOBILITY-SCENARIO-0-HYBRID-STRATA-SILENT-REBIND-REJECTED"
            }
            Self::MobilityHardSoftMixedPassRejected => {
                "MOBILITY-SCENARIO-0-HARD-SOFT-MIXED-PASS-REJECTED"
            }
            Self::MobilityFloatStructuralGateRejected => {
                "MOBILITY-SCENARIO-0-FLOAT-STRUCTURAL-GATE-REJECTED"
            }
            Self::MobilityMaxFactionsPerCellExceeded => {
                "MOBILITY-SCENARIO-0-MAX-FACTIONS-PER-CELL-EXCEEDED"
            }
            Self::MobilityRoutingEmlNodeBudgetExceeded => {
                "MOBILITY-SCENARIO-0-ROUTING-EML-NODE-BUDGET-EXCEEDED"
            }
            Self::MobilityHardCurrencyThroughResourceFlowRejected => {
                "MOBILITY-SCENARIO-0-HARD-CURRENCY-THROUGH-RESOURCE-FLOW-REJECTED"
            }
            Self::MobilityClosedLadderReopenRejected => {
                "MOBILITY-SCENARIO-0-CLOSED-LADDER-REOPEN-REJECTED"
            }
            Self::ClauseScriptParserRequestParked => "L1-0-CLAUSESCRIPT-PARSER-REQUEST-PARKED",
            Self::ClauseThingRuntimeRequestParked => "L1-0-CLAUSETHING-RUNTIME-REQUEST-PARKED",
            Self::FrontierV2FiveRequestRejected => "L1-0-FRONTIERV2-5-REQUEST-REJECTED",
            Self::ActEventObsPipeLadderReopenRejected => {
                "L1-0-ACT-EVENT-OBS-PIPE-LADDER-REOPEN-REJECTED"
            }
        }
    }

    pub fn guardrail_class(self) -> DesignerFacingGuardrailClass {
        match self {
            Self::MalformedManifestRejected | Self::UnknownArtifactTargetRejected => {
                DesignerFacingGuardrailClass::AuthoringFrontEnd
            }
            Self::DefaultOnRejected => DesignerFacingGuardrailClass::DefaultOff,
            Self::ResourceFlowBypassRejected => DesignerFacingGuardrailClass::ResourceFlowRouting,
            Self::CrossEntityMovementWriteRejected | Self::ProductionMovementWriteRejected => {
                DesignerFacingGuardrailClass::MovementWriteBoundary
            }
            Self::ProductionCommitmentEmissionRejected => {
                DesignerFacingGuardrailClass::CommitmentEmission
            }
            Self::SharedPoolTickWriteRejected => DesignerFacingGuardrailClass::TickTimeContention,
            Self::ParallelFixtureEconomyRejected => DesignerFacingGuardrailClass::EconomySubstrate,
            Self::CpuPlannerRejected
            | Self::CpuUrgencyRejected
            | Self::CpuCommitmentEmissionRejected => DesignerFacingGuardrailClass::CpuDecisionPath,
            Self::SemanticWgslRequestRejected => DesignerFacingGuardrailClass::ShaderSemantics,
            Self::SchedulerCacheRequestRejected | Self::AtlasProductionRuntimeRejected => {
                DesignerFacingGuardrailClass::RuntimeWiring
            }
            Self::SimthingSimSemanticStateRequestRejected => {
                DesignerFacingGuardrailClass::SimSemanticLeakage
            }
            Self::AtlasRequestedWithoutGate
            | Self::AtlasSpecNotHomogeneousSquareRejected
            | Self::AtlasSpecUnsupportedIsolationRejected
            | Self::AtlasSpecMissingProtocolOracleRejected
            | Self::AtlasSpecOverActiveBudgetRejected
            | Self::AtlasSpecMissingMultiplierReportingRejected
            | Self::AtlasSpecPhysicalGutterRequiresRaisedGateRejected
            | Self::ActiveMaskRequestedWithoutGate
            | Self::PerceptionFogRequestedWithoutGate
            | Self::SourceIdentityRequestedWithoutGate => {
                DesignerFacingGuardrailClass::MappingExpansion
            }
            Self::NestedE11BRequestedWithoutNamedScenario
            | Self::E11B5RequestedWithoutNamedScenario => {
                DesignerFacingGuardrailClass::ResourceFlowExpansion
            }
            Self::D2aRequestedWithoutNamedScenario => {
                DesignerFacingGuardrailClass::DiscreteOrderingExpansion
            }
            Self::MobilityOwnerSpatialParentRejected
            | Self::MobilityCaptureAsReparentingRejected
            | Self::MobilityGpuAllocatorSemaphoreRejected
            | Self::MobilityIndirectionBeforeSlabRejected
            | Self::MobilityArrivalOrderReplayOrderingRejected
            | Self::MobilityHybridStrataSilentRebindRejected
            | Self::MobilityHardSoftMixedPassRejected
            | Self::MobilityFloatStructuralGateRejected
            | Self::MobilityMaxFactionsPerCellExceeded
            | Self::MobilityRoutingEmlNodeBudgetExceeded
            | Self::MobilityHardCurrencyThroughResourceFlowRejected
            | Self::MobilityClosedLadderReopenRejected => {
                DesignerFacingGuardrailClass::MobilityScenario
            }
            Self::ClauseScriptParserRequestParked | Self::ClauseThingRuntimeRequestParked => {
                DesignerFacingGuardrailClass::AuthoringFrontEnd
            }
            Self::FrontierV2FiveRequestRejected | Self::ActEventObsPipeLadderReopenRejected => {
                DesignerFacingGuardrailClass::ConsumerProofLadder
            }
        }
    }

    pub fn rejection_kind(self) -> DesignerAdmissionRejectionKind {
        match self {
            Self::MalformedManifestRejected => DesignerAdmissionRejectionKind::MalformedManifest,
            Self::UnknownArtifactTargetRejected => {
                DesignerAdmissionRejectionKind::UnknownArtifactTarget
            }
            Self::DefaultOnRejected => DesignerAdmissionRejectionKind::DefaultOnRequest,
            Self::ResourceFlowBypassRejected => DesignerAdmissionRejectionKind::ResourceFlowBypass,
            Self::CrossEntityMovementWriteRejected => {
                DesignerAdmissionRejectionKind::CrossEntityMovementWrite
            }
            Self::ProductionMovementWriteRejected => {
                DesignerAdmissionRejectionKind::ProductionMovementWrite
            }
            Self::ProductionCommitmentEmissionRejected => {
                DesignerAdmissionRejectionKind::ProductionCommitmentEmission
            }
            Self::SharedPoolTickWriteRejected => {
                DesignerAdmissionRejectionKind::SharedPoolTickWrite
            }
            Self::ParallelFixtureEconomyRejected => {
                DesignerAdmissionRejectionKind::ParallelFixtureEconomy
            }
            Self::CpuPlannerRejected => DesignerAdmissionRejectionKind::CpuPlanner,
            Self::CpuUrgencyRejected => DesignerAdmissionRejectionKind::CpuUrgency,
            Self::CpuCommitmentEmissionRejected => {
                DesignerAdmissionRejectionKind::CpuCommitmentEmission
            }
            Self::SemanticWgslRequestRejected => {
                DesignerAdmissionRejectionKind::SemanticWgslRequest
            }
            Self::SchedulerCacheRequestRejected => {
                DesignerAdmissionRejectionKind::SchedulerCacheRequest
            }
            Self::SimthingSimSemanticStateRequestRejected => {
                DesignerAdmissionRejectionKind::SimthingSimSemanticStateRequest
            }
            Self::AtlasRequestedWithoutGate
            | Self::AtlasSpecNotHomogeneousSquareRejected
            | Self::AtlasSpecUnsupportedIsolationRejected
            | Self::AtlasSpecMissingProtocolOracleRejected
            | Self::AtlasSpecOverActiveBudgetRejected
            | Self::AtlasSpecMissingMultiplierReportingRejected
            | Self::AtlasSpecPhysicalGutterRequiresRaisedGateRejected
            | Self::AtlasProductionRuntimeRejected => {
                DesignerAdmissionRejectionKind::AtlasWithoutGate
            }
            Self::ActiveMaskRequestedWithoutGate => {
                DesignerAdmissionRejectionKind::ActiveMaskWithoutGate
            }
            Self::PerceptionFogRequestedWithoutGate => {
                DesignerAdmissionRejectionKind::PerceptionFogWithoutGate
            }
            Self::SourceIdentityRequestedWithoutGate => {
                DesignerAdmissionRejectionKind::SourceIdentityWithoutGate
            }
            Self::NestedE11BRequestedWithoutNamedScenario => {
                DesignerAdmissionRejectionKind::NestedE11BWithoutNamedScenario
            }
            Self::E11B5RequestedWithoutNamedScenario => {
                DesignerAdmissionRejectionKind::E11B5WithoutNamedScenario
            }
            Self::D2aRequestedWithoutNamedScenario => {
                DesignerAdmissionRejectionKind::D2aWithoutNamedScenario
            }
            Self::MobilityOwnerSpatialParentRejected => {
                DesignerAdmissionRejectionKind::OwnerEntitySpatialParent
            }
            Self::MobilityCaptureAsReparentingRejected => {
                DesignerAdmissionRejectionKind::CaptureAsReparenting
            }
            Self::MobilityGpuAllocatorSemaphoreRejected => {
                DesignerAdmissionRejectionKind::GpuAllocatorSemaphore
            }
            Self::MobilityIndirectionBeforeSlabRejected => {
                DesignerAdmissionRejectionKind::IndirectionBeforeSlab
            }
            Self::MobilityArrivalOrderReplayOrderingRejected => {
                DesignerAdmissionRejectionKind::ArrivalOrderReplayOrdering
            }
            Self::MobilityHybridStrataSilentRebindRejected => {
                DesignerAdmissionRejectionKind::HybridStrataSilentRebind
            }
            Self::MobilityHardSoftMixedPassRejected => {
                DesignerAdmissionRejectionKind::HardSoftMixedPass
            }
            Self::MobilityFloatStructuralGateRejected => {
                DesignerAdmissionRejectionKind::FloatStructuralGate
            }
            Self::MobilityMaxFactionsPerCellExceeded => {
                DesignerAdmissionRejectionKind::MaxFactionsPerCellExceeded
            }
            Self::MobilityRoutingEmlNodeBudgetExceeded => {
                DesignerAdmissionRejectionKind::RoutingEmlNodeBudgetExceeded
            }
            Self::MobilityHardCurrencyThroughResourceFlowRejected => {
                DesignerAdmissionRejectionKind::HardCurrencyThroughResourceFlow
            }
            Self::MobilityClosedLadderReopenRejected => {
                DesignerAdmissionRejectionKind::ClosedLadderReopen
            }
            Self::ClauseScriptParserRequestParked => {
                DesignerAdmissionRejectionKind::ClauseScriptParserParked
            }
            Self::ClauseThingRuntimeRequestParked => {
                DesignerAdmissionRejectionKind::ClauseThingRuntimeParked
            }
            Self::FrontierV2FiveRequestRejected => {
                DesignerAdmissionRejectionKind::FrontierV2FiveRejected
            }
            Self::ActEventObsPipeLadderReopenRejected => {
                DesignerAdmissionRejectionKind::FieldPolicyLadderReopenRejected
            }
        }
    }
}

/// Designer-facing admission diagnostic emitted at import/admission time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DesignerAdmissionDiagnostic {
    pub code: DesignerAdmissionDiagnosticCode,
    pub guardrail_class: DesignerFacingGuardrailClass,
    pub rejection_kind: DesignerAdmissionRejectionKind,
    pub message: String,
    pub hint: Option<String>,
}

impl DesignerAdmissionDiagnostic {
    pub fn code_str(&self) -> &'static str {
        self.code.as_str()
    }
}

pub fn designer_admission_diagnostic(
    code: DesignerAdmissionDiagnosticCode,
    message: impl Into<String>,
    hint: Option<&str>,
) -> DesignerAdmissionDiagnostic {
    DesignerAdmissionDiagnostic {
        guardrail_class: code.guardrail_class(),
        rejection_kind: code.rejection_kind(),
        code,
        message: message.into(),
        hint: hint.map(str::to_owned),
    }
}

pub fn designer_admission_diagnostic_for_rejection(
    kind: DesignerAdmissionRejectionKind,
) -> DesignerAdmissionDiagnostic {
    let (code, message, hint) = match kind {
        DesignerAdmissionRejectionKind::MalformedManifest => (
            DesignerAdmissionDiagnosticCode::MalformedManifestRejected,
            "designer admission manifest/scenario is malformed",
            Some("provide non-empty ids, FrontierV2 profile, bounded grid/ticks, and required fixture artifact targets"),
        ),
        DesignerAdmissionRejectionKind::UnknownArtifactTarget => (
            DesignerAdmissionDiagnosticCode::UnknownArtifactTargetRejected,
            "unknown FrontierV2 artifact target requested",
            Some("use accepted FrontierV2 artifact target identifiers from the L1 vocabulary"),
        ),
        DesignerAdmissionRejectionKind::DefaultOnRequest => (
            DesignerAdmissionDiagnosticCode::DefaultOnRejected,
            "default-on execution or SimSession wiring is rejected at designer admission",
            Some("keep scenarios opt-in/default-off until a separately gated product decision authorizes default wiring"),
        ),
        DesignerAdmissionRejectionKind::ResourceFlowBypass => (
            DesignerAdmissionDiagnosticCode::ResourceFlowBypassRejected,
            "resource dispatch must route through the accepted Resource Flow allocator substrate",
            Some("use OrderBand sweeps with AddToTarget on independent participant columns; do not bypass the allocator"),
        ),
        DesignerAdmissionRejectionKind::CrossEntityMovementWrite => (
            DesignerAdmissionDiagnosticCode::CrossEntityMovementWriteRejected,
            "cross-entity movement writes are rejected at designer admission",
            Some("movement feedback may update own-column fixture shadows only; source_unit_id must match shadow.unit_id"),
        ),
        DesignerAdmissionRejectionKind::ProductionMovementWrite => (
            DesignerAdmissionDiagnosticCode::ProductionMovementWriteRejected,
            "production movement writes are rejected at designer admission",
            Some("FrontierV2 movement remains fixture-only shadow state until a separately gated runtime decision lands"),
        ),
        DesignerAdmissionRejectionKind::ProductionCommitmentEmission => (
            DesignerAdmissionDiagnosticCode::ProductionCommitmentEmissionRejected,
            "production commitment emission is rejected at designer admission",
            Some("structural feedback uses BoundaryRequest fixture shadows only; Threshold+EmitEvent remains GPU-resident and separately gated for production"),
        ),
        DesignerAdmissionRejectionKind::SharedPoolTickWrite => (
            DesignerAdmissionDiagnosticCode::SharedPoolTickWriteRejected,
            "shared-pool tick-time writes are rejected at designer admission",
            Some("Resource Flow uses independent participant columns with no shared-pool contention at tick time"),
        ),
        DesignerAdmissionRejectionKind::ParallelFixtureEconomy => (
            DesignerAdmissionDiagnosticCode::ParallelFixtureEconomyRejected,
            "parallel fixture-local economies are rejected at designer admission",
            Some("economy influence must route through accepted discrete ResourceEconomySpec or Resource Flow substrates, not fixture-local parallel ledgers"),
        ),
        DesignerAdmissionRejectionKind::CpuPlanner => (
            DesignerAdmissionDiagnosticCode::CpuPlannerRejected,
            "CPU-side map or AI planner requests are rejected at designer admission",
            Some("decisions emerge as GPU Threshold+EmitEvent crossings; CPU consumes resolved summaries at boundaries only"),
        ),
        DesignerAdmissionRejectionKind::CpuUrgency => (
            DesignerAdmissionDiagnosticCode::CpuUrgencyRejected,
            "CPU-side urgency computation is rejected at designer admission",
            Some("urgency stays GPU-resident via field→reduction→EvalEML→Threshold"),
        ),
        DesignerAdmissionRejectionKind::CpuCommitmentEmission => (
            DesignerAdmissionDiagnosticCode::CpuCommitmentEmissionRejected,
            "CPU-side commitment emission is rejected at designer admission",
            Some("commitments are GPU Threshold+EmitEvent crossings; CPU must not emit structural commitments"),
        ),
        DesignerAdmissionRejectionKind::SemanticWgslRequest => (
            DesignerAdmissionDiagnosticCode::SemanticWgslRequestRejected,
            "semantic WGSL requests are rejected at designer admission",
            Some("only generic semantic-free shader extensions with CPU-oracle parity and admission-pinned meaning are admissible"),
        ),
        DesignerAdmissionRejectionKind::SchedulerCacheRequest => (
            DesignerAdmissionDiagnosticCode::SchedulerCacheRequestRejected,
            "scheduler/cache/default SimSession wiring requests are rejected at designer admission",
            Some("PROD-0 provides explicit registered exact cohort execution only; scheduler/cache remain separately gated"),
        ),
        DesignerAdmissionRejectionKind::SimthingSimSemanticStateRequest => (
            DesignerAdmissionDiagnosticCode::SimthingSimSemanticStateRequestRejected,
            "simthing-sim semantic/map/Gadget/Personality state requests are rejected at designer admission",
            Some("semantics compile away in simthing-spec/driver to flat AccumulatorOp registrations"),
        ),
        DesignerAdmissionRejectionKind::AtlasWithoutGate => (
            DesignerAdmissionDiagnosticCode::AtlasRequestedWithoutGate,
            "atlas batching is rejected until a named multi-theater scenario, VRAM budget, and §11-gate PR pass",
            Some("request_atlas_batching stays rejected at admission; see v7.8 Line C"),
        ),
        DesignerAdmissionRejectionKind::ActiveMaskWithoutGate => (
            DesignerAdmissionDiagnosticCode::ActiveMaskRequestedWithoutGate,
            "active-mask halo execution is rejected without a separately gated contract",
            Some("ActiveOnlyExperimentalNoHalo is never production-authorized"),
        ),
        DesignerAdmissionRejectionKind::PerceptionFogWithoutGate => (
            DesignerAdmissionDiagnosticCode::PerceptionFogRequestedWithoutGate,
            "perception/fog/deception fields are rejected without a separately gated scenario",
            Some("perceived columns may not write back into authoritative true fields except via explicit gameplay events"),
        ),
        DesignerAdmissionRejectionKind::SourceIdentityWithoutGate => (
            DesignerAdmissionDiagnosticCode::SourceIdentityRequestedWithoutGate,
            "behavioral source identity/source_mask is rejected without a separately gated track",
            Some("column-wide source_col zeroing is banned; source identity remains deferred under the Mapping ADR"),
        ),
        DesignerAdmissionRejectionKind::NestedE11BWithoutNamedScenario => (
            DesignerAdmissionDiagnosticCode::NestedE11BRequestedWithoutNamedScenario,
            "nested E-11B hierarchical allocation is rejected without a named depth>2 economy scenario",
            Some("FlatStarResourceFlow remains the accepted bounded posture; see v7.8 Line A"),
        ),
        DesignerAdmissionRejectionKind::E11B5WithoutNamedScenario => (
            DesignerAdmissionDiagnosticCode::E11B5RequestedWithoutNamedScenario,
            "E-11B-5 dynamic enrollment is rejected without a named depth>2 economy scenario",
            Some("nested arena enrollment remains parked behind v7.8 Line A"),
        ),
        DesignerAdmissionRejectionKind::D2aWithoutNamedScenario => (
            DesignerAdmissionDiagnosticCode::D2aRequestedWithoutNamedScenario,
            "D-2a boundary transaction scheduling is rejected without a named hard-currency contention scenario",
            Some("discrete AccumulatorOp transfer/recipe/emission remains the standing path; see v7.8 Line B"),
        ),
        DesignerAdmissionRejectionKind::OwnerEntitySpatialParent => (
            DesignerAdmissionDiagnosticCode::MobilityOwnerSpatialParentRejected,
            "owner-entities may not be spatial parents in MOBILITY-SCENARIO-0",
            Some("model factions/species/blueprints as owner columns and session-descendant owner-entities, never spatial containment"),
        ),
        DesignerAdmissionRejectionKind::CaptureAsReparenting => (
            DesignerAdmissionDiagnosticCode::MobilityCaptureAsReparentingRejected,
            "capture must be an owner-column flip, not spatial reparenting",
            Some("only physical movement reparents; political ownership changes update columns"),
        ),
        DesignerAdmissionRejectionKind::GpuAllocatorSemaphore => (
            DesignerAdmissionDiagnosticCode::MobilityGpuAllocatorSemaphoreRejected,
            "GPU-side allocator semaphores or nondeterministic atomics are rejected",
            Some("slot accounting is deterministic CPU/driver boundary work; a future GPU variant would need deterministic scan"),
        ),
        DesignerAdmissionRejectionKind::IndirectionBeforeSlab => (
            DesignerAdmissionDiagnosticCode::MobilityIndirectionBeforeSlabRejected,
            "indirection buffers are rejected before the slab/block path is attempted",
            Some("use deterministic slab/block reservation and bulk accounting first"),
        ),
        DesignerAdmissionRejectionKind::ArrivalOrderReplayOrdering => (
            DesignerAdmissionDiagnosticCode::MobilityArrivalOrderReplayOrderingRejected,
            "arrival order may not be replay-significant",
            Some("canonicalize boundary event ordering and use deterministic lowest-free-first allocation"),
        ),
        DesignerAdmissionRejectionKind::HybridStrataSilentRebind => (
            DesignerAdmissionDiagnosticCode::MobilityHybridStrataSilentRebindRejected,
            "Hybrid Strata channel rebinding requires generation bump/resync",
            Some("channel bindings are CPU enrollment metadata; silent rebinding would break replay"),
        ),
        DesignerAdmissionRejectionKind::HardSoftMixedPass => (
            DesignerAdmissionDiagnosticCode::MobilityHardSoftMixedPassRejected,
            "hard fixed-point and soft float quantities may not silently mix in one pass",
            Some("run hard Band Alpha before soft Band Beta and require explicit class boundaries"),
        ),
        DesignerAdmissionRejectionKind::FloatStructuralGate => (
            DesignerAdmissionDiagnosticCode::MobilityFloatStructuralGateRejected,
            "float values may not gate structural transitions",
            Some("structural-decision variables must be hard fixed-point / exact-authoritative"),
        ),
        DesignerAdmissionRejectionKind::MaxFactionsPerCellExceeded => (
            DesignerAdmissionDiagnosticCode::MobilityMaxFactionsPerCellExceeded,
            "scenario exceeds max_factions_per_cell",
            Some("narrow the first slice or raise the declared bound through a later accepted scenario"),
        ),
        DesignerAdmissionRejectionKind::RoutingEmlNodeBudgetExceeded => (
            DesignerAdmissionDiagnosticCode::MobilityRoutingEmlNodeBudgetExceeded,
            "routing EML node budget is exceeded",
            Some("max_factions_per_cell must fit the directed-disburse EML budget for the first slice"),
        ),
        DesignerAdmissionRejectionKind::HardCurrencyThroughResourceFlow => (
            DesignerAdmissionDiagnosticCode::MobilityHardCurrencyThroughResourceFlowRejected,
            "hard currency may not route through Resource Flow",
            Some("hard fixed-point/discrete quantities stay in Band Alpha / Phase T paths, not continuous Resource Flow"),
        ),
        DesignerAdmissionRejectionKind::ClosedLadderReopen => (
            DesignerAdmissionDiagnosticCode::MobilityClosedLadderReopenRejected,
            "MOBILITY-SCENARIO-0 may not reopen closed or parked ladders",
            Some("do not reopen A/B/C, FrontierV2-5, ACT/EVENT/OBS/PIPE, ClauseThing/L3, atlas runtime, or E-11B-5"),
        ),
        DesignerAdmissionRejectionKind::ClauseScriptParserParked => (
            DesignerAdmissionDiagnosticCode::ClauseScriptParserRequestParked,
            "ClauseScript parser/front-end requests are parked until L3 ClauseThing authorization",
            Some("L1 builds simthing-spec admission substrate; CLAUSE-SPEC-0 is L2 and remains downstream"),
        ),
        DesignerAdmissionRejectionKind::ClauseThingRuntimeParked => (
            DesignerAdmissionDiagnosticCode::ClauseThingRuntimeRequestParked,
            "ClauseThing runtime/front-end requests are parked until L2 CLAUSE-SPEC lands and L3 is explicitly authorized",
            Some("ClauseThing remains proposal-only; do not implement the parser in L1"),
        ),
        DesignerAdmissionRejectionKind::FrontierV2FiveRejected => (
            DesignerAdmissionDiagnosticCode::FrontierV2FiveRequestRejected,
            "FrontierV2-5 requests are rejected; the bounded multi-tick consumer proof is complete at fixture level",
            Some("next gate is L1 simthing-spec buildout, then L2 CLAUSE-SPEC-0"),
        ),
        DesignerAdmissionRejectionKind::FieldPolicyLadderReopenRejected => (
            DesignerAdmissionDiagnosticCode::ActEventObsPipeLadderReopenRejected,
            "ACT-5/EVENT-3/OBS-5/PIPE-1 ladder reopen requests are rejected",
            Some("FIELD_POLICY Field agent Proposal Pipeline V1 is consolidated and closed; further stages require a separately named scenario"),
        ),
    };
    designer_admission_diagnostic(code, message, hint)
}

/// All stable diagnostic codes in deterministic order.
pub fn all_designer_admission_diagnostic_codes() -> &'static [DesignerAdmissionDiagnosticCode] {
    &[
        DesignerAdmissionDiagnosticCode::MalformedManifestRejected,
        DesignerAdmissionDiagnosticCode::UnknownArtifactTargetRejected,
        DesignerAdmissionDiagnosticCode::DefaultOnRejected,
        DesignerAdmissionDiagnosticCode::ResourceFlowBypassRejected,
        DesignerAdmissionDiagnosticCode::CrossEntityMovementWriteRejected,
        DesignerAdmissionDiagnosticCode::ProductionMovementWriteRejected,
        DesignerAdmissionDiagnosticCode::ProductionCommitmentEmissionRejected,
        DesignerAdmissionDiagnosticCode::SharedPoolTickWriteRejected,
        DesignerAdmissionDiagnosticCode::ParallelFixtureEconomyRejected,
        DesignerAdmissionDiagnosticCode::CpuPlannerRejected,
        DesignerAdmissionDiagnosticCode::CpuUrgencyRejected,
        DesignerAdmissionDiagnosticCode::CpuCommitmentEmissionRejected,
        DesignerAdmissionDiagnosticCode::SemanticWgslRequestRejected,
        DesignerAdmissionDiagnosticCode::SchedulerCacheRequestRejected,
        DesignerAdmissionDiagnosticCode::SimthingSimSemanticStateRequestRejected,
        DesignerAdmissionDiagnosticCode::AtlasRequestedWithoutGate,
        DesignerAdmissionDiagnosticCode::ActiveMaskRequestedWithoutGate,
        DesignerAdmissionDiagnosticCode::PerceptionFogRequestedWithoutGate,
        DesignerAdmissionDiagnosticCode::SourceIdentityRequestedWithoutGate,
        DesignerAdmissionDiagnosticCode::NestedE11BRequestedWithoutNamedScenario,
        DesignerAdmissionDiagnosticCode::E11B5RequestedWithoutNamedScenario,
        DesignerAdmissionDiagnosticCode::D2aRequestedWithoutNamedScenario,
        DesignerAdmissionDiagnosticCode::MobilityOwnerSpatialParentRejected,
        DesignerAdmissionDiagnosticCode::MobilityCaptureAsReparentingRejected,
        DesignerAdmissionDiagnosticCode::MobilityGpuAllocatorSemaphoreRejected,
        DesignerAdmissionDiagnosticCode::MobilityIndirectionBeforeSlabRejected,
        DesignerAdmissionDiagnosticCode::MobilityArrivalOrderReplayOrderingRejected,
        DesignerAdmissionDiagnosticCode::MobilityHybridStrataSilentRebindRejected,
        DesignerAdmissionDiagnosticCode::MobilityHardSoftMixedPassRejected,
        DesignerAdmissionDiagnosticCode::MobilityFloatStructuralGateRejected,
        DesignerAdmissionDiagnosticCode::MobilityMaxFactionsPerCellExceeded,
        DesignerAdmissionDiagnosticCode::MobilityRoutingEmlNodeBudgetExceeded,
        DesignerAdmissionDiagnosticCode::MobilityHardCurrencyThroughResourceFlowRejected,
        DesignerAdmissionDiagnosticCode::MobilityClosedLadderReopenRejected,
        DesignerAdmissionDiagnosticCode::ClauseScriptParserRequestParked,
        DesignerAdmissionDiagnosticCode::ClauseThingRuntimeRequestParked,
        DesignerAdmissionDiagnosticCode::FrontierV2FiveRequestRejected,
        DesignerAdmissionDiagnosticCode::ActEventObsPipeLadderReopenRejected,
    ]
}
