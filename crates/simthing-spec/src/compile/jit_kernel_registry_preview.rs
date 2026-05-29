//! Phase M-JIT-REG-0 — Test-only kernel registry manifest preview (spec layer).
//!
//! Builds a deterministic registry-shaped manifest from admitted graph cohort previews.
//! No production registry, no runtime cache, no scheduler, no GPU dispatch, no WGSL.

use crate::compile::jit_kernel_cohort_preview::{
    preview_kernel_graph_cohorts, KernelGraphRequestSpec,
};
use crate::error::SpecError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelRegistryLane {
    TestOnlyPreview,
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
            cohort_to_registry_entry(
                cohort.stable_key,
                cohort.canonical_text,
                cohort.request_ids,
            )
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
        return Err(registry_err("registry", "manifest must contain at least one entry"));
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
