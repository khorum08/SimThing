//! Presentation state and minimal authority creation for the Studio scenario library.

use std::path::PathBuf;

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    structural_property_value_u32, SimThingScenarioGrid, SimThingScenarioProvenance,
    SimThingScenarioSpec, SimThingStructuralGridFrame, SimThingStructuralGridPlacement,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
    SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};
use thiserror::Error;

use crate::scenario_io::scenario_file_name;
use crate::{
    StudioHydrationError, StudioSession, StudioSimClockTransport, StudioSimClockTransportCommand,
};

pub const STUDIO_SCENARIO_LIBRARY_DEFAULT_CREATE_ID: &str = "new_scenario";
pub const STUDIO_SCENARIO_LIBRARY_CREATE_PROVENANCE: &str = "STUDIO-SCENARIO-LIBRARY-CREATE-0";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StudioScenarioLibraryTab {
    #[default]
    Json,
    Clause,
    Create,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioScenarioLibraryModel {
    pub visible: bool,
    pub selected_tab: StudioScenarioLibraryTab,
    pub create_scenario_id: String,
}

impl Default for StudioScenarioLibraryModel {
    fn default() -> Self {
        Self {
            visible: false,
            selected_tab: StudioScenarioLibraryTab::Json,
            create_scenario_id: STUDIO_SCENARIO_LIBRARY_DEFAULT_CREATE_ID.to_string(),
        }
    }
}

impl StudioScenarioLibraryModel {
    pub fn open(&mut self, transport: &mut StudioSimClockTransport) {
        self.visible = true;
        self.enforce_pause(transport);
    }

    pub fn toggle_visible(&mut self, transport: &mut StudioSimClockTransport) {
        if self.visible {
            self.close();
        } else {
            self.open(transport);
        }
    }

    pub fn close(&mut self) {
        self.visible = false;
    }

    /// Cancel the modal without restoring Play, even if called after other UI input.
    pub fn cancel(&mut self, transport: &mut StudioSimClockTransport) {
        self.enforce_pause(transport);
        self.close();
    }

    /// A visible library owns a pause gate. Closing deliberately does not restore Play.
    pub fn enforce_pause(&self, transport: &mut StudioSimClockTransport) {
        if self.visible {
            let result = transport.apply(StudioSimClockTransportCommand::Pause);
            debug_assert!(result.is_ok(), "Pause transport command is infallible");
        }
    }

    pub fn create_is_available(&self) -> bool {
        true
    }
}

#[derive(Debug, Error)]
pub enum StudioScenarioLibraryCreateError {
    #[error("invalid scenario id: {0}")]
    InvalidScenarioId(String),
    #[error("created scenario could not hydrate: {0}")]
    Hydration(#[from] StudioHydrationError),
}

pub fn build_blank_studio_scenario_spec(
    scenario_id: &str,
) -> Result<SimThingScenarioSpec, StudioScenarioLibraryCreateError> {
    let scenario_id = validate_create_scenario_id(scenario_id)?;
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut map = SimThing::new(SimThingKind::Location, 0);
    let map_raw = map.id.raw();

    let mut cell = SimThing::new(SimThingKind::Location, 0);
    cell.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(1),
    );
    cell.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    cell.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    let mut neutral_payload = SimThing::new(SimThingKind::Cohort, 0);
    neutral_payload.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(1),
    );
    cell.add_child(neutral_payload);
    let cell_raw = cell.id.raw();
    let location_id = format!("{scenario_id}_origin");
    map.add_child(cell);
    root.add_child(map);

    Ok(SimThingScenarioSpec {
        scenario_id: scenario_id.to_string(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 1,
                height: 1,
                occupied_cells: 1,
            },
            map_container_id: map_raw.to_string(),
            placements: vec![SimThingStructuralGridPlacement {
                location_id: location_id.clone(),
                target_id: location_id,
                system_id: 1,
                row: 0,
                col: 0,
                simthing_id_raw: cell_raw,
            }],
        },
        links: Vec::new(),
        provenance: SimThingScenarioProvenance {
            source: STUDIO_SCENARIO_LIBRARY_CREATE_PROVENANCE.to_string(),
            generator_seed: 0,
            generator_shape: "blank_minimal".to_string(),
            ..Default::default()
        },
    })
}

pub fn create_blank_studio_session(
    scenario_id: &str,
) -> Result<StudioSession, StudioScenarioLibraryCreateError> {
    let scenario = build_blank_studio_scenario_spec(scenario_id)?;
    let path = PathBuf::from(scenario_file_name(&scenario.scenario_id));
    Ok(StudioSession::from_loaded_scenario(scenario, path, None)?)
}

fn validate_create_scenario_id(
    scenario_id: &str,
) -> Result<&str, StudioScenarioLibraryCreateError> {
    let scenario_id = scenario_id.trim();
    if scenario_id.is_empty() {
        return Err(StudioScenarioLibraryCreateError::InvalidScenarioId(
            "value is empty".to_string(),
        ));
    }
    if scenario_id.len() > 64 {
        return Err(StudioScenarioLibraryCreateError::InvalidScenarioId(
            "value exceeds 64 bytes".to_string(),
        ));
    }
    if !scenario_id
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
    {
        return Err(StudioScenarioLibraryCreateError::InvalidScenarioId(
            "use ASCII letters, numbers, '_' or '-'".to_string(),
        ));
    }
    Ok(scenario_id)
}
