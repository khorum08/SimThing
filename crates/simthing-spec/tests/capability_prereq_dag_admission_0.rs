//! CAPABILITY-PREREQ-DAG-ADMISSION-0 — authored-corpus census + builder gate.
//!
//! Every fixture / example capability tree must admit under the new prereq DAG
//! validation. If any fails, that is live malformed prereq data → STOP/DA-route.

use simthing_core::DimensionRegistry;
use simthing_spec::{
    validate_capability_tree, CapabilityTreeBuilder, CapabilityTreeSpec, GameModeSpec,
};

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

fn parse_game_mode(src: &str) -> GameModeSpec {
    ron::from_str(src).expect("game mode fixture must parse")
}

fn parse_tree(src: &str) -> CapabilityTreeSpec {
    ron::from_str(src).expect("capability tree fixture must parse")
}

const FIXTURES: &[(&str, &str)] = &[
    (
        "minimal_tech_tree.ron",
        include_str!("fixtures/minimal_tech_tree.ron"),
    ),
    (
        "minimal_game_mode.ron",
        include_str!("fixtures/minimal_game_mode.ron"),
    ),
    (
        "docs/examples/game_mode_install_all_factions.ron",
        include_str!("../../../docs/examples/game_mode_install_all_factions.ron"),
    ),
    (
        "docs/examples/game_mode_install_scenario_listed.ron",
        include_str!("../../../docs/examples/game_mode_install_scenario_listed.ron"),
    ),
    (
        "docs/examples/game_mode_install_session_root.ron",
        include_str!("../../../docs/examples/game_mode_install_session_root.ron"),
    ),
];

#[test]
fn existing_authored_capability_trees_admit_unchanged() {
    let mut admitted = 0usize;
    for (label, src) in FIXTURES {
        // Strip leading comment lines that are not RON.
        let body = src
            .lines()
            .skip_while(|l| l.trim_start().starts_with("//") || l.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        if body.trim_start().starts_with("CapabilityTreeSpec") {
            let tree = parse_tree(&body);
            // Standalone tree RON has no property registry — admission
            // validation is the census surface for prereq DAG shape.
            validate_capability_tree(&tree)
                .unwrap_or_else(|e| panic!("census FAIL on {label}: {e}"));
            admitted += 1;
        } else {
            let mode = parse_game_mode(&body);
            let trees = trees_from_game_mode(&mode);
            assert!(
                !trees.is_empty(),
                "fixture {label} must contain at least one capability tree"
            );
            for tree in trees {
                validate_capability_tree(tree)
                    .unwrap_or_else(|e| panic!("census FAIL on {label} tree {}: {e}", tree.tree_id));
                let mut registry = DimensionRegistry::new();
                for prop in &mode.properties {
                    let _ = simthing_spec::compile_property(prop, &mut registry);
                }
                for pack in &mode.domain_packs {
                    for prop in &pack.properties {
                        let _ = simthing_spec::compile_property(prop, &mut registry);
                    }
                }
                CapabilityTreeBuilder::build(tree, &mut registry).unwrap_or_else(|e| {
                    panic!("builder FAIL on {label} tree {}: {e}", tree.tree_id)
                });
                admitted += 1;
            }
        }
    }
    assert!(
        admitted >= 4,
        "expected at least 4 authored trees in the census, got {admitted}"
    );
}

#[test]
fn builder_rejects_prereq_cycle_at_admission() {
    use simthing_spec::{
        ActivationMode, CapabilityCategorySpec, CapabilityEffectSpec, CapabilityPrereqSpec,
        CapabilitySpec, EffectTarget, SpecError,
    };
    use simthing_core::{OverlayLifecycle, SubFieldRole, TransformOp};

    let effect = CapabilityEffectSpec {
        targets_property: "military::fleet_speed".into(),
        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(1.1))],
        when_activated: OverlayLifecycle::Permanent,
        effect_target: EffectTarget::CapabilityTree,
        source_span_token: None,
    };
    let tree = CapabilityTreeSpec {
        tree_id: "cycle".into(),
        tree_kind: "tech_tree".into(),
        owner_kind: "Faction".into(),
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
