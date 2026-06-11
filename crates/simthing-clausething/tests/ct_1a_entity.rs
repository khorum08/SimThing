//! CT-1a literal entity hydration parity: ClauseScript ≡ hand-authored RON baseline.

use std::collections::HashMap;

use simthing_clausething::{
    admit_and_apply_domain_pack, admit_and_apply_pack, hydrate_entity_pack, parse_raw_document,
};
use simthing_core::{
    DimensionRegistry, Overlay, OverlayKind, OverlayLifecycle, OverlaySource, SimThing,
    SimThingKind, SubFieldRole, TransformOp,
};
use simthing_driver::preview_install;
use simthing_gpu::SlotAllocator;
use simthing_spec::spec::domain_pack::DomainPackSpec;
use simthing_spec::{GameModeSpec, SpecVersion};

const CLAUSE_FIXTURE: &str = include_str!("fixtures/ct1a_demo_entity.clause");
const RON_BASELINE: &str = include_str!("fixtures/ct1a_demo_entity_baseline.ron");
const SEED_AMOUNT: f32 = 40.0;

fn load_ron_baseline() -> DomainPackSpec {
    ron::from_str(RON_BASELINE).expect("parse RON baseline")
}

fn canonical_json(pack: &DomainPackSpec) -> String {
    serde_json::to_string(pack).expect("serialize domain pack")
}

fn hydrate_from_clause() -> simthing_clausething::HydratedEntityPack {
    let document = parse_raw_document(CLAUSE_FIXTURE.as_bytes()).expect("parse clause fixture");
    hydrate_entity_pack(&document).expect("hydrate clause fixture")
}

fn ct1a_scenario() -> simthing_driver::Scenario {
    let root = SimThing::new(SimThingKind::World, 0);
    let registry = DimensionRegistry::new();
    simthing_driver::Scenario {
        name: "ct1a_install".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 0.0,
        n_slots: 16,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    }
}

