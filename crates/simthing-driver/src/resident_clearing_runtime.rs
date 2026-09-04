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
    ResidencyCapacityPartition, ResidentClearingScheduleFact, ResidentScheduleError, SimProperty,
    SimPropertyId, SimThing, SimThingId, SlotIndex, SubFieldRole, SubFieldSpec,
    TreeExecutionAuthority, TreeGenerationAuthority, TreeRealmId,
};
use simthing_gpu::{
    AccumulatorOpSession, EmlGpuProgramTable, GpuContext, PackedAccumulatorUpload,
    ResidentApportionmentClaim, ResidentApportionmentDispatch, ResidentApportionmentError,
    ResidentApportionmentPlan, ResidentApportionmentSession, ResidentClearingAdmission,
    ResidentClearingBudgets, ResidentClearingBufferOwner, ResidentClearingBuffers,
    ResidentClearingGpuError, ResidentClearingLiveHead, ResidentClearingPlan,
    ResidentClearingPlanError, ResidentClearingQualification, ResidentClearingSubmission,
    ResidentConstrainedProduct, ResidentDrawId, ResidentLiveHeadError, ResidentOwnerId,
    ResidentResourceId, ResidentScopeId, ResidentTemporalDemand, ResidentTemporalDemandMintSession,
    ResidentTemporalDemandSubmission, SlotAllocator, WorldGpuState,
};
use thiserror::Error;

use crate::arena_hierarchy::{build_custom_layout, HierarchyNode, NodeColumnRefs};
use crate::arena_registry::{FissionPolicy, GpuArenaDescriptor};

const RESIDENT_RESOURCE: u64 = 0x5246_434c_4541_5200;
const RESIDENT_SCOPE: u64 = 0x5246_5343_4f50_4500;
const RESIDENT_DRAW_BASE: u64 = 0x5246_4452_4157_0000;
const RESIDENT_ALLOCATION_ARENA: &str = "resident-clearing-continuous-allocation";

fn resident_scope_id(owner: SimThingId) -> ResidentScopeId {
    ResidentScopeId::new(RESIDENT_SCOPE ^ u64::from(owner.raw()))
}

fn collect_subtree_ids(node: &SimThing, ids: &mut Vec<SimThingId>) {
    ids.push(node.id);
    for child in &node.children {
        collect_subtree_ids(child, ids);
    }
}

fn collect_descendant_sets(
    node: &SimThing,
    sets: &mut BTreeMap<SimThingId, std::collections::BTreeSet<SimThingId>>,
) -> std::collections::BTreeSet<SimThingId> {
    let mut descendants = std::collections::BTreeSet::new();
    for child in &node.children {
        descendants.insert(child.id);
        descendants.extend(collect_descendant_sets(child, sets));
    }
    sets.insert(node.id, descendants.clone());
    descendants
}

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

/// One descendant claim in a same-generation child market. Supply is absent
/// by construction because it comes only from immutable parent `T_s.G`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResidentSpatialClaimBinding {
    pub source_simthing_id: SimThingId,
    pub requested: u32,
    pub precedence: u32,
    pub continuous_weight: f32,
}

/// Authored portion of one ordinary N+1 demand row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentAuthoredDemand {
    pub source_simthing_id: SimThingId,
    pub quantity: u32,
}

/// Inputs that become authoritative only when N+1 executes. Demand quantity
/// is absent because the once-minted resident demand buffer owns it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResidentTemporalExecutionBinding {
    pub source_simthing_id: SimThingId,
    pub available: u32,
    pub precedence: u32,
    pub continuous_weight: f32,
}

/// Application-layer policy binding for the one resident semantic scope.
/// Claimant identity selects an already-admitted EML program; the binding
/// carries no authored demand quantity and cannot create another mint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResidentPersistenceDeformationBinding {
    pub source_simthing_id: SimThingId,
    pub program: PersistenceDeformationProgram,
}

pub struct ResidentClearingDispatchTicket {
    submission: ResidentClearingSubmission,
    plan: ResidentApportionmentPlan,
    semantic_scope_owner: SimThingId,
}

