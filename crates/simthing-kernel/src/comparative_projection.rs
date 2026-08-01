//! GUYANG-COMPARATIVE-PROJECTIONS-0 — sealed comparative projections over
//! co-located generic field-sweep outputs.
//!
//! Borders/fronts/chokepoints are derived observables, never services. Algebra
//! and owner-count stay out of the executor: admission unrolls the exact
//! emitter set into ordinary field-EML map/fold/post data over one generic
//! adjacency axis (`GridOffsets` or `LinkGraph`).

use std::collections::BTreeSet;

use simthing_core::{eml_opcode, ColumnIndex, EmlNodeGpu, SlotIndex};
use thiserror::Error;

use crate::field_sweep::{
    apply_field_sweep_registration, field_param, CanonicalOrderProof, FieldAdjacency, FieldLawProof,
    FieldSweepAdmissionError, FieldSweepOutput, FieldSweepRegistration,
    FieldSweepRegistrationRequest, FieldTransientCertificate,
};
use crate::wgsl_encode::encode_column;

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
    /// Both-strong floor on the runner-up value.
    pub both_strong_floor: f32,
    /// Small-margin ceiling that elevates contest pressure.
    pub small_margin: f32,
    /// PALMA low-`D` corridor threshold for chokepoint conjunction.
    pub palma_low_d: f32,
    /// Border magnitude above which a cell counts as contested-border.
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

