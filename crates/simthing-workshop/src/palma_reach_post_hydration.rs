//! TP-PALMA-REACH-0 — workshop-homed PALMA W/D reach over accepted STEAD fronts.
//!
//! Applied after generic `hydrate_scenario` + `apply_fronts_post_hydration`; impedance/reach
//! semantics live here, not in sealed engine crates.

use simthing_clausething::{
    build_palma_feedstock_from_region_field, HydratedScenarioPack,
    HydratedScenarioPalmaFeedstock,
};
use simthing_spec::spec::w_impedance_compose::{
    WImpedanceComposeProfileSpec, WImpedanceComposeSpec,
};
use simthing_spec::compile_w_impedance_compose_preview;

use crate::fronts_post_hydration::{
    apply_fronts_post_hydration, FrontsHydrationError, TpFrontsAuthoringReport,
    TP_FRONTS_CHOKE_OUTPUT_COL, TP_FRONTS_FIELD_OPERATOR_ID, TP_FRONTS_N_DIMS,
    TP_FRONTS_SOURCE_COL,
};

/// Workshop-local PALMA feedstock id for contested-border reach.
pub const TP_PALMA_FEEDSTOCK_ID: &str = "tp_contested_border_wd";

/// Composed impedance W column on the shared interleaved buffer.
pub const TP_PALMA_W_OUTPUT_COL: u32 = 3;

/// Resident PALMA reach D column on the shared interleaved buffer.
pub const TP_PALMA_D_OUTPUT_COL: u32 = 5;

/// Suppression counterbalance column (choke_b) for W composition.
pub const TP_PALMA_SUPPRESSION_COL: u32 = 1;

/// Weight on SaturatingFlux choke column — threat/choke/disruption increase impedance.
pub const TP_PALMA_W_WEIGHT_THREAT_CHOKE: f32 = 1.0;

/// Negative weight on suppression column — patrol presence counterbalances impedance.
pub const TP_PALMA_W_WEIGHT_SUPPRESSION: f32 = -0.75;

/// Min-plus relaxation iterations for the bounded 3×3 theater.
pub const TP_PALMA_MIN_PLUS_ITERATIONS: u32 = 12;

/// Floor added to base W before front-derived terms (numeric substrate only).
pub const TP_PALMA_BASE_W_FLOOR: f32 = 1.0;

#[derive(Debug, Clone)]
pub struct TpPalmaReachAuthoringReport {
    pub fronts: TpFrontsAuthoringReport,
    pub palma_feedstock: HydratedScenarioPalmaFeedstock,
    pub w_compose: WImpedanceComposeSpec,
    /// Reach seed cell (terran patrol anchor) as theater (row, col).
    pub reach_dest: (u32, u32),
}

#[derive(Debug, thiserror::Error)]
pub enum PalmaReachHydrationError {
    #[error(transparent)]
    Fronts(#[from] FrontsHydrationError),
    #[error("{0}")]
    Message(String),
}

/// Workshop-side PALMA reach authoring over accepted threat/suppression/disruption fronts.
pub fn apply_palma_reach_post_hydration(
    pack: &mut HydratedScenarioPack,
) -> Result<TpPalmaReachAuthoringReport, PalmaReachHydrationError> {
    let fronts = apply_fronts_post_hydration(pack)?;
    let field = &fronts.region_field;

    let palma = build_palma_feedstock_from_region_field(
        TP_PALMA_FEEDSTOCK_ID,
        TP_FRONTS_FIELD_OPERATOR_ID,
        TP_PALMA_W_OUTPUT_COL,
        TP_PALMA_D_OUTPUT_COL,
        field,
    )
    .map_err(|err| PalmaReachHydrationError::Message(err.to_string()))?;

    let w_compose = build_tp_palma_w_compose(&palma);
    compile_w_impedance_compose_preview(&w_compose).map_err(|err| {
        PalmaReachHydrationError::Message(format!("W impedance compose admission rejected: {err}"))
    })?;

    let reach_dest = palma_reach_dest_cell(&fronts.theater_cells)?;

    pack.palma_feedstock = Some(palma.clone());
    pack.w_impedance_compose = Some(w_compose.clone());

    Ok(TpPalmaReachAuthoringReport {
        fronts,
        palma_feedstock: palma,
        w_compose,
        reach_dest,
    })
}

/// Build workshop W composition: base pressure + choke threat + suppression counterbalance.
pub fn build_tp_palma_w_compose(palma: &HydratedScenarioPalmaFeedstock) -> WImpedanceComposeSpec {
    let choke_a = palma
        .choke_output_col
        .expect("PALMA feedstock requires saturating_flux choke_output_col");
    WImpedanceComposeSpec {
        width: palma.grid_size,
        height: palma.grid_size,
        n_dims: palma.n_dims,
        base_w_col: palma.source_col,
        choke_a_col: choke_a,
        choke_b_col: TP_PALMA_SUPPRESSION_COL,
        profiles: vec![WImpedanceComposeProfileSpec {
            weight_a: TP_PALMA_W_WEIGHT_THREAT_CHOKE,
            weight_b: TP_PALMA_W_WEIGHT_SUPPRESSION,
            output_w_col: palma.w_output_col,
        }],
    }
}

/// Terran patrol anchor is the min-plus destination (D = 0); reach radiates outward.
pub fn palma_reach_dest_cell(
    theater_cells: &[crate::fronts_post_hydration::TpFrontsTheaterCell],
) -> Result<(u32, u32), PalmaReachHydrationError> {
    let terran = theater_cells
        .iter()
        .find(|cell| cell.owner == "terran")
        .ok_or_else(|| {
            PalmaReachHydrationError::Message(
                "PALMA reach requires a terran theater cell for reach dest".into(),
            )
        })?;
    Ok((terran.theater_row, terran.theater_col))
}

/// Numeric pressure seed for column writes (workshop oracle only).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PalmaPressureSeed {
    pub row: u32,
    pub col: u32,
    pub value: f32,
}

