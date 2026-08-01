//! GUYANG-COMPARATIVE-PROJECTIONS-0 — comparative **consumer** over co-located
//! generic field-sweep outputs.
//!
//! Scope envelope (Remand 1A / handoff surfaces): lives in `simthing-driver` only.
//! Uses already-landed field-sweep admission/execution doors. Does **not** add
//! kernel/GPU public doors or sanctioned-surface allowlist rows.
//!
//! Algebra and owner-count stay out of the executor: admission unrolls the exact
//! emitter set into ordinary field-EML map/fold/post data over one generic
//! adjacency axis.

use std::collections::BTreeSet;

use simthing_core::{eml_opcode, ColumnIndex, EmlNodeGpu, SlotIndex};
use simthing_gpu::{
    apply_field_sweep_registration, encode_column, field_param, FieldAdjacency, FieldLawProof,
    FieldSweepAdmissionError, FieldSweepOutput, FieldSweepRegistration,
    FieldSweepRegistrationRequest, FieldTransientCertificate,
};
use thiserror::Error;

/// Fixed comparative + border/chokepoint derived column count. Independent of
/// the admitted emitter-class count (never one pipeline per owner).
pub const COMPARATIVE_DERIVED_COLUMN_COUNT: u32 = 5;

/// Authored event-kind tokens for ordinary anchored threshold bands.
pub mod comparative_event_kind {
    pub const FRONT_FORMED: u32 = 0x4759_0001;
    pub const FRONT_HARDENED: u32 = 0x4759_0002;
    pub const CHOKEPOINT_EMERGED: u32 = 0x4759_0003;
}

/// One competing emitter class. Vec order is the deterministic authored
/// tie-break: earlier entries win exact value ties.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComparativeEmitterClass {
    /// Authored class identity written into the dominance column.
    pub class_id: f32,
    /// Co-located field-sweep output column for this emitter.
    pub value_col: ColumnIndex,
}

/// Fixed output columns for the sealed projection bundle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComparativeProjectionOutputs {
    pub dominance_col: ColumnIndex,
    pub margin_col: ColumnIndex,
    pub contest_col: ColumnIndex,
    pub border_col: ColumnIndex,
    pub chokepoint_col: ColumnIndex,
}

/// Band thresholds that quantize the *reading* only — never the field math.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComparativeProjectionBands {
    pub both_strong_floor: f32,
    pub small_margin: f32,
    pub palma_low_d: f32,
    pub contested_border_floor: f32,
}

impl Default for ComparativeProjectionBands {
    fn default() -> Self {
        Self {
            both_strong_floor: 0.25,
            small_margin: 0.15,
            palma_low_d: 4.0,
            contested_border_floor: 0.5,
        }
    }
}

