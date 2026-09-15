//! FIELD-ADJACENCY-GENERATORS-0 — weighted adjacency, LinkGraph, conductance,
//! scheduling, and the emergence falsifier for the generic field executor.

use std::collections::BTreeSet;

use simthing_core::{eml_opcode, ColumnIndex, EmlNodeGpu, SlotIndex};
use simthing_driver::compile_structural_link_field_adjacency;
use simthing_gpu::{
    apply_field_sweep_registration, compile_w_impedance_field_sweeps,
    cpu_w_impedance_compose_oracle, execute_field_sweep_cpu, execute_field_sweep_cpu_iterations,
    execute_field_sweep_cpu_natural_order, field_param, FieldAdjacency, FieldLawProof,
    FieldSweepAdmissionError, FieldSweepOutput, FieldSweepRegistration,
    FieldSweepRegistrationRequest, FieldSweepSession, GpuContext, GridOffset, LinkGraphNeighbor,
    WImpedanceComposeConfig, WImpedanceComposeProfile, GRID_N4_NSEW,
};
use simthing_spec::deserialize_scenario_authority;

const INLINE_LINK_COMPILER_SCENARIO_JSON: &str = r#"{
  "scenario_id":"field_adjacency_inline_link_basis",
  "root":{"id":1,"kind":"World","properties":[],"overlays":[],"children":[
    {"id":2,"kind":"Location","properties":[],"overlays":[],"children":[
      {"id":3,"kind":"Location","properties":[[8300000,{"data":[1.0]}],[8300001,{"data":[0.0]}],[8300002,{"data":[0.0]}]],"overlays":[],"children":[{"id":4,"kind":"Cohort","properties":[[8300000,{"data":[1.0]}]],"overlays":[],"children":[],"spawned_day":0}],"spawned_day":0},
      {"id":5,"kind":"Location","properties":[[8300000,{"data":[2.0]}],[8300001,{"data":[1.0]}],[8300002,{"data":[0.0]}]],"overlays":[],"children":[{"id":6,"kind":"Cohort","properties":[[8300000,{"data":[2.0]}]],"overlays":[],"children":[],"spawned_day":0}],"spawned_day":0},
      {"id":7,"kind":"Location","properties":[[8300000,{"data":[4.0]}],[8300001,{"data":[2.0]}],[8300002,{"data":[0.0]}]],"overlays":[],"children":[{"id":8,"kind":"Cohort","properties":[[8300000,{"data":[4.0]}]],"overlays":[],"children":[],"spawned_day":0}],"spawned_day":0},
      {"id":9,"kind":"Location","properties":[[8300000,{"data":[3.0]}],[8300001,{"data":[1.0]}],[8300002,{"data":[1.0]}]],"overlays":[],"children":[{"id":10,"kind":"Cohort","properties":[[8300000,{"data":[3.0]}]],"overlays":[],"children":[],"spawned_day":0}],"spawned_day":0}
    ],"spawned_day":0}
  ],"spawned_day":0},
  "structural_grid":{"frame":{"width":3,"height":2,"occupied_cells":4},"map_container_id":"2","placements":[
    {"location_id":"a","target_id":"a","system_id":1,"row":0,"col":0,"simthing_id_raw":3},
    {"location_id":"b","target_id":"b","system_id":2,"row":0,"col":1,"simthing_id_raw":5},
    {"location_id":"d","target_id":"d","system_id":4,"row":0,"col":2,"simthing_id_raw":7},
    {"location_id":"c","target_id":"c","system_id":3,"row":1,"col":1,"simthing_id_raw":9}
  ]},
  "links":[
    {"from_system_id":"1","to_system_id":"2"},
    {"from_system_id":"2","to_system_id":"3"},
    {"from_system_id":"2","to_system_id":"4"}
  ],
  "provenance":{"source":"FIELD-ADJACENCY-GENERATORS-0","generator_seed":0,"generator_shape":"inline_link_basis"}
}"#;

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
    front_registration_with_dims(adjacency, 1)
}

fn front_registration_with_dims(adjacency: FieldAdjacency, n_dims: u32) -> FieldSweepRegistration {
    let order = adjacency.apply_canonical_order_proof();
    apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency,
        n_dims,
        output: FieldSweepOutput::Matrix(col()),
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
        transient_read_proof: None,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })
    .expect("front registration")
}

