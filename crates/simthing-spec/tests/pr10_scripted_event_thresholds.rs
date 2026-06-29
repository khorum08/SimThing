//! PR 10 acceptance tests: scripted-event threshold trigger path.
//!
//! Covers:
//! - `ScriptedEventDefinition::to_trigger_registration` conversion
//! - `simthing_sim::ThresholdBuilder::build_with_scripted_event_triggers` GPU + CPU output
//! - `simthing_sim::ThresholdRegistry::extract_scripted_event_triggers` filtering
//! - `ScriptedEventBoundaryHandler::handle_tick` threshold path (cooldown + priority + unknown-id)

use simthing_core::{
    DimensionRegistry, Direction, SimThing, SimThingId, SimThingKind, SubFieldRole,
};
use simthing_feeder::{
    BoundaryRequest, ScriptedEventTriggerEvent, ScriptedEventTriggerRegistration,
};
use simthing_gpu::{SlotAllocator, ThresholdEvent, DIR_DOWNWARD, DIR_UPWARD, THRESH_BUF_VALUES};
use simthing_sim::{SimRuntimeTree, ThresholdBuilder, ThresholdRegistry, ThresholdSemantic};
use simthing_spec::{
    compile_property, CompiledEffect, CompiledThresholdTrigger, CompiledTrigger, CooldownSpec,
    EventKey, EventPriority, PropertySpec, ScopeRef, ScriptPredicate, ScriptedEventBoundaryContext,
    ScriptedEventBoundaryHandler, ScriptedEventDefinition, ScriptedEventDiagnostic,
    ScriptedEventDiagnosticKind, TriggerDirection,
};
use std::collections::HashMap;

// ── Fixtures ──────────────────────────────────────────────────────────────────

fn registry_with_loyalty() -> DimensionRegistry {
    let mut registry = DimensionRegistry::new();
    compile_property(
        &PropertySpec {
            id: "core_loyalty".into(),
            namespace: "core".into(),
            name: "loyalty".into(),
            display_name: "Loyalty".into(),
            description: String::new(),
            sub_fields: vec![],
        },
        &mut registry,
    )
    .expect("loyalty property");
    registry
}

fn threshold_event_def(
    id: &str,
    trigger: CompiledThresholdTrigger,
    effects: Vec<CompiledEffect>,
    cooldown: Option<CooldownSpec>,
    priority: EventPriority,
) -> ScriptedEventDefinition {
    ScriptedEventDefinition {
        id: EventKey::new(id),
        trigger: CompiledTrigger::Threshold(trigger),
        effects,
        cooldown,
        priority,
    }
}

// ── AT-1: to_trigger_registration produces a correct registration ─────────────

#[test]
fn to_trigger_registration_resolves_slot_role_threshold_and_direction() {
    let trigger = CompiledThresholdTrigger {
        target: ScopeRef::Slot(7),
        property: simthing_core::SimPropertyId(0),
        role: SubFieldRole::Amount,
        col: 2,
        threshold: 0.25,
        direction: TriggerDirection::Falling,
    };
    let def = threshold_event_def(
        "low_loyalty",
        trigger,
        vec![CompiledEffect::Remove {
            target: ScopeRef::Current,
        }],
        None,
        EventPriority::Normal,
    );

    let registration = def.to_trigger_registration(99).expect("threshold trigger");

    assert_eq!(registration.event_id, "low_loyalty");
    assert_eq!(registration.slot, 7); // explicit Slot(7) wins over current_slot=99
    assert_eq!(registration.col, 2);
    assert_eq!(registration.threshold, 0.25);
    assert_eq!(registration.direction, Direction::Falling);
}

#[test]
fn to_trigger_registration_resolves_current_to_supplied_slot() {
    let trigger = CompiledThresholdTrigger {
        target: ScopeRef::Current,
        property: simthing_core::SimPropertyId(0),
        role: SubFieldRole::Amount,
        col: 0,
        threshold: 1.0,
        direction: TriggerDirection::Rising,
    };
    let def = threshold_event_def("self_event", trigger, vec![], None, EventPriority::Normal);

    let registration = def.to_trigger_registration(42).expect("threshold trigger");
    assert_eq!(registration.slot, 42);
    assert_eq!(registration.direction, Direction::Rising);
}

