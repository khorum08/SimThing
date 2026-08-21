//! OC-K-EML-OPCODE-GATE-0 — EvalEML opcode/combine vocabulary admission gate.
//!
//! Closed EvalEML vocabulary at registration. Extension ladder:
//! 1. EML gadget tree over the existing interpreter (sanctioned default)
//! 2. New *generic* primitive only via Tier-2 gate with bit-exact CPU-oracle parity
//! 3. Semantic / scenario-specific opcode is never admissible
//!
//! Anchor B (Odrzywołek arXiv:2603.21852) + core §1.1 / §4.1; pathway on
//! `eml-extension-ladder`.

use simthing_core::eml_nodes::{self, EmlNode};
use simthing_core::EmlResourceClass;
use simthing_core::{ClampBehavior, CombineFn, SubFieldSpec};

use crate::accumulator_op::combine_kind;

/// Closed EvalEML opcode vocabulary (fixed interpreter set).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EvalEmlOpcode(u32);

impl EvalEmlOpcode {
    pub fn raw(self) -> u32 {
        self.0
    }

    /// Admit a raw opcode only if it is in the closed vocabulary.
    pub fn from_closed(raw: u32) -> Result<Self, OpcodeGateError> {
        if opcode_in_closed_vocabulary(raw) {
            Ok(Self(raw))
        } else {
            Err(OpcodeGateError::UnwhitelistedOpcode { opcode: raw })
        }
    }
}

/// Closed EvalEML / AccumulatorOp combine vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EvalEmlCombine(u32);

impl EvalEmlCombine {
    pub fn raw(self) -> u32 {
        self.0
    }

    pub fn from_closed_kind(raw: u32) -> Result<Self, OpcodeGateError> {
        if combine_in_closed_vocabulary(raw) {
            Ok(Self(raw))
        } else {
            Err(OpcodeGateError::UnwhitelistedCombine { combine_kind: raw })
        }
    }

    pub fn from_combine_fn(cf: &CombineFn) -> Self {
        Self(combine_fn_to_kind(cf))
    }
}

/// Snapshot of the closed EvalEML vocabulary (opcodes + combines).
#[derive(Clone, Copy, Debug, Default)]
pub struct EvalEmlVocabulary;

impl EvalEmlVocabulary {
    pub fn opcode_admitted(raw: u32) -> bool {
        opcode_in_closed_vocabulary(raw)
    }

    pub fn combine_admitted(raw: u32) -> bool {
        combine_in_closed_vocabulary(raw)
    }

    pub fn closed_opcodes() -> &'static [u32] {
        CLOSED_OPCODES
    }

    pub fn closed_combine_kinds() -> &'static [u32] {
        CLOSED_COMBINE_KINDS
    }
}

/// Bit-exact CPU-oracle parity evidence for a Tier-2 generic primitive.
///
/// Private field — bare construction is uncompilable outside this module.
///
/// ```compile_fail,E0451
/// fn forge_cpu_oracle_parity_proof() {
///     let _ = simthing_kernel::CpuOracleParityProof { bits: 0 };
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CpuOracleParityProof {
    /// IEEE bits of a reference sample proving CPU/GPU agreement.
    bits: u32,
}

impl CpuOracleParityProof {
    /// Mint from a bit-exact CPU reference sample (Tier-2 evidence).
    pub fn from_bit_exact_sample(
        cpu_bits: u32,
        gpu_or_twin_bits: u32,
    ) -> Result<Self, OpcodeGateError> {
        if cpu_bits != gpu_or_twin_bits {
            return Err(OpcodeGateError::ParityMismatch {
                cpu_bits,
                other_bits: gpu_or_twin_bits,
            });
        }
        Ok(Self { bits: cpu_bits })
    }

    pub fn bits(self) -> u32 {
        self.bits
    }
}

/// Tier-2 request to admit a new *generic* (semantic-free) primitive.
#[derive(Clone, Debug, PartialEq)]
pub struct GenericPrimitiveRegistration {
    pub name: String,
    pub proposed_opcode: u32,
    pub parity: CpuOracleParityProof,
}

/// Semantic / scenario-specific opcode request — never admissible.
#[derive(Clone, Debug, PartialEq)]
pub struct SemanticOpcodeRegistration {
    pub name: String,
    pub proposed_opcode: u32,
    pub scenario_or_policy_tag: String,
}

/// Registration request class for vocabulary admission.
#[derive(Clone, Debug, PartialEq)]
pub enum OpcodeRegistrationRequest {
    /// Use an opcode already in the closed vocabulary.
    ExistingClosed(EvalEmlOpcode),
    /// New generic primitive — requires Tier-2 + CPU-oracle parity.
    GenericPrimitive(GenericPrimitiveRegistration),
    /// Semantic / scenario-specific — hard-reject.
    Semantic(SemanticOpcodeRegistration),
}

/// Combine registration request class.
#[derive(Clone, Debug, PartialEq)]
pub enum CombineRegistrationRequest {
    ExistingClosed(EvalEmlCombine),
    /// Free raw combine_kind claim (unvalidated).
    RawUnvalidated {
        combine_kind: u32,
    },
    SemanticNamed {
        name: String,
        combine_kind: u32,
    },
}

/// Admitted opcode token (only via [`OpcodeRegistrationGate`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AdmittedEvalEmlOpcode {
    opcode: u32,
}

impl AdmittedEvalEmlOpcode {
    pub fn raw(self) -> u32 {
        self.opcode
    }
}

/// Admitted combine token (only via [`OpcodeRegistrationGate`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AdmittedEvalEmlCombine {
    kind: u32,
}

impl AdmittedEvalEmlCombine {
    pub fn raw(self) -> u32 {
        self.kind
    }
}

/// EvalEML opcode/combine registration admission gate.
#[derive(Clone, Copy, Debug, Default)]
pub struct OpcodeRegistrationGate;

impl OpcodeRegistrationGate {
    /// Admit an opcode registration request.
    pub fn admit_opcode(
        request: OpcodeRegistrationRequest,
    ) -> Result<AdmittedEvalEmlOpcode, OpcodeGateError> {
        match request {
            OpcodeRegistrationRequest::ExistingClosed(op) => {
                Ok(AdmittedEvalEmlOpcode { opcode: op.raw() })
            }
            OpcodeRegistrationRequest::GenericPrimitive(reg) => {
                // Tier-2: parity proof is required to construct GenericPrimitiveRegistration.
                // Vocabulary remains closed until a future DA-scoped expansion lands the
                // opcode in CLOSED_OPCODES; this gate records the parity requirement and
                // rejects expansion into the live vocabulary in this rung.
                let _parity = reg.parity;
                if opcode_in_closed_vocabulary(reg.proposed_opcode) {
                    // Already closed — treat as existing.
                    return Ok(AdmittedEvalEmlOpcode {
                        opcode: reg.proposed_opcode,
                    });
                }
                Err(
                    OpcodeGateError::GenericPrimitiveRequiresVocabularyExpansion {
                        name: reg.name,
                        proposed_opcode: reg.proposed_opcode,
                    },
                )
            }
            OpcodeRegistrationRequest::Semantic(reg) => {
                Err(OpcodeGateError::SemanticOpcodeNeverAdmissible {
                    name: reg.name,
                    proposed_opcode: reg.proposed_opcode,
                    scenario_or_policy_tag: reg.scenario_or_policy_tag,
                })
            }
        }
    }

    /// Admit a combine registration request.
    pub fn admit_combine(
        request: CombineRegistrationRequest,
    ) -> Result<AdmittedEvalEmlCombine, OpcodeGateError> {
        match request {
            CombineRegistrationRequest::ExistingClosed(c) => {
                Ok(AdmittedEvalEmlCombine { kind: c.raw() })
            }
            CombineRegistrationRequest::RawUnvalidated { combine_kind } => {
                EvalEmlCombine::from_closed_kind(combine_kind)
                    .map(|c| AdmittedEvalEmlCombine { kind: c.raw() })
            }
            CombineRegistrationRequest::SemanticNamed { name, combine_kind } => {
                Err(OpcodeGateError::SemanticCombineNeverAdmissible { name, combine_kind })
            }
        }
    }

