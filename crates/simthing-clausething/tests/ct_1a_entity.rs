//! CT-1a literal entity hydration parity: ClauseScript ≡ hand-authored RON baseline.

use simthing_clausething::{
    admit_and_apply_domain_pack, admit_and_apply_pack, hydrate_entity_pack, parse_raw_document,
};
use simthing_spec::spec::domain_pack::DomainPackSpec;

const CLAUSE_FIXTURE: &str = include_str!("fixtures/ct1a_demo_entity.clause");
const RON_BASELINE: &str = include_str!("fixtures/ct1a_demo_entity_baseline.ron");
const SEED_AMOUNT: f32 = 40.0;

fn load_ron_baseline() -> DomainPackSpec {
    ron::from_str(RON_BASELINE).expect("parse RON baseline")
}

fn canonical_json(pack: &DomainPackSpec) -> String {
    serde_json::to_string(pack).expect("serialize domain pack")
}

fn hydrate_from_clause() -> simthing_clausething::HydratedEntityPack {
    let document = parse_raw_document(CLAUSE_FIXTURE.as_bytes()).expect("parse clause fixture");
    hydrate_entity_pack(&document).expect("hydrate clause fixture")
}

#[test]
fn hydrated_domain_pack_matches_ron_baseline() {
    let hydrated = hydrate_from_clause();
    let baseline = load_ron_baseline();
    assert_eq!(
        canonical_json(&hydrated.domain_pack),
        canonical_json(&baseline),
        "hydrated authoring struct must match RON baseline"
    );
    assert_eq!(hydrated.seed_amount, SEED_AMOUNT);
}

#[test]
fn clause_and_ron_install_snapshots_match() {
    let hydrated = hydrate_from_clause();
    let baseline = load_ron_baseline();

    let from_clause = admit_and_apply_pack(&hydrated).expect("admit hydrated pack");
    let from_ron = admit_and_apply_domain_pack(&baseline, SEED_AMOUNT).expect("admit RON baseline");

    assert_eq!(from_clause, from_ron);
    assert_eq!(from_clause.seeded_amount, SEED_AMOUNT);
    assert_eq!(from_clause.final_amount, 50.0);
    assert_eq!(
        from_clause.property_keys,
        vec!["simthing::potency".to_string()]
    );
}

#[test]
fn unsupported_entity_field_is_hard_error() {
    let text = include_str!("fixtures/ct1a_unsupported_field.clause");
    let document = parse_raw_document(text.as_bytes()).expect("parse unsupported fixture");
    let err = hydrate_entity_pack(&document).expect_err("unsupported field must fail");
    assert!(
        err.message.contains("triggered_modifier"),
        "expected unsupported field diagnostic, got: {}",
        err.message
    );
    assert!(err.span.is_some(), "expected spanned diagnostic");
}
