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
use simthing_core::CombineFn;

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
/// ```compile_fail
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
    pub fn from_bit_exact_sample(cpu_bits: u32, gpu_or_twin_bits: u32) -> Result<Self, OpcodeGateError> {
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
    RawUnvalidated { combine_kind: u32 },
    SemanticNamed { name: String, combine_kind: u32 },
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
            OpcodeRegistrationRequest::ExistingClosed(op) => Ok(AdmittedEvalEmlOpcode {
                opcode: op.raw(),
            }),
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
                Err(OpcodeGateError::GenericPrimitiveRequiresVocabularyExpansion {
                    name: reg.name,
                    proposed_opcode: reg.proposed_opcode,
                })
            }
            OpcodeRegistrationRequest::Semantic(reg) => Err(OpcodeGateError::SemanticOpcodeNeverAdmissible {
                name: reg.name,
                proposed_opcode: reg.proposed_opcode,
                scenario_or_policy_tag: reg.scenario_or_policy_tag,
            }),
        }
    }

    /// Admit a combine registration request.
    pub fn admit_combine(
        request: CombineRegistrationRequest,
    ) -> Result<AdmittedEvalEmlCombine, OpcodeGateError> {
        match request {
            CombineRegistrationRequest::ExistingClosed(c) => Ok(AdmittedEvalEmlCombine {
                kind: c.raw(),
            }),
            CombineRegistrationRequest::RawUnvalidated { combine_kind } => {
                EvalEmlCombine::from_closed_kind(combine_kind).map(|c| AdmittedEvalEmlCombine {
                    kind: c.raw(),
                })
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
        Self::admit_combine(CombineRegistrationRequest::RawUnvalidated {
            combine_kind: kind,
        })
    }
}

/// Errors from EvalEML opcode/combine admission.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum OpcodeGateError {
    #[error("unwhitelisted EvalEML opcode {opcode:#x} is not in the closed vocabulary")]
    UnwhitelistedOpcode { opcode: u32 },
    #[error("unwhitelisted combine_kind {combine_kind} is not in the closed EvalEML/AO vocabulary")]
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
    GenericPrimitiveRequiresVocabularyExpansion {
        name: String,
        proposed_opcode: u32,
    },
    #[error("generic primitive registration missing CPU-oracle parity proof")]
    MissingTier2Parity,
}

// ── Closed vocabularies ───────────────────────────────────────────────────────

