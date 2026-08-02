//! COMPARATIVE-DEFAULT-BIRTH-0 (5.8b) — DA `5154348081` narrowed scope.
//!
//! **IN:** ordinary-install field-plan product + default EMITTER birth.
//! - emitter set from `GameModeSpec.region_fields`
//! - `authored_order` = designer list position
//! - `class_id = authored_order as f32` (not name, not column, not vec iterate)
//! - topology = same-authority adjacency + neighbor rows
//!
//! **OUT:** triad columns stay explicit 5.8 consumer inputs. No producer
//! discriminant, operator→role map, or triad default.

use crate::comparative_projection::{
    admit_comparative_projections, neighbor_slots_from_grid, neighbor_slots_from_link_rows,
    ComparativeEmitterClass, ComparativeProjectionAdmission, ComparativeProjectionBands,
    ComparativeProjectionError,
};
use crate::first_slice_mapping_runtime::compiled_stencil_to_gpu_config;
use simthing_core::{ColumnIndex, DimensionRegistry, SlotIndex};
use simthing_gpu::{
    compile_structured_field_sweeps, FieldAdjacency, FieldSweepAdmissionError, FieldSweepOutput,
    FieldSweepRegistration, LinkGraphNeighbor,
};
use simthing_spec::{
    compile_region_field_preview, RegionFieldSpec, SpecError,
};
use thiserror::Error;

/// Same-authority topology: adjacency + neighbor rows sealed together.
///
/// No public constructor accepts adjacency and neighbor rows independently, so
/// a planted same-length neighbor substitution cannot be built around a correct
/// adjacency without going through a capture site.
#[derive(Clone, Debug)]
pub struct SealedFieldTopology {
    adjacency: FieldAdjacency,
    neighbor_slots: Vec<Vec<SlotIndex>>,
}

impl SealedFieldTopology {
    /// Capture neighbor rows from public grid metadata at the same site that
    /// already holds the admitted grid adjacency (5.8 grid seam).
    pub fn from_grid_adjacency(
        adjacency: FieldAdjacency,
    ) -> Result<Self, FieldPlanAdmissionError> {
        let neighbor_slots = neighbor_slots_from_grid(&adjacency)
            .ok_or(FieldPlanAdmissionError::GridNeighborCaptureUnavailable)?;
        if neighbor_slots.len() as u32 != adjacency.slots() {
            return Err(FieldPlanAdmissionError::NeighborSlotsMismatch {
                actual: neighbor_slots.len(),
                expected: adjacency.slots(),
            });
        }
        Ok(Self {
            adjacency,
            neighbor_slots,
        })
    }

    /// Build LinkGraph adjacency and seal the exact construction-time rows
    /// (5.8 LinkGraph seam — no accessor, no reconstruction).
    pub fn from_link_graph(
        slot_count: u32,
        neighbors: Vec<Vec<LinkGraphNeighbor>>,
        gather_col: ColumnIndex,
    ) -> Result<Self, FieldPlanAdmissionError> {
        let neighbor_slots = neighbor_slots_from_link_rows(&neighbors);
        let adjacency = FieldAdjacency::link_graph(slot_count, neighbors, gather_col)
            .map_err(FieldPlanAdmissionError::FieldSweep)?;
        if neighbor_slots.len() as u32 != adjacency.slots() {
            return Err(FieldPlanAdmissionError::NeighborSlotsMismatch {
                actual: neighbor_slots.len(),
                expected: adjacency.slots(),
            });
        }
        Ok(Self {
            adjacency,
            neighbor_slots,
        })
    }

    pub fn adjacency(&self) -> &FieldAdjacency {
        &self.adjacency
    }

    pub fn neighbor_slots(&self) -> &[Vec<SlotIndex>] {
        &self.neighbor_slots
    }
}

/// Ordinary-install admission product (S3): topology + default emitters.
///
/// Precedented by `SpecSessionState.property_admission` delivery. No triad
/// columns, no role-named registration fields.
#[derive(Clone, Debug)]
pub struct FieldPlanAdmissionReport {
    topology: SealedFieldTopology,
    /// Emitter classes in **authored** `region_fields` order.
    emitters: Vec<ComparativeEmitterClass>,
    /// Human-facing `RegionFieldSpec::name` parallel to emitters (diagnostics only).
    emitter_names: Vec<String>,
}

impl FieldPlanAdmissionReport {
    pub fn topology(&self) -> &SealedFieldTopology {
        &self.topology
    }

    pub fn emitters(&self) -> &[ComparativeEmitterClass] {
        &self.emitters
    }

    pub fn emitter_names(&self) -> &[String] {
        &self.emitter_names
    }
}