#[test]
fn to_trigger_registration_returns_none_for_predicate_triggers() {
    let def = ScriptedEventDefinition {
        id: EventKey::new("predicate_event"),
        trigger: CompiledTrigger::Predicate(ScriptPredicate::True),
        effects: vec![],
        cooldown: None,
        priority: EventPriority::Normal,
    };
    assert!(def.to_trigger_registration(0).is_none());
}

// ── AT-2: ThresholdBuilder produces parallel GPU + CPU registrations ──────────

#[test]
fn threshold_builder_emits_parallel_scripted_event_trigger_entry() {
    let registry = registry_with_loyalty();
    let allocator = SlotAllocator::new();
    let root = SimRuntimeTree::admit(SimThing::new(SimThingKind::World, 0));

    let triggers = vec![ScriptedEventTriggerRegistration {
        event_id: "rebellion_warning".into(),
        slot: 3,
        col: 1,
        threshold: 0.1,
        direction: Direction::Falling,
    }];

    let (gpu, cpu) = ThresholdBuilder::build_with_scripted_event_triggers(
        &root,
        &registry,
        &allocator,
        &[],
        &triggers,
    );

    assert_eq!(gpu.len(), 1);
    assert_eq!(cpu.len(), 1);
    let reg = &gpu[0];
    assert_eq!(reg.slot, 3);
    assert_eq!(reg.col, 1);
    assert_eq!(reg.threshold, 0.1);
    assert_eq!(reg.direction, DIR_DOWNWARD);
    assert_eq!(reg.buffer, THRESH_BUF_VALUES);

    match cpu.get(reg.event_kind) {
        Some(ThresholdSemantic::ScriptedEventTrigger { event_id }) => {
            assert_eq!(event_id, "rebellion_warning");
        }
        other => panic!("expected ScriptedEventTrigger arm, got {other:?}"),
    }
}

#[test]
fn threshold_builder_handles_rising_direction() {
    let registry = registry_with_loyalty();
    let allocator = SlotAllocator::new();
    let root = SimRuntimeTree::admit(SimThing::new(SimThingKind::World, 0));

    let triggers = vec![ScriptedEventTriggerRegistration {
        event_id: "loyalty_climb".into(),
        slot: 0,
        col: 0,
        threshold: 0.9,
        direction: Direction::Rising,
    }];

    let (gpu, _cpu) = ThresholdBuilder::build_with_scripted_event_triggers(
        &root,
        &registry,
        &allocator,
        &[],
        &triggers,
    );
    assert_eq!(gpu[0].direction, DIR_UPWARD);
}

// ── AT-3: extract_scripted_event_triggers filters and resolves correctly ──────

#[test]
fn extract_scripted_event_triggers_filters_to_scripted_arms() {
    let id = SimThingId::new();
    let mut cpu = ThresholdRegistry::new();
    let ek_scripted = cpu.push(ThresholdSemantic::ScriptedEventTrigger {
        event_id: "alpha".into(),
    });
    let ek_capability = cpu.push(ThresholdSemantic::CapabilityUnlock {
        sim_thing_id: id,
        property_id: simthing_core::SimPropertyId(0),
        sub_field: SubFieldRole::Amount,
    });
    let ek_scripted2 = cpu.push(ThresholdSemantic::ScriptedEventTrigger {
        event_id: "beta".into(),
    });

    let events = vec![
        ThresholdEvent::from_boundary_delivery(0, 0, 1.0, ek_scripted),
        ThresholdEvent::from_boundary_delivery(1, 1, 2.0, ek_capability),
        ThresholdEvent::from_boundary_delivery(2, 2, 3.0, ek_scripted2),
        ThresholdEvent::from_boundary_delivery(3, 3, 4.0, 99), // out of range
    ];

    let resolved = cpu.extract_scripted_event_triggers(&events);

    assert_eq!(resolved.len(), 2);
    assert_eq!(resolved[0].event_id, "alpha");
    assert_eq!(resolved[1].event_id, "beta");
}

// ── AT-4: Handler fires threshold events under cooldown/priority gating ──────

