//! FIELD-ADJACENCY-GENERATORS-0 — weighted adjacency, LinkGraph, conductance,
//! scheduling, and the emergence falsifier for the generic field executor.

use std::collections::BTreeSet;

use simthing_core::{eml_opcode, ColumnIndex, EmlNodeGpu, SlotIndex};
use simthing_driver::compile_structural_link_field_adjacency;
use simthing_gpu::{
    apply_field_sweep_registration, compile_w_impedance_field_sweeps,
    cpu_w_impedance_compose_oracle, execute_field_sweep_cpu, execute_field_sweep_cpu_iterations,
    field_param, FieldAdjacency, FieldLawProof, FieldSweepAdmissionError, FieldSweepRegistration,
    FieldSweepRegistrationRequest, FieldSweepResourceClassRequest, GridOffset, LinkGraphNeighbor,
    WImpedanceComposeConfig, WImpedanceComposeProfile, GRID_N4_NSEW,
};
use simthing_spec::deserialize_scenario_authority;

const TERRAN_PIRATE_SKELETON_SCENARIO_JSON: &str =
    include_str!("../../../scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json");

fn col() -> ColumnIndex {
    ColumnIndex::try_from_admitted_authored(0, 1).expect("test column")
}

fn node(opcode: u32, a: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode,
        flags: 0,
        a,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn front_registration(adjacency: FieldAdjacency) -> FieldSweepRegistration {
    let order = adjacency.apply_canonical_order_proof();
    apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency,
        n_dims: 1,
        output_col: col(),
        map_program: vec![
            node(eml_opcode::NEIGHBOR_VALUE, 0),
            node(eml_opcode::PARAM, field_param::EDGE_SCALAR),
            node(eml_opcode::MUL, 0),
            node(eml_opcode::RETURN_TOP, 0),
        ],
        fold_program: vec![
            node(eml_opcode::PARAM, field_param::ACCUMULATOR),
            node(eml_opcode::PARAM, field_param::MAPPED),
            node(eml_opcode::MAX, 0),
            node(eml_opcode::RETURN_TOP, 0),
        ],
        identity_bits: 0.0f32.to_bits(),
        post_program: vec![
            node(eml_opcode::TARGET_VALUE, 0),
            node(eml_opcode::PARAM, field_param::FOLDED),
            node(eml_opcode::MAX, 0),
            node(eml_opcode::RETURN_TOP, 0),
        ],
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        canonical_order_proof: Some(order),
        resource_class: FieldSweepResourceClassRequest::default(),
        dt: 1.0,
    })
    .expect("front registration")
}

fn undirected_rows(slot_count: usize, edges: &[(u32, u32, f32)]) -> Vec<Vec<LinkGraphNeighbor>> {
    let mut rows = vec![Vec::new(); slot_count];
    for &(from, to, weight) in edges {
        rows[from as usize].push(LinkGraphNeighbor {
            slot: SlotIndex::new(to),
            weight,
        });
        rows[to as usize].push(LinkGraphNeighbor {
            slot: SlotIndex::new(from),
            weight,
        });
    }
    for row in &mut rows {
        row.sort_by_key(|neighbor| neighbor.slot.raw());
    }
    rows
}

#[test]
fn weighted_grid_presets_keep_all_weights_authored() {
    let n8 = FieldAdjacency::grid_n8(5, 5, 1.0, 0.625, col()).expect("N8");
    let offsets = n8.grid_offsets_data().expect("grid metadata");
    assert_eq!(offsets.len(), 8);
    assert_eq!(
        offsets
            .iter()
            .filter(|offset| offset.dx() != 0 && offset.dy() != 0)
            .map(|offset| offset.weight().to_bits())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([0.625f32.to_bits()])
    );

    let radius = FieldAdjacency::grid_radius(7, 7, 2, &[1.0, 0.25], col()).expect("radius 2");
    let offsets = radius.grid_offsets_data().expect("grid metadata");
    assert_eq!(offsets.len(), 24);
    assert_eq!(
        offsets
            .iter()
            .filter(|offset| offset.dx().abs().max(offset.dy().abs()) == 1)
            .count(),
        8
    );
    assert!(offsets
        .iter()
        .filter(|offset| offset.dx().abs().max(offset.dy().abs()) == 2)
        .all(|offset| offset.weight() == 0.25));

    assert!(matches!(
        FieldAdjacency::grid_offsets(3, 3, vec![GridOffset::new(1, 0, 1.0)], col(),)
            .expect("directed grid adjacency is valid")
            .apply_undirected_symmetry_certificate(),
        Err(FieldSweepAdmissionError::AdjacencyNotUndirected)
    ));
    assert!(matches!(
        FieldAdjacency::grid_n8(3, 3, 1.0, 0.0, col()),
        Err(FieldSweepAdmissionError::InvalidEdgeWeight(_))
    ));
}

