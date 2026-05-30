//! C-1 — 2000-Star Atlas Scale Model and Budget Gate (pure model, no production changes).
//!
//! This module provides a reusable, allocation-safe scale model for evaluating
//! whether the C-0 atlas path (algebraic tile-local mask G=0 preferred) scales
//! to a realistic 2000-star grand-strategy envelope under the active
//! V78AtlasVramBudget.
//!
//! It is intentionally **not** a production mapping runtime, does not allocate
//! giant atlases, and does not relax any admission guardrails.

use simthing_spec::{V78AtlasVramBudget, V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES};

/// The exact user-supplied 2000-star target envelope (immutable model input).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct C1AtlasScaleModel {
    pub starmap_width: u32,
    pub starmap_height: u32,
    pub star_count: u32,
    pub star_grid_width: u32,
    pub star_grid_height: u32,
    pub avg_planet_systems_per_star: u32,
    pub planet_system_grid_width: u32,
    pub planet_system_grid_height: u32,
    pub avg_satellites_per_planet_system: u32,
    pub body_surface_width: u32,
    pub body_surface_height: u32,
    pub n_dims: u32,
    /// Effective bytes per payload cell under algebraic mask (C-0 measured basis).
    /// C-0 report: 32,768 algebraic-mask bytes for 256 payload cells → 128 bytes/cell.
    pub effective_algebraic_bytes_per_payload_cell: u64,
}

impl Default for C1AtlasScaleModel {
    fn default() -> Self {
        Self {
            starmap_width: 200,
            starmap_height: 150,
            star_count: 2000,
            star_grid_width: 10,
            star_grid_height: 10,
            avg_planet_systems_per_star: 5,
            planet_system_grid_width: 10,
            planet_system_grid_height: 10,
            avg_satellites_per_planet_system: 5,
            body_surface_width: 10,
            body_surface_height: 10,
            n_dims: 4,
            effective_algebraic_bytes_per_payload_cell: 128,
        }
    }
}

impl C1AtlasScaleModel {
    pub fn new_2000_star_target() -> Self {
        Self::default()
    }

    // --- Derived counts (exact per handoff) ---

    pub fn starmap_cells(&self) -> u64 {
        self.starmap_width as u64 * self.starmap_height as u64
    }

    pub fn star_local_cells(&self) -> u64 {
        self.star_count as u64 * (self.star_grid_width as u64 * self.star_grid_height as u64)
    }

    pub fn planet_system_count(&self) -> u64 {
        self.star_count as u64 * self.avg_planet_systems_per_star as u64
    }

    pub fn planet_system_orbital_cells(&self) -> u64 {
        self.planet_system_count()
            * (self.planet_system_grid_width as u64 * self.planet_system_grid_height as u64)
    }

    pub fn satellite_count(&self) -> u64 {
        self.planet_system_count() * self.avg_satellites_per_planet_system as u64
    }

    pub fn surface_body_count(&self) -> u64 {
        // planets + satellites
        self.planet_system_count() + self.satellite_count()
    }

    pub fn surface_cells(&self) -> u64 {
        self.surface_body_count()
            * (self.body_surface_width as u64 * self.body_surface_height as u64)
    }

    pub fn total_dense_cells_if_all_resident(&self) -> u64 {
        self.starmap_cells()
            + self.star_local_cells()
            + self.planet_system_orbital_cells()
            + self.surface_cells()
    }

    // --- VRAM estimates using C-0 effective accounting ---

    /// All-resident algebraic mask (G=0) footprint using C-0 measured 128 bytes/cell effective.
    pub fn algebraic_mask_bytes(&self) -> u64 {
        self.total_dense_cells_if_all_resident()
            * self.effective_algebraic_bytes_per_payload_cell
    }

    pub fn algebraic_mask_gib(&self) -> f64 {
        self.algebraic_mask_bytes() as f64 / (1024.0 * 1024.0 * 1024.0)
    }

    /// Physical gutter fallback using the ratified 10×10 H=8 reference multiplier (~6.76x).
    /// Source: C-0 isolation policy ratification + design docs.
    pub fn physical_gutter_multiplier(&self) -> f64 {
        // 10×10 tile, H=8 reference from C-0 / atlas isolation note
        6.76
    }

    pub fn physical_gutter_bytes(&self) -> u64 {
        (self.algebraic_mask_bytes() as f64 * self.physical_gutter_multiplier()).ceil() as u64
    }

    pub fn physical_gutter_gib(&self) -> f64 {
        self.physical_gutter_bytes() as f64 / (1024.0 * 1024.0 * 1024.0)
    }

    // --- Budget comparison against active configurable budget ---

    pub fn active_default_budget_bytes(&self) -> u64 {
        V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES
    }

    pub fn active_default_budget_gib(&self) -> f64 {
        self.active_default_budget_bytes() as f64 / (1024.0 * 1024.0 * 1024.0)
    }

