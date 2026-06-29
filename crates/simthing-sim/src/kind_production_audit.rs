//! AS-KIND-OUT-OF-TICK-0E closure evidence — production source must not branch on SimThing kind.
//!
//! This module exists only under `#[cfg(test)]`; it is not part of the runtime surface.

#[cfg(test)]
pub mod tests {
    use std::path::{Path, PathBuf};

    const FORBIDDEN: &[&str] = &["SimThingKind", "SimThingKindTag", "kind_tag_to_kind"];

    fn sim_src_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
    }

    /// Strip `#[cfg(test)]` modules and line/block comments so production text can be scanned.
    fn production_text(source: &str) -> String {
        let without_tests = strip_cfg_test_modules(source);
        strip_comments(&without_tests)
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

    fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
        if !dir.is_dir() {
            return;
        }
        for entry in std::fs::read_dir(dir).expect("read simthing-sim src") {
            let entry = entry.expect("dir entry");
            let path = entry.path();
            if path.is_dir() {
                collect_rs_files(&path, out);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                out.push(path);
            }
        }
    }

    fn relative_src_path(path: &Path) -> String {
        path.strip_prefix(sim_src_dir())
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    fn simthing_kind_field_access(line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed.starts_with("let kind = value.get(\"kind\")") {
            return false;
        }
        trimmed.contains(".kind")
            && !trimmed.contains("child_kind")
            && !trimmed.contains("event_kind")
    }

    #[test]
    pub fn as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads() {
        let src_dir = sim_src_dir();
        let mut files = Vec::new();
        collect_rs_files(&src_dir, &mut files);

        let mut violations = Vec::new();
        for path in files {
            let rel = relative_src_path(&path);
            if rel == "kind_production_audit.rs" {
                continue;
            }
            let source = std::fs::read_to_string(&path).expect("read source");
            let production = production_text(&source);
            for (line_no, line) in production.lines().enumerate() {
                let line_no = line_no + 1;
                for token in FORBIDDEN {
                    if line.contains(token) {
                        violations.push(format!("{rel}:{line_no}: contains `{token}`"));
                    }
                }
                if simthing_kind_field_access(line) {
                    violations.push(format!("{rel}:{line_no}: SimThing `.kind` field access"));
                }
            }
        }

        assert!(
            violations.is_empty(),
            "production simthing-sim must not read SimThing kind at runtime:\n{}",
            violations.join("\n")
        );
    }
}
