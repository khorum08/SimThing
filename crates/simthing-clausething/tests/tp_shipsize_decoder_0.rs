//! TP-SHIPSIZE-DECODER-0 table-driven decoder / lowering proof.

use simthing_clausething::{
    ShipModifierFamily, ShipModifierOp, compile_value_formula_eml, decode_ship_modifier_key,
    hydrate_shipsize_decoder_pack, parse_raw_document, MAX_SHIP_EML_NODES,
};
use simthing_core::{OverlayLifecycle, TransformOp, eml_nodes};
use simthing_spec::RateFormulaOperandSpec;

const FIXTURE: &str = include_str!("fixtures/tp_shipsize_decoder_0.clause");

struct DecodeCase {
    label: &'static str,
    classes: &'static [&'static str],
    key: &'static str,
    expect_ok: bool,
    expect_shipsize_class: Option<&'static str>,
    expect_family_tag: Option<&'static str>,
    expect_attribute: Option<&'static str>,
    expect_op: Option<ShipModifierOp>,
    expect_err_contains: Option<&'static str>,
}

const DECODE_CASES: &[DecodeCase] = &[
    DecodeCase {
        label: "shipsize_corvette_hull_add",
        classes: &["corvette", "destroyer"],
        key: "shipsize_corvette_hull_add",
        expect_ok: true,
        expect_shipsize_class: Some("corvette"),
        expect_family_tag: Some("shipsize"),
        expect_attribute: Some("hull"),
        expect_op: Some(ShipModifierOp::Add),
        expect_err_contains: None,
    },
    DecodeCase {
        label: "ship_weapon_damage_mult",
        classes: &["corvette"],
        key: "ship_weapon_damage_mult",
        expect_ok: true,
        expect_shipsize_class: None,
        expect_family_tag: Some("ship"),
        expect_attribute: Some("weapon_damage"),
        expect_op: Some(ShipModifierOp::Mult),
        expect_err_contains: None,
    },
    DecodeCase {
        label: "ships_upkeep_mult",
        classes: &["corvette"],
        key: "ships_upkeep_mult",
        expect_ok: true,
        expect_shipsize_class: None,
        expect_family_tag: Some("ships"),
        expect_attribute: Some("upkeep"),
        expect_op: Some(ShipModifierOp::Mult),
        expect_err_contains: None,
    },
    DecodeCase {
        label: "country_naval_cap_add",
        classes: &["corvette"],
        key: "country_naval_cap_add",
        expect_ok: true,
        expect_shipsize_class: None,
        expect_family_tag: Some("country"),
        expect_attribute: Some("naval_cap"),
        expect_op: Some(ShipModifierOp::Add),
        expect_err_contains: None,
    },
    DecodeCase {
        label: "longest_class_segmentation_wins",
        classes: &["corvette", "corvette_hull"],
        key: "shipsize_corvette_hull_hull_add",
        expect_ok: true,
        expect_shipsize_class: Some("corvette_hull"),
        expect_family_tag: Some("shipsize"),
        expect_attribute: Some("hull"),
        expect_op: Some(ShipModifierOp::Add),
        expect_err_contains: None,
    },
    DecodeCase {
        label: "unknown_shipsize_attribute",
        classes: &["corvette"],
        key: "shipsize_corvette_unknown_add",
        expect_ok: false,
        expect_shipsize_class: None,
        expect_family_tag: None,
        expect_attribute: None,
        expect_op: None,
        expect_err_contains: Some("unknown shipsize attribute"),
    },
    DecodeCase {
        label: "unknown_ship_key",
        classes: &["corvette"],
        key: "fleet_weapon_damage_mult",
        expect_ok: false,
        expect_shipsize_class: None,
        expect_family_tag: None,
        expect_attribute: None,
        expect_op: None,
        expect_err_contains: Some("unknown ship modifier key"),
    },
    DecodeCase {
        label: "cost_context_restricted",
        classes: &["corvette"],
        key: "country_naval_cap_cost_add",
        expect_ok: false,
        expect_shipsize_class: None,
        expect_family_tag: None,
        expect_attribute: None,
        expect_op: None,
        expect_err_contains: Some("discrete ResourceEconomySpec context"),
    },
];

