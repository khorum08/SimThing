//! EML expression tree registry with execution-class and consumer admissibility.
//!
//! See `docs/workshop/c8_eml_transfer_intensity_design.md` and PR C-8a.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::eml_nodes::{self, EmlNode, EML_STACK_MAX};

pub use crate::eml_nodes::{opcode as eml_opcode, EmlNode as EmlNodeGpu};

/// Maximum expression-tree node count for GPU `ExactDeterministic` baseline.
pub const MAX_EML_TREE_NODES: u32 = 32;

/// Formula classes admitted under the C-8 `ExactDeterministic` baseline policy.
pub const WHITELISTED_FORMULA_CLASSES: &[&str] = &[
    "intensity_update",
    "emission_formula",
    "conversion_rate",
];

/// Stable identifier for a registered EML expression tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EmlTreeId(pub u32);

impl EmlTreeId {
    pub fn raw(self) -> u32 {
        self.0
    }
}

/// Execution policy for an EML formula (C-8 execution-class taxonomy).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmlExecutionClass {
    ExactDeterministic,
    SoftDeterministic,
    FastApproximate,
    CpuOracleOnly,
}

/// Consumer site referencing an EML formula.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmlConsumerKind {
    TransferConservation,
    HardThreshold,
    SoftThreshold,
    Intensity,
    Emission,
    DebugOracle,
}

/// Bitmask of consumer kinds a formula may serve.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct EmlConsumerMask(pub u32);

impl EmlConsumerMask {
    pub const TRANSFER_CONSERVATION: u32 = 1 << 0;
    pub const HARD_THRESHOLD: u32 = 1 << 1;
    pub const SOFT_THRESHOLD: u32 = 1 << 2;
    pub const INTENSITY: u32 = 1 << 3;
    pub const EMISSION: u32 = 1 << 4;
    pub const DEBUG_ORACLE: u32 = 1 << 5;

    pub const ALL_PRODUCTION: u32 = Self::TRANSFER_CONSERVATION
        | Self::HARD_THRESHOLD
        | Self::SOFT_THRESHOLD
        | Self::INTENSITY
        | Self::EMISSION;

    pub fn from_kind(kind: EmlConsumerKind) -> Self {
        Self(kind_bit(kind))
    }

    pub fn contains_kind(self, kind: EmlConsumerKind) -> bool {
        self.0 & kind_bit(kind) != 0
    }

    pub fn insert_kind(&mut self, kind: EmlConsumerKind) {
        self.0 |= kind_bit(kind);
    }
}

fn kind_bit(kind: EmlConsumerKind) -> u32 {
    match kind {
        EmlConsumerKind::TransferConservation => EmlConsumerMask::TRANSFER_CONSERVATION,
        EmlConsumerKind::HardThreshold => EmlConsumerMask::HARD_THRESHOLD,
        EmlConsumerKind::SoftThreshold => EmlConsumerMask::SOFT_THRESHOLD,
        EmlConsumerKind::Intensity => EmlConsumerMask::INTENSITY,
        EmlConsumerKind::Emission => EmlConsumerMask::EMISSION,
        EmlConsumerKind::DebugOracle => EmlConsumerMask::DEBUG_ORACLE,
    }
}

/// C-8 formula metadata (replaces A-3 `EmlTreeMeta` for new registrations).
#[derive(Clone, Debug, PartialEq)]
pub struct EmlFormulaMeta {
    pub tree_id: EmlTreeId,
    pub execution_class: EmlExecutionClass,
    pub allowed_consumers: EmlConsumerMask,
    pub max_abs_error: Option<f32>,
    pub deterministic_gpu: bool,
    pub requires_guard_for_hard_threshold: bool,
    pub node_count: u32,
    pub max_stack_depth: u32,
    pub has_loops: bool,
    pub has_recursion: bool,
    pub display_name: String,
}

/// Legacy A-3 metadata — deprecated; migrate via [`From<EmlTreeMeta> for EmlFormulaMeta`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[deprecated(note = "use EmlFormulaMeta (C-8a)")]
pub struct EmlTreeMeta {
    pub node_count: u32,
    pub has_transcendental: bool,
    pub formula_class: String,
}

