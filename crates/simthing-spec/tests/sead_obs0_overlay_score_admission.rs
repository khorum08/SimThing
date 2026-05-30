//! SEAD-OBS-1 — Descriptor/admission for mobile observer overlay score.

use simthing_spec::{
    exact_sqrt_f_artifact_descriptor, is_sead_obs0_overlay_score_descriptor,
    landed_jit_kernel_descriptors, sead_obs0_overlay_score_kernel_descriptor,
    sead_obs2_multilayer_overlay_score_kernel_descriptor,
    sead_obs3_multilayer_fixed_score_kernel_descriptor,
    sead_obs4_threshold_event_kernel_descriptor,
    validate_kernel_descriptor_admission, ExactPreSqrtInputContract,
    KernelDescriptorSpec, Mag2SourceContract, NativeMathClass, OutputAuthority,
    ScoreAuthorityContract, SEAD_OBS0_DESCRIPTOR_ID, SEAD_OBS0_LABEL, SEAD_OBS2_DESCRIPTOR_ID,
    SEAD_OBS2_LABEL, SEAD_OBS2_LAYER_COUNT, SEAD_OBS3_DESCRIPTOR_ID, SEAD_OBS3_LABEL,
    SEAD_OBS4_DESCRIPTOR_ID, SEAD_OBS4_LABEL, SEAD_EVENT0_DESCRIPTOR_ID,
    sead_event0_compaction_kernel_descriptor,
    sead_pipe0_observer_event_pipeline_kernel_descriptor,
    SEAD_PIPE0_DESCRIPTOR_ID,
    SEAD_EVENT1_DESCRIPTOR_ID,
    sead_event1_code_bucketing_kernel_descriptor,
    SEAD_EVENT2_DESCRIPTOR_ID,
    sead_event2_bucket_reductions_kernel_descriptor,
    SpecError, MAG2_Q16_FRAC_BITS, SQRT_F_ARTIFACT_HASH,
    is_sead_obs2_multilayer_overlay_score_descriptor,
    is_sead_obs3_multilayer_fixed_score_descriptor,
    is_sead_obs4_threshold_event_descriptor,
    is_sead_event0_compaction_descriptor,
    is_sead_pipe0_observer_event_pipeline_descriptor,
    is_sead_event1_code_bucketing_descriptor,
    is_sead_event2_bucket_reductions_descriptor,
};

fn obs2() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == SEAD_OBS2_DESCRIPTOR_ID)
        .expect("sead obs2 descriptor")
}

fn obs3() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == SEAD_OBS3_DESCRIPTOR_ID)
        .expect("sead obs3 descriptor")
}

fn obs4() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == SEAD_OBS4_DESCRIPTOR_ID)
        .expect("sead obs4 descriptor")
}

fn event0() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == SEAD_EVENT0_DESCRIPTOR_ID)
        .expect("sead event0 descriptor")
}

fn pipe0() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == SEAD_PIPE0_DESCRIPTOR_ID)
        .expect("sead pipe0 descriptor")
}

fn event1() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == SEAD_EVENT1_DESCRIPTOR_ID)
        .expect("sead event1 descriptor")
}

fn event2() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == SEAD_EVENT2_DESCRIPTOR_ID)
        .expect("sead event2 descriptor")
}

fn obs0() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == SEAD_OBS0_DESCRIPTOR_ID)
        .expect("sead obs0 descriptor")
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

#[test]
fn sead_obs1_descriptor_admits_default_off_overlay_score() {
    let desc = obs0();
    assert_eq!(desc.id, SEAD_OBS0_DESCRIPTOR_ID);
    assert!(desc.default_off);
    assert!(!desc.production_wiring);
    assert!(is_sead_obs0_overlay_score_descriptor(&desc));
    validate_kernel_descriptor_admission(&desc).expect("obs0 admits");
    println!(
        "sead_obs1_descriptor: id={SEAD_OBS0_DESCRIPTOR_ID} label={SEAD_OBS0_LABEL} default_off=true production_wiring=false"
    );
}

