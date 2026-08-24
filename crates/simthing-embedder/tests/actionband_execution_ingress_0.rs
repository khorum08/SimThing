//! ACTIONBAND-EXECUTION-INGRESS-0: advertised commitments execute in ordinary SimSession.

use std::collections::HashMap;

use simthing_core::{
    eml_opcode, ColumnIndex, Direction, EmitOnThresholdBuffer, EmitOnThresholdRegistration,
    EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu,
    EmlTreeId, PropertyValue, SimProperty, SimThing, SimThingKind, SlotIndex, SubFieldRole,
    ThresholdDirection,
};
use simthing_driver::{
    ActionBandExecutionIngressError, Scenario, SessionError, StructuralAuthorization,
};
use simthing_embedder::{bind, run};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::{
    DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD, THRESH_BUF_OUTPUT, THRESH_BUF_OWNING_GENERATION,
    THRESH_BUF_VALUES,
};
use simthing_sim::CostBandSemantic;
use simthing_spec::{
    ActionBandAdmissionBudgetSpec, ActionBandBandSpec, ActionBandChannelBindingSpec,
    ActionBandChannelKind, ActionBandSessionBuildDoor, ActionBandSessionSpec, ActionBandTargetSpec,
    ActionBandTemplateSpec, GameModeSpec, ScalarBoundDirection,
};

fn live_thresholds(session: &run::SimSession) -> Vec<EmitOnThresholdRegistration> {
    session
        .state
        .accumulator_runtime
        .as_ref()
        .expect("ordinary initial sync provisions the threshold accumulator")
        .threshold_registrations()
        .iter()
        .map(|registration| EmitOnThresholdRegistration {
            slot: SlotIndex::new(registration.slot),
            col: ColumnIndex::try_from_admitted_authored(
                registration.col,
                session.proto.registry.total_columns as u32,
            )
            .expect("live threshold column remains inside the admitted registry"),
            threshold: registration.threshold,
            direction: match registration.direction {
                DIR_UPWARD => ThresholdDirection::Upward,
                DIR_DOWNWARD => ThresholdDirection::Downward,
                DIR_EITHER => ThresholdDirection::Either,
                other => panic!("unknown live threshold direction {other}"),
            },
            event_kind: registration.event_kind,
            buffer: match registration.buffer {
                THRESH_BUF_VALUES => EmitOnThresholdBuffer::Values,
                THRESH_BUF_OUTPUT => EmitOnThresholdBuffer::Output,
                THRESH_BUF_OWNING_GENERATION => EmitOnThresholdBuffer::OwningGeneration,
                other => panic!("unknown live threshold buffer {other}"),
            },
        })
        .collect()
}

fn eml_registry() -> EmlExpressionRegistry {
    let tree_id = EmlTreeId(8111);
    let nodes = vec![
        EmlNodeGpu {
            opcode: eml_opcode::PARAM,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::LITERAL_F32,
            flags: 0,
            a: 1.0f32.to_bits(),
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::MUL,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
    ];
    let mut registry = EmlExpressionRegistry::new();
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
                node_count: nodes.len() as u32,
                max_stack_depth: 2,
                has_loops: false,
                has_recursion: false,
                display_name: "ordinary-session-actionband-ingress".into(),
            },
            nodes,
        )
        .expect("existing exact EML program admits");
    registry
}

