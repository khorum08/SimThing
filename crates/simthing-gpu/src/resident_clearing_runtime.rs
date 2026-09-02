//! Production ownership for the bounded resident clearing live head.
//!
//! Exact settlement remains the graduated kernel implementation. This module
//! only wires its canonical `T_s` bytes into the one admission-bounded
//! IntegrationSchedule segment and then into the identical N+1 intake buffer
//! before any host mapping is requested.

use std::process::Command;
use std::sync::mpsc;

use bytemuck::Pod;
use simthing_core::ResidentScheduleReservation;
use simthing_kernel::{
    GpuContext, ResidentApportionmentPlan, ResidentConstrainedProduct,
    RESIDENT_APPORTIONMENT_SCRATCH_BYTES_PER_ROW,
};
use thiserror::Error;
use wgpu::{Buffer, BufferDescriptor, BufferUsages, CommandEncoder, MapMode};

const PRODUCT_BYTES: u64 = std::mem::size_of::<ResidentConstrainedProduct>() as u64;
pub const QUALIFIED_RESIDENT_CLEARING_FINGERPRINT: u64 = 0x73ae_5e62_1b3e_5021;

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

/// Per-tree device buffers for the authoritative schedule head and the direct
/// recursive N+1 intake. No registry, global queue, or host identity exists.
pub struct ResidentClearingLiveHead {
    capacity: u32,
    segment: Buffer,
    next_intake: Buffer,
}

impl ResidentClearingLiveHead {
    pub fn admit(ctx: &GpuContext, capacity: u32) -> Result<Self, ResidentLiveHeadError> {
        if capacity == 0 {
            return Err(ResidentLiveHeadError::ZeroCapacity);
        }
        let bytes = u64::from(capacity)
            .checked_mul(PRODUCT_BYTES)
            .ok_or(ResidentLiveHeadError::ArithmeticOverflow)?;
        let buffer = |label| {
            ctx.device.create_buffer(&BufferDescriptor {
                label: Some(label),
                size: bytes,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };
        Ok(Self {
            capacity,
            segment: buffer("resident_clearing_schedule_live_head"),
            next_intake: buffer("resident_clearing_next_generation_intake"),
        })
    }

    pub const fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Encode only byte-identical GPU copies from the graduated exact output.
    /// The first copy appends to the bounded live head; the second binds that
    /// same product representation as the recursive N+1 intake. Both execute
    /// in queue order before any subsequent readback command.
    pub fn encode_append_and_n_plus_one(
        &self,
        encoder: &mut CommandEncoder,
        scratch: &Buffer,
        plan: &ResidentApportionmentPlan,
        reservation: ResidentScheduleReservation,
    ) -> Result<ResidentNPlusOneSubmission, ResidentLiveHeadError> {
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
        let source = u64::from(reservation.start())
            .checked_mul(PRODUCT_BYTES)
            .ok_or(ResidentLiveHeadError::ArithmeticOverflow)?;
        let bytes = u64::from(claim_count)
            .checked_mul(PRODUCT_BYTES)
            .ok_or(ResidentLiveHeadError::ArithmeticOverflow)?;
        if bytes != 0 {
            encoder.copy_buffer_to_buffer(&self.segment, source, &self.next_intake, 0, bytes);
        }
        let intake_generation = plan
            .generation()
            .get()
            .checked_add(1)
            .map(simthing_core::GenerationStamp::new)
            .ok_or(ResidentLiveHeadError::GenerationOverflow)?;
        Ok(ResidentNPlusOneSubmission {
            reservation,
            intake_generation,
            product_count: claim_count,
        })
    }

    /// Proof/observer readback. Production calls this only after the queue has
    /// already received the resident N+1 intake copy.
    pub fn readback_segment(
        &self,
        ctx: &GpuContext,
        submission: ResidentNPlusOneSubmission,
    ) -> Result<Vec<ResidentConstrainedProduct>, ResidentLiveHeadError> {
        readback_products(
            ctx,
            &self.segment,
            submission.reservation.start(),
            submission.product_count,
            "resident_clearing_schedule_materialization",
        )
    }

    /// Referee-only observation of the direct N+1 port. It has the identical
    /// canonical product type; there is no translated seam or role newtype.
    pub fn readback_next_intake_for_proof(
        &self,
        ctx: &GpuContext,
        submission: ResidentNPlusOneSubmission,
    ) -> Result<Vec<ResidentConstrainedProduct>, ResidentLiveHeadError> {
        readback_products(
            ctx,
            &self.next_intake,
            0,
            submission.product_count,
            "resident_clearing_next_intake_proof",
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentNPlusOneSubmission {
    reservation: ResidentScheduleReservation,
    intake_generation: simthing_core::GenerationStamp,
    product_count: u32,
}

impl ResidentNPlusOneSubmission {
    pub const fn reservation(self) -> ResidentScheduleReservation {
        self.reservation
    }

    pub const fn intake_generation(self) -> simthing_core::GenerationStamp {
        self.intake_generation
    }

    pub const fn product_count(self) -> u32 {
        self.product_count
    }
}

fn readback_products<T: Pod>(
    ctx: &GpuContext,
    source: &Buffer,
    start: u32,
    count: u32,
    label: &'static str,
) -> Result<Vec<T>, ResidentLiveHeadError> {
    let offset = u64::from(start)
        .checked_mul(PRODUCT_BYTES)
        .ok_or(ResidentLiveHeadError::ArithmeticOverflow)?;
    let bytes = u64::from(count)
        .checked_mul(PRODUCT_BYTES)
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
    #[error("resident recursive intake generation overflow")]
    GenerationOverflow,
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
