//! Phase M-JIT-DESC-2 — JIT kernel graph composition admission preview (spec layer).
//!
//! Validates directed descriptor graphs before any production registry/scheduler exists.
//! No scheduling order bands, no kernel cache, no GPU dispatch.

use std::collections::{HashMap, HashSet};

use crate::compile::jit_kernel_descriptor_admission::{
    validate_kernel_descriptor_admission, KernelDescriptorSpec, OutputAuthority,
};
use crate::error::SpecError;

#[derive(Debug, Clone)]
pub struct KernelGraphSpec {
    pub nodes: Vec<KernelDescriptorSpec>,
    pub edges: Vec<KernelGraphEdgeSpec>,
}

#[derive(Debug, Clone)]
pub struct KernelGraphEdgeSpec {
    pub from_kernel: String,
    pub from_output: String,
    pub to_kernel: String,
    pub to_input: String,
    pub required_authority: OutputAuthority,
}

fn graph_err(context: &str, reason: impl Into<String>) -> SpecError {
    SpecError::JitKernelDescriptorAdmission {
        kernel: context.to_string(),
        reason: reason.into(),
    }
}

fn node_index(nodes: &[KernelDescriptorSpec]) -> HashMap<String, usize> {
    nodes
        .iter()
        .enumerate()
        .map(|(idx, node)| (node.id.clone(), idx))
        .collect()
}

fn producer_output_authority(
    producer: &KernelDescriptorSpec,
    output: &str,
) -> Option<OutputAuthority> {
    producer
        .writes
        .iter()
        .find(|out| out.name == output)
        .map(|out| out.authority)
}

fn has_cycle(nodes: &[KernelDescriptorSpec], edges: &[KernelGraphEdgeSpec]) -> bool {
    let index = node_index(nodes);
    let n = nodes.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];

    for edge in edges {
        let Some(&from) = index.get(&edge.from_kernel) else {
            continue;
        };
        let Some(&to) = index.get(&edge.to_kernel) else {
            continue;
        };
        if from != to {
            adj[from].push(to);
        }
    }

    let mut state = vec![0u8; n]; // 0=unvisited, 1=visiting, 2=done

    fn dfs(u: usize, adj: &[Vec<usize>], state: &mut [u8]) -> bool {
        state[u] = 1;
        for &v in &adj[u] {
            if state[v] == 1 {
                return true;
            }
            if state[v] == 0 && dfs(v, adj, state) {
                return true;
            }
        }
        state[u] = 2;
        false
    }

    (0..n).any(|i| state[i] == 0 && dfs(i, &adj, &mut state))
}

/// Admit a kernel descriptor graph under DESC-2 preview policy.
pub fn validate_kernel_graph_admission(graph: &KernelGraphSpec) -> Result<(), SpecError> {
    if graph.nodes.is_empty() {
        return Err(graph_err("graph", "graph must contain at least one node"));
    }

    let mut seen_ids = HashSet::new();
    for node in &graph.nodes {
        if !seen_ids.insert(node.id.clone()) {
            return Err(graph_err(
                &node.id,
                format!("duplicate node id `{}`", node.id),
            ));
        }
        validate_kernel_descriptor_admission(node)?;
    }

    let index = node_index(&graph.nodes);

    for edge in &graph.edges {
        if edge.from_kernel == edge.to_kernel {
            return Err(graph_err(
                &edge.from_kernel,
                format!(
                    "self-edge from `{}` output `{}` to input `{}`",
                    edge.from_kernel, edge.from_output, edge.to_input
                ),
            ));
        }

        let Some(&from_idx) = index.get(&edge.from_kernel) else {
            return Err(graph_err(
                "graph",
                format!(
                    "edge references missing producer node `{}`",
                    edge.from_kernel
                ),
            ));
        };
        let Some(&to_idx) = index.get(&edge.to_kernel) else {
            return Err(graph_err(
                "graph",
                format!("edge references missing consumer node `{}`", edge.to_kernel),
            ));
        };

        let producer = &graph.nodes[from_idx];
        let consumer = &graph.nodes[to_idx];

        let Some(actual_authority) = producer_output_authority(producer, &edge.from_output) else {
            return Err(graph_err(
                &producer.id,
                format!(
                    "edge output `{}` not produced by kernel `{}`",
                    edge.from_output, producer.id
                ),
            ));
        };

        if !consumer.reads.iter().any(|read| read == &edge.to_input) {
            return Err(graph_err(
                &consumer.id,
                format!(
                    "edge input `{}` not declared by consumer kernel `{}`",
                    edge.to_input, consumer.id
                ),
            ));
        }

        match edge.required_authority {
            OutputAuthority::ExactAuthoritative => match actual_authority {
                OutputAuthority::ExactAuthoritative => {}
                OutputAuthority::ApproximateDiagnostic => {
                    return Err(graph_err(
                        &producer.id,
                        format!(
                            "output `{}` is approximate/diagnostic but edge to `{}`.{} requires exact-authoritative input",
                            edge.from_output, consumer.id, edge.to_input
                        ),
                    ));
                }
                OutputAuthority::RejectedDeferred => {
                    return Err(graph_err(
                        &producer.id,
                        format!(
                            "output `{}` is rejected/deferred but edge to `{}`.{} requires exact-authoritative input",
                            edge.from_output, consumer.id, edge.to_input
                        ),
                    ));
                }
            },
            OutputAuthority::ApproximateDiagnostic => {
                if actual_authority != OutputAuthority::ApproximateDiagnostic {
                    return Err(graph_err(
                        &producer.id,
                        format!(
                            "output `{}` is not approximate/diagnostic but edge requires ApproximateDiagnostic authority",
                            edge.from_output
                        ),
                    ));
                }
            }
            OutputAuthority::RejectedDeferred => {
                if actual_authority != OutputAuthority::RejectedDeferred {
                    return Err(graph_err(
                        &producer.id,
                        format!(
                            "output `{}` is not rejected/deferred but edge requires RejectedDeferred authority",
                            edge.from_output
                        ),
                    ));
                }
            }
        }
    }

    if has_cycle(&graph.nodes, &graph.edges) {
        return Err(graph_err("graph", "descriptor graph contains a cycle"));
    }

    Ok(())
}
