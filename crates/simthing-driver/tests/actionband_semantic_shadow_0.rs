//! ACTIONBAND-SEMANTIC-SHADOW-0 focused proof (remand 3: same-dispatch seal + admitted loci).

use std::sync::Mutex;

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{
    DimensionRegistry, EmitOnThresholdBuffer, EmitOnThresholdRegistration, GenerationStamp,
    SimProperty, SimThing, SimThingId, SimThingKind, SlotIndex, SubFieldRole, ThresholdDirection,
};
use simthing_driver::{
    compile_action_band_gpu_execution, dispatch_and_seal, ActionBandActiveInstance,
    ActionBandSemanticSession, BoundObservableIdentity, FieldNeutralityGate,
    FrozenActionBandStructuralRequests, SemanticShadowError, FIELD_NEUTRALITY_OUTCOME,
};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, emit_on_threshold_registrations_to_gpu,
    scoped_debug_readback_allowed, wgpu, AccumulatorOpSession, ActionBandEmissionBindingGpu,
    ActionBandGpuExecution, GpuContext, PackedThresholdUpload, SlotAllocator,
};
use simthing_spec::{
    ActionBandAdmissionBudgetSpec, ActionBandBandSpec, ActionBandChannelBindingSpec,
    ActionBandChannelKind, ActionBandSessionBuildDoor, ActionBandSessionSpec, ActionBandTargetSpec,
    ActionBandTemplateSpec, FleetPresenceLocation, FrozenActionBandTemplates, ScalarBoundDirection,
};
use wgpu::util::DeviceExt;

static GPU_MUTEX: Mutex<()> = Mutex::new(());
const EVENT_KIND: u32 = 750;

struct Fixture {
    registry: DimensionRegistry,
    thresholds: Vec<EmitOnThresholdRegistration>,
    column: simthing_core::ColumnIndex,
}

fn fixture() -> Fixture {
    let mut registry = DimensionRegistry::new();
    let property = registry.register(SimProperty::simple("semantic-shadow", "axis", 1));
    let column = registry
        .column_range(property)
        .col_for_role(&SubFieldRole::Amount, &registry.property(property).layout)
        .expect("amount");
    let thresholds = vec![EmitOnThresholdRegistration {
        slot: SlotIndex::new(0),
        col: column,
        threshold: 1.0,
        direction: ThresholdDirection::Upward,
        event_kind: EVENT_KIND,
        buffer: EmitOnThresholdBuffer::Values,
    }];
    Fixture {
        registry,
        thresholds,
        column,
    }
}

fn session_spec(column: u32, id: &str, label: &str) -> ActionBandSessionSpec {
    ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 1,
            dependency_binding_count: 0,
            storage_rows: 1,
            eml_program_count: 0,
            emission_binding_count: 1,
        },
        templates: vec![ActionBandTemplateSpec {
            id: id.into(),
            label: Some(label.into()),
            axis_channels: vec![ActionBandChannelBindingSpec {
                column,
                kind: ActionBandChannelKind::Primitive,
            }],
            target: ActionBandTargetSpec::ScalarBound {
                channel: column,
                bound: 2.0,
                direction: ScalarBoundDirection::AtLeast,
            },
            velocity: None,
            bands: vec![ActionBandBandSpec {
                threshold_registration_index: 0,
                eml_program: None,
                emission_binding_indices: vec![0],
            }],
            subordinate_template_ids: vec![],
            max_active_subordinates: 0,
            reserved_instance_rows: 1,
            requirement_semantics: Default::default(),
        }],
    }
}

fn admit(fixture: &Fixture, id: &str, label: &str) -> FrozenActionBandTemplates {
    let mut door = ActionBandSessionBuildDoor::new();
    door.admit_once_at_session_build(
        &session_spec(fixture.column.raw_u32(), id, label),
        &fixture.registry,
        &simthing_core::EmlExpressionRegistry::new(),
        &fixture.thresholds,
    )
    .expect("7.1 admission")
    .clone()
}

fn require_gpu() -> GpuContext {
    let _g = GPU_MUTEX
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    use simthing_gpu::accumulator_op::set_debug_readback_allowed;
    set_debug_readback_allowed(true);
    GpuContext::new_blocking().expect("load-bearing GPU required")
}

struct WorldTree {
    tree: SimThing,
    actor: SimThingId,
    from: SimThingId,
    to: SimThingId,
}

fn world_tree(owner: &str) -> WorldTree {
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut from = SimThing::new(SimThingKind::Location, 0);
    let to = SimThing::new(SimThingKind::Location, 0);
    let mut actor = SimThing::new(SimThingKind::Cohort, 0);
    bind_owner(&mut actor, &OwnerRef::new(owner));
    let actor_id = actor.id;
    let from_id = from.id;
    let to_id = to.id;
    from.add_child(actor);
    root.add_child(from);
    root.add_child(to);
    WorldTree {
        tree: root,
        actor: actor_id,
        from: from_id,
        to: to_id,
    }
}