/// Scenario-neutral comparative projection request over already-admitted columns.
#[derive(Clone, Debug)]
pub struct ComparativeProjectionRequest {
    pub adjacency: FieldAdjacency,
    pub n_dims: u32,
    pub emitters: Vec<ComparativeEmitterClass>,
    pub outputs: ComparativeProjectionOutputs,
    pub palma_d_col: ColumnIndex,
    /// Gu-Yang flux-stall magnitude column (`1−C/χ` or equivalent admitted stall
    /// readout). Required for truthful contest; not a runner-up proxy.
    pub guyang_stall_col: ColumnIndex,
    pub bands: ComparativeProjectionBands,
    pub authored_opt_out_reason: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComparativeProjectionDisposition {
    InsufficientEmitters { emitter_count: u32 },
    AuthoredOptOut { reason: &'static str },
    Born {
        emitter_count: u32,
        derived_column_count: u32,
    },
}

#[derive(Clone, Debug)]
pub struct ComparativeProjectionBundle {
    pub disposition: ComparativeProjectionDisposition,
    pub registrations: Vec<FieldSweepRegistration>,
    pub outputs: ComparativeProjectionOutputs,
    pub emitter_count: u32,
    pub derived_column_count: u32,
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum ComparativeProjectionError {
    #[error("comparative projection requires at least one emitter class")]
    EmptyEmitters,
    #[error("duplicate emitter value column {col}")]
    DuplicateEmitterColumn { col: u32 },
    #[error("emitter class_id must be finite (got {0})")]
    NonFiniteClassId(f32),
    #[error("emitter value column {col} out of range for n_dims {n_dims}")]
    EmitterColumnOutOfRange { col: u32, n_dims: u32 },
    #[error("output column {col} out of range for n_dims {n_dims}")]
    OutputColumnOutOfRange { col: u32, n_dims: u32 },
    #[error("palma_d column {col} out of range for n_dims {n_dims}")]
    PalmaColumnOutOfRange { col: u32, n_dims: u32 },
    #[error("guyang_stall column {col} out of range for n_dims {n_dims}")]
    StallColumnOutOfRange { col: u32, n_dims: u32 },
    #[error("output columns must be unique and disjoint from emitter/palma/stall inputs")]
    OutputColumnCollision,
    #[error("band thresholds must be finite and non-negative")]
    InvalidBands,
    #[error(transparent)]
    FieldSweep(#[from] FieldSweepAdmissionError),
}

/// Independent CPU oracle for comparative law. Uses only public adjacency
/// metadata (`grid_shape` + `grid_offsets_data`) for neighbor expansion — no
/// kernel-private gather-list door.
pub fn comparative_projection_cpu_oracle(
    values: &[f32],
    slots: u32,
    n_dims: u32,
    emitters: &[ComparativeEmitterClass],
    outputs: ComparativeProjectionOutputs,
    palma_d_col: ColumnIndex,
    guyang_stall_col: ColumnIndex,
    bands: ComparativeProjectionBands,
    adjacency: &FieldAdjacency,
) -> Vec<f32> {
    let required = slots as usize * n_dims as usize;
    assert_eq!(values.len(), required, "oracle values length");
    assert!(!emitters.is_empty(), "oracle emitters");
    let mut out = values.to_vec();
    for slot in 0..slots {
        let base = slot as usize * n_dims as usize;
        let mut best_idx = 0usize;
        let mut best_val = read(values, base, emitters[0].value_col);
        let mut second = f32::NEG_INFINITY;
        for (idx, emitter) in emitters.iter().enumerate().skip(1) {
            let v = read(values, base, emitter.value_col);
            if v > best_val {
                second = best_val;
                best_val = v;
                best_idx = idx;
            } else if v > second {
                second = v;
            }
        }
        if !second.is_finite() {
            second = best_val;
        }
        // Exact top1−top2 (always ≥ 0). Sign-flip of this field is unreachable;
        // that is design residue (Remand 1 item 4), not silently redefined here.
        let margin = best_val - second;
        let dominance = emitters[best_idx].class_id;
        let both_strong = second >= bands.both_strong_floor;
        let small = margin <= bands.small_margin;
        // Contest = admitted Gu-Yang stall magnitude under both-strong/small-margin.
        let stall = read(values, base, guyang_stall_col);
        let contest = if both_strong && small { stall } else { 0.0 };
        write(&mut out, base, outputs.dominance_col, dominance);
        write(&mut out, base, outputs.margin_col, margin);
        write(&mut out, base, outputs.contest_col, contest);
    }
    // Border: sign-flip only (no near-zero proxy). With non-negative margin this
    // arm is expected empty until DA rules a signed comparative coordinate.
    for slot in 0..slots {
        let base = slot as usize * n_dims as usize;
        let target_margin = read(&out, base, outputs.margin_col);
        let mut border = 0.0f32;
        for neighbor in public_neighbors(adjacency, SlotIndex::new(slot)) {
            let n_base = neighbor.as_usize() * n_dims as usize;
            let neighbor_margin = read(&out, n_base, outputs.margin_col);
            if target_margin * neighbor_margin < 0.0 {
                border = 1.0;
            }
        }
        write(&mut out, base, outputs.border_col, border);
    }
    for slot in 0..slots {
        let base = slot as usize * n_dims as usize;
        let border = read(&out, base, outputs.border_col);
        let d = read(values, base, palma_d_col);
        let contested = border >= bands.contested_border_floor;
        let low_d = d.is_finite() && d <= bands.palma_low_d;
        let choke = if contested && low_d { 1.0 } else { 0.0 };
        write(&mut out, base, outputs.chokepoint_col, choke);
    }
    out
}

/// Expand neighbors using public grid metadata only (handoff-admitted consumer).
fn public_neighbors(adjacency: &FieldAdjacency, slot: SlotIndex) -> Vec<SlotIndex> {
    let Some((width, height)) = adjacency.grid_shape() else {
        return Vec::new();
    };
    let Some(offsets) = adjacency.grid_offsets_data() else {
        return Vec::new();
    };
    let x = slot.raw() % width;
    let y = slot.raw() / width;
    let mut out = Vec::new();
    for offset in offsets {
        let nx = x as i64 + i64::from(offset.dx());
        let ny = y as i64 + i64::from(offset.dy());
        if nx < 0 || ny < 0 || nx >= i64::from(width) || ny >= i64::from(height) {
            continue;
        }
        out.push(SlotIndex::new((ny as u32) * width + (nx as u32)));
    }
    let _ = height;
    out
}

fn read(values: &[f32], base: usize, col: ColumnIndex) -> f32 {
    values[base + col.raw()]
}

fn write(values: &mut [f32], base: usize, col: ColumnIndex, value: f32) {
    values[base + col.raw()] = value;
}

/// Compile comparative projections as ordinary field-sweep registrations over
/// already-admitted co-located columns. No new kernel authority.
pub fn admit_comparative_projections(
    request: ComparativeProjectionRequest,
) -> Result<ComparativeProjectionBundle, ComparativeProjectionError> {
    validate_request(&request)?;
    let emitter_count = request.emitters.len() as u32;
    if let Some(reason) = request.authored_opt_out_reason {
        return Ok(ComparativeProjectionBundle {
            disposition: ComparativeProjectionDisposition::AuthoredOptOut { reason },
            registrations: Vec::new(),
            outputs: request.outputs,
            emitter_count,
            derived_column_count: 0,
        });
    }
    if request.emitters.len() < 2 {
        return Ok(ComparativeProjectionBundle {
            disposition: ComparativeProjectionDisposition::InsufficientEmitters { emitter_count },
            registrations: Vec::new(),
            outputs: request.outputs,
            emitter_count,
            derived_column_count: 0,
        });
    }

    let registrations = compile_projection_chain(&request)?;
    Ok(ComparativeProjectionBundle {
        disposition: ComparativeProjectionDisposition::Born {
            emitter_count,
            derived_column_count: COMPARATIVE_DERIVED_COLUMN_COUNT,
        },
        registrations,
        outputs: request.outputs,
        emitter_count,
        derived_column_count: COMPARATIVE_DERIVED_COLUMN_COUNT,
    })
}

fn validate_request(
    request: &ComparativeProjectionRequest,
) -> Result<(), ComparativeProjectionError> {
    if request.emitters.is_empty() {
        return Err(ComparativeProjectionError::EmptyEmitters);
    }
    if !request.bands.both_strong_floor.is_finite()
        || request.bands.both_strong_floor < 0.0
        || !request.bands.small_margin.is_finite()
        || request.bands.small_margin < 0.0
        || !request.bands.palma_low_d.is_finite()
        || request.bands.palma_low_d < 0.0
        || !request.bands.contested_border_floor.is_finite()
        || request.bands.contested_border_floor < 0.0
    {
        return Err(ComparativeProjectionError::InvalidBands);
    }
    if request.palma_d_col.raw_u32() >= request.n_dims {
        return Err(ComparativeProjectionError::PalmaColumnOutOfRange {
            col: request.palma_d_col.raw_u32(),
            n_dims: request.n_dims,
        });
    }
    if request.guyang_stall_col.raw_u32() >= request.n_dims {
        return Err(ComparativeProjectionError::StallColumnOutOfRange {
            col: request.guyang_stall_col.raw_u32(),
            n_dims: request.n_dims,
        });
    }
    let outs = [
        request.outputs.dominance_col,
        request.outputs.margin_col,
        request.outputs.contest_col,
        request.outputs.border_col,
        request.outputs.chokepoint_col,
    ];
    let mut seen = BTreeSet::new();
    for col in outs {
        if col.raw_u32() >= request.n_dims {
            return Err(ComparativeProjectionError::OutputColumnOutOfRange {
                col: col.raw_u32(),
                n_dims: request.n_dims,
            });
        }
        if !seen.insert(col.raw_u32()) {
            return Err(ComparativeProjectionError::OutputColumnCollision);
        }
    }
    let mut emitter_cols = BTreeSet::new();
    for emitter in &request.emitters {
        if !emitter.class_id.is_finite() {
            return Err(ComparativeProjectionError::NonFiniteClassId(emitter.class_id));
        }
        if emitter.value_col.raw_u32() >= request.n_dims {
            return Err(ComparativeProjectionError::EmitterColumnOutOfRange {
                col: emitter.value_col.raw_u32(),
                n_dims: request.n_dims,
            });
        }
        if !emitter_cols.insert(emitter.value_col.raw_u32()) {
            return Err(ComparativeProjectionError::DuplicateEmitterColumn {
                col: emitter.value_col.raw_u32(),
            });
        }
        if seen.contains(&emitter.value_col.raw_u32())
            || emitter.value_col.raw_u32() == request.palma_d_col.raw_u32()
            || emitter.value_col.raw_u32() == request.guyang_stall_col.raw_u32()
        {
            return Err(ComparativeProjectionError::OutputColumnCollision);
        }
    }
    if seen.contains(&request.palma_d_col.raw_u32())
        || seen.contains(&request.guyang_stall_col.raw_u32())
        || request.palma_d_col.raw_u32() == request.guyang_stall_col.raw_u32()
    {
        return Err(ComparativeProjectionError::OutputColumnCollision);
    }
    Ok(())
}

fn compile_projection_chain(
    request: &ComparativeProjectionRequest,
) -> Result<Vec<FieldSweepRegistration>, ComparativeProjectionError> {
    let order = request.adjacency.apply_canonical_order_proof();
    let mut regs = Vec::new();

    let top1 = admit(
        request,
        order,
        FieldSweepOutput::Transient,
        0.0f32.to_bits(),
        unrolled_max_post(&request.emitters),
        None,
    )?;
    let top1_cert = top1.apply_transient_certificate()?;
    regs.push(top1);

    let last = request.emitters.len() - 1;
    regs.push(admit(
        request,
        order,
        FieldSweepOutput::Matrix(request.outputs.dominance_col),
        0.0f32.to_bits(),
        vec![literal(request.emitters[last].class_id), ret()],
        None,
    )?);
    for emitter in request.emitters.iter().take(last).rev() {
        regs.push(admit(
            request,
            order,
            FieldSweepOutput::Matrix(request.outputs.dominance_col),
            0.0f32.to_bits(),
            dominance_step_post(*emitter, request.outputs.dominance_col),
            Some(top1_cert),
        )?);
    }

    let second_init = f32::from_bits(0xff7fffff);
    regs.push(admit(
        request,
        order,
        FieldSweepOutput::Matrix(request.outputs.margin_col),
        second_init.to_bits(),
        vec![literal(second_init), ret()],
        None,
    )?);
    for emitter in &request.emitters {
        regs.push(admit(
            request,
            order,
            FieldSweepOutput::Matrix(request.outputs.margin_col),
            second_init.to_bits(),
            second_max_step_post(*emitter, request.outputs.margin_col, second_init),
            Some(top1_cert),
        )?);
    }

    regs.push(admit(
        request,
        order,
        FieldSweepOutput::Matrix(request.outputs.margin_col),
        0.0f32.to_bits(),
        margin_from_second_post(request.outputs.margin_col, second_init),
        Some(top1_cert),
    )?);

    // Contest from admitted stall column under both-strong/small-margin.
    regs.push(admit(
        request,
        order,
        FieldSweepOutput::Matrix(request.outputs.contest_col),
        0.0f32.to_bits(),
        contest_from_stall_post(
            request.outputs.margin_col,
            request.guyang_stall_col,
            &request.bands,
        ),
        Some(top1_cert),
    )?);

    // Border: sign-flip of margin only (no near-zero stand-in).
    regs.push(admit_border_sign_flip(
        request,
        order,
        request.outputs.margin_col,
        request.outputs.border_col,
    )?);

    regs.push(admit(
        request,
        order,
        FieldSweepOutput::Matrix(request.outputs.chokepoint_col),
        0.0f32.to_bits(),
        chokepoint_post(
            request.outputs.border_col,
            request.palma_d_col,
            &request.bands,
        ),
        None,
    )?);

    Ok(regs)
}

fn admit(
    request: &ComparativeProjectionRequest,
    order: simthing_gpu::CanonicalOrderProof,
    output: FieldSweepOutput,
    identity_bits: u32,
    post_program: Vec<EmlNodeGpu>,
    transient_read_proof: Option<FieldTransientCertificate>,
) -> Result<FieldSweepRegistration, ComparativeProjectionError> {
    Ok(apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency: request.adjacency.clone(),
        n_dims: request.n_dims,
        output,
        map_program: ignore_edge_map(),
        fold_program: keep_accumulator_fold(),
        identity_bits,
        post_program,
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        transient_read_proof,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })?)
}

fn admit_border_sign_flip(
    request: &ComparativeProjectionRequest,
    order: simthing_gpu::CanonicalOrderProof,
    margin_col: ColumnIndex,
    border_col: ColumnIndex,
) -> Result<FieldSweepRegistration, ComparativeProjectionError> {
    let map_program = vec![
        target(margin_col),
        neighbor(margin_col),
        binary(eml_opcode::MUL),
        literal(0.0),
        binary(eml_opcode::CMP_LT),
        ret(),
    ];
    let fold_program = vec![
        param(field_param::ACCUMULATOR),
        param(field_param::MAPPED),
        binary(eml_opcode::MAX),
        ret(),
    ];
    // Sign-flip fold only — no near-zero margin proxy.
    let post_program = vec![param(field_param::FOLDED), ret()];
    Ok(apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency: request.adjacency.clone(),
        n_dims: request.n_dims,
        output: FieldSweepOutput::Matrix(border_col),
        map_program,
        fold_program,
        identity_bits: 0.0f32.to_bits(),
        post_program,
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        transient_read_proof: None,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })?)
}