impl EmlTreeMeta {
    pub fn validate_for_whitelist(&self) -> Result<(), EmlRegistryError> {
        if self.has_transcendental {
            return Err(EmlRegistryError::TranscendentalNotAllowed);
        }
        if self.node_count == 0 {
            return Err(EmlRegistryError::EmptyTree);
        }
        if self.node_count > MAX_EML_TREE_NODES {
            return Err(EmlRegistryError::TooManyNodes(self.node_count));
        }
        if !WHITELISTED_FORMULA_CLASSES.contains(&self.formula_class.as_str()) {
            return Err(EmlRegistryError::UnknownFormulaClass(
                self.formula_class.clone(),
            ));
        }
        Ok(())
    }
}

impl From<EmlTreeMeta> for EmlFormulaMeta {
    fn from(legacy: EmlTreeMeta) -> Self {
        let tree_id = EmlTreeId(0);
        if legacy.has_transcendental {
            EmlFormulaMeta {
                tree_id,
                execution_class: EmlExecutionClass::FastApproximate,
                allowed_consumers: EmlConsumerMask(EmlConsumerMask::DEBUG_ORACLE),
                max_abs_error: None,
                deterministic_gpu: false,
                requires_guard_for_hard_threshold: false,
                node_count: legacy.node_count,
                max_stack_depth: 0,
                has_loops: false,
                has_recursion: false,
                display_name: legacy.formula_class,
            }
        } else {
            EmlFormulaMeta {
                tree_id,
                execution_class: EmlExecutionClass::ExactDeterministic,
                allowed_consumers: default_mask_for_class(EmlExecutionClass::ExactDeterministic),
                max_abs_error: None,
                deterministic_gpu: true,
                requires_guard_for_hard_threshold: false,
                node_count: legacy.node_count,
                max_stack_depth: 0,
                has_loops: false,
                has_recursion: false,
                display_name: legacy.formula_class,
            }
        }
    }
}

fn default_mask_for_class(class: EmlExecutionClass) -> EmlConsumerMask {
    match class {
        EmlExecutionClass::ExactDeterministic => {
            EmlConsumerMask(EmlConsumerMask::ALL_PRODUCTION | EmlConsumerMask::DEBUG_ORACLE)
        }
        EmlExecutionClass::SoftDeterministic => {
            EmlConsumerMask(EmlConsumerMask::SOFT_THRESHOLD | EmlConsumerMask::DEBUG_ORACLE)
        }
        EmlExecutionClass::FastApproximate => EmlConsumerMask(EmlConsumerMask::DEBUG_ORACLE),
        EmlExecutionClass::CpuOracleOnly => EmlConsumerMask(EmlConsumerMask::DEBUG_ORACLE),
    }
}

#[derive(Clone, Debug)]
struct RegisteredFormula {
    meta: EmlFormulaMeta,
    nodes: Vec<EmlNode>,
    range_index: Option<u32>,
    upload_generation: Option<u64>,
}

/// Registry of EML formulas and GPU upload state.
#[derive(Clone, Debug, Default)]
pub struct EmlExpressionRegistry {
    formulas: HashMap<EmlTreeId, RegisteredFormula>,
    generation: u64,
}

