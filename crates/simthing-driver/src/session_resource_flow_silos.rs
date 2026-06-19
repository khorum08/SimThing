//! SESSION-RESOURCE-FLOW-SILOS-0 — driver materialization over generic ResourceFlow surfaces.

use simthing_core::DimensionRegistry;
use simthing_spec::{
    compile_resource_flow_admission, evaluate_owner_silo_flow, owner_silo_flow_participant_roots,
    ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, OwnerSiloAdmissionClassification,
    OwnerSiloAdmissionReport, PropertyKey, ResourceFlowSpec, SimThingScenarioSpec, SpecError,
};

use crate::arena_registry::ArenaRegistry;
use crate::resource_flow_compile::{
    compile_and_materialize_resource_flow, materialize_arena_registry,
};

/// Driver-side owner-silo flow compile/materialization outcome.
#[derive(Debug, Clone, PartialEq)]
pub struct OwnerSiloFlowMaterializationReport {
    pub silo_admission: OwnerSiloAdmissionReport,
    pub explicit_participant_count: u32,
    /// Admitted participants can lower to existing AccumulatorOp/GPU surfaces.
    pub gpu_participant_accumulation_ready: bool,
    /// Full owner-silo state mutation (reduce-up/disburse-down writes) remains deferred.
    pub gpu_full_state_mutation_deferred: bool,
    pub gpu_execution_note: &'static str,
}

/// Build a generic [`ResourceFlowSpec`] with explicit participants from admitted owner-silo flow.
pub fn build_owner_silo_resource_flow_spec(
    scenario: &SimThingScenarioSpec,
) -> Option<ResourceFlowSpec> {
    let report = evaluate_owner_silo_flow(scenario);
    if report.classification == OwnerSiloAdmissionClassification::Rejected
        || report.participant_count == 0
    {
        return None;
    }

    let roots = owner_silo_flow_participant_roots(scenario);
    if roots.is_empty() {
        return None;
    }

    let max_participants = roots.len().max(1) as u32;
    let explicit_participants = roots
        .into_iter()
        .enumerate()
        .map(|(slot, subtree_root_id)| ExplicitParticipantSpec::flat(slot as u32, subtree_root_id))
        .collect();

    Some(ResourceFlowSpec {
        arenas: vec![ArenaSpec {
            name: "owner_silo".into(),
            flow_property: PropertyKey::new("session", "owner_silo_flow"),
            balance_property: None,
            max_participants,
            max_coupling_fanout: 0,
            max_orderband_depth: 1,
            fission_policy: FissionPolicySpec::Reevaluate,
            reserved_orderband_depth: 0,
            reserved_gap_per_intermediate: 0,
            expected_max_children_per_intermediate: 0,
            explicit_participants,
            enrollment: None,
            wildcard_admission: None,
        }],
        couplings: vec![],
        ..Default::default()
    })
}

/// Compile owner-silo flow admission through existing ResourceFlow surfaces.
pub fn compile_owner_silo_flow_admission(
    scenario: &SimThingScenarioSpec,
    registry: &DimensionRegistry,
) -> Result<
    (
        simthing_spec::CompiledResourceFlowAdmission,
        OwnerSiloFlowMaterializationReport,
    ),
    SpecError,
> {
    let silo_admission = evaluate_owner_silo_flow(scenario);
    let flow_spec =
        build_owner_silo_resource_flow_spec(scenario).ok_or(SpecError::ValidationFailed)?;
    let admission = compile_resource_flow_admission(&flow_spec, registry)?;
    Ok((
        admission,
        owner_silo_materialization_report(&silo_admission),
    ))
}

fn owner_silo_materialization_report(
    silo_admission: &OwnerSiloAdmissionReport,
) -> OwnerSiloFlowMaterializationReport {
    let gpu_participant_accumulation_ready = silo_admission.participant_count > 0
        && silo_admission.classification != OwnerSiloAdmissionClassification::Rejected;
    OwnerSiloFlowMaterializationReport {
        explicit_participant_count: silo_admission.participant_count,
        silo_admission: silo_admission.clone(),
        gpu_participant_accumulation_ready,
        gpu_full_state_mutation_deferred: true,
        gpu_execution_note: if gpu_participant_accumulation_ready {
            "GPU participant accumulation via existing AccumulatorOp; full owner-silo state mutation deferred"
        } else {
            "owner-silo flow not admitted for GPU participant accumulation"
        },
    }
}

/// Compile and materialize owner-silo flow through existing arena registry surfaces.
pub fn compile_and_materialize_owner_silo_flow(
    scenario: &SimThingScenarioSpec,
    registry: &DimensionRegistry,
) -> Result<(ArenaRegistry, OwnerSiloFlowMaterializationReport), SpecError> {
    let (admission, report) = compile_owner_silo_flow_admission(scenario, registry)?;
    let (registry, _) = materialize_arena_registry(&admission).map_err(map_registry_error)?;
    Ok((registry, report))
}

/// Full compile + materialize via the existing combined ResourceFlow entry point.
pub fn compile_and_materialize_owner_silo_flow_via_resource_flow(
    scenario: &SimThingScenarioSpec,
    registry: &DimensionRegistry,
) -> Result<(ArenaRegistry, OwnerSiloFlowMaterializationReport), SpecError> {
    let silo_admission = evaluate_owner_silo_flow(scenario);
    let flow_spec =
        build_owner_silo_resource_flow_spec(scenario).ok_or(SpecError::ValidationFailed)?;
    let (registry, _) = compile_and_materialize_resource_flow(&flow_spec, registry)?;
    Ok((registry, owner_silo_materialization_report(&silo_admission)))
}

fn map_registry_error(err: crate::arena_registry::ArenaRegistryError) -> SpecError {
    match err {
        crate::arena_registry::ArenaRegistryError::ImplicitParticipation { arena } => {
            SpecError::ImplicitParticipation {
                arena: arena.to_string(),
            }
        }
        _ => SpecError::ValidationFailed,
    }
}
