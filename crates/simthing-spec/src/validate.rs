use crate::diagnostics::{SpecDiagnostic, SpecDiagnostics};
use crate::error::SpecError;
use crate::spec::capability::{
    ActivationMode, CapabilityPrereqSpec, CapabilityTreeSpec, MaxActivePolicy, ReplacementPolicy,
};
use std::collections::{HashMap, HashSet};

/// Authored-spec admission validation for capability trees (0.0.8.7 rung 2.2).
///
/// Proves the prereq graph is typed DAG data at admission: no cycles, no dangling
/// or cross-tree references, no self-prerequisites, tier order consistent with
/// edges, and `max_active` well-formed. Runtime gate checks stay boundary work —
/// only the DATA is proven-shaped here.
pub fn validate_capability_tree(spec: &CapabilityTreeSpec) -> Result<SpecDiagnostics, SpecError> {
    let mut diagnostics = SpecDiagnostics::default();
    let mut seen_categories = HashSet::new();
    let mut seen_entries = HashSet::new();

    // Index: category_key -> (tier, source_span, entry_ids)
    let mut category_index: HashMap<String, CategoryIndex> = HashMap::new();
    // Index: "category::entry" node id for the DAG.
    let mut entry_nodes: HashMap<String, EntryNode> = HashMap::new();

    for category in &spec.categories {
        let category_key = format!(
            "{}::{}",
            category.property_namespace, category.property_name
        );
        if !seen_categories.insert(category_key.clone()) {
            return Err(SpecError::DuplicateCategory(
                category_key,
                spec.tree_id.clone(),
            ));
        }

        // max_active well-formed: bounds sane; members (entries) exist.
        match &category.max_active {
            None | Some(MaxActivePolicy::Unlimited) => {}
            Some(MaxActivePolicy::Limited { count, replacement }) => {
                if *count == 0 {
                    return Err(SpecError::MalformedMaxActive {
                        in_tree: spec.tree_id.clone(),
                        category: category_key.clone(),
                        reason: "Limited count must be >= 1".into(),
                        source_span_token: category.source_span_token,
                    });
                }
                if category.entries.is_empty() {
                    return Err(SpecError::MalformedMaxActive {
                        in_tree: spec.tree_id.clone(),
                        category: category_key.clone(),
                        reason: "Limited max_active requires at least one member entry".into(),
                        source_span_token: category.source_span_token,
                    });
                }
                // v0 policy: Unlimited or Limited(1, SuspendOldest) only.
                if *count != 1 || *replacement != ReplacementPolicy::SuspendOldest {
                    return Err(SpecError::UnsupportedMaxActive {
                        in_tree: spec.tree_id.clone(),
                        category: category_key.clone(),
                        count: *count,
                        source_span_token: category.source_span_token,
                    });
                }
            }
        }

        let mut entry_ids = HashSet::new();
        for entry in &category.entries {
            let entry_key = format!("{category_key}::{}", entry.id);
            if !seen_entries.insert(entry_key.clone()) {
                return Err(SpecError::DuplicateEntry(
                    entry.id.clone(),
                    spec.tree_id.clone(),
                ));
            }
            entry_ids.insert(entry.id.clone());

            if entry.activation == ActivationMode::OnPrereqMet {
                return Err(SpecError::OnPrereqMetAuthoredDefault(entry.id.clone()));
            }

            if entry.research_cost < 0.0 {
                return Err(SpecError::NegativeResearchCost(entry.id.clone()));
            }

            if entry.activation == ActivationMode::Threshold && entry.research_cost <= 0.0 {
                return Err(SpecError::ThresholdRequiresPositiveCost(entry.id.clone()));
            }

            if entry.effects.is_empty() {
                diagnostics.push(SpecDiagnostic::warning(
                    "capability.empty_effects",
                    format!("entry `{}` has no effects", entry.id),
                ));
            }

            entry_nodes.insert(
                entry_key,
                EntryNode {
                    entry_id: entry.id.clone(),
                    tier: category.tier,
                    prereqs: entry.prereqs.clone(),
                },
            );
        }

        if category.max_active.is_some()
            && !category
                .entries
                .iter()
                .any(|e| e.activation == ActivationMode::PlayerSelection)
        {
            diagnostics.push(SpecDiagnostic::warning(
                "capability.max_active_without_player_selection",
                format!(
                    "category `{category_key}` sets max_active but has no PlayerSelection entries"
                ),
            ));
        }

        category_index.insert(
            category_key,
            CategoryIndex {
                tier: category.tier,
                entry_ids,
            },
        );
    }

    // Resolve every prereq edge: dangling / cross-tree / self / tier order.
    // Graph adjacency: entry_node -> list of prereq entry_nodes.
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    // Span tokens keyed by the dependent entry node (for cycle reporting).
    let mut edge_spans: HashMap<(String, String), Option<usize>> = HashMap::new();

    for (node_id, node) in &entry_nodes {
        let mut prereq_nodes = Vec::new();
        for pre in &node.prereqs {
            let resolved = resolve_prereq_edge(
                &spec.tree_id,
                &node.entry_id,
                node.tier,
                pre,
                &category_index,
            )?;
            if resolved.prereq_node == *node_id {
                return Err(SpecError::SelfReferentialPrereq {
                    in_tree: spec.tree_id.clone(),
                    entry_id: node.entry_id.clone(),
                    source_span_token: pre.source_span_token,
                });
            }
            edge_spans.insert((node_id.clone(), resolved.prereq_node.clone()), pre.source_span_token);
            prereq_nodes.push(resolved.prereq_node);
        }
        adjacency.insert(node_id.clone(), prereq_nodes);
    }

    // Cycle detection (DFS with path stack).
    let mut color: HashMap<String, u8> = HashMap::new(); // 0 white, 1 gray, 2 black
    for node_id in entry_nodes.keys() {
        if color.get(node_id).copied().unwrap_or(0) == 0 {
            detect_cycle(
                &spec.tree_id,
                node_id,
                &adjacency,
                &edge_spans,
                &mut color,
                &mut Vec::new(),
            )?;
        }
    }

    Ok(diagnostics)
}