    /// Validate every node opcode against the closed vocabulary (tree admission).
    pub fn admit_tree_nodes(nodes: &[EmlNode]) -> Result<(), OpcodeGateError> {
        for node in nodes {
            if !opcode_in_closed_vocabulary(node.opcode) {
                return Err(OpcodeGateError::UnwhitelistedOpcode {
                    opcode: node.opcode,
                });
            }
        }
        Ok(())
    }

    /// Validate a GPU combine_kind against the closed combine vocabulary.
    pub fn admit_combine_kind(kind: u32) -> Result<AdmittedEvalEmlCombine, OpcodeGateError> {
        Self::admit_combine(CombineRegistrationRequest::RawUnvalidated { combine_kind: kind })
    }
}

/// Closed bit semantics for an exact primitive candidate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExactPrimitiveBitSemantics {
    Ieee754Binary32Bits,
}

/// Closed special-value policy for an exact primitive domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExactPrimitiveDomainPolicy {
    FiniteOnlyRejectNanAndInfinity,
    PreserveIeeeNanInfinityAndSignedZero,
}

/// Validated binary32 input domain for an exact primitive.
///
/// ```compile_fail,E0451
/// use simthing_kernel::ExactPrimitiveDomainPolicy;
/// use simthing_kernel::eml_opcode_gate::PrimitiveDomain;
/// let _ = PrimitiveDomain {
///     min_bits: 0,
///     max_bits: 1,
///     special_value_policy: ExactPrimitiveDomainPolicy::FiniteOnlyRejectNanAndInfinity,
/// };
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PrimitiveDomain {
    min_bits: u32,
    max_bits: u32,
    special_value_policy: ExactPrimitiveDomainPolicy,
}

impl PrimitiveDomain {
    pub fn from_bits(
        min_bits: u32,
        max_bits: u32,
        special_value_policy: ExactPrimitiveDomainPolicy,
    ) -> Result<Self, OpcodeGateError> {
        if binary32_total_order_key(min_bits) > binary32_total_order_key(max_bits) {
            return Err(OpcodeGateError::ExactPrimitiveDomainBoundsReversed { min_bits, max_bits });
        }
        if special_value_policy == ExactPrimitiveDomainPolicy::FiniteOnlyRejectNanAndInfinity
            && (!f32::from_bits(min_bits).is_finite() || !f32::from_bits(max_bits).is_finite())
        {
            return Err(OpcodeGateError::ExactPrimitiveDomainSpecialEndpoint {
                min_bits,
                max_bits,
            });
        }
        Ok(Self {
            min_bits,
            max_bits,
            special_value_policy,
        })
    }

    pub fn min_bits(self) -> u32 {
        self.min_bits
    }

    pub fn max_bits(self) -> u32 {
        self.max_bits
    }

    pub fn special_value_policy(self) -> ExactPrimitiveDomainPolicy {
        self.special_value_policy
    }

    pub fn contains_bits(self, bits: u32) -> bool {
        if self.special_value_policy == ExactPrimitiveDomainPolicy::FiniteOnlyRejectNanAndInfinity
            && !f32::from_bits(bits).is_finite()
        {
            return false;
        }
        let key = binary32_total_order_key(bits);
        binary32_total_order_key(self.min_bits) <= key
            && key <= binary32_total_order_key(self.max_bits)
    }

    fn contains_range(self, min_bits: u32, max_bits: u32) -> bool {
        binary32_total_order_key(min_bits) <= binary32_total_order_key(max_bits)
            && self.contains_bits(min_bits)
            && self.contains_bits(max_bits)
    }
}

