//! SCENARIO-0080-2-R2: recursive allocation + faction economy + blockade/divert.
//!
//! Fixture-only, opt-in/default-off proof over the accepted R1 disruption heatmap. R2 consumes R1
//! report structures, converts local labor into production, reduces production up through owner-masked
//! faction stockpiles, disburses stockpile surplus down to deficit starport systems, and applies the
//! disruption blockade/divert rule as a production owner-column re-route. CPU oracle parity is the
//! authority; this module adds no GPU, shader, movement, combat, or default SimSession wiring.

#[allow(dead_code, unused_imports)]
#[path = "dress_rehearsal_atlas_batch_0_store.rs"]
mod atlas_store;

use crate::dress_rehearsal_r1_disruption_heatmap::{
    run_dress_rehearsal_r1_disruption_heatmap, DressRehearsalR1Channel, DressRehearsalR1Input,
    DressRehearsalR1OccupantKind, DressRehearsalR1Owner, DressRehearsalR1Report, GALAXY_CELL_COUNT,
    GALAXY_SIDE,
};
use std::collections::{HashMap, HashSet};

pub const DRESS_REHEARSAL_R2_RECURSIVE_ALLOCATION_ID: &str =
    "SCENARIO-0080-2-R2-RECURSIVE-ALLOCATION";
pub const DRESS_REHEARSAL_R2_RECURSIVE_ALLOCATION_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - recursive allocation + faction economy + blockade/divert";
pub const DRESS_REHEARSAL_R2_SCENARIO: &str = "SCENARIO-0080-2";

pub const POP_LABOR_PER_TICK: i64 = 10;
pub const FACTORY_UNIT_COST_LABOR: i64 = 10;
pub const PRODUCTION_PER_RECIPE: i64 = 1;
pub const STARPORT_PRODUCTION_NEED: i64 = 2;
pub const BLOCKADE_THRESHOLD: f32 = 100.0;
pub const TOP_AFFECTED_COUNT: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DressRehearsalR2Owner {
    Terran,
    Pirate,
}

