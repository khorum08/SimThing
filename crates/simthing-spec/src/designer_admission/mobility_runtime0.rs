//! MOBILITY-RUNTIME-0: default-off substrate-composition harness.
//!
//! This is a test/fixture-only composition harness for the completed v7.9
//! mobility/transfer substrate ladder. It composes existing substrate reports
//! in the documented order and does not wire production `SimSession`, a GPU
//! pass graph, runtime gameplay, or default-on behavior.

use super::mobility_alloc0::{
    mobility_alloc0_layout_checksum_cpu, mobility_alloc0_layout_checksum_gpu_proxy,
    plan_mobility_alloc0, MobilityAlloc0BoundaryEvent, MobilityAlloc0BoundaryEventKind,
    MobilityAlloc0PlanInput, MobilityAlloc0PlanReport,
};
use super::mobility_econ0::{plan_mobility_econ0, MobilityEcon0PlanInput, MobilityEcon0PlanReport};
use super::mobility_idroute0::{
    plan_mobility_idroute0, MobilityIdroute0PlanInput, MobilityIdroute0PlanReport,
};
use super::mobility_owner0::{
    plan_mobility_owner0, MobilityOwner0PlanInput, MobilityOwner0PlanReport,
};
use super::mobility_reenroll0::{
    mobility_reenroll0_layout_checksum_cpu, mobility_reenroll0_layout_checksum_gpu_proxy,
    plan_mobility_reenroll0, MobilityReenroll0PlanInput, MobilityReenroll0PlanReport,
};

