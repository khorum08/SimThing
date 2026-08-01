//! Production lowering of established field descriptors into the one generic
//! field-sweep registration surface. Legacy operator modules remain parity
//! oracles; this module does not dispatch their shaders.

use simthing_core::{eml_opcode, ColumnIndex, EmlNodeGpu, SlotIndex};
use simthing_kernel::{
    apply_field_sweep_registration, encode_column, field_param, FieldAdjacency, FieldLawProof,
    FieldSweepAdmissionError, FieldSweepOutput, FieldSweepRegistration,
    FieldSweepRegistrationRequest, GridOffset, GRID_N4_NSEW, GRID_N4_WENS,
};
use thiserror::Error;

use crate::min_plus_stencil::{MinPlusStencilConfig, MinPlusStencilError};
use crate::structured_field_stencil::{
    StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig, StructuredFieldStencilError,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOperator,
};
use crate::w_impedance_compose::{WImpedanceComposeConfig, WImpedanceComposeError};

#[derive(Debug, Error)]
pub enum FieldSweepInstanceError {
    #[error(transparent)]
    Admission(#[from] FieldSweepAdmissionError),
    #[error(transparent)]
    MinPlus(#[from] MinPlusStencilError),
    #[error(transparent)]
    Structured(#[from] StructuredFieldStencilError),
    #[error(transparent)]
    WCompose(#[from] WImpedanceComposeError),
    #[error("structured field mask mode requires an admitted adjacency mask axis")]
    UnsupportedMaskMode,
    #[error("clamped boundary sampling is not admitted for this field law")]
    UnsupportedClampBoundary,
    #[error("field instance column {raw} is outside n_dims {n_dims}")]
    InvalidColumn { raw: u32, n_dims: u32 },
}

pub fn compile_min_plus_field_sweep(
    config: &MinPlusStencilConfig,
) -> Result<FieldSweepRegistration, FieldSweepInstanceError> {
    config.validate()?;
    let d_col = admitted_col(config.d_col, config.n_dims)?;
    let w_col = admitted_col(config.w_col, config.n_dims)?;
    let destination_slot = SlotIndex::new(config.dest_y * config.width + config.dest_x);
    let adjacency = FieldAdjacency::grid_n4(config.width, config.height, GRID_N4_WENS, d_col)?;
    let order = adjacency.apply_canonical_order_proof();
    Ok(apply_field_sweep_registration(
        FieldSweepRegistrationRequest {
            adjacency,
            n_dims: config.n_dims,
            output: FieldSweepOutput::Matrix(d_col),
            map_program: vec![neighbor(d_col), ret()],
            fold_program: vec![
                param(field_param::ACCUMULATOR),
                param(field_param::MAPPED),
                binary(eml_opcode::MIN),
                ret(),
            ],
            identity_bits: config.inf_sentinel.to_bits(),
            post_program: vec![
                param(field_param::TARGET_SLOT),
                literal(destination_slot.raw() as f32),
                binary(eml_opcode::CMP_EQ),
                literal(0.0),
                target(w_col),
                param(field_param::FOLDED),
                binary(eml_opcode::ADD),
                select(),
                ret(),
            ],
            field_law_proof: Some(FieldLawProof::apply_non_conservative()),
            transient_read_proof: None,
            canonical_order_proof: Some(order),
            dt: 1.0,
        },
    )?)
}

pub fn compile_w_impedance_field_sweeps(
    config: &WImpedanceComposeConfig,
) -> Result<Vec<FieldSweepRegistration>, FieldSweepInstanceError> {
    config.validate()?;
    let base_w_col = admitted_col(config.base_w_col, config.n_dims)?;
    let choke_a_col = admitted_col(config.choke_a_col, config.n_dims)?;
    let choke_b_col = admitted_col(config.choke_b_col, config.n_dims)?;
    let adjacency = FieldAdjacency::independent_slots(config.cells(), base_w_col)?;
    let mut registrations = Vec::with_capacity(config.profiles.len());
    for profile in &config.profiles {
        let output_col = admitted_col(profile.output_w_col, config.n_dims)?;
        let order = adjacency.apply_canonical_order_proof();
        registrations.push(apply_field_sweep_registration(
            FieldSweepRegistrationRequest {
                adjacency: adjacency.clone(),
                n_dims: config.n_dims,
                output: FieldSweepOutput::Matrix(output_col),
                map_program: vec![literal(0.0), ret()],
                fold_program: vec![param(field_param::MAPPED), ret()],
                identity_bits: 0,
                post_program: vec![
                    target(base_w_col),
                    literal(profile.weight_a),
                    target(choke_a_col),
                    binary(eml_opcode::MUL),
                    binary(eml_opcode::ADD),
                    literal(profile.weight_b),
                    target(choke_b_col),
                    binary(eml_opcode::MUL),
                    binary(eml_opcode::ADD),
                    ret(),
                ],
                field_law_proof: Some(FieldLawProof::apply_non_conservative()),
                transient_read_proof: None,
                canonical_order_proof: Some(order),
                dt: 1.0,
            },
        )?);
    }
    Ok(registrations)
}

pub fn compile_structured_field_sweeps(
    config: &StructuredFieldStencilConfig,
) -> Result<Vec<FieldSweepRegistration>, FieldSweepInstanceError> {
    config.validate()?;
    if config.mask_mode != StructuredFieldStencilMaskMode::All {
        return Err(FieldSweepInstanceError::UnsupportedMaskMode);
    }
    if let StructuredFieldStencilOperator::SaturatingFlux {
        u_sat,
        chi,
        choke_output_col,
    } = config.operator
    {
        return compile_saturating_flux(config, u_sat, chi, choke_output_col);
    }
    if config.boundary_mode != StructuredFieldStencilBoundaryMode::Zero {
        return Err(FieldSweepInstanceError::UnsupportedClampBoundary);
    }

    let source_col = admitted_col(config.source_col, config.n_dims)?;
    let (north, south, east, west) = config.resolved_directional_weights();
    if let StructuredFieldStencilOperator::GradientXY { target_col_y } = config.operator {
        return Ok(vec![
            compile_weighted_linear(
                config,
                source_col,
                admitted_col(config.target_col, config.n_dims)?,
                0.0,
                &[(1, 0, east), (-1, 0, west)],
                None,
            )?,
            compile_weighted_linear(
                config,
                source_col,
                admitted_col(target_col_y, config.n_dims)?,
                0.0,
                &[(0, -1, north), (0, 1, south)],
                None,
            )?,
        ]);
    }
    let cap = match config.operator {
        StructuredFieldStencilOperator::SourceCappedNormalized => config.source_cap,
        _ => None,
    };
    Ok(vec![compile_weighted_linear(
        config,
        source_col,
        admitted_col(config.target_col, config.n_dims)?,
        config.alpha_self,
        &[(0, -1, north), (0, 1, south), (1, 0, east), (-1, 0, west)],
        cap,
    )?])
}

fn compile_weighted_linear(
    config: &StructuredFieldStencilConfig,
    source_col: ColumnIndex,
    output_col: ColumnIndex,
    alpha_self: f32,
    weighted_offsets: &[(i32, i32, f32)],
    cap: Option<f32>,
) -> Result<FieldSweepRegistration, FieldSweepInstanceError> {
    let offsets: Vec<_> = weighted_offsets
        .iter()
        .filter(|(_, _, weight)| *weight != 0.0)
        .map(|&(dx, dy, weight)| GridOffset::new(dx, dy, weight))
        .collect();
    let adjacency = if offsets.is_empty() {
        FieldAdjacency::independent_slots(config.cells(), source_col)?
    } else {
        FieldAdjacency::grid_offsets(config.width, config.height, offsets, source_col)?
    };
    let order = adjacency.apply_canonical_order_proof();
    let mut post_program = vec![
        literal(alpha_self),
        target(source_col),
        binary(eml_opcode::MUL),
        param(field_param::FOLDED),
        binary(eml_opcode::ADD),
    ];
    if let Some(cap) = cap {
        post_program.push(node(
            eml_opcode::CLAMP_BOUNDED,
            0,
            0.0f32.to_bits(),
            cap.to_bits(),
        ));
    }
    post_program.push(ret());
    Ok(apply_field_sweep_registration(
        FieldSweepRegistrationRequest {
            adjacency,
            n_dims: config.n_dims,
            output: FieldSweepOutput::Matrix(output_col),
            map_program: vec![
                neighbor(source_col),
                param(field_param::EDGE_SCALAR),
                binary(eml_opcode::MUL),
                ret(),
            ],
            fold_program: vec![
                param(field_param::ACCUMULATOR),
                param(field_param::MAPPED),
                binary(eml_opcode::ADD),
                ret(),
            ],
            identity_bits: 0.0f32.to_bits(),
            post_program,
            field_law_proof: Some(FieldLawProof::apply_non_conservative()),
            transient_read_proof: None,
            canonical_order_proof: Some(order),
            dt: 1.0,
        },
    )?)
}

fn compile_saturating_flux(
    config: &StructuredFieldStencilConfig,
    u_sat: f32,
    chi: f32,
    choke_output_col: Option<u32>,
) -> Result<Vec<FieldSweepRegistration>, FieldSweepInstanceError> {
    let value_col = admitted_col(config.source_col, config.n_dims)?;
    let adjacency = FieldAdjacency::grid_n4(config.width, config.height, GRID_N4_NSEW, value_col)?;
    let order = adjacency.apply_canonical_order_proof();
    let conductance = apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency: adjacency.clone(),
        n_dims: config.n_dims,
        output: FieldSweepOutput::Transient,
        map_program: vec![
            neighbor(value_col),
            literal(u_sat),
            safe_binary(eml_opcode::DIV),
            node(
                eml_opcode::CLAMP_BOUNDED,
                0,
                0.0f32.to_bits(),
                1.0f32.to_bits(),
            ),
            unary(eml_opcode::NEG),
            literal(1.0),
            binary(eml_opcode::ADD),
            ret(),
        ],
        fold_program: vec![
            param(field_param::ACCUMULATOR),
            param(field_param::MAPPED),
            binary(eml_opcode::MUL),
            ret(),
        ],
        identity_bits: chi.to_bits(),
        post_program: vec![param(field_param::FOLDED), ret()],
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        transient_read_proof: None,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })?;
    let transient = conductance.apply_transient_certificate()?;

    let symmetry = adjacency.apply_undirected_symmetry_certificate()?;
    let certificate =
        adjacency.apply_conductance_certificate(vec![chi; adjacency.slots() as usize], 1.0)?;
    let flux = apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency: adjacency.clone(),
        n_dims: config.n_dims,
        output: FieldSweepOutput::Matrix(value_col),
        map_program: vec![
            param(field_param::TARGET_TRANSIENT),
            param(field_param::NEIGHBOR_TRANSIENT),
            binary(eml_opcode::ADD),
            literal(0.5),
            binary(eml_opcode::MUL),
            neighbor(value_col),
            target(value_col),
            binary(eml_opcode::SUB),
            binary(eml_opcode::MUL),
            ret(),
        ],
        fold_program: vec![
            param(field_param::ACCUMULATOR),
            param(field_param::MAPPED),
            binary(eml_opcode::ADD),
            ret(),
        ],
        identity_bits: 0.0f32.to_bits(),
        post_program: vec![
            target(value_col),
            param(field_param::FOLDED),
            binary(eml_opcode::ADD),
            ret(),
        ],
        field_law_proof: Some(FieldLawProof::apply_conservative(symmetry, certificate)),
        transient_read_proof: Some(transient),
        canonical_order_proof: Some(order),
        dt: 1.0,
    })?;
    let mut registrations = vec![conductance, flux];
    if let Some(choke_raw) = choke_output_col {
        let choke_col = admitted_col(choke_raw, config.n_dims)?;
        let choke_order = adjacency.apply_canonical_order_proof();
        registrations.push(apply_field_sweep_registration(
            FieldSweepRegistrationRequest {
                adjacency,
                n_dims: config.n_dims,
                output: FieldSweepOutput::Matrix(choke_col),
                map_program: vec![literal(0.0), ret()],
                fold_program: vec![param(field_param::MAPPED), ret()],
                identity_bits: 0,
                post_program: vec![
                    param(field_param::TARGET_TRANSIENT),
                    literal(chi),
                    safe_binary(eml_opcode::DIV),
                    unary(eml_opcode::NEG),
                    literal(1.0),
                    binary(eml_opcode::ADD),
                    node(
                        eml_opcode::CLAMP_BOUNDED,
                        0,
                        0.0f32.to_bits(),
                        1.0f32.to_bits(),
                    ),
                    ret(),
                ],
                field_law_proof: Some(FieldLawProof::apply_non_conservative()),
                transient_read_proof: Some(transient),
                canonical_order_proof: Some(choke_order),
                dt: 1.0,
            },
        )?);
    }
    Ok(registrations)
}

