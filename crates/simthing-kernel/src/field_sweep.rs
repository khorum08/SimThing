//! FIELD-SWEEP-N4-PARITY-0 — one proof-admitted EML field sweep over the existing
//! AccumulatorOp input-list gather representation.
//!
//! Algebra stays in authored `map_program` / `fold_program` / `post_program` data.
//! The executor has one fixed linear fold and never branches on a field kind,
//! algebra identity, or operator identity.
//!
//! Raw semantic identities cannot enter a field-sweep registration:
//!
//! ```compile_fail,E0308
//! use simthing_kernel::FieldSweepRegistrationRequest;
//!
//! fn field_sweep_registration_rejects_raw_column(
//!     mut request: FieldSweepRegistrationRequest,
//! ) {
//!     request.output = 0u32;
//! }
//! ```
//!
//! ```compile_fail,E0308
//! use simthing_kernel::field_sweep::FieldEmlContext;
//!
//! fn field_sweep_context_rejects_raw_slot(mut context: FieldEmlContext) {
//!     context.target_slot = 0u32;
//! }
//! ```

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc;

use bytemuck::{Pod, Zeroable};
use simthing_core::{eml_opcode, ColumnIndex, EmlNodeGpu, EmlResourceClass, InputSpec, SlotIndex};
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
pub const FIELD_SWEEP_LEGACY_STACK_SLOTS: u32 = EmlResourceClass::LegacyFixed32.stack_slots();
pub const FIELD_SWEEP_LEGACY_PROGRAM_NODES: u32 = EmlResourceClass::LegacyFixed32.max_tree_nodes();

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

pub type FieldSweepResourceClass = EmlResourceClass;

/// Stable report identity for the exact admitted map/fold/post postfix IR.
/// Pipeline caching additionally retains the full canonical word sequence, so
/// a digest collision cannot alias two programs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FieldSweepProgramIdentity {
    digest: u64,
    word_count: u32,
}

impl FieldSweepProgramIdentity {
    pub fn digest(self) -> u64 {
        self.digest
    }

    pub fn word_count(self) -> u32 {
        self.word_count
    }
}

/// Stable report identity for a generated pipeline cache entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FieldSweepJitCacheIdentity(u64);

impl FieldSweepJitCacheIdentity {
    pub fn digest(self) -> u64 {
        self.0
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
        self.transient_certificate()
            .ok_or(FieldSweepAdmissionError::TransientCertificateFromMatrixOutput)
    }

