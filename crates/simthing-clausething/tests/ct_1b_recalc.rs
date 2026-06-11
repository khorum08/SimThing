//! CT-1b recalc stress: a large `triggered_modifier` corpus hydrates to
//! `Suspended` overlays + same-scope GPU threshold registrations; the suite
//! proves RON-baseline identity at scale and measures counts + tick cost.
//!
//! The measurement ladder (`ct_1b_recalc_stress_measurement`) is `#[ignore]`d
//! (run manually for the report); the always-on tests stay small.

use std::collections::HashMap;

use simthing_clausething::{HydratedEntityPack, hydrate_entity_pack, parse_raw_document};
use simthing_core::{
    DimensionRegistry, OverlayKind, OverlayLifecycle, OverlaySource, SimProperty, SimThing,
    SimThingKind, SubFieldRole, TransformOp,
};
use simthing_driver::{Scenario, SimSession, preview_install};
use simthing_gpu::SlotAllocator;
use simthing_spec::spec::domain_pack::DomainPackSpec;
use simthing_spec::spec::effect::EffectSpec;
use simthing_spec::spec::event::EventSpec;
use simthing_spec::spec::install_target::InstallTargetSpec;
use simthing_spec::spec::overlay::OverlaySpec;
use simthing_spec::spec::property::PropertySpec;
use simthing_spec::spec::script::{PropertyKey, ScopeRef};
use simthing_spec::spec::trigger::{TriggerDirection, TriggerSpec};
use simthing_spec::{GameModeSpec, SpecVersion};

const POTENTIAL_THRESHOLD: f32 = 10.0;
const PAYLOAD_MULT: f32 = 1.25;

// ── Corpus generation ─────────────────────────────────────────────────────────

/// ClauseScript corpus: N triggered modifiers, each with its own potential
/// property and its own payload-target property. `drive_first` adds one
/// permanent modifier that pushes `stress_0` over the threshold every tick
/// (the firing smoke); without it every threshold stays armed and unfired
/// (the steady-state recalc measurement).
fn clause_corpus(n: usize, drive_first: bool) -> String {
    let mut out = String::new();
    out.push_str("simthing_ct1b_corpus = {\n");
    out.push_str("    display_name = \"CT-1b Corpus\"\n");
    for i in 0..n {
        out.push_str(&format!(
            "    property = {{\n        id = ct1b_stress_{i}\n        namespace = ct1b\n        name = stress_{i}\n        display_name = \"Stress {i}\"\n        seed_amount = 0.0\n    }}\n"
        ));
        out.push_str(&format!(
            "    property = {{\n        id = ct1b_potency_{i}\n        namespace = ct1b\n        name = potency_{i}\n        display_name = \"Potency {i}\"\n        seed_amount = 40.0\n    }}\n"
        ));
    }
    if drive_first {
        out.push_str(
            "    modifier = {\n        id = stress_driver\n        display_name = \"Stress Driver\"\n        targets_property = ct1b::stress_0\n        amount_add = 12.0\n    }\n",
        );
    }
    for i in 0..n {
        out.push_str(&format!(
            "    triggered_modifier = {{\n        id = tm_{i}\n        potential = {{\n            property = ct1b::stress_{i}\n            at_least = 10.0\n        }}\n        modifier = {{\n            id = tm_{i}_payload\n            display_name = \"TM {i}\"\n            targets_property = ct1b::potency_{i}\n            amount_mult = 1.25\n        }}\n    }}\n"
        ));
    }
    out.push_str("}\n");
    out
}

fn baseline_property(id: &str, name: &str, display_name: &str) -> PropertySpec {
    PropertySpec {
        id: id.into(),
        namespace: "ct1b".into(),
        name: name.into(),
        display_name: display_name.into(),
        description: String::new(),
        sub_fields: Vec::new(),
    }
}

fn baseline_overlay(id: &str, display_name: &str, target: &str, op: TransformOp) -> OverlaySpec {
    OverlaySpec {
        id: id.into(),
        display_name: display_name.into(),
        targets_property: target.into(),
        sub_field_deltas: vec![(SubFieldRole::Amount, op)],
        lifecycle: OverlayLifecycle::Permanent,
        kind: OverlayKind::Policy,
        source: OverlaySource::Player,
        install: InstallTargetSpec::SessionRoot,
    }
}

