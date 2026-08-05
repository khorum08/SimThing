//! EML-LN-PRIMITIVE-0 — pinned exhaustive-qualification artifacts and
//! trust-chain identity for the admitted `LN` exact primitive.
//!
//! Certification is a phase-boundary LOCAL act, never CI re-execution. The
//! exhaustive admitted-domain sweep will fill the digest placeholders below;
//! standing checks verify PRESENCE and FRESHNESS of the pinned values only.

use crate::eml_opcode_gate::{
    ln_primitive_domain, ExactPrimitiveBitSemantics, ExactPrimitiveDeterminismEvidence,
};

/// Algorithm identity at qualification time (must equal the live
/// [`simthing_core::eml_ln::EML_LN_ALGORITHM_IDENTITY`] or the artifacts
/// are stale).
pub const EML_LN_QUALIFIED_ALGORITHM_IDENTITY: u64 = 0x1084_43cf_aeea_adfe;

/// Number of admitted-domain binary32 patterns the sweep enumerates
/// (positive finite normals `0x00800000..=0x7F7FFFFF`, ascending).
pub const EML_LN_EXHAUSTIVE_DOMAIN_SIZE: u64 = 2_130_706_432;

/// FNV-1a-64 over every output's little-endian bits in canonical enumeration
/// order — the CPU-twin reference digest (placeholder until LN1N local requal).
pub const EML_LN_EXHAUSTIVE_REFERENCE_DIGEST: u64 = 0x0;

/// One certified (compiler, backend, driver) tuple with its replay digests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmlLnCertifiedToolchain {
    pub adapter: &'static str,
    pub backend: &'static str,
    pub driver: &'static str,
    pub compiler: &'static str,
    pub interpreted_replay_digest: u64,
    pub jit_replay_digest: u64,
    pub qualified_on: &'static str,
}

/// The certified roster. GPU replay digests are filled by the local LN1N
/// exhaustive requalification act.
pub const EML_LN_CERTIFIED_TOOLCHAINS: &[EmlLnCertifiedToolchain] = &[EmlLnCertifiedToolchain {
    adapter: "NVIDIA GeForce RTX 4080 Laptop GPU",
    backend: "Vulkan",
    driver: "NVIDIA 595.79",
    compiler: "rustc 1.95.0 + wgpu 22.1.0 / naga 22.1.0 (Cargo.lock)",
    interpreted_replay_digest: 0x0,
    jit_replay_digest: 0x0,
    qualified_on: "2026-08-04",
}];

pub fn ln_qualified_determinism_evidence(
    toolchain: &EmlLnCertifiedToolchain,
) -> ExactPrimitiveDeterminismEvidence {
    ExactPrimitiveDeterminismEvidence {
        bit_semantics: ExactPrimitiveBitSemantics::Ieee754Binary32Bits,
        domain: ln_primitive_domain(),
        exhaustive_reference_digest: EML_LN_EXHAUSTIVE_REFERENCE_DIGEST,
        supported_backend_replay_digest: toolchain.jit_replay_digest,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmlLnLiveToolchainIdentity {
    pub adapter: String,
    pub backend: String,
    pub driver: String,
}

impl EmlLnLiveToolchainIdentity {
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
pub enum EmlLnToolchainError {
    #[error(
        "live GPU tuple (adapter `{adapter}`, backend `{backend}`, driver `{driver}`) is NOT in the certified LN toolchain roster; requalify locally and append a roster row"
    )]
    UncertifiedToolchain {
        adapter: String,
        backend: String,
        driver: String,
    },
}

pub fn require_certified_toolchain(
    live: &EmlLnLiveToolchainIdentity,
) -> Result<&'static EmlLnCertifiedToolchain, EmlLnToolchainError> {
    EML_LN_CERTIFIED_TOOLCHAINS
        .iter()
        .find(|certified| {
            certified.adapter == live.adapter
                && certified.backend == live.backend
                && certified.driver == live.driver
        })
        .ok_or_else(|| EmlLnToolchainError::UncertifiedToolchain {
            adapter: live.adapter.clone(),
            backend: live.backend.clone(),
            driver: live.driver.clone(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::eml_ln;

    use crate::eml_opcode_gate::{
        admit_ln_call_sites, ExactPrimitiveAdmissionDoor, OpcodeGateError,
    };

    #[test]
    fn eml_ln_primitive_0_qualified_artifacts_bind_to_the_live_algorithm_identity() {
        assert_eq!(
            eml_ln::EML_LN_ALGORITHM_IDENTITY,
            EML_LN_QUALIFIED_ALGORITHM_IDENTITY,
            "pinned LN sequence drifted; exhaustive artifacts are STALE — requalify locally"
        );
        assert_eq!(simthing_core::eml_opcode::LN, 27, "LN opcode value is pinned");
        assert_eq!(eml_ln::EML_LN_SEQUENCE_VERSION, 1);
        assert_eq!(
            EML_LN_EXHAUSTIVE_DOMAIN_SIZE,
            2_130_706_432,
            "admitted LN domain spans every positive finite normal bit pattern"
        );
    }

    #[test]
    fn eml_ln_primitive_0_uncertified_live_tuple_is_hard_red() {
        let certified = &EML_LN_CERTIFIED_TOOLCHAINS[0];
        let live_certified = EmlLnLiveToolchainIdentity {
            adapter: certified.adapter.to_owned(),
            backend: certified.backend.to_owned(),
            driver: certified.driver.to_owned(),
        };
        assert_eq!(
            require_certified_toolchain(&live_certified).expect("roster tuple admits"),
            certified
        );

        let planted = EmlLnLiveToolchainIdentity {
            driver: "NVIDIA 999.99".to_owned(),
            ..live_certified.clone()
        };
        let verdict = require_certified_toolchain(&planted);
        assert_eq!(
            verdict,
            Err(EmlLnToolchainError::UncertifiedToolchain {
                adapter: planted.adapter.clone(),
                backend: planted.backend.clone(),
                driver: planted.driver.clone(),
            }),
            "planted uncertified tuple must be RED"
        );
    }

    #[test]
    fn eml_ln_primitive_0_door_rejects_unpinned_exhaustive_digests() {
        // STOP (2026-08-04): exhaustive GPU replay does not match the CPU twin
        // on the certified toolchain, so reference digests stay at 0. The door
        // must refuse to mint a determinism key from incomplete evidence —
        // never admit LN on placeholder digests.
        let verdict = ExactPrimitiveAdmissionDoor::verify_determinism(
            ln_qualified_determinism_evidence(&EML_LN_CERTIFIED_TOOLCHAINS[0]),
        );
        assert_eq!(
            verdict,
            Err(OpcodeGateError::IncompleteExactDeterminismEvidence),
            "unpinned exhaustive digests must not mint a determinism key"
        );

        admit_ln_call_sites(&[
            simthing_core::eml_nodes::EmlNode {
                opcode: simthing_core::eml_opcode::CLAMP_BOUNDED,
                flags: 0,
                a: simthing_core::EML_LN_DOMAIN_MIN_BITS,
                b: simthing_core::EML_LN_DOMAIN_MAX_BITS,
                c: 0,
                d: 0,
            },
            simthing_core::eml_nodes::EmlNode {
                opcode: simthing_core::eml_opcode::LN,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
        ])
        .expect("shape-2 LN call-site admission is independent of door minting");
    }
}