/// Scenario-neutral comparative projection admission request.
#[derive(Clone, Debug)]
pub struct ComparativeProjectionRequest {
    pub adjacency: FieldAdjacency,
    pub n_dims: u32,
    pub emitters: Vec<ComparativeEmitterClass>,
    pub outputs: ComparativeProjectionOutputs,
    /// PALMA potential column consumed only by the chokepoint conjunction.
    pub palma_d_col: ColumnIndex,
    pub bands: ComparativeProjectionBands,
    /// Authored disposition opt-out reason. When set, no projections are born.
    pub authored_opt_out_reason: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComparativeProjectionDisposition {
    /// Fewer than two competing emitter classes — no fabricated comparison.
    InsufficientEmitters { emitter_count: u32 },
    /// Explicit authored opt-out with a visible reason (dark-cell posture).
    AuthoredOptOut { reason: &'static str },
    /// Projections born: fixed derived columns + registrations.
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
    #[error("output columns must be unique and disjoint from emitter/palma inputs")]
    OutputColumnCollision,
    #[error("band thresholds must be finite and non-negative")]
    InvalidBands,
    #[error(transparent)]
    FieldSweep(#[from] FieldSweepAdmissionError),
}

/// Independent CPU oracle for the sealed comparative law. Used as a parity
/// judge against the field-EML registration chain — never a production service.
pub fn comparative_projection_cpu_oracle(
    values: &[f32],
    slots: u32,
    n_dims: u32,
    emitters: &[ComparativeEmitterClass],
    outputs: ComparativeProjectionOutputs,
    palma_d_col: ColumnIndex,
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
        let margin = best_val - second;
        let dominance = emitters[best_idx].class_id;
        let both_strong = second >= bands.both_strong_floor;
        let small = margin <= bands.small_margin;
        let contest = if both_strong && small { second } else { 0.0 };
        write(&mut out, base, outputs.dominance_col, dominance);
        write(&mut out, base, outputs.margin_col, margin);
        write(&mut out, base, outputs.contest_col, contest);
    }
    for slot in 0..slots {
        let base = slot as usize * n_dims as usize;
        let target_margin = read(&out, base, outputs.margin_col);
        let mut border: f32 = if target_margin.abs() <= bands.small_margin {
            1.0
        } else {
            0.0
        };
        for input in adjacency.neighbor_inputs(SlotIndex::new(slot)) {
            let n_base = input.slot.as_usize() * n_dims as usize;
            let neighbor_margin = read(&out, n_base, outputs.margin_col);
            if target_margin * neighbor_margin < 0.0 {
                border = border.max(1.0);
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

fn read(values: &[f32], base: usize, col: ColumnIndex) -> f32 {
    values[base + col.raw()]
}

fn write(values: &mut [f32], base: usize, col: ColumnIndex, value: f32) {
    values[base + col.raw()] = value;
}

/// Admit the sealed comparative-projection authority.
///
/// - 0 emitters → error
/// - 1 emitter → `InsufficientEmitters` (no fabricated comparison)
/// - authored opt-out → `AuthoredOptOut` with visible reason
/// - ≥2 emitters → fixed derived columns + field-sweep registration chain
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
        {
            return Err(ComparativeProjectionError::OutputColumnCollision);
        }
    }
    if seen.contains(&request.palma_d_col.raw_u32()) {
        return Err(ComparativeProjectionError::OutputColumnCollision);
    }
    Ok(())
}

fn compile_projection_chain(
    request: &ComparativeProjectionRequest,
) -> Result<Vec<FieldSweepRegistration>, ComparativeProjectionError> {
    let order = request.adjacency.apply_canonical_order_proof();
    let mut regs = Vec::new();

    // 1) top1 → Transient (unrolled MAX over emitter TARGET_VALUE columns).
    let top1 = admit(
        request,
        order,
        FieldLawProof::apply_non_conservative(),
        FieldSweepOutput::Transient,
        0.0f32.to_bits(),
        unrolled_max_post(&request.emitters),
        None,
    )?;
    let top1_cert = top1.apply_transient_certificate()?;
    regs.push(top1);

    // 2) dominance class id — multi-pass reverse walk so node counts stay inside
    // the resource-class envelope for large emitter sets. Authored order wins
    // exact ties via first-equal-to-top1 (earliest overwrite on reverse walk).
    let last = request.emitters.len() - 1;
    regs.push(admit(
        request,
        order,
        FieldLawProof::apply_non_conservative(),
        FieldSweepOutput::Matrix(request.outputs.dominance_col),
        0.0f32.to_bits(),
        vec![literal(request.emitters[last].class_id), ret()],
        None,
    )?);
    for emitter in request.emitters.iter().take(last).rev() {
        regs.push(admit(
            request,
            order,
            FieldLawProof::apply_non_conservative(),
            FieldSweepOutput::Matrix(request.outputs.dominance_col),
            0.0f32.to_bits(),
            dominance_step_post(*emitter, request.outputs.dominance_col),
            Some(top1_cert),
        )?);
    }

    // 3) second value workspace in margin_col (vals strictly below top1).
    // Multi-pass keeps each program inside the resource-class node envelope.
    let second_init = f32::from_bits(0xff7fffff);
    regs.push(admit(
        request,
        order,
        FieldLawProof::apply_non_conservative(),
        FieldSweepOutput::Matrix(request.outputs.margin_col),
        second_init.to_bits(),
        vec![literal(second_init), ret()],
        None,
    )?);
    for emitter in &request.emitters {
        regs.push(admit(
            request,
            order,
            FieldLawProof::apply_non_conservative(),
            FieldSweepOutput::Matrix(request.outputs.margin_col),
            second_init.to_bits(),
            second_max_step_post(*emitter, request.outputs.margin_col, second_init),
            Some(top1_cert),
        )?);
    }

    // 4) margin = top1 - second (all-equal → 0).
    regs.push(admit(
        request,
        order,
        FieldLawProof::apply_non_conservative(),
        FieldSweepOutput::Matrix(request.outputs.margin_col),
        0.0f32.to_bits(),
        margin_from_second_post(request.outputs.margin_col, second_init),
        Some(top1_cert),
    )?);

    // 5) contest: both-strong at small margin → runner-up (= top1 - margin).
    regs.push(admit(
        request,
        order,
        FieldLawProof::apply_non_conservative(),
        FieldSweepOutput::Matrix(request.outputs.contest_col),
        0.0f32.to_bits(),
        contest_post(request.outputs.margin_col, &request.bands),
        Some(top1_cert),
    )?);

    // 6) border: adjacency sign-flip of margin OR near-zero margin band.
    regs.push(admit_border(
        request,
        order,
        FieldLawProof::apply_non_conservative(),
        request.outputs.margin_col,
        request.outputs.border_col,
        request.bands.small_margin,
    )?);

    // 7) chokepoint: contested-border ∧ PALMA-low-D.
    regs.push(admit(
        request,
        order,
        FieldLawProof::apply_non_conservative(),
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
    order: CanonicalOrderProof,
    law: FieldLawProof,
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
        field_law_proof: Some(law),
        transient_read_proof,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })?)
}

fn admit_border(
    request: &ComparativeProjectionRequest,
    order: CanonicalOrderProof,
    law: FieldLawProof,
    margin_col: ColumnIndex,
    border_col: ColumnIndex,
    small_margin: f32,
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
    let post_program = vec![
        target(margin_col),
        unary(eml_opcode::ABS),
        literal(small_margin),
        binary(eml_opcode::CMP_LE),
        literal(1.0),
        literal(0.0),
        select(),
        param(field_param::FOLDED),
        binary(eml_opcode::MAX),
        ret(),
    ];
    Ok(apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency: request.adjacency.clone(),
        n_dims: request.n_dims,
        output: FieldSweepOutput::Matrix(border_col),
        map_program,
        fold_program,
        identity_bits: 0.0f32.to_bits(),
        post_program,
        field_law_proof: Some(law),
        transient_read_proof: None,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })?)
}

// ── EML program builders (data only; no owner-count dispatch in the executor) ─

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

/// One reverse-walk step: `select(ci == top1, ei.id, current_dominance)`.
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

/// One second-max step: `max(running, select(ci < top1, ci, init))`.
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
    // select(second == init, 0, top1 - second)
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

fn contest_post(margin_col: ColumnIndex, bands: &ComparativeProjectionBands) -> Vec<EmlNodeGpu> {
    // contest = select((margin <= small) * (top1 - margin >= floor), top2, 0)
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
        param(field_param::TARGET_TRANSIENT),
        target(margin_col),
        binary(eml_opcode::SUB),
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

// ── node helpers ────────────────────────────────────────────────────────────

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

fn unary(opcode: u32) -> EmlNodeGpu {
    node(opcode, 0, 0)
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
