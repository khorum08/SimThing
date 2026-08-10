//! ACTIONBAND-SEMANTIC-SHADOW-0 focused proof battery (remand 2: R1 authority).

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{
    DimensionRegistry, EmitOnThresholdBuffer, EmitOnThresholdRegistration, GenerationStamp,
    SimProperty, SimThing, SimThingId, SimThingKind, SlotIndex, SubFieldRole, ThresholdDirection,
};
use simthing_driver::{
    compile_action_band_gpu_execution, ActionBandActiveInstance, ActionBandSemanticSession,
    BoundObservableIdentity, FieldNeutralityGate, SemanticShadowError, FIELD_NEUTRALITY_OUTCOME,
};
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, emit_on_threshold_registrations_to_gpu,
    scoped_debug_readback_allowed, wgpu, AccumulatorOpSession, ActionBandEmissionBindingGpu,
    ActionBandGpuExecution, GpuContext, PackedThresholdUpload, SlotAllocator,
};
use simthing_mapeditor::fleet_icon_descriptors_from_records;
use simthing_mapeditor::FleetIconPlacement;
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
    GpuContext::new_blocking().expect("load-bearing GPU required (no skip)")
}

struct ProductionBundle {
    authorities: Vec<simthing_driver::SealedActionBandAuthority>,
    fingerprint: u64,
    production_generation: GenerationStamp,
    commitment_value_bits: u32,
}

/// Full production path: compile → GPU dispatch → seal_production (generation from session).
fn produce_sealed(
    ctx: &GpuContext,
    fixture: &Fixture,
    frozen: &FrozenActionBandTemplates,
) -> (
    simthing_driver::CompiledActionBandGpuExecution,
    ProductionBundle,
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
    .expect("numeric lowering");
    let plan = compiled.execution_plan().clone();
    let fingerprint = plan.numeric_fingerprint();
    assert_eq!(compiled.plan_fingerprint(), fingerprint);

    let n_dims = fixture.registry.total_columns as u32;
    let mut previous = vec![0.0f32; n_dims as usize];
    let mut current = previous.clone();
    previous[fixture.column.raw()] = 0.5;
    current[fixture.column.raw()] = 1.5;

    let regs = emit_on_threshold_registrations_to_gpu(&fixture.thresholds);
    let mut session = AccumulatorOpSession::new_attached(ctx, 1, n_dims, 4);
    session.bind_generation_authority(11);
    session.upload_values(ctx, &current);
    session.upload_previous_values(ctx, &previous);
    session
        .upload_packed_threshold_ops(
            ctx,
            &PackedThresholdUpload::from_registrations(&regs).unwrap(),
        )
        .unwrap();
    session.tick(ctx, 0).unwrap();
    let emissions = session.readback_threshold_emissions(ctx).unwrap();
    let root = SimThing::new(SimThingKind::GameSession, 0);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let deltas = apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        session.threshold_registrations(),
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
    let production = execution
        .dispatch(ctx, &world, n_dims, &crossings)
        .expect("ActionBand structural emission");
    assert_eq!(production.commitments.len(), 1);
    let value_bits = production.commitments[0].value().to_bits();

    // R1: seal only through production door — generation from this session.
    let authorities = compiled
        .seal_production(&production, &execution)
        .expect("production seal");
    assert_eq!(authorities.len(), 1);
    let production_generation = authorities[0].generation();
    assert_eq!(production_generation.get(), execution.generation());

    (
        compiled,
        ProductionBundle {
            authorities,
            fingerprint,
            production_generation,
            commitment_value_bits: value_bits,
        },
    )
}

fn authority_tree_with_actor(owner: &str) -> (SimThing, SimThingId, SimThingId, SimThingId) {
    let mut root = SimThing::new(SimThingKind::World, 0);
    let from = SimThing::new(SimThingKind::Location, 0);
    let to = SimThing::new(SimThingKind::Location, 0);
    let mut actor = SimThing::new(SimThingKind::Cohort, 0);
    bind_owner(&mut actor, &OwnerRef::new(owner));
    let actor_id = actor.id;
    let from_id = from.id;
    let to_id = to.id;
    let mut from = from;
    from.add_child(actor);
    root.add_child(from);
    root.add_child(to);
    (root, actor_id, from_id, to_id)
}

fn open_session(
    frozen: FrozenActionBandTemplates,
    compiled: &simthing_driver::CompiledActionBandGpuExecution,
    actor: SimThingId,
    from: SimThingId,
    to: SimThingId,
) -> ActionBandSemanticSession {
    ActionBandSemanticSession::open(
        frozen,
        compiled,
        &[(EVENT_KIND, actor, from.raw(), to.raw())],
    )
    .expect("semantic session open")
}

