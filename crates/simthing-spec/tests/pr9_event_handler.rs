use simthing_core::{DimensionRegistry, OverlayId, SimThingId};
use simthing_feeder::BoundaryRequest;
use simthing_spec::{
    CompiledEffect, CompiledTrigger, CooldownSpec, EventKey, EventPriority, ScopeRef,
    ScriptPredicate, ScriptedEventBoundaryContext, ScriptedEventBoundaryHandler,
    ScriptedEventDefinition, ScriptedEventDiagnostic,
};
use std::collections::HashMap;

// ── Fixtures ──────────────────────────────────────────────────────────────────

fn empty_registry() -> DimensionRegistry {
    DimensionRegistry::new()
}

fn predicate_def(
    id: &str,
    predicate: ScriptPredicate,
    effects: Vec<CompiledEffect>,
    cooldown: Option<CooldownSpec>,
    priority: EventPriority,
) -> ScriptedEventDefinition {
    ScriptedEventDefinition {
        id: EventKey::new(id),
        trigger: CompiledTrigger::Predicate(predicate),
        effects,
        cooldown,
        priority,
    }
}

fn run_tick(
    registry: &DimensionRegistry,
    definitions: &[ScriptedEventDefinition],
    shadow: &[f32],
    n_dims: usize,
    current_slot: u32,
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
        &[],
        &mut ScriptedEventBoundaryContext {
            n_dims,
            shadow,
            current_slot,
            slot_to_thing,
            cooldowns,
            requests: &mut requests,
            diagnostics: &mut diagnostics,
        },
    );
    (requests, diagnostics)
}

// ── Acceptance tests ──────────────────────────────────────────────────────────

/// AT-1: A predicate event with `ScriptPredicate::True` emits the expected
/// `BoundaryRequest::Remove` when the target slot is in `slot_to_thing`.
#[test]
fn predicate_true_event_emits_remove_request() {
    let registry = empty_registry();
    let target_id = SimThingId::new();
    let mut slot_to_thing = HashMap::new();
    slot_to_thing.insert(0u32, target_id);

    let defs = vec![predicate_def(
        "remove_event",
        ScriptPredicate::True,
        vec![CompiledEffect::Remove {
            target: ScopeRef::Current,
        }],
        None,
        EventPriority::Normal,
    )];

    let mut cooldowns = HashMap::new();
    let (requests, diagnostics) =
        run_tick(&registry, &defs, &[], 0, 0, &slot_to_thing, &mut cooldowns);

    assert!(diagnostics.is_empty(), "no diagnostics expected");
    assert_eq!(requests.len(), 1);
    match &requests[0] {
        BoundaryRequest::Remove { target } => assert_eq!(*target, target_id),
        other => panic!("expected Remove, got {other:?}"),
    }
}

/// AT-2: A predicate event with `ScriptPredicate::False` emits no requests.
#[test]
fn predicate_false_event_emits_no_requests() {
    let registry = empty_registry();
    let mut slot_to_thing = HashMap::new();
    slot_to_thing.insert(0u32, SimThingId::new());

    let defs = vec![predicate_def(
        "false_event",
        ScriptPredicate::False,
        vec![CompiledEffect::Remove {
            target: ScopeRef::Current,
        }],
        None,
        EventPriority::Normal,
    )];

    let mut cooldowns = HashMap::new();
    let (requests, diagnostics) =
        run_tick(&registry, &defs, &[], 0, 0, &slot_to_thing, &mut cooldowns);

    assert!(diagnostics.is_empty());
    assert!(requests.is_empty());
}