fn run_tick_with_thresholds(
    registry: &DimensionRegistry,
    definitions: &[ScriptedEventDefinition],
    threshold_events: &[ScriptedEventTriggerEvent],
    slot_to_thing: &HashMap<u32, SimThingId>,
    cooldowns: &mut HashMap<EventKey, u32>,
) -> (Vec<BoundaryRequest>, Vec<ScriptedEventDiagnostic>) {
    let handler = ScriptedEventBoundaryHandler {
        registry,
        definitions,
    };
    let mut requests = Vec::new();
    let mut diagnostics = Vec::new();
    handler.handle_tick(
        threshold_events,
        &mut ScriptedEventBoundaryContext {
            n_dims: 0,
            shadow: &[],
            current_slot: 0,
            slot_to_thing,
            cooldowns,
            requests: &mut requests,
            diagnostics: &mut diagnostics,
        },
    );
    (requests, diagnostics)
}

#[test]
fn threshold_fired_event_emits_effect_request() {
    let registry = registry_with_loyalty();
    let target_id = SimThingId::new();
    let mut slot_to_thing = HashMap::new();
    slot_to_thing.insert(0u32, target_id);

    let defs = vec![threshold_event_def(
        "low_loyalty",
        CompiledThresholdTrigger {
            target: ScopeRef::Current,
            property: simthing_core::SimPropertyId(0),
            role: SubFieldRole::Amount,
            col: 0,
            threshold: 0.25,
            direction: TriggerDirection::Falling,
        },
        vec![CompiledEffect::Remove {
            target: ScopeRef::Current,
        }],
        None,
        EventPriority::Normal,
    )];

    let events = vec![ScriptedEventTriggerEvent {
        event_id: "low_loyalty".into(),
    }];
    let mut cooldowns = HashMap::new();

    let (requests, diagnostics) =
        run_tick_with_thresholds(&registry, &defs, &events, &slot_to_thing, &mut cooldowns);

    assert!(diagnostics.is_empty(), "no diagnostics");
    assert_eq!(requests.len(), 1);
    match &requests[0] {
        BoundaryRequest::Remove { target } => assert_eq!(*target, target_id),
        other => panic!("expected Remove, got {other:?}"),
    }
}

#[test]
fn threshold_event_with_no_matching_definition_produces_unknown_event_id_diagnostic() {
    let registry = registry_with_loyalty();
    let slot_to_thing = HashMap::new();
    // Definitions list is empty; the fired threshold event references nothing.
    let defs: Vec<ScriptedEventDefinition> = vec![];
    let events = vec![ScriptedEventTriggerEvent {
        event_id: "ghost_event".into(),
    }];
    let mut cooldowns = HashMap::new();

    let (requests, diagnostics) =
        run_tick_with_thresholds(&registry, &defs, &events, &slot_to_thing, &mut cooldowns);

    assert!(requests.is_empty());
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].event_id.0, "ghost_event");
    assert!(matches!(
        diagnostics[0].kind,
        ScriptedEventDiagnosticKind::UnknownEventId
    ));
}

#[test]
fn threshold_path_respects_cooldown_armed_by_predicate_path() {
    // An event with a Predicate trigger and a Threshold trigger can't exist
    // (one trigger per definition), but cooldown is shared across all events
    // by EventKey. This test fires the same event_id twice via threshold path:
    // first call fires + arms cooldown, second call (with active cooldown)
    // skips even though the same threshold event is supplied.
    let registry = registry_with_loyalty();
    let target_id = SimThingId::new();
    let mut slot_to_thing = HashMap::new();
    slot_to_thing.insert(0u32, target_id);

    let defs = vec![threshold_event_def(
        "cooled_event",
        CompiledThresholdTrigger {
            target: ScopeRef::Current,
            property: simthing_core::SimPropertyId(0),
            role: SubFieldRole::Amount,
            col: 0,
            threshold: 0.5,
            direction: TriggerDirection::Falling,
        },
        vec![CompiledEffect::Remove {
            target: ScopeRef::Current,
        }],
        Some(CooldownSpec { ticks: 3 }),
        EventPriority::Normal,
    )];

    let events = vec![ScriptedEventTriggerEvent {
        event_id: "cooled_event".into(),
    }];
    let mut cooldowns = HashMap::new();

    let (req1, _) =
        run_tick_with_thresholds(&registry, &defs, &events, &slot_to_thing, &mut cooldowns);
    assert_eq!(req1.len(), 1, "tick 1: should fire");

    let (req2, _) =
        run_tick_with_thresholds(&registry, &defs, &events, &slot_to_thing, &mut cooldowns);
    assert!(
        req2.is_empty(),
        "tick 2: on cooldown, threshold event must NOT re-fire"
    );
}

