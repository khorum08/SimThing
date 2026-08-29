//! CLAUSETHING-ADMISSION-CONVERGENCE-0 corpus-horizon witness.
//!
//! The inventory is discovered from the fixture tree. Executable dialects pass
//! through their production hydrator and the ordinary driver admission door;
//! syntax/expansion/scope fixtures retain their stage-specific contract, and
//! existing negative inputs retain their exact semantic refusal reason.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use simthing_clausething::raw::{RawBlock, RawDocument, RawValue};
use simthing_clausething::{
    extract_scopes, extract_scopes_validated, hydrate_category_economy_pack,
    hydrate_daily_economy_game_mode, hydrate_entity_pack, hydrate_field_operator_pack,
    hydrate_resource_flow_pack, hydrate_scenario_with_source_base, hydrate_shipsize_decoder_pack,
    parse_raw_document, synthetic_scope_table,
};
use simthing_core::{DimensionRegistry, SimThing, SimThingKind};
use simthing_driver::{preview_install, Scenario};
use simthing_gpu::SlotAllocator;
use simthing_spec::{compile_property, GameModeSpec, InstallTargetSpec};

const DIRECT_FIXTURE_COUNT: usize = 41;
const GOVERNED_CORPUS_COUNT: usize = 42;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Dialect {
    Scenario,
    FieldOperator,
    CategoryEconomy,
    ResourceFlow,
    DailyEconomy,
    Entity,
    Shipsize,
}

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn governed_corpus() -> Vec<PathBuf> {
    let root = fixture_root();
    let mut direct = std::fs::read_dir(&root)
        .expect("read ClauseThing fixture root")
        .map(|entry| entry.expect("read fixture entry").path())
        .filter(|path| path.is_file() && path.extension().is_some_and(|ext| ext == "clause"))
        .collect::<Vec<_>>();
    direct.sort();
    assert_eq!(direct.len(), DIRECT_FIXTURE_COUNT);

    let canonical = root.join("scenario/terran_pirate_galaxy.clause");
    assert!(canonical.is_file(), "canonical scenario fixture must exist");
    direct.push(canonical);
    assert_eq!(direct.len(), GOVERNED_CORPUS_COUNT);
    direct
}

fn body(document: &RawDocument) -> Option<&RawBlock> {
    let RawValue::Block(root) = &document.root else {
        return None;
    };
    let property = root.properties.first()?;
    match &property.value {
        RawValue::Block(block) => Some(block),
        RawValue::Header(header) => match header.payload.as_ref() {
            RawValue::Block(block) => Some(block),
            _ => None,
        },
        _ => None,
    }
}

fn dialect(document: &RawDocument) -> Option<Dialect> {
    let RawValue::Block(root) = &document.root else {
        return None;
    };
    let first = root.properties.first()?;
    if root.properties.len() == 1 && first.key.text == "scenario" {
        return Some(Dialect::Scenario);
    }

    let keys = body(document)?
        .properties
        .iter()
        .map(|property| property.key.text.as_str())
        .collect::<Vec<_>>();
    if keys.contains(&"ship_class_map") {
        Some(Dialect::Shipsize)
    } else if keys.contains(&"resource_flow") && keys.contains(&"category_map") {
        Some(Dialect::CategoryEconomy)
    } else if keys.contains(&"flow_property") && keys.contains(&"arena") {
        Some(Dialect::ResourceFlow)
    } else if keys.contains(&"saturating_flux") {
        Some(Dialect::FieldOperator)
    } else if keys.contains(&"property") && (keys.contains(&"transfer") || keys.contains(&"recipe"))
    {
        Some(Dialect::DailyEconomy)
    } else if keys.contains(&"property")
        && (keys.contains(&"modifier")
            || keys.contains(&"triggered_modifier")
            || keys.contains(&"tradition_tree"))
    {
        Some(Dialect::Entity)
    } else {
        None
    }
}