#[test]
fn tp_shipsize_decoder_0_table() {
    for case in DECODE_CASES {
        let classes: Vec<String> = case.classes.iter().map(|s| (*s).into()).collect();
        let result = decode_ship_modifier_key(case.key, &classes);
        if case.expect_ok {
            let decoded = result.unwrap_or_else(|err| panic!("{}: decode failed: {err}", case.label));
            if let Some(tag) = case.expect_family_tag {
                let ok = match (&decoded.family, tag) {
                    (ShipModifierFamily::Shipsize { .. }, "shipsize") => true,
                    (ShipModifierFamily::Ship, "ship") => true,
                    (ShipModifierFamily::Ships, "ships") => true,
                    (ShipModifierFamily::Country, "country") => true,
                    _ => false,
                };
                assert!(ok, "{}: family tag {tag}", case.label);
            }
            if let Some(class) = case.expect_shipsize_class {
                match &decoded.family {
                    ShipModifierFamily::Shipsize { class: got } => {
                        assert_eq!(got.as_str(), class, "{}", case.label);
                    }
                    other => panic!("{}: expected shipsize class {class}, got {other:?}", case.label),
                }
            }
            if let Some(attr) = case.expect_attribute {
                assert_eq!(decoded.attribute.as_str(), attr, "{}", case.label);
            }
            if let Some(op) = case.expect_op {
                assert_eq!(decoded.op, op, "{}", case.label);
            }
        } else {
            let err = result.expect_err(case.label);
            let text = err.to_string();
            if let Some(needle) = case.expect_err_contains {
                assert!(
                    text.contains(needle),
                    "{}: expected `{needle}` in `{text}`",
                    case.label
                );
            }
        }
    }

    let document = parse_raw_document(FIXTURE.as_bytes()).expect("parse fixture");
    let pack = hydrate_shipsize_decoder_pack(&document).expect("hydrate fixture");

    assert!(
        pack.decoded_keys.len() >= 7,
        "expected representative decoded keys"
    );
    assert!(
        pack
            .decoded_keys
            .iter()
            .any(|key| matches!(&key.family, ShipModifierFamily::Shipsize { class } if class == "corvette")),
        "shipsize class lowering"
    );
    assert!(
        pack
            .decoded_keys
            .iter()
            .any(|key| key.attribute == "weapon_damage"),
        "ship_weapon_damage path"
    );
    assert!(
        pack
            .decoded_keys
            .iter()
            .any(|key| key.attribute == "naval_cap"),
        "country_naval_cap path"
    );

    assert_eq!(pack.ship_class_custom_kinds.get("corvette").map(String::as_str), Some("ship_hull_corvette"));
    assert!(
        pack
            .game_mode
            .capability_trees
            .iter()
            .any(|tree| tree.tree_kind == "ship_hull_corvette"),
        "Custom(...) capability-tree shape"
    );

    let add_overlay = pack
        .game_mode
        .overlays
        .iter()
        .find(|overlay| overlay.id.contains("shipsize_corvette_hull_add"))
        .expect("hull add overlay");
    assert!(matches!(add_overlay.sub_field_deltas[0].1, TransformOp::Add(100.0)));
    assert_eq!(
        add_overlay.install,
        simthing_spec::InstallTargetSpec::AllOfKind {
            kind: "ship_hull_corvette".into()
        }
    );

    let mult_overlay = pack
        .game_mode
        .overlays
        .iter()
        .find(|overlay| overlay.id.contains("ship_weapon_damage_mult") && !overlay.id.contains("value"))
        .expect("weapon damage mult overlay");
    assert!(matches!(
        mult_overlay.sub_field_deltas[0].1,
        TransformOp::Multiply(v) if (v - 1.10).abs() < 1e-6
    ));

    let triggered = pack
        .game_mode
        .overlays
        .iter()
        .find(|overlay| overlay.id == "war_damage_boost")
        .expect("triggered modifier overlay");
    assert!(matches!(
        triggered.lifecycle,
        OverlayLifecycle::Suspended { .. }
    ));
    assert_eq!(pack.game_mode.events.len(), 1);

    let complex = pack
        .game_mode
        .resource_flow
        .as_ref()
        .expect("complex trigger gated rates")
        .gated_rates
        .iter()
        .find(|rate| rate.id == "war_fire_rate")
        .expect("complex_trigger_modifier gated rate");
    assert!(complex.trigger.is_some());
    assert!(complex.rate_formula.is_some());

    for count in &pack.eml_node_counts {
        assert!(*count <= MAX_SHIP_EML_NODES, "EvalEML <=32 nodes");
    }

    let formula = pack
        .game_mode
        .resource_flow
        .as_ref()
        .unwrap()
        .gated_rates
        .iter()
        .find_map(|rate| rate.rate_formula.clone())
        .expect("value formula");
    let nodes = compile_value_formula_eml(&formula, false);
    assert!(nodes.len() <= MAX_SHIP_EML_NODES);
    let property_col = nodes
        .iter()
        .find(|node| node.opcode == eml_nodes::opcode::SLOT_VALUE)
        .map(|node| node.a as usize)
        .unwrap_or(1);
    let mut values = vec![0.0f32; property_col + 1];
    values[property_col] = 1.0;
    let oracle = eval_eml_cpu_inline(&nodes, &values);
    let manual = eval_value_formula_cpu(&formula, values[property_col]);
    assert_eq!(oracle.to_bits(), manual.to_bits(), "CPU oracle bit-exact");

    let bad_complex = r#"
simthing_tp_shipsize_decoder_bad = {
    display_name = "bad complex trigger"
    ship_class_map = { corvette = { custom_kind = "ship_hull_corvette" } }
    ship_property = { id = "tp_hull" namespace = "tp" name = "hull" display_name = "Hull" }
    complex_trigger_modifier = {
        id = bad_trigger
        trigger = { leader_has_trait = yes }
        ship_fire_rate_mult = 0.1
    }
}
"#;
    let bad_doc = parse_raw_document(bad_complex.as_bytes()).expect("parse bad complex");
    let err =
        hydrate_shipsize_decoder_pack(&bad_doc).expect_err("non-column-backed complex trigger must hard-error");
    assert!(err.to_string().contains("non-column-backed"));
}

