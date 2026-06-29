//! TERRAN-PIRATE-MAPPING-FIRST-SLICE-0 — structural N4 Gu-Yang/PALMA first-slice GPU proof.
//!
//! Loads canonical `SimThingScenarioSpec` authority, compiles structural N4 theater via
//! `compile_structural_n4_theater`, and exercises existing SaturatingFlux + W-impedance
//! compose + min-plus operator surfaces with CPU/GPU parity.

use std::collections::BTreeSet;
use std::sync::Mutex;

use simthing_core::{CombineFn, SourceSpec, StructuralScalarChannel};
use simthing_driver::{
    compile_structural_link_neighbor_sum_plan, compile_structural_n4_theater,
    compiled_stencil_to_gpu_config, compiled_w_impedance_compose_to_gpu_config,
    composed_w_min_plus_stencil_config, StructuralCoord, StructuralTheaterAdmission,
};
use simthing_gpu::wgpu::util::DeviceExt;
use simthing_gpu::{
    cpu_min_plus_d_from_w, cpu_stencil_step, extract_d_flat, max_d_field_error,
    pack_w_and_initial_d, params_from_config, GpuContext, MinPlusStencilOp,
    StructuredFieldStencilOp, WImpedanceComposeOp, MIN_PLUS_INF,
};
use simthing_spec::{
    compile_region_field_preview, compile_w_impedance_compose_preview,
    deserialize_scenario_authority, validate_scenario_links, validate_stead_mapping_consistency,
    MappingExecutionProfile, RegionFieldCadenceSpec, RegionFieldGridProfile,
    RegionFieldOperatorSpec, RegionFieldSourcePolicySpec, RegionFieldSpec, SimThingScenarioSpec,
    WImpedanceComposeProfileSpec, WImpedanceComposeSpec,
};

const TERRAN_PIRATE_SKELETON_SCENARIO_JSON: &str =
    include_str!("../../../scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json");

const SATURATING_FLUX_HOPS: u32 = 4;
const MIN_PLUS_ITERATIONS: u32 = 16;

const FORBIDDEN_RUNTIME: &[&str] = &[
    "pathfinding",
    "predecessor",
    "came_from",
    "movement_engine",
    "border_service",
    "frontline_service",
    "cpu_planner",
];

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn canonical_skeleton_scenario() -> SimThingScenarioSpec {
    let scenario = deserialize_scenario_authority(TERRAN_PIRATE_SKELETON_SCENARIO_JSON)
        .expect("deserialize canonical skeleton");
    validate_stead_mapping_consistency(&scenario).expect("STEAD valid");
    validate_scenario_links(&scenario).expect("links valid");
    scenario
}

fn admit_structural_theater(
    spec: &SimThingScenarioSpec,
) -> simthing_driver::CompiledStructuralN4Theater {
    match compile_structural_n4_theater(spec, MappingExecutionProfile::SparseRegionFieldV1)
        .expect("compile structural theater")
    {
        StructuralTheaterAdmission::Admit(theater) => theater,
        StructuralTheaterAdmission::AtlasDeferred { reason, .. } => {
            panic!("expected admission, got atlas deferral: {reason:?}")
        }
    }
}

