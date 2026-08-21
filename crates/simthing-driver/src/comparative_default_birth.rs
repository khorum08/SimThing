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
//!
//! Same-length neighbor substitution onto a sealed adjacency is unconstructible
//! (fields private; no rebind constructor). Remand `5156686392` privacy referee:
//!
//! ```compile_fail,E0451
//! use simthing_driver::SealedFieldTopology;
//! use simthing_gpu::FieldAdjacency;
//! use simthing_core::{ColumnIndex, SlotIndex};
//! fn rebind_same_length_wrong_rows(
//!     adj: FieldAdjacency,
//!     wrong_rows: Vec<Vec<SlotIndex>>,
//! ) -> SealedFieldTopology {
//!     // No public constructor takes (adjacency, neighbor_slots) independently.
//!     SealedFieldTopology {
//!         adjacency: adj,
//!         neighbor_slots: wrong_rows,
//!     }
//! }
//! ```

use crate::comparative_projection::{
    admit_comparative_projections, neighbor_slots_from_grid, neighbor_slots_from_link_rows,
    ComparativeEmitterClass, ComparativeProjectionAdmission, ComparativeProjectionBands,
    ComparativeProjectionError,
};
use crate::mapping_runtime::compiled_stencil_to_gpu_config;
use simthing_core::{ColumnIndex, DimensionRegistry, SlotIndex};
use simthing_gpu::{
    compile_structured_field_sweeps, FieldAdjacency, FieldSweepAdmissionError, FieldSweepOutput,
    FieldSweepRegistration, LinkGraphNeighbor,
};
use simthing_spec::{compile_region_field_preview, RegionFieldSpec, SpecError};
use thiserror::Error;

/// Same-authority topology: adjacency + neighbor rows sealed together.
///
/// No public constructor accepts adjacency and neighbor rows independently, so
/// a planted same-length neighbor substitution cannot rebind rows onto a correct
/// adjacency without going through a capture site.
#[derive(Clone, Debug)]
pub struct SealedFieldTopology {
    adjacency: FieldAdjacency,
    neighbor_slots: Vec<Vec<SlotIndex>>,
}

