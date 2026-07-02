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