fn cell_slot(coord: StructuralCoord, width: u32) -> u32 {
    coord.row() * width + coord.col()
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn terran_pirate_guyang_field_spec(grid_size: u32) -> RegionFieldSpec {
    RegionFieldSpec {
        name: "terran_pirate_guyang_first_slice".into(),
        grid_size,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        operator: RegionFieldOperatorSpec::SaturatingFlux {
            u_sat: 2.0,
            chi: 0.25,
            choke_output_col: Some(1),
        },
        horizon: SATURATING_FLUX_HOPS,
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

fn terran_pirate_w_compose_spec(grid_size: u32) -> WImpedanceComposeSpec {
    WImpedanceComposeSpec {
        width: grid_size,
        height: grid_size,
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

fn seed_guyang_values(
    theater: &simthing_driver::CompiledStructuralN4Theater,
    n_dims: u32,
) -> Vec<f32> {
    let cells = theater.frame_width * theater.frame_height;
    let mut values = vec![0.0f32; (cells * n_dims) as usize];
    let hub = theater.coord_for_system(1).expect("hub placement");
    let corridor = theater.coord_for_system(2).expect("corridor placement");
    let branch = theater.coord_for_system(3).expect("branch placement");
    values[idx(theater.cell_slot(hub), 0, n_dims)] = 80.0;
    values[idx(theater.cell_slot(corridor), 0, n_dims)] = 20.0;
    values[idx(theater.cell_slot(branch), 0, n_dims)] = 10.0;
    values
}

fn cpu_saturating_flux_horizon(
    values: &[f32],
    params: &simthing_gpu::FieldStencilParamsGpu,
) -> Vec<f32> {
    let mut cur = values.to_vec();
    for _ in 0..SATURATING_FLUX_HOPS {
        cur = cpu_stencil_step(&cur, params);
    }
    cur
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for mapping first-slice proof");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn hyperlane_neighbor_system_pairs(spec: &SimThingScenarioSpec) -> BTreeSet<(u32, u32)> {
    let mut pairs = BTreeSet::new();
    for link in &spec.links {
        let from: u32 = link.from_system_id.parse().expect("from system id");
        let to: u32 = link.to_system_id.parse().expect("to system id");
        let pair = if from <= to { (from, to) } else { (to, from) };
        pairs.insert(pair);
    }
    pairs
}

#[test]
fn mapping_first_slice_derives_structural_n4_theater_from_scenario_authority() {
    let spec = canonical_skeleton_scenario();
    let theater = admit_structural_theater(&spec);

    assert_eq!(theater.frame_width, 8);
    assert_eq!(theater.frame_height, 8);
    assert_eq!(theater.occupied_cells.len(), 4);
    assert_eq!(theater.n4_edges.len(), 3);

    let hub = theater.coord_for_system(1).expect("hub");
    let corridor = theater.coord_for_system(2).expect("corridor");
    let branch = theater.coord_for_system(3).expect("branch");
    let choke = theater.coord_for_system(4).expect("choke");

    assert_eq!(hub, StructuralCoord::new(0, 0));
    assert_eq!(corridor, StructuralCoord::new(1, 0));
    assert_eq!(choke, StructuralCoord::new(2, 0));
    assert_eq!(branch, StructuralCoord::new(1, 1));

    assert!(theater.has_n4_edge(hub, corridor));
    assert!(theater.has_n4_edge(corridor, choke));
    assert!(theater.has_n4_edge(corridor, branch));
    assert!(!theater.has_n4_edge(hub, branch));
}

#[test]
fn mapping_first_slice_grid_n4_adjacency_is_separate_from_hyperlane_links() {
    let spec = canonical_skeleton_scenario();
    let theater = admit_structural_theater(&spec);
    let hyperlane_pairs = hyperlane_neighbor_system_pairs(&spec);

    let n4_system_pairs: BTreeSet<_> = theater
        .n4_edges
        .iter()
        .map(|(a, b)| {
            let system = |coord: StructuralCoord| {
                theater
                    .system_placements
                    .iter()
                    .find(|placement| placement.col == coord.col() && placement.row == coord.row())
                    .expect("system for coord")
                    .system_id
            };
            let sa = system(*a);
            let sb = system(*b);
            if sa <= sb {
                (sa, sb)
            } else {
                (sb, sa)
            }
        })
        .collect();

    assert_eq!(hyperlane_pairs.len(), 3);
    assert_eq!(n4_system_pairs.len(), 3);
    assert!(hyperlane_pairs.contains(&(1, 2)));
    assert!(hyperlane_pairs.contains(&(2, 3)));
    assert!(hyperlane_pairs.contains(&(2, 4)));
    assert!(
        !hyperlane_pairs.contains(&(1, 3)),
        "hub-branch is not a hyperlane edge"
    );

    let hub = theater.coord_for_system(1).expect("hub");
    let branch = theater.coord_for_system(3).expect("branch");
    assert!(
        !theater.has_n4_edge(hub, branch),
        "grid N4 must not connect hub and branch directly"
    );

    let mut link_only = spec.clone();
    link_only.links.clear();
    let theater_without_links = admit_structural_theater(&link_only);
    assert_eq!(
        theater.n4_edges, theater_without_links.n4_edges,
        "N4 adjacency must not depend on scenario.links"
    );

    let plan = compile_structural_link_neighbor_sum_plan(
        &spec,
        StructuralScalarChannel(0),
        StructuralScalarChannel(1),
    )
    .expect("link gather compile");
    let corridor = plan
        .ops
        .iter()
        .find(|op| op.targets[0].0 == 1)
        .expect("corridor op");
    let SourceSpec::ConjunctiveCrossing { inputs } = &corridor.source else {
        panic!("expected input list gather");
    };
    let mut link_neighbor_slots: Vec<_> = inputs.iter().map(|input| input.slot).collect();
    link_neighbor_slots.sort_unstable();
    assert_eq!(link_neighbor_slots, vec![0, 2, 3]);
    assert_eq!(plan.ops.len(), 4);
    for op in &plan.ops {
        assert_eq!(op.combine, CombineFn::Sum);
    }
}

#[test]
fn mapping_first_slice_guyang_saturating_flux_gpu_matches_cpu_oracle() {
    let spec = canonical_skeleton_scenario();
    let theater = admit_structural_theater(&spec);
    let field_spec = terran_pirate_guyang_field_spec(theater.frame_width);
    let preview = compile_region_field_preview(&field_spec).expect("region field admission");
    let gpu_config = compiled_stencil_to_gpu_config(&preview.stencil);
    let params = params_from_config(&gpu_config);
    let values = seed_guyang_values(&theater, gpu_config.n_dims);
    let cpu = cpu_saturating_flux_horizon(&values, &params);

    for cell in &theater.occupied_cells {
        let slot = theater.cell_slot(*cell);
        let u = cpu[idx(slot, 0, gpu_config.n_dims)];
        let choke = cpu[idx(slot, 1, gpu_config.n_dims)];
        assert!(u.is_finite(), "target field must be finite at {cell:?}");
        assert!(
            choke.is_finite(),
            "choke readout must be finite at {cell:?}"
        );
        assert!(
            choke >= 0.0 && choke <= 1.0,
            "choke must stay bounded at {cell:?}"
        );
    }

    with_gpu(|ctx| {
        let op = StructuredFieldStencilOp::new(ctx, gpu_config.clone()).expect("stencil op");
        op.upload_values(ctx, &values).expect("upload");
        let gpu = op
            .run_ping_pong(ctx, SATURATING_FLUX_HOPS)
            .expect("gpu saturating flux")
            .0;
        assert_eq!(gpu.len(), cpu.len());
        for (i, (g, c)) in gpu.iter().zip(cpu.iter()).enumerate() {
            assert!(
                (g - c).abs() < 1e-4,
                "gpu/cpu mismatch at index {i}: gpu={g} cpu={c}"
            );
        }
        eprintln!("TERRAN-PIRATE-MAPPING-FIRST-SLICE-0: REAL_ADAPTER_OBSERVED (SaturatingFlux)");
    });

    let compile_src = include_str!("../src/structural_n4_theater_compile.rs");
    let bridge = include_str!("../src/w_impedance_compose_bridge.rs");
    for token in FORBIDDEN_RUNTIME {
        assert!(
            !compile_src.contains(token),
            "compile surface must not contain `{token}`"
        );
        assert!(!bridge.contains(token), "bridge must not contain `{token}`");
    }
    assert_eq!(
        MappingExecutionProfile::Disabled.enables_execution(),
        false,
        "first-slice proof does not enable Studio/runtime mapping dispatch"
    );
}

#[test]
fn mapping_first_slice_palma_w_compose_and_min_plus_gpu_matches_cpu_oracle() {
    let spec = canonical_skeleton_scenario();
    let theater = admit_structural_theater(&spec);
    let grid = theater.frame_width;

    let guyang_spec = terran_pirate_guyang_field_spec(grid);
    let guyang_preview = compile_region_field_preview(&guyang_spec).expect("guyang admission");
    let guyang_config = compiled_stencil_to_gpu_config(&guyang_preview.stencil);
    let guyang_params = params_from_config(&guyang_config);
    let guyang_values = seed_guyang_values(&theater, guyang_config.n_dims);
    let guyang_cpu = cpu_saturating_flux_horizon(&guyang_values, &guyang_params);

    let w_spec = terran_pirate_w_compose_spec(grid);
    let w_compiled = compile_w_impedance_compose_preview(&w_spec).expect("w compose admission");
    let w_gpu = compiled_w_impedance_compose_to_gpu_config(&w_compiled);
    let hub = theater.coord_for_system(1).expect("hub");
    let stencil =
        composed_w_min_plus_stencil_config(&w_gpu, 0, 4, (hub.col(), hub.row()), MIN_PLUS_INF);

    let cells = (grid * grid) as usize;
    let n_dims = w_spec.n_dims as usize;
    let mut interleaved = vec![0.0f32; cells * n_dims];
    for slot in 0..cells as u32 {
        interleaved[idx(slot, 0, w_spec.n_dims)] = 1.0;
        interleaved[idx(slot, 1, w_spec.n_dims)] = guyang_cpu[idx(slot, 1, guyang_config.n_dims)];
        interleaved[idx(slot, 2, w_spec.n_dims)] = 0.0;
    }

    let mut w_flat = vec![0.0f32; cells];
    for slot in 0..cells as u32 {
        let base = interleaved[idx(slot, 0, w_spec.n_dims)];
        let choke_a = interleaved[idx(slot, 1, w_spec.n_dims)];
        let choke_b = interleaved[idx(slot, 2, w_spec.n_dims)];
        w_flat[slot as usize] = base + choke_a + 0.25 * choke_b;
    }

    let cpu_d =
        cpu_min_plus_d_from_w(&w_flat, &stencil, MIN_PLUS_ITERATIONS).expect("cpu min-plus oracle");
    for cell in &theater.occupied_cells {
        let d = cpu_d[theater.cell_slot(*cell) as usize];
        assert!(d.is_finite(), "D field must be finite at occupied {cell:?}");
    }
    assert!(
        cpu_d[stencil.dest_idx()] <= 1e-6,
        "destination D must be pinned to zero"
    );

    with_gpu(|ctx| {
        let values_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("terran_pirate_mapping_first_slice_interleaved"),
                contents: bytemuck::cast_slice(&interleaved),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });
        WImpedanceComposeOp::new(ctx)
            .compose_resident_field(ctx, &values_buffer, &w_gpu)
            .expect("w compose dispatch");

        let packed = pack_w_and_initial_d(&w_flat, &stencil).expect("pack w/d");
        let op = MinPlusStencilOp::new(ctx, stencil.clone()).expect("min-plus op");
        op.upload_values(ctx, &packed).expect("upload");
        let gpu_values = op
            .run_ping_pong(ctx, MIN_PLUS_ITERATIONS)
            .expect("gpu min-plus");
        let gpu_d = extract_d_flat(&gpu_values, &stencil).expect("extract d");
        assert!(
            max_d_field_error(&cpu_d, &gpu_d) < 1e-4,
            "PALMA D field GPU/CPU parity over structural theater"
        );
        eprintln!("TERRAN-PIRATE-MAPPING-FIRST-SLICE-0: REAL_ADAPTER_OBSERVED (MinPlus)");
    });

    let bridge = include_str!("../src/w_impedance_compose_bridge.rs");
    for token in FORBIDDEN_RUNTIME {
        assert!(
            !bridge.contains(token),
            "PALMA bridge must not contain forbidden token `{token}`"
        );
    }
}
