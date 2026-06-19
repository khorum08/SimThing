//! DRIVER-MAPPING-PLAN-COMPILE-0 — driver mapping plan compile guards.

use simthing_driver::{
    compile_mapping_plan_from_admitted_theater, compile_structured_field_mapping_plan,
    CompiledStructuralN4Theater, MappingPlanCompileError, MappingPlanCompileSpec,
    StructuralGridCoordinate,
};
use simthing_gpu::MIN_PLUS_INF;
use simthing_sim::CompiledMappingStep;
use simthing_spec::{
    compile_region_field_preview, compile_w_impedance_compose_preview, MappingExecutionProfile,
    RegionFieldCadenceSpec, RegionFieldGridProfile, RegionFieldOperatorSpec,
    RegionFieldSourcePolicySpec, RegionFieldSpec, WImpedanceComposeProfileSpec,
    WImpedanceComposeSpec,
};

const FORBIDDEN_COMPILE_TOKENS: &[&str] = &[
    "pathfinding",
    "predecessor",
    "came_from",
    "route_object",
    "movement_order",
    "border_service",
    "frontline_service",
    "cpu_planner",
    "semantic_wgsl",
];

fn theater_8x8() -> CompiledStructuralN4Theater {
    CompiledStructuralN4Theater {
        frame_width: 8,
        frame_height: 8,
        occupied_cells: vec![StructuralGridCoordinate { col: 0, row: 0 }],
        n4_edges: Vec::new(),
        system_placements: Vec::new(),
        execution_profile: MappingExecutionProfile::SparseRegionFieldV1,
    }
}

fn saturating_flux_field_spec(grid: u32) -> RegionFieldSpec {
    RegionFieldSpec {
        name: "mapping_plan_compile_structured_field".into(),
        grid_size: grid,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        operator: RegionFieldOperatorSpec::SaturatingFlux {
            u_sat: 2.0,
            chi: 0.25,
            choke_output_col: Some(1),
        },
        horizon: 4,
        allow_extended_horizon: false,
        alpha_self: 1.0,
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

fn w_compose_spec(grid: u32) -> WImpedanceComposeSpec {
    WImpedanceComposeSpec {
        width: grid,
        height: grid,
        n_dims: 5,
        base_w_col: 0,
        choke_a_col: 1,
        choke_b_col: 2,
        profiles: vec![WImpedanceComposeProfileSpec {
            weight_a: 1.0,
            weight_b: 0.25,
            output_w_col: 3,
        }],
    }
}

fn full_compile_spec(grid: u32) -> MappingPlanCompileSpec {
    let structured_field =
        compile_region_field_preview(&saturating_flux_field_spec(grid)).expect("field admission");
    let w_compose =
        compile_w_impedance_compose_preview(&w_compose_spec(grid)).expect("w admission");
    MappingPlanCompileSpec {
        structured_field,
        structured_hops: 4,
        structured_to_interleaved_writes: vec![(1, 1)],
        w_compose,
        min_plus_profile_index: 0,
        min_plus_dest: StructuralGridCoordinate { col: 0, row: 0 },
        min_plus_d_col: 4,
        min_plus_iterations: 16,
        min_plus_inf: MIN_PLUS_INF,
    }
}

#[test]
fn mapping_plan_compile_builds_generic_structured_field_plan() {
    let theater = theater_8x8();
    let structured_field =
        compile_region_field_preview(&saturating_flux_field_spec(8)).expect("field admission");
    let plan = compile_structured_field_mapping_plan(&theater, &structured_field, 2, Vec::new(), 0)
        .expect("structured field plan");
    assert_eq!(plan.steps.len(), 1);
    let CompiledMappingStep::StructuredFieldStencil {
        config,
        hops,
        interleaved_column_writes,
    } = &plan.steps[0]
    else {
        panic!("expected structured field step");
    };
    assert_eq!(config.width, 8);
    assert_eq!(config.height, 8);
    assert_eq!(*hops, 2);
    assert!(interleaved_column_writes.is_empty());
}

#[test]
fn mapping_plan_compile_builds_w_compose_min_plus_plan() {
    let theater = theater_8x8();
    let plan =
        compile_mapping_plan_from_admitted_theater(&theater, full_compile_spec(8)).expect("plan");
    assert_eq!(plan.steps.len(), 3);
    assert_eq!(plan.interleaved_width, 8);
    assert_eq!(plan.interleaved_height, 8);
    assert_eq!(plan.interleaved_n_dims, 5);

    let CompiledMappingStep::StructuredFieldStencil {
        interleaved_column_writes,
        ..
    } = &plan.steps[0]
    else {
        panic!("expected structured field");
    };
    assert_eq!(interleaved_column_writes, &vec![(1, 1)]);

    assert!(matches!(
        plan.steps[1],
        CompiledMappingStep::WImpedanceCompose { .. }
    ));
    let CompiledMappingStep::MinPlusStencil { iterations, config } = &plan.steps[2] else {
        panic!("expected min plus");
    };
    assert_eq!(*iterations, 16);
    assert_eq!(config.dest_x, 0);
    assert_eq!(config.dest_y, 0);
    assert_eq!(config.d_col, 4);
    assert_eq!(config.w_col, 3);
}

#[test]
fn mapping_plan_compile_rejects_mismatched_theater_dimensions() {
    let theater = theater_8x8();
    let err = compile_mapping_plan_from_admitted_theater(&theater, full_compile_spec(5))
        .expect_err("dimension mismatch");
    assert!(matches!(
        err,
        MappingPlanCompileError::StructuredFieldTheaterMismatch { .. }
            | MappingPlanCompileError::WComposeTheaterMismatch { .. }
    ));
}

#[test]
fn mapping_plan_compile_preserves_generic_operator_only_boundary() {
    let source = include_str!("../src/mapping_plan_compile.rs");
    assert!(!source.contains("SimThingScenarioSpec"));
    assert!(!source.contains("deserialize_scenario_authority"));
    assert!(!source.contains("include_str!("));
    assert!(!source.contains("StructuredFieldStencilOp"));
    assert!(!source.contains("WImpedanceComposeOp"));
    assert!(!source.contains("MinPlusStencilOp"));
    assert!(!source.contains("simthing_mapeditor"));
}

#[test]
fn mapping_plan_compile_forbidden_token_guard() {
    let source = include_str!("../src/mapping_plan_compile.rs");
    for token in FORBIDDEN_COMPILE_TOKENS {
        assert!(
            !source.contains(token),
            "mapping_plan_compile must not contain `{token}`"
        );
    }
}
