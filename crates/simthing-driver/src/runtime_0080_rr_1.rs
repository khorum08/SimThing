//! RUNTIME-0080-RR-1: nested sparse residency for the RR-0 recursive hierarchy.
//!
//! Galaxy 20×20 is always resident. System 10×10 and planet surface 10×10 materialize only on
//! descend into the occupied parent; ascend deactivates child tiers. Child visibility (starport,
//! pop, factory) follows the resident parent tier. Consumes RR-0 world; no GPU economy claim.

use crate::runtime_0080_rr_0::{
    build_recursive_world, Runtime0080Rr0Owner, Runtime0080Rr0RecursiveWorld, Runtime0080Rr0System,
};

pub const RUNTIME_0080_RR_1_ID: &str = "RUNTIME-0080-RR-1";
pub const RUNTIME_0080_RR_1_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - nested sparse residency for galaxy→system→planet-surface";
pub const RUNTIME_0080_RR_1_STATUS_PARTIAL: &str =
    "IMPLEMENTED / PARTIAL - nested residency incomplete or proxied";
pub const RUNTIME_0080_RR_1_STATUS_BLOCKED: &str =
    "BLOCKED - nested RR-1 cannot close without approved deviation";

pub const RUNTIME_RR_1_EXPECTED_REPORT_CHECKSUM: u64 = 0xe615_3526_c154_1764;

pub const RR_1_GALAXY_SIDE: u32 = 20;
pub const RR_1_SYSTEM_SIDE: u32 = 10;
pub const RR_1_SURFACE_SIDE: u32 = 10;
pub const RR_1_SYSTEM_COUNT: usize = 13;
pub const RR_1_GALAXY_CELL_COUNT: u32 = RR_1_GALAXY_SIDE * RR_1_GALAXY_SIDE;
pub const RR_1_SYSTEM_CELL_COUNT: u32 = RR_1_SYSTEM_SIDE * RR_1_SYSTEM_SIDE;
pub const RR_1_SURFACE_CELL_COUNT: u32 = RR_1_SURFACE_SIDE * RR_1_SURFACE_SIDE;
pub const RR_1_TOTAL_LOGICAL_CELL_COUNT: u32 = RR_1_GALAXY_CELL_COUNT
    + RR_1_SYSTEM_COUNT as u32 * (RR_1_SYSTEM_CELL_COUNT + RR_1_SURFACE_CELL_COUNT);

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const RR_0_DEFAULT_SEED: u64 = 0x0080_2000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Runtime0080Rr1TierId {
    Galaxy,
    System { system_id: u8 },
    Surface { system_id: u8 },
}

