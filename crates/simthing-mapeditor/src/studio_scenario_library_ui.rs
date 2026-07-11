//! Presentation state and minimal authority creation for the Studio scenario library.

use std::path::PathBuf;

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    game_session_owners, structural_property_value_u32, SimThingScenarioGrid,
    SimThingScenarioProvenance, SimThingScenarioSpec, SimThingStructuralGridFrame,
    SimThingStructuralGridPlacement, SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};
use thiserror::Error;

use crate::scenario_io::scenario_file_name;
use crate::studio_sim_clock_ui::StudioSimClockReadout;
use crate::{
    StudioHydrationError, StudioSession, StudioSimClockTransport, StudioSimClockTransportCommand,
};

/// Read-only Scenario telemetry projection for Telemetry dialog (11.4). Presentation only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioScenarioTelemetryReadout {
    pub scenario_id: String,
    pub clause_path: String,
    pub source_path: String,
    pub source_resolution: String,
    pub resolver_state: String,
    pub system_count: u32,
    pub owner_count: u32,
    pub stead_label: String,
    pub tick_index: u64,
    pub paused: bool,
}

/// Pure projection from loaded session + clause path + clock readout. Never mutates Spec.
pub fn build_studio_scenario_telemetry_readout(
    session: Option<&StudioSession>,
    clause_path: &str,
    clock: &StudioSimClockReadout,
) -> StudioScenarioTelemetryReadout {
    match session {
        None => StudioScenarioTelemetryReadout {
            scenario_id: "(none)".into(),
            clause_path: clause_path.to_string(),
            source_path: "(no loaded session)".into(),
            source_resolution: "n/a".into(),
            resolver_state: "empty operator resolver".into(),
            system_count: 0,
            owner_count: 0,
            stead_label: "n/a".into(),
            tick_index: clock.tick_index,
            paused: clock.paused,
        },
        Some(session) => {
            let summary = &session.scenario_summary;
            let owner_count = game_session_owners(&session.scenario_authority)
                .map(|o| o.len() as u32)
                .unwrap_or(0);
            let source_path = session
                .scenario_path
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "(in-memory)".into());
            let stead_label = if summary.stead_valid {
                "STEAD valid"
            } else {
                "STEAD invalid"
            };
            StudioScenarioTelemetryReadout {
                scenario_id: summary.scenario_id.clone(),
                clause_path: clause_path.to_string(),
                source_path,
                source_resolution: "sibling/canonical source_base".into(),
                resolver_state: "empty operator resolver".into(),
                system_count: summary.system_count,
                owner_count,
                stead_label: stead_label.into(),
                tick_index: clock.tick_index,
                paused: clock.paused,
            }
        }
    }
}

pub const STUDIO_SCENARIO_LIBRARY_DEFAULT_CREATE_ID: &str = "new_scenario";
pub const STUDIO_SCENARIO_LIBRARY_CREATE_PROVENANCE: &str = "STUDIO-SCENARIO-LIBRARY-CREATE-0";

/// Scenario Library tabs. Operator load path is **ClauseScript-only** (11.4).
/// `Json` remains as a non-default enum variant for legacy tests only — not shown in UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StudioScenarioLibraryTab {
    Json,
    #[default]
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
            // Operator default: ClauseScript load (STUDIO-CLAUSE-LOADER-SIMPLIFY-0).
            selected_tab: StudioScenarioLibraryTab::Clause,
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