/// AT-3: `ScopeRef::Current` resolves to the `SimThingId` at `current_slot`.
#[test]
fn current_scope_resolves_to_current_slot() {
    let registry = empty_registry();
    let id_slot2 = SimThingId::new();
    let id_slot5 = SimThingId::new();
    let mut slot_to_thing = HashMap::new();
    slot_to_thing.insert(2u32, id_slot2);
    slot_to_thing.insert(5u32, id_slot5);

    let defs = vec![predicate_def(
        "current_scope_event",
        ScriptPredicate::True,
        vec![CompiledEffect::Remove {
            target: ScopeRef::Current,
        }],
        None,
        EventPriority::Normal,
    )];

    let mut cooldowns = HashMap::new();
    // current_slot = 2 → should target id_slot2, not id_slot5
    let (requests, _) = run_tick(&registry, &defs, &[], 0, 2, &slot_to_thing, &mut cooldowns);

    assert_eq!(requests.len(), 1);
    match &requests[0] {
        BoundaryRequest::Remove { target } => assert_eq!(*target, id_slot2),
        other => panic!("expected Remove selection slot 2, got {other:?}"),
    }
}

/// AT-4: `ScopeRef::Slot(n)` resolves to the `SimThingId` stored under key `n`
/// in `slot_to_thing`, regardless of `current_slot`.
#[test]
fn slot_scope_resolves_to_named_slot() {
    let registry = empty_registry();
    let id_slot0 = SimThingId::new();
    let id_slot7 = SimThingId::new();
    let mut slot_to_thing = HashMap::new();
    slot_to_thing.insert(0u32, id_slot0);
    slot_to_thing.insert(7u32, id_slot7);

    let defs = vec![predicate_def(
        "slot_scope_event",
        ScriptPredicate::True,
        // Effect explicitly names slot 7, not current
        vec![CompiledEffect::Remove {
            target: ScopeRef::Slot(7),
        }],
        None,
        EventPriority::Normal,
    )];

    let mut cooldowns = HashMap::new();
    let (requests, _) = run_tick(&registry, &defs, &[], 0, 0, &slot_to_thing, &mut cooldowns);

    assert_eq!(requests.len(), 1);
    match &requests[0] {
        BoundaryRequest::Remove { target } => assert_eq!(*target, id_slot7),
        other => panic!("expected Remove selection slot 7, got {other:?}"),
    }
}

/// AT-5: An effect whose target slot is absent from `slot_to_thing` pushes a
/// diagnostic and emits no `BoundaryRequest` — it does not panic.

/// AT-6: Multiple effects on a single event are emitted in declaration order.
#[test]
fn effect_order_is_preserved() {
    let registry = empty_registry();
    let id_a = SimThingId::new();
    let id_b = SimThingId::new();
    let overlay_id = OverlayId::new();
    let mut slot_to_thing = HashMap::new();
    slot_to_thing.insert(1u32, id_a);
    slot_to_thing.insert(2u32, id_b);

    let defs = vec![predicate_def(
        "ordered_effects_event",
        ScriptPredicate::True,
        vec![
            CompiledEffect::ActivateOverlay {
                target: ScopeRef::Slot(1),
                overlay_id,
            },
            CompiledEffect::SuspendOverlay {
                target: ScopeRef::Slot(2),
                overlay_id,
            },
            CompiledEffect::Remove {
                target: ScopeRef::Slot(1),
            },
        ],
        None,
        EventPriority::Normal,
    )];

    let mut cooldowns = HashMap::new();
    let (requests, diagnostics) =
        run_tick(&registry, &defs, &[], 0, 0, &slot_to_thing, &mut cooldowns);

    assert!(diagnostics.is_empty());
    assert_eq!(requests.len(), 3);
    assert!(
        matches!(&requests[0], BoundaryRequest::ActivateOverlay { target, overlay_id: oid }
            if *target == id_a && *oid == overlay_id),
        "first request should be ActivateOverlay for id_a"
    );
    assert!(
        matches!(&requests[1], BoundaryRequest::SuspendOverlay { target, overlay_id: oid }
            if *target == id_b && *oid == overlay_id),
        "second request should be SuspendOverlay for id_b"
    );
    assert!(
        matches!(&requests[2], BoundaryRequest::Remove { target } if *target == id_a),
        "third request should be Remove for id_a"
    );
}

