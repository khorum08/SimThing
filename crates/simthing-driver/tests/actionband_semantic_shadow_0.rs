//! ACTIONBAND-SEMANTIC-SHADOW-0 focused proof
//! (remand 5: label-blind association; same-authority dual semantic views).

use std::sync::Mutex;

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{
    DimensionRegistry, EmitOnThresholdBuffer, EmitOnThresholdRegistration, GenerationStamp,
    SimProperty, SimThing, SimThingId, SimThingKind, SlotIndex, SubFieldRole, ThresholdDirection,
};
use simthing_driver::{
    compile_action_band_gpu_execution, frozen_admission_binding_id, ActionBandActiveInstance,
    ActionBandSemanticSession, BoundObservableIdentity, FieldNeutralityGate,
    FrozenActionBandStructuralRequests, SemanticShadowError, FIELD_NEUTRALITY_OUTCOME,
};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, emit_on_threshold_registrations_to_gpu,
    scoped_debug_readback_allowed, wgpu, AccumulatorOpSession, ActionBandEmissionBindingGpu,
    GpuContext, PackedThresholdUpload, SlotAllocator,
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

/// Dual-actor world for same-shape cross-session structural mutant (R1b.1).
struct DualWorld {
    tree: SimThing,
    actor_a: SimThingId,
    actor_b: SimThingId,
    from_a: SimThingId,
    from_b: SimThingId,
    to_a: SimThingId,
    to_b: SimThingId,
}

fn dual_world() -> DualWorld {
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut from_a = SimThing::new(SimThingKind::Location, 0);
    let mut from_b = SimThing::new(SimThingKind::Location, 0);
    let to_a = SimThing::new(SimThingKind::Location, 0);
    let to_b = SimThing::new(SimThingKind::Location, 0);
    let mut actor_a = SimThing::new(SimThingKind::Cohort, 0);
    let mut actor_b = SimThing::new(SimThingKind::Cohort, 0);
    bind_owner(&mut actor_a, &OwnerRef::new("owner-a"));
    bind_owner(&mut actor_b, &OwnerRef::new("owner-b"));
    let actor_a_id = actor_a.id;
    let actor_b_id = actor_b.id;
    let from_a_id = from_a.id;
    let from_b_id = from_b.id;
    let to_a_id = to_a.id;
    let to_b_id = to_b.id;
    from_a.add_child(actor_a);
    from_b.add_child(actor_b);
    root.add_child(from_a);
    root.add_child(from_b);
    root.add_child(to_a);
    root.add_child(to_b);
    DualWorld {
        tree: root,
        actor_a: actor_a_id,
        actor_b: actor_b_id,
        from_a: from_a_id,
        from_b: from_b_id,
        to_a: to_a_id,
        to_b: to_b_id,
    }
}

/// Compile + admit structural Reparent + open cohesive session + dispatch_and_seal.
fn produce(
    ctx: &GpuContext,
    fixture: &Fixture,
    frozen: &FrozenActionBandTemplates,
    actor: SimThingId,
    to: SimThingId,
) -> (
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

    let mut pre_admitted = vec![None; 1];
    pre_admitted[0] = Some(BoundaryRequest::Reparent {
        child: actor,
        new_parent: to,
    });
    let structural =
        FrozenActionBandStructuralRequests::from_compiled_admission(&compiled, pre_admitted)
            .expect("admitted structural door");
    let semantic = ActionBandSemanticSession::open(frozen.clone(), compiled, structural)
        .expect("semantic session");

    let plan = semantic.compiled().execution_plan().clone();
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
    allocator.install_initial_tree(&SimThing::new(SimThingKind::GameSession, 0));
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
    let _scope = scoped_debug_readback_allowed(true);
    // R1a: seal uses only this session's compile product via bind_dispatch.
    let mut bound = semantic.bind_dispatch(ctx).expect("bind_dispatch");
    let sealed = bound
        .dispatch_and_seal(ctx, &world, n_dims, &crossings)
        .expect("dispatch_and_seal");
    assert_eq!(sealed.authorities().len(), 1);
    assert_eq!(
        sealed.authorities()[0].generation().get(),
        bound.generation()
    );
    assert_eq!(
        sealed.authorities()[0].session_origin(),
        semantic.session_origin()
    );
    (semantic, sealed)
}

