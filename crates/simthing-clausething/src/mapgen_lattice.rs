//! MapGen PR3 — gridcell lattice hierarchy generator (M2/M5).
//!
//! Lowers a tiny neutral-AST MapGen fixture into scenario-container-compatible hierarchy with
//! ordinary `SimThingKind::Location` gridcells (mapping-role metadata only), inert render positions,
//! and bounded fixture-local lattice placement. No RF, Movement-Front, PALMA, FIELD_POLICY, links,
//! or runtime surfaces.

use std::collections::{BTreeMap, BTreeSet};

use simthing_core::SimThingKind;
use simthing_spec::PropertySpec;

use crate::error::HydrateError;
use crate::hydrate_scenario::{
    HydratedScenarioGridMetadata, HydratedScenarioGridPlacement, HydratedScenarioNode,
    HydratedScenarioPack, PR3_MAX_LINK_FANOUT, hydrate_scenario,
};
use crate::mapgen_neutral_ast::MapGenNeutralDocument;
use crate::parse::parse_raw_document;
use crate::raw::{RawBlock, RawProperty, RawValue};

/// Canonical medium galaxy lattice edge (square). Fixture-local subsets document this authority.
pub const MAPGEN_CANONICAL_LATTICE_EDGE: u32 = 200;

/// Maximum admitted fixture-local lattice edge for PR3 generator options.
pub const MAPGEN_MAX_LATTICE_EDGE: u32 = 256;

/// Default fixture-local lattice edge for the tiny pentad slice (3×3 active subset).
pub const MAPGEN_DEFAULT_FIXTURE_LATTICE_EDGE: u32 = 3;

const FORBIDDEN_GENERATED_PROPERTY_NAMES: &[&str] = &[
    "route",
    "path",
    "pathfinding",
    "predecessor",
    "movement",
    "movement_order",
    "border",
    "frontline",
    "cpu_planner",
    "fleet_path",
];

/// Fixture-local square lattice options for MapGen PR3 hierarchy generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapGenLatticeOptions {
    pub fixture_lattice_edge: u32,
}

impl Default for MapGenLatticeOptions {
    fn default() -> Self {
        Self {
            fixture_lattice_edge: MAPGEN_DEFAULT_FIXTURE_LATTICE_EDGE,
        }
    }
}

/// Scenario-container-compatible MapGen hierarchy output.
#[derive(Debug, Clone)]
pub struct MapGenLatticeHierarchy {
    pub pack: HydratedScenarioPack,
    pub canonical_lattice_edge: u32,
    pub fixture_lattice_edge: u32,
}

/// MapGen PR3 hierarchy generation failure.
#[derive(Debug)]
pub struct MapGenLatticeError {
    pub message: String,
}

impl MapGenLatticeError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for MapGenLatticeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MapGen lattice error: {}", self.message)
    }
}

impl std::error::Error for MapGenLatticeError {}

impl From<HydrateError> for MapGenLatticeError {
    fn from(err: HydrateError) -> Self {
        Self::new(err.to_string())
    }
}

impl From<crate::error::ParseError> for MapGenLatticeError {
    fn from(err: crate::error::ParseError) -> Self {
        Self::new(err.to_string())
    }
}

/// Validate a fixture-local square lattice edge.
pub fn validate_fixture_lattice_edge(edge: u32) -> Result<u32, MapGenLatticeError> {
    if edge == 0 {
        return Err(MapGenLatticeError::new(
            "fixture lattice edge must be positive",
        ));
    }
    if edge > MAPGEN_MAX_LATTICE_EDGE {
        return Err(MapGenLatticeError::new(format!(
            "fixture lattice edge {edge} exceeds PR3 cap {MAPGEN_MAX_LATTICE_EDGE}"
        )));
    }
    Ok(edge)
}

