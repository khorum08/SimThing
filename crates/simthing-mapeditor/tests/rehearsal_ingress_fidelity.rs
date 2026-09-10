//! Owning ingress falsifier at the existing hydrated-source -> Studio boundary.
//! The specimen authors ordinary spec properties, programs and resource-parent edges;
//! no execution state or expected value is injected into the opened session.

use std::collections::BTreeMap;

use simthing_clausething::{hydrate_scenario, parse_raw_document, HydratedScenarioPack};
use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    OverlayKind, OverlayLifecycle, OverlaySource, SimThing, SimThingId, SimThingKind, SubFieldRole,
    SubFieldSpec, TransformOp,
};
use simthing_driver::SimSession;
use simthing_mapeditor::studio_live_session_bridge::{
    authored_live_profile_from_pack, driver_scenario_field_bearing_from_profile,
    StudioAuthoredLiveProfile, StudioLiveSessionBridge,
};
use simthing_mapeditor::StudioSession;
use simthing_spec::spec::install_target::InstallTargetSpec;
use simthing_spec::spec::resource_economy::{EmissionFormulaSpec, ResourceEmissionSpec};
use simthing_spec::{
    apply_gridcell_role_metadata, apply_participant_owner_flow_metadata, compile_property,
    game_session_child, make_planet_gridcell, scenario_metadata_string_value,
    structural_property_value_u32, DomainPackSpec, OverlaySpec, PropertyKey, PropertySpec,
    SimThingScenarioGrid, SimThingScenarioSpec, SimThingStructuralGridFrame,
    SimThingStructuralGridPlacement, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    PLANET_OWNER_REF_PROPERTY_ID, SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

const NS: &str = "studio_live_rf";
const PROPERTY: &str = "owner_flow";
const ARENA: &str = "studio_recursive_owner_flow";

fn subfield(name: &str, role: Option<AccumulatorRole>, default: f32) -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named(name.into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default,
        display_name: name.into(),
        display_range: None,
        governed_by: (name == "balance").then(|| SubFieldRole::Named("balance_rate".into())),
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: role.map(|role| AccumulatorSpec {
            role,
            log_tier: LogTier::Summary,
        }),
    }
}

fn property() -> PropertySpec {
    PropertySpec {
        id: PROPERTY.into(),
        namespace: NS.into(),
        name: PROPERTY.into(),
        display_name: "Authored resource".into(),
        description: String::new(),
        admission_disposition: Default::default(),
        sub_fields: vec![
            subfield("flow", Some(AccumulatorRole::IntrinsicFlow), 0.0),
            subfield(
                "allocated",
                Some(AccumulatorRole::AllocatedFlow {
                    arena: ARENA.into(),
                }),
                0.0,
            ),
            subfield(
                "weight",
                Some(AccumulatorRole::AllocatorWeight {
                    arena: ARENA.into(),
                }),
                1.0,
            ),
            subfield("balance_rate", None, 0.0),
            subfield(
                "balance",
                Some(AccumulatorRole::Balance(BalanceSpec::default())),
                0.0,
            ),
        ],
    }
}

fn populate(
    node: &mut SimThing,
    registry: &DimensionRegistry,
    parent: Option<SimThingId>,
    rate: f32,
) {
    let id = registry.id_of(NS, PROPERTY).unwrap();
    let prop = registry.property(id);
    let mut value = prop.default_value();
    value.set_role(&SubFieldRole::Named("flow".into()), &prop.layout, rate);
    node.add_property(id, value);
    if let Some(parent) = parent {
        node.add_resource_parent_edge(NS, PROPERTY, parent, None);
    }
}

fn policy(owner: &str, multiplier: f32) -> OverlaySpec {
    OverlaySpec {
        id: format!("{owner}_policy"),
        display_name: format!("{owner} policy"),
        targets_property: format!("{NS}::{PROPERTY}"),
        sub_field_deltas: vec![(
            SubFieldRole::Named("weight".into()),
            TransformOp::multiply(multiplier),
        )],
        lifecycle: OverlayLifecycle::UntilDissolved,
        kind: OverlayKind::Policy,
        source: OverlaySource::Player,
        install: InstallTargetSpec::ScenarioListed {
            target_id: owner.into(),
        },
        order_weight_class: None,
        composition_class: None,
        current_dependency_edges: vec![],
        next_dependency_edges: vec![],
        source_span_token: None,
    }
}