impl EmlExpressionRegistry {
    pub fn new() -> Self {
        Self {
            formulas: HashMap::new(),
            generation: 0,
        }
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// A-3 compatibility shim — maps legacy metadata to C-8 and registers empty nodes.
    pub fn register(&mut self, tree_id: EmlTreeId, meta: EmlTreeMeta) -> Result<(), EmlRegistryError> {
        meta.validate_for_whitelist()?;
        let mut formula = EmlFormulaMeta::from(meta);
        formula.tree_id = tree_id;
        formula.node_count = formula.node_count.max(1);
        let nodes = vec![EmlNode {
            opcode: eml_nodes::opcode::LITERAL_F32,
            flags: 0,
            a: 0f32.to_bits(),
            b: 0,
            c: 0,
            d: 0,
        }];
        self.register_formula(tree_id, formula, nodes)
    }

    /// Register a formula with nodes. Validates opcodes, stack depth, and class policy.
    pub fn register_formula(
        &mut self,
        tree_id: EmlTreeId,
        mut meta: EmlFormulaMeta,
        nodes: Vec<EmlNode>,
    ) -> Result<(), EmlRegistryError> {
        if self.formulas.contains_key(&tree_id) {
            return Err(EmlRegistryError::DuplicateTreeId(tree_id));
        }
        meta.tree_id = tree_id;
        if nodes.is_empty() {
            return Err(EmlRegistryError::EmptyTree);
        }
        if nodes.len() as u32 > MAX_EML_TREE_NODES {
            return Err(EmlRegistryError::TooManyNodes(nodes.len() as u32));
        }
        meta.node_count = nodes.len() as u32;
        meta.max_stack_depth = validate_stack_depth(&nodes)?;
        validate_nodes_for_class(meta.execution_class, &nodes)?;
        if meta.execution_class == EmlExecutionClass::CpuOracleOnly {
            return Err(EmlRegistryError::CannotUploadCpuOracleOnly { tree_id });
        }
        if meta.allowed_consumers.0 == 0 {
            meta.allowed_consumers = default_mask_for_class(meta.execution_class);
        }
        meta.deterministic_gpu = matches!(
            meta.execution_class,
            EmlExecutionClass::ExactDeterministic | EmlExecutionClass::SoftDeterministic
        );
        meta.requires_guard_for_hard_threshold =
            meta.execution_class == EmlExecutionClass::SoftDeterministic;
        self.formulas.insert(
            tree_id,
            RegisteredFormula {
                meta,
                nodes,
                range_index: None,
                upload_generation: None,
            },
        );
        self.generation = self.generation.wrapping_add(1);
        Ok(())
    }

    /// Register a CpuOracleOnly formula for debug/test oracle paths (never GPU-uploaded).
    pub fn register_cpu_oracle_formula(
        &mut self,
        tree_id: EmlTreeId,
        mut meta: EmlFormulaMeta,
        nodes: Vec<EmlNode>,
    ) -> Result<(), EmlRegistryError> {
        if self.formulas.contains_key(&tree_id) {
            return Err(EmlRegistryError::DuplicateTreeId(tree_id));
        }
        if meta.execution_class != EmlExecutionClass::CpuOracleOnly {
            return Err(EmlRegistryError::NotCpuOracleOnly { tree_id });
        }
        meta.tree_id = tree_id;
        if nodes.is_empty() {
            return Err(EmlRegistryError::EmptyTree);
        }
        if nodes.len() as u32 > MAX_EML_TREE_NODES {
            return Err(EmlRegistryError::TooManyNodes(nodes.len() as u32));
        }
        meta.node_count = nodes.len() as u32;
        meta.max_stack_depth = validate_stack_depth(&nodes)?;
        validate_nodes_for_class(meta.execution_class, &nodes)?;
        meta.allowed_consumers = EmlConsumerMask(EmlConsumerMask::DEBUG_ORACLE);
        meta.deterministic_gpu = false;
        meta.requires_guard_for_hard_threshold = false;
        self.formulas.insert(
            tree_id,
            RegisteredFormula {
                meta,
                nodes,
                range_index: None,
                upload_generation: None,
            },
        );
        self.generation = self.generation.wrapping_add(1);
        Ok(())
    }

    /// Replace an existing formula or register a new one (boundary sync for intensity).
    pub fn replace_formula(
        &mut self,
        tree_id: EmlTreeId,
        mut meta: EmlFormulaMeta,
        nodes: Vec<EmlNode>,
    ) -> Result<(), EmlRegistryError> {
        meta.tree_id = tree_id;
        if nodes.is_empty() {
            return Err(EmlRegistryError::EmptyTree);
        }
        if nodes.len() as u32 > MAX_EML_TREE_NODES {
            return Err(EmlRegistryError::TooManyNodes(nodes.len() as u32));
        }
        meta.node_count = nodes.len() as u32;
        meta.max_stack_depth = validate_stack_depth(&nodes)?;
        validate_nodes_for_class(meta.execution_class, &nodes)?;
        if meta.execution_class == EmlExecutionClass::CpuOracleOnly {
            return Err(EmlRegistryError::CannotUploadCpuOracleOnly { tree_id });
        }
        if meta.allowed_consumers.0 == 0 {
            meta.allowed_consumers = default_mask_for_class(meta.execution_class);
        }
        meta.deterministic_gpu = matches!(
            meta.execution_class,
            EmlExecutionClass::ExactDeterministic | EmlExecutionClass::SoftDeterministic
        );
        meta.requires_guard_for_hard_threshold =
            meta.execution_class == EmlExecutionClass::SoftDeterministic;
        self.formulas.insert(
            tree_id,
            RegisteredFormula {
                meta,
                nodes,
                range_index: None,
                upload_generation: None,
            },
        );
        self.generation = self.generation.wrapping_add(1);
        Ok(())
    }

    /// Remove a formula from the registry (intensity resync).
    pub fn remove_tree(&mut self, tree_id: EmlTreeId) -> bool {
        if self.formulas.remove(&tree_id).is_some() {
            self.generation = self.generation.wrapping_add(1);
            true
        } else {
            false
        }
    }

    pub fn assert_whitelisted(&self, tree_id: EmlTreeId) -> Result<(), EmlRegistryError> {
        self.formulas
            .get(&tree_id)
            .ok_or(EmlRegistryError::NotRegistered(tree_id))?;
        Ok(())
    }

    pub fn assert_consumer_admissible(
        &self,
        tree_id: EmlTreeId,
        consumer: EmlConsumerKind,
    ) -> Result<(), EmlRegistryError> {
        let entry = self
            .formulas
            .get(&tree_id)
            .ok_or(EmlRegistryError::NotRegistered(tree_id))?;
        let class = entry.meta.execution_class;
        if !entry.meta.allowed_consumers.contains_kind(consumer) {
            return Err(EmlRegistryError::ClassNotAdmissibleForConsumer {
                class,
                consumer,
            });
        }
        match consumer {
            EmlConsumerKind::DebugOracle => return Ok(()),
            EmlConsumerKind::TransferConservation => {
                if class != EmlExecutionClass::ExactDeterministic {
                    return Err(EmlRegistryError::ClassNotAdmissibleForConsumer {
                        class,
                        consumer,
                    });
                }
            }
            EmlConsumerKind::HardThreshold => {
                if class != EmlExecutionClass::ExactDeterministic {
                    return Err(EmlRegistryError::ClassNotAdmissibleForConsumer {
                        class,
                        consumer,
                    });
                }
            }
            EmlConsumerKind::SoftThreshold
            | EmlConsumerKind::Intensity
            | EmlConsumerKind::Emission => {
                if class != EmlExecutionClass::ExactDeterministic {
                    return Err(EmlRegistryError::ClassNotAdmissibleForConsumer {
                        class,
                        consumer,
                    });
                }
            }
        }
        Ok(())
    }

    /// Future hook: soft hard-threshold admission requires an explicit guard (C-8a rejects all non-exact).
    pub fn assert_hard_threshold_admissible(
        &self,
        tree_id: EmlTreeId,
        guard_present: bool,
    ) -> Result<(), EmlRegistryError> {
        let entry = self
            .formulas
            .get(&tree_id)
            .ok_or(EmlRegistryError::NotRegistered(tree_id))?;
        let class = entry.meta.execution_class;
        match class {
            EmlExecutionClass::ExactDeterministic => Ok(()),
            EmlExecutionClass::SoftDeterministic => {
                if guard_present {
                    Err(EmlRegistryError::GuardedSoftHardThresholdDeferred { tree_id })
                } else {
                    Err(EmlRegistryError::GuardRequiredForSoftHardThreshold { tree_id })
                }
            }
            EmlExecutionClass::FastApproximate | EmlExecutionClass::CpuOracleOnly => {
                Err(EmlRegistryError::ClassNotAdmissibleForConsumer {
                    class,
                    consumer: EmlConsumerKind::HardThreshold,
                })
            }
        }
    }

    pub fn tree_range_index(&self, tree_id: EmlTreeId) -> Option<u32> {
        self.formulas
            .get(&tree_id)
            .and_then(|f| f.range_index)
    }

    pub fn mark_tree_uploaded(
        &mut self,
        tree_id: EmlTreeId,
        range_index: u32,
        generation: u64,
    ) -> Result<(), EmlRegistryError> {
        let entry = self
            .formulas
            .get_mut(&tree_id)
            .ok_or(EmlRegistryError::NotRegistered(tree_id))?;
        entry.range_index = Some(range_index);
        entry.upload_generation = Some(generation);
        Ok(())
    }

    pub fn bump_generation(&mut self) -> u64 {
        self.generation = self.generation.wrapping_add(1);
        self.generation
    }

    pub fn formulas_for_gpu_upload(
        &self,
    ) -> impl Iterator<Item = (EmlTreeId, &EmlFormulaMeta, &[EmlNode])> {
        self.formulas.iter().filter_map(|(id, f)| {
            if f.meta.execution_class == EmlExecutionClass::CpuOracleOnly {
                return None;
            }
            Some((*id, &f.meta, f.nodes.as_slice()))
        })
    }

    pub fn get(&self, tree_id: EmlTreeId) -> Option<&EmlFormulaMeta> {
        self.formulas.get(&tree_id).map(|f| &f.meta)
    }

    pub fn get_nodes(&self, tree_id: EmlTreeId) -> Option<&[EmlNode]> {
        self.formulas.get(&tree_id).map(|f| f.nodes.as_slice())
    }

    #[deprecated(note = "use get() returning EmlFormulaMeta")]
    pub fn get_legacy_meta(&self, tree_id: EmlTreeId) -> Option<EmlTreeMeta> {
        self.formulas.get(&tree_id).map(|f| EmlTreeMeta {
            node_count: f.meta.node_count,
            has_transcendental: f.meta.execution_class == EmlExecutionClass::FastApproximate,
            formula_class: f.meta.display_name.clone(),
        })
    }

    pub fn len(&self) -> usize {
        self.formulas.len()
    }

    pub fn is_empty(&self) -> bool {
        self.formulas.is_empty()
    }
}

fn validate_stack_depth(nodes: &[EmlNode]) -> Result<u32, EmlRegistryError> {
    let mut sp: u32 = 0;
    let mut max_sp: u32 = 0;
    for node in nodes {
        match node.opcode {
            eml_nodes::opcode::LITERAL_F32
            | eml_nodes::opcode::SLOT_VALUE
            | eml_nodes::opcode::PARAM => {
                sp += 1;
            }
            eml_nodes::opcode::NEG | eml_nodes::opcode::CLAMP_BOUNDED | eml_nodes::opcode::CLAMP_FLOORED | eml_nodes::opcode::ABS => {}
            eml_nodes::opcode::ADD
            | eml_nodes::opcode::SUB
            | eml_nodes::opcode::MUL
            | eml_nodes::opcode::DIV
            | eml_nodes::opcode::MIN
            | eml_nodes::opcode::MAX
            | eml_nodes::opcode::CMP_LT
            | eml_nodes::opcode::CMP_LE
            | eml_nodes::opcode::CMP_GT
            | eml_nodes::opcode::CMP_GE
            | eml_nodes::opcode::CMP_EQ => {
                if sp < 2 {
                    return Err(EmlRegistryError::StackUnderflow);
                }
                sp -= 1;
            }
            eml_nodes::opcode::SELECT => {
                if sp < 3 {
                    return Err(EmlRegistryError::StackUnderflow);
                }
                sp -= 2;
            }
            eml_nodes::opcode::RETURN_TOP => {}
            other => {
                return Err(EmlRegistryError::OpcodeNotAllowedInClass {
                    opcode: other,
                    class: EmlExecutionClass::ExactDeterministic,
                });
            }
        }
        max_sp = max_sp.max(sp);
        if max_sp > EML_STACK_MAX {
            return Err(EmlRegistryError::StackDepthExceeded {
                depth: max_sp,
                max: EML_STACK_MAX,
            });
        }
    }
    if sp == 0 {
        return Err(EmlRegistryError::StackUnderflow);
    }
    Ok(max_sp)
}

fn validate_nodes_for_class(
    class: EmlExecutionClass,
    nodes: &[EmlNode],
) -> Result<(), EmlRegistryError> {
    for node in nodes {
        if node.opcode == eml_nodes::opcode::PARAM && node.a > 3 {
            return Err(EmlRegistryError::ParamIndexOutOfRange { index: node.a });
        }
    }
    if class == EmlExecutionClass::CpuOracleOnly {
        return Ok(());
    }
    for node in nodes {
        if !opcode_allowed_in_exact(node.opcode) {
            return Err(EmlRegistryError::OpcodeNotAllowedInClass {
                opcode: node.opcode,
                class,
            });
        }
        if node.opcode == eml_nodes::opcode::DIV {
            validate_div_node(node)?;
        }
    }
    Ok(())
}

fn opcode_allowed_in_exact(op: u32) -> bool {
    matches!(
        op,
        eml_nodes::opcode::LITERAL_F32
            | eml_nodes::opcode::SLOT_VALUE
            | eml_nodes::opcode::PARAM
            | eml_nodes::opcode::ADD
            | eml_nodes::opcode::SUB
            | eml_nodes::opcode::MUL
            | eml_nodes::opcode::NEG
            | eml_nodes::opcode::DIV
            | eml_nodes::opcode::MIN
            | eml_nodes::opcode::MAX
            | eml_nodes::opcode::CLAMP_BOUNDED
            | eml_nodes::opcode::CLAMP_FLOORED
            | eml_nodes::opcode::ABS
            | eml_nodes::opcode::CMP_LT
            | eml_nodes::opcode::CMP_LE
            | eml_nodes::opcode::CMP_GT
            | eml_nodes::opcode::CMP_GE
            | eml_nodes::opcode::CMP_EQ
            | eml_nodes::opcode::SELECT
            | eml_nodes::opcode::RETURN_TOP
    )
}

fn validate_div_node(node: &EmlNode) -> Result<(), EmlRegistryError> {
    if node.flags & 1 != 0 {
        return Ok(());
    }
    Err(EmlRegistryError::UnwrappedDivision)
}

fn is_whitelisted_formula_class(class: &str) -> bool {
    WHITELISTED_FORMULA_CLASSES.contains(&class)
}

/// Classify legacy whitelist metadata into execution class.
pub fn classify_legacy_tree_meta(meta: &EmlTreeMeta) -> EmlExecutionClass {
    if meta.has_transcendental {
        EmlExecutionClass::FastApproximate
    } else if meta.node_count == 0 || meta.node_count > MAX_EML_TREE_NODES {
        EmlExecutionClass::CpuOracleOnly
    } else if is_whitelisted_formula_class(&meta.formula_class) {
        EmlExecutionClass::ExactDeterministic
    } else {
        EmlExecutionClass::CpuOracleOnly
    }
}

/// Errors from EML registry validation and lookup.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum EmlRegistryError {
    #[error("EML tree {0:?} is already registered")]
    DuplicateTreeId(EmlTreeId),
    #[error("EML tree {0:?} is not registered")]
    NotRegistered(EmlTreeId),
    #[error("GPU EvalEML does not permit transcendental functions")]
    TranscendentalNotAllowed,
    #[error("EML tree must have at least one node")]
    EmptyTree,
    #[error("EML tree has {0} nodes; maximum is {MAX_EML_TREE_NODES}")]
    TooManyNodes(u32),
    #[error("formula class {0:?} is not whitelisted for GPU EvalEML")]
    UnknownFormulaClass(String),
    #[error("EML opcode {opcode:#x} not allowed in execution class {class:?}")]
    OpcodeNotAllowedInClass {
        opcode: u32,
        class: EmlExecutionClass,
    },
    #[error("EML formula execution class {class:?} not admissible for consumer {consumer:?}")]
    ClassNotAdmissibleForConsumer {
        class: EmlExecutionClass,
        consumer: EmlConsumerKind,
    },
    #[error("EML formula {tree_id:?} has not been uploaded to a session yet")]
    TreeNotUploaded { tree_id: EmlTreeId },
    #[error("EML formula registration would exceed node budget (requested {requested}, max {max})")]
    NodeBudgetExceeded { requested: u32, max: u32 },
    #[error("EML stack depth {depth} exceeds maximum {max}")]
    StackDepthExceeded { depth: u32, max: u32 },
    #[error("EML program stack underflow during validation")]
    StackUnderflow,
    #[error("EML DIV node must set flags bit 0 (safe division) or use guarded divisor")]
    UnwrappedDivision,
    #[error("EML formula {tree_id:?} is CpuOracleOnly and cannot be uploaded to GPU")]
    CannotUploadCpuOracleOnly { tree_id: EmlTreeId },
    #[error("EML formula {tree_id:?} must use register_cpu_oracle_formula")]
    NotCpuOracleOnly { tree_id: EmlTreeId },
    #[error("EML PARAM index {index} is out of range (must be 0..=3)")]
    ParamIndexOutOfRange { index: u32 },
    #[error("EML formula {tree_id:?} requires SoftAggregateGuard for hard-threshold admission")]
    GuardRequiredForSoftHardThreshold { tree_id: EmlTreeId },
    #[error("guarded soft hard-threshold admission for {tree_id:?} is deferred past C-8a")]
    GuardedSoftHardThresholdDeferred { tree_id: EmlTreeId },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn literal(v: f32) -> EmlNode {
        EmlNode {
            opcode: eml_nodes::opcode::LITERAL_F32,
            flags: 0,
            a: v.to_bits(),
            b: 0,
            c: 0,
            d: 0,
        }
    }

