//! Reusable warning dialog model for unimplemented controls.

use crate::star_render::{StarFalloffSettings, StarRenderMode};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WarningDialogModel {
    pub visible: bool,
    pub title: String,
    pub message: String,
}

impl WarningDialogModel {
    pub const DEFAULT_MESSAGE: &'static str =
        "This option is visible for roadmap continuity but is not implemented in this build.";

    pub fn for_action(action: StudioAction) -> Self {
        match action {
            StudioAction::New => Self {
                visible: true,
                title: "New session".into(),
                message: "New session (clear studio state) is not implemented in PR1R.".into(),
            },
            StudioAction::Load => Self {
                visible: true,
                title: "Load session".into(),
                message: "Load session (RON/JSON editor session) is not implemented in PR1R."
                    .into(),
            },
            StudioAction::Save => Self {
                visible: true,
                title: "Save session".into(),
                message:
                    "Save session (editor session + scenario + report) is not implemented in PR1R."
                        .into(),
            },
            StudioAction::InactivePreset(name) => Self {
                visible: true,
                title: "Preset unavailable".into(),
                message: format!(
                    "Preset '{name}' is visible for roadmap continuity but is not active in PR1R."
                ),
            },
            StudioAction::InactiveControl(name) => Self {
                visible: true,
                title: "Control unavailable".into(),
                message: format!(
                    "'{name}' is visible for roadmap continuity but is not implemented in PR1R."
                ),
            },
        }
    }

    pub fn dismiss(&mut self) {
        self.visible = false;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StudioAction {
    New,
    Load,
    Save,
    InactivePreset(String),
    InactiveControl(String),
}

pub fn unimplemented_action_response(action: StudioAction) -> WarningDialogModel {
    WarningDialogModel::for_action(action)
}

pub fn inactive_control_click_action(label: &str) -> StudioAction {
    StudioAction::InactiveControl(label.to_string())
}

pub fn inactive_control_warning(label: &str) -> WarningDialogModel {
    unimplemented_action_response(inactive_control_click_action(label))
}

pub fn new_load_save_actions() -> [StudioAction; 3] {
    [StudioAction::New, StudioAction::Load, StudioAction::Save]
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SettingsDialogModel {
    pub visible: bool,
    pub position: [f32; 2],
    pub star_render: StarFalloffSettings,
    pub star_render_mode: StarRenderMode,
    pub hyperlane_render: crate::hyperlane_buckets::HyperlaneRenderSettings,
}

impl SettingsDialogModel {
    pub fn new(
        visible: bool,
        position: [f32; 2],
        star_render: StarFalloffSettings,
        star_render_mode: StarRenderMode,
        hyperlane_render: crate::hyperlane_buckets::HyperlaneRenderSettings,
    ) -> Self {
        Self {
            visible,
            position,
            star_render: star_render.clamped(),
            star_render_mode,
            hyperlane_render: hyperlane_render.clamped(),
        }
    }

    pub fn open(&mut self) {
        self.visible = true;
    }

    pub fn toggle_visible(&mut self) {
        self.visible = !self.visible;
    }

    pub fn close(&mut self) {
        self.visible = false;
    }

    pub fn close_icon(&mut self) {
        self.close();
    }

    pub fn close_button(&mut self) {
        self.close();
    }

    pub fn set_star_render(&mut self, star_render: StarFalloffSettings) {
        self.star_render = star_render.clamped();
    }

    pub fn set_star_render_mode(&mut self, star_render_mode: StarRenderMode) {
        self.star_render_mode = star_render_mode;
    }

    pub fn set_hyperlane_render(
        &mut self,
        hyperlane_render: crate::hyperlane_buckets::HyperlaneRenderSettings,
    ) {
        self.hyperlane_render = hyperlane_render.clamped();
    }
}

/// Movable Performance Telemetry window state (presentation only).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TelemetryDialogModel {
    pub visible: bool,
    pub position: [f32; 2],
}

impl TelemetryDialogModel {
    pub fn new(visible: bool, position: [f32; 2]) -> Self {
        Self { visible, position }
    }

    pub fn toggle_visible(&mut self) {
        self.visible = !self.visible;
    }

    pub fn close(&mut self) {
        self.visible = false;
    }

    pub fn close_icon(&mut self) {
        self.close();
    }

    pub fn close_button(&mut self) {
        self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
