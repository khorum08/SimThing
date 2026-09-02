//! RESIDENT-CLEARING-APPORTIONMENT-0 — exact resident constrained settlement.
//!
//! This is the integer residue stage over the existing resident
//! `AllocatedFlow` authority. It owns no score, pressure, or scheduling law:
//! the caller binds already-admitted semantic rows and the existing arena
//! integration band. The GPU implementation preserves the frozen software
//! `u64` pairs and adds an exact common-Q149 representation for binary32
//! numerator bases; it never uses atomics or physical arrival order.
//!
//! Settlement output and recursive supply intake are aliases of one canonical
//! product. The original role-projection fence remains load-bearing:
//!
//! ```compile_fail,E0308
//! use simthing_kernel::ResidentRecursiveSupplyIntake;
//!
//! struct GrantRow(u32);
//! struct ChildSupplyRow(u32);
//! impl From<GrantRow> for ChildSupplyRow {
//!     fn from(row: GrantRow) -> Self { Self(row.0) }
//! }
//! fn recursive_intake(_: &[ResidentRecursiveSupplyIntake]) {}
//!
//! let projected = ChildSupplyRow::from(GrantRow(7));
//! recursive_intake(&[projected]);
//! ```
//!
//! A settlement-specific `From`/`Into` adapter cannot enter that port either:
//!
//! ```compile_fail,E0308
//! use simthing_kernel::{ResidentRecursiveSupplyIntake, ResidentSettlementOutput};
//!
//! struct ChildSupplyRow(ResidentSettlementOutput);
//! impl From<ResidentSettlementOutput> for ChildSupplyRow {
//!     fn from(row: ResidentSettlementOutput) -> Self { Self(row) }
//! }
//! fn recursive_intake(_: &[ResidentRecursiveSupplyIntake]) {}
//!
//! fn converted_bridge(settled: ResidentSettlementOutput) {
//!     let projected: ChildSupplyRow = settled.into();
//!     recursive_intake(&[projected]);
//! }
//! ```
//!
//! A seam payload translation is equally inadmissible even when it copies all
//! currently visible fields. The recursive port accepts the original `T_s`,
//! never a look-alike payload:
//!
//! ```compile_fail,E0308
//! use simthing_kernel::{ResidentRecursiveSupplyIntake, ResidentSettlementOutput};
//!
//! struct SeamPayload {
//!     semantic_row: u32,
//!     granted: u32,
//!     unresolved: u32,
//! }
//! fn translate(row: ResidentSettlementOutput) -> SeamPayload {
//!     SeamPayload {
//!         semantic_row: row.semantic_row(),
//!         granted: row.granted(),
//!         unresolved: row.unresolved(),
//!     }
//! }
//! fn recursive_intake(_: &[ResidentRecursiveSupplyIntake]) {}
//!
//! fn translated_bridge(settled: ResidentSettlementOutput) {
//!     recursive_intake(&[translate(settled)]);
//! }
//! ```

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::mpsc;

use bytemuck::{Pod, Zeroable};
use simthing_core::{ColumnIndex, GenerationStamp, SimThingId, SlotIndex};
use thiserror::Error;
use wgpu::util::DeviceExt;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages, CommandEncoder,
    ComputePassDescriptor, ComputePipeline, ComputePipelineDescriptor, MapMode,
    PipelineLayoutDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages,
};

use crate::{GpuContext, ResidentClearingPlan, ResidentClearingRow, SemanticPlanDigest};

pub const RESIDENT_APPORTIONMENT_SCRATCH_BYTES_PER_ROW: u32 = 64;
pub const RESIDENT_APPORTIONMENT_WORKGROUP_SIZE: u32 = 64;

/// Two admitted shader shapes used to prove that workgroup cardinality is not
/// semantic input to exact remainder assignment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResidentApportionmentWorkgroupSize {
    W32,
    W64,
}

impl ResidentApportionmentWorkgroupSize {
    pub const fn get(self) -> u32 {
        match self {
            Self::W32 => 32,
            Self::W64 => 64,
        }
    }
}

/// Physical dispatch shape only. `rows_per_dispatch` may split the input into
/// several command-encoder passes; logical scope and claimant identity remain
/// the only inputs to the exact law.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentApportionmentDispatch {
    workgroup_size: ResidentApportionmentWorkgroupSize,
    rows_per_dispatch: u32,
}

