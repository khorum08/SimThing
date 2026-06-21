use std::{
    fs,
    path::{Path, PathBuf},
};

const FORBIDDEN: &[&str] = &[
    "ScenarioSpec",
    "Accumulator",
    "ResourceFlow",
    "STEAD",
    "owner",
    "faction",
    "diplomacy",
    "planet",
    "fleet",
    "economy",
    "combat",
    "route",
    "region",
    "border",
    "empire",
    "map_view",
    "star_system",
];

const SCAN_ROOT: &str = "src";
const SHADER_PATH: &str = "src/shaders/text_instanced.wgsl";

#[test]
fn shader_and_src_are_semantic_free() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut violations = Vec::new();

    scan_dir(&crate_root.join(SCAN_ROOT), &mut violations);
    scan_file(crate_root.join(SHADER_PATH), &mut violations);

    assert!(
        violations.is_empty(),
        "semantic-free guard violations:\n{}",
        violations.join("\n")
    );
}

fn scan_dir(dir: &Path, violations: &mut Vec<String>) {
    let entries = fs::read_dir(dir).expect("read scan dir");
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan_dir(&path, violations);
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            scan_file(path, violations);
        }
    }
}

fn scan_file(path: PathBuf, violations: &mut Vec<String>) {
    let content = fs::read_to_string(&path).expect("read file");
    for token in FORBIDDEN {
        for (idx, line) in content.lines().enumerate() {
            if line_contains_forbidden_token(line, token) {
                violations.push(format!(
                    "{}:{}: forbidden token `{token}`",
                    path.display(),
                    idx + 1
                ));
            }
        }
    }
}

fn line_contains_forbidden_token(line: &str, token: &str) -> bool {
    if token == "region"
        && (line.contains("dirty_region")
            || line.contains("dirty-region")
            || line.contains("dirty_rect"))
    {
        return false;
    }
    line.split(|c: char| !c.is_alphanumeric() && c != '_')
        .any(|word| word == token)
}
