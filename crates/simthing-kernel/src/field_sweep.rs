//! FIELD-SWEEP-N4-PARITY-0 — one proof-admitted EML field sweep over the existing
//! AccumulatorOp input-list gather representation.
//!
//! Algebra stays in authored `map_program` / `fold_program` / `post_program` data.
//! The executor has one fixed linear fold and never branches on a field kind,
//! algebra identity, or operator identity.
//!
//! Raw semantic identities cannot enter a field-sweep registration:
//!
//! ```compile_fail
//! use simthing_gpu::FieldSweepRegistrationRequest;
//!
//! fn field_sweep_registration_rejects_raw_column(
//!     mut request: FieldSweepRegistrationRequest,
//! ) {
//!     request.output = 0u32;
//! }
//! ```
//!
//! ```compile_fail
//! use simthing_gpu::FieldEmlContext;
//!
//! fn field_sweep_context_rejects_raw_slot(mut context: FieldEmlContext) {
//!     context.target_slot = 0u32;
//! }
//! ```

use std::collections::{BTreeMap, BTreeSet};
use std::sync::mpsc;

use bytemuck::{Pod, Zeroable};
use simthing_core::{
    eml_opcode, ColumnIndex, EmlNodeGpu, InputSpec, SlotIndex, EML_STACK_MAX, MAX_EML_TREE_NODES,
};
use thiserror::Error;
use wgpu::util::DeviceExt;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePassDescriptor, ComputePipeline, ComputePipelineDescriptor,
    MapMode, PipelineLayoutDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages,
};

use crate::accumulator_op::{AccumulatorInputGpu, InputListRange};
use crate::context::GpuContext;
use crate::eml_opcode_gate::{opcode_in_closed_vocabulary, OpcodeGateError};
use crate::wgsl_encode::{column_from_wire, encode_column};

pub const FIELD_SWEEP_WORKGROUP_SIZE: u32 = 64;
pub const FIELD_SWEEP_LEGACY_STACK_SLOTS: u32 = 32;
pub const FIELD_SWEEP_LEGACY_PROGRAM_NODES: u32 = 32;

/// Field-only `PARAM` indices. The five edge-context members are stable; mapped
/// and folded are stage results carried without introducing semantic opcodes.
pub mod field_param {
    pub const TARGET_SLOT: u32 = 0;
    pub const NEIGHBOR_SLOT: u32 = 1;
    pub const ACCUMULATOR: u32 = 2;
    pub const EDGE_SCALAR: u32 = 3;
    pub const DT: u32 = 4;
    pub const MAPPED: u32 = 5;
    pub const FOLDED: u32 = 6;
    /// Kernel-private per-slot transient written by an earlier registration in
    /// the same admitted session chain.
    pub const TARGET_TRANSIENT: u32 = 7;
    /// Kernel-private transient for the current authored neighbor.
    pub const NEIGHBOR_TRANSIENT: u32 = 8;
    pub const MAX: u32 = NEIGHBOR_TRANSIENT;
}

/// One authored cardinal offset retained for the graduated N4 compatibility
/// door. Rung 5.6 lowers it into [`GridOffset`] data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridN4Offset {
    pub dx: i8,
    pub dy: i8,
}

impl GridN4Offset {
    pub const fn new(dx: i8, dy: i8) -> Self {
        Self { dx, dy }
    }
}

pub const GRID_N4_WENS: [GridN4Offset; 4] = [
    GridN4Offset::new(-1, 0),
    GridN4Offset::new(1, 0),
    GridN4Offset::new(0, -1),
    GridN4Offset::new(0, 1),
];

pub const GRID_N4_NSEW: [GridN4Offset; 4] = [
    GridN4Offset::new(0, -1),
    GridN4Offset::new(0, 1),
    GridN4Offset::new(1, 0),
    GridN4Offset::new(-1, 0),
];

/// One authored weighted grid offset. The weight is carried to field EML as
/// `EDGE_SCALAR`; diagonal and radius weights are therefore explicit data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridOffset {
    dx: i32,
    dy: i32,
    weight: f32,
}

impl GridOffset {
    pub const fn new(dx: i32, dy: i32, weight: f32) -> Self {
        Self { dx, dy, weight }
    }

    pub fn dx(self) -> i32 {
        self.dx
    }

    pub fn dy(self) -> i32 {
        self.dy
    }

    pub fn weight(self) -> f32 {
        self.weight
    }
}

/// Canonical N4 preset with an explicitly authored cardinal weight and order.
pub fn grid_n4_offsets(
    order: [GridN4Offset; 4],
    cardinal_weight: f32,
) -> Result<Vec<GridOffset>, FieldSweepAdmissionError> {
    validate_edge_weight(cardinal_weight)?;
    Ok(order
        .into_iter()
        .map(|offset| GridOffset::new(i32::from(offset.dx), i32::from(offset.dy), cardinal_weight))
        .collect())
}

/// Canonical N8 preset. Diagonal weight is a required authored argument; the
/// generator never silently chooses a Chebyshev or Euclidean metric.
pub fn grid_n8_offsets(
    cardinal_weight: f32,
    diagonal_weight: f32,
) -> Result<Vec<GridOffset>, FieldSweepAdmissionError> {
    validate_edge_weight(cardinal_weight)?;
    validate_edge_weight(diagonal_weight)?;
    Ok(vec![
        GridOffset::new(0, -1, cardinal_weight),
        GridOffset::new(0, 1, cardinal_weight),
        GridOffset::new(1, 0, cardinal_weight),
        GridOffset::new(-1, 0, cardinal_weight),
        GridOffset::new(-1, -1, diagonal_weight),
        GridOffset::new(1, -1, diagonal_weight),
        GridOffset::new(1, 1, diagonal_weight),
        GridOffset::new(-1, 1, diagonal_weight),
    ])
}

/// Canonical Chebyshev radius-r preset. One authored weight is required for
/// each shell `1..=radius`; iteration order is stable row-major within shells.
pub fn grid_radius_offsets(
    radius: u32,
    shell_weights: &[f32],
) -> Result<Vec<GridOffset>, FieldSweepAdmissionError> {
    if radius == 0 {
        return Err(FieldSweepAdmissionError::InvalidGridRadius(radius));
    }
    if shell_weights.len() != radius as usize {
        return Err(FieldSweepAdmissionError::RadiusWeightCount {
            radius,
            actual: shell_weights.len(),
        });
    }
    for &weight in shell_weights {
        validate_edge_weight(weight)?;
    }
    let radius_i32: i32 =
        i32::try_from(radius).map_err(|_| FieldSweepAdmissionError::InvalidGridRadius(radius))?;
    let mut offsets = Vec::new();
    for shell in 1..=radius_i32 {
        let weight = shell_weights[(shell - 1) as usize];
        for dy in -shell..=shell {
            for dx in -shell..=shell {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if dx.abs().max(dy.abs()) == shell {
                    offsets.push(GridOffset::new(dx, dy, weight));
                }
            }
        }
    }
    Ok(offsets)
}

/// One canonical LinkGraph neighbor. Rows must be sorted by slot, deduplicated,
/// and exactly undirected; the driver link compiler already produces that basis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinkGraphNeighbor {
    pub slot: SlotIndex,
    pub weight: f32,
}

/// Public scheduling metadata. Buckets group target slots by degree only; each
/// target's authored neighbor order stays private in [`FieldAdjacency`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldDegreeBucket {
    degree: u32,
    slots: Vec<SlotIndex>,
}

impl FieldDegreeBucket {
    pub fn degree(&self) -> u32 {
        self.degree
    }

    pub fn slots(&self) -> &[SlotIndex] {
        &self.slots
    }
}

/// One proof-bound adjacency axis over the existing input-list gather. Grid
/// generators and LinkGraph lowering converge here before CPU/GPU execution.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldAdjacency {
    grid_shape: Option<(u32, u32)>,
    grid_offsets: Option<Vec<GridOffset>>,
    lists: Vec<Vec<InputSpec>>,
    degree_buckets: Vec<FieldDegreeBucket>,
    schedule: Vec<SlotIndex>,
    order_fingerprint: u64,
    symmetry_fingerprint: Option<u64>,
}

impl FieldAdjacency {
    /// Canonical no-edge adjacency for pointwise field programs. Post programs
    /// still execute once per slot; no synthetic self-edge is introduced.
    pub fn independent_slots(
        slot_count: u32,
        gather_col: ColumnIndex,
    ) -> Result<Self, FieldSweepAdmissionError> {
        Self::link_graph(
            slot_count,
            vec![Vec::new(); slot_count as usize],
            gather_col,
        )
    }

