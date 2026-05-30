//! SQRT-PROMOTE-0 — Artifact-backed Candidate F exact sqrt descriptor/admission (spec layer).
//!
//! Pins exact sqrt authority to the proven F WGSL artifact identity. No WGSL source text
//! is accepted dynamically; admission binds to path/hash/entrypoint/proof metadata only.

use crate::compile::jit_kernel_descriptor_admission::{
    KernelDescriptorSpec, KernelLane, KernelOutputSpec, NativeMathClass, OutputAuthority,
};
use crate::error::SpecError;

/// Exact sqrt authority class for artifact-backed kernels (maps to `OutputAuthority::ExactAuthoritative`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExactSqrtAuthorityClass {
    ExactDeterministic,
}

/// Pinned artifact identity for Candidate F exact sqrt (SQRT-EXACT-5F proof).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactSqrtArtifactDescriptor {
    pub artifact_path: String,
    pub artifact_hash_fnv1a64: String,
    pub entrypoint: String,
    pub io_contract: String,
    pub proof_report: String,
    pub domain: String,
    pub authority_class: ExactSqrtAuthorityClass,
}

pub const SQRT_F_DESCRIPTOR_ID: &str = "m_jit_sqrt_f_exact";
pub const SQRT_F_ARTIFACT_PATH: &str =
    "crates/simthing-driver/tests/wgsl/sqrt_cr_f_candidate.wgsl";
pub const SQRT_F_ARTIFACT_HASH: &str = "e2e9e27601ee2e13";
pub const SQRT_F_ENTRYPOINT: &str = "sqrt_cr_f_bits";
pub const SQRT_F_IO_CONTRACT: &str = "u32_bit_io";
pub const SQRT_F_PROOF_REPORT: &str =
    "docs/tests/phase_m_jit_sqrt_exact5f_exhaustive_sweep_results.md";
pub const SQRT_F_DOMAIN: &str = "0x0000_0000..=0x7F7F_FFFF";

fn artifact_err(kernel: &str, reason: impl Into<String>) -> SpecError {
    SpecError::JitKernelDescriptorAdmission {
        kernel: kernel.to_string(),
        reason: reason.into(),
    }
}

/// Return the pinned Candidate F artifact descriptor metadata.
pub fn exact_sqrt_f_artifact_descriptor() -> ExactSqrtArtifactDescriptor {
    ExactSqrtArtifactDescriptor {
        artifact_path: SQRT_F_ARTIFACT_PATH.to_string(),
        artifact_hash_fnv1a64: SQRT_F_ARTIFACT_HASH.to_string(),
        entrypoint: SQRT_F_ENTRYPOINT.to_string(),
        io_contract: SQRT_F_IO_CONTRACT.to_string(),
        proof_report: SQRT_F_PROOF_REPORT.to_string(),
        domain: SQRT_F_DOMAIN.to_string(),
        authority_class: ExactSqrtAuthorityClass::ExactDeterministic,
    }
}

