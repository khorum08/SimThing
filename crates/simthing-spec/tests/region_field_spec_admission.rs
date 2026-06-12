//! Phase M-3 — RegionFieldSpec RON roundtrip and mapping admission framework.

use simthing_core::ColumnAwareReductionCombine;
use simthing_driver::field_scheduler::FieldCadence;
use simthing_gpu::{
    StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy,
};
use simthing_sim::PipelineFlags;
use simthing_spec::{
    admit_region_field_formula_class, compile_region_field_frame_preview,
    compile_region_field_preview, compile_region_field_stencil_config,
    deserialize_region_field_ron, validate_region_field_frame_gradient_sinks, CompiledFieldCadence,
    CompiledRegionFieldMaskMode, CompiledRegionFieldOperator, CompiledRegionFieldSourcePolicy,
    FirstSliceCommitmentDirectionSpec, FirstSliceCommitmentSpec, GameModeSpec, GradientAxisSpec,
    MappingExecutionProfile, RegionFieldCadenceSpec, RegionFieldFormulaBindingSpec,
    RegionFieldGridProfile, RegionFieldOperatorSpec, RegionFieldReductionSpec,
    RegionFieldSourcePolicySpec, RegionFieldSpec, ResourceFlowExecutionProfile, SpecError,
};

fn standard_suppression_field() -> RegionFieldSpec {
    RegionFieldSpec {
        name: "suppression_field".into(),
        grid_size: 10,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        operator: RegionFieldOperatorSpec::SourceCappedNormalized,
        horizon: 8,
        allow_extended_horizon: false,
        alpha_self: 1.0,
        gamma_neighbor: 0.8,
        source_cap: Some(500.0),
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::StandardSquare,
        reduction: None,
        parent_formula: None,
        commitment: None,
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: Default::default(),
        pressure_binding: None,
    }
}

fn to_gpu_stencil_config(
    compiled: &simthing_spec::CompiledRegionFieldStencilSpec,
) -> StructuredFieldStencilConfig {
    StructuredFieldStencilConfig {
        width: compiled.width,
        height: compiled.height,
        n_dims: compiled.n_dims,
        source_col: compiled.source_col,
        target_col: compiled.target_col,
        horizon: compiled.horizon,
        alpha_self: compiled.alpha_self,
        gamma_neighbor: compiled.gamma_neighbor,
        weight_north: compiled.weight_north,
        weight_south: compiled.weight_south,
        weight_east: compiled.weight_east,
        weight_west: compiled.weight_west,
        source_cap: compiled.source_cap,
        operator: match compiled.operator {
            CompiledRegionFieldOperator::Normalized => StructuredFieldStencilOperator::Normalized,
            CompiledRegionFieldOperator::SourceCappedNormalized => {
                StructuredFieldStencilOperator::SourceCappedNormalized
            }
            CompiledRegionFieldOperator::Gradient { axis } => match axis {
                simthing_spec::CompiledGradientAxis::X => StructuredFieldStencilOperator::GradientX,
                simthing_spec::CompiledGradientAxis::Y => StructuredFieldStencilOperator::GradientY,
            },
            CompiledRegionFieldOperator::SaturatingFlux {
                u_sat,
                chi,
                choke_output_col,
            } => StructuredFieldStencilOperator::SaturatingFlux {
                u_sat,
                chi,
                choke_output_col,
            },
        },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: compiled.allow_extended_horizon,
    }
}

fn to_field_cadence(compiled: CompiledFieldCadence) -> FieldCadence {
    match compiled {
        CompiledFieldCadence::EveryTick => FieldCadence::EveryTick,
        CompiledFieldCadence::EveryN { n } => FieldCadence::EveryN { n },
        CompiledFieldCadence::OnEvent => FieldCadence::OnEvent,
    }
}

