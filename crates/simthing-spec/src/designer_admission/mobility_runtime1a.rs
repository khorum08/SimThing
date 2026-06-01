//! MOBILITY-RUNTIME-1A: CPU-only default-off production-fixture model in `simthing-spec`.
//!
//! Models the authorized production `SimSession` fixture surface and delegates to the
//! green RUNTIME-0 composition harness when explicitly opted in. This is a designer/spec
//! admission fixture model only — it does **not** wire `simthing-driver`, `simthing-gpu`,
//! or any production runtime crate. CPU-only; no GPU pass-graph registration, default
//! schedule, or gameplay integration. Actual runtime-crate fixture wiring is a separate,
//! currently-closed gate (MOBILITY-RUNTIME-1A-RUNTIME-FIXTURE).

use super::mobility_runtime0::{
    compose_mobility_runtime0, MobilityRuntime0CompositionInput,
    MobilityRuntime0CompositionReport, MobilityRuntime0ForbiddenPathRequests,
    MobilityRuntime0HarnessConfig, MOBILITY_RUNTIME0_ORDER,
};

pub const MOBILITY_RUNTIME1A_ID: &str = "mobility_runtime1a_cpu_only_production_fixture";
pub const MOBILITY_RUNTIME1A_NAMED_GATE: &str = "mobility_runtime1a_explicit_opt_in_gate";
/// Closed follow-on gate for actual `simthing-driver` / production runtime crate fixture wiring.
pub const MOBILITY_RUNTIME1A_RUNTIME_FIXTURE_GATE: &str =
    "mobility_runtime1a_runtime_crate_fixture_closed";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityRuntime1aFixtureGate {
    pub explicit_named_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl MobilityRuntime1aFixtureGate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_named_gate_enabled: true,
            enabled_by_default: false,
        }
    }
}

/// Production `SimSession` fixture surface **model** in `simthing-spec`. Default-off until the
/// named gate opts in. Does not mutate or register hooks in production runtime crates.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MobilityRuntime1aSimSessionSurface {
    pub gate: MobilityRuntime1aFixtureGate,
    pub named_fixture_registered: bool,
    pub composition_invocations: u32,
}

impl MobilityRuntime1aSimSessionSurface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: MobilityRuntime1aFixtureGate::explicit_opt_in(),
            ..Self::default()
        }
    }
}

