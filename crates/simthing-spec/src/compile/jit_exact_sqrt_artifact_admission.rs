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

pub const MAG_F_FROM_MAG2_DESCRIPTOR_ID: &str = "m_jit_mag_f_from_exact_mag2";
pub const MAG_F_FROM_DXDY_PROBE_DESCRIPTOR_ID: &str = "m_jit_mag_f_from_dxdy_probe";
pub const MAG_F_FROM_MAG2_LABEL: &str = "ExactEuclideanMagnitudeFFromExactMag2";
pub const MAG_F_FROM_DXDY_PROBE_LABEL: &str = "RawDxDyMagnitudeFProbe";

pub const MAG2_FIXED_DESCRIPTOR_ID: &str = "m_jit_mag2_fixed_exact";
pub const MAG2_FIXED_LABEL: &str = "ExactFixedPointMag2";
/// Q16.16 fixed-point fraction bits for SEAD gradient magnitude probe.
pub const MAG2_Q16_FRAC_BITS: u32 = 16;
pub const MAG2_Q16_SCALE: u32 = 1 << MAG2_Q16_FRAC_BITS;
pub const MAG2_Q16_SCALE_SQ: u64 = (MAG2_Q16_SCALE as u64) * (MAG2_Q16_SCALE as u64);
/// Bounded SEAD gradient component range (±16.0) under Q16.16.
pub const MAG2_Q16_COMPONENT_MAX: f32 = 16.0;

/// Source contract for exact-authoritative pre-sqrt mag2 construction (SQRT-MAG2-0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mag2SourceContract {
    /// Signed fixed-point `dx_fixed`/`dy_fixed` with integer `dx²+dy²` then pinned f32 conversion.
    ExactFixedPointDxDy { fraction_bits: u32 },
}

/// Pre-sqrt input contract for F-backed magnitude kernels (SQRT-MAG-0 R1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExactPreSqrtInputContract {
    /// F sqrt over already exact-authoritative non-negative `mag2` bits.
    ExactMag2Bits,
    /// Raw `dx`/`dy` f32 multiply-add benchmark probe — not exact-authoritative `mag`.
    RawDxDyProbe,
    /// Inline fixed-point gx/gy → exact mag2 → F sqrt (SEAD-OBS-1).
    InlineFixedPointMag2Sqrt,
}

/// Score output authority contract for observer overlay kernels (SEAD-OBS-1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreAuthorityContract {
    /// `score = bias + weight * mag` via f32 multiply/add — diagnostic only.
    ApproximateDiagnosticF32,
    /// `score_fixed = bias + Σ q16_mul(weight, mag_fixed)` — pinned Q16.16 accumulation (SEAD-OBS-3).
    ExactQ16WeightedSum,
}

pub const SEAD_OBS0_DESCRIPTOR_ID: &str = "m_jit_sead_obs0_overlay_score";
pub const SEAD_OBS0_LABEL: &str = "SeadObserverOverlayScore";

pub const SEAD_OBS2_DESCRIPTOR_ID: &str = "m_jit_sead_obs2_multilayer_overlay_score";
pub const SEAD_OBS2_LABEL: &str = "SeadMultilayerOverlayScore";
pub const SEAD_OBS2_LAYER_COUNT: u32 = 4;

pub const SEAD_OBS3_DESCRIPTOR_ID: &str = "m_jit_sead_obs3_multilayer_fixed_score";
pub const SEAD_OBS3_LABEL: &str = "SeadMultilayerFixedScore";
pub const SEAD_OBS3_LAYER_COUNT: u32 = SEAD_OBS2_LAYER_COUNT;

pub const SEAD_OBS4_DESCRIPTOR_ID: &str = "m_jit_sead_obs4_threshold_event";
pub const SEAD_OBS4_LABEL: &str = "SeadThresholdEvent";
pub const SEAD_OBS4_LAYER_COUNT: u32 = SEAD_OBS3_LAYER_COUNT;

/// Threshold operand contract for observer event kernels (SEAD-OBS-4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdAuthorityContract {
    /// `threshold_fixed` / `hysteresis_fixed` are Q16.16 signed integers compared to `score_fixed`.
    ExactQ16Threshold,
}

/// Event output authority contract for observer event kernels (SEAD-OBS-4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventAuthorityContract {
    /// `state_u32` / `event_code_u32` are exact deterministic under fixed score + threshold operands.
    ExactDeterministicEventFlag,
}

pub const SEAD_EVENT0_DESCRIPTOR_ID: &str = "m_jit_sead_event0_compaction";
pub const SEAD_EVENT0_LABEL: &str = "SeadEventCompaction";

pub const SEAD_PIPE0_DESCRIPTOR_ID: &str = "m_jit_sead_pipe0_observer_event_pipeline";
pub const SEAD_PIPE0_LABEL: &str = "SeadObserverEventPipeline";
pub const SEAD_PIPE0_LAYER_COUNT: u32 = SEAD_OBS4_LAYER_COUNT;

pub const SEAD_EVENT1_DESCRIPTOR_ID: &str = "m_jit_sead_event1_code_bucketing";
pub const SEAD_EVENT1_LABEL: &str = "SeadEventCodeBucketing";
pub const SEAD_EVENT1_CODE_COUNT: u32 = 4;

pub const SEAD_EVENT2_DESCRIPTOR_ID: &str = "m_jit_sead_event2_bucket_reductions";
pub const SEAD_EVENT2_LABEL: &str = "SeadBucketReductions";
pub const SEAD_EVENT2_CODE_COUNT: u32 = SEAD_EVENT1_CODE_COUNT;

/// Compacted event membership authority (SEAD-EVENT-0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventCompactionMembershipAuthority {
    /// Event records form an exact multiset under capacity; order is not specified.
    ExactAuthoritativeUnordered,
}

/// Compacted event ordering authority (SEAD-EVENT-0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventCompactionOrderAuthority {
    /// Atomic slot assignment does not guarantee cross-workgroup order.
    UnspecifiedAtomicOrder,
}

/// Event-code bucket membership authority (SEAD-EVENT-1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventCodeBucketMembershipAuthority {
    /// Bucket records form an exact multiset per code under capacity; order is not specified.
    ExactAuthoritativeUnordered,
}

/// Event-code bucket ordering authority (SEAD-EVENT-1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventCodeBucketOrderAuthority {
    /// Atomic slot assignment does not guarantee order within a bucket.
    UnspecifiedAtomicOrder,
}

/// Event-bucket reduction membership authority (SEAD-EVENT-2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventBucketReductionInputAuthority {
    /// Reduction input records form an exact multiset per code under capacity.
    ExactAuthoritativeUnordered,
}

/// Event-bucket reduction ordering authority (SEAD-EVENT-2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventBucketReductionOrderAuthority {
    /// Reductions are order-invariant; bucket record order remains unspecified.
    UnspecifiedAtomicOrder,
}

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

/// True when the descriptor is the landed F-backed exact magnitude-from-mag2 kernel.
pub fn is_exact_mag_f_from_mag2_descriptor(spec: &KernelDescriptorSpec) -> bool {
    spec.id == MAG_F_FROM_MAG2_DESCRIPTOR_ID
        && spec.exact_sqrt_artifact.is_some()
        && spec.pre_sqrt_contract == Some(ExactPreSqrtInputContract::ExactMag2Bits)
}

