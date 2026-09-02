//! RESIDENT-CLEARING-CUTOVER-0 — ordinary production resident executor.
//!
//! This owner composes the graduated semantic plan, exact apportionment
//! kernel, per-tree buffers, bounded IntegrationSchedule live head, and direct
//! N+1 product intake. It adds no score, pressure, remainder, or recurrence
//! law; `CpuVendorizedOracle` remains a separately selected diagnostic route.

use simthing_core::{
    ColumnIndex, DimensionRegistry, ExecutionIncarnation, GenerationStamp, IntegrationSchedule,
    ResidentClearingScheduleFact, ResidentScheduleError, SimProperty, SimThing, SimThingId,
    SlotIndex, TreeExecutionAuthority, TreeGenerationAuthority, TreeRealmId,
};
use simthing_gpu::{
    GpuContext, ResidentApportionmentClaim, ResidentApportionmentDispatch,
    ResidentApportionmentError, ResidentApportionmentPlan, ResidentApportionmentSession,
    ResidentClearingAdmission, ResidentClearingBudgets, ResidentClearingBufferOwner,
    ResidentClearingBuffers, ResidentClearingGpuError, ResidentClearingLiveHead,
    ResidentClearingPlan, ResidentClearingPlanError, ResidentClearingQualification,
    ResidentConstrainedProduct, ResidentDrawId, ResidentLiveHeadError, ResidentNPlusOneSubmission,
    ResidentOwnerId, ResidentResourceId, ResidentScopeId, SlotAllocator, WorldGpuState,
};
use thiserror::Error;

const RESIDENT_RESOURCE: u64 = 0x5246_434c_4541_5200;
const RESIDENT_SCOPE: u64 = 0x5246_5343_4f50_4500;
const RESIDENT_DRAW_BASE: u64 = 0x5246_4452_4157_0000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResidentClearingBatchBinding {
    pub source_simthing_id: SimThingId,
    pub requested: u32,
    pub available: u32,
    pub precedence: u32,
    /// The already-resolved resident `AllocatedFlow` input. The cutover owner
    /// does not evaluate policy, scan descendants, or query field facilities.
    pub allocated_flow: f32,
}

pub struct ResidentClearingDispatchTicket {
    submission: ResidentNPlusOneSubmission,
}

impl ResidentClearingDispatchTicket {
    pub const fn submission(&self) -> ResidentNPlusOneSubmission {
        self.submission
    }
}

/// One independently-owned production executor per tree.
pub struct ResidentClearingRuntime {
    realm: TreeRealmId,
    qualification: ResidentClearingQualification,
    semantic_plan: ResidentClearingPlan,
    buffers: ResidentClearingBuffers,
    exact_session: ResidentApportionmentSession,
    allocated_flow_plane: WorldGpuState,
    allocated_flow_values: Vec<f32>,
    live_head: ResidentClearingLiveHead,
    lane_capacity: u32,
}