fn assert_region_field_err(spec: &RegionFieldSpec, reason_substr: &str) {
    let err = compile_region_field_preview(spec).expect_err("expected admission failure");
    match err {
        SpecError::RegionFieldAdmission { reason, .. } => {
            assert!(
                reason.contains(reason_substr),
                "expected `{reason_substr}` in `{reason}`"
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn test_a_ron_roundtrip_valid_standard_field() {
    let ron_text = r#"
        (
            name: "suppression_field",
            grid_size: 10,
            n_dims: 4,
            source_col: 0,
            target_col: 0,
            operator: SourceCappedNormalized,
            horizon: 8,
            allow_extended_horizon: false,
            alpha_self: 1.0,
            gamma_neighbor: 0.8,
            source_cap: Some(500.0),
            source_policy: CallerManagedOneShotSeedThenZero,
            cadence: EveryTick,
        )
    "#;

    let parsed = deserialize_region_field_ron(ron_text).expect("parse RON");
    let reserialized = ron::ser::to_string(&parsed).expect("serialize");
    let reparsed = deserialize_region_field_ron(&reserialized).expect("reparse");
    assert_eq!(reparsed, parsed);

    let preview = compile_region_field_preview(&parsed).expect("admit");
    assert_eq!(preview.stencil.width, 10);
    assert_eq!(preview.stencil.height, 10);
    assert_eq!(preview.stencil.mask_mode, CompiledRegionFieldMaskMode::All);
    assert_eq!(
        preview.stencil.source_policy,
        CompiledRegionFieldSourcePolicy::CallerManagedOneShotSeedThenZero
    );

    let gpu_cfg = to_gpu_stencil_config(&preview.stencil);
    gpu_cfg.validate().expect("gpu stencil config validates");
}

#[test]
fn test_b_square_grid_admission() {
    for size in [5u32, 10] {
        let mut spec = standard_suppression_field();
        spec.grid_size = size;
        let preview = compile_region_field_preview(&spec).expect("admit");
        assert_eq!(preview.stencil.width, size);
        assert_eq!(preview.stencil.height, size);
        assert_eq!(preview.cell_count, size * size);
    }

    let mut extended = standard_suppression_field();
    extended.grid_size = 32;
    extended.grid_profile = RegionFieldGridProfile::ExtendedSquare;
    compile_region_field_preview(&extended).expect("extended 32");

    let mut over_standard = standard_suppression_field();
    over_standard.grid_size = 20;
    assert_region_field_err(&over_standard, "exceeds profile cap");

    let mut zero = standard_suppression_field();
    zero.grid_size = 0;
    assert_region_field_err(&zero, "grid_size must be > 0");

    let mut over_extended = standard_suppression_field();
    over_extended.grid_size = 33;
    over_extended.grid_profile = RegionFieldGridProfile::ExtendedSquare;
    assert_region_field_err(&over_extended, "exceeds profile cap");

    let ron = ron::ser::to_string(&standard_suppression_field()).expect("serialize");
    assert!(!ron.contains("width"));
    assert!(!ron.contains("height"));
}

#[test]
fn test_c_operator_and_source_validation() {
    let mut normalized = standard_suppression_field();
    normalized.operator = RegionFieldOperatorSpec::Normalized;
    normalized.source_cap = None;
    compile_region_field_preview(&normalized).expect("normalized without cap");

    compile_region_field_preview(&standard_suppression_field()).expect("source capped with cap");

    let mut missing_cap = standard_suppression_field();
    missing_cap.source_cap = None;
    assert_region_field_err(&missing_cap, "requires source_cap");

    let mut cap_on_normalized = standard_suppression_field();
    cap_on_normalized.operator = RegionFieldOperatorSpec::Normalized;
    cap_on_normalized.source_cap = Some(1.0);
    assert_region_field_err(&cap_on_normalized, "not allowed with Normalized");

    let atlas = r#"(name: "atlas", grid_size: 10, n_dims: 1, source_col: 0, target_col: 0, operator: Normalized, horizon: 1, alpha_self: 1.0, gamma_neighbor: 0.5, cadence: EveryTick, request_atlas_batching: true)"#;
    let atlas_spec = deserialize_region_field_ron(atlas).expect("parse atlas flag");
    assert_region_field_err(&atlas_spec, "atlas batching");
}

#[test]
fn test_d_horizon_admission() {
    for h in [1u32, 8] {
        let mut spec = standard_suppression_field();
        spec.horizon = h;
        compile_region_field_preview(&spec).expect("horizon admit");
    }

    let mut zero = standard_suppression_field();
    zero.horizon = 0;
    assert_region_field_err(&zero, "horizon must be >= 1");

    let mut over_default = standard_suppression_field();
    over_default.horizon = 12;
    assert_region_field_err(&over_default, "without allow_extended_horizon");

    let mut over_absolute = standard_suppression_field();
    over_absolute.horizon = 17;
    over_absolute.allow_extended_horizon = true;
    assert_region_field_err(&over_absolute, "absolute cap");

    let mut extended_no_cap = standard_suppression_field();
    extended_no_cap.horizon = 12;
    extended_no_cap.allow_extended_horizon = true;
    extended_no_cap.operator = RegionFieldOperatorSpec::Normalized;
    extended_no_cap.source_cap = None;
    assert_region_field_err(&extended_no_cap, "extended horizon requires");
}

#[test]
fn test_e_cadence_admission() {
    let cases = [
        (RegionFieldCadenceSpec::EveryTick, FieldCadence::EveryTick),
        (
            RegionFieldCadenceSpec::EveryN(4),
            FieldCadence::EveryN { n: 4 },
        ),
        (
            RegionFieldCadenceSpec::EveryN(10),
            FieldCadence::EveryN { n: 10 },
        ),
        (RegionFieldCadenceSpec::OnEvent, FieldCadence::OnEvent),
    ];

    for (cadence, expected) in cases {
        let mut spec = standard_suppression_field();
        spec.cadence = cadence;
        let preview = compile_region_field_preview(&spec).expect("cadence admit");
        assert_eq!(to_field_cadence(preview.cadence), expected);
    }

    let mut zero_n = standard_suppression_field();
    zero_n.cadence = RegionFieldCadenceSpec::EveryN(0);
    assert_region_field_err(&zero_n, "EveryN cadence requires n > 0");
}

#[test]
fn test_f_reduction_binding_compile() {
    let mut spec = standard_suppression_field();
    spec.reduction = Some(RegionFieldReductionSpec {
        child_slot_start: 1,
        child_slot_count: 9,
        child_col: 2,
        parent_slot: 0,
        parent_col: 3,
        order_band: 4,
    });
    let preview = compile_region_field_preview(&spec).expect("reduction admit");
    let reduction = preview.reduction.expect("reduction present");
    assert_eq!(reduction.child_slot_count, 9);
    assert_eq!(reduction.combine, ColumnAwareReductionCombine::Sum);
    assert_eq!(reduction.order_band, 4);

    let mut zero_children = spec.clone();
    zero_children.reduction = Some(RegionFieldReductionSpec {
        child_slot_count: 0,
        ..spec.reduction.unwrap()
    });
    assert_region_field_err(&zero_children, "child_slot_count must be > 0");
}

#[test]
fn test_g_eml_formula_class_admission() {
    let admitted = [
        "field_pressure",
        "field_urgency",
        "field_decay",
        "bounded_field_update",
        "conversion_rate",
    ];
    for class in admitted {
        let mut spec = standard_suppression_field();
        spec.parent_formula = Some(RegionFieldFormulaBindingSpec {
            formula_class: class.into(),
            tree_id: None,
            weight_pressure: None,
            weight_resource: None,
        });
        admit_region_field_formula_class(&spec, class).expect("admit class");
        compile_region_field_preview(&spec).expect("compile with class");
    }

    for rejected in ["unknown_unbounded_field_formula", "cpu_callback_formula"] {
        let mut spec = standard_suppression_field();
        spec.parent_formula = Some(RegionFieldFormulaBindingSpec {
            formula_class: rejected.into(),
            tree_id: None,
            weight_pressure: None,
            weight_resource: None,
        });
        assert_region_field_err(&spec, "unknown or unbounded");
    }
}

#[test]
fn test_h_default_off_profile() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert_eq!(
        ResourceFlowExecutionProfile::default(),
        ResourceFlowExecutionProfile::DefaultDisabled
    );
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let mode = GameModeSpec {
        id: "mapping_structure_only".into(),
        display_name: String::new(),
        description: String::new(),
        spec_version: Default::default(),
        metadata: Default::default(),
        domain_packs: vec![],
        properties: vec![],
        overlays: vec![],
        capability_trees: vec![],
        events: vec![],
        resource_flow: None,
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: vec![standard_suppression_field()],
        mapping_execution_profile: MappingExecutionProfile::Disabled,
    };
    assert_eq!(
        mode.mapping_execution_profile,
        MappingExecutionProfile::Disabled
    );
    assert!(!mode.mapping_execution_profile.enables_execution());
    compile_region_field_preview(&mode.region_fields[0])
        .expect("structure compiles without execution");
}

#[test]
fn test_i_no_production_runtime_wiring() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let passes = include_str!("../../simthing-gpu/src/passes.rs");
    assert!(!passes.contains("RegionFieldSpec"));
    assert!(!passes.contains("compile_region_field_preview"));

    let session = include_str!("../../simthing-driver/src/session.rs");
    assert!(!session.contains("RegionFieldSpec"));
    assert!(!session.contains("compile_region_field_preview"));
    assert!(!session.contains("mapping_execution_profile"));

    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("RegionFieldSpec"));
    assert!(!sim_lib.contains("RegionField"));
    assert!(!sim_lib.contains("MappingExecutionProfile"));
}

#[test]
fn test_j_first_slice_compile_preview_only() {
    let spec = RegionFieldSpec {
        name: "tactical_suppression".into(),
        grid_size: 10,
        n_dims: 4,
        source_col: 0,
        target_col: 1,
        operator: RegionFieldOperatorSpec::SourceCappedNormalized,
        horizon: 8,
        allow_extended_horizon: false,
        alpha_self: 1.0,
        gamma_neighbor: 0.75,
        source_cap: Some(500.0),
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::StandardSquare,
        reduction: Some(RegionFieldReductionSpec {
            child_slot_start: 1,
            child_slot_count: 100,
            child_col: 1,
            parent_slot: 0,
            parent_col: 2,
            order_band: 0,
        }),
        parent_formula: Some(RegionFieldFormulaBindingSpec {
            formula_class: "field_urgency".into(),
            tree_id: Some(42),
            weight_pressure: None,
            weight_resource: None,
        }),
        commitment: None,
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: Default::default(),
        pressure_binding: None,
    };

    let preview = compile_region_field_preview(&spec).expect("first slice admit");
    assert_eq!(preview.grid_size, 10);
    assert_eq!(preview.stencil.width, 10);
    assert_eq!(preview.stencil.height, 10);
    assert_eq!(
        preview.parent_formula_class.as_deref(),
        Some("field_urgency")
    );
    assert!(preview.reduction.is_some());

    let stencil_only = compile_region_field_stencil_config(&spec).expect("stencil config");
    assert_eq!(stencil_only.width, 10);
    assert_eq!(stencil_only.height, 10);

    let gpu_cfg = to_gpu_stencil_config(&preview.stencil);
    gpu_cfg.validate().expect("gpu config validates");

    assert_eq!(to_field_cadence(preview.cadence), FieldCadence::EveryTick);

    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("RegionField"));
    assert!(!sim_lib.contains("Mapping"));
}

