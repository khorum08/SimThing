//! MapGen PR6 — Movement-Front L1/L2/L3 authoring feedstock (core §7 / ADR-MAP).
//!
//! Lowers the PR5-enrolled tiny MapGen slice into existing RegionField / SaturatingFlux,
//! hierarchy reduction, and FIELD_POLICY commitment surfaces bound to PR4 suppression RF
//! pressure. Authoring/lowering only — no driver/GPU/runtime execution, no PALMA, no
//! pathfinding/movement/routes/predecessors/border/frontline semantics.

use simthing_spec::FIRST_SLICE_FIELD_URGENCY_COL;
use simthing_spec::spec::region_field::{
    ArenaPressureBindingSpec, FirstSliceCommitmentDirectionSpec, FirstSliceCommitmentSpec,
    MappingExecutionProfile, PressurePlacementSpec, PressureSourceSpec, RegionFieldCadenceSpec,
    RegionFieldFormulaBindingSpec, RegionFieldGridProfile, RegionFieldOperatorSpec,
    RegionFieldReductionSpec, RegionFieldSourcePolicySpec, RegionFieldSpec,
};
use simthing_spec::{PropertySpec, compile_region_field_preview};

use crate::hydrate_field_operator::BH3_SATURATING_FLUX_CHI_CFL_MAX;
use crate::hydrate_scenario::HydratedScenarioPack;
use crate::hydrate_scenario_commitment::HydratedScenarioCommitment;
use crate::mapgen_lattice::assert_allowed_simthing_kinds;
use crate::mapgen_links::MapGenLinksEnrollment;
use crate::mapgen_neutral_ast::MapGenNeutralDocument;
use crate::mapgen_resource_flow::MAPGEN_RF_SUPPRESSION_ARENA;

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
    "palma_feedstock",
];

/// MapGen PR6 field-operator id for the suppression-front lattice.
pub const MAPGEN_MF_FIELD_OPERATOR_ID: &str = "mapgen_suppression_front";

/// MapGen PR6 commitment id for sector threshold feedstock.
pub const MAPGEN_MF_COMMITMENT_ID: &str = "mapgen_sector_threshold";

/// Default bounded L1 horizon for the tiny pentad slice (core §7 P1).
pub const MAPGEN_MF_DEFAULT_HORIZON: u32 = 4;

/// Maximum admitted L1 horizon for MapGen PR6 (no horizon widening).
pub const MAPGEN_MF_MAX_HORIZON: u32 = 8;

/// Default SaturatingFlux source/target column for the tiny slice.
pub const MAPGEN_MF_SOURCE_COL: u32 = 0;

/// Column width for the tiny slice field operator.
pub const MAPGEN_MF_N_DIMS: u32 = 6;

/// Choke output column written by SaturatingFlux in the tiny slice.
pub const MAPGEN_MF_CHOKE_OUTPUT_COL: u32 = 2;

/// L2 reduction scope label for the pentad sector parent.
pub const MAPGEN_MF_L2_REDUCTION_SCOPE: &str = "gridcell_lattice_to_sector_parent";

/// Bounded Movement-Front authoring options for MapGen PR6.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapGenMovementFrontOptions {
    pub horizon: u32,
    pub max_horizon: u32,
    pub threshold: f32,
    pub event_kind: u32,
    pub weight_pressure: f32,
    pub weight_resource: f32,
    pub u_sat: f32,
    pub chi: f32,
}

impl Default for MapGenMovementFrontOptions {
    fn default() -> Self {
        Self {
            horizon: MAPGEN_MF_DEFAULT_HORIZON,
            max_horizon: MAPGEN_MF_MAX_HORIZON,
            threshold: 0.75,
            event_kind: 7,
            weight_pressure: 1.0,
            weight_resource: 1.0,
            u_sat: 1.0,
            chi: BH3_SATURATING_FLUX_CHI_CFL_MAX,
        }
    }
}

/// Compact Movement-Front authoring report for the tiny MapGen slice.
#[derive(Debug, Clone, PartialEq)]
pub struct MapGenMovementFrontAuthoringReport {
    pub l1_field_operator_count: u32,
    pub l1_horizon: u32,
    pub l1_locality_bound: u32,
    pub l2_reduction_count: u32,
    pub l2_reduction_scope: String,
    pub l3_commitment_count: u32,
    pub l3_thresholds: Vec<f32>,
    pub generated_columns: Vec<u32>,
    pub rf_source_bindings: Vec<String>,
    pub forbidden_surface_count: u32,
    pub unsafe_expansion_flags: Vec<String>,
}

