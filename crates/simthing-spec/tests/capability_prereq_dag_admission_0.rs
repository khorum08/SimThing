//! CAPABILITY-PREREQ-DAG-ADMISSION-0 — RON authored-corpus census + builder gate.
//!
//! ClauseThing tradition trees and live GPU atomicity live in
//! `simthing-clausething/tests/capability_prereq_dag_admission_0.rs` (crate
//! dependency direction forbids clausething from being a simthing-spec
//! dev-dep).

use simthing_core::DimensionRegistry;
use simthing_spec::{
    validate_capability_tree, CapabilityTreeBuilder, CapabilityTreeSpec, GameModeSpec, SpecError,
};

const MINIMAL_TECH: &str = include_str!("fixtures/minimal_tech_tree.ron");
const MINIMAL_GAME_MODE: &str = include_str!("fixtures/minimal_game_mode.ron");
const EXAMPLE_ALL_FACTIONS: &str =
    include_str!("../../../docs/examples/game_mode_install_all_factions.ron");
const EXAMPLE_SCENARIO_LISTED: &str =
    include_str!("../../../docs/examples/game_mode_install_scenario_listed.ron");
const EXAMPLE_SESSION_ROOT: &str =
    include_str!("../../../docs/examples/game_mode_install_session_root.ron");

/// RON authored-corpus labels checked by this crate's census.
const RON_CORPUS_LABELS: &[&str] = &[
    "minimal_tech_tree.ron",
    "minimal_game_mode.ron",
    "docs/examples/game_mode_install_all_factions.ron",
    "docs/examples/game_mode_install_scenario_listed.ron",
    "docs/examples/game_mode_install_session_root.ron",
];

fn strip_ron_comments(src: &str) -> String {
    src.lines()
        .skip_while(|l| l.trim_start().starts_with("//") || l.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn trees_from_game_mode(mode: &GameModeSpec) -> Vec<&CapabilityTreeSpec> {
    let mut out = Vec::new();
    for tree in &mode.capability_trees {
        out.push(tree);
    }
    for pack in &mode.domain_packs {
        for tree in &pack.capability_trees {
            out.push(tree);
        }
    }
    out
}

fn seed_mode_properties(mode: &GameModeSpec, registry: &mut DimensionRegistry) {
    for prop in &mode.properties {
        let _ = simthing_spec::compile_property(prop, registry);
    }
    for pack in &mode.domain_packs {
        for prop in &pack.properties {
            let _ = simthing_spec::compile_property(prop, registry);
        }
    }
}

#[test]
fn existing_authored_ron_capability_trees_admit_unchanged() {
    let mut admitted_trees = 0usize;
    let mut sources_checked = 0usize;

    {
        let tree: CapabilityTreeSpec =
            ron::from_str(&strip_ron_comments(MINIMAL_TECH)).expect("minimal_tech_tree parse");
        validate_capability_tree(&tree)
            .unwrap_or_else(|e| panic!("census FAIL minimal_tech_tree.ron: {e}"));
        admitted_trees += 1;
        sources_checked += 1;
    }

    for (label, src) in [
        ("minimal_game_mode.ron", MINIMAL_GAME_MODE),
        (
            "docs/examples/game_mode_install_all_factions.ron",
            EXAMPLE_ALL_FACTIONS,
        ),
        (
            "docs/examples/game_mode_install_scenario_listed.ron",
            EXAMPLE_SCENARIO_LISTED,
        ),
        (
            "docs/examples/game_mode_install_session_root.ron",
            EXAMPLE_SESSION_ROOT,
        ),
    ] {
        let mode: GameModeSpec =
            ron::from_str(&strip_ron_comments(src)).unwrap_or_else(|e| panic!("{label}: {e}"));
        let trees = trees_from_game_mode(&mode);
        assert!(
            !trees.is_empty(),
            "{label} must contain at least one capability tree"
        );
        for tree in trees {
            validate_capability_tree(tree)
                .unwrap_or_else(|e| panic!("census FAIL {label} tree {}: {e}", tree.tree_id));
            let mut registry = DimensionRegistry::new();
            seed_mode_properties(&mode, &mut registry);
            CapabilityTreeBuilder::build(tree, &mut registry).unwrap_or_else(|e| {
                panic!("builder FAIL {label} tree {}: {e}", tree.tree_id)
            });
            admitted_trees += 1;
        }
        sources_checked += 1;
    }

    assert_eq!(sources_checked, RON_CORPUS_LABELS.len());
    assert!(admitted_trees >= 4, "got {admitted_trees}");
    eprintln!(
        "CAPABILITY-PREREQ-DAG-RON-CENSUS sources={} trees_admitted={} labels={:?}",
        sources_checked, admitted_trees, RON_CORPUS_LABELS
    );
}

#[test]
fn builder_rejects_prereq_cycle_at_admission() {
    use simthing_core::{OverlayLifecycle, SubFieldRole, TransformOp};
    use simthing_spec::{
        ActivationMode, CapabilityCategorySpec, CapabilityEffectSpec, CapabilityPrereqSpec,
        CapabilitySpec, EffectTarget,
    };

    let effect = CapabilityEffectSpec {
        targets_property: "military::fleet_speed".into(),
        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(1.1))],
        when_activated: OverlayLifecycle::UntilDissolved,
        effect_target: EffectTarget::CapabilityTree,
        source_span_token: None,
    };
    let tree = CapabilityTreeSpec {
        tree_id: "cycle".into(),
        tree_kind: "tech_tree".into(),
        owner_kind: "Owner".into(),
        install: simthing_spec::InstallTargetSpec::faction_default(),
        categories: vec![CapabilityCategorySpec {
            property_namespace: "tech".into(),
            property_name: "prop".into(),
            display_name: "Prop".into(),
            tier: 0,
            max_active: None,
            source_span_token: None,
            entries: vec![
                CapabilitySpec {
                    id: "a".into(),
                    display_name: "A".into(),
                    description: String::new(),
                    flavor_text: String::new(),
                    research_cost: 10.0,
                    activation: ActivationMode::Threshold,
                    icon: String::new(),
                    thumbnail: String::new(),
                    card_image: String::new(),
                    unlock_video: None,
                    model_preview: None,
                    prereqs: vec![CapabilityPrereqSpec {
                        category: "tech::prop".into(),
                        entry_id: "b".into(),
                        source_span_token: Some(99),
                    }],
                    unlocks_ship_components: vec![],
                    unlocks_buildings: vec![],
                    unlocks_units: vec![],
                    unlocks_weapons: vec![],
                    effects: vec![effect.clone()],
                },
                CapabilitySpec {
                    id: "b".into(),
                    display_name: "B".into(),
                    description: String::new(),
                    flavor_text: String::new(),
                    research_cost: 10.0,
                    activation: ActivationMode::Threshold,
                    icon: String::new(),
                    thumbnail: String::new(),
                    card_image: String::new(),
                    unlock_video: None,
                    model_preview: None,
                    prereqs: vec![CapabilityPrereqSpec {
                        category: "tech::prop".into(),
                        entry_id: "a".into(),
                        source_span_token: Some(100),
                    }],
                    unlocks_ship_components: vec![],
                    unlocks_buildings: vec![],
                    unlocks_units: vec![],
                    unlocks_weapons: vec![],
                    effects: vec![effect],
                },
            ],
        }],
    };

    match validate_capability_tree(&tree) {
        Err(SpecError::PrereqCycle {
            source_span_token, ..
        }) => {
            assert!(source_span_token.is_some());
        }
        other => panic!("expected PrereqCycle at admission, got {other:?}"),
    }
}