    pub fn grid_n4(
        width: u32,
        height: u32,
        offsets: [GridN4Offset; 4],
        gather_col: ColumnIndex,
    ) -> Result<Self, FieldSweepAdmissionError> {
        let required = BTreeSet::from([(-1, 0), (1, 0), (0, -1), (0, 1)]);
        let actual: BTreeSet<_> = offsets
            .iter()
            .map(|offset| (i32::from(offset.dx), i32::from(offset.dy)))
            .collect();
        if actual != required || actual.len() != offsets.len() {
            return Err(FieldSweepAdmissionError::NotGridN4);
        }
        Self::grid_offsets(width, height, grid_n4_offsets(offsets, 1.0)?, gather_col)
    }

    pub fn grid_n8(
        width: u32,
        height: u32,
        cardinal_weight: f32,
        diagonal_weight: f32,
        gather_col: ColumnIndex,
    ) -> Result<Self, FieldSweepAdmissionError> {
        Self::grid_offsets(
            width,
            height,
            grid_n8_offsets(cardinal_weight, diagonal_weight)?,
            gather_col,
        )
    }

    pub fn grid_radius(
        width: u32,
        height: u32,
        radius: u32,
        shell_weights: &[f32],
        gather_col: ColumnIndex,
    ) -> Result<Self, FieldSweepAdmissionError> {
        Self::grid_offsets(
            width,
            height,
            grid_radius_offsets(radius, shell_weights)?,
            gather_col,
        )
    }

    pub fn grid_offsets(
        width: u32,
        height: u32,
        offsets: Vec<GridOffset>,
        gather_col: ColumnIndex,
    ) -> Result<Self, FieldSweepAdmissionError> {
        if width == 0 || height == 0 {
            return Err(FieldSweepAdmissionError::InvalidDimensions { width, height });
        }
        validate_grid_offsets(&offsets)?;

        let slots = width
            .checked_mul(height)
            .ok_or(FieldSweepAdmissionError::GridSlotCountOverflow { width, height })?;
        let mut lists = Vec::with_capacity(slots as usize);
        for y in 0..height {
            for x in 0..width {
                let mut row = Vec::with_capacity(offsets.len());
                for offset in &offsets {
                    let nx = i64::from(x) + i64::from(offset.dx);
                    let ny = i64::from(y) + i64::from(offset.dy);
                    if nx >= 0 && ny >= 0 && nx < i64::from(width) && ny < i64::from(height) {
                        row.push(InputSpec {
                            slot: SlotIndex::new(ny as u32 * width + nx as u32),
                            col: gather_col,
                            unit_cost: offset.weight,
                        });
                    }
                }
                lists.push(row);
            }
        }
        Self::from_admitted_lists(Some((width, height)), Some(offsets), lists)
    }

    pub fn link_graph(
        slot_count: u32,
        neighbors: Vec<Vec<LinkGraphNeighbor>>,
        gather_col: ColumnIndex,
    ) -> Result<Self, FieldSweepAdmissionError> {
        if slot_count == 0 {
            return Err(FieldSweepAdmissionError::InvalidLinkGraphSlotCount(
                slot_count,
            ));
        }
        if neighbors.len() != slot_count as usize {
            return Err(FieldSweepAdmissionError::LinkGraphRowCount {
                slot_count,
                rows: neighbors.len(),
            });
        }
        for (target, row) in neighbors.iter().enumerate() {
            let mut previous = None;
            for neighbor in row {
                validate_edge_weight(neighbor.weight)?;
                let slot = neighbor.slot.raw();
                if slot >= slot_count {
                    return Err(FieldSweepAdmissionError::InvalidGatherSlot {
                        slot: neighbor.slot,
                        slots: slot_count,
                    });
                }
                if slot == target as u32 {
                    return Err(FieldSweepAdmissionError::LinkGraphSelfEdge {
                        slot: neighbor.slot,
                    });
                }
                if previous.is_some_and(|prior| slot <= prior) {
                    return Err(FieldSweepAdmissionError::LinkGraphNonCanonicalOrder {
                        target: SlotIndex::new(target as u32),
                    });
                }
                previous = Some(slot);
            }
        }
        for (target, row) in neighbors.iter().enumerate() {
            for neighbor in row {
                let reverse_row = &neighbors[neighbor.slot.as_usize()];
                let reverse = reverse_row
                    .binary_search_by_key(&(target as u32), |candidate| candidate.slot.raw())
                    .ok()
                    .map(|index| reverse_row[index]);
                let Some(reverse) = reverse else {
                    return Err(FieldSweepAdmissionError::LinkGraphMissingReverse {
                        from: SlotIndex::new(target as u32),
                        to: neighbor.slot,
                    });
                };
                if reverse.weight.to_bits() != neighbor.weight.to_bits() {
                    return Err(FieldSweepAdmissionError::LinkGraphWeightMismatch {
                        from: SlotIndex::new(target as u32),
                        to: neighbor.slot,
                    });
                }
            }
        }
        let lists = neighbors
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|neighbor| InputSpec {
                        slot: neighbor.slot,
                        col: gather_col,
                        unit_cost: neighbor.weight,
                    })
                    .collect()
            })
            .collect();
        Self::from_admitted_lists(None, None, lists)
    }

    fn from_admitted_lists(
        grid_shape: Option<(u32, u32)>,
        grid_offsets: Option<Vec<GridOffset>>,
        lists: Vec<Vec<InputSpec>>,
    ) -> Result<Self, FieldSweepAdmissionError> {
        let order_fingerprint = fingerprint_order(&lists);
        let symmetry_fingerprint = fingerprint_symmetry(&lists);
        let (degree_buckets, schedule) = build_degree_schedule(&lists);
        Ok(Self {
            grid_shape,
            grid_offsets,
            lists,
            degree_buckets,
            schedule,
            order_fingerprint,
            symmetry_fingerprint,
        })
    }

    pub fn grid_shape(&self) -> Option<(u32, u32)> {
        self.grid_shape
    }

    pub fn grid_offsets_data(&self) -> Option<&[GridOffset]> {
        self.grid_offsets.as_deref()
    }

    pub fn slots(&self) -> u32 {
        self.lists.len() as u32
    }

    pub fn degree_buckets(&self) -> &[FieldDegreeBucket] {
        &self.degree_buckets
    }

    /// Mint a sealed per-node conductance certificate. Admission reads the
    /// adjacency rows directly; scheduling buckets are deliberately absent
    /// from this calculation and cannot set the physical bound.
    pub fn apply_conductance_certificate(
        &self,
        per_node_chi: Vec<f32>,
        admitted_bound: f32,
    ) -> Result<FieldConductanceCertificate, FieldSweepAdmissionError> {
        if per_node_chi.len() != self.lists.len() {
            return Err(FieldSweepAdmissionError::ConductanceChiCount {
                slots: self.slots(),
                actual: per_node_chi.len(),
            });
        }
        if !admitted_bound.is_finite() || admitted_bound < 0.0 {
            return Err(FieldSweepAdmissionError::InvalidConductanceBound(
                admitted_bound,
            ));
        }
        for (slot, (&chi, row)) in per_node_chi.iter().zip(&self.lists).enumerate() {
            if !chi.is_finite() || chi < 0.0 {
                return Err(FieldSweepAdmissionError::InvalidNodeChi {
                    slot: SlotIndex::new(slot as u32),
                    chi,
                });
            }
            let weighted_degree: f32 = row.iter().map(|input| input.unit_cost.abs()).sum();
            let product = chi * weighted_degree;
            if !product.is_finite() || product > admitted_bound {
                return Err(FieldSweepAdmissionError::ConductanceBoundExceeded {
                    slot: SlotIndex::new(slot as u32),
                    chi,
                    weighted_degree,
                    admitted_bound,
                });
            }
        }
        Ok(FieldConductanceCertificate {
            adjacency_order_fingerprint: self.order_fingerprint,
            per_node_chi,
            admitted_bound,
        })
    }

    pub fn apply_canonical_order_proof(&self) -> CanonicalOrderProof {
        CanonicalOrderProof {
            fingerprint: self.order_fingerprint,
        }
    }

    pub fn apply_undirected_symmetry_certificate(
        &self,
    ) -> Result<UndirectedSymmetryCertificate, FieldSweepAdmissionError> {
        self.symmetry_fingerprint
            .map(|fingerprint| UndirectedSymmetryCertificate { fingerprint })
            .ok_or(FieldSweepAdmissionError::AdjacencyNotUndirected)
    }
}