/// True when the descriptor is the raw dx/dy magnitude probe (non-exact mag output).
pub fn is_mag_f_dxdy_probe_descriptor(spec: &KernelDescriptorSpec) -> bool {
    spec.id == MAG_F_FROM_DXDY_PROBE_DESCRIPTOR_ID
        && spec.pre_sqrt_contract == Some(ExactPreSqrtInputContract::RawDxDyProbe)
}

/// True when the descriptor is the landed exact fixed-point mag2 construction kernel.
pub fn is_exact_mag2_fixed_descriptor(spec: &KernelDescriptorSpec) -> bool {
    spec.id == MAG2_FIXED_DESCRIPTOR_ID
        && spec.mag2_source_contract
            == Some(Mag2SourceContract::ExactFixedPointDxDy {
                fraction_bits: MAG2_Q16_FRAC_BITS,
            })
}

/// True when the descriptor is the landed SEAD observer overlay score kernel.
pub fn is_sead_obs0_overlay_score_descriptor(spec: &KernelDescriptorSpec) -> bool {
    spec.id == SEAD_OBS0_DESCRIPTOR_ID
        && spec.pre_sqrt_contract == Some(ExactPreSqrtInputContract::InlineFixedPointMag2Sqrt)
        && spec.score_authority_contract == Some(ScoreAuthorityContract::ApproximateDiagnosticF32)
}

/// True when the descriptor is the landed SEAD multilayer overlay score kernel.
pub fn is_sead_obs2_multilayer_overlay_score_descriptor(spec: &KernelDescriptorSpec) -> bool {
    spec.id == SEAD_OBS2_DESCRIPTOR_ID
        && spec.pre_sqrt_contract == Some(ExactPreSqrtInputContract::InlineFixedPointMag2Sqrt)
        && spec.score_authority_contract == Some(ScoreAuthorityContract::ApproximateDiagnosticF32)
        && multilayer_exact_mag_bits_outputs(spec) == SEAD_OBS2_LAYER_COUNT
}

/// True when the descriptor is the landed SEAD multilayer fixed-point score kernel.
pub fn is_sead_obs3_multilayer_fixed_score_descriptor(spec: &KernelDescriptorSpec) -> bool {
    spec.id == SEAD_OBS3_DESCRIPTOR_ID
        && spec.pre_sqrt_contract == Some(ExactPreSqrtInputContract::InlineFixedPointMag2Sqrt)
        && spec.score_authority_contract == Some(ScoreAuthorityContract::ExactQ16WeightedSum)
        && multilayer_exact_mag_bits_outputs(spec) == SEAD_OBS3_LAYER_COUNT
        && spec
            .writes
            .iter()
            .any(|out| out.name == "score_fixed" && out.authority == OutputAuthority::ExactAuthoritative)
}

/// True when the descriptor is the landed SEAD threshold event kernel.
pub fn is_sead_obs4_threshold_event_descriptor(spec: &KernelDescriptorSpec) -> bool {
    spec.id == SEAD_OBS4_DESCRIPTOR_ID
        && spec.pre_sqrt_contract == Some(ExactPreSqrtInputContract::InlineFixedPointMag2Sqrt)
        && spec.score_authority_contract == Some(ScoreAuthorityContract::ExactQ16WeightedSum)
        && multilayer_exact_mag_bits_outputs(spec) == SEAD_OBS4_LAYER_COUNT
        && has_threshold_event_reads(spec)
        && spec.writes.iter().any(|out| {
            out.name == "state_u32" && out.authority == OutputAuthority::ExactAuthoritative
        })
        && spec.writes.iter().any(|out| {
            out.name == "event_code_u32" && out.authority == OutputAuthority::ExactAuthoritative
        })
}

fn is_multilayer_overlay_descriptor_id(id: &str) -> bool {
    id == SEAD_OBS2_DESCRIPTOR_ID || id == SEAD_OBS3_DESCRIPTOR_ID || id == SEAD_OBS4_DESCRIPTOR_ID
}

fn has_threshold_event_reads(spec: &KernelDescriptorSpec) -> bool {
    spec.reads.iter().any(|read| read == "threshold_fixed")
        && spec.reads.iter().any(|read| read == "hysteresis_fixed")
        && spec.reads.iter().any(|read| read == "prior_state_u32")
}

fn multilayer_exact_mag_bits_outputs(spec: &KernelDescriptorSpec) -> u32 {
    (0..SEAD_OBS2_LAYER_COUNT)
        .filter(|layer| {
            spec.writes.iter().any(|out| {
                out.name == format!("layer{layer}_mag_bits")
                    && out.authority == OutputAuthority::ExactAuthoritative
            })
        })
        .count() as u32
}

fn has_multilayer_fixed_reads(spec: &KernelDescriptorSpec) -> bool {
    (0..SEAD_OBS2_LAYER_COUNT).all(|layer| {
        spec.reads.iter().any(|read| read == &format!("layer{layer}_gx_fixed"))
            && spec.reads.iter().any(|read| read == &format!("layer{layer}_gy_fixed"))
            && spec.reads
                .iter()
                .any(|read| read == &format!("layer{layer}_w_fixed"))
    }) && spec.reads.iter().any(|read| read == "bias_fixed")
}

fn has_fixed_point_gradient_reads(spec: &KernelDescriptorSpec) -> bool {
    (spec.reads.iter().any(|read| read == "dx_fixed")
        && spec.reads.iter().any(|read| read == "dy_fixed"))
        || (spec.reads.iter().any(|read| read == "gx_fixed")
            && spec.reads.iter().any(|read| read == "gy_fixed"))
}

fn has_exact_mag2_bits_output(spec: &KernelDescriptorSpec) -> bool {
    spec.writes.iter().any(|out| {
        (out.name == "mag2_bits" || out.name == "mag2")
            && out.authority == OutputAuthority::ExactAuthoritative
    })
}

fn has_exact_mag_bits_output(spec: &KernelDescriptorSpec) -> bool {
    spec.writes.iter().any(|out| {
        (out.name == "mag" || out.name == "mag_bits")
            && out.authority == OutputAuthority::ExactAuthoritative
    })
}

fn has_exact_f_authoritative_output(spec: &KernelDescriptorSpec) -> bool {
    spec.writes.iter().any(|out| {
        out.authority == OutputAuthority::ExactAuthoritative
            && (out.name == "sqrt_out"
                || out.name == "mag"
                || out.name == "mag_bits"
                || (out.name.starts_with("layer") && out.name.ends_with("_mag_bits")))
    })
}

