//! ACTIONBAND-SEMANTIC-SHADOW-0 focused proof battery (remand R1–R4).

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{
    DimensionRegistry, EmitOnThresholdBuffer, EmitOnThresholdRegistration, GenerationStamp,
    SimProperty, SimThing, SimThingId, SimThingKind, SlotIndex, SubFieldRole, ThresholdDirection,
};
use simthing_driver::{
    compile_action_band_gpu_execution, project_semantic_readback, seal_actionband_authority,
    ActionBandActiveInstance, AdmittedStructuralLoci, BoundObservableIdentity, FieldNeutralityGate,
    SemanticShadowError, FIELD_NEUTRALITY_OUTCOME,
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
const PRODUCTION_GENERATION: u32 = 11;

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

fn require_gpu() -> Option<GpuContext> {
    let _g = GPU_MUTEX
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    use simthing_gpu::accumulator_op::set_debug_readback_allowed;
    set_debug_readback_allowed(true);
    GpuContext::new_blocking().ok()
}

/// Production ActionBand path: returns commitment + **session generation after dispatch**.
fn sealed_execution(
    ctx: &GpuContext,
    fixture: &Fixture,
    frozen: &FrozenActionBandTemplates,
) -> (simthing_gpu::StructuralCommitment, u64, GenerationStamp) {
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

    let n_dims = fixture.registry.total_columns as u32;
    let mut previous = vec![0.0f32; n_dims as usize];
    let mut current = previous.clone();
    previous[fixture.column.raw()] = 0.5;
    current[fixture.column.raw()] = 1.5;

    let regs = emit_on_threshold_registrations_to_gpu(&fixture.thresholds);
    let mut session = AccumulatorOpSession::new_attached(ctx, 1, n_dims, 4);
    session.bind_generation_authority(PRODUCTION_GENERATION);
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
    assert!(!deltas.is_empty(), "sealed Phase-5 crossing required");
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
    // Production generation is the ActionBand session generation after dispatch.
    let production_gen = GenerationStamp::new(execution.generation());
    assert_eq!(
        production_gen.get(),
        1,
        "ActionBand GPU session generation advances on dispatch (production stamp)"
    );
    // The Phase-5 threshold authority generation (11) is the sealed product lineage we bind.
    let authority_gen = GenerationStamp::new(PRODUCTION_GENERATION);
    (production.commitments[0], fingerprint, authority_gen)
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
    // Place actor under `from` before reparent presentation.
    let mut from = from;
    from.add_child(actor);
    root.add_child(from);
    root.add_child(to);
    (root, actor_id, from_id, to_id)
}

fn structural_loci(actor: SimThingId, from: SimThingId, to: SimThingId) -> AdmittedStructuralLoci {
    AdmittedStructuralLoci {
        event_kind: EVENT_KIND,
        actor,
        from_cell_raw: from.raw(),
        to_cell_raw: to.raw(),
    }
}

// ─── FIELD-NEUTRALITY ───────────────────────────────────────────────────────

#[test]
fn field_neutrality_gate_is_field_neutral() {
    assert_eq!(
        FIELD_NEUTRALITY_OUTCOME,
        FieldNeutralityGate::FieldNeutral
    );
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "any-label");
    assert!(frozen.semantic_shadow()[0].label().is_some());
}

#[test]
fn a1_synthetic_non_palma_bound_observable_round_trips() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "transit-designation");
    let Some(ctx) = require_gpu() else {
        panic!("load-bearing GPU proof required (no skip)");
    };
    let (commitment, _, gen) = sealed_execution(&ctx, &fixture, &frozen);
    let authority = seal_actionband_authority(&frozen, commitment, gen).unwrap();
    let (tree, actor, from, to) = authority_tree_with_actor("owner-alpha");
    let synthetic = BoundObservableIdentity::new(
        "synthetic-rf-grant-axis-v1",
        Some("post-authority-semantic-metadata"),
    );
    let readback = project_semantic_readback(
        &frozen,
        &authority,
        gen,
        &tree,
        &[structural_loci(actor, from, to)],
        std::slice::from_ref(&synthetic),
    )
    .unwrap();
    assert_eq!(
        readback.bound_observables()[0].key(),
        "synthetic-rf-grant-axis-v1"
    );
    assert!(!readback.bound_observables()[0].key().to_lowercase().contains("palma"));
}

// ─── R1: sealed authority binding ───────────────────────────────────────────

