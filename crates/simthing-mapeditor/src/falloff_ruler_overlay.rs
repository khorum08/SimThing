//! Diagnostic egui overlay for the Studio visual high-horizon falloff ruler (presentation only).

use bevy_egui::egui::{self, Align2, Color32, FontId, LayerId, Order, Pos2, Stroke};

use crate::star_render::{
    nameplate_effective_falloff_distance_percent, visual_horizon_ruler_point_at_progress_percent,
    VisualHorizonFalloffRuler,
};

const RULER_TICKS: &[(f32, &str)] = &[
    (0.0, "0%"),
    (10.0, "10%"),
    (25.0, "25%"),
    (50.0, "50%"),
    (65.0, "65%"),
    (75.0, "75%"),
    (100.0, "100% horizon"),
];

/// Inputs for the falloff ruler debug overlay.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FalloffRulerOverlayParams {
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub star_falloff_percent: f32,
    pub nameplate_relative_falloff_percent: f32,
}

impl FalloffRulerOverlayParams {
    pub fn effective_nameplate_falloff_percent(self) -> f32 {
        nameplate_effective_falloff_distance_percent(
            self.star_falloff_percent,
            self.nameplate_relative_falloff_percent,
        )
    }
}

/// Draw the bottom-center → high-horizon falloff ruler overlay using egui (diagnostic only).
pub fn draw_falloff_ruler_overlay(ctx: &egui::Context, params: FalloffRulerOverlayParams) {
    if params.viewport_width <= 0.0 || params.viewport_height <= 0.0 {
        return;
    }

    let ruler =
        VisualHorizonFalloffRuler::from_viewport(params.viewport_width, params.viewport_height);
    let base = Pos2::new(ruler.base_px[0], ruler.base_px[1]);
    let vanish = Pos2::new(ruler.vanishing_px[0], ruler.vanishing_px[1]);
    let effective = params.effective_nameplate_falloff_percent();

    let layer_id = LayerId::new(Order::Foreground, egui::Id::new("falloff_ruler_overlay"));
    let painter = ctx.layer_painter(layer_id);
    let screen = ctx.screen_rect();

    let ruler_stroke = Stroke::new(2.0, Color32::from_rgba_premultiplied(220, 235, 255, 140));
    painter.line_segment([base, vanish], ruler_stroke);
    painter.circle_filled(base, 4.0, Color32::from_rgb(120, 200, 255));
    painter.circle_filled(vanish, 4.0, Color32::from_rgb(255, 210, 120));

    let tick_color = Color32::from_rgba_premultiplied(200, 210, 230, 120);
    let guide_stroke = Stroke::new(1.0, Color32::from_rgba_premultiplied(180, 190, 210, 70));
    let cross_half = 18.0;

    for (pct, label) in RULER_TICKS {
        let pt = visual_horizon_ruler_point_at_progress_percent(&ruler, *pct);
        let y = pt[1];
        painter.line_segment(
            [Pos2::new(screen.min.x, y), Pos2::new(screen.max.x, y)],
            guide_stroke,
        );
        painter.line_segment(
            [
                Pos2::new(pt[0] - cross_half, y),
                Pos2::new(pt[0] + cross_half, y),
            ],
            Stroke::new(1.5, tick_color),
        );
        painter.text(
            Pos2::new(pt[0] + cross_half + 6.0, y),
            Align2::LEFT_CENTER,
            *label,
            FontId::proportional(12.0),
            tick_color,
        );
    }

    draw_emphasized_cut_line(
        &painter,
        &ruler,
        screen,
        params.star_falloff_percent,
        Color32::from_rgb(80, 210, 255),
        "Star falloff",
    );
    draw_emphasized_cut_line(
        &painter,
        &ruler,
        screen,
        effective,
        Color32::from_rgb(255, 120, 210),
        "Effective nameplate falloff",
    );

    let legend = format!(
        "Star falloff: {:.0}%\nNameplate relative falloff: {:.0}%\nEffective nameplate falloff: {:.1}%",
        params.star_falloff_percent,
        params.nameplate_relative_falloff_percent,
        effective,
    );
    let legend_pos = Pos2::new(screen.min.x + 12.0, screen.min.y + 12.0);
    let legend_font = FontId::proportional(13.0);
    let legend_galley = painter.layout(
        legend.clone(),
        legend_font.clone(),
        Color32::from_rgb(230, 240, 255),
        f32::INFINITY,
    );
    let legend_rect = egui::Rect::from_min_size(
        legend_pos - egui::vec2(4.0, 2.0),
        legend_galley.size() + egui::vec2(8.0, 4.0),
    );
    painter.rect_filled(
        legend_rect,
        4.0,
        Color32::from_rgba_premultiplied(10, 16, 28, 180),
    );
    painter.text(
        legend_pos,
        Align2::LEFT_TOP,
        legend,
        legend_font,
        Color32::from_rgb(230, 240, 255),
    );
}

fn draw_emphasized_cut_line(
    painter: &egui::Painter,
    ruler: &VisualHorizonFalloffRuler,
    screen: egui::Rect,
    progress_percent: f32,
    color: Color32,
    label: &str,
) {
    let pt = visual_horizon_ruler_point_at_progress_percent(ruler, progress_percent);
    let y = pt[1];
    painter.line_segment(
        [Pos2::new(screen.min.x, y), Pos2::new(screen.max.x, y)],
        Stroke::new(2.5, color),
    );
    painter.text(
        Pos2::new(screen.max.x - 8.0, y),
        Align2::RIGHT_CENTER,
        format!("{label}: {progress_percent:.0}%"),
        FontId::proportional(13.0),
        color,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlay_params_effective_falloff_matches_formula() {
        let params = FalloffRulerOverlayParams {
            viewport_width: 1920.0,
            viewport_height: 1080.0,
            star_falloff_percent: 100.0,
            nameplate_relative_falloff_percent: 65.0,
        };
        assert!((params.effective_nameplate_falloff_percent() - 65.0).abs() < f32::EPSILON);
    }
}
