//! RESIDENT-CLEARING-CUTOVER-0 — ordinary production resident executor.
//!
//! This owner composes the graduated semantic plan, exact apportionment
//! kernel, per-tree buffers, bounded IntegrationSchedule live head, and direct
//! N+1 product intake. It adds no score, pressure, remainder, or recurrence
//! law; `CpuVendorizedOracle` remains a separately selected diagnostic route.

use std::collections::BTreeMap;

use simthing_core::{
    expand_arena_internal_columns, AccumulatorRole, AccumulatorSpec, ClampBehavior,
    DimensionRegistry, ExecutionIncarnation, GenerationStamp, IntegrationSchedule, LogTier,
    PersistenceDeformationProgram, PropertyLayout, ResidencyCapacityPartition,
    ResidentClearingScheduleFact, ResidentScheduleError, SimProperty, SimPropertyId, SimThing,
    SimThingId, SlotIndex, SubFieldRole, SubFieldSpec, TreeExecutionAuthority,
    TreeExecutionBinding, TreeGenerationAuthority, TreeRealmId,
};
use simthing_gpu::{
    GpuContext, ResidentApportionmentClaim, ResidentApportionmentDispatch,
    ResidentApportionmentError, ResidentApportionmentPlan, ResidentApportionmentSession,
    ResidentClearingAdmission, ResidentClearingBudgets, ResidentClearingBufferOwner,
    ResidentClearingBuffers, ResidentClearingGpuError, ResidentClearingLiveHead,
    ResidentClearingPlan, ResidentClearingPlanError, ResidentClearingQualification,
    ResidentClearingSubmission, ResidentConstrainedProduct, ResidentDrawId,
    ResidentExactBasisIdentity, ResidentLiveHeadError, ResidentOwnerId, ResidentResourceId,
    ResidentScopeId, ResidentTemporalDemand, ResidentTemporalDemandMintSession,
    ResidentTemporalDemandSubmission, SlotAllocator, WorldGpuState,
};
use thiserror::Error;

use crate::arena_hierarchy::{build_execution_plan, ArenaTreeLayout, NodeColumnRefs};
use crate::arena_registry::{
    ArenaIdx, ArenaRegistry, ArenaRegistryBuilder, FissionPolicy, GpuArenaDescriptor,
};

pub const RESIDENT_MARKET_RF_NAMESPACE: &str = "simthing";
pub const RESIDENT_MARKET_RF_PROPERTY: &str = "residency-row-capacity";
pub const RESIDENT_MARKET_RF_ARENA: &str = "residency-row-capacity";
const RESIDENT_EXACT_PROJECTION_ABI: &str =
    "resident-q/u32-request+live-allocated-flow+exact-basis-identity/v3";
const RESIDENT_CONTINUOUS_POLICY_EML: &str = "child-share-eml/e11-0001";

/// Install the canonical RF substrate used by the implicit resident market.
///
/// This is also the narrow integration-test admission door: callers still
/// receive an ordinary property and must project/synchronize it through the
/// shared [`WorldGpuState`].
pub fn install_default_resident_rf_property(
    registry: &mut DimensionRegistry,
    root: &mut SimThing,
) -> SimPropertyId {
    let property_id = if let Some(property_id) =
        registry.id_of(RESIDENT_MARKET_RF_NAMESPACE, RESIDENT_MARKET_RF_PROPERTY)
    {
        property_id
    } else {
        let field = |name: &str, role: AccumulatorRole, default: f32| SubFieldSpec {
            role: SubFieldRole::Named(name.into()),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default,
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
                field("intrinsic-flow", AccumulatorRole::IntrinsicFlow, 1.0),
                field(
                    "allocated-flow",
                    AccumulatorRole::AllocatedFlow {
                        arena: RESIDENT_MARKET_RF_ARENA.into(),
                    },
                    0.0,
                ),
                field(
                    "allocator-weight",
                    AccumulatorRole::AllocatorWeight {
                        arena: RESIDENT_MARKET_RF_ARENA.into(),
                    },
                    1.0,
                ),
            ],
        });
        let mut property =
            SimProperty::simple(RESIDENT_MARKET_RF_NAMESPACE, RESIDENT_MARKET_RF_PROPERTY, 0);
        property.layout = layout;
        registry.register(property)
    };
    let default = registry.property(property_id).default_value();
    fn attach(
        node: &mut SimThing,
        property_id: SimPropertyId,
        default: &simthing_core::PropertyValue,
    ) {
        if node.property(property_id).is_none() {
            node.add_property(property_id, default.clone());
        }
        for child in &mut node.children {
            attach(child, property_id, default);
        }
    }
    attach(root, property_id, &default);
    property_id
}

