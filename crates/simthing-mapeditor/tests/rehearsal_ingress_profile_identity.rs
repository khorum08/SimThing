//! 0088 C0 escaped-bug witness: native profile pinning and reference-sensitive save refusal.
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use simthing_core::{
    Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta,
    PropertyValue, SimPropertyId, SimThing, SimThingId, SubFieldRole, TransformOp,
};
use simthing_mapeditor::app::StudioAppState;
use simthing_mapeditor::clause_scenario_ingest::{
    load_studio_session_from_clause_ingest_result, ClauseScenarioIngestResult,
};
use simthing_mapeditor::clause_scenario_picker::{
    run_clause_picker_action, ClausePickerActionResult, ClausePickerSelection,
};
use simthing_mapeditor::settings::EditorSettings;
use simthing_mapeditor::studio_live_session_bridge::StudioAuthoredLiveProfile;
use simthing_mapeditor::{
    save_current_session_scenario_to_path, StudioLiveSessionBridge, StudioSession,
};
use simthing_spec::{ArenaSpec, ExplicitParticipantSpec, PropertyKey, ResourceFlowSpec};

const CHILD_OUTPUT: &str = "SIMTHING_REHEARSAL_PROFILE_CHILD_OUTPUT";
const TEST: &str = "rehearsal_ingress_profile_identity_preserves_workload_and_references";

fn open_shipped(directory: &Path) -> (StudioSession, ClauseScenarioIngestResult) {
    let selection = ClausePickerSelection {
        clause_path: Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../scenarios/stellaristhing_base.clause"),
        scenario_json_path: Some(directory.join("source.simthing-scenario.json")),
        ..Default::default()
    };
    match run_clause_picker_action(&selection, None) {
        ClausePickerActionResult::Loaded {
            session, ingest, ..
        } => (session, ingest),
        other => panic!("ordinary native picker: {other:?}"),
    }
}

fn bound_digest(session: &StudioSession) -> String {
    // Read the existing private provenance through its Debug view; do not expose
    // a new production API or reimplement the identity algorithm in this witness.
    let debug = format!("{:?}", session.authored_live_profile.as_ref().unwrap());
    debug
        .rsplit_once("profile_identity: ")
        .unwrap()
        .1
        .split('"')
        .nth(1)
        .unwrap()
        .into()
}

fn observe(session: &StudioSession, ingest: &ClauseScenarioIngestResult) -> serde_json::Value {
    let profile = session.authored_live_profile.as_ref().unwrap();
    let root_id = profile.session_root.id.raw();
    let targets = profile.install_targets.len();
    let cache = ingest.source_cache.as_ref().unwrap();
    let mut app = StudioAppState::default();
    let mut bridge = StudioLiveSessionBridge::default();
    app.try_adopt_loaded_scenario_session(
        session.clone(),
        &mut EditorSettings::default(),
        &mut bridge,
        "identity witness".into(),
    )
    .unwrap();
    let readout = bridge.readout();
    let material: BTreeMap<_, _> = readout
        .field_accretion_samples
        .iter()
        .filter(|sample| sample.tick_index == readout.executed_ticks)
        .map(|sample| (&sample.property_key, sample.amount))
        .collect();
    assert_eq!(readout.executed_ticks, 0);
    assert_eq!(bridge.sim_session().unwrap().coord.day_index(), 0);
    serde_json::json!({
        "pid": std::process::id(), "root_id": root_id,
        "profile_digest": bound_digest(session), "source_identity": cache.source_identity,
        "dependencies": cache.dependencies, "resolver_entries": cache.resolver_entries,
        "install_targets": targets, "material": material,
    })
}

fn visit(node: &mut SimThing, action: &mut impl FnMut(&mut SimThing)) {
    action(node);
    for child in &mut node.children {
        visit(child, action);
    }
}