    fn transient_certificate(&self) -> Option<FieldTransientCertificate> {
        (self.output == FieldSweepOutput::Transient).then_some(FieldTransientCertificate {
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

    pub fn program_identity(&self) -> FieldSweepProgramIdentity {
        let identity = self.canonical_program_identity();
        FieldSweepProgramIdentity {
            digest: identity.digest(),
            word_count: identity.word_count(),
        }
    }

    pub fn jit_cache_identity(&self) -> FieldSweepJitCacheIdentity {
        let identity = self.canonical_program_identity();
        FieldSweepJitCacheIdentity(crate::eml_resource_class::pipeline_cache_digest(
            self.resource_class,
            &identity,
        ))
    }

    #[cfg(feature = "eml-resource-profiling")]
    pub fn jit_cache_identity_for_profiling_class(
        &self,
        resource_class: EmlResourceClass,
    ) -> Result<FieldSweepJitCacheIdentity, FieldSweepExecutionError> {
        if resource_class < self.resource_class {
            return Err(FieldSweepExecutionError::ProfilingClassTooSmall {
                admitted: self.resource_class,
                requested: resource_class,
            });
        }
        let identity = self.canonical_program_identity();
        Ok(FieldSweepJitCacheIdentity(
            crate::eml_resource_class::pipeline_cache_digest(resource_class, &identity),
        ))
    }

    #[cfg(feature = "eml-resource-profiling")]
    pub fn fused_jit_identity_for_profiling(
        producer: &Self,
        consumer: &Self,
        resource_class: EmlResourceClass,
    ) -> Result<(FieldSweepProgramIdentity, FieldSweepJitCacheIdentity), FieldSweepExecutionError>
    {
        if !can_fuse_transient_pair(producer, consumer) {
            return Err(FieldSweepExecutionError::UnprovenTransientFusion);
        }
        let required_class = producer.resource_class.join(consumer.resource_class);
        if resource_class < required_class {
            return Err(FieldSweepExecutionError::ProfilingClassTooSmall {
                admitted: required_class,
                requested: resource_class,
            });
        }
        let identity = crate::eml_resource_class::CanonicalFieldProgramIdentity::fused_pair(
            &producer.canonical_program_identity(),
            &consumer.canonical_program_identity(),
        );
        Ok((
            FieldSweepProgramIdentity {
                digest: identity.digest(),
                word_count: identity.word_count(),
            },
            FieldSweepJitCacheIdentity(crate::eml_resource_class::pipeline_cache_digest(
                resource_class,
                &identity,
            )),
        ))
    }

    fn canonical_program_identity(
        &self,
    ) -> crate::eml_resource_class::CanonicalFieldProgramIdentity {
        crate::eml_resource_class::CanonicalFieldProgramIdentity::new(
            &self.map_program,
            &self.fold_program,
            &self.post_program,
        )
    }

    #[cfg(feature = "eml-resource-profiling")]
    pub fn generated_jit_wgsl_for_profiling(
        &self,
        resource_class: EmlResourceClass,
    ) -> Result<String, FieldSweepExecutionError> {
        if resource_class < self.resource_class {
            return Err(FieldSweepExecutionError::ProfilingClassTooSmall {
                admitted: self.resource_class,
                requested: resource_class,
            });
        }
        Ok(crate::eml_resource_class::generate_field_sweep_jit(
            include_str!("shaders/field_sweep.wgsl"),
            resource_class,
            &self.map_program,
            &self.fold_program,
            &self.post_program,
        ))
    }

    #[cfg(feature = "eml-resource-profiling")]
    pub fn generated_fused_jit_wgsl_for_profiling(
        producer: &Self,
        consumer: &Self,
        resource_class: EmlResourceClass,
    ) -> Result<String, FieldSweepExecutionError> {
        if !can_fuse_transient_pair(producer, consumer) {
            return Err(FieldSweepExecutionError::UnprovenTransientFusion);
        }
        let required_class = producer.resource_class.join(consumer.resource_class);
        if resource_class < required_class {
            return Err(FieldSweepExecutionError::ProfilingClassTooSmall {
                admitted: required_class,
                requested: resource_class,
            });
        }
        Ok(
            crate::eml_resource_class::generate_fused_transient_field_sweep_jit(
                include_str!("shaders/field_sweep.wgsl"),
                resource_class,
                &producer.map_program,
                &producer.fold_program,
                &producer.post_program,
                &consumer.map_program,
                &consumer.fold_program,
                &consumer.post_program,
            ),
        )
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
    if !request.dt.is_finite() || request.dt < 0.0 {
        return Err(FieldSweepAdmissionError::InvalidDt(request.dt));
    }

    let map_facts = validate_field_program(
        "map",
        &request.map_program,
        request.n_dims,
        FieldProgramContext::Edge,
    )?;
    let fold_facts = validate_field_program(
        "fold",
        &request.fold_program,
        request.n_dims,
        FieldProgramContext::Edge,
    )?;
    let post_facts = validate_field_program(
        "post",
        &request.post_program,
        request.n_dims,
        FieldProgramContext::TargetOnly,
    )?;
    let requested_nodes = map_facts
        .node_count
        .max(fold_facts.node_count)
        .max(post_facts.node_count);
    let requested_stack = map_facts
        .peak_stack
        .max(fold_facts.peak_stack)
        .max(post_facts.peak_stack);
    let resource_class = EmlResourceClass::smallest_fitting(requested_nodes, requested_stack)
        .ok_or(FieldSweepAdmissionError::UnsupportedResourceClass {
            requested_nodes,
            requested_stack,
            attempted: EmlResourceClass::LegacyFixed32,
        })?;

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
        resource_class,
        dt: request.dt,
    })
}

#[derive(Clone, Copy)]
enum FieldProgramContext {
    Edge,
    TargetOnly,
}

#[derive(Clone, Copy)]
struct FieldProgramFacts {
    node_count: u32,
    peak_stack: u32,
}

/// Production plants for uniqueness-fused seam lowerings (5.14 EXIT-PROOF).
/// Each mutates the real arm path — never a test-only alternate executor.
/// Reachable as `simthing_kernel::field_sweep::*` (module already allowlisted);
/// not new crate-root exports.
static PLANT_SEAM_CPU_SEPARATE_ROUNDING: AtomicBool = AtomicBool::new(false);
static PLANT_SEAM_INTERPRETED_DISABLE_FUSE: AtomicBool = AtomicBool::new(false);

/// Plant: CPU uniqueness-fused seam uses separate `*` then `+` instead of `mul_add`.
pub fn plant_seam_cpu_separate_rounding(on: bool) {
    PLANT_SEAM_CPU_SEPARATE_ROUNDING.store(on, Ordering::SeqCst);
}

/// Plant: interpreted arm clears the fused-seam flag so the WGSL path runs
/// separate map + Sum fold instead of `fma`.
pub fn plant_seam_interpreted_disable_fuse(on: bool) {
    PLANT_SEAM_INTERPRETED_DISABLE_FUSE.store(on, Ordering::SeqCst);
}

/// Plant: SSA-JIT uniqueness-fused seam emits the ordinary Sum fold so the
/// map `MUL` and fold `ADD` are separate roundings (violates FUSED meaning).
pub fn plant_seam_jit_separate_rounding(on: bool) {
    crate::eml_resource_class::plant_seam_jit_separate_rounding(on);
}

/// Uniqueness-rule instance (5.14 / DA `5192270934`): map ends
/// `[.., MUL, RETURN]` and fold is the canonical Sum
/// `[PARAM ACC, PARAM MAPPED, ADD, RETURN]`. Exactly one MUL feeds the fold
/// ADD, so the seam IS FUSED (one-rounding fma) on every execution arm.
/// Historical name "SEAM LAW" is this instance — not a peer semantic law.
pub(crate) fn seam_fused_shape(map: &[EmlNodeGpu], fold: &[EmlNodeGpu]) -> bool {
    let map_shape = map.len() >= 3
        && map[map.len() - 2].opcode == eml_opcode::MUL
        && map[map.len() - 1].opcode == eml_opcode::RETURN_TOP;
    let fold_shape = fold.len() == 4
        && fold[0].opcode == eml_opcode::PARAM
        && fold[0].a == field_param::ACCUMULATOR
        && fold[1].opcode == eml_opcode::PARAM
        && fold[1].a == field_param::MAPPED
        && fold[2].opcode == eml_opcode::ADD
        && fold[3].opcode == eml_opcode::RETURN_TOP;
    map_shape && fold_shape
}

fn validate_field_program(
    name: &'static str,
    nodes: &[EmlNodeGpu],
    n_dims: u32,
    context: FieldProgramContext,
) -> Result<FieldProgramFacts, FieldSweepAdmissionError> {
    if nodes.is_empty() {
        return Err(FieldSweepAdmissionError::EmptyProgram { name });
    }
    let mut depth = 0u32;
    let mut peak_stack = 0u32;
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
            | eml_opcode::FLOOR
            | eml_opcode::EXP
            | eml_opcode::LN => {
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
        peak_stack = peak_stack.max(depth);
    }
    if !saw_return || depth != 1 {
        return Err(FieldSweepAdmissionError::MalformedReturn { name });
    }
    // EML-EXP-PRIMITIVE-0: every EXP call site discharges a 5.10 admission
    // shape (in-domain CLAMP_BOUNDED guard or in-domain literal certificate);
    // unguarded/uncertified sites are spanned admission errors from the door.
    crate::eml_opcode_gate::admit_exp_call_sites(nodes)
        .map_err(FieldSweepAdmissionError::OpcodeGate)?;
    Ok(FieldProgramFacts {
        node_count: nodes.len() as u32,
        peak_stack,
    })
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
    let facts = field_program_facts_for_execution(nodes);
    let resource_class = EmlResourceClass::smallest_fitting(facts.node_count, facts.peak_stack)
        .ok_or(FieldSweepExecutionError::UnsupportedResourceClass {
            requested_nodes: facts.node_count,
            requested_stack: facts.peak_stack,
        })?;
    eval_field_eml_cpu_in_class(nodes, context, values, n_dims, resource_class)
}

/// SEAM LAW helper: evaluate the first `count` nodes of an admitted program
/// and return the stack value `depth_from_top` positions below the top —
/// used to recover the map's final-MUL operands for the fused seam.
fn eval_field_eml_cpu_prefix(
    nodes: &[EmlNodeGpu],
    count: usize,
    context: FieldEmlContext,
    values: &[f32],
    n_dims: u32,
    depth_from_top: usize,
) -> Result<f32, FieldSweepExecutionError> {
    let facts = field_program_facts_for_execution(nodes);
    let resource_class = EmlResourceClass::smallest_fitting(facts.node_count, facts.peak_stack)
        .ok_or(FieldSweepExecutionError::UnsupportedResourceClass {
            requested_nodes: facts.node_count,
            requested_stack: facts.peak_stack,
        })?;
    let slots = resource_class.stack_slots() as usize;
    let mut stack = vec![0.0f32; slots];
    let mut mul_a = vec![0.0f32; slots];
    let mut mul_b = vec![0.0f32; slots];
    let mut is_mul = vec![false; slots];
    let mut sp = 0usize;
    for node in &nodes[..count] {
        eval_field_eml_step(
            node,
            context,
            values,
            n_dims,
            &mut stack,
            &mut mul_a,
            &mut mul_b,
            &mut is_mul,
            &mut sp,
        )?;
    }
    Ok(stack[sp - depth_from_top])
}

fn eval_field_eml_cpu_in_class(
    nodes: &[EmlNodeGpu],
    context: FieldEmlContext,
    values: &[f32],
    n_dims: u32,
    resource_class: EmlResourceClass,
) -> Result<f32, FieldSweepExecutionError> {
    let slots = resource_class.stack_slots() as usize;
    let mut stack = vec![0.0f32; slots];
    let mut mul_a = vec![0.0f32; slots];
    let mut mul_b = vec![0.0f32; slots];
    let mut is_mul = vec![false; slots];
    let mut sp = 0usize;
    for node in nodes {
        if let Some(returned) = eval_field_eml_step(
            node,
            context,
            values,
            n_dims,
            &mut stack,
            &mut mul_a,
            &mut mul_b,
            &mut is_mul,
            &mut sp,
        )? {
            return Ok(returned);
        }
    }
    Err(FieldSweepExecutionError::ProgramDidNotReturn)
}

/// One interpreter step over the shared CPU stack (SEAM LAW refactor: the
/// prefix evaluator and the full evaluator execute the identical arms).
fn eval_field_eml_step(
    node: &EmlNodeGpu,
    context: FieldEmlContext,
    values: &[f32],
    n_dims: u32,
    stack: &mut [f32],
    mul_a: &mut [f32],
    mul_b: &mut [f32],
    is_mul: &mut [bool],
    sp: &mut usize,
) -> Result<Option<f32>, FieldSweepExecutionError> {
    match node.opcode {
        eml_opcode::LITERAL_F32 => {
            push_step(stack, mul_a, mul_b, is_mul, sp, f32::from_bits(node.a))?
        }
        eml_opcode::TARGET_VALUE => {
            let value = read_cell(
                values,
                context.target_slot,
                column_from_wire(node.a),
                n_dims,
            )?;
            push_step(stack, mul_a, mul_b, is_mul, sp, value)?;
        }
        eml_opcode::NEIGHBOR_VALUE => {
            let neighbor_slot = context
                .neighbor_slot
                .ok_or(FieldSweepExecutionError::MissingNeighborContext)?;
            let value = read_cell(values, neighbor_slot, column_from_wire(node.a), n_dims)?;
            push_step(stack, mul_a, mul_b, is_mul, sp, value)?;
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
            push_step(stack, mul_a, mul_b, is_mul, sp, value)?;
        }
        eml_opcode::NEG => {
            stack[*sp - 1] = -stack[*sp - 1];
            is_mul[*sp - 1] = false;
        }
        eml_opcode::CLAMP_BOUNDED => {
            stack[*sp - 1] = stack[*sp - 1].clamp(f32::from_bits(node.a), f32::from_bits(node.b));
            is_mul[*sp - 1] = false;
        }
        eml_opcode::CLAMP_FLOORED => {
            stack[*sp - 1] = stack[*sp - 1].max(f32::from_bits(node.a));
            is_mul[*sp - 1] = false;
        }
        eml_opcode::ABS => {
            stack[*sp - 1] = stack[*sp - 1].abs();
            is_mul[*sp - 1] = false;
        }
        eml_opcode::FLOOR => {
            stack[*sp - 1] = stack[*sp - 1].floor();
            is_mul[*sp - 1] = false;
        }
        eml_opcode::EXP => {
            stack[*sp - 1] = simthing_core::eml_exp_pinned_f32(stack[*sp - 1]);
            is_mul[*sp - 1] = false;
        }
        eml_opcode::LN => {
            stack[*sp - 1] = simthing_core::eml_ln::eml_ln_pinned_f32(stack[*sp - 1]);
            is_mul[*sp - 1] = false;
        }
        eml_opcode::SELECT => {
            let false_value = stack[*sp - 1];
            let true_value = stack[*sp - 2];
            let condition = stack[*sp - 3] != 0.0;
            stack[*sp - 3] = if condition { true_value } else { false_value };
            is_mul[*sp - 3] = false;
            *sp -= 2;
        }
        eml_opcode::RETURN_TOP => return Ok(Some(stack[*sp - 1])),
        eml_opcode::ADD | eml_opcode::SUB => {
            let rhs = stack[*sp - 1];
            let lhs = stack[*sp - 2];
            let rhs_mul = is_mul[*sp - 1].then_some((mul_a[*sp - 1], mul_b[*sp - 1]));
            let lhs_mul = is_mul[*sp - 2].then_some((mul_a[*sp - 2], mul_b[*sp - 2]));
            stack[*sp - 2] = crate::eml_uniqueness::uniqueness_add_sub(
                node.opcode == eml_opcode::SUB,
                lhs,
                rhs,
                lhs_mul,
                rhs_mul,
            );
            is_mul[*sp - 2] = false;
            *sp -= 1;
        }
        eml_opcode::MUL => {
            let rhs = stack[*sp - 1];
            let lhs = stack[*sp - 2];
            stack[*sp - 2] = lhs * rhs;
            mul_a[*sp - 2] = lhs;
            mul_b[*sp - 2] = rhs;
            is_mul[*sp - 2] = true;
            *sp -= 1;
        }
        opcode => {
            let rhs = stack[*sp - 1];
            let lhs = stack[*sp - 2];
            stack[*sp - 2] = apply_binary(opcode, lhs, rhs);
            is_mul[*sp - 2] = false;
            *sp -= 1;
        }
    }
    Ok(None)
}

fn push_step(
    stack: &mut [f32],
    _mul_a: &mut [f32],
    _mul_b: &mut [f32],
    is_mul: &mut [bool],
    sp: &mut usize,
    value: f32,
) -> Result<(), FieldSweepExecutionError> {
    if *sp >= stack.len() {
        return Err(FieldSweepExecutionError::StackOverflow);
    }
    stack[*sp] = value;
    is_mul[*sp] = false;
    *sp += 1;
    Ok(())
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

fn push(stack: &mut [f32], sp: &mut usize, value: f32) -> Result<(), FieldSweepExecutionError> {
    if *sp >= stack.len() {
        return Err(FieldSweepExecutionError::StackOverflow);
    }
    stack[*sp] = value;
    *sp += 1;
    Ok(())
}

fn field_program_facts_for_execution(nodes: &[EmlNodeGpu]) -> FieldProgramFacts {
    let mut depth = 0u32;
    let mut peak_stack = 0u32;
    for node in nodes {
        match node.opcode {
            eml_opcode::LITERAL_F32
            | eml_opcode::TARGET_VALUE
            | eml_opcode::NEIGHBOR_VALUE
            | eml_opcode::PARAM => depth = depth.saturating_add(1),
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
            | eml_opcode::CMP_EQ => depth = depth.saturating_sub(1),
            eml_opcode::SELECT => depth = depth.saturating_sub(2),
            _ => {}
        }
        peak_stack = peak_stack.max(depth);
    }
    FieldProgramFacts {
        node_count: nodes.len() as u32,
        peak_stack,
    }
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
    let mut transient_producer = None;
    execute_field_sweep_cpu_with_state(
        values,
        registration,
        &mut transient,
        &mut transient_producer,
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
    let mut transient_producer = None;
    for registration in registrations {
        if registration.slots() != first.slots() || registration.n_dims != first.n_dims {
            return Err(FieldSweepExecutionError::RegistrationBindingChanged);
        }
        current = execute_field_sweep_cpu_with_state(
            &current,
            registration,
            &mut transient,
            &mut transient_producer,
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
    let mut transient_producer = None;
    execute_field_sweep_cpu_with_state(
        values,
        registration,
        &mut transient,
        &mut transient_producer,
        &natural_order,
    )
}

fn execute_field_sweep_cpu_with_state(
    values: &[f32],
    registration: &FieldSweepRegistration,
    transient: &mut [f32],
    transient_producer: &mut Option<FieldTransientCertificate>,
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
    if let Some(required_producer) = registration.transient_read_proof {
        match *transient_producer {
            None => return Err(FieldSweepExecutionError::TransientNotInitialized),
            Some(actual_producer) if actual_producer != required_producer => {
                return Err(FieldSweepExecutionError::TransientProducerBindingMismatch)
            }
            Some(_) => {}
        }
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
            if seam_fused_shape(&registration.map_program, &registration.fold_program) {
                // Uniqueness instance: map's final MUL + canonical Sum fold are
                // one fused rounding — evaluate map minus [MUL, RETURN], then
                // acc = fma(a, b, acc).
                let map = &registration.map_program;
                let rhs = eval_field_eml_cpu_prefix(
                    map,
                    map.len() - 2,
                    base_context,
                    values,
                    registration.n_dims,
                    1,
                )?;
                let lhs = eval_field_eml_cpu_prefix(
                    map,
                    map.len() - 2,
                    base_context,
                    values,
                    registration.n_dims,
                    2,
                )?;
                accumulator = if PLANT_SEAM_CPU_SEPARATE_ROUNDING.load(Ordering::SeqCst) {
                    lhs * rhs + accumulator
                } else {
                    lhs.mul_add(rhs, accumulator)
                };
            } else {
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
        *transient_producer = registration.transient_certificate();
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
    let mut transient_producer = None;
    for _ in 0..iterations {
        current = execute_field_sweep_cpu_with_state(
            &current,
            registration,
            &mut transient,
            &mut transient_producer,
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
    fused_identity_bits: u32,
    fused_dt_bits: u32,
    _pad2: u32,
    _pad3: u32,
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

fn can_fuse_transient_pair(
    producer: &FieldSweepRegistration,
    consumer: &FieldSweepRegistration,
) -> bool {
    producer.output == FieldSweepOutput::Transient
        && consumer.output != FieldSweepOutput::Transient
        && consumer.transient_read_proof == producer.transient_certificate()
        && producer.n_dims == consumer.n_dims
        && producer.adjacency == consumer.adjacency
}

/// Kernel-owned generic field executor. It owns the resolved ping-pong buffers;
/// callers can upload values and receive copied readback, never raw handles.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FieldSweepPipelineKey {
    resource_class: EmlResourceClass,
    program: crate::eml_resource_class::CanonicalFieldProgramIdentity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FieldSweepExecutionMode {
    GeneratedJit,
    #[cfg(feature = "eml-resource-profiling")]
    Interpreted,
}

pub struct FieldSweepSession {
    pipelines: BTreeMap<FieldSweepPipelineKey, ComputePipeline>,
    mode: FieldSweepExecutionMode,
    resource_class: EmlResourceClass,
    layout: BindGroupLayout,
    values_a: Buffer,
    values_b: Buffer,
    ranges: Buffer,
    inputs: Buffer,
    nodes: Buffer,
    schedule: Buffer,
    transient: Buffer,
    binding: FieldSweepSessionBinding,
    values_len: usize,
    read_a: bool,
    transient_initialized: bool,
    registration_dispatches: AtomicU64,
    resident_exports: AtomicU64,
    host_readbacks: AtomicU64,
}

impl FieldSweepSession {
    pub fn new(
        ctx: &GpuContext,
        registration: &FieldSweepRegistration,
    ) -> Result<Self, FieldSweepExecutionError> {
        Self::new_with_resource_class(
            ctx,
            registration,
            registration.resource_class,
            FieldSweepExecutionMode::GeneratedJit,
        )
    }

    /// Test/profiling-only adapter for matched canonical-interpreter class runs.
    /// Admission remains registration-owned; the override may only widen it.
    #[cfg(feature = "eml-resource-profiling")]
    pub fn new_with_profiling_resource_class(
        ctx: &GpuContext,
        registration: &FieldSweepRegistration,
        resource_class: EmlResourceClass,
    ) -> Result<Self, FieldSweepExecutionError> {
        if resource_class < registration.resource_class {
            return Err(FieldSweepExecutionError::ProfilingClassTooSmall {
                admitted: registration.resource_class,
                requested: resource_class,
            });
        }
        Self::new_with_resource_class(
            ctx,
            registration,
            resource_class,
            FieldSweepExecutionMode::GeneratedJit,
        )
    }

    /// Profiling-only preservation of the canonical storage-backed interpreter.
    #[cfg(feature = "eml-resource-profiling")]
    pub fn new_interpreted_for_profiling(
        ctx: &GpuContext,
        registration: &FieldSweepRegistration,
        resource_class: EmlResourceClass,
    ) -> Result<Self, FieldSweepExecutionError> {
        if resource_class < registration.resource_class {
            return Err(FieldSweepExecutionError::ProfilingClassTooSmall {
                admitted: registration.resource_class,
                requested: resource_class,
            });
        }
        Self::new_with_resource_class(
            ctx,
            registration,
            resource_class,
            FieldSweepExecutionMode::Interpreted,
        )
    }

    fn new_with_resource_class(
        ctx: &GpuContext,
        registration: &FieldSweepRegistration,
        resource_class: EmlResourceClass,
        mode: FieldSweepExecutionMode,
    ) -> Result<Self, FieldSweepExecutionError> {
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
        let initial_key = FieldSweepPipelineKey {
            resource_class,
            program: registration.canonical_program_identity(),
        };
        let initial_pipeline =
            create_field_sweep_pipeline(ctx, &layout, registration, resource_class, mode);
        let mut pipelines = BTreeMap::new();
        pipelines.insert(initial_key, initial_pipeline);

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
        let node_capacity = 3
            * u64::from(resource_class.max_tree_nodes())
            * std::mem::size_of::<EmlNodeGpu>() as u64;
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
        let _ = gpu_params;

        Ok(Self {
            pipelines,
            mode,
            resource_class,
            layout,
            values_a,
            values_b,
            ranges,
            inputs,
            nodes,
            schedule,
            transient,
            binding: FieldSweepSessionBinding::from_registration(registration),
            values_len,
            read_a: true,
            transient_initialized: false,
            registration_dispatches: AtomicU64::new(0),
            resident_exports: AtomicU64::new(0),
            host_readbacks: AtomicU64::new(0),
        })
    }

    /// Number of admitted registration executions submitted by this session.
    /// This observes the execution call graph and exposes no numerical values.
    pub fn registration_dispatches(&self) -> u64 {
        self.registration_dispatches.load(Ordering::Relaxed)
    }

    /// Number of GPU-resident exports submitted by this session.
    pub fn resident_exports(&self) -> u64 {
        self.resident_exports.load(Ordering::Relaxed)
    }

    /// Number of successful host readbacks performed by this session.
    pub fn host_readbacks(&self) -> u64 {
        self.host_readbacks.load(Ordering::Relaxed)
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
        self.resident_exports.fetch_add(1, Ordering::Relaxed);
    }

    pub fn dispatch(
        &mut self,
        ctx: &GpuContext,
        registration: &FieldSweepRegistration,
        iterations: u32,
    ) -> Result<(), FieldSweepExecutionError> {
        self.dispatch_chain(ctx, std::slice::from_ref(registration), iterations)
    }

    /// Execute mechanically adjacent admitted stages in one ordered command
    /// submission. Each stage retains its own generated program pipeline;
    /// ping-pong copies and transient producer/consumer ordering are unchanged.
    pub fn dispatch_chain(
        &mut self,
        ctx: &GpuContext,
        registrations: &[FieldSweepRegistration],
        iterations: u32,
    ) -> Result<(), FieldSweepExecutionError> {
        if iterations == 0 {
            return Err(FieldSweepExecutionError::InvalidIterations(iterations));
        }
        if registrations.is_empty() {
            return Err(FieldSweepExecutionError::EmptyRegistrationChain);
        }

        #[cfg(feature = "eml-resource-profiling")]
        if self.mode == FieldSweepExecutionMode::Interpreted && registrations.len() > 1 {
            for registration in registrations {
                self.dispatch_batch(ctx, std::slice::from_ref(registration), iterations)?;
            }
            return Ok(());
        }

        if self.mode == FieldSweepExecutionMode::GeneratedJit
            && iterations == 1
            && registrations.len() == 2
            && can_fuse_transient_pair(&registrations[0], &registrations[1])
        {
            return self.dispatch_fused_transient_pair(ctx, &registrations[0], &registrations[1]);
        }

        self.dispatch_batch(ctx, registrations, iterations)
    }

    fn dispatch_fused_transient_pair(
        &mut self,
        ctx: &GpuContext,
        producer: &FieldSweepRegistration,
        consumer: &FieldSweepRegistration,
    ) -> Result<(), FieldSweepExecutionError> {
        for registration in [producer, consumer] {
            if !self.binding.accepts(registration) {
                return Err(FieldSweepExecutionError::RegistrationBindingChanged);
            }
            if registration.resource_class > self.resource_class {
                return Err(FieldSweepExecutionError::SessionResourceClassTooSmall {
                    session: self.resource_class,
                    registration: registration.resource_class,
                });
            }
        }
        let program = crate::eml_resource_class::CanonicalFieldProgramIdentity::fused_pair(
            &producer.canonical_program_identity(),
            &consumer.canonical_program_identity(),
        );
        let key = FieldSweepPipelineKey {
            resource_class: self.resource_class,
            program,
        };
        if !self.pipelines.contains_key(&key) {
            let source = crate::eml_resource_class::generate_fused_transient_field_sweep_jit(
                include_str!("shaders/field_sweep.wgsl"),
                self.resource_class,
                &producer.map_program,
                &producer.fold_program,
                &producer.post_program,
                &consumer.map_program,
                &consumer.fold_program,
                &consumer.post_program,
            );
            let pipeline = create_field_sweep_pipeline_from_source(ctx, &self.layout, source);
            self.pipelines.insert(key.clone(), pipeline);
        }

        let (_, mut gpu_params) = pack_programs(consumer);
        gpu_params.fused_identity_bits = producer.identity_bits;
        gpu_params.fused_dt_bits = producer.dt.to_bits();
        let params = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("field_sweep_fused_jit_params"),
                contents: bytemuck::bytes_of(&gpu_params),
                usage: BufferUsages::UNIFORM,
            });
        let (source, target) = if self.read_a {
            (&self.values_a, &self.values_b)
        } else {
            (&self.values_b, &self.values_a)
        };
        let bind_group = self.bind_group(&ctx.device, source, target, &params);
        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("field_sweep_fused_transient_jit"),
            });
        let byte_len = (self.values_len * std::mem::size_of::<f32>()) as u64;
        encoder.copy_buffer_to_buffer(source, 0, target, 0, byte_len);
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("field_sweep_fused_transient_jit_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(
                self.pipelines
                    .get(&key)
                    .expect("fused field JIT pipeline was populated before encoding"),
            );
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
        self.registration_dispatches.fetch_add(2, Ordering::Relaxed);
        self.read_a = !self.read_a;
        self.transient_initialized = true;
        ctx.device.poll(wgpu::Maintain::Wait);
        Ok(())
    }

    fn dispatch_batch(
        &mut self,
        ctx: &GpuContext,
        registrations: &[FieldSweepRegistration],
        iterations: u32,
    ) -> Result<(), FieldSweepExecutionError> {
        let mut transient_available = self.transient_initialized;
        for registration in registrations {
            if !self.binding.accepts(registration) {
                return Err(FieldSweepExecutionError::RegistrationBindingChanged);
            }
            if registration.resource_class > self.resource_class {
                return Err(FieldSweepExecutionError::SessionResourceClassTooSmall {
                    session: self.resource_class,
                    registration: registration.resource_class,
                });
            }
            if registration.transient_read_proof.is_some() && !transient_available {
                return Err(FieldSweepExecutionError::TransientNotInitialized);
            }
            if registration.output == FieldSweepOutput::Transient {
                transient_available = true;
            }
        }

        for registration in registrations {
            let key = FieldSweepPipelineKey {
                resource_class: self.resource_class,
                program: registration.canonical_program_identity(),
            };
            if !self.pipelines.contains_key(&key) {
                let pipeline = create_field_sweep_pipeline(
                    ctx,
                    &self.layout,
                    registration,
                    self.resource_class,
                    self.mode,
                );
                self.pipelines.insert(key, pipeline);
            }
        }

        #[cfg(feature = "eml-resource-profiling")]
        if self.mode == FieldSweepExecutionMode::Interpreted {
            let (flat_nodes, _) = pack_programs(&registrations[0]);
            ctx.queue
                .write_buffer(&self.nodes, 0, bytemuck::cast_slice(&flat_nodes));
        }

        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("field_sweep_jit_chain"),
            });
        let byte_len = (self.values_len * std::mem::size_of::<f32>()) as u64;
        let mut read_a = self.read_a;
        for registration in registrations {
            let key = FieldSweepPipelineKey {
                resource_class: self.resource_class,
                program: registration.canonical_program_identity(),
            };
            let pipeline = self
                .pipelines
                .get(&key)
                .expect("field JIT pipeline was populated before encoding");
            let (_, gpu_params) = pack_programs(registration);
            let params = ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("field_sweep_jit_params"),
                    contents: bytemuck::bytes_of(&gpu_params),
                    usage: BufferUsages::UNIFORM,
                });

            for _ in 0..iterations {
                let (source, target) = if read_a {
                    (&self.values_a, &self.values_b)
                } else {
                    (&self.values_b, &self.values_a)
                };
                if registration.output != FieldSweepOutput::Transient {
                    encoder.copy_buffer_to_buffer(source, 0, target, 0, byte_len);
                }
                let bind_group = self.bind_group(&ctx.device, source, target, &params);
                {
                    let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                        label: Some("field_sweep_jit_pass"),
                        timestamp_writes: None,
                    });
                    pass.set_pipeline(pipeline);
                    pass.set_bind_group(0, &bind_group, &[]);
                    pass.dispatch_workgroups(
                        gpu_params
                            .schedule_count
                            .div_ceil(FIELD_SWEEP_WORKGROUP_SIZE),
                        1,
                        1,
                    );
                }
                if registration.output != FieldSweepOutput::Transient {
                    read_a = !read_a;
                }
            }
        }
        ctx.queue.submit(Some(encoder.finish()));
        self.registration_dispatches.fetch_add(
            registrations.len() as u64 * u64::from(iterations),
            Ordering::Relaxed,
        );
        self.read_a = read_a;
        self.transient_initialized = transient_available;
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
        self.host_readbacks.fetch_add(1, Ordering::Relaxed);
        Ok(output)
    }

    fn bind_group(
        &self,
        device: &wgpu::Device,
        source: &Buffer,
        target: &Buffer,
        params: &Buffer,
    ) -> BindGroup {
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
                    resource: params.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 7,
                    resource: self.transient.as_entire_binding(),
                },
            ],
        })
    }
}

