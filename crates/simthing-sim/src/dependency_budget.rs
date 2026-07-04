//! KERNEL-DEP-BUDGET-0 — build-time gate on `simthing-sim` direct dependencies.
//!
//! New runtime or dev dependencies require DA sign-off and an allowlist update here.

#[cfg(test)]
mod tests {
    /// Direct `[dependencies]` allowlisted at KERNEL-DEP-BUDGET-0 (DA sign-off required to extend).
    const RUNTIME_ALLOWLIST: &[&str] = &[
        "simthing-core",
        "simthing-kernel",
        "simthing-gpu",
        "simthing-feeder",
        "bytemuck",
        "thiserror",
        "serde",
        "serde_json",
    ];

    /// Direct `[dev-dependencies]` allowlisted at KERNEL-DEP-BUDGET-0.
    const DEV_ALLOWLIST: &[&str] = &[];

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
                    "unallowlisted simthing-sim runtime dependency `{dep}` — update KERNEL-DEP-BUDGET-0 allowlist after DA sign-off"
                ));
            }
        }
        for allowed in RUNTIME_ALLOWLIST {
            if !runtime_deps.iter().any(|dep| dep == *allowed) {
                return Err(format!(
                    "allowlist entry `{allowed}` missing from simthing-sim [dependencies]"
                ));
            }
        }

        let dev_deps = if manifest.contains("[dev-dependencies]") {
            direct_dependency_names(&dependency_section(manifest, "dev-dependencies"))
        } else {
            Vec::new()
        };
        for dep in &dev_deps {
            if !DEV_ALLOWLIST.contains(&dep.as_str()) {
                return Err(format!(
                    "unallowlisted simthing-sim dev dependency `{dep}` — update KERNEL-DEP-BUDGET-0 allowlist after DA sign-off"
                ));
            }
        }
        for allowed in DEV_ALLOWLIST {
            if !dev_deps.iter().any(|dep| dep == *allowed) {
                return Err(format!(
                    "allowlist entry `{allowed}` missing from simthing-sim [dev-dependencies]"
                ));
            }
        }

        Ok(())
    }

}