#[test]
fn link_graph_admission_and_conductance_ignore_degree_schedule_as_physics() {
    let rows = undirected_rows(3, &[(0, 1, 2.0), (1, 2, 2.0)]);
    let adjacency = FieldAdjacency::link_graph(3, rows.clone(), col()).expect("canonical chain");
    assert_eq!(adjacency.grid_shape(), None);
    assert_eq!(adjacency.degree_buckets().len(), 2);
    assert_eq!(adjacency.degree_buckets()[0].degree(), 1);
    assert_eq!(
        adjacency.degree_buckets()[0].slots(),
        &[SlotIndex::new(0), SlotIndex::new(2)]
    );
    assert_eq!(adjacency.degree_buckets()[1].degree(), 2);
    assert_eq!(adjacency.degree_buckets()[1].slots(), &[SlotIndex::new(1)]);

    let certificate = adjacency
        .apply_conductance_certificate(vec![0.25, 0.20, 0.25], 1.0)
        .expect("per-row weighted-degree certificate");
    assert_eq!(certificate.admitted_bound(), 1.0);
    assert!(matches!(
        adjacency.apply_conductance_certificate(vec![0.25, 0.26, 0.25], 1.0),
        Err(FieldSweepAdmissionError::ConductanceBoundExceeded {
            slot,
            weighted_degree,
            ..
        }) if slot == SlotIndex::new(1) && weighted_degree == 4.0
    ));

    let mut reversed = rows;
    reversed[1].reverse();
    assert!(matches!(
        FieldAdjacency::link_graph(3, reversed, col()),
        Err(FieldSweepAdmissionError::LinkGraphNonCanonicalOrder { .. })
    ));
    assert!(matches!(
        FieldAdjacency::link_graph(
            2,
            vec![
                vec![LinkGraphNeighbor {
                    slot: SlotIndex::new(1),
                    weight: 1.0
                }],
                vec![],
            ],
            col(),
        ),
        Err(FieldSweepAdmissionError::LinkGraphMissingReverse { .. })
    ));
}

#[test]
fn existing_link_compiler_is_the_link_graph_canonical_order_basis() {
    let scenario = deserialize_scenario_authority(TERRAN_PIRATE_SKELETON_SCENARIO_JSON)
        .expect("canonical skeleton");
    let adjacency = compile_structural_link_field_adjacency(&scenario, col(), 0.75)
        .expect("canonical link projection lowers to field adjacency");
    assert_eq!(adjacency.slots(), 4);
    assert_eq!(adjacency.grid_shape(), None);
    assert_eq!(
        adjacency
            .degree_buckets()
            .iter()
            .map(|bucket| (bucket.degree(), bucket.slots().len()))
            .collect::<Vec<_>>(),
        vec![(1, 3), (3, 1)]
    );
    adjacency
        .apply_conductance_certificate(vec![0.5; 4], 1.0)
        .expect_err("hub weighted degree 2.25 must reject chi 0.5 at bound 1");
}

#[test]
fn conservative_certificate_is_bound_to_the_exact_authored_adjacency() {
    let n4 = FieldAdjacency::grid_n4(5, 5, GRID_N4_NSEW, col()).expect("N4");
    let n8 = FieldAdjacency::grid_n8(5, 5, 1.0, 0.5, col()).expect("N8");
    let order = n4.apply_canonical_order_proof();
    let law = FieldLawProof::apply_conservative(
        n4.apply_undirected_symmetry_certificate()
            .expect("N4 symmetry"),
        n8.apply_conductance_certificate(vec![0.1; 25], 1.0)
            .expect("N8 certificate"),
    );
    let mut request = valid_minimal_request(n4);
    request.canonical_order_proof = Some(order);
    request.field_law_proof = Some(law);
    assert!(matches!(
        apply_field_sweep_registration(request),
        Err(FieldSweepAdmissionError::ConductanceCertificateMismatch)
    ));
}

