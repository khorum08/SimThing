//! Deterministic declarative scenario text emitter (PR4 — no topology, no lowering).
//!
//! Emits a single-root `static_galaxy_scenario` neutral-AST block compatible with the closed
//! 0.0.8.2.5 `mapgen_lattice` reader. Positions are inert integer lattice metadata only.

use std::collections::BTreeSet;

use thiserror::Error;

use crate::lattice::SquareLattice;
use crate::params::MapGeneratorParams;
use crate::strategy::{PlacedSystemSeed, ShapePlacement};

pub const DEFAULT_INITIALIZER_REF: &str = "example_rim_initializer";
pub const DEFAULT_SCENARIO_ID: &str = "generated_mapgen";
pub const DEFAULT_INITIALIZER_DISPLAY_NAME: &str = "Initializer Payload";

/// Byte-stable emitted scenario text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioText(pub String);

impl ScenarioText {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

/// Emitter configuration (deterministic naming defaults).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioEmitterConfig {
    pub scenario_id: String,
    pub scenario_display_name: String,
    pub default_initializer_ref: String,
}

impl Default for ScenarioEmitterConfig {
    fn default() -> Self {
        Self {
            scenario_id: DEFAULT_SCENARIO_ID.into(),
            scenario_display_name: "MapGeneratorCLI generated map".into(),
            default_initializer_ref: DEFAULT_INITIALIZER_REF.into(),
        }
    }
}

impl ScenarioEmitterConfig {
    pub fn from_params(params: &MapGeneratorParams) -> Self {
        let shape = sanitize_identifier(&params.shape.shape);
        Self {
            scenario_id: format!("generated_{shape}"),
            scenario_display_name: format!("MapGeneratorCLI {shape} seed {}", params.seed),
            default_initializer_ref: DEFAULT_INITIALIZER_REF.into(),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ScenarioEmitError {
    #[error("placement contains no systems")]
    EmptyPlacement,
    #[error("duplicate system id '{id}'")]
    DuplicateSystemId { id: String },
    #[error("invalid initializer bareword '{name}'")]
    InvalidInitializerBareword { name: String },
}

/// Deterministic scenario text emitter (producer-side only).
#[derive(Debug, Clone, Default)]
pub struct ScenarioEmitter {
    config: ScenarioEmitterConfig,
}

impl ScenarioEmitter {
    pub fn new(config: ScenarioEmitterConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::default()
    }

    pub fn config(&self) -> &ScenarioEmitterConfig {
        &self.config
    }

    pub fn emit(
        &self,
        params: &MapGeneratorParams,
        _lattice: &SquareLattice,
        placement: &ShapePlacement,
    ) -> Result<ScenarioText, ScenarioEmitError> {
        let _ = params;
        if placement.systems.is_empty() {
            return Err(ScenarioEmitError::EmptyPlacement);
        }

        let mut seen_ids = BTreeSet::new();
        let mut initializer_refs = BTreeSet::new();
        for system in &placement.systems {
            let id = system_id_scalar(system);
            if !seen_ids.insert(id.clone()) {
                return Err(ScenarioEmitError::DuplicateSystemId { id });
            }
            let initializer = initializer_bareword(system, &self.config.default_initializer_ref)?;
            initializer_refs.insert(initializer);
        }

        let mut out = String::new();
        write_line(&mut out, &format!("{} = {{", self.config.scenario_id));
        write_static_galaxy_block(&mut out, &self.config, placement)?;
        for initializer in initializer_refs {
            write_initializer_definition(&mut out, &initializer)?;
        }
        out.push_str("}\n");
        Ok(ScenarioText(out))
    }
}

fn write_static_galaxy_block(
    out: &mut String,
    config: &ScenarioEmitterConfig,
    placement: &ShapePlacement,
) -> Result<(), ScenarioEmitError> {
    out.push_str("    static_galaxy_scenario = {\n");
    write_line(
        out,
        &format!(
            "        name = \"{}\"",
            escape_clause_string(&config.scenario_display_name)
        ),
    );
    write_line(out, "        random_hyperlanes = no");
    out.push('\n');
    for system in &placement.systems {
        write_system_block(out, system, &config.default_initializer_ref)?;
        out.push('\n');
    }
    out.push_str("    }\n\n");
    Ok(())
}

fn write_system_block(
    out: &mut String,
    system: &PlacedSystemSeed,
    default_initializer_ref: &str,
) -> Result<(), ScenarioEmitError> {
    let initializer = initializer_bareword(system, default_initializer_ref)?;
    out.push_str("        system = {\n");
    write_line(
        out,
        &format!("            id = \"{}\"", system_id_scalar(system)),
    );
    write_line(out, "            name = \"\"");
    write_line(
        out,
        &format!(
            "            position = {{ x = {} y = {} z = 0 }}",
            system.coord.col, system.coord.row
        ),
    );
    write_line(out, &format!("            initializer = {initializer}"));
    out.push_str("        }\n");
    Ok(())
}

fn write_initializer_definition(
    out: &mut String,
    initializer: &str,
) -> Result<(), ScenarioEmitError> {
    validate_initializer_bareword(initializer)?;
    write_line(out, &format!("    {initializer} = {{"));
    write_line(
        out,
        &format!(
            "        name = \"{}\"",
            escape_clause_string(DEFAULT_INITIALIZER_DISPLAY_NAME)
        ),
    );
    write_line(out, "        planet = { count = 1 }");
    out.push_str("    }\n\n");
    Ok(())
}

fn initializer_bareword(
    system: &PlacedSystemSeed,
    default_initializer_ref: &str,
) -> Result<String, ScenarioEmitError> {
    let name = system
        .bucket
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or(default_initializer_ref);
    validate_initializer_bareword(name)?;
    Ok(name.to_string())
}

fn validate_initializer_bareword(name: &str) -> Result<(), ScenarioEmitError> {
    let valid = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
        && !name.starts_with('.');
    if valid {
        Ok(())
    } else {
        Err(ScenarioEmitError::InvalidInitializerBareword { name: name.into() })
    }
}

fn system_id_scalar(system: &PlacedSystemSeed) -> String {
    system.id.to_string()
}

fn sanitize_identifier(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        "mapgen".into()
    } else {
        out
    }
}

fn escape_clause_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn write_line(out: &mut String, line: &str) {
    out.push_str(line);
    out.push('\n');
}