#[test]
fn field_neutrality_gate_is_field_neutral() {
    assert_eq!(FIELD_NEUTRALITY_OUTCOME, FieldNeutralityGate::FieldNeutral);
}

#[test]
fn a1_synthetic_non_palma_bound_observable_round_trips() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "transit-designation");
    let ctx = require_gpu();
    let world = world_tree("owner-alpha");
    let (session, sealed) = produce(&ctx, &fixture, &frozen, world.actor, world.to);
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
    let (session, sealed) = produce(&ctx, &fixture, &frozen, world.actor, world.to);
    let gen = sealed.authorities()[0].generation();
    let readback = session
        .project(&sealed.authorities()[0], gen, &world.tree, &[])
        .unwrap();
    assert_eq!(readback.generation(), gen);
    assert_eq!(readback.owner().as_ref().unwrap().as_str(), "beta-owner");
    assert_eq!(readback.actor(), world.actor);
    assert_eq!(readback.from_cell_raw(), world.from.raw());
    assert_eq!(readback.to_cell_raw(), world.to.raw());
}

#[test]
fn cross_dispatch_restamp_and_foreign_compile_api_is_absent() {
    // R1a / R1a.1: no free seal doors that pair independent compiled + execution.
    let src = include_str!("../src/action_band_semantic_shadow.rs");
    assert!(!src.contains("pub fn seal_production"));
    assert!(!src.contains("pub fn seal_actionband_authority"));
    // Free function form taking compiled is gone; only ActionBandBoundDispatch method remains.
    assert!(!src.contains("pub fn dispatch_and_seal(\n    compiled"));
    assert!(!src.contains("compiled: &CompiledActionBandGpuExecution"));
    assert!(!src.contains("production: &ActionBandProductionDispatch"));
    assert!(src.contains("impl ActionBandBoundDispatch"));
    assert!(src.contains("pub fn bind_dispatch"));
    assert!(src.contains("pub fn dispatch_and_seal"));
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
    let (_, sealed_a) = produce(&ctx, &fixture, &frozen_a, world.actor, world.to);
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
    let mut pre = vec![None; 1];
    pre[0] = Some(BoundaryRequest::Reparent {
        child: world.actor,
        new_parent: world.to,
    });
    let structural =
        FrozenActionBandStructuralRequests::from_compiled_admission(&compiled_b, pre).unwrap();
    let session_b = ActionBandSemanticSession::open(frozen_b, compiled_b, structural).unwrap();
    let err = session_b
        .project(
            &sealed_a.authorities()[0],
            sealed_a.authorities()[0].generation(),
            &world.tree,
            &[],
        )
        .unwrap_err();
    // Same-shape or not: origin mismatch is the association fence.
    assert!(matches!(
        err,
        SemanticShadowError::SessionOriginMismatch { .. }
            | SemanticShadowError::PlanFingerprintMismatch { .. }
    ));
}

#[test]
fn structural_loci_come_from_admitted_reparent_not_caller_table() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "loci");
    let ctx = require_gpu();
    let world = world_tree("gamma");
    let (session, sealed) = produce(&ctx, &fixture, &frozen, world.actor, world.to);
    let readback = session
        .project(
            &sealed.authorities()[0],
            sealed.authorities()[0].generation(),
            &world.tree,
            &[],
        )
        .unwrap();
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
    let (session, sealed) = produce(&ctx, &fixture, &frozen, world.actor, world.to);
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