#[test]
fn advertised_commitments_execute_and_stale_shape_refuses() {
    let mut registry = simthing_core::DimensionRegistry::new();
    let property_id = registry.register(SimProperty::simple("ingress", "signal", 0));
    let layout = registry.property(property_id).layout.clone();
    let amount_col = registry
        .column_range(property_id)
        .col_for_role(&SubFieldRole::Amount, &layout)
        .expect("amount column");
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    let mut value = PropertyValue::from_layout(&layout);
    value.set_role(&SubFieldRole::Amount, &layout, 0.0);
    value.set_role(&SubFieldRole::Velocity, &layout, 1.0);
    root.add_property(property_id, value);
    let root_id = root.id;
    let child = SimThing::new(SimThingKind::Cohort, 0);
    let child_id = child.id;
    let new_parent = SimThing::new(SimThingKind::Cohort, 0);
    let new_parent_id = new_parent.id;
    root.add_child(child);
    root.add_child(new_parent);
    let scenario = Scenario {
        name: "actionband-execution-ingress".into(),
        ticks_per_day: 1,
        max_days: 3,
        dt: 1.0,
        n_slots: 3,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    };
    let mut session = run::initialize(scenario, &GameModeSpec::default())
        .expect("ordinary SimSession opens on a real adapter");
    bind::velocity_threshold(
        &mut session,
        simthing_sim::VelocityAlertRegistration {
            sim_thing_id: root_id,
            property_id,
            sub_field: SubFieldRole::Amount,
            threshold: 0.5,
            direction: Direction::Rising,
            cost_band: CostBandSemantic::observation(),
        },
    );
    session
        .proto
        .initial_gpu_sync(&session.coord, &mut session.state)
        .expect("session-build threshold sync uses the ordinary boundary compiler");
    let thresholds = live_thresholds(&session);
    let threshold_index = thresholds
        .iter()
        .position(|registration| {
            registration.slot == SlotIndex::new(0)
                && registration.col == amount_col
                && registration.threshold.to_bits() == 0.5f32.to_bits()
        })
        .expect("the live velocity registration is present");
    let eml = eml_registry();
    let action_band = ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 1,
            dependency_binding_count: 0,
            storage_rows: 1,
            eml_program_count: 1,
            emission_binding_count: 1,
        },
        templates: vec![ActionBandTemplateSpec {
            id: "ordinary-ingress".into(),
            label: None,
            axis_channels: vec![ActionBandChannelBindingSpec {
                column: amount_col.raw_u32(),
                kind: ActionBandChannelKind::Primitive,
            }],
            target: ActionBandTargetSpec::ScalarBound {
                channel: amount_col.raw_u32(),
                bound: 2.0,
                direction: ScalarBoundDirection::AtLeast,
            },
            velocity: None,
            bands: vec![ActionBandBandSpec {
                threshold_registration_index: threshold_index as u32,
                eml_program: Some(8111),
                emission_binding_indices: vec![0],
            }],
            subordinate_template_ids: Vec::new(),
            max_active_subordinates: 0,
            reserved_instance_rows: 1,
            requirement_semantics: Default::default(),
        }],
    };
    let mut door = ActionBandSessionBuildDoor::new();
    let frozen = door
        .admit_once_at_session_build(&action_band, &session.proto.registry, &eml, &thresholds)
        .expect("embedder-authored ActionBand freezes once")
        .clone();
    let native_lanes = bind::ActionBandNativeLaneAdmission::from_existing_surfaces(
        &session.proto.registry,
        &[amount_col],
        &[],
        &thresholds,
        session.proto.threshold_registry(),
    );
    let active = [bind::ActionBandActiveInstance::new(
        frozen.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    let consequence = StructuralAuthorization::admit(BoundaryRequest::Reparent {
        child: child_id,
        new_parent: new_parent_id,
    })
    .expect("existing reparent verb is an admitted structural consequence");
    bind::action_band_commitments(
        &mut session,
        &frozen,
        &eml,
        &[consequence],
        &active,
        &native_lanes,
    )
    .expect("advertised door consumes the product into ordinary SimSession");
    assert_eq!(session.action_band_execution_generation(), Some(0));

    let summary = session
        .run(3)
        .expect("ordinary hot-cycle and boundary path");
    assert_eq!(
        summary.action_band_crossing_batches, 1,
        "ACTIONBAND-EXECUTION-INGRESS-DROPPED-PRODUCT: the ordinary boundary must consume the compiled ActionBand product"
    );
    assert_eq!(
        summary.action_band_crossings, 1,
        "ACTIONBAND-EXECUTION-INGRESS-DROPPED-PRODUCT: the canonical Phase-5 crossing must reach the retained dispatcher"
    );
    assert_eq!(
        summary.action_band_structural_authorizations, 1,
        "ACTIONBAND-EXECUTION-INGRESS-DROPPED-PRODUCT: GPU production must submit the frozen consequence"
    );
    assert_eq!(
        session.action_band_execution_generation(),
        Some(1),
        "ACTIONBAND-EXECUTION-INGRESS-DROPPED-PRODUCT: the existing facility boundary must advance"
    );
    assert_eq!(
        session.proto.root.child_id(new_parent_id, 0),
        Some(child_id),
        "ACTIONBAND-EXECUTION-INGRESS-DROPPED-PRODUCT: the GPU commitment must traverse the feeder and apply its observable structural consequence on the next ordinary boundary"
    );

    session
        .proto
        .registry
        .register(SimProperty::simple("ingress", "late-growth", 0));
    assert!(matches!(
        session.step_once(),
        Err(SessionError::ActionBandIngress(
            ActionBandExecutionIngressError::RegistryStale
        ))
    ));
}