pub type MobilityRuntime1aForbiddenPathRequests = MobilityRuntime0ForbiddenPathRequests;

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityRuntime1aProductionFixtureInput {
    pub surface: MobilityRuntime1aSimSessionSurface,
    pub composition: MobilityRuntime0CompositionInput,
    pub forbidden: MobilityRuntime1aForbiddenPathRequests,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityRuntime1aProductionFixtureReport {
    pub fixture_id: &'static str,
    pub named_gate: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub cpu_only: bool,
    pub disabled_no_op: bool,
    pub fixture_invoked: bool,
    pub named_fixture_registered: bool,
    pub default_simsession_behavior_unchanged: bool,
    pub passgraph_schedule_registered: bool,
    pub gpu_passgraph_registered: bool,
    pub gpu_hook_or_pass_graph_present: bool,
    pub simsession_passgraph_wiring_present: bool,
    pub unscoped_gpu_passgraph_wiring: bool,
    pub production_fixture_wiring_authorized: bool,
    pub runtime1b_gpu_gate_closed: bool,
    pub runtime0_harness_delegated: bool,
    pub simthing_spec_fixture_model_only: bool,
    pub real_simsession_runtime_crate_wiring_present: bool,
    pub runtime_crate_fixture_gate_closed: bool,

    pub substrate_order: Vec<&'static str>,
    pub composition: Option<MobilityRuntime0CompositionReport>,

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

    pub later_econ_scaling_parked: bool,
    pub closed_ladders_reopened: bool,
    pub composition_invocations: u32,
}

pub fn run_mobility_runtime1a_production_fixture(
    input: &MobilityRuntime1aProductionFixtureInput,
) -> MobilityRuntime1aProductionFixtureReport {
    let mut diagnostics = Vec::new();
    validate_gate(input.surface.gate, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(&input.surface, diagnostics);
    }

    if !input.surface.gate.explicit_named_gate_enabled {
        return disabled_no_op_report(&input.surface);
    }

    let mut composition_input = input.composition.clone();
    composition_input.config = MobilityRuntime0HarnessConfig::opt_in_test_harness();
    composition_input.forbidden = MobilityRuntime0ForbiddenPathRequests::default();

    let composition = compose_mobility_runtime0(&composition_input);
    if !composition.admitted {
        let mut merged = diagnostics;
        merged.extend(composition.diagnostics);
        return rejected_report(&input.surface, merged);
    }

    let mut surface = input.surface.clone();
    surface.named_fixture_registered = true;
    surface.composition_invocations = surface.composition_invocations.saturating_add(1);

    admitted_report(surface, composition)
}

fn validate_gate(gate: MobilityRuntime1aFixtureGate, diagnostics: &mut Vec<&'static str>) {
    if gate.enabled_by_default {
        diagnostics.push("runtime1_default_on_behavior_rejected");
    }
}

fn validate_forbidden(
    forbidden: &MobilityRuntime1aForbiddenPathRequests,
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
    if forbidden.gpu_pass_graph_wiring {
        diagnostics.push("unscoped_gpu_passgraph_wiring");
    }
}

fn disabled_no_op_report(
    surface: &MobilityRuntime1aSimSessionSurface,
) -> MobilityRuntime1aProductionFixtureReport {
    MobilityRuntime1aProductionFixtureReport {
        fixture_id: MOBILITY_RUNTIME1A_ID,
        named_gate: MOBILITY_RUNTIME1A_NAMED_GATE,
        admitted: true,
        diagnostics: Vec::new(),
        explicit_opt_in: false,
        default_off: true,
        cpu_only: true,
        disabled_no_op: true,
        fixture_invoked: false,
        named_fixture_registered: surface.named_fixture_registered,
        default_simsession_behavior_unchanged: true,
        passgraph_schedule_registered: false,
        gpu_passgraph_registered: false,
        gpu_hook_or_pass_graph_present: false,
        simsession_passgraph_wiring_present: false,
        unscoped_gpu_passgraph_wiring: false,
        production_fixture_wiring_authorized: false,
        runtime1b_gpu_gate_closed: true,
        runtime0_harness_delegated: false,
        simthing_spec_fixture_model_only: true,
        real_simsession_runtime_crate_wiring_present: false,
        runtime_crate_fixture_gate_closed: true,
        substrate_order: MOBILITY_RUNTIME0_ORDER.to_vec(),
        composition: None,
        composed_cpu_checksum: 0,
        composed_gpu_proxy_checksum: 0,
        deterministic_replay_checksum: 0,
        cpu_gpu_parity_preserved: true,
        movement_writes_only_moving_simthing_columns: true,
        capture_remains_owner_column_flip: true,
        owner_overlay_reaches_isolated_owned_unit: false,
        econ_resource_flow_separate_from_owner_modifier_overlay: true,
        hard_soft_silent_mix: false,
        dirty_owner_modifier_steady_state_zero_redisperse: true,
        later_econ_scaling_parked: true,
        closed_ladders_reopened: false,
        composition_invocations: surface.composition_invocations,
    }
}

fn rejected_report(
    surface: &MobilityRuntime1aSimSessionSurface,
    diagnostics: Vec<&'static str>,
) -> MobilityRuntime1aProductionFixtureReport {
    MobilityRuntime1aProductionFixtureReport {
        fixture_id: MOBILITY_RUNTIME1A_ID,
        named_gate: MOBILITY_RUNTIME1A_NAMED_GATE,
        admitted: false,
        diagnostics,
        explicit_opt_in: surface.gate.explicit_named_gate_enabled,
        default_off: !surface.gate.enabled_by_default,
        cpu_only: true,
        disabled_no_op: false,
        fixture_invoked: false,
        named_fixture_registered: surface.named_fixture_registered,
        default_simsession_behavior_unchanged: !surface.gate.explicit_named_gate_enabled,
        passgraph_schedule_registered: false,
        gpu_passgraph_registered: false,
        gpu_hook_or_pass_graph_present: false,
        simsession_passgraph_wiring_present: false,
        unscoped_gpu_passgraph_wiring: false,
        production_fixture_wiring_authorized: false,
        runtime1b_gpu_gate_closed: true,
        runtime0_harness_delegated: false,
        simthing_spec_fixture_model_only: true,
        real_simsession_runtime_crate_wiring_present: false,
        runtime_crate_fixture_gate_closed: true,
        substrate_order: MOBILITY_RUNTIME0_ORDER.to_vec(),
        composition: None,
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
        later_econ_scaling_parked: true,
        closed_ladders_reopened: false,
        composition_invocations: surface.composition_invocations,
    }
}

fn admitted_report(
    surface: MobilityRuntime1aSimSessionSurface,
    composition: MobilityRuntime0CompositionReport,
) -> MobilityRuntime1aProductionFixtureReport {
    MobilityRuntime1aProductionFixtureReport {
        fixture_id: MOBILITY_RUNTIME1A_ID,
        named_gate: MOBILITY_RUNTIME1A_NAMED_GATE,
        admitted: true,
        diagnostics: Vec::new(),
        explicit_opt_in: true,
        default_off: true,
        cpu_only: true,
        disabled_no_op: false,
        fixture_invoked: true,
        named_fixture_registered: surface.named_fixture_registered,
        default_simsession_behavior_unchanged: false,
        passgraph_schedule_registered: false,
        gpu_passgraph_registered: false,
        gpu_hook_or_pass_graph_present: false,
        simsession_passgraph_wiring_present: false,
        unscoped_gpu_passgraph_wiring: false,
        production_fixture_wiring_authorized: true,
        runtime1b_gpu_gate_closed: true,
        runtime0_harness_delegated: true,
        simthing_spec_fixture_model_only: true,
        real_simsession_runtime_crate_wiring_present: false,
        runtime_crate_fixture_gate_closed: true,
        substrate_order: composition.substrate_order.clone(),
        composed_cpu_checksum: composition.composed_cpu_checksum,
        composed_gpu_proxy_checksum: composition.composed_gpu_proxy_checksum,
        deterministic_replay_checksum: composition.deterministic_replay_checksum,
        cpu_gpu_parity_preserved: composition.cpu_gpu_parity_preserved,
        movement_writes_only_moving_simthing_columns: composition
            .movement_writes_only_moving_simthing_columns,
        capture_remains_owner_column_flip: composition.capture_remains_owner_column_flip,
        owner_overlay_reaches_isolated_owned_unit: composition
            .owner_overlay_reaches_isolated_owned_unit,
        econ_resource_flow_separate_from_owner_modifier_overlay: composition
            .econ_resource_flow_separate_from_owner_modifier_overlay,
        hard_soft_silent_mix: composition.hard_soft_silent_mix,
        dirty_owner_modifier_steady_state_zero_redisperse: composition
            .dirty_owner_modifier_steady_state_zero_redisperse,
        later_econ_scaling_parked: composition.later_econ_scaling_parked,
        closed_ladders_reopened: composition.closed_ladders_reopened,
        composition_invocations: surface.composition_invocations,
        composition: Some(composition),
    }
}