#[test]
fn test_k_first_slice_commitment_spec_admission() {
    let mut spec = standard_suppression_field();
    spec.n_dims = 8;
    spec.reduction = Some(RegionFieldReductionSpec {
        child_slot_start: 0,
        child_slot_count: 100,
        child_col: 0,
        parent_slot: 100,
        parent_col: 0,
        order_band: 0,
    });
    spec.parent_formula = Some(RegionFieldFormulaBindingSpec {
        formula_class: "field_urgency".into(),
        tree_id: Some(7),
        weight_pressure: None,
        weight_resource: None,
    });
    spec.commitment = Some(FirstSliceCommitmentSpec {
        source_formula_class: "field_urgency".into(),
        parent_slot: 100,
        urgency_col: 4,
        threshold: 5490.8657,
        direction: FirstSliceCommitmentDirectionSpec::Upward,
        event_kind: 0x5345_4144,
        effect: None,
    });

    let preview = compile_region_field_preview(&spec).expect("commitment admits");
    let commitment = preview.commitment.expect("compiled commitment");
    assert_eq!(commitment.parent_slot, 100);
    assert_eq!(commitment.urgency_col, 4);
    assert_eq!(commitment.event_kind, 0x5345_4144);

    let mut zero_event = spec.clone();
    zero_event.commitment.as_mut().unwrap().event_kind = 0;
    assert_region_field_err(&zero_event, "commitment event_kind must be nonzero");

    let mut missing_formula = spec.clone();
    missing_formula.parent_formula = None;
    assert_region_field_err(
        &missing_formula,
        "commitment requires parent_formula field_urgency",
    );
}

