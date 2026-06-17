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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_unimplemented_action_returns_warning_dialog_model() {
        let dialog = unimplemented_action_response(StudioAction::Save);
        assert!(dialog.visible);
        assert!(dialog.message.contains("not implemented"));
        let preset = unimplemented_action_response(StudioAction::InactivePreset("static".into()));
        assert!(preset.message.contains("static"));
    }

    #[test]
    fn inactive_control_click_returns_warning_action() {
        let action = inactive_control_click_action("Layer toggles");
        assert_eq!(
            action,
            StudioAction::InactiveControl("Layer toggles".into())
        );
        let dialog = inactive_control_warning("Layer toggles");
        assert!(dialog.visible);
        assert!(dialog.message.contains("Layer toggles"));
    }

    #[test]
    fn inactive_preset_click_returns_warning_action() {
        let dialog = unimplemented_action_response(StudioAction::InactivePreset("disc".into()));
        assert!(dialog.visible);
        assert!(dialog.message.contains("disc"));
    }

    #[test]
    fn new_load_save_warning_actions_are_reachable() {
        for action in new_load_save_actions() {
            let dialog = unimplemented_action_response(action);
            assert!(dialog.visible);
            assert!(dialog.message.contains("not implemented"));
        }
    }

    #[test]
    fn settings_dialog_defaults_exist() {
        let dialog = SettingsDialogModel::new(
            false,
            [520.0, 96.0],
            StarFalloffSettings::default(),
            StarRenderMode::default(),
            crate::hyperlane_buckets::HyperlaneRenderSettings::default(),
        );
        assert!(!dialog.visible);
        assert_eq!(dialog.position, [520.0, 96.0]);
        assert_eq!(dialog.star_render, StarFalloffSettings::default());
        assert_eq!(dialog.star_render_mode, StarRenderMode::default());
        assert_eq!(
            dialog.hyperlane_render,
            crate::hyperlane_buckets::HyperlaneRenderSettings::default()
        );
    }

    #[test]
    fn settings_dialog_open_close_preserves_values() {
        let values = StarFalloffSettings {
            base_blur_radius: 0.37,
            falloff_distance_percent: 61.0,
            falloff_blur_radius_percent: 41.0,
            falloff_opacity_percent: 59.0,
        };
        let mut dialog = SettingsDialogModel::new(
            false,
            [520.0, 96.0],
            values,
            StarRenderMode::CrispCircle,
            crate::hyperlane_buckets::HyperlaneRenderSettings::default(),
        );
        dialog.open();
        dialog.close_button();
        dialog.open();
        assert!(dialog.visible);
        assert_eq!(dialog.star_render, values);
        assert_eq!(dialog.star_render_mode, StarRenderMode::CrispCircle);
    }

    #[test]
    fn settings_dialog_close_icon_hides_dialog() {
        let mut dialog = SettingsDialogModel::new(
            true,
            [520.0, 96.0],
            StarFalloffSettings::default(),
            StarRenderMode::default(),
            crate::hyperlane_buckets::HyperlaneRenderSettings::default(),
        );
        dialog.close_icon();
        assert!(!dialog.visible);
    }

    #[test]
    fn settings_dialog_close_button_hides_dialog() {
        let mut dialog = SettingsDialogModel::new(
            true,
            [520.0, 96.0],
            StarFalloffSettings::default(),
            StarRenderMode::default(),
            crate::hyperlane_buckets::HyperlaneRenderSettings::default(),
        );
        dialog.close_button();
        assert!(!dialog.visible);
    }

    #[test]
    fn settings_dialog_preserves_star_render_mode() {
        let mut dialog = SettingsDialogModel::new(
            false,
            [520.0, 96.0],
            StarFalloffSettings::default(),
            StarRenderMode::BloomStarburst,
            crate::hyperlane_buckets::HyperlaneRenderSettings::default(),
        );
        dialog.set_star_render_mode(StarRenderMode::CrispCircle);
        dialog.open();
        dialog.close_icon();
        dialog.open();
        assert_eq!(dialog.star_render_mode, StarRenderMode::CrispCircle);
    }

    #[test]
    fn settings_dialog_bottom_close_hides_dialog() {
        let mut dialog = SettingsDialogModel::new(
            true,
            [520.0, 96.0],
            StarFalloffSettings::default(),
            StarRenderMode::default(),
            crate::hyperlane_buckets::HyperlaneRenderSettings::default(),
        );
        dialog.close_button();
        assert!(!dialog.visible);
    }

    #[test]
    fn settings_dialog_close_paths_preserve_star_values() {
        let values = StarFalloffSettings {
            base_blur_radius: 0.37,
            falloff_distance_percent: 61.0,
            falloff_blur_radius_percent: 41.0,
            falloff_opacity_percent: 59.0,
        };
        let mut icon_dialog = SettingsDialogModel::new(
            true,
            [520.0, 96.0],
            values,
            StarRenderMode::CrispCircle,
            crate::hyperlane_buckets::HyperlaneRenderSettings::default(),
        );
        icon_dialog.close_icon();
        let mut button_dialog = icon_dialog;
        button_dialog.open();
        button_dialog.close_button();
        assert_eq!(icon_dialog.star_render, values);
        assert_eq!(button_dialog.star_render, values);
        assert_eq!(button_dialog.star_render_mode, StarRenderMode::CrispCircle);
    }

    #[test]
    fn settings_dialog_close_paths_preserve_hyperlane_values() {
        let hyperlane = crate::hyperlane_buckets::HyperlaneRenderSettings {
            base_thickness_percent_of_star: 14.0,
            base_opacity_percent: 38.0,
            falloff_distance_percent: 57.0,
            falloff_thickness_percent: 24.0,
            falloff_opacity_percent: 19.0,
        };
        let mut dialog = SettingsDialogModel::new(
            true,
            [520.0, 96.0],
            StarFalloffSettings::default(),
            StarRenderMode::default(),
            hyperlane,
        );
        dialog.close_icon();
        dialog.open();
        dialog.close_button();
        assert_eq!(dialog.hyperlane_render, hyperlane);
    }

    #[test]
    fn settings_dialog_close_paths_preserve_values() {
        let star = StarFalloffSettings {
            base_blur_radius: 0.42,
            ..Default::default()
        };
        let hyperlane = crate::hyperlane_buckets::HyperlaneRenderSettings {
            base_opacity_percent: 0.0,
            ..Default::default()
        };
        let mut dialog = SettingsDialogModel::new(
            true,
            [640.0, 120.0],
            star,
            StarRenderMode::CrispCircle,
            hyperlane,
        );
        dialog.close_icon();
        dialog.open();
        dialog.close_button();
        assert_eq!(dialog.position, [640.0, 120.0]);
        assert_eq!(dialog.star_render, star);
        assert_eq!(dialog.star_render_mode, StarRenderMode::CrispCircle);
        assert_eq!(dialog.hyperlane_render, hyperlane);
    }

    #[test]
    fn settings_dialog_reopen_restores_position_and_values() {
        let star = StarFalloffSettings {
            falloff_opacity_percent: 45.0,
            ..Default::default()
        };
        let hyperlane = crate::hyperlane_buckets::HyperlaneRenderSettings {
            falloff_thickness_percent: 33.0,
            ..Default::default()
        };
        let mut dialog = SettingsDialogModel::new(
            true,
            [720.0, 144.0],
            star,
            StarRenderMode::BloomStarburst,
            hyperlane,
        );
        dialog.close_icon();
        dialog.open();
        assert!(dialog.visible);
        assert_eq!(dialog.position, [720.0, 144.0]);
        assert_eq!(dialog.star_render, star);
        assert_eq!(dialog.hyperlane_render, hyperlane);
    }
}
