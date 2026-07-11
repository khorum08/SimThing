//! STUDIO-FACTION-IDENTITY-FIELDS-0 — owner faction identity authority fields.

use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario, hydrate_scenario_with_source_base, parse_raw_document,
    rebind_pack_to_structural_rebind_ready, HydratedScenarioPack,
};
use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_owner_faction_identity_metadata, deserialize_scenario_authority, format_color_rgb,
    is_owner_entity_kind, make_owner_entity, owner_entity_id, owner_faction_alliance,
    owner_faction_color_rgb, owner_faction_display_name, parse_color_rgb_text,
    serialize_scenario_authority, OWNER_FACTION_ALLIANCE_NONE,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn canonical_clause_path() -> PathBuf {
    repo_root().join("scenarios/terran_pirate_galaxy.clause")
}

fn hydrate_canonical() -> HydratedScenarioPack {
    let path = canonical_clause_path();
    let text = std::fs::read_to_string(&path).expect("read canonical");
    let document = parse_raw_document(text.as_bytes()).expect("parse");
    let base = path.parent().expect("parent");
    hydrate_scenario_with_source_base(&document, Some(base)).expect("hydrate")
}

fn find_owner_by_key<'a>(root: &'a SimThing, key: &str) -> Option<&'a SimThing> {
    fn walk<'a>(node: &'a SimThing, key: &str) -> Option<&'a SimThing> {
        if is_owner_entity_kind(&node.kind) && owner_entity_id(node).as_deref() == Some(key) {
            return Some(node);
        }
        for child in &node.children {
            if let Some(found) = walk(child, key) {
                return Some(found);
            }
        }
        None
    }
    walk(root, key)
}

/// catches: missing serialize/deserialize support for faction identity.
#[test]
fn faction_identity_fields_roundtrip_authority() {
    let mut owner = make_owner_entity("terran", "Terran Compact", "settler");
    apply_owner_faction_identity_metadata(&mut owner, (64, 160, 255), "Terran", "none");
    assert_eq!(owner_faction_color_rgb(&owner), Some((64, 160, 255)));
    assert_eq!(
        owner_faction_display_name(&owner).as_deref(),
        Some("Terran")
    );
    assert_eq!(
        owner_faction_alliance(&owner).as_deref(),
        Some(OWNER_FACTION_ALLIANCE_NONE)
    );

    // Property-level JSON roundtrip (full ScenarioSpec deserialize requires GalaxyMap+STEAD).
    let json = serde_json::to_string(&owner).expect("serialize owner");
    let loaded: SimThing = serde_json::from_str(&json).expect("deserialize owner");
    assert_eq!(owner_faction_color_rgb(&loaded), Some((64, 160, 255)));
    assert_eq!(
        owner_faction_display_name(&loaded).as_deref(),
        Some("Terran")
    );
    assert_eq!(
        owner_faction_alliance(&loaded).as_deref(),
        Some(OWNER_FACTION_ALLIANCE_NONE)
    );
    // Canonical TP path also roundtrips through full authority serialize.
    let pack = hydrate_canonical();
    let (spec, _) = rebind_pack_to_structural_rebind_ready(&pack).expect("rebind");
    let full = serialize_scenario_authority(&spec).expect("serialize full");
    let reloaded = deserialize_scenario_authority(&full).expect("deserialize full");
    let terran = find_owner_by_key(&reloaded.root, "terran").expect("terran");
    assert_eq!(owner_faction_color_rgb(terran), Some((64, 160, 255)));
    assert_eq!(format_color_rgb((1, 2, 3)), "1,2,3");
    assert_eq!(parse_color_rgb_text("#40A0FF").unwrap(), (0x40, 0xA0, 0xFF));
}

/// catches: grammar/hydrate not writing authority fields.
#[test]
fn faction_identity_clause_hydrates_owner_fields() {
    let pack = hydrate_canonical();
    let (spec, _) = rebind_pack_to_structural_rebind_ready(&pack).expect("rebind");
    let terran = find_owner_by_key(&spec.root, "terran").expect("terran owner");
    let pirate = find_owner_by_key(&spec.root, "pirate").expect("pirate owner");
    assert_eq!(owner_faction_color_rgb(terran), Some((64, 160, 255)));
    assert_eq!(owner_faction_color_rgb(pirate), Some((220, 64, 48)));
    assert_eq!(
        owner_faction_display_name(terran).as_deref(),
        Some("Terran")
    );
    assert_eq!(
        owner_faction_display_name(pirate).as_deref(),
        Some("Pirate")
    );
    assert_eq!(
        owner_faction_alliance(terran).as_deref(),
        Some(OWNER_FACTION_ALLIANCE_NONE)
    );
}