fn create_field_sweep_pipeline(
    ctx: &GpuContext,
    layout: &BindGroupLayout,
    registration: &FieldSweepRegistration,
    resource_class: EmlResourceClass,
    mode: FieldSweepExecutionMode,
) -> ComputePipeline {
    let source = match mode {
        FieldSweepExecutionMode::GeneratedJit => {
            crate::eml_resource_class::generate_field_sweep_jit(
                include_str!("shaders/field_sweep.wgsl"),
                resource_class,
                &registration.map_program,
                &registration.fold_program,
                &registration.post_program,
            )
        }
        #[cfg(feature = "eml-resource-profiling")]
        FieldSweepExecutionMode::Interpreted => {
            crate::eml_resource_class::specialize_eml_stack_limit(
                include_str!("shaders/field_sweep.wgsl"),
                resource_class,
            )
        }
    };
    create_field_sweep_pipeline_from_source(ctx, layout, source)
}

fn create_field_sweep_pipeline_from_source(
    ctx: &GpuContext,
    layout: &BindGroupLayout,
    source: String,
) -> ComputePipeline {
    let shader = ctx.device.create_shader_module(ShaderModuleDescriptor {
        label: Some("field_sweep_generated_jit"),
        source: ShaderSource::Wgsl(source.into()),
    });
    let pipeline_layout = ctx
        .device
        .create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("field_sweep_generated_jit_layout"),
            bind_group_layouts: &[layout],
            push_constant_ranges: &[],
        });
    ctx.device
        .create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("field_sweep_generated_jit_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        })
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
            // Uniqueness-fused flag: 1 routes the interpreted arm's canonical
            // Sum fold through fma, matching CPU twin and JIT.
            _pad1: u32::from(
                seam_fused_shape(&registration.map_program, &registration.fold_program)
                    && !PLANT_SEAM_INTERPRETED_DISABLE_FUSE.load(Ordering::SeqCst),
            ),
            fused_identity_bits: 0,
            fused_dt_bits: 0,
            _pad2: 0,
            _pad3: 0,
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
        "field EML facts (nodes {requested_nodes}, peak stack {requested_stack}) do not fit closed class {attempted:?}"
    )]
    UnsupportedResourceClass {
        requested_nodes: u32,
        requested_stack: u32,
        attempted: EmlResourceClass,
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
    #[error("field session class {session:?} cannot execute registration class {registration:?}")]
    SessionResourceClassTooSmall {
        session: EmlResourceClass,
        registration: EmlResourceClass,
    },
    #[cfg(feature = "eml-resource-profiling")]
    #[error("profiling class {requested:?} is smaller than admitted class {admitted:?}")]
    ProfilingClassTooSmall {
        admitted: EmlResourceClass,
        requested: EmlResourceClass,
    },
    #[error(
        "field EML facts (nodes {requested_nodes}, peak stack {requested_stack}) do not fit a closed resource class"
    )]
    UnsupportedResourceClass {
        requested_nodes: u32,
        requested_stack: u32,
    },
    #[error("values length {actual} does not match required {required}")]
    ValuesLength { actual: usize, required: usize },
    #[error("transient length {actual} does not match required {required}")]
    TransientLength { actual: usize, required: usize },
    #[error("field sweep transient input has not been produced in this session chain")]
    TransientNotInitialized,
    #[error(
        "field sweep transient input was produced under a different adjacency or layout binding"
    )]
    TransientProducerBindingMismatch,
    #[error("field sweep iterations must be > 0 (got {0})")]
    InvalidIterations(u32),
    #[error("field sweep registration chain must not be empty")]
    EmptyRegistrationChain,
    #[cfg(feature = "eml-resource-profiling")]
    #[error("field sweep transient fusion lacks the exact producer/consumer certificate")]
    UnprovenTransientFusion,
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