fn gradient_field(axis: GradientAxisSpec, output_col: u32) -> RegionFieldSpec {
    RegionFieldSpec {
        name: "grad_field".into(),
        grid_size: 3,
        n_dims: 4,
        source_col: 0,
        target_col: output_col,
        operator: RegionFieldOperatorSpec::Gradient { axis, output_col },
        horizon: 1,
        allow_extended_horizon: false,
        alpha_self: 0.0,
        gamma_neighbor: 0.0,
        source_cap: None,
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::StandardSquare,
        reduction: None,
        parent_formula: None,
        commitment: None,
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: Default::default(),
        pressure_binding: None,
    }
}

#[test]
fn m5a_admits_gradient_x() {
    let spec = gradient_field(GradientAxisSpec::X, 1);
    let preview = compile_region_field_preview(&spec).expect("gradient X admits");
    assert_eq!(preview.stencil.target_col, 1);
    assert_eq!(preview.stencil.weight_east, 0.5);
    assert_eq!(preview.stencil.weight_west, -0.5);
    assert_eq!(preview.stencil.weight_north, 0.0);
    assert_eq!(preview.stencil.weight_south, 0.0);
    assert_eq!(preview.stencil.alpha_self, 0.0);
}

#[test]
fn m5a_admits_gradient_y() {
    let spec = gradient_field(GradientAxisSpec::Y, 2);
    let preview = compile_region_field_preview(&spec).expect("gradient Y admits");
    assert_eq!(preview.stencil.target_col, 2);
    assert_eq!(preview.stencil.weight_north, -0.5);
    assert_eq!(preview.stencil.weight_south, 0.5);
}