    fn exact_meta(id: u32, name: &str) -> EmlFormulaMeta {
        EmlFormulaMeta {
            tree_id: EmlTreeId(id),
            execution_class: EmlExecutionClass::ExactDeterministic,
            allowed_consumers: default_mask_for_class(EmlExecutionClass::ExactDeterministic),
            max_abs_error: None,
            deterministic_gpu: true,
            requires_guard_for_hard_threshold: false,
            node_count: 0,
            max_stack_depth: 0,
            has_loops: false,
            has_recursion: false,
            display_name: name.to_string(),
        }
    }

    #[test]
    fn register_valid_tree_succeeds() {
        let mut registry = EmlExpressionRegistry::new();
        for (idx, class) in WHITELISTED_FORMULA_CLASSES.iter().enumerate() {
            let id = EmlTreeId(idx as u32 + 1);
            registry
                .register(id, EmlTreeMeta {
                    node_count: 1,
                    has_transcendental: false,
                    formula_class: class.to_string(),
                })
                .unwrap_or_else(|e| panic!("register {class}: {e}"));
            assert!(registry.assert_whitelisted(id).is_ok());
        }
    }

    #[test]
    fn reject_transcendental_tree() {
        let mut registry = EmlExpressionRegistry::new();
        assert_eq!(
            registry.register(
                EmlTreeId(1),
                EmlTreeMeta {
                    node_count: 8,
                    has_transcendental: true,
                    formula_class: "intensity_update".to_string(),
                }
            ),
            Err(EmlRegistryError::TranscendentalNotAllowed)
        );
    }

