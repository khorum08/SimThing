//! KERNEL-DEP-BUDGET-0 — build-time gate on `simthing-kernel` direct dependencies.

#[cfg(test)]
mod tests {
    const RUNTIME_ALLOWLIST: &[&str] =
        &["simthing-core", "bytemuck", "pollster", "thiserror", "wgpu"];

    fn dependency_section<'a>(manifest: &'a str, section: &str) -> &'a str {
        let header = format!("[{section}]");
        let after = manifest
            .split(&header)
            .nth(1)
            .unwrap_or_else(|| panic!("Cargo.toml missing {header}"));
        after.split('[').next().unwrap_or(after).trim()
    }

    fn direct_dependency_names(section_body: &str) -> Vec<String> {
        section_body
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    return None;
                }
                let name = line.split('=').next()?.trim();
                if name.is_empty() {
                    return None;
                }
                Some(name.to_string())
            })
            .collect()
    }

    fn assert_dependency_budget(manifest: &str) -> Result<(), String> {
        let runtime_deps = direct_dependency_names(&dependency_section(manifest, "dependencies"));
        for dep in &runtime_deps {
            if !RUNTIME_ALLOWLIST.contains(&dep.as_str()) {
                return Err(format!(
                    "unallowlisted simthing-kernel runtime dependency `{dep}`"
                ));
            }
        }
        for allowed in RUNTIME_ALLOWLIST {
            if !runtime_deps.iter().any(|dep| dep == *allowed) {
                return Err(format!(
                    "allowlist entry `{allowed}` missing from simthing-kernel [dependencies]"
                ));
            }
        }
        Ok(())
    }

}