#[test]
fn m5a_rejects_gradient_output_col_out_of_range() {
    let mut spec = gradient_field(GradientAxisSpec::X, 1);
    spec.operator = RegionFieldOperatorSpec::Gradient {
        axis: GradientAxisSpec::X,
        output_col: 4,
    };
    assert_region_field_err(&spec, "out of range");
}

#[test]
fn m5a_rejects_gradient_same_pass_read_write_loop() {
    let mut spec = gradient_field(GradientAxisSpec::X, 1);
    spec.source_col = 1;
    spec.target_col = 1;
    spec.operator = RegionFieldOperatorSpec::Gradient {
        axis: GradientAxisSpec::X,
        output_col: 1,
    };
    assert_region_field_err(&spec, "same-pass read/write loop");
}

#[test]
fn m5a_rejects_gradient_target_col_mismatch() {
    let mut spec = gradient_field(GradientAxisSpec::X, 1);
    spec.target_col = 2;
    assert_region_field_err(&spec, "must equal operator output_col");
}

#[test]
fn m5a_normalized_behavior_unchanged() {
    let preview = compile_region_field_preview(&standard_suppression_field()).expect("admit");
    let w = preview.stencil.gamma_neighbor / 4.0;
    assert!((preview.stencil.weight_north - w).abs() < 1e-6);
    assert!((preview.stencil.weight_east - w).abs() < 1e-6);
    assert!(matches!(
        preview.stencil.operator,
        CompiledRegionFieldOperator::SourceCappedNormalized
    ));
}

#[test]
fn m5a_mapping_execution_profile_default_disabled() {
    assert!(!MappingExecutionProfile::default().enables_execution());
}