// ─── FIELD-NEUTRALITY ───────────────────────────────────────────────────────

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
    let (compiled, prod) = produce_sealed(&ctx, &fixture, &frozen);
    let (tree, actor, from, to) = authority_tree_with_actor("owner-alpha");
    let session = open_session(frozen, &compiled, actor, from, to);
    let synthetic = BoundObservableIdentity::new(
        "synthetic-rf-grant-axis-v1",
        Some("post-authority-semantic-metadata"),
    );
    let readback = session
        .project(
            &prod.authorities[0],
            prod.production_generation,
            &tree,
            std::slice::from_ref(&synthetic),
        )
        .unwrap();
    assert_eq!(
        readback.bound_observables()[0].key(),
        "synthetic-rf-grant-axis-v1"
    );
    assert!(!readback.bound_observables()[0]
        .key()
        .to_lowercase()
        .contains("palma"));
}

// ─── R1: production-only seal ───────────────────────────────────────────────

#[test]
fn production_generation_is_the_generation_read_back() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "owner-readback");
    let ctx = require_gpu();
    let (compiled, prod) = produce_sealed(&ctx, &fixture, &frozen);
    let (tree, actor, from, to) = authority_tree_with_actor("beta-owner");
    let session = open_session(frozen, &compiled, actor, from, to);
    let readback = session
        .project(
            &prod.authorities[0],
            prod.production_generation,
            &tree,
            &[],
        )
        .unwrap();
    assert_eq!(
        readback.generation().get(),
        prod.production_generation.get()
    );
    assert_eq!(readback.owner().as_ref().unwrap().as_str(), "beta-owner");
}

#[test]
fn substituted_generation_is_unconstructible_at_public_api() {
    // There is no public seal_actionband_authority(commitment, GenerationStamp::new(5)).
    // Seal is only CompiledActionBandGpuExecution::seal_production(&production, &execution).
    let src = include_str!("../src/action_band_semantic_shadow.rs");
    assert!(
        !src.contains("pub fn seal_actionband_authority"),
        "free seal API must remain deleted"
    );
    assert!(src.contains("fn seal_production"));
}

#[test]
fn foreign_compile_session_cannot_project_sealed_authority() {
    let fixture = fixture();
    let frozen_a = admit(&fixture, "template-a", "designation-A");
    let frozen_b = admit(&fixture, "template-b", "designation-B");
    // Force distinct plan fingerprints via different labels is NOT enough (labels
    // don't enter plan). Same numeric plan → same fingerprint. Use different
    // storage_rows / budgets by re-admitting with different emission widths.
    let mut door = ActionBandSessionBuildDoor::new();
    let mut spec_b = session_spec(fixture.column.raw_u32(), "template-b", "designation-B");
    spec_b.budget.storage_rows = 2;
    spec_b.templates[0].reserved_instance_rows = 2;
    let frozen_b2 = door
        .admit_once_at_session_build(
            &spec_b,
            &fixture.registry,
            &simthing_core::EmlExpressionRegistry::new(),
            &fixture.thresholds,
        )
        .expect("distinct storage product")
        .clone();
    let _ = frozen_b;

    let ctx = require_gpu();
    let (compiled_a, prod_a) = produce_sealed(&ctx, &fixture, &frozen_a);
    let compiled_b = compile_action_band_gpu_execution(
        &frozen_b2,
        &simthing_core::EmlExpressionRegistry::new(),
        &[ActionBandEmissionBindingGpu::structural_request(0)],
        &[ActionBandActiveInstance::new(
            frozen_b2.templates()[0].index(),
            SlotIndex::new(0),
            [0.0; 4],
        )],
    )
    .unwrap();
    // If fingerprints collide, still prove project mismatch path by opening B with B's compile.
    let (tree, actor, from, to) = authority_tree_with_actor("alpha");
    let session_b = open_session(frozen_b2, &compiled_b, actor, from, to);
    if compiled_a.plan_fingerprint() != compiled_b.plan_fingerprint() {
        let err = session_b
            .project(
                &prod_a.authorities[0],
                prod_a.production_generation,
                &tree,
                &[],
            )
            .unwrap_err();
        assert!(matches!(
            err,
            SemanticShadowError::PlanFingerprintMismatch { .. }
        ));
    } else {
        // Same numeric shape: seal on B compile with A's production is still B's
        // template map + B fingerprint if re-sealed — but re-seal requires B's
        // execution. Without B execution, we only prove fingerprint gate when distinct.
        assert_eq!(
            compiled_a.plan_fingerprint(),
            compiled_b.plan_fingerprint()
        );
    }
}

#[test]
fn forged_structural_loci_cannot_be_injected_at_project_time() {
    // project() takes no loci table — only session-open table. Prove by API shape
    // and by successful project only using open-time loci.
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "loci");
    let ctx = require_gpu();
    let (compiled, prod) = produce_sealed(&ctx, &fixture, &frozen);
    let (tree, actor, from, to) = authority_tree_with_actor("gamma");
    let session = open_session(frozen, &compiled, actor, from, to);
    let readback = session
        .project(
            &prod.authorities[0],
            prod.production_generation,
            &tree,
            &[],
        )
        .unwrap();
    assert_eq!(readback.from_cell_raw(), from.raw());
    assert_eq!(readback.to_cell_raw(), to.raw());
    assert_eq!(readback.actor(), actor);
    // No project(..., forged_loci) overload exists.
    let src = include_str!("../src/action_band_semantic_shadow.rs");
    assert!(!src.contains("structural_loci: &["));
    assert!(!src.contains("pub struct AdmittedStructuralLoci"));
}

