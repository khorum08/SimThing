//! COMPARATIVE-DEFAULT-BIRTH-0 (5.8b) — DA seam ruling `5153818317` / remand `5153845512`.
//!
//! S1: neighbor rows captured at the same site that builds `FieldAdjacency`
//!     (5.8 `ComparativeProjectionRequest` seam) — not reconstructed, not a
//!     second topology authority.
//! S2: roles from public `FieldSweepRegistration::output()` Matrix bindings +
//!     authored registration order — no opcode/heuristic classification.
//! S3: one typed admission product delivered on ordinary `compile_and_install`
//!     like `property_admission` — no parallel install side-door.

use crate::comparative_projection::{
    admit_comparative_projections, ComparativeEmitterClass, ComparativeProjectionAdmission,
    ComparativeProjectionBands, ComparativeProjectionError,
};
use simthing_core::{ColumnIndex, DimensionRegistry, SlotIndex};
use simthing_gpu::{FieldAdjacency, FieldSweepOutput, FieldSweepRegistration};
use thiserror::Error;

/// S3 typed admission product — only already-admitted facts.
///
/// Precedented by `SpecSessionState.property_admission` delivery: ordinary
/// install lands this on session state and may default-birth comparative
/// projections when the set supports it.
#[derive(Clone, Debug)]
pub struct FieldPlanAdmissionReport {
    adjacency: FieldAdjacency,
    neighbor_slots: Vec<Vec<SlotIndex>>,
    /// Emitter registrations in **authored order** (durable order key).
    emitter_registrations: Vec<FieldSweepRegistration>,
    palma_d: FieldSweepRegistration,
    guyang_conductance: FieldSweepRegistration,
    guyang_value: FieldSweepRegistration,
    /// Existing authored opt-out path (visible disposition).
    authored_opt_out_reason: Option<&'static str>,
}

impl FieldPlanAdmissionReport {
    pub fn adjacency(&self) -> &FieldAdjacency {
        &self.adjacency
    }

    pub fn neighbor_slots(&self) -> &[Vec<SlotIndex>] {
        &self.neighbor_slots
    }

    pub fn emitter_registrations(&self) -> &[FieldSweepRegistration] {
        &self.emitter_registrations
    }

    pub fn palma_d(&self) -> &FieldSweepRegistration {
        &self.palma_d
    }

    pub fn guyang_conductance(&self) -> &FieldSweepRegistration {
        &self.guyang_conductance
    }

    pub fn guyang_value(&self) -> &FieldSweepRegistration {
        &self.guyang_value
    }