/// Scenario-container pack plus Movement-Front authoring feedstock.
#[derive(Debug, Clone)]
pub struct MapGenMovementFrontAuthoring {
    pub pack: HydratedScenarioPack,
    pub expansion_report: MapGenMovementFrontAuthoringReport,
}

/// MapGen PR6 Movement-Front authoring failure.
#[derive(Debug)]
pub struct MapGenMovementFrontError {
    pub message: String,
}

impl MapGenMovementFrontError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for MapGenMovementFrontError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MapGen movement front error: {}", self.message)
    }
}

impl std::error::Error for MapGenMovementFrontError {}

/// Generate Movement-Front L1/L2/L3 authoring feedstock from a PR5 links enrollment.
pub fn generate_mapgen_movement_front_authoring(
    links: &MapGenLinksEnrollment,
    options: MapGenMovementFrontOptions,
) -> Result<MapGenMovementFrontAuthoring, MapGenMovementFrontError> {
    validate_options(&options)?;
    let surfaces = build_movement_front_surfaces(&links.pack, &options)?;
    validate_l1_operator_locality(&surfaces.region_field)?;
    compile_region_field_preview(&surfaces.region_field).map_err(|err| {
        MapGenMovementFrontError::new(format!("PR6 region field admission rejected: {err}"))
    })?;

    let expansion_report = build_expansion_report(&links.pack, &options, &surfaces.region_field);
    let mut pack = links.pack.clone();
    pack.w_impedance_compose = None;
    pack.stress_compose = None;
    pack.palma_feedstock = None;
    pack.game_mode.region_fields = vec![surfaces.region_field];
    pack.game_mode.mapping_execution_profile = MappingExecutionProfile::Disabled;
    pack.commitment = Some(surfaces.commitment_metadata);

    assert_no_deferred_pr6_surfaces(&pack)?;
    assert_no_forbidden_generated_properties(&pack)?;
    assert_allowed_simthing_kinds(&pack.root_node)
        .map_err(|err| MapGenMovementFrontError::new(err.message))?;

    Ok(MapGenMovementFrontAuthoring {
        pack,
        expansion_report,
    })
}

/// Convenience: parse raw fixture → PR3 → PR4 → PR5 → PR6 with defaults.
pub fn generate_default_mapgen_movement_front_authoring(
    document: &MapGenNeutralDocument,
) -> Result<MapGenMovementFrontAuthoring, MapGenMovementFrontError> {
    use crate::mapgen_links::generate_default_mapgen_links_enrollment;
    let links = generate_default_mapgen_links_enrollment(document)
        .map_err(|err| MapGenMovementFrontError::new(err.message))?;
    generate_mapgen_movement_front_authoring(&links, MapGenMovementFrontOptions::default())
}

struct BuiltMovementFrontSurfaces {
    region_field: RegionFieldSpec,
    commitment_metadata: HydratedScenarioCommitment,
}