fn valid_minimal_request(adjacency: FieldAdjacency) -> FieldSweepRegistrationRequest {
    let order = adjacency.apply_canonical_order_proof();
    FieldSweepRegistrationRequest {
        adjacency,
        n_dims: 1,
        output_col: col(),
        map_program: vec![
            node(eml_opcode::NEIGHBOR_VALUE, 0),
            node(eml_opcode::RETURN_TOP, 0),
        ],
        fold_program: vec![
            node(eml_opcode::PARAM, field_param::MAPPED),
            node(eml_opcode::RETURN_TOP, 0),
        ],
        identity_bits: 0,
        post_program: vec![
            node(eml_opcode::PARAM, field_param::FOLDED),
            node(eml_opcode::RETURN_TOP, 0),
        ],
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        canonical_order_proof: Some(order),
        resource_class: FieldSweepResourceClassRequest::default(),
        dt: 1.0,
    }
}

#[test]
fn same_authored_field_law_emerges_as_diamond_octagon_and_link_topology() {
    let n4 = front_registration(
        FieldAdjacency::grid_n4(9, 9, GRID_N4_NSEW, col()).expect("N4 adjacency"),
    );
    let n8 =
        front_registration(FieldAdjacency::grid_n8(9, 9, 1.0, 0.5, col()).expect("N8 adjacency"));
    let link = front_registration(
        FieldAdjacency::link_graph(
            81,
            undirected_rows(81, &[(40, 0, 1.0), (0, 80, 1.0), (80, 8, 1.0)]),
            col(),
        )
        .expect("LinkGraph adjacency"),
    );
    assert_eq!(n4.map_program(), n8.map_program());
    assert_eq!(n4.map_program(), link.map_program());
    assert_eq!(n4.fold_program(), n8.fold_program());
    assert_eq!(n4.fold_program(), link.fold_program());

    let mut seed = vec![0.0; 81];
    seed[40] = 1.0;
    let n4_values = execute_field_sweep_cpu_iterations(&seed, &n4, 3).expect("N4 front");
    let n8_values = execute_field_sweep_cpu_iterations(&seed, &n8, 3).expect("N8 front");
    let link_values = execute_field_sweep_cpu_iterations(&seed, &link, 3).expect("link front");

    let active = |values: &[f32]| -> BTreeSet<usize> {
        values
            .iter()
            .enumerate()
            .filter_map(|(slot, &value)| (value >= 0.24).then_some(slot))
            .collect()
    };
    let n4_active = active(&n4_values);
    let n8_active = active(&n8_values);
    let link_active = active(&link_values);

    // N4 is the Manhattan diamond: (dx=2,dy=2) lies outside three steps.
    assert!(!n4_active.contains(&(6 + 6 * 9)));
    assert!(n4_active.contains(&(7 + 4 * 9)));
    // Authored half-weight diagonals cut the N8 square's corners into an octagon.
    assert!(n8_active.contains(&(6 + 6 * 9)));
    assert!(!n8_active.contains(&(7 + 7 * 9)));
    // LinkGraph follows authored remote topology, not embedding distance.
    assert_eq!(link_active, BTreeSet::from([0, 8, 40, 80]));
    assert_ne!(n4_active, n8_active);
    assert_ne!(n8_active, link_active);
}

#[test]
fn production_w_compose_lowering_matches_the_unedited_cpu_oracle_bit_exactly() {
    let config = WImpedanceComposeConfig {
        width: 3,
        height: 2,
        n_dims: 6,
        base_w_col: 0,
        choke_a_col: 1,
        choke_b_col: 2,
        profiles: vec![
            WImpedanceComposeProfile {
                weight_a: 0.25,
                weight_b: -0.5,
                output_w_col: 3,
            },
            WImpedanceComposeProfile {
                weight_a: -0.75,
                weight_b: 1.25,
                output_w_col: 5,
            },
        ],
    };
    let values = (0..config.values_len())
        .map(|index| index as f32 * 0.125 - 1.0)
        .collect::<Vec<_>>();
    let expected = cpu_w_impedance_compose_oracle(&values, &config);
    let mut actual = values;
    for registration in
        compile_w_impedance_field_sweeps(&config).expect("admit production W lowering")
    {
        actual = execute_field_sweep_cpu(&actual, &registration).expect("generic W pass");
    }
    assert_eq!(
        actual
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        expected
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>()
    );
}
