//! MapGen PR7 — PALMA W/D reach feedstock authoring (A3 / M7 / core §7).
//!
//! Lowers PR6 Movement-Front enrollment into existing `HydratedScenarioPalmaFeedstock` and
//! generic W-impedance compose surfaces bound to PR6 SaturatingFlux choke/suppression columns.
//! Authoring/lowering only — no driver/GPU/runtime execution, no routes, paths, predecessors,
//! movement orders, or pathfinding semantics.

use simthing_spec::spec::region_field::MappingExecutionProfile;
use simthing_spec::spec::w_impedance_compose::{
    WImpedanceComposeProfileSpec, WImpedanceComposeSpec,
};
use simthing_spec::{PropertySpec, compile_w_impedance_compose_preview};

use crate::error::HydrateError;
use crate::hydrate_palma_feedstock::{
    HydratedScenarioPalmaFeedstock, build_palma_feedstock_from_region_field,
};
use crate::hydrate_scenario::HydratedScenarioPack;
use crate::mapgen_lattice::assert_allowed_simthing_kinds;
use crate::mapgen_movement_front::{MAPGEN_MF_FIELD_OPERATOR_ID, MapGenMovementFrontAuthoring};
use crate::mapgen_neutral_ast::MapGenNeutralDocument;

const FORBIDDEN_GENERATED_PROPERTY_NAMES: &[&str] = &[
    "route",
    "path",
    "pathfinding",
    "predecessor",
    "movement",
    "movement_order",
    "destination",
    "fleet_path",
    "border",
    "frontline",
    "cpu_planner",
    "waypoint",
];

/// MapGen PR7 PALMA feedstock id for the tiny pentad slice.
pub const MAPGEN_PALMA_FEEDSTOCK_ID: &str = "mapgen_pentad_wd";

/// Default W output column for PALMA feedstock on the tiny slice.
pub const MAPGEN_PALMA_W_OUTPUT_COL: u32 = 3;

/// Default D output column for PALMA min-plus traversal feedstock on the tiny slice.
pub const MAPGEN_PALMA_D_OUTPUT_COL: u32 = 4;

/// Bounded PALMA feedstock authoring options for MapGen PR7.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapGenPalmaOptions {
    pub w_source_field_operator_id: &'static str,
    pub w_output_col: u32,
    pub d_output_col: u32,
}

impl Default for MapGenPalmaOptions {
    fn default() -> Self {
        Self {
            w_source_field_operator_id: MAPGEN_MF_FIELD_OPERATOR_ID,
            w_output_col: MAPGEN_PALMA_W_OUTPUT_COL,
            d_output_col: MAPGEN_PALMA_D_OUTPUT_COL,
        }
    }
}

/// Compact PALMA authoring report for the tiny MapGen slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapGenPalmaAuthoringReport {
    pub palma_feedstock_count: u32,
    pub w_source_field_operator_id: String,
    pub w_source_column: u32,
    pub w_output_column: u32,
    pub d_output_column: u32,
    pub grid_size: u32,
    pub n_dims: u32,
    pub source_col: u32,
    pub choke_output_col: u32,
    pub default_off_status: bool,
    pub route_surface_count: u32,
    pub predecessor_surface_count: u32,
    pub unsafe_expansion_flags: Vec<String>,
}

/// Scenario-container pack plus PALMA W/D authoring feedstock.
#[derive(Debug, Clone)]
pub struct MapGenPalmaFeedstockAuthoring {
    pub pack: HydratedScenarioPack,
    pub expansion_report: MapGenPalmaAuthoringReport,
}

/// MapGen PR7 PALMA feedstock failure.
#[derive(Debug)]
pub struct MapGenPalmaError {
    pub message: String,
}

impl MapGenPalmaError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    fn from_hydrate(err: HydrateError) -> Self {
        Self::new(err.to_string())
    }
}

impl std::fmt::Display for MapGenPalmaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MapGen PALMA error: {}", self.message)
    }
}

impl std::error::Error for MapGenPalmaError {}