fn binary32_total_order_key(bits: u32) -> u32 {
    const SIGN_MASK: u32 = 0x8000_0000;
    if bits & SIGN_MASK != 0 {
        !bits
    } else {
        bits | SIGN_MASK
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExactPrimitiveSourceSpan {
    start: u32,
    end: u32,
}

impl ExactPrimitiveSourceSpan {
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub const fn start(self) -> u32 {
        self.start
    }

    pub const fn end(self) -> u32 {
        self.end
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExactPrimitiveRangeCertificate {
    domain: PrimitiveDomain,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExactPrimitiveGuardedInput {
    domain: PrimitiveDomain,
    guard_opcode: EvalEmlOpcode,
}

impl ExactPrimitiveGuardedInput {
    pub fn guard_opcode(self) -> EvalEmlOpcode {
        self.guard_opcode
    }
}

pub enum ExactPrimitiveRangeEvidence<'a> {
    PropertySubField(&'a SubFieldSpec),
    GuardedFormula(&'a ExactPrimitiveGuardedInput),
    /// EML-EXP-PRIMITIVE-0: an authored in-domain literal is its own shape-1
    /// certificate (full_eml_unification §4 names "literal in range").
    LiteralInRange {
        bits: u32,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExactPrimitiveCallSiteShape {
    RangeCertified(ExactPrimitiveRangeCertificate),
    GuardedSemantics(ExactPrimitiveGuardedInput),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExactPrimitiveCallSiteKind {
    RangeCertified,
    GuardedSemantics,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExactPrimitiveCallSiteAdmission {
    kind: ExactPrimitiveCallSiteKind,
}

impl ExactPrimitiveCallSiteAdmission {
    pub fn kind(self) -> ExactPrimitiveCallSiteKind {
        self.kind
    }

    pub const fn added_runtime_nodes(self) -> u32 {
        0
    }
}

/// Evidence used by the sealed door to mint a determinism key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExactPrimitiveDeterminismEvidence {
    pub bit_semantics: ExactPrimitiveBitSemantics,
    pub domain: PrimitiveDomain,
    pub exhaustive_reference_digest: u64,
    pub supported_backend_replay_digest: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExactPrimitiveDeterminismKey {
    evidence_digest: u64,
    domain: PrimitiveDomain,
}

/// Driver-originated compiled resource effects for one canonical-interpreter class.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExactPrimitiveResourceEffect {
    pub register_count: u64,
    pub binary_size_bytes: u64,
    pub local_memory_bytes: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExactPrimitiveCostEvidence {
    pub resource_class: EmlResourceClass,
    pub canonical_interpreter: ExactPrimitiveResourceEffect,
    pub primitive_candidate: ExactPrimitiveResourceEffect,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExactPrimitiveCostKey {
    resource_class: EmlResourceClass,
}

/// Concrete production consumers; free-form scenario labels are not admissible.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExactPrimitiveConsumer {
    OrdinaryAccumulatorEvalEml,
    FieldSweepEvalEml,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactPrimitiveConsumerEvidence {
    pub consumer: ExactPrimitiveConsumer,
    /// Measured excess over the governing threshold, in basis points.
    pub measured_threshold_excess_bps: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExactPrimitiveConsumerKey {
    consumer: ExactPrimitiveConsumer,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactPrimitiveAdmissionRequest {
    pub name: String,
    pub determinism: Option<ExactPrimitiveDeterminismKey>,
    pub cost: Option<ExactPrimitiveCostKey>,
    pub consumer: Option<ExactPrimitiveConsumerKey>,
}

/// Sealed proof token. It does not expand `CLOSED_OPCODES`; vocabulary expansion
/// remains a separate DA-scoped change.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactPrimitiveAdmission {
    name: String,
    resource_class: EmlResourceClass,
    consumer: ExactPrimitiveConsumer,
    domain: PrimitiveDomain,
}

impl ExactPrimitiveAdmission {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn resource_class(&self) -> EmlResourceClass {
        self.resource_class
    }

    pub fn consumer(&self) -> ExactPrimitiveConsumer {
        self.consumer
    }

    pub fn domain(&self) -> PrimitiveDomain {
        self.domain
    }
}

/// Stateful sealed door: zero admissions is valid and no instance can admit
/// more than one primitive.
#[derive(Debug, Default)]
pub struct ExactPrimitiveAdmissionDoor {
    admitted: bool,
}

impl ExactPrimitiveAdmissionDoor {
    pub fn admitted_count(&self) -> u32 {
        u32::from(self.admitted)
    }

    pub fn finite_domain(min_bits: u32, max_bits: u32) -> Result<PrimitiveDomain, OpcodeGateError> {
        PrimitiveDomain::from_bits(
            min_bits,
            max_bits,
            ExactPrimitiveDomainPolicy::FiniteOnlyRejectNanAndInfinity,
        )
    }

    pub fn special_value_preserving_domain(
        min_bits: u32,
        max_bits: u32,
    ) -> Result<PrimitiveDomain, OpcodeGateError> {
        PrimitiveDomain::from_bits(
            min_bits,
            max_bits,
            ExactPrimitiveDomainPolicy::PreserveIeeeNanInfinityAndSignedZero,
        )
    }

    pub fn verify_determinism(
        evidence: ExactPrimitiveDeterminismEvidence,
    ) -> Result<ExactPrimitiveDeterminismKey, OpcodeGateError> {
        if evidence.exhaustive_reference_digest == 0
            || evidence.supported_backend_replay_digest == 0
        {
            return Err(OpcodeGateError::IncompleteExactDeterminismEvidence);
        }
        if evidence.exhaustive_reference_digest != evidence.supported_backend_replay_digest {
            return Err(OpcodeGateError::ExactDeterminismReplayMismatch {
                reference_digest: evidence.exhaustive_reference_digest,
                replay_digest: evidence.supported_backend_replay_digest,
            });
        }
        Ok(ExactPrimitiveDeterminismKey {
            evidence_digest: evidence.exhaustive_reference_digest,
            domain: evidence.domain,
        })
    }

    pub fn verify_range_certificate(
        domain: PrimitiveDomain,
        evidence: ExactPrimitiveRangeEvidence<'_>,
        span: ExactPrimitiveSourceSpan,
    ) -> Result<ExactPrimitiveRangeCertificate, OpcodeGateError> {
        let (min_bits, max_bits) = match evidence {
            ExactPrimitiveRangeEvidence::PropertySubField(sub_field) => match sub_field.clamp {
                ClampBehavior::Bounded { min, max } => (min.to_bits(), max.to_bits()),
                ClampBehavior::Floored { .. } | ClampBehavior::Unbounded => {
                    return Err(OpcodeGateError::ExactPrimitiveRangeCertificateRequired {
                        span_start: span.start,
                        span_end: span.end,
                    });
                }
            },
            ExactPrimitiveRangeEvidence::GuardedFormula(guard) => {
                return Err(OpcodeGateError::ExactPrimitiveGuardIsNotRangeCertificate {
                    guard_opcode: guard.guard_opcode.raw(),
                    span_start: span.start,
                    span_end: span.end,
                });
            }
            ExactPrimitiveRangeEvidence::LiteralInRange { bits } => (bits, bits),
        };
        if !domain.contains_range(min_bits, max_bits) {
            return Err(OpcodeGateError::ExactPrimitiveRangeOutsideDomain {
                range_min_bits: min_bits,
                range_max_bits: max_bits,
                domain_min_bits: domain.min_bits,
                domain_max_bits: domain.max_bits,
                span_start: span.start,
                span_end: span.end,
            });
        }
        Ok(ExactPrimitiveRangeCertificate { domain })
    }

    pub fn admit_range_certified_sub_field(
        domain: PrimitiveDomain,
        sub_field: &SubFieldSpec,
        span_start: u32,
        span_end: u32,
    ) -> Result<(), OpcodeGateError> {
        let span = ExactPrimitiveSourceSpan::new(span_start, span_end);
        let certificate = Self::verify_range_certificate(
            domain,
            ExactPrimitiveRangeEvidence::PropertySubField(sub_field),
            span,
        )?;
        Self::admit_call_site(
            domain,
            Some(ExactPrimitiveCallSiteShape::RangeCertified(certificate)),
            span,
        )?;
        Ok(())
    }

    pub fn verify_guarded_semantics(
        domain: PrimitiveDomain,
        guard_opcode: EvalEmlOpcode,
        output_min_bits: u32,
        output_max_bits: u32,
        span: ExactPrimitiveSourceSpan,
    ) -> Result<ExactPrimitiveGuardedInput, OpcodeGateError> {
        if !matches!(
            guard_opcode.raw(),
            eml_nodes::opcode::CLAMP_BOUNDED | eml_nodes::opcode::MAX | eml_nodes::opcode::SELECT
        ) {
            return Err(OpcodeGateError::UnsupportedExactPrimitiveGuard {
                guard_opcode: guard_opcode.raw(),
                span_start: span.start,
                span_end: span.end,
            });
        }
        if !domain.contains_range(output_min_bits, output_max_bits) {
            return Err(OpcodeGateError::ExactPrimitiveRangeOutsideDomain {
                range_min_bits: output_min_bits,
                range_max_bits: output_max_bits,
                domain_min_bits: domain.min_bits,
                domain_max_bits: domain.max_bits,
                span_start: span.start,
                span_end: span.end,
            });
        }
        Ok(ExactPrimitiveGuardedInput {
            domain,
            guard_opcode,
        })
    }

    pub fn admit_explicitly_guarded_call_site(
        domain: PrimitiveDomain,
        guard_opcode: EvalEmlOpcode,
        output_min_bits: u32,
        output_max_bits: u32,
        span_start: u32,
        span_end: u32,
    ) -> Result<(), OpcodeGateError> {
        let span = ExactPrimitiveSourceSpan::new(span_start, span_end);
        let guarded = Self::verify_guarded_semantics(
            domain,
            guard_opcode,
            output_min_bits,
            output_max_bits,
            span,
        )?;
        Self::admit_call_site(
            domain,
            Some(ExactPrimitiveCallSiteShape::GuardedSemantics(guarded)),
            span,
        )?;
        Ok(())
    }

    pub fn admit_call_site(
        domain: PrimitiveDomain,
        shape: Option<ExactPrimitiveCallSiteShape>,
        span: ExactPrimitiveSourceSpan,
    ) -> Result<ExactPrimitiveCallSiteAdmission, OpcodeGateError> {
        let (kind, evidence_domain) = match shape {
            Some(ExactPrimitiveCallSiteShape::RangeCertified(certificate)) => (
                ExactPrimitiveCallSiteKind::RangeCertified,
                certificate.domain,
            ),
            Some(ExactPrimitiveCallSiteShape::GuardedSemantics(guarded)) => {
                (ExactPrimitiveCallSiteKind::GuardedSemantics, guarded.domain)
            }
            None => {
                return Err(OpcodeGateError::UnguardedExactPrimitiveCallSite {
                    span_start: span.start,
                    span_end: span.end,
                });
            }
        };
        if evidence_domain != domain {
            return Err(OpcodeGateError::ExactPrimitiveCallSiteDomainMismatch {
                span_start: span.start,
                span_end: span.end,
            });
        }
        Ok(ExactPrimitiveCallSiteAdmission { kind })
    }

    pub fn verify_cost(
        evidence: ExactPrimitiveCostEvidence,
    ) -> Result<ExactPrimitiveCostKey, OpcodeGateError> {
        let before = evidence.canonical_interpreter;
        let after = evidence.primitive_candidate;
        let no_regression = after.register_count <= before.register_count
            && after.binary_size_bytes <= before.binary_size_bytes
            && after.local_memory_bytes <= before.local_memory_bytes;
        let strict_improvement = after.register_count < before.register_count
            || after.binary_size_bytes < before.binary_size_bytes
            || after.local_memory_bytes < before.local_memory_bytes;
        if !no_regression || !strict_improvement {
            return Err(OpcodeGateError::ExactPrimitiveCostNotImproved);
        }
        Ok(ExactPrimitiveCostKey {
            resource_class: evidence.resource_class,
        })
    }

    pub fn verify_consumer(
        evidence: ExactPrimitiveConsumerEvidence,
    ) -> Result<ExactPrimitiveConsumerKey, OpcodeGateError> {
        // Necessity only — 5.14 deleted per-consumer exactness digest policing.
        // Cross-arm bit identity is a language-level witness, not admission evidence.
        if evidence.measured_threshold_excess_bps == 0 {
            return Err(OpcodeGateError::ExactPrimitiveConsumerNotNecessary);
        }
        Ok(ExactPrimitiveConsumerKey {
            consumer: evidence.consumer,
        })
    }

    pub fn admit(
        &mut self,
        request: ExactPrimitiveAdmissionRequest,
    ) -> Result<ExactPrimitiveAdmission, OpcodeGateError> {
        if self.admitted {
            return Err(OpcodeGateError::ExactPrimitiveLimitReached);
        }
        if request.name.trim().is_empty() {
            return Err(OpcodeGateError::ExactPrimitiveNameEmpty);
        }
        let determinism = request
            .determinism
            .ok_or(OpcodeGateError::MissingExactPrimitiveDeterminismKey)?;
        let cost = request
            .cost
            .ok_or(OpcodeGateError::MissingExactPrimitiveCostKey)?;
        let consumer = request
            .consumer
            .ok_or(OpcodeGateError::MissingExactPrimitiveConsumerKey)?;
        let _evidence_digest = determinism.evidence_digest;
        self.admitted = true;
        Ok(ExactPrimitiveAdmission {
            name: request.name,
            resource_class: cost.resource_class,
            consumer: consumer.consumer,
            domain: determinism.domain,
        })
    }
}

/// Errors from EvalEML opcode/combine admission.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum OpcodeGateError {
    #[error("unwhitelisted EvalEML opcode {opcode:#x} is not in the closed vocabulary")]
    UnwhitelistedOpcode { opcode: u32 },
    #[error(
        "unwhitelisted combine_kind {combine_kind} is not in the closed EvalEML/AO vocabulary"
    )]
    UnwhitelistedCombine { combine_kind: u32 },
    #[error(
        "semantic/scenario opcode `{name}` (opcode {proposed_opcode:#x}, tag `{scenario_or_policy_tag}`) is never admissible"
    )]
    SemanticOpcodeNeverAdmissible {
        name: String,
        proposed_opcode: u32,
        scenario_or_policy_tag: String,
    },
    #[error("semantic combine `{name}` (kind {combine_kind}) is never admissible")]
    SemanticCombineNeverAdmissible { name: String, combine_kind: u32 },
    #[error("CPU-oracle parity mismatch: cpu_bits={cpu_bits:#x} other_bits={other_bits:#x}")]
    ParityMismatch { cpu_bits: u32, other_bits: u32 },
    #[error(
        "generic primitive `{name}` (opcode {proposed_opcode:#x}) requires Tier-2 vocabulary expansion with DA scope; parity alone does not open the closed set"
    )]
    GenericPrimitiveRequiresVocabularyExpansion { name: String, proposed_opcode: u32 },
    #[error("generic primitive registration missing CPU-oracle parity proof")]
    MissingTier2Parity,
    #[error("exact primitive determinism evidence is incomplete")]
    IncompleteExactDeterminismEvidence,
    #[error(
        "exact primitive backend replay digest {replay_digest:#x} differs from exhaustive reference {reference_digest:#x}"
    )]
    ExactDeterminismReplayMismatch {
        reference_digest: u64,
        replay_digest: u64,
    },
    #[error(
        "exact primitive candidate has no strict non-regressing compiled-resource improvement"
    )]
    ExactPrimitiveCostNotImproved,
    #[error("exact primitive has no measured threshold excess for its concrete consumer")]
    ExactPrimitiveConsumerNotNecessary,
    #[error("exact primitive admission requires a determinism key")]
    MissingExactPrimitiveDeterminismKey,
    #[error("exact primitive admission requires a measured cost key")]
    MissingExactPrimitiveCostKey,
    #[error("exact primitive admission requires a concrete consumer key")]
    MissingExactPrimitiveConsumerKey,
    #[error("exact primitive admission permits at most one primitive")]
    ExactPrimitiveLimitReached,
    #[error("exact primitive name must not be empty")]
    ExactPrimitiveNameEmpty,
    #[error("exact primitive domain bounds are reversed: min_bits={min_bits:#010x} max_bits={max_bits:#010x}")]
    ExactPrimitiveDomainBoundsReversed { min_bits: u32, max_bits: u32 },
    #[error("finite-only exact primitive domain has a non-finite endpoint: min_bits={min_bits:#010x} max_bits={max_bits:#010x}")]
    ExactPrimitiveDomainSpecialEndpoint { min_bits: u32, max_bits: u32 },
    #[error("exact primitive input at {span_start}..{span_end} lacks a bounded property/sub-field range certificate")]
    ExactPrimitiveRangeCertificateRequired { span_start: u32, span_end: u32 },
    #[error("guard opcode {guard_opcode:#x} at {span_start}..{span_end} is formula semantics, not a range certificate")]
    ExactPrimitiveGuardIsNotRangeCertificate {
        guard_opcode: u32,
        span_start: u32,
        span_end: u32,
    },
    #[error("range [{range_min_bits:#010x}, {range_max_bits:#010x}] at {span_start}..{span_end} is outside exact primitive domain [{domain_min_bits:#010x}, {domain_max_bits:#010x}]")]
    ExactPrimitiveRangeOutsideDomain {
        range_min_bits: u32,
        range_max_bits: u32,
        domain_min_bits: u32,
        domain_max_bits: u32,
        span_start: u32,
        span_end: u32,
    },
    #[error(
        "opcode {guard_opcode:#x} at {span_start}..{span_end} is not an exact primitive guard"
    )]
    UnsupportedExactPrimitiveGuard {
        guard_opcode: u32,
        span_start: u32,
        span_end: u32,
    },
    #[error("exact primitive call at {span_start}..{span_end} is unguarded and uncertified")]
    UnguardedExactPrimitiveCallSite { span_start: u32, span_end: u32 },
    #[error("exact primitive call evidence at {span_start}..{span_end} was minted for a different domain")]
    ExactPrimitiveCallSiteDomainMismatch { span_start: u32, span_end: u32 },
}

