//! SCENARIO-0080-2-R1: disruption heatmap / EC1.
//!
//! Fixture-only, opt-in/default-off vertical proof over the accepted ATLAS-BATCH-0
//! galactic Location grid. Sources are produced by real occupants, then advanced
//! through the pinned BoundedFeedback recurrence and diffused into a strict sink
//! column. CPU oracle is the authority for this rung; no GPU, shader, movement,
//! or production session wiring is introduced here.

#[allow(dead_code)]
#[path = "dress_rehearsal_atlas_batch_0_loc.rs"]
mod atlas_loc;

use atlas_loc::{
    DressRehearsalMap, GridCell as AtlasGridCell, LocationId, LocationMaterialization,
    LocationRole, OccupantKind, Owner as AtlasOwner,
};
use simthing_spec::EmlGadgetInstanceSpec;

pub const DRESS_REHEARSAL_R1_DISRUPTION_HEATMAP_ID: &str = "SCENARIO-0080-2-R1-DISRUPTION-HEATMAP";
pub const DRESS_REHEARSAL_R1_DISRUPTION_HEATMAP_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - R1 Disruption Heatmap / EC1; CPU oracle parity; opt-in/default-off";
pub const DRESS_REHEARSAL_R1_SCENARIO: &str = "SCENARIO-0080-2";

pub const GALAXY_SIDE: u32 = 20;
pub const GALAXY_CELL_COUNT: usize = (GALAXY_SIDE as usize) * (GALAXY_SIDE as usize);
pub const SYSTEM_COUNT: usize = 13;
pub const HOTSPOT_COUNT: usize = 8;

pub const DECAY: f32 = 0.80;
pub const GAIN: f32 = 1.00;
pub const FLOOR: f32 = 0.0;
pub const CEILING: f32 = 100.0;
pub const PIRATE_EMIT: f32 = 20.0;
pub const PATROL_SUPPRESS: f32 = 15.0;
pub const H_WEIGHT: f32 = 0.25;