impl ResidentClearingRuntime {
    #[allow(clippy::too_many_arguments)]
    pub fn admit(
        ctx: &GpuContext,
        realm: TreeRealmId,
        root: &SimThing,
        registry: &DimensionRegistry,
        residency: &SlotAllocator,
        schedule: &IntegrationSchedule,
        generation: GenerationStamp,
        lane_capacity: u32,
    ) -> Result<Self, ResidentClearingRuntimeError> {
        if lane_capacity == 0 {
            return Err(ResidentClearingRuntimeError::ZeroLaneCapacity);
        }
        let qualification = ResidentClearingQualification::admit(ctx)?;
        let generation_authority = TreeGenerationAuthority::new(generation);
        let authority = TreeExecutionAuthority::seal(
            realm,
            ExecutionIncarnation::new(1)
                .map_err(|error| ResidentClearingRuntimeError::Identity(error.to_string()))?,
            root,
            &generation_authority,
            schedule,
            registry,
            residency,
        )
        .map_err(|error| ResidentClearingRuntimeError::Identity(error.to_string()))?;
        let context = authority
            .seal_context()
            .map_err(|error| ResidentClearingRuntimeError::Identity(error.to_string()))?;
        let binding = context
            .bind(&authority)
            .map_err(|error| ResidentClearingRuntimeError::Identity(error.to_string()))?;
        let owner = ResidentOwnerId::new(context.qualify(root.id));
        let admissions = (0..lane_capacity).map(|lane| ResidentClearingAdmission {
            owner,
            resource: ResidentResourceId::new(RESIDENT_RESOURCE),
            scope: ResidentScopeId::new(RESIDENT_SCOPE),
            draw: ResidentDrawId::new(RESIDENT_DRAW_BASE + u64::from(lane)),
        });
        let scratch_bytes = u64::from(lane_capacity)
            .checked_mul(64)
            .ok_or(ResidentClearingRuntimeError::ArithmeticOverflow)?;
        let semantic_bytes = u64::from(lane_capacity)
            .checked_mul(128)
            .and_then(|bytes| bytes.checked_add(4096))
            .ok_or(ResidentClearingRuntimeError::ArithmeticOverflow)?;
        let resident_bytes = u64::from(lane_capacity)
            .checked_mul(192)
            .and_then(|bytes| bytes.checked_add(4096))
            .ok_or(ResidentClearingRuntimeError::ArithmeticOverflow)?;
        let budgets = ResidentClearingBudgets::new(
            1,
            1,
            1,
            lane_capacity,
            lane_capacity,
            semantic_bytes,
            resident_bytes,
            scratch_bytes,
            64,
        )?;
        let semantic_plan = ResidentClearingPlan::build(&binding, admissions, budgets)?;
        let buffers = ResidentClearingBuffers::allocate(&ctx.device, &binding, &semantic_plan)?;
        drop(binding);
        drop(context);
        drop(authority);

        let mut flow_registry = DimensionRegistry::new();
        flow_registry.register(SimProperty::simple(
            "resident-clearing",
            "allocated-flow",
            1,
        ));
        let allocated_flow_plane =
            WorldGpuState::new(ctx.shared_device_context(), &flow_registry, lane_capacity);
        let allocated_flow_values = vec![0.0; allocated_flow_plane.values_len()];
        allocated_flow_plane.install_resolved_values_at_boundary(&allocated_flow_values);
        let exact_session = ResidentApportionmentSession::new(&allocated_flow_plane.ctx);
        let live_head = ResidentClearingLiveHead::admit(&allocated_flow_plane.ctx, lane_capacity)?;
        Ok(Self {
            realm,
            qualification,
            semantic_plan,
            buffers,
            exact_session,
            allocated_flow_plane,
            allocated_flow_values,
            live_head,
            lane_capacity,
        })
    }

    pub const fn realm(&self) -> TreeRealmId {
        self.realm
    }

    pub fn qualification(&self) -> &ResidentClearingQualification {
        &self.qualification
    }

    pub const fn lane_capacity(&self) -> u32 {
        self.lane_capacity
    }

    /// Stable semantic identity of this tree's resident allocation. This is a
    /// proof/diagnostic view; it exposes no device handle or physical row.
    pub const fn buffer_owner(&self) -> ResidentClearingBufferOwner {
        self.buffers.owner()
    }

    /// Submit resident exact settlement, schedule append, and identical N+1
    /// intake in queue order. This returns before any host readback or vector
    /// IntegrationSchedule append.
    pub fn dispatch(
        &mut self,
        schedule: &mut IntegrationSchedule,
        granter: SimThingId,
        generation: GenerationStamp,
        rows: &[ResidentClearingBatchBinding],
    ) -> Result<ResidentClearingDispatchTicket, ResidentClearingRuntimeError> {
        if rows.len() > self.lane_capacity as usize {
            return Err(ResidentClearingRuntimeError::ClaimCapacityExceeded {
                claims: rows.len(),
                admitted: self.lane_capacity,
            });
        }
        if rows.is_empty() {
            return Err(ResidentClearingRuntimeError::ZeroClaimBatch);
        }
        if rows
            .windows(2)
            .any(|pair| pair[0].source_simthing_id >= pair[1].source_simthing_id)
        {
            return Err(ResidentClearingRuntimeError::NonCanonicalClaimOrder);
        }
        self.allocated_flow_values.fill(0.0);
        let claims = rows
            .iter()
            .enumerate()
            .map(|(lane, row)| {
                self.allocated_flow_values[lane] = row.allocated_flow;
                ResidentApportionmentClaim::new(
                    self.semantic_row_for_lane(lane as u32),
                    row.source_simthing_id,
                    row.requested,
                    row.available,
                    row.precedence,
                    SlotIndex::new(lane as u32),
                    ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
                )
            })
            .collect();
        self.allocated_flow_plane
            .install_resolved_values_at_boundary(&self.allocated_flow_values);
        let plan =
            ResidentApportionmentPlan::build(&self.semantic_plan, claims, granter, generation, 0)?;
        let count = u32::try_from(plan.claims().len())
            .map_err(|_| ResidentClearingRuntimeError::ArithmeticOverflow)?;
        let reservation = schedule.reserve_resident_rows(count)?;
        let (semantic_rows, scratch) = self.buffers.apportionment_buffers(&plan)?;
        let mut encoder = self.allocated_flow_plane.ctx.device.create_command_encoder(
            &simthing_gpu::wgpu::CommandEncoderDescriptor {
                label: Some("resident_clearing_production_dispatch"),
            },
        );
        self.allocated_flow_plane
            .encode_resident_apportionment_with_dispatch_into(
                &mut self.exact_session,
                &mut encoder,
                semantic_rows,
                scratch,
                &plan,
                ResidentApportionmentDispatch::single_pass(),
            )?;
        let submission = self.live_head.encode_append_and_n_plus_one(
            &mut encoder,
            scratch,
            &plan,
            reservation,
        )?;
        self.allocated_flow_plane
            .ctx
            .queue
            .submit(Some(encoder.finish()));
        Ok(ResidentClearingDispatchTicket { submission })
    }

