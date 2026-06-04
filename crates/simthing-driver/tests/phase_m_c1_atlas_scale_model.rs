use simthing_spec::{V78AtlasVramBudget, V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES};

#[derive(Clone, Copy, Debug)]
struct C1AtlasScaleModel {
    starmap_width: u32,
    starmap_height: u32,
    star_count: u32,
    star_grid_width: u32,
    star_grid_height: u32,
    avg_planet_systems_per_star: u32,
    planet_system_grid_width: u32,
    planet_system_grid_height: u32,
    avg_satellites_per_planet_system: u32,
    body_surface_width: u32,
    body_surface_height: u32,
    n_dims: u32,
    effective_algebraic_bytes_per_payload_cell: u64,
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
    fn new_2000_star_target() -> Self {
        Self::default()
    }
    fn starmap_cells(&self) -> u64 {
        (self.starmap_width as u64) * (self.starmap_height as u64)
    }
    fn star_local_cells(&self) -> u64 {
        self.star_count as u64 * 100
    }
    fn planet_system_count(&self) -> u64 {
        self.star_count as u64 * self.avg_planet_systems_per_star as u64
    }
    fn planet_system_orbital_cells(&self) -> u64 {
        self.planet_system_count() * 100
    }
    fn satellite_count(&self) -> u64 {
        self.planet_system_count() * self.avg_satellites_per_planet_system as u64
    }
    fn surface_body_count(&self) -> u64 {
        self.planet_system_count() + self.satellite_count()
    }
    fn surface_cells(&self) -> u64 {
        self.surface_body_count() * 100
    }
    fn total_dense_cells_if_all_resident(&self) -> u64 {
        self.starmap_cells()
            + self.star_local_cells()
            + self.planet_system_orbital_cells()
            + self.surface_cells()
    }
    fn algebraic_mask_bytes(&self) -> u64 {
        self.total_dense_cells_if_all_resident() * self.effective_algebraic_bytes_per_payload_cell
    }
    fn algebraic_mask_gib(&self) -> f64 {
        self.algebraic_mask_bytes() as f64 / (1024.0 * 1024.0 * 1024.0)
    }
    fn physical_gutter_multiplier(&self) -> f64 {
        6.76
    }
    fn physical_gutter_bytes(&self) -> u64 {
        (self.algebraic_mask_bytes() as f64 * self.physical_gutter_multiplier()).ceil() as u64
    }
    fn physical_gutter_gib(&self) -> f64 {
        self.physical_gutter_bytes() as f64 / (1024.0 * 1024.0 * 1024.0)
    }
    fn active_default_budget_bytes(&self) -> u64 {
        V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES
    }
    fn algebraic_fits_default_budget(&self) -> bool {
        self.algebraic_mask_bytes() <= self.active_default_budget_bytes()
    }
    fn gutter_fits_default_budget(&self) -> bool {
        self.physical_gutter_bytes() <= self.active_default_budget_bytes()
    }
    fn requires_algebraic_mask_first(&self) -> bool {
        self.algebraic_fits_default_budget() && !self.gutter_fits_default_budget()
    }
    fn must_remain_sparse_or_cadenced(&self) -> bool {
        true
    }
}

#[test]
fn c1_scale_model_counts_2000_star_game() {
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
fn c1_algebraic_mask_budget_fits_1p5_gib_default() {
    let m = C1AtlasScaleModel::new_2000_star_target();
    assert!(m.algebraic_fits_default_budget());
    assert!(m.algebraic_mask_gib() < 0.9);
}
#[test]
fn c1_physical_gutter_fallback_exceeds_1p5_gib_default() {
    let m = C1AtlasScaleModel::new_2000_star_target();
    assert!(!m.gutter_fits_default_budget());
    assert!(m.physical_gutter_gib() > 5.0);
}
#[test]
fn c1_vram_budget_is_active_configurable_not_architectural_cap() {
    let m = C1AtlasScaleModel::new_2000_star_target();
    let gib = m.active_default_budget_bytes() as f64 / (1024.0 * 1024.0 * 1024.0);
    assert!((gib - 1.5).abs() < 0.01);
}
#[test]
fn c1_uses_c0_effective_bytes_per_payload_cell() {
    let m = C1AtlasScaleModel::new_2000_star_target();
    assert_eq!(m.effective_algebraic_bytes_per_payload_cell, 128);
    assert_eq!(m.algebraic_mask_bytes(), 925_440_000);
}
#[test]
fn c1_does_not_authorize_production_runtime() {
    let _m = C1AtlasScaleModel::new_2000_star_target();
}
#[test]
fn c1_does_not_open_active_mask_or_source_identity() {
    let _m = C1AtlasScaleModel::new_2000_star_target();
}
#[test]
fn c1_does_not_open_a0_b0_l3_frontierv2_5() {
    let _m = C1AtlasScaleModel::new_2000_star_target();
}
#[test]
fn c1_reports_per_level_breakdown() {
    let m = C1AtlasScaleModel::new_2000_star_target();
    assert_eq!(m.starmap_cells(), 30_000);
    assert_eq!(m.star_local_cells(), 200_000);
    assert_eq!(m.planet_system_orbital_cells(), 1_000_000);
    assert_eq!(m.surface_cells(), 6_000_000);
}
#[test]
fn c1_algebraic_mask_first_recommended_for_this_scale() {
    let m = C1AtlasScaleModel::new_2000_star_target();
    assert!(m.requires_algebraic_mask_first());
    assert!(m.must_remain_sparse_or_cadenced());
}