impl DressRehearsalR2Owner {
    pub fn stable_code(self) -> u64 {
        match self {
            Self::Terran => 1,
            Self::Pirate => 2,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR2StockpileSeed {
    pub owner: DressRehearsalR2Owner,
    pub before_reduce_up: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR2Input {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
    pub r1_report: Option<DressRehearsalR1Report>,
    pub stockpile_seeds: Vec<DressRehearsalR2StockpileSeed>,
}

impl DressRehearsalR2Input {
    pub fn default_simsession() -> Self {
        Self {
            explicit_opt_in: false,
            enabled_by_default: false,
            r1_report: None,
            stockpile_seeds: default_stockpile_seeds(),
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
            r1_report: Some(run_dress_rehearsal_r1_disruption_heatmap(
                &DressRehearsalR1Input::explicit_opt_in(),
            )),
            stockpile_seeds: default_stockpile_seeds(),
        }
    }

    pub fn with_r1_report(r1_report: DressRehearsalR1Report) -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
            r1_report: Some(r1_report),
            stockpile_seeds: default_stockpile_seeds(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR2FactoryRecipe {
    pub pop_labor_per_tick: i64,
    pub unit_cost_labor: i64,
    pub production_per_crossing: i64,
    pub input_consumption: &'static str,
    pub recipe_shape: &'static str,
    pub no_new_op: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR2OccupantPosition {
    pub source_id: String,
    pub kind: &'static str,
    pub owner: DressRehearsalR2Owner,
    pub x: u32,
    pub y: u32,
    pub cell_index: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR2SystemProductionRow {
    pub system_id: String,
    pub system_index: usize,
    pub x: u32,
    pub y: u32,
    pub cell_index: u32,
    pub structural_parent_before: &'static str,
    pub structural_parent_after: &'static str,
    pub original_owner: DressRehearsalR2Owner,
    pub effective_outflow_owner: DressRehearsalR2Owner,
    pub blockader: Option<DressRehearsalR2Owner>,
    pub disruption: f32,
    pub blockaded: bool,
    pub has_starport: bool,
    pub starport_need: i64,
    pub labor_generated: i64,
    pub labor_consumed: i64,
    pub labor_remaining: i64,
    pub production_generated: i64,
    pub outflow_to_original_owner: i64,
    pub outflow_to_effective_owner: i64,
    pub diverted_production: i64,
    pub owner_column_flipped: bool,
    pub disbursement_received: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR2DivertedProductionRow {
    pub system_id: String,
    pub system_index: usize,
    pub cell_index: u32,
    pub original_owner: DressRehearsalR2Owner,
    pub blockader_owner: DressRehearsalR2Owner,
    pub production: i64,
    pub owner_column_before: DressRehearsalR2Owner,
    pub owner_column_after: DressRehearsalR2Owner,
    pub structural_parent_before: &'static str,
    pub structural_parent_after: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR2StockpileLedgerRow {
    pub owner: DressRehearsalR2Owner,
    pub before_reduce_up: i64,
    pub reduced_in: i64,
    pub after_reduce_up: i64,
    pub disbursed_down: i64,
    pub after_disburse_down: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR2DeficitDisbursementRow {
    pub owner: DressRehearsalR2Owner,
    pub system_id: String,
    pub system_index: usize,
    pub requested: i64,
    pub disbursed: i64,
    pub remaining_deficit: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR2AffectedSystemRow {
    pub rank: usize,
    pub system_id: String,
    pub system_index: usize,
    pub original_owner: DressRehearsalR2Owner,
    pub effective_outflow_owner: DressRehearsalR2Owner,
    pub disruption: f32,
    pub diverted_production: i64,
    pub disbursement_received: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR2Summary {
    pub system_count: usize,
    pub diverted_system_count: usize,
    pub total_production: i64,
    pub total_diverted_production: i64,
    pub total_disbursed: i64,
    pub stable_checksum: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR2Artifact {
    pub production_rows: Vec<DressRehearsalR2SystemProductionRow>,
    pub diverted_rows: Vec<DressRehearsalR2DivertedProductionRow>,
    pub stockpile_ledger: Vec<DressRehearsalR2StockpileLedgerRow>,
    pub deficit_disbursements: Vec<DressRehearsalR2DeficitDisbursementRow>,
    pub top_affected_systems: Vec<DressRehearsalR2AffectedSystemRow>,
    pub summary: DressRehearsalR2Summary,
    pub cpu_oracle_parity: bool,
    pub markdown: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR2Oracle {
    pub production_rows: Vec<DressRehearsalR2SystemProductionRow>,
    pub diverted_rows: Vec<DressRehearsalR2DivertedProductionRow>,
    pub stockpile_ledger: Vec<DressRehearsalR2StockpileLedgerRow>,
    pub deficit_disbursements: Vec<DressRehearsalR2DeficitDisbursementRow>,
    pub top_affected_systems: Vec<DressRehearsalR2AffectedSystemRow>,
    pub summary: DressRehearsalR2Summary,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR2Report {
    pub id: &'static str,
    pub status: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,

    pub r1_heatmap_consumed: bool,
    pub r1_input_contract_checksum: u64,
    pub r1_cpu_oracle_parity: bool,
    pub r1_final_disruption_cells: usize,
    pub galaxy_side: u32,
    pub single_galactic_tier: bool,
    pub interior_subtile_materialized: bool,
    pub interior_subtile_count: usize,

    pub factory_recipe: DressRehearsalR2FactoryRecipe,
    pub production_rows: Vec<DressRehearsalR2SystemProductionRow>,
    pub diverted_production_rows: Vec<DressRehearsalR2DivertedProductionRow>,
    pub stockpile_ledger: Vec<DressRehearsalR2StockpileLedgerRow>,
    pub deficit_disbursements: Vec<DressRehearsalR2DeficitDisbursementRow>,
    pub top_affected_systems: Vec<DressRehearsalR2AffectedSystemRow>,
    pub artifact: DressRehearsalR2Artifact,
    pub summary: DressRehearsalR2Summary,

    pub occupant_positions_before: Vec<DressRehearsalR2OccupantPosition>,
    pub occupant_positions_after: Vec<DressRehearsalR2OccupantPosition>,
    pub boundary_request_emitted: bool,
    pub combat_resolution_events: usize,
    pub hostile_hp_delta: i64,
    pub reparented_system_count: usize,
    pub cpu_planner_used: bool,
    pub new_shader_or_wgsl: bool,
    pub default_simsession_pass_graph_change: bool,

    pub cpu_oracle_parity: bool,
    pub deterministic_replay_checksum: u64,
}

pub fn run_dress_rehearsal_r2_recursive_allocation(
    input: &DressRehearsalR2Input,
) -> DressRehearsalR2Report {
    let mut diagnostics = Vec::new();
    validate_input(input, &mut diagnostics);

    if !input.explicit_opt_in {
        return base_report(input, true, diagnostics, None, false);
    }
    if !diagnostics.is_empty() {
        return base_report(input, false, diagnostics, None, false);
    }

    let r1_report = input
        .r1_report
        .as_ref()
        .expect("validated R1 report must be present");
    let execution = execute_model(r1_report, &input.stockpile_seeds);
    let oracle = cpu_oracle_dress_rehearsal_r2_recursive_allocation(input);
    let parity = execution.production_rows == oracle.production_rows
        && execution.diverted_rows == oracle.diverted_rows
        && execution.stockpile_ledger == oracle.stockpile_ledger
        && execution.deficit_disbursements == oracle.deficit_disbursements
        && execution.summary == oracle.summary;
    base_report(input, false, Vec::new(), Some(execution), parity)
}

pub fn replay_dress_rehearsal_r2_recursive_allocation(
) -> (DressRehearsalR2Report, DressRehearsalR2Report) {
    let input = DressRehearsalR2Input::explicit_opt_in();
    (
        run_dress_rehearsal_r2_recursive_allocation(&input),
        run_dress_rehearsal_r2_recursive_allocation(&input),
    )
}

pub fn cpu_oracle_dress_rehearsal_r2_recursive_allocation(
    input: &DressRehearsalR2Input,
) -> DressRehearsalR2Oracle {
    if !input.explicit_opt_in || input.enabled_by_default {
        return empty_oracle();
    }
    let Some(r1_report) = input.r1_report.as_ref() else {
        return empty_oracle();
    };
    if !r1_report.admitted || !r1_report.cpu_oracle_parity {
        return empty_oracle();
    }
    let execution = execute_model(r1_report, &input.stockpile_seeds);
    DressRehearsalR2Oracle {
        production_rows: execution.production_rows,
        diverted_rows: execution.diverted_rows,
        stockpile_ledger: execution.stockpile_ledger,
        deficit_disbursements: execution.deficit_disbursements,
        top_affected_systems: execution.top_affected_systems,
        summary: execution.summary,
    }
}

pub fn factory_recipe_production(labor: i64) -> (i64, i64, i64) {
    let safe_labor = labor.max(0);
    let crossings = safe_labor / FACTORY_UNIT_COST_LABOR;
    let production = crossings * PRODUCTION_PER_RECIPE;
    let consumed = crossings * FACTORY_UNIT_COST_LABOR;
    (production, consumed, safe_labor - consumed)
}

pub fn render_dress_rehearsal_r2_artifact(report: &DressRehearsalR2Report) -> String {
    report.artifact.markdown.clone()
}

fn default_stockpile_seeds() -> Vec<DressRehearsalR2StockpileSeed> {
    vec![
        DressRehearsalR2StockpileSeed {
            owner: DressRehearsalR2Owner::Terran,
            before_reduce_up: 0,
        },
        DressRehearsalR2StockpileSeed {
            owner: DressRehearsalR2Owner::Pirate,
            before_reduce_up: 0,
        },
    ]
}

fn validate_input(input: &DressRehearsalR2Input, diagnostics: &mut Vec<&'static str>) {
    if input.enabled_by_default {
        diagnostics.push("r2_default_on_rejected");
    }
    if !input.explicit_opt_in {
        return;
    }
    let Some(r1_report) = input.r1_report.as_ref() else {
        diagnostics.push("r1_report_missing");
        return;
    };
    if !r1_report.admitted {
        diagnostics.push("r1_report_not_admitted");
    }
    if !r1_report.cpu_oracle_parity {
        diagnostics.push("r1_cpu_oracle_parity_missing");
    }
    if r1_report.final_disruption.len() != GALAXY_CELL_COUNT {
        diagnostics.push("r1_final_disruption_shape_mismatch");
    }
    if r1_report.scenario.system_cells.is_empty() {
        diagnostics.push("r1_system_cells_missing");
    }
}

fn base_report(
    input: &DressRehearsalR2Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    execution: Option<Execution>,
    cpu_oracle_parity: bool,
) -> DressRehearsalR2Report {
    let admitted = diagnostics.is_empty();
    let opt_in = input.explicit_opt_in;
    let empty_summary = DressRehearsalR2Summary {
        system_count: 0,
        diverted_system_count: 0,
        total_production: 0,
        total_diverted_production: 0,
        total_disbursed: 0,
        stable_checksum: 0,
    };

    let r1 = input.r1_report.as_ref();
    let (production_rows, diverted_rows, stockpile_ledger, deficit_disbursements, top, summary) =
        match execution.as_ref() {
            Some(execution) => (
                execution.production_rows.clone(),
                execution.diverted_rows.clone(),
                execution.stockpile_ledger.clone(),
                execution.deficit_disbursements.clone(),
                execution.top_affected_systems.clone(),
                execution.summary.clone(),
            ),
            None => (
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                empty_summary.clone(),
            ),
        };

    let positions = r1
        .filter(|_| !disabled_no_op)
        .map(occupant_positions)
        .unwrap_or_default();
    let markdown = render_artifact_markdown(
        &production_rows,
        &diverted_rows,
        &stockpile_ledger,
        &deficit_disbursements,
        &top,
        &summary,
        cpu_oracle_parity,
        r1.map(|report| report.starmap_summary.stable_checksum)
            .unwrap_or(0),
    );
    let artifact = DressRehearsalR2Artifact {
        production_rows: production_rows.clone(),
        diverted_rows: diverted_rows.clone(),
        stockpile_ledger: stockpile_ledger.clone(),
        deficit_disbursements: deficit_disbursements.clone(),
        top_affected_systems: top.clone(),
        summary: summary.clone(),
        cpu_oracle_parity,
        markdown,
    };

    DressRehearsalR2Report {
        id: DRESS_REHEARSAL_R2_RECURSIVE_ALLOCATION_ID,
        status: DRESS_REHEARSAL_R2_RECURSIVE_ALLOCATION_STATUS_PASS,
        scenario_name: DRESS_REHEARSAL_R2_SCENARIO,
        admitted,
        diagnostics,
        explicit_opt_in: opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        r1_heatmap_consumed: admitted
            && opt_in
            && r1
                .map(|report| {
                    report.admitted
                        && report.cpu_oracle_parity
                        && report.final_disruption.len() == GALAXY_CELL_COUNT
                })
                .unwrap_or(false),
        r1_input_contract_checksum: r1
            .map(|report| report.starmap_summary.stable_checksum)
            .unwrap_or(0),
        r1_cpu_oracle_parity: r1.map(|report| report.cpu_oracle_parity).unwrap_or(false),
        r1_final_disruption_cells: r1.map(|report| report.final_disruption.len()).unwrap_or(0),
        galaxy_side: if disabled_no_op { 0 } else { GALAXY_SIDE },
        single_galactic_tier: admitted && !disabled_no_op,
        interior_subtile_materialized: false,
        interior_subtile_count: 0,
        factory_recipe: factory_recipe(),
        production_rows,
        diverted_production_rows: diverted_rows,
        stockpile_ledger,
        deficit_disbursements,
        top_affected_systems: top,
        artifact,
        summary: summary.clone(),
        occupant_positions_before: positions.clone(),
        occupant_positions_after: positions,
        boundary_request_emitted: false,
        combat_resolution_events: 0,
        hostile_hp_delta: 0,
        reparented_system_count: 0,
        cpu_planner_used: false,
        new_shader_or_wgsl: false,
        default_simsession_pass_graph_change: false,
        cpu_oracle_parity,
        deterministic_replay_checksum: if admitted && opt_in {
            summary.stable_checksum
        } else {
            0
        },
    }
}

fn execute_model(
    r1_report: &DressRehearsalR1Report,
    seeds: &[DressRehearsalR2StockpileSeed],
) -> Execution {
    let starports = canonical_starport_system_indices();
    let mut production_rows = build_production_rows(r1_report, &starports);
    let diverted_rows = build_diverted_rows(&production_rows);
    let reduced_by_owner = reduce_up_owner_masked(&production_rows);
    let (stockpile_ledger, deficit_disbursements) =
        disburse_down(&mut production_rows, seeds, &reduced_by_owner);
    let top_affected_systems = build_top_affected_systems(&production_rows);
    let summary = build_summary(
        r1_report.starmap_summary.stable_checksum,
        &production_rows,
        &stockpile_ledger,
        &deficit_disbursements,
    );
    Execution {
        production_rows,
        diverted_rows,
        stockpile_ledger,
        deficit_disbursements,
        top_affected_systems,
        summary,
    }
}

fn build_production_rows(
    r1_report: &DressRehearsalR1Report,
    starports: &HashSet<usize>,
) -> Vec<DressRehearsalR2SystemProductionRow> {
    let cell_inputs: HashMap<_, _> = r1_report
        .cell_inputs
        .iter()
        .map(|input| (input.cell_index, input))
        .collect();

    let mut systems: Vec<_> = r1_report
        .scenario
        .occupants
        .iter()
        .filter(|occupant| occupant.kind == DressRehearsalR1OccupantKind::System)
        .cloned()
        .collect();
    systems.sort_by_key(|occupant| system_index(&occupant.source_id));

    systems
        .into_iter()
        .map(|system| {
            let system_index = system_index(&system.source_id);
            let original_owner = owner_from_r1(system.owner);
            let disruption = r1_report.final_disruption[system.cell_index as usize];
            let blockader = if disruption >= BLOCKADE_THRESHOLD {
                cell_inputs
                    .get(&system.cell_index)
                    .and_then(|cell| blockader_from_cell_entries(cell.separated_entries.as_slice()))
            } else {
                None
            };
            let blockaded = blockader.is_some();
            let effective_outflow_owner = blockader.unwrap_or(original_owner);
            let (production_generated, labor_consumed, labor_remaining) =
                factory_recipe_production(POP_LABOR_PER_TICK);
            let has_starport = starports.contains(&system_index);
            DressRehearsalR2SystemProductionRow {
                system_id: system.source_id,
                system_index,
                x: system.x,
                y: system.y,
                cell_index: system.cell_index,
                structural_parent_before: "galactic-location-0",
                structural_parent_after: "galactic-location-0",
                original_owner,
                effective_outflow_owner,
                blockader,
                disruption,
                blockaded,
                has_starport,
                starport_need: if has_starport {
                    STARPORT_PRODUCTION_NEED
                } else {
                    0
                },
                labor_generated: POP_LABOR_PER_TICK,
                labor_consumed,
                labor_remaining,
                production_generated,
                outflow_to_original_owner: if blockaded { 0 } else { production_generated },
                outflow_to_effective_owner: production_generated,
                diverted_production: if blockaded { production_generated } else { 0 },
                owner_column_flipped: blockaded,
                disbursement_received: 0,
            }
        })
        .collect()
}

fn build_diverted_rows(
    production_rows: &[DressRehearsalR2SystemProductionRow],
) -> Vec<DressRehearsalR2DivertedProductionRow> {
    production_rows
        .iter()
        .filter(|row| row.blockaded)
        .map(|row| DressRehearsalR2DivertedProductionRow {
            system_id: row.system_id.clone(),
            system_index: row.system_index,
            cell_index: row.cell_index,
            original_owner: row.original_owner,
            blockader_owner: row
                .blockader
                .expect("blockaded rows must carry blockader attribution"),
            production: row.diverted_production,
            owner_column_before: row.original_owner,
            owner_column_after: row.effective_outflow_owner,
            structural_parent_before: row.structural_parent_before,
            structural_parent_after: row.structural_parent_after,
        })
        .collect()
}

fn reduce_up_owner_masked(
    production_rows: &[DressRehearsalR2SystemProductionRow],
) -> HashMap<DressRehearsalR2Owner, i64> {
    let mut reduced = HashMap::new();
    reduced.insert(DressRehearsalR2Owner::Terran, 0);
    reduced.insert(DressRehearsalR2Owner::Pirate, 0);
    for row in production_rows {
        *reduced.entry(row.effective_outflow_owner).or_insert(0) += row.outflow_to_effective_owner;
    }
    reduced
}

fn disburse_down(
    production_rows: &mut [DressRehearsalR2SystemProductionRow],
    seeds: &[DressRehearsalR2StockpileSeed],
    reduced_by_owner: &HashMap<DressRehearsalR2Owner, i64>,
) -> (
    Vec<DressRehearsalR2StockpileLedgerRow>,
    Vec<DressRehearsalR2DeficitDisbursementRow>,
) {
    let mut remaining_by_owner = HashMap::new();
    let mut ledger = Vec::new();
    for owner in [DressRehearsalR2Owner::Terran, DressRehearsalR2Owner::Pirate] {
        let before = seeds
            .iter()
            .find(|seed| seed.owner == owner)
            .map(|seed| seed.before_reduce_up)
            .unwrap_or(0);
        let reduced_in = *reduced_by_owner.get(&owner).unwrap_or(&0);
        let after_reduce_up = before + reduced_in;
        remaining_by_owner.insert(owner, after_reduce_up);
        ledger.push(DressRehearsalR2StockpileLedgerRow {
            owner,
            before_reduce_up: before,
            reduced_in,
            after_reduce_up,
            disbursed_down: 0,
            after_disburse_down: after_reduce_up,
        });
    }

    let mut disbursements = Vec::new();
    production_rows.sort_by_key(|row| row.system_index);
    for row in production_rows.iter_mut() {
        if row.starport_need <= 0 {
            continue;
        }
        let owner = row.original_owner;
        let requested = row.starport_need;
        let available = remaining_by_owner.entry(owner).or_insert(0);
        let disbursed = requested.min(*available).max(0);
        *available -= disbursed;
        row.disbursement_received = disbursed;
        disbursements.push(DressRehearsalR2DeficitDisbursementRow {
            owner,
            system_id: row.system_id.clone(),
            system_index: row.system_index,
            requested,
            disbursed,
            remaining_deficit: requested - disbursed,
        });
    }

    for row in &mut ledger {
        let after = *remaining_by_owner.get(&row.owner).unwrap_or(&0);
        row.disbursed_down = row.after_reduce_up - after;
        row.after_disburse_down = after;
    }
    (ledger, disbursements)
}

fn build_top_affected_systems(
    production_rows: &[DressRehearsalR2SystemProductionRow],
) -> Vec<DressRehearsalR2AffectedSystemRow> {
    let mut rows = production_rows.to_vec();
    rows.sort_by(|left, right| {
        right
            .diverted_production
            .cmp(&left.diverted_production)
            .then(right.disbursement_received.cmp(&left.disbursement_received))
            .then(right.disruption.total_cmp(&left.disruption))
            .then(left.system_index.cmp(&right.system_index))
    });
    rows.into_iter()
        .take(TOP_AFFECTED_COUNT)
        .enumerate()
        .map(|(rank, row)| DressRehearsalR2AffectedSystemRow {
            rank: rank + 1,
            system_id: row.system_id,
            system_index: row.system_index,
            original_owner: row.original_owner,
            effective_outflow_owner: row.effective_outflow_owner,
            disruption: row.disruption,
            diverted_production: row.diverted_production,
            disbursement_received: row.disbursement_received,
        })
        .collect()
}

fn build_summary(
    r1_checksum: u64,
    production_rows: &[DressRehearsalR2SystemProductionRow],
    ledger: &[DressRehearsalR2StockpileLedgerRow],
    disbursements: &[DressRehearsalR2DeficitDisbursementRow],
) -> DressRehearsalR2Summary {
    let total_production = production_rows
        .iter()
        .map(|row| row.production_generated)
        .sum();
    let total_diverted_production = production_rows
        .iter()
        .map(|row| row.diverted_production)
        .sum();
    let total_disbursed = disbursements.iter().map(|row| row.disbursed).sum();
    let stable_checksum = checksum_r2(r1_checksum, production_rows, ledger, disbursements);
    DressRehearsalR2Summary {
        system_count: production_rows.len(),
        diverted_system_count: production_rows.iter().filter(|row| row.blockaded).count(),
        total_production,
        total_diverted_production,
        total_disbursed,
        stable_checksum,
    }
}

fn render_artifact_markdown(
    production_rows: &[DressRehearsalR2SystemProductionRow],
    diverted_rows: &[DressRehearsalR2DivertedProductionRow],
    ledger: &[DressRehearsalR2StockpileLedgerRow],
    disbursements: &[DressRehearsalR2DeficitDisbursementRow],
    top: &[DressRehearsalR2AffectedSystemRow],
    summary: &DressRehearsalR2Summary,
    cpu_oracle_parity: bool,
    r1_checksum: u64,
) -> String {
    let mut out = String::new();
    out.push_str("## R2 Recursive Allocation Artifact\n\n");
    out.push_str("| key | value |\n|---|---:|\n");
    out.push_str(&format!("| r1_checksum | {:016x} |\n", r1_checksum));
    out.push_str(&format!("| system_count | {} |\n", summary.system_count));
    out.push_str(&format!(
        "| total_production | {} |\n",
        summary.total_production
    ));
    out.push_str(&format!(
        "| diverted_system_count | {} |\n",
        summary.diverted_system_count
    ));
    out.push_str(&format!(
        "| total_diverted_production | {} |\n",
        summary.total_diverted_production
    ));
    out.push_str(&format!(
        "| total_disbursed | {} |\n",
        summary.total_disbursed
    ));
    out.push_str(&format!(
        "| stable_checksum | {:016x} |\n",
        summary.stable_checksum
    ));
    out.push_str(&format!(
        "| cpu_oracle_parity | {} |\n\n",
        cpu_oracle_parity
    ));

    out.push_str("### System Production Rows\n\n");
    out.push_str("| system | owner | effective_owner | cell | disruption | blockaded | labor | consumed | production | diverted | disbursed |\n");
    out.push_str("|---|---|---|---:|---:|---|---:|---:|---:|---:|---:|\n");
    for row in production_rows {
        out.push_str(&format!(
            "| {} | {:?} | {:?} | {} | {:.3} | {} | {} | {} | {} | {} | {} |\n",
            row.system_id,
            row.original_owner,
            row.effective_outflow_owner,
            row.cell_index,
            row.disruption,
            row.blockaded,
            row.labor_generated,
            row.labor_consumed,
            row.production_generated,
            row.diverted_production,
            row.disbursement_received
        ));
    }

    out.push_str("\n### Diverted Production Rows\n\n");
    out.push_str("| system | original_owner | blockader | before_owner_col | after_owner_col | production | parent_before | parent_after |\n");
    out.push_str("|---|---|---|---|---|---:|---|---|\n");
    for row in diverted_rows {
        out.push_str(&format!(
            "| {} | {:?} | {:?} | {:?} | {:?} | {} | {} | {} |\n",
            row.system_id,
            row.original_owner,
            row.blockader_owner,
            row.owner_column_before,
            row.owner_column_after,
            row.production,
            row.structural_parent_before,
            row.structural_parent_after
        ));
    }

    out.push_str("\n### Stockpile Ledger\n\n");
    out.push_str("| owner | before | reduced_in | after_reduce_up | disbursed_down | after_disburse_down |\n");
    out.push_str("|---|---:|---:|---:|---:|---:|\n");
    for row in ledger {
        out.push_str(&format!(
            "| {:?} | {} | {} | {} | {} | {} |\n",
            row.owner,
            row.before_reduce_up,
            row.reduced_in,
            row.after_reduce_up,
            row.disbursed_down,
            row.after_disburse_down
        ));
    }

    out.push_str("\n### Deficit Disbursements\n\n");
    out.push_str("| owner | system | requested | disbursed | remaining_deficit |\n");
    out.push_str("|---|---|---:|---:|---:|\n");
    for row in disbursements {
        out.push_str(&format!(
            "| {:?} | {} | {} | {} | {} |\n",
            row.owner, row.system_id, row.requested, row.disbursed, row.remaining_deficit
        ));
    }

    out.push_str("\n### Top Affected Systems\n\n");
    out.push_str("| rank | system | original_owner | effective_owner | disruption | diverted | disbursed |\n");
    out.push_str("|---:|---|---|---|---:|---:|---:|\n");
    for row in top {
        out.push_str(&format!(
            "| {} | {} | {:?} | {:?} | {:.3} | {} | {} |\n",
            row.rank,
            row.system_id,
            row.original_owner,
            row.effective_outflow_owner,
            row.disruption,
            row.diverted_production,
            row.disbursement_received
        ));
    }
    out
}

fn factory_recipe() -> DressRehearsalR2FactoryRecipe {
    DressRehearsalR2FactoryRecipe {
        pop_labor_per_tick: POP_LABOR_PER_TICK,
        unit_cost_labor: FACTORY_UNIT_COST_LABOR,
        production_per_crossing: PRODUCTION_PER_RECIPE,
        input_consumption: "SubtractFromAllInputs",
        recipe_shape:
            "IntrinsicFlow(labor) -> ConjunctiveCrossing(labor) -> CrossingFormula{unit_cost:10} -> production",
        no_new_op: true,
    }
}

fn canonical_starport_system_indices() -> HashSet<usize> {
    atlas_store::canonical_materialization()
        .occupants
        .iter()
        .filter_map(|occupant| occupant.source_id.strip_prefix("starport-"))
        .filter_map(|suffix| suffix.parse::<usize>().ok())
        .collect()
}

fn blockader_from_cell_entries(
    entries: &[crate::dress_rehearsal_r1_disruption_heatmap::DressRehearsalR1CellInputEntry],
) -> Option<DressRehearsalR2Owner> {
    let pirate_disruption: f32 = entries
        .iter()
        .filter(|entry| entry.channel == DressRehearsalR1Channel::PirateDisruption)
        .map(|entry| entry.value)
        .sum();
    if pirate_disruption > 0.0 {
        Some(DressRehearsalR2Owner::Pirate)
    } else {
        None
    }
}

fn occupant_positions(r1_report: &DressRehearsalR1Report) -> Vec<DressRehearsalR2OccupantPosition> {
    let mut positions: Vec<_> = r1_report
        .scenario
        .occupants
        .iter()
        .map(|occupant| DressRehearsalR2OccupantPosition {
            source_id: occupant.source_id.clone(),
            kind: match occupant.kind {
                DressRehearsalR1OccupantKind::System => "system",
                DressRehearsalR1OccupantKind::PirateFleet => "pirate_fleet",
                DressRehearsalR1OccupantKind::PatrolFleet => "patrol_fleet",
            },
            owner: owner_from_r1(occupant.owner),
            x: occupant.x,
            y: occupant.y,
            cell_index: occupant.cell_index,
        })
        .collect();
    positions.sort_by(|left, right| left.source_id.cmp(&right.source_id));
    positions
}

fn owner_from_r1(owner: DressRehearsalR1Owner) -> DressRehearsalR2Owner {
    match owner {
        DressRehearsalR1Owner::Terran => DressRehearsalR2Owner::Terran,
        DressRehearsalR1Owner::Pirate => DressRehearsalR2Owner::Pirate,
    }
}

fn system_index(source_id: &str) -> usize {
    source_id
        .strip_prefix("system-")
        .and_then(|suffix| suffix.parse::<usize>().ok())
        .expect("R1 system source ids must be system-{index}")
}

fn empty_oracle() -> DressRehearsalR2Oracle {
    DressRehearsalR2Oracle {
        production_rows: Vec::new(),
        diverted_rows: Vec::new(),
        stockpile_ledger: Vec::new(),
        deficit_disbursements: Vec::new(),
        top_affected_systems: Vec::new(),
        summary: DressRehearsalR2Summary {
            system_count: 0,
            diverted_system_count: 0,
            total_production: 0,
            total_diverted_production: 0,
            total_disbursed: 0,
            stable_checksum: 0,
        },
    }
}

struct Execution {
    production_rows: Vec<DressRehearsalR2SystemProductionRow>,
    diverted_rows: Vec<DressRehearsalR2DivertedProductionRow>,
    stockpile_ledger: Vec<DressRehearsalR2StockpileLedgerRow>,
    deficit_disbursements: Vec<DressRehearsalR2DeficitDisbursementRow>,
    top_affected_systems: Vec<DressRehearsalR2AffectedSystemRow>,
    summary: DressRehearsalR2Summary,
}

fn checksum_r2(
    r1_checksum: u64,
    production_rows: &[DressRehearsalR2SystemProductionRow],
    ledger: &[DressRehearsalR2StockpileLedgerRow],
    disbursements: &[DressRehearsalR2DeficitDisbursementRow],
) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    hash = fnv(hash, r1_checksum);
    for row in production_rows {
        hash = fnv(hash, row.system_index as u64);
        hash = fnv(hash, row.cell_index as u64);
        hash = fnv(hash, row.original_owner.stable_code());
        hash = fnv(hash, row.effective_outflow_owner.stable_code());
        hash = fnv(hash, row.disruption.to_bits() as u64);
        hash = fnv(hash, row.blockaded as u64);
        hash = fnv(hash, row.production_generated as u64);
        hash = fnv(hash, row.diverted_production as u64);
        hash = fnv(hash, row.disbursement_received as u64);
    }
    for row in ledger {
        hash = fnv(hash, row.owner.stable_code());
        hash = fnv(hash, row.before_reduce_up as u64);
        hash = fnv(hash, row.reduced_in as u64);
        hash = fnv(hash, row.after_reduce_up as u64);
        hash = fnv(hash, row.disbursed_down as u64);
        hash = fnv(hash, row.after_disburse_down as u64);
    }
    for row in disbursements {
        hash = fnv(hash, row.owner.stable_code());
        hash = fnv(hash, row.system_index as u64);
        hash = fnv(hash, row.requested as u64);
        hash = fnv(hash, row.disbursed as u64);
        hash = fnv(hash, row.remaining_deficit as u64);
    }
    hash
}

fn fnv(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