#[derive(Debug, Error)]
pub enum FieldPlanAdmissionError {
    #[error("grid neighbor capture unavailable for this FieldAdjacency")]
    GridNeighborCaptureUnavailable,
    #[error("neighbor_slots length {actual} != adjacency slots {expected}")]
    NeighborSlotsMismatch { actual: usize, expected: u32 },
    #[error("region-field `{name}` registration produced no Matrix value column")]
    MissingMatrixValueColumn { name: String },
    #[error("region-field set has inconsistent FieldAdjacency across authored entries")]
    InconsistentAdjacency,
    #[error("region-field admission: {0}")]
    Spec(String),
    #[error(transparent)]
    FieldSweep(#[from] FieldSweepAdmissionError),
    #[error("field-sweep instance: {0}")]
    FieldSweepInstance(String),
    #[error(transparent)]
    Comparative(#[from] ComparativeProjectionError),
}

impl From<SpecError> for FieldPlanAdmissionError {
    fn from(err: SpecError) -> Self {
        Self::Spec(err.to_string())
    }
}

impl From<simthing_gpu::FieldSweepInstanceError> for FieldPlanAdmissionError {
    fn from(err: simthing_gpu::FieldSweepInstanceError) -> Self {
        Self::FieldSweepInstance(err.to_string())
    }
}

/// Mint the install product from designer-authored `region_fields`.
///
/// Empty input → `Ok(None)` (ordinary install without field plan).
pub fn admit_field_plan_from_region_fields(
    region_fields: &[RegionFieldSpec],
) -> Result<Option<FieldPlanAdmissionReport>, FieldPlanAdmissionError> {
    if region_fields.is_empty() {
        return Ok(None);
    }

    let mut emitters = Vec::with_capacity(region_fields.len());
    let mut emitter_names = Vec::with_capacity(region_fields.len());
    let mut topology: Option<SealedFieldTopology> = None;

    for (authored_order, spec) in region_fields.iter().enumerate() {
        let preview = compile_region_field_preview(spec)?;
        let config = compiled_stencil_to_gpu_config(&preview.stencil);
        let registrations = compile_structured_field_sweeps(&config)?;
        let value_col = matrix_value_col_for_field(spec, &registrations)?;
        let adjacency = registrations
            .first()
            .map(|r| r.adjacency().clone())
            .ok_or_else(|| FieldPlanAdmissionError::MissingMatrixValueColumn {
                name: spec.name.clone(),
            })?;

        match &topology {
            None => {
                // Shared comparative topology = first authored field's geometry.
                // Later fields may differ only by gather_col (source) while
                // still sharing grid shape/offsets; that is normal for multi-
                // emitter Normalized fields and is not an inconsistency.
                topology = Some(SealedFieldTopology::from_grid_adjacency(adjacency)?);
            }
            Some(existing) => {
                if !same_grid_geometry(existing.adjacency(), &adjacency) {
                    return Err(FieldPlanAdmissionError::InconsistentAdjacency);
                }
            }
        }

        let order = authored_order as u32;
        emitters.push(ComparativeEmitterClass {
            authored_order: order,
            class_id: order as f32,
            value_col,
        });
        emitter_names.push(spec.name.clone());
    }

    Ok(Some(FieldPlanAdmissionReport {
        topology: topology.expect("non-empty region_fields"),
        emitters,
        emitter_names,
    }))
}

fn same_grid_geometry(a: &FieldAdjacency, b: &FieldAdjacency) -> bool {
    a.slots() == b.slots()
        && a.grid_shape() == b.grid_shape()
        && a.grid_offsets_data() == b.grid_offsets_data()
}

fn matrix_value_col_for_field(
    spec: &RegionFieldSpec,
    registrations: &[FieldSweepRegistration],
) -> Result<ColumnIndex, FieldPlanAdmissionError> {
    // Prefer Matrix(target_col) when present (Normalized / SourceCapped /
    // Gradient sinks). SaturatingFlux writes Matrix(source_col) for value.
    let target = ColumnIndex::from_gpu_round_trip(spec.target_col);
    let source = ColumnIndex::from_gpu_round_trip(spec.source_col);
    for reg in registrations {
        if let FieldSweepOutput::Matrix(col) = reg.output() {
            if col == target || col == source {
                return Ok(col);
            }
        }
    }
    for reg in registrations {
        if let FieldSweepOutput::Matrix(col) = reg.output() {
            return Ok(col);
        }
    }
    Err(FieldPlanAdmissionError::MissingMatrixValueColumn {
        name: spec.name.clone(),
    })
}

/// Default-emitter + **explicit triad** comparative admission via settled 5.8.
///
/// Topology and emitters come from the install product; triad columns remain
/// caller-supplied (not defaulted).
pub fn admit_comparative_from_field_plan(
    registry: &mut DimensionRegistry,
    report: &FieldPlanAdmissionReport,
    palma_d_col: ColumnIndex,
    guyang_value_col: ColumnIndex,
    guyang_conductance_col: ColumnIndex,
    bands: ComparativeProjectionBands,
    authored_opt_out_reason: Option<&'static str>,
) -> Result<ComparativeProjectionAdmission, FieldPlanAdmissionError> {
    Ok(admit_comparative_projections(
        registry,
        report.topology.adjacency().clone(),
        report.topology.neighbor_slots().to_vec(),
        report.emitters.clone(),
        palma_d_col,
        guyang_value_col,
        guyang_conductance_col,
        bands,
        authored_opt_out_reason,
    )?)
}
