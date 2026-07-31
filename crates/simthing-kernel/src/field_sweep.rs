//! FIELD-SWEEP-N4-PARITY-0 — one proof-admitted EML field sweep over the existing
//! AccumulatorOp input-list gather representation.
//!
//! Algebra stays in authored `map_program` / `fold_program` / `post_program` data.
//! The executor has one fixed linear fold and never branches on a field kind,
//! algebra identity, or operator identity.

use std::collections::BTreeSet;
use std::sync::mpsc;

use bytemuck::{Pod, Zeroable};
use simthing_core::{eml_opcode, EmlNodeGpu, EML_STACK_MAX, MAX_EML_TREE_NODES};
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
    pub const MAX: u32 = FOLDED;
}

/// One authored cardinal offset. Rung 5.5 admits unit-weight N4 only; weighted
/// adjacency remains owned by rung 5.6.
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

/// N4 adjacency plus the existing input-list gather rows. Fields are private so
/// authored order cannot be changed after proof minting.
#[derive(Clone, Debug)]
pub struct FieldAdjacency {
    width: u32,
    height: u32,
    offsets: [GridN4Offset; 4],
    lists: Vec<Vec<AccumulatorInputGpu>>,
    order_fingerprint: u64,
    symmetry_fingerprint: u64,
}