fn validate_edge_weight(weight: f32) -> Result<(), FieldSweepAdmissionError> {
    if !weight.is_finite() || weight == 0.0 {
        return Err(FieldSweepAdmissionError::InvalidEdgeWeight(weight));
    }
    Ok(())
}

fn validate_grid_offsets(offsets: &[GridOffset]) -> Result<(), FieldSweepAdmissionError> {
    if offsets.is_empty() {
        return Err(FieldSweepAdmissionError::EmptyGridOffsets);
    }
    let mut authored = BTreeMap::new();
    for offset in offsets {
        validate_edge_weight(offset.weight)?;
        if offset.dx == 0 && offset.dy == 0 {
            return Err(FieldSweepAdmissionError::GridSelfOffset);
        }
        if authored
            .insert((offset.dx, offset.dy), offset.weight.to_bits())
            .is_some()
        {
            return Err(FieldSweepAdmissionError::DuplicateGridOffset {
                dx: offset.dx,
                dy: offset.dy,
            });
        }
    }
    Ok(())
}

fn fingerprint_order(lists: &[Vec<InputSpec>]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for (target, row) in lists.iter().enumerate() {
        for word in [target as u32, row.len() as u32] {
            for byte in word.to_le_bytes() {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }
        for input in row {
            for word in [
                input.slot.raw(),
                input.col.raw_u32(),
                input.unit_cost.to_bits(),
            ] {
                for byte in word.to_le_bytes() {
                    hash ^= u64::from(byte);
                    hash = hash.wrapping_mul(0x100000001b3);
                }
            }
        }
    }
    hash
}

fn fingerprint_symmetry(lists: &[Vec<InputSpec>]) -> Option<u64> {
    let mut hash = lists.len() as u64;
    for (from, row) in lists.iter().enumerate() {
        for input in row {
            let to = input.slot.as_usize();
            let reverse = lists.get(to).and_then(|reverse_row| {
                reverse_row.iter().find(|candidate| {
                    candidate.slot.as_usize() == from && candidate.col == input.col
                })
            });
            let Some(reverse) = reverse else {
                return None;
            };
            if reverse.unit_cost.to_bits() != input.unit_cost.to_bits() {
                return None;
            }
            if from < to {
                for word in [from as u32, to as u32, input.unit_cost.to_bits()] {
                    hash = hash.rotate_left(9) ^ u64::from(word);
                }
            }
        }
    }
    Some(hash)
}

fn build_degree_schedule(lists: &[Vec<InputSpec>]) -> (Vec<FieldDegreeBucket>, Vec<SlotIndex>) {
    let mut by_degree: BTreeMap<u32, Vec<SlotIndex>> = BTreeMap::new();
    for (slot, row) in lists.iter().enumerate() {
        by_degree
            .entry(row.len() as u32)
            .or_default()
            .push(SlotIndex::new(slot as u32));
    }
    let degree_buckets: Vec<_> = by_degree
        .into_iter()
        .map(|(degree, slots)| FieldDegreeBucket { degree, slots })
        .collect();
    let schedule = degree_buckets
        .iter()
        .flat_map(|bucket| bucket.slots.iter().copied())
        .collect();
    (degree_buckets, schedule)
}

/// Sealed proof that an authored adjacency has a fixed, registration-bound
/// linear neighbor order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanonicalOrderProof {
    fingerprint: u64,
}

/// Sealed certificate that every admitted directed edge has its reverse.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UndirectedSymmetryCertificate {
    fingerprint: u64,
}

/// Sealed proof of `chi_i * sum_j(abs(c_ij)) <= admitted_bound` for every
/// node. The retained values make the certificate auditable after admission.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldConductanceCertificate {
    adjacency_order_fingerprint: u64,
    per_node_chi: Vec<f32>,
    admitted_bound: f32,
}

impl FieldConductanceCertificate {
    pub fn per_node_chi(&self) -> &[f32] {
        &self.per_node_chi
    }

    pub fn admitted_bound(&self) -> f32 {
        self.admitted_bound
    }
}

/// Sealed law proof. Execution does not inspect this value; admission consumes
/// it before a registration can exist.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldLawProof {
    required_symmetry_fingerprint: Option<u64>,
    conductance_certificate: Option<FieldConductanceCertificate>,
}

impl FieldLawProof {
    pub fn apply_non_conservative() -> Self {
        Self {
            required_symmetry_fingerprint: None,
            conductance_certificate: None,
        }
    }

    pub fn apply_conservative(
        symmetry: UndirectedSymmetryCertificate,
        conductance: FieldConductanceCertificate,
    ) -> Self {
        Self {
            required_symmetry_fingerprint: Some(symmetry.fingerprint),
            conductance_certificate: Some(conductance),
        }
    }
}

/// Admitted destination for one sweep result. `Transient` is a kernel-private
/// per-slot lane: it is never part of the authored matrix and cannot corrupt
/// an unrelated property column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldSweepOutput {
    Matrix(ColumnIndex),
    Transient,
}

/// Sealed witness that a compatible earlier registration produces the
/// kernel-private transient lane used by a later registration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldTransientCertificate {
    adjacency_order_fingerprint: u64,
    n_dims: u32,
}

/// Untrusted request surface. Only the one legacy fixed-32 class is admitted
/// until rung 5.7.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldSweepResourceClassRequest {
    pub stack_slots: u32,
    pub max_program_nodes: u32,
}

