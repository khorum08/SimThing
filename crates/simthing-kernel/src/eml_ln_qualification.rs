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

    /// Freshness tripwire: live identity == qualified identity, opcode value,
    /// domain size, and roster replay digests all pinned.
    #[test]
    fn eml_ln_primitive_0_qualified_artifacts_bind_to_the_live_algorithm_identity() {
        assert_eq!(
            eml_ln::EML_LN_ALGORITHM_IDENTITY,
            EML_LN_QUALIFIED_ALGORITHM_IDENTITY,
            "pinned LN sequence/table drifted; exhaustive artifacts are STALE — requalify locally"
        );
        assert_eq!(simthing_core::eml_opcode::LN, 27, "LN opcode value is pinned");
        assert_eq!(eml_ln::EML_LN_DOMAIN_SIZE, EML_LN_EXHAUSTIVE_DOMAIN_SIZE);
        for toolchain in EML_LN_CERTIFIED_TOOLCHAINS {
            assert_eq!(toolchain.interpreted_replay_digest, EML_LN_EXHAUSTIVE_REFERENCE_DIGEST);
            assert_eq!(toolchain.jit_replay_digest, EML_LN_EXHAUSTIVE_REFERENCE_DIGEST);
        }
    }

    /// The 5.12 admission ritual: a fresh door admits exactly one `LN` from
    /// pinned qualification + measured cost + measured consumer evidence.
    #[test]
    fn eml_ln_primitive_0_door_admits_exactly_one_ln_from_pinned_evidence() {
        let mut door = ExactPrimitiveAdmissionDoor::default();
        assert_eq!(door.admitted_count(), 0);
        let determinism = ExactPrimitiveAdmissionDoor::verify_determinism(
            ln_qualified_determinism_evidence(&EML_LN_CERTIFIED_TOOLCHAINS[0]),
        )
        .expect("pinned exhaustive digests mint the determinism key");
        // Driver-originated compiled resource effects, measured 2026-08-05 by
        // eml_ln_primitive_0_cost_gate_beats_the_pinned_gadget_baseline
        // (VK pipeline statistics, certified adapter): canonical interpreter
        // (Legacy32 21-node LN gadget baseline) vs SSA-JIT LN block (Compact4).
        let cost = ExactPrimitiveAdmissionDoor::verify_cost(ExactPrimitiveCostEvidence {
            resource_class: EmlResourceClass::CompactStack4,
            canonical_interpreter: ExactPrimitiveResourceEffect {
                register_count: 32,
                binary_size_bytes: 17_664,
                local_memory_bytes: 68_719_476_864,
            },
            primitive_candidate: ExactPrimitiveResourceEffect {
                register_count: 18,
                binary_size_bytes: 2_688,
                local_memory_bytes: 68_719_476_736,
            },
        })
        .expect("measured strict non-regressing compiled-resource win");
        // Measured by the LogAccumulate consumer referee: multiplicative
        // dynamics cannot ride the Sum lane at all without LN (excess = the
        // measured product-vs-logsum representability gap, in bps).
        // 5.14: consumer necessity only — LogAccumulate cross-arm bit identity
        // survives as a language-level witness outside admission.
        let consumer =
            ExactPrimitiveAdmissionDoor::verify_consumer(ExactPrimitiveConsumerEvidence {
                consumer: ExactPrimitiveConsumer::FieldSweepEvalEml,
                measured_threshold_excess_bps: 10_000,
            })
            .expect("measured consumer necessity");
        let admission = door
            .admit(ExactPrimitiveAdmissionRequest {
                name: LN_PRIMITIVE_NAME.to_owned(),
                determinism: Some(determinism),
                cost: Some(cost),
                consumer: Some(consumer),
            })
            .expect("LN admits through the 5.10 door");
        assert_eq!(admission.name(), LN_PRIMITIVE_NAME);
        assert_eq!(admission.domain(), ln_primitive_domain());
        assert_eq!(door.admitted_count(), 1);
        assert_eq!(
            door.admit(ExactPrimitiveAdmissionRequest {
                name: "POW".to_owned(),
                determinism: None,
                cost: None,
                consumer: None,
            }),
            Err(OpcodeGateError::ExactPrimitiveLimitReached),
            "one proven primitive per landing; POW is a gadget, never an opcode"
        );
        // Call-site law holds for LN: guarded shape admits, naive is spanned.
        let guard = simthing_core::eml_nodes::EmlNode {
            opcode: simthing_core::eml_opcode::CLAMP_BOUNDED,
            flags: 0,
            a: eml_ln::EML_LN_DOMAIN_MIN_BITS,
            b: eml_ln::EML_LN_DOMAIN_MAX_BITS,
            c: 0,
            d: 0,
        };
        let ln_node = simthing_core::eml_nodes::EmlNode {
            opcode: simthing_core::eml_opcode::LN,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        };
        admit_exp_call_sites(&[guard, ln_node]).expect("guarded LN call site admits");
        assert!(matches!(
            admit_exp_call_sites(&[ln_node]),
            Err(OpcodeGateError::UnguardedExactPrimitiveCallSite { .. })
        ));
    }
}