struct CategoryIndex {
    tier: u32,
    entry_ids: HashSet<String>,
}

struct EntryNode {
    entry_id: String,
    tier: u32,
    prereqs: Vec<CapabilityPrereqSpec>,
}

struct ResolvedPrereq {
    prereq_node: String,
}

fn resolve_prereq_edge(
    in_tree: &str,
    entry_id: &str,
    entry_tier: u32,
    pre: &CapabilityPrereqSpec,
    category_index: &HashMap<String, CategoryIndex>,
) -> Result<ResolvedPrereq, SpecError> {
    let Some((ns, name)) = split_ns_name(&pre.category) else {
        return Err(SpecError::UnknownPrereqCategory {
            in_tree: in_tree.to_owned(),
            entry_id: entry_id.to_owned(),
            category: pre.category.clone(),
            source_span_token: pre.source_span_token,
        });
    };
    let category_key = format!("{ns}::{name}");
    let Some(cat) = category_index.get(&category_key) else {
        // Category not in this tree ⇒ dangling / cross-tree reference.
        return Err(SpecError::UnknownPrereqCategory {
            in_tree: in_tree.to_owned(),
            entry_id: entry_id.to_owned(),
            category: pre.category.clone(),
            source_span_token: pre.source_span_token,
        });
    };
    if !cat.entry_ids.contains(&pre.entry_id) {
        return Err(SpecError::UnknownPrereqEntry {
            in_tree: in_tree.to_owned(),
            entry_id: entry_id.to_owned(),
            category: pre.category.clone(),
            prereq_entry_id: pre.entry_id.clone(),
            source_span_token: pre.source_span_token,
        });
    }
    // Tiered AND: a dependent entry may only require prereqs from equal-or-lower tiers.
    // Requiring a higher-tier entry is a tier-order / dup-tier conflict.
    if cat.tier > entry_tier {
        return Err(SpecError::PrereqTierOrderViolation {
            in_tree: in_tree.to_owned(),
            entry_id: entry_id.to_owned(),
            prereq_entry_id: pre.entry_id.clone(),
            entry_tier,
            prereq_tier: cat.tier,
            source_span_token: pre.source_span_token,
        });
    }
    Ok(ResolvedPrereq {
        prereq_node: format!("{category_key}::{}", pre.entry_id),
    })
}

fn split_ns_name(refstr: &str) -> Option<(&str, &str)> {
    let (ns, name) = refstr.split_once("::")?;
    if ns.is_empty() || name.is_empty() {
        return None;
    }
    Some((ns, name))
}

fn detect_cycle(
    in_tree: &str,
    node: &str,
    adjacency: &HashMap<String, Vec<String>>,
    edge_spans: &HashMap<(String, String), Option<usize>>,
    color: &mut HashMap<String, u8>,
    stack: &mut Vec<String>,
) -> Result<(), SpecError> {
    color.insert(node.to_owned(), 1);
    stack.push(node.to_owned());

    for next in adjacency.get(node).into_iter().flatten() {
        match color.get(next).copied().unwrap_or(0) {
            1 => {
                // Cycle found. Path from first occurrence of `next` through stack.
                let start = stack.iter().position(|n| n == next).unwrap_or(0);
                let mut path: Vec<&str> = stack[start..].iter().map(|s| s.as_str()).collect();
                path.push(next.as_str());
                let cycle_path = path.join(" -> ");
                let entry_id = next.rsplit("::").next().unwrap_or(next).to_owned();
                let source_span_token = edge_spans.get(&(node.to_owned(), next.clone())).copied().flatten();
                return Err(SpecError::PrereqCycle {
                    in_tree: in_tree.to_owned(),
                    entry_id,
                    cycle_path,
                    source_span_token,
                });
            }
            0 => detect_cycle(in_tree, next, adjacency, edge_spans, color, stack)?,
            _ => {}
        }
    }

    stack.pop();
    color.insert(node.to_owned(), 2);
    Ok(())
}

