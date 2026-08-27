use std::collections::{BTreeSet, HashMap};

use simthing_core::owner_channel::OwnerRef;
use simthing_core::{
    prepare_fission_clone_sources_for_registry, DimensionRegistry, Direction, FissionTemplate,
    FissionThreshold, GenerationStamp, Overlay, OverlayId, OverlayKind, OverlayLifecycle,
    OverlaySource, PropertyTransformDelta, ReductionRule, SimProperty, SimThing, SimThingId,
    SimThingKind, SimThingKindTag, SoftAggregateGuard, SubFieldRole, TransformOp,
};
use simthing_gpu::{
    cpu_oracle_threshold_events, SlotAllocator, ThresholdRegistration, DIR_DOWNWARD, DIR_UPWARD,
    THRESH_BUF_OUTPUT, THRESH_BUF_VALUES,
};
use simthing_sim::{
    assert_no_hard_trigger_on_soft_aggregate as soft_guard_check, AggregateAlertRegistration,
    SoftAggregateViolation, ThresholdBuilder, ThresholdRegistry, ThresholdSemantic,
};
use simthing_spec::{
    admit_specialization_flow_market, clear_constrained_claims_at_generation,
    AuthoredClearingProgram, ClearingRemainderAuthority, ConservedOfferingSpec, ConstrainedClaim,
    ConstrainedSupply, DrawEnvelopeTemplateSpec, OfferingPriceVectorSpec, OwnerChannelScopeKey,
    ResourceKey, RuntimeOwnerSiloDemandBucket, ScopeId, SpecializationFlowMarketSpec,
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
        col: col.raw_u32(),
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
    allocator.install_initial_tree(&root);
    let runtime = simthing_sim::SimRuntimeTree::admit(root);
    let aggregate_alert = AggregateAlertRegistration {
        sim_thing_id: owner_id,
        property_id,
        sub_field: SubFieldRole::Amount,
        threshold: 0.75,
        direction: Direction::Rising,
        cost_band: simthing_sim::CostBandSemantic::observation(),
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
        origin: source_tree_id,
        affects: vec![faction_id],
        transform: PropertyTransformDelta {
            property_id,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(0.2))],
        },
        lifecycle: OverlayLifecycle::Suspended {
            when_activated: Box::new(OverlayLifecycle::UntilDissolved),
        },
    });
    faction.add_child(capability_tree);

    let mut root = SimThing::new(SimThingKind::Location, 0);
    root.add_child(faction);
    prepare_fission_clone_sources_for_registry(&mut root, &registry);

    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);
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
    let events = cpu_oracle_threshold_events(
        &previous,
        &values,
        &previous,
        &values,
        n_dims as u32,
        &regs,
        0,
    );
    assert_eq!(events.len(), 1, "fission threshold must fire once");

    let paths = HashMap::from([(faction_id, vec![0])]);
    let generation = GenerationStamp::new(1);
    allocator
        .declare_root_residency_extent(
            root.id,
            simthing_gpu::ResidencyExtent::try_new(
                0,
                u32::try_from(values.len() / n_dims).unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
    let (prepared, initial) = simthing_sim::fission::prepare_fission_growth_candidates(
        &root, &paths, &registry, &allocator, &events, &cpu_reg, &values, n_dims, 1,
    );
    let market = admit_specialization_flow_market(
        &simthing_core::seed_profiles(),
        &BTreeSet::from(["while-resident".into()]),
        SpecializationFlowMarketSpec {
            specialization_profile_id: "session-root".into(),
            offerings: vec![ConservedOfferingSpec {
                id: "residency-claim".into(),
                resource_key: ResourceKey::new("residency-slots"),
                price: OfferingPriceVectorSpec {
                    unit_cost: 1.0,
                    default_clearing_weight: 1.0,
                },
            }],
            draw_envelopes: vec![DrawEnvelopeTemplateSpec {
                id: "residency-draw".into(),
                offering_refs: vec!["residency-claim".into()],
                lifecycle_trigger_refs: vec!["while-resident".into()],
                min_quantity: 1,
                max_quantity: u32::MAX,
            }],
        },
    )
    .expect("test residency market admits");
    let mut schedule = simthing_core::IntegrationSchedule::new();
    let commits = prepared
        .values()
        .map(|prepared| {
            let candidate = prepared.candidate();
            let scope = OwnerChannelScopeKey {
                owner_ref: OwnerRef::new("protected-restore"),
                resource_key: ResourceKey::new("residency-slots"),
                scope_id: ScopeId::from_boundary(root.id),
            };
            let demand = RuntimeOwnerSiloDemandBucket {
                owner_ref: scope.owner_ref.clone(),
                resource_key: scope.resource_key.clone(),
                scope_id: scope.scope_id.clone(),
                requested: candidate.quantity(),
                priority: 0,
                source_simthing_id_raw: Some(candidate.grantee().raw()),
            };
            let claim = ConstrainedClaim::from_runtime_demand(&demand, 1.0).unwrap();
            let cleared = clear_constrained_claims_at_generation(
                &[ConstrainedSupply {
                    scope,
                    available: candidate.quantity(),
                }],
                &[claim],
                &AuthoredClearingProgram::new(TransformOp::set(1.0)),
                ClearingRemainderAuthority {
                    granter: root.id,
                    generation,
                },
            )
            .expect("real constrained clearing mints the test grant");
            let grant = market
                .record_cleared_grant(
                    root.id,
                    "residency-claim",
                    &cleared[0].grants[0],
                    generation,
                    &mut schedule,
                )
                .expect("sealed clearing grant records");
            let provenance = market
                .residency_provenance(&grant)
                .expect("recorded grant projects opaque provenance");
            let entitlement = simthing_gpu::ProvisionalResidencyEntitlement::try_new(
                provenance.granter(),
                provenance.grantee(),
                provenance.stable_key(),
                provenance.quantity(),
                provenance.granted_generation(),
            )
            .unwrap();
            let commit = allocator
                .realize_unattached_growth_residency(
                    entitlement,
                    candidate.structural_parent(),
                    generation,
                    &mut schedule,
                )
                .unwrap();
            (
                candidate.grantee(),
                simthing_sim::VerifiedGrowthResidencyCommit::try_from_market_grant(
                    commit, provenance,
                )
                .expect("opaque market provenance verifies the mutation commit"),
            )
        })
        .collect();
    let outcome = simthing_sim::fission::resolve_prepared_fission_fusion(
        &mut root,
        &paths,
        &registry,
        &mut allocator,
        &events,
        &cpu_reg,
        &mut values,
        n_dims,
        prepared,
        &commits,
        initial,
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
    assert_eq!(cloned_tree.overlays[0].origin, cloned_tree.id);
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