/// Validate pre-sqrt contract rules (SQRT-MAG-0 R1).
pub fn validate_exact_pre_sqrt_contract(spec: &KernelDescriptorSpec) -> Result<(), SpecError> {
    match spec.pre_sqrt_contract {
        Some(ExactPreSqrtInputContract::ExactMag2Bits) => {
            if !spec.reads.iter().any(|read| read == "mag2") {
                return Err(artifact_err(
                    &spec.id,
                    "ExactMag2Bits contract requires reading exact-authoritative mag2 input",
                ));
            }
            if spec.exact_sqrt_artifact.is_none() {
                return Err(artifact_err(
                    &spec.id,
                    "ExactMag2Bits magnitude requires artifact-backed Candidate F binding",
                ));
            }
            match spec
                .writes
                .iter()
                .find(|out| out.name == "mag")
                .map(|out| out.authority)
            {
                Some(OutputAuthority::ExactAuthoritative) => {}
                _ => {
                    return Err(artifact_err(
                        &spec.id,
                        "ExactMag2Bits contract requires exact-authoritative mag output",
                    ));
                }
            }
        }
        Some(ExactPreSqrtInputContract::RawDxDyProbe) => {
            if !spec.reads.iter().any(|read| read == "dx")
                || !spec.reads.iter().any(|read| read == "dy")
            {
                return Err(artifact_err(
                    &spec.id,
                    "RawDxDyProbe contract requires dx and dy reads",
                ));
            }
            if spec
                .writes
                .iter()
                .any(|out| out.name == "mag" && out.authority == OutputAuthority::ExactAuthoritative)
            {
                return Err(artifact_err(
                    &spec.id,
                    "raw dx/dy multiply-add probe cannot claim exact-authoritative mag output",
                ));
            }
            match spec
                .writes
                .iter()
                .find(|out| out.name == "mag")
                .map(|out| out.authority)
            {
                Some(OutputAuthority::ApproximateDiagnostic) => {}
                _ => {
                    return Err(artifact_err(
                        &spec.id,
                        "RawDxDyProbe contract requires approximate/diagnostic mag output",
                    ));
                }
            }
        }
        Some(ExactPreSqrtInputContract::InlineFixedPointMag2Sqrt) => {
            if spec.mag2_source_contract.is_none() {
                return Err(artifact_err(
                    &spec.id,
                    "InlineFixedPointMag2Sqrt requires Mag2SourceContract",
                ));
            }
            if !has_fixed_point_gradient_reads(spec) && !has_multilayer_fixed_reads(spec) {
                return Err(artifact_err(
                    &spec.id,
                    "InlineFixedPointMag2Sqrt requires gx_fixed/gy_fixed, dx_fixed/dy_fixed, or multilayer fixed reads",
                ));
            }
            if spec.exact_sqrt_artifact.is_none() {
                return Err(artifact_err(
                    &spec.id,
                    "InlineFixedPointMag2Sqrt requires artifact-backed Candidate F binding",
                ));
            }
            let multilayer = multilayer_exact_mag_bits_outputs(spec) == SEAD_OBS2_LAYER_COUNT;
            if !has_exact_mag2_bits_output(spec) && !multilayer {
                return Err(artifact_err(
                    &spec.id,
                    "InlineFixedPointMag2Sqrt requires exact-authoritative mag2_bits output",
                ));
            }
            if !has_exact_mag_bits_output(spec) && !multilayer {
                return Err(artifact_err(
                    &spec.id,
                    "InlineFixedPointMag2Sqrt requires exact-authoritative mag_bits output",
                ));
            }
            if multilayer && multilayer_exact_mag_bits_outputs(spec) != SEAD_OBS2_LAYER_COUNT {
                return Err(artifact_err(
                    &spec.id,
                    "multilayer InlineFixedPointMag2Sqrt requires all layer mag_bits outputs",
                ));
            }
        }
        None => {
            if spec.writes.iter().any(|out| {
                (out.name == "mag" || out.name == "mag_bits")
                    && out.authority == OutputAuthority::ExactAuthoritative
            }) {
                return Err(artifact_err(
                    &spec.id,
                    "exact-authoritative mag requires an ExactPreSqrtInputContract",
                ));
            }
        }
    }
    Ok(())
}

/// Validate exact mag2 source contract rules (SQRT-MAG2-0).
pub fn validate_mag2_source_contract(spec: &KernelDescriptorSpec) -> Result<(), SpecError> {
    let exact_mag2_out = spec.writes.iter().any(|out| {
        (out.name == "mag2_bits" || out.name == "mag2")
            && out.authority == OutputAuthority::ExactAuthoritative
    });

    match (&spec.mag2_source_contract, exact_mag2_out) {
        (Some(Mag2SourceContract::ExactFixedPointDxDy { fraction_bits }), true) => {
            if *fraction_bits == 0 {
                return Err(artifact_err(
                    &spec.id,
                    "ExactFixedPointDxDy fraction_bits must be non-zero",
                ));
            }
            if !has_fixed_point_gradient_reads(spec) {
                return Err(artifact_err(
                    &spec.id,
                    "ExactFixedPointDxDy contract requires dx_fixed/dy_fixed or gx_fixed/gy_fixed reads",
                ));
            }
            if spec.reads.iter().any(|read| read == "dx" || read == "dy") {
                return Err(artifact_err(
                    &spec.id,
                    "exact fixed-point mag2 cannot read raw f32 dx/dy",
                ));
            }
            match spec
                .writes
                .iter()
                .find(|out| out.name == "mag2_bits" || out.name == "mag2")
                .map(|out| out.authority)
            {
                Some(OutputAuthority::ExactAuthoritative) => {}
                _ => {
                    return Err(artifact_err(
                        &spec.id,
                        "ExactFixedPointDxDy contract requires exact-authoritative mag2_bits output",
                    ));
                }
            }
        }
        (None, true) => {
            if spec
                .writes
                .iter()
                .any(|out| out.name == "mag2_bits" && out.authority == OutputAuthority::ExactAuthoritative)
            {
                return Err(artifact_err(
                    &spec.id,
                    "exact-authoritative mag2_bits requires a Mag2SourceContract",
                ));
            }
        }
        (Some(Mag2SourceContract::ExactFixedPointDxDy { .. }), false) => {
            let multilayer = is_multilayer_overlay_descriptor_id(&spec.id)
                && multilayer_exact_mag_bits_outputs(spec) == SEAD_OBS2_LAYER_COUNT;
            if !multilayer {
                return Err(artifact_err(
                    &spec.id,
                    "Mag2SourceContract declared without exact-authoritative mag2_bits output",
                ));
            }
        }
        (None, false) => {}
    }

    if spec.reads.iter().any(|read| read == "dx" || read == "dy")
        && spec.writes.iter().any(|out| {
            (out.name == "mag2_bits" || out.name == "mag2")
                && out.authority == OutputAuthority::ExactAuthoritative
        })
        && spec.mag2_source_contract.is_none()
    {
        return Err(artifact_err(
            &spec.id,
            "raw f32 dx/dy cannot produce exact-authoritative mag2 without Mag2SourceContract",
        ));
    }

    Ok(())
}

