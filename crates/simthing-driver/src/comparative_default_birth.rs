//! COMPARATIVE-DEFAULT-BIRTH-0 (5.8b) — transport + derivation only.
//!
//! Seam A: carry already-admitted `FieldAdjacency` / field-plan registrations
//! into `SpecSessionState` (no second topology authority, no inference).
//!
//! Seam B: derive emitter/PALMA-D/Gu-Yang-U/Gu-Yang-C from the registrations'
//! existing program/column bindings + authored registration order. No string
//! namespace, no property-name grammar, no role enum.
//!
//! Default birth invokes existing 5.8 [`admit_comparative_projections`].

use crate::comparative_projection::{
    admit_comparative_projections, neighbor_slots_from_grid, ComparativeEmitterClass,
    ComparativeProjectionAdmission, ComparativeProjectionBands, ComparativeProjectionError,
};
use simthing_core::{eml_opcode, ColumnIndex, DimensionRegistry, SlotIndex};
use simthing_gpu::{FieldAdjacency, FieldSweepOutput, FieldSweepRegistration};
use thiserror::Error;

/// Exact admitted field-plan binding (seam A). Topology is the sealed adjacency
/// identity carried from admission — never re-derived from shape/names.
#[derive(Clone, Debug)]
pub struct AdmittedFieldPlanBinding {
    adjacency: FieldAdjacency,
    neighbor_slots: Vec<Vec<SlotIndex>>,
    /// Authored admission order of field-sweep registrations (durable order key).
    registrations: Vec<FieldSweepRegistration>,
    adjacency_order_fingerprint: u64,
}

impl AdmittedFieldPlanBinding {
    pub fn adjacency(&self) -> &FieldAdjacency {
        &self.adjacency
    }

    pub fn neighbor_slots(&self) -> &[Vec<SlotIndex>] {
        &self.neighbor_slots
    }

    pub fn registrations(&self) -> &[FieldSweepRegistration] {
        &self.registrations
    }

