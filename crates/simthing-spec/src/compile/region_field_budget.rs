//! Designer/spec-layer RegionField VRAM budget preview (Phase M-first-slice).
//!
//! Estimates admitted RegionField memory footprint without GPU allocation.

use thiserror::Error;

/// Isolation policy for budget estimation only — does not enable runtime execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionFieldIsolationPolicyEstimate {
    /// First-slice single grid, no atlas.
    SingleGridNoAtlas,
    /// Future M-4A candidate; estimate-only.
    AlgebraicTileLocalMask,
    /// Physical gutter atlas fallback; estimate-only unless atlas implementation exists.
    PhysicalGutter { gutter: u32, horizon: u32 },
}

/// Inputs for a non-executing RegionField VRAM budget estimate.
#[derive(Clone, Debug, PartialEq)]
pub struct RegionFieldBudgetSpec {
    pub grid_size: u32,
    pub column_count: u32,
    /// Ping-pong typically requires 2× persistent field storage.
    pub buffer_multiplier: f32,
    pub copy_multiplier: f32,
    pub tile_count: u32,
    pub isolation_policy: RegionFieldIsolationPolicyEstimate,
    pub max_region_field_vram_bytes: Option<u64>,
}

/// Result of a budget estimate.
#[derive(Clone, Debug, PartialEq)]
pub struct RegionFieldBudgetEstimate {
    pub useful_cells: u64,
    pub bytes_per_cell: u64,
    pub base_bytes: u64,
    pub isolation_multiplier: f64,
    pub buffer_multiplier: f32,
    pub copy_multiplier: f32,
    pub estimated_bytes: u64,
}

/// Budget admission failure with designer-facing detail.
#[derive(Clone, Debug, PartialEq, Error)]
#[error(
    "region field VRAM budget exceeded: requested {requested_bytes} bytes, allowed {allowed_bytes} bytes (grid_size={grid_size}, columns={column_count}, isolation={isolation_policy:?})"
)]
pub struct RegionFieldBudgetError {
    pub requested_bytes: u64,
    pub allowed_bytes: u64,
    pub estimate: RegionFieldBudgetEstimate,
    pub grid_size: u32,
    pub column_count: u32,
    pub isolation_policy: RegionFieldIsolationPolicyEstimate,
}

impl RegionFieldBudgetError {
    pub fn suggested_fixes(&self) -> &'static [&'static str] {
        &[
            "reduce grid_size",
            "reduce column_count",
            "reduce copy_multiplier",
            "avoid physical gutter (use SingleGridNoAtlas or AlgebraicTileLocalMask estimate)",
        ]
    }
}

/// Compute isolation multiplier for estimate-only policies.
pub fn region_field_isolation_multiplier(
    policy: RegionFieldIsolationPolicyEstimate,
    grid_size: u32,
) -> f64 {
    match policy {
        RegionFieldIsolationPolicyEstimate::SingleGridNoAtlas
        | RegionFieldIsolationPolicyEstimate::AlgebraicTileLocalMask => 1.0,
        RegionFieldIsolationPolicyEstimate::PhysicalGutter { gutter, horizon: _ } => {
            let n = grid_size as f64;
            let g = gutter as f64;
            let packed = n + 2.0 * g;
            (packed * packed) / (n * n)
        }
    }
}

/// Estimate RegionField VRAM footprint without allocating GPU memory.
pub fn estimate_region_field_budget(
    spec: &RegionFieldBudgetSpec,
) -> Result<RegionFieldBudgetEstimate, RegionFieldBudgetError> {
    if spec.grid_size == 0 || spec.column_count == 0 || spec.tile_count == 0 {
        return Ok(RegionFieldBudgetEstimate {
            useful_cells: 0,
            bytes_per_cell: 0,
            base_bytes: 0,
            isolation_multiplier: 1.0,
            buffer_multiplier: spec.buffer_multiplier,
            copy_multiplier: spec.copy_multiplier,
            estimated_bytes: 0,
        });
    }

    let useful_cells = spec.tile_count as u64 * spec.grid_size as u64 * spec.grid_size as u64;
    let bytes_per_cell = spec.column_count as u64 * 4;
    let base_bytes = useful_cells * bytes_per_cell;
    let isolation_multiplier =
        region_field_isolation_multiplier(spec.isolation_policy, spec.grid_size);
    let estimated_f = base_bytes as f64
        * isolation_multiplier
        * f64::from(spec.buffer_multiplier)
        * f64::from(spec.copy_multiplier);
    let estimated_bytes = estimated_f.ceil() as u64;

    let estimate = RegionFieldBudgetEstimate {
        useful_cells,
        bytes_per_cell,
        base_bytes,
        isolation_multiplier,
        buffer_multiplier: spec.buffer_multiplier,
        copy_multiplier: spec.copy_multiplier,
        estimated_bytes,
    };

    if let Some(max_bytes) = spec.max_region_field_vram_bytes {
        if estimated_bytes > max_bytes {
            return Err(RegionFieldBudgetError {
                requested_bytes: estimated_bytes,
                allowed_bytes: max_bytes,
                estimate,
                grid_size: spec.grid_size,
                column_count: spec.column_count,
                isolation_policy: spec.isolation_policy,
            });
        }
    }

    Ok(estimate)
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn single_grid_multiplier_one() {
        let spec = RegionFieldBudgetSpec {
            grid_size: 10,
            column_count: 4,
            buffer_multiplier: 2.0,
            copy_multiplier: 1.0,
            tile_count: 1,
            isolation_policy: RegionFieldIsolationPolicyEstimate::SingleGridNoAtlas,
            max_region_field_vram_bytes: None,
        };
        let est = estimate_region_field_budget(&spec).unwrap();
        assert!((est.isolation_multiplier - 1.0).abs() < 1e-9);
        assert_eq!(est.useful_cells, 100);
        assert_eq!(est.estimated_bytes, 100 * 4 * 4 * 2);
    }

    #[test]
    fn physical_gutter_n10_h8() {
        let mult = region_field_isolation_multiplier(
            RegionFieldIsolationPolicyEstimate::PhysicalGutter {
                gutter: 8,
                horizon: 8,
            },
            10,
        );
        assert!((mult - 6.76).abs() < 0.01);
    }

    #[test]
    fn over_budget_rejects() {
        let spec = RegionFieldBudgetSpec {
            grid_size: 32,
            column_count: 16,
            buffer_multiplier: 2.0,
            copy_multiplier: 1.0,
            tile_count: 1,
            isolation_policy: RegionFieldIsolationPolicyEstimate::SingleGridNoAtlas,
            max_region_field_vram_bytes: Some(1024),
        };
        let err = estimate_region_field_budget(&spec).unwrap_err();
        assert!(err.requested_bytes > err.allowed_bytes);
        assert!(!err.suggested_fixes().is_empty());
    }
}