/// Generate PALMA W/D reach feedstock from PR6 Movement-Front authoring.
pub fn generate_mapgen_palma_feedstock(
    movement_front: &MapGenMovementFrontAuthoring,
    options: MapGenPalmaOptions,
) -> Result<MapGenPalmaFeedstockAuthoring, MapGenPalmaError> {
    validate_palma_options(&options)?;
    let field = require_pr6_region_field(&movement_front.pack)?;
    validate_pr6_w_source(field, options.w_source_field_operator_id)?;

    let palma = build_palma_feedstock_from_region_field(
        MAPGEN_PALMA_FEEDSTOCK_ID,
        options.w_source_field_operator_id,
        options.w_output_col,
        options.d_output_col,
        field,
    )
    .map_err(MapGenPalmaError::from_hydrate)?;

    let w_compose = build_w_impedance_compose_from_palma(&palma);
    compile_w_impedance_compose_preview(&w_compose).map_err(|err| {
        MapGenPalmaError::new(format!("PR7 W impedance compose admission rejected: {err}"))
    })?;

    let expansion_report = build_expansion_report(&palma);
    let mut pack = movement_front.pack.clone();
    pack.palma_feedstock = Some(palma);
    pack.w_impedance_compose = Some(w_compose);
    pack.stress_compose = None;
    pack.game_mode.mapping_execution_profile = MappingExecutionProfile::Disabled;

    assert_no_route_or_predecessor_surfaces(&pack)?;
    assert_no_forbidden_generated_properties(&pack)?;
    assert_allowed_simthing_kinds(&pack.root_node)
        .map_err(|err| MapGenPalmaError::new(err.message))?;

    Ok(MapGenPalmaFeedstockAuthoring {
        pack,
        expansion_report,
    })
}

/// Convenience: parse raw fixture → PR2 → PR3 → PR4 → PR5 → PR6 → PR7 with defaults.
pub fn generate_default_mapgen_palma_feedstock(
    document: &MapGenNeutralDocument,
) -> Result<MapGenPalmaFeedstockAuthoring, MapGenPalmaError> {
    use crate::mapgen_movement_front::generate_default_mapgen_movement_front_authoring;
    let movement_front = generate_default_mapgen_movement_front_authoring(document)
        .map_err(|err| MapGenPalmaError::new(err.message))?;
    generate_mapgen_palma_feedstock(&movement_front, MapGenPalmaOptions::default())
}

pub fn validate_palma_options(options: &MapGenPalmaOptions) -> Result<(), MapGenPalmaError> {
    if options.w_source_field_operator_id.is_empty() {
        return Err(MapGenPalmaError::new(
            "PR7 PALMA w_source_field_operator_id must be non-empty",
        ));
    }
    Ok(())
}

pub fn build_w_impedance_compose_from_palma(
    palma: &HydratedScenarioPalmaFeedstock,
) -> WImpedanceComposeSpec {
    let choke_a = palma
        .choke_output_col
        .expect("PALMA feedstock requires saturating_flux choke_output_col");
    WImpedanceComposeSpec {
        width: palma.grid_size,
        height: palma.grid_size,
        n_dims: palma.n_dims,
        base_w_col: palma.source_col,
        choke_a_col: choke_a,
        choke_b_col: spare_compose_choke_b_col(palma),
        profiles: vec![WImpedanceComposeProfileSpec {
            weight_a: 1.0,
            weight_b: 1.0,
            output_w_col: palma.w_output_col,
        }],
    }
}

fn require_pr6_region_field(
    pack: &HydratedScenarioPack,
) -> Result<&simthing_spec::spec::region_field::RegionFieldSpec, MapGenPalmaError> {
    if pack.commitment.is_none() {
        return Err(MapGenPalmaError::new(
            "PR7 PALMA requires PR6 Movement-Front commitment feedstock on the pack",
        ));
    }
    let field = pack.game_mode.region_fields.first().ok_or_else(|| {
        MapGenPalmaError::new("PR7 PALMA requires PR6 Movement-Front region field on the pack")
    })?;
    Ok(field)
}

fn validate_pr6_w_source(
    field: &simthing_spec::spec::region_field::RegionFieldSpec,
    w_source_field_operator_id: &str,
) -> Result<(), MapGenPalmaError> {
    if w_source_field_operator_id != MAPGEN_MF_FIELD_OPERATOR_ID {
        return Err(MapGenPalmaError::new(format!(
            "PR7 PALMA w_source `{w_source_field_operator_id}` is not the PR6 Movement-Front field operator id"
        )));
    }
    if field.name != format!("{MAPGEN_MF_FIELD_OPERATOR_ID}_field") {
        return Err(MapGenPalmaError::new(
            "PR7 PALMA w_source does not match PR6 Movement-Front region field",
        ));
    }
    Ok(())
}

