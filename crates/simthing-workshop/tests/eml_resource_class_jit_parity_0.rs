//! Full admitted field-compiler census: CPU oracle, preserved GPU interpreter,
//! and canonical IR-generated JIT must agree bit-for-bit.

use simthing_core::{eml_opcode, ColumnIndex, EmlNodeGpu, EmlResourceClass};
use simthing_gpu::{
    apply_field_sweep_registration, compile_min_plus_field_sweep, compile_structured_field_sweeps,
    compile_w_impedance_field_sweeps, execute_field_sweep_cpu_chain,
    execute_field_sweep_cpu_iterations, field_param, FieldAdjacency, FieldLawProof,
    FieldSweepOutput, FieldSweepRegistration, FieldSweepRegistrationRequest, FieldSweepSession,
    GpuContext, MinPlusStencilConfig, StructuredFieldStencilBoundaryMode,
    StructuredFieldStencilConfig, StructuredFieldStencilMaskMode, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy, WImpedanceComposeConfig, WImpedanceComposeProfile,
    MIN_PLUS_INF, SATURATING_FLUX_CHI_CFL_MAX,
};

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

fn bits(values: &[f32]) -> Vec<u32> {
    values.iter().map(|value| value.to_bits()).collect()
}

fn cpu_result(
    values: &[f32],
    registrations: &[FieldSweepRegistration],
    iterations: u32,
) -> Vec<f32> {
    if registrations.len() == 1 {
        execute_field_sweep_cpu_iterations(values, &registrations[0], iterations)
            .expect("CPU census execution")
    } else {
        assert_eq!(iterations, 1, "multi-stage census is one admitted chain");
        execute_field_sweep_cpu_chain(values, registrations).expect("CPU census chain")
    }
}

fn gpu_result(
    ctx: &GpuContext,
    values: &[f32],
    registrations: &[FieldSweepRegistration],
    iterations: u32,
    interpreted: bool,
) -> Vec<f32> {
    let class = registrations
        .iter()
        .fold(registrations[0].resource_class(), |class, registration| {
            class.join(registration.resource_class())
        });
    let mut session = if interpreted {
        FieldSweepSession::new_interpreted_for_profiling(ctx, &registrations[0], class)
            .expect("preserved interpreted session")
    } else {
        FieldSweepSession::new_with_profiling_resource_class(ctx, &registrations[0], class)
            .expect("generated JIT session")
    };
    session.upload_values(ctx, values).expect("census upload");
    session
        .dispatch_chain(ctx, registrations, iterations)
        .expect("census dispatch chain");
    session.readback(ctx).expect("census readback")
}

fn assert_three_way(
    ctx: &GpuContext,
    name: &str,
    values: &[f32],
    registrations: &[FieldSweepRegistration],
    iterations: u32,
) {
    let cpu = cpu_result(values, registrations, iterations);
    let interpreted = gpu_result(ctx, values, registrations, iterations, true);
    let generated = gpu_result(ctx, values, registrations, iterations, false);
    assert_eq!(bits(&interpreted), bits(&cpu), "{name}: interpreter vs CPU");
    assert_eq!(bits(&generated), bits(&cpu), "{name}: JIT vs CPU");
    eprintln!(
        "EML_RC_JIT_PARITY case={name} programs={} class={:?} CPU/interpreted/JIT=bit-exact",
        registrations.len(),
        registrations
            .iter()
            .fold(registrations[0].resource_class(), |class, registration| {
                class.join(registration.resource_class())
            }),
    );
}

fn structured_config(operator: StructuredFieldStencilOperator) -> StructuredFieldStencilConfig {
    let (north, south, east, west) = StructuredFieldStencilConfig::zero_directional_weights();
    let saturating = matches!(
        operator,
        StructuredFieldStencilOperator::SaturatingFlux { .. }
    );
    StructuredFieldStencilConfig {
        width: 4,
        height: 4,
        n_dims: 4,
        source_col: 0,
        target_col: if saturating { 0 } else { 1 },
        horizon: 1,
        alpha_self: 0.25,
        gamma_neighbor: 0.5,
        weight_north: north,
        weight_south: south,
        weight_east: east,
        weight_west: west,
        source_cap: matches!(
            operator,
            StructuredFieldStencilOperator::SourceCappedNormalized
        )
        .then_some(0.75),
        operator,
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: if saturating {
            StructuredFieldStencilBoundaryMode::Clamp
        } else {
            StructuredFieldStencilBoundaryMode::Zero
        },
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    }
}