    /// Asynchronous observer/materializer. The dispatch ticket proves N+1 was
    /// already submitted before this method can map either buffer.
    pub fn materialize(
        &mut self,
        schedule: &mut IntegrationSchedule,
        ticket: ResidentClearingDispatchTicket,
    ) -> Result<Vec<ResidentConstrainedProduct>, ResidentClearingRuntimeError> {
        let resident = self
            .live_head
            .readback_segment(&self.allocated_flow_plane.ctx, ticket.submission)?;
        if resident.iter().any(|product| !product.is_successful()) {
            return Err(ResidentClearingRuntimeError::ResidentProductFailure);
        }
        let facts: Vec<_> = resident
            .iter()
            .copied()
            .map(|product| ResidentClearingScheduleFact {
                semantic_row: product.semantic_row(),
                source_simthing_id_raw: product.source_simthing_id().raw(),
                granted: product.granted(),
                unresolved: product.unresolved(),
                generation: product.generation(),
                integration_band: product.integration_band(),
            })
            .collect();
        schedule.materialize_resident_rows(ticket.submission.reservation(), &facts)?;
        Ok(resident)
    }

    pub fn readback_next_intake_for_proof(
        &self,
        ticket: &ResidentClearingDispatchTicket,
    ) -> Result<Vec<ResidentConstrainedProduct>, ResidentClearingRuntimeError> {
        Ok(self
            .live_head
            .readback_next_intake_for_proof(&self.allocated_flow_plane.ctx, ticket.submission)?)
    }

    fn semantic_row_for_lane(&self, lane: u32) -> u32 {
        let draw = RESIDENT_DRAW_BASE + u64::from(lane);
        self.semantic_plan
            .rows()
            .iter()
            .position(|row| {
                self.semantic_plan.dictionaries().draws()[row.draw().get() as usize].get() == draw
            })
            .and_then(|index| u32::try_from(index).ok())
            .expect("admitted resident lane has one canonical semantic row")
    }
}

#[derive(Debug, Error)]
pub enum ResidentClearingRuntimeError {
    #[error("resident clearing lane capacity must be at least one")]
    ZeroLaneCapacity,
    #[error("resident clearing identity admission failed: {0}")]
    Identity(String),
    #[error("resident clearing arithmetic overflow")]
    ArithmeticOverflow,
    #[error("resident clear has {claims} claims, admitted {admitted}")]
    ClaimCapacityExceeded { claims: usize, admitted: u32 },
    #[error("resident clearing dispatch requires at least one claim")]
    ZeroClaimBatch,
    #[error("resident claims must arrive in strict logical SimThing-id order")]
    NonCanonicalClaimOrder,
    #[error("resident exact output reported a typed GPU product failure")]
    ResidentProductFailure,
    #[error(transparent)]
    Plan(#[from] ResidentClearingPlanError),
    #[error(transparent)]
    Gpu(#[from] ResidentClearingGpuError),
    #[error(transparent)]
    Exact(#[from] ResidentApportionmentError),
    #[error(transparent)]
    LiveHead(#[from] ResidentLiveHeadError),
    #[error(transparent)]
    Schedule(#[from] ResidentScheduleError),
}