fn spare_compose_choke_b_col(palma: &HydratedScenarioPalmaFeedstock) -> u32 {
    let choke_a = palma
        .choke_output_col
        .expect("PALMA feedstock requires saturating_flux choke_output_col");
    let claimed = [
        palma.source_col,
        choke_a,
        palma.w_output_col,
        palma.d_output_col,
    ];
    (0..palma.n_dims)
        .find(|col| !claimed.contains(col))
        .expect("MapGen PALMA n_dims must leave a spare column for compose choke_b")
}

fn build_expansion_report(palma: &HydratedScenarioPalmaFeedstock) -> MapGenPalmaAuthoringReport {
    let choke_output_col = palma
        .choke_output_col
        .expect("PALMA feedstock requires saturating_flux choke_output_col");
    MapGenPalmaAuthoringReport {
        palma_feedstock_count: 1,
        w_source_field_operator_id: palma.w_source_field_operator_id.clone(),
        w_source_column: choke_output_col,
        w_output_column: palma.w_output_col,
        d_output_column: palma.d_output_col,
        grid_size: palma.grid_size,
        n_dims: palma.n_dims,
        source_col: palma.source_col,
        choke_output_col,
        default_off_status: true,
        route_surface_count: 0,
        predecessor_surface_count: 0,
        unsafe_expansion_flags: Vec::new(),
    }
}

fn assert_no_route_or_predecessor_surfaces(
    pack: &HydratedScenarioPack,
) -> Result<(), MapGenPalmaError> {
    let json = serde_json::to_string(&pack.game_mode).map_err(|err| {
        MapGenPalmaError::new(format!("PR7 PALMA pack serialization failed: {err}"))
    })?;
    for forbidden in [
        "route",
        "predecessor",
        "pathfinding",
        "movement_order",
        "destination",
    ] {
        if json.contains(forbidden) {
            return Err(MapGenPalmaError::new(format!(
                "PR7 PALMA generator must not emit `{forbidden}` surfaces"
            )));
        }
    }
    Ok(())
}

fn assert_no_forbidden_generated_properties(
    pack: &HydratedScenarioPack,
) -> Result<(), MapGenPalmaError> {
    for property in &pack.game_mode.properties {
        reject_forbidden_property_name(property)?;
    }
    walk_forbidden_properties(&pack.root_node)?;
    Ok(())
}

fn walk_forbidden_properties(
    node: &crate::hydrate_scenario::HydratedScenarioNode,
) -> Result<(), MapGenPalmaError> {
    for property in &node.properties {
        reject_forbidden_property_name(property)?;
    }
    for child in &node.children {
        walk_forbidden_properties(child)?;
    }
    Ok(())
}

fn reject_forbidden_property_name(property: &PropertySpec) -> Result<(), MapGenPalmaError> {
    let haystack = format!(
        "{} {} {} {}",
        property.id, property.namespace, property.name, property.description
    );
    for forbidden in FORBIDDEN_GENERATED_PROPERTY_NAMES {
        if haystack.contains(forbidden) {
            return Err(MapGenPalmaError::new(format!(
                "generated property must not reference forbidden vocabulary `{forbidden}`"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mapgen_movement_front::{
        MAPGEN_MF_CHOKE_OUTPUT_COL, MAPGEN_MF_N_DIMS, MAPGEN_MF_SOURCE_COL,
    };

    #[test]
    fn default_columns_match_pr6_slice_geometry() {
        assert_eq!(MAPGEN_PALMA_W_OUTPUT_COL, 3);
        assert_eq!(MAPGEN_PALMA_D_OUTPUT_COL, 4);
        assert_eq!(MAPGEN_MF_N_DIMS, 6);
        assert_eq!(MAPGEN_MF_SOURCE_COL, 0);
        assert_eq!(MAPGEN_MF_CHOKE_OUTPUT_COL, 2);
    }
}
