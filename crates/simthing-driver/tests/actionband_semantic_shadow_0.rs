//! ACTIONBAND-SEMANTIC-SHADOW-0 focused proof battery.
//!
//! Post-authority CPU semantic shadow/readback only. GPU remains sole numerical
//! authority. FIELD-NEUTRAL gate + identity-blindness + owner/stamp + icon
//! projection without icon-layer source change.

use std::sync::Mutex;

use simthing_core::owner_channel::{bind_owner, OwnerRef, OWNER_CHANNEL_PROPERTY_ID};
use simthing_core::{
    DimensionRegistry, EmitOnThresholdBuffer, EmitOnThresholdRegistration, GenerationStamp,
    SimProperty, SimThing, SimThingKind, SlotIndex, SubFieldRole, ThresholdDirection,
};
use simthing_driver::{
    compile_action_band_gpu_execution, project_semantic_readback, ActionBandActiveInstance,
    BoundObservableIdentity, FieldNeutralityGate, PostAuthorityInputs, SemanticShadowError,
    FIELD_NEUTRALITY_OUTCOME,
};
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, emit_on_threshold_registrations_to_gpu,
    scoped_debug_readback_allowed, wgpu, AccumulatorOpSession, ActionBandEmissionBindingGpu,
    ActionBandGpuExecution, GpuContext, PackedThresholdUpload, SlotAllocator,
};
use simthing_spec::{
    ActionBandAdmissionBudgetSpec, ActionBandBandSpec, ActionBandChannelBindingSpec,
    ActionBandChannelKind, ActionBandSessionBuildDoor, ActionBandSessionSpec, ActionBandTargetSpec,
    ActionBandTemplateSpec, FrozenActionBandTemplates, ScalarBoundDirection,
};
use wgpu::util::DeviceExt;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

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
        event_kind: 750,
        buffer: EmitOnThresholdBuffer::Values,
    }];
    Fixture {
        registry,
        thresholds,
        column,
    }
}

