//! RESIDENT-CLEARING-CUTOVER-0 — ordinary production resident executor.
//!
//! This owner composes the graduated semantic plan, exact apportionment
//! kernel, per-tree buffers, bounded IntegrationSchedule live head, and direct
//! N+1 product intake. It adds no score, pressure, remainder, or recurrence
//! law; `CpuVendorizedOracle` remains a separately selected diagnostic route.

use std::collections::BTreeMap;

use simthing_core::{
    expand_arena_internal_columns, AccumulatorRole, AccumulatorSpec, ClampBehavior,
    DimensionRegistry, EmlExpressionRegistry, ExecutionIncarnation, GenerationStamp,
    IntegrationSchedule, LogTier, PersistenceDeformationProgram, PropertyLayout,
    ResidentClearingScheduleFact, ResidentScheduleError, SimProperty, SimPropertyId, SimThing,
    SimThingId, SlotIndex, SubFieldRole, SubFieldSpec, TreeExecutionAuthority,
    TreeGenerationAuthority, TreeRealmId,
};
use simthing_gpu::{
    AccumulatorOpSession, EmlGpuProgramTable, GpuContext, PackedAccumulatorUpload,
    ResidentApportionmentClaim, ResidentApportionmentDispatch, ResidentApportionmentError,
    ResidentApportionmentPlan, ResidentApportionmentSession, ResidentClearingAdmission,
    ResidentClearingBudgets, ResidentClearingBufferOwner, ResidentClearingBuffers,
    ResidentClearingGpuError, ResidentClearingLiveHead, ResidentClearingPlan,
    ResidentClearingPlanError, ResidentClearingQualification, ResidentConstrainedProduct,
    ResidentDrawId, ResidentLiveHeadError, ResidentNPlusOneSubmission, ResidentOwnerId,
    ResidentRecursiveIntakeTransformSession, ResidentResourceId, ResidentScopeId, SlotAllocator,
    WorldGpuState,
};
use thiserror::Error;

use crate::arena_hierarchy::{build_custom_layout, HierarchyNode, NodeColumnRefs};
use crate::arena_registry::{FissionPolicy, GpuArenaDescriptor};

const RESIDENT_RESOURCE: u64 = 0x5246_434c_4541_5200;
const RESIDENT_SCOPE: u64 = 0x5246_5343_4f50_4500;
const RESIDENT_DRAW_BASE: u64 = 0x5246_4452_4157_0000;
const RESIDENT_ALLOCATION_ARENA: &str = "resident-clearing-continuous-allocation";

