use simthing_core::{
    ColumnIndex, DimensionRegistry, EmitOnThresholdBuffer, EmitOnThresholdRegistration,
    EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu,
    EmlTreeId, SimProperty, SimThing, SimThingKind, SlotIndex, SubFieldRole, ThresholdDirection,
};
use simthing_spec::{
    ActionBandAdmissionBudgetSpec, ActionBandAdmissionError, ActionBandBandSpec,
    ActionBandChannelBindingSpec, ActionBandChannelKind, ActionBandRequirementSemantics,
    ActionBandSessionBuildDoor, ActionBandSessionSpec, ActionBandTargetSpec,
    ActionBandTemplateSpec, ActionBandVelocitySpec, ScalarBoundDirection,
};

fn registry_and_threshold() -> (
    DimensionRegistry,
    Vec<EmitOnThresholdRegistration>,
    ColumnIndex,
) {
    let mut registry = DimensionRegistry::new();
    let property_id = registry.register(SimProperty::simple("synthetic", "action_axis", 1));
    let column = registry
        .column_range(property_id)
        .col_for_role(
            &SubFieldRole::Amount,
            &registry.property(property_id).layout,
        )
        .expect("amount column");
    let thresholds = vec![EmitOnThresholdRegistration {
        slot: SlotIndex::new(0),
        col: column,
        threshold: 1.0,
        direction: ThresholdDirection::Upward,
        event_kind: 41,
        buffer: EmitOnThresholdBuffer::Values,
    }];
    (registry, thresholds, column)
}

fn eml_registry() -> EmlExpressionRegistry {
    let mut registry = EmlExpressionRegistry::new();
    for id in 0..3 {
        let tree_id = EmlTreeId(id);
        registry
            .register_formula(
                tree_id,
                EmlFormulaMeta {
                    tree_id,
                    execution_class: EmlExecutionClass::ExactDeterministic,
                    allowed_consumers: EmlConsumerMask(EmlConsumerMask::ALL_PRODUCTION),
                    max_abs_error: None,
                    deterministic_gpu: true,
                    requires_guard_for_hard_threshold: false,
                    node_count: 1,
                    max_stack_depth: 1,
                    has_loops: false,
                    has_recursion: false,
                    display_name: format!("actionband-test-{id}"),
                },
                vec![EmlNodeGpu {
                    opcode: simthing_core::eml_opcode::LITERAL_F32,
                    flags: 0,
                    a: 1.0_f32.to_bits(),
                    b: 0,
                    c: 0,
                    d: 0,
                }],
            )
            .expect("exact deterministic EML registration");
    }
    registry
}

fn channel(column: u32, kind: ActionBandChannelKind) -> ActionBandChannelBindingSpec {
    ActionBandChannelBindingSpec { column, kind }
}

fn base_template(column: u32) -> ActionBandTemplateSpec {
    ActionBandTemplateSpec {
        id: "synthetic-root".into(),
        label: Some("Readable diagnostic designation".into()),
        axis_channels: vec![channel(column, ActionBandChannelKind::Primitive)],
        target: ActionBandTargetSpec::ScalarBound {
            channel: column,
            bound: 2.0,
            direction: ScalarBoundDirection::AtLeast,
        },
        velocity: None,
        bands: vec![ActionBandBandSpec {
            threshold_registration_index: 0,
            eml_program: Some(0),
            emission_binding_indices: vec![0],
        }],
        subordinate_template_ids: vec![],
        max_active_subordinates: 0,
        reserved_instance_rows: 1,
        requirement_semantics: ActionBandRequirementSemantics::Ordinary,
    }
}

fn session_spec(template: ActionBandTemplateSpec) -> ActionBandSessionSpec {
    ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 4,
            dependency_binding_count: 4,
            storage_rows: 8,
            eml_program_count: 4,
            emission_binding_count: 4,
        },
        templates: vec![template],
    }
}