#[test]
fn sead_obs1_output_authority_is_correct() {
    let desc = obs0();
    let mag2 = desc
        .writes
        .iter()
        .find(|out| out.name == "mag2_bits")
        .expect("mag2_bits");
    let mag = desc
        .writes
        .iter()
        .find(|out| out.name == "mag_bits")
        .expect("mag_bits");
    let score = desc
        .writes
        .iter()
        .find(|out| out.name == "score_bits")
        .expect("score_bits");
    let flags = desc.writes.iter().find(|out| out.name == "flags").expect("flags");
    assert_eq!(mag2.authority, OutputAuthority::ExactAuthoritative);
    assert_eq!(mag.authority, OutputAuthority::ExactAuthoritative);
    assert_eq!(score.authority, OutputAuthority::ApproximateDiagnostic);
    assert_eq!(flags.authority, OutputAuthority::ApproximateDiagnostic);
    println!(
        "sead_obs1_authority: mag2_bits=Exact mag_bits=Exact score_bits=Approximate flags=Approximate"
    );
}

#[test]
fn sead_obs1_rejects_exact_score_for_f32_score_arithmetic() {
    let mut bad = sead_obs0_overlay_score_kernel_descriptor();
    for out in &mut bad.writes {
        if out.name == "score_bits" {
            out.authority = OutputAuthority::ExactAuthoritative;
        }
    }
    assert_admission_err(
        &bad,
        "score_bits cannot be ExactAuthoritative under ApproximateDiagnosticF32",
    );
}

#[test]
fn sead_obs1_exact_mag_requires_q16_and_f_artifact() {
    let mut wrong_q = sead_obs0_overlay_score_kernel_descriptor();
    wrong_q.mag2_source_contract = Some(Mag2SourceContract::ExactFixedPointDxDy {
        fraction_bits: 12,
    });
    assert_admission_err(&wrong_q, "Q16.16");

    let mut no_mag2_contract = sead_obs0_overlay_score_kernel_descriptor();
    no_mag2_contract.mag2_source_contract = None;
    assert_admission_err(&no_mag2_contract, "Mag2SourceContract");

    let mut no_f = sead_obs0_overlay_score_kernel_descriptor();
    no_f.exact_sqrt_artifact = None;
    assert_admission_err(&no_f, "artifact-backed Candidate F");

    let mut bad_hash = sead_obs0_overlay_score_kernel_descriptor();
    let mut binding = exact_sqrt_f_artifact_descriptor();
    binding.artifact_hash_fnv1a64 = "0000000000000000".into();
    bad_hash.exact_sqrt_artifact = Some(binding);
    assert_admission_err(&bad_hash, "hash mismatch");

    let mut native_sqrt = sead_obs0_overlay_score_kernel_descriptor();
    native_sqrt.native_math = NativeMathClass::ApproximateJitOnly;
    assert_admission_err(&native_sqrt, "approximate native math");

    let desc = obs0();
    let binding = desc
        .exact_sqrt_artifact
        .as_ref()
        .expect("F binding");
    assert_eq!(binding.artifact_hash_fnv1a64, SQRT_F_ARTIFACT_HASH);
    assert_eq!(
        desc.mag2_source_contract,
        Some(Mag2SourceContract::ExactFixedPointDxDy {
            fraction_bits: MAG2_Q16_FRAC_BITS,
        })
    );
    assert_eq!(
        desc.pre_sqrt_contract,
        Some(ExactPreSqrtInputContract::InlineFixedPointMag2Sqrt)
    );
    assert_eq!(
        desc.score_authority_contract,
        Some(ScoreAuthorityContract::ApproximateDiagnosticF32)
    );
}

#[test]
fn sead_obs1_rejects_production_wiring_and_default_on() {
    let mut prod = sead_obs0_overlay_score_kernel_descriptor();
    prod.production_wiring = true;
    assert_admission_err(&prod, "production_wiring");

    let mut default_on = sead_obs0_overlay_score_kernel_descriptor();
    default_on.default_off = false;
    assert_admission_err(&default_on, "default_off");
}

#[test]
fn sead_obs1_builder_matches_landed_registry() {
    let built = sead_obs0_overlay_score_kernel_descriptor();
    let landed = obs0();
    assert_eq!(built.id, landed.id);
    assert_eq!(built.reads, landed.reads);
    assert_eq!(built.writes.len(), landed.writes.len());
    validate_kernel_descriptor_admission(&built).expect("builder admits");
}

#[test]
fn sead_obs2_descriptor_admits_default_off_multilayer_score() {
    let desc = obs2();
    assert_eq!(desc.id, SEAD_OBS2_DESCRIPTOR_ID);
    assert!(desc.default_off);
    assert!(!desc.production_wiring);
    assert!(is_sead_obs2_multilayer_overlay_score_descriptor(&desc));
    validate_kernel_descriptor_admission(&desc).expect("obs2 admits");
    println!(
        "sead_obs2_descriptor: id={SEAD_OBS2_DESCRIPTOR_ID} label={SEAD_OBS2_LABEL} layers={SEAD_OBS2_LAYER_COUNT}"
    );
}