impl Default for FieldSweepResourceClassRequest {
    fn default() -> Self {
        Self {
            stack_slots: FIELD_SWEEP_LEGACY_STACK_SLOTS,
            max_program_nodes: FIELD_SWEEP_LEGACY_PROGRAM_NODES,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldSweepResourceClass {
    stack_slots: u32,
    max_program_nodes: u32,
}

impl FieldSweepResourceClass {
    pub fn stack_slots(self) -> u32 {
        self.stack_slots
    }

    pub fn max_program_nodes(self) -> u32 {
        self.max_program_nodes
    }
}

#[derive(Clone, Debug)]
pub struct FieldSweepRegistrationRequest {
    pub adjacency: FieldAdjacency,
    pub n_dims: u32,
    pub output: FieldSweepOutput,
    pub map_program: Vec<EmlNodeGpu>,
    pub fold_program: Vec<EmlNodeGpu>,
    pub identity_bits: u32,
    pub post_program: Vec<EmlNodeGpu>,
    pub field_law_proof: Option<FieldLawProof>,
    pub transient_read_proof: Option<FieldTransientCertificate>,
    pub canonical_order_proof: Option<CanonicalOrderProof>,
    pub resource_class: FieldSweepResourceClassRequest,
    pub dt: f32,
}

/// Proof-present production registration. Every field is immutable after
/// admission; no free constructor exists.
#[derive(Clone, Debug)]
pub struct FieldSweepRegistration {
    adjacency: FieldAdjacency,
    n_dims: u32,
    output: FieldSweepOutput,
    map_program: Vec<EmlNodeGpu>,
    fold_program: Vec<EmlNodeGpu>,
    identity_bits: u32,
    post_program: Vec<EmlNodeGpu>,
    field_law_proof: FieldLawProof,
    transient_read_proof: Option<FieldTransientCertificate>,
    canonical_order_proof: CanonicalOrderProof,
    resource_class: FieldSweepResourceClass,
    dt: f32,
}

impl FieldSweepRegistration {
    pub fn slots(&self) -> u32 {
        self.adjacency.slots()
    }

    pub fn n_dims(&self) -> u32 {
        self.n_dims
    }

    pub fn output(&self) -> FieldSweepOutput {
        self.output
    }

    pub fn apply_transient_certificate(
        &self,
    ) -> Result<FieldTransientCertificate, FieldSweepAdmissionError> {
        if self.output != FieldSweepOutput::Transient {
            return Err(FieldSweepAdmissionError::TransientCertificateFromMatrixOutput);
        }
        Ok(FieldTransientCertificate {
            adjacency_order_fingerprint: self.adjacency.order_fingerprint,
            n_dims: self.n_dims,
        })
    }

    pub fn adjacency(&self) -> &FieldAdjacency {
        &self.adjacency
    }

    pub fn map_program(&self) -> &[EmlNodeGpu] {
        &self.map_program
    }

    pub fn fold_program(&self) -> &[EmlNodeGpu] {
        &self.fold_program
    }

    pub fn post_program(&self) -> &[EmlNodeGpu] {
        &self.post_program
    }

    pub fn identity_bits(&self) -> u32 {
        self.identity_bits
    }

    pub fn resource_class(&self) -> FieldSweepResourceClass {
        self.resource_class
    }

    pub fn dt(&self) -> f32 {
        self.dt
    }
}

pub fn apply_field_sweep_registration(
    request: FieldSweepRegistrationRequest,
) -> Result<FieldSweepRegistration, FieldSweepAdmissionError> {
    if request.n_dims == 0 {
        return Err(FieldSweepAdmissionError::InvalidDims(request.n_dims));
    }
    if let FieldSweepOutput::Matrix(output_col) = request.output {
        if output_col.raw_u32() >= request.n_dims {
            return Err(FieldSweepAdmissionError::InvalidOutputColumn {
                output_col,
                n_dims: request.n_dims,
            });
        }
    }
    for input in request.adjacency.lists.iter().flatten() {
        if input.slot.raw() >= request.adjacency.slots() {
            return Err(FieldSweepAdmissionError::InvalidGatherSlot {
                slot: input.slot,
                slots: request.adjacency.slots(),
            });
        }
        if input.col.raw_u32() >= request.n_dims {
            return Err(FieldSweepAdmissionError::InvalidGatherColumn {
                col: input.col,
                n_dims: request.n_dims,
            });
        }
    }
    let field_law_proof = request
        .field_law_proof
        .ok_or(FieldSweepAdmissionError::MissingFieldLawProof)?;
    let canonical_order_proof = request
        .canonical_order_proof
        .ok_or(FieldSweepAdmissionError::MissingCanonicalOrderProof)?;
    if canonical_order_proof.fingerprint != request.adjacency.order_fingerprint {
        return Err(FieldSweepAdmissionError::CanonicalOrderProofMismatch);
    }
    if let Some(required) = field_law_proof.required_symmetry_fingerprint {
        if Some(required) != request.adjacency.symmetry_fingerprint {
            return Err(FieldSweepAdmissionError::UndirectedSymmetryCertificateMismatch);
        }
        let conductance = field_law_proof
            .conductance_certificate
            .as_ref()
            .ok_or(FieldSweepAdmissionError::MissingConductanceCertificate)?;
        if conductance.adjacency_order_fingerprint != request.adjacency.order_fingerprint {
            return Err(FieldSweepAdmissionError::ConductanceCertificateMismatch);
        }
    }
    if request.resource_class != FieldSweepResourceClassRequest::default() {
        return Err(FieldSweepAdmissionError::UnsupportedResourceClass {
            stack_slots: request.resource_class.stack_slots,
            max_program_nodes: request.resource_class.max_program_nodes,
        });
    }
    if !request.dt.is_finite() || request.dt < 0.0 {
        return Err(FieldSweepAdmissionError::InvalidDt(request.dt));
    }

    validate_field_program(
        "map",
        &request.map_program,
        request.n_dims,
        FieldProgramContext::Edge,
    )?;
    validate_field_program(
        "fold",
        &request.fold_program,
        request.n_dims,
        FieldProgramContext::Edge,
    )?;
    validate_field_program(
        "post",
        &request.post_program,
        request.n_dims,
        FieldProgramContext::TargetOnly,
    )?;

    let reads_transient = [
        &request.map_program,
        &request.fold_program,
        &request.post_program,
    ]
    .into_iter()
    .flatten()
    .any(|node| {
        node.opcode == eml_opcode::PARAM
            && matches!(
                node.a,
                field_param::TARGET_TRANSIENT | field_param::NEIGHBOR_TRANSIENT
            )
    });
    if reads_transient {
        let proof = request
            .transient_read_proof
            .ok_or(FieldSweepAdmissionError::MissingTransientReadProof)?;
        if proof.adjacency_order_fingerprint != request.adjacency.order_fingerprint
            || proof.n_dims != request.n_dims
        {
            return Err(FieldSweepAdmissionError::TransientReadProofMismatch);
        }
    } else if request.transient_read_proof.is_some() {
        return Err(FieldSweepAdmissionError::UnusedTransientReadProof);
    }
    if reads_transient && request.output == FieldSweepOutput::Transient {
        return Err(FieldSweepAdmissionError::TransientReadWriteAliasing);
    }

    Ok(FieldSweepRegistration {
        adjacency: request.adjacency,
        n_dims: request.n_dims,
        output: request.output,
        map_program: request.map_program,
        fold_program: request.fold_program,
        identity_bits: request.identity_bits,
        post_program: request.post_program,
        field_law_proof,
        transient_read_proof: request.transient_read_proof,
        canonical_order_proof,
        resource_class: FieldSweepResourceClass {
            stack_slots: FIELD_SWEEP_LEGACY_STACK_SLOTS,
            max_program_nodes: FIELD_SWEEP_LEGACY_PROGRAM_NODES,
        },
        dt: request.dt,
    })
}

#[derive(Clone, Copy)]
enum FieldProgramContext {
    Edge,
    TargetOnly,
}

fn validate_field_program(
    name: &'static str,
    nodes: &[EmlNodeGpu],
    n_dims: u32,
    context: FieldProgramContext,
) -> Result<(), FieldSweepAdmissionError> {
    if nodes.is_empty() {
        return Err(FieldSweepAdmissionError::EmptyProgram { name });
    }
    if nodes.len() as u32 > MAX_EML_TREE_NODES {
        return Err(FieldSweepAdmissionError::ProgramTooLarge {
            name,
            nodes: nodes.len() as u32,
            max: MAX_EML_TREE_NODES,
        });
    }
    let mut depth = 0u32;
    let mut saw_return = false;
    for (index, node) in nodes.iter().enumerate() {
        if !opcode_in_closed_vocabulary(node.opcode) {
            return Err(FieldSweepAdmissionError::OpcodeGate(
                OpcodeGateError::UnwhitelistedOpcode {
                    opcode: node.opcode,
                },
            ));
        }
        match node.opcode {
            eml_opcode::LITERAL_F32 | eml_opcode::TARGET_VALUE => depth += 1,
            eml_opcode::NEIGHBOR_VALUE => {
                if matches!(context, FieldProgramContext::TargetOnly) {
                    return Err(FieldSweepAdmissionError::MalformedEdgeContext {
                        name,
                        node: index as u32,
                    });
                }
                depth += 1;
            }
            eml_opcode::PARAM => {
                if matches!(context, FieldProgramContext::TargetOnly)
                    && matches!(
                        node.a,
                        field_param::NEIGHBOR_SLOT | field_param::NEIGHBOR_TRANSIENT
                    )
                {
                    return Err(FieldSweepAdmissionError::MalformedEdgeContext {
                        name,
                        node: index as u32,
                    });
                }
                if node.a > field_param::MAX {
                    return Err(FieldSweepAdmissionError::FieldParamOutOfRange {
                        name,
                        index: node.a,
                    });
                }
                depth += 1;
            }
            eml_opcode::SLOT_VALUE => {
                return Err(FieldSweepAdmissionError::SlotValueAmbiguousInFieldProgram { name });
            }
            eml_opcode::NEG
            | eml_opcode::CLAMP_BOUNDED
            | eml_opcode::CLAMP_FLOORED
            | eml_opcode::ABS
            | eml_opcode::FLOOR => {
                if depth < 1 {
                    return Err(FieldSweepAdmissionError::StackUnderflow { name });
                }
            }
            eml_opcode::ADD
            | eml_opcode::SUB
            | eml_opcode::MUL
            | eml_opcode::DIV
            | eml_opcode::MIN
            | eml_opcode::MAX
            | eml_opcode::CMP_LT
            | eml_opcode::CMP_LE
            | eml_opcode::CMP_GT
            | eml_opcode::CMP_GE
            | eml_opcode::CMP_EQ => {
                if depth < 2 {
                    return Err(FieldSweepAdmissionError::StackUnderflow { name });
                }
                if node.opcode == eml_opcode::DIV && node.flags & 1 == 0 {
                    return Err(FieldSweepAdmissionError::UnsafeDivision { name });
                }
                depth -= 1;
            }
            eml_opcode::SELECT => {
                if depth < 3 {
                    return Err(FieldSweepAdmissionError::StackUnderflow { name });
                }
                depth -= 2;
            }
            eml_opcode::RETURN_TOP => {
                if depth < 1 || index + 1 != nodes.len() {
                    return Err(FieldSweepAdmissionError::MalformedReturn { name });
                }
                saw_return = true;
            }
            _ => unreachable!("closed vocabulary is exhaustive for field admission"),
        }
        if matches!(
            node.opcode,
            eml_opcode::TARGET_VALUE | eml_opcode::NEIGHBOR_VALUE
        ) && node.a >= n_dims
        {
            return Err(FieldSweepAdmissionError::MalformedEdgeColumn {
                name,
                col: node.a,
                n_dims,
            });
        }
        if depth > EML_STACK_MAX {
            return Err(FieldSweepAdmissionError::StackDepthExceeded {
                name,
                depth,
                max: EML_STACK_MAX,
            });
        }
    }
    if !saw_return || depth != 1 {
        return Err(FieldSweepAdmissionError::MalformedReturn { name });
    }
    Ok(())
}

#[derive(Clone, Copy, Debug)]
pub struct FieldEmlContext {
    pub target_slot: SlotIndex,
    pub neighbor_slot: Option<SlotIndex>,
    pub accumulator: f32,
    pub edge_scalar: f32,
    pub dt: f32,
    pub mapped: f32,
    pub folded: f32,
    pub target_transient: f32,
    pub neighbor_transient: Option<f32>,
}

pub fn eval_field_eml_cpu(
    nodes: &[EmlNodeGpu],
    context: FieldEmlContext,
    values: &[f32],
    n_dims: u32,
) -> Result<f32, FieldSweepExecutionError> {
    let mut stack = [0.0f32; FIELD_SWEEP_LEGACY_STACK_SLOTS as usize];
    let mut sp = 0usize;
    for node in nodes {
        match node.opcode {
            eml_opcode::LITERAL_F32 => push(&mut stack, &mut sp, f32::from_bits(node.a))?,
            eml_opcode::TARGET_VALUE => {
                let value = read_cell(
                    values,
                    context.target_slot,
                    column_from_wire(node.a),
                    n_dims,
                )?;
                push(&mut stack, &mut sp, value)?;
            }
            eml_opcode::NEIGHBOR_VALUE => {
                let neighbor_slot = context
                    .neighbor_slot
                    .ok_or(FieldSweepExecutionError::MissingNeighborContext)?;
                let value = read_cell(values, neighbor_slot, column_from_wire(node.a), n_dims)?;
                push(&mut stack, &mut sp, value)?;
            }
            eml_opcode::PARAM => {
                let value = match node.a {
                    field_param::TARGET_SLOT => context.target_slot.raw() as f32,
                    field_param::NEIGHBOR_SLOT => context
                        .neighbor_slot
                        .map(|slot| slot.raw() as f32)
                        .unwrap_or(f32::NAN),
                    field_param::ACCUMULATOR => context.accumulator,
                    field_param::EDGE_SCALAR => context.edge_scalar,
                    field_param::DT => context.dt,
                    field_param::MAPPED => context.mapped,
                    field_param::FOLDED => context.folded,
                    field_param::TARGET_TRANSIENT => context.target_transient,
                    field_param::NEIGHBOR_TRANSIENT => context
                        .neighbor_transient
                        .ok_or(FieldSweepExecutionError::MissingNeighborContext)?,
                    _ => return Err(FieldSweepExecutionError::InvalidFieldParam(node.a)),
                };
                push(&mut stack, &mut sp, value)?;
            }
            eml_opcode::NEG => stack[sp - 1] = -stack[sp - 1],
            eml_opcode::CLAMP_BOUNDED => {
                stack[sp - 1] = stack[sp - 1].clamp(f32::from_bits(node.a), f32::from_bits(node.b));
            }
            eml_opcode::CLAMP_FLOORED => {
                stack[sp - 1] = stack[sp - 1].max(f32::from_bits(node.a));
            }
            eml_opcode::ABS => stack[sp - 1] = stack[sp - 1].abs(),
            eml_opcode::FLOOR => stack[sp - 1] = stack[sp - 1].floor(),
            eml_opcode::SELECT => {
                let false_value = stack[sp - 1];
                let true_value = stack[sp - 2];
                let condition = stack[sp - 3] != 0.0;
                stack[sp - 3] = if condition { true_value } else { false_value };
                sp -= 2;
            }
            eml_opcode::RETURN_TOP => return Ok(stack[sp - 1]),
            opcode => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = apply_binary(opcode, lhs, rhs);
                sp -= 1;
            }
        }
    }
    Err(FieldSweepExecutionError::ProgramDidNotReturn)
}

fn apply_binary(opcode: u32, lhs: f32, rhs: f32) -> f32 {
    if opcode == eml_opcode::ADD {
        lhs + rhs
    } else if opcode == eml_opcode::SUB {
        lhs - rhs
    } else if opcode == eml_opcode::MUL {
        lhs * rhs
    } else if opcode == eml_opcode::DIV {
        lhs / rhs
    } else if opcode == eml_opcode::MIN {
        lhs.min(rhs)
    } else if opcode == eml_opcode::MAX {
        lhs.max(rhs)
    } else if opcode == eml_opcode::CMP_LT {
        if lhs < rhs {
            1.0
        } else {
            0.0
        }
    } else if opcode == eml_opcode::CMP_LE {
        if lhs <= rhs {
            1.0
        } else {
            0.0
        }
    } else if opcode == eml_opcode::CMP_GT {
        if lhs > rhs {
            1.0
        } else {
            0.0
        }
    } else if opcode == eml_opcode::CMP_GE {
        if lhs >= rhs {
            1.0
        } else {
            0.0
        }
    } else {
        if lhs == rhs {
            1.0
        } else {
            0.0
        }
    }
}

fn push(
    stack: &mut [f32; FIELD_SWEEP_LEGACY_STACK_SLOTS as usize],
    sp: &mut usize,
    value: f32,
) -> Result<(), FieldSweepExecutionError> {
    if *sp >= stack.len() {
        return Err(FieldSweepExecutionError::StackOverflow);
    }
    stack[*sp] = value;
    *sp += 1;
    Ok(())
}

fn read_cell(
    values: &[f32],
    slot: SlotIndex,
    col: ColumnIndex,
    n_dims: u32,
) -> Result<f32, FieldSweepExecutionError> {
    if col.raw_u32() >= n_dims {
        return Err(FieldSweepExecutionError::MalformedEdgeContext {
            slot: slot.raw(),
            col: col.raw_u32(),
            n_dims,
            values_len: values.len(),
        });
    }
    let index = slot.as_usize() * n_dims as usize + col.raw();
    values
        .get(index)
        .copied()
        .ok_or(FieldSweepExecutionError::MalformedEdgeContext {
            slot: slot.raw(),
            col: col.raw_u32(),
            n_dims,
            values_len: values.len(),
        })
}

pub fn execute_field_sweep_cpu(
    values: &[f32],
    registration: &FieldSweepRegistration,
) -> Result<Vec<f32>, FieldSweepExecutionError> {
    let mut transient = vec![0.0; registration.slots() as usize];
    let mut transient_initialized = false;
    execute_field_sweep_cpu_with_state(
        values,
        registration,
        &mut transient,
        &mut transient_initialized,
        &registration.adjacency.schedule,
    )
}

/// Execute a sequence while retaining the kernel-private transient lane
/// between compatible registrations.
pub fn execute_field_sweep_cpu_chain(
    values: &[f32],
    registrations: &[FieldSweepRegistration],
) -> Result<Vec<f32>, FieldSweepExecutionError> {
    let Some(first) = registrations.first() else {
        return Ok(values.to_vec());
    };
    let mut current = values.to_vec();
    let mut transient = vec![0.0; first.slots() as usize];
    let mut transient_initialized = false;
    for registration in registrations {
        if registration.slots() != first.slots() || registration.n_dims != first.n_dims {
            return Err(FieldSweepExecutionError::RegistrationBindingChanged);
        }
        current = execute_field_sweep_cpu_with_state(
            &current,
            registration,
            &mut transient,
            &mut transient_initialized,
            &registration.adjacency.schedule,
        )?;
    }
    Ok(current)
}

/// Independent reference execution in natural target-slot order. This is a
/// parity judge for the degree-bucket schedule, never a production scheduler.
pub fn execute_field_sweep_cpu_natural_order(
    values: &[f32],
    registration: &FieldSweepRegistration,
) -> Result<Vec<f32>, FieldSweepExecutionError> {
    let natural_order: Vec<_> = (0..registration.slots()).map(SlotIndex::new).collect();
    let mut transient = vec![0.0; registration.slots() as usize];
    let mut transient_initialized = false;
    execute_field_sweep_cpu_with_state(
        values,
        registration,
        &mut transient,
        &mut transient_initialized,
        &natural_order,
    )
}

fn execute_field_sweep_cpu_with_state(
    values: &[f32],
    registration: &FieldSweepRegistration,
    transient: &mut [f32],
    transient_initialized: &mut bool,
    target_order: &[SlotIndex],
) -> Result<Vec<f32>, FieldSweepExecutionError> {
    let required = registration.slots() as usize * registration.n_dims as usize;
    if values.len() != required {
        return Err(FieldSweepExecutionError::ValuesLength {
            actual: values.len(),
            required,
        });
    }
    if transient.len() != registration.slots() as usize {
        return Err(FieldSweepExecutionError::TransientLength {
            actual: transient.len(),
            required: registration.slots() as usize,
        });
    }
    if registration.transient_read_proof.is_some() && !*transient_initialized {
        return Err(FieldSweepExecutionError::TransientNotInitialized);
    }
    let mut output = values.to_vec();
    for &target_slot in target_order {
        let list = &registration.adjacency.lists[target_slot.as_usize()];
        let mut accumulator = f32::from_bits(registration.identity_bits);
        for input in list {
            let base_context = FieldEmlContext {
                target_slot,
                neighbor_slot: Some(input.slot),
                accumulator,
                edge_scalar: input.unit_cost,
                dt: registration.dt,
                mapped: 0.0,
                folded: 0.0,
                target_transient: transient[target_slot.as_usize()],
                neighbor_transient: Some(transient[input.slot.as_usize()]),
            };
            let mapped = eval_field_eml_cpu(
                &registration.map_program,
                base_context,
                values,
                registration.n_dims,
            )?;
            accumulator = eval_field_eml_cpu(
                &registration.fold_program,
                FieldEmlContext {
                    mapped,
                    ..base_context
                },
                values,
                registration.n_dims,
            )?;
        }
        let written = eval_field_eml_cpu(
            &registration.post_program,
            FieldEmlContext {
                target_slot,
                neighbor_slot: None,
                accumulator,
                edge_scalar: 0.0,
                dt: registration.dt,
                mapped: 0.0,
                folded: accumulator,
                target_transient: transient[target_slot.as_usize()],
                neighbor_transient: None,
            },
            values,
            registration.n_dims,
        )?;
        match registration.output {
            FieldSweepOutput::Matrix(output_col) => {
                let output_index =
                    target_slot.as_usize() * registration.n_dims as usize + output_col.raw();
                output[output_index] = written;
            }
            FieldSweepOutput::Transient => transient[target_slot.as_usize()] = written,
        }
    }
    if registration.output == FieldSweepOutput::Transient {
        *transient_initialized = true;
    }
    Ok(output)
}

pub fn execute_field_sweep_cpu_iterations(
    values: &[f32],
    registration: &FieldSweepRegistration,
    iterations: u32,
) -> Result<Vec<f32>, FieldSweepExecutionError> {
    if iterations == 0 {
        return Err(FieldSweepExecutionError::InvalidIterations(iterations));
    }
    let mut current = values.to_vec();
    let mut transient = vec![0.0; registration.slots() as usize];
    let mut transient_initialized = false;
    for _ in 0..iterations {
        current = execute_field_sweep_cpu_with_state(
            &current,
            registration,
            &mut transient,
            &mut transient_initialized,
            &registration.adjacency.schedule,
        )?;
    }
    Ok(current)
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct FieldRangeGpu {
    offset: u32,
    count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct FieldSweepParamsGpu {
    n_slots: u32,
    n_dims: u32,
    output_col: u32,
    map_offset: u32,
    map_count: u32,
    fold_offset: u32,
    fold_count: u32,
    post_offset: u32,
    post_count: u32,
    identity_bits: u32,
    dt_bits: u32,
    schedule_offset: u32,
    schedule_count: u32,
    output_mode: u32,
    _pad1: u32,
}

#[derive(Clone, Debug, PartialEq)]
struct FieldSweepSessionBinding {
    adjacency: FieldAdjacency,
    n_slots: u32,
    n_dims: u32,
}

impl FieldSweepSessionBinding {
    fn from_registration(registration: &FieldSweepRegistration) -> Self {
        Self {
            adjacency: registration.adjacency.clone(),
            n_slots: registration.slots(),
            n_dims: registration.n_dims,
        }
    }

    fn accepts(&self, registration: &FieldSweepRegistration) -> bool {
        self.n_slots == registration.slots()
            && self.n_dims == registration.n_dims
            && self.adjacency == registration.adjacency
    }
}

/// Kernel-owned generic field executor. It owns the resolved ping-pong buffers;
/// callers can upload values and receive copied readback, never raw handles.
pub struct FieldSweepSession {
    pipeline: ComputePipeline,
    layout: BindGroupLayout,
    values_a: Buffer,
    values_b: Buffer,
    ranges: Buffer,
    inputs: Buffer,
    nodes: Buffer,
    schedule: Buffer,
    transient: Buffer,
    params: Buffer,
    binding: FieldSweepSessionBinding,
    values_len: usize,
    read_a: bool,
    transient_initialized: bool,
}

impl FieldSweepSession {
    pub fn new(
        ctx: &GpuContext,
        registration: &FieldSweepRegistration,
    ) -> Result<Self, FieldSweepExecutionError> {
        let shader = ctx.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("field_sweep"),
            source: ShaderSource::Wgsl(include_str!("shaders/field_sweep.wgsl").into()),
        });
        let layout = ctx
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("field_sweep_layout"),
                entries: &[
                    storage_entry(0, true),
                    storage_entry(1, false),
                    storage_entry(2, true),
                    storage_entry(3, true),
                    storage_entry(4, true),
                    storage_entry(5, true),
                    uniform_entry(6),
                    storage_entry(7, false),
                ],
            });
        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("field_sweep_pipeline_layout"),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            });
        let pipeline = ctx
            .device
            .create_compute_pipeline(&ComputePipelineDescriptor {
                label: Some("field_sweep_pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader,
                entry_point: "main",
                compilation_options: Default::default(),
                cache: None,
            });

        let values_len = registration.slots() as usize * registration.n_dims as usize;
        let value_bytes = (values_len * std::mem::size_of::<f32>()) as u64;
        let values_a = storage_buffer(&ctx.device, "field_sweep_values_a", value_bytes, true);
        let values_b = storage_buffer(&ctx.device, "field_sweep_values_b", value_bytes, true);
        let transient = storage_buffer(
            &ctx.device,
            "field_sweep_transient",
            registration.slots() as u64 * std::mem::size_of::<f32>() as u64,
            false,
        );

        let (range_rows, flat_inputs) = flatten_gather(&registration.adjacency);
        let ranges = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("field_sweep_ranges"),
                contents: bytemuck::cast_slice(&range_rows),
                usage: BufferUsages::STORAGE,
            });
        let input_bytes = (flat_inputs.len() * std::mem::size_of::<AccumulatorInputGpu>())
            .max(std::mem::size_of::<AccumulatorInputGpu>()) as u64;
        let inputs = storage_buffer(&ctx.device, "field_sweep_inputs", input_bytes, false);
        if !flat_inputs.is_empty() {
            ctx.queue
                .write_buffer(&inputs, 0, bytemuck::cast_slice(&flat_inputs));
        }
        let (flat_nodes, gpu_params) = pack_programs(registration);
        let node_capacity =
            3 * FIELD_SWEEP_LEGACY_PROGRAM_NODES as u64 * std::mem::size_of::<EmlNodeGpu>() as u64;
        let nodes = storage_buffer(&ctx.device, "field_sweep_nodes", node_capacity, false);
        ctx.queue
            .write_buffer(&nodes, 0, bytemuck::cast_slice(&flat_nodes));
        let schedule_slots: Vec<u32> = registration
            .adjacency
            .schedule
            .iter()
            .map(|slot| slot.raw())
            .collect();
        let schedule = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("field_sweep_degree_schedule"),
                contents: bytemuck::cast_slice(&schedule_slots),
                usage: BufferUsages::STORAGE,
            });
        let params = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("field_sweep_params"),
                contents: bytemuck::bytes_of(&gpu_params),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            });

        Ok(Self {
            pipeline,
            layout,
            values_a,
            values_b,
            ranges,
            inputs,
            nodes,
            schedule,
            transient,
            params,
            binding: FieldSweepSessionBinding::from_registration(registration),
            values_len,
            read_a: true,
            transient_initialized: false,
        })
    }

    pub fn upload_values(
        &mut self,
        ctx: &GpuContext,
        values: &[f32],
    ) -> Result<(), FieldSweepExecutionError> {
        if values.len() != self.values_len {
            return Err(FieldSweepExecutionError::ValuesLength {
                actual: values.len(),
                required: self.values_len,
            });
        }
        ctx.queue
            .write_buffer(&self.values_a, 0, bytemuck::cast_slice(values));
        self.read_a = true;
        self.transient_initialized = false;
        Ok(())
    }

    /// Import the exact field prefix from another resident GPU buffer without
    /// a host readback. The source must have `COPY_SRC` usage.
    pub fn upload_values_from_buffer(&mut self, ctx: &GpuContext, source: &Buffer) {
        let byte_len = (self.values_len * std::mem::size_of::<f32>()) as u64;
        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("field_sweep_resident_import"),
            });
        encoder.copy_buffer_to_buffer(source, 0, &self.values_a, 0, byte_len);
        ctx.queue.submit(Some(encoder.finish()));
        self.read_a = true;
        self.transient_initialized = false;
    }

    /// Copy the current ping-pong result into another resident GPU buffer
    /// without exposing the kernel-owned buffer handle. The target must have
    /// `COPY_DST` usage.
    pub fn copy_values_to_buffer(&self, ctx: &GpuContext, target: &Buffer) {
        let source = if self.read_a {
            &self.values_a
        } else {
            &self.values_b
        };
        let byte_len = (self.values_len * std::mem::size_of::<f32>()) as u64;
        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("field_sweep_resident_export"),
            });
        encoder.copy_buffer_to_buffer(source, 0, target, 0, byte_len);
        ctx.queue.submit(Some(encoder.finish()));
    }

    pub fn dispatch(
        &mut self,
        ctx: &GpuContext,
        registration: &FieldSweepRegistration,
        iterations: u32,
    ) -> Result<(), FieldSweepExecutionError> {
        if iterations == 0 {
            return Err(FieldSweepExecutionError::InvalidIterations(iterations));
        }
        if !self.binding.accepts(registration) {
            return Err(FieldSweepExecutionError::RegistrationBindingChanged);
        }
        if registration.transient_read_proof.is_some() && !self.transient_initialized {
            return Err(FieldSweepExecutionError::TransientNotInitialized);
        }
        let (flat_nodes, base_params) = pack_programs(registration);
        ctx.queue
            .write_buffer(&self.nodes, 0, bytemuck::cast_slice(&flat_nodes));

        for _ in 0..iterations {
            let (source, target) = if self.read_a {
                (&self.values_a, &self.values_b)
            } else {
                (&self.values_b, &self.values_a)
            };
            if registration.output != FieldSweepOutput::Transient {
                let byte_len = (self.values_len * std::mem::size_of::<f32>()) as u64;
                let mut encoder = ctx
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor {
                        label: Some("field_sweep_preserve_unrelated_columns"),
                    });
                encoder.copy_buffer_to_buffer(source, 0, target, 0, byte_len);
                ctx.queue.submit(Some(encoder.finish()));
            }
            let mut schedule_offset = 0u32;
            for bucket in &registration.adjacency.degree_buckets {
                let mut gpu_params = base_params;
                gpu_params.schedule_offset = schedule_offset;
                gpu_params.schedule_count = bucket.slots.len() as u32;
                ctx.queue
                    .write_buffer(&self.params, 0, bytemuck::bytes_of(&gpu_params));
                let bind_group = self.bind_group(&ctx.device, source, target);
                let mut encoder = ctx
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor {
                        label: Some("field_sweep_degree_bucket_dispatch"),
                    });
                {
                    let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                        label: Some("field_sweep_degree_bucket_pass"),
                        timestamp_writes: None,
                    });
                    pass.set_pipeline(&self.pipeline);
                    pass.set_bind_group(0, &bind_group, &[]);
                    pass.dispatch_workgroups(
                        gpu_params
                            .schedule_count
                            .div_ceil(FIELD_SWEEP_WORKGROUP_SIZE),
                        1,
                        1,
                    );
                }
                ctx.queue.submit(Some(encoder.finish()));
                schedule_offset += gpu_params.schedule_count;
            }
            if registration.output != FieldSweepOutput::Transient {
                self.read_a = !self.read_a;
            }
        }
        if registration.output == FieldSweepOutput::Transient {
            self.transient_initialized = true;
        }
        ctx.device.poll(wgpu::Maintain::Wait);
        Ok(())
    }

    pub fn readback(&self, ctx: &GpuContext) -> Result<Vec<f32>, FieldSweepExecutionError> {
        let source = if self.read_a {
            &self.values_a
        } else {
            &self.values_b
        };
        let byte_len = (self.values_len * std::mem::size_of::<f32>()) as u64;
        let staging = ctx.device.create_buffer(&BufferDescriptor {
            label: Some("field_sweep_readback"),
            size: byte_len,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("field_sweep_readback_encoder"),
            });
        encoder.copy_buffer_to_buffer(source, 0, &staging, 0, byte_len);
        ctx.queue.submit(Some(encoder.finish()));
        let slice = staging.slice(..);
        let (sender, receiver) = mpsc::channel();
        slice.map_async(MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
        ctx.device.poll(wgpu::Maintain::Wait);
        receiver
            .recv()
            .map_err(|_| FieldSweepExecutionError::ReadbackChannel)?
            .map_err(FieldSweepExecutionError::ReadbackMap)?;
        let mapped = slice.get_mapped_range();
        let output = bytemuck::cast_slice(&mapped).to_vec();
        drop(mapped);
        staging.unmap();
        Ok(output)
    }

    fn bind_group(&self, device: &wgpu::Device, source: &Buffer, target: &Buffer) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("field_sweep_bind_group"),
            layout: &self.layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: source.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: target.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: self.ranges.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: self.inputs.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: self.nodes.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 5,
                    resource: self.schedule.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 6,
                    resource: self.params.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 7,
                    resource: self.transient.as_entire_binding(),
                },
            ],
        })
    }
}