/// Compile + admit structural Reparent + single dispatch_and_seal.
fn produce(
    ctx: &GpuContext,
    fixture: &Fixture,
    frozen: &FrozenActionBandTemplates,
    actor: SimThingId,
    to: SimThingId,
) -> (
    simthing_driver::CompiledActionBandGpuExecution,
    ActionBandSemanticSession,
    simthing_driver::SemanticallySealedProduction,
) {
    let active = [ActionBandActiveInstance::new(
        frozen.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    let compiled = compile_action_band_gpu_execution(
        frozen,
        &simthing_core::EmlExpressionRegistry::new(),
        &[ActionBandEmissionBindingGpu::structural_request(0)],
        &active,
    )
    .expect("compile");

    // R1b: structural loci from admission-sealed Reparent only.
    let mut pre_admitted = vec![None; 1];
    pre_admitted[0] = Some(BoundaryRequest::Reparent {
        child: actor,
        new_parent: to,
    });
    let structural =
        FrozenActionBandStructuralRequests::from_compiled_admission(&compiled, pre_admitted)
            .expect("admitted structural door");
    let semantic = ActionBandSemanticSession::open(frozen.clone(), &compiled, structural)
        .expect("semantic session");

    let plan = compiled.execution_plan().clone();
    let n_dims = fixture.registry.total_columns as u32;
    let mut previous = vec![0.0f32; n_dims as usize];
    let mut current = previous.clone();
    previous[fixture.column.raw()] = 0.5;
    current[fixture.column.raw()] = 1.5;
    let regs = emit_on_threshold_registrations_to_gpu(&fixture.thresholds);
    let mut thresh = AccumulatorOpSession::new_attached(ctx, 1, n_dims, 4);
    thresh.bind_generation_authority(11);
    thresh.upload_values(ctx, &current);
    thresh.upload_previous_values(ctx, &previous);
    thresh
        .upload_packed_threshold_ops(
            ctx,
            &PackedThresholdUpload::from_registrations(&regs).unwrap(),
        )
        .unwrap();
    thresh.tick(ctx, 0).unwrap();
    let emissions = thresh.readback_threshold_emissions(ctx).unwrap();
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&SimThing::new(SimThingKind::GameSession, 0));
    let deltas = apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        thresh.threshold_registrations(),
        &fixture.registry,
        &allocator,
    );
    assert!(!deltas.is_empty());
    let crossings = plan.crossings_from_sealed(&deltas).unwrap();
    let world = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("semantic_shadow_world"),
            contents: bytemuck::cast_slice(&current),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        });
    let mut execution = match ActionBandGpuExecution::new(ctx, plan).unwrap() {
        ActionBandGpuExecution::Active(s) => s,
        ActionBandGpuExecution::Inactive => panic!("active"),
    };
    let _scope = scoped_debug_readback_allowed(true);
    // R1a: only door — dispatch and seal are the same call.
    let sealed = dispatch_and_seal(&compiled, &mut execution, ctx, &world, n_dims, &crossings)
        .expect("dispatch_and_seal");
    assert_eq!(sealed.authorities().len(), 1);
    assert_eq!(
        sealed.authorities()[0].generation().get(),
        execution.generation()
    );
    (compiled, semantic, sealed)
}

#[test]
fn field_neutrality_gate_is_field_neutral() {
    assert_eq!(
        FIELD_NEUTRALITY_OUTCOME,
        FieldNeutralityGate::FieldNeutral
    );
}

#[test]
fn a1_synthetic_non_palma_bound_observable_round_trips() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "transit-designation");
    let ctx = require_gpu();
    let world = world_tree("owner-alpha");
    let (_, session, sealed) = produce(&ctx, &fixture, &frozen, world.actor, world.to);
    let synthetic = BoundObservableIdentity::new(
        "synthetic-rf-grant-axis-v1",
        Some("post-authority-semantic-metadata"),
    );
    let readback = session
        .project(
            &sealed.authorities()[0],
            sealed.authorities()[0].generation(),
            &world.tree,
            std::slice::from_ref(&synthetic),
        )
        .unwrap();
    assert_eq!(
        readback.bound_observables()[0].key(),
        "synthetic-rf-grant-axis-v1"
    );
}