    #[test]
    fn c8a_soft_can_register_but_not_transfer() {
        let mut registry = EmlExpressionRegistry::new();
        let id = EmlTreeId(1);
        let mut meta = exact_meta(1, "soft_demo");
        meta.execution_class = EmlExecutionClass::SoftDeterministic;
        meta.allowed_consumers =
            EmlConsumerMask(EmlConsumerMask::SOFT_THRESHOLD | EmlConsumerMask::DEBUG_ORACLE);
        registry
            .register_formula(id, meta, vec![literal(1.0), literal(2.0), EmlNode {
                opcode: eml_nodes::opcode::ADD,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            }])
            .unwrap();
        assert!(registry
            .assert_consumer_admissible(id, EmlConsumerKind::TransferConservation)
            .is_err());
    }

    #[test]
    fn c8a_cpu_oracle_only_rejected_from_gpu_registration() {
        let mut registry = EmlExpressionRegistry::new();
        let id = EmlTreeId(9);
        let mut meta = exact_meta(9, "cpu_only");
        meta.execution_class = EmlExecutionClass::CpuOracleOnly;
        assert_eq!(
            registry.register_formula(id, meta, vec![literal(1.0)]),
            Err(EmlRegistryError::CannotUploadCpuOracleOnly { tree_id: id })
        );
    }

