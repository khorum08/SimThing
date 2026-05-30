//! Phase M-JIT-KEY-0 — Deterministic kernel graph identity / cache-key preview (spec layer).
//!
//! Canonicalizes admitted descriptor graphs into stable identity suitable for future
//! cohorting/cache lookup. No runtime cache, no scheduler, no GPU dispatch, no WGSL.

use crate::compile::jit_kernel_descriptor_admission::{
    KernelDescriptorSpec, KernelLane, KernelOutputSpec, NativeMathClass, OutputAuthority,
};
use crate::compile::jit_kernel_graph_admission::{
    validate_kernel_graph_admission, KernelGraphEdgeSpec, KernelGraphSpec,
};
use crate::error::SpecError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelGraphIdentity {
    pub canonical_text: String,
    pub stable_key: String,
}

fn format_lane(lane: KernelLane) -> &'static str {
    match lane {
        KernelLane::TestOnly => "TestOnly",
        KernelLane::ProductionCandidate => "ProductionCandidate",
    }
}

fn format_authority(authority: OutputAuthority) -> &'static str {
    match authority {
        OutputAuthority::ExactAuthoritative => "ExactAuthoritative",
        OutputAuthority::ApproximateDiagnostic => "ApproximateDiagnostic",
        OutputAuthority::RejectedDeferred => "RejectedDeferred",
    }
}

fn format_native_math(native_math: NativeMathClass) -> &'static str {
    match native_math {
        NativeMathClass::None => "None",
        NativeMathClass::ApproximateJitOnly => "ApproximateJitOnly",
    }
}

fn format_mag2_source_contract(
    contract: Option<crate::compile::jit_exact_sqrt_artifact_admission::Mag2SourceContract>,
) -> String {
    match contract {
        None => "None".to_string(),
        Some(crate::compile::jit_exact_sqrt_artifact_admission::Mag2SourceContract::ExactFixedPointDxDy {
            fraction_bits,
        }) => format!("ExactFixedPointDxDy(fraction_bits={fraction_bits})"),
    }
}

fn format_pre_sqrt_contract(
    contract: Option<crate::compile::jit_exact_sqrt_artifact_admission::ExactPreSqrtInputContract>,
) -> &'static str {
    match contract {
        None => "None",
        Some(crate::compile::jit_exact_sqrt_artifact_admission::ExactPreSqrtInputContract::ExactMag2Bits) => {
            "ExactMag2Bits"
        }
        Some(crate::compile::jit_exact_sqrt_artifact_admission::ExactPreSqrtInputContract::RawDxDyProbe) => {
            "RawDxDyProbe"
        }
    }
}

fn format_bool(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn canonicalize_node(node: &KernelDescriptorSpec) -> String {
    let mut reads = node.reads.clone();
    reads.sort();

    let mut writes = node.writes.clone();
    writes.sort_by(|a, b| a.name.cmp(&b.name));

    let mut lines = vec![format!(
        "node id={} lane={} native_math={} semantic_free={} default_off={} production_wiring={} pre_sqrt_contract={} m2src_contract={}",
        node.id,
        format_lane(node.lane),
        format_native_math(node.native_math),
        format_bool(node.semantic_free),
        format_bool(node.default_off),
        format_bool(node.production_wiring),
        format_pre_sqrt_contract(node.pre_sqrt_contract),
        format_mag2_source_contract(node.mag2_source_contract),
    )];

    for read in reads {
        lines.push(format!("  read={read}"));
    }
    for KernelOutputSpec { name, authority } in writes {
        lines.push(format!(
            "  write={name} authority={}",
            format_authority(authority)
        ));
    }

    if let Some(artifact) = &node.exact_sqrt_artifact {
        lines.push(format!("  artifact_path={}", artifact.artifact_path));
        lines.push(format!(
            "  artifact_hash_fnv1a64={}",
            artifact.artifact_hash_fnv1a64
        ));
        lines.push(format!("  artifact_entrypoint={}", artifact.entrypoint));
        lines.push(format!("  artifact_io_contract={}", artifact.io_contract));
        lines.push(format!("  artifact_proof_report={}", artifact.proof_report));
        lines.push(format!("  artifact_domain={}", artifact.domain));
        lines.push("  artifact_authority_class=ExactDeterministic".to_string());
    }

    lines.join("\n")
}

fn canonicalize_edge(edge: &KernelGraphEdgeSpec) -> String {
    format!(
        "edge from={} out={} to={} in={} authority={}",
        edge.from_kernel,
        edge.from_output,
        edge.to_kernel,
        edge.to_input,
        format_authority(edge.required_authority),
    )
}

fn canonicalize_graph(graph: &KernelGraphSpec) -> String {
    let mut nodes = graph.nodes.clone();
    nodes.sort_by(|a, b| a.id.cmp(&b.id));

    let mut edges = graph.edges.clone();
    edges.sort_by(|a, b| {
        (
            &a.from_kernel,
            &a.from_output,
            &a.to_kernel,
            &a.to_input,
            format_authority(a.required_authority),
        )
            .cmp(&(
                &b.from_kernel,
                &b.from_output,
                &b.to_kernel,
                &b.to_input,
                format_authority(b.required_authority),
            ))
    });

    let mut lines = vec!["graph".to_string()];
    for node in &nodes {
        lines.push(canonicalize_node(node));
    }
    for edge in &edges {
        lines.push(canonicalize_edge(edge));
    }
    lines.join("\n")
}

fn fnv1a_64(data: &[u8]) -> u64 {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;
    let mut hash = OFFSET;
    for byte in data {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

fn stable_key_from_canonical(canonical_text: &str) -> String {
    format!("jit-graph-v1:{:016x}", fnv1a_64(canonical_text.as_bytes()))
}

/// Preview deterministic identity for an admitted kernel graph.
pub fn preview_kernel_graph_identity(
    graph: &KernelGraphSpec,
) -> Result<KernelGraphIdentity, SpecError> {
    validate_kernel_graph_admission(graph)?;
    let canonical_text = canonicalize_graph(graph);
    let stable_key = stable_key_from_canonical(&canonical_text);
    Ok(KernelGraphIdentity {
        canonical_text,
        stable_key,
    })
}