#[test]
fn depth_one_template_binds_the_existing_sealed_crossing_path() {
    let (registry, thresholds, column) = registry_and_threshold();
    let eml = eml_registry();
    let mut door = ActionBandSessionBuildDoor::new();
    let product = door
        .admit_once_at_session_build(
            &session_spec(base_template(column.raw_u32())),
            &registry,
            &eml,
            &thresholds,
        )
        .expect("depth-1 template admits");

    let gpu_thresholds = simthing_gpu::emit_on_threshold_registrations_to_gpu(&thresholds);
    let root = SimThing::new(SimThingKind::GameSession, 0);
    let mut allocator = simthing_gpu::SlotAllocator::new();
    allocator.install_initial_tree(&root);
    let n_dims = registry.total_columns as u32;
    let mut previous = vec![0.0; n_dims as usize];
    let mut current = vec![0.0; n_dims as usize];
    previous[column.raw()] = 0.5;
    current[column.raw()] = 1.5;
    let deltas = simthing_gpu::cpu_oracle_band_crossing_deltas(
        &previous,
        &current,
        &[],
        &[],
        n_dims,
        &gpu_thresholds,
        &registry,
        &allocator,
    );
    assert_eq!(deltas.len(), 1, "existing fused crossing path emits once");

    let binding = product
        .bindings_for_existing_threshold(deltas[0].reg_idx())
        .next()
        .expect("sealed delta resolves opaque ActionBand metadata");
    assert_eq!(binding.threshold_registration().raw(), 0);
    assert_eq!(binding.template().raw(), 0);
    assert_eq!(product.templates().len(), 1);
    assert_eq!(product.bands().len(), 1);
}

#[test]
fn closed_targets_admit_total_forms_and_reject_predicate_only_or_unretained_velocity() {
    let (mut registry, thresholds, column) = registry_and_threshold();
    let eml = eml_registry();
    let previous_property_id =
        registry.register(SimProperty::simple("synthetic", "previous_action_axis", 1));
    let previous_column = registry
        .column_range(previous_property_id)
        .col_for_role(
            &SubFieldRole::Amount,
            &registry.property(previous_property_id).layout,
        )
        .expect("previous-generation amount column");
    let second_column = previous_column.raw_u32();
    let forms = vec![
        ActionBandTargetSpec::Point {
            current_channels: vec![column.raw_u32(), second_column],
            target: vec![1.0, -1.0],
        },
        ActionBandTargetSpec::ScalarBound {
            channel: column.raw_u32(),
            bound: 1.0,
            direction: ScalarBoundDirection::AtMost,
        },
        ActionBandTargetSpec::Interval {
            channel: column.raw_u32(),
            lo: -1.0,
            hi: 1.0,
        },
        ActionBandTargetSpec::AxisAlignedBox {
            channels: vec![column.raw_u32(), second_column],
            lo: vec![-1.0, -2.0],
            hi: vec![1.0, 2.0],
        },
        ActionBandTargetSpec::LocusRadius {
            distance_channel: column.raw_u32(),
            radius: 0.25,
        },
        ActionBandTargetSpec::PalmaReachableSet {
            distance_channel: column.raw_u32(),
            maximum_distance: 9.0,
        },
        ActionBandTargetSpec::EmlProjectedSet {
            input_channels: vec![column.raw_u32()],
            membership_program: 1,
            projection_program: Some(2),
            projection_width: 1,
        },
    ];
    for (index, target) in forms.into_iter().enumerate() {
        let mut template = base_template(column.raw_u32());
        template.id = format!("closed-form-{index}");
        template
            .axis_channels
            .push(channel(second_column, ActionBandChannelKind::CachedDerived));
        template.target = target;
        let mut door = ActionBandSessionBuildDoor::new();
        door.admit_once_at_session_build(&session_spec(template), &registry, &eml, &thresholds)
            .expect("each closed target form supplies a complete lowering");
    }

    let mut predicate_only = base_template(column.raw_u32());
    predicate_only.target = ActionBandTargetSpec::EmlProjectedSet {
        input_channels: vec![column.raw_u32()],
        membership_program: 1,
        projection_program: None,
        projection_width: 1,
    };
    let mut door = ActionBandSessionBuildDoor::new();
    assert!(matches!(
        door.admit_once_at_session_build(
            &session_spec(predicate_only),
            &registry,
            &eml,
            &thresholds,
        ),
        Err(ActionBandAdmissionError::PredicateOnlyTarget { .. })
    ));

    let mut unretained_velocity = base_template(column.raw_u32());
    unretained_velocity.velocity = Some(ActionBandVelocitySpec {
        current_channel: column.raw_u32(),
        previous_generation_channel: None,
    });
    let mut door = ActionBandSessionBuildDoor::new();
    assert!(matches!(
        door.admit_once_at_session_build(
            &session_spec(unretained_velocity),
            &registry,
            &eml,
            &thresholds,
        ),
        Err(ActionBandAdmissionError::PreviousGenerationPlaneRequired { .. })
    ));

    let mut retained_velocity = base_template(column.raw_u32());
    retained_velocity.axis_channels.push(channel(
        previous_column.raw_u32(),
        ActionBandChannelKind::CachedDerived,
    ));
    retained_velocity.velocity = Some(ActionBandVelocitySpec {
        current_channel: column.raw_u32(),
        previous_generation_channel: Some(previous_column.raw_u32()),
    });
    let mut door = ActionBandSessionBuildDoor::new();
    let admitted = door
        .admit_once_at_session_build(
            &session_spec(retained_velocity),
            &registry,
            &eml,
            &thresholds,
        )
        .expect("retained velocity pair admits");
    let frozen_velocity = admitted.templates()[0]
        .velocity()
        .expect("admission freezes the velocity binding");
    assert_eq!(frozen_velocity.current_channel(), column);
    assert_eq!(
        frozen_velocity.previous_generation_channel(),
        previous_column
    );
}

