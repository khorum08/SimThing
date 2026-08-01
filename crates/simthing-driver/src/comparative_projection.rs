//! GUYANG-COMPARATIVE-PROJECTIONS-0 — comparative **consumer** over co-located
//! generic field-sweep outputs (driver-only; no kernel/GPU/allowlist doors).
//!
//! DA Remand 2 / ruling `5150877754`:
//! - margin = exact `top1 − top2` (non-negative magnitude)
//! - border = winner-identity change across canonical adjacency
//! - stall = gross_flux − |net_flux| from authored second Gu-Yang registration
//! - contest consumes admitted stall under both-strong/small-margin

use std::collections::BTreeSet;

use simthing_core::{
    eml_opcode, ColumnIndex, DimensionRegistry, EmlNodeGpu, PropertyAdmissionDisposition,
    SimProperty, SimPropertyId, SlotIndex,
};
use simthing_gpu::{
    apply_field_sweep_registration, encode_column, field_param, FieldAdjacency, FieldLawProof,
    FieldSweepAdmissionError, FieldSweepOutput, FieldSweepRegistration,
    FieldSweepRegistrationRequest, FieldTransientCertificate,
};
use thiserror::Error;

/// Comparative projection columns (independent of emitter/owner count).
pub const COMPARATIVE_DERIVED_COLUMN_COUNT: u32 = 5;

/// Gu-Yang stall-path columns born with comparative (net, gross, stall).
pub const GUYANG_STALL_DERIVED_COLUMN_COUNT: u32 = 3;