fn node_mut(node: &mut SimThing, id: SimThingId) -> &mut SimThing {
    if node.id == id {
        return node;
    }
    fn contains(node: &SimThing, id: SimThingId) -> bool {
        node.id == id || node.children.iter().any(|child| contains(child, id))
    }
    let child = node
        .children
        .iter_mut()
        .find(|child| contains(child, id))
        .unwrap();
    node_mut(child, id)
}

fn assert_edited_profile_refuses(
    original: &StudioSession,
    label: &str,
    edit: impl FnOnce(&mut StudioAuthoredLiveProfile),
) {
    let mut edited = original.clone();
    edit(edited.authored_live_profile.as_mut().unwrap());
    assert_eq!(
        serde_json::to_value(&edited.scenario_authority).unwrap(),
        serde_json::to_value(&original.scenario_authority).unwrap()
    );
    let dir = tempfile::tempdir().unwrap();
    let destination = dir.path().join("refused.simthing-scenario.json");
    let error = save_current_session_scenario_to_path(&edited, &destination).unwrap_err();
    // With unchanged document and unchanged bound provenance, this exact refusal
    // proves the newly computed profile digest differs, not merely a loader error.
    assert!(
        error
            .to_string()
            .contains("session differs from its native source"),
        "{label}: {error}"
    );
    assert!(!destination.exists(), "{label} wrote before refusal");
    eprintln!("DIFFERENT_DIGEST / SAVE_REFUSED: {label}");
}

fn assert_equivalent_save(original: &StudioSession, changed: &StudioSession, label: &str) {
    let dir = tempfile::tempdir().unwrap();
    let before = dir.path().join("before.simthing-scenario.json");
    let after = dir.path().join("after.simthing-scenario.json");
    save_current_session_scenario_to_path(original, &before).unwrap();
    save_current_session_scenario_to_path(changed, &after).unwrap();
    assert_eq!(
        std::fs::read(before).unwrap(),
        std::fs::read(after).unwrap()
    );
    eprintln!("SAME_DIGEST / SAVE_ACCEPTED: {label}");
}

