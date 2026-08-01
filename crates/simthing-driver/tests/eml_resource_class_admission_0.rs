use simthing_core::{
    eml_opcode, ColumnIndex, EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry,
    EmlFormulaMeta, EmlNodeGpu, EmlRegistryError, EmlResourceClass, EmlTreeId,
};
use simthing_driver::{
    compile_gu_yang_n4_field_sweeps, compile_palma_n4_field_sweep, GuYangN4FieldSweepSpec,
    PalmaN4FieldSweepSpec,
};
use simthing_gpu::{
    apply_field_sweep_registration, field_param, CanonicalOrderProof, FieldAdjacency,
    FieldLawProof, FieldSweepOutput, FieldSweepRegistrationRequest,
};

fn admitted_col(raw: u32, n_dims: u32) -> ColumnIndex {
    ColumnIndex::try_from_admitted_authored(raw, n_dims).expect("bounded column")
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

fn literal(value: f32) -> EmlNodeGpu {
    node(eml_opcode::LITERAL_F32, value.to_bits())
}

fn exact_meta(tree_id: EmlTreeId) -> EmlFormulaMeta {
    EmlFormulaMeta {
        tree_id,
        execution_class: EmlExecutionClass::ExactDeterministic,
        allowed_consumers: EmlConsumerMask::default(),
        max_abs_error: None,
        deterministic_gpu: true,
        requires_guard_for_hard_threshold: false,
        node_count: 0,
        max_stack_depth: 0,
        has_loops: false,
        has_recursion: false,
        display_name: "resource-class-boundary".to_owned(),
    }
}

#[test]
fn eml_resource_class_boundaries_and_smallest_fit_are_deterministic() {
    assert_eq!(
        EmlResourceClass::smallest_fitting(16, 4),
        Some(EmlResourceClass::CompactStack4)
    );
    assert_eq!(
        EmlResourceClass::smallest_fitting(17, 4),
        Some(EmlResourceClass::LegacyFixed32)
    );
    assert_eq!(
        EmlResourceClass::smallest_fitting(32, 32),
        Some(EmlResourceClass::LegacyFixed32)
    );
    assert_eq!(EmlResourceClass::smallest_fitting(33, 32), None);
    assert_eq!(EmlResourceClass::smallest_fitting(32, 33), None);

    let mut registry = EmlExpressionRegistry::new();
    let boundary_id = EmlTreeId(700);
    registry
        .register_formula(boundary_id, exact_meta(boundary_id), vec![literal(1.0); 32])
        .expect("legacy 32-node / stack-32 boundary remains admitted");
    assert_eq!(
        registry.resource_class(boundary_id).expect("registered"),
        EmlResourceClass::LegacyFixed32
    );

    let over_id = EmlTreeId(701);
    let over = registry.register_formula(over_id, exact_meta(over_id), vec![literal(1.0); 33]);
    assert!(matches!(
        over,
        Err(EmlRegistryError::UnsupportedResourceClass {
            requested_nodes: 33,
            requested_stack: 33,
            attempted: EmlResourceClass::LegacyFixed32,
            max_nodes: 32,
            max_stack: 32,
        })
    ));
}

#[test]
fn admitted_field_census_maps_to_one_smallest_class_without_caller_sizes() {
    let palma = compile_palma_n4_field_sweep(PalmaN4FieldSweepSpec {
        width: 4,
        height: 4,
        n_dims: 2,
        d_col: admitted_col(0, 2),
        w_col: admitted_col(1, 2),
        destination_slot: simthing_core::SlotIndex::new(0),
        inf_sentinel: 1.0e20,
    })
    .expect("PALMA registration");
    assert_eq!(palma.resource_class(), EmlResourceClass::CompactStack4);

    let gu_yang = compile_gu_yang_n4_field_sweeps(GuYangN4FieldSweepSpec {
        width: 4,
        height: 4,
        n_dims: 2,
        value_col: admitted_col(0, 2),
        conductance_col: admitted_col(1, 2),
        saturation: 1.0,
        chi: 0.1,
        dt: 1.0,
    })
    .expect("Gu-Yang registrations");
    assert!(gu_yang
        .iter()
        .all(|registration| registration.resource_class() == EmlResourceClass::CompactStack4));

    let adjacency =
        FieldAdjacency::independent_slots(1, admitted_col(0, 1)).expect("independent adjacency");
    let order: CanonicalOrderProof = adjacency.apply_canonical_order_proof();
    let mut map_program = vec![literal(1.0); 5];
    map_program.extend((0..4).map(|_| node(eml_opcode::ADD, 0)));
    map_program.push(node(eml_opcode::RETURN_TOP, 0));
    let legacy = apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency,
        n_dims: 1,
        output: FieldSweepOutput::Matrix(admitted_col(0, 1)),
        map_program,
        fold_program: vec![
            node(eml_opcode::PARAM, field_param::ACCUMULATOR),
            node(eml_opcode::PARAM, field_param::MAPPED),
            node(eml_opcode::ADD, 0),
            node(eml_opcode::RETURN_TOP, 0),
        ],
        identity_bits: 0.0f32.to_bits(),
        post_program: vec![
            node(eml_opcode::PARAM, field_param::FOLDED),
            node(eml_opcode::RETURN_TOP, 0),
        ],
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        transient_read_proof: None,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })
    .expect("peak-stack five selects compatibility class");
    assert_eq!(legacy.resource_class(), EmlResourceClass::LegacyFixed32);
}