fn build_movement_front_surfaces(
    pack: &HydratedScenarioPack,
    options: &MapGenMovementFrontOptions,
) -> Result<BuiltMovementFrontSurfaces, MapGenMovementFrontError> {
    if pack.game_mode.resource_flow.is_none() {
        return Err(MapGenMovementFrontError::new(
            "PR6 requires PR4 Resource Flow enrollment on the pack",
        ));
    }
    let grid_size = pack.grid_metadata.grid_size;
    if grid_size == 0 {
        return Err(MapGenMovementFrontError::new(
            "PR6 requires a positive fixture lattice grid_size",
        ));
    }
    let cell_count = grid_size.saturating_mul(grid_size);
    if pack.grid_metadata.placements.is_empty() {
        return Err(MapGenMovementFrontError::new(
            "PR6 requires at least one gridcell placement for pressure_binding",
        ));
    }

    let pressure_binding = ArenaPressureBindingSpec {
        arena: MAPGEN_RF_SUPPRESSION_ARENA.into(),
        source: PressureSourceSpec::Named {
            sub_field: "flow".into(),
        },
        placements: pack
            .grid_metadata
            .placements
            .iter()
            .map(|placement| PressurePlacementSpec {
                target_id: placement.target_id.clone(),
                row: placement.row,
                col: placement.col,
            })
            .collect(),
    };

    let parent_formula = RegionFieldFormulaBindingSpec {
        formula_class: "field_urgency".into(),
        tree_id: None,
        weight_pressure: Some(options.weight_pressure),
        weight_resource: Some(options.weight_resource),
    };
    let reduction = RegionFieldReductionSpec {
        child_slot_start: 0,
        child_slot_count: cell_count,
        child_col: 0,
        parent_slot: cell_count,
        parent_col: 0,
        order_band: 0,
    };
    let commitment = FirstSliceCommitmentSpec {
        source_formula_class: "field_urgency".into(),
        parent_slot: cell_count,
        urgency_col: FIRST_SLICE_FIELD_URGENCY_COL,
        threshold: options.threshold,
        direction: FirstSliceCommitmentDirectionSpec::Upward,
        event_kind: options.event_kind,
        effect: None,
    };

    let region_field = RegionFieldSpec {
        name: format!("{MAPGEN_MF_FIELD_OPERATOR_ID}_field"),
        grid_size,
        n_dims: MAPGEN_MF_N_DIMS,
        source_col: MAPGEN_MF_SOURCE_COL,
        target_col: MAPGEN_MF_SOURCE_COL,
        operator: RegionFieldOperatorSpec::SaturatingFlux {
            u_sat: options.u_sat,
            chi: options.chi,
            choke_output_col: Some(MAPGEN_MF_CHOKE_OUTPUT_COL),
        },
        horizon: options.horizon,
        allow_extended_horizon: false,
        alpha_self: 0.5,
        gamma_neighbor: 0.125,
        source_cap: None,
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::StandardSquare,
        reduction: Some(reduction.clone()),
        parent_formula: Some(parent_formula.clone()),
        commitment: Some(commitment.clone()),
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: Default::default(),
        pressure_binding: Some(pressure_binding),
    };

    Ok(BuiltMovementFrontSurfaces {
        region_field,
        commitment_metadata: HydratedScenarioCommitment {
            commitment_id: MAPGEN_MF_COMMITMENT_ID.into(),
            source_field_operator_id: MAPGEN_MF_FIELD_OPERATOR_ID.into(),
            field_urgency_column: Some(MAPGEN_MF_CHOKE_OUTPUT_COL),
            commitment,
        },
    })
}

pub fn validate_options(
    options: &MapGenMovementFrontOptions,
) -> Result<(), MapGenMovementFrontError> {
    if options.max_horizon == 0 || options.horizon == 0 {
        return Err(MapGenMovementFrontError::new(
            "PR6 L1 horizon caps must be positive",
        ));
    }
    if options.horizon > options.max_horizon {
        return Err(MapGenMovementFrontError::new(format!(
            "PR6 L1 horizon {} exceeds max_horizon {}",
            options.horizon, options.max_horizon
        )));
    }
    if options.max_horizon > MAPGEN_MF_MAX_HORIZON {
        return Err(MapGenMovementFrontError::new(format!(
            "PR6 max_horizon {} exceeds PR6 cap {MAPGEN_MF_MAX_HORIZON}",
            options.max_horizon
        )));
    }
    if !options.threshold.is_finite() || options.threshold <= 0.0 {
        return Err(MapGenMovementFrontError::new(
            "PR6 L3 commitment threshold must be finite and > 0",
        ));
    }
    if !options.u_sat.is_finite() || options.u_sat <= 0.0 {
        return Err(MapGenMovementFrontError::new(
            "PR6 SaturatingFlux u_sat must be finite and > 0",
        ));
    }
    if !options.chi.is_finite()
        || options.chi <= 0.0
        || options.chi > BH3_SATURATING_FLUX_CHI_CFL_MAX
    {
        return Err(MapGenMovementFrontError::new(format!(
            "PR6 SaturatingFlux chi must be finite, > 0, and <= {BH3_SATURATING_FLUX_CHI_CFL_MAX}"
        )));
    }
    if !options.weight_pressure.is_finite() || !options.weight_resource.is_finite() {
        return Err(MapGenMovementFrontError::new(
            "PR6 L3 formula weights must be finite",
        ));
    }
    Ok(())
}