/// Hand-constructed RON-equivalent baseline: what a RON author would write for
/// the same corpus. Built independently of the hydration path.
fn baseline_pack(n: usize, drive_first: bool) -> DomainPackSpec {
    let mut properties = Vec::new();
    let mut overlays = Vec::new();
    let mut events = Vec::new();
    for i in 0..n {
        properties.push(baseline_property(
            &format!("ct1b_stress_{i}"),
            &format!("stress_{i}"),
            &format!("Stress {i}"),
        ));
        properties.push(baseline_property(
            &format!("ct1b_potency_{i}"),
            &format!("potency_{i}"),
            &format!("Potency {i}"),
        ));
    }
    if drive_first {
        overlays.push(baseline_overlay(
            "stress_driver",
            "Stress Driver",
            "ct1b::stress_0",
            TransformOp::Add(12.0),
        ));
    }
    for i in 0..n {
        let mut payload = baseline_overlay(
            &format!("tm_{i}_payload"),
            &format!("TM {i}"),
            &format!("ct1b::potency_{i}"),
            TransformOp::Multiply(PAYLOAD_MULT),
        );
        payload.lifecycle = OverlayLifecycle::Suspended {
            when_activated: Box::new(OverlayLifecycle::Permanent),
        };
        overlays.push(payload);
        events.push(EventSpec {
            id: format!("tm_{i}"),
            trigger: TriggerSpec::Threshold {
                target: ScopeRef::Current,
                property: PropertyKey::new("ct1b", format!("stress_{i}")),
                role: SubFieldRole::Amount,
                threshold: POTENTIAL_THRESHOLD,
                direction: TriggerDirection::Rising,
            },
            effects: vec![EffectSpec::ActivateOverlayRef {
                target: ScopeRef::Current,
                overlay_ref: format!("tm_{i}_payload"),
            }],
            cooldown: None,
            priority: Default::default(),
            install: InstallTargetSpec::SessionRoot,
        });
    }
    DomainPackSpec {
        id: "simthing_ct1b_corpus".into(),
        display_name: "CT-1b Corpus".into(),
        metadata: Default::default(),
        properties,
        overlays,
        capability_trees: Vec::new(),
        events,
    }
}

/// Measurement baseline: identical property surface, N *permanent* modifiers,
/// zero triggers/events — isolates the marginal cost of the armed-threshold
/// machinery.
fn permanent_only_pack(n: usize) -> DomainPackSpec {
    let mut pack = baseline_pack(n, false);
    pack.events.clear();
    for overlay in &mut pack.overlays {
        overlay.lifecycle = OverlayLifecycle::Permanent;
    }
    pack
}

// ── Harness ───────────────────────────────────────────────────────────────────

fn hydrate(text: &str) -> HydratedEntityPack {
    let document = parse_raw_document(text.as_bytes()).expect("parse corpus");
    hydrate_entity_pack(&document).expect("hydrate corpus")
}

fn canonical_json(pack: &DomainPackSpec) -> String {
    serde_json::to_string(pack).expect("serialize domain pack")
}

fn ct1b_scenario(ticks_per_day: u32, max_days: u32) -> Scenario {
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_placeholder", "seed", 0));
    Scenario {
        name: "ct1b_recalc".into(),
        ticks_per_day,
        max_days,
        dt: 1.0,
        n_slots: 16,
        registry,
        root: SimThing::new(SimThingKind::World, 0),
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    }
}

