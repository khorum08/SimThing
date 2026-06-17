//! Pure floating left-panel geometry for SimThing Studio.

use bevy_egui::egui::{pos2, Rect};

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
pub const FLOATING_DIALOG_PANEL_GAP_PX: f32 = 8.0;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatingDialogBounds {
    pub viewport: Rect,
    pub left_panel: Option<Rect>,
    pub right_panel: Option<Rect>,
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

/// Estimated title/header block height reserved above the scrollable panel body.
pub const LEFT_PANEL_TITLE_BLOCK_ESTIMATE_PX: f32 = 48.0;

/// Floating left panel rectangle: `(x, y, width, height)`.
/// Bottom inset equals the left inset (`margin_x`), not the top inset.
pub fn left_panel_rect(screen_width: f32, screen_height: f32) -> (f32, f32, f32, f32) {
    let (margin_x, margin_y) = panel_margin(screen_width, screen_height);
    let width = resting_panel_width(screen_width);
    let x = margin_x;
    let y = margin_y;
    let height = screen_height - margin_y - margin_x;
    (x, y, width, height)
}

pub fn left_panel_bottom_y(screen_width: f32, screen_height: f32) -> f32 {
    let (margin_x, _) = panel_margin(screen_width, screen_height);
    screen_height - margin_x
}

pub fn left_panel_content_scroll_height(layout: &FloatingPanelLayout) -> f32 {
    (layout.height - LEFT_PANEL_TITLE_BLOCK_ESTIMATE_PX).max(64.0)
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
    let (x, y, width, height) = left_panel_rect(screen_width, screen_height);
    FloatingPanelLayout {
        x,
        y,
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

pub fn rect_from_xywh(x: f32, y: f32, width: f32, height: f32) -> Rect {
    Rect::from_min_max(pos2(x, y), pos2(x + width, y + height))
}

pub fn clamp_dialog_rect_away_from_panels(desired: Rect, bounds: &FloatingDialogBounds) -> Rect {
    let size = desired.size();
    let mut min_x = bounds.viewport.min.x;
    let mut max_x = bounds.viewport.max.x - size.x;
    let min_y = bounds.viewport.min.y;
    let max_y = bounds.viewport.max.y - size.y;

    if let Some(left) = bounds.left_panel {
        min_x = min_x.max(left.max.x + FLOATING_DIALOG_PANEL_GAP_PX);
    }
    if let Some(right) = bounds.right_panel {
        max_x = max_x.min(right.min.x - size.x - FLOATING_DIALOG_PANEL_GAP_PX);
    }
    if max_x < min_x {
        min_x = bounds.viewport.min.x;
        max_x = bounds.viewport.max.x - size.x;
    }

    let x = desired.min.x.clamp(min_x, max_x);
    let y = desired.min.y.clamp(min_y, max_y.max(min_y));
    Rect::from_min_size(pos2(x, y), size)
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
    fn left_panel_bottom_gap_matches_left_gap() {
        let layout = compute_floating_panel_layout(SCREEN_W, SCREEN_H, false);
        let (margin_x, _) = panel_margin(SCREEN_W, SCREEN_H);
        let bottom_y = layout.y + layout.height;
        assert!((bottom_y - left_panel_bottom_y(SCREEN_W, SCREEN_H)).abs() < 0.01);
        assert!((SCREEN_H - bottom_y - margin_x).abs() < 0.01);
        assert!((layout.x - margin_x).abs() < 0.01);
    }

    #[test]
    fn left_panel_bottom_gap_survives_resize() {
        for (w, h) in [(1280.0, 720.0), (1920.0, 1080.0), (2560.0, 1440.0)] {
            let layout = compute_floating_panel_layout(w, h, false);
            let (margin_x, _) = panel_margin(w, h);
            let bottom_gap = h - (layout.y + layout.height);
            assert!(
                (bottom_gap - margin_x).abs() < 0.01,
                "bottom gap {bottom_gap} != left gap {margin_x} at {w}x{h}"
            );
        }
    }

    #[test]
    fn left_panel_content_scrolls_inside_inset_panel() {
        let layout = compute_floating_panel_layout(SCREEN_W, SCREEN_H, false);
        let scroll_h = left_panel_content_scroll_height(&layout);
        assert!(scroll_h > 0.0);
        assert!(scroll_h < layout.height);
        assert!(scroll_h <= layout.height - LEFT_PANEL_TITLE_BLOCK_ESTIMATE_PX + f32::EPSILON);
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

    #[test]
    fn settings_dialog_drag_clamps_to_viewport() {
        let bounds = FloatingDialogBounds {
            viewport: rect_from_xywh(0.0, 0.0, 800.0, 600.0),
            left_panel: None,
            right_panel: None,
        };
        let desired = rect_from_xywh(760.0, 580.0, 120.0, 80.0);
        let clamped = clamp_dialog_rect_away_from_panels(desired, &bounds);
        assert_eq!(clamped.min.x, 680.0);
        assert_eq!(clamped.min.y, 520.0);
    }

    #[test]
    fn settings_dialog_drag_stops_at_left_panel_bounds() {
        let bounds = FloatingDialogBounds {
            viewport: rect_from_xywh(0.0, 0.0, 1000.0, 700.0),
            left_panel: Some(rect_from_xywh(30.0, 30.0, 220.0, 640.0)),
            right_panel: None,
        };
        let desired = rect_from_xywh(100.0, 80.0, 300.0, 240.0);
        let clamped = clamp_dialog_rect_away_from_panels(desired, &bounds);
        assert!(clamped.min.x >= 250.0 + FLOATING_DIALOG_PANEL_GAP_PX);
    }

    #[test]
    fn settings_dialog_drag_stops_at_right_panel_bounds() {
        let bounds = FloatingDialogBounds {
            viewport: rect_from_xywh(0.0, 0.0, 1200.0, 700.0),
            left_panel: None,
            right_panel: Some(rect_from_xywh(880.0, 48.0, 300.0, 620.0)),
        };
        let desired = rect_from_xywh(820.0, 80.0, 300.0, 240.0);
        let clamped = clamp_dialog_rect_away_from_panels(desired, &bounds);
        assert!(clamped.max.x <= 880.0 - FLOATING_DIALOG_PANEL_GAP_PX);
    }
}