/// AT-7: A cooldown of N ticks suppresses re-firing for exactly N calls after
/// the initial fire. With `ticks: 2`, the sequence is: fire, skip, fire, skip, ...
#[test]
fn cooldown_suppresses_repeat_firing_until_elapsed() {
    let registry = empty_registry();
    let target_id = SimThingId::new();
    let mut slot_to_thing = HashMap::new();
    slot_to_thing.insert(0u32, target_id);

    let defs = vec![predicate_def(
        "cooldown_event",
        ScriptPredicate::True,
        vec![CompiledEffect::Remove {
            target: ScopeRef::Current,
        }],
        Some(CooldownSpec { ticks: 2 }),
        EventPriority::Normal,
    )];

    let registry_ref = &registry;
    let defs_ref = defs.as_slice();
    let shadow = &[];
    let slot_to_thing = &slot_to_thing;
    let mut cooldowns = HashMap::new();

    let tick = |cooldowns: &mut HashMap<EventKey, u32>| -> usize {
        let handler = ScriptedEventBoundaryHandler {
            registry: registry_ref,
            definitions: defs_ref,
        };
        let mut requests = Vec::new();
        let mut diagnostics = Vec::new();
        handler.handle_tick(
            &[],
            &mut ScriptedEventBoundaryContext {
                n_dims: 0,
                shadow,
                current_slot: 0,
                slot_to_thing,
                cooldowns,
                requests: &mut requests,
                diagnostics: &mut diagnostics,
            },
        );
        requests.len()
    };

    assert_eq!(tick(&mut cooldowns), 1, "tick 1: should fire");
    assert_eq!(tick(&mut cooldowns), 0, "tick 2: on cooldown (1 remaining)");
    assert_eq!(
        tick(&mut cooldowns),
        1,
        "tick 3: cooldown expired, should fire again"
    );
    assert_eq!(tick(&mut cooldowns), 0, "tick 4: on cooldown again");
}

/// AT-8: Events with higher priority fire before lower-priority events —
/// their `BoundaryRequest`s appear earlier in the output even when the
/// low-priority definition was registered first.
#[test]
fn priority_ordering_critical_fires_before_low() {
    let registry = empty_registry();
    let id_low = SimThingId::new();
    let id_critical = SimThingId::new();
    let mut slot_to_thing = HashMap::new();
    slot_to_thing.insert(0u32, id_low);
    slot_to_thing.insert(1u32, id_critical);

    // Deliberately list Low before Critical to prove sorting happens.
    let defs = vec![
        predicate_def(
            "low_event",
            ScriptPredicate::True,
            vec![CompiledEffect::Remove {
                target: ScopeRef::Slot(0),
            }],
            None,
            EventPriority::Low,
        ),
        predicate_def(
            "critical_event",
            ScriptPredicate::True,
            vec![CompiledEffect::Remove {
                target: ScopeRef::Slot(1),
            }],
            None,
            EventPriority::Critical,
        ),
    ];

    let mut cooldowns = HashMap::new();
    let (requests, diagnostics) =
        run_tick(&registry, &defs, &[], 0, 0, &slot_to_thing, &mut cooldowns);

    assert!(diagnostics.is_empty());
    assert_eq!(requests.len(), 2);
    // Critical's request (selection id_critical / slot 1) must come first.
    match &requests[0] {
        BoundaryRequest::Remove { target } => assert_eq!(
            *target, id_critical,
            "first request should be from Critical event"
        ),
        other => panic!("expected Remove, got {other:?}"),
    }
    match &requests[1] {
        BoundaryRequest::Remove { target } => {
            assert_eq!(*target, id_low, "second request should be from Low event")
        }
        other => panic!("expected Remove, got {other:?}"),
    }
}