fn game_mode_with_pack(pack: DomainPackSpec) -> GameModeSpec {
    GameModeSpec {
        id: "ct1b_recalc".into(),
        display_name: "CT-1b Recalc Stress".into(),
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

fn lifecycle_counts(root: &SimThing) -> (usize, usize) {
    let suspended = root
        .overlays
        .iter()
        .filter(|o| matches!(o.lifecycle, OverlayLifecycle::Suspended { .. }))
        .count();
    let permanent = root
        .overlays
        .iter()
        .filter(|o| matches!(o.lifecycle, OverlayLifecycle::Permanent))
        .count();
    (suspended, permanent)
}

// ── Always-on: authoring identity + counts (CPU only) ─────────────────────────

#[test]
fn hydrated_corpus_matches_ron_baseline() {
    for n in [16, 256] {
        let hydrated = hydrate(&clause_corpus(n, false));
        let baseline = baseline_pack(n, false);
        assert_eq!(
            canonical_json(&hydrated.domain_pack),
            canonical_json(&baseline),
            "hydrated corpus must match the RON-equivalent baseline at N={n}"
        );

        // The baseline is genuinely RON-authorable: serialize → parse → identical.
        let ron_text = ron::ser::to_string(&baseline).expect("serialize baseline RON");
        let reparsed: DomainPackSpec = ron::from_str(&ron_text).expect("parse baseline RON");
        assert_eq!(canonical_json(&baseline), canonical_json(&reparsed));
    }
}

#[test]
fn corpus_counts_through_preview_install() {
    let n = 16;
    let hydrated = hydrate(&clause_corpus(n, false));
    let baseline = baseline_pack(n, false);

    let scenario = ct1b_scenario(1, 1);
    let allocator = SlotAllocator::new();
    let preview_clause = preview_install(
        &game_mode_with_pack(hydrated.domain_pack),
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect("preview hydrated corpus");
    let preview_ron = preview_install(
        &game_mode_with_pack(baseline),
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect("preview RON baseline");

    for (label, preview) in [("clause", &preview_clause), ("ron", &preview_ron)] {
        let (suspended, permanent) = lifecycle_counts(&preview.root);
        assert_eq!(suspended, n, "{label}: one Suspended overlay per tm");
        assert_eq!(
            permanent, 0,
            "{label}: no permanent overlays in armed corpus"
        );
        assert_eq!(
            preview.state.scripted_event_trigger_registrations().len(),
            n,
            "{label}: one GPU threshold registration per tm"
        );
        assert_eq!(
            preview.registry.properties.len(),
            2 * n + 1,
            "{label}: 2N corpus properties + placeholder"
        );
    }
    assert_eq!(
        preview_clause.registry.total_columns, preview_ron.registry.total_columns,
        "column counts must match between paths"
    );
}

#[test]
fn unknown_overlay_ref_is_hard_install_error() {
    let mut pack = baseline_pack(1, true);
    // Keep the unrelated driver overlay (so install proceeds past overlay
    // setup) but drop the payload the event references.
    pack.overlays
        .retain(|overlay| overlay.id == "stress_driver");
    let scenario = ct1b_scenario(1, 1);
    let allocator = SlotAllocator::new();
    let err = preview_install(
        &game_mode_with_pack(pack),
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect_err("dangling overlay ref must fail install");
    assert!(
        err.to_string().contains("tm_0_payload"),
        "expected unknown-overlay-ref error, got: {err}"
    );
}

// ── GPU: the consumer actually runs ───────────────────────────────────────────

#[test]
fn triggered_modifier_fires_and_activates_overlay() {
    let n = 4;
    let hydrated = hydrate(&clause_corpus(n, true));
    // ticks_per_day = 1: the boundary consumes the firing tick's events
    // directly (threshold events are per-tick readback, not day-latched).
    let scenario = ct1b_scenario(1, 3);
    let mut session =
        match SimSession::open_from_spec(scenario, &game_mode_with_pack(hydrated.domain_pack)) {
            Ok(session) => session,
            Err(err) => {
                eprintln!("skipping: no GPU session ({err})");
                return;
            }
        };

    let (suspended_before, permanent_before) = lifecycle_counts(&session.proto.root);
    assert_eq!(suspended_before, n);
    assert_eq!(permanent_before, 1, "the stress driver modifier");

    let summary = session.run(3).expect("session run");
    assert!(summary.ticks_run > 0);

    let registry = &session.proto.registry;
    let stress_id = registry.id_of("ct1b", "stress_0").expect("stress_0");
    let stress_col = registry
        .column_range(stress_id)
        .col_for_role(&SubFieldRole::Amount, &registry.property(stress_id).layout)
        .expect("stress_0 amount col");
    let root_slot = session
        .proto
        .allocator
        .slot_of(session.proto.root.id)
        .expect("root slot");
    let values = session.state.read_values();
    let stress_value = values[root_slot as usize * session.coord.n_dims() as usize + stress_col];
    assert!(
        stress_value >= POTENTIAL_THRESHOLD,
        "stress driver must push stress_0 over the potential threshold, got {stress_value}"
    );

    let (suspended_after, permanent_after) = lifecycle_counts(&session.proto.root);
    assert_eq!(
        suspended_after,
        n - 1,
        "exactly tm_0 must activate (stress_0 crossed; others stayed at 0): {:?}",
        session.spec_state.handler_errors
    );
    assert_eq!(permanent_after, 2, "stress driver + activated tm_0 payload");
}

// ── Ignored: the measurement ladder (run manually for the report) ─────────────

#[test]
#[ignore = "CT-1b measurement ladder; run manually on a GPU host for the report"]
fn ct_1b_recalc_stress_measurement() {
    eprintln!(
        "| corpus | N | columns | thresholds | suspended | ticks | tick total ms | gpu pipeline ms | boundary total ms | us/tick |"
    );
    eprintln!("|---|---|---|---|---|---|---|---|---|---|");
    for &n in &[16usize, 64, 256] {
        for (label, pack) in [
            ("triggered", baseline_pack(n, false)),
            ("permanent", permanent_only_pack(n)),
        ] {
            let scenario = ct1b_scenario(10, 10);
            let mut session = match SimSession::open_from_spec(scenario, &game_mode_with_pack(pack))
            {
                Ok(session) => session,
                Err(err) => {
                    eprintln!("skipping measurement: no GPU session ({err})");
                    return;
                }
            };
            let columns = session.proto.registry.total_columns;
            let thresholds = session
                .spec_state
                .scripted_event_trigger_registrations()
                .len();
            let (suspended, _) = lifecycle_counts(&session.proto.root);
            let summary = session.run(10).expect("measurement run");
            let per_tick_us = summary.tick_total_ms * 1000.0 / summary.ticks_run.max(1) as f64;
            eprintln!(
                "| {label} | {n} | {columns} | {thresholds} | {suspended} | {} | {:.3} | {:.3} | {:.3} | {:.1} |",
                summary.ticks_run,
                summary.tick_total_ms,
                summary.tick_gpu_pipeline_ms,
                summary.boundary_total_ms,
                per_tick_us,
            );
        }
    }
}