fn game_mode_with_pack(pack: DomainPackSpec) -> GameModeSpec {
    GameModeSpec {
        id: "ct1a_install".into(),
        display_name: "CT-1a Install Proof".into(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: vec![pack],
        properties: Vec::new(),
        overlays: Vec::new(),
        capability_trees: Vec::new(),
        events: Vec::new(),
        resource_flow: None,
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: vec![],
        mapping_execution_profile: Default::default(),
    }
}

fn preview_installed_tree(pack: &DomainPackSpec) -> simthing_driver::InstallPreview {
    let scenario = ct1a_scenario();
    let game_mode = game_mode_with_pack(pack.clone());
    let allocator = SlotAllocator::new();
    preview_install(
        &game_mode,
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect("preview_install domain pack")
}

#[derive(Debug, PartialEq, Eq)]
struct OverlayFingerprint {
    kind: String,
    source: String,
    lifecycle: String,
    property_key: String,
    sub_field_deltas: Vec<(String, String)>,
    affects_paths: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct NodeFingerprint {
    path: String,
    kind: String,
    property_keys: Vec<String>,
    overlays: Vec<OverlayFingerprint>,
}

#[derive(Debug, PartialEq, Eq)]
struct InstalledTreeFingerprint {
    registry_property_keys: Vec<String>,
    nodes: Vec<NodeFingerprint>,
}

fn node_path(
    root_id: simthing_core::SimThingId,
    node: &SimThing,
    target: simthing_core::SimThingId,
) -> Option<String> {
    fn walk(
        root_id: simthing_core::SimThingId,
        node: &SimThing,
        target: simthing_core::SimThingId,
        path: &str,
    ) -> Option<String> {
        if node.id == target {
            return Some(path.to_string());
        }
        for (idx, child) in node.children.iter().enumerate() {
            if let Some(found) = walk(root_id, child, target, &format!("{path}/child[{idx}]")) {
                return Some(found);
            }
        }
        None
    }
    walk(root_id, node, target, "root")
}

fn format_role(role: &SubFieldRole) -> String {
    match role {
        SubFieldRole::Amount => "Amount".into(),
        SubFieldRole::Velocity => "Velocity".into(),
        SubFieldRole::Intensity => "Intensity".into(),
        SubFieldRole::Named(name) => format!("Named({name})"),
        SubFieldRole::Custom(name) => format!("Custom({name})"),
    }
}

fn format_op(op: &TransformOp) -> String {
    match op {
        TransformOp::Add(v) => format!("Add({v})"),
        TransformOp::Multiply(v) => format!("Multiply({v})"),
        TransformOp::Set(v) => format!("Set({v})"),
    }
}

fn overlay_fingerprint(
    registry: &DimensionRegistry,
    root: &SimThing,
    overlay: &Overlay,
) -> OverlayFingerprint {
    let prop = registry.property(overlay.transform.property_id);
    let property_key = format!("{}::{}", prop.namespace, prop.name);
    let mut sub_field_deltas = overlay
        .transform
        .sub_field_deltas
        .iter()
        .map(|(role, op)| (format_role(role), format_op(op)))
        .collect::<Vec<_>>();
    sub_field_deltas.sort();
    let mut affects_paths = overlay
        .affects
        .iter()
        .filter_map(|id| node_path(root.id, root, *id))
        .collect::<Vec<_>>();
    affects_paths.sort();
    OverlayFingerprint {
        kind: format!("{:?}", overlay.kind),
        source: format!("{:?}", overlay.source),
        lifecycle: format!("{:?}", overlay.lifecycle),
        property_key,
        sub_field_deltas,
        affects_paths,
    }
}

fn collect_nodes(
    registry: &DimensionRegistry,
    root: &SimThing,
    node: &SimThing,
    path: &str,
    out: &mut Vec<NodeFingerprint>,
) {
    let mut property_keys = node
        .properties
        .keys()
        .filter_map(|id| {
            let prop = registry.property(*id);
            Some(format!("{}::{}", prop.namespace, prop.name))
        })
        .collect::<Vec<_>>();
    property_keys.sort();

    let mut overlays = node
        .overlays
        .iter()
        .map(|overlay| overlay_fingerprint(registry, root, overlay))
        .collect::<Vec<_>>();
    overlays.sort_by(|a, b| {
        (
            &a.property_key,
            &a.kind,
            &a.sub_field_deltas,
            &a.affects_paths,
        )
            .cmp(&(
                &b.property_key,
                &b.kind,
                &b.sub_field_deltas,
                &b.affects_paths,
            ))
    });

    out.push(NodeFingerprint {
        path: path.to_string(),
        kind: format!("{:?}", node.kind),
        property_keys,
        overlays,
    });

    for (idx, child) in node.children.iter().enumerate() {
        collect_nodes(registry, root, child, &format!("{path}/child[{idx}]"), out);
    }
}

fn installed_tree_fingerprint(
    registry: &DimensionRegistry,
    root: &SimThing,
) -> InstalledTreeFingerprint {
    let mut registry_property_keys = registry
        .properties
        .iter()
        .map(|prop| format!("{}::{}", prop.namespace, prop.name))
        .collect::<Vec<_>>();
    registry_property_keys.sort();

    let mut nodes = Vec::new();
    collect_nodes(registry, root, root, "root", &mut nodes);
    InstalledTreeFingerprint {
        registry_property_keys,
        nodes,
    }
}

#[test]
fn hydrated_domain_pack_matches_ron_baseline() {
    let hydrated = hydrate_from_clause();
    let baseline = load_ron_baseline();
    assert_eq!(
        canonical_json(&hydrated.domain_pack),
        canonical_json(&baseline),
        "hydrated authoring struct must match RON baseline"
    );
    assert_eq!(hydrated.seed_amount, SEED_AMOUNT);
}

#[test]
fn clause_and_ron_cpu_overlay_parity_match() {
    let hydrated = hydrate_from_clause();
    let baseline = load_ron_baseline();

    let from_clause = admit_and_apply_pack(&hydrated).expect("admit hydrated pack");
    let from_ron = admit_and_apply_domain_pack(&baseline, SEED_AMOUNT).expect("admit RON baseline");

    assert_eq!(
        from_clause, from_ron,
        "CPU overlay/property parity must match between ClauseScript and RON paths"
    );
    assert_eq!(from_clause.seeded_amount, SEED_AMOUNT);
    assert_eq!(from_clause.final_amount, 50.0);
    assert_eq!(
        from_clause.property_keys,
        vec!["simthing::potency".to_string()]
    );
}

#[test]
fn clause_and_ron_installed_trees_match_via_preview_install() {
    let hydrated = hydrate_from_clause();
    let baseline = load_ron_baseline();

    let from_clause = preview_installed_tree(&hydrated.domain_pack);
    let from_ron = preview_installed_tree(&baseline);

    let clause_fp = installed_tree_fingerprint(&from_clause.registry, &from_clause.root);
    let ron_fp = installed_tree_fingerprint(&from_ron.registry, &from_ron.root);

    assert_eq!(
        clause_fp, ron_fp,
        "installed SimThing tree must be canonically identical"
    );

    let root_node = clause_fp
        .nodes
        .iter()
        .find(|node| node.path == "root")
        .expect("root node fingerprint");
    assert_eq!(
        root_node.property_keys,
        vec!["simthing::potency".to_string()]
    );
    assert_eq!(root_node.overlays.len(), 1);
    assert_eq!(root_node.overlays[0].property_key, "simthing::potency");
    assert_eq!(
        root_node.overlays[0].sub_field_deltas,
        vec![("Amount".to_string(), "Multiply(1.25)".to_string())]
    );
    assert_eq!(
        root_node.overlays[0].affects_paths,
        vec!["root".to_string()]
    );
    assert_eq!(
        root_node.overlays[0].kind,
        format!("{:?}", OverlayKind::Policy)
    );
    assert_eq!(
        root_node.overlays[0].source,
        format!("{:?}", OverlaySource::Player)
    );
    assert_eq!(
        root_node.overlays[0].lifecycle,
        format!("{:?}", OverlayLifecycle::Permanent)
    );
}

#[test]
fn unsupported_entity_field_is_hard_error() {
    let text = include_str!("fixtures/ct1a_unsupported_field.clause");
    let document = parse_raw_document(text.as_bytes()).expect("parse unsupported fixture");
    let err = hydrate_entity_pack(&document).expect_err("unsupported field must fail");
    assert!(
        err.message.contains("triggered_modifier"),
        "expected unsupported field diagnostic, got: {}",
        err.message
    );
    assert!(err.span.is_some(), "expected spanned diagnostic");
}
