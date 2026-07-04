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

}