fn ignore_edge_map() -> Vec<EmlNodeGpu> {
    vec![literal(0.0), ret()]
}

fn keep_accumulator_fold() -> Vec<EmlNodeGpu> {
    vec![param(field_param::ACCUMULATOR), ret()]
}

fn unrolled_max_post(emitters: &[ComparativeEmitterClass]) -> Vec<EmlNodeGpu> {
    let mut nodes = vec![target(emitters[0].value_col)];
    for emitter in emitters.iter().skip(1) {
        nodes.push(target(emitter.value_col));
        nodes.push(binary(eml_opcode::MAX));
    }
    nodes.push(ret());
    nodes
}

fn dominance_step_post(
    emitter: ComparativeEmitterClass,
    dominance_col: ColumnIndex,
) -> Vec<EmlNodeGpu> {
    vec![
        target(emitter.value_col),
        param(field_param::TARGET_TRANSIENT),
        binary(eml_opcode::CMP_EQ),
        literal(emitter.class_id),
        target(dominance_col),
        select(),
        ret(),
    ]
}

fn second_max_step_post(
    emitter: ComparativeEmitterClass,
    second_col: ColumnIndex,
    second_init: f32,
) -> Vec<EmlNodeGpu> {
    vec![
        target(second_col),
        target(emitter.value_col),
        param(field_param::TARGET_TRANSIENT),
        binary(eml_opcode::CMP_LT),
        target(emitter.value_col),
        literal(second_init),
        select(),
        binary(eml_opcode::MAX),
        ret(),
    ]
}