#[test]
fn production_generation_is_the_generation_read_back() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "owner-readback");
    let ctx = require_gpu();
    let world = world_tree("beta-owner");
    let (_, session, sealed) = produce(&ctx, &fixture, &frozen, world.actor, world.to);
    let gen = sealed.authorities()[0].generation();
    let readback = session
        .project(&sealed.authorities()[0], gen, &world.tree, &[])
        .unwrap();
    assert_eq!(readback.generation(), gen);
    assert_eq!(readback.owner().as_ref().unwrap().as_str(), "beta-owner");
    // Loci from admitted Reparent + tree parent (from), not caller table.
    assert_eq!(readback.actor(), world.actor);
    assert_eq!(readback.from_cell_raw(), world.from.raw());
    assert_eq!(readback.to_cell_raw(), world.to.raw());
}

#[test]
fn cross_dispatch_restamp_api_is_absent() {
    // R1a: no seal_production(production, execution) and no free seal_actionband_authority.
    let src = include_str!("../src/action_band_semantic_shadow.rs");
    assert!(!src.contains("pub fn seal_production"));
    assert!(!src.contains("pub fn seal_actionband_authority"));
    assert!(src.contains("pub fn dispatch_and_seal"));
    // Production and execution are not independently pairable for stamping.
    assert!(!src.contains("production: &ActionBandProductionDispatch"));
}

#[test]
fn foreign_compile_session_cannot_project_sealed_authority() {
    let fixture = fixture();
    let frozen_a = admit(&fixture, "template-a", "designation-A");
    let mut door = ActionBandSessionBuildDoor::new();
    let mut spec_b = session_spec(fixture.column.raw_u32(), "template-b", "designation-B");
    spec_b.budget.storage_rows = 2;
    spec_b.templates[0].reserved_instance_rows = 2;
    let frozen_b = door
        .admit_once_at_session_build(
            &spec_b,
            &fixture.registry,
            &simthing_core::EmlExpressionRegistry::new(),
            &fixture.thresholds,
        )
        .unwrap()
        .clone();
    let ctx = require_gpu();
    let world = world_tree("alpha");
    let (compiled_a, _, sealed_a) = produce(&ctx, &fixture, &frozen_a, world.actor, world.to);
    let compiled_b = compile_action_band_gpu_execution(
        &frozen_b,
        &simthing_core::EmlExpressionRegistry::new(),
        &[ActionBandEmissionBindingGpu::structural_request(0)],
        &[ActionBandActiveInstance::new(
            frozen_b.templates()[0].index(),
            SlotIndex::new(0),
            [0.0; 4],
        )],
    )
    .unwrap();
    if compiled_a.plan_fingerprint() != compiled_b.plan_fingerprint() {
        let mut pre = vec![None; 1];
        pre[0] = Some(BoundaryRequest::Reparent {
            child: world.actor,
            new_parent: world.to,
        });
        let structural =
            FrozenActionBandStructuralRequests::from_compiled_admission(&compiled_b, pre).unwrap();
        let session_b =
            ActionBandSemanticSession::open(frozen_b, &compiled_b, structural).unwrap();
        let err = session_b
            .project(
                &sealed_a.authorities()[0],
                sealed_a.authorities()[0].generation(),
                &world.tree,
                &[],
            )
            .unwrap_err();
        assert!(matches!(
            err,
            SemanticShadowError::PlanFingerprintMismatch { .. }
        ));
    }
}

#[test]
fn structural_loci_come_from_admitted_reparent_not_caller_table() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "loci");
    let ctx = require_gpu();
    let world = world_tree("gamma");
    let (_, session, sealed) = produce(&ctx, &fixture, &frozen, world.actor, world.to);
    let readback = session
        .project(
            &sealed.authorities()[0],
            sealed.authorities()[0].generation(),
            &world.tree,
            &[],
        )
        .unwrap();
    // Dest from admitted Reparent; source from tree parent of actor.
    assert_eq!(readback.to_cell_raw(), world.to.raw());
    assert_eq!(readback.from_cell_raw(), world.from.raw());
    let src = include_str!("../src/action_band_semantic_shadow.rs");
    assert!(!src.contains("structural: &[(u32"));
    assert!(!src.contains("pub struct AdmittedStructuralLoci"));
}

#[test]
fn stale_production_stamp_fails_closed() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "stale");
    let ctx = require_gpu();
    let world = world_tree("beta");
    let (_, session, sealed) = produce(&ctx, &fixture, &frozen, world.actor, world.to);
    let gen = sealed.authorities()[0].generation().get();
    let err = session
        .project(
            &sealed.authorities()[0],
            GenerationStamp::new(gen + 3),
            &world.tree,
            &[],
        )
        .unwrap_err();
    assert!(matches!(
        err,
        SemanticShadowError::StaleGenerationStamp { .. }
    ));
}