#[test]
#[ignore = "lab-only modifiers.log subset; requires CLAUSER_LAB_DIR"]
fn tp_shipsize_decoder_0_lab_modifiers_log_subset() {
    let lab_dir = std::env::var("CLAUSER_LAB_DIR").expect("CLAUSER_LAB_DIR");
    let path = std::path::Path::new(&lab_dir).join("modifiers.log");
    assert!(path.is_file(), "modifiers.log missing under CLAUSER_LAB_DIR");
    let _ = path;
}

fn eval_value_formula_cpu(formula: &simthing_spec::RateFormulaSpec, property_value: f32) -> f32 {
    let mut acc = formula.base;
    for op in &formula.ops {
        let operand = match &op.operand {
            RateFormulaOperandSpec::Literal(v) => *v,
            RateFormulaOperandSpec::Property(_) => property_value,
        };
        acc = match op.op {
            simthing_spec::RateFormulaOp::Add => acc + operand,
            simthing_spec::RateFormulaOp::Mult => acc * operand,
            simthing_spec::RateFormulaOp::FloorAt => acc.max(operand),
            simthing_spec::RateFormulaOp::CeilAt => acc.min(operand),
        };
    }
    acc
}

fn eval_eml_cpu_inline(nodes: &[simthing_core::EmlNodeGpu], values: &[f32]) -> f32 {
    let mut stack = [0.0f32; 32];
    let mut sp = 0usize;
    for node in nodes {
        match node.opcode {
            eml_nodes::opcode::LITERAL_F32 => {
                stack[sp] = f32::from_bits(node.a);
                sp += 1;
            }
            eml_nodes::opcode::SLOT_VALUE => {
                let i = (node.a) as usize;
                stack[sp] = values.get(i).copied().unwrap_or(0.0);
                sp += 1;
            }
            eml_nodes::opcode::ADD => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs + rhs;
                sp -= 1;
            }
            eml_nodes::opcode::MUL => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs * rhs;
                sp -= 1;
            }
            eml_nodes::opcode::MAX => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs.max(rhs);
                sp -= 1;
            }
            eml_nodes::opcode::MIN => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs.min(rhs);
                sp -= 1;
            }
            eml_nodes::opcode::RETURN_TOP => return stack[sp - 1],
            _ => {}
        }
    }
    stack[0]
}