/// Validate score output authority contract (SEAD-OBS-1).
pub fn validate_score_authority_contract(spec: &KernelDescriptorSpec) -> Result<(), SpecError> {
    let score_out = spec
        .writes
        .iter()
        .find(|out| out.name == "score_bits" || out.name == "score_fixed")
        .map(|out| (out.name.as_str(), out.authority));

    match spec.score_authority_contract {
        Some(ScoreAuthorityContract::ApproximateDiagnosticF32) => {
            match score_out {
                Some(("score_bits", OutputAuthority::ApproximateDiagnostic)) => {}
                Some(("score_fixed", _)) => {
                    return Err(artifact_err(
                        &spec.id,
                        "ApproximateDiagnosticF32 score contract requires score_bits output, not score_fixed",
                    ));
                }
                Some((_, OutputAuthority::ExactAuthoritative)) => {
                    return Err(artifact_err(
                        &spec.id,
                        "score_bits cannot be ExactAuthoritative under ApproximateDiagnosticF32 score contract",
                    ));
                }
                Some((_, OutputAuthority::RejectedDeferred)) => {
                    return Err(artifact_err(
                        &spec.id,
                        "score_bits is rejected/deferred under ApproximateDiagnosticF32 score contract",
                    ));
                }
                Some((name, OutputAuthority::ApproximateDiagnostic)) => {
                    return Err(artifact_err(
                        &spec.id,
                        format!(
                            "ApproximateDiagnosticF32 score contract requires score_bits output, not `{name}`"
                        ),
                    ));
                }
                None => {
                    return Err(artifact_err(
                        &spec.id,
                        "ApproximateDiagnosticF32 score contract requires score_bits output",
                    ));
                }
            }
        }
        Some(ScoreAuthorityContract::ExactQ16WeightedSum) => {
            match score_out {
                Some(("score_fixed", OutputAuthority::ExactAuthoritative)) => {}
                Some(("score_bits", OutputAuthority::ExactAuthoritative)) => {
                    return Err(artifact_err(
                        &spec.id,
                        "score_bits cannot be ExactAuthoritative under ExactQ16WeightedSum; use score_fixed",
                    ));
                }
                Some(("score_fixed", OutputAuthority::ApproximateDiagnostic)) => {
                    return Err(artifact_err(
                        &spec.id,
                        "score_fixed must be ExactAuthoritative under ExactQ16WeightedSum score contract",
                    ));
                }
                Some((_, OutputAuthority::RejectedDeferred)) => {
                    return Err(artifact_err(
                        &spec.id,
                        "score_fixed is rejected/deferred under ExactQ16WeightedSum score contract",
                    ));
                }
                Some(("score_bits", OutputAuthority::ApproximateDiagnostic)) => {
                    return Err(artifact_err(
                        &spec.id,
                        "ExactQ16WeightedSum score contract requires score_fixed output, not score_bits",
                    ));
                }
                Some((name, OutputAuthority::ApproximateDiagnostic)) => {
                    return Err(artifact_err(
                        &spec.id,
                        format!(
                            "score_fixed must be ExactAuthoritative under ExactQ16WeightedSum; got approximate `{name}`"
                        ),
                    ));
                }
                Some((name, OutputAuthority::ExactAuthoritative)) => {
                    return Err(artifact_err(
                        &spec.id,
                        format!(
                            "ExactQ16WeightedSum score contract requires score_fixed output, not `{name}`"
                        ),
                    ));
                }
                None => {
                    return Err(artifact_err(
                        &spec.id,
                        "ExactQ16WeightedSum score contract requires score_fixed output",
                    ));
                }
            }
        }
        None => {
            if score_out.map(|(_, auth)| auth) == Some(OutputAuthority::ExactAuthoritative) {
                return Err(artifact_err(
                    &spec.id,
                    "score output cannot be ExactAuthoritative without a pinned score authority contract",
                ));
            }
        }
    }

    if score_out.map(|(_, auth)| auth) == Some(OutputAuthority::ExactAuthoritative)
        && spec.score_authority_contract.is_none()
    {
        // covered above
    }

    Ok(())
}

/// Validate landed SEAD-OBS-0 overlay score descriptor pins (SEAD-OBS-1).
pub fn validate_sead_obs0_overlay_score_contract(spec: &KernelDescriptorSpec) -> Result<(), SpecError> {
    if spec.id != SEAD_OBS0_DESCRIPTOR_ID {
        return Ok(());
    }
    match spec.mag2_source_contract {
        Some(Mag2SourceContract::ExactFixedPointDxDy { fraction_bits })
            if fraction_bits == MAG2_Q16_FRAC_BITS => {}
        _ => {
            return Err(artifact_err(
                &spec.id,
                "SEAD observer overlay score requires Q16.16 ExactFixedPointDxDy mag2 contract",
            ));
        }
    }
    if spec.pre_sqrt_contract != Some(ExactPreSqrtInputContract::InlineFixedPointMag2Sqrt) {
        return Err(artifact_err(
            &spec.id,
            "SEAD observer overlay score requires InlineFixedPointMag2Sqrt pre-sqrt contract",
        ));
    }
    if spec.score_authority_contract != Some(ScoreAuthorityContract::ApproximateDiagnosticF32) {
        return Err(artifact_err(
            &spec.id,
            "SEAD observer overlay score requires ApproximateDiagnosticF32 score contract",
        ));
    }
    Ok(())
}

/// Validate landed SEAD-OBS-2 multilayer overlay score descriptor pins.
pub fn validate_sead_obs2_multilayer_overlay_score_contract(
    spec: &KernelDescriptorSpec,
) -> Result<(), SpecError> {
    if spec.id != SEAD_OBS2_DESCRIPTOR_ID {
        return Ok(());
    }
    match spec.mag2_source_contract {
        Some(Mag2SourceContract::ExactFixedPointDxDy { fraction_bits })
            if fraction_bits == MAG2_Q16_FRAC_BITS => {}
        _ => {
            return Err(artifact_err(
                &spec.id,
                "SEAD multilayer overlay score requires Q16.16 ExactFixedPointDxDy mag2 contract",
            ));
        }
    }
    if spec.pre_sqrt_contract != Some(ExactPreSqrtInputContract::InlineFixedPointMag2Sqrt) {
        return Err(artifact_err(
            &spec.id,
            "SEAD multilayer overlay score requires InlineFixedPointMag2Sqrt pre-sqrt contract",
        ));
    }
    if spec.score_authority_contract != Some(ScoreAuthorityContract::ApproximateDiagnosticF32) {
        return Err(artifact_err(
            &spec.id,
            "SEAD multilayer overlay score requires ApproximateDiagnosticF32 score contract",
        ));
    }
    if multilayer_exact_mag_bits_outputs(spec) != SEAD_OBS2_LAYER_COUNT {
        return Err(artifact_err(
            &spec.id,
            format!(
                "SEAD multilayer overlay score requires {SEAD_OBS2_LAYER_COUNT} exact layer mag_bits outputs"
            ),
        ));
    }
    if !has_multilayer_fixed_reads(spec) {
        return Err(artifact_err(
            &spec.id,
            "SEAD multilayer overlay score requires layer gx/gy/weight reads and bias_fixed",
        ));
    }
    Ok(())
}

/// Validate landed SEAD-OBS-3 multilayer fixed-point score descriptor pins.
pub fn validate_sead_obs3_multilayer_fixed_score_contract(
    spec: &KernelDescriptorSpec,
) -> Result<(), SpecError> {
    if spec.id != SEAD_OBS3_DESCRIPTOR_ID {
        return Ok(());
    }
    match spec.mag2_source_contract {
        Some(Mag2SourceContract::ExactFixedPointDxDy { fraction_bits })
            if fraction_bits == MAG2_Q16_FRAC_BITS => {}
        _ => {
            return Err(artifact_err(
                &spec.id,
                "SEAD multilayer fixed score requires Q16.16 ExactFixedPointDxDy mag2 contract",
            ));
        }
    }
    if spec.pre_sqrt_contract != Some(ExactPreSqrtInputContract::InlineFixedPointMag2Sqrt) {
        return Err(artifact_err(
            &spec.id,
            "SEAD multilayer fixed score requires InlineFixedPointMag2Sqrt pre-sqrt contract",
        ));
    }
    if spec.score_authority_contract != Some(ScoreAuthorityContract::ExactQ16WeightedSum) {
        return Err(artifact_err(
            &spec.id,
            "SEAD multilayer fixed score requires ExactQ16WeightedSum score contract",
        ));
    }
    if multilayer_exact_mag_bits_outputs(spec) != SEAD_OBS3_LAYER_COUNT {
        return Err(artifact_err(
            &spec.id,
            format!(
                "SEAD multilayer fixed score requires {SEAD_OBS3_LAYER_COUNT} exact layer mag_bits outputs"
            ),
        ));
    }
    if !has_multilayer_fixed_reads(spec) {
        return Err(artifact_err(
            &spec.id,
            "SEAD multilayer fixed score requires layer gx/gy/weight reads and bias_fixed",
        ));
    }
    let score_fixed = spec
        .writes
        .iter()
        .find(|out| out.name == "score_fixed")
        .map(|out| out.authority);
    if score_fixed != Some(OutputAuthority::ExactAuthoritative) {
        return Err(artifact_err(
            &spec.id,
            "SEAD multilayer fixed score requires exact-authoritative score_fixed output",
        ));
    }
    Ok(())
}