fn resident_continuous_registry(
) -> Result<(DimensionRegistry, SimPropertyId, NodeColumnRefs), ResidentClearingRuntimeError> {
    let field = |name: &str, role: AccumulatorRole| SubFieldSpec {
        role: SubFieldRole::Named(name.into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: name.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: Some(AccumulatorSpec {
            role,
            log_tier: LogTier::Summary,
        }),
    };
    let layout = expand_arena_internal_columns(PropertyLayout {
        sub_fields: vec![
            field("intrinsic-flow", AccumulatorRole::IntrinsicFlow),
            field(
                "allocated-flow",
                AccumulatorRole::AllocatedFlow {
                    arena: RESIDENT_ALLOCATION_ARENA.into(),
                },
            ),
            field(
                "allocator-weight",
                AccumulatorRole::AllocatorWeight {
                    arena: RESIDENT_ALLOCATION_ARENA.into(),
                },
            ),
        ],
    });
    let mut property = SimProperty::simple("resident-clearing", "continuous-allocation", 0);
    property.layout = layout;
    let mut registry = DimensionRegistry::new();
    let property_id = registry.register(property);
    let cols = crate::arena_hierarchy::resolve_node_columns_for_property(
        &registry,
        property_id,
        RESIDENT_ALLOCATION_ARENA,
    )
    .map_err(|error| ResidentClearingRuntimeError::ContinuousAllocation(error.to_string()))?;
    Ok((registry, property_id, cols))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResidentClearingBatchBinding {
    pub source_simthing_id: SimThingId,
    pub requested: u32,
    pub available: u32,
    pub precedence: u32,
    /// Existing upstream eligible pressure bound to the graduated allocator's
    /// `AllocatorWeight` operand. Under the neutral profile this is the request
    /// magnitude; settlement receives only the AllocatedFlow that the real
    /// child-share evaluator emits.
    pub continuous_weight: f32,
}

/// Application-layer policy binding for the one resident semantic scope.
/// Claimant identity selects an already-admitted EML program; the binding
/// carries no demand quantity and cannot create another recursive intake.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResidentPersistenceDeformationBinding {
    pub source_simthing_id: SimThingId,
    pub program: PersistenceDeformationProgram,
}

pub struct ResidentClearingDispatchTicket {
    submission: ResidentNPlusOneSubmission,
    consumed_resident_intake: bool,
}

impl ResidentClearingDispatchTicket {
    pub const fn submission(&self) -> ResidentNPlusOneSubmission {
        self.submission
    }

    pub const fn consumed_resident_intake(&self) -> bool {
        self.consumed_resident_intake
    }
}

/// One independently-owned production executor per tree.
pub struct ResidentClearingRuntime {
    realm: TreeRealmId,
    qualification: ResidentClearingQualification,
    semantic_plan: ResidentClearingPlan,
    buffers: ResidentClearingBuffers,
    exact_session: ResidentApportionmentSession,
    continuous_property: SimPropertyId,
    continuous_cols: NodeColumnRefs,
    continuous_eml: EmlExpressionRegistry,
    continuous_eml_table: EmlGpuProgramTable,
    continuous_session: AccumulatorOpSession,
    continuous_plane: WorldGpuState,
    continuous_values: Vec<f32>,
    live_head: ResidentClearingLiveHead,
    lane_capacity: u32,
    pending_intake: Option<ResidentNPlusOneSubmission>,
    recursive_plan_template: Option<ResidentApportionmentPlan>,
    persistence_deformations: BTreeMap<SimThingId, PersistenceDeformationProgram>,
    intake_transform_session: Option<ResidentRecursiveIntakeTransformSession>,
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
        Self::admit_with_persistence_deformations(
            ctx,
            realm,
            root,
            registry,
            residency,
            schedule,
            generation,
            lane_capacity,
            &[],
        )
    }

    /// Admit the same production executor with optional sealed policy. The
    /// map is immutable for the executor lifetime and is consumed only while
    /// the existing recursive plan is minted.
    #[allow(clippy::too_many_arguments)]
    pub fn admit_with_persistence_deformations(
        ctx: &GpuContext,
        realm: TreeRealmId,
        root: &SimThing,
        registry: &DimensionRegistry,
        residency: &SlotAllocator,
        schedule: &IntegrationSchedule,
        generation: GenerationStamp,
        lane_capacity: u32,
        deformation_bindings: &[ResidentPersistenceDeformationBinding],
    ) -> Result<Self, ResidentClearingRuntimeError> {
        if lane_capacity == 0 {
            return Err(ResidentClearingRuntimeError::ZeroLaneCapacity);
        }
        let mut persistence_deformations = BTreeMap::new();
        for binding in deformation_bindings {
            if persistence_deformations
                .insert(binding.source_simthing_id, binding.program.clone())
                .is_some()
            {
                return Err(
                    ResidentClearingRuntimeError::DuplicatePersistenceDeformation {
                        source_id: binding.source_simthing_id,
                    },
                );
            }
        }
        let qualification = ResidentClearingQualification::admit(ctx)?;
        let intake_transform_session = if persistence_deformations.is_empty() {
            None
        } else {
            Some(ResidentRecursiveIntakeTransformSession::new(ctx))
        };
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

        let (continuous_registry, continuous_property, continuous_cols) =
            resident_continuous_registry()?;
        let continuous_slots = lane_capacity
            .checked_add(1)
            .ok_or(ResidentClearingRuntimeError::ArithmeticOverflow)?;
        let continuous_plane = WorldGpuState::new(
            ctx.shared_device_context(),
            &continuous_registry,
            continuous_slots,
        );
        let continuous_values = vec![0.0; continuous_plane.values_len()];
        continuous_plane.install_resolved_values_at_boundary(&continuous_values);

        let mut continuous_eml = EmlExpressionRegistry::new();
        crate::child_share_eml::register_child_share_formula(&mut continuous_eml, continuous_cols)
            .map_err(|error| {
                ResidentClearingRuntimeError::ContinuousAllocation(error.to_string())
            })?;
        let upload_rows: Vec<_> = continuous_eml
            .formulas_for_gpu_upload()
            .map(|(id, meta, nodes)| {
                (
                    id,
                    meta.clone(),
                    nodes
                        .iter()
                        .map(|node| simthing_core::EmlNodeGpu {
                            opcode: node.opcode,
                            flags: node.flags,
                            a: node.a,
                            b: node.b,
                            c: node.c,
                            d: node.d,
                        })
                        .collect(),
                )
            })
            .collect();
        let mut continuous_eml_table = EmlGpuProgramTable::new(&continuous_plane.ctx, 64, 4);
        for (tree_id, range_index) in continuous_eml_table
            .upload_trees(&continuous_plane.ctx, &upload_rows)
            .map_err(|error| {
                ResidentClearingRuntimeError::ContinuousAllocation(error.to_string())
            })?
        {
            continuous_eml
                .mark_tree_uploaded(tree_id, range_index, continuous_eml_table.generation)
                .map_err(|error| {
                    ResidentClearingRuntimeError::ContinuousAllocation(error.to_string())
                })?;
        }
        let continuous_session = AccumulatorOpSession::new_attached(
            &continuous_plane.ctx,
            continuous_slots,
            continuous_plane.n_dims,
            1,
        );
        let exact_session = ResidentApportionmentSession::new(&continuous_plane.ctx);
        let live_head_capacity = schedule
            .resident_live_head_capacity()
            .ok_or(ResidentClearingRuntimeError::ZeroLaneCapacity)?;
        let live_head = ResidentClearingLiveHead::admit(&continuous_plane.ctx, live_head_capacity)?;
        Ok(Self {
            realm,
            qualification,
            semantic_plan,
            buffers,
            exact_session,
            continuous_property,
            continuous_cols,
            continuous_eml,
            continuous_eml_table,
            continuous_session,
            continuous_plane,
            continuous_values,
            live_head,
            lane_capacity,
            pending_intake: None,
            recursive_plan_template: None,
            persistence_deformations,
            intake_transform_session,
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
    /// IntegrationSchedule append. `Some(rows)` is the lawful root intake;
    /// `None` advances an interior generation directly from the prior resident
    /// `T_s` and refuses when no such intake is pending.
    pub fn dispatch(
        &mut self,
        schedule: &mut IntegrationSchedule,
        granter: SimThingId,
        generation: GenerationStamp,
        root_rows: Option<&[ResidentClearingBatchBinding]>,
    ) -> Result<ResidentClearingDispatchTicket, ResidentClearingRuntimeError> {
        let consumed_resident_intake = root_rows.is_none();
        let (plan, continuous_plan, prior_submission) = if let Some(rows) = root_rows {
            self.validate_root_rows(rows)?;
            let continuous_plan = self.prepare_root_continuous_allocation(granter, rows)?;
            let claims = rows
                .iter()
                .enumerate()
                .map(|(lane, row)| {
                    ResidentApportionmentClaim::new(
                        self.semantic_row_for_lane(lane as u32),
                        row.source_simthing_id,
                        row.requested,
                        row.available,
                        row.precedence,
                        SlotIndex::new(lane as u32 + 1),
                        self.continuous_cols.allocated_flow_col,
                    )
                })
                .collect();
            let plan = ResidentApportionmentPlan::build(
                &self.semantic_plan,
                claims,
                granter,
                generation,
                continuous_plan.integration_band,
            )?
            .with_persistence_deformations(rows.iter().enumerate().filter_map(|(lane, row)| {
                if row.requested == 0 {
                    return None;
                }
                self.persistence_deformations
                    .get(&row.source_simthing_id)
                    .cloned()
                    .map(|program| (self.semantic_row_for_lane(lane as u32), program))
            }))?;
            (plan, Some(continuous_plan), None)
        } else {
            let prior_submission = self
                .pending_intake
                .ok_or(ResidentClearingRuntimeError::MissingResidentIntake)?;
            let template = self
                .recursive_plan_template
                .as_ref()
                .ok_or(ResidentClearingRuntimeError::MissingResidentIntake)?;
            if template.authority_granter() != granter {
                return Err(ResidentClearingRuntimeError::RecursiveGranterMismatch {
                    expected: template.authority_granter(),
                    observed: granter,
                });
            }
            (
                template.for_recursive_intake_generation(generation),
                None,
                Some(prior_submission),
            )
        };
        let count = u32::try_from(plan.claims().len())
            .map_err(|_| ResidentClearingRuntimeError::ArithmeticOverflow)?;
        let reservation = schedule.reserve_resident_rows(count)?;
        let (semantic_rows, scratch) = self.buffers.apportionment_buffers(&plan)?;
        let mut encoder = self.continuous_plane.ctx.device.create_command_encoder(
            &simthing_gpu::wgpu::CommandEncoderDescriptor {
                label: Some("resident_clearing_production_dispatch"),
            },
        );
        if let Some(continuous_plan) = continuous_plan.as_ref() {
            self.continuous_plane.encode_accumulator_orderband_into(
                &mut self.continuous_session,
                &mut encoder,
                continuous_plan.n_bands,
                0.0,
                Some(&self.continuous_eml_table),
                false,
            );
            self.continuous_plane
                .encode_resident_apportionment_with_dispatch_into(
                    &mut self.exact_session,
                    &mut encoder,
                    semantic_rows,
                    scratch,
                    &plan,
                    ResidentApportionmentDispatch::single_pass(),
                )?;
        } else {
            self.live_head.encode_recursive_apportionment(
                &self.continuous_plane,
                &mut self.exact_session,
                &mut encoder,
                semantic_rows,
                scratch,
                &plan,
                prior_submission.expect("recursive dispatch proved a pending intake"),
            )?;
        }
        let submission = if plan.has_persistence_deformations() {
            self.live_head
                .encode_append_and_n_plus_one_with_deformation(
                    &self.continuous_plane.ctx,
                    self.intake_transform_session
                        .as_ref()
                        .expect("admitted deformation has one resident mint session"),
                    &mut encoder,
                    scratch,
                    &plan,
                    reservation,
                )?
        } else {
            self.live_head.encode_append_and_n_plus_one(
                &mut encoder,
                scratch,
                &plan,
                reservation,
            )?
        };
        self.continuous_plane
            .ctx
            .queue
            .submit(Some(encoder.finish()));
        if !consumed_resident_intake {
            self.recursive_plan_template = Some(plan.clone());
        }
        self.pending_intake = Some(submission);
        Ok(ResidentClearingDispatchTicket {
            submission,
            consumed_resident_intake,
        })
    }

    fn validate_root_rows(
        &self,
        rows: &[ResidentClearingBatchBinding],
    ) -> Result<(), ResidentClearingRuntimeError> {
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
        if let Some(row) = rows
            .iter()
            .find(|row| !row.continuous_weight.is_finite() || row.continuous_weight < 0.0)
        {
            return Err(ResidentClearingRuntimeError::InvalidContinuousWeight {
                source_id: row.source_simthing_id,
            });
        }
        Ok(())
    }

    fn prepare_root_continuous_allocation(
        &mut self,
        granter: SimThingId,
        rows: &[ResidentClearingBatchBinding],
    ) -> Result<crate::ArenaAllocationPlan, ResidentClearingRuntimeError> {
        let children = rows
            .iter()
            .enumerate()
            .map(|(lane, row)| HierarchyNode {
                participant_slot: SlotIndex::new(lane as u32 + 1),
                hosted_simthing_id: row.source_simthing_id,
                depth: 1,
                children: Vec::new(),
                cols: self.continuous_cols,
            })
            .collect();
        let descriptor = GpuArenaDescriptor {
            name: RESIDENT_ALLOCATION_ARENA.into(),
            flow_property_id: self.continuous_property,
            balance_property_id: None,
            max_participants: self.lane_capacity.saturating_add(1),
            max_coupling_fanout: 0,
            max_orderband_depth: 16,
            fission_policy: FissionPolicy::default(),
            participant_range: (0, 0),
            wildcard_max_expansion: None,
            reserved_orderband_depth: 0,
        };
        let layout = build_custom_layout(
            0,
            &descriptor,
            self.continuous_cols,
            vec![HierarchyNode {
                participant_slot: SlotIndex::new(0),
                hosted_simthing_id: granter,
                depth: 0,
                children,
                cols: self.continuous_cols,
            }],
        )
        .map_err(|error| ResidentClearingRuntimeError::ContinuousAllocation(error.to_string()))?;
        let plan = crate::plan_arena_allocation_with_pressure(
            &layout,
            &[],
            self.continuous_plane.n_slots,
            &[],
            &[],
            GenerationStamp::new(0),
            GenerationStamp::new(1),
        )
        .map_err(|error| ResidentClearingRuntimeError::ContinuousAllocation(error.to_string()))?;
        let upload = PackedAccumulatorUpload::from_ops_resolving_input_lists_with_eml(
            &plan.cpu_ops,
            Some(&self.continuous_eml),
        )
        .map_err(|error| ResidentClearingRuntimeError::ContinuousAllocation(error.to_string()))?;
        self.continuous_session
            .upload_packed_ops(&self.continuous_plane.ctx, &upload)
            .map_err(|error| {
                ResidentClearingRuntimeError::ContinuousAllocation(error.to_string())
            })?;

        self.continuous_values.fill(0.0);
        let requested_total = rows.iter().try_fold(0u64, |total, row| {
            total
                .checked_add(u64::from(row.requested))
                .ok_or(ResidentClearingRuntimeError::ArithmeticOverflow)
        })?;
        let n_dims = self.continuous_plane.n_dims as usize;
        self.continuous_values[self.continuous_cols.intrinsic_flow_col.raw()] =
            requested_total as f32;
        for (lane, row) in rows.iter().enumerate() {
            let slot = lane + 1;
            self.continuous_values[slot * n_dims + self.continuous_cols.weight_col.raw()] =
                row.continuous_weight;
        }
        self.continuous_plane
            .install_resolved_values_at_boundary(&self.continuous_values);
        Ok(plan)
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
            .readback_segment(&self.continuous_plane.ctx, ticket.submission)?;
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
            .readback_next_intake_for_proof(&self.continuous_plane.ctx, ticket.submission)?)
    }

    /// Referee-only observation of values already emitted by the production
    /// child-share EML. Exact settlement reads these cells on-device before
    /// this diagnostic mapping is possible.
    pub fn readback_allocated_flow_for_proof(&self, count: u32) -> Vec<f32> {
        let values = self.continuous_plane.read_values();
        let n_dims = self.continuous_plane.n_dims as usize;
        (0..count.min(self.lane_capacity))
            .map(|lane| {
                values[(lane as usize + 1) * n_dims + self.continuous_cols.allocated_flow_col.raw()]
            })
            .collect()
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
    #[error("resident interior dispatch has no prior canonical T_s intake")]
    MissingResidentIntake,
    #[error("resident recursive granter is {observed:?}, expected root authority {expected:?}")]
    RecursiveGranterMismatch {
        expected: SimThingId,
        observed: SimThingId,
    },
    #[error("resident claims must arrive in strict logical SimThing-id order")]
    NonCanonicalClaimOrder,
    #[error("resident continuous weight for {source_id:?} is non-finite or negative")]
    InvalidContinuousWeight { source_id: SimThingId },
    #[error("duplicate resident persistence deformation for claimant {source_id:?}")]
    DuplicatePersistenceDeformation { source_id: SimThingId },
    #[error("resident continuous allocation failed: {0}")]
    ContinuousAllocation(String),
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
