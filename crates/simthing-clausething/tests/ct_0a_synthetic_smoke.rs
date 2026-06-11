//! CT-0a safe synthetic ClauseScript smoke test.
//!
//! Fixture is original SimThing-authored text — no Paradox / Stellaris / Clausewitz corpora.

use simthing_clausething::jomini::{TextTape, TextToken};

const FIXTURE: &[u8] = br#"simthing_demo_entity = {
    category = simthing_demo
    enabled = yes
    potency = 42
    tags = { alpha beta }
}"#;

#[test]
fn synthetic_clausescript_text_path_parses() {
    let tape = TextTape::from_slice(FIXTURE).expect("text tape parse");
    let tokens = tape.tokens();
    assert!(!tokens.is_empty());

    let mut saw_key = false;
    let mut saw_object = false;
    for token in tokens {
        match token {
            TextToken::Unquoted(scalar) if scalar.as_bytes() == b"simthing_demo_entity" => {
                saw_key = true
            }
            TextToken::Object { .. } => saw_object = true,
            _ => {}
        }
    }
    assert!(saw_key, "expected root entity key token");
    assert!(saw_object, "expected object container token");

    let reader = tape.windows1252_reader();
    let mut fields = reader.fields();
    let (key, _op, _value) = fields.next().expect("at least one field");
    assert_eq!(key.read_str(), "simthing_demo_entity");
}