// ── Closed vocabularies ───────────────────────────────────────────────────────

const CLOSED_OPCODES: &[u32] = &[
    eml_nodes::opcode::LITERAL_F32,
    eml_nodes::opcode::SLOT_VALUE,
    eml_nodes::opcode::PARAM,
    eml_nodes::opcode::TARGET_VALUE,
    eml_nodes::opcode::NEIGHBOR_VALUE,
    eml_nodes::opcode::ADD,
    eml_nodes::opcode::SUB,
    eml_nodes::opcode::MUL,
    eml_nodes::opcode::NEG,
    eml_nodes::opcode::DIV,
    eml_nodes::opcode::MIN,
    eml_nodes::opcode::MAX,
    eml_nodes::opcode::CLAMP_BOUNDED,
    eml_nodes::opcode::CLAMP_FLOORED,
    eml_nodes::opcode::ABS,
    eml_nodes::opcode::FLOOR,
    // EML-EXP-PRIMITIVE-0: the sole 5.11 vocabulary widening — the first
    // admitted exact primitive (full-domain EXP through the 5.10 door).
    eml_nodes::opcode::EXP,
    // EML-LN-PRIMITIVE-0: the sole 5.12 widening — LN over positive finite
    // normals (candidate LND4), promoted only after the exhaustive
    // admitted-domain three-arm sweep went green.
    eml_nodes::opcode::LN,
    eml_nodes::opcode::CMP_LT,
    eml_nodes::opcode::CMP_LE,
    eml_nodes::opcode::CMP_GT,
    eml_nodes::opcode::CMP_GE,
    eml_nodes::opcode::CMP_EQ,
    eml_nodes::opcode::SELECT,
    eml_nodes::opcode::RETURN_TOP,
];

const CLOSED_COMBINE_KINDS: &[u32] = &[
    combine_kind::IDENTITY,
    combine_kind::SUM,
    combine_kind::MEAN,
    combine_kind::MAX,
    combine_kind::MIN,
    combine_kind::WEIGHTED_MEAN,
    combine_kind::AFFINE_INTENT,
    combine_kind::PRODUCT,
    combine_kind::LAST_BY_PRIORITY,
    combine_kind::INTEGRATE_CLAMP,
    combine_kind::CROSSING_FORMULA,
    combine_kind::MIN_ACROSS_INPUTS,
    combine_kind::EVAL_EML,
    combine_kind::FIRST,
];

pub fn opcode_in_closed_vocabulary(op: u32) -> bool {
    CLOSED_OPCODES.contains(&op)
}

// ── EML-EXP-PRIMITIVE-0: the admitted EXP primitive ──────────────────────────

