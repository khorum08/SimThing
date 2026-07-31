use simthing_core::{eml_opcode, ColumnIndex, EmlNodeGpu, SlotIndex};
use simthing_driver::{compile_palma_n4_field_sweep, PalmaN4FieldSweepSpec};
use simthing_gpu::{
    apply_field_sweep_registration, compile_min_plus_field_sweep, compile_structured_field_sweeps,
    cpu_horizon, cpu_min_plus_d_from_w, execute_field_sweep_cpu, execute_field_sweep_cpu_chain,
    execute_field_sweep_cpu_iterations, pack_w_and_initial_d, params_from_config, FieldAdjacency,
    FieldLawProof, FieldSweepAdmissionError, FieldSweepOutput, FieldSweepRegistrationRequest,
    FieldSweepResourceClassRequest, FieldSweepSession, GpuContext, MinPlusStencilConfig,
    MinPlusStencilOp, StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOp, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy, GRID_N4_NSEW, GRID_N4_WENS, MIN_PLUS_INF,
    SATURATING_FLUX_CHI_CFL_MAX,
};

fn bits_equal(left: &[f32], right: &[f32]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(a, b)| a.to_bits() == b.to_bits())
}

fn column(values: &[f32], n_dims: usize, col: usize) -> Vec<f32> {
    values.chunks_exact(n_dims).map(|row| row[col]).collect()
}

fn admitted_col(raw: u32, n_dims: u32) -> ColumnIndex {
    ColumnIndex::try_from_admitted_authored(raw, n_dims).expect("bounded authored column")
}

fn synthetic_w(width: u32, height: u32) -> Vec<f32> {
    let mut values = vec![1.0; (width * height) as usize];
    for y in 0..height {
        for x in 0..width {
            values[(y * width + x) as usize] = 1.0 + ((x * 3 + y * 5) % 7) as f32 * 0.125;
        }
    }
    values
}