impl ResidentApportionmentDispatch {
    pub fn new(
        workgroup_size: ResidentApportionmentWorkgroupSize,
        rows_per_dispatch: u32,
    ) -> Result<Self, ResidentApportionmentError> {
        if rows_per_dispatch == 0 {
            return Err(ResidentApportionmentError::ZeroDispatchPartition);
        }
        Ok(Self {
            workgroup_size,
            rows_per_dispatch,
        })
    }

    pub const fn single_pass() -> Self {
        Self {
            workgroup_size: ResidentApportionmentWorkgroupSize::W64,
            rows_per_dispatch: u32::MAX,
        }
    }

    pub const fn workgroup_size(self) -> ResidentApportionmentWorkgroupSize {
        self.workgroup_size
    }

    pub const fn rows_per_dispatch(self) -> u32 {
        self.rows_per_dispatch
    }
}

const STATUS_OK: u32 = 0;
const STATUS_INVALID_CONTINUOUS: u32 = 1;
const STATUS_ARITHMETIC_OVERFLOW: u32 = 2;
const STATUS_INACTIVE: u32 = u32::MAX;

/// Canonical exact constrained product `T_s`, with exact unresolved `T_d/U`.
///
/// The semantic row identifies the owner/resource/scope/draw tuple in the
/// immutable resident plan. Realm identity remains on the per-tree buffer
/// owner, so no foreign raw id or physical row becomes durable identity.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Pod, Zeroable)]
pub struct ResidentConstrainedProduct {
    semantic_row: u32,
    source_simthing_id_raw: u32,
    granted: u32,
    unresolved: u32,
    generation: u32,
    status: u32,
    integration_band: u32,
    _reserved: u32,
}

/// Role name for the exact settlement/emission port.
pub type ResidentSettlementOutput = ResidentConstrainedProduct;
/// Role name for the same product at the next recursive level's intake port.
pub type ResidentRecursiveSupplyIntake = ResidentConstrainedProduct;

impl ResidentConstrainedProduct {
    pub const fn semantic_row(self) -> u32 {
        self.semantic_row
    }

    pub fn source_simthing_id(self) -> SimThingId {
        SimThingId::from_session_raw(self.source_simthing_id_raw)
    }

    pub const fn granted(self) -> u32 {
        self.granted
    }

    pub const fn unresolved(self) -> u32 {
        self.unresolved
    }

    pub const fn generation(self) -> GenerationStamp {
        GenerationStamp::new(self.generation)
    }

    pub const fn integration_band(self) -> u32 {
        self.integration_band
    }

    fn successful(
        semantic_row: u32,
        source_simthing_id: SimThingId,
        granted: u32,
        unresolved: u32,
        generation: GenerationStamp,
        integration_band: u32,
    ) -> Self {
        Self {
            semantic_row,
            source_simthing_id_raw: source_simthing_id.raw(),
            granted,
            unresolved,
            generation: generation.get(),
            status: STATUS_OK,
            integration_band,
            _reserved: 0,
        }
    }
}

/// One exact request bound to an existing semantic row and live
/// `AllocatedFlow` cell. `precedence` is the already-decided hard economic
/// band order; smaller values clear first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentApportionmentClaim {
    semantic_row: u32,
    source_simthing_id: SimThingId,
    requested: u32,
    available: u32,
    precedence: u32,
    allocated_flow_slot: SlotIndex,
    allocated_flow_col: ColumnIndex,
}

impl ResidentApportionmentClaim {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        semantic_row: u32,
        source_simthing_id: SimThingId,
        requested: u32,
        available: u32,
        precedence: u32,
        allocated_flow_slot: SlotIndex,
        allocated_flow_col: ColumnIndex,
    ) -> Self {
        Self {
            semantic_row,
            source_simthing_id,
            requested,
            available,
            precedence,
            allocated_flow_slot,
            allocated_flow_col,
        }
    }

    pub const fn semantic_row(self) -> u32 {
        self.semantic_row
    }
    pub const fn source_simthing_id(self) -> SimThingId {
        self.source_simthing_id
    }
    pub const fn requested(self) -> u32 {
        self.requested
    }
    pub const fn available(self) -> u32 {
        self.available
    }
    pub const fn precedence(self) -> u32 {
        self.precedence
    }
    pub const fn allocated_flow_slot(self) -> SlotIndex {
        self.allocated_flow_slot
    }
    pub const fn allocated_flow_col(self) -> ColumnIndex {
        self.allocated_flow_col
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ScopeKey(u32, u32, u32);

fn scope_key(row: ResidentClearingRow) -> ScopeKey {
    ScopeKey(row.owner().get(), row.resource().get(), row.scope().get())
}

/// Immutable exact-residue plan. Claim vector order is deliberately retained:
/// the GPU proof permutes it while producing the same canonical output slots.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResidentApportionmentPlan {
    claims: Vec<ResidentApportionmentClaim>,
    semantic_rows: Vec<ResidentClearingRow>,
    semantic_digest: SemanticPlanDigest,
    row_count: u32,
    authority_granter: SimThingId,
    generation: GenerationStamp,
    integration_band: u32,
}