    #[test]
    fn c8a_cpu_oracle_only_can_register_for_debug_oracle() {
        let mut registry = EmlExpressionRegistry::new();
        let id = EmlTreeId(9);
        let mut meta = exact_meta(9, "cpu_only");
        meta.execution_class = EmlExecutionClass::CpuOracleOnly;
        registry
            .register_cpu_oracle_formula(id, meta, vec![literal(1.0)])
            .unwrap();
        assert!(registry
            .assert_consumer_admissible(id, EmlConsumerKind::DebugOracle)
            .is_ok());
        assert_eq!(registry.tree_range_index(id), None);
    }

    #[test]
    fn c8a_cpu_oracle_only_not_returned_for_gpu_upload() {
        let mut registry = EmlExpressionRegistry::new();
        let id = EmlTreeId(9);
        let mut meta = exact_meta(9, "cpu_only");
        meta.execution_class = EmlExecutionClass::CpuOracleOnly;
        registry
            .register_cpu_oracle_formula(id, meta, vec![literal(1.0)])
            .unwrap();
        assert_eq!(registry.formulas_for_gpu_upload().count(), 0);
    }

    #[test]
    fn c8a_cpu_oracle_only_rejected_from_production_consumers() {
        let mut registry = EmlExpressionRegistry::new();
        let id = EmlTreeId(9);
        let mut meta = exact_meta(9, "cpu_only");
        meta.execution_class = EmlExecutionClass::CpuOracleOnly;
        registry
            .register_cpu_oracle_formula(id, meta, vec![literal(1.0)])
            .unwrap();
        assert!(registry
            .assert_consumer_admissible(id, EmlConsumerKind::Intensity)
            .is_err());
    }