#[test]
fn admission_freezes_axis_dependency_and_storage_budgets() {
    let (registry, thresholds, column) = registry_and_threshold();
    let eml = eml_registry();
    let second_column = column.raw_u32() + 1;

    let mut over_axis = base_template(column.raw_u32());
    over_axis
        .axis_channels
        .push(channel(second_column, ActionBandChannelKind::CachedDerived));
    let mut spec = session_spec(over_axis);
    spec.budget.axis_channel_count = 1;
    let mut door = ActionBandSessionBuildDoor::new();
    assert!(matches!(
        door.admit_once_at_session_build(&spec, &registry, &eml, &thresholds),
        Err(ActionBandAdmissionError::AxisChannelBudgetExceeded { .. })
    ));

    let mut over_subordination = base_template(column.raw_u32());
    over_subordination.max_active_subordinates = 1;
    let mut door = ActionBandSessionBuildDoor::new();
    assert!(matches!(
        door.admit_once_at_session_build(
            &session_spec(over_subordination),
            &registry,
            &eml,
            &thresholds,
        ),
        Err(ActionBandAdmissionError::MaxActiveSubordinatesExceedsSpan { .. })
    ));

    let mut parent = base_template(column.raw_u32());
    parent.subordinate_template_ids = vec!["synthetic-child".into()];
    parent.max_active_subordinates = 1;
    let mut child = base_template(column.raw_u32());
    child.id = "synthetic-child".into();
    child.bands.clear();
    let mut spec = session_spec(parent);
    spec.templates.push(child);
    spec.budget.dependency_binding_count = 0;
    let mut door = ActionBandSessionBuildDoor::new();
    assert!(matches!(
        door.admit_once_at_session_build(&spec, &registry, &eml, &thresholds),
        Err(ActionBandAdmissionError::DependencyBudgetExceeded { .. })
    ));

    let mut spec = session_spec(base_template(column.raw_u32()));
    spec.budget.storage_rows = 0;
    let mut door = ActionBandSessionBuildDoor::new();
    assert!(matches!(
        door.admit_once_at_session_build(&spec, &registry, &eml, &thresholds),
        Err(ActionBandAdmissionError::StorageBudgetExceeded { .. })
    ));
}

#[test]
fn session_door_refuses_mid_session_template_mint() {
    let (registry, thresholds, column) = registry_and_threshold();
    let eml = eml_registry();
    let spec = session_spec(base_template(column.raw_u32()));
    let mut door = ActionBandSessionBuildDoor::new();
    door.admit_once_at_session_build(&spec, &registry, &eml, &thresholds)
        .expect("first session-build admission");
    assert!(matches!(
        door.admit_once_at_session_build(&spec, &registry, &eml, &thresholds),
        Err(ActionBandAdmissionError::MidSessionTemplateMintRefused {
            admitted: 1,
            attempted: 1
        })
    ));
}

#[test]
fn pre_8x_scarce_lane_semantics_fail_closed_and_labels_stay_shadow_only() {
    let (registry, thresholds, column) = registry_and_threshold();
    let eml = eml_registry();
    for requirement in [
        ActionBandRequirementSemantics::AtomicCommonDepthCommitment,
        ActionBandRequirementSemantics::PersistentScarceGrantHolding,
    ] {
        let mut template = base_template(column.raw_u32());
        template.requirement_semantics = requirement;
        let mut door = ActionBandSessionBuildDoor::new();
        assert!(matches!(
            door.admit_once_at_session_build(
                &session_spec(template),
                &registry,
                &eml,
                &thresholds,
            ),
            Err(ActionBandAdmissionError::Pre8xScarceLaneSemanticsUnsupported { .. })
        ));
    }

    let mut door = ActionBandSessionBuildDoor::new();
    let product = door
        .admit_once_at_session_build(
            &session_spec(base_template(column.raw_u32())),
            &registry,
            &eml,
            &thresholds,
        )
        .expect("ordinary requirements admit");
    assert!(!format!("{:?}", product.templates()).contains("Readable diagnostic designation"));
    assert_eq!(
        product.semantic_shadow()[0].label(),
        Some("Readable diagnostic designation")
    );
}
