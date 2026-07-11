//! Presentation-only state for the Studio scenario library modal.

use crate::{StudioSimClockTransport, StudioSimClockTransportCommand};

pub const STUDIO_SCENARIO_LIBRARY_CREATE_DEFERRED_MESSAGE: &str =
    "Coming next: STUDIO-SCENARIO-LIBRARY-CREATE-0";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StudioScenarioLibraryTab {
    #[default]
    Json,
    Clause,
    CreateDeferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StudioScenarioLibraryModel {
    pub visible: bool,
    pub selected_tab: StudioScenarioLibraryTab,
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

    /// A visible library owns a pause gate. Closing deliberately does not restore Play.
    pub fn enforce_pause(&self, transport: &mut StudioSimClockTransport) {
        if self.visible {
            let result = transport.apply(StudioSimClockTransportCommand::Pause);
            debug_assert!(result.is_ok(), "Pause transport command is infallible");
        }
    }

    pub fn create_is_deferred(&self) -> bool {
        true
    }
}