#[test]
fn threshold_and_predicate_definitions_compete_under_priority_ordering() {
    // Critical-priority threshold-triggered event must fire before
    // Low-priority predicate event, even if the Low predicate is also true.
    let registry = registry_with_loyalty();
    let id_a = SimThingId::new();
    let id_b = SimThingId::new();
    let mut slot_to_thing = HashMap::new();
    slot_to_thing.insert(1u32, id_a);
    slot_to_thing.insert(2u32, id_b);

    let critical_threshold = threshold_event_def(
        "crit_threshold",
        CompiledThresholdTrigger {
            target: ScopeRef::Slot(1),
            property: simthing_core::SimPropertyId(0),
            role: SubFieldRole::Amount,
            col: 0,
            threshold: 0.5,
            direction: TriggerDirection::Rising,
        },
        vec![CompiledEffect::Remove {
            target: ScopeRef::Slot(1),
        }],
        None,
        EventPriority::Critical,
    );
    let low_predicate = ScriptedEventDefinition {
        id: EventKey::new("low_predicate"),
        trigger: CompiledTrigger::Predicate(ScriptPredicate::True),
        effects: vec![CompiledEffect::Remove {
            target: ScopeRef::Slot(2),
        }],
        cooldown: None,
        priority: EventPriority::Low,
    };

    // Order list with Low first to prove sorting actually runs.
    let defs = vec![low_predicate, critical_threshold];

    let events = vec![ScriptedEventTriggerEvent {
        event_id: "crit_threshold".into(),
    }];
    let mut cooldowns = HashMap::new();

    let (requests, diagnostics) =
        run_tick_with_thresholds(&registry, &defs, &events, &slot_to_thing, &mut cooldowns);

    assert!(diagnostics.is_empty());
    assert_eq!(requests.len(), 2);
    // Critical (threshold-triggered, selection id_a) must come first.
    match &requests[0] {
        BoundaryRequest::Remove { target } => assert_eq!(
            *target, id_a,
            "Critical-priority threshold event should fire first"
        ),
        other => panic!("expected Remove selection id_a, got {other:?}"),
    }
    match &requests[1] {
        BoundaryRequest::Remove { target } => assert_eq!(
            *target, id_b,
            "Low-priority predicate event should fire second"
        ),
        other => panic!("expected Remove selection id_b, got {other:?}"),
    }
}

#[test]
fn threshold_event_not_fired_when_definition_has_threshold_trigger_but_no_event() {
    // A definition with CompiledTrigger::Threshold but NO matching
    // threshold_event this tick must not fire (the GPU has not fired it).
    let registry = registry_with_loyalty();
    let target_id = SimThingId::new();
    let mut slot_to_thing = HashMap::new();
    slot_to_thing.insert(0u32, target_id);

    let defs = vec![threshold_event_def(
        "not_fired",
        CompiledThresholdTrigger {
            target: ScopeRef::Current,
            property: simthing_core::SimPropertyId(0),
            role: SubFieldRole::Amount,
            col: 0,
            threshold: 0.5,
            direction: TriggerDirection::Rising,
        },
        vec![CompiledEffect::Remove {
            target: ScopeRef::Current,
        }],
        None,
        EventPriority::Normal,
    )];

    let mut cooldowns = HashMap::new();
    let (requests, diagnostics) =
        run_tick_with_thresholds(&registry, &defs, &[], &slot_to_thing, &mut cooldowns);

    assert!(
        requests.is_empty(),
        "threshold event must not fire without a matching GPU event"
    );
    assert!(diagnostics.is_empty());
}