impl ResidentApportionmentPlan {
    pub fn build(
        semantic_plan: &ResidentClearingPlan,
        claims: Vec<ResidentApportionmentClaim>,
        authority_granter: SimThingId,
        generation: GenerationStamp,
        integration_band: u32,
    ) -> Result<Self, ResidentApportionmentError> {
        if semantic_plan.budgets().scratch_bytes_per_row()
            < RESIDENT_APPORTIONMENT_SCRATCH_BYTES_PER_ROW
        {
            return Err(ResidentApportionmentError::ScratchRowTooSmall {
                required: RESIDENT_APPORTIONMENT_SCRATCH_BYTES_PER_ROW,
                admitted: semantic_plan.budgets().scratch_bytes_per_row(),
            });
        }
        let row_count = u32::try_from(semantic_plan.rows().len())
            .map_err(|_| ResidentApportionmentError::RowCountNarrowing)?;
        if claims.len() > semantic_plan.rows().len() {
            return Err(ResidentApportionmentError::TooManyClaims {
                claims: claims.len(),
                rows: semantic_plan.rows().len(),
            });
        }

        let mut semantic_targets = BTreeSet::new();
        let mut logical_sources = BTreeSet::new();
        let mut available_by_scope = BTreeMap::new();
        for claim in &claims {
            let row = semantic_plan
                .rows()
                .get(claim.semantic_row as usize)
                .copied()
                .ok_or(ResidentApportionmentError::UnknownSemanticRow {
                    row: claim.semantic_row,
                    row_count,
                })?;
            if !semantic_targets.insert(claim.semantic_row) {
                return Err(ResidentApportionmentError::DuplicateSemanticTarget {
                    row: claim.semantic_row,
                });
            }
            let key = scope_key(row);
            if !logical_sources.insert((key, claim.source_simthing_id)) {
                return Err(ResidentApportionmentError::DuplicateLogicalClaim {
                    source_id: claim.source_simthing_id,
                });
            }
            match available_by_scope.insert(key, claim.available) {
                Some(previous) if previous != claim.available => {
                    return Err(ResidentApportionmentError::InconsistentSupply {
                        expected: previous,
                        observed: claim.available,
                    });
                }
                _ => {}
            }
        }

        // The frozen CPU authority performs identity/supply admission above,
        // then silently omits zero-request rows before apportionment. Preserve
        // that full u32 request domain rather than rejecting or materializing
        // a zero grant.
        let claims = claims
            .into_iter()
            .filter(|claim| claim.requested != 0)
            .collect();

        Ok(Self {
            claims,
            semantic_rows: semantic_plan.rows().to_vec(),
            semantic_digest: semantic_plan.digest(),
            row_count,
            authority_granter,
            generation,
            integration_band,
        })
    }

    pub fn claims(&self) -> &[ResidentApportionmentClaim] {
        &self.claims
    }
    pub const fn semantic_digest(&self) -> SemanticPlanDigest {
        self.semantic_digest
    }
    pub const fn row_count(&self) -> u32 {
        self.row_count
    }
    pub const fn authority_granter(&self) -> SimThingId {
        self.authority_granter
    }
    pub const fn generation(&self) -> GenerationStamp {
        self.generation
    }
    pub const fn integration_band(&self) -> u32 {
        self.integration_band
    }
}

const EXACT_BASIS_LIMBS: usize = 7;
const EXACT_BASIS_FRACTION_BITS: u32 = 149;
type ExactBasis = [u32; EXACT_BASIS_LIMBS];

fn exact_cmp(left: &ExactBasis, right: &ExactBasis) -> Ordering {
    for limb in (0..EXACT_BASIS_LIMBS).rev() {
        match left[limb].cmp(&right[limb]) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
    }
    Ordering::Equal
}

fn exact_is_zero(value: &ExactBasis) -> bool {
    value.iter().all(|&limb| limb == 0)
}

