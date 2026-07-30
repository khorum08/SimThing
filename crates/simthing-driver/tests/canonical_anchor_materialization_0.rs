//! CANONICAL-ANCHOR-MATERIALIZATION-0 — totality exit proofs + derivation-set census.
//!
//! Remand `5124550917` / DA `5124532506` / HD `dfacf5e8bb04`:
//! RESIDENCY = value-placing relations ONLY (typed economy/RF hosts; no id/name substring classing).
//! Governance may corroborate but never elects. Prove TOTALITY on the ordinary
//! unmutated install (overlays/domain packs ENABLED), not 1:1 cardinality.
//! Census locks `zero=0 / conflict=0` over the derivation set only
//! (Anchored properties with zero live loci after admitted structures resolve).

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

use simthing_clausething::{hydrate_scenario_with_source_base, parse_raw_document};
use simthing_core::{
    mint_anchor_table_from_admission, DimensionRegistry, PropertyAdmissionDisposition,
    SimPropertyId, SimThing, SubFieldRole,
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
    // Derived corpus counts (published, not targets).
    eprintln!(
        "HYDRATE DISPOSITION CENSUS: Anchored={} Unobserved={} total={}",
        anchored.len(),
        unobserved.len(),
        pack.game_mode.properties.len()
    );
    assert!(
        !anchored.is_empty(),
        "canonical TP must still author Anchored resource properties"
    );
    assert!(
        !unobserved.is_empty(),
        "canonical TP must still author Unobserved dark cells where hostless"
    );

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
    // Ordinary production field-bearing door: domain packs + overlays ENABLED.
    let profile = authored_live_profile_from_pack(&pack);
    let scenario = driver_scenario_field_bearing_from_profile(&profile)
        .expect("field-bearing scenario from canonical TP pack");
    let game_mode = field_bearing_game_mode(&profile.game_mode);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);
    preview_install(
        &game_mode,
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .unwrap_or_else(|err| panic!("canonical TP ordinary preview_install: {err:?}"))
}

/// Count live PropertyValue stores per SimPropertyId (tree presence).
/// Distinct from `snapshot_anchored_loci`, which only admits Amount/Velocity
/// primary columns into the 5.3 observation table.
fn count_tree_property_loci(node: &SimThing, counts: &mut HashMap<SimPropertyId, usize>) {
    for &pid in node.properties.keys() {
        *counts.entry(pid).or_default() += 1;
    }
    for child in &node.children {
        count_tree_property_loci(child, counts);
    }
}

/// Corrected post-ruling census: after ordinary install, residual derivation set
/// (Anchored with zero live loci) must be empty — zero=0 / conflict=0.
fn assert_derivation_set_census_zero_conflict_free() {
    let (exact_all, zero_all, conflict_all, hydrate_md) = census_markdown();
    eprintln!("\n=== HYDRATE VALUE-PLACING TABLE (diagnostic) ===\n{hydrate_md}");
    assert_eq!(zero_all, 0, "hydrate Anchored must not have zero-candidate rows");
    assert_eq!(
        conflict_all, 0,
        "hydrate Anchored must not conflict under value-placing residency \
         (governance overlays are not electors); exact_all={exact_all}"
    );

    let preview = full_tp_preview();
    let report = preview.registry.property_admission_report();
    let mut live_counts = HashMap::new();
    count_tree_property_loci(&preview.root, &mut live_counts);
    let mut derivation_set = Vec::new();
    for row in &report.resource_properties {
        if !row.disposition.is_anchored() {
            continue;
        }
        if live_counts.get(&row.property_id).copied().unwrap_or(0) == 0 {
            derivation_set.push(row.canonical_identity());
        }
    }
    eprintln!(
        "=== DERIVATION SET (|{}|) ===\n{}\n**Summary:** exact=0 / zero=0 / conflict=0 / |set|={}\n",
        derivation_set.len(),
        if derivation_set.is_empty() {
            "(empty — all Anchored already have ≥1 live PropertyValue after admitted structures)"
                .to_string()
        } else {
            derivation_set
                .iter()
                .map(|s| format!("- `{s}`"))
                .collect::<Vec<_>>()
                .join("\n")
        },
        derivation_set.len()
    );
    assert!(
        derivation_set.is_empty(),
        "derivation set must be empty after ordinary install (totality); residual={derivation_set:?}"
    );
}

