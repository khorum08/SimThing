use std::collections::HashMap;

use simthing_core::{
    DimensionRegistry, Direction, FissionTemplate, FissionThreshold, Overlay, OverlayId,
    OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta, ReductionRule,
    SimProperty, SimThing, SimThingId, SimThingKind, SimThingKindTag, SoftAggregateGuard,
    SubFieldRole, TransformOp, prepare_fission_clone_sources_for_registry,
};
use simthing_gpu::{
    DIR_DOWNWARD, DIR_UPWARD, SlotAllocator, THRESH_BUF_OUTPUT, THRESH_BUF_VALUES,
    ThresholdRegistration, cpu_oracle_threshold_events,
};
use simthing_sim::{
    AggregateAlertRegistration, SoftAggregateViolation, ThresholdBuilder, ThresholdRegistry,
    ThresholdSemantic, assert_no_hard_trigger_on_soft_aggregate as soft_guard_check,
};

fn weighted_mean_property(
    guard: Option<SoftAggregateGuard>,
) -> (DimensionRegistry, simthing_core::SimPropertyId) {
    let mut registry = DimensionRegistry::new();
    let weight_pid = registry.register(SimProperty::simple("core", "headcount", 0));
    let mut property = SimProperty::simple("tech", "research", 0);
    property.layout.sub_fields[0].reduction_override =
        Some(ReductionRule::WeightedMean { by: weight_pid });
    property.layout.sub_fields[0].soft_aggregate_guard = guard;
    let property_id = registry.register(property);
    (registry, property_id)
}

fn push_hard_output_registration(
    registry: &DimensionRegistry,
    property_id: simthing_core::SimPropertyId,
    owner_id: SimThingId,
    gpu_regs: &mut Vec<ThresholdRegistration>,
    cpu_reg: &mut ThresholdRegistry,
) -> Result<(), SoftAggregateViolation> {
    let semantic = ThresholdSemantic::FissionTrigger {
        sim_thing_id: owner_id,
        property_id,
        template_idx: 0,
    };
    soft_guard_check(
        &semantic,
        property_id,
        &SubFieldRole::Amount,
        THRESH_BUF_OUTPUT,
        registry,
    )?;
    let col = registry
        .column_range(property_id)
        .col_for_role(
            &SubFieldRole::Amount,
            &registry.property(property_id).layout,
        )
        .expect("amount column");
    let event_kind = cpu_reg.push(semantic);
    gpu_regs.push(ThresholdRegistration {
        slot: 0,
        col: col as u32,
        threshold: 0.75,
        direction: DIR_UPWARD,
        event_kind,
        buffer: THRESH_BUF_OUTPUT,
    });
    Ok(())
}

#[test]
fn assert_no_hard_trigger_on_soft_aggregate() {
    let owner_id = SimThing::new(SimThingKind::Owner, 0).id;
    let cases = [
        ("unguarded", None, false),
        (
            "quantized",
            Some(SoftAggregateGuard::Quantized { step: 0.01 }),
            true,
        ),
    ];

    for (name, guard, should_register) in cases {
        let (registry, property_id) = weighted_mean_property(guard);
        let mut gpu_regs = Vec::new();
        let mut cpu_reg = ThresholdRegistry::new();
        let result = push_hard_output_registration(
            &registry,
            property_id,
            owner_id,
            &mut gpu_regs,
            &mut cpu_reg,
        );

        if should_register {
            result.unwrap_or_else(|err| panic!("{name} should allow hard boundary: {err}"));
            assert_eq!(gpu_regs.len(), 1, "{name} should install one hard boundary");
            assert_eq!(gpu_regs[0].buffer, THRESH_BUF_OUTPUT);
            assert!(matches!(
                cpu_reg.get(gpu_regs[0].event_kind),
                Some(ThresholdSemantic::FissionTrigger { property_id: pid, .. }) if *pid == property_id
            ));
        } else {
            assert!(
                matches!(
                    result,
                    Err(SoftAggregateViolation::HardTriggerOnUnguardedSoftAggregate { .. })
                ),
                "{name} should block unguarded soft aggregate hard boundary"
            );
            assert!(
                gpu_regs.is_empty() && cpu_reg.is_empty(),
                "{name} must not install a hard-boundary registration"
            );
        }
    }

    let (registry, property_id) = weighted_mean_property(None);
    let mut root = SimThing::new(SimThingKind::World, 0);
    root.add_property(property_id, registry.property(property_id).default_value());
    let owner_id = root.id;
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let runtime = simthing_sim::SimRuntimeTree::admit(root);
    let aggregate_alert = AggregateAlertRegistration {
        sim_thing_id: owner_id,
        property_id,
        sub_field: SubFieldRole::Amount,
        threshold: 0.75,
        direction: Direction::Rising,
    };
    let (gpu_regs, cpu_reg) = ThresholdBuilder::build_with_alerts(
        &runtime,
        &registry,
        &allocator,
        &[],
        &[aggregate_alert],
    );

    assert_eq!(gpu_regs.len(), 1);
    assert_eq!(gpu_regs[0].buffer, THRESH_BUF_OUTPUT);
    assert!(matches!(
        cpu_reg.get(gpu_regs[0].event_kind),
        Some(ThresholdSemantic::AggregateAlert { property_id: pid, .. }) if *pid == property_id
    ));
}