const CLOSED_OPCODES: &[u32] = &[
    eml_nodes::opcode::LITERAL_F32,
    eml_nodes::opcode::SLOT_VALUE,
    eml_nodes::opcode::PARAM,
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

    /// Forgeability: unwhitelisted opcode nodes remain freely constructible and can
    /// be assembled into an EvalEML registration *payload* today (POD residual on
    /// `EmlNode.opcode: u32`). Pre-door, `EmlGpuProgramTable::upload_trees` accepted
    /// such nodes without vocabulary check — that path is now gated; residual is free
    /// construction + free raw registration-request assembly.
    #[test]
    fn oc_k_eml_opcode_gate_0_unwhitelisted_opcode_registers_today() {
        let forged = forged_node(UNWHITELISTED_OPCODE);
        assert_eq!(forged.opcode, UNWHITELISTED_OPCODE);
        // Payload is representable without OpcodeRegistrationGate.
        let payload: Vec<EmlNode> = vec![forged];
        assert_eq!(payload[0].opcode, UNWHITELISTED_OPCODE);
        // Historical forge: formula registration path still accepts Vec<EmlNode> with
        // raw u32 opcodes at the type level (admission hard-errors — see blocked test).
        let mut reg = EmlExpressionRegistry::new();
        let meta = EmlFormulaMeta {
            tree_id: EmlTreeId(99),
            execution_class: EmlExecutionClass::ExactDeterministic,
            allowed_consumers: EmlConsumerMask(0),
            max_abs_error: None,
            deterministic_gpu: true,
            requires_guard_for_hard_threshold: false,
            node_count: 0,
            max_stack_depth: 0,
            has_loops: false,
            has_recursion: false,
            display_name: "forged".into(),
        };
        // Compiles and runs: registration API is open to raw opcode payloads.
        let attempt = reg.register_formula(EmlTreeId(99), meta, payload);
        // Runtime already rejected unknown stack opcodes; gate makes vocabulary class explicit.
        assert!(attempt.is_err());
        // Free construction residual remains (illegal state representable as data).
        let _still_forgeable = forged_node(UNWHITELISTED_OPCODE);
    }

    #[test]
    fn oc_k_eml_opcode_gate_0_unwhitelisted_opcode_blocked() {
        let nodes = vec![forged_node(UNWHITELISTED_OPCODE)];
        let err = OpcodeRegistrationGate::admit_tree_nodes(&nodes).unwrap_err();
        assert!(matches!(
            err,
            OpcodeGateError::UnwhitelistedOpcode {
                opcode: UNWHITELISTED_OPCODE
            }
        ));
        assert!(EvalEmlOpcode::from_closed(UNWHITELISTED_OPCODE).is_err());
    }

    #[test]
    fn oc_k_eml_opcode_gate_0_unwhitelisted_combine_blocked() {
        let err = OpcodeRegistrationGate::admit_combine_kind(UNWHITELISTED_COMBINE).unwrap_err();
        assert!(matches!(
            err,
            OpcodeGateError::UnwhitelistedCombine {
                combine_kind: UNWHITELISTED_COMBINE
            }
        ));
        let err2 = OpcodeRegistrationGate::admit_combine(
            CombineRegistrationRequest::RawUnvalidated {
                combine_kind: UNWHITELISTED_COMBINE,
            },
        )
        .unwrap_err();
        assert!(matches!(err2, OpcodeGateError::UnwhitelistedCombine { .. }));
    }

    #[test]
    fn oc_k_eml_opcode_gate_0_generic_primitive_requires_tier2_parity() {
        // Without parity proof, CpuOracleParityProof cannot be minted with mismatch.
        let bad = CpuOracleParityProof::from_bit_exact_sample(1, 2);
        assert!(matches!(bad, Err(OpcodeGateError::ParityMismatch { .. })));
        // With parity, new opcode still cannot expand closed vocabulary without DA scope.
        let parity = CpuOracleParityProof::from_bit_exact_sample(0x3f80_0000, 0x3f80_0000).unwrap();
        let req = OpcodeRegistrationRequest::GenericPrimitive(GenericPrimitiveRegistration {
            name: "generic_sqrt_like".into(),
            proposed_opcode: 0x0100,
            parity,
        });
        let err = OpcodeRegistrationGate::admit_opcode(req).unwrap_err();
        assert!(matches!(
            err,
            OpcodeGateError::GenericPrimitiveRequiresVocabularyExpansion { .. }
        ));
        // Existing closed opcode with parity is admitted as closed path.
        let parity2 = CpuOracleParityProof::from_bit_exact_sample(0, 0).unwrap();
        let closed = OpcodeRegistrationGate::admit_opcode(
            OpcodeRegistrationRequest::GenericPrimitive(GenericPrimitiveRegistration {
                name: "already_closed_add".into(),
                proposed_opcode: eml_nodes::opcode::ADD,
                parity: parity2,
            }),
        )
        .unwrap();
        assert_eq!(closed.raw(), eml_nodes::opcode::ADD);
    }

    #[test]
    fn oc_k_eml_opcode_gate_0_semantic_opcode_never_admissible() {
        let req = OpcodeRegistrationRequest::Semantic(SemanticOpcodeRegistration {
            name: "faction_aggression_branch".into(),
            proposed_opcode: 0xF00D,
            scenario_or_policy_tag: "terran_pirate_urgency".into(),
        });
        let err = OpcodeRegistrationGate::admit_opcode(req).unwrap_err();
        assert!(matches!(
            err,
            OpcodeGateError::SemanticOpcodeNeverAdmissible { .. }
        ));
        let cerr = OpcodeRegistrationGate::admit_combine(
            CombineRegistrationRequest::SemanticNamed {
                name: "scenario_policy_combine".into(),
                combine_kind: 99,
            },
        )
        .unwrap_err();
        assert!(matches!(
            cerr,
            OpcodeGateError::SemanticCombineNeverAdmissible { .. }
        ));
    }

    #[test]
    fn oc_k_eml_opcode_gate_0_softstep_policy_conditional_compiles_as_gadget() {
        let policy = SoftStepPolicyConditional {
            input_col: 0,
            center: 0.5,
            steepness: 4.0,
            branch_a_col: 1,
            branch_b_col: 2,
            output_col: 3,
        };
        let nodes = policy.compile_nodes().expect("SoftStep policy compiles");
        // Closed vocabulary only.
        OpcodeRegistrationGate::admit_tree_nodes(&nodes).unwrap();
        // Register with EvalEML formula registry (existing closed path).
        let mut reg = EmlExpressionRegistry::new();
        let meta = EmlFormulaMeta {
            tree_id: EmlTreeId(7),
            execution_class: EmlExecutionClass::ExactDeterministic,
            allowed_consumers: EmlConsumerMask(EmlConsumerMask::ALL_PRODUCTION),
            max_abs_error: None,
            deterministic_gpu: true,
            requires_guard_for_hard_threshold: false,
            node_count: 0,
            max_stack_depth: 0,
            has_loops: false,
            has_recursion: false,
            display_name: "softstep_policy_conditional".into(),
        };
        reg.register_formula(EmlTreeId(7), meta, nodes.clone())
            .expect("closed-vocab SoftStep policy registers");
        // Oracle sanity: at center, soft≈0.5 → midpoint of A/B.
        let y = policy.oracle(0.5, 10.0, 0.0);
        assert!((y - 5.0).abs() < 1e-5);
        // Branchless: no semantic opcode in nodes.
        for n in &nodes {
            assert!(opcode_in_closed_vocabulary(n.opcode));
        }
    }
}
