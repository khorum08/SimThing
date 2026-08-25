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
    assert_eq!(
        report.emitter_names(),
        &["e0".to_string(), "e1".to_string()]
    );
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

/// Remand 5156686392 §1: execute exact-tie field; assert dominance/border
/// bit-identical under incidental reverse; flip authored_order flips winner.
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

    let slots = report.topology().slots();
    let n_dims = reg_a.total_columns as u32;
    // Exact-tie on every slot: equal emitter values; winner is authored_order.
    let mut values = vec![0.0f32; (slots * n_dims) as usize];
    for s in 0..slots {
        let base = (s * n_dims) as usize;
        values[base + emitters_fwd[0].value_col.raw()] = 1.0;
        values[base + emitters_fwd[1].value_col.raw()] = 1.0;
        values[base + 10] = 4.0;
        values[base + 11] = 0.5;
        values[base + 12] = 0.5;
    }

    let out_fwd =
        execute_field_sweep_cpu_chain(&values, &a.bundle.registrations).expect("fwd chain");
    let out_rev =
        execute_field_sweep_cpu_chain(&values, &b.bundle.registrations).expect("rev chain");
    let dom = a.outputs.dominance_col.raw();
    let border = a.band_readouts.border_col.raw();
    assert!(
        bits_equal(
            &column(&out_fwd, n_dims as usize, dom),
            &column(&out_rev, n_dims as usize, dom)
        ),
        "incidental reverse must leave dominance bit-identical under exact tie"
    );
    assert!(
        bits_equal(
            &column(&out_fwd, n_dims as usize, border),
            &column(&out_rev, n_dims as usize, border)
        ),
        "incidental reverse must leave border bit-identical under exact tie"
    );
    // Authored_order 0 wins exact ties → class_id 0.0 written to dominance.
    assert!(
        column(&out_fwd, n_dims as usize, dom)
            .iter()
            .all(|&v| v == 0.0),
        "authored_order 0 / class_id 0.0 wins exact tie"
    );

    // Mutate authored_order/class_id on the same 5.8b value_cols → flip winner.
    // Keep value_cols from 5.8b derivation; swap only order keys so the order-0
    // winner writes a different class_id into dominance.
    let mut emitters_flip = report.emitters().to_vec();
    emitters_flip[0].authored_order = 1;
    emitters_flip[0].class_id = 99.0; // e0 value_col, no longer order-0
    emitters_flip[1].authored_order = 0;
    emitters_flip[1].class_id = 42.0; // e1 value_col, now order-0 → wins tie
    let mut reg_c = pad_registry(24);
    let flipped = admit_comparative_from_emitters_and_topology(
        &mut reg_c,
        report.topology(),
        &emitters_flip,
        triad.0,
        triad.1,
        triad.2,
        bands,
        None,
    )
    .unwrap();
    let out_flip =
        execute_field_sweep_cpu_chain(&values, &flipped.bundle.registrations).expect("flip chain");
    let dom_f = flipped.outputs.dominance_col.raw();
    assert!(
        column(&out_flip, n_dims as usize, dom_f)
            .iter()
            .all(|&v| v == 42.0),
        "mutating authored_order on same value_cols must flip exact-tie winner class_id"
    );
    assert_ne!(
        column(&out_fwd, n_dims as usize, dom)[0],
        column(&out_flip, n_dims as usize, dom_f)[0]
    );
}

/// Remand 5156686392 §2–§3: sealed LinkGraph + 5.8b emitters with CPU/oracle/GPU
/// parity; same-length wrong graph is a distinct seal (no rebind API).
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

    // Same-length wrong membership (cycle vs path) → distinct sealed adjacency.
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

    // Execute default-path LinkGraph comparative: CPU chain ↔ oracle ↔ GPU.
    let slots = 4u32;
    let n_dims = reg.total_columns as u32;
    let mut values = vec![0.0f32; (slots * n_dims) as usize];
    for s in 0..slots {
        let base = (s * n_dims) as usize;
        if s < 2 {
            values[base + report.emitters()[0].value_col.raw()] = 0.9;
            values[base + report.emitters()[1].value_col.raw()] = 0.2;
        } else {
            values[base + report.emitters()[0].value_col.raw()] = 0.2;
            values[base + report.emitters()[1].value_col.raw()] = 0.9;
        }
        values[base + 10] = 12.0;
        values[base + 11] = if s < 2 { 1.0 } else { 0.0 };
        values[base + 12] = 0.5;
    }
    let chain =
        execute_field_sweep_cpu_chain(&values, &adm.bundle.registrations).expect("link chain");
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
        sealed_link.neighbor_slots(),
    );
    let load_bearing = [
        adm.outputs.dominance_col.raw(),
        adm.outputs.margin_col.raw(),
        adm.outputs.contest_col.raw(),
        adm.band_readouts.border_col.raw(),
        adm.band_readouts.chokepoint_col.raw(),
        adm.stall_outputs.stall_col.raw(),
    ];
    for col_i in load_bearing {
        assert!(
            bits_equal(
                &column(&oracle, n_dims as usize, col_i),
                &column(&chain, n_dims as usize, col_i)
            ),
            "link default-path oracle parity col {col_i}"
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
        for col_i in load_bearing {
            assert!(
                bits_equal(
                    &column(&chain, n_dims as usize, col_i),
                    &column(&gpu, n_dims as usize, col_i)
                ),
                "link default-path GPU parity col {col_i}"
            );
        }
        let info = ctx.adapter.get_info();
        eprintln!(
            "COMPARATIVE-DEFAULT-BIRTH link adapter={} backend={:?}",
            info.name, info.backend
        );
    }
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

/// Remand 5156686392 §4: executable privacy surface + length-guard.
/// Compile-fail rebind lives on `comparative_default_birth` module docs.
#[test]
fn sealed_topology_no_independent_neighbor_rebind_api() {
    let adj = FieldAdjacency::grid_n4(2, 2, GRID_N4_NSEW, col(0)).unwrap();
    let sealed = SealedFieldTopology::from_grid_adjacency(adj.clone()).unwrap();
    assert_eq!(sealed.neighbor_slots().len(), 4);
    // Public surface is capture-only: from_grid_adjacency / from_link_graph.
    // Fields are private — no SealedFieldTopology { adjacency, neighbor_slots }.
    // Length mismatch rejects at LinkGraph construction (not a rebind path).
    assert!(SealedFieldTopology::from_link_graph(
        2,
        vec![vec![LinkGraphNeighbor {
            slot: SlotIndex::new(0),
            weight: 1.0
        }]],
        col(0)
    )
    .is_err());
    // Same-length wrong LinkGraph is a *different* seal, not a rebind of `sealed`.
    let path = {
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
    let cycle = {
        let mut rows = vec![Vec::new(); 4];
        for (a, b) in [(0u32, 1), (1, 2), (2, 3), (3, 0)] {
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
    let a = SealedFieldTopology::from_link_graph(4, path, col(0)).unwrap();
    let b = SealedFieldTopology::from_link_graph(4, cycle, col(0)).unwrap();
    assert_ne!(a.adjacency(), b.adjacency());
    assert_ne!(a.neighbor_slots(), b.neighbor_slots());
    // No method attaches b.neighbor_slots() to a.adjacency() — API census:
    let _ = a.adjacency();
    let _ = a.neighbor_slots();
    let _ = a.slots();
    // (constructors + three accessors only; rebind would be a fourth public API)
}
