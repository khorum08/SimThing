//! Phase M-JIT-DESC-1 — JIT kernel descriptor admission preview (spec layer).
//!
//! Formalizes exact vs approximate output authority for landed M-JIT proof kernels.
//! No production scheduler, no default wiring, no GPU runtime dispatch.

use crate::compile::jit_exact_sqrt_artifact_admission::validate_exact_sqrt_artifact_admission;
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
    "SEAD",
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
    pub exact_sqrt_artifact: Option<crate::compile::jit_exact_sqrt_artifact_admission::ExactSqrtArtifactDescriptor>,
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
        return Err(admission_err(&spec.id, "descriptor must declare at least one output"));
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
        if *name == "sqrt_out" {
            validate_exact_sqrt_artifact_admission(producer)?;
        }
        match output_authority(producer, name) {
            Some(OutputAuthority::ExactAuthoritative) => {}
            Some(OutputAuthority::ApproximateDiagnostic) => {
                return Err(admission_err(
                    &producer.id,
                    format!(
                        "output `{name}` is approximate/diagnostic, not exact-authoritative"
                    ),
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
    ]
}