/// Author at the public HydratedScenarioPack boundary, which already represents this graph.
/// The ClauseThing text parser supplies the two owner identities. Remaining authored input
/// uses existing canonical types; JSON roundtrip fixes the complete input before live loading.
fn specimen() -> (
    HydratedScenarioPack,
    SimThingScenarioSpec,
    BTreeMap<String, SimThingId>,
) {
    let raw = parse_raw_document(
        br#"scenario = ingress_f1 {
        owner = alpha { owner_key = alpha }
        owner = beta { owner_key = beta }
        location = authoring_marker { display_name = "Authoring marker" }
    }"#,
    )
    .unwrap();
    let mut pack = hydrate_scenario(&raw).unwrap();
    let mut registry = DimensionRegistry::new();
    compile_property(&property(), &mut registry).unwrap();
    pack.game_mode.properties = vec![property()];
    pack.game_mode.domain_packs = vec![DomainPackSpec {
        id: "authored_policies".into(),
        display_name: "Authored policies".into(),
        metadata: Default::default(),
        properties: vec![],
        overlays: vec![policy("alpha", 2.0), policy("beta", 3.0)],
        capability_trees: vec![],
        events: vec![],
    }];
    let root = pack.authority_root.as_mut().unwrap();
    let session = root
        .children
        .iter_mut()
        .find(|n| n.kind == SimThingKind::GameSession)
        .unwrap();
    let session_id = session.id;
    populate(session, &registry, None, 47.0);
    let mut ids = BTreeMap::from([("session".into(), session_id)]);
    for (index, owner) in ["alpha", "beta"].iter().enumerate() {
        let owner_node = &mut session.children[index];
        populate(owner_node, &registry, None, 0.0);
        ids.insert((*owner).into(), owner_node.id);
        pack.install_targets
            .insert((*owner).into(), vec![owner_node.id]);
    }
    let map = session
        .children
        .iter_mut()
        .find(|n| n.kind == SimThingKind::Location)
        .unwrap();
    let map_id = map.id;
    let mut placements = vec![];
    for (index, owner) in ["alpha", "beta"].iter().enumerate() {
        let mut system = SimThing::new(SimThingKind::Location, 0);
        apply_gridcell_role_metadata(&mut system, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM);
        for (pid, value) in [
            (SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, index as u32 + 1),
            (SCENARIO_STRUCTURAL_COL_PROPERTY_ID, index as u32),
            (SCENARIO_STRUCTURAL_ROW_PROPERTY_ID, 0),
        ] {
            system.add_property(pid, structural_property_value_u32(value));
        }
        let mut planet = make_planet_gridcell(&format!("{owner}_site"), 0, 0, None);
        planet.add_property(
            PLANET_OWNER_REF_PROPERTY_ID,
            scenario_metadata_string_value(owner),
        );
        populate(&mut planet, &registry, Some(ids[*owner]), 0.0);
        ids.insert(format!("{owner}_site"), planet.id);
        let site_id = planet.id;
        let surface = &mut planet.children[0];
        populate(surface, &registry, None, 0.0);
        ids.insert(format!("{owner}_surface"), surface.id);
        let mut previous = site_id;
        for child_index in 0..5 {
            let rate = if child_index == 4 { 0 } else { 5 + child_index };
            let mut child = SimThing::new(SimThingKind::Custom("Infrastructure".into()), 0);
            apply_participant_owner_flow_metadata(&mut child, owner, rate, 0);
            // Beta's last positive child has one extra RF edge; spatial placement is unchanged.
            let parent = if *owner == "beta" && child_index == 3 {
                Some(previous)
            } else {
                None
            };
            populate(&mut child, &registry, parent, rate as f32);
            previous = child.id;
            ids.insert(format!("{owner}_{child_index}"), child.id);
            surface.add_child(child);
        }
        system.add_child(planet);
        placements.push(SimThingStructuralGridPlacement {
            location_id: format!("system_{index}"),
            target_id: format!("system_{index}"),
            system_id: index as u32 + 1,
            row: 0,
            col: index as u32,
            simthing_id_raw: system.id.raw(),
        });
        map.add_child(system);
    }
    pack.install_targets
        .insert("ingress_f1".into(), vec![session_id]);
    // Ordinary executable registration makes the authored profile field-bearing.
    pack.game_mode
        .resource_economy
        .get_or_insert_with(Default::default)
        .emissions
        .push(ResourceEmissionSpec {
            id: "authored_root_supply".into(),
            source: PropertyKey::new(NS, PROPERTY),
            source_role: SubFieldRole::Named("flow".into()),
            formula: EmissionFormulaSpec::Constant(47.0),
            host_entity: Some("ingress_f1".into()),
            host_span_token: None,
        });
    let pack: HydratedScenarioPack =
        serde_json::from_slice(&serde_json::to_vec(&pack).unwrap()).unwrap();
    let scenario = SimThingScenarioSpec {
        scenario_id: pack.scenario_id.clone(),
        root: pack.authority_root.clone().unwrap(),
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 2,
                height: 1,
                occupied_cells: 2,
            },
            map_container_id: map_id.raw().to_string(),
            placements,
        },
        links: vec![],
        provenance: Default::default(),
    };
    assert_eq!(game_session_child(&scenario).unwrap().id, session_id);
    (pack, scenario, ids)
}