/// The admitted exact primitives' registry names (append-only semantics; a
/// different bit law is a future NEW name, never a mutation).
pub const EXP_PRIMITIVE_NAME: &str = "EXP";
pub const LN_PRIMITIVE_NAME: &str = "LN";

/// Canonical LN domain: positive finite normals `[2^-126, f32::MAX]`.
pub fn ln_primitive_domain() -> PrimitiveDomain {
    PrimitiveDomain::from_bits(
        simthing_core::eml_ln::EML_LN_DOMAIN_MIN_BITS,
        simthing_core::eml_ln::EML_LN_DOMAIN_MAX_BITS,
        ExactPrimitiveDomainPolicy::FiniteOnlyRejectNanAndInfinity,
    )
    .expect("pinned LN endpoint bits form an ordered finite binary32 interval")
}

/// Canonical full-domain EXP interval `[-87.33, +88.72]` (finite-only policy).
/// Every output over this domain is a positive normal finite f32.
pub fn exp_primitive_domain() -> PrimitiveDomain {
    PrimitiveDomain::from_bits(
        simthing_core::EML_EXP_DOMAIN_MIN_BITS,
        simthing_core::EML_EXP_DOMAIN_MAX_BITS,
        ExactPrimitiveDomainPolicy::FiniteOnlyRejectNanAndInfinity,
    )
    .expect("pinned EXP endpoint bits form an ordered finite binary32 interval")
}

/// Tree-scan call-site admission for `EXP` nodes (the production wiring of the
/// 5.10 shapes). Each `EXP` node must be immediately preceded in postfix by:
///  - `CLAMP_BOUNDED` whose authored bounds lie inside the EXP domain —
///    shape 2, the guard IS the authored (saturated) semantics; or
///  - `LITERAL_F32` whose value lies inside the EXP domain — shape 1, the
///    literal is its own range certificate.
/// Anything else is the spanned unguarded/uncertified admission error. Spans
/// are node indices (`exp_index..exp_index+1`). Richer shape-1 certificates
/// (bounded sub-field ranges) remain available through
/// [`ExactPrimitiveAdmissionDoor::admit_range_certified_sub_field`] where a
/// `SubFieldSpec` is in scope; the tree scan admits only what it can verify
/// mechanically from node data.
pub fn admit_exp_call_sites(nodes: &[EmlNode]) -> Result<(), OpcodeGateError> {
    for (index, node) in nodes.iter().enumerate() {
        let domain = match node.opcode {
            eml_nodes::opcode::EXP => exp_primitive_domain(),
            eml_nodes::opcode::LN => ln_primitive_domain(),
            _ => continue,
        };
        let span = ExactPrimitiveSourceSpan::new(index as u32, index as u32 + 1);
        let shape = match index.checked_sub(1).map(|prev_index| &nodes[prev_index]) {
            Some(prev) if prev.opcode == eml_nodes::opcode::CLAMP_BOUNDED => {
                let guard_opcode = EvalEmlOpcode::from_closed(eml_nodes::opcode::CLAMP_BOUNDED)?;
                let guarded = ExactPrimitiveAdmissionDoor::verify_guarded_semantics(
                    domain,
                    guard_opcode,
                    prev.a,
                    prev.b,
                    span,
                )?;
                Some(ExactPrimitiveCallSiteShape::GuardedSemantics(guarded))
            }
            Some(prev) if prev.opcode == eml_nodes::opcode::LITERAL_F32 => {
                let certificate = ExactPrimitiveAdmissionDoor::verify_range_certificate(
                    domain,
                    ExactPrimitiveRangeEvidence::LiteralInRange { bits: prev.a },
                    span,
                )?;
                Some(ExactPrimitiveCallSiteShape::RangeCertified(certificate))
            }
            _ => None,
        };
        ExactPrimitiveAdmissionDoor::admit_call_site(domain, shape, span)?;
    }
    Ok(())
}

/// Slot-local AccumulatorOp EvalEML excludes the two field-context reads.
/// Those opcodes are closed and admitted only by `FieldSweepRegistration`.
pub fn opcode_in_accumulator_vocabulary(op: u32) -> bool {
    opcode_in_closed_vocabulary(op)
        && !matches!(
            op,
            eml_nodes::opcode::TARGET_VALUE | eml_nodes::opcode::NEIGHBOR_VALUE
        )
}

pub fn combine_in_closed_vocabulary(kind: u32) -> bool {
    CLOSED_COMBINE_KINDS.contains(&kind)
}

fn combine_fn_to_kind(cf: &CombineFn) -> u32 {
    match cf {
        CombineFn::Identity => combine_kind::IDENTITY,
        CombineFn::Sum => combine_kind::SUM,
        CombineFn::Mean => combine_kind::MEAN,
        CombineFn::Max => combine_kind::MAX,
        CombineFn::Min => combine_kind::MIN,
        CombineFn::WeightedMean { .. } => combine_kind::WEIGHTED_MEAN,
        CombineFn::Product => combine_kind::PRODUCT,
        CombineFn::LastByPriority => combine_kind::LAST_BY_PRIORITY,
        CombineFn::IntegrateWithClamp { .. } => combine_kind::INTEGRATE_CLAMP,
        CombineFn::CrossingFormula { .. } => combine_kind::CROSSING_FORMULA,
        CombineFn::MinAcrossInputs => combine_kind::MIN_ACROSS_INPUTS,
        CombineFn::EvalEML { .. } => combine_kind::EVAL_EML,
    }
}

/// SoftStep policy-conditional gadget template (branchless branching as column data).
///
/// Policy conditional:
///   input columns → SoftStep predicate → weighted branch A/B contribution → accumulator column
///
/// No if/else WGSL. No semantic opcode. No scenario-specific combine.
#[derive(Clone, Debug, PartialEq)]
pub struct SoftStepPolicyConditional {
    pub input_col: u32,
    pub center: f32,
    pub steepness: f32,
    pub branch_a_col: u32,
    pub branch_b_col: u32,
    pub output_col: u32,
}

impl SoftStepPolicyConditional {
    /// Compile to closed-vocabulary EvalEML nodes:
    /// `out = softstep(x) * A + (1 - softstep(x)) * B`
    /// using only LITERAL/SLOT/ADD/SUB/MUL/ABS/DIV/RETURN_TOP.
    pub fn compile_nodes(&self) -> Result<Vec<EmlNode>, OpcodeGateError> {
        // out = B + softstep(x) * (A - B); algebraic SoftStep; closed opcodes only.
        let mut nodes = Vec::new();
        // A - B
        nodes.push(slot(self.branch_a_col));
        nodes.push(slot(self.branch_b_col));
        nodes.push(bin(eml_nodes::opcode::SUB));
        // soft * (A - B)
        nodes.extend(softstep_nodes(self.input_col, self.center, self.steepness));
        nodes.push(bin(eml_nodes::opcode::MUL));
        // B + soft*(A-B)
        nodes.push(slot(self.branch_b_col));
        nodes.push(bin(eml_nodes::opcode::ADD));
        nodes.push(bin(eml_nodes::opcode::RETURN_TOP));
        OpcodeRegistrationGate::admit_tree_nodes(&nodes)?;
        Ok(nodes)
    }

    /// CPU oracle for the policy conditional (bit-exact with algebraic SoftStep).
    pub fn oracle(&self, input: f32, branch_a: f32, branch_b: f32) -> f32 {
        let soft = oracle_soft_step(input, self.center, self.steepness);
        branch_b + soft * (branch_a - branch_b)
    }
}

fn oracle_soft_step(x: f32, center: f32, steepness: f32) -> f32 {
    let u = steepness * (x - center);
    0.5 + 0.5 * u / (1.0 + u.abs())
}