/// Validate landed SEAD-OBS-4 threshold event descriptor pins.
pub fn validate_sead_obs4_threshold_event_contract(
    spec: &KernelDescriptorSpec,
) -> Result<(), SpecError> {
    if spec.id != SEAD_OBS4_DESCRIPTOR_ID {
        return Ok(());
    }
    match spec.mag2_source_contract {
        Some(Mag2SourceContract::ExactFixedPointDxDy { fraction_bits })
            if fraction_bits == MAG2_Q16_FRAC_BITS => {}
        _ => {
            return Err(artifact_err(
                &spec.id,
                "SEAD threshold event requires Q16.16 ExactFixedPointDxDy mag2 contract",
            ));
        }
    }
    if spec.pre_sqrt_contract != Some(ExactPreSqrtInputContract::InlineFixedPointMag2Sqrt) {
        return Err(artifact_err(
            &spec.id,
            "SEAD threshold event requires InlineFixedPointMag2Sqrt pre-sqrt contract",
        ));
    }
    if spec.score_authority_contract != Some(ScoreAuthorityContract::ExactQ16WeightedSum) {
        return Err(artifact_err(
            &spec.id,
            "SEAD threshold event requires ExactQ16WeightedSum score contract",
        ));
    }
    if multilayer_exact_mag_bits_outputs(spec) != SEAD_OBS4_LAYER_COUNT {
        return Err(artifact_err(
            &spec.id,
            format!(
                "SEAD threshold event requires {SEAD_OBS4_LAYER_COUNT} exact layer mag_bits outputs"
            ),
        ));
    }
    if !has_multilayer_fixed_reads(spec) {
        return Err(artifact_err(
            &spec.id,
            "SEAD threshold event requires layer gx/gy/weight reads and bias_fixed",
        ));
    }
    let score_fixed = spec
        .writes
        .iter()
        .find(|out| out.name == "score_fixed")
        .map(|out| out.authority);
    if score_fixed != Some(OutputAuthority::ExactAuthoritative) {
        return Err(artifact_err(
            &spec.id,
            "SEAD threshold event requires exact-authoritative score_fixed output",
        ));
    }
    if !has_threshold_event_reads(spec) {
        return Err(artifact_err(
            &spec.id,
            "SEAD threshold event requires threshold_fixed, hysteresis_fixed, and prior_state_u32 reads",
        ));
    }
    let state = spec
        .writes
        .iter()
        .find(|out| out.name == "state_u32")
        .map(|out| out.authority);
    let event = spec
        .writes
        .iter()
        .find(|out| out.name == "event_code_u32")
        .map(|out| out.authority);
    if state != Some(OutputAuthority::ExactAuthoritative) {
        return Err(artifact_err(
            &spec.id,
            "SEAD threshold event requires exact-authoritative state_u32 under ExactDeterministicEventFlag",
        ));
    }
    if event != Some(OutputAuthority::ExactAuthoritative) {
        return Err(artifact_err(
            &spec.id,
            "SEAD threshold event requires exact-authoritative event_code_u32 under ExactDeterministicEventFlag",
        ));
    }
    Ok(())
}

/// True when the descriptor is the landed SEAD event compaction kernel.
pub fn is_sead_event0_compaction_descriptor(spec: &KernelDescriptorSpec) -> bool {
    spec.id == SEAD_EVENT0_DESCRIPTOR_ID
        && has_event_compaction_reads(spec)
        && spec.writes.iter().any(|out| {
            out.name == "event_count" && out.authority == OutputAuthority::ExactAuthoritative
        })
        && spec.writes.iter().any(|out| {
            out.name == "overflow_flag" && out.authority == OutputAuthority::ExactAuthoritative
        })
        && spec.writes.iter().any(|out| {
            out.name == "event_record" && out.authority == OutputAuthority::ExactAuthoritative
        })
}

fn has_event_compaction_reads(spec: &KernelDescriptorSpec) -> bool {
    spec.reads.iter().any(|read| read == "observer_index_u32")
        && spec.reads.iter().any(|read| read == "event_code_u32")
        && spec.reads.iter().any(|read| read == "state_u32")
        && spec.reads.iter().any(|read| read == "score_fixed")
        && spec.reads.iter().any(|read| read == "flags_u32")
}

/// Validate landed SEAD-EVENT-0 event compaction descriptor pins.
pub fn validate_sead_event0_compaction_contract(
    spec: &KernelDescriptorSpec,
) -> Result<(), SpecError> {
    if spec.id != SEAD_EVENT0_DESCRIPTOR_ID {
        return Ok(());
    }
    if !has_event_compaction_reads(spec) {
        return Err(artifact_err(
            &spec.id,
            "SEAD event compaction requires observer_index_u32, event_code_u32, state_u32, score_fixed, flags_u32 reads",
        ));
    }
    if spec.exact_sqrt_artifact.is_some() {
        return Err(artifact_err(
            &spec.id,
            "SEAD event compaction must not bind sqrt artifact",
        ));
    }
    if spec.score_authority_contract.is_some() {
        return Err(artifact_err(
            &spec.id,
            "SEAD event compaction must not declare score authority contract",
        ));
    }
    let event_count = spec
        .writes
        .iter()
        .find(|out| out.name == "event_count")
        .map(|out| out.authority);
    let overflow = spec
        .writes
        .iter()
        .find(|out| out.name == "overflow_flag")
        .map(|out| out.authority);
    let record = spec
        .writes
        .iter()
        .find(|out| out.name == "event_record")
        .map(|out| out.authority);
    if event_count != Some(OutputAuthority::ExactAuthoritative) {
        return Err(artifact_err(
            &spec.id,
            "SEAD event compaction requires exact-authoritative event_count",
        ));
    }
    if overflow != Some(OutputAuthority::ExactAuthoritative) {
        return Err(artifact_err(
            &spec.id,
            "SEAD event compaction requires exact-authoritative overflow_flag",
        ));
    }
    if record != Some(OutputAuthority::ExactAuthoritative) {
        return Err(artifact_err(
            &spec.id,
            "SEAD event compaction requires exact-authoritative event_record (unordered membership)",
        ));
    }
    Ok(())
}

/// True when the descriptor is the landed integrated observer-event pipeline kernel.
pub fn is_sead_pipe0_observer_event_pipeline_descriptor(spec: &KernelDescriptorSpec) -> bool {
    spec.id == SEAD_PIPE0_DESCRIPTOR_ID
        && has_multilayer_fixed_reads(spec)
        && has_threshold_event_reads(spec)
        && spec.score_authority_contract == Some(ScoreAuthorityContract::ExactQ16WeightedSum)
        && spec.writes.iter().any(|out| out.name == "event_count")
        && spec.writes.iter().any(|out| out.name == "overflow_flag")
        && spec.writes.iter().any(|out| out.name == "event_record")
}