/// Build the canonical recursive arena over the already-admitted physical
/// tree and its existing residency slots.
pub fn build_default_resident_arena_registry(
    property_id: SimPropertyId,
    root: &SimThing,
    residency: &SlotAllocator,
    capacity: u32,
) -> Result<ArenaRegistry, ResidentClearingRuntimeError> {
    let admitted_capacity = capacity.max(root.subtree_size() as u32).max(1);
    let rebind_capacity = admitted_capacity.saturating_mul(2).max(admitted_capacity);
    let mut builder = ArenaRegistryBuilder::new();
    let arena_idx = builder.push_arena(GpuArenaDescriptor {
        name: RESIDENT_MARKET_RF_ARENA.into(),
        flow_property_id: property_id,
        balance_property_id: None,
        max_participants: rebind_capacity,
        max_coupling_fanout: rebind_capacity,
        max_orderband_depth: rebind_capacity.saturating_mul(3).saturating_add(8),
        fission_policy: FissionPolicy::Inherit,
        participant_range: (0, 0),
        wildcard_max_expansion: None,
        reserved_orderband_depth: 0,
    });
    fn admit_tree(
        builder: &mut ArenaRegistryBuilder,
        arena_idx: ArenaIdx,
        node: &SimThing,
        parent: Option<SimThingId>,
        residency: &SlotAllocator,
    ) -> Result<(), ResidentClearingRuntimeError> {
        let slot = residency.slot_of(node.id).ok_or(
            ResidentClearingRuntimeError::UnboundArenaParticipant {
                participant: node.id,
            },
        )?;
        builder
            .admit_participant(arena_idx, slot, node.id, parent)
            .map_err(|error| ResidentClearingRuntimeError::ArenaBinding(error.to_string()))?;
        for child in &node.children {
            admit_tree(builder, arena_idx, child, Some(node.id), residency)?;
        }
        Ok(())
    }
    admit_tree(&mut builder, arena_idx, root, None, residency)?;
    builder
        .build()
        .map(|(registry, _)| registry)
        .map_err(|error| ResidentClearingRuntimeError::ArenaBinding(error.to_string()))
}

