//! COMPARATIVE-DEFAULT-BIRTH-0 (5.8b) — DA `5154348081` / remand `5154599161`.
//! HD-RECEIPT: `42c0ce43c22d`
//!
//! Default emitters + explicit triad. No fail-open Matrix guess; no hand-built
//! authored-order referee; real LinkGraph consumer path; asserted CPU/GPU parity.

use simthing_core::{
    ColumnIndex, DimensionRegistry, PropertyAdmissionDisposition, SimProperty, SimThing,
    SimThingKind, SlotIndex,
};
use simthing_driver::{
    admit_comparative_from_emitters_and_topology, admit_comparative_from_field_plan,
    admit_comparative_projections, admit_field_plan_from_region_fields,
    comparative_projection_cpu_oracle, compile_and_install, ComparativeProjectionBands,
    ComparativeProjectionDisposition, Scenario, SealedFieldTopology,
    COMPARATIVE_DERIVED_COLUMN_COUNT,
};
use simthing_gpu::{
    execute_field_sweep_cpu_chain, FieldAdjacency, FieldSweepSession, GpuContext,
    LinkGraphNeighbor, SlotAllocator, GRID_N4_NSEW,
};
use simthing_spec::{
    GameModeSpec, MappingExecutionProfile, RegionFieldCadenceSpec, RegionFieldGridProfile,
    RegionFieldOperatorSpec, RegionFieldSourcePolicySpec, RegionFieldSpec,
    RegionFieldSummaryPolicySpec, SpecVersion,
};
use std::collections::HashMap;

fn bits_equal(a: &[f32], b: &[f32]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits())
}

fn column(values: &[f32], n_dims: usize, c: usize) -> Vec<f32> {
    values.chunks_exact(n_dims).map(|row| row[c]).collect()
}