#[cfg(test)]
mod tests {
    //! Table-driven prereq-DAG admission referee (TEST-BUDGET form).
    use super::*;
    use crate::spec::capability::{
        CapabilityCategorySpec, CapabilityEffectSpec, CapabilityPrereqSpec, CapabilitySpec,
    };
    use simthing_core::{OverlayLifecycle, SubFieldRole, TransformOp};

    #[derive(Clone, Copy, Debug)]
    enum Expect {
        Ok,
        SelfPrereq { span: usize },
        DanglingEntry { span: usize },
        CrossTreeCategory { span: usize },
        Cycle,
        TierOrder { span: usize, entry_tier: u32, prereq_tier: u32 },
        MalformedMaxActive { span: usize, reason_sub: &'static str },
    }

    fn effect() -> CapabilityEffectSpec {
        CapabilityEffectSpec {
            targets_property: "military::fleet_speed".into(),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(1.1))],
            when_activated: OverlayLifecycle::Permanent,
            effect_target: crate::spec::capability::EffectTarget::CapabilityTree,
            source_span_token: None,
        }
    }

    fn entry(id: &str, prereqs: Vec<CapabilityPrereqSpec>) -> CapabilitySpec {
        CapabilitySpec {
            id: id.into(),
            display_name: id.into(),
            description: String::new(),
            flavor_text: String::new(),
            research_cost: 100.0,
            activation: ActivationMode::Threshold,
            icon: String::new(),
            thumbnail: String::new(),
            card_image: String::new(),
            unlock_video: None,
            model_preview: None,
            prereqs,
            unlocks_ship_components: vec![],
            unlocks_buildings: vec![],
            unlocks_units: vec![],
            unlocks_weapons: vec![],
            effects: vec![effect()],
        }
    }

    fn category(
        ns: &str,
        name: &str,
        tier: u32,
        max_active: Option<MaxActivePolicy>,
        entries: Vec<CapabilitySpec>,
        span: Option<usize>,
    ) -> CapabilityCategorySpec {
        CapabilityCategorySpec {
            property_namespace: ns.into(),
            property_name: name.into(),
            display_name: name.into(),
            tier,
            max_active,
            entries,
            source_span_token: span,
        }
    }

    fn tree(categories: Vec<CapabilityCategorySpec>) -> CapabilityTreeSpec {
        CapabilityTreeSpec {
            tree_id: "test".into(),
            tree_kind: "tech_tree".into(),
            owner_kind: "Faction".into(),
            install: crate::spec::InstallTargetSpec::faction_default(),
            categories,
        }
    }

    fn pre(category: &str, entry_id: &str, span: Option<usize>) -> CapabilityPrereqSpec {
        CapabilityPrereqSpec {
            category: category.into(),
            entry_id: entry_id.into(),
            source_span_token: span,
        }
    }

    fn build_case(label: &str) -> CapabilityTreeSpec {
        match label {
            "minimal_admits" => tree(vec![category(
                "tech",
                "propulsion",
                0,
                None,
                vec![entry("drive", vec![])],
                None,
            )]),
            "tiered_chain_admits" => tree(vec![
                category(
                    "tech",
                    "basic",
                    0,
                    None,
                    vec![entry("chem", vec![])],
                    None,
                ),
                category(
                    "tech",
                    "advanced",
                    1,
                    None,
                    vec![entry("ion", vec![pre("tech::basic", "chem", Some(88))])],
                    None,
                ),
            ]),
            "self_prereq" => tree(vec![category(
                "tech",
                "propulsion",
                0,
                None,
                vec![entry(
                    "drive",
                    vec![pre("tech::propulsion", "drive", Some(11))],
                )],
                None,
            )]),
            "dangling_entry" => tree(vec![category(
                "tech",
                "propulsion",
                0,
                None,
                vec![entry(
                    "drive",
                    vec![pre("tech::propulsion", "missing", Some(22))],
                )],
                None,
            )]),
            "cross_tree_category" => tree(vec![category(
                "tech",
                "propulsion",
                0,
                None,
                vec![entry(
                    "drive",
                    vec![pre("other::tree_cat", "foreign", Some(33))],
                )],
                None,
            )]),
            "cycle" => tree(vec![category(
                "tech",
                "propulsion",
                0,
                None,
                vec![
                    entry("a", vec![pre("tech::propulsion", "b", Some(44))]),
                    entry("b", vec![pre("tech::propulsion", "a", Some(45))]),
                ],
                None,
            )]),
            "tier_order" => tree(vec![
                category(
                    "tech",
                    "basic",
                    0,
                    None,
                    vec![entry(
                        "low",
                        vec![pre("tech::advanced", "high", Some(55))],
                    )],
                    None,
                ),
                category(
                    "tech",
                    "advanced",
                    2,
                    None,
                    vec![entry("high", vec![])],
                    None,
                ),
            ]),
            "max_active_zero" => tree(vec![category(
                "ideas",
                "tier1",
                0,
                Some(MaxActivePolicy::Limited {
                    count: 0,
                    replacement: ReplacementPolicy::SuspendOldest,
                }),
                vec![entry("idea_a", vec![])],
                Some(66),
            )]),
            "max_active_empty" => tree(vec![category(
                "ideas",
                "tier1",
                0,
                Some(MaxActivePolicy::Limited {
                    count: 1,
                    replacement: ReplacementPolicy::SuspendOldest,
                }),
                vec![],
                Some(77),
            )]),
            other => panic!("unknown case label {other}"),
        }
    }

    /// Table-driven prereq-DAG admission cases (admits + spanned negatives).
    const CASES: &[(&str, Expect)] = &[
        ("minimal_admits", Expect::Ok),
        ("tiered_chain_admits", Expect::Ok),
        ("self_prereq", Expect::SelfPrereq { span: 11 }),
        ("dangling_entry", Expect::DanglingEntry { span: 22 }),
        ("cross_tree_category", Expect::CrossTreeCategory { span: 33 }),
        ("cycle", Expect::Cycle),
        (
            "tier_order",
            Expect::TierOrder {
                span: 55,
                entry_tier: 0,
                prereq_tier: 2,
            },
        ),
        (
            "max_active_zero",
            Expect::MalformedMaxActive {
                span: 66,
                reason_sub: ">= 1",
            },
        ),
        (
            "max_active_empty",
            Expect::MalformedMaxActive {
                span: 77,
                reason_sub: "member",
            },
        ),
    ];

    #[test]
    fn prereq_dag_admission_table() {
        for (label, expect) in CASES {
            let spec = build_case(label);
            let result = validate_capability_tree(&spec);
            match (expect, result) {
                (Expect::Ok, Ok(_)) => {}
                (
                    Expect::SelfPrereq { span },
                    Err(SpecError::SelfReferentialPrereq {
                        entry_id,
                        source_span_token,
                        ..
                    }),
                ) => {
                    assert_eq!(entry_id, "drive", "{label}");
                    assert_eq!(source_span_token, Some(*span), "{label}");
                }
                (
                    Expect::DanglingEntry { span },
                    Err(SpecError::UnknownPrereqEntry {
                        prereq_entry_id,
                        source_span_token,
                        ..
                    }),
                ) => {
                    assert_eq!(prereq_entry_id, "missing", "{label}");
                    assert_eq!(source_span_token, Some(*span), "{label}");
                }
                (
                    Expect::CrossTreeCategory { span },
                    Err(SpecError::UnknownPrereqCategory {
                        category,
                        source_span_token,
                        ..
                    }),
                ) => {
                    assert_eq!(category, "other::tree_cat", "{label}");
                    assert_eq!(source_span_token, Some(*span), "{label}");
                }
                (
                    Expect::Cycle,
                    Err(SpecError::PrereqCycle {
                        cycle_path,
                        source_span_token,
                        ..
                    }),
                ) => {
                    assert!(
                        cycle_path.contains('a') && cycle_path.contains('b'),
                        "{label}: {cycle_path}"
                    );
                    assert!(source_span_token.is_some(), "{label}");
                }
                (
                    Expect::TierOrder {
                        span,
                        entry_tier,
                        prereq_tier,
                    },
                    Err(SpecError::PrereqTierOrderViolation {
                        entry_tier: got_entry,
                        prereq_tier: got_prereq,
                        source_span_token,
                        ..
                    }),
                ) => {
                    assert_eq!(got_entry, *entry_tier, "{label}");
                    assert_eq!(got_prereq, *prereq_tier, "{label}");
                    assert_eq!(source_span_token, Some(*span), "{label}");
                }
                (
                    Expect::MalformedMaxActive { span, reason_sub },
                    Err(SpecError::MalformedMaxActive {
                        reason,
                        source_span_token,
                        ..
                    }),
                ) => {
                    assert!(reason.contains(reason_sub), "{label}: {reason}");
                    assert_eq!(source_span_token, Some(*span), "{label}");
                }
                (expect, other) => panic!("{label}: unexpected result {other:?} for {expect:?}"),
            }
        }
    }
}