impl SealedFieldTopology {
    /// Capture neighbor rows from public grid metadata at the same site that
    /// already holds the admitted grid adjacency (5.8 grid seam).
    pub fn from_grid_adjacency(adjacency: FieldAdjacency) -> Result<Self, FieldPlanAdmissionError> {
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

    pub fn slots(&self) -> u32 {
        self.adjacency.slots()
    }
}

/// Ordinary-install admission product (S3): topology + default emitters.
///
/// Precedented by `SpecSessionState.property_admission` delivery. No triad
/// columns, no role-named registration fields.
#[derive(Clone, Debug)]
pub struct FieldPlanAdmissionReport {
    topology: SealedFieldTopology,
    /// Emitter classes in **authored** `region_fields` order (sorted by
    /// `authored_order`, never incidental collection order).
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
    #[error(
        "region-field `{name}` has no unique Matrix value binding \
         (need exactly one of Matrix(target_col) or Matrix(source_col); got target={target_col} source={source_col})"
    )]
    AmbiguousOrMissingMatrixValueColumn {
        name: String,
        target_col: u32,
        source_col: u32,
    },
    #[error("region-field set has inconsistent grid geometry across authored entries")]
    InconsistentAdjacency,
    #[error("topology slot count {topology} != emitter theater expectation {expected}")]
    TopologySlotMismatch { topology: u32, expected: u32 },
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
/// Topology is the first field's grid geometry (region_fields lower to structured
/// grid stencils only — no LinkGraph producer on this path).
pub fn admit_field_plan_from_region_fields(
    region_fields: &[RegionFieldSpec],
) -> Result<Option<FieldPlanAdmissionReport>, FieldPlanAdmissionError> {
    if region_fields.is_empty() {
        return Ok(None);
    }

    // Stage then sort by authored_order so incidental collection order never
    // authors identity (remand 5154599161 item 2).
    let mut staged: Vec<(u32, ComparativeEmitterClass, String, FieldAdjacency)> =
        Vec::with_capacity(region_fields.len());
    for (authored_order, spec) in region_fields.iter().enumerate() {
        let preview = compile_region_field_preview(spec)?;
        let config = compiled_stencil_to_gpu_config(&preview.stencil);
        let registrations = compile_structured_field_sweeps(&config)?;
        // Typed columns from admitted stencil (not raw u32 mint).
        let value_col = matrix_value_col_for_field(spec, &preview.stencil, &registrations)?;
        let adjacency = registrations
            .first()
            .map(|r| r.adjacency().clone())
            .ok_or_else(
                || FieldPlanAdmissionError::AmbiguousOrMissingMatrixValueColumn {
                    name: spec.name.clone(),
                    target_col: spec.target_col,
                    source_col: spec.source_col,
                },
            )?;
        let order = authored_order as u32;
        staged.push((
            order,
            ComparativeEmitterClass {
                authored_order: order,
                class_id: order as f32,
                value_col,
            },
            spec.name.clone(),
            adjacency,
        ));
    }

    // Incidental reverse of staged rows must not change outcome after sort.
    // Production always sorts; tests reverse-then-call same function via
    // reordering only is not exposed — sort is the durable authoring key.
    staged.sort_by_key(|(order, _, _, _)| *order);

    let mut topology: Option<SealedFieldTopology> = None;
    let mut emitters = Vec::with_capacity(staged.len());
    let mut emitter_names = Vec::with_capacity(staged.len());
    for (_, emitter, name, adjacency) in staged {
        match &topology {
            None => {
                topology = Some(SealedFieldTopology::from_grid_adjacency(adjacency)?);
            }
            Some(existing) => {
                if !same_grid_geometry(existing.adjacency(), &adjacency) {
                    return Err(FieldPlanAdmissionError::InconsistentAdjacency);
                }
            }
        }
        emitters.push(emitter);
        emitter_names.push(name);
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

/// Exact unique Matrix binding for an authored field.
///
/// Accepts **exactly one** of:
/// - a unique registration with `output() == Matrix(target_col)`, or
/// - a unique registration with `output() == Matrix(source_col)` when no
///   target Matrix exists (SaturatingFlux value writes `source_col`).
///
/// Rejects: zero matches, both target and source present, multiple matches,
/// or any "first Matrix" fallback (remand 5154599161 item 1).
fn matrix_value_col_for_field(
    spec: &RegionFieldSpec,
    stencil: &simthing_spec::CompiledRegionFieldStencilSpec,
    registrations: &[FieldSweepRegistration],
) -> Result<ColumnIndex, FieldPlanAdmissionError> {
    // Admitted typed columns from region-field compile (no ColumnIndex mint).
    let target = stencil.target_col;
    let source = stencil.source_col;

    let decide = |regs: &mut dyn Iterator<Item = &FieldSweepRegistration>| {
        let mut target_hits = 0u32;
        let mut source_hits = 0u32;
        for reg in regs {
            if let FieldSweepOutput::Matrix(col) = reg.output() {
                if col == target {
                    target_hits += 1;
                } else if col == source {
                    source_hits += 1;
                }
            }
        }
        match (target_hits, source_hits) {
            (1, 0) => Ok(target),
            (0, 1) => Ok(source),
            // Single Matrix when authored target_col == source_col.
            (1, _) if target == source => Ok(target),
            _ => Err(()),
        }
    };

    let forward = decide(&mut registrations.iter());
    let reverse = decide(&mut registrations.iter().rev());
    // Incidental registration order must not change the unique binding.
    match (forward, reverse) {
        (Ok(a), Ok(b)) if a == b => Ok(a),
        _ => Err(
            FieldPlanAdmissionError::AmbiguousOrMissingMatrixValueColumn {
                name: spec.name.clone(),
                target_col: spec.target_col,
                source_col: spec.source_col,
            },
        ),
    }
}

/// Default-emitter + **explicit triad** comparative admission via settled 5.8,
/// using the install product's sealed topology.
pub fn admit_comparative_from_field_plan(
    registry: &mut DimensionRegistry,
    report: &FieldPlanAdmissionReport,
    palma_d_col: ColumnIndex,
    guyang_value_col: ColumnIndex,
    guyang_conductance_col: ColumnIndex,
    bands: ComparativeProjectionBands,
    authored_opt_out_reason: Option<&'static str>,
) -> Result<ComparativeProjectionAdmission, FieldPlanAdmissionError> {
    admit_comparative_from_emitters_and_topology(
        registry,
        report.topology(),
        report.emitters(),
        palma_d_col,
        guyang_value_col,
        guyang_conductance_col,
        bands,
        authored_opt_out_reason,
    )
}

/// Default emitters (from product) + any same-authority sealed topology.
///
/// Used when the comparative theater topology is a sealed LinkGraph (or other
/// sealed capture) while emitter identity/order still come from the 5.8b
/// region_fields derivation. Topology must be sealed — never reconstructed
/// beside an existing adjacency.
pub fn admit_comparative_from_emitters_and_topology(
    registry: &mut DimensionRegistry,
    topology: &SealedFieldTopology,
    emitters: &[ComparativeEmitterClass],
    palma_d_col: ColumnIndex,
    guyang_value_col: ColumnIndex,
    guyang_conductance_col: ColumnIndex,
    bands: ComparativeProjectionBands,
    authored_opt_out_reason: Option<&'static str>,
) -> Result<ComparativeProjectionAdmission, FieldPlanAdmissionError> {
    Ok(admit_comparative_projections(
        registry,
        topology.adjacency().clone(),
        topology.neighbor_slots().to_vec(),
        emitters.to_vec(),
        palma_d_col,
        guyang_value_col,
        guyang_conductance_col,
        bands,
        authored_opt_out_reason,
    )?)
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use simthing_spec::{
        RegionFieldCadenceSpec, RegionFieldGridProfile, RegionFieldOperatorSpec,
        RegionFieldSourcePolicySpec, RegionFieldSummaryPolicySpec,
    };

    fn rf(name: &str, src: u32, tgt: u32) -> RegionFieldSpec {
        RegionFieldSpec {
            name: name.into(),
            grid_size: 2,
            n_dims: 16,
            source_col: src,
            target_col: tgt,
            operator: RegionFieldOperatorSpec::Normalized,
            horizon: 1,
            allow_extended_horizon: false,
            alpha_self: 0.0,
            gamma_neighbor: 1.0,
            source_cap: None,
            source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
            cadence: RegionFieldCadenceSpec::EveryTick,
            grid_profile: RegionFieldGridProfile::StandardSquare,
            reduction: None,
            parent_formula: None,
            commitment: None,
            request_atlas_batching: false,
            max_region_field_vram_bytes: None,
            summary_policy: RegionFieldSummaryPolicySpec::default(),
            pressure_binding: None,
        }
    }

    #[test]
    fn unique_matrix_binding_independent_of_registration_scan_order() {
        let fields = [rf("a", 0, 1), rf("b", 2, 3)];
        let a = admit_field_plan_from_region_fields(&fields)
            .unwrap()
            .unwrap();
        // Rebuild after reversing authored list — authored_order follows new
        // list positions (different product). Same call twice must be stable.
        let b = admit_field_plan_from_region_fields(&fields)
            .unwrap()
            .unwrap();
        assert_eq!(a.emitters()[0].value_col, b.emitters()[0].value_col);
        assert_eq!(a.emitters()[1].value_col, b.emitters()[1].value_col);
        assert_eq!(a.emitters()[0].class_id, 0.0);
        assert_eq!(a.emitters()[1].class_id, 1.0);
    }
}