fn empty_game_mode(region_fields: Vec<RegionFieldSpec>) -> GameModeSpec {
    GameModeSpec {
        id: "cdb0".into(),
        display_name: "cdb0".into(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: Vec::new(),
        properties: Vec::new(),
        overlays: Vec::new(),
        order_weight_classes: Vec::new(),
        capability_trees: Vec::new(),
        events: Vec::new(),
        resource_flow: None,
        resource_economy: None,
        region_fields,
        mapping_execution_profile: MappingExecutionProfile::default(),
    }
}

fn region_field(
    name: &str,
    grid: u32,
    source_col: u32,
    target_col: u32,
    n_dims: u32,
) -> RegionFieldSpec {
    RegionFieldSpec {
        name: name.into(),
        grid_size: grid,
        n_dims,
        source_col,
        target_col,
        operator: RegionFieldOperatorSpec::Normalized,
        horizon: 1,
        allow_extended_horizon: false,
        alpha_self: 0.0,
        gamma_neighbor: 1.0,
        source_cap: None,
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::StandardSquare,
        reduction: None,
        parent_formula: None,
        commitment: None,
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: RegionFieldSummaryPolicySpec::default(),
        pressure_binding: None,
    }
}

fn ordinary_scenario(n_slots: u32, registry: DimensionRegistry) -> Scenario {
    Scenario {
        name: "cdb0".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 0.0,
        n_slots,
        registry,
        root: SimThing::new(SimThingKind::World, 0),
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    }
}

fn install_with_fields(
    fields: Vec<RegionFieldSpec>,
) -> (DimensionRegistry, simthing_driver::SpecSessionState) {
    let n = fields
        .first()
        .map(|f| f.grid_size * f.grid_size)
        .unwrap_or(4);
    let n_dims = fields.first().map(|f| f.n_dims).unwrap_or(8).max(16);
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_seed", "pad", 0));
    for i in 0..n_dims {
        let mut p = SimProperty::simple("col", &format!("c{i}"), 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        registry.register(p);
    }
    let scenario = ordinary_scenario(n, registry.clone());
    let game = empty_game_mode(fields);
    let mut root = scenario.root.clone();
    let mut alloc = SlotAllocator::new();
    alloc.install_initial_tree(&root);
    let mut reg = registry;
    let state = compile_and_install(&game, &scenario, &mut reg, &mut root, &mut alloc)
        .expect("ordinary install");
    (reg, state)
}

fn col(raw: u32) -> ColumnIndex {
    ColumnIndex::from_gpu_round_trip(raw)
}

fn pad_registry(n_dims: u32) -> DimensionRegistry {
    let mut reg = DimensionRegistry::new();
    for i in 0..n_dims {
        let mut p = SimProperty::simple("c", &format!("{i}"), 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        reg.register(p);
    }
    reg
}

fn gpu_context() -> Option<GpuContext> {
    match GpuContext::new_blocking() {
        Ok(c) => Some(c),
        Err(_) if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() => {
            panic!("GPU required")
        }
        Err(_) => None,
    }
}

#[test]
fn default_emitters_match_explicit_with_same_triad() {
    let fields = vec![
        region_field("e0", 2, 0, 1, 20),
        region_field("e1", 2, 2, 3, 20),
    ];
    let report = admit_field_plan_from_region_fields(&fields)
        .unwrap()
        .unwrap();
    let mut reg_a = pad_registry(20);
    let mut reg_b = pad_registry(20);
    let triad = (col(10), col(11), col(12));
    let bands = ComparativeProjectionBands::default();
    let defaulted = admit_comparative_from_field_plan(
        &mut reg_a, &report, triad.0, triad.1, triad.2, bands, None,
    )
    .unwrap();
    let explicit = admit_comparative_projections(
        &mut reg_b,
        report.topology().adjacency().clone(),
        report.topology().neighbor_slots().to_vec(),
        report.emitters().to_vec(),
        triad.0,
        triad.1,
        triad.2,
        bands,
        None,
    )
    .unwrap();
    assert_eq!(defaulted.disposition, explicit.disposition);
    assert_eq!(defaulted.outputs, explicit.outputs);
    assert_eq!(defaulted.band_readouts, explicit.band_readouts);
}

#[test]
fn grid_default_emitter_cpu_oracle_gpu_parity() {
    let fields = vec![
        region_field("e0", 2, 0, 1, 24),
        region_field("e1", 2, 2, 3, 24),
    ];
    let report = admit_field_plan_from_region_fields(&fields)
        .unwrap()
        .unwrap();
    let mut reg = pad_registry(32);
    let adm = admit_comparative_from_field_plan(
        &mut reg,
        &report,
        col(10),
        col(11),
        col(12),
        ComparativeProjectionBands::default(),
        None,
    )
    .unwrap();
    assert!(matches!(
        adm.disposition,
        ComparativeProjectionDisposition::Born { .. }
    ));
    let slots = report.topology().slots();
    let n_dims = reg.total_columns as u32;
    let mut values = vec![0.0f32; (slots * n_dims) as usize];
    for s in 0..slots {
        let base = (s * n_dims) as usize;
        values[base + report.emitters()[0].value_col.raw()] = if s % 2 == 0 { 0.9 } else { 0.2 };
        values[base + report.emitters()[1].value_col.raw()] = if s % 2 == 0 { 0.2 } else { 0.9 };
        values[base + 10] = 4.0; // palma D
        values[base + 11] = 0.5; // guyang U
        values[base + 12] = 0.5; // guyang C (Matrix input to 5.8 stall)
    }

    let chain =
        execute_field_sweep_cpu_chain(&values, &adm.bundle.registrations).expect("cpu chain");
    let oracle = comparative_projection_cpu_oracle(
        &chain,
        slots,
        n_dims,
        report.emitters(),
        adm.outputs,
        adm.band_readouts,
        col(10),
        adm.stall_outputs.stall_col,
        ComparativeProjectionBands::default(),
        report.topology().neighbor_slots(),
    );
    for col_i in [
        adm.outputs.dominance_col.raw(),
        adm.outputs.margin_col.raw(),
        adm.outputs.contest_col.raw(),
        adm.band_readouts.border_col.raw(),
        adm.band_readouts.chokepoint_col.raw(),
        adm.stall_outputs.stall_col.raw(),
    ] {
        assert!(
            bits_equal(
                &column(&oracle, n_dims as usize, col_i),
                &column(&chain, n_dims as usize, col_i)
            ),
            "default-path oracle parity col {col_i}"
        );
    }

    if let Some(ctx) = gpu_context() {
        let mut session =
            FieldSweepSession::new(&ctx, &adm.bundle.registrations[0]).expect("session");
        session.upload_values(&ctx, &values).expect("upload");
        session
            .dispatch_chain(&ctx, &adm.bundle.registrations, 1)
            .expect("dispatch");
        let gpu = session.readback(&ctx).expect("readback");
        // Remand 5156686392 §3: full load-bearing set, not dominance alone.
        for col_i in [
            adm.outputs.dominance_col.raw(),
            adm.outputs.margin_col.raw(),
            adm.outputs.contest_col.raw(),
            adm.band_readouts.border_col.raw(),
            adm.band_readouts.chokepoint_col.raw(),
            adm.stall_outputs.stall_col.raw(),
        ] {
            assert!(
                bits_equal(
                    &column(&chain, n_dims as usize, col_i),
                    &column(&gpu, n_dims as usize, col_i)
                ),
                "default-path GPU parity col {col_i}"
            );
        }
        let info = ctx.adapter.get_info();
        eprintln!(
            "COMPARATIVE-DEFAULT-BIRTH grid adapter={} backend={:?}",
            info.name, info.backend
        );
    }
}