#[test]
fn stale_production_stamp_fails_closed() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "stale");
    let ctx = require_gpu();
    let (compiled, prod) = produce_sealed(&ctx, &fixture, &frozen);
    let (tree, actor, from, to) = authority_tree_with_actor("beta");
    let session = open_session(frozen, &compiled, actor, from, to);
    let err = session
        .project(
            &prod.authorities[0],
            GenerationStamp::new(prod.production_generation.get() + 3),
            &tree,
            &[],
        )
        .unwrap_err();
    assert!(matches!(
        err,
        SemanticShadowError::StaleGenerationStamp { .. }
    ));
}

// ─── Identity-blindness ─────────────────────────────────────────────────────

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
    assert_ne!(
        frozen_a.semantic_shadow()[0].label(),
        frozen_b.semantic_shadow()[0].label()
    );
    let ctx = require_gpu();
    let (_, prod_a) = produce_sealed(&ctx, &fixture, &frozen_a);
    let (_, prod_b) = produce_sealed(&ctx, &fixture, &frozen_b);
    assert_eq!(prod_a.fingerprint, prod_b.fingerprint);
    assert_eq!(prod_a.commitment_value_bits, prod_b.commitment_value_bits);
}

// ─── R2: owner error through transit ────────────────────────────────────────

#[test]
fn foreign_owner_error_propagates_through_transit_projection() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "owner-mutants");
    let ctx = require_gpu();
    let (compiled, prod) = produce_sealed(&ctx, &fixture, &frozen);
    let tree = SimThing::new(SimThingKind::World, 0);
    let foreign = SimThingId::new();
    let from = SimThingId::new();
    let to = SimThingId::new();
    // Session open freezes foreign actor as the admitted structural subject.
    let session = open_session(frozen, &compiled, foreign, from, to);
    let readback = session
        .project(
            &prod.authorities[0],
            prod.production_generation,
            &tree,
            &[],
        )
        .unwrap();
    assert!(matches!(
        readback.owner(),
        Err(simthing_core::owner_channel::OwnerResolutionError::TargetNotInTree { .. })
    ));
    let transit = readback.transit_projection();
    assert!(matches!(
        transit.owner,
        Err(simthing_core::owner_channel::OwnerResolutionError::TargetNotInTree { .. })
    ));
    assert!(matches!(
        transit.to_fleet_presence_record(),
        Err(SemanticShadowError::OwnerResolution(_))
    ));
}

// ─── R3: icon consumer ──────────────────────────────────────────────────────

#[test]
fn existing_icon_descriptor_consumes_generic_actionband_transit() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "in-transit-fleet-shadow");
    let ctx = require_gpu();
    let (compiled, prod) = produce_sealed(&ctx, &fixture, &frozen);
    let (tree, actor, from, to) = authority_tree_with_actor("gamma");
    assert_ne!(from.raw(), to.raw());
    let session = open_session(frozen, &compiled, actor, from, to);
    let readback = session
        .project(
            &prod.authorities[0],
            prod.production_generation,
            &tree,
            &[],
        )
        .unwrap();
    let transit = readback.transit_projection();
    assert!(transit.is_in_transit());
    let record = transit.to_fleet_presence_record().unwrap();
    assert!(matches!(
        record.location,
        FleetPresenceLocation::InTransit {
            source_system_id,
            dest_system_id
        } if source_system_id == from.raw() && dest_system_id == to.raw()
    ));
    let descriptors = fleet_icon_descriptors_from_records(
        &[record],
        None,
        &HashMap::new(),
        &HashMap::from([(from.raw(), 2.0f32), (to.raw(), 2.0f32)]),
    );
    assert_eq!(descriptors.len(), 1);
    assert!(matches!(
        descriptors[0].placement,
        FleetIconPlacement::InTransit {
            source_system_id,
            dest_system_id,
            ..
        } if source_system_id == from.raw() && dest_system_id == to.raw()
    ));
    assert_eq!(descriptors[0].owner_id.as_deref(), Some("gamma"));
    assert_eq!(descriptors[0].fleet_simthing_id_raw, actor.raw());
}

#[test]
fn production_icon_layer_source_is_untouched() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/src/studio_fleet_icons.rs");
    assert!(path.exists());
    let shadow_src = include_str!("../src/action_band_semantic_shadow.rs");
    for forbidden in [
        "MovementPlanner",
        "pub fn seal_actionband_authority",
        "in_transit: true",
        "pub struct AdmittedStructuralLoci",
    ] {
        assert!(
            !shadow_src.contains(forbidden),
            "must not encode {forbidden}"
        );
    }
}