    pub fn adjacency_order_fingerprint(&self) -> u64 {
        self.adjacency_order_fingerprint
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum FieldPlanBindingError {
    #[error("field-plan binding requires at least one FieldSweepRegistration")]
    EmptyRegistrations,
    #[error(
        "field-plan registration adjacency fingerprint mismatch (expected {expected}, got {actual})"
    )]
    AdjacencyIdentityMismatch { expected: u64, actual: u64 },
    #[error("neighbor_slots length {actual} != adjacency slots {expected}")]
    NeighborSlotsMismatch { actual: usize, expected: u32 },
    #[error("cannot seal neighbor rows from the admitted FieldAdjacency (no public grid/link data)")]
    NeighborSlotsUnavailable,
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum DefaultComparativeBirthError {
    #[error(transparent)]
    FieldPlan(#[from] FieldPlanBindingError),
    #[error(transparent)]
    Comparative(#[from] ComparativeProjectionError),
    #[error("ambiguous or missing triad role from admitted field registrations: {detail}")]
    AmbiguousRole { detail: String },
}

/// Seal an already-admitted field plan into a session-carried binding.
///
/// `registrations` must all bind the **exact same** `FieldAdjacency` identity
/// (order fingerprint). Neighbor rows are sealed from the adjacency's public
/// grid offsets or link rows — never reinvented from slot-count heuristics.
pub fn admit_field_plan_binding(
    adjacency: FieldAdjacency,
    registrations: Vec<FieldSweepRegistration>,
) -> Result<AdmittedFieldPlanBinding, FieldPlanBindingError> {
    if registrations.is_empty() {
        return Err(FieldPlanBindingError::EmptyRegistrations);
    }
    let expected = adjacency.order_fingerprint();
    for reg in &registrations {
        let actual = reg.adjacency().order_fingerprint();
        if actual != expected {
            return Err(FieldPlanBindingError::AdjacencyIdentityMismatch {
                expected,
                actual,
            });
        }
    }
    let neighbor_slots = seal_neighbor_slots(&adjacency)?;
    if neighbor_slots.len() as u32 != adjacency.slots() {
        return Err(FieldPlanBindingError::NeighborSlotsMismatch {
            actual: neighbor_slots.len(),
            expected: adjacency.slots(),
        });
    }
    Ok(AdmittedFieldPlanBinding {
        adjacency_order_fingerprint: expected,
        adjacency,
        neighbor_slots,
        registrations,
    })
}

fn seal_neighbor_slots(
    adjacency: &FieldAdjacency,
) -> Result<Vec<Vec<SlotIndex>>, FieldPlanBindingError> {
    if let Some(slots) = neighbor_slots_from_grid(adjacency) {
        return Ok(slots);
    }
    // LinkGraph: rebuild sealed neighbor rows from public link construction
    // data if available via grid_shape absence + lists. FieldAdjacency does not
    // re-export private link rows; use degree-bucket walk via public_neighbors
    // is not available. Fail closed rather than invent topology.
    //
    // For LinkGraph-admitted adjacency, neighbor_slots_from_link_rows must be
    // supplied at binding construction. Use the grid path when shape exists.
    let _ = adjacency;
    Err(FieldPlanBindingError::NeighborSlotsUnavailable)
}

/// Seal a LinkGraph field plan when neighbor rows are already known (from the
/// same link construction that admitted the adjacency). No topology re-inference.
pub fn admit_field_plan_binding_with_neighbors(
    adjacency: FieldAdjacency,
    registrations: Vec<FieldSweepRegistration>,
    neighbor_slots: Vec<Vec<SlotIndex>>,
) -> Result<AdmittedFieldPlanBinding, FieldPlanBindingError> {
    if registrations.is_empty() {
        return Err(FieldPlanBindingError::EmptyRegistrations);
    }
    let expected = adjacency.order_fingerprint();
    for reg in &registrations {
        let actual = reg.adjacency().order_fingerprint();
        if actual != expected {
            return Err(FieldPlanBindingError::AdjacencyIdentityMismatch {
                expected,
                actual,
            });
        }
    }
    if neighbor_slots.len() as u32 != adjacency.slots() {
        return Err(FieldPlanBindingError::NeighborSlotsMismatch {
            actual: neighbor_slots.len(),
            expected: adjacency.slots(),
        });
    }
    // Sanity: neighbor rows must match sealed grid path when grid data exists.
    if let Some(grid_slots) = neighbor_slots_from_grid(&adjacency) {
        if grid_slots != neighbor_slots {
            return Err(FieldPlanBindingError::AdjacencyIdentityMismatch {
                expected,
                actual: expected.wrapping_add(1), // distinct fail for neighbor substitute
            });
        }
    }
    Ok(AdmittedFieldPlanBinding {
        adjacency_order_fingerprint: expected,
        adjacency,
        neighbor_slots,
        registrations,
    })
}

/// Inputs derived from admitted field-plan registrations (seam B).
#[derive(Clone, Debug)]
pub struct DerivedComparativeInputs {
    pub emitters: Vec<ComparativeEmitterClass>,
    pub palma_d_col: ColumnIndex,
    pub guyang_value_col: ColumnIndex,
    pub guyang_conductance_col: ColumnIndex,
}

/// Derive comparative inputs from field-plan registration structure only.
///
/// Classification uses **existing sealed facts** on each registration:
/// - conservative field-law proof → Gu-Yang value (U)
/// - non-conservative fold containing MUL → Gu-Yang conductance (C)
/// - non-conservative fold containing MIN → PALMA D
/// - remaining matrix outputs, in **authored registration order** → emitters
///
/// Ambiguous or missing roles fail closed (no guessing).
pub fn derive_comparative_inputs_from_field_plan(
    plan: &AdmittedFieldPlanBinding,
) -> Result<DerivedComparativeInputs, DefaultComparativeBirthError> {
    let mut palma: Option<ColumnIndex> = None;
    let mut guyang_u: Option<ColumnIndex> = None;
    let mut guyang_c: Option<ColumnIndex> = None;
    let mut emitters: Vec<ComparativeEmitterClass> = Vec::new();

    for (authored_order, reg) in plan.registrations.iter().enumerate() {
        let FieldSweepOutput::Matrix(col) = reg.output() else {
            continue;
        };
        if reg.field_law_proof().is_conservative() {
            if guyang_u.replace(col).is_some() {
                return Err(DefaultComparativeBirthError::AmbiguousRole {
                    detail: "multiple conservative matrix outputs (Gu-Yang U)".into(),
                });
            }
            continue;
        }
        let fold = reg.fold_program();
        let has_min = fold.iter().any(|n| n.opcode == eml_opcode::MIN);
        let has_mul = fold.iter().any(|n| n.opcode == eml_opcode::MUL);
        if has_min && has_mul {
            return Err(DefaultComparativeBirthError::AmbiguousRole {
                detail: "registration fold has both MIN and MUL".into(),
            });
        }
        if has_min {
            if palma.replace(col).is_some() {
                return Err(DefaultComparativeBirthError::AmbiguousRole {
                    detail: "multiple MIN-fold matrix outputs (PALMA D)".into(),
                });
            }
            continue;
        }
        if has_mul {
            if guyang_c.replace(col).is_some() {
                return Err(DefaultComparativeBirthError::AmbiguousRole {
                    detail: "multiple MUL-fold matrix outputs (Gu-Yang C)".into(),
                });
            }
            continue;
        }
        // Residual matrix output: competing emitter class. Authored order is
        // the registration's position in the admitted field-plan list.
        emitters.push(ComparativeEmitterClass {
            authored_order: authored_order as u32,
            class_id: durable_emitter_class_id(authored_order as u32, col),
            value_col: col,
        });
    }

    let palma_d_col = palma.ok_or_else(|| DefaultComparativeBirthError::AmbiguousRole {
        detail: "no MIN-fold matrix output for PALMA D".into(),
    })?;
    let guyang_value_col = guyang_u.ok_or_else(|| DefaultComparativeBirthError::AmbiguousRole {
        detail: "no conservative matrix output for Gu-Yang U".into(),
    })?;
    let guyang_conductance_col =
        guyang_c.ok_or_else(|| DefaultComparativeBirthError::AmbiguousRole {
            detail: "no MUL-fold matrix output for Gu-Yang C".into(),
        })?;

    // Re-index emitter authored_order densely 0..n while preserving relative order.
    emitters.sort_by_key(|e| e.authored_order);
    for (i, e) in emitters.iter_mut().enumerate() {
        e.authored_order = i as u32;
        e.class_id = durable_emitter_class_id(i as u32, e.value_col);
    }

    Ok(DerivedComparativeInputs {
        emitters,
        palma_d_col,
        guyang_value_col,
        guyang_conductance_col,
    })
}

fn durable_emitter_class_id(authored_order: u32, col: ColumnIndex) -> f32 {
    // Durable finite identity from authored order + column (not registration hash walk).
    (authored_order as f32 + 1.0) * 1000.0 + col.raw() as f32
}

/// Default-derived comparative birth from an admitted field-plan binding.
/// Invokes the existing 5.8 production door; fails closed on role/topology gaps.
pub fn default_comparative_birth_from_field_plan(
    registry: &mut DimensionRegistry,
    plan: &AdmittedFieldPlanBinding,
    bands: ComparativeProjectionBands,
    authored_opt_out_reason: Option<&'static str>,
) -> Result<ComparativeProjectionAdmission, DefaultComparativeBirthError> {
    let inputs = if authored_opt_out_reason.is_some() {
        // Opt-out still requires a lawful plan identity; emitter list may be empty.
        DerivedComparativeInputs {
            emitters: Vec::new(),
            palma_d_col: ColumnIndex::from_gpu_round_trip(0),
            guyang_value_col: ColumnIndex::from_gpu_round_trip(0),
            guyang_conductance_col: ColumnIndex::from_gpu_round_trip(0),
        }
    } else {
        derive_comparative_inputs_from_field_plan(plan)?
    };

    Ok(admit_comparative_projections(
        registry,
        plan.adjacency.clone(),
        plan.neighbor_slots.clone(),
        inputs.emitters,
        inputs.palma_d_col,
        inputs.guyang_value_col,
        inputs.guyang_conductance_col,
        bands,
        authored_opt_out_reason,
    )?)
}

