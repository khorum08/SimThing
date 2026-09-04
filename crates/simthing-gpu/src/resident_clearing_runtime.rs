//! Production ownership for the bounded resident clearing live head.
//!
//! Exact settlement remains the graduated kernel implementation. This module
//! wires canonical `T_s` bytes into the admission-bounded IntegrationSchedule
//! segment. Spatial consumers read immutable G there at the same generation;
//! the separate Current-to-Next mint reads immutable U into ordinary demand.

use std::process::Command;
use std::sync::mpsc;

use bytemuck::Pod;
use simthing_core::ResidentScheduleReservation;
use simthing_kernel::{
    GpuContext, ResidentApportionmentDispatch, ResidentApportionmentError,
    ResidentApportionmentPlan, ResidentApportionmentSession, ResidentConstrainedProduct,
    ResidentTemporalDemand, ResidentTemporalDemandMintError, ResidentTemporalDemandMintSession,
    WorldGpuState, RESIDENT_APPORTIONMENT_SCRATCH_BYTES_PER_ROW,
};
use thiserror::Error;
use wgpu::{Buffer, BufferDescriptor, BufferUsages, CommandEncoder, MapMode};

const PRODUCT_BYTES: u64 = std::mem::size_of::<ResidentConstrainedProduct>() as u64;
pub const QUALIFIED_RESIDENT_CLEARING_FINGERPRINT: u64 = 0xbfc8_db39_1f8c_d256;

/// Exact adapter/compiler/ABI record inherited from the graduated 14.5
/// certificate. A changed tuple must be separately qualified; it never
/// inherits production authority by merely being able to create a device.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResidentClearingQualification {
    backend: String,
    adapter: String,
    vendor: u32,
    device: u32,
    device_class: String,
    driver_runtime: String,
    features: String,
    compiler: String,
    shader_compiler: String,
    cargo_lock_hash: u64,
    shader_source_hash: u64,
    workgroups: [u32; 2],
    subgroup_assumption: String,
    abi_version: u32,
}

impl ResidentClearingQualification {
    pub fn capture(ctx: &GpuContext) -> Result<Self, ResidentLiveHeadError> {
        let info = ctx.adapter.get_info();
        let compiler = Command::new("rustc")
            .arg("-Vv")
            .output()
            .map_err(|error| ResidentLiveHeadError::QualificationProbe(error.to_string()))?;
        if !compiler.status.success() {
            return Err(ResidentLiveHeadError::QualificationProbe(
                "rustc -Vv returned a non-success status".into(),
            ));
        }
        let compiler = String::from_utf8(compiler.stdout)
            .map_err(|error| ResidentLiveHeadError::QualificationProbe(error.to_string()))?
            .replace("\r\n", "\n")
            .trim()
            .to_owned();
        Ok(Self {
            backend: format!("{:?}", info.backend),
            adapter: info.name,
            vendor: info.vendor,
            device: info.device,
            device_class: format!("{:?}", info.device_type),
            driver_runtime: format!("{} {}", info.driver, info.driver_info),
            features: format!("{:?}", ctx.adapter.features()),
            compiler,
            shader_compiler: "wgpu 22.1.0 / naga 22.1.0".into(),
            cargo_lock_hash: stable_hash(
                0xcbf2_9ce4_8422_2325,
                include_bytes!("../../../Cargo.lock"),
            ),
            shader_source_hash: stable_hash(
                0xcbf2_9ce4_8422_2325,
                include_bytes!(
                    "../../simthing-kernel/src/shaders/resident_clearing_apportionment.wgsl"
                ),
            ),
            workgroups: [32, 64],
            subgroup_assumption: "subgroup-independent:no-subgroup-builtins-or-size-authority"
                .into(),
            abi_version: crate::RESIDENT_CLEARING_ABI_VERSION,
        })
    }

    pub fn admit(ctx: &GpuContext) -> Result<Self, ResidentLiveHeadError> {
        let record = Self::capture(ctx)?;
        record.ensure_production_qualified()?;
        Ok(record)
    }

    fn ensure_production_qualified(&self) -> Result<(), ResidentLiveHeadError> {
        let observed = self.fingerprint();
        if observed == QUALIFIED_RESIDENT_CLEARING_FINGERPRINT {
            Ok(())
        } else {
            Err(ResidentLiveHeadError::UnqualifiedAdapter {
                required: QUALIFIED_RESIDENT_CLEARING_FINGERPRINT,
                observed,
            })
        }
    }

