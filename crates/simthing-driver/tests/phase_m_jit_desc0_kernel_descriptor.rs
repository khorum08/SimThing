//! Phase M-JIT-DESC-0 — Test-only kernel descriptor / admission manifest (Tier-2).
//!
//! Formalizes metadata for landed M-JIT-0, M-JIT-SQRT-0, M-JIT-GRAD-0/R1, and M-JIT-GRAD-1
//! proof kernels. No production scheduler, no default wiring, no semantic WGSL.

use simthing_spec::MappingExecutionProfile;

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
enum KernelLane {
    TestOnly,
    ProductionCandidate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputAuthority {
    ExactAuthoritative,
    ApproximateDiagnostic,
    RejectedDeferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NativeMathClass {
    None,
    ApproximateJitOnly,
}

#[derive(Debug, Clone)]
struct KernelDescriptor {
    id: &'static str,
    lane: KernelLane,
    reads: &'static [&'static str],
    writes: &'static [(&'static str, OutputAuthority)],
    native_math: NativeMathClass,
    semantic_free: bool,
    default_off: bool,
    production_wiring: bool,
}

fn output_authority(producer: &KernelDescriptor, name: &str) -> Option<OutputAuthority> {
    producer
        .writes
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, auth)| *auth)
}

fn validate_exact_inputs(
    producer: &KernelDescriptor,
    required_exact_inputs: &[&str],
) -> Result<(), String> {
    for name in required_exact_inputs {
        match output_authority(producer, name) {
            Some(OutputAuthority::ExactAuthoritative) => {}
            Some(OutputAuthority::ApproximateDiagnostic) => {
                return Err(format!(
                    "output `{name}` from kernel `{}` is approximate/diagnostic, not exact-authoritative",
                    producer.id
                ));
            }
            Some(OutputAuthority::RejectedDeferred) => {
                return Err(format!(
                    "output `{name}` from kernel `{}` is rejected/deferred",
                    producer.id
                ));
            }
            None => {
                return Err(format!(
                    "output `{name}` not produced by kernel `{}`",
                    producer.id
                ));
            }
        }
    }
    Ok(())
}

const M_JIT_0_WEIGHTED_ACCUMULATOR: KernelDescriptor = KernelDescriptor {
    id: "m_jit_0_weighted_accumulator",
    lane: KernelLane::TestOnly,
    reads: &["values"],
    writes: &[("out_col", OutputAuthority::ExactAuthoritative)],
    native_math: NativeMathClass::None,
    semantic_free: true,
    default_off: true,
    production_wiring: false,
};

const M_JIT_0_EMA: KernelDescriptor = KernelDescriptor {
    id: "m_jit_0_ema",
    lane: KernelLane::TestOnly,
    reads: &["values"],
    writes: &[("out_col", OutputAuthority::ExactAuthoritative)],
    native_math: NativeMathClass::None,
    semantic_free: true,
    default_off: true,
    production_wiring: false,
};

const M_JIT_SQRT_0_CANDIDATE: KernelDescriptor = KernelDescriptor {
    id: "m_jit_sqrt_0_candidate",
    lane: KernelLane::TestOnly,
    reads: &["values"],
    writes: &[
        ("sqrt_out", OutputAuthority::ApproximateDiagnostic),
        ("magnitude_out", OutputAuthority::ApproximateDiagnostic),
    ],
    native_math: NativeMathClass::ApproximateJitOnly,
    semantic_free: true,
    default_off: true,
    production_wiring: false,
};

const M_JIT_GRAD_0_OBSERVER: KernelDescriptor = KernelDescriptor {
    id: "m_jit_grad_0_observer",
    lane: KernelLane::TestOnly,
    reads: &["fields", "observers"],
    writes: &[
        ("dx", OutputAuthority::ExactAuthoritative),
        ("dy", OutputAuthority::ExactAuthoritative),
        ("mag2", OutputAuthority::ApproximateDiagnostic),
        ("descent_x", OutputAuthority::ExactAuthoritative),
        ("descent_y", OutputAuthority::ExactAuthoritative),
    ],
    native_math: NativeMathClass::None,
    semantic_free: true,
    default_off: true,
    production_wiring: false,
};

const M_JIT_GRAD_1_OBSERVER_SCORE: KernelDescriptor = KernelDescriptor {
    id: "m_jit_grad_1_observer_score",
    lane: KernelLane::TestOnly,
    reads: &["fields", "observers"],
    writes: &[
        ("dx", OutputAuthority::ExactAuthoritative),
        ("dy", OutputAuthority::ExactAuthoritative),
        ("descent_x", OutputAuthority::ExactAuthoritative),
        ("descent_y", OutputAuthority::ExactAuthoritative),
        ("score", OutputAuthority::ExactAuthoritative),
    ],
    native_math: NativeMathClass::None,
    semantic_free: true,
    default_off: true,
    production_wiring: false,
};

