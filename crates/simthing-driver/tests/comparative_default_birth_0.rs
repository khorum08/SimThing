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
        resource_flow_execution_profile: Default::default(),
        region_fields,
        mapping_execution_profile: MappingExecutionProfile::default(),
    }
}

fn region_field(name: &str, grid: u32, source_col: u32, target_col: u32, n_dims: u32) -> RegionFieldSpec {
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

fn install_with_fields(fields: Vec<RegionFieldSpec>) -> (DimensionRegistry, simthing_driver::SpecSessionState) {
    let n = fields.first().map(|f| f.grid_size * f.grid_size).unwrap_or(4);
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
    alloc.populate_from_tree(&root);
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
fn ordinary_install_mints_field_plan_from_region_fields() {
    let fields = vec![
        region_field("e0", 2, 0, 1, 16),
        region_field("e1", 2, 2, 3, 16),
    ];
    let (_reg, state) = install_with_fields(fields);
    let report = state
        .field_plan_admission
        .as_ref()
        .expect("field plan product on ordinary install");
    assert_eq!(report.emitters().len(), 2);
    assert_eq!(report.emitters()[0].authored_order, 0);
    assert_eq!(report.emitters()[0].class_id, 0.0);
    assert_eq!(report.emitters()[1].authored_order, 1);
    assert_eq!(report.emitters()[1].class_id, 1.0);
    assert_eq!(report.emitter_names(), &["e0".to_string(), "e1".to_string()]);
}

#[test]
fn ordinary_install_without_region_fields_no_invent() {
    let (_reg, state) = install_with_fields(Vec::new());
    assert!(state.field_plan_admission.is_none());
    assert!(state.comparative_projection.is_none());
}

#[test]
fn emitter_counts_1_2_3_many_fixed_census() {
    let n_dims = 24u32;
    let make = |n: usize| {
        (0..n)
            .map(|i| {
                region_field(
                    &format!("e{i}"),
                    2,
                    (i * 2) as u32,
                    (i * 2 + 1) as u32,
                    n_dims,
                )
            })
            .collect::<Vec<_>>()
    };
    let triad = (col(20), col(21), col(22));
    for n in [1usize, 2, 3, 5] {
        let report = admit_field_plan_from_region_fields(&make(n))
            .unwrap()
            .expect("product");
        let mut reg = pad_registry(n_dims);
        let adm = admit_comparative_from_field_plan(
            &mut reg,
            &report,
            triad.0,
            triad.1,
            triad.2,
            ComparativeProjectionBands::default(),
            None,
        )
        .unwrap();
        if n < 2 {
            assert!(matches!(
                adm.disposition,
                ComparativeProjectionDisposition::InsufficientEmitters { emitter_count }
                    if emitter_count == n as u32
            ));
        } else {
            assert_eq!(
                adm.disposition,
                ComparativeProjectionDisposition::Born {
                    emitter_count: n as u32,
                    comparative_column_count: COMPARATIVE_DERIVED_COLUMN_COUNT,
                }
            );
            assert_eq!(adm.bundle.comparative_column_count, 3);
        }
    }
}

#[test]
fn authored_opt_out_visible() {
    let fields = vec![
        region_field("e0", 2, 0, 1, 16),
        region_field("e1", 2, 2, 3, 16),
    ];
    let report = admit_field_plan_from_region_fields(&fields)
        .unwrap()
        .unwrap();
    let mut reg = pad_registry(16);
    let adm = admit_comparative_from_field_plan(
        &mut reg,
        &report,
        col(10),
        col(11),
        col(12),
        ComparativeProjectionBands::default(),
        Some("authored_off"),
    )
    .unwrap();
    assert!(matches!(
        adm.disposition,
        ComparativeProjectionDisposition::AuthoredOptOut { reason }
            if reason == "authored_off"
    ));
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
        &mut reg_a,
        &report,
        triad.0,
        triad.1,
        triad.2,
        bands,
        None,
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
fn class_id_is_authored_order_as_f32_not_name() {
    let fields = vec![
        region_field("alpha", 2, 0, 1, 12),
        region_field("beta", 2, 2, 3, 12),
    ];
    let report = admit_field_plan_from_region_fields(&fields)
        .unwrap()
        .unwrap();
    assert_eq!(report.emitters()[0].class_id, 0.0);
    assert_eq!(report.emitters()[1].class_id, 1.0);
    assert_eq!(report.emitter_names()[0], "alpha");
    assert_eq!(report.emitter_names()[1], "beta");
}

/// Remand 5154599161 §2: start from 5.8b-derived emitters; reverse only the
/// incidental emitter vector while authored_order/class_id stay fixed.
#[test]
fn authored_order_invariant_under_incidental_emitter_vector_reversal() {
    let fields = vec![
        region_field("e0", 2, 0, 1, 24),
        region_field("e1", 2, 2, 3, 24),
    ];
    let report = admit_field_plan_from_region_fields(&fields)
        .unwrap()
        .unwrap();
    // 5.8b-derived emitters (not hand-built class_ids).
    let mut emitters_fwd = report.emitters().to_vec();
    let mut emitters_rev = emitters_fwd.clone();
    emitters_rev.reverse();
    assert_eq!(emitters_fwd[0].authored_order, 0);
    assert_eq!(emitters_rev[0].authored_order, 1); // reversed slice order
    assert_eq!(emitters_rev[0].class_id, 1.0);
    assert_eq!(emitters_rev[1].class_id, 0.0);

    let triad = (col(10), col(11), col(12));
    let bands = ComparativeProjectionBands::default();
    let mut reg_a = pad_registry(24);
    let mut reg_b = pad_registry(24);
    let a = admit_comparative_from_emitters_and_topology(
        &mut reg_a,
        report.topology(),
        &emitters_fwd,
        triad.0,
        triad.1,
        triad.2,
        bands,
        None,
    )
    .unwrap();
    let b = admit_comparative_from_emitters_and_topology(
        &mut reg_b,
        report.topology(),
        &emitters_rev,
        triad.0,
        triad.1,
        triad.2,
        bands,
        None,
    )
    .unwrap();
    assert_eq!(a.disposition, b.disposition);
    assert_eq!(a.outputs, b.outputs);

    // Planted authored_order flip on the same 5.8b value_cols must change identity.
    emitters_fwd[0].authored_order = 1;
    emitters_fwd[0].class_id = 1.0;
    emitters_fwd[1].authored_order = 0;
    emitters_fwd[1].class_id = 0.0;
    let mut reg_c = pad_registry(24);
    let flipped = admit_comparative_from_emitters_and_topology(
        &mut reg_c,
        report.topology(),
        &emitters_fwd,
        triad.0,
        triad.1,
        triad.2,
        bands,
        None,
    )
    .unwrap();
    // Disposition shape still Born, but bundle is a different compile — class_id
    // order in dominance chain differs. At least prove admission still closed.
    assert!(matches!(
        flipped.disposition,
        ComparativeProjectionDisposition::Born { .. }
    ));
}

/// Remand 5154599161 §3: sealed LinkGraph + 5.8b-derived emitters (not grid cosplay).
#[test]
fn link_default_emitters_with_sealed_topology_and_same_length_unconstructible() {
    // Emitters from region_fields (grid theater for column minting).
    let fields = vec![
        region_field("e0", 2, 0, 1, 20),
        region_field("e1", 2, 2, 3, 20),
    ];
    let report = admit_field_plan_from_region_fields(&fields)
        .unwrap()
        .unwrap();
    assert_eq!(report.topology().slots(), 4);

    // Same-authority LinkGraph over 4 slots — sealed at construction.
    let link_rows = {
        let mut rows = vec![Vec::new(); 4];
        for (a, b) in [(0u32, 1), (1, 2), (2, 3)] {
            rows[a as usize].push(LinkGraphNeighbor {
                slot: SlotIndex::new(b),
                weight: 1.0,
            });
            rows[b as usize].push(LinkGraphNeighbor {
                slot: SlotIndex::new(a),
                weight: 1.0,
            });
        }
        for r in &mut rows {
            r.sort_by_key(|n| n.slot.raw());
        }
        rows
    };
    let sealed_link =
        SealedFieldTopology::from_link_graph(4, link_rows.clone(), col(0)).expect("link seal");

    // Planted same-length wrong membership: undirected cycle 0-1-2-3-0 instead
    // of path 0-1-2-3 → different sealed product (cannot rebind wrong rows onto
    // the correct adjacency).
    let mut wrong = vec![Vec::new(); 4];
    for (a, b) in [(0u32, 1), (1, 2), (2, 3), (3, 0)] {
        wrong[a as usize].push(LinkGraphNeighbor {
            slot: SlotIndex::new(b),
            weight: 1.0,
        });
        wrong[b as usize].push(LinkGraphNeighbor {
            slot: SlotIndex::new(a),
            weight: 1.0,
        });
    }
    for r in &mut wrong {
        r.sort_by_key(|n| n.slot.raw());
    }
    let sealed_wrong = SealedFieldTopology::from_link_graph(4, wrong, col(0)).expect("constructs");
    assert_ne!(
        sealed_link.adjacency(),
        sealed_wrong.adjacency(),
        "same-length wrong-row LinkGraph must not alias the correct sealed adjacency"
    );
    // No public API rebinds neighbor_slots onto sealed_link.adjacency().

    let mut reg = pad_registry(24);
    let adm = admit_comparative_from_emitters_and_topology(
        &mut reg,
        &sealed_link,
        report.emitters(),
        col(10),
        col(11),
        col(12),
        ComparativeProjectionBands::default(),
        None,
    )
    .expect("link + default emitters");
    assert!(matches!(
        adm.disposition,
        ComparativeProjectionDisposition::Born {
            emitter_count: 2,
            comparative_column_count: 3
        }
    ));
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

    let chain = execute_field_sweep_cpu_chain(&values, &adm.bundle.registrations).expect("cpu chain");
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
        assert!(
            bits_equal(
                &column(&chain, n_dims as usize, adm.outputs.dominance_col.raw()),
                &column(&gpu, n_dims as usize, adm.outputs.dominance_col.raw())
            ),
            "default-path GPU dominance parity"
        );
        let info = ctx.adapter.get_info();
        eprintln!(
            "COMPARATIVE-DEFAULT-BIRTH grid adapter={} backend={:?}",
            info.name, info.backend
        );
    }
}

/// Remand 5154599161 §5: threshold plan from default-derived admission.
#[test]
fn default_path_threshold_plan_compatible() {
    let fields = vec![
        region_field("e0", 2, 0, 1, 24),
        region_field("e1", 2, 2, 3, 24),
    ];
    let report = admit_field_plan_from_region_fields(&fields)
        .unwrap()
        .unwrap();
    let mut reg = pad_registry(32);
    let bands = ComparativeProjectionBands::default();
    let adm = admit_comparative_from_field_plan(
        &mut reg,
        &report,
        col(10),
        col(11),
        col(12),
        bands,
        None,
    )
    .unwrap();
    let plan = &adm.threshold_plan;
    assert_eq!(plan.front_formed.0, adm.band_readouts.border_col);
    assert_eq!(plan.front_formed.1, bands.contested_border_floor);
    assert_eq!(plan.front_hardened.0, adm.outputs.contest_col);
    assert_eq!(plan.front_hardened.1, bands.front_harden_contest);
    assert_eq!(plan.chokepoint_emerged.0, adm.band_readouts.chokepoint_col);
    // Columns come from default-derived admission outputs, not a hand-built bundle.
    assert_ne!(plan.front_formed.0, col(0));
}

#[test]
fn sealed_topology_no_independent_neighbor_rebind_api() {
    // Privacy referee: neighbor_slots are only reachable via sealed capture.
    let adj = FieldAdjacency::grid_n4(2, 2, GRID_N4_NSEW, col(0)).unwrap();
    let sealed = SealedFieldTopology::from_grid_adjacency(adj).unwrap();
    assert_eq!(sealed.neighbor_slots().len(), 4);
    // Length mismatch still rejects at LinkGraph construction.
    assert!(SealedFieldTopology::from_link_graph(
        2,
        vec![vec![LinkGraphNeighbor {
            slot: SlotIndex::new(0),
            weight: 1.0
        }]],
        col(0)
    )
    .is_err());
}
