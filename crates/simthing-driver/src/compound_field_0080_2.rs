//! `COMPOUND-FIELD-0080-2` — rung 2 of the Pirate Gradient Pathfinding scenario.
//!
//! Composes the rung-1 disruption-decay field with a patrol-presence field to produce a
//! **compound desirability field** per node per tick. This scalar field is what rung 3
//! (dual-output `GradientXY` kernel) will extract direction arrows from.
//!
//! Per node, per tick (single-writer; reads rung-1 disruption state as input, never writes it):
//!
//! ```text
//! patrol_field[node]   = patrol_presence_units at this node this tick
//!
//! desirability[node]   = clamp(
//!     BASE_DESIRABILITY
//!     - patrol_repulsion  * patrol_field[node]
//!     - disruption_penalty * (disruption[node] / DISRUPTION_SCALE),
//!     0,
//!     DESIRABILITY_MAX,
//! )
//! ```
//!
//! Interpretation:
//! - **No patrol, no disruption** → highest desirability (virgin target).
//! - **Patrol present** → strongly repels (pirates avoid).
//! - **Disruption present, no patrol** → reduced desirability but not zero — the pirate can
//!   transit through a disrupted system toward a cleaner one. The gradient still points toward
//!   the cleaner system; the corridor emerges naturally from gradient-follow (rung 4).
//!
//! Node positions (`x`, `y` as integer grid coordinates) are stored here so rung 3 has the
//! spatial layout it needs to compute the discrete gradient across neighbours.
//!
//! Opt-in/default-off. Pure CPU-deterministic integer arithmetic — this module is part of
//! the CPU oracle chain a later GPU kernel is parity-checked against (I8 bit-exact parity).
//! No gradient-follow movement (rung 4); no GPU kernel (rung 3); no CPU planner.

use crate::{run_disruption_decay_0080_2, DisruptionDecay0082Input, DISRUPTION_SCALE};

pub const COMPOUND_FIELD_0080_2_ID: &str = "COMPOUND-FIELD-0080-2";
pub const COMPOUND_FIELD_0080_2_SCENARIO: &str = "Pirate Gradient Pathfinding";
pub const COMPOUND_FIELD_0080_2_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - scenario-scoped patrol-presence + compound desirability field";

/// Baseline desirability before patrol/disruption modifiers.
pub const BASE_DESIRABILITY: i64 = 50_000;
/// Ceiling on desirability (clamp target).
pub const DESIRABILITY_MAX: i64 = 100_000;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompoundField0082Gate {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl CompoundField0082Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CompoundField0082Surface {
    pub gate: CompoundField0082Gate,
    pub scenario_scoped_field_registered: bool,
    pub gradient_follow_movement: bool,
    pub default_session_pass_graph_wiring: bool,
    pub global_default_schedule: bool,
    pub realtime_loop: bool,
    pub ui_framework: bool,
}

impl CompoundField0082Surface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: CompoundField0082Gate::explicit_opt_in(),
            scenario_scoped_field_registered: true,
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CompoundField0082ForbiddenRequests {
    pub gradient_follow_movement: bool,
    pub write_to_disruption_column: bool,
    pub second_writer_on_any_field_column: bool,
    pub direct_movement_command: bool,
    pub external_boundary_request: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub default_session_pass_graph_wiring: bool,
    pub global_default_schedule: bool,
    pub realtime_loop_or_ui: bool,
    pub semantic_or_raw_wgsl: bool,
    pub new_shader_or_gpu_kernel: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub clausething_dependency: bool,
    pub simthing_spec_alteration: bool,
    pub invariant_edit: bool,
    pub reopen_closed_0080_1_ladder: bool,
}

/// Integer 2D grid coordinates for a location node.
/// Rung 3 uses these to identify which nodes are neighbours when computing the discrete gradient.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompoundField0082NodePos {
    pub x: i32,
    pub y: i32,
}

/// Weights controlling how patrol presence and disruption level each shape desirability.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompoundField0082Weights {
    /// Desirability reduction per patrol unit present at a node.
    pub patrol_repulsion: i64,
    /// Desirability reduction per disruption *unit* (i.e. per `DISRUPTION_SCALE` of raw disruption).
    pub disruption_penalty: i64,
}