/// FNV-1a 64-bit hash as lowercase hex (matches driver battery and graph identity).
pub fn fnv1a64_hex(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

/// True when the descriptor is the landed artifact-backed Candidate F exact sqrt kernel.
pub fn is_exact_sqrt_f_descriptor(spec: &KernelDescriptorSpec) -> bool {
    spec.id == SQRT_F_DESCRIPTOR_ID && spec.exact_sqrt_artifact.is_some()
}

/// Validate pinned Candidate F artifact binding fields.
pub fn validate_exact_sqrt_artifact_binding(
    kernel_id: &str,
    binding: &ExactSqrtArtifactDescriptor,
) -> Result<(), SpecError> {
    let expected = exact_sqrt_f_artifact_descriptor();

    if binding.authority_class != ExactSqrtAuthorityClass::ExactDeterministic {
        return Err(artifact_err(
            kernel_id,
            "exact sqrt artifact authority_class must be ExactDeterministic",
        ));
    }

    if binding.artifact_path != expected.artifact_path {
        return Err(artifact_err(
            kernel_id,
            format!(
                "exact sqrt artifact path mismatch: expected `{}`, got `{}`",
                expected.artifact_path, binding.artifact_path
            ),
        ));
    }

    if binding.artifact_hash_fnv1a64.is_empty() {
        return Err(artifact_err(
            kernel_id,
            "exact sqrt artifact hash must not be empty",
        ));
    }

    if binding.artifact_hash_fnv1a64 != expected.artifact_hash_fnv1a64 {
        return Err(artifact_err(
            kernel_id,
            format!(
                "exact sqrt artifact hash mismatch: expected `{}`, got `{}`",
                expected.artifact_hash_fnv1a64, binding.artifact_hash_fnv1a64
            ),
        ));
    }

    if binding.entrypoint != expected.entrypoint {
        return Err(artifact_err(
            kernel_id,
            format!(
                "exact sqrt artifact entrypoint mismatch: expected `{}`, got `{}`",
                expected.entrypoint, binding.entrypoint
            ),
        ));
    }

    if binding.io_contract != expected.io_contract {
        return Err(artifact_err(
            kernel_id,
            format!(
                "exact sqrt artifact io_contract mismatch: expected `{}`, got `{}`",
                expected.io_contract, binding.io_contract
            ),
        ));
    }

    if binding.proof_report.is_empty() {
        return Err(artifact_err(
            kernel_id,
            "exact sqrt artifact proof_report must not be empty",
        ));
    }

    if binding.proof_report != expected.proof_report {
        return Err(artifact_err(
            kernel_id,
            format!(
                "exact sqrt artifact proof_report mismatch: expected `{}`, got `{}`",
                expected.proof_report, binding.proof_report
            ),
        ));
    }

    if binding.domain != expected.domain {
        return Err(artifact_err(
            kernel_id,
            format!(
                "exact sqrt artifact domain mismatch: expected `{}`, got `{}`",
                expected.domain, binding.domain
            ),
        ));
    }

    Ok(())
}

/// Validate artifact-backed exact sqrt admission rules for a kernel descriptor.
pub fn validate_exact_sqrt_artifact_admission(spec: &KernelDescriptorSpec) -> Result<(), SpecError> {
    let has_exact_sqrt_out = spec.writes.iter().any(|out| {
        out.name == "sqrt_out" && out.authority == OutputAuthority::ExactAuthoritative
    });

    match (&spec.exact_sqrt_artifact, has_exact_sqrt_out) {
        (None, true) => {
            return Err(artifact_err(
                &spec.id,
                "exact-authoritative sqrt_out requires artifact-backed Candidate F binding",
            ));
        }
        (Some(binding), _) => {
            validate_exact_sqrt_artifact_binding(&spec.id, binding)?;
            if spec.native_math == NativeMathClass::ApproximateJitOnly {
                return Err(artifact_err(
                    &spec.id,
                    "artifact-backed exact sqrt cannot use ApproximateJitOnly native math",
                ));
            }
            if !has_exact_sqrt_out {
                return Err(artifact_err(
                    &spec.id,
                    "artifact-backed exact sqrt descriptor must declare exact-authoritative sqrt_out",
                ));
            }
        }
        (None, false) => {}
    }

    Ok(())
}

fn exact_out(name: &str) -> KernelOutputSpec {
    KernelOutputSpec {
        name: name.to_string(),
        authority: OutputAuthority::ExactAuthoritative,
    }
}

/// Build the landed artifact-backed Candidate F exact sqrt kernel descriptor.
pub fn sqrt_f_exact_kernel_descriptor() -> KernelDescriptorSpec {
    KernelDescriptorSpec {
        id: SQRT_F_DESCRIPTOR_ID.to_string(),
        lane: KernelLane::TestOnly,
        reads: vec!["values".to_string()],
        writes: vec![exact_out("sqrt_out")],
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
        exact_sqrt_artifact: Some(exact_sqrt_f_artifact_descriptor()),
    }
}