fn exact_shifted_u32(value: u32, shift: u32) -> ExactBasis {
    let mut result = [0; EXACT_BASIS_LIMBS];
    if value == 0 {
        return result;
    }
    let limb = (shift / 32) as usize;
    let offset = shift % 32;
    result[limb] = value << offset;
    if offset != 0 && limb + 1 < EXACT_BASIS_LIMBS {
        result[limb + 1] = value >> (32 - offset);
    }
    result
}

/// Converts one non-negative finite binary32 allocation into an exact common
/// Q149 numerator and applies the lawful integer request cap without a float
/// conversion. Every finite binary32 value is an integer multiple of 2^-149,
/// so this is an identity representation, not a settlement rounding rule.
fn exact_capped_basis(allocated: f32, requested: u32) -> ExactBasis {
    let cap = exact_shifted_u32(requested, EXACT_BASIS_FRACTION_BITS);
    let bits = allocated.to_bits();
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x007f_ffff;
    if exponent == 0 {
        let subnormal = exact_shifted_u32(fraction, 0);
        return if exact_cmp(&subnormal, &cap).is_gt() {
            cap
        } else {
            subnormal
        };
    }
    // exponent >= 159 means the finite value is at least 2^32 and therefore
    // exceeds every admitted u32 request.
    if exponent >= 159 {
        return cap;
    }
    let significand = 0x0080_0000 | fraction;
    let exact = exact_shifted_u32(significand, exponent - 1);
    if exact_cmp(&exact, &cap).is_gt() {
        cap
    } else {
        exact
    }
}

fn exact_checked_add(
    left: &ExactBasis,
    right: &ExactBasis,
) -> Result<ExactBasis, ResidentApportionmentError> {
    let mut result = [0; EXACT_BASIS_LIMBS];
    let mut carry = 0u32;
    for limb in 0..EXACT_BASIS_LIMBS {
        let (partial, overflow0) = left[limb].overflowing_add(right[limb]);
        let (sum, overflow1) = partial.overflowing_add(carry);
        result[limb] = sum;
        carry = u32::from(overflow0 || overflow1);
    }
    if carry != 0 {
        return Err(ResidentApportionmentError::ArithmeticOverflow);
    }
    Ok(result)
}

fn exact_shl1(value: &ExactBasis) -> Result<ExactBasis, ResidentApportionmentError> {
    if value[EXACT_BASIS_LIMBS - 1] >> 31 != 0 {
        return Err(ResidentApportionmentError::ArithmeticOverflow);
    }
    let mut result = [0; EXACT_BASIS_LIMBS];
    let mut carry = 0u32;
    for limb in 0..EXACT_BASIS_LIMBS {
        result[limb] = (value[limb] << 1) | carry;
        carry = value[limb] >> 31;
    }
    Ok(result)
}

fn exact_sub(left: &ExactBasis, right: &ExactBasis) -> ExactBasis {
    debug_assert!(!exact_cmp(left, right).is_lt());
    let mut result = [0; EXACT_BASIS_LIMBS];
    let mut borrow = 0u32;
    for limb in 0..EXACT_BASIS_LIMBS {
        let (partial, borrow0) = left[limb].overflowing_sub(right[limb]);
        let (difference, borrow1) = partial.overflowing_sub(borrow);
        result[limb] = difference;
        borrow = u32::from(borrow0 || borrow1);
    }
    debug_assert_eq!(borrow, 0);
    result
}

fn exact_mul_u32(
    value: &ExactBasis,
    multiplier: u32,
) -> Result<ExactBasis, ResidentApportionmentError> {
    let mut result = [0; EXACT_BASIS_LIMBS];
    let mut addend = *value;
    let mut remaining = multiplier;
    for bit in 0..32 {
        if remaining & 1 != 0 {
            result = exact_checked_add(&result, &addend)?;
        }
        remaining >>= 1;
        if bit != 31 {
            addend = exact_shl1(&addend)?;
        }
    }
    Ok(result)
}

fn exact_divmod_u32(
    numerator: &ExactBasis,
    denominator: &ExactBasis,
) -> Result<(u32, ExactBasis), ResidentApportionmentError> {
    debug_assert!(!exact_is_zero(denominator));
    let mut quotient = 0u32;
    let mut remainder = [0; EXACT_BASIS_LIMBS];
    for bit_index in (0..(EXACT_BASIS_LIMBS * 32)).rev() {
        remainder = exact_shl1(&remainder)?;
        remainder[0] |= (numerator[bit_index / 32] >> (bit_index % 32)) & 1;
        if !exact_cmp(&remainder, denominator).is_lt() {
            remainder = exact_sub(&remainder, denominator);
            if bit_index >= 32 {
                return Err(ResidentApportionmentError::ArithmeticOverflow);
            }
            quotient |= 1u32 << bit_index;
        }
    }
    Ok((quotient, remainder))
}