fn softstep_nodes(input_col: u32, center: f32, steepness: f32) -> Vec<EmlNode> {
    let mut nodes = Vec::new();
    // u = steepness * (x - center)
    nodes.push(slot(input_col));
    nodes.push(lit(center));
    nodes.push(bin(eml_nodes::opcode::SUB));
    nodes.push(lit(steepness));
    nodes.push(bin(eml_nodes::opcode::MUL));
    // keep u: recompute path for 1+abs(u)
    // stack: [u]
    // duplicate u via recompute:
    nodes.push(slot(input_col));
    nodes.push(lit(center));
    nodes.push(bin(eml_nodes::opcode::SUB));
    nodes.push(lit(steepness));
    nodes.push(bin(eml_nodes::opcode::MUL));
    // stack: [u, u]
    nodes.push(bin(eml_nodes::opcode::ABS));
    nodes.push(lit(1.0));
    nodes.push(bin(eml_nodes::opcode::ADD));
    // stack: [u, 1+abs(u)]
    nodes.push(div_safe());
    // stack: [u/(1+abs(u))]
    nodes.push(lit(0.5));
    nodes.push(bin(eml_nodes::opcode::MUL));
    nodes.push(lit(0.5));
    nodes.push(bin(eml_nodes::opcode::ADD));
    // stack: [softstep]
    nodes
}

/// EML-EXP-PRIMITIVE-0 consumer form: the **stabilized** softmax weight
/// `EXP(β·(zᵢ − max z))` as authored data over existing machinery — `max z`
/// arrives from the existing `MAX` reduction band in `max_col`, the weight map
/// rides ordinary EvalEML, and normalization rides the existing `Sum` band.
///
/// Every exponential argument is `≤ 0` by construction and explicitly
/// `CLAMP_BOUNDED`-guarded (5.10 shape 2 — saturated tail below the domain
/// floor, where the true weight is `< 1.2e-38` and semantically zero). The
/// naive unstabilized `EXP(β·zᵢ)` form is NOT the canonical softmax: it is
/// numerically unsound regardless of domain policy, and its unguarded call
/// site cannot pass [`admit_exp_call_sites`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SoftmaxWeightGadget {
    pub z_col: u32,
    pub max_col: u32,
    pub beta: f32,
}

impl SoftmaxWeightGadget {
    /// Compile to closed-vocabulary nodes:
    /// `EXP(CLAMP(MUL(beta, SUB(z, max)), domain_min, 0))`, `RETURN_TOP`.
    /// 7 nodes, peak stack 2 — `CompactStack4`.
    pub fn compile_nodes(&self) -> Result<Vec<EmlNode>, OpcodeGateError> {
        let nodes = vec![
            slot(self.z_col),
            slot(self.max_col),
            bin(eml_nodes::opcode::SUB),
            lit(self.beta),
            bin(eml_nodes::opcode::MUL),
            saturation_clamp_guard(),
            bin(eml_nodes::opcode::EXP),
            bin(eml_nodes::opcode::RETURN_TOP),
        ];
        OpcodeRegistrationGate::admit_tree_nodes(&nodes)?;
        admit_exp_call_sites(&nodes)?;
        Ok(nodes)
    }

    /// CPU twin (bit-exact with the compiled node program).
    pub fn oracle(&self, z: f32, max_z: f32) -> f32 {
        let arg = ((z - max_z) * self.beta).clamp(
            f32::from_bits(simthing_core::EML_EXP_DOMAIN_MIN_BITS),
            f32::from_bits(simthing_core::EML_EXP_SATURATION_CEILING_BITS),
        );
        simthing_core::eml_exp_pinned_f32(arg)
    }
}

/// EML-LN-PRIMITIVE-0 consumers — authored EML data over the completed
/// vocabulary; no new opcode, no new mechanism (full_eml_unification §5).
pub struct LnConsumerGadgets;

impl LnConsumerGadgets {
    fn ln_guard() -> EmlNode {
        EmlNode {
            opcode: eml_nodes::opcode::CLAMP_BOUNDED,
            flags: 0,
            a: simthing_core::eml_ln::EML_LN_DOMAIN_MIN_BITS,
            b: simthing_core::eml_ln::EML_LN_DOMAIN_MAX_BITS,
            c: 0,
            d: 0,
        }
    }

    /// `PowerLaw(x; a) = EXP(a * LN(x))` — the entire power-law family with
    /// no third opcode; both primitive calls carry 5.10 shape-2 guards.
    pub fn power_law_nodes(x_col: u32, a: f32) -> Result<Vec<EmlNode>, OpcodeGateError> {
        let nodes = vec![
            slot(x_col),
            Self::ln_guard(),
            bin(eml_nodes::opcode::LN),
            lit(a),
            bin(eml_nodes::opcode::MUL),
            saturation_exp_full_guard(),
            bin(eml_nodes::opcode::EXP),
            bin(eml_nodes::opcode::RETURN_TOP),
        ];
        OpcodeRegistrationGate::admit_tree_nodes(&nodes)?;
        admit_exp_call_sites(&nodes)?;
        Ok(nodes)
    }

    /// `eml(x, y) = SUB(EXP(x), LN(y))` — the operator the interpreter is
    /// named for, as a guarded three-op library entry (Anchor B literal).
    pub fn eml_operator_nodes(x_col: u32, y_col: u32) -> Result<Vec<EmlNode>, OpcodeGateError> {
        let nodes = vec![
            slot(x_col),
            saturation_exp_full_guard(),
            bin(eml_nodes::opcode::EXP),
            slot(y_col),
            Self::ln_guard(),
            bin(eml_nodes::opcode::LN),
            bin(eml_nodes::opcode::SUB),
            bin(eml_nodes::opcode::RETURN_TOP),
        ];
        OpcodeRegistrationGate::admit_tree_nodes(&nodes)?;
        admit_exp_call_sites(&nodes)?;
        Ok(nodes)
    }

    /// Entropy term `-p * LN(p)` with the measure-theoretic `p = 0` case
    /// authored away via `SELECT` (diagnostic lane).
    pub fn entropy_term_nodes(p_col: u32) -> Result<Vec<EmlNode>, OpcodeGateError> {
        let nodes = vec![
            slot(p_col),
            lit(0.0),
            bin(eml_nodes::opcode::CMP_GT),
            slot(p_col),
            bin(eml_nodes::opcode::NEG),
            slot(p_col),
            Self::ln_guard(),
            bin(eml_nodes::opcode::LN),
            bin(eml_nodes::opcode::MUL),
            lit(0.0),
            bin(eml_nodes::opcode::SELECT),
            bin(eml_nodes::opcode::RETURN_TOP),
        ];
        OpcodeRegistrationGate::admit_tree_nodes(&nodes)?;
        admit_exp_call_sites(&nodes)?;
        Ok(nodes)
    }

    /// `LogAccumulate` map: `LN(x)` guarded — a NEW authored numerical law
    /// riding the existing Sum lane unchanged (never a Product substitution).
    pub fn log_accumulate_map_nodes(x_col: u32) -> Result<Vec<EmlNode>, OpcodeGateError> {
        let nodes = vec![
            slot(x_col),
            Self::ln_guard(),
            bin(eml_nodes::opcode::LN),
            bin(eml_nodes::opcode::RETURN_TOP),
        ];
        OpcodeRegistrationGate::admit_tree_nodes(&nodes)?;
        admit_exp_call_sites(&nodes)?;
        Ok(nodes)
    }
}

/// EXP full-domain guard node for gadget composition.
fn saturation_exp_full_guard() -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::CLAMP_BOUNDED,
        flags: 0,
        a: simthing_core::EML_EXP_DOMAIN_MIN_BITS,
        b: simthing_core::EML_EXP_DOMAIN_MAX_BITS,
        c: 0,
        d: 0,
    }
}

/// The canonical saturated-tail guard node: `CLAMP_BOUNDED(x, domain_min, 0)`.
fn saturation_clamp_guard() -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::CLAMP_BOUNDED,
        flags: 0,
        a: simthing_core::EML_EXP_DOMAIN_MIN_BITS,
        b: simthing_core::EML_EXP_SATURATION_CEILING_BITS,
        c: 0,
        d: 0,
    }
}

fn lit(v: f32) -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::LITERAL_F32,
        flags: 0,
        a: v.to_bits(),
        b: 0,
        c: 0,
        d: 0,
    }
}