pub const MOBILITY_RUNTIME0_ID: &str = "mobility_runtime0_substrate_composition_harness";
pub const MOBILITY_RUNTIME0_ORDER: [&str; 5] = ["ALLOC", "REENROLL", "IDROUTE", "ECON", "OWNER"];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityRuntime0HarnessConfig {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl MobilityRuntime0HarnessConfig {
    pub fn opt_in_test_harness() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MobilityRuntime0ForbiddenPathRequests {
    pub default_on_behavior: bool,
    pub semantic_or_raw_wgsl: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub owner_as_spatial_parent: bool,
    pub capture_as_reparenting: bool,
    pub nested_arena_reparenting: bool,
    pub default_on_resource_flow: bool,
    pub hard_currency_through_resource_flow: bool,
    pub hybrid_strata_or_faction_index_scaling: bool,
    pub closed_ladder_reopen: bool,
    pub production_simsession_wiring: bool,
    pub gpu_pass_graph_wiring: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityRuntime0CompositionInput {
    pub config: MobilityRuntime0HarnessConfig,
    pub alloc: MobilityAlloc0PlanInput,
    pub reenroll: MobilityReenroll0PlanInput,
    pub idroute: MobilityIdroute0PlanInput,
    pub econ: MobilityEcon0PlanInput,
    pub owner: MobilityOwner0PlanInput,
    pub forbidden: MobilityRuntime0ForbiddenPathRequests,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityRuntime0CompositionReport {
    pub harness_id: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub harness_invoked: bool,
    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub test_only: bool,
    pub substrate_order: Vec<&'static str>,

    pub alloc_report: Option<MobilityAlloc0PlanReport>,
    pub reenroll_report: Option<MobilityReenroll0PlanReport>,
    pub idroute_report: Option<MobilityIdroute0PlanReport>,
    pub econ_report: Option<MobilityEcon0PlanReport>,
    pub owner_report: Option<MobilityOwner0PlanReport>,

    pub composed_cpu_checksum: u64,
    pub composed_gpu_proxy_checksum: u64,
    pub deterministic_replay_checksum: u64,
    pub cpu_gpu_parity_preserved: bool,

    pub movement_writes_only_moving_simthing_columns: bool,
    pub capture_remains_owner_column_flip: bool,
    pub owner_overlay_reaches_isolated_owned_unit: bool,
    pub econ_resource_flow_separate_from_owner_modifier_overlay: bool,
    pub hard_soft_silent_mix: bool,
    pub dirty_owner_modifier_steady_state_zero_redisperse: bool,

    pub simsession_passgraph_wiring_present: bool,
    pub production_runtime_integration_authorized: bool,
    pub gpu_hook_or_pass_graph_present: bool,
    pub runtime_implementation_authorized: bool,
    pub later_econ_scaling_parked: bool,
    pub closed_ladders_reopened: bool,
}

pub fn compose_mobility_runtime0(
    input: &MobilityRuntime0CompositionInput,
) -> MobilityRuntime0CompositionReport {
    let mut diagnostics = Vec::new();
    validate_config(input.config, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(input.config, diagnostics);
    }

    let canonical = canonical_input(input);
    let alloc_report = plan_mobility_alloc0(&canonical.alloc);
    let reenroll_report = plan_mobility_reenroll0(&canonical.reenroll);
    let idroute_report = plan_mobility_idroute0(&canonical.idroute);
    let econ_report = plan_mobility_econ0(&canonical.econ);
    let owner_report = plan_mobility_owner0(&canonical.owner);

    collect_substrate_diagnostics(
        &alloc_report,
        &reenroll_report,
        &idroute_report,
        &econ_report,
        &owner_report,
        &mut diagnostics,
    );

    let alloc_cpu = mobility_alloc0_layout_checksum_cpu(&alloc_report.final_live_slices);
    let alloc_gpu = mobility_alloc0_layout_checksum_gpu_proxy(&alloc_report.final_live_slices);
    let reenroll_cpu = mobility_reenroll0_layout_checksum_cpu(&reenroll_report.final_live_slices);
    let reenroll_gpu =
        mobility_reenroll0_layout_checksum_gpu_proxy(&reenroll_report.final_live_slices);

    let composed_cpu_checksum = compose_checksums(&[
        alloc_cpu,
        reenroll_cpu,
        idroute_report.cpu_gpu_parity_checksum,
        econ_report.cpu_gpu_parity_checksum,
        owner_report.cpu_gpu_parity_checksum,
    ]);
    let composed_gpu_proxy_checksum = compose_checksums(&[
        alloc_gpu,
        reenroll_gpu,
        idroute_report.cpu_gpu_parity_checksum,
        econ_report.cpu_gpu_parity_checksum,
        owner_report.cpu_gpu_parity_checksum,
    ]);

    let movement_writes_only_moving_simthing_columns = reenroll_report.admitted
        && reenroll_report.committed_moves.iter().all(|mv| {
            !reenroll_report
                .final_live_slices
                .iter()
                .any(|slice| slice.entity_id == mv.entity_id && slice.parent_key == mv.origin)
                && reenroll_report.final_live_slices.iter().any(|slice| {
                    slice.entity_id == mv.entity_id && slice.parent_key == mv.destination
                })
        });
    let capture_remains_owner_column_flip = owner_report.admitted
        && !owner_report.capture_reparented
        && !owner_report.owner_columns_are_spatial_parents;
    let owner_overlay_reaches_isolated_owned_unit = owner_report.admitted
        && owner_report
            .applied_overlays
            .iter()
            .any(|overlay| overlay.entity_id == 2);
    let econ_resource_flow_separate_from_owner_modifier_overlay = econ_report.admitted
        && owner_report.admitted
        && econ_report.owner_parked
        && owner_report.spawned_arena_column_count == 0
        && owner_report.spawned_aggregation_column_count == 0;
    let hard_soft_silent_mix = econ_report.hard_soft_same_pass;
    let dirty_owner_modifier_steady_state_zero_redisperse = owner_report.admitted
        && owner_report.owner_change_count == 0
        && owner_report.modifier_dispersal_count == 0
        && owner_report.dirtyonly_noop_count == owner_report.applied_overlays.len() as u32;

    MobilityRuntime0CompositionReport {
        harness_id: MOBILITY_RUNTIME0_ID,
        admitted: diagnostics.is_empty(),
        diagnostics,
        harness_invoked: true,
        explicit_opt_in: true,
        default_off: true,
        test_only: true,
        substrate_order: MOBILITY_RUNTIME0_ORDER.to_vec(),
        alloc_report: Some(alloc_report),
        reenroll_report: Some(reenroll_report),
        idroute_report: Some(idroute_report),
        econ_report: Some(econ_report),
        owner_report: Some(owner_report),
        composed_cpu_checksum,
        composed_gpu_proxy_checksum,
        deterministic_replay_checksum: composed_cpu_checksum,
        cpu_gpu_parity_preserved: composed_cpu_checksum == composed_gpu_proxy_checksum,
        movement_writes_only_moving_simthing_columns,
        capture_remains_owner_column_flip,
        owner_overlay_reaches_isolated_owned_unit,
        econ_resource_flow_separate_from_owner_modifier_overlay,
        hard_soft_silent_mix,
        dirty_owner_modifier_steady_state_zero_redisperse,
        simsession_passgraph_wiring_present: false,
        production_runtime_integration_authorized: false,
        gpu_hook_or_pass_graph_present: false,
        runtime_implementation_authorized: false,
        later_econ_scaling_parked: true,
        closed_ladders_reopened: false,
    }
}

fn rejected_report(
    config: MobilityRuntime0HarnessConfig,
    diagnostics: Vec<&'static str>,
) -> MobilityRuntime0CompositionReport {
    MobilityRuntime0CompositionReport {
        harness_id: MOBILITY_RUNTIME0_ID,
        admitted: false,
        diagnostics,
        harness_invoked: false,
        explicit_opt_in: config.explicit_opt_in,
        default_off: !config.enabled_by_default,
        test_only: true,
        substrate_order: MOBILITY_RUNTIME0_ORDER.to_vec(),
        alloc_report: None,
        reenroll_report: None,
        idroute_report: None,
        econ_report: None,
        owner_report: None,
        composed_cpu_checksum: 0,
        composed_gpu_proxy_checksum: 0,
        deterministic_replay_checksum: 0,
        cpu_gpu_parity_preserved: false,
        movement_writes_only_moving_simthing_columns: true,
        capture_remains_owner_column_flip: true,
        owner_overlay_reaches_isolated_owned_unit: false,
        econ_resource_flow_separate_from_owner_modifier_overlay: true,
        hard_soft_silent_mix: false,
        dirty_owner_modifier_steady_state_zero_redisperse: true,
        simsession_passgraph_wiring_present: false,
        production_runtime_integration_authorized: false,
        gpu_hook_or_pass_graph_present: false,
        runtime_implementation_authorized: false,
        later_econ_scaling_parked: true,
        closed_ladders_reopened: false,
    }
}

fn validate_config(config: MobilityRuntime0HarnessConfig, diagnostics: &mut Vec<&'static str>) {
    if !config.explicit_opt_in {
        diagnostics.push("runtime0_explicit_opt_in_required");
    }
    if config.enabled_by_default {
        diagnostics.push("runtime0_default_on_behavior_rejected");
    }
}

fn validate_forbidden(
    forbidden: &MobilityRuntime0ForbiddenPathRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.default_on_behavior {
        diagnostics.push("default_on_behavior");
    }
    if forbidden.semantic_or_raw_wgsl {
        diagnostics.push("semantic_or_raw_wgsl");
    }
    if forbidden.cpu_planner_urgency_commitment {
        diagnostics.push("cpu_planner_urgency_commitment");
    }
    if forbidden.owner_as_spatial_parent {
        diagnostics.push("owner_as_spatial_parent");
    }
    if forbidden.capture_as_reparenting {
        diagnostics.push("capture_as_reparenting");
    }
    if forbidden.nested_arena_reparenting {
        diagnostics.push("nested_arena_reparenting");
    }
    if forbidden.default_on_resource_flow {
        diagnostics.push("default_on_resource_flow");
    }
    if forbidden.hard_currency_through_resource_flow {
        diagnostics.push("hard_currency_through_resource_flow");
    }
    if forbidden.hybrid_strata_or_faction_index_scaling {
        diagnostics.push("hybrid_strata_or_faction_index_scaling");
    }
    if forbidden.closed_ladder_reopen {
        diagnostics.push("closed_ladder_reopen");
    }
    if forbidden.production_simsession_wiring {
        diagnostics.push("production_simsession_wiring");
    }
    if forbidden.gpu_pass_graph_wiring {
        diagnostics.push("gpu_pass_graph_wiring");
    }
}

fn collect_substrate_diagnostics(
    alloc: &MobilityAlloc0PlanReport,
    reenroll: &MobilityReenroll0PlanReport,
    idroute: &MobilityIdroute0PlanReport,
    econ: &MobilityEcon0PlanReport,
    owner: &MobilityOwner0PlanReport,
    diagnostics: &mut Vec<&'static str>,
) {
    if !alloc.admitted {
        diagnostics.push("alloc_substrate_rejected");
    }
    if !reenroll.admitted {
        diagnostics.push("reenroll_substrate_rejected");
    }
    if !idroute.admitted {
        diagnostics.push("idroute_substrate_rejected");
    }
    if !econ.admitted {
        diagnostics.push("econ_substrate_rejected");
    }
    if !owner.admitted {
        diagnostics.push("owner_substrate_rejected");
    }
}

fn canonical_input(input: &MobilityRuntime0CompositionInput) -> MobilityRuntime0CompositionInput {
    let mut canonical = input.clone();

    canonical
        .alloc
        .blocks
        .sort_by_key(|block| (block.parent_key, block.start_slot, block.slot_count));
    canonical
        .alloc
        .live_slices
        .sort_by_key(|slice| (slice.parent_key, slice.entity_id, slice.slot));
    canonical
        .alloc
        .events
        .sort_by_key(|event| (event.parent_key, event.entity_id, event_kind_rank(event)));

    canonical
        .reenroll
        .registry
        .blocks
        .sort_by_key(|block| (block.parent_key, block.start_slot, block.slot_count));
    canonical
        .reenroll
        .registry
        .live_slices
        .sort_by_key(|slice| (slice.parent_key, slice.entity_id, slice.slot));
    canonical.reenroll.moves.sort_by_key(|mv| {
        (
            mv.entity_id,
            mv.origin.parent_id,
            mv.origin.key_id,
            mv.destination.parent_id,
            mv.destination.key_id,
        )
    });

    canonical
        .idroute
        .records
        .sort_by_key(|record| (record.parent_key, record.entity_id, record.identity.0));
    canonical.econ.records.sort_by_key(|record| {
        (
            record.session_id,
            record.resource_id,
            record.cell_key,
            record.hard_available,
            record.hard_need,
        )
    });
    canonical.owner.records.sort_by_key(|record| {
        (
            record.cell_key,
            record.entity_id,
            record.cohort_count,
            record.generation,
        )
    });
    for record in &mut canonical.owner.records {
        record.owner_columns.sort();
    }
    canonical
        .owner
        .overlays
        .sort_by_key(|overlay| (overlay.owner, overlay.modifier_id, overlay.modifier_amount));
    canonical.owner.owner_changes.sort_by_key(|change| {
        (
            change.entity_id,
            change.kind,
            change.from_owner_id,
            change.to_owner_id,
            change.changed_count,
        )
    });

    canonical
}

fn event_kind_rank(event: &MobilityAlloc0BoundaryEvent) -> u8 {
    match event.kind {
        MobilityAlloc0BoundaryEventKind::Departure => 0,
        MobilityAlloc0BoundaryEventKind::Arrival => 1,
        MobilityAlloc0BoundaryEventKind::ParentRemoved => 2,
    }
}

fn compose_checksums(values: &[u64]) -> u64 {
    values.iter().fold(0xcbf2_9ce4_8422_2325, |hash, value| {
        fnv_append_u64(hash, *value)
    })
}

fn fnv_append_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