#[test]
fn rehearsal_ingress_profile_identity_preserves_workload_and_references() {
    if let Some(output) = std::env::var_os(CHILD_OUTPUT) {
        let output = Path::new(&output);
        let (session, ingest) = open_shipped(output.parent().unwrap());
        std::fs::write(
            output,
            serde_json::to_vec_pretty(&observe(&session, &ingest)).unwrap(),
        )
        .unwrap();
        return;
    }
    let mut reports = Vec::new();
    for repetition in 1..=3 {
        let dir = tempfile::tempdir().unwrap();
        let report = dir.path().join("profile.json");
        let child = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", TEST, "--nocapture"])
            .env(CHILD_OUTPUT, &report)
            .current_dir(dir.path())
            .output()
            .unwrap();
        assert!(
            child.status.success(),
            "fresh {repetition}: {} {}",
            String::from_utf8_lossy(&child.stdout),
            String::from_utf8_lossy(&child.stderr)
        );
        let observed: serde_json::Value =
            serde_json::from_slice(&std::fs::read(report).unwrap()).unwrap();
        eprintln!("FRESH_PROCESS_{repetition}: {observed}");
        reports.push(observed);
    }
    for observed in &reports[1..] {
        for key in [
            "profile_digest",
            "source_identity",
            "dependencies",
            "resolver_entries",
            "install_targets",
            "material",
        ] {
            assert_eq!(reports[0][key], observed[key], "fresh-process {key}");
        }
    }
    let dir = tempfile::tempdir().unwrap();
    let (original, ingest) = open_shipped(dir.path());
    let first = observe(&original, &ingest); // Resident bridge drops before reopen.
    let second_dir = tempfile::tempdir().unwrap();
    let (second, second_ingest) = open_shipped(second_dir.path());
    let reopened = observe(&second, &second_ingest);
    eprintln!("SAME_PROCESS_FIRST: {first}\nSAME_PROCESS_REOPEN: {reopened}");
    assert_ne!(
        first["root_id"], reopened["root_id"],
        "must exercise allocation history"
    );
    assert_eq!(first["profile_digest"], reopened["profile_digest"]);
    assert_eq!(reports[0]["profile_digest"], reopened["profile_digest"]);

    let mut reordered = original.clone();
    let old_wire = serde_json::to_vec(
        &original
            .authored_live_profile
            .as_ref()
            .unwrap()
            .session_root,
    )
    .unwrap();
    for _ in 0..16 {
        visit(
            &mut reordered
                .authored_live_profile
                .as_mut()
                .unwrap()
                .session_root,
            &mut |node| {
                let mut pairs: Vec<_> = node.properties.drain().collect();
                pairs.sort_by_key(|(id, _)| std::cmp::Reverse(*id));
                node.properties = pairs.into_iter().collect::<HashMap<_, _>>();
            },
        );
        if serde_json::to_vec(
            &reordered
                .authored_live_profile
                .as_ref()
                .unwrap()
                .session_root,
        )
        .unwrap()
            != old_wire
        {
            break;
        }
    }
    assert_ne!(
        serde_json::to_vec(
            &reordered
                .authored_live_profile
                .as_ref()
                .unwrap()
                .session_root
        )
        .unwrap(),
        old_wire
    );
    assert_equivalent_save(
        &original,
        &reordered,
        "different property insertion/iteration order",
    );

    assert_edited_profile_refuses(&original, "property value", |profile| {
        let a = profile.install_targets["A1"][0];
        node_mut(&mut profile.session_root, a).add_property(
            SimPropertyId(7_000_001),
            PropertyValue::from_raw_lanes(vec![17.0]),
        );
    });
    assert_edited_profile_refuses(&original, "owner policy", |profile| {
        let policy = profile
            .game_mode
            .overlays
            .iter_mut()
            .find(|o| o.kind == OverlayKind::Policy)
            .unwrap();
        policy.sub_field_deltas[0].1 = TransformOp::multiply(7.0);
    });
    assert_edited_profile_refuses(&original, "install target reference", |profile| {
        profile
            .install_targets
            .insert("A1".into(), profile.install_targets["E1"].clone());
    });
    assert_edited_profile_refuses(&original, "resource parent reference", |profile| {
        let a = profile.install_targets["A1"][0];
        let pirate = profile.install_targets["pirate"][0];
        node_mut(&mut profile.session_root, a).resource_parent_edges[0].parent = pirate;
    });
    assert_edited_profile_refuses(&original, "ordered tree topology", |profile| {
        profile.session_root.children[0].children.swap(0, 1);
    });

    // Exercise reference-bearing forms not present in this shipped slice through
    // the existing hydrated-pack -> profile/provenance boundary (no live effects).
    let mut enriched = ingest.clone();
    let a = enriched.pack.install_targets["A1"][0];
    let e = enriched.pack.install_targets["E1"][0];
    let host = node_mut(&mut enriched.pack.root, a);
    host.add_property(
        SimPropertyId(7_000_002),
        PropertyValue::from_raw_lanes(vec![a.raw() as f32]),
    );
    host.overlays.push(Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Policy,
        source: OverlaySource::Player,
        origin: a,
        affects: vec![a, e],
        transform: PropertyTransformDelta {
            property_id: SimPropertyId(7_000_002),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::multiply(2.0))],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    });
    enriched
        .pack
        .install_targets
        .insert("two_loci".into(), vec![a, e]);
    enriched.pack.game_mode.resource_flow = Some(ResourceFlowSpec {
        arenas: vec![ArenaSpec {
            name: "profile_reference_witness".into(),
            flow_property: PropertyKey::new("meridian", "energy"),
            balance_property: None,
            max_participants: 2,
            max_coupling_fanout: 1,
            max_orderband_depth: 2,
            fission_policy: Default::default(),
            reserved_orderband_depth: 0,
            explicit_participants: vec![
                ExplicitParticipantSpec::flat(0, a.raw()),
                ExplicitParticipantSpec::nested(1, e.raw(), u64::from(a.raw())),
            ],
            enrollment: None,
            wildcard_admission: None,
        }],
        ..Default::default()
    });
    let reference_session = load_studio_session_from_clause_ingest_result(
        &enriched,
        dir.path().join("reference.json"),
        None,
    )
    .unwrap();
    let mut renamed = reference_session.clone();
    let profile = renamed.authored_live_profile.as_mut().unwrap();
    let mut identities = Vec::new();
    visit(&mut profile.session_root, &mut |node| {
        identities.push(node.id)
    });
    let map: BTreeMap<_, _> = identities
        .iter()
        .enumerate()
        .map(|(index, id)| (*id, SimThingId::from_session_raw(1_000_000 - index as u32)))
        .collect();
    visit(&mut profile.session_root, &mut |node| {
        node.id = map[&node.id];
        for edge in &mut node.resource_parent_edges {
            edge.parent = map[&edge.parent];
        }
        for overlay in &mut node.overlays {
            overlay.origin = map[&overlay.origin];
            for target in &mut overlay.affects {
                *target = map[target];
            }
        }
    });
    for members in profile.install_targets.values_mut() {
        for member in members {
            *member = map[member];
        }
    }
    for participant in
        &mut profile.game_mode.resource_flow.as_mut().unwrap().arenas[0].explicit_participants
    {
        participant.subtree_root_id =
            map[&SimThingId::from_session_raw(participant.subtree_root_id)].raw();
        participant.parent_subtree_root_id = participant
            .parent_subtree_root_id
            .map(|id| u64::from(map[&SimThingId::from_session_raw(id as u32)].raw()));
    }
    assert_equivalent_save(
        &reference_session,
        &renamed,
        "bijective reversed-ID renaming of every reference form",
    );
    assert_edited_profile_refuses(
        &reference_session,
        "distinct target references conflated",
        |p| {
            p.install_targets.insert("two_loci".into(), vec![a, a]);
        },
    );
    assert_edited_profile_refuses(&reference_session, "overlay origin", |p| {
        node_mut(&mut p.session_root, a).overlays[0].origin = e;
    });
    assert_edited_profile_refuses(&reference_session, "overlay affected reference", |p| {
        node_mut(&mut p.session_root, a).overlays[0].affects[1] = a;
    });
    assert_edited_profile_refuses(&reference_session, "GameMode participant reference", |p| {
        p.game_mode.resource_flow.as_mut().unwrap().arenas[0].explicit_participants[1]
            .subtree_root_id = a.raw();
    });
    assert_edited_profile_refuses(&reference_session, "GameMode parent reference", |p| {
        p.game_mode.resource_flow.as_mut().unwrap().arenas[0].explicit_participants[1]
            .parent_subtree_root_id = Some(u64::from(e.raw()));
    });
    assert_edited_profile_refuses(
        &reference_session,
        "numeric value equal to a node ID is still a value",
        |p| {
            node_mut(&mut p.session_root, a).add_property(
                SimPropertyId(7_000_002),
                PropertyValue::from_raw_lanes(vec![e.raw() as f32]),
            );
        },
    );

    for duplicate in [false, true] {
        let mut invalid = original.clone();
        let profile = invalid.authored_live_profile.as_mut().unwrap();
        if duplicate {
            profile.session_root.children[0].id = profile.session_root.id;
        } else {
            profile
                .install_targets
                .insert("A1".into(), vec![SimThingId::from_session_raw(u32::MAX)]);
        }
        let destination = dir.path().join("invalid.simthing-scenario.json");
        assert!(save_current_session_scenario_to_path(&invalid, &destination).is_err());
        assert!(!destination.exists());
    }
}