/// CPU oracle: write pressure seeds into an arbitrary interleaved column.
pub fn write_pressure_seeds_to_column(
    values: &mut [f32],
    seeds: &[PalmaPressureSeed],
    grid_size: u32,
    n_dims: u32,
    col: u32,
) {
    let idx = |slot: u32, column: u32| (slot * n_dims + column) as usize;
    for seed in seeds {
        let slot = seed.row * grid_size + seed.col;
        values[idx(slot, col)] = seed.value;
    }
}

/// Apply numeric base-W floor before W composition.
pub fn apply_base_w_floor(
    values: &mut [f32],
    grid_size: u32,
    n_dims: u32,
    base_col: u32,
    floor: f32,
) {
    let cells = grid_size * grid_size;
    let idx = |slot: u32, column: u32| (slot * n_dims + column) as usize;
    for slot in 0..cells {
        let i = idx(slot, base_col);
        if values[i] < floor {
            values[i] = floor;
        }
    }
}

/// Oracle: higher threat/choke/disruption increases W; suppression counterbalances via choke_b.
pub fn impedance_w_composition_oracle(
    values: &[f32],
    grid_size: u32,
    n_dims: u32,
    w_col: u32,
    suppression_col: u32,
    theater_cells: &[crate::fronts_post_hydration::TpFrontsTheaterCell],
) -> Result<(), String> {
    let idx = |slot: u32, col: u32| (slot * n_dims + col) as usize;

    let terran = theater_cells
        .iter()
        .find(|cell| cell.owner == "terran")
        .ok_or_else(|| "missing terran theater cell".to_string())?;
    let pirate = theater_cells
        .iter()
        .find(|cell| cell.owner == "pirate")
        .ok_or_else(|| "missing pirate theater cell".to_string())?;

    let terran_slot = terran.theater_row * grid_size + terran.theater_col;
    let pirate_slot = pirate.theater_row * grid_size + pirate.theater_col;

    let w_terran = values
        .get(idx(terran_slot, w_col))
        .copied()
        .ok_or_else(|| "missing terran W".to_string())?;
    let w_pirate = values
        .get(idx(pirate_slot, w_col))
        .copied()
        .ok_or_else(|| "missing pirate W".to_string())?;

    if !w_terran.is_finite() || !w_pirate.is_finite() {
        return Err("composed W must be finite".into());
    }
    if w_pirate <= w_terran {
        return Err(format!(
            "pirate-side composed W must exceed terran-side W: terran={w_terran} pirate={w_pirate}"
        ));
    }

    let suppression_terran = values
        .get(idx(terran_slot, suppression_col))
        .copied()
        .unwrap_or(0.0);
    if suppression_terran <= 0.0 {
        return Err(format!(
            "terran suppression column must be positive: got {suppression_terran}"
        ));
    }
    Ok(())
}

/// Local gradient probe: lowest-D N4 neighbor direction (no route object).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PalmaReachGradientStep {
    pub from_slot: u32,
    pub to_slot: u32,
    pub sampled_d: f32,
}

/// Sample the lowest finite D among four neighbors — gradient-following data only.
pub fn palma_reach_gradient_probe(
    d_flat: &[f32],
    width: u32,
    height: u32,
    from_slot: u32,
) -> Option<PalmaReachGradientStep> {
    let from_x = from_slot % width;
    let from_y = from_slot / width;
    let mut best: Option<PalmaReachGradientStep> = None;
    let candidates = [
        (from_x.wrapping_sub(1), from_y),
        (from_x + 1, from_y),
        (from_x, from_y.wrapping_sub(1)),
        (from_x, from_y + 1),
    ];
    for (x, y) in candidates {
        if x >= width || y >= height {
            continue;
        }
        let to_slot = y * width + x;
        let v = d_flat[to_slot as usize];
        if !v.is_finite() {
            continue;
        }
        let step = PalmaReachGradientStep {
            from_slot,
            to_slot,
            sampled_d: v,
        };
        if best.as_ref().is_none_or(|b| v < b.sampled_d) {
            best = Some(step);
        }
    }
    best
}

/// Expose front column indices for workshop proofs.
pub fn palma_front_source_col() -> u32 {
    TP_FRONTS_SOURCE_COL
}

pub fn palma_front_choke_col() -> u32 {
    TP_FRONTS_CHOKE_OUTPUT_COL
}

pub fn palma_front_n_dims() -> u32 {
    TP_FRONTS_N_DIMS
}