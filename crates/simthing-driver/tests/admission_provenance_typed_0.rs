use simthing_core::{SimThingId, SubFieldRole};
use simthing_driver::{install::InstallError, Scenario, SessionError, SimSession};
use simthing_spec::{
    EmissionFormulaSpec, GameModeSpec, PropertyKey, ResourceEconomySpec, ResourceEmissionSpec,
    SpecError,
};

fn scenario() -> Scenario {
    Scenario::rebellion_demo("admission-provenance-typed-0".into(), 1, 1, 0.0, 16)
}

fn emission(source: PropertyKey, host_entity: Option<&str>) -> ResourceEmissionSpec {
    ResourceEmissionSpec {
        id: "typed-source-emission".into(),
        source,
        source_role: SubFieldRole::Amount,
        formula: EmissionFormulaSpec::Constant(1.0),
        host_entity: host_entity.map(str::to_owned),
        host_span_token: None,
    }
}

fn game_mode_with_emission(emission: ResourceEmissionSpec) -> GameModeSpec {
    GameModeSpec {
        id: "admission-provenance-typed-0".into(),
        display_name: "Admission provenance typed 0".into(),
        resource_economy: Some(ResourceEconomySpec {
            emissions: vec![emission],
            ..ResourceEconomySpec::default()
        }),
        ..GameModeSpec::default()
    }
}

fn assert_typed_refusal(error: SessionError, expected_law: &str, expected_path: &str) {
    match error {
        SessionError::Install(InstallError::Spec(SpecError::AdmissionRefused {
            law_id,
            element_path,
        })) => {
            assert_eq!(law_id, expected_law);
            assert_eq!(element_path, expected_path);
        }
        other => panic!("unexpected open_from_spec error: {other:?}"),
    }
}

fn rejected_open(scenario: Scenario, game_mode: &GameModeSpec) -> SessionError {
    match SimSession::open_from_spec(scenario, game_mode) {
        Ok(_) => panic!("fixture must remain rejected"),
        Err(error) => error,
    }
}

#[test]
fn open_from_spec_preserves_typed_law_and_element_through_both_wrappers() {
    let missing = PropertyKey {
        namespace: "missing".into(),
        name: "stock".into(),
    };
    let expected_path = "resource_economy.properties[key=\"missing::stock\"]";

    for host in [None, Some("authored-host")] {
        let game_mode = game_mode_with_emission(emission(missing.clone(), host));
        let error = rejected_open(scenario(), &game_mode);
        assert_typed_refusal(error, "resource-economy-property-registered", expected_path);
    }
}

#[test]
fn admitted_property_with_missing_authored_host_keeps_the_host_law_and_identity() {
    let mut scenario = scenario();
    scenario.install_targets.insert(
        "missing-live-host".into(),
        vec![SimThingId::from_session_raw(9_999)],
    );
    let game_mode = game_mode_with_emission(emission(
        PropertyKey {
            namespace: "core".into(),
            name: "loyalty".into(),
        },
        Some("missing-live-host"),
    ));

    let error = rejected_open(scenario, &game_mode);
    assert_typed_refusal(
        error,
        "resource-economy-property-host-live",
        "simthings[id=9999].properties[key=\"core::loyalty\"]",
    );
}

#[test]
fn accepted_resource_economy_fixture_still_opens() {
    let game_mode = game_mode_with_emission(emission(
        PropertyKey {
            namespace: "core".into(),
            name: "loyalty".into(),
        },
        None,
    ));

    SimSession::open_from_spec(scenario(), &game_mode)
        .expect("the paired registered-property fixture must remain accepted");
}
