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

    /// Freshness tripwire: the live algorithm identity must equal the
    /// qualified identity — any pinned-constant or sequence drift REDs this
    /// before anything else runs, invalidating the exhaustive artifacts.
    #[test]
    fn eml_exp_primitive_0_qualified_artifacts_bind_to_the_live_algorithm_identity() {
        assert_eq!(
            eml_exp::EML_EXP_ALGORITHM_IDENTITY,
            EML_EXP_QUALIFIED_ALGORITHM_IDENTITY,
            "pinned EXP sequence drifted; exhaustive artifacts are STALE — requalify locally"
        );
        assert_eq!(simthing_core::eml_opcode::EXP, 26, "EXP opcode value is pinned");
        assert_eq!(eml_exp::EML_EXP_SEQUENCE_VERSION, 1);
        assert!(!EML_EXP_CERTIFIED_TOOLCHAINS.is_empty());
        for toolchain in EML_EXP_CERTIFIED_TOOLCHAINS {
            assert_eq!(
                toolchain.interpreted_replay_digest, EML_EXP_EXHAUSTIVE_REFERENCE_DIGEST,
                "{}: interpreted arm replay must match the CPU reference",
                toolchain.adapter
            );
            assert_eq!(
                toolchain.jit_replay_digest, EML_EXP_EXHAUSTIVE_REFERENCE_DIGEST,
                "{}: JIT arm replay must match the CPU reference",
                toolchain.adapter
            );
        }
    }

    /// DA remand `5185563460` referee: the live-tuple comparator admits
    /// exactly the certified roster and HARD-ERRORS on anything else —
    /// planted uncertified tuples (driver drift, backend swap, foreign
    /// adapter) must all be RED, and a planted bypass must not be what the
    /// comparator returns.
    #[test]
    fn eml_exp_primitive_0_uncertified_live_tuple_is_hard_red() {
        let certified = &EML_EXP_CERTIFIED_TOOLCHAINS[0];
        let live_certified = EmlExpLiveToolchainIdentity {
            adapter: certified.adapter.to_owned(),
            backend: certified.backend.to_owned(),
            driver: certified.driver.to_owned(),
        };
        assert_eq!(
            require_certified_toolchain(&live_certified).expect("roster tuple admits"),
            certified
        );

        let planted = [
            // Driver drift: the exact post-update shape the freshness law exists for.
            EmlExpLiveToolchainIdentity {
                driver: "NVIDIA 999.99".to_owned(),
                ..live_certified.clone()
            },
            // Backend swap on the same adapter.
            EmlExpLiveToolchainIdentity {
                backend: "Dx12".to_owned(),
                ..live_certified.clone()
            },
            // Foreign adapter (e.g. the host's uncertified iGPU).
            EmlExpLiveToolchainIdentity {
                adapter: "Intel(R) UHD Graphics".to_owned(),
                ..live_certified.clone()
            },
        ];
        for live in planted {
            let verdict = require_certified_toolchain(&live);
            assert_eq!(
                verdict,
                Err(EmlExpToolchainError::UncertifiedToolchain {
                    adapter: live.adapter.clone(),
                    backend: live.backend.clone(),
                    driver: live.driver.clone(),
                }),
                "planted uncertified tuple must be RED"
            );
            let bypass: Result<&EmlExpCertifiedToolchain, EmlExpToolchainError> = Ok(certified);
            assert_ne!(verdict, bypass, "planted comparator bypass must be RED");
        }
    }

    /// The 5.11 admission ritual: the door starts at zero (the 5.10 base
    /// census), admits exactly one primitive named `EXP` from the pinned
    /// qualification + measured cost + measured consumer evidence, and can
    /// never admit a second.
    #[test]
    fn eml_exp_primitive_0_door_admits_exactly_one_exp_from_pinned_evidence() {
        let mut door = ExactPrimitiveAdmissionDoor::default();
        assert_eq!(door.admitted_count(), 0, "the exact base admits zero primitives");

        let determinism = ExactPrimitiveAdmissionDoor::verify_determinism(
            exp_qualified_determinism_evidence(&EML_EXP_CERTIFIED_TOOLCHAINS[0]),
        )
        .expect("pinned exhaustive digests mint the determinism key");
        // Driver-originated compiled resource effects, measured 2026-08-04 by
        // eml_exp_primitive_0_cost_gate_beats_the_pinned_gadget_baseline
        // (VK_KHR_pipeline_executable_properties on the certified adapter):
        // canonical interpreter (Legacy32, 21-node gadget baseline) vs the
        // SSA-JIT EXP block (Compact4). Raw driver values verbatim.
        let cost = ExactPrimitiveAdmissionDoor::verify_cost(ExactPrimitiveCostEvidence {
            resource_class: EmlResourceClass::CompactStack4,
            canonical_interpreter: ExactPrimitiveResourceEffect {
                register_count: 28,
                binary_size_bytes: 16_000,
                local_memory_bytes: 68_719_476_864,
            },
            primitive_candidate: ExactPrimitiveResourceEffect {
                register_count: 17,
                binary_size_bytes: 2_560,
                local_memory_bytes: 68_719_476_736,
            },
        })
        .expect("measured strict non-regressing compiled-resource win");
        // Measured 2026-08-04 by the steering consumer referee
        // (eml_exp_primitive_0_staircase_deviation…, simthing-core): worst
        // staircase deviation from the smooth curve = 2967 bps of span.
        let consumer =
            ExactPrimitiveAdmissionDoor::verify_consumer(ExactPrimitiveConsumerEvidence {
                consumer: ExactPrimitiveConsumer::OrdinaryAccumulatorEvalEml,
                measured_threshold_excess_bps: 2_967,
            })
            .expect("measured staircase excess mints the consumer key");

        let admission = door
            .admit(ExactPrimitiveAdmissionRequest {
                name: EXP_PRIMITIVE_NAME.to_owned(),
                determinism: Some(determinism),
                cost: Some(cost),
                consumer: Some(consumer),
            })
            .expect("EXP admits through the 5.10 door");
        assert_eq!(admission.name(), EXP_PRIMITIVE_NAME);
        assert_eq!(admission.domain(), exp_primitive_domain());
        assert_eq!(door.admitted_count(), 1, "exactly one admitted primitive");
        assert_eq!(
            door.admit(ExactPrimitiveAdmissionRequest {
                name: "LN".to_owned(),
                determinism: None,
                cost: None,
                consumer: None,
            }),
            Err(OpcodeGateError::ExactPrimitiveLimitReached),
            "one proven primitive per landing; 5.12 brings its own door instance"
        );

        // The admitted vocabulary and the admitted call-site law hold
        // together: the canonical guarded shape admits, period.
        admit_exp_call_sites(&[
            simthing_core::eml_nodes::EmlNode {
                opcode: simthing_core::eml_opcode::CLAMP_BOUNDED,
                flags: 0,
                a: simthing_core::EML_EXP_DOMAIN_MIN_BITS,
                b: simthing_core::EML_EXP_SATURATION_CEILING_BITS,
                c: 0,
                d: 0,
            },
            simthing_core::eml_nodes::EmlNode {
                opcode: simthing_core::eml_opcode::EXP,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
        ])
        .expect("guarded call sites stay admissible after admission");
    }
}
