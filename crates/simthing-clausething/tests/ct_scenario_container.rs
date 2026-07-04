//! BH3-CLOSEOUT PR2–PR7 scenario-container grammar/lowering guardrails (LIVE_GUARDRAIL battery).
//!
//! Focused closeout command: `cargo test -p simthing-clausething --test ct_scenario_container`.
//! Covers parse/lower for canonical sample, SaturatingFlux, PALMA W/D feedstock, FIELD_POLICY
//! commitment, bounded links/grid metadata, default-off posture, and semantic-free lowering.

use simthing_clausething::{
    HydratedScenarioGridPlacement, HydratedScenarioLink, hydrate_scenario, parse_raw_document,
};
use simthing_core::{SimThingKind, TransformOp};
use simthing_spec::compile_region_field_preview;
use simthing_spec::{
    FIRST_SLICE_FIELD_URGENCY_COL, InstallTargetSpec, MappingExecutionProfile,
    RegionFieldOperatorSpec,
};

const FIXTURE: &str = include_str!("fixtures/ct_scenario_container_minimal.clause");
const LINK_FIXTURE: &str = include_str!("fixtures/ct_scenario_container_with_links.clause");
const FIELD_OPERATOR_FIXTURE: &str =
    include_str!("fixtures/ct_scenario_container_with_field_operator.clause");
const PALMA_FEEDSTOCK_FIXTURE: &str =
    include_str!("fixtures/ct_scenario_container_with_palma_feedstock.clause");
const COMMITMENT_FIXTURE: &str =
    include_str!("fixtures/ct_scenario_container_with_commitment.clause");
const CANONICAL_SAMPLE_FIXTURE: &str = include_str!("fixtures/ct_bh3_closeout_sample.clause");

fn hydrate_fixture() -> simthing_clausething::HydratedScenarioPack {
    let document = parse_raw_document(FIXTURE.as_bytes()).expect("parse scenario fixture");
    hydrate_scenario(&document).expect("hydrate scenario fixture")
}

fn hydrate_link_fixture() -> simthing_clausething::HydratedScenarioPack {
    let document = parse_raw_document(LINK_FIXTURE.as_bytes()).expect("parse linked fixture");
    hydrate_scenario(&document).expect("hydrate linked fixture")
}

fn hydrate_field_operator_fixture() -> simthing_clausething::HydratedScenarioPack {
    let document =
        parse_raw_document(FIELD_OPERATOR_FIXTURE.as_bytes()).expect("parse field op fixture");
    hydrate_scenario(&document).expect("hydrate field op fixture")
}

fn hydrate_palma_feedstock_fixture() -> simthing_clausething::HydratedScenarioPack {
    let document = parse_raw_document(PALMA_FEEDSTOCK_FIXTURE.as_bytes())
        .expect("parse palma feedstock fixture");
    hydrate_scenario(&document).expect("hydrate palma feedstock fixture")
}

fn hydrate_commitment_fixture() -> simthing_clausething::HydratedScenarioPack {
    let document =
        parse_raw_document(COMMITMENT_FIXTURE.as_bytes()).expect("parse commitment fixture");
    hydrate_scenario(&document).expect("hydrate commitment fixture")
}

fn hydrate_canonical_sample_fixture() -> simthing_clausething::HydratedScenarioPack {
    let document =
        parse_raw_document(CANONICAL_SAMPLE_FIXTURE.as_bytes()).expect("parse canonical sample");
    hydrate_scenario(&document).expect("hydrate canonical sample")
}

#[test]
fn duplicate_and_reversed_links_are_canonicalized_deterministically() {
    let source = br#"
scenario = duplicate_links {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    link = { from = beta to = alpha }
    link = { from = alpha to = beta }
    link = { from = beta to = alpha }
}
"#;
    let document = parse_raw_document(source).expect("parse duplicate links scenario");
    let pack = hydrate_scenario(&document).expect("hydrate duplicate links scenario");

    assert_eq!(
        pack.grid_metadata.links,
        vec![HydratedScenarioLink {
            from: "alpha".into(),
            to: "beta".into()
        }]
    );
}