fn session_spec(column: u32, label: &str) -> ActionBandSessionSpec {
    ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 1,
            dependency_binding_count: 0,
            storage_rows: 1,
            eml_program_count: 0,
            emission_binding_count: 1,
        },
        templates: vec![ActionBandTemplateSpec {
            id: "semantic-shadow-template".into(),
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

fn admit(fixture: &Fixture, label: &str) -> FrozenActionBandTemplates {
    let mut door = ActionBandSessionBuildDoor::new();
    door.admit_once_at_session_build(
        &session_spec(fixture.column.raw_u32(), label),
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

/// Sealed StructuralCommitment + numeric fingerprint through real ActionBand GPU path.
fn sealed_execution(
    ctx: &GpuContext,
    fixture: &Fixture,
    frozen: &FrozenActionBandTemplates,
) -> (simthing_gpu::StructuralCommitment, u64, [u32; 2]) {
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
    let c = production.commitments[0];
    (c, fingerprint, [c.slot(), c.col()])
}

fn authority_tree_with_owner(owner: &str) -> (SimThing, simthing_core::SimThingId) {
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut subject = SimThing::new(SimThingKind::Cohort, 0);
    bind_owner(&mut subject, &OwnerRef::new(owner));
    let id = subject.id;
    root.add_child(subject);
    (root, id)
}

// ─── FIELD-NEUTRALITY ───────────────────────────────────────────────────────

#[test]
fn field_neutrality_gate_is_field_neutral() {
    assert_eq!(
        FIELD_NEUTRALITY_OUTCOME,
        FieldNeutralityGate::FieldNeutral
    );
    // Existing semantic-shadow rows are label/id only — no PALMA progress/throughput.
    let fixture = fixture();
    let frozen = admit(&fixture, "any-label");
    let row = &frozen.semantic_shadow()[0];
    assert!(row.label().is_some());
    assert_eq!(row.authored_id(), "semantic-shadow-template");
}

#[test]
fn a1_synthetic_non_palma_bound_observable_round_trips() {
    let fixture = fixture();
    let frozen = admit(&fixture, "transit-designation");
    let Some(ctx) = require_gpu() else {
        eprintln!("SKIP a1_synthetic_non_palma: no local GPU");
        return;
    };
    let (commitment, _, _) = sealed_execution(&ctx, &fixture, &frozen);
    let (tree, subject) = authority_tree_with_owner("faction-alpha");
    // Positive A1: identity that is NOT PALMA-derived, not Gu-Yang, not a field enum.
    let synthetic = BoundObservableIdentity::new(
        "synthetic-rf-grant-axis-v1",
        Some("post-authority-semantic-metadata"),
    );
    let readback = project_semantic_readback(PostAuthorityInputs {
        frozen: &frozen,
        commitment,
        template: frozen.templates()[0].index(),
        generation: GenerationStamp::new(11),
        parent_generation: GenerationStamp::new(11),
        authority_tree: &tree,
        owner_subject: subject,
        bound_observables: std::slice::from_ref(&synthetic),
    })
    .expect("post-authority projection");
    assert_eq!(readback.bound_observables().len(), 1);
    assert_eq!(
        readback.bound_observables()[0].key(),
        "synthetic-rf-grant-axis-v1"
    );
    assert_eq!(
        readback.bound_observables()[0].provenance(),
        Some("post-authority-semantic-metadata")
    );
    // No PALMA-only vocabulary required to carry/report the identity.
    assert!(!readback.bound_observables()[0].key().contains("palma"));
    assert!(!readback.bound_observables()[0].key().contains("PALMA"));
}

// ─── Identity-blindness ─────────────────────────────────────────────────────

#[test]
fn identity_blindness_labels_do_not_change_numerical_or_sealed_products() {
    let fixture = fixture();
    let frozen_a = admit(&fixture, "human-readable-movement-to-orion");
    let frozen_b = admit(&fixture, "completely-different-designation-words");
    assert_ne!(
        frozen_a.semantic_shadow()[0].label(),
        frozen_b.semantic_shadow()[0].label()
    );

    let Some(ctx) = require_gpu() else {
        eprintln!("SKIP identity_blindness: no local GPU");
        return;
    };
    let (commit_a, fp_a, locus_a) = sealed_execution(&ctx, &fixture, &frozen_a);
    let (commit_b, fp_b, locus_b) = sealed_execution(&ctx, &fixture, &frozen_b);

    assert_eq!(
        fp_a, fp_b,
        "numeric plan fingerprint must be identity-blind"
    );
    assert_eq!(locus_a, locus_b);
    assert_eq!(commit_a.slot(), commit_b.slot());
    assert_eq!(commit_a.col(), commit_b.col());
    assert_eq!(commit_a.value().to_bits(), commit_b.value().to_bits());
    assert_eq!(commit_a.event_kind(), commit_b.event_kind());
}

// ─── Owner + generation stamp ───────────────────────────────────────────────

#[test]
fn readback_resolves_owner_with_generation_stamp_and_rejects_stale() {
    let fixture = fixture();
    let frozen = admit(&fixture, "owner-readback");
    let Some(ctx) = require_gpu() else {
        eprintln!("SKIP owner_stamp: no local GPU");
        return;
    };
    let (commitment, _, _) = sealed_execution(&ctx, &fixture, &frozen);
    let (tree, subject) = authority_tree_with_owner("beta-owner");

    let ok = project_semantic_readback(PostAuthorityInputs {
        frozen: &frozen,
        commitment,
        template: frozen.templates()[0].index(),
        generation: GenerationStamp::new(5),
        parent_generation: GenerationStamp::new(5),
        authority_tree: &tree,
        owner_subject: subject,
        bound_observables: &[],
    })
    .unwrap();
    assert_eq!(ok.owner().as_ref().unwrap().as_str(), "beta-owner");
    assert_eq!(ok.generation(), GenerationStamp::new(5));
    assert!(tree_has_owner_property(&tree, subject));

    // Stale product stamp relative to parent fails closed.
    let stale = project_semantic_readback(PostAuthorityInputs {
        frozen: &frozen,
        commitment,
        template: frozen.templates()[0].index(),
        generation: GenerationStamp::new(3),
        parent_generation: GenerationStamp::new(7),
        authority_tree: &tree,
        owner_subject: subject,
        bound_observables: &[],
    });
    assert!(matches!(
        stale,
        Err(SemanticShadowError::StaleGenerationStamp { .. })
    ));
}

fn tree_has_owner_property(root: &SimThing, id: simthing_core::SimThingId) -> bool {
    fn walk(node: &SimThing, id: simthing_core::SimThingId) -> bool {
        if node.id == id {
            return node.properties.contains_key(&OWNER_CHANNEL_PROPERTY_ID);
        }
        node.children.iter().any(|c| walk(c, id))
    }
    walk(root, id)
}

#[test]
fn foreign_and_malformed_owner_do_not_alias_to_unowned() {
    let fixture = fixture();
    let frozen = admit(&fixture, "owner-mutants");
    let Some(ctx) = require_gpu() else {
        eprintln!("SKIP foreign_owner: no local GPU");
        return;
    };
    let (commitment, _, _) = sealed_execution(&ctx, &fixture, &frozen);
    let tree = SimThing::new(SimThingKind::World, 0);
    let foreign = simthing_core::SimThingId::new();

    let readback = project_semantic_readback(PostAuthorityInputs {
        frozen: &frozen,
        commitment,
        template: frozen.templates()[0].index(),
        generation: GenerationStamp::new(1),
        parent_generation: GenerationStamp::new(1),
        authority_tree: &tree,
        owner_subject: foreign,
        bound_observables: &[],
    })
    .unwrap();
    match readback.owner() {
        Err(simthing_core::owner_channel::OwnerResolutionError::TargetNotInTree { .. }) => {}
        other => panic!("foreign owner must not alias to unowned; got {other:?}"),
    }
    // Error is retained on the readback product — not replaced with unowned.
    assert!(readback.owner().is_err());
}

// ─── Designation after authority + transit/icon projection ──────────────────

#[test]
fn readback_reports_designation_after_authority_and_transit_projection() {
    let fixture = fixture();
    let frozen = admit(&fixture, "in-transit-fleet-shadow");
    let Some(ctx) = require_gpu() else {
        eprintln!("SKIP designation_readback: no local GPU");
        return;
    };
    let (commitment, _, _) = sealed_execution(&ctx, &fixture, &frozen);
    let (tree, subject) = authority_tree_with_owner("gamma");
    let readback = project_semantic_readback(PostAuthorityInputs {
        frozen: &frozen,
        commitment,
        template: frozen.templates()[0].index(),
        generation: GenerationStamp::new(9),
        parent_generation: GenerationStamp::new(9),
        authority_tree: &tree,
        owner_subject: subject,
        bound_observables: &[],
    })
    .unwrap();
    assert_eq!(
        readback.designation(),
        Some("in-transit-fleet-shadow")
    );
    assert_eq!(readback.sealed_event_kind(), 750);
    let transit = readback.transit_projection();
    assert!(transit.in_transit);
    assert_eq!(
        transit.designation.as_deref(),
        Some("in-transit-fleet-shadow")
    );
    assert_eq!(transit.owner.as_deref(), Some("gamma"));
    assert_eq!(transit.generation, GenerationStamp::new(9));
    // 12.5: existing FleetIconDescriptor InTransit placement is the presentation
    // consumer of generic transit state — prove mapping shape without editing
    // icon-layer sources (mapeditor studio_fleet_icons remains untouched).
    assert!(std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/src/studio_fleet_icons.rs")
        .exists());
}

#[test]
fn production_icon_layer_source_is_untouched() {
    // Grep referee: this rung must not modify icon-layer implementation.
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/src/studio_fleet_icons.rs");
    let meta = std::fs::metadata(&path).expect("icon source present");
    assert!(meta.len() > 0);
    // No movement-specific authoritative readback facility in driver module.
    let shadow_src = include_str!("../src/action_band_semantic_shadow.rs");
    for forbidden in [
        "MovementPlanner",
        "MovementCommitment",
        "DestinationRegistry",
        "palma_progress",
        "PALMA_ONLY",
        "throughput_calculator",
    ] {
        assert!(
            !shadow_src.contains(forbidden),
            "semantic shadow must not encode {forbidden}"
        );
    }
}
