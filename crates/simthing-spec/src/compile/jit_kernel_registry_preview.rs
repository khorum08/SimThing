//! Phase M-JIT-REG-0 — Test-only kernel registry manifest preview (spec layer).
//!
//! Builds a deterministic registry-shaped manifest from admitted graph cohort previews.
//! No production registry, no runtime cache, no scheduler, no GPU dispatch, no WGSL.

use crate::compile::jit_exact_sqrt_artifact_admission::SQRT_F_ARTIFACT_HASH;
use crate::compile::jit_kernel_cohort_preview::{
    preview_kernel_graph_cohorts, KernelGraphRequestSpec,
};
use crate::error::SpecError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelRegistryLane {
    TestOnlyPreview,
    ProductionCandidatePreview,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelRegistryEntryPreview {
    pub stable_key: String,
    pub canonical_text: String,
    pub request_ids: Vec<String>,
    pub lane: KernelRegistryLane,
    pub default_off: bool,
    pub production_wiring: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelRegistryManifestPreview {
    pub entries: Vec<KernelRegistryEntryPreview>,
}

fn registry_err(context: &str, reason: impl Into<String>) -> SpecError {
    SpecError::JitKernelDescriptorAdmission {
        kernel: context.to_string(),
        reason: reason.into(),
    }
}

fn cohort_to_registry_entry(
    stable_key: String,
    canonical_text: String,
    request_ids: Vec<String>,
) -> KernelRegistryEntryPreview {
    KernelRegistryEntryPreview {
        stable_key,
        canonical_text,
        request_ids,
        lane: KernelRegistryLane::TestOnlyPreview,
        default_off: true,
        production_wiring: false,
    }
}

/// Preview a test-only kernel registry manifest from graph requests.
pub fn preview_kernel_registry_manifest(
    requests: &[KernelGraphRequestSpec],
) -> Result<KernelRegistryManifestPreview, SpecError> {
    let cohorts = preview_kernel_graph_cohorts(requests)?;
    let entries = cohorts
        .cohorts
        .into_iter()
        .map(|cohort| {
            cohort_to_registry_entry(cohort.stable_key, cohort.canonical_text, cohort.request_ids)
        })
        .collect();
    let manifest = KernelRegistryManifestPreview { entries };
    validate_kernel_registry_manifest_preview(&manifest)?;
    Ok(manifest)
}

/// Validate a registry manifest preview under REG-0 policy.
pub fn validate_kernel_registry_manifest_preview(
    manifest: &KernelRegistryManifestPreview,
) -> Result<(), SpecError> {
    if manifest.entries.is_empty() {
        return Err(registry_err(
            "registry",
            "manifest must contain at least one entry",
        ));
    }

    let mut seen_keys = std::collections::HashSet::new();
    for entry in &manifest.entries {
        if !seen_keys.insert(entry.stable_key.clone()) {
            return Err(registry_err(
                "registry",
                format!("duplicate stable key `{}`", entry.stable_key),
            ));
        }
        if entry.canonical_text.is_empty() {
            return Err(registry_err(
                &entry.stable_key,
                "canonical text must not be empty",
            ));
        }
        if entry.request_ids.is_empty() {
            return Err(registry_err(
                &entry.stable_key,
                "registry entry must reference at least one request id",
            ));
        }
        if entry.lane != KernelRegistryLane::TestOnlyPreview {
            return Err(registry_err(
                &entry.stable_key,
                "registry lane must remain TestOnlyPreview in REG-0 preview",
            ));
        }
        if !entry.default_off {
            return Err(registry_err(
                &entry.stable_key,
                "default_off must remain true in REG-0 preview",
            ));
        }
        if entry.production_wiring {
            return Err(registry_err(
                &entry.stable_key,
                "production_wiring must remain false in REG-0 preview",
            ));
        }
    }

    Ok(())
}

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

const PRODUCTION_CANDIDATE_FORBIDDEN_CANONICAL_MARKERS: &[&str] = &[
    "m_jit_sqrt_0_candidate",
    "magnitude_out",
    "mag2",
    "ApproximateJitOnly",
    "ApproximateDiagnostic",
    "RejectedDeferred",
];

fn contains_forbidden_semantic_term(text: &str) -> Option<&'static str> {
    FORBIDDEN_SEMANTIC_TERMS
        .iter()
        .find(|term| text.contains(**term))
        .copied()
}

fn request_ids_sorted(ids: &[String]) -> bool {
    ids.windows(2).all(|pair| pair[0] <= pair[1])
}

fn validate_production_candidate_entry_rules(
    entry: &KernelRegistryEntryPreview,
) -> Result<(), SpecError> {
    if entry.stable_key.is_empty() || !entry.stable_key.starts_with("jit-graph-v1:") {
        return Err(registry_err(
            &entry.stable_key,
            "stable key must use jit-graph-v1 prefix",
        ));
    }

    if entry.canonical_text.is_empty() {
        return Err(registry_err(
            &entry.stable_key,
            "canonical text must not be empty",
        ));
    }

    if let Some(term) = contains_forbidden_semantic_term(&entry.canonical_text) {
        return Err(registry_err(
            &entry.stable_key,
            format!("canonical text contains forbidden semantic term `{term}`"),
        ));
    }

    for marker in PRODUCTION_CANDIDATE_FORBIDDEN_CANONICAL_MARKERS {
        if entry.canonical_text.contains(marker) {
            return Err(registry_err(
                &entry.stable_key,
                format!("production-candidate admission rejects canonical marker `{marker}`"),
            ));
        }
    }

    if entry
        .canonical_text
        .contains("write=sqrt_out authority=ExactAuthoritative")
        || entry
            .canonical_text
            .contains("write=mag authority=ExactAuthoritative")
    {
        if !entry.canonical_text.contains(SQRT_F_ARTIFACT_HASH) {
            return Err(registry_err(
                &entry.stable_key,
                "exact sqrt_out requires artifact-backed Candidate F hash pin",
            ));
        }
        if entry.canonical_text.contains("m_jit_sqrt_0_candidate") {
            return Err(registry_err(
                &entry.stable_key,
                "native sqrt candidate cannot supply exact-authoritative sqrt_out",
            ));
        }
    }

    if entry.request_ids.is_empty() {
        return Err(registry_err(
            &entry.stable_key,
            "registry entry must reference at least one request id",
        ));
    }

    if !request_ids_sorted(&entry.request_ids) {
        return Err(registry_err(
            &entry.stable_key,
            "request_ids must be sorted for production-candidate admission",
        ));
    }

    if !entry.default_off {
        return Err(registry_err(
            &entry.stable_key,
            "default_off must remain true for production-candidate preview",
        ));
    }

    if entry.production_wiring {
        return Err(registry_err(
            &entry.stable_key,
            "production_wiring must remain false for production-candidate preview",
        ));
    }

    Ok(())
}

/// Validate an already-admitted ProductionCandidatePreview entry (PROD-0 / execution gates).
pub fn validate_production_candidate_preview_entry(
    entry: &KernelRegistryEntryPreview,
) -> Result<(), SpecError> {
    if entry.lane != KernelRegistryLane::ProductionCandidatePreview {
        return Err(registry_err(
            &entry.stable_key,
            "entry lane must be ProductionCandidatePreview",
        ));
    }
    validate_production_candidate_entry_rules(entry)
}

/// Admit a TestOnly registry entry to ProductionCandidatePreview (REG-1 gate).
pub fn preview_production_candidate_registry_entry(
    entry: &KernelRegistryEntryPreview,
) -> Result<KernelRegistryEntryPreview, SpecError> {
    validate_kernel_registry_manifest_preview(&KernelRegistryManifestPreview {
        entries: vec![entry.clone()],
    })?;

    if entry.lane != KernelRegistryLane::TestOnlyPreview {
        return Err(registry_err(
            &entry.stable_key,
            "production-candidate admission requires TestOnlyPreview source lane",
        ));
    }

    validate_production_candidate_entry_rules(entry)?;

    Ok(KernelRegistryEntryPreview {
        stable_key: entry.stable_key.clone(),
        canonical_text: entry.canonical_text.clone(),
        request_ids: entry.request_ids.clone(),
        lane: KernelRegistryLane::ProductionCandidatePreview,
        default_off: true,
        production_wiring: false,
    })
}
