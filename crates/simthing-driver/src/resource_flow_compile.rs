//! E-10 — Materialize compiled Resource Flow admission into `ArenaRegistry`.

use simthing_core::{DimensionRegistry, SimThingId};
use simthing_spec::{
    compile_resource_flow_admission, CompiledCouplingDelay, CompiledResourceFlowAdmission,
    FissionPolicySpec, ResourceFlowExpansionReport, ResourceFlowSpec, SpecError,
};
use std::collections::HashMap;

use crate::arena_registry::{
    ArenaCoupling, ArenaRegistry, ArenaRegistryBuilder, ArenaRegistryError, CouplingDelay,
    FissionPolicy, GpuArenaDescriptor,
};

/// Compile authored Resource Flow spec and materialize a session `ArenaRegistry`.
pub fn compile_and_materialize_resource_flow(
    spec: &ResourceFlowSpec,
    registry: &DimensionRegistry,
) -> Result<(ArenaRegistry, ResourceFlowExpansionReport), SpecError> {
    let admission = compile_resource_flow_admission(spec, registry)?;
    materialize_arena_registry(&admission).map_err(map_registry_error)
}

/// Build `ArenaRegistry` from a validated [`CompiledResourceFlowAdmission`].
pub fn materialize_arena_registry(
    admission: &CompiledResourceFlowAdmission,
) -> Result<(ArenaRegistry, ResourceFlowExpansionReport), ArenaRegistryError> {
    if admission.arenas.is_empty() {
        return Ok((
            ArenaRegistry::empty(),
            ResourceFlowExpansionReport::default(),
        ));
    }

    let mut builder = ArenaRegistryBuilder::new();
    let mut arena_idx_by_name: HashMap<String, u32> = HashMap::new();

    for arena in &admission.arenas {
        let idx = builder.push_arena(GpuArenaDescriptor {
            name: arena.name.clone(),
            flow_property_id: arena.flow_property_id,
            balance_property_id: arena.balance_property_id,
            max_participants: arena.max_participants,
            max_coupling_fanout: arena.max_coupling_fanout,
            max_orderband_depth: arena.max_orderband_depth,
            fission_policy: map_fission_policy(arena.fission_policy),
            participant_range: (0, 0),
            wildcard_max_expansion: arena.wildcard_max_expansion,
            reserved_orderband_depth: arena.reserved_orderband_depth,
        });
        arena_idx_by_name.insert(arena.name.clone(), idx);

        for (slot, subtree_root_raw) in &arena.explicit_participants {
            builder.admit_participant(
                idx,
                *slot,
                SimThingId::from_session_raw(*subtree_root_raw),
            )?;
        }

        if let Some(max) = arena.wildcard_max_expansion {
            builder.declare_wildcard_admission(idx, Some(max))?;
        }

        if arena.reserved_orderband_depth > 0 {
            builder.reserve_orderband_depth(idx, arena.reserved_orderband_depth)?;
        }
    }

    for coupling in &admission.couplings {
        let from = *arena_idx_by_name
            .get(&coupling.from_arena)
            .expect("compiled admission validated arena refs");
        let to = *arena_idx_by_name
            .get(&coupling.to_arena)
            .expect("compiled admission validated arena refs");
        builder.push_coupling(ArenaCoupling {
            from_arena: from,
            to_arena: to,
            delay: map_coupling_delay(&coupling.delay),
        })?;
    }

    let (registry, _) = builder.build()?;
    let report = expansion_report_from_registry(&registry);
    Ok((registry, report))
}

fn map_fission_policy(policy: FissionPolicySpec) -> FissionPolicy {
    match policy {
        FissionPolicySpec::Inherit => FissionPolicy::Inherit,
        FissionPolicySpec::Reevaluate => FissionPolicy::Reevaluate,
        FissionPolicySpec::Reject => FissionPolicy::Reject,
    }
}

fn map_coupling_delay(delay: &CompiledCouplingDelay) -> CouplingDelay {
    match delay {
        CompiledCouplingDelay::Algebraic => CouplingDelay::Algebraic,
        CompiledCouplingDelay::OneTickDelay => CouplingDelay::OneTickDelay,
        CompiledCouplingDelay::BoundaryStage { stage } => {
            CouplingDelay::BoundaryStage { stage: *stage }
        }
        CompiledCouplingDelay::AccumulatorState { property_id } => {
            CouplingDelay::AccumulatorState {
                property: *property_id,
            }
        }
    }
}

fn expansion_report_from_registry(registry: &ArenaRegistry) -> ResourceFlowExpansionReport {
    let mut per_arena_participant_counts = Vec::with_capacity(registry.arenas.len());
    let mut per_arena_coupling_fanout = Vec::with_capacity(registry.arenas.len());
    let mut out_fanout = vec![0u32; registry.arenas.len()];
    let mut in_fanout = vec![0u32; registry.arenas.len()];

    for c in &registry.couplings {
        out_fanout[c.from_arena as usize] += 1;
        in_fanout[c.to_arena as usize] += 1;
    }

    let mut total_orderband_depth_reserved = 0u32;
    for (idx, arena) in registry.arenas.iter().enumerate() {
        per_arena_participant_counts.push((arena.name.clone(), arena.participant_range.1));
        let fanout = out_fanout[idx].max(in_fanout[idx]);
        per_arena_coupling_fanout.push((arena.name.clone(), fanout));
        total_orderband_depth_reserved =
            total_orderband_depth_reserved.saturating_add(arena.reserved_orderband_depth);
    }

    let participant_count = registry.participants.len();
    let coupling_count = registry.couplings.len();
    let total_registration_estimate =
        u32::try_from(participant_count.saturating_add(coupling_count))
            .ok()
            .map(|n| n);

    ResourceFlowExpansionReport {
        arena_count: registry.arenas.len(),
        participant_count,
        coupling_count,
        per_arena_participant_counts,
        per_arena_coupling_fanout,
        total_registration_estimate,
        total_orderband_depth_reserved,
        rejected: Vec::new(),
    }
}

fn map_registry_error(err: ArenaRegistryError) -> SpecError {
    match err {
        ArenaRegistryError::InvalidArenaIdx(_) => SpecError::ValidationFailed,
        ArenaRegistryError::ImplicitParticipation { arena } => SpecError::ImplicitParticipation {
            arena: arena.to_string(),
        },
        ArenaRegistryError::UnboundedWildcard { arena } => SpecError::UnboundedWildcardAdmission {
            arena: arena.to_string(),
        },
        ArenaRegistryError::MaxParticipantsExceeded {
            arena,
            declared,
            computed,
        } => SpecError::MaxParticipantsExceeded {
            arena: arena.to_string(),
            declared,
            computed,
        },
        ArenaRegistryError::MaxCouplingFanoutExceeded {
            arena,
            declared,
            computed,
        } => SpecError::MaxCouplingFanoutExceeded {
            arena: arena.to_string(),
            declared,
            computed,
        },
        ArenaRegistryError::MaxOrderBandDepthExceeded {
            arena,
            declared,
            computed,
        } => SpecError::MaxOrderBandDepthExceeded {
            arena: arena.to_string(),
            declared,
            computed,
        },
        ArenaRegistryError::AllAlgebraicCouplingCycle { .. } => {
            SpecError::AllAlgebraicCouplingCycle
        }
        ArenaRegistryError::HiddenFanoutExceeded {
            arena,
            declared,
            computed,
        } => SpecError::HiddenFanoutExceeded {
            arena: arena.to_string(),
            declared,
            computed,
        },
    }
}
