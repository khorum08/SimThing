//! E-9 — Resource Flow `ArenaRegistry` session artifact (driver/spec layer).
//!
//! The registry is compile-time/session metadata. `simthing-sim` never imports
//! or branches on these types; the driver compiles flat `AccumulatorOp`
//! registrations at boundary sync (E-10/E-11).

use serde::{Deserialize, Serialize};
use simthing_core::{ArenaName, SimPropertyId, SimThingId};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

pub type ArenaIdx = u32;
pub type SlotId = u32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum FissionPolicy {
    Inherit,
    #[default]
    Reevaluate,
    Reject,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CouplingDelay {
    Algebraic,
    OneTickDelay,
    BoundaryStage { stage: u32 },
    AccumulatorState { property: SimPropertyId },
}

impl CouplingDelay {
    fn is_algebraic(self) -> bool {
        matches!(self, Self::Algebraic)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GpuArenaDescriptor {
    pub name: ArenaName,
    pub flow_property_id: SimPropertyId,
    pub balance_property_id: Option<SimPropertyId>,
    pub max_participants: u32,
    pub max_coupling_fanout: u32,
    pub max_orderband_depth: u32,
    pub fission_policy: FissionPolicy,
    /// Filled by the builder: `(start, len)` into [`ArenaRegistry::participants`].
    pub participant_range: (u32, u32),
    /// Declared upper bound for wildcard admission expansion (E-10 selector compile).
    pub wildcard_max_expansion: Option<u32>,
    /// Structural OrderBand reservation for future E-11 allocation (0 until enrolled).
    pub reserved_orderband_depth: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArenaParticipant {
    pub arena_idx: ArenaIdx,
    pub slot: SlotId,
    /// Subtree root for incremental refresh scoping.
    pub subtree_root: SimThingId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArenaCoupling {
    pub from_arena: ArenaIdx,
    pub to_arena: ArenaIdx,
    pub delay: CouplingDelay,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArenaRegistry {
    pub arenas: Vec<GpuArenaDescriptor>,
    pub participants: Vec<ArenaParticipant>,
    pub couplings: Vec<ArenaCoupling>,
    pub generation: u64,
    /// Per-subtree refresh generation — bumped only by subtree-scoped refresh.
    pub subtree_generations: HashMap<SimThingId, u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArenaDiagnostic {
    pub code: &'static str,
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ArenaExpansionReport {
    pub arena_count: usize,
    pub participant_count: usize,
    pub coupling_count: usize,
    pub rejected: Vec<ArenaDiagnostic>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArenaRefreshReport {
    pub changed_root: SimThingId,
    pub participants_reevaluated: usize,
    pub generation_before: u64,
    pub generation_after: u64,
    pub untouched_participant_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ArenaRegistryError {
    #[error("arena index {0} out of range")]
    InvalidArenaIdx(ArenaIdx),
    #[error("arena `{arena}` requires explicit admission; implicit participation is forbidden")]
    ImplicitParticipation { arena: ArenaName },
    #[error("arena `{arena}` declares wildcard admission without a declared upper bound")]
    UnboundedWildcard { arena: ArenaName },
    #[error("arena `{arena}` exceeds max_participants ({declared} declared, {computed} computed)")]
    MaxParticipantsExceeded {
        arena: ArenaName,
        declared: u32,
        computed: u32,
    },
    #[error("arena `{arena}` exceeds max_coupling_fanout ({declared} declared, {computed} computed)")]
    MaxCouplingFanoutExceeded {
        arena: ArenaName,
        declared: u32,
        computed: u32,
    },
    #[error("arena `{arena}` exceeds max_orderband_depth ({declared} declared, {computed} computed)")]
    MaxOrderBandDepthExceeded {
        arena: ArenaName,
        declared: u32,
        computed: u32,
    },
    #[error("coupling graph contains an all-algebraic cycle: {cycle:?}")]
    AllAlgebraicCouplingCycle { cycle: Vec<ArenaIdx> },
    #[error("arena `{arena}` hidden fanout {computed} exceeds declared budget {declared}")]
    HiddenFanoutExceeded {
        arena: ArenaName,
        declared: u32,
        computed: u32,
    },
}

/// Session-build draft for [`ArenaRegistry`].
#[derive(Clone, Debug, Default)]
pub struct ArenaRegistryBuilder {
    arenas: Vec<GpuArenaDescriptor>,
    participants: Vec<ArenaParticipant>,
    couplings: Vec<ArenaCoupling>,
    /// Wildcard admission declared without explicit slots — requires `max_expansion`.
    wildcard_declarations: Vec<(ArenaIdx, Option<u32>)>,
}

impl ArenaRegistryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_arena(&mut self, mut desc: GpuArenaDescriptor) -> ArenaIdx {
        let idx = self.arenas.len() as ArenaIdx;
        desc.participant_range = (0, 0);
        self.arenas.push(desc);
        idx
    }

    /// Explicit admission only — property possession does not enroll.
    pub fn admit_participant(
        &mut self,
        arena_idx: ArenaIdx,
        slot: SlotId,
        subtree_root: SimThingId,
    ) -> Result<(), ArenaRegistryError> {
        self.arena_name(arena_idx)?;
        self.participants.push(ArenaParticipant {
            arena_idx,
            slot,
            subtree_root,
        });
        Ok(())
    }

    /// Declare wildcard admission budget. `max_expansion: None` is rejected at build.
    pub fn declare_wildcard_admission(
        &mut self,
        arena_idx: ArenaIdx,
        max_expansion: Option<u32>,
    ) -> Result<(), ArenaRegistryError> {
        self.arena_name(arena_idx)?;
        self.wildcard_declarations.push((arena_idx, max_expansion));
        Ok(())
    }

    pub fn push_coupling(&mut self, coupling: ArenaCoupling) -> Result<(), ArenaRegistryError> {
        self.arena_name(coupling.from_arena)?;
        self.arena_name(coupling.to_arena)?;
        self.couplings.push(coupling);
        Ok(())
    }

    pub fn reserve_orderband_depth(
        &mut self,
        arena_idx: ArenaIdx,
        depth: u32,
    ) -> Result<(), ArenaRegistryError> {
        let arena = self
            .arenas
            .get_mut(arena_idx as usize)
            .ok_or(ArenaRegistryError::InvalidArenaIdx(arena_idx))?;
        arena.reserved_orderband_depth = depth;
        Ok(())
    }

    pub fn build(self) -> Result<(ArenaRegistry, ArenaExpansionReport), ArenaRegistryError> {
        validate_and_finalize(self)
    }

    fn arena_name(&self, arena_idx: ArenaIdx) -> Result<&ArenaName, ArenaRegistryError> {
        self.arenas
            .get(arena_idx as usize)
            .map(|a| &a.name)
            .ok_or(ArenaRegistryError::InvalidArenaIdx(arena_idx))
    }
}

impl ArenaRegistry {
    pub fn empty() -> Self {
        Self::default()
    }

    /// Subtree-scoped refresh — re-evaluates admission only for participants
    /// rooted at `changed_root`. Does not rebuild the global registry.
    pub fn refresh_subtree(&mut self, changed_root: SimThingId) -> ArenaRefreshReport {
        let generation_before = self.generation;
        let mut participants_reevaluated = 0usize;
        for p in &self.participants {
            if p.subtree_root == changed_root {
                participants_reevaluated += 1;
            }
        }
        if participants_reevaluated > 0 {
            self.generation = self.generation.saturating_add(1);
            *self
                .subtree_generations
                .entry(changed_root)
                .or_insert(0) += 1;
        }
        let untouched_participant_count = self
            .participants
            .len()
            .saturating_sub(participants_reevaluated);
        ArenaRefreshReport {
            changed_root,
            participants_reevaluated,
            generation_before,
            generation_after: self.generation,
            untouched_participant_count,
        }
    }

    pub fn expansion_report(&self) -> ArenaExpansionReport {
        ArenaExpansionReport {
            arena_count: self.arenas.len(),
            participant_count: self.participants.len(),
            coupling_count: self.couplings.len(),
            rejected: Vec::new(),
        }
    }
}

fn validate_and_finalize(
    mut builder: ArenaRegistryBuilder,
) -> Result<(ArenaRegistry, ArenaExpansionReport), ArenaRegistryError> {
    let rejected = Vec::new();

    for (arena_idx, max_expansion) in &builder.wildcard_declarations {
        let arena = &builder.arenas[*arena_idx as usize];
        match max_expansion {
            None | Some(0) => {
                return Err(ArenaRegistryError::UnboundedWildcard {
                    arena: arena.name.clone(),
                });
            }
            Some(cap) => {
                builder.arenas[*arena_idx as usize].wildcard_max_expansion = Some(*cap);
            }
        }
    }

    // Participant ranges and per-arena counts.
    let mut per_arena_counts = vec![0u32; builder.arenas.len()];
    for p in &builder.participants {
        per_arena_counts[p.arena_idx as usize] += 1;
    }

    for (arena_idx, arena) in builder.arenas.iter_mut().enumerate() {
        let count = per_arena_counts[arena_idx];
        if count > arena.max_participants {
            return Err(ArenaRegistryError::MaxParticipantsExceeded {
                arena: arena.name.clone(),
                declared: arena.max_participants,
                computed: count,
            });
        }
        if count == 0 && arena.wildcard_max_expansion.is_none() {
            return Err(ArenaRegistryError::ImplicitParticipation {
                arena: arena.name.clone(),
            });
        }
        if arena.reserved_orderband_depth > arena.max_orderband_depth {
            return Err(ArenaRegistryError::MaxOrderBandDepthExceeded {
                arena: arena.name.clone(),
                declared: arena.max_orderband_depth,
                computed: arena.reserved_orderband_depth,
            });
        }
    }

    // E-9R: arena-major canonical order so participant_range is a contiguous slice.
    canonicalize_participants_arena_major(&mut builder.participants);
    let mut range_start = 0u32;
    for (arena_idx, arena) in builder.arenas.iter_mut().enumerate() {
        let count = per_arena_counts[arena_idx];
        arena.participant_range = (range_start, count);
        range_start += count;
    }

    // Coupling fanout: out-edges per arena.
    let n_arenas = builder.arenas.len();
    let mut out_fanout = vec![0u32; n_arenas];
    let mut in_fanout = vec![0u32; n_arenas];
    for c in &builder.couplings {
        out_fanout[c.from_arena as usize] += 1;
        in_fanout[c.to_arena as usize] += 1;
    }
    for (idx, arena) in builder.arenas.iter().enumerate() {
        let computed = out_fanout[idx].max(in_fanout[idx]);
        if computed > arena.max_coupling_fanout {
            return Err(ArenaRegistryError::MaxCouplingFanoutExceeded {
                arena: arena.name.clone(),
                declared: arena.max_coupling_fanout,
                computed,
            });
        }
        // Hidden fanout: combined in+out edges must fit the declared budget.
        let total = out_fanout[idx] + in_fanout[idx];
        if total > arena.max_coupling_fanout {
            return Err(ArenaRegistryError::HiddenFanoutExceeded {
                arena: arena.name.clone(),
                declared: arena.max_coupling_fanout,
                computed: total,
            });
        }
    }

    if let Some(cycle) = find_all_algebraic_cycle(&builder.couplings) {
        return Err(ArenaRegistryError::AllAlgebraicCouplingCycle { cycle });
    }

    let registry = ArenaRegistry {
        arenas: builder.arenas,
        participants: builder.participants,
        couplings: builder.couplings,
        generation: 1,
        subtree_generations: HashMap::new(),
    };

    let report = ArenaExpansionReport {
        arena_count: registry.arenas.len(),
        participant_count: registry.participants.len(),
        coupling_count: registry.couplings.len(),
        rejected,
    };

    Ok((registry, report))
}

/// Sort participants arena-major (0..n) preserving stable within-arena order.
fn canonicalize_participants_arena_major(participants: &mut [ArenaParticipant]) {
    participants.sort_by_key(|p| p.arena_idx);
}

fn find_all_algebraic_cycle(couplings: &[ArenaCoupling]) -> Option<Vec<ArenaIdx>> {
    if couplings.is_empty() {
        return None;
    }
    let mut adj: HashMap<ArenaIdx, Vec<(ArenaIdx, CouplingDelay)>> = HashMap::new();
    for c in couplings {
        adj.entry(c.from_arena)
            .or_default()
            .push((c.to_arena, c.delay));
    }
    let nodes: HashSet<ArenaIdx> = couplings
        .iter()
        .flat_map(|c| [c.from_arena, c.to_arena])
        .collect();

    for start in &nodes {
        let mut stack = vec![(*start, vec![*start], true)];
        let mut visiting = HashSet::new();
        visiting.insert(*start);
        while let Some((node, path, all_algebraic)) = stack.pop() {
            let Some(edges) = adj.get(&node) else {
                continue;
            };
            for (next, delay) in edges {
                if *next == *start && path.len() > 1 {
                    if all_algebraic && delay.is_algebraic() {
                        return Some(path.clone());
                    }
                    continue;
                }
                if visiting.contains(next) {
                    continue;
                }
                let mut next_path = path.clone();
                next_path.push(*next);
                visiting.insert(*next);
                stack.push((
                    *next,
                    next_path,
                    all_algebraic && delay.is_algebraic(),
                ));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::SimPropertyId;

    fn food_arena(max_participants: u32) -> GpuArenaDescriptor {
        GpuArenaDescriptor {
            name: "food".into(),
            flow_property_id: SimPropertyId(0),
            balance_property_id: Some(SimPropertyId(1)),
            max_participants,
            max_coupling_fanout: 4,
            max_orderband_depth: 8,
            fission_policy: FissionPolicy::default(),
            participant_range: (0, 0),
            wildcard_max_expansion: None,
            reserved_orderband_depth: 0,
        }
    }

    fn research_arena() -> GpuArenaDescriptor {
        GpuArenaDescriptor {
            name: "research".into(),
            flow_property_id: SimPropertyId(2),
            balance_property_id: None,
            max_participants: 8,
            max_coupling_fanout: 4,
            max_orderband_depth: 8,
            fission_policy: FissionPolicy::Reevaluate,
            participant_range: (0, 0),
            wildcard_max_expansion: None,
            reserved_orderband_depth: 0,
        }
    }

    fn participants_in_range<'a>(
        reg: &'a ArenaRegistry,
        arena_idx: ArenaIdx,
    ) -> &'a [ArenaParticipant] {
        let (start, len) = reg.arenas[arena_idx as usize].participant_range;
        let start = start as usize;
        let end = start + len as usize;
        &reg.participants[start..end]
    }

    #[test]
    fn arena_registry_participant_ranges_are_contiguous_when_admissions_interleaved() {
        let mut b = ArenaRegistryBuilder::new();
        let food = b.push_arena(food_arena(4));
        let research = b.push_arena(research_arena());
        let root_a = SimThingId::new();
        let root_b = SimThingId::new();
        let root_c = SimThingId::new();
        // Interleaved: food A, research B, food C
        b.admit_participant(food, 10, root_a).unwrap();
        b.admit_participant(research, 20, root_b).unwrap();
        b.admit_participant(food, 30, root_c).unwrap();
        let (reg, _) = b.build().unwrap();

        assert_eq!(reg.arenas[food as usize].participant_range, (0, 2));
        assert_eq!(reg.arenas[research as usize].participant_range, (2, 1));

        let food_slice = participants_in_range(&reg, food);
        assert_eq!(food_slice.len(), 2);
        assert!(food_slice.iter().all(|p| p.arena_idx == food));
        assert_eq!(food_slice[0].slot, 10);
        assert_eq!(food_slice[1].slot, 30);
        assert_eq!(food_slice[0].subtree_root, root_a);
        assert_eq!(food_slice[1].subtree_root, root_c);

        let research_slice = participants_in_range(&reg, research);
        assert_eq!(research_slice.len(), 1);
        assert_eq!(research_slice[0].slot, 20);
        assert_eq!(research_slice[0].subtree_root, root_b);
        assert!(research_slice.iter().all(|p| p.arena_idx == research));
    }

    #[test]
    fn arena_registry_participant_range_slices_match_arena_idx() {
        let mut b = ArenaRegistryBuilder::new();
        let food = b.push_arena(food_arena(4));
        let research = b.push_arena(research_arena());
        let suppression = b.push_arena(GpuArenaDescriptor {
            name: "suppression".into(),
            flow_property_id: SimPropertyId(3),
            balance_property_id: None,
            max_participants: 4,
            max_coupling_fanout: 4,
            max_orderband_depth: 8,
            fission_policy: FissionPolicy::Reject,
            participant_range: (0, 0),
            wildcard_max_expansion: None,
            reserved_orderband_depth: 0,
        });
        b.admit_participant(research, 1, SimThingId::new()).unwrap();
        b.admit_participant(food, 2, SimThingId::new()).unwrap();
        b.admit_participant(suppression, 3, SimThingId::new()).unwrap();
        b.admit_participant(food, 4, SimThingId::new()).unwrap();
        let (reg, _) = b.build().unwrap();

        for (arena_idx, arena) in reg.arenas.iter().enumerate() {
            let slice = participants_in_range(&reg, arena_idx as ArenaIdx);
            assert_eq!(slice.len(), arena.participant_range.1 as usize);
            assert!(slice.iter().all(|p| p.arena_idx == arena_idx as ArenaIdx));
        }
        assert_eq!(reg.arenas[0].participant_range, (0, 2));
        assert_eq!(reg.arenas[1].participant_range, (2, 1));
        assert_eq!(reg.arenas[2].participant_range, (3, 1));
    }

    #[test]
    fn arena_registry_participant_range_is_stable_with_multiple_arenas() {
        let build = |roots: [SimThingId; 4]| {
            let mut b = ArenaRegistryBuilder::new();
            let food = b.push_arena(food_arena(4));
            let research = b.push_arena(research_arena());
            b.admit_participant(research, 5, roots[0]).unwrap();
            b.admit_participant(food, 1, roots[1]).unwrap();
            b.admit_participant(food, 2, roots[2]).unwrap();
            b.admit_participant(research, 6, roots[3]).unwrap();
            b.build().unwrap().0
        };
        let roots = [
            SimThingId::new(),
            SimThingId::new(),
            SimThingId::new(),
            SimThingId::new(),
        ];
        let a = build(roots);
        let b = build(roots);
        assert_eq!(a.participants, b.participants);
        assert_eq!(
            a.arenas
                .iter()
                .map(|arena| arena.participant_range)
                .collect::<Vec<_>>(),
            b.arenas
                .iter()
                .map(|arena| arena.participant_range)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn arena_registry_refresh_subtree_still_counts_after_participant_canonicalization() {
        let mut b = ArenaRegistryBuilder::new();
        let food = b.push_arena(food_arena(4));
        let research = b.push_arena(research_arena());
        let root_a = SimThingId::new();
        let root_b = SimThingId::new();
        b.admit_participant(food, 10, root_a).unwrap();
        b.admit_participant(research, 20, root_b).unwrap();
        b.admit_participant(food, 30, root_a).unwrap();
        let (mut reg, _) = b.build().unwrap();
        let report = reg.refresh_subtree(root_a);
        assert_eq!(report.participants_reevaluated, 2);
        assert_eq!(report.untouched_participant_count, 1);
    }

    #[test]
    fn arena_registry_explicit_admission_only() {
        let mut b = ArenaRegistryBuilder::new();
        let food = b.push_arena(food_arena(4));
        b.admit_participant(food, 1, SimThingId::new()).unwrap();
        let (reg, report) = b.build().unwrap();
        assert_eq!(report.participant_count, 1);
        assert_eq!(reg.participants[0].slot, 1);
    }

    #[test]
    fn arena_registry_rejects_implicit_participation() {
        let mut b = ArenaRegistryBuilder::new();
        b.push_arena(food_arena(4));
        let err = b.build().unwrap_err();
        assert!(matches!(
            err,
            ArenaRegistryError::ImplicitParticipation { .. }
        ));
    }

    #[test]
    fn arena_registry_rejects_unbounded_wildcard_without_cap() {
        let mut b = ArenaRegistryBuilder::new();
        let food = b.push_arena(food_arena(4));
        b.declare_wildcard_admission(food, None).unwrap();
        let err = b.build().unwrap_err();
        assert!(matches!(err, ArenaRegistryError::UnboundedWildcard { .. }));
    }

    #[test]
    fn arena_registry_wildcard_with_cap_satisfies_admission() {
        let mut b = ArenaRegistryBuilder::new();
        let food = b.push_arena(food_arena(4));
        b.declare_wildcard_admission(food, Some(4)).unwrap();
        let (reg, _) = b.build().unwrap();
        assert_eq!(reg.arenas[0].wildcard_max_expansion, Some(4));
    }

    #[test]
    fn arena_registry_enforces_max_participants() {
        let mut b = ArenaRegistryBuilder::new();
        let food = b.push_arena(food_arena(1));
        b.admit_participant(food, 1, SimThingId::new()).unwrap();
        b.admit_participant(food, 2, SimThingId::new()).unwrap();
        let err = b.build().unwrap_err();
        assert!(matches!(
            err,
            ArenaRegistryError::MaxParticipantsExceeded { .. }
        ));
    }

    #[test]
    fn arena_registry_rejects_all_algebraic_coupling_cycle() {
        let mut b = ArenaRegistryBuilder::new();
        let a = b.push_arena(food_arena(4));
        let b_idx = b.push_arena(research_arena());
        let c = b.push_arena(GpuArenaDescriptor {
            name: "suppression".into(),
            flow_property_id: SimPropertyId(3),
            balance_property_id: None,
            max_participants: 4,
            max_coupling_fanout: 4,
            max_orderband_depth: 8,
            fission_policy: FissionPolicy::Reevaluate,
            participant_range: (0, 0),
            wildcard_max_expansion: None,
            reserved_orderband_depth: 0,
        });
        b.admit_participant(a, 0, SimThingId::new()).unwrap();
        b.admit_participant(b_idx, 1, SimThingId::new()).unwrap();
        b.admit_participant(c, 2, SimThingId::new()).unwrap();
        b.push_coupling(ArenaCoupling {
            from_arena: a,
            to_arena: b_idx,
            delay: CouplingDelay::Algebraic,
        })
        .unwrap();
        b.push_coupling(ArenaCoupling {
            from_arena: b_idx,
            to_arena: c,
            delay: CouplingDelay::Algebraic,
        })
        .unwrap();
        b.push_coupling(ArenaCoupling {
            from_arena: c,
            to_arena: a,
            delay: CouplingDelay::Algebraic,
        })
        .unwrap();
        let err = b.build().unwrap_err();
        assert!(matches!(
            err,
            ArenaRegistryError::AllAlgebraicCouplingCycle { .. }
        ));
    }

    #[test]
    fn arena_registry_allows_cycle_with_delay_edge() {
        let mut b = ArenaRegistryBuilder::new();
        let a = b.push_arena(food_arena(4));
        let b_idx = b.push_arena(research_arena());
        b.admit_participant(a, 0, SimThingId::new()).unwrap();
        b.admit_participant(b_idx, 1, SimThingId::new()).unwrap();
        b.declare_wildcard_admission(b_idx, Some(2)).unwrap();
        b.push_coupling(ArenaCoupling {
            from_arena: a,
            to_arena: b_idx,
            delay: CouplingDelay::Algebraic,
        })
        .unwrap();
        b.push_coupling(ArenaCoupling {
            from_arena: b_idx,
            to_arena: a,
            delay: CouplingDelay::OneTickDelay,
        })
        .unwrap();
        assert!(b.build().is_ok());
    }

    #[test]
    fn arena_registry_fission_policy_defaults_reevaluate() {
        assert_eq!(FissionPolicy::default(), FissionPolicy::Reevaluate);
        let arena = food_arena(1);
        assert_eq!(arena.fission_policy, FissionPolicy::Reevaluate);
    }

    #[test]
    fn arena_registry_refresh_is_subtree_scoped() {
        let mut b = ArenaRegistryBuilder::new();
        let food = b.push_arena(food_arena(4));
        let research = b.push_arena(research_arena());
        let root_a = SimThingId::new();
        let root_b = SimThingId::new();
        b.admit_participant(food, 1, root_a).unwrap();
        b.admit_participant(food, 2, root_b).unwrap();
        b.admit_participant(research, 3, root_a).unwrap();
        let (mut reg, _) = b.build().unwrap();
        let gen0 = reg.generation;
        let report = reg.refresh_subtree(root_a);
        assert_eq!(report.participants_reevaluated, 2);
        assert_eq!(report.untouched_participant_count, 1);
        assert_eq!(report.generation_before, gen0);
        assert_eq!(report.generation_after, gen0 + 1);
        assert_eq!(reg.subtree_generations.get(&root_a), Some(&1));

        let gen1 = reg.generation;
        let report2 = reg.refresh_subtree(root_b);
        assert_eq!(report2.participants_reevaluated, 1);
        assert_eq!(report2.generation_after, gen1 + 1);
        assert_eq!(reg.subtree_generations.get(&root_b), Some(&1));
    }

    #[test]
    fn arena_registry_does_not_cross_into_simthing_sim() {
        let sim_cargo = include_str!("../../simthing-sim/Cargo.toml");
        assert!(
            !sim_cargo.contains("simthing-driver"),
            "simthing-sim must not depend on simthing-driver"
        );
        let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
        assert!(!sim_lib.contains("ArenaRegistry"));
        assert!(!sim_lib.contains("arena_registry"));
    }
}