fn admitted_col(raw: u32, n_dims: u32) -> Result<ColumnIndex, FieldSweepInstanceError> {
    ColumnIndex::try_from_admitted_authored(raw, n_dims)
        .map_err(|_| FieldSweepInstanceError::InvalidColumn { raw, n_dims })
}

fn node(opcode: u32, flags: u32, a: u32, b: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode,
        flags,
        a,
        b,
        c: 0,
        d: 0,
    }
}

fn literal(value: f32) -> EmlNodeGpu {
    node(eml_opcode::LITERAL_F32, 0, value.to_bits(), 0)
}
fn target(col: ColumnIndex) -> EmlNodeGpu {
    node(eml_opcode::TARGET_VALUE, 0, encode_column(col), 0)
}
fn neighbor(col: ColumnIndex) -> EmlNodeGpu {
    node(eml_opcode::NEIGHBOR_VALUE, 0, encode_column(col), 0)
}
fn param(index: u32) -> EmlNodeGpu {
    node(eml_opcode::PARAM, 0, index, 0)
}
fn unary(opcode: u32) -> EmlNodeGpu {
    node(opcode, 0, 0, 0)
}
fn binary(opcode: u32) -> EmlNodeGpu {
    node(opcode, 0, 0, 0)
}
fn safe_binary(opcode: u32) -> EmlNodeGpu {
    node(opcode, 1, 0, 0)
}
fn select() -> EmlNodeGpu {
    node(eml_opcode::SELECT, 0, 0, 0)
}
fn ret() -> EmlNodeGpu {
    node(eml_opcode::RETURN_TOP, 0, 0, 0)
}