/// Validate landed SEAD-PIPE-0 integrated observer-event pipeline descriptor pins.
pub fn validate_sead_pipe0_observer_event_pipeline_contract(
    spec: &KernelDescriptorSpec,
) -> Result<(), SpecError> {
    if spec.id != SEAD_PIPE0_DESCRIPTOR_ID {
        return Ok(());
    }
    match spec.mag2_source_contract {
        Some(Mag2SourceContract::ExactFixedPointDxDy { fraction_bits })
            if fraction_bits == MAG2_Q16_FRAC_BITS => {}
        _ => {
            return Err(artifact_err(
                &spec.id,
                "SEAD observer-event pipeline requires Q16.16 ExactFixedPointDxDy mag2 contract",
            ));
        }
    }
    if spec.pre_sqrt_contract != Some(ExactPreSqrtInputContract::InlineFixedPointMag2Sqrt) {
        return Err(artifact_err(
            &spec.id,
            "SEAD observer-event pipeline requires InlineFixedPointMag2Sqrt pre-sqrt contract",
        ));
    }
    if spec.score_authority_contract != Some(ScoreAuthorityContract::ExactQ16WeightedSum) {
        return Err(artifact_err(
            &spec.id,
            "SEAD observer-event pipeline requires ExactQ16WeightedSum score contract",
        ));
    }
    if spec.exact_sqrt_artifact.is_none() {
        return Err(artifact_err(
            &spec.id,
            "SEAD observer-event pipeline requires artifact-backed Candidate F binding",
        ));
    }
    if !has_multilayer_fixed_reads(spec) || !has_threshold_event_reads(spec) {
        return Err(artifact_err(
            &spec.id,
            "SEAD observer-event pipeline requires multilayer observer and threshold reads",
        ));
    }
    for (name, msg) in [
        ("event_count", "exact-authoritative event_count"),
        ("overflow_flag", "exact-authoritative overflow_flag"),
        ("event_record", "exact-authoritative event_record"),
    ] {
        match spec
            .writes
            .iter()
            .find(|out| out.name == name)
            .map(|out| out.authority)
        {
            Some(OutputAuthority::ExactAuthoritative) => {}
            _ => {
                return Err(artifact_err(
                    &spec.id,
                    format!("SEAD observer-event pipeline requires {msg}"),
                ));
            }
        }
    }
    Ok(())
}

fn has_event_bucket_reads(spec: &KernelDescriptorSpec) -> bool {
    spec.reads.iter().any(|read| read == "source_index_u32")
        && spec.reads.iter().any(|read| read == "event_code_u32")
        && spec.reads.iter().any(|read| read == "state_u32")
        && spec.reads.iter().any(|read| read == "score_fixed")
}

/// True when the descriptor is the landed SEAD event-code bucketing kernel.
pub fn is_sead_event1_code_bucketing_descriptor(spec: &KernelDescriptorSpec) -> bool {
    spec.id == SEAD_EVENT1_DESCRIPTOR_ID
        && has_event_bucket_reads(spec)
        && spec.writes.iter().any(|out| out.name == "bucket_counts")
        && spec.writes.iter().any(|out| out.name == "bucket_overflow")
        && spec.writes.iter().any(|out| out.name == "bucket_record")
        && spec.writes.iter().any(|out| out.name == "invalid_code_count")
}

/// Validate landed SEAD-EVENT-1 event-code bucketing descriptor pins.
pub fn validate_sead_event1_code_bucketing_contract(
    spec: &KernelDescriptorSpec,
) -> Result<(), SpecError> {
    if spec.id != SEAD_EVENT1_DESCRIPTOR_ID {
        return Ok(());
    }
    if !has_event_bucket_reads(spec) {
        return Err(artifact_err(
            &spec.id,
            "SEAD event-code bucketing requires source_index_u32, event_code_u32, state_u32, score_fixed reads",
        ));
    }
    if spec.exact_sqrt_artifact.is_some() {
        return Err(artifact_err(
            &spec.id,
            "SEAD event-code bucketing must not bind sqrt artifact",
        ));
    }
    if spec.score_authority_contract.is_some() {
        return Err(artifact_err(
            &spec.id,
            "SEAD event-code bucketing must not declare score authority contract",
        ));
    }
    for (name, msg) in [
        ("bucket_counts", "exact-authoritative bucket_counts"),
        ("bucket_overflow", "exact-authoritative bucket_overflow"),
        ("bucket_record", "exact-authoritative bucket_record"),
        ("invalid_code_count", "exact-authoritative invalid_code_count"),
    ] {
        match spec
            .writes
            .iter()
            .find(|out| out.name == name)
            .map(|out| out.authority)
        {
            Some(OutputAuthority::ExactAuthoritative) => {}
            _ => {
                return Err(artifact_err(
                    &spec.id,
                    format!("SEAD event-code bucketing requires {msg}"),
                ));
            }
        }
    }
    Ok(())
}

fn has_event_bucket_reduction_reads(spec: &KernelDescriptorSpec) -> bool {
    spec.reads.iter().any(|read| read == "bucket_counts")
        && spec.reads.iter().any(|read| read == "bucket_record")
}

/// True when the descriptor is the landed SEAD event-bucket reductions kernel.
pub fn is_sead_event2_bucket_reductions_descriptor(spec: &KernelDescriptorSpec) -> bool {
    spec.id == SEAD_EVENT2_DESCRIPTOR_ID
        && has_event_bucket_reduction_reads(spec)
        && spec.writes.iter().any(|out| out.name == "reduction_count")
        && spec.writes.iter().any(|out| out.name == "sum_score")
        && spec.writes.iter().any(|out| out.name == "min_score")
        && spec.writes.iter().any(|out| out.name == "max_score")
        && spec.writes.iter().any(|out| out.name == "reduction_overflow_flag")
}

/// Validate landed SEAD-EVENT-2 bucket reductions descriptor pins.
pub fn validate_sead_event2_bucket_reductions_contract(
    spec: &KernelDescriptorSpec,
) -> Result<(), SpecError> {
    if spec.id != SEAD_EVENT2_DESCRIPTOR_ID {
        return Ok(());
    }
    if !has_event_bucket_reduction_reads(spec) {
        return Err(artifact_err(
            &spec.id,
            "SEAD bucket reductions requires bucket_counts and bucket_record reads",
        ));
    }
    if spec.exact_sqrt_artifact.is_some() {
        return Err(artifact_err(
            &spec.id,
            "SEAD bucket reductions must not bind sqrt artifact",
        ));
    }
    if spec.score_authority_contract.is_some() {
        return Err(artifact_err(
            &spec.id,
            "SEAD bucket reductions must not declare score authority contract",
        ));
    }
    for (name, msg) in [
        ("reduction_count", "exact-authoritative reduction_count"),
        ("sum_score", "exact-authoritative sum_score"),
        ("min_score", "exact-authoritative min_score"),
        ("max_score", "exact-authoritative max_score"),
        ("reduction_overflow_flag", "exact-authoritative reduction_overflow_flag"),
    ] {
        match spec
            .writes
            .iter()
            .find(|out| out.name == name)
            .map(|out| out.authority)
        {
            Some(OutputAuthority::ExactAuthoritative) => {}
            _ => {
                return Err(artifact_err(
                    &spec.id,
                    format!("SEAD bucket reductions requires {msg}"),
                ));
            }
        }
    }
    Ok(())
}

