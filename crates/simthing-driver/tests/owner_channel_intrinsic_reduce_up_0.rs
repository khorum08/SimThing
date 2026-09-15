//! OWNER-CHANNEL-INTRINSIC-0 deliverables (d)/(e).
//!
//! All feedstock is inline and synthetic. The proofs exercise arbitrary tree structure and
//! generic resource keys; no asset or corpus participates.

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{SimThing, SimThingId, SimThingKind};
use simthing_driver::{
    compile_owner_channel_rf_gpu_proof_plan, prove_owner_channel_rf_cpu_gpu_parity,
};
use simthing_gpu::GpuContext;
use simthing_spec::{
    admit_intrinsic_owner_channels, apply_gridcell_role_metadata, apply_owner_silo_metadata,
    apply_participant_owner_flow_metadata, apply_scenario_metadata_to_root,
    apply_star_system_local_grid_frame_metadata, is_surface_gridcell, make_galaxy_map,
    make_owner_entity, make_planet_gridcell, reconstruct_owner_channel_rf_map,
    reduce_owner_channel_rf, structural_property_value_u32, OwnerChannelRfOwnAggregate,
    ResourceKey, SimThingScenarioGrid, SimThingScenarioProvenance, SimThingScenarioSpec,
    SimThingStructuralGridFrame, SimThingStructuralGridPlacement, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_SCHEMA_VERSION,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
    STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS, STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS,
};

fn node() -> SimThing {
    SimThing::new(SimThingKind::Custom("synthetic".into()), 0)
}

fn own(
    simthing_id: SimThingId,
    resource: &str,
    surplus: u32,
    deficit: u32,
) -> OwnerChannelRfOwnAggregate {
    OwnerChannelRfOwnAggregate {
        simthing_id,
        resource_key: ResourceKey::new(resource),
        surplus,
        deficit,
    }
}

fn three_owner_tree() -> (SimThing, Vec<OwnerChannelRfOwnAggregate>) {
    let mut root = node();
    bind_owner(&mut root, &OwnerRef::new("alpha"));

    let mut inherited = node();
    let inherited_leaf = node();
    let inherited_leaf_id = inherited_leaf.id;
    inherited.add_child(inherited_leaf);

    let mut crossing = node();
    bind_owner(&mut crossing, &OwnerRef::new("beta"));
    let crossing_id = crossing.id;
    let crossing_leaf = node();
    let crossing_leaf_id = crossing_leaf.id;
    crossing.add_child(crossing_leaf);

    let mut nested_crossing = node();
    bind_owner(&mut nested_crossing, &OwnerRef::new("gamma"));
    let nested_crossing_id = nested_crossing.id;
    crossing.add_child(nested_crossing);

    let root_id = root.id;
    root.add_child(inherited);
    root.add_child(crossing);

    let rows = vec![
        own(root_id, "ore", 3, 0),
        own(inherited_leaf_id, "ore", 4, 0),
        own(crossing_id, "ore", 5, 1),
        own(crossing_leaf_id, "ore", 2, 3),
        own(nested_crossing_id, "ore", 0, 6),
        own(root_id, "water", 1, 2),
        own(crossing_leaf_id, "water", 8, 1),
        own(nested_crossing_id, "water", 2, 2),
    ];
    (root, rows)
}

fn gpu_context() -> Option<GpuContext> {
    match GpuContext::new_blocking() {
        Ok(context) => Some(context),
        Err(_) if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() => {
            panic!("GPU adapter is required for owner-channel parity")
        }
        Err(_) => None,
    }
}

fn intrinsic_rf_compile_scenario() -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    let provenance = SimThingScenarioProvenance {
        source: "OWNER-CHANNEL-INTRINSIC-0".into(),
        generator_seed: 0x6000,
        generator_shape: "intrinsic_owner_compile".into(),
        ..SimThingScenarioProvenance::default()
    };
    apply_scenario_metadata_to_root(
        &mut root,
        "intrinsic_owner_compile",
        &provenance,
        SCENARIO_SCHEMA_VERSION,
    );

    let mut owner_a = make_owner_entity("owner_a", "Owner A", "synthetic");
    apply_owner_silo_metadata(&mut owner_a, 10, Some(100));
    let mut owner_b = make_owner_entity("owner_b", "Owner B", "synthetic");
    apply_owner_silo_metadata(&mut owner_b, 20, Some(100));

    let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
    game_session.add_child(owner_a);
    game_session.add_child(owner_b);

    let mut galaxy_map = make_galaxy_map("map", "Synthetic Map");
    let map_raw = galaxy_map.id.raw();
    let mut star = SimThing::new(SimThingKind::Location, 0);
    apply_gridcell_role_metadata(&mut star, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM);
    apply_star_system_local_grid_frame_metadata(
        &mut star,
        STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
        STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS,
    );
    star.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(1),
    );
    star.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    star.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    let star_raw = star.id.raw();

    for (planet_id, owner, surplus, deficit, col) in
        [("p0", "owner_a", 7, 1, 0), ("p1", "owner_b", 3, 5, 1)]
    {
        let mut planet = make_planet_gridcell(planet_id, col, 0, None);
        let mut participant = SimThing::new(SimThingKind::Cohort, 0);
        apply_participant_owner_flow_metadata(&mut participant, owner, surplus, deficit);
        planet
            .children
            .iter_mut()
            .find(|child| is_surface_gridcell(child))
            .expect("surface")
            .add_child(participant);
        star.add_child(planet);
    }

    galaxy_map.add_child(star);
    game_session.add_child(galaxy_map);
    root.add_child(game_session);

    let mut scenario = SimThingScenarioSpec {
        scenario_id: "intrinsic_owner_compile".into(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 1,
                height: 1,
                occupied_cells: 1,
            },
            map_container_id: map_raw.to_string(),
            placements: vec![SimThingStructuralGridPlacement {
                location_id: "star".into(),
                target_id: "star".into(),
                system_id: 1,
                row: 0,
                col: 0,
                simthing_id_raw: star_raw,
            }],
        },
        links: Vec::new(),
        provenance,
    };
    scenario.sync_sidecar_from_root_metadata();
    scenario
}

#[test]
fn every_owner_resource_scope_bucket_is_bit_exact_on_cpu_and_gpu() {
    let Some(ctx) = gpu_context() else {
        return;
    };
    let (root, rows) = three_owner_tree();
    let mut session = simthing_driver::SpecSessionState::new();
    let plan =
        compile_owner_channel_rf_gpu_proof_plan(&root, &rows, &mut session.persistent_rf_layout)
            .expect("compile");
    let parity = prove_owner_channel_rf_cpu_gpu_parity(&ctx, &plan).expect("parity");
    assert_eq!(parity.bucket_count, plan.reduce_up_report.bucket_count);
    assert!(parity.canonical_bucket_ordering);
    assert!(parity.cpu_gpu_bit_exact);
}