fn stable_digest<'a>(components: impl IntoIterator<Item = &'a [u8]>) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for component in components {
        hash ^= component.len() as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        for byte in component {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    hash
}

fn stable_digest_strings<'a>(components: impl IntoIterator<Item = &'a str>) -> u64 {
    stable_digest(components.into_iter().map(str::as_bytes))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResidentMarketAdmission {
    market_identity: String,
    resource_identity: String,
    scope_identity: String,
    draw_identity: String,
    preferred_arena: Option<String>,
    precedence_identity: String,
    continuous_policy_identity: String,
    /// Immutable semantic lowering for the exact projection. Dispatch rows
    /// cannot override this fact; a different mode requires a different
    /// admission and therefore a different qualification seal.
    exact_basis_identity: ResidentExactBasisIdentity,
}

impl ResidentMarketAdmission {
    pub fn new(
        market_identity: impl Into<String>,
        resource_identity: impl Into<String>,
        scope_identity: impl Into<String>,
        draw_identity: impl Into<String>,
        preferred_arena: Option<String>,
        precedence_identity: impl Into<String>,
        continuous_policy_identity: impl Into<String>,
        exact_basis_identity: ResidentExactBasisIdentity,
    ) -> Self {
        Self {
            market_identity: market_identity.into(),
            resource_identity: resource_identity.into(),
            scope_identity: scope_identity.into(),
            draw_identity: draw_identity.into(),
            preferred_arena,
            precedence_identity: precedence_identity.into(),
            continuous_policy_identity: continuous_policy_identity.into(),
            exact_basis_identity,
        }
    }

    pub fn implicit_growth() -> Self {
        Self::new(
            "simthing::implicit-root-standing-growth",
            "simthing::residency-row-capacity",
            "simthing::ordinary-growth/root-scope",
            "simthing::ordinary-growth-draw",
            None,
            "hard-precedence/u32-ascending",
            RESIDENT_CONTINUOUS_POLICY_EML,
            ResidentExactBasisIdentity::LiveAllocatedFlow,
        )
    }

    fn market_digest(&self) -> u64 {
        let exact_basis_identity = (self.exact_basis_identity as u32).to_le_bytes();
        stable_digest([
            self.market_identity.as_bytes(),
            self.resource_identity.as_bytes(),
            self.scope_identity.as_bytes(),
            self.draw_identity.as_bytes(),
            self.precedence_identity.as_bytes(),
            self.continuous_policy_identity.as_bytes(),
            exact_basis_identity.as_slice(),
        ])
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResidentMarketQualification {
    market_semantic_digest: u64,
    resource_shape_digest: u64,
    scope_draw_shape_digest: u64,
    arena_idx: ArenaIdx,
    flow_property_id: SimPropertyId,
    topology_digest: u64,
    registry_layout_digest: u64,
    registry_generation: u64,
    precedence_digest: u64,
    continuous_policy_digest: u64,
    exact_projection_abi_digest: u64,
    exact_basis_identity: ResidentExactBasisIdentity,
    seal: u64,
}

impl ResidentMarketQualification {
    fn seal_components(&self) -> u64 {
        let words = [
            self.market_semantic_digest,
            self.resource_shape_digest,
            self.scope_draw_shape_digest,
            u64::from(self.arena_idx),
            u64::from(self.flow_property_id.0),
            self.topology_digest,
            self.registry_layout_digest,
            self.registry_generation,
            self.precedence_digest,
            self.continuous_policy_digest,
            self.exact_projection_abi_digest,
            u64::from(self.exact_basis_identity as u32),
        ];
        let bytes = words.map(u64::to_le_bytes);
        stable_digest(bytes.iter().map(|word| word.as_slice()))
    }

    pub fn has_intact_seal(&self) -> bool {
        self.seal == self.seal_components()
    }

    pub const fn topology_digest(&self) -> u64 {
        self.topology_digest
    }

    pub const fn registry_layout_digest(&self) -> u64 {
        self.registry_layout_digest
    }

    pub const fn registry_generation(&self) -> u64 {
        self.registry_generation
    }

    pub const fn market_semantic_digest(&self) -> u64 {
        self.market_semantic_digest
    }

    pub const fn resource_shape_digest(&self) -> u64 {
        self.resource_shape_digest
    }

    pub const fn scope_draw_shape_digest(&self) -> u64 {
        self.scope_draw_shape_digest
    }

    pub const fn arena_idx(&self) -> ArenaIdx {
        self.arena_idx
    }

    pub const fn flow_property_id(&self) -> SimPropertyId {
        self.flow_property_id
    }

    pub const fn precedence_digest(&self) -> u64 {
        self.precedence_digest
    }

    pub const fn continuous_policy_digest(&self) -> u64 {
        self.continuous_policy_digest
    }

    pub const fn exact_projection_abi_digest(&self) -> u64 {
        self.exact_projection_abi_digest
    }

    pub const fn exact_basis_identity(&self) -> ResidentExactBasisIdentity {
        self.exact_basis_identity
    }
}

#[derive(Clone, Debug)]
struct ResidentRfArenaBinding {
    layout: ArenaTreeLayout,
    columns: NodeColumnRefs,
    participant_slots: BTreeMap<SimThingId, SlotIndex>,
    registry_layout_digest: u64,
    topology_digest: u64,
    registry_generation: u64,
    market: ResidentMarketAdmission,
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

fn resident_resource_id(market: &ResidentMarketAdmission) -> ResidentResourceId {
    ResidentResourceId::new(stable_digest_strings([market.resource_identity.as_str()]))
}

fn resident_scope_id(market: &ResidentMarketAdmission, owner: SimThingId) -> ResidentScopeId {
    let owner_raw = owner.raw().to_le_bytes();
    ResidentScopeId::new(stable_digest([
        market.scope_identity.as_bytes(),
        owner_raw.as_slice(),
    ]))
}

fn resident_draw_id(market: &ResidentMarketAdmission, lane: u32) -> ResidentDrawId {
    let lane_raw = lane.to_le_bytes();
    ResidentDrawId::new(stable_digest([
        market.draw_identity.as_bytes(),
        lane_raw.as_slice(),
    ]))
}

fn registry_layout_digest(registry: &DimensionRegistry) -> u64 {
    let mut components = Vec::<Vec<u8>>::new();
    components.push((registry.total_columns as u64).to_le_bytes().to_vec());
    for (index, property) in registry.properties.iter().enumerate() {
        components.push((index as u64).to_le_bytes().to_vec());
        components.push(property.namespace.as_bytes().to_vec());
        components.push(property.name.as_bytes().to_vec());
        components.push(
            registry
                .active
                .get(index)
                .copied()
                .unwrap_or(false)
                .to_string()
                .into_bytes(),
        );
        components.push(format!("{:?}", property.layout.sub_fields).into_bytes());
        if let Some(range) = registry.column_ranges.get(index) {
            components.push((range.start as u64).to_le_bytes().to_vec());
            components.push((range.stride as u64).to_le_bytes().to_vec());
        }
    }
    stable_digest(components.iter().map(Vec::as_slice))
}

fn topology_digest(arena_idx: ArenaIdx, arena_registry: &ArenaRegistry) -> u64 {
    let mut members = arena_registry
        .participants
        .iter()
        .filter(|member| member.arena_idx == arena_idx)
        .collect::<Vec<_>>();
    members.sort_by_key(|member| (member.subtree_root, member.slot));
    let mut components = Vec::<Vec<u8>>::new();
    for member in members {
        components.push(member.subtree_root.raw().to_le_bytes().to_vec());
        components.push(member.slot.raw().to_le_bytes().to_vec());
        components.push(
            member
                .parent
                .map(SimThingId::raw)
                .unwrap_or(u32::MAX)
                .to_le_bytes()
                .to_vec(),
        );
    }
    stable_digest(components.iter().map(Vec::as_slice))
}

impl ResidentRfArenaBinding {
    fn admit(
        registry: &DimensionRegistry,
        arena_registry: &ArenaRegistry,
        market: ResidentMarketAdmission,
    ) -> Result<Self, ResidentClearingRuntimeError> {
        let execution = build_execution_plan(registry, arena_registry)
            .map_err(|error| ResidentClearingRuntimeError::ArenaBinding(error.to_string()))?;
        let selected = match market.preferred_arena.as_deref() {
            Some(expected) => arena_registry
                .arenas
                .iter()
                .position(|arena| arena.name == expected),
            None => (!arena_registry.arenas.is_empty()).then_some(0),
        }
        .ok_or_else(|| ResidentClearingRuntimeError::MarketCannotLower {
            reason: market
                .preferred_arena
                .as_deref()
                .map(|arena| format!("RF arena `{arena}` is not admitted"))
                .unwrap_or_else(|| "no RF arena is admitted".into()),
        })? as ArenaIdx;
        let layout = execution
            .arenas
            .iter()
            .find(|layout| layout.arena_idx == selected)
            .cloned()
            .ok_or_else(|| ResidentClearingRuntimeError::MarketCannotLower {
                reason: format!("RF arena index {selected} has no execution layout"),
            })?;
        let descriptor = &arena_registry.arenas[selected as usize];
        let columns = crate::arena_hierarchy::resolve_node_columns_for_property(
            registry,
            descriptor.flow_property_id,
            &descriptor.name,
        )
        .map_err(|error| ResidentClearingRuntimeError::ArenaBinding(error.to_string()))?;
        let participant_slots = arena_registry
            .participants
            .iter()
            .filter(|member| member.arena_idx == selected)
            .map(|member| (member.subtree_root, member.slot))
            .collect::<BTreeMap<_, _>>();
        if participant_slots.is_empty() {
            return Err(ResidentClearingRuntimeError::MarketCannotLower {
                reason: format!("RF arena `{}` has no participants", descriptor.name),
            });
        }
        Ok(Self {
            layout,
            columns,
            participant_slots,
            registry_layout_digest: registry_layout_digest(registry),
            topology_digest: topology_digest(selected, arena_registry),
            registry_generation: arena_registry.generation,
            market,
        })
    }

    fn qualification(&self, semantic_scope_draw_shape_digest: u64) -> ResidentMarketQualification {
        let mut qualification = ResidentMarketQualification {
            market_semantic_digest: self.market.market_digest(),
            resource_shape_digest: stable_digest_strings([
                self.market.resource_identity.as_str(),
                self.layout.flow_property_id.0.to_string().as_str(),
            ]),
            scope_draw_shape_digest: semantic_scope_draw_shape_digest,
            arena_idx: self.layout.arena_idx,
            flow_property_id: self.layout.flow_property_id,
            topology_digest: self.topology_digest,
            registry_layout_digest: self.registry_layout_digest,
            registry_generation: self.registry_generation,
            precedence_digest: stable_digest_strings([self.market.precedence_identity.as_str()]),
            continuous_policy_digest: stable_digest_strings([
                self.market.continuous_policy_identity.as_str(),
                RESIDENT_CONTINUOUS_POLICY_EML,
            ]),
            exact_projection_abi_digest: stable_digest_strings([RESIDENT_EXACT_PROJECTION_ABI]),
            exact_basis_identity: self.market.exact_basis_identity,
            seal: 0,
        };
        qualification.seal = qualification.seal_components();
        qualification
    }

    fn participant_slot(
        &self,
        participant: SimThingId,
    ) -> Result<SlotIndex, ResidentClearingRuntimeError> {
        self.participant_slots
            .get(&participant)
            .copied()
            .ok_or(ResidentClearingRuntimeError::UnboundArenaParticipant { participant })
    }
}

struct ResidentProjection {
    arena_binding: ResidentRfArenaBinding,
    semantic_plan: ResidentClearingPlan,
    buffers: ResidentClearingBuffers,
    root_scope_owner: SimThingId,
    admitted_scope_owners: std::collections::BTreeSet<SimThingId>,
    descendants_by_scope_owner: BTreeMap<SimThingId, std::collections::BTreeSet<SimThingId>>,
    semantic_scope_draw_shape_digest: u64,
}

fn build_resident_projection(
    ctx: &GpuContext,
    binding: &TreeExecutionBinding<'_, SlotAllocator>,
    arena_binding: ResidentRfArenaBinding,
    lane_capacity: u32,
) -> Result<ResidentProjection, ResidentClearingRuntimeError> {
    let root = binding.root();
    let context = binding.context();
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
        let market = &arena_binding.market;
        (0..lane_capacity).map(move |lane| ResidentClearingAdmission {
            owner,
            resource: resident_resource_id(market),
            scope: resident_scope_id(market, *scope_owner),
            draw: resident_draw_id(market, lane),
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
    let scope_count = u32::try_from(scope_owners.len())
        .map_err(|_| ResidentClearingRuntimeError::ArithmeticOverflow)?;
    let budgets = ResidentClearingBudgets::new(
        scope_count,
        1,
        scope_count,
        lane_capacity,
        semantic_row_count,
        semantic_bytes,
        resident_bytes,
        scratch_bytes,
        64,
    )?;
    let semantic_plan = ResidentClearingPlan::build(binding, admissions, budgets)?;
    let buffers = ResidentClearingBuffers::allocate(&ctx.device, binding, &semantic_plan)?;
    let mut semantic_shape = vec![arena_binding.market.scope_identity.as_bytes().to_vec()];
    semantic_shape.push(arena_binding.market.draw_identity.as_bytes().to_vec());
    semantic_shape.push(lane_capacity.to_le_bytes().to_vec());
    semantic_shape.extend(
        scope_owners
            .iter()
            .map(|owner| owner.raw().to_le_bytes().to_vec()),
    );
    let semantic_scope_draw_shape_digest = stable_digest(semantic_shape.iter().map(Vec::as_slice));
    Ok(ResidentProjection {
        arena_binding,
        semantic_plan,
        buffers,
        root_scope_owner: root.id,
        admitted_scope_owners,
        descendants_by_scope_owner,
        semantic_scope_draw_shape_digest,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResidentClearingBatchBinding {
    pub source_simthing_id: SimThingId,
    /// Existing participant whose live RF cells carry the branch policy for
    /// this exact Draw. This is semantic topology identity, never a host value.
    pub rf_participant: SimThingId,
    pub requested: u32,
    pub available: u32,
    pub precedence: u32,
    // Exact-basis identity is deliberately absent: Q recovers it from the
    // qualified admitted market, never from a dispatch-authored row.
}

/// One descendant claim in a same-generation child market. Supply is absent
/// by construction because it comes only from immutable parent `T_s.G`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResidentSpatialClaimBinding {
    pub source_simthing_id: SimThingId,
    pub rf_participant: SimThingId,
    pub requested: u32,
    pub precedence: u32,
    // Basis mode remains the parent runtime's admitted market fact.
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
    pub rf_participant: SimThingId,
    pub available: u32,
    pub precedence: u32,
    // Basis mode remains the runtime's admitted market fact across N -> N+1.
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
    incarnation: ExecutionIncarnation,
    executor_qualification: ResidentClearingQualification,
    arena_binding: ResidentRfArenaBinding,
    semantic_plan: ResidentClearingPlan,
    buffers: ResidentClearingBuffers,
    exact_session: ResidentApportionmentSession,
    live_head: ResidentClearingLiveHead,
    lane_capacity: u32,
    root_scope_owner: SimThingId,
    admitted_scope_owners: std::collections::BTreeSet<SimThingId>,
    descendants_by_scope_owner: BTreeMap<SimThingId, std::collections::BTreeSet<SimThingId>>,
    semantic_scope_draw_shape_digest: u64,
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
        arena_registry: &ArenaRegistry,
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
            arena_registry,
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
        arena_registry: &ArenaRegistry,
        residency: &SlotAllocator,
        schedule: &IntegrationSchedule,
        generation: GenerationStamp,
        lane_capacity: u32,
        deformation_bindings: &[ResidentPersistenceDeformationBinding],
    ) -> Result<Self, ResidentClearingRuntimeError> {
        Self::admit_market_with_persistence_deformations(
            ctx,
            realm,
            root,
            registry,
            arena_registry,
            residency,
            schedule,
            generation,
            lane_capacity,
            ResidentMarketAdmission::implicit_growth(),
            deformation_bindings,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn admit_market_with_persistence_deformations(
        ctx: &GpuContext,
        realm: TreeRealmId,
        root: &SimThing,
        registry: &DimensionRegistry,
        arena_registry: &ArenaRegistry,
        residency: &SlotAllocator,
        schedule: &IntegrationSchedule,
        generation: GenerationStamp,
        lane_capacity: u32,
        market: ResidentMarketAdmission,
        deformation_bindings: &[ResidentPersistenceDeformationBinding],
    ) -> Result<Self, ResidentClearingRuntimeError> {
        let generation_authority = TreeGenerationAuthority::new(generation);
        let incarnation = ExecutionIncarnation::new(1)
            .map_err(|error| ResidentClearingRuntimeError::Identity(error.to_string()))?;
        let authority = TreeExecutionAuthority::seal(
            realm,
            incarnation,
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
        Self::admit_sealed_market_with_persistence_deformations(
            ctx,
            &binding,
            arena_registry,
            lane_capacity,
            market,
            deformation_bindings,
        )
    }

    /// Admit the executor from the boundary's one freshly sealed view of the
    /// authoritative tree. Session/spec installation uses this door so first
    /// admission and later topology rebinds share the same authority capsule;
    /// no cloned scenario tree can become a second execution authority.
    pub fn admit_sealed_market_with_persistence_deformations(
        ctx: &GpuContext,
        binding: &TreeExecutionBinding<'_, SlotAllocator>,
        arena_registry: &ArenaRegistry,
        lane_capacity: u32,
        market: ResidentMarketAdmission,
        deformation_bindings: &[ResidentPersistenceDeformationBinding],
    ) -> Result<Self, ResidentClearingRuntimeError> {
        binding
            .validate()
            .map_err(|error| ResidentClearingRuntimeError::Identity(error.to_string()))?;
        if lane_capacity == 0 {
            return Err(ResidentClearingRuntimeError::ZeroLaneCapacity);
        }
        let mut persistence_deformations = BTreeMap::new();
        for deformation in deformation_bindings {
            if persistence_deformations
                .insert(deformation.source_simthing_id, deformation.program.clone())
                .is_some()
            {
                return Err(
                    ResidentClearingRuntimeError::DuplicatePersistenceDeformation {
                        source_id: deformation.source_simthing_id,
                    },
                );
            }
        }
        let executor_qualification = ResidentClearingQualification::admit(ctx)?;
        let arena_binding =
            ResidentRfArenaBinding::admit(binding.registry(), arena_registry, market)?;
        let temporal_mint_session = ResidentTemporalDemandMintSession::new(ctx);
        let projection = build_resident_projection(ctx, binding, arena_binding, lane_capacity)?;
        let exact_session = ResidentApportionmentSession::new(ctx);
        let live_head_capacity = binding
            .schedule()
            .resident_live_head_capacity()
            .ok_or(ResidentClearingRuntimeError::ZeroLaneCapacity)?;
        let live_head = ResidentClearingLiveHead::admit(ctx, live_head_capacity)?;
        Ok(Self {
            realm: binding.context().realm(),
            incarnation: binding.context().incarnation(),
            executor_qualification,
            arena_binding: projection.arena_binding,
            semantic_plan: projection.semantic_plan,
            buffers: projection.buffers,
            exact_session,
            live_head,
            lane_capacity,
            root_scope_owner: projection.root_scope_owner,
            admitted_scope_owners: projection.admitted_scope_owners,
            descendants_by_scope_owner: projection.descendants_by_scope_owner,
            semantic_scope_draw_shape_digest: projection.semantic_scope_draw_shape_digest,
            persistence_deformations,
            temporal_mint_session,
        })
    }

    pub const fn realm(&self) -> TreeRealmId {
        self.realm
    }

    pub fn qualification(&self) -> &ResidentClearingQualification {
        &self.executor_qualification
    }

    pub fn market_qualification(&self) -> ResidentMarketQualification {
        self.arena_binding
            .qualification(self.semantic_scope_draw_shape_digest)
    }

    pub const fn incarnation(&self) -> ExecutionIncarnation {
        self.incarnation
    }

    /// Re-seal only the topology-dependent projection after an admitted
    /// boundary mutation. The exact executor, immutable live head, temporal
    /// mint, and their pending `T_s` rows stay resident and are not recreated.
    ///
    /// The caller must provide a fresh seal over the already-mutated
    /// authoritative tree without changing incarnation (topology rebind is
    /// not migration). A stale market qualification stops
    /// working as soon as this succeeds; the returned seal is the sole token
    /// for subsequent dispatch/materialization.
    pub fn rebind_after_topology_change(
        &mut self,
        ctx: &GpuContext,
        binding: &TreeExecutionBinding<'_, SlotAllocator>,
        arena_registry: &ArenaRegistry,
    ) -> Result<ResidentMarketQualification, ResidentClearingRuntimeError> {
        if binding.context().realm() != self.realm
            || binding.context().incarnation() != self.incarnation
        {
            return Err(ResidentClearingRuntimeError::Identity(
                "resident topology rebind must preserve realm and execution incarnation".into(),
            ));
        }
        let observed_qualification = ResidentClearingQualification::capture(ctx)?;
        if observed_qualification != self.executor_qualification {
            return Err(ResidentClearingRuntimeError::Identity(
                "resident topology rebind changed the qualified GPU/compiler/ABI tuple".into(),
            ));
        }
        let arena_binding = ResidentRfArenaBinding::admit(
            binding.registry(),
            arena_registry,
            self.arena_binding.market.clone(),
        )?;
        let projection =
            build_resident_projection(ctx, binding, arena_binding, self.lane_capacity)?;
        self.arena_binding = projection.arena_binding;
        self.semantic_plan = projection.semantic_plan;
        self.buffers = projection.buffers;
        self.root_scope_owner = projection.root_scope_owner;
        self.admitted_scope_owners = projection.admitted_scope_owners;
        self.descendants_by_scope_owner = projection.descendants_by_scope_owner;
        self.semantic_scope_draw_shape_digest = projection.semantic_scope_draw_shape_digest;
        Ok(self.market_qualification())
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
        state: &WorldGpuState,
        qualification: &ResidentMarketQualification,
        schedule: &mut IntegrationSchedule,
        granter: SimThingId,
        generation: GenerationStamp,
        rows: &[ResidentClearingBatchBinding],
    ) -> Result<ResidentClearingDispatchTicket, ResidentClearingRuntimeError> {
        self.dispatch_market(
            state,
            qualification,
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
        state: &WorldGpuState,
        qualification: &ResidentMarketQualification,
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
            state,
            qualification,
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
        state: &WorldGpuState,
        qualification: &ResidentMarketQualification,
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
                rf_participant: row.rf_participant,
                requested: row.requested,
                available: 0,
                precedence: row.precedence,
            })
            .collect();
        self.dispatch_market(
            state,
            qualification,
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
        state: &WorldGpuState,
        qualification: &ResidentMarketQualification,
        products: &ResidentClearingDispatchTicket,
        demand_generation: GenerationStamp,
        authored: &[ResidentAuthoredDemand],
    ) -> Result<ResidentTemporalDemandTicket, ResidentClearingRuntimeError> {
        self.ensure_market_qualification(qualification)?;
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
        let mut encoder = state.ctx.device.create_command_encoder(
            &simthing_gpu::wgpu::CommandEncoderDescriptor {
                label: Some("resident_temporal_demand_prepare"),
            },
        );
        let submission = self.live_head.encode_temporal_demand_mint(
            &state.ctx,
            &self.temporal_mint_session,
            &mut encoder,
            &products.plan,
            products.submission,
            &quantities,
            demand_generation,
        )?;
        state.ctx.queue.submit(Some(encoder.finish()));
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
        state: &WorldGpuState,
        qualification: &ResidentMarketQualification,
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
                rf_participant: row.rf_participant,
                // The plan needs an active physical row; the resident demand
                // buffer replaces this sentinel before exact settlement.
                requested: 1,
                available: row.available,
                precedence: row.precedence,
            })
            .collect();
        self.dispatch_market(
            state,
            qualification,
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
        state: &WorldGpuState,
        qualification: &ResidentMarketQualification,
        schedule: &mut IntegrationSchedule,
        granter: SimThingId,
        generation: GenerationStamp,
        semantic_scope_owner: SimThingId,
        rows: &[ResidentClearingBatchBinding],
        input: ResidentDispatchInput,
        _weights_are_allocated_flow: bool,
    ) -> Result<ResidentClearingDispatchTicket, ResidentClearingRuntimeError> {
        self.ensure_market_qualification(qualification)?;
        self.validate_root_rows(rows)?;
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
                    self.arena_binding.participant_slot(row.rf_participant)?,
                    self.arena_binding.columns.allocated_flow_col,
                    self.arena_binding.market.exact_basis_identity,
                ))
            })
            .collect::<Result<Vec<_>, ResidentClearingRuntimeError>>()?;
        let mut plan = ResidentApportionmentPlan::build(
            &self.semantic_plan,
            claims,
            granter,
            generation,
            self.arena_binding.layout.band_layout.integration_band,
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
        let mut encoder = state.ctx.device.create_command_encoder(
            &simthing_gpu::wgpu::CommandEncoderDescriptor {
                label: Some("resident_clearing_production_dispatch"),
            },
        );
        match input {
            ResidentDispatchInput::Immediate => {
                state.encode_resident_apportionment_with_dispatch_into(
                    &mut self.exact_session,
                    &mut encoder,
                    semantic_rows,
                    scratch,
                    &plan,
                    ResidentApportionmentDispatch::single_pass(),
                )?;
            }
            ResidentDispatchInput::Spatial(parent) => self.live_head.encode_spatial_apportionment(
                state,
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
                    state,
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
        state.ctx.queue.submit(Some(encoder.finish()));
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
        Ok(())
    }

    fn ensure_market_qualification(
        &self,
        qualification: &ResidentMarketQualification,
    ) -> Result<(), ResidentClearingRuntimeError> {
        if !qualification.has_intact_seal()
            || *qualification
                != self
                    .arena_binding
                    .qualification(self.semantic_scope_draw_shape_digest)
        {
            return Err(ResidentClearingRuntimeError::StaleMarketQualification);
        }
        Ok(())
    }

    /// Asynchronous observer/materializer. Economic dispatch and immutable
    /// live-head append are already submitted before this maps the segment.
    pub fn materialize(
        &mut self,
        state: &WorldGpuState,
        qualification: &ResidentMarketQualification,
        schedule: &mut IntegrationSchedule,
        ticket: ResidentClearingDispatchTicket,
    ) -> Result<Vec<ResidentConstrainedProduct>, ResidentClearingRuntimeError> {
        self.ensure_market_qualification(qualification)?;
        let resident = self
            .live_head
            .readback_segment(&state.ctx, ticket.submission)?;
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
        state: &WorldGpuState,
        qualification: &ResidentMarketQualification,
        ticket: &ResidentTemporalDemandTicket,
    ) -> Result<Vec<ResidentTemporalDemand>, ResidentClearingRuntimeError> {
        self.ensure_market_qualification(qualification)?;
        Ok(self
            .live_head
            .readback_temporal_demands_for_proof(&state.ctx, ticket.submission)?)
    }

    /// Referee-only observation of values already emitted by the production
    /// child-share EML. Exact settlement reads these cells on-device before
    /// this diagnostic mapping is possible.
    pub fn readback_allocated_flow_for_proof(
        &self,
        state: &WorldGpuState,
        qualification: &ResidentMarketQualification,
        participants: &[SimThingId],
    ) -> Result<Vec<f32>, ResidentClearingRuntimeError> {
        self.ensure_market_qualification(qualification)?;
        let values = state.read_values();
        let n_dims = state.n_dims as usize;
        participants
            .iter()
            .map(|participant| {
                let slot = self.arena_binding.participant_slot(*participant)?.raw() as usize;
                Ok(values[slot * n_dims + self.arena_binding.columns.allocated_flow_col.raw()])
            })
            .collect()
    }

    fn semantic_row_for_market_lane(
        &self,
        scope_owner: SimThingId,
        lane: u32,
    ) -> Result<u32, ResidentClearingRuntimeError> {
        let draw = resident_draw_id(&self.arena_binding.market, lane);
        self.semantic_plan
            .rows()
            .iter()
            .position(|row| {
                let dictionaries = self.semantic_plan.dictionaries();
                dictionaries.draws()[row.draw().get() as usize] == draw
                    && dictionaries.scopes()[row.scope().get() as usize]
                        == resident_scope_id(&self.arena_binding.market, scope_owner)
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
    #[error("duplicate resident persistence deformation for claimant {source_id:?}")]
    DuplicatePersistenceDeformation { source_id: SimThingId },
    #[error("resident market cannot lower completely: {reason}")]
    MarketCannotLower { reason: String },
    #[error("resident RF arena binding failed: {0}")]
    ArenaBinding(String),
    #[error("resident exact Draw names unbound RF participant {participant:?}")]
    UnboundArenaParticipant { participant: SimThingId },
    #[error("resident market qualification is stale or its seal is not intact")]
    StaleMarketQualification,
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
