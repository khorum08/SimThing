pub mod commitments_post_hydration;
pub mod diplomacy_post_hydration;
pub mod fronts_post_hydration;
pub mod fleet_movement_post_hydration;
pub mod live_run_post_hydration;
pub mod palma_reach_post_hydration;
pub mod eml_phase5;
pub mod multitarget_replay;
mod multitarget_replay_report;
pub mod overlay_order;
mod overlay_order_report;
pub mod persistent_bench;
mod persistent_bench_report;
mod report;
pub mod transfer_contention;
mod transfer_contention_report;
pub mod typeface;
pub mod weighted_mean;

pub use commitments_post_hydration::{
    apply_commitments_post_hydration, commitment_property_key, compiled_faction_commitment,
    default_fronts_eml_weights,
    patch_personality_profile, personality_eml_weights, pirate_personality_profile,
    terran_personality_profile, CommitmentsHydrationError, TpCommitmentsAuthoringReport,
    TpFactionCommitmentSpec, TpPersonalityUrgencyProfile, TP_COMMITMENT_ATTACK_EVENT_KIND,
    TP_COMMITMENT_FORTIFY_EVENT_KIND, TP_COMMITMENT_PROPERTY_NAMESPACE, TP_COMMITMENT_TYPE_ATTACK,
    TP_COMMITMENT_TYPE_FORTIFY, TP_COMMITMENT_TYPE_RAID, TP_COMMITMENT_TYPE_REINFORCE,
    TP_COMMITMENT_TYPE_WITHDRAW, TP_COMMITMENT_WITHDRAW_EVENT_KIND, TP_PIRATE_RAID_EVENT_KIND,
    TP_PIRATE_RAID_THRESHOLD, TP_PIRATE_WEIGHT_PRESSURE, TP_PIRATE_WEIGHT_RESOURCE,
    TP_TERRAN_REINFORCE_EVENT_KIND, TP_TERRAN_REINFORCE_THRESHOLD, TP_TERRAN_WEIGHT_PRESSURE,
    TP_TERRAN_WEIGHT_RESOURCE,
};
pub use diplomacy_post_hydration::{
    apply_diplomacy_post_hydration, DiplomacyHydrationError, BASELINE_BORDER_DISTRUST_SURPLUS,
    HOSTILITY_COMMITMENT_EVENT_KIND, HOSTILITY_DISTRUST_THRESHOLD, TP_DISTRUST_RESOURCE_KEY,
};
pub use palma_reach_post_hydration::{
    apply_base_w_floor, apply_palma_reach_post_hydration, build_tp_palma_w_compose,
    impedance_w_composition_oracle, palma_front_choke_col, palma_front_n_dims,
    palma_front_source_col, palma_reach_dest_cell, palma_reach_gradient_probe,
    PalmaPressureSeed, PalmaReachGradientStep, PalmaReachHydrationError,
    TpPalmaReachAuthoringReport, TP_PALMA_BASE_W_FLOOR, TP_PALMA_D_OUTPUT_COL,
    TP_PALMA_FEEDSTOCK_ID, TP_PALMA_MIN_PLUS_ITERATIONS, TP_PALMA_SUPPRESSION_COL,
    TP_PALMA_W_OUTPUT_COL, TP_PALMA_W_WEIGHT_SUPPRESSION, TP_PALMA_W_WEIGHT_THREAT_CHOKE,
    write_pressure_seeds_to_column,
};
pub use fleet_movement_post_hydration::{
    apply_fleet_movement_post_hydration, arena_enrollment_matches_fleet_cell,
    fleet_movement_gradient_step, horizon_truncation_engages_oracle, init_fleet_arena_enrollment,
    init_fleet_movement_state, movement_cell_lookup, movement_grid_size, movement_horizon,
    movement_reach_dest, movement_source_col, simulate_fleet_movement_cpu, theater_manhattan,
    FleetMovementHydrationError, TpFleetArenaEnrollment, TpFleetMovementAuthoringReport,
    TpFleetMovementState, TpFleetTheaterCoord, TpMovementObservation, TP_MOVEMENT_FLEET_START,
    TP_MOVEMENT_GRID_SIZE, TP_MOVEMENT_HORIZON, TP_MOVEMENT_MIN_CELLS, TP_MOVEMENT_MIN_TICKS,
    TP_MOVEMENT_REACH_DEST, TP_MOVEMENT_TRUNCATION_SEED,
};
pub use fronts_post_hydration::{
    apply_fronts_post_hydration, apply_fronts_post_hydration_with_theater,
    collect_contested_border_systems, contested_border_settling_oracle, fronts_l3_urgency_col,
    FrontsHydrationError, TpFrontsAuthoringReport, TpFrontsTheaterCell, DEFAULT_DISRUPTION_INTRINSIC_RATE,
    DEFAULT_SUPPRESSION_INTRINSIC_RATE, DEFAULT_THREAT_INTRINSIC_RATE, TP_DISRUPTION_ARENA,
    TP_FRONTS_CHOKE_OUTPUT_COL, TP_FRONTS_DEFAULT_HORIZON, TP_FRONTS_FIELD_OPERATOR_ID,
    TP_FRONTS_N_DIMS, TP_FRONTS_SOURCE_COL, TP_FRONTS_WEIGHT_PRESSURE, TP_FRONTS_WEIGHT_RESOURCE,
    TP_SUPPRESSION_ARENA, TP_THREAT_ARENA,
};
pub use live_run_post_hydration::{
    apply_live_run_post_hydration, rf_emission_band_destroyed_ships, rf_num_ships_after_emission,
    validate_rebind_table, LiveRunHydrationError, TpLiveRunAuthoringReport,
    TpPlacementRebindEntry, TpRfCombatEconomicsReport, TpRfCombatShipFlow, TP_LIVE_RUN_MIN_TICKS,
    TP_LIVE_RUN_THEATER_GRID, TP_RF_COMBAT_DESTROYED_SHIPS_PROPERTY, TP_RF_COMBAT_DTK_PROPERTY,
    TP_RF_COMBAT_NUM_SHIPS_PROPERTY, TP_RF_COMBAT_PROPERTY_NAMESPACE,
};
pub mod weighted_mean_perf;
mod weighted_mean_perf_report;
mod weighted_mean_report;
