//! GUYANG-COMPARATIVE-PROJECTIONS-0 — comparative **consumer** over co-located
//! generic field-sweep outputs (driver-only; no kernel/GPU/allowlist doors).
//!
//! Owner correction 3A (`5150987561`): **no TP proof, fixture, or coupling.**
//!
//! DA law (`5150877754`):
//! - margin = exact `top1 − top2` (non-negative)
//! - border = winner-identity change across canonical adjacency
//! - stall = gross − |net| from authored second Gu-Yang registration
//! - contest consumes stall under both-strong/small-margin
//!
//! Handoff column shape: **2–3 comparative columns** (dominance, margin, contest).
//! Border and chokepoint are **band/readout** properties for thresholds, not
//! comparative census columns.

use std::collections::BTreeSet;

use simthing_core::{
    eml_opcode, ColumnIndex, DimensionRegistry, EmlNodeGpu, PropertyAdmissionDisposition,
    SimProperty, SimPropertyId, SlotIndex,
};
use simthing_gpu::{
    apply_field_sweep_registration, encode_column, field_param, FieldAdjacency, FieldLawProof,
    FieldSweepAdmissionError, FieldSweepOutput, FieldSweepRegistration,
    FieldSweepRegistrationRequest, FieldTransientCertificate, LinkGraphNeighbor,
};
use thiserror::Error;

/// Comparative output columns (dominance, margin, contest) — owner-count independent.
pub const COMPARATIVE_DERIVED_COLUMN_COUNT: u32 = 3;

/// Band/readout properties (border, chokepoint) — threshold surface, not comparative census.
pub const BAND_READOUT_COLUMN_COUNT: u32 = 2;

/// Gu-Yang stall-path columns (net, gross, stall).
pub const GUYANG_STALL_DERIVED_COLUMN_COUNT: u32 = 3;

/// Reserved property namespace for competing emitter classes (admission data, not scenario grammar).
pub const COMPARATIVE_EMITTER_NAMESPACE: &str = "comparative_emitter";

/// Reserved triad property names for automatic birth.
pub const TRIAD_PALMA_D: &str = "palma_d";
pub const TRIAD_GUYANG_U: &str = "guyang_u";
pub const TRIAD_GUYANG_C: &str = "guyang_c";
pub const TRIAD_NAMESPACE: &str = "triad";

pub mod comparative_event_kind {
    pub const FRONT_FORMED: u32 = 0x4759_0001;
    pub const FRONT_HARDENED: u32 = 0x4759_0002;
    pub const CHOKEPOINT_EMERGED: u32 = 0x4759_0003;
}

/// One competing emitter class.
///
/// `authored_order` is the durable tie-break key (lower wins exact ties). It is
/// **not** registration/vector iteration order — reverse the input vec and the
/// winner must stay the same when authored_order is unchanged.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComparativeEmitterClass {
    pub authored_order: u32,
    /// Identity written into the dominance column (typically derived from authored name).
    pub class_id: f32,
    pub value_col: ColumnIndex,
}

/// The 2–3 comparative columns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComparativeProjectionOutputs {
    pub dominance_col: ColumnIndex,
    pub margin_col: ColumnIndex,
    pub contest_col: ColumnIndex,
}

/// Band/readout columns for ordinary threshold events (not comparative census).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComparativeBandReadouts {
    pub border_col: ColumnIndex,
    pub chokepoint_col: ColumnIndex,
}

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
    /// Contest magnitude that hardens a front (FRONT_HARDENED band edge).
    pub front_harden_contest: f32,
}