fn settle_resident_apportionment_over_share_vector(
    plan: &ResidentApportionmentPlan,
    bases: &[ExactBasis],
) -> Result<Vec<ResidentConstrainedProduct>, ResidentApportionmentError> {
    debug_assert_eq!(plan.claims.len(), bases.len());
    let mut products = Vec::with_capacity(plan.claims.len());
    for (claim_index, claim) in plan.claims.iter().enumerate() {
        let row = plan.semantic_rows[claim.semantic_row as usize];
        let key = scope_key(row);
        let group: Vec<_> = plan
            .claims
            .iter()
            .enumerate()
            .filter(|(_, other)| scope_key(plan.semantic_rows[other.semantic_row as usize]) == key)
            .collect();
        let total_requested = group
            .iter()
            .try_fold(0u64, |sum, (_, other)| {
                sum.checked_add(u64::from(other.requested))
            })
            .ok_or(ResidentApportionmentError::ArithmeticOverflow)?;
        let unresolved_total = total_requested.saturating_sub(u64::from(claim.available));
        if unresolved_total > u64::from(u32::MAX) {
            return Err(ResidentApportionmentError::ArithmeticOverflow);
        }
        let prior_requested = group
            .iter()
            .filter(|(_, other)| other.precedence < claim.precedence)
            .try_fold(0u64, |sum, (_, other)| {
                sum.checked_add(u64::from(other.requested))
            })
            .ok_or(ResidentApportionmentError::ArithmeticOverflow)?;
        let band: Vec<_> = group
            .into_iter()
            .filter(|(_, other)| other.precedence == claim.precedence)
            .collect();
        let requested_total = band
            .iter()
            .try_fold(0u64, |sum, (_, other)| {
                sum.checked_add(u64::from(other.requested))
            })
            .ok_or(ResidentApportionmentError::ArithmeticOverflow)?;
        let remaining = u64::from(claim.available).saturating_sub(prior_requested);
        let available_for_band = u32::try_from(remaining.min(requested_total))
            .map_err(|_| ResidentApportionmentError::ArithmeticOverflow)?;
        let basis_total = band
            .iter()
            .try_fold([0; EXACT_BASIS_LIMBS], |sum, (index, _)| {
                exact_checked_add(&sum, &bases[*index])
            })?;

        let granted = if exact_is_zero(&basis_total) {
            0
        } else {
            let mut base_total = 0u64;
            let mut remainders = Vec::with_capacity(band.len());
            let mut base_grants = Vec::with_capacity(band.len());
            for (index, _) in &band {
                let numerator = exact_mul_u32(&bases[*index], available_for_band)?;
                let (base, remainder) = exact_divmod_u32(&numerator, &basis_total)?;
                base_total = base_total
                    .checked_add(u64::from(base))
                    .ok_or(ResidentApportionmentError::ArithmeticOverflow)?;
                base_grants.push(base);
                remainders.push(remainder);
            }
            let leftover = u64::from(available_for_band)
                .checked_sub(base_total)
                .ok_or(ResidentApportionmentError::ArithmeticOverflow)?
                as usize;
            let mut order: Vec<usize> = (0..band.len()).collect();
            order.sort_by(|&left, &right| {
                exact_cmp(&remainders[right], &remainders[left]).then_with(|| {
                    band[left]
                        .1
                        .source_simthing_id
                        .cmp(&band[right].1.source_simthing_id)
                })
            });
            let mut tie_start = 0usize;
            while tie_start < order.len() {
                let remainder = remainders[order[tie_start]];
                let mut tie_end = tie_start + 1;
                while tie_end < order.len() && remainders[order[tie_end]] == remainder {
                    tie_end += 1;
                }
                let tie_len = tie_end - tie_start;
                let rotation = (u64::from(plan.authority_granter.raw())
                    + u64::from(plan.generation.get()))
                    % tie_len as u64;
                order[tie_start..tie_end].rotate_left(rotation as usize);
                tie_start = tie_end;
            }
            let band_index = band
                .iter()
                .position(|(index, _)| *index == claim_index)
                .expect("the current claim is in its equality band");
            let grant = u64::from(base_grants[band_index])
                + u64::from(
                    order
                        .iter()
                        .take(leftover)
                        .any(|&winner| winner == band_index),
                );
            let grant =
                u32::try_from(grant).map_err(|_| ResidentApportionmentError::ArithmeticOverflow)?;
            if grant > claim.requested {
                return Err(ResidentApportionmentError::ArithmeticOverflow);
            }
            grant
        };
        products.push(ResidentConstrainedProduct::successful(
            claim.semantic_row,
            claim.source_simthing_id,
            granted,
            claim.requested - granted,
            plan.generation,
            plan.integration_band,
        ));
    }
    products.sort_by_key(|product| {
        (
            scope_key(plan.semantic_rows[product.semantic_row as usize]),
            product.source_simthing_id_raw,
        )
    });
    Ok(products)
}

