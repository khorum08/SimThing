//! STUDIO-DISRUPTION-SELECT-SCREEN-0 — selected-star disruption blur/tint screen.
//!
//! Presentation-only piecewise mapping over the admitted 12.2 disruption readout.
//! Composes with 11.6 owned-set brighten: disruption screen applies only to the
//! **actual** selected system id, never the co-owned highlight set.

use crate::studio_disruption_readout::StudioDisruptionReadoutMap;

/// Piecewise screen parameters for the selected star.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DisruptionSelectScreen {
    pub raw_disruption: f32,
    pub blur_scale: f32,
    pub red_fraction: f32,
}

impl DisruptionSelectScreen {
    pub const IDENTITY: Self = Self {
        raw_disruption: 0.0,
        blur_scale: 1.0,
        red_fraction: 0.0,
    };

    pub fn is_identity(self) -> bool {
        self.blur_scale == 1.0 && self.red_fraction == 0.0
    }
}

/// Exact piecewise-linear clamp:
/// - 0 → blur 1.0, red 0.0
/// - 50 → blur 2.0, red 0.5
/// - 100 → blur 5.0, red 1.0
/// - >100 clamps to the 100 endpoint
pub fn disruption_select_screen_from_raw(raw_disruption: f32) -> DisruptionSelectScreen {
    let d = if raw_disruption.is_finite() {
        raw_disruption.max(0.0)
    } else {
        0.0
    };
    let clamped = d.min(100.0);
    let (blur_scale, red_fraction) = if clamped <= 50.0 {
        let t = clamped / 50.0;
        (1.0 + t * 1.0, t * 0.5)
    } else {
        let t = (clamped - 50.0) / 50.0;
        (2.0 + t * 3.0, 0.5 + t * 0.5)
    };
    DisruptionSelectScreen {
        raw_disruption: d,
        blur_scale,
        red_fraction,
    }
}

/// Fail-soft read of admitted disruption for a generated system id.
pub fn raw_disruption_for_system(
    readout: &StudioDisruptionReadoutMap,
    system_id: u32,
) -> f32 {
    readout
        .by_system_id
        .get(&system_id)
        .map(|record| record.max_disruption_accreted())
        .filter(|value| value.is_finite())
        .unwrap_or(0.0)
        .max(0.0)
}

/// Screen for the actual selected system, or identity when nothing is selected.
pub fn selected_disruption_select_screen(
    selected_system_id: Option<u32>,
    readout: &StudioDisruptionReadoutMap,
) -> DisruptionSelectScreen {
    match selected_system_id {
        Some(system_id) => {
            disruption_select_screen_from_raw(raw_disruption_for_system(readout, system_id))
        }
        None => DisruptionSelectScreen::IDENTITY,
    }
}

/// Apply selected-star disruption blur on top of the 11.6 scale path.
/// Co-owned highlight stars keep `is_actual_selected = false` → identity mul.
pub fn compose_disruption_blur_scale(
    base_scale: f32,
    is_actual_selected: bool,
    screen: DisruptionSelectScreen,
) -> f32 {
    if is_actual_selected {
        base_scale * screen.blur_scale
    } else {
        base_scale
    }
}

/// Lerp RGB toward red by `red_fraction` when this star is the actual selection.
pub fn compose_disruption_rgb(
    base_rgb: (f32, f32, f32),
    is_actual_selected: bool,
    screen: DisruptionSelectScreen,
) -> (f32, f32, f32) {
    if !is_actual_selected || screen.red_fraction <= 0.0 {
        return base_rgb;
    }
    let t = screen.red_fraction.clamp(0.0, 1.0);
    (
        base_rgb.0 * (1.0 - t) + 1.0 * t,
        base_rgb.1 * (1.0 - t),
        base_rgb.2 * (1.0 - t),
    )
}

/// Quantize disruption for dirty-gate keys (milli-units).
pub fn quantize_disruption_milli(raw_disruption: f32) -> u32 {
    let d = if raw_disruption.is_finite() {
        raw_disruption.max(0.0).min(1000.0)
    } else {
        0.0
    };
    (d * 1000.0).round() as u32
}

pub fn quantize_blur_scale_milli(blur_scale: f32) -> u32 {
    let s = if blur_scale.is_finite() {
        blur_scale.max(0.0).min(100.0)
    } else {
        1.0
    };
    (s * 1000.0).round() as u32
}

pub fn quantize_red_fraction_milli(red_fraction: f32) -> u32 {
    let r = if red_fraction.is_finite() {
        red_fraction.clamp(0.0, 1.0)
    } else {
        0.0
    };
    (r * 1000.0).round() as u32
}

#[cfg(test)]
mod unit_smoke {
    use super::*;

    #[test]
    fn piecewise_breakpoints_and_clamp_are_exact() {
        let z = disruption_select_screen_from_raw(0.0);
        assert_eq!(z.blur_scale, 1.0);
        assert_eq!(z.red_fraction, 0.0);

        let mid = disruption_select_screen_from_raw(50.0);
        assert!((mid.blur_scale - 2.0).abs() < 1e-6);
        assert!((mid.red_fraction - 0.5).abs() < 1e-6);

        let hi = disruption_select_screen_from_raw(100.0);
        assert!((hi.blur_scale - 5.0).abs() < 1e-6);
        assert!((hi.red_fraction - 1.0).abs() < 1e-6);

        let over = disruption_select_screen_from_raw(150.0);
        assert!((over.blur_scale - 5.0).abs() < 1e-6);
        assert!((over.red_fraction - 1.0).abs() < 1e-6);
        assert_eq!(over.raw_disruption, 150.0);
    }

    #[test]
    fn deselect_is_identity_and_owned_set_does_not_inherit_screen() {
        let screen = disruption_select_screen_from_raw(100.0);
        assert_eq!(
            compose_disruption_blur_scale(1.85, false, screen),
            1.85,
            "co-owned brighten star must not receive disruption blur"
        );
        assert_eq!(
            compose_disruption_rgb((0.88, 0.95, 1.0), false, screen),
            (0.88, 0.95, 1.0)
        );
        let none = selected_disruption_select_screen(None, &StudioDisruptionReadoutMap::default());
        assert!(none.is_identity());
        assert_eq!(compose_disruption_blur_scale(1.85, true, none), 1.85);
    }

    #[test]
    fn actual_selected_composes_blur_and_red_over_brighten_base() {
        let screen = disruption_select_screen_from_raw(50.0);
        let scaled = compose_disruption_blur_scale(1.85, true, screen);
        assert!((scaled - 1.85 * 2.0).abs() < 1e-6);
        let rgb = compose_disruption_rgb((0.0, 1.0, 1.0), true, screen);
        assert!((rgb.0 - 0.5).abs() < 1e-6);
        assert!((rgb.1 - 0.5).abs() < 1e-6);
        assert!((rgb.2 - 0.5).abs() < 1e-6);
    }
}