    #[test]
    fn c8a_soft_deterministic_hard_threshold_rejected_without_guard() {
        let mut registry = EmlExpressionRegistry::new();
        let id = EmlTreeId(1);
        let mut meta = exact_meta(1, "soft");
        meta.execution_class = EmlExecutionClass::SoftDeterministic;
        meta.allowed_consumers =
            EmlConsumerMask(EmlConsumerMask::HARD_THRESHOLD | EmlConsumerMask::DEBUG_ORACLE);
        registry
            .register_formula(id, meta, vec![literal(1.0), literal(2.0), EmlNode {
                opcode: eml_nodes::opcode::ADD,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            }])
            .unwrap();
        assert!(registry
            .assert_consumer_admissible(id, EmlConsumerKind::HardThreshold)
            .is_err());
        assert!(matches!(
            registry.assert_hard_threshold_admissible(id, false),
            Err(EmlRegistryError::GuardRequiredForSoftHardThreshold { .. })
        ));
    }

    #[test]
    fn c8a_soft_hard_threshold_requires_explicit_guard() {
        let mut registry = EmlExpressionRegistry::new();
        let id = EmlTreeId(1);
        let mut meta = exact_meta(1, "soft");
        meta.execution_class = EmlExecutionClass::SoftDeterministic;
        registry
            .register_formula(id, meta, vec![literal(1.0)])
            .unwrap();
        assert!(matches!(
            registry.assert_hard_threshold_admissible(id, true),
            Err(EmlRegistryError::GuardedSoftHardThresholdDeferred { .. })
        ));
    }

