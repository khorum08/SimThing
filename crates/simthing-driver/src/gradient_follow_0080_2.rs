//! `GRADIENT-FOLLOW-0080-2` — rung 4 (final) of the Pirate Gradient Pathfinding scenario.
//!
//! Closes the loop: a pirate mover does GPU-resident-style **field-as-policy** movement by
//! ascending the compound desirability field (rungs 1–2) along its **dual-output gradient**
//! (the rung-3 `GradientXY` contract, computed in integer over the sparse node graph).
//!
//! Per tick (single writer per field column; deterministic integer/fixed-point):
//! 1. the pirate emits disruption at its current node; patrols are scripted per tick;
//! 2. each node's disruption advances by the rung-1 bounded-feedback recurrence;
//! 3. each node's compound desirability is recomputed by the rung-2 formula;
//! 4. the dual-output gradient `(dx, dy)` at the pirate's node is read from neighbour
//!    desirability (`dx = desir[E] − desir[W]`, `dy = desir[S] − desir[N]`, clamp boundary);
//! 5. **SEAD threshold gate:** if `max(|dx|,|dy|) >= movement_threshold`, an event is emitted
//!    and the pirate takes **one** step toward the higher-desirability neighbour (greedy local
//!    ascent, dominant axis, deterministic tie-break). Otherwise it stays.
//!
//! The self-disruption left behind lowers desirability where the pirate has been, so the
//! gradient continually points toward fresher systems; patrol presence repels. Movement is
//! **not** a CPU planner: direction is the field gradient, the commitment is a threshold
//! crossing, and exactly one node-step happens per tick. No lookahead, no multi-step
//! pathfinding, no urgency computation. Opt-in/default-off. Does not reopen the closed
//! `0080-1` ladder.

use std::collections::HashMap;

use crate::{
    CompoundField0082NodePos, CompoundField0082Weights, DisruptionDecay0082DecayWeights,
    BASE_DESIRABILITY, DESIRABILITY_MAX, DISRUPTION_MAX, DISRUPTION_SCALE,
};

pub const GRADIENT_FOLLOW_0080_2_ID: &str = "GRADIENT-FOLLOW-0080-2";
pub const GRADIENT_FOLLOW_0080_2_SCENARIO: &str = "Pirate Gradient Pathfinding";
pub const GRADIENT_FOLLOW_0080_2_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - scenario-scoped gradient-follow SEAD movement + 20-tick schedule";