#[test]
fn eml_resource_class_jit_full_admission_census_is_three_way_bit_exact() {
    let Ok(ctx) = GpuContext::new_blocking() else {
        return;
    };

    let palma_config = MinPlusStencilConfig {
        width: 4,
        height: 4,
        n_dims: 2,
        d_col: 0,
        w_col: 1,
        dest_x: 1,
        dest_y: 1,
        inf_sentinel: MIN_PLUS_INF,
    };
    let mut palma_values = vec![0.0; palma_config.values_len()];
    for slot in 0..palma_config.width * palma_config.height {
        palma_values[slot as usize * 2] = MIN_PLUS_INF;
        palma_values[slot as usize * 2 + 1] = 1.0 + (slot % 5) as f32 * 0.125;
    }
    palma_values[((palma_config.dest_y * palma_config.width + palma_config.dest_x) * 2) as usize] =
        0.0;
    let palma = vec![compile_min_plus_field_sweep(&palma_config).expect("PALMA admission")];
    assert_three_way(&ctx, "PALMA", &palma_values, &palma, 3);

    let structured_values = (0..4 * 4 * 4)
        .map(|index| index as f32 * 0.03125 - 0.5)
        .collect::<Vec<_>>();
    for (name, operator) in [
        ("normalized", StructuredFieldStencilOperator::Normalized),
        (
            "source-capped",
            StructuredFieldStencilOperator::SourceCappedNormalized,
        ),
        (
            "gradient-xy",
            StructuredFieldStencilOperator::GradientXY { target_col_y: 2 },
        ),
        (
            "saturating-fused",
            StructuredFieldStencilOperator::SaturatingFlux {
                u_sat: 1.0,
                chi: SATURATING_FLUX_CHI_CFL_MAX,
                choke_output_col: None,
            },
        ),
        (
            "saturating-with-choke",
            StructuredFieldStencilOperator::SaturatingFlux {
                u_sat: 1.0,
                chi: SATURATING_FLUX_CHI_CFL_MAX,
                choke_output_col: Some(3),
            },
        ),
    ] {
        let registrations =
            compile_structured_field_sweeps(&structured_config(operator)).expect("field admission");
        assert_three_way(&ctx, name, &structured_values, &registrations, 1);
    }

    let w_config = WImpedanceComposeConfig {
        width: 4,
        height: 4,
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
    let w_values = (0..w_config.values_len())
        .map(|index| index as f32 * 0.015625 - 0.75)
        .collect::<Vec<_>>();
    let w = compile_w_impedance_field_sweeps(&w_config).expect("W admission");
    assert_three_way(&ctx, "W-impedance", &w_values, &w, 1);

    let legacy_col = ColumnIndex::try_from_admitted_authored(0, 1).expect("bounded column");
    let adjacency = FieldAdjacency::independent_slots(1, legacy_col).expect("legacy adjacency");
    let order = adjacency.apply_canonical_order_proof();
    let mut legacy_map = vec![literal(1.0); 5];
    legacy_map.extend((0..4).map(|_| node(eml_opcode::ADD, 0)));
    legacy_map.push(node(eml_opcode::RETURN_TOP, 0));
    let legacy = vec![
        apply_field_sweep_registration(FieldSweepRegistrationRequest {
            adjacency,
            n_dims: 1,
            output: FieldSweepOutput::Matrix(legacy_col),
            map_program: legacy_map,
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
        .expect("legacy-class planted boundary"),
    ];
    assert_eq!(legacy[0].resource_class(), EmlResourceClass::LegacyFixed32);
    assert_three_way(&ctx, "legacy-peak-stack-5", &[2.0], &legacy, 1);
}
