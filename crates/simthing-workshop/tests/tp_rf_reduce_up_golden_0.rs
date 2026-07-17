//! RF-CONSERVATION-ORACLE-0 — TP child→ancestor reduce-up golden (workshop-homed).
//!
//! §12: scenario-flavored golden lives in workshop; generic oracle is sealed engine.

use simthing_driver::{check_recipe_exact, RecipeInvocationObservation};
use simthing_workshop::tp_rf_reduce_up_golden::{
    authored, compute_tp_reduce_up_golden, tp_factory_recipe_observation,
};

/// RF-4 OVL target: expected Owner/ancestor aggregates from ADR + authored TP economy.
#[test]
fn canonical_tp_reduce_up_golden_is_analytically_derived() {
    let g = compute_tp_reduce_up_golden();
    assert_eq!(g.factory_emit_count, authored::FACTORY_THROTTLE_MAX_PER_TICK);
    assert_eq!(
        g.terran_shipyard_hulls_emitted,
        authored::FACTORY_THROTTLE_MAX_PER_TICK as f32
    );
    assert_eq!(
        g.terran_shipyard_minerals_after_recipe,
        authored::SHIPYARD_MINERALS_AMOUNT
            - (g.factory_emit_count as f32) * authored::FACTORY_MINERALS_UNIT_COST
    );
    assert_eq!(
        g.terran_owner_minerals_stockpile,
        authored::TERRAN_SILO_STOCKPILE_TRANSFER
    );
    assert_eq!(
        g.pirate_owner_disruption_aggregate,
        authored::PIRATE_DISRUPTION_PRESENCE * authored::PIRATE_DISRUPTION_POLICY_MULT
    );
}

/// Golden recipe face is accepted by the independent conservation oracle (exact).
#[test]
fn canonical_tp_factory_recipe_passes_conservation_oracle() {
    let (need_deltas, unit_costs, emit_count) = tp_factory_recipe_observation();
    let obs = RecipeInvocationObservation {
        need_deltas,
        unit_costs,
        emit_count,
    };
    assert!(
        check_recipe_exact(&obs).is_ok(),
        "TP factory recipe must satisfy ADR per-recipe exact identity"
    );
}
