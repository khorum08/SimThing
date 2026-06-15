//! Declarative nebula field-operator feedstock emission (PR9).
//!
//! Emits only the closed MapGen neutral-AST `nebula = { name radius }` surface accepted by
//! `mapgen_lattice`. Does not emit scenario-container `field_operator` blocks or invent grammar.

use crate::nebula::NebulaField;

/// Keys admitted by the closed MapGen `static_galaxy_scenario` nebula block reader.
pub const ACCEPTED_NEBULA_KEYS: &[&str] = &["name", "radius"];

const FORBIDDEN_FIELD_SURFACE_TERMS: &[&str] = &[
    "field_operator",
    " route",
    " path",
    "predecessor",
    "movement",
    "border",
    "frontline",
    "runtime_field",
    "gpu_operator",
    "semantic_wgsl",
    "palma_feedstock",
    "commitment",
];

/// Emit closed-surface nebula declarations into a `static_galaxy_scenario` block body.
pub fn emit_nebula_declarations(out: &mut String, nebulas: &[NebulaField]) {
    for nebula in nebulas {
        out.push_str("        nebula = {\n");
        write_line(
            out,
            &format!(
                "            name = \"{}\"",
                escape_clause_string(&nebula.name)
            ),
        );
        write_line(
            out,
            &format!("            radius = {}", nebula.radius_cells),
        );
        out.push_str("        }\n\n");
    }
}

/// Return the first forbidden route/path/movement/etc. term found in emitted text, if any.
pub fn forbidden_field_surface_term(text: &str) -> Option<&'static str> {
    let lower = text.to_ascii_lowercase();
    FORBIDDEN_FIELD_SURFACE_TERMS
        .iter()
        .find(|term| lower.contains(&term.to_ascii_lowercase()))
        .copied()
}

fn escape_clause_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn write_line(out: &mut String, line: &str) {
    out.push_str(line);
    out.push('\n');
}
