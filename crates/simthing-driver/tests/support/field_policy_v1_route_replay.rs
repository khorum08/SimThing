//! FIELD_POLICY Field agent Proposal Pipeline V1 consumption for FrontierV1 route replay (test-only).
//!
//! Validates accepted PIPE-0 and ACT-2 kernel descriptors are registered and admitted.
//! Does not extend the FIELD_POLICY ladder or add new pipeline stages.

use simthing_spec::{
    is_field_policy_act2_proposal_admission_records_descriptor,
    is_field_policy_pipe0_observer_event_pipeline_descriptor, landed_jit_kernel_descriptors,
    validate_kernel_descriptor_admission, MappingExecutionProfile, FIELD_POLICY_ACT2_DESCRIPTOR_ID,
    FIELD_POLICY_PIPE0_DESCRIPTOR_ID,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldPolicyV1ConsumptionReport {
    pub pipe0_registered: bool,
    pub act2_registered: bool,
}

/// Confirm FIELD_POLICY V1 PIPE-0 and ACT-2 evidence is consumed via landed descriptors (not extended).
pub fn validate_field_policy_v1_consumed() -> FieldPolicyV1ConsumptionReport {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );

    let pipe0 = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == FIELD_POLICY_PIPE0_DESCRIPTOR_ID);
    let act2 = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == FIELD_POLICY_ACT2_DESCRIPTOR_ID);

    if let Some(desc) = &pipe0 {
        validate_kernel_descriptor_admission(desc).expect("PIPE-0 descriptor admits");
        assert!(is_field_policy_pipe0_observer_event_pipeline_descriptor(
            desc
        ));
    }
    if let Some(desc) = &act2 {
        validate_kernel_descriptor_admission(desc).expect("ACT-2 descriptor admits");
        assert!(is_field_policy_act2_proposal_admission_records_descriptor(
            desc
        ));
    }

    assert!(
        pipe0.is_some(),
        "FIELD_POLICY PIPE-0 descriptor must be registered"
    );
    assert!(
        act2.is_some(),
        "FIELD_POLICY ACT-2 descriptor must be registered"
    );

    FieldPolicyV1ConsumptionReport {
        pipe0_registered: true,
        act2_registered: true,
    }
}