/// catches: Terran/Pirate absent or same color.
#[test]
fn faction_identity_canonical_tp_has_distinct_colors() {
    let pack = hydrate_canonical();
    let terran = pack
        .owners
        .iter()
        .find(|o| o.owner_key == "terran")
        .expect("terran");
    let pirate = pack
        .owners
        .iter()
        .find(|o| o.owner_key == "pirate")
        .expect("pirate");
    assert_ne!(terran.color_rgb, pirate.color_rgb);
    assert_eq!(terran.color_rgb, Some((64, 160, 255)));
    assert_eq!(pirate.color_rgb, Some((220, 64, 48)));
}

/// catches: missing display names.
#[test]
fn faction_identity_canonical_tp_names_present() {
    let pack = hydrate_canonical();
    let terran = pack
        .owners
        .iter()
        .find(|o| o.owner_key == "terran")
        .expect("terran");
    let pirate = pack
        .owners
        .iter()
        .find(|o| o.owner_key == "pirate")
        .expect("pirate");
    assert_eq!(terran.faction_name.as_deref(), Some("Terran"));
    assert_eq!(pirate.faction_name.as_deref(), Some("Pirate"));
    assert!(!terran.display_name.is_empty());
    assert!(!pirate.display_name.is_empty());
}

/// catches: silent bad color parse.
#[test]
fn faction_identity_rejects_malformed_color_rgb() {
    let source = r#"
scenario = bad_color {
    owner = terran {
        owner_key = "terran"
        display_name = "Terran"
        faction_name = "Terran"
        color_rgb = "not-a-color"
        faction_alliance = "none"
    }
}
"#;
    let document = parse_raw_document(source.as_bytes()).expect("parse");
    let err = hydrate_scenario(&document).expect_err("malformed color must fail");
    assert!(
        err.to_string().contains("color_rgb"),
        "error must mention color_rgb: {err}"
    );
}

/// catches: silent fallback/default color authority.
#[test]
fn faction_identity_missing_required_color_fails_loud() {
    let source = r#"
scenario = missing_color {
    owner = terran {
        owner_key = "terran"
        display_name = "Terran"
        faction_name = "Terran"
        faction_alliance = "none"
    }
}
"#;
    let document = parse_raw_document(source.as_bytes()).expect("parse");
    let err = hydrate_scenario(&document).expect_err("missing color_rgb must fail");
    assert!(
        err.to_string().contains("color_rgb"),
        "error must mention color_rgb: {err}"
    );
}

/// catches: regression of portable canonical ClauseScript empty-resolver path.
#[test]
fn faction_identity_11_1_canonical_load_still_empty_resolver() {
    let original = std::env::current_dir().expect("cwd");
    let alien = repo_root().join("crates").join("simthing-spec");
    std::env::set_current_dir(&alien).expect("chdir");
    let result = std::panic::catch_unwind(|| {
        let pack = hydrate_canonical();
        assert_eq!(pack.scenario_id, "terran_pirate_galaxy");
        assert_eq!(pack.embedded_static_galaxy_scenarios.len(), 1);
        assert_eq!(pack.owners.len(), 2);
        assert!(pack.owners.iter().all(|o| o.color_rgb.is_some()));
    });
    std::env::set_current_dir(&original).expect("restore");
    result.expect("empty-resolver canonical still hydrates");
}

/// catches: mapeditor/gameplay leakage via hydrate_scenario module scan.
#[test]
fn faction_identity_no_ui_or_gameplay_semantics() {
    let source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/hydrate_scenario.rs"
    ));
    for banned in [
        "simthing_mapeditor",
        "DiplomacySystem",
        "CombatSystem",
        "CpuPlanner",
        "GameModeAttach",
        "war_state",
        "relation_graph",
    ] {
        // Only fail on import/use shapes, not comment mentions of future rungs.
        for line in source.lines() {
            let t = line.trim_start();
            if t.starts_with("//") || t.starts_with("//!") || t.starts_with("///") {
                continue;
            }
            if t.contains("use ") && t.contains(banned) {
                panic!("hydrate_scenario imports banned surface: {banned} in {t}");
            }
        }
    }
    // Spec helpers are pure readers — no gameplay attach APIs.
    let spec_src = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../simthing-spec/src/spec/scenario.rs"
    ));
    assert!(spec_src.contains("OWNER_COLOR_RGB_PROPERTY_ID"));
    assert!(
        spec_src.contains("no diplomacy")
            || spec_src.contains("Alliance grouping placeholder")
            || spec_src.contains("faction_alliance")
    );
}
