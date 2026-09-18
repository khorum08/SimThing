//! EML-EXP-PRIMITIVE-0 — pinned exhaustive-qualification artifacts and
//! trust-chain identity for the admitted `EXP` exact primitive.
//!
//! **Certification is a phase-boundary LOCAL act, never CI re-execution**
//! (standing Owner ruling; full_eml_unification §10.3). The values below were
//! produced by the local exhaustive admitted-domain sweep
//! (`crates/simthing-workshop/tests/eml_exp_primitive_0_qualification.rs`,
//! run `-- --ignored`) over every admitted-domain bit pattern, on every
//! certified toolchain tuple. Standing checks verify PRESENCE and FRESHNESS
//! of these pinned values only:
//! - the algorithm-identity tripwire below goes RED if any pinned constant or
//!   the sequence version moves without requalification;
//! - `scripts/ci/eml_exp_qualification_check.sh` verifies the doc artifact,
//!   the pinned digest, and the recorded compiler/backend versions against
//!   the live tree without executing anything.
//!
//! **Invalidation law:** any drift in algorithm identity, opcode value,
//! domain endpoints, or a certified tuple's (compiler, backend, driver)
//! identity invalidates the artifact for that tuple; requalify locally and
//! re-pin. A driver/backend change on a running host is detected at the next
//! local qualification or GPU referee run — CI has no GPU and never claims
//! otherwise.

use crate::eml_opcode_gate::{
    exp_primitive_domain, ExactPrimitiveBitSemantics, ExactPrimitiveDeterminismEvidence,
};

/// Algorithm identity at qualification time (must equal the live
/// [`simthing_core::eml_exp::EML_EXP_ALGORITHM_IDENTITY`] or the artifacts
/// are stale).
pub const EML_EXP_QUALIFIED_ALGORITHM_IDENTITY: u64 = 0x2976_5ea9_251c_2ae1;

/// Number of admitted-domain binary32 patterns the sweep enumerates
/// (positive bits `0x00000000..=0x42B170A4` + negative bits
/// `0x80000000..=0xC2AEA8F6`, ascending).
pub const EML_EXP_EXHAUSTIVE_DOMAIN_SIZE: u64 = 2_237_667_740;

/// FNV-1a-64 over every output's little-endian bits in canonical enumeration
/// order — the CPU-twin reference digest.
pub const EML_EXP_EXHAUSTIVE_REFERENCE_DIGEST: u64 = 0x7875_a45b_a919_d588;

/// One certified (compiler, backend, driver) tuple with its replay digests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmlExpCertifiedToolchain {
    pub adapter: &'static str,
    pub backend: &'static str,
    pub driver: &'static str,
    /// Host compiler + shader-compiler chain identity.
    pub compiler: &'static str,
    /// Exhaustive replay digest of the interpreted GPU arm.
    pub interpreted_replay_digest: u64,
    /// Exhaustive replay digest of the SSA-JIT arm.
    pub jit_replay_digest: u64,
    pub qualified_on: &'static str,
}

/// The certified roster. Growing it is a local requalification act (run the
/// sweep on the new tuple, append the row); shrinking it silently to make
/// evidence green is forbidden by the handoff fences.
pub const EML_EXP_CERTIFIED_TOOLCHAINS: &[EmlExpCertifiedToolchain] = &[EmlExpCertifiedToolchain {
    adapter: "NVIDIA GeForce RTX 4080 Laptop GPU",
    backend: "Vulkan",
    driver: "NVIDIA 595.79",
    compiler: "rustc 1.95.0 + wgpu 22.1.0 / naga 22.1.0 (Cargo.lock)",
    interpreted_replay_digest: 0x7875_a45b_a919_d588,
    jit_replay_digest: 0x7875_a45b_a919_d588,
    qualified_on: "2026-08-04",
}];

/// Determinism evidence for the door, built strictly from the pinned
/// artifacts of one certified tuple.
pub fn exp_qualified_determinism_evidence(
    toolchain: &EmlExpCertifiedToolchain,
) -> ExactPrimitiveDeterminismEvidence {
    ExactPrimitiveDeterminismEvidence {
        bit_semantics: ExactPrimitiveBitSemantics::Ieee754Binary32Bits,
        domain: exp_primitive_domain(),
        exhaustive_reference_digest: EML_EXP_EXHAUSTIVE_REFERENCE_DIGEST,
        supported_backend_replay_digest: toolchain.jit_replay_digest,
    }
}

/// Live `(adapter, backend, driver)` identity of a running GPU context —
/// the observed half of the trust chain the roster pins.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmlExpLiveToolchainIdentity {
    pub adapter: String,
    pub backend: String,
    pub driver: String,
}

impl EmlExpLiveToolchainIdentity {
    /// Read the live tuple from the running context (wgpu adapter info;
    /// driver identity composed exactly as the roster records it).
    pub fn from_context(ctx: &crate::context::GpuContext) -> Self {
        let info = ctx.adapter.get_info();
        Self {
            adapter: info.name,
            backend: format!("{:?}", info.backend),
            driver: format!("{} {}", info.driver, info.driver_info),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum EmlExpToolchainError {
    #[error(
        "live GPU tuple (adapter `{adapter}`, backend `{backend}`, driver `{driver}`) is NOT in the certified EXP toolchain roster; the exhaustive admitted-domain qualification does not cover it — requalify locally and append a roster row (never shrink or loosen the roster to pass)"
    )]
    UncertifiedToolchain {
        adapter: String,
        backend: String,
        driver: String,
    },
}

/// Live-tuple freshness enforcement (DA remand `5185563460` repair): every
/// local GPU qualification/referee path must call this and HARD-ERROR when
/// the running `(adapter, backend, driver)` tuple is absent from
/// [`EML_EXP_CERTIFIED_TOOLCHAINS`]. Matching is exact string equality on all
/// three fields — a driver update IS an uncertified tuple until the
/// admitted-domain sweep is re-run and the roster row appended. Local-only by
/// design: standing CI has no GPU and never claims this leg.
pub fn require_certified_toolchain(
    live: &EmlExpLiveToolchainIdentity,
) -> Result<&'static EmlExpCertifiedToolchain, EmlExpToolchainError> {
    EML_EXP_CERTIFIED_TOOLCHAINS
        .iter()
        .find(|certified| {
            certified.adapter == live.adapter
                && certified.backend == live.backend
                && certified.driver == live.driver
        })
        .ok_or_else(|| EmlExpToolchainError::UncertifiedToolchain {
            adapter: live.adapter.clone(),
            backend: live.backend.clone(),
            driver: live.driver.clone(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::eml_exp;

    use crate::eml_opcode_gate::{
        admit_exp_call_sites, ExactPrimitiveAdmissionDoor, ExactPrimitiveAdmissionRequest,
        ExactPrimitiveConsumer, ExactPrimitiveConsumerEvidence, ExactPrimitiveCostEvidence,
        ExactPrimitiveResourceEffect, OpcodeGateError, EXP_PRIMITIVE_NAME,
    };
    use simthing_core::EmlResourceClass;

}