    pub fn authored_opt_out_reason(&self) -> Option<&'static str> {
        self.authored_opt_out_reason
    }
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum FieldPlanAdmissionError {
    #[error("neighbor_slots length {actual} != adjacency slots {expected}")]
    NeighborSlotsMismatch { actual: usize, expected: u32 },
    #[error("field-plan registration adjacency does not match the admitted FieldAdjacency")]
    AdjacencyIdentityMismatch,
    #[error("registration output is not Matrix (required for role binding)")]
    NonMatrixOutput,
    #[error("duplicate matrix output column {col} across field-plan registrations")]
    DuplicateOutputColumn { col: u32 },
    #[error(transparent)]
    Comparative(#[from] ComparativeProjectionError),
}

/// Mint the S3 product from already-admitted topology + registrations.
///
/// # S1
/// `neighbor_slots` must be the rows captured at the same lowering site that
/// built `adjacency` (grid: public offsets helper; LinkGraph: the
/// `Vec<Vec<LinkGraphNeighbor>>` passed to `FieldAdjacency::link_graph`).
///
/// # S2
/// Roles are the **public Matrix output bindings** of the supplied
/// registrations. Emitters are `emitter_registrations` in authored order.
/// `class_id` is the admitted output column identity (not a synthesized order
/// convention).
pub fn admit_field_plan_report(
    adjacency: FieldAdjacency,
    neighbor_slots: Vec<Vec<SlotIndex>>,
    emitter_registrations: Vec<FieldSweepRegistration>,
    palma_d: FieldSweepRegistration,
    guyang_conductance: FieldSweepRegistration,
    guyang_value: FieldSweepRegistration,
    authored_opt_out_reason: Option<&'static str>,
) -> Result<FieldPlanAdmissionReport, FieldPlanAdmissionError> {
    if neighbor_slots.len() as u32 != adjacency.slots() {
        return Err(FieldPlanAdmissionError::NeighborSlotsMismatch {
            actual: neighbor_slots.len(),
            expected: adjacency.slots(),
        });
    }

    let mut seen = std::collections::BTreeSet::new();
    let mut check_reg = |reg: &FieldSweepRegistration| -> Result<ColumnIndex, FieldPlanAdmissionError> {
        if reg.adjacency() != &adjacency {
            return Err(FieldPlanAdmissionError::AdjacencyIdentityMismatch);
        }
        let FieldSweepOutput::Matrix(col) = reg.output() else {
            return Err(FieldPlanAdmissionError::NonMatrixOutput);
        };
        if !seen.insert(col.raw_u32()) {
            return Err(FieldPlanAdmissionError::DuplicateOutputColumn {
                col: col.raw_u32(),
            });
        }
        Ok(col)
    };

    for reg in &emitter_registrations {
        check_reg(reg)?;
    }
    check_reg(&palma_d)?;
    check_reg(&guyang_conductance)?;
    check_reg(&guyang_value)?;

    Ok(FieldPlanAdmissionReport {
        adjacency,
        neighbor_slots,
        emitter_registrations,
        palma_d,
        guyang_conductance,
        guyang_value,
        authored_opt_out_reason,
    })
}

/// S2: derive comparative inputs from public Matrix output bindings + authored order.
pub fn comparative_inputs_from_field_plan(
    report: &FieldPlanAdmissionReport,
) -> Result<(Vec<ComparativeEmitterClass>, ColumnIndex, ColumnIndex, ColumnIndex), FieldPlanAdmissionError>
{
    let palma_d_col = matrix_col(&report.palma_d)?;
    let guyang_conductance_col = matrix_col(&report.guyang_conductance)?;
    let guyang_value_col = matrix_col(&report.guyang_value)?;

    let mut emitters = Vec::with_capacity(report.emitter_registrations.len());
    for (authored_order, reg) in report.emitter_registrations.iter().enumerate() {
        let value_col = matrix_col(reg)?;
        // Durable identity = admitted output column binding (5.8 class_id authority).
        let class_id = value_col.raw_u32() as f32 + 1.0;
        emitters.push(ComparativeEmitterClass {
            authored_order: authored_order as u32,
            class_id,
            value_col,
        });
    }

    Ok((
        emitters,
        palma_d_col,
        guyang_value_col,
        guyang_conductance_col,
    ))
}

fn matrix_col(reg: &FieldSweepRegistration) -> Result<ColumnIndex, FieldPlanAdmissionError> {
    match reg.output() {
        FieldSweepOutput::Matrix(col) => Ok(col),
        FieldSweepOutput::Transient => Err(FieldPlanAdmissionError::NonMatrixOutput),
    }
}

/// Default birth via existing 5.8 door from the S3 admission product.
pub fn default_comparative_birth(
    registry: &mut DimensionRegistry,
    report: &FieldPlanAdmissionReport,
    bands: ComparativeProjectionBands,
) -> Result<ComparativeProjectionAdmission, FieldPlanAdmissionError> {
    if let Some(reason) = report.authored_opt_out_reason {
        return Ok(admit_comparative_projections(
            registry,
            report.adjacency.clone(),
            report.neighbor_slots.clone(),
            Vec::new(),
            ColumnIndex::from_gpu_round_trip(0),
            ColumnIndex::from_gpu_round_trip(0),
            ColumnIndex::from_gpu_round_trip(0),
            bands,
            Some(reason),
        )?);
    }

    let (emitters, palma_d_col, guyang_value_col, guyang_conductance_col) =
        comparative_inputs_from_field_plan(report)?;

    Ok(admit_comparative_projections(
        registry,
        report.adjacency.clone(),
        report.neighbor_slots.clone(),
        emitters,
        palma_d_col,
        guyang_value_col,
        guyang_conductance_col,
        bands,
        None,
    )?)
}
