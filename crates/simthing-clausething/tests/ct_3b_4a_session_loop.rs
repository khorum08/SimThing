//! CT-3b+4a Line 3: the session loop itself runs the RF-fed heatmap chain.
//! `SimSession::run()` dispatches the arena bands and the mapping step
//! (on-device scatter → stencil → reduce → ai_will_do EML → commitment scan)
//! every tick under the authored profile — no harness driving, no readback
//! in the runtime path — and journals commitment crossings per tick.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_clausething::{hydrate_category_economy_pack, parse_raw_document};
use simthing_core::{DimensionRegistry, SimThing, SimThingId, SimThingKind};
use simthing_driver::{Scenario, SimSession};
use simthing_gpu::SlotAllocator;
use simthing_spec::{ExplicitParticipantSpec, GameModeSpec, MappingExecutionProfile};

const HEADLINE_FIXTURE: &str = include_str!("fixtures/ct3b4a_headline.clause");

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn hydrate() -> simthing_clausething::HydratedCategoryEconomyPack {
    let document = parse_raw_document(HEADLINE_FIXTURE.as_bytes()).expect("parse headline fixture");
    hydrate_category_economy_pack(&document).expect("hydrate headline fixture")
}

fn scenario(game_mode: &GameModeSpec, max_days: u32) -> (Scenario, SimThingId) {
    let mut registry = DimensionRegistry::new();
    for prop in &game_mode.properties {
        simthing_spec::compile_property(prop, &mut registry).expect("register scenario property");
    }
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut farmer = None;
    for i in 0..3 {
        let child = SimThing::new(SimThingKind::Cohort, 0);
        if i == 0 {
            farmer = Some(child.id);
        }
        root.add_child(child);
    }
    let farmer_id = farmer.expect("farmer cohort");
    let mut install_targets = HashMap::new();
    install_targets.insert("farmer".to_string(), vec![farmer_id]);
    (
        Scenario {
            name: "ct3b4a_session_loop".into(),
            ticks_per_day: 1,
            max_days,
            dt: 1.0,
            n_slots: 32,
            registry,
            root,
            shadow_seeds: Vec::new(),
            tick_patches: Vec::new(),
            install_targets,
        },
        farmer_id,
    )
}

fn prepared_game_mode(
    hydrated: &simthing_clausething::HydratedCategoryEconomyPack,
    scenario: &Scenario,
) -> GameModeSpec {
    let mut game_mode = hydrated.game_mode.clone();
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&scenario.root);
    let participants: Vec<_> = scenario
        .root
        .children
        .iter()
        .map(|c| ExplicitParticipantSpec::flat(alloc.slot_of(c.id).unwrap(), c.id.raw()))
        .collect();
    for arena in &mut game_mode.resource_flow.as_mut().unwrap().arenas {
        arena.explicit_participants = participants.clone();
    }
    game_mode.properties.clear();
    game_mode
}

#[test]
fn session_loop_runs_rf_heatmap_and_journals_commitments() {
    let Ok(_probe) = simthing_gpu::GpuContext::new_blocking() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let hydrated = hydrate();
    let (scenario, farmer_id) = scenario(&hydrated.game_mode, 3);
    let game_mode = prepared_game_mode(&hydrated, &scenario);

    let mut session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");
    assert!(
        session.mapping.is_some(),
        "authored profile + binding installs the in-loop mapping state"
    );

    let summary = session.run(3).expect("session run");
    assert_eq!(summary.boundaries_run, 3);
    assert_eq!(
        summary.resource_flow_band_dispatches, summary.ticks_run,
        "RF arena bands dispatch in-loop every tick"
    );
    assert_eq!(
        summary.mapping_ticks, summary.ticks_run,
        "the mapping chain runs in-loop every tick"
    );
    assert!(
        summary.mapping_commitment_events >= 1,
        "the authored ai_will_do threshold fires inside the loop"
    );
    let first = &session.mapping_commitments[0];
    assert_eq!(first.event.event_kind, 7, "authored commitment event kind");
    assert!(
        session
            .mapping_commitments
            .iter()
            .all(|record| record.event.event_kind == 7),
        "journal carries only authored crossings"
    );

    // The authored commitment effect lands once (latch) on the acting
    // SimThing through the real boundary path, and its Permanent overlay
    // transforms the alarm column on subsequent GPU ticks.
    assert_eq!(
        summary.mapping_commitment_effects_applied, 1,
        "once-latched effect applies exactly once across boundaries"
    );
    fn find<'a>(node: &'a SimThing, id: SimThingId) -> Option<&'a SimThing> {
        if node.id == id {
            return Some(node);
        }
        node.children.iter().find_map(|c| find(c, id))
    }
    let farmer = find(&session.proto.root, farmer_id).expect("farmer in tree");
    let commitment_overlays = farmer
        .overlays
        .iter()
        .filter(
            |o| matches!(&o.kind, simthing_core::OverlayKind::Custom(k) if k == "mapping_commitment"),
        )
        .count();
    assert_eq!(commitment_overlays, 1, "exactly one commitment overlay");
    let registry = &session.proto.registry;
    let alarm_id = registry.id_of("simthing", "alarm").expect("alarm property");
    let alarm_col = registry
        .column_range(alarm_id)
        .col_for_role(
            &simthing_core::SubFieldRole::Amount,
            &registry.property(alarm_id).layout,
        )
        .expect("alarm col");
    let farmer_slot = session.proto.allocator.slot_of(farmer_id).expect("slot");
    let values = session.state.read_values();
    let alarm = values[farmer_slot as usize * session.coord.n_dims() as usize + alarm_col];
    assert!(
        alarm > 0.0,
        "commitment overlay transforms the alarm column on GPU ticks, got {alarm}"
    );
}

#[test]
fn session_loop_mapping_stays_off_without_profile_or_binding() {
    let Ok(_probe) = simthing_gpu::GpuContext::new_blocking() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let hydrated = hydrate();
    let (scenario, _farmer_id) = scenario(&hydrated.game_mode, 1);

    // Profile authored Disabled → no mapping state, no mapping ticks,
    // presence of region_fields alone wires nothing.
    let mut disabled = prepared_game_mode(&hydrated, &scenario);
    disabled.mapping_execution_profile = MappingExecutionProfile::Disabled;
    let mut session =
        SimSession::open_from_spec(scenario, &disabled).expect("open disabled profile");
    assert!(session.mapping.is_none(), "default-off preserved");
    let summary = session.run(1).expect("run disabled");
    assert_eq!(summary.mapping_ticks, 0);
    assert_eq!(summary.mapping_commitment_events, 0);
    assert!(session.mapping_commitments.is_empty());

    // Profile on but binding stripped → hard open error, never a silent skip.
    let hydrated2 = hydrate();
    let (scenario2, _) = scenario_fn_again(&hydrated2);
    let mut half_authored = prepared_game_mode(&hydrated2, &scenario2);
    half_authored.region_fields[0].pressure_binding = None;
    let err = SimSession::open_from_spec(scenario2, &half_authored)
        .err()
        .expect("half-authored mapping must fail open");
    assert!(
        format!("{err}").contains("pressure_binding"),
        "error names the missing surface: {err}"
    );
}

fn scenario_fn_again(
    hydrated: &simthing_clausething::HydratedCategoryEconomyPack,
) -> (Scenario, SimThingId) {
    scenario(&hydrated.game_mode, 1)
}
