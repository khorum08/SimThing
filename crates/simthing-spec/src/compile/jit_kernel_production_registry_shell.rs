//! Phase M-JIT-PROD-0 — Default-off production-shaped kernel registry shell (spec layer).
//!
//! Holds REG-1-admitted `ProductionCandidatePreview` entries for explicit opt-in registration.
//! No runtime cache, no scheduler, no GPU dispatch, no default wiring.

use std::collections::BTreeMap;

use crate::compile::jit_kernel_registry_preview::{
    validate_production_candidate_preview_entry, KernelRegistryEntryPreview, KernelRegistryLane,
};
use crate::error::SpecError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductionKernelRegistryShellConfig {
    pub default_off: bool,
    pub allow_production_wiring: bool,
}

impl Default for ProductionKernelRegistryShellConfig {
    fn default() -> Self {
        Self {
            default_off: true,
            allow_production_wiring: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisteredProductionCandidate {
    pub stable_key: String,
    pub canonical_text: String,
    pub request_ids: Vec<String>,
    pub default_off: bool,
    pub production_wiring: bool,
}

#[derive(Debug, Default)]
pub struct ProductionKernelRegistryShell {
    config: ProductionKernelRegistryShellConfig,
    entries: BTreeMap<String, RegisteredProductionCandidate>,
}

fn shell_err(context: &str, reason: impl Into<String>) -> SpecError {
    SpecError::JitKernelDescriptorAdmission {
        kernel: context.to_string(),
        reason: reason.into(),
    }
}

impl ProductionKernelRegistryShell {
    pub fn new(config: ProductionKernelRegistryShellConfig) -> Self {
        Self {
            config,
            entries: BTreeMap::new(),
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(ProductionKernelRegistryShellConfig::default())
    }

    pub fn config(&self) -> &ProductionKernelRegistryShellConfig {
        &self.config
    }

    pub fn registered_count(&self) -> usize {
        self.entries.len()
    }

    pub fn is_registered(&self, stable_key: &str) -> bool {
        self.entries.contains_key(stable_key)
    }

    pub fn get_registered(&self, stable_key: &str) -> Option<&RegisteredProductionCandidate> {
        self.entries.get(stable_key)
    }

    /// Register a ProductionCandidatePreview entry. Duplicate stable keys are idempotent when
    /// canonical text and request IDs match byte-for-byte; otherwise reject.
    pub fn register_production_candidate(
        &mut self,
        candidate: &KernelRegistryEntryPreview,
    ) -> Result<RegisteredProductionCandidate, SpecError> {
        if candidate.lane != KernelRegistryLane::ProductionCandidatePreview {
            return Err(shell_err(
                &candidate.stable_key,
                "production shell accepts only ProductionCandidatePreview entries",
            ));
        }

        validate_production_candidate_preview_entry(candidate)?;

        if self.config.default_off && !candidate.default_off {
            return Err(shell_err(
                &candidate.stable_key,
                "production shell requires default_off=true",
            ));
        }

        if !self.config.allow_production_wiring && candidate.production_wiring {
            return Err(shell_err(
                &candidate.stable_key,
                "production shell rejects production_wiring=true",
            ));
        }

        let registered = RegisteredProductionCandidate {
            stable_key: candidate.stable_key.clone(),
            canonical_text: candidate.canonical_text.clone(),
            request_ids: candidate.request_ids.clone(),
            default_off: candidate.default_off,
            production_wiring: candidate.production_wiring,
        };

        if let Some(existing) = self.entries.get(&registered.stable_key) {
            if existing.canonical_text != registered.canonical_text
                || existing.request_ids != registered.request_ids
            {
                return Err(shell_err(
                    &registered.stable_key,
                    "duplicate stable key with non-identical canonical text or request_ids",
                ));
            }
            return Ok(existing.clone());
        }

        self.entries
            .insert(registered.stable_key.clone(), registered.clone());
        Ok(registered)
    }

    /// Execution gate: returns registered candidate or rejects if not registered.
    pub fn require_registered_for_execution(
        &self,
        stable_key: &str,
    ) -> Result<&RegisteredProductionCandidate, SpecError> {
        self.entries.get(stable_key).ok_or_else(|| {
            shell_err(
                stable_key,
                "execution requires prior production shell registration",
            )
        })
    }
}