fn synthetic_flux_values(width: u32, height: u32, n_dims: u32) -> Vec<f32> {
    let mut values = vec![0.0; (width * height * n_dims) as usize];
    for slot in 0..(width * height) as usize {
        for col in 1..n_dims as usize {
            values[slot * n_dims as usize + col] = (slot as f32 * 0.03125) + (col as f32 * 7.0);
        }
    }
    let center = (height / 2 * width + width / 2) as usize;
    values[center * n_dims as usize] = 0.8;
    values
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

#[test]
fn field_sweep_n4_parity_0_palma_and_gu_yang_are_bit_exact_cpu_and_gpu() {
    let width = 16;
    let height = 16;
    let iterations = 8;
    let w = synthetic_w(width, height);
    let palma_config = MinPlusStencilConfig {
        width,
        height,
        n_dims: 2,
        d_col: 0,
        w_col: 1,
        dest_x: 2,
        dest_y: 2,
        inf_sentinel: MIN_PLUS_INF,
    };
    let palma_values = pack_w_and_initial_d(&w, &palma_config).expect("pack PALMA values");
    let palma_registration =
        compile_min_plus_field_sweep(&palma_config).expect("admit PALMA registration");
    let legacy_palma_cpu =
        cpu_min_plus_d_from_w(&w, &palma_config, iterations).expect("PALMA CPU oracle");
    let generic_palma_cpu =
        execute_field_sweep_cpu_iterations(&palma_values, &palma_registration, iterations)
            .expect("generic PALMA CPU");
    assert!(
        bits_equal(&legacy_palma_cpu, &column(&generic_palma_cpu, 2, 0)),
        "authored PALMA registration must match the unedited CPU referee bit-for-bit"
    );

    let (north, south, east, west) = StructuredFieldStencilConfig::zero_directional_weights();
    let gu_yang_config = StructuredFieldStencilConfig {
        width,
        height,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        horizon: 1,
        alpha_self: 0.0,
        gamma_neighbor: 0.0,
        weight_north: north,
        weight_south: south,
        weight_east: east,
        weight_west: west,
        source_cap: None,
        operator: StructuredFieldStencilOperator::SaturatingFlux {
            u_sat: 1.0,
            chi: SATURATING_FLUX_CHI_CFL_MAX,
            choke_output_col: None,
        },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Clamp,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    };
    let gu_yang_values = synthetic_flux_values(width, height, gu_yang_config.n_dims);
    let legacy_gu_yang_cpu = cpu_horizon(&gu_yang_values, &params_from_config(&gu_yang_config), 1);
    let [conductance_registration, flux_registration] =
        compile_structured_field_sweeps(&gu_yang_config)
            .expect("admit Gu-Yang registrations")
            .try_into()
            .expect("BH-0 lowers to conductance plus flux");
    let generic_gu_yang_cpu = execute_field_sweep_cpu_chain(
        &gu_yang_values,
        &[conductance_registration.clone(), flux_registration.clone()],
    )
    .expect("generic transient-conductance plus flux CPU");
    assert!(
        bits_equal(&legacy_gu_yang_cpu, &generic_gu_yang_cpu),
        "authored Gu-Yang registration must preserve the full matrix against the unedited CPU referee"
    );

    let Some(context) = gpu_context() else {
        eprintln!("field_sweep_n4_parity_0: GPU leg skipped (no adapter)");
        return;
    };
    let adapter = context.adapter.get_info();

    let legacy_palma_gpu_op =
        MinPlusStencilOp::new(&context, palma_config).expect("legacy PALMA GPU op");
    legacy_palma_gpu_op
        .upload_values(&context, &palma_values)
        .expect("upload PALMA values");
    let legacy_palma_gpu = legacy_palma_gpu_op
        .run_ping_pong(&context, iterations)
        .expect("legacy PALMA GPU run");
    let mut generic_palma_gpu_session =
        FieldSweepSession::new(&context, &palma_registration).expect("generic PALMA GPU session");
    generic_palma_gpu_session
        .upload_values(&context, &palma_values)
        .expect("upload generic PALMA values");
    generic_palma_gpu_session
        .dispatch(&context, &palma_registration, iterations)
        .expect("dispatch generic PALMA");
    let generic_palma_gpu = generic_palma_gpu_session
        .readback(&context)
        .expect("read generic PALMA");
    assert!(bits_equal(
        &column(&legacy_palma_gpu, 2, 0),
        &column(&generic_palma_gpu, 2, 0)
    ));
    assert!(bits_equal(
        &column(&generic_palma_cpu, 2, 0),
        &column(&generic_palma_gpu, 2, 0)
    ));

    let legacy_gu_yang_gpu_op =
        StructuredFieldStencilOp::new(&context, gu_yang_config).expect("legacy Gu-Yang GPU op");
    legacy_gu_yang_gpu_op
        .upload_values(&context, &gu_yang_values)
        .expect("upload Gu-Yang values");
    let (legacy_gu_yang_gpu, _) = legacy_gu_yang_gpu_op
        .run_ping_pong(&context, 1)
        .expect("legacy Gu-Yang GPU run");
    let mut generic_gu_yang_gpu_session =
        FieldSweepSession::new(&context, &conductance_registration)
            .expect("generic Gu-Yang GPU session");
    generic_gu_yang_gpu_session
        .upload_values(&context, &gu_yang_values)
        .expect("upload generic Gu-Yang values");
    generic_gu_yang_gpu_session
        .dispatch(&context, &conductance_registration, 1)
        .expect("dispatch generic conductance");
    generic_gu_yang_gpu_session
        .dispatch(&context, &flux_registration, 1)
        .expect("dispatch generic flux");
    let generic_gu_yang_gpu = generic_gu_yang_gpu_session
        .readback(&context)
        .expect("read generic Gu-Yang");
    assert!(bits_equal(&legacy_gu_yang_gpu, &generic_gu_yang_gpu));
    assert!(bits_equal(&generic_gu_yang_cpu, &generic_gu_yang_gpu));

    eprintln!(
        "FIELD-SWEEP-N4-PARITY adapter={} backend={:?} PALMA=bit-exact Gu-Yang=bit-exact",
        adapter.name, adapter.backend
    );
}

fn node(opcode: u32, flags: u32, a: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode,
        flags,
        a,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn valid_request() -> FieldSweepRegistrationRequest {
    let adjacency = FieldAdjacency::grid_n4(4, 4, GRID_N4_WENS, admitted_col(0, 2)).expect("N4");
    let order = adjacency.apply_canonical_order_proof();
    FieldSweepRegistrationRequest {
        adjacency,
        n_dims: 2,
        output: FieldSweepOutput::Matrix(admitted_col(0, 2)),
        map_program: vec![
            node(eml_opcode::NEIGHBOR_VALUE, 0, 0),
            node(eml_opcode::RETURN_TOP, 0, 0),
        ],
        fold_program: vec![
            node(eml_opcode::PARAM, 0, simthing_gpu::field_param::ACCUMULATOR),
            node(eml_opcode::PARAM, 0, simthing_gpu::field_param::MAPPED),
            node(eml_opcode::ADD, 0, 0),
            node(eml_opcode::RETURN_TOP, 0, 0),
        ],
        identity_bits: 0.0f32.to_bits(),
        post_program: vec![
            node(eml_opcode::PARAM, 0, simthing_gpu::field_param::FOLDED),
            node(eml_opcode::RETURN_TOP, 0, 0),
        ],
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        transient_read_proof: None,
        canonical_order_proof: Some(order),
        resource_class: FieldSweepResourceClassRequest::default(),
        dt: 1.0,
    }
}

#[test]
fn field_sweep_n4_parity_0_typed_pre_dispatch_negatives_bite() {
    let mut missing_law = valid_request();
    missing_law.field_law_proof = None;
    assert!(matches!(
        apply_field_sweep_registration(missing_law),
        Err(FieldSweepAdmissionError::MissingFieldLawProof)
    ));

    let mut missing_order = valid_request();
    missing_order.canonical_order_proof = None;
    assert!(matches!(
        apply_field_sweep_registration(missing_order),
        Err(FieldSweepAdmissionError::MissingCanonicalOrderProof)
    ));

    let mut wrong_symmetry = valid_request();
    let other = FieldAdjacency::grid_n4(5, 4, GRID_N4_NSEW, admitted_col(0, 2)).expect("other N4");
    let other_conductance = other
        .apply_conductance_certificate(vec![0.1; other.slots() as usize], 1.0)
        .expect("other conductance certificate");
    wrong_symmetry.field_law_proof = Some(FieldLawProof::apply_conservative(
        other
            .apply_undirected_symmetry_certificate()
            .expect("other symmetry"),
        other_conductance,
    ));
    assert!(matches!(
        apply_field_sweep_registration(wrong_symmetry),
        Err(FieldSweepAdmissionError::UndirectedSymmetryCertificateMismatch)
    ));

    let mut non_default_resource = valid_request();
    non_default_resource.resource_class = FieldSweepResourceClassRequest {
        stack_slots: 64,
        max_program_nodes: 32,
    };
    assert!(matches!(
        apply_field_sweep_registration(non_default_resource),
        Err(FieldSweepAdmissionError::UnsupportedResourceClass { .. })
    ));

    let mut malformed_edge = valid_request();
    malformed_edge.post_program = vec![
        node(eml_opcode::NEIGHBOR_VALUE, 0, 0),
        node(eml_opcode::RETURN_TOP, 0, 0),
    ];
    assert!(matches!(
        apply_field_sweep_registration(malformed_edge),
        Err(FieldSweepAdmissionError::MalformedEdgeContext { .. })
    ));

    let mut missing_neighbor_slot = valid_request();
    missing_neighbor_slot.post_program = vec![
        node(
            eml_opcode::PARAM,
            0,
            simthing_gpu::field_param::NEIGHBOR_SLOT,
        ),
        node(eml_opcode::RETURN_TOP, 0, 0),
    ];
    assert!(matches!(
        apply_field_sweep_registration(missing_neighbor_slot),
        Err(FieldSweepAdmissionError::MalformedEdgeContext { .. })
    ));

    let mut invalid_output = valid_request();
    invalid_output.output =
        FieldSweepOutput::Matrix(ColumnIndex::from_raw_for_oracle_or_rehearsal(2));
    assert!(matches!(
        apply_field_sweep_registration(invalid_output),
        Err(FieldSweepAdmissionError::InvalidOutputColumn { .. })
    ));

    let mut producer_request = valid_request();
    producer_request.output = FieldSweepOutput::Transient;
    let producer =
        apply_field_sweep_registration(producer_request).expect("admit transient producer");
    let transient = producer
        .apply_transient_certificate()
        .expect("mint transient witness");
    let mut consumer_request = valid_request();
    consumer_request.post_program = vec![
        node(
            eml_opcode::PARAM,
            0,
            simthing_gpu::field_param::TARGET_TRANSIENT,
        ),
        node(eml_opcode::RETURN_TOP, 0, 0),
    ];
    assert!(matches!(
        apply_field_sweep_registration(consumer_request.clone()),
        Err(FieldSweepAdmissionError::MissingTransientReadProof)
    ));
    let mut other_producer_request = valid_request();
    other_producer_request.adjacency =
        FieldAdjacency::grid_n4(5, 4, GRID_N4_WENS, admitted_col(0, 2)).expect("other N4");
    other_producer_request.canonical_order_proof = Some(
        other_producer_request
            .adjacency
            .apply_canonical_order_proof(),
    );
    other_producer_request.output = FieldSweepOutput::Transient;
    let wrong_transient = apply_field_sweep_registration(other_producer_request)
        .expect("other transient producer")
        .apply_transient_certificate()
        .expect("other transient witness");
    consumer_request.transient_read_proof = Some(wrong_transient);
    assert!(matches!(
        apply_field_sweep_registration(consumer_request.clone()),
        Err(FieldSweepAdmissionError::TransientReadProofMismatch)
    ));
    consumer_request.transient_read_proof = Some(transient);
    let consumer =
        apply_field_sweep_registration(consumer_request).expect("admit transient consumer");
    assert!(matches!(
        execute_field_sweep_cpu(&vec![0.0; 4 * 4 * 2], &consumer),
        Err(simthing_gpu::FieldSweepExecutionError::TransientNotInitialized)
    ));

    let mut invalid_gather = valid_request();
    invalid_gather.adjacency = FieldAdjacency::grid_n4(
        4,
        4,
        GRID_N4_WENS,
        ColumnIndex::from_raw_for_oracle_or_rehearsal(2),
    )
    .expect("structurally valid N4 with forged test column");
    invalid_gather.canonical_order_proof =
        Some(invalid_gather.adjacency.apply_canonical_order_proof());
    assert!(matches!(
        apply_field_sweep_registration(invalid_gather),
        Err(FieldSweepAdmissionError::InvalidGatherColumn { .. })
    ));

    assert!(matches!(
        compile_palma_n4_field_sweep(PalmaN4FieldSweepSpec {
            width: 4,
            height: 4,
            n_dims: 2,
            d_col: admitted_col(0, 2),
            w_col: admitted_col(1, 2),
            destination_slot: SlotIndex::new(16),
            inf_sentinel: MIN_PLUS_INF,
        }),
        Err(FieldSweepAdmissionError::InvalidDestinationSlot { .. })
    ));
}

#[test]
fn field_sweep_n4_parity_0_session_binding_mismatch_rejects_before_dispatch() {
    let registration_a =
        apply_field_sweep_registration(valid_request()).expect("admit session registration A");
    let mut request_b = valid_request();
    request_b.adjacency =
        FieldAdjacency::grid_n4(4, 4, GRID_N4_NSEW, admitted_col(0, 2)).expect("N4 B");
    request_b.canonical_order_proof = Some(request_b.adjacency.apply_canonical_order_proof());
    let registration_b =
        apply_field_sweep_registration(request_b).expect("admit equal-sized registration B");
    assert_eq!(
        registration_a.slots() * registration_a.n_dims(),
        registration_b.slots() * registration_b.n_dims(),
        "negative control must preserve total scalar length"
    );

    let Some(context) = gpu_context() else {
        eprintln!("field_sweep_n4_parity_0 session-binding leg skipped (no adapter)");
        return;
    };
    let mut session =
        FieldSweepSession::new(&context, &registration_a).expect("create session for A");
    session
        .upload_values(&context, &vec![0.0; 4 * 4 * 2])
        .expect("upload A-shaped values");
    assert!(matches!(
        session.dispatch(&context, &registration_b, 1),
        Err(simthing_gpu::FieldSweepExecutionError::RegistrationBindingChanged)
    ));
}