#[test]
fn sead_obs2_rejects_exact_score_for_f32_score_arithmetic() {
    let mut bad = sead_obs2_multilayer_overlay_score_kernel_descriptor();
    for out in &mut bad.writes {
        if out.name == "score_bits" {
            out.authority = OutputAuthority::ExactAuthoritative;
        }
    }
    assert_admission_err(
        &bad,
        "score_bits cannot be ExactAuthoritative under ApproximateDiagnosticF32",
    );
}

#[test]
fn sead_obs3_descriptor_admits_fixed_point_score() {
    let desc = obs3();
    assert_eq!(desc.id, SEAD_OBS3_DESCRIPTOR_ID);
    assert!(desc.default_off);
    assert!(!desc.production_wiring);
    assert!(is_sead_obs3_multilayer_fixed_score_descriptor(&desc));
    validate_kernel_descriptor_admission(&desc).expect("obs3 admits");

    let score_fixed = desc
        .writes
        .iter()
        .find(|out| out.name == "score_fixed")
        .expect("score_fixed");
    assert_eq!(score_fixed.authority, OutputAuthority::ExactAuthoritative);
    assert_eq!(
        desc.score_authority_contract,
        Some(ScoreAuthorityContract::ExactQ16WeightedSum)
    );

    let obs2_desc = obs2();
    assert!(is_sead_obs2_multilayer_overlay_score_descriptor(&obs2_desc));
    let obs2_score = obs2_desc
        .writes
        .iter()
        .find(|out| out.name == "score_bits")
        .expect("obs2 score_bits");
    assert_eq!(obs2_score.authority, OutputAuthority::ApproximateDiagnostic);
    println!(
        "sead_obs3_descriptor: id={SEAD_OBS3_DESCRIPTOR_ID} label={SEAD_OBS3_LABEL} score=ExactQ16WeightedSum obs2_f32=ApproximateDiagnostic"
    );
}

#[test]
fn sead_obs3_f32_score_exact_still_rejects() {
    let mut bad_obs2 = sead_obs2_multilayer_overlay_score_kernel_descriptor();
    for out in &mut bad_obs2.writes {
        if out.name == "score_bits" {
            out.authority = OutputAuthority::ExactAuthoritative;
        }
    }
    assert_admission_err(
        &bad_obs2,
        "score_bits cannot be ExactAuthoritative under ApproximateDiagnosticF32",
    );

    let mut bad_obs3_score_bits = sead_obs3_multilayer_fixed_score_kernel_descriptor();
    for out in &mut bad_obs3_score_bits.writes {
        if out.name == "score_fixed" {
            out.name = "score_bits".to_string();
        }
    }
    assert_admission_err(
        &bad_obs3_score_bits,
        "score_bits cannot be ExactAuthoritative under ExactQ16WeightedSum",
    );

    let mut native_sqrt = sead_obs3_multilayer_fixed_score_kernel_descriptor();
    native_sqrt.native_math = NativeMathClass::ApproximateJitOnly;
    assert_admission_err(&native_sqrt, "approximate native math");

    let mut wrong_mag2 = sead_obs3_multilayer_fixed_score_kernel_descriptor();
    wrong_mag2.mag2_source_contract = None;
    assert_admission_err(&wrong_mag2, "Mag2SourceContract");
}

#[test]
fn sead_obs4_descriptor_admits_threshold_event() {
    let desc = obs4();
    assert_eq!(desc.id, SEAD_OBS4_DESCRIPTOR_ID);
    assert!(desc.default_off);
    assert!(!desc.production_wiring);
    assert!(is_sead_obs4_threshold_event_descriptor(&desc));
    validate_kernel_descriptor_admission(&desc).expect("obs4 admits");
    let state = desc
        .writes
        .iter()
        .find(|out| out.name == "state_u32")
        .expect("state_u32");
    let event = desc
        .writes
        .iter()
        .find(|out| out.name == "event_code_u32")
        .expect("event_code_u32");
    assert_eq!(state.authority, OutputAuthority::ExactAuthoritative);
    assert_eq!(event.authority, OutputAuthority::ExactAuthoritative);
    assert_eq!(
        desc.score_authority_contract,
        Some(ScoreAuthorityContract::ExactQ16WeightedSum)
    );
    println!(
        "sead_obs4_descriptor: id={SEAD_OBS4_DESCRIPTOR_ID} label={SEAD_OBS4_LABEL} threshold=ExactQ16Threshold event=ExactDeterministicEventFlag"
    );
}