impl ResidentClearingDispatchTicket {
    pub const fn submission(&self) -> ResidentClearingSubmission {
        self.submission
    }

    pub const fn semantic_scope_owner(&self) -> SimThingId {
        self.semantic_scope_owner
    }
}

pub struct ResidentTemporalDemandTicket {
    submission: ResidentTemporalDemandSubmission,
    sources: Vec<SimThingId>,
    authority_granter: SimThingId,
    semantic_scope_owner: SimThingId,
}

#[derive(Clone, Copy)]
enum ResidentDispatchInput {
    Immediate,
    Spatial(ResidentClearingSubmission),
    Temporal(ResidentTemporalDemandSubmission),
}

impl ResidentTemporalDemandTicket {
    pub const fn submission(&self) -> ResidentTemporalDemandSubmission {
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
    continuous_property: SimPropertyId,
    continuous_cols: NodeColumnRefs,
    continuous_eml: EmlExpressionRegistry,
    continuous_eml_table: EmlGpuProgramTable,
    continuous_session: AccumulatorOpSession,
    continuous_plane: WorldGpuState,
    continuous_values: Vec<f32>,
    live_head: ResidentClearingLiveHead,
    lane_capacity: u32,
    root_scope_owner: SimThingId,
    admitted_scope_owners: std::collections::BTreeSet<SimThingId>,
    descendants_by_scope_owner: BTreeMap<SimThingId, std::collections::BTreeSet<SimThingId>>,
    persistence_deformations: BTreeMap<SimThingId, PersistenceDeformationProgram>,
    temporal_mint_session: ResidentTemporalDemandMintSession,
}

/// Canonical Phase-15 API name for the existing resident R->Q executor.
/// Type identity is deliberate: filter vocabulary adds no adapter, storage,
/// arithmetic, or second runtime authority.
pub type RecursiveResourceFilterRuntime = ResidentClearingRuntime;

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
    /// map is immutable for the executor lifetime and is consumed only by the
    /// one Current-to-Next demand mint.
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
        let temporal_mint_session = ResidentTemporalDemandMintSession::new(ctx);
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
        let mut scope_owners = Vec::new();
        collect_subtree_ids(root, &mut scope_owners);
        scope_owners.sort_unstable();
        scope_owners.dedup();
        let admitted_scope_owners: std::collections::BTreeSet<_> =
            scope_owners.iter().copied().collect();
        let mut descendants_by_scope_owner = BTreeMap::new();
        collect_descendant_sets(root, &mut descendants_by_scope_owner);
        let semantic_row_count = u32::try_from(scope_owners.len())
            .ok()
            .and_then(|owners| owners.checked_mul(lane_capacity))
            .ok_or(ResidentClearingRuntimeError::ArithmeticOverflow)?;
        let admissions = scope_owners.iter().flat_map(|scope_owner| {
            let owner = ResidentOwnerId::new(context.qualify(*scope_owner));
            (0..lane_capacity).map(move |lane| ResidentClearingAdmission {
                owner,
                resource: ResidentResourceId::new(RESIDENT_RESOURCE),
                scope: resident_scope_id(*scope_owner),
                draw: ResidentDrawId::new(RESIDENT_DRAW_BASE + u64::from(lane)),
            })
        });
        let scratch_bytes = u64::from(semantic_row_count)
            .checked_mul(64)
            .ok_or(ResidentClearingRuntimeError::ArithmeticOverflow)?;
        let semantic_bytes = u64::from(semantic_row_count)
            .checked_mul(128)
            .and_then(|bytes| bytes.checked_add(4096))
            .ok_or(ResidentClearingRuntimeError::ArithmeticOverflow)?;
        let resident_bytes = u64::from(semantic_row_count)
            .checked_mul(192)
            .and_then(|bytes| bytes.checked_add(4096))
            .ok_or(ResidentClearingRuntimeError::ArithmeticOverflow)?;
        let budgets = ResidentClearingBudgets::new(
            u32::try_from(scope_owners.len())
                .map_err(|_| ResidentClearingRuntimeError::ArithmeticOverflow)?,
            1,
            u32::try_from(scope_owners.len())
                .map_err(|_| ResidentClearingRuntimeError::ArithmeticOverflow)?,
            lane_capacity,
            semantic_row_count,
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
        let continuous_slots = semantic_row_count
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
            root_scope_owner: root.id,
            admitted_scope_owners,
            descendants_by_scope_owner,
            persistence_deformations,
            temporal_mint_session,
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

    /// Submit one immediate-flow market. Precedence orders feasible work and
    /// no request reserves capacity.
    pub fn dispatch(
        &mut self,
        schedule: &mut IntegrationSchedule,
        granter: SimThingId,
        generation: GenerationStamp,
        rows: &[ResidentClearingBatchBinding],
    ) -> Result<ResidentClearingDispatchTicket, ResidentClearingRuntimeError> {
        self.dispatch_market(
            schedule,
            granter,
            generation,
            self.root_scope_owner,
            rows,
            ResidentDispatchInput::Immediate,
            false,
        )
    }

    /// Immediate flow with free supply derived from the existing conserved
    /// `free + in_flight + occupied = capacity` lifecycle. Only the exact
    /// in-flight holding reserves; no class, request, or precedence can do so.
    pub fn dispatch_with_commitment_partition(
        &mut self,
        schedule: &mut IntegrationSchedule,
        granter: SimThingId,
        generation: GenerationStamp,
        rows: &[ResidentClearingBatchBinding],
        commitment: &ResidencyCapacityPartition,
    ) -> Result<ResidentClearingDispatchTicket, ResidentClearingRuntimeError> {
        let allocatable_total = commitment
            .free()
            .checked_add(commitment.in_flight())
            .ok_or(ResidentClearingRuntimeError::ArithmeticOverflow)?;
        let free = u32::try_from(commitment.free())
            .map_err(|_| ResidentClearingRuntimeError::ArithmeticOverflow)?;
        let mut effective = rows.to_vec();
        for row in &mut effective {
            if u64::from(row.available) != allocatable_total {
                return Err(ResidentClearingRuntimeError::CommitmentSupplyMismatch {
                    market: row.available,
                    lifecycle: allocatable_total,
                });
            }
            row.available = free;
        }
        self.dispatch_market(
            schedule,
            granter,
            generation,
            self.root_scope_owner,
            &effective,
            ResidentDispatchInput::Immediate,
            false,
        )
    }

    /// Clear a child's own market directly from immutable parent `T_s.G` at
    /// the same generation. No host supply field exists on the child claims.
    pub fn dispatch_spatial(
        &mut self,
        schedule: &mut IntegrationSchedule,
        parent: &ResidentClearingDispatchTicket,
        child_granter: SimThingId,
        generation: GenerationStamp,
        rows: &[ResidentSpatialClaimBinding],
    ) -> Result<ResidentClearingDispatchTicket, ResidentClearingRuntimeError> {
        if !self.admitted_scope_owners.contains(&child_granter) {
            return Err(ResidentClearingRuntimeError::UnadmittedSemanticScope {
                owner: child_granter,
            });
        }
        if !parent
            .plan
            .claims()
            .iter()
            .any(|claim| claim.source_simthing_id() == child_granter)
        {
            return Err(
                ResidentClearingRuntimeError::SpatialGranterHasNoParentProduct {
                    granter: child_granter,
                },
            );
        }
        let descendants = self
            .descendants_by_scope_owner
            .get(&child_granter)
            .expect("admitted scope owner has a descendant set");
        if let Some(row) = rows
            .iter()
            .find(|row| !descendants.contains(&row.source_simthing_id))
        {
            return Err(
                ResidentClearingRuntimeError::SpatialClaimOutsideChildScope {
                    granter: child_granter,
                    claimant: row.source_simthing_id,
                },
            );
        }
        let rows: Vec<_> = rows
            .iter()
            .map(|row| ResidentClearingBatchBinding {
                source_simthing_id: row.source_simthing_id,
                requested: row.requested,
                available: 0,
                precedence: row.precedence,
                continuous_weight: row.continuous_weight,
            })
            .collect();
        self.dispatch_market(
            schedule,
            child_granter,
            generation,
            child_granter,
            &rows,
            ResidentDispatchInput::Spatial(parent.submission),
            false,
        )
    }

    /// Prepare, but do not execute, generation N+1 demand from immutable U.
    pub fn prepare_temporal_demands(
        &mut self,
        products: &ResidentClearingDispatchTicket,
        demand_generation: GenerationStamp,
        authored: &[ResidentAuthoredDemand],
    ) -> Result<ResidentTemporalDemandTicket, ResidentClearingRuntimeError> {
        let sources: Vec<_> = products
            .plan
            .claims()
            .iter()
            .map(|claim| claim.source_simthing_id())
            .collect();
        if authored.len() != sources.len()
            || authored
                .iter()
                .zip(&sources)
                .any(|(row, source)| row.source_simthing_id != *source)
        {
            return Err(ResidentClearingRuntimeError::TemporalSourceMismatch);
        }
        let quantities: Vec<_> = authored.iter().map(|row| row.quantity).collect();
        let mut encoder = self.continuous_plane.ctx.device.create_command_encoder(
            &simthing_gpu::wgpu::CommandEncoderDescriptor {
                label: Some("resident_temporal_demand_prepare"),
            },
        );
        let submission = self.live_head.encode_temporal_demand_mint(
            &self.continuous_plane.ctx,
            &self.temporal_mint_session,
            &mut encoder,
            &products.plan,
            products.submission,
            &quantities,
            demand_generation,
        )?;
        self.continuous_plane
            .ctx
            .queue
            .submit(Some(encoder.finish()));
        Ok(ResidentTemporalDemandTicket {
            submission,
            sources,
            authority_granter: products.submission.authority_granter(),
            semantic_scope_owner: products.semantic_scope_owner,
        })
    }

    /// Execute generation N+1 from a prepared ordinary demand and inputs that
    /// are authoritative only at N+1.
    pub fn dispatch_temporal(
        &mut self,
        schedule: &mut IntegrationSchedule,
        demands: &ResidentTemporalDemandTicket,
        granter: SimThingId,
        generation: GenerationStamp,
        rows: &[ResidentTemporalExecutionBinding],
    ) -> Result<ResidentClearingDispatchTicket, ResidentClearingRuntimeError> {
        if generation != demands.submission.generation()
            || granter != demands.authority_granter
            || rows.len() != demands.sources.len()
            || rows
                .iter()
                .zip(&demands.sources)
                .any(|(row, source)| row.source_simthing_id != *source)
        {
            return Err(ResidentClearingRuntimeError::TemporalExecutionMismatch);
        }
        let rows: Vec<_> = rows
            .iter()
            .map(|row| ResidentClearingBatchBinding {
                source_simthing_id: row.source_simthing_id,
                // The plan needs an active physical row; the resident demand
                // buffer replaces this sentinel before exact settlement.
                requested: 1,
                available: row.available,
                precedence: row.precedence,
                continuous_weight: row.continuous_weight,
            })
            .collect();
        self.dispatch_market(
            schedule,
            granter,
            generation,
            demands.semantic_scope_owner,
            &rows,
            ResidentDispatchInput::Temporal(demands.submission),
            true,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn dispatch_market(
        &mut self,
        schedule: &mut IntegrationSchedule,
        granter: SimThingId,
        generation: GenerationStamp,
        semantic_scope_owner: SimThingId,
        rows: &[ResidentClearingBatchBinding],
        input: ResidentDispatchInput,
        weights_are_allocated_flow: bool,
    ) -> Result<ResidentClearingDispatchTicket, ResidentClearingRuntimeError> {
        self.validate_root_rows(rows)?;
        let continuous_plan = self.prepare_continuous_allocation(
            granter,
            semantic_scope_owner,
            rows,
            weights_are_allocated_flow,
        )?;
        let claims = rows
            .iter()
            .enumerate()
            .map(|(lane, row)| {
                Ok(ResidentApportionmentClaim::new(
                    self.semantic_row_for_market_lane(semantic_scope_owner, lane as u32)?,
                    row.source_simthing_id,
                    row.requested,
                    row.available,
                    row.precedence,
                    SlotIndex::new(
                        self.semantic_row_for_market_lane(semantic_scope_owner, lane as u32)? + 1,
                    ),
                    self.continuous_cols.allocated_flow_col,
                ))
            })
            .collect::<Result<Vec<_>, ResidentClearingRuntimeError>>()?;
        let mut plan = ResidentApportionmentPlan::build(
            &self.semantic_plan,
            claims,
            granter,
            generation,
            continuous_plan.integration_band,
        )?;
        plan = plan.with_persistence_deformations(rows.iter().enumerate().filter_map(
            |(lane, row)| {
                if row.requested == 0 {
                    return None;
                }
                self.persistence_deformations
                    .get(&row.source_simthing_id)
                    .cloned()
                    .map(|program| {
                        (
                            self.semantic_row_for_market_lane(semantic_scope_owner, lane as u32)
                                .expect("validated admitted market row"),
                            program,
                        )
                    })
            },
        ))?;
        let count = u32::try_from(plan.claims().len())
            .map_err(|_| ResidentClearingRuntimeError::ArithmeticOverflow)?;
        let reservation = schedule.reserve_resident_rows(count)?;
        let (semantic_rows, scratch) = self.buffers.apportionment_buffers(&plan)?;
        let mut encoder = self.continuous_plane.ctx.device.create_command_encoder(
            &simthing_gpu::wgpu::CommandEncoderDescriptor {
                label: Some("resident_clearing_production_dispatch"),
            },
        );
        self.continuous_plane.encode_accumulator_orderband_into(
            &mut self.continuous_session,
            &mut encoder,
            continuous_plan.n_bands,
            0.0,
            Some(&self.continuous_eml_table),
            false,
        );
        match input {
            ResidentDispatchInput::Immediate => {
                self.continuous_plane
                    .encode_resident_apportionment_with_dispatch_into(
                        &mut self.exact_session,
                        &mut encoder,
                        semantic_rows,
                        scratch,
                        &plan,
                        ResidentApportionmentDispatch::single_pass(),
                    )?;
            }
            ResidentDispatchInput::Spatial(parent) => self.live_head.encode_spatial_apportionment(
                &self.continuous_plane,
                &mut self.exact_session,
                &mut encoder,
                semantic_rows,
                scratch,
                &plan,
                parent,
                semantic_scope_owner,
            )?,
            ResidentDispatchInput::Temporal(demands) => {
                self.live_head.encode_temporal_apportionment(
                    &self.continuous_plane,
                    &mut self.exact_session,
                    &mut encoder,
                    semantic_rows,
                    scratch,
                    &plan,
                    demands,
                )?;
            }
        }
        let submission = self.live_head.encode_append(
            &mut encoder,
            scratch,
            &plan,
            reservation,
            semantic_scope_owner,
        )?;
        self.continuous_plane
            .ctx
            .queue
            .submit(Some(encoder.finish()));
        Ok(ResidentClearingDispatchTicket {
            submission,
            plan,
            semantic_scope_owner,
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

    fn prepare_continuous_allocation(
        &mut self,
        granter: SimThingId,
        semantic_scope_owner: SimThingId,
        rows: &[ResidentClearingBatchBinding],
        weights_are_allocated_flow: bool,
    ) -> Result<crate::ArenaAllocationPlan, ResidentClearingRuntimeError> {
        let children = rows
            .iter()
            .enumerate()
            .map(|(lane, row)| {
                Ok(HierarchyNode {
                    participant_slot: SlotIndex::new(
                        self.semantic_row_for_market_lane(semantic_scope_owner, lane as u32)? + 1,
                    ),
                    hosted_simthing_id: row.source_simthing_id,
                    depth: 1,
                    children: Vec::new(),
                    cols: self.continuous_cols,
                })
            })
            .collect::<Result<Vec<_>, ResidentClearingRuntimeError>>()?;
        let descriptor = GpuArenaDescriptor {
            name: RESIDENT_ALLOCATION_ARENA.into(),
            flow_property_id: self.continuous_property,
            balance_property_id: None,
            max_participants: self.continuous_plane.n_slots,
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
        let requested_total = if weights_are_allocated_flow {
            rows.iter().try_fold(0.0f32, |total, row| {
                let next = total + row.continuous_weight;
                next.is_finite()
                    .then_some(next)
                    .ok_or(ResidentClearingRuntimeError::ArithmeticOverflow)
            })?
        } else {
            rows.iter().try_fold(0u64, |total, row| {
                total
                    .checked_add(u64::from(row.requested))
                    .ok_or(ResidentClearingRuntimeError::ArithmeticOverflow)
            })? as f32
        };
        let n_dims = self.continuous_plane.n_dims as usize;
        self.continuous_values[self.continuous_cols.intrinsic_flow_col.raw()] = requested_total;
        self.continuous_plane
            .install_resolved_value_rows_at_boundary(0, &self.continuous_values[..n_dims]);
        for (lane, row) in rows.iter().enumerate() {
            let slot =
                self.semantic_row_for_market_lane(semantic_scope_owner, lane as u32)? as usize + 1;
            self.continuous_values[slot * n_dims + self.continuous_cols.weight_col.raw()] =
                row.continuous_weight;
            self.continuous_plane
                .install_resolved_value_rows_at_boundary(
                    slot as u32,
                    &self.continuous_values[slot * n_dims..(slot + 1) * n_dims],
                );
        }
        Ok(plan)
    }

    /// Asynchronous observer/materializer. Economic dispatch and immutable
    /// live-head append are already submitted before this maps the segment.
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

    pub fn readback_temporal_demands_for_proof(
        &self,
        ticket: &ResidentTemporalDemandTicket,
    ) -> Result<Vec<ResidentTemporalDemand>, ResidentClearingRuntimeError> {
        Ok(self
            .live_head
            .readback_temporal_demands_for_proof(&self.continuous_plane.ctx, ticket.submission)?)
    }

    /// Referee-only observation of values already emitted by the production
    /// child-share EML. Exact settlement reads these cells on-device before
    /// this diagnostic mapping is possible.
    pub fn readback_allocated_flow_for_proof(&self, count: u32) -> Vec<f32> {
        let values = self.continuous_plane.read_values();
        let n_dims = self.continuous_plane.n_dims as usize;
        (0..count.min(self.lane_capacity))
            .map(|lane| {
                let semantic_row = self
                    .semantic_row_for_market_lane(self.root_scope_owner, lane)
                    .expect("root semantic scope was admitted");
                values[(semantic_row as usize + 1) * n_dims
                    + self.continuous_cols.allocated_flow_col.raw()]
            })
            .collect()
    }

    fn semantic_row_for_market_lane(
        &self,
        scope_owner: SimThingId,
        lane: u32,
    ) -> Result<u32, ResidentClearingRuntimeError> {
        let draw = RESIDENT_DRAW_BASE + u64::from(lane);
        self.semantic_plan
            .rows()
            .iter()
            .position(|row| {
                let dictionaries = self.semantic_plan.dictionaries();
                dictionaries.draws()[row.draw().get() as usize].get() == draw
                    && dictionaries.scopes()[row.scope().get() as usize]
                        == resident_scope_id(scope_owner)
                    && dictionaries.owners()[row.owner().get() as usize]
                        .identity()
                        .local()
                        == &scope_owner
            })
            .and_then(|index| u32::try_from(index).ok())
            .ok_or(ResidentClearingRuntimeError::UnadmittedSemanticScope { owner: scope_owner })
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
    #[error("resident semantic scope owner {owner:?} was not admitted from the tree")]
    UnadmittedSemanticScope { owner: SimThingId },
    #[error("resident child granter {granter:?} has no parent T_s product")]
    SpatialGranterHasNoParentProduct { granter: SimThingId },
    #[error("resident child claim {claimant:?} is outside granter {granter:?}'s semantic subtree")]
    SpatialClaimOutsideChildScope {
        granter: SimThingId,
        claimant: SimThingId,
    },
    #[error("resident temporal authored-demand sources do not match immutable T_s sources")]
    TemporalSourceMismatch,
    #[error("resident N+1 execution does not match its prepared demand authority")]
    TemporalExecutionMismatch,
    #[error("resident market supply {market} differs from conserved free+in_flight {lifecycle}")]
    CommitmentSupplyMismatch { market: u32, lifecycle: u64 },
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
