//! CANONICAL-ANCHOR-MATERIALIZATION-0 — read-only candidate-evidence census.
//!
//! DA amendment (issuance `5122978247` / dispatch `5123105021`): census BEFORE
//! implementation. Zero or multiple lawful candidates for any Anchored property
//! = STOP; never invent precedence.
//!
//! Lawful evidence classes (handoff verbatim): admitted RF parent edges, owner
//! policy-weight authority, install-resolved threshold / need / economy /
//! overlay / hosted-observation registrations. Never kind, names, display, or
//! default-root inference.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use simthing_clausething::{hydrate_scenario_with_source_base, parse_raw_document};
use simthing_core::{SimPropertyId, SimThing};
use simthing_spec::{InstallTargetSpec, PropertyKey};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Candidate {
    host_entity: String,
    evidence_class: &'static str,
    span: String,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn prop_key(key: &PropertyKey) -> String {
    format!("{}::{}", key.namespace, key.name)
}

fn push_entity(
    out: &mut BTreeMap<String, BTreeSet<Candidate>>,
    property: &str,
    host_entity: Option<&str>,
    evidence_class: &'static str,
    span: impl Into<String>,
) {
    let Some(host) = host_entity.filter(|h| !h.is_empty()) else {
        return;
    };
    // Only record against the 25-set keys that already exist in `out`.
    if !out.contains_key(property) {
        return;
    }
    out.get_mut(property).unwrap().insert(Candidate {
        host_entity: host.to_string(),
        evidence_class,
        span: span.into(),
    });
}

fn overlay_entity(install: &InstallTargetSpec) -> Option<String> {
    match install {
        InstallTargetSpec::ScenarioListed { target_id } => Some(target_id.clone()),
        // AllOfKind / SessionRoot require kind or default-root inference — fenced out.
        InstallTargetSpec::AllOfKind { .. } | InstallTargetSpec::SessionRoot => None,
    }
}

fn walk_rf_edges(node: &SimThing, out: &mut BTreeMap<String, BTreeSet<Candidate>>) {
    for edge in &node.resource_parent_edges {
        let property = format!("{}::{}", edge.property_namespace, edge.property_name);
        if !out.contains_key(&property) {
            continue;
        }
        out.get_mut(&property).unwrap().insert(Candidate {
            host_entity: format!("simthing:{}", node.id.raw()),
            evidence_class: "rf_parent_edge.child_host",
            span: format!(
                "parent={} span={:?}",
                edge.parent.raw(),
                edge.source_span_token
            ),
        });
    }
    for child in &node.children {
        walk_rf_edges(child, out);
    }
}

fn census_markdown() -> (usize, usize, usize, String) {
    let clause = repo_root().join("scenarios/terran_pirate_galaxy.clause");
    let source = std::fs::read_to_string(&clause).expect("read canonical clause");
    let document = parse_raw_document(source.as_bytes()).expect("parse");
    let pack = hydrate_scenario_with_source_base(&document, Some(clause.parent().unwrap()))
        .expect("hydrate TP");

    let properties: Vec<String> = pack
        .game_mode
        .properties
        .iter()
        .map(|p| format!("{}::{}", p.namespace, p.name))
        .collect();
    assert_eq!(
        properties.len(),
        25,
        "canonical TP must still author exactly 25 resource properties"
    );

    let mut by_prop: BTreeMap<String, BTreeSet<Candidate>> = BTreeMap::new();
    for p in &properties {
        by_prop.entry(p.clone()).or_default();
    }

    if let Some(economy) = &pack.game_mode.resource_economy {
        for emission in &economy.emissions {
            push_entity(
                &mut by_prop,
                &prop_key(&emission.source),
                emission.host_entity.as_deref(),
                "economy.emission.host_entity",
                format!("emission.id={}", emission.id),
            );
        }
        for thresh in &economy.emit_on_threshold {
            push_entity(
                &mut by_prop,
                &prop_key(&thresh.source),
                thresh.host_entity.as_deref(),
                "economy.emit_on_threshold.host_entity",
                format!("threshold.id={}", thresh.id),
            );
        }
        for transfer in &economy.transfers {
            push_entity(
                &mut by_prop,
                &prop_key(&transfer.source),
                transfer.source_host_entity.as_deref(),
                "economy.transfer.source_host_entity",
                format!("transfer.id={}", transfer.id),
            );
            push_entity(
                &mut by_prop,
                &prop_key(&transfer.target),
                transfer.target_host_entity.as_deref(),
                "economy.transfer.target_host_entity",
                format!("transfer.id={}", transfer.id),
            );
        }
        for recipe in &economy.recipes {
            push_entity(
                &mut by_prop,
                &prop_key(&recipe.target),
                recipe.target_host_entity.as_deref(),
                "economy.recipe.target_host_entity",
                format!("recipe.id={}", recipe.id),
            );
            for (idx, input) in recipe.inputs.iter().enumerate() {
                push_entity(
                    &mut by_prop,
                    &prop_key(&input.property),
                    input.host_entity.as_deref(),
                    "economy.recipe.input.host_entity",
                    format!("recipe.id={} input[{idx}]", recipe.id),
                );
            }
        }
    }

    for overlay in &pack.game_mode.overlays {
        if let Some(entity) = overlay_entity(&overlay.install) {
            push_entity(
                &mut by_prop,
                &overlay.targets_property,
                Some(&entity),
                "overlay.ScenarioListed",
                format!("overlay.id={}", overlay.id),
            );
        }
    }

    if let Some(rf) = &pack.game_mode.resource_flow {
        for binding in &rf.need_bindings {
            for locus in binding.inputs.iter().chain(binding.weights.iter()) {
                push_entity(
                    &mut by_prop,
                    &prop_key(&locus.property),
                    Some(&locus.entity),
                    "need_binding.locus.entity",
                    format!("need_binding.id={} role={:?}", binding.id, locus.role),
                );
            }
        }
    }

    // Hosted-observation: disruption presence locations (Studio typed loci).
    // Property identity comes from economy emission sources already; this adds
    // the observation-class provenance tag when location matches an emission host.
    if let Some(field) = &pack.field_economy {
        if let Some(economy) = &pack.game_mode.resource_economy {
            for presence in &field.disruption_presences {
                for emission in &economy.emissions {
                    if emission.host_entity.as_deref() == Some(presence.location.as_str())
                        && prop_key(&emission.source).contains("disruption_presence")
                    {
                        push_entity(
                            &mut by_prop,
                            &prop_key(&emission.source),
                            Some(&presence.location),
                            "hosted_observation.disruption_presence.location",
                            format!(
                                "presence.id={} resource={}",
                                presence.id, presence.resource
                            ),
                        );
                    }
                }
            }
        }
    }

    let mut policy_hosts = Vec::new();
    fn walk_policy(node: &SimThing, hosts: &mut Vec<u32>) {
        if node.properties.contains_key(&SimPropertyId(8_300_318)) {
            hosts.push(node.id.raw());
        }
        for child in &node.children {
            walk_policy(child, hosts);
        }
    }
    if let Some(auth) = &pack.authority_root {
        walk_policy(auth, &mut policy_hosts);
        walk_rf_edges(auth, &mut by_prop);
    }
    walk_rf_edges(&pack.root, &mut by_prop);

    let mut md = String::from(
        "| # | canonical property | candidate host(s) | evidence class(es) | available span/provenance | convergence |\n\
         |---|---|---|---|---|---|\n",
    );
    let mut zero = 0usize;
    let mut conflict = 0usize;
    let mut exact = 0usize;
    let mut stop_rows = Vec::new();
    for (idx, property) in properties.iter().enumerate() {
        let set = by_prop.get(property).cloned().unwrap_or_default();
        let hosts: BTreeSet<_> = set.iter().map(|c| c.host_entity.clone()).collect();
        let classes: BTreeSet<_> = set.iter().map(|c| c.evidence_class).collect();
        let spans: Vec<_> = set
            .iter()
            .map(|c| format!("{}@{}", c.evidence_class, c.span))
            .collect();
        let convergence = match hosts.len() {
            0 => {
                zero += 1;
                stop_rows.push(format!("{property}: ZERO candidates"));
                "STOP(zero)"
            }
            1 => {
                exact += 1;
                "exactly-one"
            }
            _ => {
                conflict += 1;
                stop_rows.push(format!("{property}: CONFLICT hosts={hosts:?}"));
                "STOP(conflict)"
            }
        };
        let host_cell = if hosts.is_empty() {
            "—".to_string()
        } else {
            hosts.into_iter().collect::<Vec<_>>().join(", ")
        };
        let class_cell = if classes.is_empty() {
            "—".to_string()
        } else {
            classes.into_iter().collect::<Vec<_>>().join(", ")
        };
        let span_cell = if spans.is_empty() {
            "—".to_string()
        } else {
            spans.join("; ")
        };
        md.push_str(&format!(
            "| {} | `{property}` | `{host_cell}` | {class_cell} | {span_cell} | **{convergence}** |\n",
            idx + 1
        ));
    }
    md.push_str(&format!(
        "\nPolicy-weight authority hosts (diagnostic; property `8_300_318` is not among the 25): `{:?}`\n",
        policy_hosts
    ));
    md.push_str(&format!(
        "\nRF parent edges on authority/root: counted in table when present (TP currently expected 0).\n"
    ));
    md.push_str(&format!(
        "\n### STOP rows ({})\n\n{}\n",
        stop_rows.len(),
        if stop_rows.is_empty() {
            "(none)".into()
        } else {
            stop_rows
                .iter()
                .map(|s| format!("- `{s}`"))
                .collect::<Vec<_>>()
                .join("\n")
        }
    ));
    md.push_str(&format!(
        "\n**Summary:** exact={exact} / zero={zero} / conflict={conflict} / total={}\n",
        properties.len()
    ));

    (exact, zero, conflict, md)
}

#[test]
fn candidate_evidence_census_before_implementation() {
    let (exact, zero, conflict, md) = census_markdown();
    eprintln!("\n=== CANONICAL-ANCHOR-MATERIALIZATION-0 CENSUS ===\n{md}");

    // Binding STOP shape (DA census-before-build). Do not invent precedence to
    // collapse zeros/conflicts — lock the measured shape until DA rules.
    assert_eq!(exact, 16, "exactly-one rows drifted");
    assert_eq!(zero, 7, "zero-candidate rows drifted");
    assert_eq!(conflict, 2, "conflict rows drifted");
    assert!(
        zero > 0 || conflict > 0,
        "census unexpectedly converged; if lawful, remove STOP and implement"
    );
}