const DEFAULT_TICK_COUNT: u32 = 20;
const MAX_DECAY_MODIFIERS: usize = 8;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GradientFollow0082Gate {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl GradientFollow0082Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GradientFollow0082Surface {
    pub gate: GradientFollow0082Gate,
    pub scenario_scoped_schedule_registered: bool,
    pub cpu_planner_or_lookahead: bool,
    pub multi_step_pathfinding: bool,
    pub direct_movement_command: bool,
    pub global_default_schedule: bool,
    pub default_session_pass_graph_wiring: bool,
    pub realtime_loop: bool,
    pub ui_framework: bool,
}

impl GradientFollow0082Surface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: GradientFollow0082Gate::explicit_opt_in(),
            scenario_scoped_schedule_registered: true,
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GradientFollow0082ForbiddenRequests {
    pub cpu_planner_or_lookahead: bool,
    pub multi_step_pathfinding: bool,
    pub direct_movement_command: bool,
    pub external_boundary_request: bool,
    pub global_default_schedule: bool,
    pub default_session_pass_graph_wiring: bool,
    pub realtime_loop_or_ui: bool,
    pub semantic_or_raw_wgsl: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub clausething_dependency: bool,
    pub simthing_spec_alteration: bool,
    pub invariant_edit: bool,
    pub reopen_closed_0080_1_ladder: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GradientFollow0082Input {
    pub surface: GradientFollow0082Surface,
    pub node_positions: Vec<CompoundField0082NodePos>,
    pub weights: CompoundField0082Weights,
    pub decay_weights: DisruptionDecay0082DecayWeights,
    pub gain_per_presence_unit: i64,
    pub suppression_per_patrol: i64,
    /// Disruption the moving pirate emits at whatever node it currently occupies.
    pub pirate_presence_units: i64,
    pub start_node: usize,
    pub tick_count: u32,
    /// SEAD threshold on the gradient magnitude `max(|dx|,|dy|)`; below this the pirate stays.
    pub movement_threshold: i64,
    /// `patrol_schedule[tick][node]` — scripted patrol presence (patrols are not gradient-followers).
    pub patrol_schedule: Vec<Vec<i64>>,
    pub forbidden: GradientFollow0082ForbiddenRequests,
}

impl GradientFollow0082Input {
    pub fn default_simsession() -> Self {
        Self {
            surface: GradientFollow0082Surface::default_simsession(),
            node_positions: Vec::new(),
            weights: CompoundField0082Weights::canonical(),
            decay_weights: DisruptionDecay0082DecayWeights::canonical(),
            gain_per_presence_unit: 0,
            suppression_per_patrol: 0,
            pirate_presence_units: 0,
            start_node: 0,
            tick_count: 0,
            movement_threshold: 0,
            patrol_schedule: Vec::new(),
            forbidden: GradientFollow0082ForbiddenRequests::default(),
        }
    }

    /// Canonical 5-node line trial. Pirate starts at node 0; no patrols. The field starts flat
    /// (gradient 0 → no move), but the pirate's self-disruption lowers its current node's
    /// desirability, opening a forward gradient that crosses the threshold and drives an
    /// eastward migration along the line over 20 ticks.
    ///
    /// ```text
    ///  [0]──[1]──[2]──[3]──[4]
    /// ```
    pub fn explicit_opt_in() -> Self {
        let node_positions = (0..5)
            .map(|x| CompoundField0082NodePos { x, y: 0 })
            .collect::<Vec<_>>();
        let tick_count = DEFAULT_TICK_COUNT;
        let patrol_schedule = vec![vec![0i64; 5]; tick_count as usize];
        Self {
            surface: GradientFollow0082Surface::with_explicit_opt_in(),
            node_positions,
            weights: CompoundField0082Weights::canonical(),
            decay_weights: DisruptionDecay0082DecayWeights::canonical(),
            gain_per_presence_unit: 2_000,
            suppression_per_patrol: 5_000,
            pirate_presence_units: 10,
            start_node: 0,
            tick_count,
            movement_threshold: 5_000,
            patrol_schedule,
            forbidden: GradientFollow0082ForbiddenRequests::default(),
        }
    }

    /// Variant: a stationary patrol parked mid-line (node 2) that repels the pirate.
    pub fn explicit_opt_in_with_mid_patrol() -> Self {
        let mut input = Self::explicit_opt_in();
        for row in &mut input.patrol_schedule {
            row[2] = 1;
        }
        input
    }
}

/// One tick of the gradient-follow schedule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GradientFollow0082MoveRow {
    pub tick: u32,
    pub pirate_node: usize,
    pub gradient_dx: i64,
    pub gradient_dy: i64,
    pub gradient_magnitude: i64,
    pub threshold_crossed: bool,
    pub event_emitted: bool,
    pub moved: bool,
    pub moved_to_node: usize,
    pub move_direction: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GradientFollow0082Report {
    pub id: &'static str,
    pub status: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,

    // Guardrail confirmations.
    pub field_sourced_movement: bool,
    pub no_cpu_planner_or_lookahead: bool,
    pub no_multi_step_pathfinding: bool,
    pub single_step_per_tick: bool,
    pub threshold_gated: bool,
    pub no_direct_movement_command: bool,
    pub no_global_default_schedule: bool,
    pub no_default_session_pass_graph_wiring: bool,
    pub does_not_reopen_closed_0080_1_ladder: bool,

    // Results.
    pub node_count: usize,
    pub tick_count: u32,
    pub start_node: usize,
    pub final_node: usize,
    pub total_moves: u32,
    pub visited_nodes: Vec<usize>,
    pub max_distance_from_start: u32,
    pub move_rows: Vec<GradientFollow0082MoveRow>,

    pub text_export: String,
    pub deterministic_replay_checksum: u64,
}

pub fn run_gradient_follow_0080_2(
    input: &GradientFollow0082Input,
) -> GradientFollow0082Report {
    let mut diagnostics = Vec::new();
    validate_surface(&input.surface, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);
    validate_params(input, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(input, diagnostics);
    }

    if !input.surface.gate.explicit_opt_in {
        return disabled_no_op_report(input);
    }

    let run = run_schedule(input);
    admitted_report(input, run)
}

pub fn replay_gradient_follow_0080_2() -> (GradientFollow0082Report, GradientFollow0082Report) {
    let input = GradientFollow0082Input::explicit_opt_in();
    (
        run_gradient_follow_0080_2(&input),
        run_gradient_follow_0080_2(&input),
    )
}

fn validate_surface(surface: &GradientFollow0082Surface, diagnostics: &mut Vec<&'static str>) {
    if surface.gate.enabled_by_default {
        diagnostics.push("gradient_follow_default_on_rejected");
    }
    if surface.cpu_planner_or_lookahead {
        diagnostics.push("cpu_planner_or_lookahead");
    }
    if surface.multi_step_pathfinding {
        diagnostics.push("multi_step_pathfinding");
    }
    if surface.direct_movement_command {
        diagnostics.push("direct_movement_command");
    }
    if surface.global_default_schedule {
        diagnostics.push("global_default_schedule");
    }
    if surface.default_session_pass_graph_wiring {
        diagnostics.push("default_session_pass_graph_wiring");
    }
    if surface.realtime_loop {
        diagnostics.push("realtime_loop");
    }
    if surface.ui_framework {
        diagnostics.push("ui_framework");
    }
}

fn validate_forbidden(
    forbidden: &GradientFollow0082ForbiddenRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.cpu_planner_or_lookahead {
        diagnostics.push("cpu_planner_or_lookahead");
    }
    if forbidden.multi_step_pathfinding {
        diagnostics.push("multi_step_pathfinding");
    }
    if forbidden.direct_movement_command {
        diagnostics.push("direct_movement_command");
    }
    if forbidden.external_boundary_request {
        diagnostics.push("external_boundary_request");
    }
    if forbidden.global_default_schedule {
        diagnostics.push("global_default_schedule");
    }
    if forbidden.default_session_pass_graph_wiring {
        diagnostics.push("default_session_pass_graph_wiring");
    }
    if forbidden.realtime_loop_or_ui {
        diagnostics.push("realtime_loop_or_ui");
    }
    if forbidden.semantic_or_raw_wgsl {
        diagnostics.push("semantic_or_raw_wgsl");
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
    if forbidden.simthing_spec_alteration {
        diagnostics.push("simthing_spec_alteration");
    }
    if forbidden.invariant_edit {
        diagnostics.push("invariant_edit");
    }
    if forbidden.reopen_closed_0080_1_ladder {
        diagnostics.push("reopen_closed_0080_1_ladder");
    }
}

fn validate_params(input: &GradientFollow0082Input, diagnostics: &mut Vec<&'static str>) {
    if input.decay_weights.modifiers.len() > MAX_DECAY_MODIFIERS {
        diagnostics.push("too_many_decay_modifiers");
    }
    if input.decay_weights.base_retain_den == 0
        || input.decay_weights.base_retain_num >= input.decay_weights.base_retain_den
    {
        diagnostics.push("unbounded_decay_coefficient");
    }
    if input.gain_per_presence_unit < 0 || input.suppression_per_patrol < 0 {
        diagnostics.push("negative_rate");
    }
    if input.pirate_presence_units < 0 {
        diagnostics.push("negative_pirate_presence");
    }
    if input.movement_threshold < 0 {
        diagnostics.push("negative_movement_threshold");
    }
    if input.surface.gate.explicit_opt_in {
        let n = input.node_positions.len();
        if n == 0 {
            diagnostics.push("empty_node_set");
        }
        if input.start_node >= n {
            diagnostics.push("start_node_out_of_range");
        }
        if input.tick_count == 0 {
            diagnostics.push("empty_tick_set");
        }
        if input.patrol_schedule.len() != input.tick_count as usize {
            diagnostics.push("patrol_schedule_tick_shape_mismatch");
        }
        if input.patrol_schedule.iter().any(|row| row.len() != n) {
            diagnostics.push("patrol_schedule_node_shape_mismatch");
        }
    }
}

struct ScheduleResult {
    move_rows: Vec<GradientFollow0082MoveRow>,
    final_node: usize,
    total_moves: u32,
    visited_nodes: Vec<usize>,
    max_distance_from_start: u32,
    single_step_per_tick: bool,
}

/// Compose the bounded retention coefficient (mirrors rung 1). Returns `(num, den)`.
fn compose_retention(weights: &DisruptionDecay0082DecayWeights) -> (u128, u128) {
    let mut num: u128 = weights.base_retain_num as u128;
    let mut den: u128 = weights.base_retain_den as u128;
    for m in &weights.modifiers {
        num *= m.num as u128;
        den *= m.den as u128;
    }
    (num, den)
}

fn run_schedule(input: &GradientFollow0082Input) -> ScheduleResult {
    let n = input.node_positions.len();
    let (retain_num, retain_den) = compose_retention(&input.decay_weights);

    // Position -> node index, for neighbour lookup.
    let mut pos_to_node: HashMap<(i32, i32), usize> = HashMap::with_capacity(n);
    for (i, p) in input.node_positions.iter().enumerate() {
        pos_to_node.insert((p.x, p.y), i);
    }

    let mut disruption = vec![0i64; n];
    let mut pirate = input.start_node;
    let mut move_rows = Vec::with_capacity(input.tick_count as usize);
    let mut total_moves = 0u32;
    let mut visited = vec![pirate];
    let mut single_step_per_tick = true;
    let start_pos = input.node_positions[input.start_node];
    let mut max_distance = 0u32;

    for tick in 0..input.tick_count {
        let patrol_row = &input.patrol_schedule[tick as usize];

        // 1-3. advance disruption (single writer) and derive desirability per node.
        let mut desirability = vec![0i64; n];
        for node in 0..n {
            let mover_units = if node == pirate {
                input.pirate_presence_units
            } else {
                0
            };
            let retained = ((disruption[node] as u128 * retain_num) / retain_den) as i64;
            let gained = mover_units.saturating_mul(input.gain_per_presence_unit);
            let suppressed = patrol_row[node].saturating_mul(input.suppression_per_patrol);
            let next = (retained + gained - suppressed).clamp(0, DISRUPTION_MAX);
            disruption[node] = next;

            let disruption_units = next / DISRUPTION_SCALE;
            desirability[node] = (BASE_DESIRABILITY
                - input.weights.patrol_repulsion * patrol_row[node]
                - input.weights.disruption_penalty * disruption_units)
                .clamp(0, DESIRABILITY_MAX);
        }

        // 4. dual-output gradient at the pirate's node (clamp boundary: missing neighbour = center).
        let here = input.node_positions[pirate];
        let center = desirability[pirate];
        let sample = |dx: i32, dy: i32| -> i64 {
            pos_to_node
                .get(&(here.x + dx, here.y + dy))
                .map(|&idx| desirability[idx])
                .unwrap_or(center)
        };
        let gx = sample(1, 0) - sample(-1, 0); // east - west
        let gy = sample(0, 1) - sample(0, -1); // south - north
        let magnitude = gx.abs().max(gy.abs());

        // 5. SEAD threshold gate + single greedy ascent step (dominant axis, deterministic).
        let threshold_crossed = magnitude >= input.movement_threshold && magnitude > 0;
        let mut moved = false;
        let mut moved_to = pirate;
        let mut direction = "none";
        if threshold_crossed {
            let (ddx, ddy, dir) = if gx.abs() >= gy.abs() {
                if gx > 0 {
                    (1, 0, "east")
                } else {
                    (-1, 0, "west")
                }
            } else if gy > 0 {
                (0, 1, "south")
            } else {
                (0, -1, "north")
            };
            if let Some(&target) = pos_to_node.get(&(here.x + ddx, here.y + ddy)) {
                moved_to = target;
                moved = true;
                direction = dir;
            }
        }

        move_rows.push(GradientFollow0082MoveRow {
            tick,
            pirate_node: pirate,
            gradient_dx: gx,
            gradient_dy: gy,
            gradient_magnitude: magnitude,
            threshold_crossed,
            event_emitted: threshold_crossed,
            moved,
            moved_to_node: moved_to,
            move_direction: direction,
        });

        if moved {
            // Verify single-step: Manhattan distance between old and new node positions == 1.
            let from = input.node_positions[pirate];
            let to = input.node_positions[moved_to];
            let manhattan = (from.x - to.x).unsigned_abs() + (from.y - to.y).unsigned_abs();
            if manhattan != 1 {
                single_step_per_tick = false;
            }
            total_moves += 1;
            pirate = moved_to;
            if !visited.contains(&pirate) {
                visited.push(pirate);
            }
            let cur = input.node_positions[pirate];
            let dist = (cur.x - start_pos.x).unsigned_abs() + (cur.y - start_pos.y).unsigned_abs();
            max_distance = max_distance.max(dist);
        }
    }

    ScheduleResult {
        move_rows,
        final_node: pirate,
        total_moves,
        visited_nodes: visited,
        max_distance_from_start: max_distance,
        single_step_per_tick,
    }
}

fn disabled_no_op_report(input: &GradientFollow0082Input) -> GradientFollow0082Report {
    base_report(input, true, Vec::new(), None)
}

fn rejected_report(
    input: &GradientFollow0082Input,
    diagnostics: Vec<&'static str>,
) -> GradientFollow0082Report {
    let mut report = base_report(input, false, diagnostics, None);
    report.admitted = false;
    report
}

fn admitted_report(
    input: &GradientFollow0082Input,
    run: ScheduleResult,
) -> GradientFollow0082Report {
    base_report(input, false, Vec::new(), Some(run))
}

fn base_report(
    input: &GradientFollow0082Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    run: Option<ScheduleResult>,
) -> GradientFollow0082Report {
    let opt_in = input.surface.gate.explicit_opt_in;
    let (
        move_rows,
        final_node,
        total_moves,
        visited_nodes,
        max_distance_from_start,
        single_step_per_tick,
    ) = match run {
        Some(r) => (
            r.move_rows,
            r.final_node,
            r.total_moves,
            r.visited_nodes,
            r.max_distance_from_start,
            r.single_step_per_tick,
        ),
        None => (Vec::new(), input.start_node, 0, Vec::new(), 0, true),
    };

    // Threshold gating is demonstrable iff some tick was below threshold OR all moves obeyed it.
    let threshold_gated = move_rows.iter().all(|r| r.moved == (r.threshold_crossed && r.moved));

    let text_export = if !disabled_no_op && opt_in {
        render_export(input, &move_rows, final_node, total_moves, &visited_nodes)
    } else {
        String::new()
    };

    let mut report = GradientFollow0082Report {
        id: GRADIENT_FOLLOW_0080_2_ID,
        status: GRADIENT_FOLLOW_0080_2_STATUS_PASS,
        scenario_name: GRADIENT_FOLLOW_0080_2_SCENARIO,
        admitted: true,
        diagnostics,
        explicit_opt_in: opt_in,
        default_off: !input.surface.gate.enabled_by_default,
        disabled_no_op,
        field_sourced_movement: true,
        no_cpu_planner_or_lookahead: !input.surface.cpu_planner_or_lookahead
            && !input.forbidden.cpu_planner_or_lookahead,
        no_multi_step_pathfinding: !input.surface.multi_step_pathfinding
            && !input.forbidden.multi_step_pathfinding,
        single_step_per_tick,
        threshold_gated,
        no_direct_movement_command: !input.surface.direct_movement_command
            && !input.forbidden.direct_movement_command,
        no_global_default_schedule: !input.surface.global_default_schedule
            && !input.forbidden.global_default_schedule,
        no_default_session_pass_graph_wiring: !input.surface.default_session_pass_graph_wiring
            && !input.forbidden.default_session_pass_graph_wiring,
        does_not_reopen_closed_0080_1_ladder: !input.forbidden.reopen_closed_0080_1_ladder,
        node_count: if disabled_no_op {
            0
        } else {
            input.node_positions.len()
        },
        tick_count: if disabled_no_op { 0 } else { input.tick_count },
        start_node: input.start_node,
        final_node,
        total_moves,
        visited_nodes,
        max_distance_from_start,
        move_rows,
        text_export,
        deterministic_replay_checksum: 0,
    };
    report.deterministic_replay_checksum = checksum_report(&report);
    report
}

fn render_export(
    input: &GradientFollow0082Input,
    move_rows: &[GradientFollow0082MoveRow],
    final_node: usize,
    total_moves: u32,
    visited_nodes: &[usize],
) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "GRADIENT-FOLLOW-0080-2|scenario={}|nodes={}|ticks={}|start={}|threshold={}|final={}|moves={}|visited={}",
        GRADIENT_FOLLOW_0080_2_SCENARIO,
        input.node_positions.len(),
        input.tick_count,
        input.start_node,
        input.movement_threshold,
        final_node,
        total_moves,
        visited_nodes.len(),
    ));
    for row in move_rows {
        lines.push(format!(
            "STEP|t={}|at={}|dx={}|dy={}|mag={}|event={}|moved={}|to={}|dir={}",
            row.tick,
            row.pirate_node,
            row.gradient_dx,
            row.gradient_dy,
            row.gradient_magnitude,
            row.event_emitted,
            row.moved,
            row.moved_to_node,
            row.move_direction,
        ));
    }
    lines.join("\n")
}

fn checksum_report(report: &GradientFollow0082Report) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    hash = fnv_append_u64(hash, report.node_count as u64);
    hash = fnv_append_u64(hash, report.tick_count as u64);
    hash = fnv_append_u64(hash, report.start_node as u64);
    hash = fnv_append_u64(hash, report.final_node as u64);
    hash = fnv_append_u64(hash, report.total_moves as u64);
    for row in &report.move_rows {
        hash = fnv_append_u64(hash, row.tick as u64);
        hash = fnv_append_u64(hash, row.pirate_node as u64);
        hash = fnv_append_u64(hash, row.gradient_dx as u64);
        hash = fnv_append_u64(hash, row.gradient_dy as u64);
        hash = fnv_append_u64(hash, row.moved_to_node as u64);
        hash = fnv_append_u64(hash, row.event_emitted as u64);
    }
    hash
}

fn fnv_append_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
