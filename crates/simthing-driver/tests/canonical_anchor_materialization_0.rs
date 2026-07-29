//! CANONICAL-ANCHOR-MATERIALIZATION-0 — value-placing residency census + exit proofs.
//!
//! DA HOLD lift `5124095512` / dispatch `5124117080` / HD `63f01c28e4df`:
//! RESIDENCY = value-placing relations ONLY. Governance (owner-policy overlays,
//! policy-weight authority) may corroborate but never elects. After the seven
//! DA-authorized Unobserved{reason} conversions, Anchored census must lock
//! `exact=18 / zero=0 / conflict=0`.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

use simthing_clausething::{hydrate_scenario_with_source_base, parse_raw_document};
use simthing_core::{
    DimensionRegistry, PropertyAdmissionDisposition, SimPropertyId, SimThing, SubFieldRole,
};
use simthing_driver::{preview_install, Scenario};
use simthing_gpu::SlotAllocator;
use simthing_sim::snapshot_anchored_loci;
use simthing_spec::PropertyKey;

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
    if !out.contains_key(property) {
        return;
    }
    out.get_mut(property).unwrap().insert(Candidate {
        host_entity: host.to_string(),
        evidence_class,
        span: span.into(),
    });
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

/// Census over Anchored properties only (Unobserved dark cells are excluded).
fn census_markdown() -> (usize, usize, usize, String) {
    let clause = repo_root().join("scenarios/terran_pirate_galaxy.clause");
    let source = std::fs::read_to_string(&clause).expect("read canonical clause");
    let document = parse_raw_document(source.as_bytes()).expect("parse");
    let pack = hydrate_scenario_with_source_base(&document, Some(clause.parent().unwrap()))
        .expect("hydrate TP");

    assert_eq!(
        pack.game_mode.properties.len(),
        25,
        "canonical TP must still author exactly 25 resource properties"
    );
    let anchored: Vec<String> = pack
        .game_mode
        .properties
        .iter()
        .filter(|p| p.admission_disposition.is_anchored())
        .map(|p| format!("{}::{}", p.namespace, p.name))
        .collect();
    let unobserved: Vec<String> = pack
        .game_mode
        .properties
        .iter()
        .filter(|p| !p.admission_disposition.is_anchored())
        .map(|p| format!("{}::{}", p.namespace, p.name))
        .collect();
    assert_eq!(anchored.len(), 18, "expected 18 Anchored after DA Unobserved edits");
    assert_eq!(unobserved.len(), 7, "expected 7 Unobserved dark cells");

    let mut by_prop: BTreeMap<String, BTreeSet<Candidate>> = BTreeMap::new();
    for p in &anchored {
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

    // Governance overlays intentionally omitted as electors (DA residency law).

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

    if let Some(auth) = &pack.authority_root {
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
    for (idx, property) in anchored.iter().enumerate() {
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
        "\nUnobserved dark cells (excluded from residency census): `{}`\n",
        unobserved.join("`, `")
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
        "\n**Summary (Anchored only):** exact={exact} / zero={zero} / conflict={conflict} / total={}\n",
        anchored.len()
    ));

    (exact, zero, conflict, md)
}

fn full_tp_preview() -> simthing_driver::InstallPreview {
    use simthing_mapeditor::{
        authored_live_profile_from_pack, driver_scenario_field_bearing_from_profile,
        field_bearing_game_mode,
    };

    let clause = repo_root().join("scenarios/terran_pirate_galaxy.clause");
    let source = std::fs::read_to_string(&clause).expect("read canonical clause");
    let document = parse_raw_document(source.as_bytes()).expect("parse");
    let pack = hydrate_scenario_with_source_base(&document, Some(clause.parent().unwrap()))
        .expect("hydrate TP");
    // Production field-bearing door: strips hydrate-local property ids, keeps
    // authority topology + install_targets, reinstalls via compile_and_install.
    let profile = authored_live_profile_from_pack(&pack);
    let scenario = driver_scenario_field_bearing_from_profile(&profile)
        .expect("field-bearing scenario from canonical TP pack");
    // Governance overlays corroborate but must not elect/place observation hosts
    // (DA residency law). Drop the field-bearing overlay pack so locus cardinality
    // reflects value-placing relations only.
    let mut game_mode = field_bearing_game_mode(&profile.game_mode);
    game_mode
        .domain_packs
        .retain(|pack| pack.id != "field_bearing_overlays");
    game_mode.overlays.clear();
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);
    preview_install(
        &game_mode,
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .unwrap_or_else(|err| panic!("canonical TP field-bearing preview_install: {err:?}"))
}

#[test]
fn candidate_evidence_census_value_placing_residency() {
    let (exact, zero, conflict, md) = census_markdown();
    eprintln!("\n=== CANONICAL-ANCHOR-MATERIALIZATION-0 CENSUS ===\n{md}");
    assert_eq!(exact, 18, "exactly-one Anchored rows drifted");
    assert_eq!(zero, 0, "zero-candidate Anchored rows remain");
    assert_eq!(conflict, 0, "conflict Anchored rows remain");
}

fn properties_only_inventory_preview() -> simthing_driver::InstallPreview {
    let clause = repo_root().join("scenarios/terran_pirate_galaxy.clause");
    let source = std::fs::read_to_string(&clause).expect("read canonical clause");
    let document = parse_raw_document(source.as_bytes()).expect("parse");
    let pack = hydrate_scenario_with_source_base(&document, Some(clause.parent().unwrap()))
        .expect("hydrate TP");
    let game_mode = simthing_spec::GameModeSpec {
        id: pack.game_mode.id.clone(),
        display_name: pack.game_mode.display_name.clone(),
        properties: pack.game_mode.properties.clone(),
        ..Default::default()
    };
    let root = pack.root.clone();
    let scenario = Scenario {
        name: pack.scenario_id.clone(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: (root.subtree_size() as u32).saturating_add(2048),
        registry: DimensionRegistry::new(),
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    };
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);
    preview_install(
        &game_mode,
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect("properties-only inventory install")
}

#[test]
fn canonical_tp_materializes_18_anchored_7_unobserved() {
    // Binding inventory door (same as property_admission_inventory.tsv): 18/7.
    let inventory = properties_only_inventory_preview()
        .registry
        .property_admission_report();
    assert_eq!(inventory.anchored_count(), 18, "inventory Anchored");
    assert_eq!(inventory.unobserved_count(), 7, "inventory Unobserved");

    // Live materialization door (Studio field-bearing / compile_and_install).
    let preview = full_tp_preview();
    let report = preview.registry.property_admission_report();
    let tp_anchored: HashSet<SimPropertyId> = report
        .resource_properties
        .iter()
        .filter(|row| row.disposition.is_anchored() && row.namespace == "tp_economy")
        .map(|row| row.property_id)
        .collect();
    assert_eq!(
        tp_anchored.len(),
        18,
        "canonical TP economy corpus must expose 18 Anchored identities"
    );
    assert_eq!(
        report.dark_properties().count(),
        7,
        "7 Unobserved dark cells retained under field-bearing install"
    );

    let loci = snapshot_anchored_loci(&preview.root, &preview.registry, &preview.allocator);
    let tp_loci: Vec<_> = loci
        .iter()
        .filter(|((_, pid), _)| tp_anchored.contains(pid))
        .collect();
    let live_prop_count = {
        let mut props = HashSet::new();
        for ((_, pid), _) in &tp_loci {
            props.insert(pid.0);
        }
        props.len()
    };
    assert_eq!(
        live_prop_count, 18,
        "18 distinct Anchored TP economy identities with live loci (locus rows={})",
        tp_loci.len()
    );
    let mut by_prop: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    for ((sid, pid), _) in &tp_loci {
        let p = preview.registry.property(*pid);
        by_prop
            .entry(format!("{}::{}", p.namespace, p.name))
            .or_default()
            .push(sid.raw());
    }
    let dupes: Vec<_> = by_prop
        .iter()
        .filter(|(_, hosts)| hosts.len() != 1)
        .map(|(name, hosts)| format!("{name} hosts={hosts:?}"))
        .collect();
    assert!(
        dupes.is_empty(),
        "each Anchored TP economy property must have exactly one live locus; dupes: {}",
        dupes.join("; ")
    );
    assert_eq!(tp_loci.len(), 18, "18 live Anchored loci over TP economy props");

    for row in report.dark_properties() {
        assert!(
            !loci.keys().any(|(_, p)| *p == row.property_id),
            "Unobserved {} must remain a dark cell (no anchored locus)",
            row.canonical_identity()
        );
    }
}

#[test]
fn materialization_preserves_existing_economy_values() {
    let preview = full_tp_preview();
    let pid = preview
        .registry
        .id_of("tp_economy", "terran_shipyard_minerals_quantity")
        .expect("missing terran_shipyard_minerals_quantity");
    let layout = preview.registry.property(pid).layout.clone();
    let mut found = None;
    fn walk(
        node: &SimThing,
        pid: SimPropertyId,
        layout: &simthing_core::PropertyLayout,
        found: &mut Option<f32>,
    ) {
        if let Some(v) = node.properties.get(&pid) {
            *found = Some(v.get_role(&SubFieldRole::Amount, layout));
        }
        for child in &node.children {
            walk(child, pid, layout, found);
        }
    }
    walk(&preview.root, pid, &layout, &mut found);
    let amount = found.expect("economy property must already be live on a host");
    assert!(
        amount > 0.0,
        "materialization must not wipe economy seed (got {amount})"
    );
}

#[test]
fn unobserved_disposition_reasons_name_phase_8() {
    let clause = repo_root().join("scenarios/terran_pirate_galaxy.clause");
    let source = std::fs::read_to_string(&clause).expect("read");
    let document = parse_raw_document(source.as_bytes()).expect("parse");
    let pack = hydrate_scenario_with_source_base(&document, Some(clause.parent().unwrap()))
        .expect("hydrate");
    let mut reasons = 0usize;
    for prop in &pack.game_mode.properties {
        if let PropertyAdmissionDisposition::Unobserved { reason, .. } =
            &prop.admission_disposition
        {
            reasons += 1;
            assert!(
                reason.contains("Phase 8") || reason.contains("8.1"),
                "{} reason must name Phase 8 successor: {reason}",
                prop.name
            );
            assert!(
                reason.contains("uninstantiated"),
                "{} reason must name uninstantiated host class: {reason}",
                prop.name
            );
        }
    }
    assert_eq!(reasons, 7);
}