fn flow_at(sim: &SimSession, id: SimThingId) -> Option<f32> {
    // Admission-time authored data on the installed runtime tree, before any tick.
    // This is not a readback of born GPU output (the both-routes successor owns that proof).
    let pid = sim.proto.registry.id_of(NS, PROPERTY)?;
    let prop = sim.proto.registry.property(pid);
    sim.spec_state.arena_registry.participant_slot(id, 0)?;
    Some(
        sim.proto
            .root
            .property_on_node(id, pid)?
            .get_role(&SubFieldRole::Named("flow".into()), &prop.layout),
    )
}

fn parents(sim: &SimSession) -> BTreeMap<SimThingId, Option<SimThingId>> {
    sim.spec_state
        .resource_flow_derivation
        .arenas
        .iter()
        .find(|arena| arena.arena == ARENA)
        .unwrap()
        .participants
        .iter()
        .map(|participant| (participant.simthing_id, participant.parent))
        .collect()
}

#[test]
fn rehearsal_ingress_installs_full_authored_economy() {
    let (pack, scenario, ids) = specimen();
    let authored = StudioAuthoredLiveProfile::from_hydrated_pack(
        pack.game_mode.clone(),
        pack.install_targets.clone(),
        scenario.root.clone(),
        true,
    );
    let control_scenario = driver_scenario_field_bearing_from_profile(&authored).unwrap();
    let mut control_mode = pack.game_mode.clone();
    // The driver scenario already owns the compiled property registry. Keep all programs.
    control_mode.properties.clear();
    let control = SimSession::open_from_spec(control_scenario, &control_mode)
        .expect("the authored input is admitted through the canonical programmatic door");
    let expected: BTreeMap<_, _> = ids
        .iter()
        .map(|(label, id)| (label.clone(), flow_at(&control, *id)))
        .collect();
    assert!(
        expected.values().all(Option::is_some),
        "canonical control must host every authored participant: {expected:?}"
    );
    assert_eq!(expected["session"], Some(47.0));
    assert_eq!(control.proto.root.overlay_count(ids["alpha"]), Some(1));
    assert_eq!(control.proto.root.overlay_count(ids["beta"]), Some(1));
    let expected_parents = parents(&control);
    assert_eq!(expected_parents.len(), 17);
    assert_eq!(
        expected_parents[&ids["alpha_3"]],
        Some(ids["alpha_surface"])
    );
    assert_eq!(expected_parents[&ids["beta_3"]], Some(ids["beta_2"]));
    eprintln!("F1 canonical control intrinsic flows: {expected:?}");
    drop(control);
    let studio = StudioSession::from_loaded_scenario(scenario, "authored-f1.json".into(), None)
        .unwrap()
        .with_authored_live_profile(authored_live_profile_from_pack(&pack));
    let mut bridge = StudioLiveSessionBridge::default();
    bridge
        .open_from_loaded_studio_session(&studio)
        .expect("ordinary live session opens");
    let sim = bridge.sim_session().unwrap();
    let actual: BTreeMap<_, _> = ids
        .iter()
        .map(|(label, id)| (label.clone(), flow_at(sim, *id)))
        .collect();
    eprintln!("F1 installed intrinsic flows: {actual:?}");
    eprintln!(
        "F1 installed policy counts: alpha={:?}, beta={:?}",
        sim.proto.root.overlay_count(ids["alpha"]),
        sim.proto.root.overlay_count(ids["beta"])
    );
    let missing: Vec<_> = actual
        .iter()
        .filter(|(_, value)| value.is_none())
        .map(|(label, _)| label)
        .collect();
    assert!(
        missing.is_empty(),
        "F1 substituted economy: authored PRESENT participants missing: {missing:?}"
    );
    assert_eq!(
        actual["session"],
        Some(47.0),
        "authored supply remains installed"
    );
    assert_eq!(
        parents(sim),
        expected_parents,
        "authored RF parentage must survive independently of spatial parentage"
    );
    assert_eq!(sim.proto.root.overlay_count(ids["alpha"]), Some(1));
    assert_eq!(sim.proto.root.overlay_count(ids["beta"]), Some(1));
    for owner in ["alpha", "beta"] {
        assert_eq!(actual[&format!("{owner}_3")], Some(8.0));
        assert_eq!(actual[&format!("{owner}_4")], Some(0.0), "zero is PRESENT");
    }
}
