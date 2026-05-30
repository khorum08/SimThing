//! Accepted FrontierV2 artifact target identifiers for future L2 lowering.
//!
//! Metadata / admission vocabulary only — no runtime invocation.

/// Lowering target for accepted FrontierV2 fixture artifacts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AcceptedFrontierArtifactTarget {
    /// Container for the accepted FrontierV2 fixture artifact set.
    AcceptedFrontierV2FixtureArtifacts,
    /// FrontierV2-4 combined movement + structural feedback loop fixture.
    FrontierV2CombinedFeedbackFixture,
    /// Fixture-only own-column movement shadow (not production state).
    FrontierV2OwnColumnShadow,
    /// Fixture-only BoundaryRequest shadow (not production commitment).
    FrontierV2BoundaryRequestShadow,
    /// Resource dispatch route through the accepted Resource Flow allocator.
    ResourceFlowAllocatorRoute,
}

impl AcceptedFrontierArtifactTarget {
    pub const fn id(self) -> &'static str {
        match self {
            Self::AcceptedFrontierV2FixtureArtifacts => "AcceptedFrontierV2FixtureArtifacts",
            Self::FrontierV2CombinedFeedbackFixture => "FrontierV2CombinedFeedbackFixture",
            Self::FrontierV2OwnColumnShadow => "FrontierV2OwnColumnShadow",
            Self::FrontierV2BoundaryRequestShadow => "FrontierV2BoundaryRequestShadow",
            Self::ResourceFlowAllocatorRoute => "ResourceFlowAllocatorRoute",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::AcceptedFrontierV2FixtureArtifacts => {
                "Accepted FrontierV2 fixture artifact set exercised by L0 (V1-5 → V2-0..4)"
            }
            Self::FrontierV2CombinedFeedbackFixture => {
                "FrontierV2-4 combined movement + structural feedback loop fixture (fingerprint dbb54b952f9face8)"
            }
            Self::FrontierV2OwnColumnShadow => {
                "Fixture-only own-column movement shadow; production movement writes remain rejected"
            }
            Self::FrontierV2BoundaryRequestShadow => {
                "Fixture-only BoundaryRequest shadow; production commitment emission remains rejected"
            }
            Self::ResourceFlowAllocatorRoute => {
                "Resource dispatch through accepted Resource Flow allocator OrderBand sweeps"
            }
        }
    }
}

/// All accepted FrontierV2 artifact lowering targets in deterministic order.
pub fn accepted_frontier_v2_artifact_targets() -> &'static [AcceptedFrontierArtifactTarget] {
    &[
        AcceptedFrontierArtifactTarget::AcceptedFrontierV2FixtureArtifacts,
        AcceptedFrontierArtifactTarget::FrontierV2CombinedFeedbackFixture,
        AcceptedFrontierArtifactTarget::FrontierV2OwnColumnShadow,
        AcceptedFrontierArtifactTarget::FrontierV2BoundaryRequestShadow,
        AcceptedFrontierArtifactTarget::ResourceFlowAllocatorRoute,
    ]
}

/// Stable string identifiers for accepted FrontierV2 artifact targets.
pub fn accepted_frontier_v2_artifact_target_ids() -> &'static [&'static str] {
    &[
        "AcceptedFrontierV2FixtureArtifacts",
        "FrontierV2CombinedFeedbackFixture",
        "FrontierV2OwnColumnShadow",
        "FrontierV2BoundaryRequestShadow",
        "ResourceFlowAllocatorRoute",
    ]
}