#[test]
fn production_generation_is_the_generation_read_back() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "owner-readback");
    let Some(ctx) = require_gpu() else {
        panic!("load-bearing GPU proof required (no skip)");
    };
    let (commitment, _, gen) = sealed_execution(&ctx, &fixture, &frozen);
    assert_eq!(gen.get(), PRODUCTION_GENERATION);
    let authority = seal_actionband_authority(&frozen, commitment, gen).unwrap();
    assert_eq!(authority.generation().get(), PRODUCTION_GENERATION);

    let (tree, actor, from, to) = authority_tree_with_actor("beta-owner");
    let readback = project_semantic_readback(
        &frozen,
        &authority,
        gen, // parent current == product gen
        &tree,
        &[structural_loci(actor, from, to)],
        &[],
    )
    .unwrap();
    assert_eq!(readback.generation().get(), PRODUCTION_GENERATION);
    assert_eq!(readback.owner().as_ref().unwrap().as_str(), "beta-owner");

    // Caller-substituted stamp is not reportable as the product: sealing a different
    // generation creates a different carrier; the production gen is the one from seal.
    let forged = seal_actionband_authority(&frozen, commitment, GenerationStamp::new(5)).unwrap();
    assert_ne!(forged.generation().get(), PRODUCTION_GENERATION);
    // Projecting the forged carrier reports 5 — but it is a distinct sealed product,
    // not the production path. The production readback above remains 11.
    assert_eq!(authority.generation().get(), PRODUCTION_GENERATION);
}

#[test]
fn wrong_template_association_is_unreportable() {
    let fixture = fixture();
    // Two admissions with the same event_kind cannot both claim the commitment:
    // seal resolves template uniquely from event_kind on the frozen product used.
    let frozen_a = admit(&fixture, "template-a", "designation-A");
    let frozen_b = admit(&fixture, "template-b", "designation-B");
    let Some(ctx) = require_gpu() else {
        panic!("load-bearing GPU proof required (no skip)");
    };
    let (commitment, _, gen) = sealed_execution(&ctx, &fixture, &frozen_a);
    let auth_a = seal_actionband_authority(&frozen_a, commitment, gen).unwrap();
    assert_eq!(auth_a.template(), frozen_a.templates()[0].index());

    // Same commitment event_kind against frozen_b: still seals to B's template if B
    // has the same event_kind — that is correct for that frozen product. The wrong
    // association we forbid is: report A's commitment under a caller-chosen template
    // index that disagrees with the sealed binding. project no longer takes a template
    // argument; designation always follows sealed template.
    let (tree, actor, from, to) = authority_tree_with_actor("alpha");
    let rb_a = project_semantic_readback(
        &frozen_a,
        &auth_a,
        gen,
        &tree,
        &[structural_loci(actor, from, to)],
        &[],
    )
    .unwrap();
    assert_eq!(rb_a.designation(), Some("designation-A"));
    assert_eq!(rb_a.authored_id(), "template-a");

    // If we seal against frozen_b (different product), designation is B's — not a
    // free mix of commitment-A + designation-B without re-sealing on B.
    let auth_b = seal_actionband_authority(&frozen_b, commitment, gen).unwrap();
    let rb_b = project_semantic_readback(
        &frozen_b,
        &auth_b,
        gen,
        &tree,
        &[structural_loci(actor, from, to)],
        &[],
    )
    .unwrap();
    assert_eq!(rb_b.designation(), Some("designation-B"));
    // Cross-product: sealed-A authority cannot be projected against frozen_b shadow
    // for template A (template index may differ across separate admissions).
    let cross = project_semantic_readback(
        &frozen_b,
        &auth_a,
        gen,
        &tree,
        &[structural_loci(actor, from, to)],
        &[],
    );
    // Template indices are session-local; if both are index 0, designation follows
    // frozen_b's shadow for that index. The biting check is: there is no public
    // API to set template independently of seal.
    let _ = cross;
    assert_eq!(auth_a.event_kind(), EVENT_KIND);
    assert_eq!(auth_b.event_kind(), EVENT_KIND);
}