    #[test]
    fn c8a_param_index_must_be_0_to_3() {
        let mut registry = EmlExpressionRegistry::new();
        let nodes = vec![EmlNode {
            opcode: eml_nodes::opcode::PARAM,
            flags: 0,
            a: 4,
            b: 0,
            c: 0,
            d: 0,
        }];
        assert_eq!(
            registry.register_formula(EmlTreeId(1), exact_meta(1, "param"), nodes),
            Err(EmlRegistryError::ParamIndexOutOfRange { index: 4 })
        );
    }

    #[test]
    fn register_formula_bumps_registry_generation() {
        let mut registry = EmlExpressionRegistry::new();
        assert_eq!(registry.generation(), 0);
        registry
            .register_formula(EmlTreeId(1), exact_meta(1, "a"), vec![literal(1.0)])
            .unwrap();
        assert_eq!(registry.generation(), 1);
    }

    #[test]
    fn c8a_stack_depth_validator_rejects_overflow() {
        let mut nodes = Vec::new();
        for i in 0..=EML_STACK_MAX {
            nodes.push(literal(i as f32));
        }
        assert!(matches!(
            validate_stack_depth(&nodes),
            Err(EmlRegistryError::StackDepthExceeded { .. })
        ));
    }

    #[test]
    fn c8a_division_by_zero_validator_rejects_unwrapped_division() {
        let nodes = vec![
            literal(1.0),
            literal(0.0),
            EmlNode {
                opcode: eml_nodes::opcode::DIV,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
        ];
        let mut registry = EmlExpressionRegistry::new();
        assert_eq!(
            registry.register_formula(EmlTreeId(1), exact_meta(1, "div"), nodes),
            Err(EmlRegistryError::UnwrappedDivision)
        );
    }
}
