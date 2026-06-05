//! Phase M-JIT-DESC-1 — JIT kernel descriptor admission preview (spec layer).
//!
//! Formalizes exact vs approximate output authority for landed M-JIT proof kernels.
//! No production scheduler, no default wiring, no GPU runtime dispatch.

use crate::compile::jit_exact_sqrt_artifact_admission::validate_exact_sqrt_artifact_admission;
use crate::compile::jit_exact_sqrt_artifact_admission::{
    validate_mag2_source_contract, ExactPreSqrtInputContract, Mag2SourceContract,
};
use crate::error::SpecError;

const FORBIDDEN_SEMANTIC_TERMS: &[&str] = &[
    "faction",
    "ownership",
    "owner",
    "AI",
    "threat",
    "scarcity",
    "opportunity",
    "labor",
    "price",
    "logistics",
    "routing",
    "need",
    "demand",
    "supply",
    "personality",
    "drone",
    "FIELD_POLICY",
    "simthing-sim",
    "ResourceEconomySpec",
    "SimSession",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelLane {
    TestOnly,
    ProductionCandidate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputAuthority {
    ExactAuthoritative,
    ApproximateDiagnostic,
    RejectedDeferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeMathClass {
    None,
    ApproximateJitOnly,
}

#[derive(Debug, Clone)]
pub struct KernelDescriptorSpec {
    pub id: String,
    pub lane: KernelLane,
    pub reads: Vec<String>,
    pub writes: Vec<KernelOutputSpec>,
    pub native_math: NativeMathClass,
    pub semantic_free: bool,
    pub default_off: bool,
    pub production_wiring: bool,
    /// Artifact-backed exact sqrt binding (Candidate F only when hash-valid).
    pub exact_sqrt_artifact:
        Option<crate::compile::jit_exact_sqrt_artifact_admission::ExactSqrtArtifactDescriptor>,
    /// Pre-sqrt input contract for F-backed magnitude kernels (SQRT-MAG-0 R1).
    pub pre_sqrt_contract:
        Option<crate::compile::jit_exact_sqrt_artifact_admission::ExactPreSqrtInputContract>,
    /// Exact mag2 construction source contract (SQRT-MAG2-0).
    pub mag2_source_contract: Option<Mag2SourceContract>,
    /// Score output authority contract for observer overlay kernels (FIELD_POLICY-OBS-1).
    pub score_authority_contract:
        Option<crate::compile::jit_exact_sqrt_artifact_admission::ScoreAuthorityContract>,
}

#[derive(Debug, Clone)]
pub struct KernelOutputSpec {
    pub name: String,
    pub authority: OutputAuthority,
}

fn admission_err(kernel: &str, reason: impl Into<String>) -> SpecError {
    SpecError::JitKernelDescriptorAdmission {
        kernel: kernel.to_string(),
        reason: reason.into(),
    }
}

fn contains_forbidden_semantic_term(name: &str) -> Option<&'static str> {
    FORBIDDEN_SEMANTIC_TERMS
        .iter()
        .find(|term| name.contains(**term))
        .copied()
}

fn output_authority(producer: &KernelDescriptorSpec, name: &str) -> Option<OutputAuthority> {
    producer
        .writes
        .iter()
        .find(|out| out.name == name)
        .map(|out| out.authority)
}

/// Admit a kernel descriptor under DESC-1 preview policy.
pub fn validate_kernel_descriptor_admission(spec: &KernelDescriptorSpec) -> Result<(), SpecError> {
    if spec.production_wiring {
        return Err(admission_err(
            &spec.id,
            "production_wiring must remain false in DESC-1 preview",
        ));
    }

    if !spec.default_off {
        return Err(admission_err(
            &spec.id,
            "default_off must remain true in DESC-1 preview",
        ));
    }

    if !spec.semantic_free {
        return Err(admission_err(
            &spec.id,
            "semantic_free must remain true in DESC-1 preview",
        ));
    }

    if spec.lane == KernelLane::ProductionCandidate {
        return Err(admission_err(
            &spec.id,
            "ProductionCandidate lane requires a separate production registry gate",
        ));
    }

    if let Some(term) = contains_forbidden_semantic_term(&spec.id) {
        return Err(admission_err(
            &spec.id,
            format!("descriptor id contains forbidden semantic term `{term}`"),
        ));
    }

    for read in &spec.reads {
        if let Some(term) = contains_forbidden_semantic_term(read) {
            return Err(admission_err(
                &spec.id,
                format!("read name `{read}` contains forbidden semantic term `{term}`"),
            ));
        }
    }

    if spec.writes.is_empty() {
        return Err(admission_err(
            &spec.id,
            "descriptor must declare at least one output",
        ));
    }

    let mut seen_outputs = std::collections::HashSet::new();
    for out in &spec.writes {
        if let Some(term) = contains_forbidden_semantic_term(&out.name) {
            return Err(admission_err(
                &spec.id,
                format!(
                    "output name `{}` contains forbidden semantic term `{term}`",
                    out.name
                ),
            ));
        }
        if !seen_outputs.insert(out.name.clone()) {
            return Err(admission_err(
                &spec.id,
                format!("duplicate output name `{}`", out.name),
            ));
        }
    }

    if spec.native_math == NativeMathClass::ApproximateJitOnly {
        for out in &spec.writes {
            if out.authority == OutputAuthority::ExactAuthoritative {
                return Err(admission_err(
                    &spec.id,
                    format!(
                        "approximate native math cannot claim exact-authoritative output `{}`",
                        out.name
                    ),
                ));
            }
        }
    }

    validate_exact_sqrt_artifact_admission(spec)?;

    Ok(())
}

/// Validate that required inputs are exact-authoritative outputs of the producer.
pub fn validate_exact_kernel_inputs(
    producer: &KernelDescriptorSpec,
    required_exact_inputs: &[&str],
) -> Result<(), SpecError> {
    for name in required_exact_inputs {
        if *name == "sqrt_out" || *name == "mag" {
            validate_exact_sqrt_artifact_admission(producer)?;
            if *name == "mag"
                && producer.pre_sqrt_contract != Some(ExactPreSqrtInputContract::ExactMag2Bits)
            {
                return Err(admission_err(
                    &producer.id,
                    "exact-authoritative mag requires ExactMag2Bits pre-sqrt contract",
                ));
            }
        }
        if *name == "mag2" || *name == "mag2_bits" {
            let out_name = if *name == "mag2_bits" {
                "mag2_bits"
            } else {
                "mag2"
            };
            match output_authority(producer, out_name) {
                Some(OutputAuthority::ExactAuthoritative) => {
                    validate_mag2_source_contract(producer)?;
                }
                Some(OutputAuthority::ApproximateDiagnostic) => {
                    return Err(admission_err(
                        &producer.id,
                        format!("output `{out_name}` is approximate/diagnostic, not exact-authoritative"),
                    ));
                }
                Some(OutputAuthority::RejectedDeferred) => {
                    return Err(admission_err(
                        &producer.id,
                        format!("output `{out_name}` is rejected/deferred"),
                    ));
                }
                None => {
                    return Err(admission_err(
                        &producer.id,
                        format!("output `{out_name}` not produced by kernel"),
                    ));
                }
            }
        }
        match output_authority(producer, name) {
            Some(OutputAuthority::ExactAuthoritative) => {}
            Some(OutputAuthority::ApproximateDiagnostic) => {
                return Err(admission_err(
                    &producer.id,
                    format!("output `{name}` is approximate/diagnostic, not exact-authoritative"),
                ));
            }
            Some(OutputAuthority::RejectedDeferred) => {
                return Err(admission_err(
                    &producer.id,
                    format!("output `{name}` is rejected/deferred"),
                ));
            }
            None => {
                return Err(admission_err(
                    &producer.id,
                    format!("output `{name}` not produced by kernel"),
                ));
            }
        }
    }
    Ok(())
}

fn exact_out(name: &str) -> KernelOutputSpec {
    KernelOutputSpec {
        name: name.to_string(),
        authority: OutputAuthority::ExactAuthoritative,
    }
}

fn approx_out(name: &str) -> KernelOutputSpec {
    KernelOutputSpec {
        name: name.to_string(),
        authority: OutputAuthority::ApproximateDiagnostic,
    }
}

fn test_only_descriptor(
    id: &str,
    reads: &[&str],
    writes: Vec<KernelOutputSpec>,
    native_math: NativeMathClass,
) -> KernelDescriptorSpec {
    KernelDescriptorSpec {
        id: id.to_string(),
        lane: KernelLane::TestOnly,
        reads: reads.iter().map(|name| (*name).to_string()).collect(),
        writes,
        native_math,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
        exact_sqrt_artifact: None,
        pre_sqrt_contract: None,
        mag2_source_contract: None,
        score_authority_contract: None,
    }
}

/// Landed M-JIT proof kernel descriptors mirrored from DESC-0 evidence.
pub fn landed_jit_kernel_descriptors() -> Vec<KernelDescriptorSpec> {
    vec![
        test_only_descriptor(
            "m_jit_0_weighted_accumulator",
            &["values"],
            vec![exact_out("out_col")],
            NativeMathClass::None,
        ),
        test_only_descriptor(
            "m_jit_0_ema",
            &["values"],
            vec![exact_out("out_col")],
            NativeMathClass::None,
        ),
        test_only_descriptor(
            "m_jit_sqrt_0_candidate",
            &["values"],
            vec![approx_out("sqrt_out"), approx_out("magnitude_out")],
            NativeMathClass::ApproximateJitOnly,
        ),
        test_only_descriptor(
            "m_jit_grad_0_observer",
            &["fields", "observers"],
            vec![
                exact_out("dx"),
                exact_out("dy"),
                approx_out("mag2"),
                exact_out("descent_x"),
                exact_out("descent_y"),
            ],
            NativeMathClass::None,
        ),
        test_only_descriptor(
            "m_jit_grad_1_observer_score",
            &["fields", "observers"],
            vec![
                exact_out("dx"),
                exact_out("dy"),
                exact_out("descent_x"),
                exact_out("descent_y"),
                exact_out("score"),
            ],
            NativeMathClass::None,
        ),
        crate::compile::jit_exact_sqrt_artifact_admission::sqrt_f_exact_kernel_descriptor(),
        crate::compile::jit_exact_sqrt_artifact_admission::mag_f_from_exact_mag2_kernel_descriptor(),
        crate::compile::jit_exact_sqrt_artifact_admission::mag_f_from_dxdy_probe_kernel_descriptor(),
        crate::compile::jit_exact_sqrt_artifact_admission::mag2_fixed_exact_kernel_descriptor(),
        crate::compile::jit_exact_sqrt_artifact_admission::field_policy_obs0_overlay_score_kernel_descriptor(),
        crate::compile::jit_exact_sqrt_artifact_admission::field_policy_obs2_multilayer_overlay_score_kernel_descriptor(),
        crate::compile::jit_exact_sqrt_artifact_admission::field_policy_obs3_multilayer_fixed_score_kernel_descriptor(),
        crate::compile::jit_exact_sqrt_artifact_admission::field_policy_obs4_threshold_event_kernel_descriptor(),
        crate::compile::jit_exact_sqrt_artifact_admission::field_policy_event0_compaction_kernel_descriptor(),
        crate::compile::jit_exact_sqrt_artifact_admission::field_policy_pipe0_observer_event_pipeline_kernel_descriptor(),
        crate::compile::jit_exact_sqrt_artifact_admission::field_policy_event1_code_bucketing_kernel_descriptor(),
        crate::compile::jit_exact_sqrt_artifact_admission::field_policy_event2_bucket_reductions_kernel_descriptor(),
        crate::compile::jit_exact_sqrt_artifact_admission::field_policy_act0_numeric_proposals_kernel_descriptor(),
        crate::compile::jit_exact_sqrt_artifact_admission::field_policy_act1_phase_e_proposal_consumer_kernel_descriptor(),
        crate::compile::jit_exact_sqrt_artifact_admission::field_policy_act2_proposal_admission_records_kernel_descriptor(),
        crate::compile::jit_exact_sqrt_artifact_admission::field_policy_act3_economic_fixture_records_kernel_descriptor(),
    ]
}