/// Protected inventory identity (birth track 0.0.8.7); wrapper over corrected census helper.
#[test]
fn candidate_evidence_census_before_implementation() {
    assert_derivation_set_census_zero_conflict_free();
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
fn ordinary_install_proves_admission_totality() {
    // Binding inventory door — publish derived counts (not asserted targets).
    let inventory = properties_only_inventory_preview()
        .registry
        .property_admission_report();
    eprintln!(
        "INVENTORY (derived): Anchored={} Unobserved={} total={}",
        inventory.anchored_count(),
        inventory.unobserved_count(),
        inventory.resource_properties.len()
    );
    assert_eq!(
        inventory.resource_properties.len(),
        inventory.anchored_count() + inventory.unobserved_count(),
        "closed disposition partition"
    );

    // Ordinary unmutated production install (overlays + domain packs enabled).
    let preview = full_tp_preview();
    let report = preview.registry.property_admission_report();
    let tp_anchored: HashSet<SimPropertyId> = report
        .resource_properties
        .iter()
        .filter(|row| row.disposition.is_anchored() && row.namespace == "tp_economy")
        .map(|row| row.property_id)
        .collect();
    let dark: Vec<_> = report
        .dark_properties()
        .map(|row| row.canonical_identity())
        .collect();
    eprintln!(
        "ORDINARY INSTALL (derived): Anchored={} Unobserved={} tp_economy_anchored={} dark={dark:?}",
        report.anchored_count(),
        report.unobserved_count(),
        tp_anchored.len()
    );

    // TOTALITY: every Anchored property has ≥1 live PropertyValue on the tree.
    // (RF Named-role properties such as studio_live_rf::owner_flow are live stores
    // but intentionally absent from the Amount/Velocity observation snapshot.)
    let mut live_counts = HashMap::new();
    count_tree_property_loci(&preview.root, &mut live_counts);
    let uncovered: Vec<_> = report
        .resource_properties
        .iter()
        .filter(|row| row.disposition.is_anchored())
        .filter(|row| live_counts.get(&row.property_id).copied().unwrap_or(0) == 0)
        .map(|row| row.canonical_identity())
        .collect();
    assert!(
        uncovered.is_empty(),
        "totality requires every Anchored property to have ≥1 live PropertyValue; uncovered={uncovered:?}"
    );

    // Observation-table locus map (Amount/Velocity primary); multi-host lawful.
    let loci = snapshot_anchored_loci(&preview.root, &preview.registry, &preview.allocator);
    let key_count = loci.len();
    let unique_keys = loci.keys().collect::<HashSet<_>>().len();
    assert_eq!(key_count, unique_keys, "no repeated (SimThingId, SimPropertyId)");

    let mut hosts_per_prop: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
    for ((sid, pid), _) in &loci {
        if !tp_anchored.contains(pid) {
            continue;
        }
        let p = preview.registry.property(*pid);
        hosts_per_prop
            .entry(format!("{}::{}", p.namespace, p.name))
            .or_default()
            .insert(sid.raw());
    }
    let multi = hosts_per_prop
        .iter()
        .filter(|(_, h)| h.len() > 1)
        .count();
    eprintln!(
        "TOTALITY: tree_anchored_live={} observation_locus_rows={} \
         tp_economy_props={} multi_host_props={} (lawful)",
        report
            .resource_properties
            .iter()
            .filter(|row| row.disposition.is_anchored())
            .filter(|row| live_counts.get(&row.property_id).copied().unwrap_or(0) > 0)
            .count(),
        loci.iter()
            .filter(|((_, pid), _)| tp_anchored.contains(pid))
            .count(),
        hosts_per_prop.len(),
        multi
    );
    assert_eq!(
        hosts_per_prop.len(),
        tp_anchored.len(),
        "every Anchored tp_economy identity must appear in the observation map"
    );

    // One GPU table identity coverage per observation-table live locus.
    let n_dims = preview.registry.total_columns.max(1);
    let zeros = vec![0.0_f32; n_dims * preview.allocator.capacity().max(1)];
    let table = mint_anchor_table_from_admission(
        &preview.root,
        &preview.registry,
        &loci,
        &zeros,
        n_dims,
    );
    let mut covered = HashSet::new();
    for row in table.rows() {
        covered.insert((row.identity.sim_thing_id, row.identity.property_id));
    }
    for key in loci.keys() {
        assert!(
            covered.contains(key),
            "GPU table must cover live locus {key:?}"
        );
    }

    for row in report.dark_properties() {
        assert!(
            !loci.keys().any(|(_, p)| *p == row.property_id),
            "Unobserved {} must remain a dark cell (absent from observation locus map)",
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
    eprintln!("Unobserved disposition reasons naming Phase 8 (derived count): {reasons}");
    assert!(
        reasons > 0,
        "canonical TP must enumerate authored Unobserved{{reason}} dark cells"
    );
}