    pub fn algebraic_fits_default_budget(&self) -> bool {
        self.algebraic_mask_bytes() <= self.active_default_budget_bytes()
    }

    pub fn gutter_fits_default_budget(&self) -> bool {
        self.physical_gutter_bytes() <= self.active_default_budget_bytes()
    }

    pub fn algebraic_headroom_bytes(&self) -> i64 {
        self.active_default_budget_bytes() as i64 - self.algebraic_mask_bytes() as i64
    }

    pub fn algebraic_headroom_gib(&self) -> f64 {
        self.algebraic_headroom_bytes() as f64 / (1024.0 * 1024.0 * 1024.0)
    }

    /// Minimum budget required for physical gutter fallback (rounded up).
    pub fn minimum_budget_for_gutter_gib(&self) -> f64 {
        self.physical_gutter_gib().ceil()
    }

    // --- Posture / admission implications (for tests and report) ---

    pub fn requires_algebraic_mask_first(&self) -> bool {
        // Algebraic fits; gutter does not under default commodity profile.
        self.algebraic_fits_default_budget() && !self.gutter_fits_default_budget()
    }

    pub fn must_remain_sparse_or_cadenced(&self) -> bool {
        // Even algebraic at full 7.23M cells leaves limited headroom; real games
        // will have additional fields, ping-pong, command buffers, etc.
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c1_default_model_matches_user_2000_star_envelope() {
        let m = C1AtlasScaleModel::new_2000_star_target();
        assert_eq!(m.starmap_width, 200);
        assert_eq!(m.starmap_height, 150);
        assert_eq!(m.star_count, 2000);
        assert_eq!(m.star_grid_width, 10);
        assert_eq!(m.avg_planet_systems_per_star, 5);
        assert_eq!(m.avg_satellites_per_planet_system, 5);
        assert_eq!(m.effective_algebraic_bytes_per_payload_cell, 128);
    }

    #[test]
    fn c1_derives_exact_cell_counts_from_handoff() {
        let m = C1AtlasScaleModel::new_2000_star_target();
        assert_eq!(m.starmap_cells(), 30_000);
        assert_eq!(m.star_local_cells(), 200_000);
        assert_eq!(m.planet_system_count(), 10_000);
        assert_eq!(m.planet_system_orbital_cells(), 1_000_000);
        assert_eq!(m.satellite_count(), 50_000);
        assert_eq!(m.surface_body_count(), 60_000);
        assert_eq!(m.surface_cells(), 6_000_000);
        assert_eq!(m.total_dense_cells_if_all_resident(), 7_230_000);
    }

    #[test]
    fn c1_algebraic_mask_uses_c0_effective_128_bytes() {
        let m = C1AtlasScaleModel::new_2000_star_target();
        let algebraic = m.algebraic_mask_bytes();
        // 7,230,000 * 128
        assert_eq!(algebraic, 925_440_000);
        assert!((m.algebraic_mask_gib() - 0.862).abs() < 0.01);
    }

    #[test]
    fn c1_physical_gutter_uses_ratified_6_76x_multiplier() {
        let m = C1AtlasScaleModel::new_2000_star_target();
        assert!((m.physical_gutter_multiplier() - 6.76).abs() < 0.01);
        let gutter = m.physical_gutter_bytes();
        // ~6.255 GiB
        assert!(gutter > 6_000_000_000);
        assert!((m.physical_gutter_gib() - 5.826).abs() < 0.1);
    }

    #[test]
    fn c1_algebraic_fits_default_1p5_gib_budget() {
        let m = C1AtlasScaleModel::new_2000_star_target();
        assert!(m.algebraic_fits_default_budget());
        let headroom = m.algebraic_headroom_bytes();
        assert!(headroom > 600_000_000); // ~0.638 GiB headroom
    }

    #[test]
    fn c1_gutter_fallback_exceeds_default_budget() {
        let m = C1AtlasScaleModel::new_2000_star_target();
        assert!(!m.gutter_fits_default_budget());
        assert!(m.minimum_budget_for_gutter_gib() >= 5.83);
    }

    #[test]
    fn c1_budget_is_active_configurable_not_hard_cap() {
        let budget = V78AtlasVramBudget::default_1p5_gib();
        assert_eq!(budget.max_bytes, V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES);
        // The type itself encodes "configurable, no architectural hard cap"
        // (enforced in designer admission and C-0 support).
    }

    #[test]
    fn c1_model_enforces_posture_no_production_runtime() {
        // This test, combined with the required rg scans in the handoff report,
        // documents that C-1 performs pure modeling only.
        let m = C1AtlasScaleModel::new_2000_star_target();
        assert!(m.requires_algebraic_mask_first());
        assert!(m.must_remain_sparse_or_cadenced());
        // No production claims are made by this model.
    }
}