/// Exact CPU oracle over the resident continuous share vector. Each live
/// binary32 magnitude is represented exactly in common Q149 units before the
/// frozen integer quotient/remainder, tie, cap, and unresolved laws run.
pub fn execute_resident_apportionment_cpu(
    plan: &ResidentApportionmentPlan,
    values: &[f32],
    n_dims: u32,
) -> Result<Vec<ResidentConstrainedProduct>, ResidentApportionmentError> {
    let mut bases = Vec::with_capacity(plan.claims.len());
    for claim in &plan.claims {
        let value_index = claim
            .allocated_flow_slot
            .raw()
            .checked_mul(n_dims)
            .and_then(|base| base.checked_add(claim.allocated_flow_col.raw_u32()))
            .ok_or(ResidentApportionmentError::ValueIndexOverflow)?
            as usize;
        let allocated =
            *values
                .get(value_index)
                .ok_or(ResidentApportionmentError::ValueIndexOutOfBounds {
                    index: value_index,
                    len: values.len(),
                })?;
        if !allocated.is_finite() || allocated < 0.0 {
            return Err(ResidentApportionmentError::InvalidContinuousAllocation {
                source_id: claim.source_simthing_id,
            });
        }
        bases.push(exact_capped_basis(allocated, claim.requested));
    }
    settle_resident_apportionment_over_share_vector(plan, &bases)
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct ResidentApportionmentScratchRowGpu {
    semantic_row: u32,
    source_simthing_id_raw: u32,
    requested: u32,
    available: u32,
    precedence: u32,
    allocated_flow_slot: u32,
    allocated_flow_col: u32,
    active: u32,
    product: ResidentConstrainedProduct,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct ResidentApportionmentParamsGpu {
    row_count: u32,
    n_slots: u32,
    n_dims: u32,
    generation: u32,
    granter: u32,
    integration_band: u32,
    dispatch_base: u32,
    dispatch_count: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
    _pad3: u32,
}

/// GPU executor for the exact residue stage. It writes canonical product slots
/// inside the already-admitted 14.2 scratch buffer.
pub struct ResidentApportionmentSession {
    layout: wgpu::BindGroupLayout,
    pipeline_32: ComputePipeline,
    pipeline_64: ComputePipeline,
}

impl ResidentApportionmentSession {
    pub fn new(ctx: &GpuContext) -> Self {
        let layout = ctx
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("resident_apportionment_layout"),
                entries: &[
                    uniform_layout_entry(0),
                    storage_layout_entry(1, true),
                    storage_layout_entry(2, true),
                    storage_layout_entry(3, false),
                ],
            });
        let shader = ctx.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("resident_clearing_apportionment"),
            source: ShaderSource::Wgsl(
                include_str!("shaders/resident_clearing_apportionment.wgsl").into(),
            ),
        });
        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("resident_apportionment_pipeline_layout"),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            });
        let pipeline = |label, entry_point| {
            ctx.device
                .create_compute_pipeline(&ComputePipelineDescriptor {
                    label: Some(label),
                    layout: Some(&pipeline_layout),
                    module: &shader,
                    entry_point,
                    compilation_options: Default::default(),
                    cache: None,
                })
        };
        Self {
            layout,
            pipeline_32: pipeline("resident_apportionment_pipeline_w32", "settle_exact_w32"),
            pipeline_64: pipeline("resident_apportionment_pipeline_w64", "settle_exact_w64"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn encode_at_integration_band_with_dispatch(
        &mut self,
        ctx: &GpuContext,
        encoder: &mut CommandEncoder,
        values: &Buffer,
        semantic_rows: &Buffer,
        scratch: &Buffer,
        n_slots: u32,
        n_dims: u32,
        plan: &ResidentApportionmentPlan,
        dispatch: ResidentApportionmentDispatch,
    ) -> Result<(), ResidentApportionmentError> {
        for claim in &plan.claims {
            if claim.allocated_flow_slot.raw() >= n_slots
                || claim.allocated_flow_col.raw_u32() >= n_dims
            {
                return Err(ResidentApportionmentError::ValueBindingOutOfBounds {
                    slot: claim.allocated_flow_slot.raw(),
                    col: claim.allocated_flow_col.raw_u32(),
                    n_slots,
                    n_dims,
                });
            }
            claim
                .allocated_flow_slot
                .raw()
                .checked_mul(n_dims)
                .and_then(|base| base.checked_add(claim.allocated_flow_col.raw_u32()))
                .ok_or(ResidentApportionmentError::ValueIndexOverflow)?;
        }
        let required = u64::from(plan.row_count)
            .checked_mul(u64::from(RESIDENT_APPORTIONMENT_SCRATCH_BYTES_PER_ROW))
            .ok_or(ResidentApportionmentError::ArithmeticOverflow)?;
        if scratch.size() < required {
            return Err(ResidentApportionmentError::ScratchBufferTooSmall {
                required,
                observed: scratch.size(),
            });
        }
        let mut upload =
            vec![ResidentApportionmentScratchRowGpu::zeroed(); plan.row_count as usize];
        for row in &mut upload {
            row.product.status = STATUS_INACTIVE;
        }
        for (physical, claim) in plan.claims.iter().enumerate() {
            upload[physical] = ResidentApportionmentScratchRowGpu {
                semantic_row: claim.semantic_row,
                source_simthing_id_raw: claim.source_simthing_id.raw(),
                requested: claim.requested,
                available: claim.available,
                precedence: claim.precedence,
                allocated_flow_slot: claim.allocated_flow_slot.raw(),
                allocated_flow_col: claim.allocated_flow_col.raw_u32(),
                active: 1,
                product: ResidentConstrainedProduct {
                    status: STATUS_INACTIVE,
                    ..ResidentConstrainedProduct::default()
                },
            };
        }
        ctx.queue
            .write_buffer(scratch, 0, bytemuck::cast_slice(&upload));
        let pipeline = match dispatch.workgroup_size {
            ResidentApportionmentWorkgroupSize::W32 => &self.pipeline_32,
            ResidentApportionmentWorkgroupSize::W64 => &self.pipeline_64,
        };
        let partition = dispatch.rows_per_dispatch.min(plan.row_count.max(1));
        let mut dispatch_base = 0u32;
        while dispatch_base < plan.row_count {
            let dispatch_count = partition.min(plan.row_count - dispatch_base);
            let params = ResidentApportionmentParamsGpu {
                row_count: plan.row_count,
                n_slots,
                n_dims,
                generation: plan.generation.get(),
                granter: plan.authority_granter.raw(),
                integration_band: plan.integration_band,
                dispatch_base,
                dispatch_count,
                _pad0: 0,
                _pad1: 0,
                _pad2: 0,
                _pad3: 0,
            };
            let params_buffer = ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("resident_apportionment_partition_params"),
                    contents: bytemuck::bytes_of(&params),
                    usage: BufferUsages::UNIFORM,
                });
            let bind_group = ctx.device.create_bind_group(&BindGroupDescriptor {
                label: Some("resident_apportionment_partition_bind_group"),
                layout: &self.layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: params_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: semantic_rows.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: values.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: scratch.as_entire_binding(),
                    },
                ],
            });
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("resident_exact_apportionment_at_integration_band"),
                timestamp_writes: None,
            });
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(dispatch_count.div_ceil(dispatch.workgroup_size.get()), 1, 1);
            drop(pass);
            dispatch_base = dispatch_base
                .checked_add(dispatch_count)
                .ok_or(ResidentApportionmentError::ArithmeticOverflow)?;
        }
        Ok(())
    }

    pub fn readback_products(
        &self,
        ctx: &GpuContext,
        scratch: &Buffer,
        plan: &ResidentApportionmentPlan,
    ) -> Result<Vec<ResidentConstrainedProduct>, ResidentApportionmentError> {
        let bytes = u64::from(plan.row_count)
            .checked_mul(u64::from(RESIDENT_APPORTIONMENT_SCRATCH_BYTES_PER_ROW))
            .ok_or(ResidentApportionmentError::ArithmeticOverflow)?;
        let readback = ctx.device.create_buffer(&BufferDescriptor {
            label: Some("resident_apportionment_readback"),
            size: bytes,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("resident_apportionment_readback_encoder"),
            });
        encoder.copy_buffer_to_buffer(scratch, 0, &readback, 0, bytes);
        ctx.queue.submit(Some(encoder.finish()));
        let slice = readback.slice(..);
        let (tx, rx) = mpsc::channel();
        slice.map_async(MapMode::Read, move |result| {
            let _ = tx.send(result);
        });
        let _ = ctx.device.poll(wgpu::Maintain::Wait);
        rx.recv()
            .map_err(|_| ResidentApportionmentError::ReadbackChannelClosed)?
            .map_err(|error| ResidentApportionmentError::ReadbackMap(error.to_string()))?;
        let mapped = slice.get_mapped_range();
        let mut products = Vec::with_capacity(plan.claims.len());
        let mut failure = None;
        for chunk in mapped.chunks_exact(RESIDENT_APPORTIONMENT_SCRATCH_BYTES_PER_ROW as usize) {
            let row = bytemuck::pod_read_unaligned::<ResidentApportionmentScratchRowGpu>(chunk);
            match row.product.status {
                STATUS_INACTIVE => {}
                STATUS_OK => products.push(row.product),
                STATUS_INVALID_CONTINUOUS => {
                    failure = Some(ResidentApportionmentError::InvalidContinuousAllocation {
                        source_id: SimThingId::from_session_raw(row.product.source_simthing_id_raw),
                    });
                    break;
                }
                STATUS_ARITHMETIC_OVERFLOW => {
                    failure = Some(ResidentApportionmentError::ArithmeticOverflow);
                    break;
                }
                status => {
                    failure = Some(ResidentApportionmentError::UnknownGpuStatus(status));
                    break;
                }
            }
        }
        drop(mapped);
        readback.unmap();
        if let Some(error) = failure {
            return Err(error);
        }
        products.sort_by_key(|product| {
            (
                scope_key(plan.semantic_rows[product.semantic_row as usize]),
                product.source_simthing_id_raw,
            )
        });
        if products.len() != plan.claims.len() {
            return Err(ResidentApportionmentError::IncompleteGpuOutput {
                expected: plan.claims.len(),
                observed: products.len(),
            });
        }
        Ok(products)
    }
}