pub mod comparative_event_kind {
    pub const FRONT_FORMED: u32 = 0x4759_0001;
    pub const FRONT_HARDENED: u32 = 0x4759_0002;
    pub const CHOKEPOINT_EMERGED: u32 = 0x4759_0003;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComparativeEmitterClass {
    pub class_id: f32,
    pub value_col: ColumnIndex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComparativeProjectionOutputs {
    pub dominance_col: ColumnIndex,
    pub margin_col: ColumnIndex,
    pub contest_col: ColumnIndex,
    pub border_col: ColumnIndex,
    pub chokepoint_col: ColumnIndex,
}

/// Admitted Gu-Yang stall path columns (generic field-sweep outputs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuYangStallOutputs {
    pub net_flux_col: ColumnIndex,
    pub gross_flux_col: ColumnIndex,
    pub stall_col: ColumnIndex,
}

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

#[derive(Clone, Debug)]
pub struct ComparativeProjectionRequest {
    pub adjacency: FieldAdjacency,
    pub n_dims: u32,
    pub emitters: Vec<ComparativeEmitterClass>,
    pub outputs: ComparativeProjectionOutputs,
    pub palma_d_col: ColumnIndex,
    /// Gu-Yang value (u) and conductance (c) columns for net/gross/stall.
    pub guyang_value_col: ColumnIndex,
    pub guyang_conductance_col: ColumnIndex,
    pub stall_outputs: GuYangStallOutputs,
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
    /// Gu-Yang net/gross/stall then comparative chain (execute in order).
    pub registrations: Vec<FieldSweepRegistration>,
    pub outputs: ComparativeProjectionOutputs,
    pub stall_outputs: GuYangStallOutputs,
    pub emitter_count: u32,
    pub derived_column_count: u32,
}

/// Default-derived admission result: registry-minted Anchored properties + bundle.
#[derive(Clone, Debug)]
pub struct ComparativeProjectionAdmission {
    pub disposition: ComparativeProjectionDisposition,
    pub emitter_property_ids: Vec<SimPropertyId>,
    pub derived_property_ids: ComparativeDerivedPropertyIds,
    pub outputs: ComparativeProjectionOutputs,
    pub stall_outputs: GuYangStallOutputs,
    pub bundle: ComparativeProjectionBundle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComparativeDerivedPropertyIds {
    pub dominance: SimPropertyId,
    pub margin: SimPropertyId,
    pub contest: SimPropertyId,
    pub border: SimPropertyId,
    pub chokepoint: SimPropertyId,
    pub net_flux: SimPropertyId,
    pub gross_flux: SimPropertyId,
    pub stall: SimPropertyId,
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum ComparativeProjectionError {
    #[error("comparative projection requires at least one emitter class")]
    EmptyEmitters,
    #[error("duplicate emitter value column {col}")]
    DuplicateEmitterColumn { col: u32 },
    #[error("emitter class_id must be finite (got {0})")]
    NonFiniteClassId(f32),
    #[error("column {col} out of range for n_dims {n_dims}")]
    ColumnOutOfRange { col: u32, n_dims: u32 },
    #[error("output columns must be unique and disjoint from inputs")]
    OutputColumnCollision,
    #[error("band thresholds must be finite and non-negative")]
    InvalidBands,
    #[error("emitter property {0:?} is not registered")]
    UnknownEmitterProperty(SimPropertyId),
    #[error("emitter property {0:?} is not resource-bearing")]
    EmitterNotResourceBearing(SimPropertyId),
    #[error("palma_d property {0:?} is not registered")]
    UnknownPalmaProperty(SimPropertyId),
    #[error("guyang value/conductance property missing or not resource-bearing")]
    UnknownGuYangProperty,
    #[error(transparent)]
    FieldSweep(#[from] FieldSweepAdmissionError),
}

// ── Independent CPU oracle ──────────────────────────────────────────────────

pub fn comparative_projection_cpu_oracle(
    values: &[f32],
    slots: u32,
    n_dims: u32,
    emitters: &[ComparativeEmitterClass],
    outputs: ComparativeProjectionOutputs,
    palma_d_col: ColumnIndex,
    stall_col: ColumnIndex,
    bands: ComparativeProjectionBands,
    adjacency: &FieldAdjacency,
) -> Vec<f32> {
    let required = slots as usize * n_dims as usize;
    assert_eq!(values.len(), required);
    assert!(!emitters.is_empty());
    let mut out = values.to_vec();

    // Dominance + margin + contest (stall already on values or written later).
    for slot in 0..slots {
        let base = slot as usize * n_dims as usize;
        let (best_idx, best_val, second) = top_two(values, base, emitters);
        let margin = best_val - second;
        let dominance = emitters[best_idx].class_id;
        let both_strong = second >= bands.both_strong_floor;
        let small = margin <= bands.small_margin;
        let stall = read(values, base, stall_col);
        let contest = if both_strong && small { stall } else { 0.0 };
        write(&mut out, base, outputs.dominance_col, dominance);
        write(&mut out, base, outputs.margin_col, margin);
        write(&mut out, base, outputs.contest_col, contest);
    }

    // Border: winner-identity change across canonical adjacency.
    for slot in 0..slots {
        let base = slot as usize * n_dims as usize;
        let target_dom = read(&out, base, outputs.dominance_col);
        let mut border = 0.0f32;
        for neighbor in public_neighbors(adjacency, SlotIndex::new(slot)) {
            let n_base = neighbor.as_usize() * n_dims as usize;
            let neighbor_dom = read(&out, n_base, outputs.dominance_col);
            if target_dom != neighbor_dom {
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
        write(
            &mut out,
            base,
            outputs.chokepoint_col,
            if contested && low_d { 1.0 } else { 0.0 },
        );
    }
    out
}

fn top_two(
    values: &[f32],
    base: usize,
    emitters: &[ComparativeEmitterClass],
) -> (usize, f32, f32) {
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
    (best_idx, best_val, second)
}

/// Expand neighbors from public grid metadata (no kernel-private gather door).
pub fn public_neighbors(adjacency: &FieldAdjacency, slot: SlotIndex) -> Vec<SlotIndex> {
    let Some((width, _height)) = adjacency.grid_shape() else {
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
        if nx < 0 || ny < 0 || nx >= i64::from(width) || ny >= i64::from(_height) {
            continue;
        }
        out.push(SlotIndex::new((ny as u32) * width + (nx as u32)));
    }
    out
}

fn read(values: &[f32], base: usize, col: ColumnIndex) -> f32 {
    values[base + col.raw()]
}

fn write(values: &mut [f32], base: usize, col: ColumnIndex, value: f32) {
    values[base + col.raw()] = value;
}

// ── Admission ───────────────────────────────────────────────────────────────

pub fn admit_comparative_projections(
    request: ComparativeProjectionRequest,
) -> Result<ComparativeProjectionBundle, ComparativeProjectionError> {
    validate_request(&request)?;
    let emitter_count = request.emitters.len() as u32;
    if let Some(reason) = request.authored_opt_out_reason {
        return Ok(empty_bundle(
            ComparativeProjectionDisposition::AuthoredOptOut { reason },
            &request,
            emitter_count,
            0,
        ));
    }
    if request.emitters.len() < 2 {
        return Ok(empty_bundle(
            ComparativeProjectionDisposition::InsufficientEmitters { emitter_count },
            &request,
            emitter_count,
            0,
        ));
    }

    let mut registrations = compile_guyang_stall_chain(&request)?;
    registrations.extend(compile_comparative_chain(&request)?);
    Ok(ComparativeProjectionBundle {
        disposition: ComparativeProjectionDisposition::Born {
            emitter_count,
            derived_column_count: COMPARATIVE_DERIVED_COLUMN_COUNT,
        },
        registrations,
        outputs: request.outputs,
        stall_outputs: request.stall_outputs,
        emitter_count,
        derived_column_count: COMPARATIVE_DERIVED_COLUMN_COUNT,
    })
}

fn empty_bundle(
    disposition: ComparativeProjectionDisposition,
    request: &ComparativeProjectionRequest,
    emitter_count: u32,
    derived: u32,
) -> ComparativeProjectionBundle {
    ComparativeProjectionBundle {
        disposition,
        registrations: Vec::new(),
        outputs: request.outputs,
        stall_outputs: request.stall_outputs,
        emitter_count,
        derived_column_count: derived,
    }
}

/// Default-derived path: mint Anchored comparative + stall properties into the
/// registry when ≥2 competing emitter properties are admitted (Anchored).
/// Order of `emitter_property_ids` is the authored tie-break.
pub fn derive_comparative_projections_at_admission(
    registry: &mut DimensionRegistry,
    emitter_property_ids: &[SimPropertyId],
    adjacency: FieldAdjacency,
    palma_d_property: SimPropertyId,
    guyang_value_property: SimPropertyId,
    guyang_conductance_property: SimPropertyId,
    bands: ComparativeProjectionBands,
    authored_opt_out_reason: Option<&'static str>,
) -> Result<ComparativeProjectionAdmission, ComparativeProjectionError> {
    if emitter_property_ids.is_empty() {
        return Err(ComparativeProjectionError::EmptyEmitters);
    }

    let mut anchored_emitters = 0u32;
    for &pid in emitter_property_ids {
        let prop = registry
            .try_property(pid)
            .ok_or(ComparativeProjectionError::UnknownEmitterProperty(pid))?;
        if !prop.is_resource_bearing() {
            return Err(ComparativeProjectionError::EmitterNotResourceBearing(pid));
        }
        if prop.admission_disposition.is_anchored() {
            anchored_emitters += 1;
        }
    }

    if let Some(reason) = authored_opt_out_reason {
        let dummy_out = ComparativeProjectionOutputs {
            dominance_col: ColumnIndex::from_gpu_round_trip(0),
            margin_col: ColumnIndex::from_gpu_round_trip(0),
            contest_col: ColumnIndex::from_gpu_round_trip(0),
            border_col: ColumnIndex::from_gpu_round_trip(0),
            chokepoint_col: ColumnIndex::from_gpu_round_trip(0),
        };
        let dummy_stall = GuYangStallOutputs {
            net_flux_col: ColumnIndex::from_gpu_round_trip(0),
            gross_flux_col: ColumnIndex::from_gpu_round_trip(0),
            stall_col: ColumnIndex::from_gpu_round_trip(0),
        };
        return Ok(ComparativeProjectionAdmission {
            disposition: ComparativeProjectionDisposition::AuthoredOptOut { reason },
            emitter_property_ids: emitter_property_ids.to_vec(),
            derived_property_ids: ComparativeDerivedPropertyIds {
                dominance: SimPropertyId(0),
                margin: SimPropertyId(0),
                contest: SimPropertyId(0),
                border: SimPropertyId(0),
                chokepoint: SimPropertyId(0),
                net_flux: SimPropertyId(0),
                gross_flux: SimPropertyId(0),
                stall: SimPropertyId(0),
            },
            outputs: dummy_out,
            stall_outputs: dummy_stall,
            bundle: ComparativeProjectionBundle {
                disposition: ComparativeProjectionDisposition::AuthoredOptOut { reason },
                registrations: Vec::new(),
                outputs: dummy_out,
                stall_outputs: dummy_stall,
                emitter_count: emitter_property_ids.len() as u32,
                derived_column_count: 0,
            },
        });
    }

    if anchored_emitters < 2 {
        let dummy_out = ComparativeProjectionOutputs {
            dominance_col: ColumnIndex::from_gpu_round_trip(0),
            margin_col: ColumnIndex::from_gpu_round_trip(0),
            contest_col: ColumnIndex::from_gpu_round_trip(0),
            border_col: ColumnIndex::from_gpu_round_trip(0),
            chokepoint_col: ColumnIndex::from_gpu_round_trip(0),
        };
        let dummy_stall = GuYangStallOutputs {
            net_flux_col: ColumnIndex::from_gpu_round_trip(0),
            gross_flux_col: ColumnIndex::from_gpu_round_trip(0),
            stall_col: ColumnIndex::from_gpu_round_trip(0),
        };
        return Ok(ComparativeProjectionAdmission {
            disposition: ComparativeProjectionDisposition::InsufficientEmitters {
                emitter_count: anchored_emitters,
            },
            emitter_property_ids: emitter_property_ids.to_vec(),
            derived_property_ids: ComparativeDerivedPropertyIds {
                dominance: SimPropertyId(0),
                margin: SimPropertyId(0),
                contest: SimPropertyId(0),
                border: SimPropertyId(0),
                chokepoint: SimPropertyId(0),
                net_flux: SimPropertyId(0),
                gross_flux: SimPropertyId(0),
                stall: SimPropertyId(0),
            },
            outputs: dummy_out,
            stall_outputs: dummy_stall,
            bundle: ComparativeProjectionBundle {
                disposition: ComparativeProjectionDisposition::InsufficientEmitters {
                    emitter_count: anchored_emitters,
                },
                registrations: Vec::new(),
                outputs: dummy_out,
                stall_outputs: dummy_stall,
                emitter_count: anchored_emitters,
                derived_column_count: 0,
            },
        });
    }

    // Rebuild emitters with real columns from anchored emitters only, preserving
    // authored order among the full list (only Anchored count gates birth).
    let mut emitters = Vec::new();
    for (i, &pid) in emitter_property_ids.iter().enumerate() {
        let prop = registry.property(pid);
        if !prop.admission_disposition.is_anchored() {
            continue;
        }
        let range = registry.column_range(pid);
        emitters.push(ComparativeEmitterClass {
            class_id: (i as f32) + 1.0,
            value_col: ColumnIndex::from_gpu_round_trip(range.start as u32),
        });
    }

    let palma_prop = registry
        .try_property(palma_d_property)
        .ok_or(ComparativeProjectionError::UnknownPalmaProperty(palma_d_property))?;
    let palma_d_col =
        ColumnIndex::from_gpu_round_trip(registry.column_range(palma_d_property).start as u32);
    let _ = palma_prop;

    let guyang_v = registry
        .try_property(guyang_value_property)
        .ok_or(ComparativeProjectionError::UnknownGuYangProperty)?;
    let guyang_c = registry
        .try_property(guyang_conductance_property)
        .ok_or(ComparativeProjectionError::UnknownGuYangProperty)?;
    if !guyang_v.is_resource_bearing() || !guyang_c.is_resource_bearing() {
        return Err(ComparativeProjectionError::UnknownGuYangProperty);
    }
    let guyang_value_col =
        ColumnIndex::from_gpu_round_trip(registry.column_range(guyang_value_property).start as u32);
    let guyang_conductance_col = ColumnIndex::from_gpu_round_trip(
        registry.column_range(guyang_conductance_property).start as u32,
    );

    let derived_ids = mint_anchored_derived_properties(registry);
    let outputs = ComparativeProjectionOutputs {
        dominance_col: col_of(registry, derived_ids.dominance),
        margin_col: col_of(registry, derived_ids.margin),
        contest_col: col_of(registry, derived_ids.contest),
        border_col: col_of(registry, derived_ids.border),
        chokepoint_col: col_of(registry, derived_ids.chokepoint),
    };
    let stall_outputs = GuYangStallOutputs {
        net_flux_col: col_of(registry, derived_ids.net_flux),
        gross_flux_col: col_of(registry, derived_ids.gross_flux),
        stall_col: col_of(registry, derived_ids.stall),
    };

    let n_dims = registry.total_columns as u32;
    let request = ComparativeProjectionRequest {
        adjacency,
        n_dims,
        emitters,
        outputs,
        palma_d_col,
        guyang_value_col,
        guyang_conductance_col,
        stall_outputs,
        bands,
        authored_opt_out_reason: None,
    };
    let bundle = admit_comparative_projections(request)?;
    Ok(ComparativeProjectionAdmission {
        disposition: bundle.disposition.clone(),
        emitter_property_ids: emitter_property_ids.to_vec(),
        derived_property_ids: derived_ids,
        outputs,
        stall_outputs,
        bundle,
    })
}

fn col_of(registry: &DimensionRegistry, id: SimPropertyId) -> ColumnIndex {
    ColumnIndex::from_gpu_round_trip(registry.column_range(id).start as u32)
}

fn mint_anchored_derived_properties(registry: &mut DimensionRegistry) -> ComparativeDerivedPropertyIds {
    let mut mint = |ns: &str, name: &str| {
        let mut p = SimProperty::simple(ns, name, 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        p.description = format!("GUYANG-COMPARATIVE-PROJECTIONS-0 derived {name}");
        registry.register(p)
    };
    ComparativeDerivedPropertyIds {
        dominance: mint("comparative", "dominance"),
        margin: mint("comparative", "margin"),
        contest: mint("comparative", "contest"),
        border: mint("comparative", "border"),
        chokepoint: mint("comparative", "chokepoint"),
        net_flux: mint("guyang", "net_flux"),
        gross_flux: mint("guyang", "gross_flux"),
        stall: mint("guyang", "stall"),
    }
}

fn validate_request(
    request: &ComparativeProjectionRequest,
) -> Result<(), ComparativeProjectionError> {
    if request.emitters.is_empty() {
        return Err(ComparativeProjectionError::EmptyEmitters);
    }
    let b = &request.bands;
    if !b.both_strong_floor.is_finite()
        || b.both_strong_floor < 0.0
        || !b.small_margin.is_finite()
        || b.small_margin < 0.0
        || !b.palma_low_d.is_finite()
        || b.palma_low_d < 0.0
        || !b.contested_border_floor.is_finite()
        || b.contested_border_floor < 0.0
    {
        return Err(ComparativeProjectionError::InvalidBands);
    }
    let mut used = BTreeSet::new();
    let check = |col: ColumnIndex, n_dims: u32, used: &mut BTreeSet<u32>| {
        if col.raw_u32() >= n_dims {
            return Err(ComparativeProjectionError::ColumnOutOfRange {
                col: col.raw_u32(),
                n_dims,
            });
        }
        if !used.insert(col.raw_u32()) {
            return Err(ComparativeProjectionError::OutputColumnCollision);
        }
        Ok(())
    };
    for e in &request.emitters {
        if !e.class_id.is_finite() {
            return Err(ComparativeProjectionError::NonFiniteClassId(e.class_id));
        }
        check(e.value_col, request.n_dims, &mut used)?;
    }
    check(request.palma_d_col, request.n_dims, &mut used)?;
    check(request.guyang_value_col, request.n_dims, &mut used)?;
    check(request.guyang_conductance_col, request.n_dims, &mut used)?;
    check(request.stall_outputs.net_flux_col, request.n_dims, &mut used)?;
    check(request.stall_outputs.gross_flux_col, request.n_dims, &mut used)?;
    check(request.stall_outputs.stall_col, request.n_dims, &mut used)?;
    check(request.outputs.dominance_col, request.n_dims, &mut used)?;
    check(request.outputs.margin_col, request.n_dims, &mut used)?;
    check(request.outputs.contest_col, request.n_dims, &mut used)?;
    check(request.outputs.border_col, request.n_dims, &mut used)?;
    check(request.outputs.chokepoint_col, request.n_dims, &mut used)?;
    Ok(())
}

// ── Gu-Yang stall chain (authored data over generic field sweep) ────────────

/// Net: map signed flux, fold (+,0), write folded.
/// Gross: map abs(signed flux), fold (+,0), write folded.
/// Stall: gross − |net| cell-local.
fn compile_guyang_stall_chain(
    request: &ComparativeProjectionRequest,
) -> Result<Vec<FieldSweepRegistration>, ComparativeProjectionError> {
    let order = request.adjacency.apply_canonical_order_proof();
    let u = request.guyang_value_col;
    let c = request.guyang_conductance_col;
    let mut regs = Vec::new();

    // Signed edge flux: ((c_i + c_j)/2) * (u_j - u_i)
    let signed_flux_map = vec![
        target(c),
        neighbor(c),
        binary(eml_opcode::ADD),
        literal(0.5),
        binary(eml_opcode::MUL),
        neighbor(u),
        target(u),
        binary(eml_opcode::SUB),
        binary(eml_opcode::MUL),
        ret(),
    ];
    let add_fold = vec![
        param(field_param::ACCUMULATOR),
        param(field_param::MAPPED),
        binary(eml_opcode::ADD),
        ret(),
    ];

    regs.push(admit_reg(
        request,
        order,
        FieldSweepOutput::Matrix(request.stall_outputs.net_flux_col),
        0.0f32.to_bits(),
        signed_flux_map.clone(),
        add_fold.clone(),
        vec![param(field_param::FOLDED), ret()],
        None,
    )?);

    // Gross: abs(signed flux)
    let mut gross_map = signed_flux_map;
    // insert ABS before RETURN: replace ret with ABS, ret
    gross_map.pop(); // remove ret
    gross_map.push(unary(eml_opcode::ABS));
    gross_map.push(ret());
    regs.push(admit_reg(
        request,
        order,
        FieldSweepOutput::Matrix(request.stall_outputs.gross_flux_col),
        0.0f32.to_bits(),
        gross_map,
        add_fold,
        vec![param(field_param::FOLDED), ret()],
        None,
    )?);

    // stall = gross - abs(net)
    regs.push(admit_reg(
        request,
        order,
        FieldSweepOutput::Matrix(request.stall_outputs.stall_col),
        0.0f32.to_bits(),
        ignore_edge_map(),
        keep_accumulator_fold(),
        vec![
            target(request.stall_outputs.gross_flux_col),
            target(request.stall_outputs.net_flux_col),
            unary(eml_opcode::ABS),
            binary(eml_opcode::SUB),
            ret(),
        ],
        None,
    )?);

    Ok(regs)
}

fn compile_comparative_chain(
    request: &ComparativeProjectionRequest,
) -> Result<Vec<FieldSweepRegistration>, ComparativeProjectionError> {
    let order = request.adjacency.apply_canonical_order_proof();
    let mut regs = Vec::new();

    // top1 → Transient
    let top1 = admit_reg(
        request,
        order,
        FieldSweepOutput::Transient,
        0.0f32.to_bits(),
        ignore_edge_map(),
        keep_accumulator_fold(),
        unrolled_max_post(&request.emitters),
        None,
    )?;
    let top1_cert = top1.apply_transient_certificate()?;
    regs.push(top1);

    // Dominance via reverse first-equal-to-top1 (authored order wins ties).
    let last = request.emitters.len() - 1;
    regs.push(admit_reg(
        request,
        order,
        FieldSweepOutput::Matrix(request.outputs.dominance_col),
        0.0f32.to_bits(),
        ignore_edge_map(),
        keep_accumulator_fold(),
        vec![literal(request.emitters[last].class_id), ret()],
        None,
    )?);
    for emitter in request.emitters.iter().take(last).rev() {
        regs.push(admit_reg(
            request,
            order,
            FieldSweepOutput::Matrix(request.outputs.dominance_col),
            0.0f32.to_bits(),
            ignore_edge_map(),
            keep_accumulator_fold(),
            dominance_step_post(*emitter, request.outputs.dominance_col),
            Some(top1_cert),
        )?);
    }

    // Second max workspace → margin = top1 - second
    let second_init = f32::from_bits(0xff7fffff);
    regs.push(admit_reg(
        request,
        order,
        FieldSweepOutput::Matrix(request.outputs.margin_col),
        second_init.to_bits(),
        ignore_edge_map(),
        keep_accumulator_fold(),
        vec![literal(second_init), ret()],
        None,
    )?);
    for emitter in &request.emitters {
        regs.push(admit_reg(
            request,
            order,
            FieldSweepOutput::Matrix(request.outputs.margin_col),
            second_init.to_bits(),
            ignore_edge_map(),
            keep_accumulator_fold(),
            second_max_step_post(*emitter, request.outputs.margin_col, second_init),
            Some(top1_cert),
        )?);
    }
    regs.push(admit_reg(
        request,
        order,
        FieldSweepOutput::Matrix(request.outputs.margin_col),
        0.0f32.to_bits(),
        ignore_edge_map(),
        keep_accumulator_fold(),
        margin_from_second_post(request.outputs.margin_col, second_init),
        Some(top1_cert),
    )?);

    // Contest from admitted stall under both-strong/small-margin
    regs.push(admit_reg(
        request,
        order,
        FieldSweepOutput::Matrix(request.outputs.contest_col),
        0.0f32.to_bits(),
        ignore_edge_map(),
        keep_accumulator_fold(),
        contest_from_stall_post(
            request.outputs.margin_col,
            request.stall_outputs.stall_col,
            &request.bands,
        ),
        Some(top1_cert),
    )?);

    // Border: winner-identity change (dominance_target != dominance_neighbor)
    regs.push(admit_border_winner_change(
        request,
        order,
        request.outputs.dominance_col,
        request.outputs.border_col,
    )?);

    // Chokepoint: contested-border ∧ PALMA-low-D
    regs.push(admit_reg(
        request,
        order,
        FieldSweepOutput::Matrix(request.outputs.chokepoint_col),
        0.0f32.to_bits(),
        ignore_edge_map(),
        keep_accumulator_fold(),
        chokepoint_post(
            request.outputs.border_col,
            request.palma_d_col,
            &request.bands,
        ),
        None,
    )?);

    Ok(regs)
}

fn admit_border_winner_change(
    request: &ComparativeProjectionRequest,
    order: simthing_gpu::CanonicalOrderProof,
    dominance_col: ColumnIndex,
    border_col: ColumnIndex,
) -> Result<FieldSweepRegistration, ComparativeProjectionError> {
    // map: 1 if dominance differs
    let map_program = vec![
        literal(1.0),
        target(dominance_col),
        neighbor(dominance_col),
        binary(eml_opcode::CMP_EQ),
        binary(eml_opcode::SUB),
        ret(),
    ];
    let fold_program = vec![
        param(field_param::ACCUMULATOR),
        param(field_param::MAPPED),
        binary(eml_opcode::MAX),
        ret(),
    ];
    admit_reg(
        request,
        order,
        FieldSweepOutput::Matrix(border_col),
        0.0f32.to_bits(),
        map_program,
        fold_program,
        vec![param(field_param::FOLDED), ret()],
        None,
    )
}

fn admit_reg(
    request: &ComparativeProjectionRequest,
    order: simthing_gpu::CanonicalOrderProof,
    output: FieldSweepOutput,
    identity_bits: u32,
    map_program: Vec<EmlNodeGpu>,
    fold_program: Vec<EmlNodeGpu>,
    post_program: Vec<EmlNodeGpu>,
    transient_read_proof: Option<FieldTransientCertificate>,
) -> Result<FieldSweepRegistration, ComparativeProjectionError> {
    Ok(apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency: request.adjacency.clone(),
        n_dims: request.n_dims,
        output,
        map_program,
        fold_program,
        identity_bits,
        post_program,
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        transient_read_proof,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })?)
}

// ── EML helpers ─────────────────────────────────────────────────────────────

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
