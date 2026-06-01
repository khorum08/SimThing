//! MOBILITY-IDROUTE-0: local D=2 identity-routing overlay substrate.
//!
//! Local per-cell masked reduction + directed disburse using identity as a column
//! (not tree structure). This is a named, metadata/testable substrate only.
//! It does not implement ECON, OWNER, global faction vectors, production
//! `SimSession` wiring, semantic/raw WGSL, GPU kernels, or default-on behavior.

use std::collections::{BTreeMap, BTreeSet};

use super::mobility_alloc0::MobilityAlloc0ParentKey;

pub const MOBILITY_IDROUTE0_ID: &str = "mobility_idroute0_local_d2_identity_routing";

/// Bounded local identity lane (0 .. max_factions_per_cell-1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdentityLane(pub u32);

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityIdroute0LocalRecord {
    pub entity_id: u64,
    pub parent_key: MobilityAlloc0ParentKey,
    pub identity: IdentityLane,
    /// Example hard quantity (e.g. firepower, damage) — exact path.
    pub hard_value: i64,
    /// Example soft quantity (e.g. morale, repair) — approximate-deterministic path.
    pub soft_value: f32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MobilityIdroute0ForbiddenPathRequests {
    pub global_faction_vector: bool,
    pub owner_as_spatial_parent: bool,
    pub capture_as_reparenting: bool,
    pub econ_owner_runtime: bool,
    pub production_simsession_wiring: bool,
    pub default_on_behavior: bool,
    pub semantic_or_raw_wgsl: bool,
    pub exceeding_max_factions_per_cell: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityIdroute0PlanInput {
    pub records: Vec<MobilityIdroute0LocalRecord>,
    pub max_factions_per_cell: u32,
    pub forbidden: MobilityIdroute0ForbiddenPathRequests,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PerIdentitySum {
    pub identity: IdentityLane,
    pub hard_sum: i64,
    pub soft_sum: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectedDisburse {
    pub target_entity_id: u64,
    pub hard_amount: i64,
    pub soft_amount: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityIdroute0PlanReport {
    pub substrate_id: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub per_identity_sums: Vec<PerIdentitySum>,
    pub argmax_winner: Option<(IdentityLane, u64)>, // (identity, entity_id of unique winner)
    pub directed_disburses: Vec<DirectedDisburse>,

    pub touched_cell_count: u32,
    pub unique_identities_used: u32,
    pub max_local_identities_used: u32,
    pub local_d2_cell_admission: bool,
    pub identity_lanes_are_local_columns: bool,
    pub directed_disburse_immutable: bool,
    pub cpu_gpu_parity_checksum: u64,

    pub runtime_implementation_authorized: bool,
}

pub fn plan_mobility_idroute0(input: &MobilityIdroute0PlanInput) -> MobilityIdroute0PlanReport {
    let mut diagnostics = Vec::new();
    validate_forbidden(&input.forbidden, &mut diagnostics);

    let k = input.max_factions_per_cell;

    // Group first so admission is explicitly local cell-level D=2. The same
    // lane ids may repeat across cells without implying any global vector.
    let mut by_cell: BTreeMap<MobilityAlloc0ParentKey, Vec<&MobilityIdroute0LocalRecord>> =
        BTreeMap::new();
    for rec in &input.records {
        by_cell.entry(rec.parent_key).or_default().push(rec);
    }

    if k == 0 {
        diagnostics.push("exceeding_max_factions_per_cell");
    }

    let mut max_local_identities_used = 0u32;
    let mut local_lane_values = BTreeSet::new();
    for recs in by_cell.values() {
        let lanes = recs.iter().map(|r| r.identity.0).collect::<BTreeSet<_>>();
        if lanes.len() as u32 > k || lanes.iter().any(|lane| *lane >= k) {
            diagnostics.push("exceeding_max_factions_per_cell");
        }
        max_local_identities_used = max_local_identities_used.max(lanes.len() as u32);
        local_lane_values.extend(lanes);
    }

    if !diagnostics.is_empty() {
        return rejected_report(input, diagnostics, max_local_identities_used);
    }

    let mut per_identity_sums: BTreeMap<IdentityLane, (i64, f32)> = BTreeMap::new();
    let mut all_disburses = Vec::new();
    let mut argmax_winner = None;
    let mut touched = 0u32;

    for (_cell, recs) in &by_cell {
        touched += 1;

        // Masked sums per identity (local to this cell)
        let mut cell_sums: BTreeMap<IdentityLane, (i64, f32)> = BTreeMap::new();
        for rec in recs.iter() {
            let entry = cell_sums.entry(rec.identity).or_insert((0, 0.0));
            entry.0 += rec.hard_value;
            entry.1 += rec.soft_value;
        }

        for (id, (h, s)) in &cell_sums {
            let total = per_identity_sums.entry(*id).or_insert((0, 0.0));
            total.0 += *h;
            total.1 += *s;
        }

        // Deterministic argmax using packed key
        if !recs.is_empty() {
            let mut best_key: i128 = -1;
            let mut best_entity = 0u64;
            let mut best_id = IdentityLane(0);

            for rec in recs.iter() {
                let key = ((rec.hard_value as i128) << 32) | (rec.entity_id as i128);
                if key > best_key {
                    best_key = key;
                    best_entity = rec.entity_id;
                    best_id = rec.identity;
                }
            }
            argmax_winner = Some((best_id, best_entity));
        }

        // Directed disburse (local model)
        for rec in recs.iter() {
            let amount = rec.hard_value / (recs.len() as i64).max(1);
            all_disburses.push(DirectedDisburse {
                target_entity_id: rec.entity_id,
                hard_amount: amount,
                soft_amount: rec.soft_value / (recs.len() as f32).max(1.0),
            });
        }
    }

    let final_sums: Vec<PerIdentitySum> = per_identity_sums
        .into_iter()
        .map(|(id, (h, s))| PerIdentitySum {
            identity: id,
            hard_sum: h,
            soft_sum: s,
        })
        .collect();

    let checksum = compute_idroute_checksum(&input.records, &final_sums);

    MobilityIdroute0PlanReport {
        substrate_id: MOBILITY_IDROUTE0_ID,
        admitted: true,
        diagnostics,
        per_identity_sums: final_sums,
        argmax_winner,
        directed_disburses: all_disburses,
        touched_cell_count: touched,
        unique_identities_used: local_lane_values.len() as u32,
        max_local_identities_used,
        local_d2_cell_admission: true,
        identity_lanes_are_local_columns: true,
        directed_disburse_immutable: true,
        cpu_gpu_parity_checksum: checksum,
        runtime_implementation_authorized: false,
    }
}

fn validate_forbidden(
    forbidden: &MobilityIdroute0ForbiddenPathRequests,
    diags: &mut Vec<&'static str>,
) {
    if forbidden.global_faction_vector {
        diags.push("global_faction_vector");
    }
    if forbidden.owner_as_spatial_parent {
        diags.push("owner_as_spatial_parent");
    }
    if forbidden.capture_as_reparenting {
        diags.push("capture_as_reparenting");
    }
    if forbidden.econ_owner_runtime {
        diags.push("econ_owner_runtime");
    }
    if forbidden.production_simsession_wiring {
        diags.push("production_simsession_wiring");
    }
    if forbidden.default_on_behavior {
        diags.push("default_on_behavior");
    }
    if forbidden.semantic_or_raw_wgsl {
        diags.push("semantic_or_raw_wgsl");
    }
    if forbidden.exceeding_max_factions_per_cell {
        diags.push("exceeding_max_factions_per_cell");
    }
}

fn rejected_report(
    _input: &MobilityIdroute0PlanInput,
    diagnostics: Vec<&'static str>,
    max_local_identities_used: u32,
) -> MobilityIdroute0PlanReport {
    MobilityIdroute0PlanReport {
        substrate_id: MOBILITY_IDROUTE0_ID,
        admitted: false,
        diagnostics,
        per_identity_sums: vec![],
        argmax_winner: None,
        directed_disburses: vec![],
        touched_cell_count: 0,
        unique_identities_used: 0,
        max_local_identities_used,
        local_d2_cell_admission: false,
        identity_lanes_are_local_columns: true,
        directed_disburse_immutable: true,
        cpu_gpu_parity_checksum: 0,
        runtime_implementation_authorized: false,
    }
}

fn compute_idroute_checksum(
    records: &[MobilityIdroute0LocalRecord],
    sums: &[PerIdentitySum],
) -> u64 {
    // Simple deterministic checksum (FNV-like) for substrate parity proxy
    let mut h: u64 = 0xcbf29ce484222325;
    for r in records {
        h ^= r.entity_id;
        h = h.wrapping_mul(0x100000001b3);
        h ^= r.identity.0 as u64;
    }
    for s in sums {
        h ^= s.hard_sum as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

pub fn mobility_idroute0_layout_checksum_cpu(records: &[MobilityIdroute0LocalRecord]) -> u64 {
    compute_idroute_checksum(records, &[])
}

pub fn mobility_idroute0_layout_checksum_gpu_proxy(records: &[MobilityIdroute0LocalRecord]) -> u64 {
    // Proxy is intentionally identical at substrate level for now (real GPU path would differ in lowering)
    mobility_idroute0_layout_checksum_cpu(records)
}