/// EXIT-PROOF: labels/designations differ on the **same** opaque ActionBand
/// execution — one dispatch, one sealed production, one authority — projected
/// through two lawful post-authority semantic views.
#[test]
fn identity_blindness_labels_do_not_change_numerical_or_sealed_products() {
    let fixture = fixture();
    // Same logical authored/template identity; designations differ only.
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
    assert_eq!(
        frozen_a.semantic_shadow()[0].authored_id(),
        frozen_b.semantic_shadow()[0].authored_id()
    );
    assert_ne!(
        frozen_a.semantic_shadow()[0].label(),
        frozen_b.semantic_shadow()[0].label()
    );
    // Label must not affect pre-dispatch association binding.
    assert_eq!(
        frozen_admission_binding_id(&frozen_a),
        frozen_admission_binding_id(&frozen_b)
    );

    let ctx = require_gpu();
    let world = world_tree("alpha");
    let active = [ActionBandActiveInstance::new(
        frozen_a.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    // One compile / one structural admission for the logical session.
    let compiled = compile_action_band_gpu_execution(
        &frozen_a,
        &simthing_core::EmlExpressionRegistry::new(),
        &[ActionBandEmissionBindingGpu::structural_request(0)],
        &active,
    )
    .unwrap();
    let mut pre = vec![None; 1];
    pre[0] = Some(BoundaryRequest::Reparent {
        child: world.actor,
        new_parent: world.to,
    });
    let structural =
        FrozenActionBandStructuralRequests::from_compiled_admission(&compiled, pre).unwrap();

    // Two post-authority semantic views of the same logical admission/compile.
    let session_a =
        ActionBandSemanticSession::open(frozen_a.clone(), compiled.clone(), structural.clone())
            .expect("view A open");
    let session_b = ActionBandSemanticSession::open(frozen_b, compiled, structural)
        .expect("view B open — label-only change must not block open/dispatch");
    assert_eq!(session_a.session_origin(), session_b.session_origin());

    // One GPU dispatch / one sealed production on the shared logical origin.
    let plan = session_a.compiled().execution_plan().clone();
    let n_dims = fixture.registry.total_columns as u32;
    let mut previous = vec![0.0f32; n_dims as usize];
    let mut current = previous.clone();
    previous[fixture.column.raw()] = 0.5;
    current[fixture.column.raw()] = 1.5;
    let regs = emit_on_threshold_registrations_to_gpu(&fixture.thresholds);
    let mut thresh = AccumulatorOpSession::new_attached(&ctx, 1, n_dims, 4);
    thresh.bind_generation_authority(11);
    thresh.upload_values(&ctx, &current);
    thresh.upload_previous_values(&ctx, &previous);
    thresh
        .upload_packed_threshold_ops(
            &ctx,
            &PackedThresholdUpload::from_registrations(&regs).unwrap(),
        )
        .unwrap();
    thresh.tick(&ctx, 0).unwrap();
    let emissions = thresh.readback_threshold_emissions(&ctx).unwrap();
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&SimThing::new(SimThingKind::GameSession, 0));
    let deltas = apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        thresh.threshold_registrations(),
        &fixture.registry,
        &allocator,
    );
    let crossings = plan.crossings_from_sealed(&deltas).unwrap();
    let world_buf = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("identity_blind_world"),
            contents: bytemuck::cast_slice(&current),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        });
    let _scope = scoped_debug_readback_allowed(true);
    let mut bound = session_a.bind_dispatch(&ctx).expect("bind_dispatch");
    let sealed = bound
        .dispatch_and_seal(&ctx, &world_buf, n_dims, &crossings)
        .expect("single dispatch_and_seal");
    assert_eq!(sealed.authorities().len(), 1);
    let authority = &sealed.authorities()[0];
    assert_eq!(authority.session_origin(), session_a.session_origin());
    assert_eq!(authority.session_origin(), session_b.session_origin());

    let read_a = session_a
        .project(authority, authority.generation(), &world.tree, &[])
        .unwrap();
    let read_b = session_b
        .project(authority, authority.generation(), &world.tree, &[])
        .unwrap();

    // Designation differs as authored; sealed authoritative products identical.
    assert_eq!(
        read_a.designation(),
        Some("human-readable-movement-to-orion")
    );
    assert_eq!(
        read_b.designation(),
        Some("completely-different-designation-words")
    );
    assert_eq!(read_a.generation(), read_b.generation());
    assert_eq!(read_a.sealed_slot(), read_b.sealed_slot());
    assert_eq!(read_a.sealed_col(), read_b.sealed_col());
    assert_eq!(read_a.sealed_event_kind(), read_b.sealed_event_kind());
    assert_eq!(read_a.sealed_value_bits(), read_b.sealed_value_bits());
    assert_eq!(read_a.actor(), read_b.actor());
    assert_eq!(read_a.from_cell_raw(), read_b.from_cell_raw());
    assert_eq!(read_a.to_cell_raw(), read_b.to_cell_raw());
    assert_eq!(
        read_a.owner().as_ref().unwrap().as_str(),
        read_b.owner().as_ref().unwrap().as_str()
    );
    assert_eq!(authority.plan_fingerprint(), session_a.plan_fingerprint());
    assert_eq!(authority.plan_fingerprint(), session_b.plan_fingerprint());
    assert_eq!(
        sealed.production().commitments[0].value().to_bits(),
        read_a.sealed_value_bits()
    );
}

