//! FIELD-SWEEP-N4-PARITY-0 — authored PALMA and Gu-Yang registrations for
//! the one generic field-sweep door.

use simthing_core::{eml_opcode, ColumnIndex, EmlNodeGpu, SlotIndex};
use simthing_gpu::{
    apply_field_sweep_registration, encode_column, field_param, FieldAdjacency, FieldLawProof,
    FieldSweepAdmissionError, FieldSweepOutput, FieldSweepRegistration,
    FieldSweepRegistrationRequest, GRID_N4_NSEW, GRID_N4_WENS,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PalmaN4FieldSweepSpec {
    pub width: u32,
    pub height: u32,
    pub n_dims: u32,
    pub d_col: ColumnIndex,
    pub w_col: ColumnIndex,
    pub destination_slot: SlotIndex,
    pub inf_sentinel: f32,
}

pub fn compile_palma_n4_field_sweep(
    spec: PalmaN4FieldSweepSpec,
) -> Result<FieldSweepRegistration, FieldSweepAdmissionError> {
    let adjacency = FieldAdjacency::grid_n4(spec.width, spec.height, GRID_N4_WENS, spec.d_col)?;
    if spec.destination_slot.raw() >= adjacency.slots() {
        return Err(FieldSweepAdmissionError::InvalidDestinationSlot {
            slot: spec.destination_slot,
            slots: adjacency.slots(),
        });
    }
    let canonical_order_proof = adjacency.apply_canonical_order_proof();
    let map_program = vec![neighbor(spec.d_col), ret()];
    let fold_program = vec![
        param(field_param::ACCUMULATOR),
        param(field_param::MAPPED),
        binary(eml_opcode::MIN),
        ret(),
    ];
    // Destination pinning is authored EML over the target-slot member of the
    // edge context; it is data, not a kernel branch.
    let post_program = vec![
        param(field_param::TARGET_SLOT),
        literal(spec.destination_slot.raw() as f32),
        binary(eml_opcode::CMP_EQ),
        literal(0.0),
        target(spec.w_col),
        param(field_param::FOLDED),
        binary(eml_opcode::ADD),
        select(),
        ret(),
    ];
    apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency,
        n_dims: spec.n_dims,
        output: FieldSweepOutput::Matrix(spec.d_col),
        map_program,
        fold_program,
        identity_bits: spec.inf_sentinel.to_bits(),
        post_program,
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        transient_read_proof: None,
        canonical_order_proof: Some(canonical_order_proof),
        dt: 1.0,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GuYangN4FieldSweepSpec {
    pub width: u32,
    pub height: u32,
    pub n_dims: u32,
    pub value_col: ColumnIndex,
    pub conductance_col: ColumnIndex,
    pub saturation: f32,
    pub chi: f32,
    pub dt: f32,
}

/// The conservative flux instance is two ordinary authored registrations:
/// conductance materialization followed by the conservative update. Both use
/// the same fixed N4 gather and the second registration carries the symmetry
/// certificate.
pub fn compile_gu_yang_n4_field_sweeps(
    spec: GuYangN4FieldSweepSpec,
) -> Result<[FieldSweepRegistration; 2], FieldSweepAdmissionError> {
    let adjacency = FieldAdjacency::grid_n4(spec.width, spec.height, GRID_N4_NSEW, spec.value_col)?;
    let canonical_order_proof = adjacency.apply_canonical_order_proof();

    let conductance_map = vec![
        neighbor(spec.value_col),
        literal(spec.saturation),
        safe_binary(eml_opcode::DIV),
        clamp_bounded(0.0, 1.0),
        unary(eml_opcode::NEG),
        literal(1.0),
        binary(eml_opcode::ADD),
        ret(),
    ];
    let conductance_fold = vec![
        param(field_param::ACCUMULATOR),
        param(field_param::MAPPED),
        binary(eml_opcode::MUL),
        ret(),
    ];
    let conductance_post = vec![param(field_param::FOLDED), ret()];
    let conductance = apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency: adjacency.clone(),
        n_dims: spec.n_dims,
        output: FieldSweepOutput::Matrix(spec.conductance_col),
        map_program: conductance_map,
        fold_program: conductance_fold,
        identity_bits: spec.chi.to_bits(),
        post_program: conductance_post,
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        transient_read_proof: None,
        canonical_order_proof: Some(canonical_order_proof),
        dt: spec.dt,
    })?;

    let flux_map = vec![
        target(spec.conductance_col),
        neighbor(spec.conductance_col),
        binary(eml_opcode::ADD),
        literal(0.5),
        binary(eml_opcode::MUL),
        neighbor(spec.value_col),
        target(spec.value_col),
        binary(eml_opcode::SUB),
        binary(eml_opcode::MUL),
        ret(),
    ];
    let flux_fold = vec![
        param(field_param::ACCUMULATOR),
        param(field_param::MAPPED),
        binary(eml_opcode::ADD),
        ret(),
    ];
    let flux_post = vec![
        target(spec.value_col),
        param(field_param::FOLDED),
        binary(eml_opcode::ADD),
        ret(),
    ];
    let symmetry = adjacency.apply_undirected_symmetry_certificate()?;
    let conductance_certificate =
        adjacency.apply_conductance_certificate(vec![spec.chi; adjacency.slots() as usize], 1.0)?;
    let flux = apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency,
        n_dims: spec.n_dims,
        output: FieldSweepOutput::Matrix(spec.value_col),
        map_program: flux_map,
        fold_program: flux_fold,
        identity_bits: 0.0f32.to_bits(),
        post_program: flux_post,
        field_law_proof: Some(FieldLawProof::apply_conservative(
            symmetry,
            conductance_certificate,
        )),
        transient_read_proof: None,
        canonical_order_proof: Some(canonical_order_proof),
        dt: spec.dt,
    })?;
    Ok([conductance, flux])
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SteadExponentialFalloffSpec {
    pub width: u32,
    pub height: u32,
    pub n_dims: u32,
    /// Source intensity column (`u_j` read from each neighbor).
    pub value_col: ColumnIndex,
    /// Output column receiving the falloff-weighted accumulation.
    pub output_col: ColumnIndex,
    /// Decay rate `λ ≥ 0`; distance `d` arrives per edge as `EDGE_SCALAR`.
    pub lambda: f32,
    pub dt: f32,
}

