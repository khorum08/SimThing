//! Phase M-JIT-COHORT-0 — Kernel graph cohort grouping preview (spec layer).
//!
//! Groups admitted graph requests by deterministic identity for future JIT cohort dispatch.
//! No runtime cache, no scheduler, no GPU dispatch, no WGSL.

use std::collections::BTreeMap;

use crate::compile::jit_kernel_graph_admission::KernelGraphSpec;
use crate::compile::jit_kernel_graph_identity::preview_kernel_graph_identity;
use crate::error::SpecError;

#[derive(Debug, Clone)]
pub struct KernelGraphRequestSpec {
    pub request_id: String,
    pub graph: KernelGraphSpec,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelGraphCohortPreview {
    pub stable_key: String,
    pub canonical_text: String,
    pub request_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelGraphCohortPreviewSet {
    pub cohorts: Vec<KernelGraphCohortPreview>,
}

fn cohort_err(context: &str, reason: impl Into<String>) -> SpecError {
    SpecError::JitKernelDescriptorAdmission {
        kernel: context.to_string(),
        reason: reason.into(),
    }
}

fn group_resolved_requests(
    resolved: &[(String, String, String)], // (request_id, stable_key, canonical_text)
) -> Result<KernelGraphCohortPreviewSet, SpecError> {
    let mut groups: BTreeMap<String, (String, Vec<String>)> = BTreeMap::new();

    for (request_id, stable_key, canonical_text) in resolved {
        match groups.get_mut(stable_key) {
            Some((existing_canonical, request_ids)) => {
                if existing_canonical != canonical_text {
                    return Err(cohort_err(
                        "cohort",
                        format!(
                            "stable key `{stable_key}` maps to conflicting canonical text for request `{request_id}`"
                        ),
                    ));
                }
                request_ids.push(request_id.clone());
            }
            None => {
                groups.insert(
                    stable_key.clone(),
                    (canonical_text.clone(), vec![request_id.clone()]),
                );
            }
        }
    }

    let mut cohorts = Vec::with_capacity(groups.len());
    for (stable_key, (canonical_text, mut request_ids)) in groups {
        request_ids.sort();
        cohorts.push(KernelGraphCohortPreview {
            stable_key,
            canonical_text,
            request_ids,
        });
    }

    Ok(KernelGraphCohortPreviewSet { cohorts })
}

/// Preview cohort grouping for a batch of kernel graph requests.
pub fn preview_kernel_graph_cohorts(
    requests: &[KernelGraphRequestSpec],
) -> Result<KernelGraphCohortPreviewSet, SpecError> {
    if requests.is_empty() {
        return Err(cohort_err("cohort", "request set must not be empty"));
    }

    let mut seen_request_ids = std::collections::HashSet::new();
    for request in requests {
        if !seen_request_ids.insert(request.request_id.clone()) {
            return Err(cohort_err(
                &request.request_id,
                format!("duplicate request id `{}`", request.request_id),
            ));
        }
    }

    let mut resolved = Vec::with_capacity(requests.len());
    for request in requests {
        let identity = preview_kernel_graph_identity(&request.graph)?;
        resolved.push((
            request.request_id.clone(),
            identity.stable_key,
            identity.canonical_text,
        ));
    }

    group_resolved_requests(&resolved)
}

/// Test-only helper to exercise cohort grouping collision guard without weakening identity preview.
#[doc(hidden)]
pub fn test_group_cohort_previews_from_resolved(
    requests: &[KernelGraphRequestSpec],
    resolved: &[(String, String)], // (stable_key, canonical_text) per request in order
) -> Result<KernelGraphCohortPreviewSet, SpecError> {
    if requests.len() != resolved.len() {
        return Err(cohort_err(
            "cohort",
            "request count must match resolved identity count",
        ));
    }

    let mut seen_request_ids = std::collections::HashSet::new();
    for request in requests {
        if !seen_request_ids.insert(request.request_id.clone()) {
            return Err(cohort_err(
                &request.request_id,
                format!("duplicate request id `{}`", request.request_id),
            ));
        }
    }

    let triples = requests
        .iter()
        .zip(resolved.iter())
        .map(|(request, (stable_key, canonical_text))| {
            (
                request.request_id.clone(),
                stable_key.clone(),
                canonical_text.clone(),
            )
        })
        .collect::<Vec<_>>();

    group_resolved_requests(&triples)
}
