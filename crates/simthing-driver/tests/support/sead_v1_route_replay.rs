//! SEAD Self-AI Proposal Pipeline V1 consumption for FrontierV1 route replay (test-only).
//!
//! Validates accepted PIPE-0 and ACT-2 kernel descriptors are registered and admitted.
//! Does not extend the SEAD ladder or add new pipeline stages.

use simthing_spec::{
    is_sead_act2_proposal_admission_records_descriptor,
    is_sead_pipe0_observer_event_pipeline_descriptor, landed_jit_kernel_descriptors,
    validate_kernel_descriptor_admission, MappingExecutionProfile, SEAD_ACT2_DESCRIPTOR_ID,
    SEAD_PIPE0_DESCRIPTOR_ID,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeadV1ConsumptionReport {
    pub pipe0_registered: bool,
    pub act2_registered: bool,
}

/// Confirm SEAD V1 PIPE-0 and ACT-2 evidence is consumed via landed descriptors (not extended).
pub fn validate_sead_v1_consumed() -> SeadV1ConsumptionReport {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );

    let pipe0 = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == SEAD_PIPE0_DESCRIPTOR_ID);
    let act2 = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == SEAD_ACT2_DESCRIPTOR_ID);

    if let Some(desc) = &pipe0 {
        validate_kernel_descriptor_admission(desc).expect("PIPE-0 descriptor admits");
        assert!(is_sead_pipe0_observer_event_pipeline_descriptor(desc));
    }
    if let Some(desc) = &act2 {
        validate_kernel_descriptor_admission(desc).expect("ACT-2 descriptor admits");
        assert!(is_sead_act2_proposal_admission_records_descriptor(desc));
    }

    assert!(pipe0.is_some(), "SEAD PIPE-0 descriptor must be registered");
    assert!(act2.is_some(), "SEAD ACT-2 descriptor must be registered");

    SeadV1ConsumptionReport {
        pipe0_registered: true,
        act2_registered: true,
    }
}