/// Negative control: foreign **logical** admission (different authored_id),
/// bit-identical numeric plan — not a label-only difference.
#[test]
fn same_shape_foreign_logical_admission_cannot_be_selected_at_open() {
    let fixture = fixture();
    let frozen_a = admit(&fixture, "logical-template-A", "shared-designation");
    let frozen_b = admit(&fixture, "logical-template-B", "shared-designation");
    assert_ne!(
        frozen_a.semantic_shadow()[0].authored_id(),
        frozen_b.semantic_shadow()[0].authored_id()
    );
    // Labels match — foreignness is logical authored identity only.
    assert_eq!(
        frozen_a.semantic_shadow()[0].label(),
        frozen_b.semantic_shadow()[0].label()
    );
    assert_ne!(
        frozen_admission_binding_id(&frozen_a),
        frozen_admission_binding_id(&frozen_b)
    );

    let active = [ActionBandActiveInstance::new(
        frozen_a.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    let compiled_a = compile_action_band_gpu_execution(
        &frozen_a,
        &simthing_core::EmlExpressionRegistry::new(),
        &[ActionBandEmissionBindingGpu::structural_request(0)],
        &active,
    )
    .unwrap();
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

    assert_eq!(
        compiled_a.plan_fingerprint(),
        compiled_b.plan_fingerprint(),
        "mutant requires bit-identical numeric plan fingerprints"
    );
    assert_ne!(compiled_a.session_origin(), compiled_b.session_origin());
    assert_ne!(
        compiled_a.frozen_admission_binding(),
        compiled_b.frozen_admission_binding()
    );

    let world = world_tree("alpha");
    let mut pre_a = vec![None; 1];
    pre_a[0] = Some(BoundaryRequest::Reparent {
        child: world.actor,
        new_parent: world.to,
    });
    let structural_a =
        FrozenActionBandStructuralRequests::from_compiled_admission(&compiled_a, pre_a).unwrap();

    // Pair foreign logical frozen_b with compile_a — must RED at open.
    let err = ActionBandSemanticSession::open(frozen_b, compiled_a.clone(), structural_a.clone())
        .unwrap_err();
    assert!(matches!(
        err,
        SemanticShadowError::FrozenCompileBindingMismatch { .. }
    ));

    let session_a = ActionBandSemanticSession::open(frozen_a, compiled_a, structural_a).unwrap();
    let src = include_str!("../src/action_band_semantic_shadow.rs");
    assert!(
        !src.contains("pub fn dispatch_and_seal(\n    compiled"),
        "foreign compiled must not be independently selectable at seal"
    );
    let _ = session_a;
}

/// R1b.1 — same event_kind + bit-identical numeric plan; different admitted Reparent.
/// Authority from A projected through session B must RED on origin association
/// (B's actor is in the tree so ActorParentUnresolved cannot hide the mismatch).
#[test]
fn same_shape_cross_session_structural_projection_is_red() {
    let fixture = fixture();
    let frozen_a = admit(&fixture, "semantic-shadow-template", "session-A-structural");
    let frozen_b = admit(&fixture, "semantic-shadow-template", "session-B-structural");
    let ctx = require_gpu();
    let dual = dual_world();

    let (session_a, sealed_a) = produce(&ctx, &fixture, &frozen_a, dual.actor_a, dual.to_a);
    let (session_b, sealed_b) = produce(&ctx, &fixture, &frozen_b, dual.actor_b, dual.to_b);

    assert_eq!(
        sealed_a.authorities()[0].plan_fingerprint(),
        sealed_b.authorities()[0].plan_fingerprint(),
        "mutant requires bit-identical numeric plan fingerprints"
    );
    assert_eq!(
        sealed_a.authorities()[0].event_kind(),
        sealed_b.authorities()[0].event_kind()
    );
    assert_ne!(session_a.session_origin(), session_b.session_origin());

    // Both actors live in the dual tree — parent resolution would succeed for B.
    let err = session_b
        .project(
            &sealed_a.authorities()[0],
            sealed_a.authorities()[0].generation(),
            &dual.tree,
            &[],
        )
        .unwrap_err();
    assert!(
        matches!(err, SemanticShadowError::SessionOriginMismatch { .. }),
        "expected SessionOriginMismatch, got {err:?}"
    );

    // Sanity: same-session projection still yields that session's admitted loci.
    let ok = session_b
        .project(
            &sealed_b.authorities()[0],
            sealed_b.authorities()[0].generation(),
            &dual.tree,
            &[],
        )
        .unwrap();
    assert_eq!(ok.actor(), dual.actor_b);
    assert_eq!(ok.from_cell_raw(), dual.from_b.raw());
    assert_eq!(ok.to_cell_raw(), dual.to_b.raw());
}

#[test]
fn foreign_owner_error_propagates_through_transit_projection() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "owner-mutants");
    let ctx = require_gpu();
    // Same origin/session: actor is in-tree (parent resolves) but has a blank owner
    // binding so resolve_owner fails closed — not the default unowned Ok path.
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut from = SimThing::new(SimThingKind::Location, 0);
    let to = SimThing::new(SimThingKind::Location, 0);
    let mut actor = SimThing::new(SimThingKind::Cohort, 0);
    bind_owner(&mut actor, &OwnerRef::new(""));
    let actor_id = actor.id;
    let to_id = to.id;
    from.add_child(actor);
    root.add_child(from);
    root.add_child(to);

    let (session, sealed) = produce(&ctx, &fixture, &frozen, actor_id, to_id);
    let readback = session
        .project(
            &sealed.authorities()[0],
            sealed.authorities()[0].generation(),
            &root,
            &[],
        )
        .unwrap();
    assert!(readback.owner().is_err());
    let transit = readback.transit_projection();
    assert!(transit.owner.is_err());
    assert!(matches!(
        transit.to_fleet_presence_record(),
        Err(SemanticShadowError::OwnerResolution(_))
    ));
}