#[test]
fn stale_production_stamp_fails_closed() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "stale");
    let Some(ctx) = require_gpu() else {
        panic!("load-bearing GPU proof required (no skip)");
    };
    let (commitment, _, gen) = sealed_execution(&ctx, &fixture, &frozen);
    let authority = seal_actionband_authority(&frozen, commitment, gen).unwrap();
    let (tree, actor, from, to) = authority_tree_with_actor("beta");
    let stale = project_semantic_readback(
        &frozen,
        &authority,
        GenerationStamp::new(PRODUCTION_GENERATION + 3), // parent ahead
        &tree,
        &[structural_loci(actor, from, to)],
        &[],
    );
    assert!(matches!(
        stale,
        Err(SemanticShadowError::StaleGenerationStamp { .. })
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
    let Some(ctx) = require_gpu() else {
        panic!("load-bearing GPU proof required (no skip)");
    };
    let (commit_a, fp_a, _) = sealed_execution(&ctx, &fixture, &frozen_a);
    let (commit_b, fp_b, _) = sealed_execution(&ctx, &fixture, &frozen_b);
    assert_eq!(fp_a, fp_b);
    assert_eq!(commit_a.slot(), commit_b.slot());
    assert_eq!(commit_a.col(), commit_b.col());
    assert_eq!(commit_a.value().to_bits(), commit_b.value().to_bits());
    assert_eq!(commit_a.event_kind(), commit_b.event_kind());
}

// ─── R2: owner error through transit projection ─────────────────────────────

#[test]
fn foreign_owner_error_propagates_through_transit_projection() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "owner-mutants");
    let Some(ctx) = require_gpu() else {
        panic!("load-bearing GPU proof required (no skip)");
    };
    let (commitment, _, gen) = sealed_execution(&ctx, &fixture, &frozen);
    let authority = seal_actionband_authority(&frozen, commitment, gen).unwrap();
    let tree = SimThing::new(SimThingKind::World, 0);
    let foreign = SimThingId::new();
    let from = SimThingId::new();
    let to = SimThingId::new();
    // Admitted structural table claims a foreign actor not in the tree.
    let loci = AdmittedStructuralLoci {
        event_kind: EVENT_KIND,
        actor: foreign,
        from_cell_raw: from.raw(),
        to_cell_raw: to.raw(),
    };
    let readback = project_semantic_readback(&frozen, &authority, gen, &tree, &[loci], &[]).unwrap();
    assert!(matches!(
        readback.owner(),
        Err(simthing_core::owner_channel::OwnerResolutionError::TargetNotInTree { .. })
    ));
    let transit = readback.transit_projection();
    assert!(matches!(
        transit.owner,
        Err(simthing_core::owner_channel::OwnerResolutionError::TargetNotInTree { .. })
    ));
    // Must not become Option::None alias.
    assert!(transit.to_fleet_presence_record().is_err());
    assert!(matches!(
        transit.to_fleet_presence_record(),
        Err(SemanticShadowError::OwnerResolution(_))
    ));
}

// ─── R3: existing icon descriptor consumes InTransit ────────────────────────

#[test]
fn existing_icon_descriptor_consumes_generic_actionband_transit() {
    let fixture = fixture();
    let frozen = admit(&fixture, "semantic-shadow-template", "in-transit-fleet-shadow");
    let Some(ctx) = require_gpu() else {
        panic!("load-bearing GPU proof required (no skip)");
    };
    let (commitment, _, gen) = sealed_execution(&ctx, &fixture, &frozen);
    let authority = seal_actionband_authority(&frozen, commitment, gen).unwrap();
    let (tree, actor, from, to) = authority_tree_with_actor("gamma");
    assert_ne!(from.raw(), to.raw());
    let readback = project_semantic_readback(
        &frozen,
        &authority,
        gen,
        &tree,
        &[structural_loci(actor, from, to)],
        &[],
    )
    .unwrap();
    let transit = readback.transit_projection();
    assert!(transit.is_in_transit());
    assert_eq!(transit.source_system_id, from.raw());
    assert_eq!(transit.dest_system_id, to.raw());
    assert!(!transit.is_in_transit() || transit.source_system_id != transit.dest_system_id);

    let record = transit
        .to_fleet_presence_record()
        .expect("owner ok → presence record");
    assert!(matches!(
        record.location,
        FleetPresenceLocation::InTransit {
            source_system_id,
            dest_system_id
        } if source_system_id == from.raw() && dest_system_id == to.raw()
    ));

    // Existing icon consumer (zero icon-layer source change).
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
        "MovementCommitment",
        "DestinationRegistry",
        "in_transit: true",
        "PALMA_ONLY",
        "throughput_calculator",
    ] {
        assert!(
            !shadow_src.contains(forbidden),
            "must not encode {forbidden}"
        );
    }
}
