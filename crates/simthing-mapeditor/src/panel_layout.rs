//! Pure floating left-panel geometry for SimThing Studio.

/// Resting panel width as a fraction of screen width.
pub const PANEL_WIDTH_FRAC: f32 = 0.20;
/// Minimum margin from viewport edges.
pub const PANEL_MARGIN_FRAC: f32 = 0.03;
/// Minimum usable control width before auto-collapse.
pub const REQUIRED_MIN_CONTROL_WIDTH_PX: f32 = 320.0;
/// Minimum corner radius in pixels.
pub const MIN_CORNER_RADIUS_PX: f32 = 8.0;
/// Corner radius as a fraction of panel width.
pub const CORNER_RADIUS_FRAC_OF_WIDTH: f32 = 0.05;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatingPanelLayout {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub corner_radius: f32,
    pub collapsed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CollapsedPanelTab {
    pub x: f32,
    pub y: f32,
}

pub fn resting_panel_width(screen_width: f32) -> f32 {
    screen_width * PANEL_WIDTH_FRAC
}

pub fn panel_margin(screen_width: f32, screen_height: f32) -> (f32, f32) {
    (
        screen_width * PANEL_MARGIN_FRAC,
        screen_height * PANEL_MARGIN_FRAC,
    )
}

pub fn corner_radius_for_panel_width(panel_width: f32) -> f32 {
    (panel_width * CORNER_RADIUS_FRAC_OF_WIDTH).max(MIN_CORNER_RADIUS_PX)
}

pub fn should_auto_collapse_panel(screen_width: f32) -> bool {
    resting_panel_width(screen_width) < REQUIRED_MIN_CONTROL_WIDTH_PX
}

pub fn compute_floating_panel_layout(
    screen_width: f32,
    screen_height: f32,
    collapsed: bool,
) -> FloatingPanelLayout {
    let (margin_x, margin_y) = panel_margin(screen_width, screen_height);
    let width = resting_panel_width(screen_width);
    let height = screen_height - margin_y * 2.0;
    FloatingPanelLayout {
        x: margin_x,
        y: margin_y,
        width,
        height,
        corner_radius: corner_radius_for_panel_width(width),
        collapsed,
    }
}

pub fn compute_collapsed_panel_tab(screen_width: f32, screen_height: f32) -> CollapsedPanelTab {
    let (margin_x, margin_y) = panel_margin(screen_width, screen_height);
    CollapsedPanelTab {
        x: margin_x,
        y: margin_y.max(48.0),
    }
}

pub fn left_panel_title(galaxy_name: Option<&str>) -> String {
    galaxy_name.unwrap_or("").to_string()
}

pub fn uses_floating_area_not_docked_sidebar(layout: &FloatingPanelLayout) -> bool {
    layout.x > 0.0 && layout.y > 0.0
}

pub fn left_panel_contains_point(layout: &FloatingPanelLayout, x: f32, y: f32) -> bool {
    x >= layout.x && x <= layout.x + layout.width && y >= layout.y && y <= layout.y + layout.height
}

pub fn right_panel_rect(screen_width: f32, screen_height: f32) -> (f32, f32, f32, f32) {
    let width = 320.0;
    let (margin_x, margin_y) = panel_margin(screen_width, screen_height);
    let x = screen_width - width - margin_x;
    let y = margin_y.max(48.0);
    (x, y, width, screen_height - y - margin_y)
}

pub fn right_panel_contains_point(screen_width: f32, screen_height: f32, x: f32, y: f32) -> bool {
    let (rx, ry, rw, rh) = right_panel_rect(screen_width, screen_height);
    x >= rx && x <= rx + rw && y >= ry && y <= ry + rh
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCREEN_W: f32 = 1920.0;
    const SCREEN_H: f32 = 1080.0;

    #[test]
    fn left_panel_resting_width_is_twenty_percent() {
        assert_eq!(resting_panel_width(SCREEN_W), SCREEN_W * 0.20);
    }

    #[test]
    fn left_panel_never_exceeds_twenty_percent_width() {
        let layout = compute_floating_panel_layout(SCREEN_W, SCREEN_H, false);
        assert!((layout.width - SCREEN_W * 0.20).abs() < f32::EPSILON);
        assert!(layout.width <= SCREEN_W * 0.20 + f32::EPSILON);
    }

    #[test]
    fn left_panel_has_three_percent_screen_margin() {
        let layout = compute_floating_panel_layout(SCREEN_W, SCREEN_H, false);
        let (margin_x, margin_y) = panel_margin(SCREEN_W, SCREEN_H);
        assert!((layout.x - margin_x).abs() < f32::EPSILON);
        assert!((layout.y - margin_y).abs() < f32::EPSILON);
        assert!((margin_x - SCREEN_W * 0.03).abs() < f32::EPSILON);
        assert!((margin_y - SCREEN_H * 0.03).abs() < f32::EPSILON);
    }

    #[test]
    fn left_panel_uses_floating_area_not_docked_sidebar() {
        let layout = compute_floating_panel_layout(SCREEN_W, SCREEN_H, false);
        assert!(uses_floating_area_not_docked_sidebar(&layout));
        assert!(layout.x >= SCREEN_W * 0.03);
        assert!(layout.y >= SCREEN_H * 0.03);
    }

    #[test]
    fn left_panel_corner_radius_is_five_percent_of_panel_width() {
        let layout = compute_floating_panel_layout(SCREEN_W, SCREEN_H, false);
        let expected = (layout.width * 0.05).max(MIN_CORNER_RADIUS_PX);
        assert!((layout.corner_radius - expected).abs() < f32::EPSILON);
    }

    #[test]
    fn left_panel_collapses_when_twenty_percent_width_is_too_narrow() {
        assert!(should_auto_collapse_panel(1400.0)); // 280px < 320
        assert!(!should_auto_collapse_panel(SCREEN_W)); // 384px >= 320
    }

    #[test]
    fn collapsed_left_panel_respects_three_percent_margin() {
        let tab = compute_collapsed_panel_tab(SCREEN_W, SCREEN_H);
        let (margin_x, margin_y) = panel_margin(SCREEN_W, SCREEN_H);
        assert!((tab.x - margin_x).abs() < f32::EPSILON);
        assert!(tab.y >= margin_y);
    }

    #[test]
    fn left_panel_title_empty_before_generation() {
        assert!(left_panel_title(None).is_empty());
        assert!(left_panel_title(Some("")).is_empty());
    }

    #[test]
    fn left_panel_title_uses_galaxy_name_after_generation() {
        assert_eq!(
            left_panel_title(Some("Unnamed 2-Armed Spiral")),
            "Unnamed 2-Armed Spiral"
        );
    }

    #[test]
    fn left_panel_auto_collapses_when_min_pixels_exceeds_twenty_percent_width() {
        assert!(should_auto_collapse_panel(1500.0)); // 300 < 320
    }

    #[test]
    fn left_panel_does_not_auto_collapse_at_normal_width() {
        assert!(!should_auto_collapse_panel(SCREEN_W));
        assert!(!should_auto_collapse_panel(1600.0)); // 320 == 320, not less
    }

    #[test]
    fn left_panel_hover_ignores_right_panel_area() {
        let layout = compute_floating_panel_layout(SCREEN_W, SCREEN_H, false);
        let (rx, ry, rw, rh) = right_panel_rect(SCREEN_W, SCREEN_H);
        let right_center_x = rx + rw * 0.5;
        let right_center_y = ry + rh * 0.5;
        assert!(!left_panel_contains_point(
            &layout,
            right_center_x,
            right_center_y
        ));
        assert!(right_panel_contains_point(
            SCREEN_W,
            SCREEN_H,
            right_center_x,
            right_center_y
        ));
    }

    #[test]
    fn left_panel_hover_ignores_warning_dialog_area() {
        let layout = compute_floating_panel_layout(SCREEN_W, SCREEN_H, false);
        let dialog_center_x = SCREEN_W * 0.5;
        let dialog_center_y = SCREEN_H * 0.5;
        assert!(!left_panel_contains_point(
            &layout,
            dialog_center_x,
            dialog_center_y
        ));
    }
}