impl Default for ComparativeProjectionBands {
    fn default() -> Self {
        Self {
            both_strong_floor: 0.25,
            small_margin: 0.15,
            palma_low_d: 4.0,
            contested_border_floor: 0.5,
            front_harden_contest: 0.1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ComparativeProjectionRequest {
    pub adjacency: FieldAdjacency,
    /// Sealed neighbor rows for oracle + border (grid or LinkGraph). Captured at
    /// construction from public grid offsets or link rows — not a kernel door.
    pub neighbor_slots: Vec<Vec<SlotIndex>>,
    pub n_dims: u32,
    pub emitters: Vec<ComparativeEmitterClass>,
    pub outputs: ComparativeProjectionOutputs,
    pub band_readouts: ComparativeBandReadouts,
    pub palma_d_col: ColumnIndex,
    pub guyang_value_col: ColumnIndex,
    pub guyang_conductance_col: ColumnIndex,
    pub stall_outputs: GuYangStallOutputs,
    pub bands: ComparativeProjectionBands,
    pub authored_opt_out_reason: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComparativeProjectionDisposition {
    InsufficientEmitters { emitter_count: u32 },
    AuthoredOptOut { reason: String },
    Born {
        emitter_count: u32,
        comparative_column_count: u32,
    },
}

#[derive(Clone, Debug)]
pub struct ComparativeProjectionBundle {
    pub disposition: ComparativeProjectionDisposition,
    pub registrations: Vec<FieldSweepRegistration>,
    pub outputs: ComparativeProjectionOutputs,
    pub band_readouts: ComparativeBandReadouts,
    pub stall_outputs: GuYangStallOutputs,
    pub emitter_count: u32,
    /// Always 2–3; census of comparative columns only.
    pub comparative_column_count: u32,
}

#[derive(Clone, Debug)]
pub struct ComparativeProjectionAdmission {
    pub disposition: ComparativeProjectionDisposition,
    pub emitter_property_ids: Vec<SimPropertyId>,
    pub derived_property_ids: ComparativeDerivedPropertyIds,
    pub outputs: ComparativeProjectionOutputs,
    pub band_readouts: ComparativeBandReadouts,
    pub stall_outputs: GuYangStallOutputs,
    pub bundle: ComparativeProjectionBundle,
    /// Ordinary threshold plan for front-formed / front-hardened / chokepoint-emerged.
    pub threshold_plan: ComparativeThresholdPlan,
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

/// Production-path threshold bands (no new listener framework).
#[derive(Clone, Debug, PartialEq)]
pub struct ComparativeThresholdPlan {
    pub front_formed: (ColumnIndex, f32, u32),
    pub front_hardened: (ColumnIndex, f32, u32),
    pub chokepoint_emerged: (ColumnIndex, f32, u32),
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
    #[error("neighbor_slots length {actual} != adjacency slots {expected}")]
    NeighborSlotsMismatch { actual: usize, expected: u32 },
    #[error("missing triad property {namespace}::{name}")]
    MissingTriadProperty { namespace: String, name: String },
    #[error(transparent)]
    FieldSweep(#[from] FieldSweepAdmissionError),
}

// ── Neighbor sealing (topology-generic, no kernel private lists) ────────────

/// Build sealed neighbor rows from public grid metadata.
pub fn neighbor_slots_from_grid(adjacency: &FieldAdjacency) -> Option<Vec<Vec<SlotIndex>>> {
    let (width, height) = adjacency.grid_shape()?;
    let offsets = adjacency.grid_offsets_data()?;
    let slots = adjacency.slots();
    let mut out = vec![Vec::new(); slots as usize];
    for slot in 0..slots {
        let x = slot % width;
        let y = slot / width;
        for offset in offsets {
            let nx = x as i64 + i64::from(offset.dx());
            let ny = y as i64 + i64::from(offset.dy());
            if nx < 0 || ny < 0 || nx >= i64::from(width) || ny >= i64::from(height) {
                continue;
            }
            out[slot as usize].push(SlotIndex::new((ny as u32) * width + (nx as u32)));
        }
    }
    Some(out)
}

/// Build sealed neighbor rows from LinkGraph construction data (driver already has rows).
pub fn neighbor_slots_from_link_rows(rows: &[Vec<LinkGraphNeighbor>]) -> Vec<Vec<SlotIndex>> {
    rows.iter()
        .map(|row| row.iter().map(|n| n.slot).collect())
        .collect()
}

// ── CPU oracle ──────────────────────────────────────────────────────────────

pub fn comparative_projection_cpu_oracle(
    values: &[f32],
    slots: u32,
    n_dims: u32,
    emitters: &[ComparativeEmitterClass],
    outputs: ComparativeProjectionOutputs,
    band_readouts: ComparativeBandReadouts,
    palma_d_col: ColumnIndex,
    stall_col: ColumnIndex,
    bands: ComparativeProjectionBands,
    neighbor_slots: &[Vec<SlotIndex>],
) -> Vec<f32> {
    assert_eq!(values.len(), slots as usize * n_dims as usize);
    assert_eq!(neighbor_slots.len(), slots as usize);
    assert!(!emitters.is_empty());
    let mut out = values.to_vec();

    for slot in 0..slots {
        let base = slot as usize * n_dims as usize;
        let (winner, best_val, second) = top_two_authored(values, base, emitters);
        let margin = best_val - second;
        let both_strong = second >= bands.both_strong_floor;
        let small = margin <= bands.small_margin;
        let stall = read(values, base, stall_col);
        let contest = if both_strong && small { stall } else { 0.0 };
        write(&mut out, base, outputs.dominance_col, winner);
        write(&mut out, base, outputs.margin_col, margin);
        write(&mut out, base, outputs.contest_col, contest);
    }

    for slot in 0..slots {
        let base = slot as usize * n_dims as usize;
        let target_dom = read(&out, base, outputs.dominance_col);
        let mut border = 0.0f32;
        for &neighbor in &neighbor_slots[slot as usize] {
            let n_base = neighbor.as_usize() * n_dims as usize;
            if target_dom != read(&out, n_base, outputs.dominance_col) {
                border = 1.0;
            }
        }
        write(&mut out, base, band_readouts.border_col, border);
    }

    for slot in 0..slots {
        let base = slot as usize * n_dims as usize;
        let border = read(&out, base, band_readouts.border_col);
        let d = read(values, base, palma_d_col);
        let contested = border >= bands.contested_border_floor;
        let low_d = d.is_finite() && d <= bands.palma_low_d;
        write(
            &mut out,
            base,
            band_readouts.chokepoint_col,
            if contested && low_d { 1.0 } else { 0.0 },
        );
    }
    out
}

/// Argmax with durable authored_order tie-break (lower order wins).
fn top_two_authored(
    values: &[f32],
    base: usize,
    emitters: &[ComparativeEmitterClass],
) -> (f32, f32, f32) {
    // Sort by authored_order for stable evaluation independent of vec iteration.
    let mut order: Vec<usize> = (0..emitters.len()).collect();
    order.sort_by_key(|&i| emitters[i].authored_order);

    let first = order[0];
    let mut best_i = first;
    let mut best_val = read(values, base, emitters[first].value_col);
    let mut second = f32::NEG_INFINITY;
    for &i in order.iter().skip(1) {
        let v = read(values, base, emitters[i].value_col);
        if v > best_val {
            second = best_val;
            best_val = v;
            best_i = i;
        } else if v > second {
            second = v;
        }
    }
    if !second.is_finite() {
        second = best_val;
    }
    (emitters[best_i].class_id, best_val, second)
}

fn read(values: &[f32], base: usize, col: ColumnIndex) -> f32 {
    values[base + col.raw()]
}

fn write(values: &mut [f32], base: usize, col: ColumnIndex, value: f32) {
    values[base + col.raw()] = value;
}

// ── Production admission door ───────────────────────────────────────────────

/// **Sole production birth door.** Discovers Anchored `comparative_emitter::*`
/// properties (name order = authored_order), mints comparative/band/stall
/// properties, and compiles the field-sweep chain.
///
/// Call sites: install completion and field-plan binding. Tests must use this
/// door (or install that invokes it), not assemble raw requests by hand for
/// "default-derived" claims.
pub fn admit_default_comparative_projections(
    registry: &mut DimensionRegistry,
    adjacency: FieldAdjacency,
    neighbor_slots: Vec<Vec<SlotIndex>>,
    bands: ComparativeProjectionBands,
) -> Result<ComparativeProjectionAdmission, ComparativeProjectionError> {
    if neighbor_slots.len() as u32 != adjacency.slots() {
        return Err(ComparativeProjectionError::NeighborSlotsMismatch {
            actual: neighbor_slots.len(),
            expected: adjacency.slots(),
        });
    }

    // Discover emitters: namespace comparative_emitter, Anchored, sorted by name.
    let mut emitter_props: Vec<(String, SimPropertyId)> = registry
        .properties
        .iter()
        .enumerate()
        .filter(|(_, p)| {
            p.namespace == COMPARATIVE_EMITTER_NAMESPACE
                && p.is_resource_bearing()
                && p.admission_disposition.is_anchored()
        })
        .map(|(i, p)| (p.name.clone(), SimPropertyId(i as u32)))
        .collect();
    emitter_props.sort_by(|a, b| a.0.cmp(&b.0));

    // Opt-out: any comparative_emitter property that is Unobserved.
    for (i, p) in registry.properties.iter().enumerate() {
        if p.namespace == COMPARATIVE_EMITTER_NAMESPACE {
            if let PropertyAdmissionDisposition::Unobserved { reason, .. } =
                &p.admission_disposition
            {
                return Ok(opt_out_admission(reason.clone(), emitter_props.len() as u32));
            }
            let _ = i;
        }
    }

    if emitter_props.len() < 2 {
        return Ok(insufficient_admission(emitter_props.len() as u32));
    }

    let emitters: Vec<ComparativeEmitterClass> = emitter_props
        .iter()
        .enumerate()
        .map(|(order, (name, pid))| {
            let start = registry.column_range(*pid).start as u32;
            ComparativeEmitterClass {
                authored_order: order as u32,
                class_id: durable_class_id(name),
                value_col: ColumnIndex::from_gpu_round_trip(start),
            }
        })
        .collect();

    let palma_id = registry
        .id_of(TRIAD_NAMESPACE, TRIAD_PALMA_D)
        .ok_or_else(|| ComparativeProjectionError::MissingTriadProperty {
            namespace: TRIAD_NAMESPACE.into(),
            name: TRIAD_PALMA_D.into(),
        })?;
    let guyang_u = registry
        .id_of(TRIAD_NAMESPACE, TRIAD_GUYANG_U)
        .ok_or_else(|| ComparativeProjectionError::MissingTriadProperty {
            namespace: TRIAD_NAMESPACE.into(),
            name: TRIAD_GUYANG_U.into(),
        })?;
    let guyang_c = registry
        .id_of(TRIAD_NAMESPACE, TRIAD_GUYANG_C)
        .ok_or_else(|| ComparativeProjectionError::MissingTriadProperty {
            namespace: TRIAD_NAMESPACE.into(),
            name: TRIAD_GUYANG_C.into(),
        })?;

    let derived_ids = mint_derived_properties(registry);
    let outputs = ComparativeProjectionOutputs {
        dominance_col: col_of(registry, derived_ids.dominance),
        margin_col: col_of(registry, derived_ids.margin),
        contest_col: col_of(registry, derived_ids.contest),
    };
    let band_readouts = ComparativeBandReadouts {
        border_col: col_of(registry, derived_ids.border),
        chokepoint_col: col_of(registry, derived_ids.chokepoint),
    };
    let stall_outputs = GuYangStallOutputs {
        net_flux_col: col_of(registry, derived_ids.net_flux),
        gross_flux_col: col_of(registry, derived_ids.gross_flux),
        stall_col: col_of(registry, derived_ids.stall),
    };

    let request = ComparativeProjectionRequest {
        adjacency,
        neighbor_slots,
        n_dims: registry.total_columns as u32,
        emitters,
        outputs,
        band_readouts,
        palma_d_col: col_of(registry, palma_id),
        guyang_value_col: col_of(registry, guyang_u),
        guyang_conductance_col: col_of(registry, guyang_c),
        stall_outputs,
        bands,
        authored_opt_out_reason: None,
    };

    let bundle = compile_comparative_bundle(request)?;
    let threshold_plan = ComparativeThresholdPlan {
        front_formed: (
            band_readouts.border_col,
            bands.contested_border_floor,
            comparative_event_kind::FRONT_FORMED,
        ),
        front_hardened: (
            outputs.contest_col,
            bands.front_harden_contest,
            comparative_event_kind::FRONT_HARDENED,
        ),
        chokepoint_emerged: (
            band_readouts.chokepoint_col,
            0.5,
            comparative_event_kind::CHOKEPOINT_EMERGED,
        ),
    };

    Ok(ComparativeProjectionAdmission {
        disposition: bundle.disposition.clone(),
        emitter_property_ids: emitter_props.iter().map(|(_, id)| *id).collect(),
        derived_property_ids: derived_ids,
        outputs,
        band_readouts,
        stall_outputs,
        bundle,
        threshold_plan,
    })
}

fn durable_class_id(name: &str) -> f32 {
    // Stable non-zero f32 from name bytes (not registration index).
    let mut h: u32 = 2166136261;
    for b in name.as_bytes() {
        h ^= u32::from(*b);
        h = h.wrapping_mul(16777619);
    }
    // Map to a positive finite identity in a quiet float range.
    (h % 1_000_000) as f32 + 1.0
}

fn col_of(registry: &DimensionRegistry, id: SimPropertyId) -> ColumnIndex {
    ColumnIndex::from_gpu_round_trip(registry.column_range(id).start as u32)
}

fn mint_derived_properties(registry: &mut DimensionRegistry) -> ComparativeDerivedPropertyIds {
    let mut mint = |ns: &str, name: &str| {
        let mut p = SimProperty::simple(ns, name, 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        p.description = format!("GUYANG-COMPARATIVE-PROJECTIONS-0 {ns}::{name}");
        registry.register(p)
    };
    ComparativeDerivedPropertyIds {
        dominance: mint("comparative", "dominance"),
        margin: mint("comparative", "margin"),
        contest: mint("comparative", "contest"),
        border: mint("comparative_band", "border"),
        chokepoint: mint("comparative_band", "chokepoint"),
        net_flux: mint("guyang", "net_flux"),
        gross_flux: mint("guyang", "gross_flux"),
        stall: mint("guyang", "stall"),
    }
}

fn insufficient_admission(emitter_count: u32) -> ComparativeProjectionAdmission {
    let dummy_out = ComparativeProjectionOutputs {
        dominance_col: ColumnIndex::from_gpu_round_trip(0),
        margin_col: ColumnIndex::from_gpu_round_trip(0),
        contest_col: ColumnIndex::from_gpu_round_trip(0),
    };
    let dummy_band = ComparativeBandReadouts {
        border_col: ColumnIndex::from_gpu_round_trip(0),
        chokepoint_col: ColumnIndex::from_gpu_round_trip(0),
    };
    let dummy_stall = GuYangStallOutputs {
        net_flux_col: ColumnIndex::from_gpu_round_trip(0),
        gross_flux_col: ColumnIndex::from_gpu_round_trip(0),
        stall_col: ColumnIndex::from_gpu_round_trip(0),
    };
    let disp = ComparativeProjectionDisposition::InsufficientEmitters { emitter_count };
    ComparativeProjectionAdmission {
        disposition: disp.clone(),
        emitter_property_ids: Vec::new(),
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
        band_readouts: dummy_band,
        stall_outputs: dummy_stall,
        bundle: ComparativeProjectionBundle {
            disposition: disp,
            registrations: Vec::new(),
            outputs: dummy_out,
            band_readouts: dummy_band,
            stall_outputs: dummy_stall,
            emitter_count,
            comparative_column_count: 0,
        },
        threshold_plan: ComparativeThresholdPlan {
            front_formed: (dummy_band.border_col, 0.5, comparative_event_kind::FRONT_FORMED),
            front_hardened: (
                dummy_out.contest_col,
                0.1,
                comparative_event_kind::FRONT_HARDENED,
            ),
            chokepoint_emerged: (
                dummy_band.chokepoint_col,
                0.5,
                comparative_event_kind::CHOKEPOINT_EMERGED,
            ),
        },
    }
}

fn opt_out_admission(reason: String, emitter_count: u32) -> ComparativeProjectionAdmission {
    let mut adm = insufficient_admission(emitter_count);
    let disp = ComparativeProjectionDisposition::AuthoredOptOut { reason };
    adm.disposition = disp.clone();
    adm.bundle.disposition = disp;
    adm
}

/// Lower-level compile when columns are already allocated (used by production door).
pub fn compile_comparative_bundle(
    request: ComparativeProjectionRequest,
) -> Result<ComparativeProjectionBundle, ComparativeProjectionError> {
    validate_request(&request)?;
    let emitter_count = request.emitters.len() as u32;
    if let Some(reason) = request.authored_opt_out_reason {
        return Ok(ComparativeProjectionBundle {
            disposition: ComparativeProjectionDisposition::AuthoredOptOut {
                reason: reason.into(),
            },
            registrations: Vec::new(),
            outputs: request.outputs,
            band_readouts: request.band_readouts,
            stall_outputs: request.stall_outputs,
            emitter_count,
            comparative_column_count: 0,
        });
    }
    if request.emitters.len() < 2 {
        return Ok(ComparativeProjectionBundle {
            disposition: ComparativeProjectionDisposition::InsufficientEmitters { emitter_count },
            registrations: Vec::new(),
            outputs: request.outputs,
            band_readouts: request.band_readouts,
            stall_outputs: request.stall_outputs,
            emitter_count,
            comparative_column_count: 0,
        });
    }

    let mut registrations = compile_guyang_stall_chain(&request)?;
    registrations.extend(compile_comparative_chain(&request)?);
    Ok(ComparativeProjectionBundle {
        disposition: ComparativeProjectionDisposition::Born {
            emitter_count,
            comparative_column_count: COMPARATIVE_DERIVED_COLUMN_COUNT,
        },
        registrations,
        outputs: request.outputs,
        band_readouts: request.band_readouts,
        stall_outputs: request.stall_outputs,
        emitter_count,
        comparative_column_count: COMPARATIVE_DERIVED_COLUMN_COUNT,
    })
}

fn validate_request(
    request: &ComparativeProjectionRequest,
) -> Result<(), ComparativeProjectionError> {
    if request.emitters.is_empty() {
        return Err(ComparativeProjectionError::EmptyEmitters);
    }
    if request.neighbor_slots.len() as u32 != request.adjacency.slots() {
        return Err(ComparativeProjectionError::NeighborSlotsMismatch {
            actual: request.neighbor_slots.len(),
            expected: request.adjacency.slots(),
        });
    }
    let b = &request.bands;
    if ![
        b.both_strong_floor,
        b.small_margin,
        b.palma_low_d,
        b.contested_border_floor,
        b.front_harden_contest,
    ]
    .iter()
    .all(|x| x.is_finite() && *x >= 0.0)
    {
        return Err(ComparativeProjectionError::InvalidBands);
    }
    let mut used = BTreeSet::new();
    let mut check = |col: ColumnIndex| {
        if col.raw_u32() >= request.n_dims {
            return Err(ComparativeProjectionError::ColumnOutOfRange {
                col: col.raw_u32(),
                n_dims: request.n_dims,
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
        check(e.value_col)?;
    }
    check(request.palma_d_col)?;
    check(request.guyang_value_col)?;
    check(request.guyang_conductance_col)?;
    check(request.stall_outputs.net_flux_col)?;
    check(request.stall_outputs.gross_flux_col)?;
    check(request.stall_outputs.stall_col)?;
    check(request.outputs.dominance_col)?;
    check(request.outputs.margin_col)?;
    check(request.outputs.contest_col)?;
    check(request.band_readouts.border_col)?;
    check(request.band_readouts.chokepoint_col)?;
    Ok(())
}

// ── Field-sweep chains ──────────────────────────────────────────────────────

fn compile_guyang_stall_chain(
    request: &ComparativeProjectionRequest,
) -> Result<Vec<FieldSweepRegistration>, ComparativeProjectionError> {
    let order = request.adjacency.apply_canonical_order_proof();
    let u = request.guyang_value_col;
    let c = request.guyang_conductance_col;
    let mut regs = Vec::new();

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

    let mut gross_map = signed_flux_map;
    gross_map.pop();
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
    // Evaluate emitters in authored_order for program data (tie-break durable).
    let mut emitters_by_order = request.emitters.clone();
    emitters_by_order.sort_by_key(|e| e.authored_order);

    let mut regs = Vec::new();
    let top1 = admit_reg(
        request,
        order,
        FieldSweepOutput::Transient,
        0.0f32.to_bits(),
        ignore_edge_map(),
        keep_accumulator_fold(),
        unrolled_max_post(&emitters_by_order),
        None,
    )?;
    let top1_cert = top1.apply_transient_certificate()?;
    regs.push(top1);

    // Dominance: reverse walk first-equal-to-top1 in authored_order
    let last = emitters_by_order.len() - 1;
    regs.push(admit_reg(
        request,
        order,
        FieldSweepOutput::Matrix(request.outputs.dominance_col),
        0.0f32.to_bits(),
        ignore_edge_map(),
        keep_accumulator_fold(),
        vec![literal(emitters_by_order[last].class_id), ret()],
        None,
    )?);
    for emitter in emitters_by_order.iter().take(last).rev() {
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
    for emitter in &emitters_by_order {
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

    // Border: winner-identity change
    regs.push(admit_reg(
        request,
        order,
        FieldSweepOutput::Matrix(request.band_readouts.border_col),
        0.0f32.to_bits(),
        vec![
            literal(1.0),
            target(request.outputs.dominance_col),
            neighbor(request.outputs.dominance_col),
            binary(eml_opcode::CMP_EQ),
            binary(eml_opcode::SUB),
            ret(),
        ],
        vec![
            param(field_param::ACCUMULATOR),
            param(field_param::MAPPED),
            binary(eml_opcode::MAX),
            ret(),
        ],
        vec![param(field_param::FOLDED), ret()],
        None,
    )?);

    regs.push(admit_reg(
        request,
        order,
        FieldSweepOutput::Matrix(request.band_readouts.chokepoint_col),
        0.0f32.to_bits(),
        ignore_edge_map(),
        keep_accumulator_fold(),
        chokepoint_post(
            request.band_readouts.border_col,
            request.palma_d_col,
            &request.bands,
        ),
        None,
    )?);

    Ok(regs)
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
    for e in emitters.iter().skip(1) {
        nodes.push(target(e.value_col));
        nodes.push(binary(eml_opcode::MAX));
    }
    nodes.push(ret());
    nodes
}
fn dominance_step_post(e: ComparativeEmitterClass, dominance_col: ColumnIndex) -> Vec<EmlNodeGpu> {
    vec![
        target(e.value_col),
        param(field_param::TARGET_TRANSIENT),
        binary(eml_opcode::CMP_EQ),
        literal(e.class_id),
        target(dominance_col),
        select(),
        ret(),
    ]
}
fn second_max_step_post(
    e: ComparativeEmitterClass,
    second_col: ColumnIndex,
    second_init: f32,
) -> Vec<EmlNodeGpu> {
    vec![
        target(second_col),
        target(e.value_col),
        param(field_param::TARGET_TRANSIENT),
        binary(eml_opcode::CMP_LT),
        target(e.value_col),
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
fn literal(v: f32) -> EmlNodeGpu {
    node(eml_opcode::LITERAL_F32, v.to_bits(), 0)
}
fn target(col: ColumnIndex) -> EmlNodeGpu {
    node(eml_opcode::TARGET_VALUE, encode_column(col), 0)
}
fn neighbor(col: ColumnIndex) -> EmlNodeGpu {
    node(eml_opcode::NEIGHBOR_VALUE, encode_column(col), 0)
}
fn param(i: u32) -> EmlNodeGpu {
    node(eml_opcode::PARAM, i, 0)
}
fn unary(op: u32) -> EmlNodeGpu {
    node(op, 0, 0)
}
fn binary(op: u32) -> EmlNodeGpu {
    node(op, 0, 0)
}
fn select() -> EmlNodeGpu {
    node(eml_opcode::SELECT, 0, 0)
}
fn ret() -> EmlNodeGpu {
    node(eml_opcode::RETURN_TOP, 0, 0)
}
