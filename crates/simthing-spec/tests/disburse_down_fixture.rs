//! Shared disburse-down scoped fixture for spec/driver tests.
//!
//! 0R2 dependency-floor helper: `build_owner_silo_disburse_down_scoped_spec` is required by
//! surviving runtime/scenario boundary tests. Not a runnable test survivor.

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_gridcell_role_metadata, apply_owner_silo_metadata,
    apply_participant_owner_flow_demand_metadata, apply_participant_owner_flow_metadata,
    apply_scenario_metadata_to_root, apply_star_system_local_grid_frame_metadata,
    is_surface_gridcell, make_galaxy_map, make_owner_entity, make_planet_gridcell,
    structural_property_value_u32, SimThingScenarioGrid, SimThingScenarioProvenance,
    SimThingScenarioSpec, SimThingStructuralGridFrame, SimThingStructuralGridPlacement,
    GALAXY_GRIDCELL_ROLE_INERT, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_SCHEMA_VERSION,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
    STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS, STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS,
};
fn make_gridcell(role: &str, system_id: u32, col: u32, row: u32) -> SimThing {
    let mut cell = SimThing::new(SimThingKind::Location, 0);
    apply_gridcell_role_metadata(&mut cell, role);
    cell.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(system_id),
    );
    cell.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(col),
    );
    cell.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(row),
    );
    cell.add_child(SimThing::new(SimThingKind::Cohort, 0));
    cell
}
