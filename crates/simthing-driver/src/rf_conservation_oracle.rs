//! Independent RF conservation oracle (RF-1 / RF-CONSERVATION-ORACLE-0).
//!
//! Closed-form checker derived from `docs/adr/resource_flow_substrate.md`
//! §"Conservation policy" — the three ADR invariants:
//!
//! 1. **Per-recipe (exact):** `Σ_j ΔNeed_j + emit_count × Σ_j c_j = 0`
//! 2. **Per-allocator (approximate-deterministic):**
//!    `|Σ_i disbursed(I→C_i) − budget(I)| ≤ O(ε × n_children)`;
//!    residual integrates into the parent `Balance` via existing `governed_by`
//! 3. **Per-arena (structural):** intrinsic + inbound coupling =
//!    leaf allocations + Balance changes + emission consumption;
//!    no orphan participants
//!
//! # Independence fence (anti-cosplay)
//!
//! This module derives the invariants itself. It must **not** import or call
//! `owner_silo_recursive_rf_source` or the recursive branch of
//! `runtime_rf_tick_source` — those become the EXECUTED path under RF-2, so an
//! oracle built on them is circular and cannot falsify RF-2.
//!
//! No new grammar / kernel / WGSL / GPU primitive: conservation checks are pure
//! arithmetic over arena snapshots that ride existing AccumulatorOp /
//! `governed_by` observations.

use std::collections::{HashMap, HashSet};

/// Machine-epsilon residual bound for one allocator step (ADR O(ε × n_children)).
///
/// Per-child `budget × w_i / weight_sum` is independent f32 arithmetic; the sum of
/// quotients need not equal `budget` exactly even when `Σ w_i = weight_sum`. The
/// bound scales with `n` and the budget magnitude (the residual is relative to
/// the quotient operands).
pub fn allocator_eps_bound(n_children: usize, budget: f32) -> f32 {
    let n = n_children.max(1) as f32;
    // Constant factor covers mul+div residual accumulation per child share.
    8.0 * f32::EPSILON * n * budget.abs().max(1.0)
}

// ---------------------------------------------------------------------------
// (a) Per-recipe exact
// ---------------------------------------------------------------------------