    pub fn fingerprint(&self) -> u64 {
        let mut state = 0xcbf2_9ce4_8422_2325;
        for bytes in [
            self.backend.as_bytes(),
            self.adapter.as_bytes(),
            self.device_class.as_bytes(),
            self.driver_runtime.as_bytes(),
            self.features.as_bytes(),
            self.compiler.as_bytes(),
            self.shader_compiler.as_bytes(),
            self.subgroup_assumption.as_bytes(),
        ] {
            state = stable_hash(state, &(bytes.len() as u64).to_le_bytes());
            state = stable_hash(state, bytes);
        }
        for value in [
            u64::from(self.vendor),
            u64::from(self.device),
            self.cargo_lock_hash,
            self.shader_source_hash,
            u64::from(self.workgroups[0]),
            u64::from(self.workgroups[1]),
            u64::from(self.abi_version),
        ] {
            state = stable_hash(state, &value.to_le_bytes());
        }
        state
    }
}

fn stable_hash(mut state: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        state ^= u64::from(*byte);
        state = state.wrapping_mul(0x100_0000_01b3);
    }
    state
}

/// Per-tree device buffers for the authoritative immutable schedule head and
/// the ordinary Current-to-Next demand mint.
pub struct ResidentClearingLiveHead {
    capacity: u32,
    segment: Buffer,
    next_demands: Buffer,
}