#[test]
fn identity_blindness_labels_do_not_change_numerical_or_sealed_products() {
    let fixture = fixture();
    let frozen_a = admit(
        &fixture,
        "semantic-shadow-template",
        "human-readable-movement-to-orion",
    );
    let frozen_b = admit(
        &fixture,
        "semantic-shadow-template",
        "completely-different-designation-words",
    );
    let ctx = require_gpu();
    let world = world_tree("alpha");
    let (_, _, sealed_a) = produce(&ctx, &fixture, &frozen_a, world.actor, world.to);
    let (_, _, sealed_b) = produce(&ctx, &fixture, &frozen_b, world.actor, world.to);
    assert_eq!(
        sealed_a.production().commitments[0].value().to_bits(),
        sealed_b.production().commitments[0].value().to_bits()
    );
    assert_eq!(
        sealed_a.authorities()[0].plan_fingerprint(),
        sealed_b.authorities()[0].plan_fingerprint()
    );
}

#[test]
fn foreign_owner_error_propagates_through_transit_projection() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "owner-mutants");
    let ctx = require_gpu();
    // Admitted Reparent names a foreign child not in the authority tree.
    let foreign = SimThingId::new();
    let to = SimThingId::new();
    let tree = SimThing::new(SimThingKind::World, 0);
    let active = [ActionBandActiveInstance::new(
        frozen.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    let compiled = compile_action_band_gpu_execution(
        &frozen,
        &simthing_core::EmlExpressionRegistry::new(),
        &[ActionBandEmissionBindingGpu::structural_request(0)],
        &active,
    )
    .unwrap();
    let mut pre = vec![None; 1];
    pre[0] = Some(BoundaryRequest::Reparent {
        child: foreign,
        new_parent: to,
    });
    let structural =
        FrozenActionBandStructuralRequests::from_compiled_admission(&compiled, pre).unwrap();
    let session = ActionBandSemanticSession::open(frozen.clone(), &compiled, structural).unwrap();

    // Still need a real sealed authority via dispatch_and_seal.
    let world = world_tree("gamma");
    let (_, _, sealed) = produce(&ctx, &fixture, &frozen, world.actor, world.to);
    // Project with foreign-admitted structural door (session) but sealed from other produce —
    // event_kind matches; actor is foreign from admitted door.
    // Wait: sealed was produced with different structural in produce() - session has foreign actor.
    // The sealed authority event_kind is EVENT_KIND; session structural has foreign child.
    let projected = session.project(
        &sealed.authorities()[0],
        sealed.authorities()[0].generation(),
        &tree, // empty tree — foreign not present
        &[],
    );
    // Foreign actor not in tree: fail closed either at parent resolution or owner resolution.
    match projected {
        Err(SemanticShadowError::ActorParentUnresolved { .. }) => {}
        Ok(readback) => {
            assert!(readback.owner().is_err());
            let transit = readback.transit_projection();
            assert!(transit.owner.is_err());
            assert!(matches!(
                transit.to_fleet_presence_record(),
                Err(SemanticShadowError::OwnerResolution(_))
            ));
        }
        Err(other) => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn fleet_presence_in_transit_from_admitted_reparent_without_mapeditor_coupling() {
    // R3 preserved as engine-side FleetPresenceRecord product (detachability).
    // Peripheral icon consumer is covered by mapeditor unit tests on FleetPresenceRecord.
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "in-transit-fleet-shadow");
    let ctx = require_gpu();
    let world = world_tree("gamma");
    let (_, session, sealed) = produce(&ctx, &fixture, &frozen, world.actor, world.to);
    let readback = session
        .project(
            &sealed.authorities()[0],
            sealed.authorities()[0].generation(),
            &world.tree,
            &[],
        )
        .unwrap();
    let record = readback.to_fleet_presence_record().unwrap();
    assert!(matches!(
        record.location,
        FleetPresenceLocation::InTransit {
            source_system_id,
            dest_system_id
        } if source_system_id == world.from.raw() && dest_system_id == world.to.raw()
    ));
    assert_eq!(record.fleet_simthing_id_raw, world.actor.raw());
    assert_eq!(
        record.owner_ref.as_ref().map(|o| o.as_str()),
        Some("gamma")
    );
    // No mapeditor import in this crate's test (R5).
    let cargo = include_str!("../Cargo.toml");
    assert!(!cargo.contains("simthing-mapeditor"));
}

#[test]
fn identity_blindness_labels_do_not_change_plan_fingerprint() {
    let fixture = fixture();
    let a = admit(&fixture, "semantic-shadow-template", "label-A");
    let b = admit(&fixture, "semantic-shadow-template", "label-B");
    assert_ne!(a.semantic_shadow()[0].label(), b.semantic_shadow()[0].label());
    let ctx = require_gpu();
    let world = world_tree("alpha");
    let (_, _, sa) = produce(&ctx, &fixture, &a, world.actor, world.to);
    let (_, _, sb) = produce(&ctx, &fixture, &b, world.actor, world.to);
    assert_eq!(
        sa.authorities()[0].plan_fingerprint(),
        sb.authorities()[0].plan_fingerprint()
    );
}