fn make_fission_property() -> SimProperty {
    let mut property = SimProperty::simple("core", "loyalty", 0);
    property.fission_templates = vec![FissionThreshold {
        sub_field: SubFieldRole::Amount,
        threshold: 0.3,
        direction: Direction::Falling,
        template: FissionTemplate {
            child_kind: SimThingKindTag::Owner,
            fusion_intensity_threshold: 0.8,
            fusion_scar_coefficient: 0.05,
            resolution_label: "schism".into(),
            clone_capability_children: true,
            capability_container_kinds: vec!["tech_tree".into()],
        },
        secondary: None,
    }];
    property
}

#[test]
fn clone_capability_children() {
    let mut registry = DimensionRegistry::new();
    let property_id = registry.register(make_fission_property());
    let layout = registry.property(property_id).layout.clone();
    let amount_offset = layout.offset_of(&SubFieldRole::Amount).expect("amount");

    let mut faction = SimThing::new(SimThingKind::Owner, 0);
    faction.add_property(property_id, registry.property(property_id).default_value());
    let faction_id = faction.id;

    let mut capability_tree = SimThing::new(SimThingKind::Custom("tech_tree".into()), 0);
    capability_tree.add_property(property_id, registry.property(property_id).default_value());
    let source_tree_id = capability_tree.id;
    let source_overlay_id = OverlayId::new();
    capability_tree.add_overlay(Overlay {
        id: source_overlay_id,
        kind: OverlayKind::Policy,
        source: OverlaySource::System,
        affects: vec![faction_id],
        transform: PropertyTransformDelta {
            property_id,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(0.2))],
        },
        lifecycle: OverlayLifecycle::Suspended {
            when_activated: Box::new(OverlayLifecycle::Permanent),
        },
    });
    faction.add_child(capability_tree);

    let mut root = SimThing::new(SimThingKind::Location, 0);
    root.add_child(faction);
    prepare_fission_clone_sources_for_registry(&mut root, &registry);

    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let faction_slot = allocator.slot_of(faction_id).expect("faction slot").raw();
    let source_tree_slot = allocator
        .slot_of(source_tree_id)
        .expect("source tree slot")
        .raw();
    let n_dims = registry.total_columns.max(1);
    let col = amount_offset.lane() as u32;

    let mut previous = vec![0.0; 32 * n_dims];
    let mut values = vec![0.0; 32 * n_dims];
    previous[faction_slot as usize * n_dims + col as usize] = 0.5;
    values[faction_slot as usize * n_dims + col as usize] = 0.25;
    values[source_tree_slot as usize * n_dims + col as usize] = 0.42;

    let mut cpu_reg = ThresholdRegistry::new();
    let event_kind = cpu_reg.push(ThresholdSemantic::FissionTrigger {
        sim_thing_id: faction_id,
        property_id,
        template_idx: 0,
    });
    let regs = [ThresholdRegistration {
        slot: faction_slot,
        col,
        threshold: 0.3,
        direction: DIR_DOWNWARD,
        event_kind,
        buffer: THRESH_BUF_VALUES,
    }];
    let events =
        cpu_oracle_threshold_events(&previous, &values, &previous, &values, n_dims as u32, &regs);
    assert_eq!(events.len(), 1, "fission threshold must fire once");

    let paths = HashMap::from([(faction_id, vec![0])]);
    let outcome = simthing_sim::fission::resolve_fission_fusion(
        &mut root,
        &paths,
        &registry,
        &mut allocator,
        &events,
        &cpu_reg,
        &mut values,
        n_dims,
        1,
    );

    assert_eq!(outcome.fissions_executed, 1);
    assert!(outcome.cloned_capability_subtrees);
    assert_eq!(outcome.cloned_capability_roots.len(), 1);

    let spawned = root.children[0]
        .children
        .iter()
        .find(|child| child.kind == SimThingKind::Owner)
        .expect("spawned fission child");
    assert_ne!(spawned.id, faction_id);

    let cloned_tree = spawned
        .children
        .iter()
        .find(|child| child.kind == SimThingKind::Custom("tech_tree".into()))
        .expect("capability tree cloned through fission template");
    assert_ne!(cloned_tree.id, source_tree_id);
    assert_eq!(cloned_tree.overlays.len(), 1);
    assert_ne!(cloned_tree.overlays[0].id, source_overlay_id);
    assert_eq!(cloned_tree.overlays[0].affects, vec![spawned.id]);

    let clone_record = &outcome.cloned_capability_roots[0];
    assert_eq!(clone_record.spawned_owner_id, spawned.id);
    assert_eq!(clone_record.source_root_id, source_tree_id);
    assert_eq!(clone_record.cloned_root_id, cloned_tree.id);
    assert_eq!(
        clone_record.overlay_id_pairs,
        vec![(source_overlay_id, cloned_tree.overlays[0].id)]
    );

    let cloned_slot = allocator
        .slot_of(cloned_tree.id)
        .expect("cloned tree slot")
        .raw();
    assert_eq!(
        values[cloned_slot as usize * n_dims + col as usize].to_bits(),
        0.42f32.to_bits(),
        "capability subtree shadow row should be copied into the cloned overlay path"
    );
}
