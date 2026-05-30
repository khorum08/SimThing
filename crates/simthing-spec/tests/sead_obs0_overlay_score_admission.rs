//! SEAD-OBS-1 — Descriptor/admission for mobile observer overlay score.

use simthing_spec::{
    exact_sqrt_f_artifact_descriptor, is_sead_obs0_overlay_score_descriptor,
    landed_jit_kernel_descriptors, sead_obs0_overlay_score_kernel_descriptor,
    validate_kernel_descriptor_admission, ExactPreSqrtInputContract, KernelDescriptorSpec,
    Mag2SourceContract, NativeMathClass, OutputAuthority, ScoreAuthorityContract, SEAD_OBS0_DESCRIPTOR_ID,
    SEAD_OBS0_LABEL, SpecError, MAG2_Q16_FRAC_BITS, SQRT_F_ARTIFACT_HASH,
};

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