fn uniform_layout_entry(binding: u32) -> BindGroupLayoutEntry {
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

fn storage_layout_entry(binding: u32, read_only: bool) -> BindGroupLayoutEntry {
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

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ResidentApportionmentError {
    #[error("resident scratch row requires {required} bytes, admitted {admitted}")]
    ScratchRowTooSmall { required: u32, admitted: u32 },
    #[error("resident semantic row count cannot narrow to u32")]
    RowCountNarrowing,
    #[error("resident apportionment has {claims} claims for {rows} semantic rows")]
    TooManyClaims { claims: usize, rows: usize },
    #[error("resident apportionment references semantic row {row}, row count {row_count}")]
    UnknownSemanticRow { row: u32, row_count: u32 },
    #[error("more than one exact claim targets semantic row {row}")]
    DuplicateSemanticTarget { row: u32 },
    #[error("duplicate exact claim source {source_id:?} in one scope")]
    DuplicateLogicalClaim { source_id: SimThingId },
    #[error("one resident scope repeats supply inconsistently: {expected} then {observed}")]
    InconsistentSupply { expected: u32, observed: u32 },
    #[error("resident exact arithmetic overflow")]
    ArithmeticOverflow,
    #[error("resident value index arithmetic overflow")]
    ValueIndexOverflow,
    #[error("resident value index {index} exceeds value length {len}")]
    ValueIndexOutOfBounds { index: usize, len: usize },
    #[error("live AllocatedFlow for {source_id:?} is non-finite or negative")]
    InvalidContinuousAllocation { source_id: SimThingId },
    #[error("resident AllocatedFlow binding ({slot},{col}) exceeds ({n_slots},{n_dims})")]
    ValueBindingOutOfBounds {
        slot: u32,
        col: u32,
        n_slots: u32,
        n_dims: u32,
    },
    #[error("resident scratch buffer requires {required} bytes, observed {observed}")]
    ScratchBufferTooSmall { required: u64, observed: u64 },
    #[error("resident apportionment readback channel closed")]
    ReadbackChannelClosed,
    #[error("resident apportionment readback map failed: {0}")]
    ReadbackMap(String),
    #[error("resident apportionment GPU returned unknown status {0}")]
    UnknownGpuStatus(u32),
    #[error("resident apportionment dispatch partition must contain at least one row")]
    ZeroDispatchPartition,
    #[error("resident apportionment produced {observed} products, expected {expected}")]
    IncompleteGpuOutput { expected: usize, observed: usize },
}