fn assert_frame_gradient_sink_err(fields: &[&RegionFieldSpec], reason_substr: &str) {
    let err = validate_region_field_frame_gradient_sinks(fields).expect_err("expected rejection");
    match err {
        SpecError::RegionFieldAdmission { reason, .. } => {
            assert!(
                reason.contains(reason_substr),
                "expected `{reason_substr}` in `{reason}`"
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn m5d_rejects_normalized_field_using_gradient_output_as_source_col() {
    let grad = gradient_field(GradientAxisSpec::X, 1);
    let mut consumer = standard_suppression_field();
    consumer.name = "consumer_field".into();
    consumer.source_col = 1;
    assert_frame_gradient_sink_err(
        &[&grad, &consumer],
        "cannot be used as same-frame source_col",
    );
}

#[test]
fn m5d_rejects_gradient_field_using_another_gradient_output_as_source_col() {
    let grad_x = gradient_field(GradientAxisSpec::X, 1);
    let mut grad_y = gradient_field(GradientAxisSpec::Y, 2);
    grad_y.name = "grad_y_consumer".into();
    grad_y.source_col = 1;
    assert_frame_gradient_sink_err(
        &[&grad_x, &grad_y],
        "gradient output_col 1 from `grad_field`",
    );
}

#[test]
fn m5d_rejects_base_field_using_gradient_output_as_source_col() {
    let grad = gradient_field(GradientAxisSpec::X, 1);
    let mut base = standard_suppression_field();
    base.name = "base_diffusion".into();
    base.source_col = 1;
    assert_frame_gradient_sink_err(&[&grad, &base], "cannot be used as same-frame source_col");
}

#[test]
fn m5d_reaffirms_gradient_self_loop_at_frame_level() {
    let mut grad = gradient_field(GradientAxisSpec::X, 1);
    grad.source_col = 1;
    grad.target_col = 1;
    grad.operator = RegionFieldOperatorSpec::Gradient {
        axis: GradientAxisSpec::X,
        output_col: 1,
    };
    assert_frame_gradient_sink_err(&[&grad], "same-pass read/write loop");
}

#[test]
fn m5d_compile_region_field_frame_preview_admits_valid_group() {
    let scalar = standard_suppression_field();
    let grad_x = gradient_field(GradientAxisSpec::X, 1);
    let grad_y = gradient_field(GradientAxisSpec::Y, 2);
    let previews =
        compile_region_field_frame_preview(&[&scalar, &grad_x, &grad_y]).expect("frame compiles");
    assert_eq!(previews.len(), 3);
}

#[test]
fn m5d_admits_m5b_style_valid_gradient_sinks() {
    let scalar = standard_suppression_field();
    let grad_x = gradient_field(GradientAxisSpec::X, 1);
    let grad_y = gradient_field(GradientAxisSpec::Y, 2);
    validate_region_field_frame_gradient_sinks(&[&scalar, &grad_x, &grad_y])
        .expect("M-5B-style frame admits");
    for spec in [&scalar, &grad_x, &grad_y] {
        compile_region_field_preview(spec).expect("each field admits individually");
    }
}

#[test]
fn m5d_admits_m5c_fixture_rons_under_frame_validation() {
    let scalar = deserialize_region_field_ron(include_str!(
        "../../simthing-driver/tests/fixtures/m5c_need_signal_scalar_field.ron"
    ))
    .expect("scalar RON");
    let gx = deserialize_region_field_ron(include_str!(
        "../../simthing-driver/tests/fixtures/m5c_need_signal_gradient_x_field.ron"
    ))
    .expect("gx RON");
    let gy = deserialize_region_field_ron(include_str!(
        "../../simthing-driver/tests/fixtures/m5c_need_signal_gradient_y_field.ron"
    ))
    .expect("gy RON");
    let previews = compile_region_field_frame_preview(&[&scalar, &gx, &gy])
        .expect("M-5C fixture frame compiles");
    assert_eq!(previews.len(), 3);
}

#[test]
fn m5d_validator_checks_same_frame_only_not_cross_tick() {
    // Tick N frame: scalar source 0, gradients sink to 1/2 — valid.
    let scalar = standard_suppression_field();
    let grad_x = gradient_field(GradientAxisSpec::X, 1);
    let grad_y = gradient_field(GradientAxisSpec::Y, 2);
    validate_region_field_frame_gradient_sinks(&[&scalar, &grad_x, &grad_y])
        .expect("same-frame valid sinks");

    // Tick N+1 authored as a separate frame group may use source_col 1 without this
    // validator inspecting cross-tick wiring — only same-frame source_col is rejected.
    let tick_n_plus_1 = {
        let mut delayed = standard_suppression_field();
        delayed.name = "delayed_consumer".into();
        delayed.source_col = 1;
        delayed
    };
    validate_region_field_frame_gradient_sinks(&[&tick_n_plus_1])
        .expect("isolated next-tick frame group is not cross-checked against prior gradient sinks");
}