/// EML-EXP-PRIMITIVE-0 consumer: STEAD falloff as an authored LAW —
/// `Σ_j u_j · EXP(−λ·d_j)` — replacing per-hop-band weight tables
/// (full_eml_unification §7 "exact exponential falloff"). Distance rides the
/// existing `EDGE_SCALAR` seam; the exponential argument is `≤ 0` for
/// admitted `λ, d ≥ 0` and is explicitly `CLAMP_BOUNDED`-guarded (5.10
/// shape 2 — saturated tail beyond the domain floor, where the true weight is
/// below 1.2e-38 and semantically zero). One ordinary registration through
/// the one generic door; no new mechanism.
pub fn compile_stead_exponential_falloff_field_sweep(
    spec: SteadExponentialFalloffSpec,
) -> Result<FieldSweepRegistration, FieldSweepAdmissionError> {
    let adjacency = FieldAdjacency::grid_n4(spec.width, spec.height, GRID_N4_NSEW, spec.value_col)?;
    let canonical_order_proof = adjacency.apply_canonical_order_proof();
    let map_program = vec![
        neighbor(spec.value_col),
        param(field_param::EDGE_SCALAR),
        literal(spec.lambda),
        binary(eml_opcode::MUL),
        unary(eml_opcode::NEG),
        exp_saturation_guard(),
        unary(eml_opcode::EXP),
        binary(eml_opcode::MUL),
        ret(),
    ];
    let fold_program = vec![
        param(field_param::ACCUMULATOR),
        param(field_param::MAPPED),
        binary(eml_opcode::ADD),
        ret(),
    ];
    let post_program = vec![param(field_param::FOLDED), ret()];
    apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency,
        n_dims: spec.n_dims,
        output: FieldSweepOutput::Matrix(spec.output_col),
        map_program,
        fold_program,
        identity_bits: 0.0f32.to_bits(),
        post_program,
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        transient_read_proof: None,
        canonical_order_proof: Some(canonical_order_proof),
        dt: spec.dt,
    })
}

/// CPU twin of the falloff weight for one edge (bit-exact with the map program).
pub fn stead_exponential_falloff_weight_oracle(lambda: f32, distance: f32) -> f32 {
    let arg = (-(distance * lambda)).clamp(
        f32::from_bits(simthing_core::EML_EXP_DOMAIN_MIN_BITS),
        f32::from_bits(simthing_core::EML_EXP_SATURATION_CEILING_BITS),
    );
    simthing_core::eml_exp_pinned_f32(arg)
}

/// The canonical saturated-tail guard: `CLAMP_BOUNDED(x, exp_domain_min, 0)`.
fn exp_saturation_guard() -> EmlNodeGpu {
    node(
        eml_opcode::CLAMP_BOUNDED,
        0,
        simthing_core::EML_EXP_DOMAIN_MIN_BITS,
        simthing_core::EML_EXP_SATURATION_CEILING_BITS,
    )
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

fn clamp_bounded(min: f32, max: f32) -> EmlNodeGpu {
    node(eml_opcode::CLAMP_BOUNDED, 0, min.to_bits(), max.to_bits())
}

fn select() -> EmlNodeGpu {
    node(eml_opcode::SELECT, 0, 0, 0)
}

fn ret() -> EmlNodeGpu {
    node(eml_opcode::RETURN_TOP, 0, 0, 0)
}
