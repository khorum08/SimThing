//! MOBILITY-SCENARIO-0 scenario/admission tests.

use simthing_spec::{
    admit_mobility_scenario0_packet, deserialize_mobility_scenario0_packet_ron,
    mobility_scenario0_packet, serialize_mobility_scenario0_packet_ron,
    DesignerAdmissionDiagnosticCode, MobilityScenario0Packet, MOBILITY_SCENARIO0_ENTITY_TARGET,
};

fn happy() -> MobilityScenario0Packet {
    mobility_scenario0_packet()
}

fn assert_rejects_code(packet: MobilityScenario0Packet, code: DesignerAdmissionDiagnosticCode) {
    let admission = admit_mobility_scenario0_packet(&packet);
    assert!(!admission.admitted, "expected rejection for {code:?}");
    assert!(
        admission.diagnostics.iter().any(|d| d.code == code),
        "missing {code:?}: {:?}",
        admission.diagnostics
    );
}

#[test]
fn mobility_scenario0_accepts_bounded_named_scenario() {
    let packet = happy();
    let ron = serialize_mobility_scenario0_packet_ron(&packet).expect("serialize");
    let parsed = deserialize_mobility_scenario0_packet_ron(&ron).expect("roundtrip");
    let admission = admit_mobility_scenario0_packet(&parsed);

    assert!(admission.admitted, "{:?}", admission.diagnostics);
    assert!(!admission.implementation_authorized);
    assert_eq!(admission.parameter_summary.spatial_depth, 4);
    assert_eq!(admission.parameter_summary.cells, 48);
    assert_eq!(admission.parameter_summary.max_factions_per_cell, 4);
    assert_eq!(admission.parameter_summary.routing_eml_node_budget, 16);
    assert_eq!(admission.parameter_summary.moving_entity_block_size, 96);
    assert!(admission
        .parameter_summary
        .simthing_slot_kinds
        .contains(&"fleet".to_string()));
    assert!(admission
        .parameter_summary
        .count_columns
        .contains(&"population_count".to_string()));
}

#[test]
fn mobility_scenario0_rejects_owner_as_spatial_parent() {
    let mut packet = happy();
    packet.guardrails.owner_entities_as_spatial_parents = true;
    assert_rejects_code(
        packet,
        DesignerAdmissionDiagnosticCode::MobilityOwnerSpatialParentRejected,
    );
}

#[test]
fn mobility_scenario0_rejects_capture_as_reparenting() {
    let mut packet = happy();
    packet.guardrails.capture_as_reparenting = true;
    assert_rejects_code(
        packet,
        DesignerAdmissionDiagnosticCode::MobilityCaptureAsReparentingRejected,
    );
}

#[test]
fn mobility_scenario0_rejects_semantic_wgsl() {
    let mut packet = happy();
    packet.guardrails.semantic_wgsl = true;
    assert_rejects_code(
        packet,
        DesignerAdmissionDiagnosticCode::SemanticWgslRequestRejected,
    );
}

#[test]
fn mobility_scenario0_rejects_gpu_allocator_semaphore() {
    let mut packet = happy();
    packet.guardrails.gpu_allocator_semaphore = true;
    assert_rejects_code(
        packet,
        DesignerAdmissionDiagnosticCode::MobilityGpuAllocatorSemaphoreRejected,
    );
}

#[test]
fn mobility_scenario0_rejects_indirection_before_slab() {
    let mut packet = happy();
    packet.guardrails.indirection_buffer_before_slab = true;
    assert_rejects_code(
        packet,
        DesignerAdmissionDiagnosticCode::MobilityIndirectionBeforeSlabRejected,
    );
}

#[test]
fn mobility_scenario0_rejects_hard_soft_mixed_pass() {
    let mut packet = happy();
    packet.quantity_classes.hard_and_soft_never_silently_mix = false;
    assert_rejects_code(
        packet,
        DesignerAdmissionDiagnosticCode::MobilityHardSoftMixedPassRejected,
    );
}

#[test]
fn mobility_scenario0_rejects_default_on_resource_flow() {
    let mut packet = happy();
    packet.supply_scope.default_on_resource_flow = true;
    assert_rejects_code(packet, DesignerAdmissionDiagnosticCode::DefaultOnRejected);
}

#[test]
fn mobility_scenario0_rejects_clausething_or_closed_ladder_reopen() {
    let mut clausething = happy();
    clausething.guardrails.reopen_clausething_l3 = true;
    assert_rejects_code(
        clausething,
        DesignerAdmissionDiagnosticCode::ClauseThingRuntimeRequestParked,
    );

    let mut closed = happy();
    closed.guardrails.reopen_closed_ladders = true;
    assert_rejects_code(
        closed,
        DesignerAdmissionDiagnosticCode::MobilityClosedLadderReopenRejected,
    );
}

#[test]
fn mobility_scenario0_records_34k_soak_profile() {
    let packet = happy();
    let admission = admit_mobility_scenario0_packet(&packet);
    assert!(admission.admitted, "{:?}", admission.diagnostics);
    assert_eq!(
        admission.parameter_summary.soak_entity_count,
        MOBILITY_SCENARIO0_ENTITY_TARGET
    );
    assert_eq!(packet.soak.entity_count, 34_000);
    assert!(packet
        .soak
        .stress_mix
        .iter()
        .any(|entry| entry.contains("REENROLL")));
}

#[test]
fn mobility_scenario0_rejects_arrival_order_or_silent_channel_rebind() {
    let mut arrival = happy();
    arrival.routing.uses_arrival_order_as_replay_ordering = true;
    assert_rejects_code(
        arrival,
        DesignerAdmissionDiagnosticCode::MobilityArrivalOrderReplayOrderingRejected,
    );

    let mut rebind = happy();
    rebind.routing.silent_hybrid_strata_rebind = true;
    assert_rejects_code(
        rebind,
        DesignerAdmissionDiagnosticCode::MobilityHybridStrataSilentRebindRejected,
    );
}

#[test]
fn mobility_scenario0_rejects_float_structural_gate_and_budget_overrun() {
    let mut float_gate = happy();
    float_gate
        .quantity_classes
        .float_values_gate_structural_transitions = true;
    assert_rejects_code(
        float_gate,
        DesignerAdmissionDiagnosticCode::MobilityFloatStructuralGateRejected,
    );

    let mut faction_over = happy();
    faction_over
        .identity_channels
        .first_slice_expected_peak_factions_per_cell = 5;
    assert_rejects_code(
        faction_over,
        DesignerAdmissionDiagnosticCode::MobilityMaxFactionsPerCellExceeded,
    );

    let mut eml_over = happy();
    eml_over.identity_channels.routing_eml_node_budget = 12;
    assert_rejects_code(
        eml_over,
        DesignerAdmissionDiagnosticCode::MobilityRoutingEmlNodeBudgetExceeded,
    );
}

#[test]
fn mobility_scenario0_rejects_hard_currency_through_resource_flow() {
    let mut packet = happy();
    packet
        .supply_scope
        .hard_currency_routes_through_resource_flow = true;
    assert_rejects_code(
        packet,
        DesignerAdmissionDiagnosticCode::MobilityHardCurrencyThroughResourceFlowRejected,
    );
}