#[test]
fn fleet_presence_in_transit_from_admitted_reparent_without_mapeditor_coupling() {
    let fixture = fixture();
    let frozen = admit(
        &fixture,
        "semantic-shadow-template",
        "in-transit-fleet-shadow",
    );
    let ctx = require_gpu();
    let world = world_tree("gamma");
    let (session, sealed) = produce(&ctx, &fixture, &frozen, world.actor, world.to);
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
    assert_eq!(record.owner_ref.as_ref().map(|o| o.as_str()), Some("gamma"));
    let cargo = include_str!("../Cargo.toml");
    assert!(!cargo.contains("simthing-mapeditor"));
}

#[test]
fn identity_blindness_labels_do_not_change_plan_fingerprint() {
    let fixture = fixture();
    let a = admit(&fixture, "semantic-shadow-template", "label-A");
    let b = admit(&fixture, "semantic-shadow-template", "label-B");
    assert_ne!(
        a.semantic_shadow()[0].label(),
        b.semantic_shadow()[0].label()
    );
    let ctx = require_gpu();
    let world = world_tree("alpha");
    let (_, sa) = produce(&ctx, &fixture, &a, world.actor, world.to);
    let (_, sb) = produce(&ctx, &fixture, &b, world.actor, world.to);
    assert_eq!(
        sa.authorities()[0].plan_fingerprint(),
        sb.authorities()[0].plan_fingerprint()
    );
}
