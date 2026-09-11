//! Stage records retain their contract; the literal modifier successor uses ordinary admission.
use std::path::Path;

use simthing_clausething::{
    hydrate_entity_pack, hydrate_scenario, hydrate_scenario_with_source_base, parse_raw_document,
};
use simthing_core::{ClampBehavior, DimensionRegistry, SimThingKind, SubFieldRole};
use simthing_spec::{compile_property, DomainPackSpec};

const LEGACY: &str = include_str!("fixtures/ct1a_demo_entity.clause");
const BASELINE: &str = include_str!("fixtures/ct1a_demo_entity_baseline.ron");
const SUCCESSOR: &str =
    include_str!("../../../scenarios/rehearsal_ingress_literal_successor.clause");

#[test]
fn rehearsal_ingress_literal_record_retains_hydration_parity() {
    let record = parse_raw_document(LEGACY.as_bytes()).unwrap();
    let hydrated = hydrate_entity_pack(&record).unwrap();
    let baseline: DomainPackSpec = ron::from_str(BASELINE).unwrap();
    assert_eq!(
        serde_json::to_value(&hydrated.domain_pack).unwrap(),
        serde_json::to_value(&baseline).unwrap(),
        "the historical hydration record retains full authored spec parity"
    );
    assert_eq!(hydrated.seed_amount, 40.0);
    assert_eq!(hydrated.seeds, vec![("simthing::potency".into(), 40.0)]);
    assert!(
        hydrate_scenario(&record).is_err(),
        "an entity record is not a live scenario"
    );

    let source_base = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios");
    for (text, multiplier) in [
        (SUCCESSOR.to_string(), 1.25),
        (
            SUCCESSOR.replace("amount_mult = 1.25", "amount_mult = 1.5"),
            1.5,
        ),
    ] {
        let raw = parse_raw_document(text.as_bytes()).unwrap();
        let pack = hydrate_scenario_with_source_base(&raw, Some(&source_base)).unwrap();
        assert_eq!(pack.game_mode.properties.len(), 1);
        let property = &pack.game_mode.properties[0];
        let old_property = &baseline.properties[0];
        assert_eq!(property.id, old_property.id);
        assert_eq!(property.namespace, old_property.namespace);
        assert_eq!(property.name, old_property.name);
        assert_eq!(property.display_name, old_property.display_name);
        assert_eq!(property.sub_fields.len(), 1);
        assert_eq!(property.sub_fields[0].role, SubFieldRole::Amount);
        assert_eq!(property.sub_fields[0].clamp, ClampBehavior::Unbounded);
        assert_eq!(property.sub_fields[0].governed_by, None);
        assert_eq!(pack.game_mode.overlays.len(), 1);
        let overlay = &pack.game_mode.overlays[0];
        let old = &baseline.overlays[0];
        assert_eq!(overlay.id, old.id);
        assert_eq!(overlay.targets_property, old.targets_property);
        assert_eq!(overlay.kind, old.kind);
        assert_eq!(overlay.source, old.source);
        assert_eq!(overlay.lifecycle, old.lifecycle);
        assert_eq!(overlay.sub_field_deltas[0].0, SubFieldRole::Amount);
        assert_eq!(
            overlay.sub_field_deltas[0].1.as_multiply_literal(),
            Some(multiplier)
        );

        let mut registry = DimensionRegistry::new();
        for property in &pack.game_mode.properties {
            compile_property(property, &mut registry).unwrap();
        }
        let root = pack
            .authority_root
            .as_ref()
            .unwrap()
            .children
            .iter()
            .find(|node| node.kind == SimThingKind::GameSession)
            .unwrap()
            .clone();
        let host = pack.install_targets["specimen"][0];
        let owner = root.children.iter().find(|node| node.id == host).unwrap();
        let property = registry.id_of("simthing", "potency").unwrap();
        assert_eq!(
            owner.properties[&property]
                .get_role(&SubFieldRole::Amount, &registry.property(property).layout),
            hydrated.seed_amount
        );
    }
}