/// One conjunctive recipe invocation observation.
///
/// `need_deltas[j]` is the measured change on input `j` for this invocation
/// (negative when the recipe debits). `unit_costs[j]` is `c_j`.
#[derive(Clone, Debug)]
pub struct RecipeInvocationObservation {
    pub need_deltas: Vec<f32>,
    pub unit_costs: Vec<f32>,
    pub emit_count: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecipeConservationViolation {
    pub sum_need_deltas: f32,
    pub emit_times_sum_costs: f32,
    pub residual: f32,
}

/// Exact per-recipe conservation: `Σ ΔNeed + emit_count × Σ c = 0` (bit-exact f32).
pub fn check_recipe_exact(
    obs: &RecipeInvocationObservation,
) -> Result<(), RecipeConservationViolation> {
    assert_eq!(
        obs.need_deltas.len(),
        obs.unit_costs.len(),
        "recipe observation must pair each ΔNeed with its unit cost"
    );
    let sum_need: f32 = obs.need_deltas.iter().copied().sum();
    let sum_costs: f32 = obs.unit_costs.iter().copied().sum();
    let emit_times = (obs.emit_count as f32) * sum_costs;
    let residual = sum_need + emit_times;
    if residual.to_bits() == 0.0_f32.to_bits() || residual == 0.0 {
        Ok(())
    } else {
        // Also accept signed-zero / exact algebraic zero via absolute bit equality of sides.
        if sum_need.to_bits() == (-emit_times).to_bits() {
            Ok(())
        } else {
            Err(RecipeConservationViolation {
                sum_need_deltas: sum_need,
                emit_times_sum_costs: emit_times,
                residual,
            })
        }
    }
}

// ---------------------------------------------------------------------------
// (b) Per-allocator approximate-deterministic
// ---------------------------------------------------------------------------

/// One intermediate allocator step observation.
#[derive(Clone, Debug)]
pub struct AllocatorStepObservation {
    /// Budget available at intermediate `I` this step.
    pub budget: f32,
    /// Per-child disbursements `disbursed(I → C_i)`.
    pub disbursed: Vec<f32>,
    /// Residual observed integrating into parent Balance via `governed_by`
    /// (signed: budget − Σ disbursed). Required so the residual is accounted
    /// rather than silently tolerated.
    pub balance_residual: Option<f32>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AllocatorConservationViolation {
    ResidualExceedsBound {
        budget: f32,
        sum_disbursed: f32,
        abs_residual: f32,
        bound: f32,
        n_children: usize,
    },
    ResidualNotIntegrated {
        arithmetic_residual: f32,
        reported_balance_residual: Option<f32>,
    },
}

/// Approximate-deterministic per-allocator conservation (ADR).
///
/// Enforces `|Σ disbursed − budget| ≤ O(ε × n)` **and** that the residual is
/// integrated into Balance (`balance_residual` matches the arithmetic residual
/// within the same O(ε) slack — no free "any tolerance" pass).
pub fn check_allocator_step(
    obs: &AllocatorStepObservation,
) -> Result<(), AllocatorConservationViolation> {
    let n = obs.disbursed.len();
    let sum_disbursed: f32 = obs.disbursed.iter().copied().sum();
    let arithmetic_residual = obs.budget - sum_disbursed;
    let bound = allocator_eps_bound(n, obs.budget);
    let abs_residual = arithmetic_residual.abs();
    if abs_residual > bound {
        return Err(AllocatorConservationViolation::ResidualExceedsBound {
            budget: obs.budget,
            sum_disbursed,
            abs_residual,
            bound,
            n_children: n,
        });
    }
    // Residual must be accounted in Balance (same O(ε) slack; not an open-ended pass).
    let Some(measured_balance_residual) = obs.balance_residual else {
        return Err(AllocatorConservationViolation::ResidualNotIntegrated {
            arithmetic_residual,
            reported_balance_residual: None,
        });
    };
    let integrate_err = (measured_balance_residual - arithmetic_residual).abs();
    // A non-zero arithmetic residual cannot be "matched" by an observed zero:
    // that is the exact signature of a missing/disconnected governed Balance
    // integration path, even though both values individually fit inside the
    // allocator's O(epsilon * n) conservation envelope.
    let missing_nonzero_integration =
        arithmetic_residual != 0.0 && measured_balance_residual == 0.0;
    if !measured_balance_residual.is_finite()
        || missing_nonzero_integration
        || integrate_err > bound
    {
        return Err(AllocatorConservationViolation::ResidualNotIntegrated {
            arithmetic_residual,
            reported_balance_residual: Some(measured_balance_residual),
        });
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// (c) Per-arena structural + no orphans
// ---------------------------------------------------------------------------

/// Participant lineage for structural / orphan checks.
#[derive(Clone, Debug)]
pub struct ArenaParticipantObservation {
    pub id: u64,
    /// True when this participant is a leaf for allocation (receives allocated flow).
    pub is_leaf: bool,
    /// Declared intrinsic-flow contribution this tick (0 if none).
    pub intrinsic_flow: f32,
    /// Allocated flow received this tick (leaves; 0 for pure intermediates if unused).
    pub allocated_flow: f32,
    /// Balance column change this tick (via `governed_by` / residual integration).
    pub balance_delta: Option<f32>,
}

/// Explicit topology evidence from which the oracle independently derives
/// participant lineage. Callers provide facts, never an `is_orphan` verdict.
#[derive(Clone, Debug, Default)]
pub struct ArenaStructuralEvidence {
    pub declared_intrinsic_source_ids: Vec<u64>,
    pub inbound_coupling_endpoint_ids: Vec<u64>,
    pub parent_disbursement_recipient_ids: Vec<u64>,
}

/// Full arena observation for structural conservation.
#[derive(Clone, Debug)]
pub struct ArenaConservationSnapshot {
    pub participants: Vec<ArenaParticipantObservation>,
    pub structural_evidence: ArenaStructuralEvidence,
    /// Inbound coupling contributions into this arena this tick.
    pub inbound_coupling: f32,
    /// Emission consumption (recipe/transfer emit-side debit of arena mass) this tick.
    pub emission_consumption: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StructuralConservationViolation {
    MassImbalance {
        sources: f32,
        sinks: f32,
        residual: f32,
        bound: f32,
    },
    OrphanParticipants {
        orphan_ids: Vec<u64>,
    },
    MissingBalanceObservations {
        participant_ids: Vec<u64>,
    },
}

/// Structural per-arena conservation + orphan ban (ADR).
///
/// `intrinsic + inbound_coupling = leaf_allocations + Σ balance_delta + emission_consumption`
/// within an O(ε × n_participants) bound (f32 accumulation over the participant set).
pub fn check_arena_structural(
    snap: &ArenaConservationSnapshot,
) -> Result<(), StructuralConservationViolation> {
    let participant_ids: Vec<u64> = snap.participants.iter().map(|p| p.id).collect();
    let orphans = orphan_ids(&participant_ids, &snap.structural_evidence);
    if !orphans.is_empty() {
        return Err(StructuralConservationViolation::OrphanParticipants {
            orphan_ids: orphans,
        });
    }

    let missing_balance: Vec<u64> = snap
        .participants
        .iter()
        .filter(|participant| participant.balance_delta.is_none())
        .map(|participant| participant.id)
        .collect();
    if !missing_balance.is_empty() {
        return Err(
            StructuralConservationViolation::MissingBalanceObservations {
                participant_ids: missing_balance,
            },
        );
    }

    let intrinsic: f32 = snap.participants.iter().map(|p| p.intrinsic_flow).sum();
    let leaf_alloc: f32 = snap
        .participants
        .iter()
        .filter(|p| p.is_leaf)
        .map(|p| p.allocated_flow)
        .sum();
    let balance: f32 = snap
        .participants
        .iter()
        .filter_map(|participant| participant.balance_delta)
        .sum();

    let sources = intrinsic + snap.inbound_coupling;
    let sinks = leaf_alloc + balance + snap.emission_consumption;
    let residual = sources - sinks;
    let bound = allocator_eps_bound(snap.participants.len().max(1), sources);
    if residual.abs() > bound {
        return Err(StructuralConservationViolation::MassImbalance {
            sources,
            sinks,
            residual,
            bound,
        });
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Composite report
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct ConservationReport {
    pub recipe_ok: bool,
    pub allocator_ok: bool,
    pub structural_ok: bool,
    pub recipe_errors: Vec<RecipeConservationViolation>,
    pub allocator_errors: Vec<AllocatorConservationViolation>,
    pub structural_errors: Vec<StructuralConservationViolation>,
}

impl ConservationReport {
    pub fn all_pass(&self) -> bool {
        self.recipe_ok && self.allocator_ok && self.structural_ok
    }
}

/// Run all three invariant families on the provided observations.
pub fn check_conservation(
    recipes: &[RecipeInvocationObservation],
    allocators: &[AllocatorStepObservation],
    arenas: &[ArenaConservationSnapshot],
) -> ConservationReport {
    let mut report = ConservationReport {
        recipe_ok: true,
        allocator_ok: true,
        structural_ok: true,
        ..Default::default()
    };
    // Empty recipe set is vacuously exact (no recipe violated the identity).
    for r in recipes {
        if let Err(e) = check_recipe_exact(r) {
            report.recipe_ok = false;
            report.recipe_errors.push(e);
        }
    }
    if allocators.is_empty() {
        // An arena with no allocator steps still needs structural/orphan coverage;
        // allocator family is vacuously ok only when no intermediate disbursed.
        report.allocator_ok = true;
    }
    for a in allocators {
        if let Err(e) = check_allocator_step(a) {
            report.allocator_ok = false;
            report.allocator_errors.push(e);
        }
    }
    for s in arenas {
        if let Err(e) = check_arena_structural(s) {
            report.structural_ok = false;
            report.structural_errors.push(e);
        }
    }
    report
}

// ---------------------------------------------------------------------------
// Adapters from flat-star / E-11 style cell maps (no recursive RF source)
// ---------------------------------------------------------------------------

/// Build allocator + structural observations from a flat D=2 star after an
/// allocation pass. Pure arithmetic — does not call the recursive RF tick path.
///
/// `root_slot` holds the intermediate budget (`intrinsic_flow` at depth 0).
/// `leaf_slots` receive `allocated_flow`. `disbursed` is the measured leaf
/// allocation vector (same order as `leaf_slots`).
pub fn flat_star_observations(
    root_slot: u64,
    leaf_slots: &[u64],
    root_intrinsic: f32,
    leaf_allocated: &[f32],
    root_balance_delta: Option<f32>,
    leaf_balance_deltas: &[Option<f32>],
    inbound_coupling: f32,
    emission_consumption: f32,
) -> (AllocatorStepObservation, ArenaConservationSnapshot) {
    assert_eq!(leaf_slots.len(), leaf_allocated.len());
    assert_eq!(leaf_slots.len(), leaf_balance_deltas.len());
    let allocator = AllocatorStepObservation {
        budget: root_intrinsic,
        disbursed: leaf_allocated.to_vec(),
        // Never substitute arithmetic truth for the executed Balance readout.
        balance_residual: root_balance_delta,
    };

    let mut participants = Vec::with_capacity(1 + leaf_slots.len());
    participants.push(ArenaParticipantObservation {
        id: root_slot,
        is_leaf: false,
        intrinsic_flow: root_intrinsic,
        allocated_flow: 0.0,
        balance_delta: root_balance_delta,
    });
    for (i, &slot) in leaf_slots.iter().enumerate() {
        participants.push(ArenaParticipantObservation {
            id: slot,
            is_leaf: true,
            intrinsic_flow: 0.0,
            allocated_flow: leaf_allocated[i],
            balance_delta: leaf_balance_deltas[i],
        });
    }
    let arena = ArenaConservationSnapshot {
        participants,
        structural_evidence: ArenaStructuralEvidence {
            declared_intrinsic_source_ids: vec![root_slot],
            inbound_coupling_endpoint_ids: Vec::new(),
            parent_disbursement_recipient_ids: leaf_slots.to_vec(),
        },
        inbound_coupling,
        emission_consumption,
    };
    (allocator, arena)
}

/// Construct an allocator observation from measured child shares and a measured
/// Balance residual. The observation is intentionally incomplete when the
/// Balance readout is absent; the checker then returns `ResidualNotIntegrated`.
pub fn allocator_from_disbursements(
    budget: f32,
    disbursed: Vec<f32>,
    measured_balance_residual: Option<f32>,
) -> AllocatorStepObservation {
    AllocatorStepObservation {
        budget,
        balance_residual: measured_balance_residual,
        disbursed,
    }
}

/// Detect orphan ids from explicit topology evidence: intrinsic sources ∪
/// inbound-coupling endpoints ∪ parent-disbursement recipients.
pub fn orphan_ids(all_participant_ids: &[u64], evidence: &ArenaStructuralEvidence) -> Vec<u64> {
    let declared_lineage: HashSet<u64> = evidence
        .declared_intrinsic_source_ids
        .iter()
        .chain(evidence.inbound_coupling_endpoint_ids.iter())
        .chain(evidence.parent_disbursement_recipient_ids.iter())
        .copied()
        .collect();
    let mut orphans: Vec<u64> = all_participant_ids
        .iter()
        .copied()
        .filter(|id| !declared_lineage.contains(id))
        .collect();
    orphans.sort_unstable();
    orphans.dedup();
    orphans
}

/// Extract leaf allocated flows from a cell map `(slot, col) → value`.
///
/// Slot type is generic over anything that keys the cell map (typically
/// `SlotIndex` / `SlotId` in the E-11 allocation path).
pub fn leaf_allocated_from_cells<S: Copy + Eq + std::hash::Hash>(
    cells: &HashMap<(S, u32), f32>,
    leaf_slots: &[S],
    allocated_flow_col: u32,
) -> Vec<f32> {
    leaf_slots
        .iter()
        .map(|s| cells.get(&(*s, allocated_flow_col)).copied().unwrap_or(0.0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// BITE: non-conservative arenas FAIL; conservative arenas PASS.
    /// Covers recipe exact, allocator O(ε·n), and structural mass in one composite.
    #[test]
    fn composite_bite_nonconservative_fails_conservative_passes() {
        // Recipe exact: emit 2 × c=5 → ΔNeed=-10.
        let good_recipe = RecipeInvocationObservation {
            need_deltas: vec![-10.0],
            unit_costs: vec![5.0],
            emit_count: 2,
        };
        assert!(check_recipe_exact(&good_recipe).is_ok());
        let bad_recipe = RecipeInvocationObservation {
            need_deltas: vec![-6.0],
            unit_costs: vec![5.0],
            emit_count: 2,
        };
        assert!(check_recipe_exact(&bad_recipe).is_err());

        // Allocator proportional (1:3 of 10) within O(ε·n).
        let good_alloc = allocator_from_disbursements(10.0, vec![2.5, 7.5], Some(0.0));
        assert!(check_allocator_step(&good_alloc).is_ok());
        // f32 three-way split residual still within bound when integrated.
        let budget = 1.0_f32;
        let w = [1.0_f32, 1.0, 1.0];
        let wsum: f32 = w.iter().sum();
        let three_way: Vec<f32> = w.iter().map(|wi| budget * wi / wsum).collect();
        let three_way_residual = budget - three_way.iter().copied().sum::<f32>();
        assert!(check_allocator_step(&allocator_from_disbursements(
            budget,
            three_way,
            Some(three_way_residual),
        ))
        .is_ok());

        let bad_alloc = AllocatorStepObservation {
            budget: 10.0,
            disbursed: vec![1.0, 1.0],
            balance_residual: Some(8.0),
        };
        let fail = check_conservation(&[good_recipe.clone()], &[bad_alloc], &[]);
        assert!(!fail.all_pass(), "non-conservative must fail");
        assert!(!fail.allocator_ok);

        let (a2, arena) = flat_star_observations(
            1,
            &[2, 3],
            10.0,
            &[2.5, 7.5],
            Some(0.0),
            &[Some(0.0), Some(0.0)],
            0.0,
            0.0,
        );
        let pass = check_conservation(&[good_recipe], &[good_alloc, a2], &[arena]);
        assert!(pass.all_pass(), "conservative must pass: {:?}", pass);
    }

    /// BITE: disburse residual beyond O(ε·n) fails even if Balance claims residual.
    #[test]
    fn allocator_broken_disburse_exceeding_eps_bound_fails() {
        let obs = AllocatorStepObservation {
            budget: 10.0,
            disbursed: vec![2.0, 7.0], // residual 1.0 >> O(ε·n)
            balance_residual: Some(1.0),
        };
        let err = check_allocator_step(&obs).expect_err("must bite on O(ε·n) breach");
        match err {
            AllocatorConservationViolation::ResidualExceedsBound {
                abs_residual,
                bound,
                ..
            } => {
                assert!(abs_residual > bound);
                assert!(abs_residual > 0.5);
            }
            other => panic!("expected ResidualExceedsBound, got {other:?}"),
        }
        // Residual not integrated also fails when arithmetic residual is 0 but reported is not.
        let unintegrated = AllocatorStepObservation {
            budget: 10.0,
            disbursed: vec![2.5, 7.5],
            balance_residual: Some(5.0),
        };
        assert!(matches!(
            check_allocator_step(&unintegrated),
            Err(AllocatorConservationViolation::ResidualNotIntegrated { .. })
        ));
        let missing = AllocatorStepObservation {
            budget: 10.0,
            disbursed: vec![2.5, 7.5],
            balance_residual: None,
        };
        assert!(matches!(
            check_allocator_step(&missing),
            Err(AllocatorConservationViolation::ResidualNotIntegrated {
                reported_balance_residual: None,
                ..
            })
        ));
    }

    /// BITE: orphan participant fails structural; mass imbalance fails structural.
    #[test]
    fn structural_orphan_or_mass_imbalance_fails() {
        let orphan_arena = ArenaConservationSnapshot {
            participants: vec![
                ArenaParticipantObservation {
                    id: 1,
                    is_leaf: false,
                    intrinsic_flow: 5.0,
                    allocated_flow: 0.0,
                    balance_delta: Some(0.0),
                },
                ArenaParticipantObservation {
                    id: 99,
                    is_leaf: true,
                    intrinsic_flow: 0.0,
                    allocated_flow: 5.0,
                    balance_delta: Some(0.0),
                },
            ],
            structural_evidence: ArenaStructuralEvidence {
                declared_intrinsic_source_ids: vec![1],
                inbound_coupling_endpoint_ids: Vec::new(),
                parent_disbursement_recipient_ids: Vec::new(),
            },
            inbound_coupling: 0.0,
            emission_consumption: 0.0,
        };
        match check_arena_structural(&orphan_arena).expect_err("orphan must fail") {
            StructuralConservationViolation::OrphanParticipants { orphan_ids } => {
                assert_eq!(orphan_ids, vec![99]);
            }
            other => panic!("expected OrphanParticipants, got {other:?}"),
        }

        let imbalanced = ArenaConservationSnapshot {
            participants: vec![
                ArenaParticipantObservation {
                    id: 1,
                    is_leaf: false,
                    intrinsic_flow: 10.0,
                    allocated_flow: 0.0,
                    balance_delta: Some(0.0),
                },
                ArenaParticipantObservation {
                    id: 2,
                    is_leaf: true,
                    intrinsic_flow: 0.0,
                    allocated_flow: 3.0,
                    balance_delta: Some(0.0),
                },
            ],
            structural_evidence: ArenaStructuralEvidence {
                declared_intrinsic_source_ids: vec![1],
                inbound_coupling_endpoint_ids: Vec::new(),
                parent_disbursement_recipient_ids: vec![2],
            },
            inbound_coupling: 0.0,
            emission_consumption: 0.0,
        };
        assert!(matches!(
            check_arena_structural(&imbalanced),
            Err(StructuralConservationViolation::MassImbalance { .. })
        ));
    }
}