fn flatten_gather(adjacency: &FieldAdjacency) -> (Vec<FieldRangeGpu>, Vec<AccumulatorInputGpu>) {
    let mut flat = Vec::new();
    let mut ranges = Vec::with_capacity(adjacency.lists.len());
    for list in &adjacency.lists {
        let range = InputListRange {
            offset: flat.len() as u32,
            count: list.len() as u32,
        };
        ranges.push(FieldRangeGpu {
            offset: range.offset,
            count: range.count,
        });
        flat.extend(list.iter().map(|input| AccumulatorInputGpu {
            slot: input.slot.raw(),
            col: encode_column(input.col),
            unit_cost_bits: input.unit_cost.to_bits(),
            flags: 0,
        }));
    }
    (ranges, flat)
}

fn pack_programs(registration: &FieldSweepRegistration) -> (Vec<EmlNodeGpu>, FieldSweepParamsGpu) {
    let mut nodes = Vec::with_capacity(
        registration.map_program.len()
            + registration.fold_program.len()
            + registration.post_program.len(),
    );
    let map_offset = 0;
    nodes.extend_from_slice(&registration.map_program);
    let fold_offset = nodes.len() as u32;
    nodes.extend_from_slice(&registration.fold_program);
    let post_offset = nodes.len() as u32;
    nodes.extend_from_slice(&registration.post_program);
    (
        nodes,
        FieldSweepParamsGpu {
            n_slots: registration.slots(),
            n_dims: registration.n_dims,
            output_col: match registration.output {
                FieldSweepOutput::Matrix(col) => encode_column(col),
                FieldSweepOutput::Transient => 0,
            },
            map_offset,
            map_count: registration.map_program.len() as u32,
            fold_offset,
            fold_count: registration.fold_program.len() as u32,
            post_offset,
            post_count: registration.post_program.len() as u32,
            identity_bits: registration.identity_bits,
            dt_bits: registration.dt.to_bits(),
            schedule_offset: 0,
            schedule_count: registration.slots(),
            output_mode: match registration.output {
                FieldSweepOutput::Matrix(_) => 0,
                FieldSweepOutput::Transient => 1,
            },
            _pad1: 0,
        },
    )
}