/// Validate artifact-backed exact sqrt admission rules for a kernel descriptor.
pub fn validate_exact_sqrt_artifact_admission(spec: &KernelDescriptorSpec) -> Result<(), SpecError> {
    if spec.id == SEAD_EVENT2_DESCRIPTOR_ID {
        return validate_sead_event2_bucket_reductions_contract(spec);
    }
    if spec.id == SEAD_EVENT1_DESCRIPTOR_ID {
        return validate_sead_event1_code_bucketing_contract(spec);
    }
    if spec.id == SEAD_PIPE0_DESCRIPTOR_ID {
        validate_exact_sqrt_artifact_binding(
            &spec.id,
            spec.exact_sqrt_artifact.as_ref().ok_or_else(|| {
                artifact_err(&spec.id, "SEAD observer-event pipeline requires F artifact")
            })?,
        )?;
        return validate_sead_pipe0_observer_event_pipeline_contract(spec);
    }
    if spec.id == SEAD_EVENT0_DESCRIPTOR_ID {
        return validate_sead_event0_compaction_contract(spec);
    }

    let has_exact_f_out = has_exact_f_authoritative_output(spec);

    match (&spec.exact_sqrt_artifact, has_exact_f_out) {
        (None, true) => {
            let needs_binding = spec.writes.iter().any(|out| {
                out.authority == OutputAuthority::ExactAuthoritative
                    && (out.name == "sqrt_out" || out.name == "mag")
            });
            if needs_binding {
                return Err(artifact_err(
                    &spec.id,
                    "exact-authoritative sqrt_out or mag requires artifact-backed Candidate F binding",
                ));
            }
        }
        (Some(binding), has_exact) => {
            validate_exact_sqrt_artifact_binding(&spec.id, binding)?;
            if spec.native_math == NativeMathClass::ApproximateJitOnly {
                return Err(artifact_err(
                    &spec.id,
                    "artifact-backed exact sqrt cannot use ApproximateJitOnly native math",
                ));
            }
            let is_dxdy_probe = spec.pre_sqrt_contract
                == Some(ExactPreSqrtInputContract::RawDxDyProbe);
            if !has_exact && !is_dxdy_probe {
                return Err(artifact_err(
                    &spec.id,
                    "artifact-backed exact sqrt descriptor must declare exact-authoritative sqrt_out or mag",
                ));
            }
        }
        (None, false) => {}
    }

    validate_exact_pre_sqrt_contract(spec)?;
    validate_mag2_source_contract(spec)?;
    validate_score_authority_contract(spec)?;
    validate_sead_obs0_overlay_score_contract(spec)?;
    validate_sead_obs2_multilayer_overlay_score_contract(spec)?;
    validate_sead_obs3_multilayer_fixed_score_contract(spec)?;
    validate_sead_obs4_threshold_event_contract(spec)?;

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

/// F-backed exact magnitude when `mag2` input is already exact-authoritative.
pub fn mag_f_from_exact_mag2_kernel_descriptor() -> KernelDescriptorSpec {
    KernelDescriptorSpec {
        id: MAG_F_FROM_MAG2_DESCRIPTOR_ID.to_string(),
        lane: KernelLane::TestOnly,
        reads: vec!["mag2".to_string()],
        writes: vec![exact_out("mag")],
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
        exact_sqrt_artifact: Some(exact_sqrt_f_artifact_descriptor()),
        pre_sqrt_contract: Some(ExactPreSqrtInputContract::ExactMag2Bits),
        mag2_source_contract: None,
        score_authority_contract: None,
    }
}

/// Raw dx/dy multiply-add magnitude probe — F sqrt stage, non-exact mag authority.
pub fn mag_f_from_dxdy_probe_kernel_descriptor() -> KernelDescriptorSpec {
    KernelDescriptorSpec {
        id: MAG_F_FROM_DXDY_PROBE_DESCRIPTOR_ID.to_string(),
        lane: KernelLane::TestOnly,
        reads: vec!["dx".to_string(), "dy".to_string()],
        writes: vec![approx_out("mag")],
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
        exact_sqrt_artifact: Some(exact_sqrt_f_artifact_descriptor()),
        pre_sqrt_contract: Some(ExactPreSqrtInputContract::RawDxDyProbe),
        mag2_source_contract: None,
        score_authority_contract: None,
    }
}

/// Exact fixed-point pre-sqrt mag2 from signed Q16.16 gradient components.
pub fn mag2_fixed_exact_kernel_descriptor() -> KernelDescriptorSpec {
    KernelDescriptorSpec {
        id: MAG2_FIXED_DESCRIPTOR_ID.to_string(),
        lane: KernelLane::TestOnly,
        reads: vec!["dx_fixed".to_string(), "dy_fixed".to_string()],
        writes: vec![exact_out("mag2_bits")],
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
        exact_sqrt_artifact: None,
        pre_sqrt_contract: None,
        mag2_source_contract: Some(Mag2SourceContract::ExactFixedPointDxDy {
            fraction_bits: MAG2_Q16_FRAC_BITS,
        }),
        score_authority_contract: None,
    }
}

/// Build the landed SEAD observer overlay score kernel descriptor (SEAD-OBS-1).
pub fn sead_obs0_overlay_score_kernel_descriptor() -> KernelDescriptorSpec {
    KernelDescriptorSpec {
        id: SEAD_OBS0_DESCRIPTOR_ID.to_string(),
        lane: KernelLane::TestOnly,
        reads: vec![
            "gx_fixed".to_string(),
            "gy_fixed".to_string(),
            "w_mag_fixed".to_string(),
            "bias_fixed".to_string(),
        ],
        writes: vec![
            exact_out("mag2_bits"),
            exact_out("mag_bits"),
            approx_out("score_bits"),
            approx_out("flags"),
        ],
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
        exact_sqrt_artifact: Some(exact_sqrt_f_artifact_descriptor()),
        pre_sqrt_contract: Some(ExactPreSqrtInputContract::InlineFixedPointMag2Sqrt),
        mag2_source_contract: Some(Mag2SourceContract::ExactFixedPointDxDy {
            fraction_bits: MAG2_Q16_FRAC_BITS,
        }),
        score_authority_contract: Some(ScoreAuthorityContract::ApproximateDiagnosticF32),
    }
}

/// Build the landed SEAD multilayer observer overlay score kernel descriptor (SEAD-OBS-2).
pub fn sead_obs2_multilayer_overlay_score_kernel_descriptor() -> KernelDescriptorSpec {
    let mut reads = Vec::new();
    for layer in 0..SEAD_OBS2_LAYER_COUNT {
        reads.push(format!("layer{layer}_gx_fixed"));
        reads.push(format!("layer{layer}_gy_fixed"));
        reads.push(format!("layer{layer}_w_fixed"));
    }
    reads.push("bias_fixed".to_string());

    let mut writes = Vec::new();
    for layer in 0..SEAD_OBS2_LAYER_COUNT {
        writes.push(exact_out(&format!("layer{layer}_mag_bits")));
    }
    writes.push(approx_out("score_bits"));
    writes.push(approx_out("flags"));

    KernelDescriptorSpec {
        id: SEAD_OBS2_DESCRIPTOR_ID.to_string(),
        lane: KernelLane::TestOnly,
        reads,
        writes,
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
        exact_sqrt_artifact: Some(exact_sqrt_f_artifact_descriptor()),
        pre_sqrt_contract: Some(ExactPreSqrtInputContract::InlineFixedPointMag2Sqrt),
        mag2_source_contract: Some(Mag2SourceContract::ExactFixedPointDxDy {
            fraction_bits: MAG2_Q16_FRAC_BITS,
        }),
        score_authority_contract: Some(ScoreAuthorityContract::ApproximateDiagnosticF32),
    }
}

/// Build the landed SEAD multilayer fixed-point score kernel descriptor (SEAD-OBS-3).
pub fn sead_obs3_multilayer_fixed_score_kernel_descriptor() -> KernelDescriptorSpec {
    let mut reads = Vec::new();
    for layer in 0..SEAD_OBS3_LAYER_COUNT {
        reads.push(format!("layer{layer}_gx_fixed"));
        reads.push(format!("layer{layer}_gy_fixed"));
        reads.push(format!("layer{layer}_w_fixed"));
    }
    reads.push("bias_fixed".to_string());

    let mut writes = Vec::new();
    for layer in 0..SEAD_OBS3_LAYER_COUNT {
        writes.push(exact_out(&format!("layer{layer}_mag_bits")));
    }
    writes.push(exact_out("score_fixed"));
    writes.push(approx_out("flags"));

    KernelDescriptorSpec {
        id: SEAD_OBS3_DESCRIPTOR_ID.to_string(),
        lane: KernelLane::TestOnly,
        reads,
        writes,
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
        exact_sqrt_artifact: Some(exact_sqrt_f_artifact_descriptor()),
        pre_sqrt_contract: Some(ExactPreSqrtInputContract::InlineFixedPointMag2Sqrt),
        mag2_source_contract: Some(Mag2SourceContract::ExactFixedPointDxDy {
            fraction_bits: MAG2_Q16_FRAC_BITS,
        }),
        score_authority_contract: Some(ScoreAuthorityContract::ExactQ16WeightedSum),
    }
}

/// Build the landed SEAD threshold event kernel descriptor (SEAD-OBS-4).
pub fn sead_obs4_threshold_event_kernel_descriptor() -> KernelDescriptorSpec {
    let mut reads = Vec::new();
    for layer in 0..SEAD_OBS4_LAYER_COUNT {
        reads.push(format!("layer{layer}_gx_fixed"));
        reads.push(format!("layer{layer}_gy_fixed"));
        reads.push(format!("layer{layer}_w_fixed"));
    }
    reads.push("bias_fixed".to_string());
    reads.push("threshold_fixed".to_string());
    reads.push("hysteresis_fixed".to_string());
    reads.push("prior_state_u32".to_string());

    let mut writes = Vec::new();
    for layer in 0..SEAD_OBS4_LAYER_COUNT {
        writes.push(exact_out(&format!("layer{layer}_mag_bits")));
    }
    writes.push(exact_out("score_fixed"));
    writes.push(exact_out("state_u32"));
    writes.push(exact_out("event_code_u32"));
    writes.push(approx_out("flags"));

    KernelDescriptorSpec {
        id: SEAD_OBS4_DESCRIPTOR_ID.to_string(),
        lane: KernelLane::TestOnly,
        reads,
        writes,
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
        exact_sqrt_artifact: Some(exact_sqrt_f_artifact_descriptor()),
        pre_sqrt_contract: Some(ExactPreSqrtInputContract::InlineFixedPointMag2Sqrt),
        mag2_source_contract: Some(Mag2SourceContract::ExactFixedPointDxDy {
            fraction_bits: MAG2_Q16_FRAC_BITS,
        }),
        score_authority_contract: Some(ScoreAuthorityContract::ExactQ16WeightedSum),
    }
}

/// Build the landed SEAD event compaction kernel descriptor (SEAD-EVENT-0).
pub fn sead_event0_compaction_kernel_descriptor() -> KernelDescriptorSpec {
    KernelDescriptorSpec {
        id: SEAD_EVENT0_DESCRIPTOR_ID.to_string(),
        lane: KernelLane::TestOnly,
        reads: vec![
            "observer_index_u32".to_string(),
            "event_code_u32".to_string(),
            "state_u32".to_string(),
            "score_fixed".to_string(),
            "flags_u32".to_string(),
        ],
        writes: vec![
            exact_out("event_count"),
            exact_out("overflow_flag"),
            exact_out("event_record"),
        ],
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
        exact_sqrt_artifact: None,
        pre_sqrt_contract: None,
        mag2_source_contract: None,
        score_authority_contract: None,
    }
}

/// Build the landed integrated observer-event pipeline descriptor (SEAD-PIPE-0).
pub fn sead_pipe0_observer_event_pipeline_kernel_descriptor() -> KernelDescriptorSpec {
    let mut reads = Vec::new();
    for layer in 0..SEAD_PIPE0_LAYER_COUNT {
        reads.push(format!("layer{layer}_gx_fixed"));
        reads.push(format!("layer{layer}_gy_fixed"));
        reads.push(format!("layer{layer}_w_fixed"));
    }
    reads.push("bias_fixed".to_string());
    reads.push("threshold_fixed".to_string());
    reads.push("hysteresis_fixed".to_string());
    reads.push("prior_state_u32".to_string());

    KernelDescriptorSpec {
        id: SEAD_PIPE0_DESCRIPTOR_ID.to_string(),
        lane: KernelLane::TestOnly,
        reads,
        writes: vec![
            exact_out("event_count"),
            exact_out("overflow_flag"),
            exact_out("event_record"),
        ],
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
        exact_sqrt_artifact: Some(exact_sqrt_f_artifact_descriptor()),
        pre_sqrt_contract: Some(ExactPreSqrtInputContract::InlineFixedPointMag2Sqrt),
        mag2_source_contract: Some(Mag2SourceContract::ExactFixedPointDxDy {
            fraction_bits: MAG2_Q16_FRAC_BITS,
        }),
        score_authority_contract: Some(ScoreAuthorityContract::ExactQ16WeightedSum),
    }
}

/// Build the landed SEAD event-code bucketing kernel descriptor (SEAD-EVENT-1).
pub fn sead_event1_code_bucketing_kernel_descriptor() -> KernelDescriptorSpec {
    KernelDescriptorSpec {
        id: SEAD_EVENT1_DESCRIPTOR_ID.to_string(),
        lane: KernelLane::TestOnly,
        reads: vec![
            "source_index_u32".to_string(),
            "event_code_u32".to_string(),
            "state_u32".to_string(),
            "score_fixed".to_string(),
        ],
        writes: vec![
            exact_out("bucket_counts"),
            exact_out("bucket_overflow"),
            exact_out("bucket_record"),
            exact_out("invalid_code_count"),
        ],
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
        exact_sqrt_artifact: None,
        pre_sqrt_contract: None,
        mag2_source_contract: None,
        score_authority_contract: None,
    }
}

/// Build the landed SEAD event-bucket reductions kernel descriptor (SEAD-EVENT-2).
pub fn sead_event2_bucket_reductions_kernel_descriptor() -> KernelDescriptorSpec {
    KernelDescriptorSpec {
        id: SEAD_EVENT2_DESCRIPTOR_ID.to_string(),
        lane: KernelLane::TestOnly,
        reads: vec!["bucket_counts".to_string(), "bucket_record".to_string()],
        writes: vec![
            exact_out("reduction_count"),
            exact_out("sum_score"),
            exact_out("min_score"),
            exact_out("max_score"),
            exact_out("reduction_overflow_flag"),
        ],
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
        exact_sqrt_artifact: None,
        pre_sqrt_contract: None,
        mag2_source_contract: None,
        score_authority_contract: None,
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
        pre_sqrt_contract: None,
        mag2_source_contract: None,
        score_authority_contract: None,
    }
}