fn slot(col: u32) -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::SLOT_VALUE,
        flags: 0,
        a: col,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn bin(opcode: u32) -> EmlNode {
    EmlNode {
        opcode,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn div_safe() -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::DIV,
        flags: 1, // safe division flag
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlTreeId,
    };

    const UNWHITELISTED_OPCODE: u32 = 0xDEAD_BEEF;
    const UNWHITELISTED_COMBINE: u32 = 0x00C0_FFEE;

    fn forged_node(opcode: u32) -> EmlNode {
        EmlNode {
            opcode,
            flags: 0,
            a: 1f32.to_bits(),
            b: 0,
            c: 0,
            d: 0,
        }
    }

    fn determinism_key() -> ExactPrimitiveDeterminismKey {
        ExactPrimitiveAdmissionDoor::verify_determinism(ExactPrimitiveDeterminismEvidence {
            bit_semantics: ExactPrimitiveBitSemantics::Ieee754Binary32Bits,
            domain: PrimitiveDomain::from_bits(
                f32::NEG_INFINITY.to_bits(),
                f32::INFINITY.to_bits(),
                ExactPrimitiveDomainPolicy::PreserveIeeeNanInfinityAndSignedZero,
            )
            .expect("ordered binary32 domain"),
            exhaustive_reference_digest: 0x5eed,
            supported_backend_replay_digest: 0x5eed,
        })
        .expect("matching exhaustive replay")
    }

    fn cost_key() -> ExactPrimitiveCostKey {
        ExactPrimitiveAdmissionDoor::verify_cost(ExactPrimitiveCostEvidence {
            resource_class: EmlResourceClass::CompactStack4,
            canonical_interpreter: ExactPrimitiveResourceEffect {
                register_count: 31,
                binary_size_bytes: 21_504,
                local_memory_bytes: 64,
            },
            primitive_candidate: ExactPrimitiveResourceEffect {
                register_count: 30,
                binary_size_bytes: 21_504,
                local_memory_bytes: 64,
            },
        })
        .expect("strict non-regressing measured cost")
    }

    fn consumer_key() -> ExactPrimitiveConsumerKey {
        // 5.10-era door-shape fixture: exercises the necessity gate only, so
        // it declares itself non-exact-bearing (no digest obligation attaches).
        ExactPrimitiveAdmissionDoor::verify_consumer(ExactPrimitiveConsumerEvidence {
            consumer: ExactPrimitiveConsumer::FieldSweepEvalEml,
            measured_threshold_excess_bps: 1,
        })
        .expect("concrete measured consumer")
    }

    #[test]
    fn exact_primitive_admission_requires_every_independent_key_and_zero_is_valid() {
        let mut door = ExactPrimitiveAdmissionDoor::default();
        assert_eq!(door.admitted_count(), 0);
        assert_eq!(
            door.admit(ExactPrimitiveAdmissionRequest {
                name: "candidate".to_owned(),
                determinism: None,
                cost: Some(cost_key()),
                consumer: Some(consumer_key()),
            }),
            Err(OpcodeGateError::MissingExactPrimitiveDeterminismKey)
        );
        assert_eq!(
            door.admit(ExactPrimitiveAdmissionRequest {
                name: "candidate".to_owned(),
                determinism: Some(determinism_key()),
                cost: None,
                consumer: Some(consumer_key()),
            }),
            Err(OpcodeGateError::MissingExactPrimitiveCostKey)
        );
        assert_eq!(
            door.admit(ExactPrimitiveAdmissionRequest {
                name: "candidate".to_owned(),
                determinism: Some(determinism_key()),
                cost: Some(cost_key()),
                consumer: None,
            }),
            Err(OpcodeGateError::MissingExactPrimitiveConsumerKey)
        );
        assert_eq!(
            door.admitted_count(),
            0,
            "no primitive is promised or admitted"
        );
    }

    #[test]
    fn exact_primitive_evidence_rejects_backend_domain_and_cost_substitutes() {
        assert_eq!(
            ExactPrimitiveAdmissionDoor::verify_determinism(ExactPrimitiveDeterminismEvidence {
                bit_semantics: ExactPrimitiveBitSemantics::Ieee754Binary32Bits,
                domain: PrimitiveDomain::from_bits(
                    (-1.0f32).to_bits(),
                    1.0f32.to_bits(),
                    ExactPrimitiveDomainPolicy::FiniteOnlyRejectNanAndInfinity,
                )
                .expect("finite domain"),
                exhaustive_reference_digest: 0,
                supported_backend_replay_digest: 0,
            }),
            Err(OpcodeGateError::IncompleteExactDeterminismEvidence)
        );
        assert!(matches!(
            ExactPrimitiveAdmissionDoor::verify_determinism(ExactPrimitiveDeterminismEvidence {
                bit_semantics: ExactPrimitiveBitSemantics::Ieee754Binary32Bits,
                domain: PrimitiveDomain::from_bits(
                    (-1.0f32).to_bits(),
                    1.0f32.to_bits(),
                    ExactPrimitiveDomainPolicy::FiniteOnlyRejectNanAndInfinity,
                )
                .expect("finite domain"),
                exhaustive_reference_digest: 1,
                supported_backend_replay_digest: 2,
            }),
            Err(OpcodeGateError::ExactDeterminismReplayMismatch { .. })
        ));
        assert_eq!(
            ExactPrimitiveAdmissionDoor::verify_cost(ExactPrimitiveCostEvidence {
                resource_class: EmlResourceClass::CompactStack4,
                canonical_interpreter: ExactPrimitiveResourceEffect {
                    register_count: 31,
                    binary_size_bytes: 21_504,
                    local_memory_bytes: 64,
                },
                primitive_candidate: ExactPrimitiveResourceEffect {
                    register_count: 31,
                    binary_size_bytes: 21_504,
                    local_memory_bytes: 64,
                },
            }),
            Err(OpcodeGateError::ExactPrimitiveCostNotImproved)
        );
        assert_eq!(
            ExactPrimitiveAdmissionDoor::verify_consumer(ExactPrimitiveConsumerEvidence {
                consumer: ExactPrimitiveConsumer::FieldSweepEvalEml,
                measured_threshold_excess_bps: 0,
            }),
            Err(OpcodeGateError::ExactPrimitiveConsumerNotNecessary)
        );
    }

    #[test]
    fn exact_primitive_domain_call_sites_are_sealed_spanned_and_mutant_sensitive() {
        fn unguarded_contract(
            admit: impl FnOnce() -> Result<ExactPrimitiveCallSiteAdmission, OpcodeGateError>,
        ) -> bool {
            matches!(
                admit(),
                Err(OpcodeGateError::UnguardedExactPrimitiveCallSite {
                    span_start: 41,
                    span_end: 57
                })
            )
        }

        let domain = PrimitiveDomain::from_bits(
            (-1.0f32).to_bits(),
            1.0f32.to_bits(),
            ExactPrimitiveDomainPolicy::FiniteOnlyRejectNanAndInfinity,
        )
        .expect("cross-sign finite domain");
        assert!(domain.contains_bits((-0.0f32).to_bits()));
        assert!(domain.contains_bits(0.0f32.to_bits()));
        assert!(!domain.contains_bits(f32::INFINITY.to_bits()));
        assert_eq!(
            PrimitiveDomain::from_bits(
                f32::NEG_INFINITY.to_bits(),
                f32::INFINITY.to_bits(),
                ExactPrimitiveDomainPolicy::FiniteOnlyRejectNanAndInfinity,
            ),
            Err(OpcodeGateError::ExactPrimitiveDomainSpecialEndpoint {
                min_bits: f32::NEG_INFINITY.to_bits(),
                max_bits: f32::INFINITY.to_bits(),
            })
        );
        assert_eq!(
            PrimitiveDomain::from_bits(
                1.0f32.to_bits(),
                (-1.0f32).to_bits(),
                ExactPrimitiveDomainPolicy::FiniteOnlyRejectNanAndInfinity,
            ),
            Err(OpcodeGateError::ExactPrimitiveDomainBoundsReversed {
                min_bits: 1.0f32.to_bits(),
                max_bits: (-1.0f32).to_bits(),
            })
        );

        let span = ExactPrimitiveSourceSpan::new(41, 57);
        assert!(unguarded_contract(|| {
            ExactPrimitiveAdmissionDoor::admit_call_site(domain, None, span)
        }));
        assert!(
            !unguarded_contract(|| Ok(ExactPrimitiveCallSiteAdmission {
                kind: ExactPrimitiveCallSiteKind::RangeCertified,
            })),
            "planted unguarded-admission defect must be RED"
        );

        let sub_field = SubFieldSpec {
            role: simthing_core::SubFieldRole::Amount,
            width: 1,
            clamp: ClampBehavior::Bounded { min: 0.0, max: 1.0 },
            velocity_max: None,
            default: 0.0,
            display_name: "amount".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        };
        let certificate = ExactPrimitiveAdmissionDoor::verify_range_certificate(
            domain,
            ExactPrimitiveRangeEvidence::PropertySubField(&sub_field),
            span,
        )
        .expect("bounded sub-field range is inside primitive domain");
        let certified = ExactPrimitiveAdmissionDoor::admit_call_site(
            domain,
            Some(ExactPrimitiveCallSiteShape::RangeCertified(certificate)),
            span,
        )
        .expect("sealed property range admits shape one");
        assert_eq!(certified.kind(), ExactPrimitiveCallSiteKind::RangeCertified);
        assert_eq!(certified.added_runtime_nodes(), 0);
        ExactPrimitiveAdmissionDoor::admit_range_certified_sub_field(domain, &sub_field, 41, 57)
            .expect("public shape-one facade preserves the sealed proof path");

        let clamp = EvalEmlOpcode::from_closed(eml_nodes::opcode::CLAMP_BOUNDED)
            .expect("closed clamp opcode");
        let guarded = ExactPrimitiveAdmissionDoor::verify_guarded_semantics(
            domain,
            clamp,
            0.0f32.to_bits(),
            1.0f32.to_bits(),
            span,
        )
        .expect("authored clamp output is inside primitive domain");
        let guarded_call = ExactPrimitiveAdmissionDoor::admit_call_site(
            domain,
            Some(ExactPrimitiveCallSiteShape::GuardedSemantics(guarded)),
            span,
        )
        .expect("explicit formula guard admits shape two");
        assert_eq!(
            guarded_call.kind(),
            ExactPrimitiveCallSiteKind::GuardedSemantics
        );
        ExactPrimitiveAdmissionDoor::admit_explicitly_guarded_call_site(
            domain,
            clamp,
            0.0f32.to_bits(),
            1.0f32.to_bits(),
            41,
            57,
        )
        .expect("public shape-two facade preserves guarded semantics");

        let guard_as_certificate = ExactPrimitiveAdmissionDoor::verify_range_certificate(
            domain,
            ExactPrimitiveRangeEvidence::GuardedFormula(&guarded),
            span,
        );
        assert_eq!(
            guard_as_certificate,
            Err(OpcodeGateError::ExactPrimitiveGuardIsNotRangeCertificate {
                guard_opcode: eml_nodes::opcode::CLAMP_BOUNDED,
                span_start: 41,
                span_end: 57,
            })
        );
        let planted_guard_as_certificate =
            Ok::<_, OpcodeGateError>(ExactPrimitiveRangeCertificate { domain });
        assert_ne!(
            guard_as_certificate, planted_guard_as_certificate,
            "planted guard-as-certificate defect must be RED"
        );
        assert_eq!(ExactPrimitiveAdmissionDoor::default().admitted_count(), 0);
    }

    // ── EML-EXP-PRIMITIVE-0 ──────────────────────────────────────────────────

    fn exp_node() -> EmlNode {
        EmlNode {
            opcode: eml_nodes::opcode::EXP,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        }
    }

    fn returning(mut nodes: Vec<EmlNode>) -> Vec<EmlNode> {
        nodes.push(bin(eml_nodes::opcode::RETURN_TOP));
        nodes
    }

    #[test]
    fn eml_exp_primitive_0_vocabulary_widens_by_exactly_exp() {
        let closed = EvalEmlVocabulary::closed_opcodes();
        assert_eq!(
            closed
                .iter()
                .filter(|op| **op == eml_nodes::opcode::EXP)
                .count(),
            1,
            "EXP appears exactly once in the closed vocabulary"
        );
        assert_eq!(
            closed
                .iter()
                .filter(|op| **op == eml_nodes::opcode::LN)
                .count(),
            1,
            "LN appears exactly once (5.12 widening)"
        );
        assert_eq!(
            closed.len(),
            25,
            "5.11+5.12 widen the 23-opcode roster by exactly two"
        );
        assert!(opcode_in_accumulator_vocabulary(eml_nodes::opcode::EXP));
        let domain = exp_primitive_domain();
        assert_eq!(domain.min_bits(), simthing_core::EML_EXP_DOMAIN_MIN_BITS);
        assert_eq!(domain.max_bits(), simthing_core::EML_EXP_DOMAIN_MAX_BITS);
        assert_eq!(
            domain.special_value_policy(),
            ExactPrimitiveDomainPolicy::FiniteOnlyRejectNanAndInfinity
        );
    }

    #[test]
    fn eml_exp_primitive_0_call_sites_admit_only_the_two_shapes_and_span_the_rest() {
        // Shape 2: saturation clamp guard immediately precedes EXP.
        let guarded = returning(vec![slot(0), saturation_clamp_guard(), exp_node()]);
        admit_exp_call_sites(&guarded).expect("clamp-guarded EXP admits (shape 2)");

        // Shape 1: in-domain literal immediately precedes EXP.
        let certified = returning(vec![lit(-1.5), exp_node()]);
        admit_exp_call_sites(&certified).expect("in-domain literal EXP admits (shape 1)");

        // The naive unstabilized softmax shape — EXP over a raw product — is
        // the spanned unguarded admission error, not authored data.
        let naive = returning(vec![
            slot(0),
            lit(2.0),
            bin(eml_nodes::opcode::MUL),
            exp_node(),
        ]);
        assert_eq!(
            admit_exp_call_sites(&naive),
            Err(OpcodeGateError::UnguardedExactPrimitiveCallSite {
                span_start: 3,
                span_end: 4,
            })
        );

        // EXP with no operand author at all (first node) is unguarded.
        assert_eq!(
            admit_exp_call_sites(&returning(vec![exp_node()])),
            Err(OpcodeGateError::UnguardedExactPrimitiveCallSite {
                span_start: 0,
                span_end: 1,
            })
        );

        // A clamp whose authored bounds leave the primitive domain is not a
        // lawful guard.
        let wide_clamp = EmlNode {
            opcode: eml_nodes::opcode::CLAMP_BOUNDED,
            flags: 0,
            a: (-100000.0f32).to_bits(),
            b: 0.0f32.to_bits(),
            c: 0,
            d: 0,
        };
        assert!(matches!(
            admit_exp_call_sites(&returning(vec![slot(0), wide_clamp, exp_node()])),
            Err(OpcodeGateError::ExactPrimitiveRangeOutsideDomain { .. })
        ));

        // An out-of-domain literal is not a certificate.
        assert!(matches!(
            admit_exp_call_sites(&returning(vec![lit(90.0), exp_node()])),
            Err(OpcodeGateError::ExactPrimitiveRangeOutsideDomain { .. })
        ));

        // Planted gate-bypass mutant must be RED: an unguarded site admitted
        // as if certified is never what the gate returns.
        let bypass: Result<(), OpcodeGateError> = Ok(());
        assert_ne!(
            admit_exp_call_sites(&naive),
            bypass,
            "planted unguarded-bypass defect must be RED"
        );
    }

    #[test]
    fn eml_exp_primitive_0_softmax_weight_gadget_is_stabilized_authored_data() {
        let gadget = SoftmaxWeightGadget {
            z_col: 0,
            max_col: 1,
            beta: 2.5,
        };
        let nodes = gadget.compile_nodes().expect("stabilized softmax admits");
        assert_eq!(nodes.len(), 8);
        // CPU oracle parity against the shared accumulator-op stack machine.
        let values = [
            [0.0f32, 3.0],
            [1.25, 3.0],
            [3.0, 3.0],
            [-250.0, 3.0], // saturated tail: clamped at the domain floor
        ];
        for row in values {
            let via_interpreter = crate::accumulator_op::eval_eml_cpu(&nodes, 0, &row, 2, [0.0; 4]);
            assert_eq!(
                via_interpreter.to_bits(),
                gadget.oracle(row[0], row[1]).to_bits(),
                "softmax weight oracle parity at z={} max={}",
                row[0],
                row[1]
            );
            assert!(via_interpreter > 0.0 && via_interpreter <= 1.0);
        }
    }
}
