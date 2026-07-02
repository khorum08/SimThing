//! FIELD_POLICY-OBS-1 — Descriptor/admission for mobile observer overlay score.

use simthing_spec::{
    exact_sqrt_f_artifact_descriptor, field_policy_act0_numeric_proposals_kernel_descriptor,
    field_policy_act1_phase_e_proposal_consumer_kernel_descriptor,
    field_policy_act2_proposal_admission_records_kernel_descriptor,
    field_policy_act3_economic_fixture_records_kernel_descriptor,
    field_policy_event0_compaction_kernel_descriptor,
    field_policy_event1_code_bucketing_kernel_descriptor,
    field_policy_event2_bucket_reductions_kernel_descriptor,
    field_policy_obs0_overlay_score_kernel_descriptor,
    field_policy_obs2_multilayer_overlay_score_kernel_descriptor,
    field_policy_obs3_multilayer_fixed_score_kernel_descriptor,
    field_policy_obs4_threshold_event_kernel_descriptor,
    field_policy_pipe0_observer_event_pipeline_kernel_descriptor,
    is_field_policy_act0_numeric_proposals_descriptor,
    is_field_policy_act1_phase_e_proposal_consumer_descriptor,
    is_field_policy_act2_proposal_admission_records_descriptor,
    is_field_policy_act3_economic_fixture_records_descriptor,
    is_field_policy_event0_compaction_descriptor, is_field_policy_event1_code_bucketing_descriptor,
    is_field_policy_event2_bucket_reductions_descriptor,
    is_field_policy_obs0_overlay_score_descriptor,
    is_field_policy_obs2_multilayer_overlay_score_descriptor,
    is_field_policy_obs3_multilayer_fixed_score_descriptor,
    is_field_policy_obs4_threshold_event_descriptor,
    is_field_policy_pipe0_observer_event_pipeline_descriptor, landed_jit_kernel_descriptors,
    validate_kernel_descriptor_admission, ExactPreSqrtInputContract, KernelDescriptorSpec,
    Mag2SourceContract, NativeMathClass, OutputAuthority, ScoreAuthorityContract, SpecError,
    FIELD_POLICY_ACT0_DESCRIPTOR_ID, FIELD_POLICY_ACT1_DESCRIPTOR_ID,
    FIELD_POLICY_ACT2_DESCRIPTOR_ID, FIELD_POLICY_ACT3_DESCRIPTOR_ID,
    FIELD_POLICY_EVENT0_DESCRIPTOR_ID, FIELD_POLICY_EVENT1_DESCRIPTOR_ID,
    FIELD_POLICY_EVENT2_DESCRIPTOR_ID, FIELD_POLICY_OBS0_DESCRIPTOR_ID, FIELD_POLICY_OBS0_LABEL,
    FIELD_POLICY_OBS2_DESCRIPTOR_ID, FIELD_POLICY_OBS2_LABEL, FIELD_POLICY_OBS2_LAYER_COUNT,
    FIELD_POLICY_OBS3_DESCRIPTOR_ID, FIELD_POLICY_OBS3_LABEL, FIELD_POLICY_OBS4_DESCRIPTOR_ID,
    FIELD_POLICY_OBS4_LABEL, FIELD_POLICY_PIPE0_DESCRIPTOR_ID, MAG2_Q16_FRAC_BITS,
    SQRT_F_ARTIFACT_HASH,
};

fn obs2() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == FIELD_POLICY_OBS2_DESCRIPTOR_ID)
        .expect("field_policy obs2 descriptor")
}

fn obs3() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == FIELD_POLICY_OBS3_DESCRIPTOR_ID)
        .expect("field_policy obs3 descriptor")
}

fn obs4() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == FIELD_POLICY_OBS4_DESCRIPTOR_ID)
        .expect("field_policy obs4 descriptor")
}

fn event0() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == FIELD_POLICY_EVENT0_DESCRIPTOR_ID)
        .expect("field_policy event0 descriptor")
}

fn pipe0() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == FIELD_POLICY_PIPE0_DESCRIPTOR_ID)
        .expect("field_policy pipe0 descriptor")
}

fn event1() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == FIELD_POLICY_EVENT1_DESCRIPTOR_ID)
        .expect("field_policy event1 descriptor")
}

fn event2() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == FIELD_POLICY_EVENT2_DESCRIPTOR_ID)
        .expect("field_policy event2 descriptor")
}

fn act0() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == FIELD_POLICY_ACT0_DESCRIPTOR_ID)
        .expect("field_policy act0 descriptor")
}

fn act1() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == FIELD_POLICY_ACT1_DESCRIPTOR_ID)
        .expect("field_policy act1 descriptor")
}

fn act2() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == FIELD_POLICY_ACT2_DESCRIPTOR_ID)
        .expect("field_policy act2 descriptor")
}

fn act3() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == FIELD_POLICY_ACT3_DESCRIPTOR_ID)
        .expect("field_policy act3 descriptor")
}

fn obs0() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == FIELD_POLICY_OBS0_DESCRIPTOR_ID)
        .expect("field_policy obs0 descriptor")
}

fn assert_admission_err(spec: &KernelDescriptorSpec, reason_substr: &str) {
    let err = validate_kernel_descriptor_admission(spec).expect_err("expected admission failure");
    match err {
        SpecError::JitKernelDescriptorAdmission { reason, .. } => {
            assert!(
                reason.contains(reason_substr),
                "expected `{reason_substr}` in `{reason}`"
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}
