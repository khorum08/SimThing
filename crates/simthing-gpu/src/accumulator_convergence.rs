//! AccumulatorOp / driver-sim convergence analysis for structural link neighbor accumulation.
//!
//! Documents the exact generic capability gap between PROBATION `structural_link_accumulator`
//! smoke and canonical [`crate::accumulator_op`] / AO-WGSL-0 production execution.

/// Relative path (from repo root) to the convergence gap evidence report.
pub const ACCUMULATOR_CONVERGENCE_GAP_REPORT_REL: &str =
    "docs/tests/accumulator_driver_sim_convergence_0_results.md";

/// Neutral capability-gap statement: what AccumulatorOp needs for structural coupling rows.
pub const ACCUMULATOR_OP_MISSING_GENERIC_CAPABILITIES: &[&str] = &[
    "scenario-derived structural coupling rows (dense source index, dense target index)",
    "input scalar channel and output scalar channel per coupling row",
    "combine mode: checked exact sum (i32 or explicitly documented fixed-point integer semantics)",
    "overflow rejected or errored before GPU dispatch (matching PROBATION smoke contract)",
    "driver compile/assembly from SimThingScenarioSpec links into generic op plans",
    "sim tick/boundary lifecycle for accumulation dispatch (not Studio proof helpers)",
];

/// Target invariant for vertical-seed neighbor accumulation (canonical structural links).
pub const STRUCTURAL_NEIGHBOR_SUM_INVARIANT: &str =
    "for each canonical structural link (a,b): output[a] += input[b]; output[b] += input[a]";

/// Vertical-seed proof values preserved from GPU-LINK-ACCUMULATOR-SMOKE-0 / PR #756.
pub const VERTICAL_SEED_INPUT: [i32; 2] = [10, 20];
pub const VERTICAL_SEED_EXPECTED_OUTPUT: [i32; 2] = [20, 10];

/// Crate that should compile/assemble vertical-seed accumulation into generic op plans.
pub const DRIVER_STRUCTURAL_ACCUMULATOR_COMPILE_CRATE: &str = "simthing-driver";

/// Crate that should own tick/boundary lifecycle for accumulation dispatch.
pub const SIM_STRUCTURAL_ACCUMULATOR_TICK_CRATE: &str = "simthing-sim";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accumulator_convergence_gap_report_names_missing_generic_capability() {
        for capability in ACCUMULATOR_OP_MISSING_GENERIC_CAPABILITIES {
            assert!(
                !capability.is_empty(),
                "capability gap entries must be non-empty"
            );
        }
        let joined = ACCUMULATOR_OP_MISSING_GENERIC_CAPABILITIES.join(" ");
        let lower = joined.to_ascii_lowercase();
        assert!(lower.contains("dense"));
        assert!(lower.contains("checked"));
        assert!(lower.contains("channel"));
    }

    #[test]
    fn accumulator_convergence_gap_constants_use_no_domain_semantics() {
        const FORBIDDEN: &[&str] = &[
            "route",
            "predecessor",
            "pathfinding",
            "movement_order",
            "fleet",
            "faction",
            "owner",
            "border",
            "frontline",
            "combat",
            "economy",
            "diplomacy",
            "pirate",
        ];
        let corpus = [
            ACCUMULATOR_CONVERGENCE_GAP_REPORT_REL,
            STRUCTURAL_NEIGHBOR_SUM_INVARIANT,
            DRIVER_STRUCTURAL_ACCUMULATOR_COMPILE_CRATE,
            SIM_STRUCTURAL_ACCUMULATOR_TICK_CRATE,
        ]
        .into_iter()
        .chain(ACCUMULATOR_OP_MISSING_GENERIC_CAPABILITIES.iter().copied())
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase();
        for token in FORBIDDEN {
            assert!(
                !corpus.contains(token),
                "forbidden domain token {token} in convergence constants"
            );
        }
    }

    #[test]
    fn vertical_seed_invariant_values_preserved() {
        assert_eq!(VERTICAL_SEED_INPUT, [10, 20]);
        assert_eq!(VERTICAL_SEED_EXPECTED_OUTPUT, [20, 10]);
    }
}
