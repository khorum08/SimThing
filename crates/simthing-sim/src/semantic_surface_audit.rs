//! AS-SIM-SEMANTIC-FREE-0A closure evidence — public crate surface must not name semantic kinds.

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    const PUBLIC_MODULES: &[&str] = &[
        "accumulator_plan_tick.rs",
        "boundary.rs",
        "delta_log.rs",
        "fission.rs",
        "fission_clone_source_view.rs",
        "gpu_sync.rs",
        "legacy_oracle.rs",
        "mapping_atlas_scheduler.rs",
        "mapping_plan_tick.rs",
        "observability.rs",
        "overlay_lifecycle.rs",
        "property_expiry.rs",
        "reduced_field.rs",
        "replay.rs",
        "sim_runtime_tree.rs",
        "threshold_registry.rs",
        "tree_mutation.rs",
        "lib.rs",
    ];

    const FORBIDDEN: &[&str] = &["SimThingKind", "SimThingKindTag", "kind_tag_to_kind"];

    fn sim_src_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
    }

    fn strip_cfg_test_modules(source: &str) -> String {
        let marker = "#[cfg(test)]";
        source
            .split_once(marker)
            .map(|(prefix, _)| prefix)
            .unwrap_or(source)
            .to_string()
    }

    fn strip_comments(source: &str) -> String {
        let mut out = String::with_capacity(source.len());
        let mut chars = source.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '/' if matches!(chars.peek(), Some('/')) => {
                    chars.next();
                    while matches!(chars.peek(), Some(ch) if *ch != '\n') {
                        chars.next();
                    }
                }
                '/' if matches!(chars.peek(), Some('*')) => {
                    chars.next();
                    while let Some(ch) = chars.next() {
                        if ch == '*' && matches!(chars.peek(), Some('/')) {
                            chars.next();
                            break;
                        }
                    }
                }
                other => out.push(other),
            }
        }
        out
    }

    fn production_text(source: &str) -> String {
        strip_comments(&strip_cfg_test_modules(source))
    }

    #[test]
    fn as_sim_semantic_free_public_surface_audit() {
        let src_dir = sim_src_dir();
        let mut violations = Vec::new();

        for rel in PUBLIC_MODULES {
            if *rel == "semantic_surface_audit.rs" || *rel == "kind_production_audit.rs" {
                continue;
            }
            let path = src_dir.join(rel);
            let source = std::fs::read_to_string(&path).expect("read public module source");
            let production = production_text(&source);
            for (line_no, line) in production.lines().enumerate() {
                let line_no = line_no + 1;
                for token in FORBIDDEN {
                    if line.contains(token) {
                        violations.push(format!(
                            "{rel}:{line_no}: public surface contains `{token}`"
                        ));
                    }
                }
            }
        }

        assert!(
            violations.is_empty(),
            "simthing-sim public surface must not name SimThing kinds:\n{}",
            violations.join("\n")
        );
    }
}