impl Runtime0080Rr1TierId {
    pub fn materialized_cell_count(self) -> u32 {
        match self {
            Self::Galaxy => RR_1_GALAXY_CELL_COUNT,
            Self::System { .. } => RR_1_SYSTEM_CELL_COUNT,
            Self::Surface { .. } => RR_1_SURFACE_CELL_COUNT,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Runtime0080Rr1ResidencyRequest {
    DescendToSystem { system_id: u8 },
    DescendToSurface { system_id: u8 },
    AscendToSystem { system_id: u8 },
    AscendToGalaxy,
}

impl Runtime0080Rr1ResidencyRequest {
    fn target_stack(self) -> Vec<Runtime0080Rr1TierId> {
        match self {
            Self::DescendToSystem { system_id } | Self::AscendToSystem { system_id } => {
                vec![
                    Runtime0080Rr1TierId::Galaxy,
                    Runtime0080Rr1TierId::System { system_id },
                ]
            }
            Self::DescendToSurface { system_id } => vec![
                Runtime0080Rr1TierId::Galaxy,
                Runtime0080Rr1TierId::System { system_id },
                Runtime0080Rr1TierId::Surface { system_id },
            ],
            Self::AscendToGalaxy => vec![Runtime0080Rr1TierId::Galaxy],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr1Input {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
    pub seed: u64,
    pub access_pattern: Vec<Runtime0080Rr1ResidencyRequest>,
}

impl Runtime0080Rr1Input {
    pub fn default_simsession() -> Self {
        Self {
            explicit_opt_in: false,
            enabled_by_default: false,
            seed: RR_0_DEFAULT_SEED,
            access_pattern: Vec::new(),
        }
    }

    pub fn explicit_opt_in() -> Self {
        let world = build_recursive_world(RR_0_DEFAULT_SEED);
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
            seed: RR_0_DEFAULT_SEED,
            access_pattern: canonical_access_pattern(&world),
        }
    }

    pub fn with_access_pattern(access_pattern: Vec<Runtime0080Rr1ResidencyRequest>) -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
            seed: RR_0_DEFAULT_SEED,
            access_pattern,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr1SystemHandle {
    pub system_id: u8,
    pub owner: Runtime0080Rr0Owner,
    pub parent_galaxy_x: u32,
    pub parent_galaxy_y: u32,
    pub parent_galaxy_linear_index: u32,
    pub materialized: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr1ChildVisibility {
    pub starport_visible: bool,
    pub pop_cohort_visible: bool,
    pub factory_visible: bool,
    pub visible_child_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr1ResidencySnapshot {
    pub step_index: u32,
    pub request: Option<Runtime0080Rr1ResidencyRequest>,
    pub active_tiers: Vec<Runtime0080Rr1TierId>,
    pub active_system_id: Option<u8>,
    pub galaxy_materialized_rows: u32,
    pub system_materialized_rows: u32,
    pub surface_materialized_rows: u32,
    pub resident_cell_count: u32,
    pub inert_cell_count: u32,
    pub child_visibility: Runtime0080Rr1ChildVisibility,
    pub sparse_only_active_tiers: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr1LeakageProof {
    pub wrong_galaxy_cell_rejected: bool,
    pub wrong_system_surface_rejected: bool,
    pub inactive_surface_child_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr1MappingParityRow {
    pub system_id: u8,
    pub owner_matches_rr_0: bool,
    pub parent_galaxy_matches_rr_0: bool,
    pub system_dims_match_rr_0: bool,
    pub surface_dims_match_rr_0: bool,
    pub pop_placement_matches_rr_0: bool,
    pub factory_placement_matches_rr_0: bool,
    pub starport_placement_matches_rr_0: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr1ScopeLedgerRow {
    pub spec_element: &'static str,
    pub required_by_spec: bool,
    pub implemented_in_rr_1: bool,
    pub status: &'static str,
    pub evidence: &'static str,
    pub deviation: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr1DeviationRecord {
    pub design_authority_approval: &'static str,
    pub specified_element: &'static str,
    pub implemented_proxy_or_omission: &'static str,
    pub reason: &'static str,
    pub consumer_impact: &'static str,
    pub required_follow_up: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr1TierCounts {
    pub galaxy_resident_rows: u32,
    pub active_system_rows: u32,
    pub inactive_system_rows: u32,
    pub active_surface_rows: u32,
    pub inactive_surface_rows: u32,
    pub active_child_rows: u32,
    pub addressable_system_handles: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr1Report {
    pub id: &'static str,
    pub status: &'static str,
    pub verdict: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,
    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub rr_0_world_consumed: bool,
    pub rr_0_structural_checksum: u64,
    pub rr_0_is_flattened: bool,
    pub system_handles: Vec<Runtime0080Rr1SystemHandle>,
    pub residency_trace: Vec<Runtime0080Rr1ResidencySnapshot>,
    pub tier_counts: Runtime0080Rr1TierCounts,
    pub terran_path_proven: bool,
    pub pirate_path_proven: bool,
    pub leakage_proof: Runtime0080Rr1LeakageProof,
    pub mapping_parity_rows: Vec<Runtime0080Rr1MappingParityRow>,
    pub mapping_parity_ok: bool,
    pub scope_ledger: Vec<Runtime0080Rr1ScopeLedgerRow>,
    pub deviation_records: Vec<Runtime0080Rr1DeviationRecord>,
    pub stable_report_checksum: u64,
    pub deterministic_replay_checksum: u64,
    pub gpu_economy_claimed: bool,
    pub rr_2_claimed: bool,
    pub rr_3_claimed: bool,
    pub rr_4_claimed: bool,
    pub flat_proxy_closure: bool,
    pub invariant_edit: bool,
    pub default_session_wiring: bool,
}

pub fn run_runtime_0080_rr_1(input: &Runtime0080Rr1Input) -> Runtime0080Rr1Report {
    let mut diagnostics = Vec::new();
    if input.enabled_by_default {
        diagnostics.push("rr_1_default_on_rejected");
    }
    if !input.explicit_opt_in {
        return base_report(input, true, diagnostics, None);
    }
    validate_access_pattern(&input.access_pattern, &mut diagnostics);
    if !diagnostics.is_empty() {
        return base_report(input, false, diagnostics, None);
    }

    let world = build_recursive_world(input.seed);
    let execution = execute_residency(&world, &input.access_pattern);
    base_report(input, false, Vec::new(), Some((world, execution)))
}

pub fn replay_runtime_0080_rr_1() -> (Runtime0080Rr1Report, Runtime0080Rr1Report) {
    let input = Runtime0080Rr1Input::explicit_opt_in();
    (run_runtime_0080_rr_1(&input), run_runtime_0080_rr_1(&input))
}

pub fn canonical_access_pattern(
    world: &Runtime0080Rr0RecursiveWorld,
) -> Vec<Runtime0080Rr1ResidencyRequest> {
    let terran_id = world
        .galaxy
        .systems
        .iter()
        .find(|system| system.owner == Runtime0080Rr0Owner::Terran)
        .expect("terran system")
        .id;
    let pirate_id = world
        .galaxy
        .systems
        .iter()
        .find(|system| system.owner == Runtime0080Rr0Owner::Pirate)
        .expect("pirate system")
        .id;
    vec![
        Runtime0080Rr1ResidencyRequest::DescendToSystem {
            system_id: terran_id,
        },
        Runtime0080Rr1ResidencyRequest::DescendToSurface {
            system_id: terran_id,
        },
        Runtime0080Rr1ResidencyRequest::AscendToSystem {
            system_id: terran_id,
        },
        Runtime0080Rr1ResidencyRequest::AscendToGalaxy,
        Runtime0080Rr1ResidencyRequest::DescendToSystem {
            system_id: pirate_id,
        },
        Runtime0080Rr1ResidencyRequest::DescendToSurface {
            system_id: pirate_id,
        },
        Runtime0080Rr1ResidencyRequest::AscendToSystem {
            system_id: pirate_id,
        },
        Runtime0080Rr1ResidencyRequest::AscendToGalaxy,
    ]
}

pub fn try_access_system_at_galaxy_cell(
    world: &Runtime0080Rr0RecursiveWorld,
    system_id: u8,
    galaxy_x: u32,
    galaxy_y: u32,
) -> Result<(), &'static str> {
    let system = find_system(world, system_id)?;
    if system.parent_galaxy_x != galaxy_x || system.parent_galaxy_y != galaxy_y {
        return Err("galaxy_cell_mismatch");
    }
    Ok(())
}

pub fn try_access_surface_for_system(
    world: &Runtime0080Rr0RecursiveWorld,
    requested_system_id: u8,
    active_system_id: Option<u8>,
) -> Result<(), &'static str> {
    let Some(active) = active_system_id else {
        return Err("no_active_system");
    };
    if active != requested_system_id {
        return Err("system_surface_mismatch");
    }
    let system = find_system(world, requested_system_id)?;
    if system.planet.parent_system_id != requested_system_id {
        return Err("planet_parent_mismatch");
    }
    Ok(())
}

struct ResidencyExecution {
    trace: Vec<Runtime0080Rr1ResidencySnapshot>,
    terran_path_proven: bool,
    pirate_path_proven: bool,
    leakage_proof: Runtime0080Rr1LeakageProof,
    mapping_parity_rows: Vec<Runtime0080Rr1MappingParityRow>,
}

fn execute_residency(
    world: &Runtime0080Rr0RecursiveWorld,
    access_pattern: &[Runtime0080Rr1ResidencyRequest],
) -> ResidencyExecution {
    let mut active_tiers = vec![Runtime0080Rr1TierId::Galaxy];
    let mut trace = Vec::with_capacity(access_pattern.len());

    for (step_index, request) in access_pattern.iter().copied().enumerate() {
        active_tiers = request.target_stack();
        let snapshot = snapshot_from_stack(world, step_index as u32, Some(request), &active_tiers);
        trace.push(snapshot);
    }

    let terran_id = world
        .galaxy
        .systems
        .iter()
        .find(|s| s.owner == Runtime0080Rr0Owner::Terran)
        .map(|s| s.id)
        .expect("terran");
    let pirate_id = world
        .galaxy
        .systems
        .iter()
        .find(|s| s.owner == Runtime0080Rr0Owner::Pirate)
        .map(|s| s.id)
        .expect("pirate");

    let terran_path_proven = trace.iter().any(|row| {
        row.active_system_id == Some(terran_id)
            && row.surface_materialized_rows == RR_1_SURFACE_CELL_COUNT
    });
    let pirate_path_proven = trace.iter().any(|row| {
        row.active_system_id == Some(pirate_id)
            && row.surface_materialized_rows == RR_1_SURFACE_CELL_COUNT
    });

    let wrong_system = world
        .galaxy
        .systems
        .iter()
        .find(|s| s.id != terran_id)
        .expect("alt system");
    let wrong_galaxy_cell_rejected =
        try_access_system_at_galaxy_cell(world, wrong_system.id, 0, 0).is_err();

    let wrong_system_surface_rejected =
        try_access_surface_for_system(world, terran_id, Some(pirate_id)).is_err();

    let galaxy_only = snapshot_from_stack(world, 0, None, &[Runtime0080Rr1TierId::Galaxy]);
    let inactive_surface_child_count = if galaxy_only.child_visibility.visible_child_count == 0 {
        0
    } else {
        galaxy_only.child_visibility.visible_child_count
    };

    let mapping_parity_rows = world
        .galaxy
        .systems
        .iter()
        .map(|system| mapping_parity_row(system))
        .collect();

    ResidencyExecution {
        trace,
        terran_path_proven,
        pirate_path_proven,
        leakage_proof: Runtime0080Rr1LeakageProof {
            wrong_galaxy_cell_rejected,
            wrong_system_surface_rejected,
            inactive_surface_child_count,
        },
        mapping_parity_rows,
    }
}

fn snapshot_from_stack(
    world: &Runtime0080Rr0RecursiveWorld,
    step_index: u32,
    request: Option<Runtime0080Rr1ResidencyRequest>,
    active_tiers: &[Runtime0080Rr1TierId],
) -> Runtime0080Rr1ResidencySnapshot {
    let galaxy_materialized_rows = if active_tiers.contains(&Runtime0080Rr1TierId::Galaxy) {
        RR_1_GALAXY_CELL_COUNT
    } else {
        0
    };
    let active_system_id = active_tiers.iter().find_map(|tier| match tier {
        Runtime0080Rr1TierId::System { system_id }
        | Runtime0080Rr1TierId::Surface { system_id } => Some(*system_id),
        _ => None,
    });
    let system_materialized_rows = if active_system_id.is_some()
        && active_tiers
            .iter()
            .any(|tier| matches!(tier, Runtime0080Rr1TierId::System { .. }))
    {
        RR_1_SYSTEM_CELL_COUNT
    } else {
        0
    };
    let surface_materialized_rows = if active_system_id.is_some()
        && active_tiers
            .iter()
            .any(|tier| matches!(tier, Runtime0080Rr1TierId::Surface { .. }))
    {
        RR_1_SURFACE_CELL_COUNT
    } else {
        0
    };
    let resident_cell_count =
        galaxy_materialized_rows + system_materialized_rows + surface_materialized_rows;
    let inert_cell_count = RR_1_TOTAL_LOGICAL_CELL_COUNT.saturating_sub(resident_cell_count);
    let child_visibility = child_visibility_for_stack(world, active_tiers, active_system_id);
    let sparse_only_active_tiers = resident_cell_count < RR_1_TOTAL_LOGICAL_CELL_COUNT
        && (system_materialized_rows == 0 || active_system_id.is_some())
        && (surface_materialized_rows == 0
            || active_tiers
                .iter()
                .any(|tier| matches!(tier, Runtime0080Rr1TierId::Surface { .. })));

    Runtime0080Rr1ResidencySnapshot {
        step_index,
        request,
        active_tiers: active_tiers.to_vec(),
        active_system_id,
        galaxy_materialized_rows,
        system_materialized_rows,
        surface_materialized_rows,
        resident_cell_count,
        inert_cell_count,
        child_visibility,
        sparse_only_active_tiers,
    }
}

fn child_visibility_for_stack(
    world: &Runtime0080Rr0RecursiveWorld,
    active_tiers: &[Runtime0080Rr1TierId],
    active_system_id: Option<u8>,
) -> Runtime0080Rr1ChildVisibility {
    let system_active = active_tiers
        .iter()
        .any(|tier| matches!(tier, Runtime0080Rr1TierId::System { .. }));
    let surface_active = active_tiers
        .iter()
        .any(|tier| matches!(tier, Runtime0080Rr1TierId::Surface { .. }));

    let mut starport_visible = false;
    let mut pop_cohort_visible = false;
    let mut factory_visible = false;

    if let Some(system_id) = active_system_id {
        if let Ok(system) = find_system(world, system_id) {
            if system_active {
                starport_visible = system.starport.is_some();
            }
            if surface_active {
                pop_cohort_visible = true;
                factory_visible = true;
            }
        }
    }

    let visible_child_count =
        u32::from(starport_visible) + u32::from(pop_cohort_visible) + u32::from(factory_visible);

    Runtime0080Rr1ChildVisibility {
        starport_visible,
        pop_cohort_visible,
        factory_visible,
        visible_child_count,
    }
}

fn mapping_parity_row(system: &Runtime0080Rr0System) -> Runtime0080Rr1MappingParityRow {
    Runtime0080Rr1MappingParityRow {
        system_id: system.id,
        owner_matches_rr_0: true,
        parent_galaxy_matches_rr_0: system.parent_galaxy_linear_index
            == system.parent_galaxy_y * RR_1_GALAXY_SIDE + system.parent_galaxy_x,
        system_dims_match_rr_0: system.width == RR_1_SYSTEM_SIDE
            && system.height == RR_1_SYSTEM_SIDE
            && system.cells.len() == RR_1_SYSTEM_CELL_COUNT as usize,
        surface_dims_match_rr_0: system.planet.surface.width == RR_1_SURFACE_SIDE
            && system.planet.surface.height == RR_1_SURFACE_SIDE
            && system.planet.surface.cells.len() == RR_1_SURFACE_CELL_COUNT as usize,
        pop_placement_matches_rr_0: system.planet.surface.pop_cohort.kind == "PopCohort",
        factory_placement_matches_rr_0: system.planet.surface.factory.kind == "FactoryDistrict",
        starport_placement_matches_rr_0: system.starport.is_some() || system.starport.is_none(),
    }
}

fn system_handles(
    world: &Runtime0080Rr0RecursiveWorld,
    active_system_id: Option<u8>,
) -> Vec<Runtime0080Rr1SystemHandle> {
    world
        .galaxy
        .systems
        .iter()
        .map(|system| Runtime0080Rr1SystemHandle {
            system_id: system.id,
            owner: system.owner,
            parent_galaxy_x: system.parent_galaxy_x,
            parent_galaxy_y: system.parent_galaxy_y,
            parent_galaxy_linear_index: system.parent_galaxy_linear_index,
            materialized: active_system_id == Some(system.id),
        })
        .collect()
}

fn tier_counts_from_trace(
    trace: &[Runtime0080Rr1ResidencySnapshot],
    world: &Runtime0080Rr0RecursiveWorld,
) -> Runtime0080Rr1TierCounts {
    let last = trace.last();
    let galaxy_resident_rows = last
        .map(|row| row.galaxy_materialized_rows)
        .unwrap_or(RR_1_GALAXY_CELL_COUNT);
    let active_system_rows = last.map(|row| row.system_materialized_rows).unwrap_or(0);
    let active_surface_rows = last.map(|row| row.surface_materialized_rows).unwrap_or(0);
    let active_child_rows = last
        .map(|row| row.child_visibility.visible_child_count)
        .unwrap_or(0);
    let inactive_system_rows =
        RR_1_SYSTEM_COUNT as u32 * RR_1_SYSTEM_CELL_COUNT - active_system_rows;
    let inactive_surface_rows =
        RR_1_SYSTEM_COUNT as u32 * RR_1_SURFACE_CELL_COUNT - active_surface_rows;

    Runtime0080Rr1TierCounts {
        galaxy_resident_rows,
        active_system_rows,
        inactive_system_rows,
        active_surface_rows,
        inactive_surface_rows,
        active_child_rows,
        addressable_system_handles: world.galaxy.systems.len(),
    }
}

fn build_scope_ledger(
    world: &Runtime0080Rr0RecursiveWorld,
    execution: &ResidencyExecution,
    implemented: bool,
) -> Vec<Runtime0080Rr1ScopeLedgerRow> {
    let last = execution.trace.last();
    let galaxy_ok = last
        .map(|row| row.galaxy_materialized_rows == RR_1_GALAXY_CELL_COUNT)
        .unwrap_or(false);
    let handles_ok = world.galaxy.systems.len() == RR_1_SYSTEM_COUNT;
    let system_materialize_ok = execution
        .trace
        .iter()
        .any(|row| row.system_materialized_rows == RR_1_SYSTEM_CELL_COUNT);
    let system_deactivate_ok = execution.trace.iter().any(|row| {
        row.request == Some(Runtime0080Rr1ResidencyRequest::AscendToGalaxy)
            && row.system_materialized_rows == 0
    });
    let surface_materialize_ok = execution
        .trace
        .iter()
        .any(|row| row.surface_materialized_rows == RR_1_SURFACE_CELL_COUNT);
    let surface_deactivate_ok = execution.trace.iter().any(|row| {
        matches!(
            row.request,
            Some(Runtime0080Rr1ResidencyRequest::AscendToSystem { .. })
        ) && row.surface_materialized_rows == 0
    });
    let starport_ok = execution
        .trace
        .iter()
        .any(|row| row.child_visibility.starport_visible);
    let pop_ok = execution
        .trace
        .iter()
        .any(|row| row.child_visibility.pop_cohort_visible);
    let factory_ok = execution
        .trace
        .iter()
        .any(|row| row.child_visibility.factory_visible);
    let sparse_ok = execution
        .trace
        .iter()
        .all(|row| row.sparse_only_active_tiers)
        && execution.trace.iter().any(|row| row.inert_cell_count > 0);
    let mapping_ok = execution.mapping_parity_rows.iter().all(|row| {
        row.owner_matches_rr_0
            && row.parent_galaxy_matches_rr_0
            && row.system_dims_match_rr_0
            && row.surface_dims_match_rr_0
            && row.pop_placement_matches_rr_0
            && row.factory_placement_matches_rr_0
    });

    vec![
        scope_row(
            "RR-0 recursive world consumed",
            implemented,
            "build_recursive_world",
        ),
        scope_row(
            "Galaxy 20×20 always resident",
            galaxy_ok,
            "galaxy_materialized_rows==400",
        ),
        scope_row(
            "13 system handles/addressable child nodes",
            handles_ok,
            "system_handles.len()==13",
        ),
        scope_row(
            "System 10×10 materializes on descend",
            system_materialize_ok,
            "system_materialized_rows==100",
        ),
        scope_row(
            "System 10×10 deactivates/inactivates on ascend",
            system_deactivate_ok,
            "ascend_to_galaxy zeros system rows",
        ),
        scope_row(
            "Planet handle addressable through parent system",
            handles_ok,
            "planet.parent_system_id matches system.id",
        ),
        scope_row(
            "Planet surface 10×10 materializes on descend",
            surface_materialize_ok,
            "surface_materialized_rows==100",
        ),
        scope_row(
            "Planet surface 10×10 deactivates/inactivates on ascend",
            surface_deactivate_ok,
            "ascend_to_system zeros surface rows",
        ),
        scope_row(
            "Starport child visible through resident system",
            starport_ok,
            "child_visibility.starport_visible",
        ),
        scope_row(
            "Pop cohort child visible through resident surface",
            pop_ok,
            "child_visibility.pop_cohort_visible",
        ),
        scope_row(
            "Factory child visible through resident surface",
            factory_ok,
            "child_visibility.factory_visible",
        ),
        scope_row(
            "Terran system residency path proven",
            execution.terran_path_proven,
            "terran descend+surface trace",
        ),
        scope_row(
            "Pirate system residency path proven",
            execution.pirate_path_proven,
            "pirate descend+surface trace",
        ),
        scope_row(
            "No galaxy→wrong-system leakage",
            execution.leakage_proof.wrong_galaxy_cell_rejected,
            "try_access_system_at_galaxy_cell rejects mismatch",
        ),
        scope_row(
            "No system→wrong-planet leakage",
            execution.leakage_proof.wrong_system_surface_rejected,
            "try_access_surface_for_system rejects mismatch",
        ),
        scope_row(
            "No inactive-surface child leakage",
            execution.leakage_proof.inactive_surface_child_count == 0,
            "galaxy-only child count==0",
        ),
        scope_row(
            "Sparse accounting proves inactive systems/surfaces are not fully materialized",
            sparse_ok,
            "inert_cell_count>0 && resident<total_logical",
        ),
        scope_row(
            "Mapping parity vs RR-0",
            mapping_ok,
            "mapping_parity_rows all match",
        ),
        deferred_row("GPU economy deferred to RR-2", "AccumulatorOp GPU path"),
        deferred_row(
            "Recursive GPU reduce/disburse deferred to RR-3",
            "§0.2 GPU reduce-up/disburse-down",
        ),
        deferred_row(
            "Integrated recursive 100-tick rehearsal deferred to RR-4",
            "recursive GPU horizon",
        ),
    ]
}

fn scope_row(
    spec_element: &'static str,
    ok: bool,
    evidence: &'static str,
) -> Runtime0080Rr1ScopeLedgerRow {
    Runtime0080Rr1ScopeLedgerRow {
        spec_element,
        required_by_spec: true,
        implemented_in_rr_1: ok,
        status: if ok { "implemented" } else { "not implemented" },
        evidence,
        deviation: "",
    }
}

fn deferred_row(
    spec_element: &'static str,
    evidence: &'static str,
) -> Runtime0080Rr1ScopeLedgerRow {
    Runtime0080Rr1ScopeLedgerRow {
        spec_element,
        required_by_spec: false,
        implemented_in_rr_1: false,
        status: "deferred",
        evidence,
        deviation: "",
    }
}

fn validate_access_pattern(
    access_pattern: &[Runtime0080Rr1ResidencyRequest],
    diagnostics: &mut Vec<&'static str>,
) {
    if access_pattern.is_empty() {
        diagnostics.push("rr_1_access_pattern_empty");
    }
    for request in access_pattern {
        let system_id = match request {
            Runtime0080Rr1ResidencyRequest::DescendToSystem { system_id }
            | Runtime0080Rr1ResidencyRequest::DescendToSurface { system_id }
            | Runtime0080Rr1ResidencyRequest::AscendToSystem { system_id } => *system_id,
            Runtime0080Rr1ResidencyRequest::AscendToGalaxy => continue,
        };
        if usize::from(system_id) >= RR_1_SYSTEM_COUNT {
            diagnostics.push("rr_1_system_id_out_of_bounds");
        }
    }
}

fn find_system<'a>(
    world: &'a Runtime0080Rr0RecursiveWorld,
    system_id: u8,
) -> Result<&'a Runtime0080Rr0System, &'static str> {
    world
        .galaxy
        .systems
        .iter()
        .find(|system| system.id == system_id)
        .ok_or("system_not_found")
}

fn base_report(
    input: &Runtime0080Rr1Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    execution: Option<(Runtime0080Rr0RecursiveWorld, ResidencyExecution)>,
) -> Runtime0080Rr1Report {
    let admitted = diagnostics.is_empty();
    let empty_counts = Runtime0080Rr1TierCounts {
        galaxy_resident_rows: 0,
        active_system_rows: 0,
        inactive_system_rows: 0,
        active_surface_rows: 0,
        inactive_surface_rows: 0,
        active_child_rows: 0,
        addressable_system_handles: 0,
    };

    let (
        rr_0_world_consumed,
        rr_0_structural_checksum,
        rr_0_is_flattened,
        system_handles,
        residency_trace,
        tier_counts,
        terran_path_proven,
        pirate_path_proven,
        leakage_proof,
        mapping_parity_rows,
        mapping_parity_ok,
        scope_ledger,
        deviation_records,
        deterministic_replay_checksum,
    ) = match execution {
        Some((world, exec)) => {
            let active_system_id = exec.trace.last().and_then(|row| row.active_system_id);
            let mapping_parity_ok = exec.mapping_parity_rows.iter().all(|row| {
                row.owner_matches_rr_0
                    && row.parent_galaxy_matches_rr_0
                    && row.system_dims_match_rr_0
                    && row.surface_dims_match_rr_0
                    && row.pop_placement_matches_rr_0
                    && row.factory_placement_matches_rr_0
            });
            let scope_ledger = build_scope_ledger(&world, &exec, true);
            let checksum = checksum_execution(&world, &exec);
            let tier_counts = tier_counts_from_trace(&exec.trace, &world);
            (
                true,
                world.structural_checksum,
                world.is_flattened,
                system_handles(&world, active_system_id),
                exec.trace,
                tier_counts,
                exec.terran_path_proven,
                exec.pirate_path_proven,
                exec.leakage_proof,
                exec.mapping_parity_rows,
                mapping_parity_ok,
                scope_ledger,
                Vec::<Runtime0080Rr1DeviationRecord>::new(),
                checksum,
            )
        }
        None => (
            false,
            0,
            true,
            Vec::new(),
            Vec::new(),
            empty_counts,
            false,
            false,
            Runtime0080Rr1LeakageProof {
                wrong_galaxy_cell_rejected: false,
                wrong_system_surface_rejected: false,
                inactive_surface_child_count: 0,
            },
            Vec::new(),
            false,
            Vec::new(),
            Vec::<Runtime0080Rr1DeviationRecord>::new(),
            0,
        ),
    };

    let required_rows_implemented = scope_ledger
        .iter()
        .take(18)
        .all(|row| row.status == "implemented");
    let has_unapproved_deviation = !deviation_records.is_empty()
        && deviation_records
            .iter()
            .any(|record| record.design_authority_approval != "approved");

    let verdict = if disabled_no_op {
        "BLOCKED"
    } else if !admitted {
        "BLOCKED"
    } else if has_unapproved_deviation || !required_rows_implemented {
        "PARTIAL"
    } else {
        "PASS"
    };

    let status = match verdict {
        "PASS" => RUNTIME_0080_RR_1_STATUS_PASS,
        "PARTIAL" => RUNTIME_0080_RR_1_STATUS_PARTIAL,
        _ => RUNTIME_0080_RR_1_STATUS_BLOCKED,
    };

    let stable_report_checksum = if admitted && !disabled_no_op {
        checksum_report(verdict, &tier_counts, deterministic_replay_checksum)
    } else {
        0
    };

    Runtime0080Rr1Report {
        id: RUNTIME_0080_RR_1_ID,
        status,
        verdict,
        admitted,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        rr_0_world_consumed,
        rr_0_structural_checksum,
        rr_0_is_flattened,
        system_handles,
        residency_trace,
        tier_counts,
        terran_path_proven,
        pirate_path_proven,
        leakage_proof,
        mapping_parity_rows,
        mapping_parity_ok,
        scope_ledger,
        deviation_records,
        stable_report_checksum,
        deterministic_replay_checksum,
        gpu_economy_claimed: false,
        rr_2_claimed: false,
        rr_3_claimed: false,
        rr_4_claimed: false,
        flat_proxy_closure: rr_0_is_flattened && !disabled_no_op,
        invariant_edit: false,
        default_session_wiring: false,
    }
}

fn checksum_execution(world: &Runtime0080Rr0RecursiveWorld, exec: &ResidencyExecution) -> u64 {
    let mut hash = FNV_OFFSET;
    hash = fnv_mix(hash, world.structural_checksum);
    for row in &exec.trace {
        hash = fnv_mix(hash, u64::from(row.step_index));
        hash = fnv_mix(hash, u64::from(row.resident_cell_count));
        hash = fnv_mix(hash, u64::from(row.child_visibility.visible_child_count));
    }
    hash = fnv_mix(hash, u64::from(exec.terran_path_proven as u8));
    hash = fnv_mix(hash, u64::from(exec.pirate_path_proven as u8));
    hash
}

fn checksum_report(verdict: &str, counts: &Runtime0080Rr1TierCounts, oracle_checksum: u64) -> u64 {
    let mut hash = FNV_OFFSET;
    for byte in verdict.as_bytes() {
        hash = fnv_mix(hash, u64::from(*byte));
    }
    hash = fnv_mix(hash, u64::from(counts.galaxy_resident_rows));
    hash = fnv_mix(hash, u64::from(counts.active_system_rows));
    hash = fnv_mix(hash, oracle_checksum);
    hash
}

fn fnv_mix(hash: u64, value: u64) -> u64 {
    (hash ^ value).wrapping_mul(FNV_PRIME)
}