fn broad_root(custom_kinds: impl IntoIterator<Item = String>) -> SimThing {
    let mut root = SimThing::new(SimThingKind::World, 0);
    root.add_child(SimThing::new(SimThingKind::Owner, 0));
    root.add_child(SimThing::new(SimThingKind::Fleet, 0));
    root.add_child(SimThing::new(SimThingKind::Custom("Ship".into()), 0));
    for _ in 0..4 {
        root.add_child(SimThing::new(SimThingKind::Cohort, 0));
    }
    for kind in custom_kinds {
        root.add_child(SimThing::new(SimThingKind::Custom(kind), 0));
    }
    root
}

fn ordinary_preview(
    name: &str,
    game_mode: &GameModeSpec,
    registry: DimensionRegistry,
    root: SimThing,
    install_targets: HashMap<String, Vec<simthing_core::SimThingId>>,
) -> Result<(), String> {
    let scenario = Scenario {
        name: name.into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: (root.subtree_size() as u32).saturating_add(4096),
        registry,
        root,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets,
    };
    let mut allocator = SlotAllocator::new();
    allocator
        .install_initial_tree(&scenario.root)
        .map_err(|error| format!("allocator install: {error}"))?;
    preview_install(
        game_mode,
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}

fn scenario_source(path: &Path, source: String) -> String {
    if !source.contains("{{FIXTURE_JSON}}") {
        return source;
    }
    let base = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/terran_pirate_galaxy.base_disc.json")
        .canonicalize()
        .unwrap_or_else(|error| panic!("canonical base disc for {}: {error}", path.display()));
    source.replace(
        "{{FIXTURE_JSON}}",
        &base.to_string_lossy().replace('\\', "/"),
    )
}

fn hydrate_and_admit(path: &Path, document: &RawDocument, dialect: Dialect) -> Result<(), String> {
    let name = path
        .file_name()
        .expect("fixture filename")
        .to_string_lossy();
    match dialect {
        Dialect::Scenario => {
            let pack = hydrate_scenario_with_source_base(document, path.parent())
                .map_err(|error| error.message)?;
            ordinary_preview(
                &name,
                &pack.game_mode,
                DimensionRegistry::new(),
                pack.root,
                pack.install_targets.into_iter().collect(),
            )
        }
        Dialect::FieldOperator => {
            let pack = hydrate_field_operator_pack(document).map_err(|error| error.message)?;
            ordinary_preview(
                &name,
                &pack.game_mode,
                DimensionRegistry::new(),
                broad_root([]),
                HashMap::new(),
            )
        }
        Dialect::CategoryEconomy => {
            let pack = hydrate_category_economy_pack(document).map_err(|error| error.message)?;
            let root = broad_root([]);
            let host = root
                .children
                .iter()
                .find(|child| child.kind == SimThingKind::Cohort)
                .expect("category host")
                .id;
            let install_targets = pack
                .game_mode
                .resource_flow
                .iter()
                .flat_map(|resource_flow| resource_flow.base_obligations.iter())
                .filter_map(|obligation| match &obligation.install {
                    InstallTargetSpec::ScenarioListed { target_id } => {
                        Some((target_id.clone(), vec![host]))
                    }
                    _ => None,
                })
                .collect();
            ordinary_preview(
                &name,
                &pack.game_mode,
                pack.scenario_registry,
                root,
                install_targets,
            )
        }
        Dialect::ResourceFlow => {
            let pack = hydrate_resource_flow_pack(document).map_err(|error| error.message)?;
            ordinary_preview(
                &name,
                &pack.game_mode,
                DimensionRegistry::new(),
                broad_root([]),
                HashMap::new(),
            )
        }
        Dialect::DailyEconomy => {
            let game_mode =
                hydrate_daily_economy_game_mode(document).map_err(|error| error.message)?;
            ordinary_preview(
                &name,
                &game_mode,
                DimensionRegistry::new(),
                broad_root([]),
                HashMap::new(),
            )
        }
        Dialect::Entity => {
            let pack = hydrate_entity_pack(document).map_err(|error| error.message)?;
            let mut domain_pack = pack.domain_pack;
            let mut registry = DimensionRegistry::new();
            let property_ids = domain_pack
                .properties
                .drain(..)
                .map(|property| {
                    compile_property(&property, &mut registry)
                        .map(|(property_id, _)| property_id)
                        .map_err(|error| error.to_string())
                })
                .collect::<Result<Vec<_>, _>>()?;
            let game_mode = GameModeSpec {
                id: name.to_string(),
                display_name: name.to_string(),
                domain_packs: vec![domain_pack],
                ..Default::default()
            };
            let mut root = broad_root([]);
            for property_id in property_ids {
                let value = registry.property(property_id).default_value();
                root.add_property(property_id, value.clone());
                for owner in &mut root.children {
                    if owner.kind == SimThingKind::Owner {
                        owner.add_property(property_id, value.clone());
                    }
                }
            }
            ordinary_preview(&game_mode.id, &game_mode, registry, root, HashMap::new())
        }
        Dialect::Shipsize => {
            let pack = hydrate_shipsize_decoder_pack(document).map_err(|error| error.message)?;
            ordinary_preview(
                &name,
                &pack.game_mode,
                DimensionRegistry::new(),
                broad_root(pack.ship_class_custom_kinds.into_values()),
                HashMap::new(),
            )
        }
    }
}

#[test]
fn complete_clause_fixture_tree_preserves_stage_and_admission_contracts() {
    let mut admitted = Vec::new();
    let mut stage_only = Vec::new();
    let mut negative_reasons = BTreeMap::new();

    for path in governed_corpus() {
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let source = scenario_source(&path, source);
        let document = parse_raw_document(source.as_bytes())
            .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()));
        let name = path
            .file_name()
            .expect("fixture filename")
            .to_string_lossy()
            .into_owned();

        if let Some(dialect) = dialect(&document) {
            match hydrate_and_admit(&path, &document, dialect) {
                Ok(()) => admitted.push(name),
                Err(reason) => {
                    negative_reasons.insert(name, reason);
                }
            }
        } else if name.starts_with("scope_") {
            let report = if name == "scope_malformed.clause" {
                extract_scopes(&document)
            } else {
                extract_scopes_validated(&document, &synthetic_scope_table())
            };
            if !matches!(
                name.as_str(),
                "scope_malformed.clause" | "scope_unknown_domain.clause"
            ) {
                stage_only.push(name);
            } else {
                negative_reasons.insert(
                    name,
                    report
                        .diagnostics
                        .iter()
                        .map(|diagnostic| diagnostic.message.as_str())
                        .collect::<Vec<_>>()
                        .join(" | "),
                );
            }
        } else {
            stage_only.push(name);
        }
    }

    eprintln!(
        "CLAUSETHING-CORPUS governed={} admitted={} stage_only={} negatives={} admitted_files={admitted:?} stage_only_files={stage_only:?} negative_reasons={negative_reasons:?}",
        GOVERNED_CORPUS_COUNT,
        admitted.len(),
        stage_only.len(),
        negative_reasons.len(),
    );

    assert_eq!(admitted.len(), 18, "executable positive corpus drift");
    assert_eq!(stage_only.len(), 19, "stage-only positive corpus drift");
    assert_eq!(negative_reasons.len(), 5, "negative corpus drift");
    assert_eq!(
        negative_reasons,
        BTreeMap::from([
            (
                "bh3_invalid_chi.clause".into(),
                "SaturatingFlux chi 0.5 exceeds CFL bound 0.25 (dt=1.0)".into(),
            ),
            (
                "bh3_missing_u_sat.clause".into(),
                "missing required field `u_sat`".into(),
            ),
            (
                "ct1a_unsupported_field.clause".into(),
                "unsupported entity field `on_action`".into(),
            ),
            (
                "scope_malformed.clause".into(),
                "malformed scope chain `root..owner`: empty dot segment | malformed scope chain `.from`: empty dot segment".into(),
            ),
            (
                "scope_unknown_domain.clause".into(),
                "unknown domain scope `fictitious_relay_scope` (not in validation table)".into(),
            ),
        ])
    );
}