/// Generate a scenario-container-compatible gridcell hierarchy from a neutral MapGen document.
pub fn generate_mapgen_lattice_hierarchy(
    document: &MapGenNeutralDocument,
    options: MapGenLatticeOptions,
) -> Result<MapGenLatticeHierarchy, MapGenLatticeError> {
    let fixture_lattice_edge = validate_fixture_lattice_edge(options.fixture_lattice_edge)?;
    let slice = extract_mapgen_slice(document)?;
    let placements = assign_system_placements(&slice.systems, fixture_lattice_edge)?;
    let scenario_clause = build_scenario_clause(&slice, &placements)?;
    let raw = parse_raw_document(scenario_clause.as_bytes())?;
    let mut pack = hydrate_scenario(&raw)?;
    pack.grid_metadata = build_mapgen_grid_metadata(&placements, fixture_lattice_edge);
    assert_no_deferred_surfaces(&pack)?;
    assert_no_forbidden_generated_properties(&pack)?;
    assert_allowed_simthing_kinds(&pack.root_node)?;
    Ok(MapGenLatticeHierarchy {
        pack,
        canonical_lattice_edge: MAPGEN_CANONICAL_LATTICE_EDGE,
        fixture_lattice_edge,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExtractedSystem {
    id: String,
    display_name: String,
    render_x: String,
    render_y: String,
    render_z: String,
    initializer: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExtractedInitializer {
    id: String,
    display_name: String,
    planet_count: String,
    deposit_minerals: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExtractedSlice {
    scenario_id: String,
    scenario_name: String,
    nebula_name: Option<String>,
    nebula_radius: Option<String>,
    systems: Vec<ExtractedSystem>,
    initializers: BTreeMap<String, ExtractedInitializer>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SystemPlacement {
    system_id: String,
    row: u32,
    col: u32,
}

fn extract_mapgen_slice(
    document: &MapGenNeutralDocument,
) -> Result<ExtractedSlice, MapGenLatticeError> {
    let root = root_block(&document.document)?;
    if root.properties.len() != 1 {
        return Err(MapGenLatticeError::new(
            "MapGen fixture expects exactly one top-level slice block",
        ));
    }
    let slice_property = &root.properties[0];
    let scenario_id = slice_property.key.text.clone();
    let RawValue::Block(slice_block) = &slice_property.value else {
        return Err(MapGenLatticeError::new("MapGen slice root must be a block"));
    };

    let static_galaxy = require_block(slice_block, "static_galaxy_scenario")?;
    let scenario_name = optional_scalar(static_galaxy, "name")?
        .unwrap_or_else(|| "Tiny Pentad Hub Slice".to_string());

    let nebula = optional_block(static_galaxy, "nebula")?;
    let nebula_name = match nebula.as_ref() {
        Some(block) => optional_scalar(block, "name")?,
        None => None,
    };
    let nebula_radius = match nebula.as_ref() {
        Some(block) => optional_scalar(block, "radius")?,
        None => None,
    };

    let mut initializers = BTreeMap::new();
    for property in &slice_block.properties {
        if property.key.text.ends_with("_initializer") {
            let RawValue::Block(block) = &property.value else {
                continue;
            };
            let id = property.key.text.clone();
            let planet_count = optional_block(block, "planet")?
                .and_then(|planet| optional_scalar(&planet, "count").ok().flatten())
                .unwrap_or_else(|| "1".to_string());
            let deposit_minerals = optional_block(block, "deposit")?
                .and_then(|deposit| optional_block(&deposit, "resources").ok().flatten())
                .and_then(|resources| optional_scalar(&resources, "minerals").ok().flatten());
            initializers.insert(
                id.clone(),
                ExtractedInitializer {
                    id,
                    display_name: optional_scalar(block, "name")?
                        .unwrap_or_else(|| "Initializer Payload".to_string()),
                    planet_count,
                    deposit_minerals,
                },
            );
        }
    }

    let mut systems = Vec::new();
    for property in property_values_matching(static_galaxy, "system") {
        let RawValue::Block(system) = property else {
            return Err(MapGenLatticeError::new("system entry must be a block"));
        };
        let id = require_scalar(system, "id")?;
        let display_name = optional_scalar(system, "name")?.filter(|name| !name.is_empty());
        let display_name = display_name.unwrap_or_else(|| id.clone());
        let position = optional_block(system, "position")?.ok_or_else(|| {
            MapGenLatticeError::new(format!("system `{id}` requires inert position metadata"))
        })?;
        systems.push(ExtractedSystem {
            id: id.clone(),
            display_name,
            render_x: require_scalar(&position, "x")?,
            render_y: require_scalar(&position, "y")?,
            render_z: optional_scalar(&position, "z")?.unwrap_or_else(|| "0".to_string()),
            initializer: optional_scalar(system, "initializer")?,
        });
    }

    if systems.is_empty() {
        return Err(MapGenLatticeError::new(
            "MapGen slice requires at least one system entry",
        ));
    }

    Ok(ExtractedSlice {
        scenario_id,
        scenario_name,
        nebula_name,
        nebula_radius,
        systems,
        initializers,
    })
}

fn assign_system_placements(
    systems: &[ExtractedSystem],
    fixture_lattice_edge: u32,
) -> Result<Vec<SystemPlacement>, MapGenLatticeError> {
    let capacity = fixture_lattice_edge.saturating_mul(fixture_lattice_edge);
    if systems.len() as u32 > capacity {
        return Err(MapGenLatticeError::new(format!(
            "system count {} exceeds fixture lattice capacity {capacity}",
            systems.len()
        )));
    }

    let mut occupied = BTreeSet::new();
    let mut placements = Vec::with_capacity(systems.len());
    for (index, system) in systems.iter().enumerate() {
        let index = index as u32;
        let row = index / fixture_lattice_edge;
        let col = index % fixture_lattice_edge;
        if !occupied.insert((row, col)) {
            return Err(MapGenLatticeError::new(format!(
                "duplicate gridcell placement at row={row} col={col}"
            )));
        }
        placements.push(SystemPlacement {
            system_id: system.id.clone(),
            row,
            col,
        });
    }
    Ok(placements)
}

fn build_scenario_clause(
    slice: &ExtractedSlice,
    placements: &[SystemPlacement],
) -> Result<String, MapGenLatticeError> {
    let mut out = String::new();
    out.push_str(&format!("scenario = {} {{\n", slice.scenario_id));
    out.push_str("    metadata = {\n");
    out.push_str(&format!(
        "        display_name = \"{}\"\n",
        escape_clause_string(&slice.scenario_name)
    ));
    out.push_str("        description = \"MapGen PR3 gridcell lattice hierarchy; render positions are inert metadata only.\"\n");
    out.push_str("        tags = \"mapgen,pr3,gridcell_lattice\"\n");
    out.push_str(&format!(
        "        mapgen_canonical_lattice_edge = \"{MAPGEN_CANONICAL_LATTICE_EDGE}\"\n"
    ));
    out.push_str(&format!(
        "        mapgen_fixture_lattice_edge = \"{}\"\n",
        MAPGEN_DEFAULT_FIXTURE_LATTICE_EDGE
    ));
    out.push_str("        mapgen_galaxy_root = \"galaxy_map\"\n");
    out.push_str("        mapgen_sector_root = \"pentad_sector\"\n");
    if let Some(name) = &slice.nebula_name {
        out.push_str(&format!(
            "        mapgen_sector_name = \"{}\"\n",
            escape_clause_string(name)
        ));
    }
    if let Some(radius) = &slice.nebula_radius {
        out.push_str(&format!(
            "        mapgen_nebula_radius_authored = \"{}\"\n",
            escape_clause_string(radius)
        ));
    }
    out.push_str("    }\n");

    out.push_str("    location = galaxy_map {\n");
    out.push_str(&format!(
        "        name = \"{}\"\n",
        escape_clause_string(&slice.scenario_name)
    ));
    out.push_str("        properties = {\n");
    push_mapping_role_property(
        &mut out,
        "mapping_role",
        "mapgen_galaxy_mapping_role",
        "galaxy",
        4,
    );
    push_inert_property(
        &mut out,
        "mapgen_canonical_lattice_edge",
        "canonical_lattice_edge",
        &MAPGEN_CANONICAL_LATTICE_EDGE.to_string(),
        4,
    );
    out.push_str("        }\n");
    out.push_str("        children = {\n");
    out.push_str("            child = pentad_sector {\n");
    out.push_str("                kind = Location\n");
    out.push_str("                display_name = \"Pentad Sector\"\n");
    out.push_str("                properties = {\n");
    push_mapping_role_property(
        &mut out,
        "mapping_role",
        "mapgen_sector_mapping_role",
        "sector",
        5,
    );
    if let Some(name) = &slice.nebula_name {
        push_inert_property(
            &mut out,
            "mapgen_sector_nebula_name",
            "sector_nebula_name",
            name,
            5,
        );
    }
    out.push_str("                }\n");
    out.push_str("                children = {\n");

    for (system, placement) in slice.systems.iter().zip(placements) {
        if placement.system_id != system.id {
            return Err(MapGenLatticeError::new(
                "internal placement/system id mismatch",
            ));
        }
        out.push_str(&format!("                    child = {} {{\n", system.id));
        out.push_str("                        kind = Location\n");
        out.push_str(&format!(
            "                        display_name = \"{}\"\n",
            escape_clause_string(&system.display_name)
        ));
        out.push_str("                        properties = {\n");
        push_mapping_role_property(
            &mut out,
            "mapping_role",
            &format!("mapgen_gridcell_mapping_role_{}", system.id),
            "gridcell",
            6,
        );
        push_inert_property(
            &mut out,
            &format!("mapgen_grid_row_{}", system.id),
            "grid_row",
            &placement.row.to_string(),
            6,
        );
        push_inert_property(
            &mut out,
            &format!("mapgen_grid_col_{}", system.id),
            "grid_col",
            &placement.col.to_string(),
            6,
        );
        push_inert_property(
            &mut out,
            &format!("mapgen_render_x_{}", system.id),
            "render_position_x",
            &system.render_x,
            6,
        );
        push_inert_property(
            &mut out,
            &format!("mapgen_render_y_{}", system.id),
            "render_position_y",
            &system.render_y,
            6,
        );
        push_inert_property(
            &mut out,
            &format!("mapgen_render_z_{}", system.id),
            "render_position_z",
            &system.render_z,
            6,
        );
        out.push_str("                        }\n");

        if let Some(initializer_id) = &system.initializer {
            let initializer = slice.initializers.get(initializer_id).ok_or_else(|| {
                MapGenLatticeError::new(format!(
                    "system `{}` references unknown initializer `{initializer_id}`",
                    system.id
                ))
            })?;
            out.push_str("                        children = {\n");
            out.push_str(&format!(
                "                            child = {}_planet {{\n",
                initializer.id
            ));
            out.push_str("                                kind = Cohort\n");
            out.push_str(&format!(
                "                                display_name = \"{} Planet Payload\"\n",
                escape_clause_string(&initializer.display_name)
            ));
            push_inert_property_block(
                &mut out,
                &format!("mapgen_planet_count_{}", system.id),
                "planet_count_authored",
                &initializer.planet_count,
                8,
            );
            out.push_str("                            }\n");
            if let Some(minerals) = &initializer.deposit_minerals {
                out.push_str(&format!(
                    "                            child = {}_deposit {{\n",
                    initializer.id
                ));
                out.push_str("                                kind = Location\n");
                out.push_str(
                    "                                display_name = \"Deposit Payload\"\n",
                );
                push_inert_property_block(
                    &mut out,
                    &format!("mapgen_deposit_minerals_{}", system.id),
                    "deposit_minerals_authored",
                    minerals,
                    8,
                );
                out.push_str("                            }\n");
            }
            out.push_str("                        }\n");
        }

        out.push_str("                    }\n");
    }

    out.push_str("                }\n");
    out.push_str("            }\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n");
    Ok(out)
}

fn build_mapgen_grid_metadata(
    placements: &[SystemPlacement],
    fixture_lattice_edge: u32,
) -> HydratedScenarioGridMetadata {
    HydratedScenarioGridMetadata {
        grid_size: fixture_lattice_edge,
        max_fanout: PR3_MAX_LINK_FANOUT,
        placements: placements
            .iter()
            .map(|placement| HydratedScenarioGridPlacement {
                location_id: placement.system_id.clone(),
                target_id: placement.system_id.clone(),
                row: placement.row,
                col: placement.col,
            })
            .collect(),
        links: Vec::new(),
    }
}

fn assert_no_deferred_surfaces(pack: &HydratedScenarioPack) -> Result<(), MapGenLatticeError> {
    if pack.w_impedance_compose.is_some() || pack.stress_compose.is_some() {
        return Err(MapGenLatticeError::new(
            "PR3 generator must not emit field_operator surfaces",
        ));
    }
    if pack.palma_feedstock.is_some() {
        return Err(MapGenLatticeError::new(
            "PR3 generator must not emit PALMA feedstock",
        ));
    }
    if pack.commitment.is_some() {
        return Err(MapGenLatticeError::new(
            "PR3 generator must not emit FIELD_POLICY commitment",
        ));
    }
    if !pack.grid_metadata.links.is_empty() {
        return Err(MapGenLatticeError::new(
            "PR3 generator must not emit hyperlane/link topology",
        ));
    }
    Ok(())
}

fn assert_no_forbidden_generated_properties(
    pack: &HydratedScenarioPack,
) -> Result<(), MapGenLatticeError> {
    for property in &pack.game_mode.properties {
        reject_forbidden_property_name(property)?;
    }
    walk_forbidden_properties(&pack.root_node)?;
    Ok(())
}

fn walk_forbidden_properties(node: &HydratedScenarioNode) -> Result<(), MapGenLatticeError> {
    for property in &node.properties {
        reject_forbidden_property_name(property)?;
    }
    for child in &node.children {
        walk_forbidden_properties(child)?;
    }
    Ok(())
}

fn reject_forbidden_property_name(property: &PropertySpec) -> Result<(), MapGenLatticeError> {
    let haystack = format!(
        "{} {} {} {}",
        property.id, property.namespace, property.name, property.description
    );
    for forbidden in FORBIDDEN_GENERATED_PROPERTY_NAMES {
        if haystack.contains(forbidden) {
            return Err(MapGenLatticeError::new(format!(
                "generated property must not reference forbidden vocabulary `{forbidden}`"
            )));
        }
    }
    Ok(())
}

pub fn assert_allowed_simthing_kinds(
    node: &HydratedScenarioNode,
) -> Result<(), MapGenLatticeError> {
    match &node.kind {
        SimThingKind::World | SimThingKind::Location | SimThingKind::Cohort => {}
        SimThingKind::Custom(name) => {
            return Err(MapGenLatticeError::new(format!(
                "PR3 generator must not introduce custom SimThing kind `{name}`"
            )));
        }
        other => {
            return Err(MapGenLatticeError::new(format!(
                "PR3 generator must not emit SimThing kind `{other:?}`"
            )));
        }
    }
    for child in &node.children {
        assert_allowed_simthing_kinds(child)?;
    }
    Ok(())
}

pub fn collect_gridcell_location_ids(node: &HydratedScenarioNode) -> Vec<String> {
    let mut ids = Vec::new();
    collect_gridcell_location_ids_inner(node, &mut ids);
    ids
}

fn collect_gridcell_location_ids_inner(node: &HydratedScenarioNode, ids: &mut Vec<String>) {
    if node.properties.iter().any(|property| {
        property.namespace == "mapgen"
            && property.name == "mapping_role"
            && property.id.starts_with("mapgen_gridcell_mapping_role_")
    }) {
        ids.push(node.id.clone());
    }
    for child in &node.children {
        collect_gridcell_location_ids_inner(child, ids);
    }
}

fn push_mapping_role_property(out: &mut String, name: &str, id: &str, role: &str, indent: usize) {
    push_inert_property(out, id, name, role, indent);
}

fn push_inert_property(out: &mut String, id: &str, name: &str, value: &str, indent: usize) {
    let pad = " ".repeat(indent);
    out.push_str(&format!("{pad}property = {{\n"));
    out.push_str(&format!("{pad}    id = \"{id}\"\n"));
    out.push_str(&format!("{pad}    namespace = \"mapgen\"\n"));
    out.push_str(&format!("{pad}    name = \"{name}\"\n"));
    out.push_str(&format!(
        "{pad}    display_name = \"{}\"\n",
        escape_clause_string(name)
    ));
    out.push_str(&format!(
        "{pad}    description = \"inert={}\"\n",
        escape_clause_string(value)
    ));
    out.push_str(&format!("{pad}}}\n"));
}

fn push_inert_property_block(out: &mut String, id: &str, name: &str, value: &str, indent: usize) {
    out.push_str(&format!("{}properties = {{\n", " ".repeat(indent)));
    push_inert_property(out, id, name, value, indent + 1);
    out.push_str(&format!("{}}}\n", " ".repeat(indent)));
}

fn escape_clause_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn root_block(document: &crate::raw::RawDocument) -> Result<&RawBlock, MapGenLatticeError> {
    let RawValue::Block(block) = &document.root else {
        return Err(MapGenLatticeError::new("document root must be a block"));
    };
    Ok(block)
}

fn require_block<'a>(block: &'a RawBlock, key: &str) -> Result<&'a RawBlock, MapGenLatticeError> {
    let RawValue::Block(nested) = block_value(block, key)? else {
        return Err(MapGenLatticeError::new(format!("`{key}` must be a block")));
    };
    Ok(nested)
}

fn block_value<'a>(block: &'a RawBlock, key: &str) -> Result<&'a RawValue, MapGenLatticeError> {
    Ok(&require_property(block, key)?.value)
}

fn require_property<'a>(
    block: &'a RawBlock,
    key: &str,
) -> Result<&'a RawProperty, MapGenLatticeError> {
    block
        .properties
        .iter()
        .find(|property| property.key.text == key)
        .ok_or_else(|| MapGenLatticeError::new(format!("missing property `{key}`")))
}