impl CompoundField0082Weights {
    /// Canonical: 1 patrol removes 15 000 desirability; 1 disruption-unit removes 300.
    /// At max disruption (100 units) with no patrol → 50 000 − 30 000 = 20 000 (still passable
    /// as a corridor). A 3-patrol system floors to 0 regardless of disruption.
    pub fn canonical() -> Self {
        Self {
            patrol_repulsion: 15_000,
            disruption_penalty: 300,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompoundField0082Input {
    pub surface: CompoundField0082Surface,
    /// Node positions on the integer grid (one per node, same indexing as the disruption input).
    pub node_positions: Vec<CompoundField0082NodePos>,
    pub weights: CompoundField0082Weights,
    /// Disruption-decay input (rung 1) — defines node count, tick count, and presence schedule.
    pub disruption_input: DisruptionDecay0082Input,
    pub forbidden: CompoundField0082ForbiddenRequests,
}

impl CompoundField0082Input {
    pub fn default_simsession() -> Self {
        Self {
            surface: CompoundField0082Surface::default_simsession(),
            node_positions: Vec::new(),
            weights: CompoundField0082Weights::canonical(),
            disruption_input: DisruptionDecay0082Input::default_simsession(),
            forbidden: CompoundField0082ForbiddenRequests::default(),
        }
    }

    /// Canonical 4-node, 20-tick trial.
    ///
    /// Node layout (simple 1-D line; rung 3 uses these positions for neighbour-gradient):
    /// ```text
    ///  [0]──[1]──[2]──[3]
    ///  (0,0)(1,0)(2,0)(3,0)
    /// ```
    /// - node 0: pirate present ticks 0..=5, patrol ticks 12..=19 (same as rung 1).
    /// - node 1: never any presence — control zero; mid-path corridor node.
    /// - node 2: patrol throughout — heavily repelled (desirability floors to 0 or near 0).
    /// - node 3: pirate every tick — saturated disruption; reduced desirability but traversable.
    ///
    /// Expected gradient direction at tick 10 (after disruption fades from node 0): the
    /// desirability slope runs node 3 (disrupted, low desir) < node 2 (patrolled, floored) <
    /// node 0 (partially recovered) < node 1 (clean, full base_desirability). Rung 3 will
    /// extract that as spatial direction arrows.
    pub fn explicit_opt_in() -> Self {
        use crate::DisruptionDecay0082Input as DDInput;
        Self {
            surface: CompoundField0082Surface::with_explicit_opt_in(),
            node_positions: vec![
                CompoundField0082NodePos { x: 0, y: 0 },
                CompoundField0082NodePos { x: 1, y: 0 },
                CompoundField0082NodePos { x: 2, y: 0 },
                CompoundField0082NodePos { x: 3, y: 0 },
            ],
            weights: CompoundField0082Weights::canonical(),
            disruption_input: DDInput::explicit_opt_in(),
            forbidden: CompoundField0082ForbiddenRequests::default(),
        }
    }
}

/// Per-node field snapshot for one tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompoundField0082TickSnapshot {
    pub tick: u32,
    pub node: usize,
    pub disruption: i64,
    pub patrol_field: i64,
    pub desirability: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompoundField0082Report {
    pub id: &'static str,
    pub status: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,

    // Guardrail confirmations.
    pub no_gradient_follow_movement: bool,
    pub no_write_to_disruption_column: bool,
    pub single_writer_per_field_column: bool,
    pub no_new_gpu_kernel: bool,
    pub no_cpu_planner: bool,
    pub no_default_session_pass_graph_wiring: bool,
    pub no_global_default_schedule: bool,
    pub does_not_reopen_closed_0080_1_ladder: bool,

    // Field results.
    pub node_count: usize,
    pub tick_count: u32,
    pub node_positions: Vec<CompoundField0082NodePos>,
    /// `snapshots[tick * node_count + node]`.
    pub snapshots: Vec<CompoundField0082TickSnapshot>,
    /// Final (last-tick) desirability values per node — the field rung 3 takes the gradient of.
    pub final_desirability: Vec<i64>,
    /// Final disruption per node (from rung 1).
    pub final_disruption: Vec<i64>,

    // Behavioural diagnostics.
    /// A patrolled node's desirability is strictly below the base for the same disruption level.
    pub patrol_repels: bool,
    /// A disrupted node with no patrol still has positive desirability (passable corridor).
    pub disrupted_still_passable: bool,
    /// A clean node (0 patrol, 0 disruption) reaches full base desirability.
    pub clean_node_reaches_base: bool,
    /// Desirability ordering at the final tick matches expected scenario shape.
    pub final_field_ordering_correct: bool,

    pub text_export: String,
    pub deterministic_replay_checksum: u64,
}

pub fn run_compound_field_0080_2(input: &CompoundField0082Input) -> CompoundField0082Report {
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

    // Run rung 1 as a read-only dependency — this module never writes the disruption column.
    let disruption_report = run_disruption_decay_0080_2(&input.disruption_input);
    if !disruption_report.admitted || disruption_report.disabled_no_op {
        let mut r = rejected_report(input, Vec::new());
        r.diagnostics.push("disruption_rung_not_admitted");
        r.admitted = false;
        return r;
    }

    admitted_report(input, disruption_report)
}

pub fn replay_compound_field_0080_2() -> (CompoundField0082Report, CompoundField0082Report) {
    let input = CompoundField0082Input::explicit_opt_in();
    (
        run_compound_field_0080_2(&input),
        run_compound_field_0080_2(&input),
    )
}

fn validate_surface(surface: &CompoundField0082Surface, diagnostics: &mut Vec<&'static str>) {
    if surface.gate.enabled_by_default {
        diagnostics.push("compound_field_default_on_rejected");
    }
    if surface.gradient_follow_movement {
        diagnostics.push("gradient_follow_movement_not_in_this_rung");
    }
    if surface.default_session_pass_graph_wiring {
        diagnostics.push("default_session_pass_graph_wiring");
    }
    if surface.global_default_schedule {
        diagnostics.push("global_default_schedule");
    }
    if surface.realtime_loop {
        diagnostics.push("realtime_loop");
    }
    if surface.ui_framework {
        diagnostics.push("ui_framework");
    }
}

fn validate_forbidden(
    forbidden: &CompoundField0082ForbiddenRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.gradient_follow_movement {
        diagnostics.push("gradient_follow_movement_not_in_this_rung");
    }
    if forbidden.write_to_disruption_column {
        diagnostics.push("write_to_disruption_column_rejected");
    }
    if forbidden.second_writer_on_any_field_column {
        diagnostics.push("second_writer_on_any_field_column");
    }
    if forbidden.direct_movement_command {
        diagnostics.push("direct_movement_command");
    }
    if forbidden.external_boundary_request {
        diagnostics.push("external_boundary_request");
    }
    if forbidden.cpu_planner_urgency_commitment {
        diagnostics.push("cpu_planner_urgency_commitment");
    }
    if forbidden.default_session_pass_graph_wiring {
        diagnostics.push("default_session_pass_graph_wiring");
    }
    if forbidden.global_default_schedule {
        diagnostics.push("global_default_schedule");
    }
    if forbidden.realtime_loop_or_ui {
        diagnostics.push("realtime_loop_or_ui");
    }
    if forbidden.semantic_or_raw_wgsl {
        diagnostics.push("semantic_or_raw_wgsl");
    }
    if forbidden.new_shader_or_gpu_kernel {
        diagnostics.push("new_shader_or_gpu_kernel");
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

fn validate_params(input: &CompoundField0082Input, diagnostics: &mut Vec<&'static str>) {
    if input.weights.patrol_repulsion < 0 {
        diagnostics.push("negative_patrol_repulsion");
    }
    if input.weights.disruption_penalty < 0 {
        diagnostics.push("negative_disruption_penalty");
    }
    if input.surface.gate.explicit_opt_in {
        let node_count = input.disruption_input.node_count;
        if node_count == 0 {
            diagnostics.push("empty_node_set");
        }
        if input.node_positions.len() != node_count {
            diagnostics.push("node_positions_count_mismatch");
        }
    }
}

fn compute_snapshots(
    input: &CompoundField0082Input,
    disruption_rows: &[crate::DisruptionDecay0082Row],
    node_count: usize,
    tick_count: u32,
) -> Vec<CompoundField0082TickSnapshot> {
    let mut snapshots = Vec::with_capacity(tick_count as usize * node_count);
    for tick in 0..tick_count {
        for node in 0..node_count {
            // Disruption after the rung-1 update band for this tick.
            let disruption_row = disruption_rows
                .iter()
                .find(|r| r.tick == tick && r.node == node);
            let disruption = disruption_row.map(|r| r.disruption_after).unwrap_or(0);

            // Patrol field: read from the presence schedule (rung 1's patrol_presence input).
            let patrol_field = input
                .disruption_input
                .presence_schedule
                .get(tick as usize)
                .and_then(|row| row.get(node))
                .map(|p| p.patrol_presence)
                .unwrap_or(0);

            // Compound desirability: single writer, reads disruption + patrol as inputs only.
            let disruption_units = disruption / DISRUPTION_SCALE;
            let desirability = (BASE_DESIRABILITY
                - input.weights.patrol_repulsion * patrol_field
                - input.weights.disruption_penalty * disruption_units)
                .clamp(0, DESIRABILITY_MAX);

            snapshots.push(CompoundField0082TickSnapshot {
                tick,
                node,
                disruption,
                patrol_field,
                desirability,
            });
        }
    }
    snapshots
}

fn behavioural_diagnostics(
    snapshots: &[CompoundField0082TickSnapshot],
    node_count: usize,
    tick_count: u32,
) -> (bool, bool, bool, bool) {
    let last_tick = tick_count.saturating_sub(1);

    // patrol_repels: some snapshot with patrol > 0 has desirability < BASE_DESIRABILITY.
    let patrol_repels = snapshots
        .iter()
        .any(|s| s.patrol_field > 0 && s.desirability < BASE_DESIRABILITY);

    // disrupted_still_passable: some snapshot with disruption > 0 and patrol = 0 has desirability > 0.
    let disrupted_still_passable = snapshots
        .iter()
        .any(|s| s.disruption > 0 && s.patrol_field == 0 && s.desirability > 0);

    // clean_node_reaches_base: some snapshot with disruption = 0 and patrol = 0
    // has desirability == BASE_DESIRABILITY.
    let clean_node_reaches_base = snapshots
        .iter()
        .any(|s| s.disruption == 0 && s.patrol_field == 0 && s.desirability == BASE_DESIRABILITY);

    // final_field_ordering_correct: at the last tick, node 1 (clean) >= node 0 (partially
    // recovered) > node 3 (disrupted) AND node 2 (patrolled) is strictly below node 1.
    // Uses the canonical 4-node layout; skip check if node_count != 4.
    let final_field_ordering_correct = if node_count == 4 {
        let d = |node: usize| {
            snapshots
                .iter()
                .find(|s| s.tick == last_tick && s.node == node)
                .map(|s| s.desirability)
                .unwrap_or(0)
        };
        d(1) >= d(0) && d(0) > d(3) && d(2) < d(1)
    } else {
        true // skip ordering check for non-canonical layouts
    };

    (
        patrol_repels,
        disrupted_still_passable,
        clean_node_reaches_base,
        final_field_ordering_correct,
    )
}

fn disabled_no_op_report(input: &CompoundField0082Input) -> CompoundField0082Report {
    base_report(input, true, Vec::new(), Vec::new())
}

fn rejected_report(
    input: &CompoundField0082Input,
    diagnostics: Vec<&'static str>,
) -> CompoundField0082Report {
    let mut report = base_report(input, false, diagnostics, Vec::new());
    report.admitted = false;
    report
}

fn admitted_report(
    input: &CompoundField0082Input,
    disruption_report: crate::DisruptionDecay0082Report,
) -> CompoundField0082Report {
    let node_count = input.disruption_input.node_count;
    let tick_count = input.disruption_input.tick_count;
    let snapshots = compute_snapshots(input, &disruption_report.rows, node_count, tick_count);
    base_report(input, false, Vec::new(), snapshots)
}

fn base_report(
    input: &CompoundField0082Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    snapshots: Vec<CompoundField0082TickSnapshot>,
) -> CompoundField0082Report {
    let opt_in = input.surface.gate.explicit_opt_in;
    let node_count = if disabled_no_op {
        0
    } else {
        input.disruption_input.node_count
    };
    let tick_count = if disabled_no_op {
        0
    } else {
        input.disruption_input.tick_count
    };

    let final_desirability = if !snapshots.is_empty() && tick_count > 0 {
        let last_tick = tick_count - 1;
        (0..node_count)
            .map(|node| {
                snapshots
                    .iter()
                    .find(|s| s.tick == last_tick && s.node == node)
                    .map(|s| s.desirability)
                    .unwrap_or(0)
            })
            .collect()
    } else {
        Vec::new()
    };

    let final_disruption = if !snapshots.is_empty() && tick_count > 0 {
        let last_tick = tick_count - 1;
        (0..node_count)
            .map(|node| {
                snapshots
                    .iter()
                    .find(|s| s.tick == last_tick && s.node == node)
                    .map(|s| s.disruption)
                    .unwrap_or(0)
            })
            .collect()
    } else {
        Vec::new()
    };

    let (
        patrol_repels,
        disrupted_still_passable,
        clean_node_reaches_base,
        final_field_ordering_correct,
    ) = if !snapshots.is_empty() {
        behavioural_diagnostics(&snapshots, node_count, tick_count)
    } else {
        (false, false, false, false)
    };

    let text_export = if !disabled_no_op && opt_in {
        render_export(
            input,
            &snapshots,
            &final_desirability,
            node_count,
            tick_count,
        )
    } else {
        String::new()
    };

    let node_positions = if disabled_no_op {
        Vec::new()
    } else {
        input.node_positions.clone()
    };

    let mut report = CompoundField0082Report {
        id: COMPOUND_FIELD_0080_2_ID,
        status: COMPOUND_FIELD_0080_2_STATUS_PASS,
        scenario_name: COMPOUND_FIELD_0080_2_SCENARIO,
        admitted: true,
        diagnostics,
        explicit_opt_in: opt_in,
        default_off: !input.surface.gate.enabled_by_default,
        disabled_no_op,
        no_gradient_follow_movement: !input.surface.gradient_follow_movement
            && !input.forbidden.gradient_follow_movement,
        no_write_to_disruption_column: !input.forbidden.write_to_disruption_column,
        single_writer_per_field_column: !input.forbidden.second_writer_on_any_field_column,
        no_new_gpu_kernel: !input.forbidden.new_shader_or_gpu_kernel,
        no_cpu_planner: !input.forbidden.cpu_planner_urgency_commitment,
        no_default_session_pass_graph_wiring: !input.surface.default_session_pass_graph_wiring
            && !input.forbidden.default_session_pass_graph_wiring,
        no_global_default_schedule: !input.surface.global_default_schedule
            && !input.forbidden.global_default_schedule,
        does_not_reopen_closed_0080_1_ladder: !input.forbidden.reopen_closed_0080_1_ladder,
        node_count,
        tick_count,
        node_positions,
        snapshots,
        final_desirability,
        final_disruption,
        patrol_repels,
        disrupted_still_passable,
        clean_node_reaches_base,
        final_field_ordering_correct,
        text_export,
        deterministic_replay_checksum: 0,
    };
    report.deterministic_replay_checksum = checksum_report(&report);
    report
}

fn render_export(
    input: &CompoundField0082Input,
    snapshots: &[CompoundField0082TickSnapshot],
    final_desirability: &[i64],
    node_count: usize,
    tick_count: u32,
) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "COMPOUND-FIELD-0080-2|scenario={}|nodes={}|ticks={}|patrol_repulsion={}|disruption_penalty={}|base={}",
        COMPOUND_FIELD_0080_2_SCENARIO,
        node_count,
        tick_count,
        input.weights.patrol_repulsion,
        input.weights.disruption_penalty,
        BASE_DESIRABILITY,
    ));
    for pos in &input.node_positions {
        lines.push(format!("NODE_POS|x={}|y={}", pos.x, pos.y,));
    }
    for snap in snapshots {
        lines.push(format!(
            "FIELD|t={}|node={}|disruption={}|patrol={}|desirability={}",
            snap.tick, snap.node, snap.disruption, snap.patrol_field, snap.desirability,
        ));
    }
    for (node, &d) in final_desirability.iter().enumerate() {
        lines.push(format!("FINAL|node={}|desirability={}", node, d));
    }
    lines.join("\n")
}

fn checksum_report(report: &CompoundField0082Report) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    hash = fnv_append_u64(hash, report.node_count as u64);
    hash = fnv_append_u64(hash, report.tick_count as u64);
    for snap in &report.snapshots {
        hash = fnv_append_u64(hash, snap.tick as u64);
        hash = fnv_append_u64(hash, snap.node as u64);
        hash = fnv_append_u64(hash, snap.disruption as u64);
        hash = fnv_append_u64(hash, snap.patrol_field as u64);
        hash = fnv_append_u64(hash, snap.desirability as u64);
    }
    for &d in &report.final_desirability {
        hash = fnv_append_u64(hash, d as u64);
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
