//! EML-LN-PRIMITIVE-0 — pinned exhaustive-qualification artifacts and
//! certified-toolchain roster for the admitted `LN` exact primitive
//! (candidate LND4). Certification is a phase-boundary LOCAL act — CI checks
//! presence/freshness only and never re-executes the admitted-domain sweep.
//!
//! Invalidation law: algorithm/table/opcode/domain drift or a toolchain-tuple
//! change invalidates these artifacts; requalify locally and re-pin. Failure
//! archaeology retained: LN1C `0x108443cfaeeaadfe` RED at `0x008dcb6b`; LNCF
//! `0xbc2f8faa558bb920` RED at `0x00800000` (2 ULP, 1,478 probe mismatches).

use crate::eml_exp_qualification::EmlExpCertifiedToolchain;
use crate::eml_opcode_gate::{
    ln_primitive_domain, ExactPrimitiveBitSemantics, ExactPrimitiveDeterminismEvidence,
};

/// Algorithm identity at qualification (must equal the live
/// [`simthing_core::eml_ln::EML_LN_ALGORITHM_IDENTITY`]).
pub const EML_LN_QUALIFIED_ALGORITHM_IDENTITY: u64 = 0xc32c_eb9f_9807_c0ca;

/// Admitted-domain size (positive normals `0x00800000..=0x7F7FFFFF`).
pub const EML_LN_EXHAUSTIVE_DOMAIN_SIZE: u64 = 2_130_706_432;

/// FNV-1a-64 over every output's little-endian bits in ascending bit order —
/// the CPU-twin reference digest, matched bit-for-bit by the standalone
/// frozen candidate artifact on the certified tuple.
pub const EML_LN_EXHAUSTIVE_REFERENCE_DIGEST: u64 = 0x196a_ced8_2d03_f378;

/// The certified roster for LN (same tuple type as 5.11; the shared physical
/// tuple re-qualified independently for the LN sequence).
pub const EML_LN_CERTIFIED_TOOLCHAINS: &[EmlExpCertifiedToolchain] = &[EmlExpCertifiedToolchain {
    adapter: "NVIDIA GeForce RTX 4080 Laptop GPU",
    backend: "Vulkan",
    driver: "NVIDIA 595.79",
    compiler: "rustc 1.95.0 + wgpu 22.1.0 / naga 22.1.0 (Cargo.lock)",
    interpreted_replay_digest: 0x196a_ced8_2d03_f378,
    jit_replay_digest: 0x196a_ced8_2d03_f378,
    qualified_on: "2026-08-05",
}];

/// Determinism evidence for the door from the pinned artifacts.
pub fn ln_qualified_determinism_evidence(
    toolchain: &EmlExpCertifiedToolchain,
) -> ExactPrimitiveDeterminismEvidence {
    ExactPrimitiveDeterminismEvidence {
        bit_semantics: ExactPrimitiveBitSemantics::Ieee754Binary32Bits,
        domain: ln_primitive_domain(),
        exhaustive_reference_digest: EML_LN_EXHAUSTIVE_REFERENCE_DIGEST,
        supported_backend_replay_digest: toolchain.jit_replay_digest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eml_opcode_gate::{
        admit_exp_call_sites, ExactPrimitiveAdmissionDoor, ExactPrimitiveAdmissionRequest,
        ExactPrimitiveConsumer, ExactPrimitiveConsumerEvidence, ExactPrimitiveCostEvidence,
        ExactPrimitiveResourceEffect, OpcodeGateError, LN_PRIMITIVE_NAME,
    };
    use simthing_core::eml_ln;
    use simthing_core::EmlResourceClass;

}