fn storage_entry(binding: u32, read_only: bool) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn uniform_entry(binding: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn storage_buffer(device: &wgpu::Device, label: &str, size: u64, copy: bool) -> Buffer {
    let mut usage = BufferUsages::STORAGE | BufferUsages::COPY_DST;
    if copy {
        usage |= BufferUsages::COPY_SRC;
    }
    device.create_buffer(&BufferDescriptor {
        label: Some(label),
        size: size.max(4),
        usage,
        mapped_at_creation: false,
    })
}

#[derive(Clone, Debug, Error, PartialEq)]
pub enum FieldSweepAdmissionError {
    #[error("invalid field adjacency dimensions {width}x{height}")]
    InvalidDimensions { width: u32, height: u32 },
    #[error("rung 5.5 admits exactly the four cardinal GridN4 offsets")]
    NotGridN4,
    #[error("grid radius must be in 1..=i32::MAX (got {0})")]
    InvalidGridRadius(u32),
    #[error("grid radius {radius} requires one authored weight per shell (got {actual})")]
    RadiusWeightCount { radius: u32, actual: usize },
    #[error("field adjacency edge weight must be finite and non-zero (got {0})")]
    InvalidEdgeWeight(f32),
    #[error("weighted GridOffsets must not be empty")]
    EmptyGridOffsets,
    #[error("weighted GridOffsets must not contain the self offset (0, 0)")]
    GridSelfOffset,
    #[error("weighted GridOffsets contains duplicate offset ({dx}, {dy})")]
    DuplicateGridOffset { dx: i32, dy: i32 },
    #[error("grid dimensions {width}x{height} overflow the admitted slot index")]
    GridSlotCountOverflow { width: u32, height: u32 },
    #[error("LinkGraph slot count must be > 0 (got {0})")]
    InvalidLinkGraphSlotCount(u32),
    #[error("LinkGraph slot count {slot_count} does not match {rows} neighbor rows")]
    LinkGraphRowCount { slot_count: u32, rows: usize },
    #[error("LinkGraph row contains self edge at slot {slot}")]
    LinkGraphSelfEdge { slot: SlotIndex },
    #[error("LinkGraph row {target} is not strictly sorted and deduplicated")]
    LinkGraphNonCanonicalOrder { target: SlotIndex },
    #[error("LinkGraph edge {from}->{to} is missing its reverse")]
    LinkGraphMissingReverse { from: SlotIndex, to: SlotIndex },
    #[error("LinkGraph edge {from}<->{to} has unequal authored weights")]
    LinkGraphWeightMismatch { from: SlotIndex, to: SlotIndex },
    #[error("field adjacency is not exactly weighted-undirected")]
    AdjacencyNotUndirected,
    #[error("field sweep n_dims must be > 0 (got {0})")]
    InvalidDims(u32),
    #[error("field sweep output column {output_col} is outside n_dims {n_dims}")]
    InvalidOutputColumn {
        output_col: ColumnIndex,
        n_dims: u32,
    },
    #[error("field sweep gather slot {slot} is outside slot count {slots}")]
    InvalidGatherSlot { slot: SlotIndex, slots: u32 },
    #[error("field sweep gather column {col} is outside n_dims {n_dims}")]
    InvalidGatherColumn { col: ColumnIndex, n_dims: u32 },
    #[error("field sweep destination slot {slot} is outside slot count {slots}")]
    InvalidDestinationSlot { slot: SlotIndex, slots: u32 },
    #[error("field sweep registration is missing FieldLawProof")]
    MissingFieldLawProof,
    #[error("field sweep registration is missing CanonicalOrderProof")]
    MissingCanonicalOrderProof,
    #[error("CanonicalOrderProof does not bind the authored adjacency order")]
    CanonicalOrderProofMismatch,
    #[error("conservative FieldLawProof does not bind this undirected adjacency")]
    UndirectedSymmetryCertificateMismatch,
    #[error("conservative FieldLawProof is missing its per-node conductance certificate")]
    MissingConductanceCertificate,
    #[error("conductance certificate does not bind this authored adjacency")]
    ConductanceCertificateMismatch,
    #[error("a transient certificate can only be minted by a transient-output registration")]
    TransientCertificateFromMatrixOutput,
    #[error("field EML reads the transient lane without a producer certificate")]
    MissingTransientReadProof,
    #[error("transient read proof does not bind this adjacency and matrix layout")]
    TransientReadProofMismatch,
    #[error("transient read proof supplied to a registration that does not read transient state")]
    UnusedTransientReadProof,
    #[error("one registration cannot read and overwrite the transient lane in the same pass")]
    TransientReadWriteAliasing,
    #[error("conductance certificate requires {slots} chi values (got {actual})")]
    ConductanceChiCount { slots: u32, actual: usize },
    #[error("conductance admitted bound must be finite and non-negative (got {0})")]
    InvalidConductanceBound(f32),
    #[error("conductance chi at slot {slot} must be finite and non-negative (got {chi})")]
    InvalidNodeChi { slot: SlotIndex, chi: f32 },
    #[error(
        "conductance bound exceeded at slot {slot}: chi={chi} weighted_degree={weighted_degree} bound={admitted_bound}"
    )]
    ConductanceBoundExceeded {
        slot: SlotIndex,
        chi: f32,
        weighted_degree: f32,
        admitted_bound: f32,
    },
    #[error(
        "resource class stack={stack_slots} nodes={max_program_nodes} is not the admitted legacy fixed-32 class"
    )]
    UnsupportedResourceClass {
        stack_slots: u32,
        max_program_nodes: u32,
    },
    #[error("field sweep dt must be finite and non-negative (got {0})")]
    InvalidDt(f32),
    #[error("{name} EML program must not be empty")]
    EmptyProgram { name: &'static str },
    #[error("{name} EML program has {nodes} nodes; maximum is {max}")]
    ProgramTooLarge {
        name: &'static str,
        nodes: u32,
        max: u32,
    },
    #[error("{name} EML program stack underflow")]
    StackUnderflow { name: &'static str },
    #[error("{name} EML program stack depth {depth} exceeds {max}")]
    StackDepthExceeded {
        name: &'static str,
        depth: u32,
        max: u32,
    },
    #[error("{name} EML program has malformed RETURN_TOP placement")]
    MalformedReturn { name: &'static str },
    #[error("{name} EML field PARAM index {index} is out of range")]
    FieldParamOutOfRange { name: &'static str, index: u32 },
    #[error("{name} uses SLOT_VALUE; field programs must name TARGET_VALUE or NEIGHBOR_VALUE")]
    SlotValueAmbiguousInFieldProgram { name: &'static str },
    #[error("{name} division is missing the safe-division admission flag")]
    UnsafeDivision { name: &'static str },
    #[error("{name} node {node} requires a neighbor in a target-only context")]
    MalformedEdgeContext { name: &'static str, node: u32 },
    #[error("{name} references column {col} outside n_dims {n_dims}")]
    MalformedEdgeColumn {
        name: &'static str,
        col: u32,
        n_dims: u32,
    },
    #[error(transparent)]
    OpcodeGate(#[from] OpcodeGateError),
}

#[derive(Debug, Error)]
pub enum FieldSweepExecutionError {
    #[error("values length {actual} does not match required {required}")]
    ValuesLength { actual: usize, required: usize },
    #[error("transient length {actual} does not match required {required}")]
    TransientLength { actual: usize, required: usize },
    #[error("field sweep transient input has not been produced in this session chain")]
    TransientNotInitialized,
    #[error("field sweep iterations must be > 0 (got {0})")]
    InvalidIterations(u32),
    #[error("field EML execution requires neighbor context")]
    MissingNeighborContext,
    #[error("field EML PARAM index {0} is invalid")]
    InvalidFieldParam(u32),
    #[error(
        "malformed edge context slot={slot} col={col} n_dims={n_dims} values_len={values_len}"
    )]
    MalformedEdgeContext {
        slot: u32,
        col: u32,
        n_dims: u32,
        values_len: usize,
    },
    #[error("field EML operand stack overflow")]
    StackOverflow,
    #[error("field EML program did not return")]
    ProgramDidNotReturn,
    #[error("field sweep session cannot accept a registration with a different immutable binding")]
    RegistrationBindingChanged,
    #[error("field sweep readback channel closed")]
    ReadbackChannel,
    #[error("field sweep readback map failed: {0}")]
    ReadbackMap(wgpu::BufferAsyncError),
}