fn bits_equal(left: &[f32], right: &[f32]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| left.to_bits() == right.to_bits())
}

fn gpu_context() -> Option<GpuContext> {
    match GpuContext::new_blocking() {
        Ok(context) => Some(context),
        Err(_) if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() => {
            panic!("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH set but no GPU adapter is available")
        }
        Err(_) => None,
    }
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
fn all_adjacencies_are_full_buffer_bit_exact_across_natural_bucketed_and_gpu_execution() {
    let n_dims = 3u32;
    let grid_slots = 9 * 9;
    let link_slots = 1_025u32;
    let link_edges = (0..link_slots - 1)
        .map(|slot| (slot, slot + 1, 1.0))
        .collect::<Vec<_>>();
    let cases = vec![
        (
            "n4",
            front_registration_with_dims(
                FieldAdjacency::grid_n4(9, 9, GRID_N4_NSEW, col()).expect("N4"),
                n_dims,
            ),
        ),
        (
            "n8",
            front_registration_with_dims(
                FieldAdjacency::grid_n8(9, 9, 1.0, 0.5, col()).expect("N8"),
                n_dims,
            ),
        ),
        (
            "radius-r",
            front_registration_with_dims(
                FieldAdjacency::grid_radius(9, 9, 2, &[1.0, 0.25], col()).expect("radius-r"),
                n_dims,
            ),
        ),
        (
            "linkgraph-1025",
            front_registration_with_dims(
                FieldAdjacency::link_graph(
                    link_slots,
                    undirected_rows(link_slots as usize, &link_edges),
                    col(),
                )
                .expect("LinkGraph >1024"),
                n_dims,
            ),
        ),
    ];

    let Some(context) = gpu_context() else {
        eprintln!("FIELD-ADJACENCY-GENERATORS-0: GPU leg skipped (no adapter)");
        return;
    };
    let adapter = context.adapter.get_info();
    for (name, registration) in cases {
        let slots = registration.slots();
        assert!(name != "linkgraph-1025" || slots > 1_024);
        assert!(name == "linkgraph-1025" || slots == grid_slots);
        let mut values = vec![0.0; slots as usize * n_dims as usize];
        for slot in 0..slots as usize {
            values[slot * n_dims as usize + 1] = slot as f32 * 0.125 + 3.0;
            values[slot * n_dims as usize + 2] = -(slot as f32) * 0.0625 - 5.0;
        }
        values[(slots as usize / 2) * n_dims as usize] = 1.0;

        for iterations in [1, 2] {
            let bucketed = execute_field_sweep_cpu_iterations(&values, &registration, iterations)
                .expect("bucketed CPU execution");
            let mut natural = values.clone();
            for _ in 0..iterations {
                natural = execute_field_sweep_cpu_natural_order(&natural, &registration)
                    .expect("natural-order CPU execution");
            }
            assert!(
                bits_equal(&bucketed, &natural),
                "degree buckets changed authored row folding for {name} at {iterations} iterations"
            );

            let mut session =
                FieldSweepSession::new(&context, &registration).expect("generic GPU session");
            session
                .upload_values(&context, &values)
                .expect("upload parity values");
            session
                .dispatch(&context, &registration, iterations)
                .expect("dispatch parity values");
            let gpu = session.readback(&context).expect("readback parity values");
            assert!(
                bits_equal(&bucketed, &gpu),
                "full-buffer CPU/GPU mismatch for {name} at {iterations} iterations"
            );
        }
    }
    eprintln!(
        "FIELD-ADJACENCY-GENERATORS-PARITY adapter={} backend={:?} N4/N8/radius-r/LinkGraph-1025=bit-exact",
        adapter.name, adapter.backend
    );
}

fn valid_minimal_request(adjacency: FieldAdjacency) -> FieldSweepRegistrationRequest {
    let order = adjacency.apply_canonical_order_proof();
    FieldSweepRegistrationRequest {
        adjacency,
        n_dims: 1,
        output: FieldSweepOutput::Matrix(col()),
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
        transient_read_proof: None,
        canonical_order_proof: Some(order),
        dt: 1.0,
    }
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
