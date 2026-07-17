//! Canonical TP child→ancestor reduce-up golden (RF-1 → RF-4 OVL target).
//!
//! Scenario-flavored (Terran-Pirate field economy) — lives in workshop per §12
//! workshop-candidate-homing. The generic conservation oracle lives in
//! `simthing-driver::rf_conservation_oracle`; this golden is the **expected
//! ancestor/Owner aggregate** computed analytically from the ADR conservation
//! policy + the authored TP economy in `scenarios/terran_pirate_galaxy.clause`.
//!
//! It does **not** call `owner_silo_recursive_rf_source` or the recursive
//! `runtime_rf_tick_source` branch. RF-2 will execute recursive reduce-up; RF-4
//! closes Owner OVL by matching this golden.

/// Authored TP field-economy constants (12.8 clause data — not invented rates).
///
/// From `scenarios/terran_pirate_galaxy.clause` `field_economy = tp_economy`:
/// - `shipyard_minerals.amount = 100`
/// - `shipyard_factory`: input minerals unit_cost 5, output hulls, throttle 4
/// - `terran_minerals` silo: current 40 → stockpile transfer amount 40
/// - `pirate_raid_presence.amount = 8`
/// - owner policy overlays: terran expansion mult 1.15, manufacturing add 0.25,
///   pirate disruption mult 1.35
pub mod authored {
    pub const SHIPYARD_MINERALS_AMOUNT: f32 = 100.0;
    pub const FACTORY_MINERALS_UNIT_COST: f32 = 5.0;
    pub const FACTORY_THROTTLE_MAX_PER_TICK: u32 = 4;
    pub const TERRAN_SILO_CURRENT: f32 = 40.0;
    pub const TERRAN_SILO_STOCKPILE_TRANSFER: f32 = 40.0;
    pub const PIRATE_DISRUPTION_PRESENCE: f32 = 8.0;
    pub const TERRAN_EXPANSION_POLICY_MULT: f32 = 1.15;
    pub const TERRAN_MANUFACTURING_POLICY_ADD: f32 = 0.25;
    pub const PIRATE_DISRUPTION_POLICY_MULT: f32 = 1.35;
}

/// Expected ancestor/Owner aggregates after one recursive reduce-up tick of the
/// authored TP economy, derived closed-form from the ADR (not from RF-2).
#[derive(Clone, Debug, PartialEq)]
pub struct TpReduceUpGolden {
    /// Owner terran minerals stockpile after the silo current→stockpile transfer.
    pub terran_owner_minerals_stockpile: f32,
    /// Hulls produced at terran_shipyard this tick (recipe emit under throttle).
    pub terran_shipyard_hulls_emitted: f32,
    /// Minerals remaining at shipyard after recipe debit (exact recipe conservation).
    pub terran_shipyard_minerals_after_recipe: f32,
    /// Pirate disruption presence after owner-policy Multiply overlay on the field.
    pub pirate_owner_disruption_aggregate: f32,
    /// Recipe emit_count used (ADR per-recipe exact identity).
    pub factory_emit_count: u32,
}

/// Analytically compute the RF-4 reduce-up golden from authored TP economy.
///
/// Derivation (ADR + clause):
/// 1. **Recipe exact** (`MinAcrossInputs + SubtractFromAllInputs`):
///    `emit_count = min(floor(minerals / unit_cost), throttle) = min(floor(100/5), 4) = 4`
///    `ΔNeed_minerals = −emit_count × 5 = −20`
///    remaining minerals = 100 − 20 = 80
///    hulls emitted = emit_count = 4 (target Amount += emit_count)
/// 2. **Owner silo transfer** (discrete source-debit, exact):
///    stockpile += 40; current provides the transfer mass.
/// 3. **Owner aggregate / policy overlay** (overlay Multiply on pirate disruption):
///    pirate disruption field 8 × 1.35 policy mult → 10.8 aggregate observed at
///    the owner-policy-weighted readout (overlay stack, not a new primitive).
///
/// Continuous hierarchical allocator residual is O(ε·n) and integrates into
/// Balance; this golden records the exact discrete + recipe faces that RF-4
/// telemetry must show on the Owner/ancestor side once RF-2 executes reduce-up.
pub fn compute_tp_reduce_up_golden() -> TpReduceUpGolden {
    use authored::*;

    let max_by_input = (SHIPYARD_MINERALS_AMOUNT / FACTORY_MINERALS_UNIT_COST).floor() as u32;
    let emit_count = max_by_input.min(FACTORY_THROTTLE_MAX_PER_TICK);
    let minerals_debit = (emit_count as f32) * FACTORY_MINERALS_UNIT_COST;
    // Per-recipe exact: Σ ΔNeed + emit × Σ c = −minerals_debit + emit × unit_cost = 0
    debug_assert!(((-minerals_debit) + (emit_count as f32) * FACTORY_MINERALS_UNIT_COST).abs() == 0.0);

    TpReduceUpGolden {
        terran_owner_minerals_stockpile: TERRAN_SILO_STOCKPILE_TRANSFER,
        terran_shipyard_hulls_emitted: emit_count as f32,
        terran_shipyard_minerals_after_recipe: SHIPYARD_MINERALS_AMOUNT - minerals_debit,
        pirate_owner_disruption_aggregate: PIRATE_DISRUPTION_PRESENCE * PIRATE_DISRUPTION_POLICY_MULT,
        factory_emit_count: emit_count,
    }
}

/// Recipe observation that the conservation oracle must accept as exact fo
/// the canonical TP factory tick (pairs with `check_recipe_exact`).
pub fn tp_factory_recipe_observation() -> (Vec<f32>, Vec<f32>, u32) {
    let g = compute_tp_reduce_up_golden();
    let unit_cost = authored::FACTORY_MINERALS_UNIT_COST;
    let need_delta = -(g.factory_emit_count as f32) * unit_cost;
    (vec![need_delta], vec![unit_cost], g.factory_emit_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn golden_matches_authored_clause_rates() {
        let g = compute_tp_reduce_up_golden();
        assert_eq!(g.factory_emit_count, 4);
        assert_eq!(g.terran_shipyard_hulls_emitted, 4.0);
        assert_eq!(g.terran_shipyard_minerals_after_recipe, 80.0);
        assert_eq!(g.terran_owner_minerals_stockpile, 40.0);
        assert_eq!(g.pirate_owner_disruption_aggregate, 8.0 * 1.35);
    }

    #[test]
    fn golden_recipe_satisfies_adr_exact_identity() {
        let (need, costs, emit) = tp_factory_recipe_observation();
        let sum_need: f32 = need.iter().sum();
        let sum_c: f32 = costs.iter().sum();
        assert_eq!(sum_need + (emit as f32) * sum_c, 0.0);
    }
}