pub fn validate_l1_operator_locality(
    field: &RegionFieldSpec,
) -> Result<(), MapGenMovementFrontError> {
    match field.operator {
        RegionFieldOperatorSpec::SaturatingFlux { .. } => {}
        RegionFieldOperatorSpec::SourceCappedNormalized | RegionFieldOperatorSpec::Normalized => {
            return Err(MapGenMovementFrontError::new(
                "PR6 rejects dense/global normalized diffusion profiles",
            ));
        }
        RegionFieldOperatorSpec::Gradient { .. } => {
            return Err(MapGenMovementFrontError::new(
                "PR6 rejects gradient/global diffusion profiles",
            ));
        }
    }
    if field.allow_extended_horizon {
        return Err(MapGenMovementFrontError::new(
            "PR6 rejects horizon widening via allow_extended_horizon",
        ));
    }
    if field.horizon > MAPGEN_MF_MAX_HORIZON {
        return Err(MapGenMovementFrontError::new(format!(
            "PR6 L1 horizon {} exceeds bounded cap {MAPGEN_MF_MAX_HORIZON}",
            field.horizon
        )));
    }
    if field
        .reduction
        .as_ref()
        .is_some_and(|reduction| reduction.child_slot_count == 0)
    {
        return Err(MapGenMovementFrontError::new(
            "PR6 L2 reduction requires a positive child_slot_count cap",
        ));
    }
    if field
        .commitment
        .as_ref()
        .is_some_and(|commitment| !commitment.threshold.is_finite() || commitment.threshold <= 0.0)
    {
        return Err(MapGenMovementFrontError::new(
            "PR6 L3 commitment requires a finite threshold",
        ));
    }
    Ok(())
}

pub fn assert_no_palma_feedstock(
    pack: &HydratedScenarioPack,
) -> Result<(), MapGenMovementFrontError> {
    if pack.palma_feedstock.is_some() {
        return Err(MapGenMovementFrontError::new(
            "PR6 generator must not emit PALMA W/D feedstock",
        ));
    }
    Ok(())
}

fn build_expansion_report(
    links_pack: &HydratedScenarioPack,
    options: &MapGenMovementFrontOptions,
    field: &RegionFieldSpec,
) -> MapGenMovementFrontAuthoringReport {
    let mut unsafe_expansion_flags = Vec::new();
    if field.horizon >= options.max_horizon {
        unsafe_expansion_flags.push("l1_horizon_at_cap".into());
    }
    if field.reduction.as_ref().is_some_and(|reduction| {
        reduction.child_slot_count >= field.grid_size.saturating_mul(field.grid_size)
    }) {
        unsafe_expansion_flags.push("l2_reduction_spans_full_lattice".into());
    }

    MapGenMovementFrontAuthoringReport {
        l1_field_operator_count: 1,
        l1_horizon: field.horizon,
        l1_locality_bound: field.horizon,
        l2_reduction_count: if field.reduction.is_some() { 1 } else { 0 },
        l2_reduction_scope: MAPGEN_MF_L2_REDUCTION_SCOPE.into(),
        l3_commitment_count: if field.commitment.is_some() { 1 } else { 0 },
        l3_thresholds: vec![options.threshold],
        generated_columns: vec![
            MAPGEN_MF_SOURCE_COL,
            MAPGEN_MF_CHOKE_OUTPUT_COL,
            FIRST_SLICE_FIELD_URGENCY_COL,
        ],
        rf_source_bindings: vec![format!("{MAPGEN_RF_SUPPRESSION_ARENA}::flow")],
        forbidden_surface_count: if links_pack.palma_feedstock.is_some() {
            1
        } else {
            0
        },
        unsafe_expansion_flags,
    }
}

fn assert_no_deferred_pr6_surfaces(
    pack: &HydratedScenarioPack,
) -> Result<(), MapGenMovementFrontError> {
    assert_no_palma_feedstock(pack)?;
    if pack.w_impedance_compose.is_some() || pack.stress_compose.is_some() {
        return Err(MapGenMovementFrontError::new(
            "PR6 generator must not emit PALMA-adjacent W/stress compose surfaces in this rung",
        ));
    }
    Ok(())
}

fn assert_no_forbidden_generated_properties(
    pack: &HydratedScenarioPack,
) -> Result<(), MapGenMovementFrontError> {
    for property in &pack.game_mode.properties {
        reject_forbidden_property_name(property)?;
    }
    walk_forbidden_properties(&pack.root_node)?;
    Ok(())
}

fn walk_forbidden_properties(
    node: &crate::hydrate_scenario::HydratedScenarioNode,
) -> Result<(), MapGenMovementFrontError> {
    for property in &node.properties {
        reject_forbidden_property_name(property)?;
    }
    for child in &node.children {
        walk_forbidden_properties(child)?;
    }
    Ok(())
}

fn reject_forbidden_property_name(property: &PropertySpec) -> Result<(), MapGenMovementFrontError> {
    let haystack = format!(
        "{} {} {} {}",
        property.id, property.namespace, property.name, property.description
    );
    for forbidden in FORBIDDEN_GENERATED_PROPERTY_NAMES {
        if haystack.contains(forbidden) {
            return Err(MapGenMovementFrontError::new(format!(
                "generated property must not reference forbidden vocabulary `{forbidden}`"
            )));
        }
    }
    Ok(())
}