fn all_descriptors() -> Vec<&'static KernelDescriptor> {
    vec![
        &M_JIT_0_WEIGHTED_ACCUMULATOR,
        &M_JIT_0_EMA,
        &M_JIT_SQRT_0_CANDIDATE,
        &M_JIT_GRAD_0_OBSERVER,
        &M_JIT_GRAD_1_OBSERVER_SCORE,
    ]
}

fn assert_descriptor_names_semantic_free(desc: &KernelDescriptor) {
    for term in FORBIDDEN_SEMANTIC_TERMS {
        assert!(
            !desc.id.contains(term),
            "descriptor id `{}` must be semantic-free; found `{term}`",
            desc.id
        );
    }
    for name in desc.reads {
        for term in FORBIDDEN_SEMANTIC_TERMS {
            assert!(
                !name.contains(term),
                "read name `{name}` in `{}` must be semantic-free",
                desc.id
            );
        }
    }
    for (name, _) in desc.writes {
        for term in FORBIDDEN_SEMANTIC_TERMS {
            assert!(
                !name.contains(term),
                "write name `{name}` in `{}` must be semantic-free",
                desc.id
            );
        }
    }
}

#[test]
fn jit_desc0_descriptors_match_landed_evidence() {
    assert_eq!(
        output_authority(&M_JIT_GRAD_0_OBSERVER, "mag2"),
        Some(OutputAuthority::ApproximateDiagnostic)
    );
    assert_eq!(
        output_authority(&M_JIT_GRAD_1_OBSERVER_SCORE, "score"),
        Some(OutputAuthority::ExactAuthoritative)
    );
    assert_eq!(
        output_authority(&M_JIT_SQRT_0_CANDIDATE, "sqrt_out"),
        Some(OutputAuthority::ApproximateDiagnostic)
    );
    assert!(output_authority(&M_JIT_GRAD_1_OBSERVER_SCORE, "mag2").is_none());

    for desc in all_descriptors() {
        assert_eq!(desc.lane, KernelLane::TestOnly, "{}", desc.id);
    }

    println!(
        "descriptor_evidence: grad0_mag2=ApproximateDiagnostic grad1_score=ExactAuthoritative sqrt=ApproximateDiagnostic all=TestOnly"
    );
}

#[test]
fn jit_desc0_rejects_approximate_output_as_exact_input() {
    assert!(validate_exact_inputs(&M_JIT_GRAD_0_OBSERVER, &["mag2"]).is_err());
    assert!(validate_exact_inputs(&M_JIT_SQRT_0_CANDIDATE, &["sqrt_out"]).is_err());

    validate_exact_inputs(&M_JIT_GRAD_0_OBSERVER, &["dx", "dy"])
        .expect("dx/dy are exact-authoritative");
    validate_exact_inputs(&M_JIT_GRAD_0_OBSERVER, &["descent_x", "descent_y"])
        .expect("descent outputs are exact-authoritative");
    validate_exact_inputs(&M_JIT_GRAD_1_OBSERVER_SCORE, &["score"])
        .expect("score is exact-authoritative");

    // GRAD-1 score must not depend on approximate mag2 from GRAD-0.
    assert!(output_authority(&M_JIT_GRAD_1_OBSERVER_SCORE, "mag2").is_none());
}

#[test]
fn jit_desc0_descriptors_are_default_off_and_not_production_wired() {
    for desc in all_descriptors() {
        assert_eq!(desc.lane, KernelLane::TestOnly, "{}", desc.id);
        assert!(desc.default_off, "{} must be default-off", desc.id);
        assert!(
            !desc.production_wiring,
            "{} must not have production wiring",
            desc.id
        );
    }
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
}

#[test]
fn jit_desc0_semantic_free_status_is_required() {
    for desc in all_descriptors() {
        assert!(desc.semantic_free, "{} must be semantic-free", desc.id);
        assert_descriptor_names_semantic_free(desc);
    }
}

#[test]
fn jit_desc0_native_math_classification_is_explicit() {
    assert_eq!(
        M_JIT_SQRT_0_CANDIDATE.native_math,
        NativeMathClass::ApproximateJitOnly
    );
    for desc in [
        &M_JIT_0_WEIGHTED_ACCUMULATOR,
        &M_JIT_0_EMA,
        &M_JIT_GRAD_0_OBSERVER,
        &M_JIT_GRAD_1_OBSERVER_SCORE,
    ] {
        assert_eq!(
            desc.native_math,
            NativeMathClass::None,
            "{} must not use native approximate math",
            desc.id
        );
    }
}