pub const DISRUPTION_COL: u32 = 0;
pub const LOCATION_STATUS_COL: u32 = 1;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DressRehearsalR1Gate {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl DressRehearsalR1Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DressRehearsalR1Surface {
    pub gate: DressRehearsalR1Gate,
    pub galactic_gridcell_field_registered: bool,
    pub default_simsession_pass_graph_wiring: bool,
    pub global_default_schedule: bool,
    pub field_policy_movement: bool,
    pub gradientxy_consumption: bool,
    pub recursive_r2_reduce_up: bool,
    pub r3_mask_down: bool,
    pub reenroll: bool,
    pub new_shader_or_wgsl: bool,
    pub f32_bit_exact_claim: bool,
    pub ui_or_realtime: bool,
    pub cli_binary: bool,
    pub blockade_or_divert_gate: bool,
    pub owner_runtime_beyond_r1_cell_routing: bool,
}

impl DressRehearsalR1Surface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: DressRehearsalR1Gate::explicit_opt_in(),
            galactic_gridcell_field_registered: true,
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DressRehearsalR1ForbiddenRequests {
    pub field_policy_movement: bool,
    pub gradientxy_consumption: bool,
    pub recursive_r2_reduce_up: bool,
    pub r3_mask_down: bool,
    pub reenroll: bool,
    pub r4_field_consumption: bool,
    pub r5_movement_or_ship_fission: bool,
    pub r6_combat: bool,
    pub disruption_blockade_or_divert: bool,
    pub owner_runtime_beyond_r1_cell_routing: bool,
    pub default_simsession_pass_graph_wiring: bool,
    pub global_default_schedule: bool,
    pub ui_or_realtime: bool,
    pub cli_binary: bool,
    pub semantic_or_raw_wgsl: bool,
    pub new_shader_or_gpu_kernel: bool,
    pub f32_bit_exact_claim: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub clausething_dependency: bool,
    pub invariant_edit: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DressRehearsalR1Owner {
    Terran,
    Pirate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DressRehearsalR1OccupantKind {
    System,
    PirateFleet,
    PatrolFleet,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DressRehearsalR1Channel {
    InertSystem,
    PirateDisruption,
    PatrolSuppression,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DressRehearsalR1GridCell {
    pub x: u32,
    pub y: u32,
    pub cell_index: u32,
    pub disruption_col: u32,
    pub location_status_col: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR1OccupantContribution {
    pub source_id: String,
    pub kind: DressRehearsalR1OccupantKind,
    pub owner: DressRehearsalR1Owner,
    pub x: u32,
    pub y: u32,
    pub cell_index: u32,
    pub channel: DressRehearsalR1Channel,
    pub value: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR1Scenario {
    pub galaxy_side: u32,
    pub grid_cells: Vec<DressRehearsalR1GridCell>,
    pub system_cells: Vec<DressRehearsalR1GridCell>,
    pub occupants: Vec<DressRehearsalR1OccupantContribution>,
}

impl DressRehearsalR1Scenario {
    pub fn canonical() -> Self {
        let map = DressRehearsalMap::canonical();
        let materialization = LocationMaterialization::from_map(&map);
        let galactic = materialization
            .locations
            .iter()
            .find(|location| location.role == LocationRole::Galactic)
            .expect("ATLAS-BATCH-0 must include the galactic location");
        assert_eq!(galactic.id, LocationId(0));
        assert_eq!(galactic.width, GALAXY_SIDE);
        assert_eq!(galactic.height, GALAXY_SIDE);
        assert_eq!(galactic.map_base, 0);

        let grid_cells = build_grid_cells();

        let mut system_cells = Vec::with_capacity(SYSTEM_COUNT);
        let mut occupants = Vec::new();
        for system in &map.systems {
            let cell = grid_cell(system.galactic_cell.x, system.galactic_cell.y);
            system_cells.push(cell);
            occupants.push(DressRehearsalR1OccupantContribution {
                source_id: format!("system-{}", system.index),
                kind: DressRehearsalR1OccupantKind::System,
                owner: owner_from_atlas(system.owner),
                x: cell.x,
                y: cell.y,
                cell_index: cell.cell_index,
                channel: DressRehearsalR1Channel::InertSystem,
                value: 0.0,
            });
        }

        for occupant in &materialization.occupants {
            if occupant.location_id != LocationId(0) {
                continue;
            }
            match occupant.kind {
                OccupantKind::PirateFleet => {
                    occupants.push(fleet_contribution(
                        occupant.source_id.clone(),
                        DressRehearsalR1OccupantKind::PirateFleet,
                        owner_from_atlas(occupant.owner),
                        occupant.cell,
                        DressRehearsalR1Channel::PirateDisruption,
                        PIRATE_EMIT,
                    ));
                }
                OccupantKind::PatrolFleet => {
                    occupants.push(fleet_contribution(
                        occupant.source_id.clone(),
                        DressRehearsalR1OccupantKind::PatrolFleet,
                        owner_from_atlas(occupant.owner),
                        occupant.cell,
                        DressRehearsalR1Channel::PatrolSuppression,
                        -PATROL_SUPPRESS,
                    ));
                }
                _ => {}
            }
        }

        occupants.sort_by(|left, right| {
            left.cell_index
                .cmp(&right.cell_index)
                .then(channel_rank(left.channel).cmp(&channel_rank(right.channel)))
                .then(owner_rank(left.owner).cmp(&owner_rank(right.owner)))
                .then(left.source_id.cmp(&right.source_id))
        });

        Self {
            galaxy_side: GALAXY_SIDE,
            grid_cells,
            system_cells,
            occupants,
        }
    }

    pub fn empty() -> Self {
        Self {
            galaxy_side: GALAXY_SIDE,
            grid_cells: build_grid_cells(),
            system_cells: Vec::new(),
            occupants: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR1Input {
    pub surface: DressRehearsalR1Surface,
    pub scenario: DressRehearsalR1Scenario,
    pub tick_count: u32,
    pub initial_disruption: Vec<f32>,
    pub forbidden: DressRehearsalR1ForbiddenRequests,
}

impl DressRehearsalR1Input {
    pub fn default_simsession() -> Self {
        Self {
            surface: DressRehearsalR1Surface::default_simsession(),
            scenario: DressRehearsalR1Scenario::empty(),
            tick_count: 0,
            initial_disruption: Vec::new(),
            forbidden: DressRehearsalR1ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            surface: DressRehearsalR1Surface::with_explicit_opt_in(),
            scenario: DressRehearsalR1Scenario::canonical(),
            tick_count: 8,
            initial_disruption: vec![0.0; GALAXY_CELL_COUNT],
            forbidden: DressRehearsalR1ForbiddenRequests::default(),
        }
    }

    pub fn with_scenario(scenario: DressRehearsalR1Scenario, tick_count: u32) -> Self {
        Self {
            surface: DressRehearsalR1Surface::with_explicit_opt_in(),
            scenario,
            tick_count,
            initial_disruption: vec![0.0; GALAXY_CELL_COUNT],
            forbidden: DressRehearsalR1ForbiddenRequests::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR1CellInputEntry {
    pub source_id: String,
    pub owner: DressRehearsalR1Owner,
    pub channel: DressRehearsalR1Channel,
    pub value: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR1CellInput {
    pub x: u32,
    pub y: u32,
    pub cell_index: u32,
    pub pirate_count: u32,
    pub patrol_count: u32,
    pub inert_count: u32,
    pub pirate_contribution: f32,
    pub patrol_suppression: f32,
    pub input_cell: f32,
    pub separated_entries: Vec<DressRehearsalR1CellInputEntry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR1RecurrenceRow {
    pub tick: u32,
    pub x: u32,
    pub y: u32,
    pub cell_index: u32,
    pub disruption_before: f32,
    pub input_cell: f32,
    pub disruption_after: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR1DiffusionRow {
    pub x: u32,
    pub y: u32,
    pub cell_index: u32,
    pub source_col: u32,
    pub target_col: u32,
    pub disruption_source: f32,
    pub neighbor_sum: f32,
    pub neighbor_count: u32,
    pub location_status: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR1Hotspot {
    pub rank: usize,
    pub x: u32,
    pub y: u32,
    pub cell_index: u32,
    pub disruption: f32,
    pub location_status: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR1ArtifactRow {
    pub x: u32,
    pub y: u32,
    pub cell_index: u32,
    pub disruption: f32,
    pub location_status: f32,
    pub pirate_count: u32,
    pub patrol_count: u32,
    pub inert_count: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR1Summary {
    pub total_disruption: f32,
    pub max_cell_index: u32,
    pub max_x: u32,
    pub max_y: u32,
    pub max_disruption: f32,
    pub occupied_cell_count: usize,
    pub stable_checksum: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR1Artifact {
    pub rows: Vec<DressRehearsalR1ArtifactRow>,
    pub hotspots: Vec<DressRehearsalR1Hotspot>,
    pub summary: DressRehearsalR1Summary,
    pub cpu_oracle_parity: bool,
    pub gpu_cross_check: &'static str,
    pub markdown: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR1Oracle {
    pub cell_inputs: Vec<DressRehearsalR1CellInput>,
    pub recurrence_rows: Vec<DressRehearsalR1RecurrenceRow>,
    pub diffusion_rows: Vec<DressRehearsalR1DiffusionRow>,
    pub final_disruption: Vec<f32>,
    pub location_status: Vec<f32>,
    pub summary: DressRehearsalR1Summary,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR1Report {
    pub id: &'static str,
    pub status: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,

    pub galaxy_side: u32,
    pub system_count: usize,
    pub grid_cell_count: usize,
    pub disruption_col: u32,
    pub location_status_col: u32,
    pub source_target_columns_distinct: bool,
    pub bounded_feedback_gadget: EmlGadgetInstanceSpec,

    pub scenario: DressRehearsalR1Scenario,
    pub cell_inputs: Vec<DressRehearsalR1CellInput>,
    pub recurrence_rows: Vec<DressRehearsalR1RecurrenceRow>,
    pub diffusion_rows: Vec<DressRehearsalR1DiffusionRow>,
    pub final_disruption: Vec<f32>,
    pub location_status: Vec<f32>,
    pub starmap_summary: DressRehearsalR1Summary,
    pub hotspots: Vec<DressRehearsalR1Hotspot>,
    pub artifact: DressRehearsalR1Artifact,

    pub cpu_oracle_parity: bool,
    pub deterministic_replay_checksum: u64,

    pub no_field_policy_movement: bool,
    pub no_gradientxy_consumption: bool,
    pub no_recursive_r2_reduce_up: bool,
    pub no_r3_mask_down: bool,
    pub no_reenroll: bool,
    pub no_default_simsession_pass_graph_change: bool,
    pub no_global_default_schedule: bool,
    pub no_new_shader_or_wgsl: bool,
    pub no_f32_bit_exact_claim: bool,
    pub no_ui_realtime_or_cli: bool,
    pub no_blockade_or_divert_gate: bool,
}

pub fn run_dress_rehearsal_r1_disruption_heatmap(
    input: &DressRehearsalR1Input,
) -> DressRehearsalR1Report {
    let mut diagnostics = Vec::new();
    validate_surface(&input.surface, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);

    if !input.surface.gate.explicit_opt_in {
        return base_report(input, true, diagnostics, None, false);
    }

    validate_shape(input, &mut diagnostics);
    if !diagnostics.is_empty() {
        return base_report(input, false, diagnostics, None, false);
    }

    let execution = execute_model(input);
    let oracle = cpu_oracle_dress_rehearsal_r1_disruption_heatmap(input);
    let parity = execution.final_disruption == oracle.final_disruption
        && execution.location_status == oracle.location_status
        && execution.summary.stable_checksum == oracle.summary.stable_checksum;
    base_report(input, false, Vec::new(), Some(execution), parity)
}

pub fn replay_dress_rehearsal_r1_disruption_heatmap(
) -> (DressRehearsalR1Report, DressRehearsalR1Report) {
    let input = DressRehearsalR1Input::explicit_opt_in();
    (
        run_dress_rehearsal_r1_disruption_heatmap(&input),
        run_dress_rehearsal_r1_disruption_heatmap(&input),
    )
}

pub fn cpu_oracle_dress_rehearsal_r1_disruption_heatmap(
    input: &DressRehearsalR1Input,
) -> DressRehearsalR1Oracle {
    let execution = execute_model(input);
    DressRehearsalR1Oracle {
        cell_inputs: execution.cell_inputs,
        recurrence_rows: execution.recurrence_rows,
        diffusion_rows: execution.diffusion_rows,
        final_disruption: execution.final_disruption,
        location_status: execution.location_status,
        summary: execution.summary,
    }
}

pub fn bounded_feedback_next(previous: f32, input_cell: f32) -> f32 {
    (previous * DECAY + input_cell * GAIN).clamp(FLOOR, CEILING)
}

pub fn cell_index(x: u32, y: u32) -> u32 {
    y * GALAXY_SIDE + x
}

pub fn render_dress_rehearsal_r1_artifact(report: &DressRehearsalR1Report) -> String {
    report.artifact.markdown.clone()
}

fn build_grid_cells() -> Vec<DressRehearsalR1GridCell> {
    let mut cells = Vec::with_capacity(GALAXY_CELL_COUNT);
    for y in 0..GALAXY_SIDE {
        for x in 0..GALAXY_SIDE {
            cells.push(grid_cell(x, y));
        }
    }
    cells
}

fn grid_cell(x: u32, y: u32) -> DressRehearsalR1GridCell {
    DressRehearsalR1GridCell {
        x,
        y,
        cell_index: cell_index(x, y),
        disruption_col: DISRUPTION_COL,
        location_status_col: LOCATION_STATUS_COL,
    }
}

fn fleet_contribution(
    source_id: String,
    kind: DressRehearsalR1OccupantKind,
    owner: DressRehearsalR1Owner,
    cell: AtlasGridCell,
    channel: DressRehearsalR1Channel,
    value: f32,
) -> DressRehearsalR1OccupantContribution {
    DressRehearsalR1OccupantContribution {
        source_id,
        kind,
        owner,
        x: cell.x,
        y: cell.y,
        cell_index: cell_index(cell.x, cell.y),
        channel,
        value,
    }
}

fn owner_from_atlas(owner: AtlasOwner) -> DressRehearsalR1Owner {
    match owner {
        AtlasOwner::Terran => DressRehearsalR1Owner::Terran,
        AtlasOwner::Pirate => DressRehearsalR1Owner::Pirate,
    }
}

fn execute_model(input: &DressRehearsalR1Input) -> Execution {
    let cell_inputs = build_cell_inputs(&input.scenario);
    let (recurrence_rows, final_disruption) =
        run_recurrence(&cell_inputs, &input.initial_disruption, input.tick_count);
    let diffusion_rows = run_diffusion(&final_disruption);
    let location_status: Vec<f32> = diffusion_rows
        .iter()
        .map(|row| row.location_status)
        .collect();
    let summary = build_summary(&cell_inputs, &final_disruption, &location_status);
    let hotspots = build_hotspots(&final_disruption, &location_status);
    let artifact_rows = build_artifact_rows(&cell_inputs, &final_disruption, &location_status);
    let markdown = render_artifact_markdown(&artifact_rows, &hotspots, &summary, true);
    let artifact = DressRehearsalR1Artifact {
        rows: artifact_rows,
        hotspots: hotspots.clone(),
        summary: summary.clone(),
        cpu_oracle_parity: true,
        gpu_cross_check: "NotRunCpuOraclePrimary",
        markdown,
    };

    Execution {
        cell_inputs,
        recurrence_rows,
        diffusion_rows,
        final_disruption,
        location_status,
        summary,
        hotspots,
        artifact,
    }
}

fn build_cell_inputs(scenario: &DressRehearsalR1Scenario) -> Vec<DressRehearsalR1CellInput> {
    let mut inputs: Vec<_> = scenario
        .grid_cells
        .iter()
        .map(|cell| DressRehearsalR1CellInput {
            x: cell.x,
            y: cell.y,
            cell_index: cell.cell_index,
            pirate_count: 0,
            patrol_count: 0,
            inert_count: 0,
            pirate_contribution: 0.0,
            patrol_suppression: 0.0,
            input_cell: 0.0,
            separated_entries: Vec::new(),
        })
        .collect();

    for occupant in &scenario.occupants {
        let entry = &mut inputs[occupant.cell_index as usize];
        entry
            .separated_entries
            .push(DressRehearsalR1CellInputEntry {
                source_id: occupant.source_id.clone(),
                owner: occupant.owner,
                channel: occupant.channel,
                value: occupant.value,
            });
        match occupant.channel {
            DressRehearsalR1Channel::PirateDisruption => {
                entry.pirate_count += 1;
                entry.pirate_contribution += occupant.value;
            }
            DressRehearsalR1Channel::PatrolSuppression => {
                entry.patrol_count += 1;
                entry.patrol_suppression += -occupant.value;
            }
            DressRehearsalR1Channel::InertSystem => {
                entry.inert_count += 1;
            }
        }
    }

    for input in &mut inputs {
        input.separated_entries.sort_by(|left, right| {
            channel_rank(left.channel)
                .cmp(&channel_rank(right.channel))
                .then(owner_rank(left.owner).cmp(&owner_rank(right.owner)))
                .then(left.source_id.cmp(&right.source_id))
        });
        input.input_cell = input.pirate_contribution - input.patrol_suppression;
    }

    inputs
}

fn run_recurrence(
    cell_inputs: &[DressRehearsalR1CellInput],
    initial_disruption: &[f32],
    tick_count: u32,
) -> (Vec<DressRehearsalR1RecurrenceRow>, Vec<f32>) {
    let mut disruption = initial_disruption.to_vec();
    let mut rows = Vec::with_capacity(tick_count as usize * cell_inputs.len());
    for tick in 0..tick_count {
        for input in cell_inputs {
            let idx = input.cell_index as usize;
            let before = disruption[idx];
            let after = bounded_feedback_next(before, input.input_cell);
            disruption[idx] = after;
            rows.push(DressRehearsalR1RecurrenceRow {
                tick,
                x: input.x,
                y: input.y,
                cell_index: input.cell_index,
                disruption_before: before,
                input_cell: input.input_cell,
                disruption_after: after,
            });
        }
    }
    (rows, disruption)
}

fn run_diffusion(disruption: &[f32]) -> Vec<DressRehearsalR1DiffusionRow> {
    let mut rows = Vec::with_capacity(GALAXY_CELL_COUNT);
    for y in 0..GALAXY_SIDE {
        for x in 0..GALAXY_SIDE {
            let idx = cell_index(x, y) as usize;
            let mut neighbor_sum = 0.0;
            let mut neighbor_count = 0u32;
            for (nx, ny) in von_neumann_neighbors(x, y) {
                neighbor_sum += disruption[cell_index(nx, ny) as usize];
                neighbor_count += 1;
            }
            let denom = 1.0 + H_WEIGHT * neighbor_count as f32;
            let location_status =
                ((disruption[idx] + H_WEIGHT * neighbor_sum) / denom).clamp(FLOOR, CEILING);
            rows.push(DressRehearsalR1DiffusionRow {
                x,
                y,
                cell_index: cell_index(x, y),
                source_col: DISRUPTION_COL,
                target_col: LOCATION_STATUS_COL,
                disruption_source: disruption[idx],
                neighbor_sum,
                neighbor_count,
                location_status,
            });
        }
    }
    rows
}

fn von_neumann_neighbors(x: u32, y: u32) -> Vec<(u32, u32)> {
    let mut neighbors = Vec::with_capacity(4);
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x + 1 < GALAXY_SIDE {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y + 1 < GALAXY_SIDE {
        neighbors.push((x, y + 1));
    }
    neighbors
}

fn build_hotspots(disruption: &[f32], location_status: &[f32]) -> Vec<DressRehearsalR1Hotspot> {
    let mut cells: Vec<_> = (0..GALAXY_CELL_COUNT)
        .map(|idx| {
            let x = idx as u32 % GALAXY_SIDE;
            let y = idx as u32 / GALAXY_SIDE;
            (idx, x, y, disruption[idx], location_status[idx])
        })
        .collect();
    cells.sort_by(|left, right| right.3.total_cmp(&left.3).then(left.0.cmp(&right.0)));
    cells
        .into_iter()
        .take(HOTSPOT_COUNT)
        .enumerate()
        .map(
            |(rank, (idx, x, y, disruption, location_status))| DressRehearsalR1Hotspot {
                rank: rank + 1,
                x,
                y,
                cell_index: idx as u32,
                disruption,
                location_status,
            },
        )
        .collect()
}

fn build_artifact_rows(
    cell_inputs: &[DressRehearsalR1CellInput],
    disruption: &[f32],
    location_status: &[f32],
) -> Vec<DressRehearsalR1ArtifactRow> {
    cell_inputs
        .iter()
        .map(|input| {
            let idx = input.cell_index as usize;
            DressRehearsalR1ArtifactRow {
                x: input.x,
                y: input.y,
                cell_index: input.cell_index,
                disruption: disruption[idx],
                location_status: location_status[idx],
                pirate_count: input.pirate_count,
                patrol_count: input.patrol_count,
                inert_count: input.inert_count,
            }
        })
        .collect()
}

fn build_summary(
    cell_inputs: &[DressRehearsalR1CellInput],
    disruption: &[f32],
    location_status: &[f32],
) -> DressRehearsalR1Summary {
    let total_disruption: f32 = disruption.iter().copied().sum();
    let (max_idx, max_disruption) = disruption
        .iter()
        .copied()
        .enumerate()
        .max_by(|left, right| {
            left.1
                .total_cmp(&right.1)
                .then_with(|| right.0.cmp(&left.0))
        })
        .unwrap_or((0, 0.0));
    let occupied_cell_count = cell_inputs
        .iter()
        .filter(|input| !input.separated_entries.is_empty())
        .count();
    let stable_checksum = checksum_field(cell_inputs, disruption, location_status);
    DressRehearsalR1Summary {
        total_disruption,
        max_cell_index: max_idx as u32,
        max_x: max_idx as u32 % GALAXY_SIDE,
        max_y: max_idx as u32 / GALAXY_SIDE,
        max_disruption,
        occupied_cell_count,
        stable_checksum,
    }
}

fn render_artifact_markdown(
    rows: &[DressRehearsalR1ArtifactRow],
    hotspots: &[DressRehearsalR1Hotspot],
    summary: &DressRehearsalR1Summary,
    cpu_oracle_parity: bool,
) -> String {
    let mut out = String::new();
    out.push_str("## R1 Disruption Heatmap Artifact\n\n");
    out.push_str("| key | value |\n|---|---:|\n");
    out.push_str(&format!("| galaxy_side | {} |\n", GALAXY_SIDE));
    out.push_str(&format!("| cell_count | {} |\n", rows.len()));
    out.push_str(&format!(
        "| total_disruption | {:.3} |\n",
        summary.total_disruption
    ));
    out.push_str(&format!(
        "| max_cell | ({},{}) index {} |\n",
        summary.max_x, summary.max_y, summary.max_cell_index
    ));
    out.push_str(&format!(
        "| max_disruption | {:.3} |\n",
        summary.max_disruption
    ));
    out.push_str(&format!(
        "| occupied_cell_count | {} |\n",
        summary.occupied_cell_count
    ));
    out.push_str(&format!(
        "| stable_checksum | {:016x} |\n",
        summary.stable_checksum
    ));
    out.push_str(&format!("| cpu_oracle_parity | {} |\n", cpu_oracle_parity));
    out.push_str("| gpu_cross_check | NotRunCpuOraclePrimary |\n\n");

    out.push_str("### Top 8 Hotspots\n\n");
    out.push_str("| rank | x | y | cell_index | disruption | location_status |\n|---:|---:|---:|---:|---:|---:|\n");
    for hotspot in hotspots {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {:.3} | {:.3} |\n",
            hotspot.rank,
            hotspot.x,
            hotspot.y,
            hotspot.cell_index,
            hotspot.disruption,
            hotspot.location_status
        ));
    }

    out.push_str("\n### 20x20 Cell Table\n\n");
    out.push_str("| x | y | cell_index | disruption | location_status | pirate_count | patrol_count | inert_count |\n");
    out.push_str("|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in rows {
        out.push_str(&format!(
            "| {} | {} | {} | {:.3} | {:.3} | {} | {} | {} |\n",
            row.x,
            row.y,
            row.cell_index,
            row.disruption,
            row.location_status,
            row.pirate_count,
            row.patrol_count,
            row.inert_count
        ));
    }
    out
}

fn validate_surface(surface: &DressRehearsalR1Surface, diagnostics: &mut Vec<&'static str>) {
    if surface.gate.enabled_by_default {
        diagnostics.push("r1_default_on_rejected");
    }
    if surface.default_simsession_pass_graph_wiring {
        diagnostics.push("default_simsession_pass_graph_wiring");
    }
    if surface.global_default_schedule {
        diagnostics.push("global_default_schedule");
    }
    if surface.field_policy_movement {
        diagnostics.push("field_policy_movement");
    }
    if surface.gradientxy_consumption {
        diagnostics.push("gradientxy_consumption");
    }
    if surface.recursive_r2_reduce_up {
        diagnostics.push("recursive_r2_reduce_up");
    }
    if surface.r3_mask_down {
        diagnostics.push("r3_mask_down");
    }
    if surface.reenroll {
        diagnostics.push("reenroll");
    }
    if surface.new_shader_or_wgsl {
        diagnostics.push("new_shader_or_wgsl");
    }
    if surface.f32_bit_exact_claim {
        diagnostics.push("f32_bit_exact_claim");
    }
    if surface.ui_or_realtime {
        diagnostics.push("ui_or_realtime");
    }
    if surface.cli_binary {
        diagnostics.push("cli_binary");
    }
    if surface.blockade_or_divert_gate {
        diagnostics.push("disruption_blockade_or_divert");
    }
    if surface.owner_runtime_beyond_r1_cell_routing {
        diagnostics.push("owner_runtime_beyond_r1_cell_routing");
    }
}

fn validate_forbidden(
    forbidden: &DressRehearsalR1ForbiddenRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.field_policy_movement {
        diagnostics.push("field_policy_movement");
    }
    if forbidden.gradientxy_consumption {
        diagnostics.push("gradientxy_consumption");
    }
    if forbidden.recursive_r2_reduce_up {
        diagnostics.push("recursive_r2_reduce_up");
    }
    if forbidden.r3_mask_down {
        diagnostics.push("r3_mask_down");
    }
    if forbidden.reenroll {
        diagnostics.push("reenroll");
    }
    if forbidden.r4_field_consumption {
        diagnostics.push("r4_field_consumption");
    }
    if forbidden.r5_movement_or_ship_fission {
        diagnostics.push("r5_movement_or_ship_fission");
    }
    if forbidden.r6_combat {
        diagnostics.push("r6_combat");
    }
    if forbidden.disruption_blockade_or_divert {
        diagnostics.push("disruption_blockade_or_divert");
    }
    if forbidden.owner_runtime_beyond_r1_cell_routing {
        diagnostics.push("owner_runtime_beyond_r1_cell_routing");
    }
    if forbidden.default_simsession_pass_graph_wiring {
        diagnostics.push("default_simsession_pass_graph_wiring");
    }
    if forbidden.global_default_schedule {
        diagnostics.push("global_default_schedule");
    }
    if forbidden.ui_or_realtime {
        diagnostics.push("ui_or_realtime");
    }
    if forbidden.cli_binary {
        diagnostics.push("cli_binary");
    }
    if forbidden.semantic_or_raw_wgsl {
        diagnostics.push("semantic_or_raw_wgsl");
    }
    if forbidden.new_shader_or_gpu_kernel {
        diagnostics.push("new_shader_or_gpu_kernel");
    }
    if forbidden.f32_bit_exact_claim {
        diagnostics.push("f32_bit_exact_claim");
    }
    if forbidden.hard_currency_markets_trade_aibudget {
        diagnostics.push("hard_currency_markets_trade_aibudget");
    }
    if forbidden.nested_resource_flow {
        diagnostics.push("nested_resource_flow");
    }
    if forbidden.clausething_dependency {
        diagnostics.push("clausething_dependency");
    }
    if forbidden.invariant_edit {
        diagnostics.push("invariant_edit");
    }
}

fn validate_shape(input: &DressRehearsalR1Input, diagnostics: &mut Vec<&'static str>) {
    if input.scenario.galaxy_side != GALAXY_SIDE {
        diagnostics.push("galaxy_side_must_remain_20");
    }
    if input.scenario.grid_cells.len() != GALAXY_CELL_COUNT {
        diagnostics.push("galactic_cell_count_must_remain_400");
    }
    if input.scenario.system_cells.len() != SYSTEM_COUNT && !input.scenario.system_cells.is_empty()
    {
        diagnostics.push("system_count_must_remain_13");
    }
    if input.initial_disruption.len() != GALAXY_CELL_COUNT {
        diagnostics.push("initial_disruption_shape_mismatch");
    }
    if input.tick_count == 0 {
        diagnostics.push("tick_count_must_be_positive");
    }
    if input
        .scenario
        .grid_cells
        .iter()
        .any(|cell| cell.cell_index != cell_index(cell.x, cell.y))
    {
        diagnostics.push("cell_index_must_be_row_major");
    }
    if DISRUPTION_COL == LOCATION_STATUS_COL {
        diagnostics.push("disruption_location_status_columns_must_differ");
    }
}

fn base_report(
    input: &DressRehearsalR1Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    execution: Option<Execution>,
    cpu_oracle_parity: bool,
) -> DressRehearsalR1Report {
    let opt_in = input.surface.gate.explicit_opt_in;
    let admitted = diagnostics.is_empty();
    let empty_summary = DressRehearsalR1Summary {
        total_disruption: 0.0,
        max_cell_index: 0,
        max_x: 0,
        max_y: 0,
        max_disruption: 0.0,
        occupied_cell_count: 0,
        stable_checksum: 0,
    };
    let (
        cell_inputs,
        recurrence_rows,
        diffusion_rows,
        final_disruption,
        location_status,
        summary,
        hotspots,
        artifact,
    ) = match execution {
        Some(execution) => (
            execution.cell_inputs,
            execution.recurrence_rows,
            execution.diffusion_rows,
            execution.final_disruption,
            execution.location_status,
            execution.summary,
            execution.hotspots,
            execution.artifact,
        ),
        None => (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            empty_summary.clone(),
            Vec::new(),
            DressRehearsalR1Artifact {
                rows: Vec::new(),
                hotspots: Vec::new(),
                summary: empty_summary.clone(),
                cpu_oracle_parity: false,
                gpu_cross_check: "NotRunCpuOraclePrimary",
                markdown: String::new(),
            },
        ),
    };

    let checksum = if admitted && !disabled_no_op {
        summary.stable_checksum
    } else {
        0
    };

    DressRehearsalR1Report {
        id: DRESS_REHEARSAL_R1_DISRUPTION_HEATMAP_ID,
        status: DRESS_REHEARSAL_R1_DISRUPTION_HEATMAP_STATUS_PASS,
        scenario_name: DRESS_REHEARSAL_R1_SCENARIO,
        admitted,
        diagnostics,
        explicit_opt_in: opt_in,
        default_off: !input.surface.gate.enabled_by_default,
        disabled_no_op,
        galaxy_side: if disabled_no_op {
            0
        } else {
            input.scenario.galaxy_side
        },
        system_count: if disabled_no_op {
            0
        } else {
            input.scenario.system_cells.len()
        },
        grid_cell_count: if disabled_no_op {
            0
        } else {
            input.scenario.grid_cells.len()
        },
        disruption_col: DISRUPTION_COL,
        location_status_col: LOCATION_STATUS_COL,
        source_target_columns_distinct: DISRUPTION_COL != LOCATION_STATUS_COL,
        bounded_feedback_gadget: EmlGadgetInstanceSpec::BoundedFeedback {
            id: "r1_disruption_bounded_feedback".to_string(),
            previous_col: DISRUPTION_COL,
            input_col: 2,
            output_col: Some(DISRUPTION_COL),
            decay: DECAY,
            gain: GAIN,
            min: FLOOR,
            max: CEILING,
        },
        scenario: if disabled_no_op {
            DressRehearsalR1Scenario::empty()
        } else {
            input.scenario.clone()
        },
        cell_inputs,
        recurrence_rows,
        diffusion_rows,
        final_disruption,
        location_status,
        starmap_summary: summary.clone(),
        hotspots,
        artifact,
        cpu_oracle_parity,
        deterministic_replay_checksum: checksum,
        no_field_policy_movement: !input.surface.field_policy_movement
            && !input.forbidden.field_policy_movement,
        no_gradientxy_consumption: !input.surface.gradientxy_consumption
            && !input.forbidden.gradientxy_consumption,
        no_recursive_r2_reduce_up: !input.surface.recursive_r2_reduce_up
            && !input.forbidden.recursive_r2_reduce_up,
        no_r3_mask_down: !input.surface.r3_mask_down && !input.forbidden.r3_mask_down,
        no_reenroll: !input.surface.reenroll && !input.forbidden.reenroll,
        no_default_simsession_pass_graph_change: !input
            .surface
            .default_simsession_pass_graph_wiring
            && !input.forbidden.default_simsession_pass_graph_wiring,
        no_global_default_schedule: !input.surface.global_default_schedule
            && !input.forbidden.global_default_schedule,
        no_new_shader_or_wgsl: !input.surface.new_shader_or_wgsl
            && !input.forbidden.semantic_or_raw_wgsl
            && !input.forbidden.new_shader_or_gpu_kernel,
        no_f32_bit_exact_claim: !input.surface.f32_bit_exact_claim
            && !input.forbidden.f32_bit_exact_claim,
        no_ui_realtime_or_cli: !input.surface.ui_or_realtime
            && !input.surface.cli_binary
            && !input.forbidden.ui_or_realtime
            && !input.forbidden.cli_binary,
        no_blockade_or_divert_gate: !input.surface.blockade_or_divert_gate
            && !input.forbidden.disruption_blockade_or_divert,
    }
}

struct Execution {
    cell_inputs: Vec<DressRehearsalR1CellInput>,
    recurrence_rows: Vec<DressRehearsalR1RecurrenceRow>,
    diffusion_rows: Vec<DressRehearsalR1DiffusionRow>,
    final_disruption: Vec<f32>,
    location_status: Vec<f32>,
    summary: DressRehearsalR1Summary,
    hotspots: Vec<DressRehearsalR1Hotspot>,
    artifact: DressRehearsalR1Artifact,
}

fn checksum_field(
    cell_inputs: &[DressRehearsalR1CellInput],
    disruption: &[f32],
    location_status: &[f32],
) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for input in cell_inputs {
        let idx = input.cell_index as usize;
        hash = fnv_append_u64(hash, input.cell_index as u64);
        hash = fnv_append_u64(hash, disruption[idx].to_bits() as u64);
        hash = fnv_append_u64(hash, location_status[idx].to_bits() as u64);
        hash = fnv_append_u64(hash, input.pirate_count as u64);
        hash = fnv_append_u64(hash, input.patrol_count as u64);
        hash = fnv_append_u64(hash, input.inert_count as u64);
    }
    hash
}

fn fnv_append_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn channel_rank(channel: DressRehearsalR1Channel) -> u8 {
    match channel {
        DressRehearsalR1Channel::InertSystem => 0,
        DressRehearsalR1Channel::PirateDisruption => 1,
        DressRehearsalR1Channel::PatrolSuppression => 2,
    }
}

fn owner_rank(owner: DressRehearsalR1Owner) -> u8 {
    match owner {
        DressRehearsalR1Owner::Terran => 0,
        DressRehearsalR1Owner::Pirate => 1,
    }
}