impl FieldAdjacency {
    pub fn grid_n4(
        width: u32,
        height: u32,
        offsets: [GridN4Offset; 4],
    ) -> Result<Self, FieldSweepAdmissionError> {
        if width == 0 || height == 0 {
            return Err(FieldSweepAdmissionError::InvalidDimensions { width, height });
        }
        let required = BTreeSet::from([(-1, 0), (1, 0), (0, -1), (0, 1)]);
        let actual: BTreeSet<_> = offsets
            .iter()
            .map(|offset| (i32::from(offset.dx), i32::from(offset.dy)))
            .collect();
        if actual != required || actual.len() != offsets.len() {
            return Err(FieldSweepAdmissionError::NotGridN4);
        }

        let mut lists = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                let mut row = Vec::with_capacity(4);
                for offset in offsets {
                    let nx = i64::from(x) + i64::from(offset.dx);
                    let ny = i64::from(y) + i64::from(offset.dy);
                    if nx >= 0 && ny >= 0 && nx < i64::from(width) && ny < i64::from(height) {
                        row.push(AccumulatorInputGpu {
                            slot: ny as u32 * width + nx as u32,
                            col: 0,
                            unit_cost_bits: 1.0f32.to_bits(),
                            flags: 0,
                        });
                    }
                }
                lists.push(row);
            }
        }
        let order_fingerprint = fingerprint_order(width, height, &offsets);
        let symmetry_fingerprint = fingerprint_symmetry(width, height, &actual);
        Ok(Self {
            width,
            height,
            offsets,
            lists,
            order_fingerprint,
            symmetry_fingerprint,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn slots(&self) -> u32 {
        self.width * self.height
    }

    pub fn offsets(&self) -> &[GridN4Offset; 4] {
        &self.offsets
    }

    pub fn apply_canonical_order_proof(&self) -> CanonicalOrderProof {
        CanonicalOrderProof {
            fingerprint: self.order_fingerprint,
        }
    }

    pub fn apply_undirected_symmetry_certificate(&self) -> UndirectedSymmetryCertificate {
        UndirectedSymmetryCertificate {
            fingerprint: self.symmetry_fingerprint,
        }
    }
}

fn fingerprint_order(width: u32, height: u32, offsets: &[GridN4Offset; 4]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in width
        .to_le_bytes()
        .into_iter()
        .chain(height.to_le_bytes())
        .chain(offsets.iter().flat_map(|o| [o.dx as u8, o.dy as u8]))
    {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn fingerprint_symmetry(width: u32, height: u32, offsets: &BTreeSet<(i32, i32)>) -> u64 {
    let mut hash = u64::from(width) << 32 | u64::from(height);
    for &(dx, dy) in offsets {
        hash = hash.rotate_left(9) ^ ((dx as i64 as u64) << 32) ^ (dy as i64 as u64);
    }
    hash
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

/// Sealed law proof. Execution does not inspect this value; admission consumes
/// it before a registration can exist.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldLawProof {
    required_symmetry_fingerprint: Option<u64>,
}

impl FieldLawProof {
    pub fn apply_non_conservative() -> Self {
        Self {
            required_symmetry_fingerprint: None,
        }
    }

    pub fn apply_conservative(certificate: UndirectedSymmetryCertificate) -> Self {
        Self {
            required_symmetry_fingerprint: Some(certificate.fingerprint),
        }
    }
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
    pub output_col: u32,
    pub map_program: Vec<EmlNodeGpu>,
    pub fold_program: Vec<EmlNodeGpu>,
    pub identity_bits: u32,
    pub post_program: Vec<EmlNodeGpu>,
    pub field_law_proof: Option<FieldLawProof>,
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
    output_col: u32,
    map_program: Vec<EmlNodeGpu>,
    fold_program: Vec<EmlNodeGpu>,
    identity_bits: u32,
    post_program: Vec<EmlNodeGpu>,
    field_law_proof: FieldLawProof,
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

    pub fn output_col(&self) -> u32 {
        self.output_col
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
    if request.output_col >= request.n_dims {
        return Err(FieldSweepAdmissionError::InvalidOutputColumn {
            output_col: request.output_col,
            n_dims: request.n_dims,
        });
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
        if required != request.adjacency.symmetry_fingerprint {
            return Err(FieldSweepAdmissionError::UndirectedSymmetryCertificateMismatch);
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

    Ok(FieldSweepRegistration {
        adjacency: request.adjacency,
        n_dims: request.n_dims,
        output_col: request.output_col,
        map_program: request.map_program,
        fold_program: request.fold_program,
        identity_bits: request.identity_bits,
        post_program: request.post_program,
        field_law_proof,
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
                    && node.a == field_param::NEIGHBOR_SLOT
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
    pub target_slot: u32,
    pub neighbor_slot: Option<u32>,
    pub accumulator: f32,
    pub edge_scalar: f32,
    pub dt: f32,
    pub mapped: f32,
    pub folded: f32,
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
                let value = read_cell(values, context.target_slot, node.a, n_dims)?;
                push(&mut stack, &mut sp, value)?;
            }
            eml_opcode::NEIGHBOR_VALUE => {
                let neighbor_slot = context
                    .neighbor_slot
                    .ok_or(FieldSweepExecutionError::MissingNeighborContext)?;
                let value = read_cell(values, neighbor_slot, node.a, n_dims)?;
                push(&mut stack, &mut sp, value)?;
            }
            eml_opcode::PARAM => {
                let value = match node.a {
                    field_param::TARGET_SLOT => context.target_slot as f32,
                    field_param::NEIGHBOR_SLOT => context
                        .neighbor_slot
                        .map(|slot| slot as f32)
                        .unwrap_or(f32::NAN),
                    field_param::ACCUMULATOR => context.accumulator,
                    field_param::EDGE_SCALAR => context.edge_scalar,
                    field_param::DT => context.dt,
                    field_param::MAPPED => context.mapped,
                    field_param::FOLDED => context.folded,
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
    slot: u32,
    col: u32,
    n_dims: u32,
) -> Result<f32, FieldSweepExecutionError> {
    if col >= n_dims {
        return Err(FieldSweepExecutionError::MalformedEdgeContext {
            slot,
            col,
            n_dims,
            values_len: values.len(),
        });
    }
    let index = slot as usize * n_dims as usize + col as usize;
    values
        .get(index)
        .copied()
        .ok_or(FieldSweepExecutionError::MalformedEdgeContext {
            slot,
            col,
            n_dims,
            values_len: values.len(),
        })
}

pub fn execute_field_sweep_cpu(
    values: &[f32],
    registration: &FieldSweepRegistration,
) -> Result<Vec<f32>, FieldSweepExecutionError> {
    let required = registration.slots() as usize * registration.n_dims as usize;
    if values.len() != required {
        return Err(FieldSweepExecutionError::ValuesLength {
            actual: values.len(),
            required,
        });
    }
    let mut output = values.to_vec();
    for (target_slot, list) in registration.adjacency.lists.iter().enumerate() {
        let target_slot = target_slot as u32;
        let mut accumulator = f32::from_bits(registration.identity_bits);
        for input in list {
            let base_context = FieldEmlContext {
                target_slot,
                neighbor_slot: Some(input.slot),
                accumulator,
                edge_scalar: f32::from_bits(input.unit_cost_bits),
                dt: registration.dt,
                mapped: 0.0,
                folded: 0.0,
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
            },
            values,
            registration.n_dims,
        )?;
        let output_index =
            target_slot as usize * registration.n_dims as usize + registration.output_col as usize;
        output[output_index] = written;
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
    for _ in 0..iterations {
        current = execute_field_sweep_cpu(&current, registration)?;
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
    _pad0: u32,
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
    params: Buffer,
    values_len: usize,
    read_a: bool,
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
                    uniform_entry(5),
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

        let (range_rows, flat_inputs) = flatten_gather(&registration.adjacency);
        let ranges = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("field_sweep_ranges"),
                contents: bytemuck::cast_slice(&range_rows),
                usage: BufferUsages::STORAGE,
            });
        let inputs = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("field_sweep_inputs"),
                contents: bytemuck::cast_slice(&flat_inputs),
                usage: BufferUsages::STORAGE,
            });
        let (flat_nodes, gpu_params) = pack_programs(registration);
        let node_capacity =
            3 * FIELD_SWEEP_LEGACY_PROGRAM_NODES as u64 * std::mem::size_of::<EmlNodeGpu>() as u64;
        let nodes = storage_buffer(&ctx.device, "field_sweep_nodes", node_capacity, false);
        ctx.queue
            .write_buffer(&nodes, 0, bytemuck::cast_slice(&flat_nodes));
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
            params,
            values_len,
            read_a: true,
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
        Ok(())
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
        if self.values_len != registration.slots() as usize * registration.n_dims as usize {
            return Err(FieldSweepExecutionError::RegistrationShapeChanged);
        }
        let (flat_nodes, gpu_params) = pack_programs(registration);
        ctx.queue
            .write_buffer(&self.nodes, 0, bytemuck::cast_slice(&flat_nodes));
        ctx.queue
            .write_buffer(&self.params, 0, bytemuck::bytes_of(&gpu_params));

        for _ in 0..iterations {
            let (source, target) = if self.read_a {
                (&self.values_a, &self.values_b)
            } else {
                (&self.values_b, &self.values_a)
            };
            let bind_group = self.bind_group(&ctx.device, source, target);
            let mut encoder = ctx
                .device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("field_sweep_dispatch"),
                });
            {
                let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                    label: Some("field_sweep_pass"),
                    timestamp_writes: None,
                });
                pass.set_pipeline(&self.pipeline);
                pass.set_bind_group(0, &bind_group, &[]);
                pass.dispatch_workgroups(
                    registration.slots().div_ceil(FIELD_SWEEP_WORKGROUP_SIZE),
                    1,
                    1,
                );
            }
            ctx.queue.submit(Some(encoder.finish()));
            self.read_a = !self.read_a;
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
                    resource: self.params.as_entire_binding(),
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
        flat.extend_from_slice(list);
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
            output_col: registration.output_col,
            map_offset,
            map_count: registration.map_program.len() as u32,
            fold_offset,
            fold_count: registration.fold_program.len() as u32,
            post_offset,
            post_count: registration.post_program.len() as u32,
            identity_bits: registration.identity_bits,
            dt_bits: registration.dt.to_bits(),
            _pad0: 0,
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
    #[error("field sweep n_dims must be > 0 (got {0})")]
    InvalidDims(u32),
    #[error("field sweep output column {output_col} is outside n_dims {n_dims}")]
    InvalidOutputColumn { output_col: u32, n_dims: u32 },
    #[error("field sweep registration is missing FieldLawProof")]
    MissingFieldLawProof,
    #[error("field sweep registration is missing CanonicalOrderProof")]
    MissingCanonicalOrderProof,
    #[error("CanonicalOrderProof does not bind the authored adjacency order")]
    CanonicalOrderProofMismatch,
    #[error("conservative FieldLawProof does not bind this undirected adjacency")]
    UndirectedSymmetryCertificateMismatch,
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
    #[error("field sweep session cannot accept a registration with a different shape")]
    RegistrationShapeChanged,
    #[error("field sweep readback channel closed")]
    ReadbackChannel,
    #[error("field sweep readback map failed: {0}")]
    ReadbackMap(wgpu::BufferAsyncError),
}