#[test]
fn sead_obs4_rejects_approximate_event_outputs() {
    let mut bad = sead_obs4_threshold_event_kernel_descriptor();
    for out in &mut bad.writes {
        if out.name == "event_code_u32" {
            out.authority = OutputAuthority::ApproximateDiagnostic;
        }
    }
    assert_admission_err(
        &bad,
        "SEAD threshold event requires exact-authoritative event_code_u32",
    );
}

#[test]
fn sead_event0_descriptor_admits_compaction() {
    let desc = event0();
    assert_eq!(desc.id, SEAD_EVENT0_DESCRIPTOR_ID);
    assert!(desc.default_off);
    assert!(!desc.production_wiring);
    assert!(is_sead_event0_compaction_descriptor(&desc));
    validate_kernel_descriptor_admission(&desc).expect("event0 admits");
    println!(
        "sead_event0_descriptor: id={SEAD_EVENT0_DESCRIPTOR_ID} membership=ExactAuthoritativeUnordered order=UnspecifiedAtomicOrder"
    );
}

#[test]
fn sead_event0_rejects_sqrt_artifact_binding() {
    let mut bad = sead_event0_compaction_kernel_descriptor();
    bad.exact_sqrt_artifact = Some(exact_sqrt_f_artifact_descriptor());
    assert_admission_err(&bad, "SEAD event compaction must not bind sqrt artifact");
}

#[test]
fn sead_pipe0_descriptor_admits_integrated_pipeline() {
    let desc = pipe0();
    assert_eq!(desc.id, SEAD_PIPE0_DESCRIPTOR_ID);
    assert!(desc.default_off);
    assert!(!desc.production_wiring);
    assert!(is_sead_pipe0_observer_event_pipeline_descriptor(&desc));
    validate_kernel_descriptor_admission(&desc).expect("pipe0 admits");
    assert_eq!(
        desc.score_authority_contract,
        Some(ScoreAuthorityContract::ExactQ16WeightedSum)
    );
    println!(
        "sead_pipe0_descriptor: id={SEAD_PIPE0_DESCRIPTOR_ID} membership=ExactAuthoritativeUnordered order=UnspecifiedAtomicOrder"
    );
}

#[test]
fn sead_pipe0_rejects_missing_event_record_output() {
    let mut bad = sead_pipe0_observer_event_pipeline_kernel_descriptor();
    bad.writes.retain(|out| out.name != "event_record");
    assert_admission_err(&bad, "exact-authoritative event_record");
}

#[test]
fn sead_event1_descriptor_admits_code_bucketing() {
    let desc = event1();
    assert_eq!(desc.id, SEAD_EVENT1_DESCRIPTOR_ID);
    assert!(desc.default_off);
    assert!(!desc.production_wiring);
    assert!(is_sead_event1_code_bucketing_descriptor(&desc));
    validate_kernel_descriptor_admission(&desc).expect("event1 admits");
    println!(
        "sead_event1_descriptor: id={SEAD_EVENT1_DESCRIPTOR_ID} membership=ExactAuthoritativeUnordered order=UnspecifiedAtomicOrder"
    );
}

#[test]
fn sead_event1_rejects_sqrt_artifact_binding() {
    let mut bad = sead_event1_code_bucketing_kernel_descriptor();
    bad.exact_sqrt_artifact = Some(exact_sqrt_f_artifact_descriptor());
    assert_admission_err(&bad, "must not bind sqrt artifact");
}

#[test]
fn sead_event2_descriptor_admits_bucket_reductions() {
    let desc = event2();
    assert_eq!(desc.id, SEAD_EVENT2_DESCRIPTOR_ID);
    assert!(desc.default_off);
    assert!(!desc.production_wiring);
    assert!(is_sead_event2_bucket_reductions_descriptor(&desc));
    validate_kernel_descriptor_admission(&desc).expect("event2 admits");
    println!(
        "sead_event2_descriptor: id={SEAD_EVENT2_DESCRIPTOR_ID} reductions=order-invariant order=UnspecifiedAtomicOrder"
    );
}

#[test]
fn sead_event2_rejects_missing_sum_score_output() {
    let mut bad = sead_event2_bucket_reductions_kernel_descriptor();
    bad.writes.retain(|out| out.name != "sum_score");
    assert_admission_err(&bad, "exact-authoritative sum_score");
}