impl ResidentClearingLiveHead {
    pub fn admit(ctx: &GpuContext, capacity: u32) -> Result<Self, ResidentLiveHeadError> {
        if capacity == 0 {
            return Err(ResidentLiveHeadError::ZeroCapacity);
        }
        let bytes = u64::from(capacity)
            .checked_mul(PRODUCT_BYTES)
            .ok_or(ResidentLiveHeadError::ArithmeticOverflow)?;
        let product_buffer = |label| {
            ctx.device.create_buffer(&BufferDescriptor {
                label: Some(label),
                size: bytes,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };
        Ok(Self {
            capacity,
            segment: product_buffer("resident_clearing_schedule_live_head"),
            next_demands: ctx.device.create_buffer(&BufferDescriptor {
                label: Some("resident_temporal_demands"),
                size: u64::from(capacity)
                    .checked_mul(std::mem::size_of::<ResidentTemporalDemand>() as u64)
                    .ok_or(ResidentLiveHeadError::ArithmeticOverflow)?,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
        })
    }

    pub const fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Append one exact product set to the immutable live-head segment. No
    /// temporal payload is minted implicitly.
    pub fn encode_append(
        &self,
        encoder: &mut CommandEncoder,
        scratch: &Buffer,
        plan: &ResidentApportionmentPlan,
        reservation: ResidentScheduleReservation,
        semantic_scope_owner: simthing_core::SimThingId,
    ) -> Result<ResidentClearingSubmission, ResidentLiveHeadError> {
        let claim_count = u32::try_from(plan.claims().len())
            .map_err(|_| ResidentLiveHeadError::ArithmeticOverflow)?;
        if reservation.len() != claim_count
            || reservation
                .start()
                .checked_add(claim_count)
                .is_none_or(|end| end > self.capacity)
        {
            return Err(ResidentLiveHeadError::ReservationOutOfBounds {
                start: reservation.start(),
                rows: claim_count,
                capacity: self.capacity,
            });
        }
        for (physical, claim) in plan.claims().iter().enumerate() {
            let source = u64::from(claim.semantic_row())
                .checked_mul(u64::from(RESIDENT_APPORTIONMENT_SCRATCH_BYTES_PER_ROW))
                .and_then(|offset| offset.checked_add(PRODUCT_BYTES))
                .ok_or(ResidentLiveHeadError::ArithmeticOverflow)?;
            let target_row = u64::from(reservation.start())
                .checked_add(physical as u64)
                .ok_or(ResidentLiveHeadError::ArithmeticOverflow)?;
            let target = target_row
                .checked_mul(PRODUCT_BYTES)
                .ok_or(ResidentLiveHeadError::ArithmeticOverflow)?;
            encoder.copy_buffer_to_buffer(scratch, source, &self.segment, target, PRODUCT_BYTES);
        }
        Ok(ResidentClearingSubmission {
            reservation,
            generation: plan.generation(),
            product_count: claim_count,
            authority_granter: plan.authority_granter(),
            semantic_scope_owner,
        })
    }

    /// Execute a child market from immutable parent `T_s.G` at the same
    /// generation. Both granter and semantic-scope owner must change.
    #[allow(clippy::too_many_arguments)]
    pub fn encode_spatial_apportionment(
        &self,
        state: &WorldGpuState,
        session: &mut ResidentApportionmentSession,
        encoder: &mut CommandEncoder,
        semantic_rows: &Buffer,
        scratch: &Buffer,
        plan: &ResidentApportionmentPlan,
        parent: ResidentClearingSubmission,
        semantic_scope_owner: simthing_core::SimThingId,
    ) -> Result<(), ResidentLiveHeadError> {
        if plan.generation() != parent.generation {
            return Err(ResidentLiveHeadError::SpatialGenerationMismatch {
                expected: parent.generation,
                observed: plan.generation(),
            });
        }
        if plan.authority_granter() == parent.authority_granter {
            return Err(ResidentLiveHeadError::SpatialGranterRetained {
                granter: plan.authority_granter(),
            });
        }
        if semantic_scope_owner == parent.semantic_scope_owner
            || semantic_scope_owner != plan.authority_granter()
        {
            return Err(ResidentLiveHeadError::SpatialScopeRetained);
        }
        state
            .encode_resident_apportionment_from_spatial_products_with_dispatch_into(
                session,
                encoder,
                semantic_rows,
                scratch,
                &self.segment,
                parent.reservation.start(),
                parent.product_count,
                plan,
                ResidentApportionmentDispatch::single_pass(),
            )
            .map_err(ResidentLiveHeadError::ResidentApportionment)
    }

    /// Prepare the ordinary N+1 demand buffer from immutable `T_s.U`. This is
    /// the sole generation-advancing mint and performs no N+1 settlement.
    #[allow(clippy::too_many_arguments)]
    pub fn encode_temporal_demand_mint(
        &self,
        ctx: &GpuContext,
        mint: &ResidentTemporalDemandMintSession,
        encoder: &mut CommandEncoder,
        plan: &ResidentApportionmentPlan,
        products: ResidentClearingSubmission,
        authored_demands: &[u32],
        demand_generation: simthing_core::GenerationStamp,
    ) -> Result<ResidentTemporalDemandSubmission, ResidentLiveHeadError> {
        if plan.generation() != products.generation {
            return Err(ResidentLiveHeadError::TemporalSourceGenerationMismatch {
                expected: products.generation,
                observed: plan.generation(),
            });
        }
        if plan.claims().len() != products.product_count as usize {
            return Err(ResidentLiveHeadError::ProductCountMismatch {
                expected: products.product_count,
                observed: plan.claims().len() as u32,
            });
        }
        mint.encode(
            ctx,
            encoder,
            &self.segment,
            products.reservation.start(),
            &self.next_demands,
            plan,
            authored_demands,
            demand_generation,
        )?;
        Ok(ResidentTemporalDemandSubmission {
            generation: demand_generation,
            demand_count: products.product_count,
        })
    }

    /// Execute N+1 only from an already-prepared ordinary demand buffer and a
    /// newly prepared N+1 exact plan.
    #[allow(clippy::too_many_arguments)]
    pub fn encode_temporal_apportionment(
        &self,
        state: &WorldGpuState,
        session: &mut ResidentApportionmentSession,
        encoder: &mut CommandEncoder,
        semantic_rows: &Buffer,
        scratch: &Buffer,
        plan: &ResidentApportionmentPlan,
        demands: ResidentTemporalDemandSubmission,
    ) -> Result<(), ResidentLiveHeadError> {
        if plan.generation() != demands.generation {
            return Err(ResidentLiveHeadError::TemporalExecutionGenerationMismatch {
                expected: demands.generation,
                observed: plan.generation(),
            });
        }
        state
            .encode_resident_apportionment_from_temporal_demands_with_dispatch_into(
                session,
                encoder,
                semantic_rows,
                scratch,
                &self.next_demands,
                demands.demand_count,
                plan,
                ResidentApportionmentDispatch::single_pass(),
            )
            .map_err(ResidentLiveHeadError::ResidentApportionment)
    }

    /// Proof/observer readback. Production calls this only after the queue has
    /// already received the resident N+1 intake copy.
    pub fn readback_segment(
        &self,
        ctx: &GpuContext,
        submission: ResidentClearingSubmission,
    ) -> Result<Vec<ResidentConstrainedProduct>, ResidentLiveHeadError> {
        readback_products(
            ctx,
            &self.segment,
            submission.reservation.start(),
            submission.product_count,
            "resident_clearing_schedule_materialization",
        )
    }

    /// Referee-only observation of the ordinary Current-to-Next demand port.
    pub fn readback_temporal_demands_for_proof(
        &self,
        ctx: &GpuContext,
        submission: ResidentTemporalDemandSubmission,
    ) -> Result<Vec<ResidentTemporalDemand>, ResidentLiveHeadError> {
        readback_products(
            ctx,
            &self.next_demands,
            0,
            submission.demand_count,
            "resident_temporal_demand_proof",
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentClearingSubmission {
    reservation: ResidentScheduleReservation,
    generation: simthing_core::GenerationStamp,
    product_count: u32,
    authority_granter: simthing_core::SimThingId,
    semantic_scope_owner: simthing_core::SimThingId,
}

impl ResidentClearingSubmission {
    pub const fn reservation(self) -> ResidentScheduleReservation {
        self.reservation
    }

    pub const fn generation(self) -> simthing_core::GenerationStamp {
        self.generation
    }

    pub const fn product_count(self) -> u32 {
        self.product_count
    }

    pub const fn authority_granter(self) -> simthing_core::SimThingId {
        self.authority_granter
    }

    pub const fn semantic_scope_owner(self) -> simthing_core::SimThingId {
        self.semantic_scope_owner
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentTemporalDemandSubmission {
    generation: simthing_core::GenerationStamp,
    demand_count: u32,
}

impl ResidentTemporalDemandSubmission {
    pub const fn generation(self) -> simthing_core::GenerationStamp {
        self.generation
    }

    pub const fn demand_count(self) -> u32 {
        self.demand_count
    }
}

fn readback_products<T: Pod>(
    ctx: &GpuContext,
    source: &Buffer,
    start: u32,
    count: u32,
    label: &'static str,
) -> Result<Vec<T>, ResidentLiveHeadError> {
    let element_bytes = std::mem::size_of::<T>() as u64;
    let offset = u64::from(start)
        .checked_mul(element_bytes)
        .ok_or(ResidentLiveHeadError::ArithmeticOverflow)?;
    let bytes = u64::from(count)
        .checked_mul(element_bytes)
        .ok_or(ResidentLiveHeadError::ArithmeticOverflow)?;
    if bytes == 0 {
        return Ok(Vec::new());
    }
    let readback = ctx.device.create_buffer(&BufferDescriptor {
        label: Some(label),
        size: bytes,
        usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = ctx
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some(label) });
    encoder.copy_buffer_to_buffer(source, offset, &readback, 0, bytes);
    ctx.queue.submit(Some(encoder.finish()));
    let slice = readback.slice(..);
    let (tx, rx) = mpsc::channel();
    slice.map_async(MapMode::Read, move |result| {
        let _ = tx.send(result);
    });
    let _ = ctx.device.poll(wgpu::Maintain::Wait);
    rx.recv()
        .map_err(|_| ResidentLiveHeadError::ReadbackChannelClosed)?
        .map_err(|error| ResidentLiveHeadError::ReadbackMap(error.to_string()))?;
    let mapped = slice.get_mapped_range();
    let products = bytemuck::cast_slice(&mapped).to_vec();
    drop(mapped);
    readback.unmap();
    Ok(products)
}

#[derive(Debug, Error)]
pub enum ResidentLiveHeadError {
    #[error("resident clearing live-head capacity must be at least one")]
    ZeroCapacity,
    #[error("resident clearing live-head arithmetic overflow")]
    ArithmeticOverflow,
    #[error("spatial child generation is {observed:?}, expected parent generation {expected:?}")]
    SpatialGenerationMismatch {
        expected: simthing_core::GenerationStamp,
        observed: simthing_core::GenerationStamp,
    },
    #[error("spatial child retained parent granter {granter:?}")]
    SpatialGranterRetained { granter: simthing_core::SimThingId },
    #[error("spatial child retained parent semantic scope or scope/granter diverged")]
    SpatialScopeRetained,
    #[error("temporal mint source generation is {observed:?}, expected {expected:?}")]
    TemporalSourceGenerationMismatch {
        expected: simthing_core::GenerationStamp,
        observed: simthing_core::GenerationStamp,
    },
    #[error("temporal execution generation is {observed:?}, expected {expected:?}")]
    TemporalExecutionGenerationMismatch {
        expected: simthing_core::GenerationStamp,
        observed: simthing_core::GenerationStamp,
    },
    #[error("resident product set has {expected} products, observed {observed} plan claims")]
    ProductCountMismatch { expected: u32, observed: u32 },
    #[error("resident schedule reservation {start}+{rows} exceeds admitted capacity {capacity}")]
    ReservationOutOfBounds {
        start: u32,
        rows: u32,
        capacity: u32,
    },
    #[error("resident schedule readback channel closed")]
    ReadbackChannelClosed,
    #[error("resident schedule readback map failed: {0}")]
    ReadbackMap(String),
    #[error("resident clearing qualification probe failed: {0}")]
    QualificationProbe(String),
    #[error(
        "resident clearing adapter is not qualified: required {required:016x}, observed {observed:016x}"
    )]
    UnqualifiedAdapter { required: u64, observed: u64 },
    #[error("resident exact settlement failed: {0}")]
    ResidentApportionment(#[source] ResidentApportionmentError),
    #[error("resident temporal demand mint failed: {0}")]
    TemporalDemandMint(#[from] ResidentTemporalDemandMintError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changed_qualification_tuple_fails_typed_before_execution() {
        let ctx = GpuContext::new_blocking().expect("qualification fixture adapter");
        let mut record = ResidentClearingQualification::capture(&ctx).expect("capture tuple");
        record.abi_version ^= 1;
        assert!(matches!(
            record.ensure_production_qualified(),
            Err(ResidentLiveHeadError::UnqualifiedAdapter { .. })
        ));
    }
}
