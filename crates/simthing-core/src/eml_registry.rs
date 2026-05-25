//! EML expression tree registry with GPU EvalEML whitelist enforcement.
//!
//! See `docs/adr_accumulator_op_v2.md` §EML expression policy and PR A-3 in
//! `docs/accumulator_op_v2_production_plan.md`.
//!
//! # C-8 evolution
//!
//! The A-3 `EmlTreeMeta { node_count, has_transcendental, formula_class }`
//! schema is **refactored to `EmlFormulaMeta` in C-8a** with an explicit
//! `EmlExecutionClass` (`ExactDeterministic | SoftDeterministic |
//! FastApproximate | CpuOracleOnly`) and a typed `EmlConsumerMask`.
//! `WHITELISTED_FORMULA_CLASSES` becomes the C-8 `ExactDeterministic`
//! admission policy; future `SoftDeterministic` / `FastApproximate`
//! classes are admitted by explicit per-PR opt-in plus consumer
//! admissibility (`assert_consumer_admissible(tree_id, consumer)`).
//!
//! The A-3 types below are retained verbatim until C-8a lands; C-8a
//! provides `From<EmlTreeMeta> for EmlFormulaMeta` to migrate existing
//! call sites. See `docs/workshop/c8_eml_transfer_intensity_design.md`
//! §3 for the new schema.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Maximum expression-tree node count permitted for GPU `EvalEML` combine
/// under the **C-8 `ExactDeterministic` baseline**. Workshop validated ≤16
/// nodes with headroom within the 32-node GPU budget. Future execution
/// classes (`SoftDeterministic`, `FastApproximate`) may extend this budget
/// via explicit per-class limits in `EmlFormulaMeta` once C-8a lands.
pub const MAX_EML_TREE_NODES: u32 = 16;

/// Formula classes whitelisted for GPU `EvalEML` under the **C-8
/// `ExactDeterministic` admission policy**. Future SoftDeterministic /
/// FastApproximate classes are admitted via the typed `EmlConsumerKind`
/// matrix in C-8a, not by adding strings here.
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

/// Metadata describing an EML tree submitted for GPU evaluation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmlTreeMeta {
    pub node_count:         u32,
    pub has_transcendental: bool,
    pub formula_class:      String,
}

impl EmlTreeMeta {
    /// Validate against the ADR whitelist policy (no transcendentals, ≤16 nodes,
    /// whitelisted formula class).
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
        if !is_whitelisted_formula_class(&self.formula_class) {
            return Err(EmlRegistryError::UnknownFormulaClass(
                self.formula_class.clone(),
            ));
        }
        Ok(())
    }
}

fn is_whitelisted_formula_class(class: &str) -> bool {
    WHITELISTED_FORMULA_CLASSES.contains(&class)
}

/// Registry of EML trees approved for GPU `EvalEML` evaluation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EmlExpressionRegistry {
    trees: HashMap<EmlTreeId, EmlTreeMeta>,
}

impl EmlExpressionRegistry {
    pub fn new() -> Self {
        Self {
            trees: HashMap::new(),
        }
    }

    /// Register a tree after validating whitelist policy. Re-registration of
    /// the same `tree_id` is an error.
    pub fn register(&mut self, tree_id: EmlTreeId, meta: EmlTreeMeta) -> Result<(), EmlRegistryError> {
        if self.trees.contains_key(&tree_id) {
            return Err(EmlRegistryError::DuplicateTreeId(tree_id));
        }
        meta.validate_for_whitelist()?;
        self.trees.insert(tree_id, meta);
        Ok(())
    }

    /// Assert that `tree_id` was registered and passed whitelist validation.
    /// Called when an `AccumulatorOp` with `CombineFn::EvalEML` is registered.
    pub fn assert_whitelisted(&self, tree_id: EmlTreeId) -> Result<(), EmlRegistryError> {
        self.trees
            .get(&tree_id)
            .ok_or(EmlRegistryError::NotRegistered(tree_id))?;
        Ok(())
    }

    /// Returns registered metadata, if any.
    pub fn get(&self, tree_id: EmlTreeId) -> Option<&EmlTreeMeta> {
        self.trees.get(&tree_id)
    }

    pub fn len(&self) -> usize {
        self.trees.len()
    }

    pub fn is_empty(&self) -> bool {
        self.trees.is_empty()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_meta(class: &str) -> EmlTreeMeta {
        EmlTreeMeta {
            node_count:         16,
            has_transcendental: false,
            formula_class:      class.to_string(),
        }
    }

    #[test]
    fn register_valid_tree_succeeds() {
        let mut registry = EmlExpressionRegistry::new();
        for (idx, class) in WHITELISTED_FORMULA_CLASSES.iter().enumerate() {
            let id = EmlTreeId(idx as u32 + 1);
            registry
                .register(id, valid_meta(class))
                .unwrap_or_else(|e| panic!("register {class}: {e}"));
            assert!(registry.assert_whitelisted(id).is_ok());
        }
        assert_eq!(registry.len(), WHITELISTED_FORMULA_CLASSES.len());
    }

    #[test]
    fn reject_transcendental_tree() {
        let mut registry = EmlExpressionRegistry::new();
        let meta = EmlTreeMeta {
            node_count:         8,
            has_transcendental: true,
            formula_class:      "intensity_update".to_string(),
        };
        assert_eq!(
            registry.register(EmlTreeId(1), meta),
            Err(EmlRegistryError::TranscendentalNotAllowed)
        );
    }

    #[test]
    fn reject_tree_over_node_limit() {
        let mut registry = EmlExpressionRegistry::new();
        let meta = EmlTreeMeta {
            node_count:         MAX_EML_TREE_NODES + 1,
            has_transcendental: false,
            formula_class:      "intensity_update".to_string(),
        };
        assert_eq!(
            registry.register(EmlTreeId(2), meta),
            Err(EmlRegistryError::TooManyNodes(17))
        );
    }

    #[test]
    fn reject_unknown_formula_class() {
        let mut registry = EmlExpressionRegistry::new();
        let meta = EmlTreeMeta {
            node_count:         8,
            has_transcendental: false,
            formula_class:      "custom_decay_curve".to_string(),
        };
        assert_eq!(
            registry.register(EmlTreeId(3), meta),
            Err(EmlRegistryError::UnknownFormulaClass(
                "custom_decay_curve".to_string()
            ))
        );
    }

    #[test]
    fn assert_whitelisted_fails_for_unregistered_tree() {
        let registry = EmlExpressionRegistry::new();
        assert_eq!(
            registry.assert_whitelisted(EmlTreeId(99)),
            Err(EmlRegistryError::NotRegistered(EmlTreeId(99)))
        );
    }

    #[test]
    fn duplicate_registration_is_error() {
        let mut registry = EmlExpressionRegistry::new();
        let id = EmlTreeId(7);
        registry
            .register(id, valid_meta("emission_formula"))
            .unwrap();
        assert_eq!(
            registry.register(id, valid_meta("emission_formula")),
            Err(EmlRegistryError::DuplicateTreeId(id))
        );
    }

    #[test]
    fn reject_empty_tree() {
        let mut registry = EmlExpressionRegistry::new();
        let meta = EmlTreeMeta {
            node_count:         0,
            has_transcendental: false,
            formula_class:      "conversion_rate".to_string(),
        };
        assert_eq!(
            registry.register(EmlTreeId(4), meta),
            Err(EmlRegistryError::EmptyTree)
        );
    }
}