fn property_values_matching<'a>(block: &'a RawBlock, key: &str) -> Vec<&'a RawValue> {
    block
        .properties
        .iter()
        .filter(|property| property.key.text == key)
        .map(|property| &property.value)
        .collect()
}

fn require_scalar(block: &RawBlock, key: &str) -> Result<String, MapGenLatticeError> {
    optional_scalar(block, key)?.ok_or_else(|| MapGenLatticeError::new(format!("missing `{key}`")))
}

fn optional_scalar(block: &RawBlock, key: &str) -> Result<Option<String>, MapGenLatticeError> {
    match block
        .properties
        .iter()
        .find(|property| property.key.text == key)
    {
        Some(property) => Ok(Some(scalar_text(&property.value)?.to_string())),
        None => Ok(None),
    }
}

fn optional_block(block: &RawBlock, key: &str) -> Result<Option<RawBlock>, MapGenLatticeError> {
    match block
        .properties
        .iter()
        .find(|property| property.key.text == key)
    {
        Some(property) => {
            let RawValue::Block(nested) = &property.value else {
                return Err(MapGenLatticeError::new(format!("`{key}` must be a block")));
            };
            Ok(Some(nested.clone()))
        }
        None => Ok(None),
    }
}

fn scalar_text(value: &RawValue) -> Result<&str, MapGenLatticeError> {
    let RawValue::Scalar(scalar) = value else {
        return Err(MapGenLatticeError::new("expected scalar value"));
    };
    Ok(scalar.text.as_str())
}