fn margin_from_second_post(second_col: ColumnIndex, second_init: f32) -> Vec<EmlNodeGpu> {
    vec![
        target(second_col),
        literal(second_init),
        binary(eml_opcode::CMP_EQ),
        literal(0.0),
        param(field_param::TARGET_TRANSIENT),
        target(second_col),
        binary(eml_opcode::SUB),
        select(),
        ret(),
    ]
}

fn contest_from_stall_post(
    margin_col: ColumnIndex,
    stall_col: ColumnIndex,
    bands: &ComparativeProjectionBands,
) -> Vec<EmlNodeGpu> {
    // select((margin <= small) * (top1 - margin >= floor), stall, 0)
    vec![
        target(margin_col),
        literal(bands.small_margin),
        binary(eml_opcode::CMP_LE),
        param(field_param::TARGET_TRANSIENT),
        target(margin_col),
        binary(eml_opcode::SUB),
        literal(bands.both_strong_floor),
        binary(eml_opcode::CMP_GE),
        binary(eml_opcode::MUL),
        target(stall_col),
        literal(0.0),
        select(),
        ret(),
    ]
}

fn chokepoint_post(
    border_col: ColumnIndex,
    palma_d_col: ColumnIndex,
    bands: &ComparativeProjectionBands,
) -> Vec<EmlNodeGpu> {
    vec![
        target(border_col),
        literal(bands.contested_border_floor),
        binary(eml_opcode::CMP_GE),
        target(palma_d_col),
        literal(bands.palma_low_d),
        binary(eml_opcode::CMP_LE),
        binary(eml_opcode::MUL),
        literal(1.0),
        literal(0.0),
        select(),
        ret(),
    ]
}

fn node(opcode: u32, a: u32, b: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode,
        flags: 0,
        a,
        b,
        c: 0,
        d: 0,
    }
}

fn literal(value: f32) -> EmlNodeGpu {
    node(eml_opcode::LITERAL_F32, value.to_bits(), 0)
}

fn target(col: ColumnIndex) -> EmlNodeGpu {
    node(eml_opcode::TARGET_VALUE, encode_column(col), 0)
}

fn neighbor(col: ColumnIndex) -> EmlNodeGpu {
    node(eml_opcode::NEIGHBOR_VALUE, encode_column(col), 0)
}

fn param(index: u32) -> EmlNodeGpu {
    node(eml_opcode::PARAM, index, 0)
}

fn binary(opcode: u32) -> EmlNodeGpu {
    node(opcode, 0, 0)
}

fn select() -> EmlNodeGpu {
    node(eml_opcode::SELECT, 0, 0)
}

fn ret() -> EmlNodeGpu {
    node(eml_opcode::RETURN_TOP, 0, 0)
}
