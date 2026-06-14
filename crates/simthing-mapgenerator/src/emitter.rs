//! Deterministic declarative scenario text emitter (PR4 — no topology, no lowering).
//!
//! Emits `scenario { metadata … lattice … location … }` text compatible with the closed 0.0.8.2.5
//! MapGen target grammar. Positions are inert integer lattice metadata only.

use thiserror::Error;

use crate::lattice::SquareLattice;
use crate::params::{GenerationMode, MapGeneratorParams};
use crate::strategy::{PlacedSystemSeed, ShapePlacement};

pub const DEFAULT_INITIALIZER_REF: &str = "example_rim_initializer";
pub const DEFAULT_SCENARIO_NAME: &str = "generated_mapgen";

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
    pub scenario_name: String,
    pub default_initializer_ref: String,
}

impl Default for ScenarioEmitterConfig {
    fn default() -> Self {
        Self {
            scenario_name: DEFAULT_SCENARIO_NAME.into(),
            default_initializer_ref: DEFAULT_INITIALIZER_REF.into(),
        }
    }
}

impl ScenarioEmitterConfig {
    pub fn from_params(params: &MapGeneratorParams) -> Self {
        Self {
            scenario_name: format!("generated_{}", sanitize_identifier(&params.shape.shape)),
            default_initializer_ref: DEFAULT_INITIALIZER_REF.into(),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ScenarioEmitError {
    #[error("placement contains no systems")]
    EmptyPlacement,
    #[error("duplicate location name '{name}'")]
    DuplicateLocationName { name: String },
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
        lattice: &SquareLattice,
        placement: &ShapePlacement,
    ) -> Result<ScenarioText, ScenarioEmitError> {
        if placement.systems.is_empty() {
            return Err(ScenarioEmitError::EmptyPlacement);
        }

        let mut names = Vec::with_capacity(placement.systems.len());
        for system in &placement.systems {
            let name = location_name(system);
            if names.contains(&name) {
                return Err(ScenarioEmitError::DuplicateLocationName { name });
            }
            names.push(name);
        }

        let mut out = String::new();
        write_line(
            &mut out,
            &format!("scenario = {} {{", self.config.scenario_name),
        );
        write_metadata_block(&mut out, params)?;
        write_lattice_block(&mut out, lattice)?;
        for system in &placement.systems {
            write_location_block(&mut out, system, &self.config.default_initializer_ref)?;
        }
        out.push_str("}\n");
        Ok(ScenarioText(out))
    }
}

fn write_metadata_block(
    out: &mut String,
    params: &MapGeneratorParams,
) -> Result<(), ScenarioEmitError> {
    out.push_str("    metadata = {\n");
    write_line(out, "        generated_by = \"MapGeneratorCLI\"");
    write_line(
        out,
        &format!(
            "        shape = \"{}\"",
            escape_clause_string(&params.shape.shape)
        ),
    );
    write_line(out, &format!("        seed = {}", params.seed));
    write_line(
        out,
        &format!(
            "        mode = \"{}\"",
            escape_clause_string(match params.mode {
                GenerationMode::Procedural => "procedural",
                GenerationMode::ArbitraryStatic => "arbitrary_static",
            })
        ),
    );
    out.push_str("    }\n\n");
    Ok(())
}

fn write_lattice_block(out: &mut String, lattice: &SquareLattice) -> Result<(), ScenarioEmitError> {
    let edge = lattice.edge();
    write_line(
        out,
        &format!("    lattice = {{ width = {edge} height = {edge} }}"),
    );
    out.push('\n');
    Ok(())
}

fn write_location_block(
    out: &mut String,
    system: &PlacedSystemSeed,
    default_initializer_ref: &str,
) -> Result<(), ScenarioEmitError> {
    let name = location_name(system);
    let initializer = system
        .bucket
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or(default_initializer_ref);
    write_line(out, &format!("    location = {name} {{"));
    write_line(
        out,
        &format!(
            "        initializer = \"{}\"",
            escape_clause_string(initializer)
        ),
    );
    write_line(
        out,
        &format!(
            "        position = {{ x = {} y = {} }}",
            system.coord.col, system.coord.row
        ),
    );
    out.push_str("    }\n\n");
    Ok(())
}

fn location_name(system: &PlacedSystemSeed) -> String {
    format!("system_{:06}", system.id)
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
